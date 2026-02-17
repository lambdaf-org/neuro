use crate::errors::auth_errors::AuthError;
use crate::models::app_state::AppState;
use crate::models::user::ErrorResponse;
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
                    HttpResponse::Conflict().json(ErrorResponse {
                        error: "Email already registered".to_string(),
                    })
                }
                AuthError::UsernameAlreadyTaken => {
                    HttpResponse::Conflict().json(ErrorResponse {
                        error: "Username already taken".to_string(),
                    })
                }
                AuthError::AlreadyExists => {
                    HttpResponse::Conflict().json(ErrorResponse {
                        error: "Already exists".to_string(),
                    })
                }
                AuthError::WeakPassword => {
                    HttpResponse::BadRequest().json(ErrorResponse {
                        error: "Password too weak".to_string(),
                    })
                }
                AuthError::RateLimited => {
                    HttpResponse::TooManyRequests().json(ErrorResponse {
                        error: "Too many attempts".to_string(),
                    })
                }
                _ => {
                    error!("Registration failed: {e}");
                    HttpResponse::InternalServerError().json(ErrorResponse {
                        error: "Registration failed".to_string(),
                    })
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
                    return HttpResponse::Forbidden().json(ErrorResponse {
                        error: "Email not confirmed".to_string(),
                    });
                }
                AuthError::InvalidCredentials => {
                    return HttpResponse::Unauthorized().json(ErrorResponse {
                        error: "Invalid credentials".to_string(),
                    });
                }
                AuthError::RateLimited => {
                    return HttpResponse::TooManyRequests().json(ErrorResponse {
                        error: "Too many attempts".to_string(),
                    });
                }
                _ => {
                    error!("Login failed: {e}");
                    return HttpResponse::InternalServerError().json(ErrorResponse {
                        error: "Login failed".to_string(),
                    });
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
