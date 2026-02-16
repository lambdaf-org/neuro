use crate::config::middleware;
use crate::handlers::game_handler;
use crate::handlers::game_handler::get_game_session;
use crate::handlers::user_handler::login;
use crate::handlers::user_handler::register;
use crate::models::user::MiddlewareData;
use actix_web::HttpMessage;
use actix_web::body::MessageBody;
use actix_web::dev::ServiceRequest;
use actix_web::dev::ServiceResponse;
use actix_web::error::ErrorUnauthorized;
use actix_web::middleware::Next;
use actix_web::middleware::from_fn;
use actix_web::web;
use game_handler::finalize_session;
use game_handler::get_game_assets;
use game_handler::start_game;

use log::error;

pub fn init_admin_scope(cfg: &mut web::ServiceConfig) {
    if std::env::var("ADMIN_API_ENABLED").unwrap_or_default() == "true" {
        cfg.service(web::scope("/admin").wrap(from_fn(auth_filter)));
    }
}

// Routes starting with "/api", which also are protected by the middleware
pub fn init_api_scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .wrap(from_fn(auth_filter))
            .route("/game/{code}", web::post().to(start_game))
            .route("/game/{code}", web::get().to(get_game_assets))
            .route("/game/session/{id}", web::patch().to(finalize_session))
            .route("/game/session/{id}", web::get().to(get_game_session)),
    );
}

// Unprotected routes
pub fn init_anon_scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login)),
    );
}

async fn auth_filter(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let maybe_token = req
        .headers()
        .get("Authorization")
        .and_then(|x| x.to_str().ok().map(|s| s.to_string()))
        .or_else(|| {
            req.query_string()
                .split('&')
                .find_map(|p| p.strip_prefix("token="))
                .map(|s| format!("Bearer {}", s))
        });

    match middleware::validate_jwt(maybe_token).await {
        Ok(user_id) => {
            // Insert the user_id from the JWT-Token into ReqData
            req.extensions_mut().insert(MiddlewareData { user_id });
            next.call(req).await
        }
        Err(e) => {
            error!("Authorization failed: {e}");
            Err(ErrorUnauthorized("Authorization failed."))
        }
    }
}
