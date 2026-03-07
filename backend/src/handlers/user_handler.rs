use crate::errors::custom_errors::AuthError;
use crate::models::app_state::AppState;
use crate::models::user::LoginPayload;
use crate::models::user::LoginRes;
use crate::models::user::RegisterPayload;
use crate::models::validate::Validate;
use actix_web::HttpResponse;
use actix_web::web;
use log::error;
use serde_json::json;
use supabase_auth::models::SignUpWithPasswordOptions;

#[utoipa::path(
    post,
    path = "/register",
    request_body = RegisterPayload,
    responses(
        (status = 200, description = "Registration successful"),
        (status = 400, description = "Validation error or weak password", body = Object),
        (status = 409, description = "Email or username already exists", body = Object),
        (status = 429, description = "Rate limited", body = Object),
        (status = 500, description = "Registration failed", body = Object),
    ),
    tag = "user",
    security(("Authorization" = []))
)]
pub async fn register(
    body: web::Json<RegisterPayload>,
    state: web::Data<AppState>,
) -> HttpResponse {
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(json!({"errors": errors}));
    }
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
                    HttpResponse::Conflict().json(json!({"error": "Email already registered"}))
                }
                AuthError::UsernameAlreadyTaken => {
                    HttpResponse::Conflict().json(json!({"error": "Username already taken"}))
                }
                AuthError::AlreadyExists => {
                    HttpResponse::Conflict().json(json!({"error": "Already exists"}))
                }
                AuthError::WeakPassword => {
                    HttpResponse::BadRequest().json(json!({"error": "Password too weak"}))
                }
                AuthError::RateLimited => {
                    HttpResponse::TooManyRequests().json(json!({"error": "Too many attempts"}))
                }
                _ => {
                    error!("Registration failed: {e}");
                    HttpResponse::InternalServerError()
                        .json(json!({"error": "Registration failed"}))
                }
            }
        }
    }
}

#[utoipa::path(
    post,
    path = "/login",
    request_body = LoginPayload,
    responses(
        (status = 200, description = "Login successful", body = LoginRes),
        (status = 400, description = "Validation error", body = Object),
        (status = 401, description = "Invalid credentials", body = Object),
        (status = 403, description = "Email not confirmed", body = Object),
        (status = 429, description = "Rate limited", body = Object),
        (status = 500, description = "Login failed", body = Object),
    ),
    tag = "user",
    security(("Authorization" = []))
)]
pub async fn login(body: web::Json<LoginPayload>, state: web::Data<AppState>) -> HttpResponse {
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(json!({"errors": errors}));
    }

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
                    return HttpResponse::Forbidden().json(json!({"error": "Email not confirmed"}));
                }
                AuthError::InvalidCredentials => {
                    return HttpResponse::Unauthorized()
                        .json(json!({"error": "Invalid credentials"}));
                }
                AuthError::RateLimited => {
                    return HttpResponse::TooManyRequests()
                        .json(json!({"error": "Too many attempts"}));
                }
                _ => {
                    error!("Login failed: {e}");
                    return HttpResponse::InternalServerError()
                        .json(json!({"error": "Login failed"}));
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
