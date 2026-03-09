use std::env;

use crate::models::assets::{
    CreateAssetGroupReq, CreateGameAssetReq, UpdateAssetGroupReq, UpdateGameAssetReq,
};
use crate::models::game::{FinalizeSessionReq, GameSession, LeaderboardEntry};
use crate::models::user::{LoginPayload, LoginRes, RegisterPayload};
use utoipa::Modify;
use utoipa::OpenApi;
use utoipa::openapi::security::ApiKey;
use utoipa::openapi::security::ApiKeyValue;
use utoipa::openapi::security::SecurityScheme;
use utoipa_swagger_ui::SwaggerUi;
use crate::handlers::game_handler::__path_start_game;
use crate::handlers::game_handler::__path_finalize_session;
use crate::handlers::game_handler::__path_get_game_session;
use crate::handlers::user_handler::__path_login;
use crate::handlers::user_handler::__path_register;
use crate::handlers::asset_handler::__path_list_game_assets;
use crate::handlers::asset_handler::__path_create_game_asset;
use crate::handlers::asset_handler::__path_delete_game_asset;
use crate::handlers::asset_handler::__path_list_asset_groups;
use crate::handlers::asset_handler::__path_update_game_asset;
use crate::handlers::asset_handler::__path_delete_asset_group;
use crate::handlers::asset_handler::__path_get_asset_groups_by_code;
use crate::handlers::asset_handler::__path_create_asset_group;
use crate::handlers::asset_handler::__path_update_asset_group;
use crate::handlers::game_handler::__path_get_game_leaderboard;
use crate::handlers::game_handler::__path_get_player_stats;
use crate::handlers::game_handler::__path_get_recent_sessions;
use crate::models::game::GameMetadata;
use crate::handlers::game_handler::__path_get_game_metadata;
use crate::handlers::game_handler::__path_create_game_event;
use crate::handlers::game_handler::__path_get_game_events;
use crate::models::game::{CreateGameEventReq, GameEvent};
use self::models::app_state::AppState;
use self::routes::global_routes;
use actix_web::App;
use actix_web::HttpServer;
use actix_web::middleware::Logger;
use actix_web::web;
use dotenv::dotenv;
use env_logger::Env;
use log::info;
use log::warn;
use supabase_auth::models::AuthClient;
use supabase_rs::SupabaseClient;
use crate::models::assets::GameAssetRes;
use crate::models::assets::AssetGroupRes;
use crate::models::game::PlayerStats;

pub mod config;
pub mod errors;
pub mod handlers;
pub mod models;
pub mod repositories;
pub mod routes;
pub mod services;
#[cfg(test)]
pub mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    struct SecuritySchemas;
    impl Modify for SecuritySchemas {
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            let components = openapi.components.as_mut().unwrap();
            let value = ApiKeyValue::with_description("Authorization", "Bearer ey...");
            let scheme = SecurityScheme::ApiKey(ApiKey::Header(value));
            components.add_security_scheme("Authorization", scheme);
        }
    }
    #[derive(OpenApi)]
    #[openapi(
        paths(
            start_game,
            finalize_session,
            get_game_session,
            login,
            register,
            list_game_assets,
            create_game_asset,
            delete_game_asset,
            list_asset_groups,
            update_game_asset,
            delete_asset_group,
            get_asset_groups_by_code,
            create_asset_group,
            update_asset_group,
            get_game_leaderboard,
            get_player_stats,
            get_recent_sessions,
            get_game_metadata,
            create_game_event,
            get_game_events,
        ),
        components(schemas(
            RegisterPayload, 
            LoginPayload, 
            LoginRes,
            FinalizeSessionReq, 
            GameSession, 
            LeaderboardEntry,
            CreateAssetGroupReq,
            UpdateAssetGroupReq,
            CreateGameAssetReq, 
            UpdateGameAssetReq,
            GameAssetRes,
            AssetGroupRes,
            PlayerStats,
            GameMetadata,
            CreateGameEventReq,
            GameEvent,
        )),
        security(("Authorization" = [])),
        modifiers(&SecuritySchemas),
        tags(
            (name = "user", description = "Authentication"),
            (name = "games", description = "Game sessions and leaderboards"),
            (name = "assets", description = "Asset management"),
        )
    )]
    struct ApiDoc;
    let openapi = ApiDoc::openapi();

    dotenv().ok();
    let address = setup_address();
    info!("Running at http://{}:{}", address.0, address.1);

    // init the logger and define default log level
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let app_state = web::Data::new(init_app_state().await);

    HttpServer::new(move || {
        App::new()
            .service(
                SwaggerUi::new("/swagger/{_:.*}").url("/api-doc/openapi.json", openapi.clone()),
            )
            .wrap(Logger::default())
            .app_data(app_state.clone())
            .configure(global_routes::init_admin_scope)
            .configure(global_routes::init_api_scope)
            .configure(global_routes::init_anon_scope)
    })
    .bind(format!("{}:{}", address.0, address.1))?
    .run()
    .await
}

fn setup_address() -> (String, String) {
    let host = env::var("HOST").unwrap_or_else(|_| {
        warn!("Could not find HOST env, defaulting to 127.0.0.1");
        "127.0.0.1".to_string()
    });

    let port = env::var("PORT").unwrap_or_else(|_| {
        warn!("Could not find PORT env, defaulting to 8080");
        "8080".to_string()
    });

    (host, port)
}

async fn init_app_state() -> AppState {
    let sb_client = init_supabase_db_client();

    AppState {
        sb_client,
        auth_client: init_auth_client(),
    }
}

fn init_supabase_db_client() -> SupabaseClient {
    supabase_rs::SupabaseClient::new(
        env::var("SUPABASE_URL").expect("Could not find SUPABASE_URL"),
        env::var("SUPABASE_API_KEY").expect("Could not find SUPABASE_API_KEY"),
    )
    .expect("Failed initializing Supabase client")
}

fn init_auth_client() -> AuthClient {
    let url = env::var("SUPABASE_URL").expect("Undefined env: SUPABASE_URL");
    let api_key = env::var("SUPABASE_API_KEY").expect("Undefined env: SUPABASE_API_KEY");
    let anon_key = env::var("SUPABASE_ANON_KEY").expect("Undefined env: SUPABASE_ANON_KEY");

    AuthClient::new(url, api_key, anon_key)
}
