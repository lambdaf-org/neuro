use crate::models::app_state::AppState;
use crate::models::user::LoginPayload;
use crate::models::user::LoginRes;
use crate::models::user::RegisterPayload;
use actix_web::HttpResponse;
use actix_web::web;
use log::error;
use supabase_auth::models::SignUpWithPasswordOptions;

pub async fn register(
    body: web::Json<RegisterPayload>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let options = SignUpWithPasswordOptions {
        data: Some(serde_json::json!({
            "display_name": body.username
        })),
        ..Default::default()
    };
    match state
        .auth_client
        .sign_up_with_email_and_password(&body.email, &body.password, Some(options))
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            let err = e.to_string();
            if err.contains("email_exists") {
                HttpResponse::Conflict().body("Email already registered")
            } else if err.contains("weak_password") {
                HttpResponse::BadRequest().body("Password too weak")
            } else if err.contains("over_request_rate_limit") {
                HttpResponse::TooManyRequests().body("Too many attempts")
            } else {
                error!("Registration failed: {e}");
                HttpResponse::InternalServerError().body("Registration failed")
            }
        }
    }
}

pub async fn login(body: web::Json<LoginPayload>, state: web::Data<AppState>) -> HttpResponse {
    let session = match state
        .auth_client
        .login_with_email(&body.email, &body.password)
        .await
    {
        Ok(session) => session,
        Err(e) => {
            let err = e.to_string();
            if err.contains("not confirmed") {
                return HttpResponse::Forbidden().body("Email not confirmed");
            } else if err.contains("invalid credentials") {
                return HttpResponse::Unauthorized().body("Invalid credentials");
            } else if err.contains("rate limit") {
                return HttpResponse::TooManyRequests().body("Too many attempts");
            } else {
                error!("Login failed for {}: {e}", body.email);
                return HttpResponse::InternalServerError().body("Login failed");
            }
        }
    };
    HttpResponse::Ok().json(LoginRes {
        user_id: session.user.id,
        email: body.email.clone(),
        access_token: session.access_token,
    })
}
