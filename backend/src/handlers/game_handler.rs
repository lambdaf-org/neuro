use actix_web::HttpResponse;
use actix_web::web;
use serde_json::json;
use uuid::Uuid;

use crate::models::app_state::AppState;
use crate::models::game::FinalizeSessionReq;
use crate::models::user::MiddlewareData;
use crate::repositories::game_repository;

pub async fn start_game(
    path: web::Path<String>,
    ext_data: web::ReqData<MiddlewareData>,
    state: web::Data<AppState>,
) -> HttpResponse {
    match game_repository::create_session(&state.sb_client, ext_data.user_id, path.into_inner())
        .await
    {
        Ok(v) => HttpResponse::Ok().body(v),
        Err(e) => HttpResponse::BadRequest().json(json!({"error": e.to_string()})),
    }
}

pub async fn get_game_assets(path: web::Path<String>, state: web::Data<AppState>) -> HttpResponse {
    // TODO: Refine with a join to one shot this (inefficient because O(N + 1))
    // TODO: Do not expose the true answer on game asset fetch since players may cheat
    let mut asset_group =
        match game_repository::get_asset_groups(&state.sb_client, path.into_inner()).await {
            Ok(v) => v,
            Err(e) => return HttpResponse::BadRequest().json(json!({"error": e.to_string()})),
        };

    for group in asset_group.iter_mut() {
        match game_repository::get_game_assets(&state.sb_client, group.id).await {
            Ok(assets) => group.assets = assets,
            Err(e) => return HttpResponse::BadRequest().json(json!({"error": e.to_string()})),
        }
    }

    HttpResponse::Ok().json(asset_group)
}

pub async fn finalize_session(
    path: web::Path<Uuid>,
    body: web::Json<FinalizeSessionReq>,
    ext_data: web::ReqData<MiddlewareData>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let session_id = path.into_inner();

    // Ensure the session belongs to the authenticated user before finalizing
    let session = match game_repository::get_session(&state.sb_client, session_id).await {
        Ok(v) => v,
        Err(e) => return HttpResponse::BadRequest().json(json!({"error": e.to_string()})),
    };

    if session.user_id != ext_data.user_id {
        return HttpResponse::Forbidden().finish();
    }

    match game_repository::finalize_session(&state.sb_client, session_id, body.score).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::BadRequest().json(json!({"error": e.to_string()})),
    }
}

pub async fn get_game_session(
    path: web::Path<Uuid>,
    ext_data: web::ReqData<MiddlewareData>,
    state: web::Data<AppState>,
) -> HttpResponse {
    match game_repository::get_session(&state.sb_client, path.into_inner()).await {
        Ok(v) => {
            if v.user_id == ext_data.user_id {
                HttpResponse::Ok().json(v)
            } else {
                HttpResponse::Forbidden().json(json!({"error": "forbidden"}))
            }
        }
        Err(e) => HttpResponse::BadRequest().json(json!({"error": e.to_string()})),
    }
}
