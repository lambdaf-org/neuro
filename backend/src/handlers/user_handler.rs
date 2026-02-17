use crate::errors::auth_errors::AuthError;
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
            let auth_error = AuthError::from_supabase_error(&e);
            match auth_error {
                AuthError::EmailAlreadyExists => {
                    HttpResponse::Conflict().body("Email already registered")
                }
                AuthError::UsernameAlreadyTaken => {
                    HttpResponse::Conflict().body("Username already taken")
                }
                AuthError::AlreadyExists => {
                    HttpResponse::Conflict().body("Already exists")
                }
                AuthError::WeakPassword => {
                    HttpResponse::BadRequest().body("Password too weak")
                }
                AuthError::RateLimited => {
                    HttpResponse::TooManyRequests().body("Too many attempts")
                }
                AuthError::Other(_) => {
                    error!("Registration failed: {e}");
                    HttpResponse::InternalServerError().body("Registration failed")
                }
                _ => {
                    error!("Registration failed: {e}");
                    HttpResponse::InternalServerError().body("Registration failed")
                }
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
            let auth_error = AuthError::from_supabase_error(&e);
            match auth_error {
                AuthError::EmailNotConfirmed => {
                    return HttpResponse::Forbidden().body("Email not confirmed");
                }
                AuthError::InvalidCredentials => {
                    return HttpResponse::Unauthorized().body("Invalid credentials");
                }
                AuthError::RateLimited => {
                    return HttpResponse::TooManyRequests().body("Too many attempts");
                }
                _ => {
                    error!("Login failed for {}: {e}", body.email);
                    return HttpResponse::InternalServerError().body("Login failed");
                }
            }
        }
    };
    HttpResponse::Ok().json(LoginRes {
        user_id: session.user.id,
        email: body.email.clone(),
        access_token: session.access_token,
    })
}
