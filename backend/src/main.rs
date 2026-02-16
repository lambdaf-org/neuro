use std::env;

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

pub mod config;
pub mod errors;
pub mod handlers;
pub mod models;
pub mod repositories;
pub mod routes;
pub mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let address = setup_address();
    info!("Running at http://{}:{}", address.0, address.1);

    // init the logger and define default log level
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let app_state = web::Data::new(init_app_state().await);

    HttpServer::new(move || {
        App::new()
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
