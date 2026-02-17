use actix_web::HttpResponse;
use actix_web::web;
use serde_json::json;

use crate::errors::custom_errors::RepoError;
use crate::models::app_state::AppState;
use crate::models::assets::CreateAssetGroupReq;
use crate::models::assets::CreateGameAssetReq;
use crate::models::assets::UpdateAssetGroupReq;
use crate::models::assets::UpdateGameAssetReq;
use crate::repositories::asset_repository;

/// Maps repository errors to appropriate HTTP responses with stable error messages
fn map_repo_error(err: RepoError, context: &str) -> HttpResponse {
    match err {
        RepoError::NotFound(_) => {
            HttpResponse::NotFound().json(json!({
                "error": "not_found",
                "message": format!("{} not found", context)
            }))
        }
        RepoError::InsertionError(msg) => {
            // Check if it's a foreign key constraint violation
            if msg.contains("foreign key") || msg.contains("constraint") || msg.contains("fkey") || msg.contains("violates") {
                HttpResponse::Conflict().json(json!({
                    "error": "constraint_violation",
                    "message": format!("Invalid reference in {}", context)
                }))
            } else {
                HttpResponse::UnprocessableEntity().json(json!({
                    "error": "validation_error",
                    "message": format!("Failed to create {}", context)
                }))
            }
        }
        RepoError::UpdateError(msg) => {
            // Check if it's a foreign key constraint violation
            if msg.contains("foreign key") || msg.contains("constraint") || msg.contains("fkey") || msg.contains("violates") {
                HttpResponse::Conflict().json(json!({
                    "error": "constraint_violation",
                    "message": format!("Invalid reference in {}", context)
                }))
            } else {
                HttpResponse::UnprocessableEntity().json(json!({
                    "error": "validation_error",
                    "message": format!("Failed to update {}", context)
                }))
            }
        }
        RepoError::ExtractionError(_) => {
            HttpResponse::InternalServerError().json(json!({
                "error": "internal_error",
                "message": format!("Failed to retrieve {}", context)
            }))
        }
        RepoError::DeletionError(_) => {
            HttpResponse::InternalServerError().json(json!({
                "error": "internal_error",
                "message": format!("Failed to delete {}", context)
            }))
        }
    }
}

pub async fn create_asset_group(
    body: web::Json<CreateAssetGroupReq>,
    state: web::Data<AppState>,
) -> HttpResponse {
    if body.game_code.is_empty() || body.label.is_empty() {
        return HttpResponse::BadRequest().json(json!({
            "error": "validation_error",
            "message": "game_code and label are required"
        }));
    }
    match asset_repository::create_asset_group(&state.sb_client, &body.game_code, &body.label).await
    {
        Ok(id) => HttpResponse::Created().json(json!({"id": id})),
        Err(e) => map_repo_error(e, "asset group"),
    }
}

pub async fn list_asset_groups(state: web::Data<AppState>) -> HttpResponse {
    // TODO: Refine with a join to one shot this (inefficient because O(N + 1))
    let mut groups = match asset_repository::get_asset_groups(&state.sb_client).await {
        Ok(v) => v,
        Err(e) => return map_repo_error(e, "asset groups"),
    };
    for group in groups.iter_mut() {
        match asset_repository::get_game_assets(&state.sb_client, group.id).await {
            Ok(assets) => group.assets = assets,
            Err(e) => return map_repo_error(e, "game assets"),
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
            Err(e) => return map_repo_error(e, "asset group"),
        };

    for group in asset_group.iter_mut() {
        match asset_repository::get_game_assets(&state.sb_client, group.id).await {
            Ok(assets) => group.assets = assets,
            Err(e) => return map_repo_error(e, "game assets"),
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
        return HttpResponse::BadRequest().json(json!({
            "error": "validation_error",
            "message": "game_code and label are required"
        }));
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
        Err(e) => map_repo_error(e, "asset group"),
    }
}

pub async fn delete_asset_group(path: web::Path<i32>, state: web::Data<AppState>) -> HttpResponse {
    match asset_repository::delete_asset_group(&state.sb_client, path.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => map_repo_error(e, "asset group"),
    }
}

pub async fn create_game_asset(
    body: web::Json<CreateGameAssetReq>,
    state: web::Data<AppState>,
) -> HttpResponse {
    match asset_repository::create_game_asset(
        &state.sb_client,
        body.group_id,
        &body.label,
        &body.image_url,
        body.is_correct,
    )
    .await
    {
        Ok(id) => HttpResponse::Created().json(json!({"id": id})),
        Err(e) => map_repo_error(e, "game asset"),
    }
}
pub async fn list_game_assets(path: web::Path<i32>, state: web::Data<AppState>) -> HttpResponse {
    match asset_repository::get_game_assets(&state.sb_client, path.into_inner()).await {
        Ok(assets) => HttpResponse::Ok().json(assets),
        Err(e) => map_repo_error(e, "game assets"),
    }
}

pub async fn update_game_asset(
    path: web::Path<i32>,
    body: web::Json<UpdateGameAssetReq>,
    state: web::Data<AppState>,
) -> HttpResponse {
    match asset_repository::update_game_asset(
        &state.sb_client,
        path.into_inner(),
        &body.label,
        &body.image_url,
        body.is_correct,
    )
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => map_repo_error(e, "game asset"),
    }
}

pub async fn delete_game_asset(path: web::Path<i32>, state: web::Data<AppState>) -> HttpResponse {
    match asset_repository::delete_game_asset(&state.sb_client, path.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => map_repo_error(e, "game asset"),
    }
}
