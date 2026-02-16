use actix_web::HttpResponse;
use actix_web::web;
use serde_json::json;

use crate::models::app_state::AppState;
use crate::models::game::CreateAssetGroupReq;
use crate::models::game::UpdateAssetGroupReq;
use crate::repositories::asset_repository;

pub async fn create_asset_group(
    body: web::Json<CreateAssetGroupReq>,
    state: web::Data<AppState>,
) -> HttpResponse {
    if body.game_code.is_empty() || body.label.is_empty() {
        return HttpResponse::BadRequest().json(json!({"error": "game_code and label required"}));
    }
    match asset_repository::create_asset_group(&state.sb_client, &body.game_code, &body.label).await
    {
        Ok(id) => HttpResponse::Created().json(json!({"id": id})),
        Err(e) => HttpResponse::BadRequest().json(json!({"error": e.to_string()})),
    }
}

pub async fn get_asset_group(path: web::Path<i32>, state: web::Data<AppState>) -> HttpResponse {
    match asset_repository::get_asset_group(&state.sb_client, path.into_inner()).await {
        Ok(group) => HttpResponse::Ok().json(group),
        Err(e) => HttpResponse::NotFound().json(json!({"error": e.to_string()})),
    }
}

pub async fn list_asset_groups(
    path: web::Path<String>,
    state: web::Data<AppState>,
) -> HttpResponse {
    match crate::repositories::game_repository::get_asset_groups(
        &state.sb_client,
        path.into_inner(),
    )
    .await
    {
        Ok(groups) => HttpResponse::Ok().json(groups),
        Err(e) => HttpResponse::BadRequest().json(json!({"error": e.to_string()})),
    }
}

pub async fn update_asset_group(
    path: web::Path<i32>,
    body: web::Json<UpdateAssetGroupReq>,
    state: web::Data<AppState>,
) -> HttpResponse {
    if body.game_code.is_empty() || body.label.is_empty() {
        return HttpResponse::BadRequest().json(json!({"error": "game_code and label required"}));
    }
    match asset_repository::update_asset_group(
        &state.sb_client,
        path.into_inner(),
        &body.game_code,
        &body.label,
    )
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::BadRequest().json(json!({"error": e.to_string()})),
    }
}

pub async fn delete_asset_group(path: web::Path<i32>, state: web::Data<AppState>) -> HttpResponse {
    match asset_repository::delete_asset_group(&state.sb_client, path.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::BadRequest().json(json!({"error": e.to_string()})),
    }
}
