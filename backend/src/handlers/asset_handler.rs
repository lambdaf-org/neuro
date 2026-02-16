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

pub async fn list_asset_groups(state: web::Data<AppState>) -> HttpResponse {
    // TODO: Refine with a join to one shot this (inefficient because O(N + 1))
    let mut groups = match asset_repository::get_asset_groups(&state.sb_client).await {
        Ok(v) => v,
        Err(e) => return HttpResponse::BadRequest().json(json!({"error": e.to_string()})),
    };
    for group in groups.iter_mut() {
        match asset_repository::get_game_assets(&state.sb_client, group.id).await {
            Ok(assets) => group.assets = assets,
            Err(e) => return HttpResponse::BadRequest().json(json!({"error": e.to_string()})),
        }
    }
    HttpResponse::Ok().json(groups)
}

// TODO: Randomize when fetching
pub async fn get_asset_groups_by_code(
    path: web::Path<String>,
    state: web::Data<AppState>,
) -> HttpResponse {
    // TODO: Refine with a join to one shot this (inefficient because O(N + 1))
    // TODO: Do not expose the true answer on game asset fetch since players may cheat
    let mut asset_group =
        match asset_repository::get_asset_groups_by_code(&state.sb_client, path.into_inner()).await
        {
            Ok(v) => v,
            Err(e) => return HttpResponse::BadRequest().json(json!({"error": e.to_string()})),
        };

    for group in asset_group.iter_mut() {
        match asset_repository::get_game_assets(&state.sb_client, group.id).await {
            Ok(assets) => group.assets = assets,
            Err(e) => return HttpResponse::BadRequest().json(json!({"error": e.to_string()})),
        }
    }

    HttpResponse::Ok().json(asset_group)
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
