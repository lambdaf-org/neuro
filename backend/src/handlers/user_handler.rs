use crate::models::app_state::AppState;
use crate::models::user::LoginPayload;
use crate::models::user::LoginRes;
use crate::models::user::RegisterPayload;
use actix_web::HttpResponse;
use actix_web::web;
use log::error;

pub async fn register(
    body: web::Json<RegisterPayload>,
    state: web::Data<AppState>,
) -> HttpResponse {
    match state
        .auth_client
        .sign_up_with_email_and_password(&body.email, &body.password, None)
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            error!("User tried registering but it failed: {e}");
            HttpResponse::InternalServerError().body("Failed registering, try again later.")
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
            error!("Login failed for email {}: {e}", body.email);
            return HttpResponse::Unauthorized().body("Invalid credentials");
        }
    };

    HttpResponse::Ok().json(LoginRes {
        user_id: session.user.id,
        email: body.email.clone(),
        access_token: session.access_token,
    })
}
