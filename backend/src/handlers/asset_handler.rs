use actix_web::HttpResponse;
use actix_web::web;
use serde_json::json;

use crate::models::app_state::AppState;
use crate::models::assets::PublicAssetGroupRes;
use crate::models::assets::is_playable_fluid_group;
use crate::models::assets::is_playable_mental_rotation_group;
use crate::models::assets::{
    CreateAssetGroupReq, CreateGameAssetReq, UpdateAssetGroupReq, UpdateGameAssetReq,
};
use crate::models::validate::Validate;
use crate::repositories::asset_repository;

#[utoipa::path(
    post,
    path = "/admin/asset-groups",
    request_body = CreateAssetGroupReq,
    responses(
        (status = 201, description = "Asset group created", body = Object),
        (status = 400, description = "Validation error", body = Object),
        (status = 409, description = "Constraint violation", body = Object),
        (status = 422, description = "Operation failed", body = Object),
        (status = 500, description = "Internal error", body = Object),
    ),
    tag = "assets", 
    security(("Authorization" = []))
)]
pub async fn create_asset_group(
    body: web::Json<CreateAssetGroupReq>,
    state: web::Data<AppState>,
) -> HttpResponse {
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(json!({"errors": errors}));
    }
    match asset_repository::create_asset_group(&state.sb_client, &body.game_code, &body.label).await
    {
        Ok(id) => HttpResponse::Created().json(json!({"id": id})),
        Err(e) => e.to_response(),
    }
}

#[utoipa::path(
    get,
    path = "/admin/asset-groups",
    responses(
        (status = 200, description = "All asset groups with assets", body = Vec<AssetGroupRes>),
        (status = 500, description = "Internal error", body = Object),
    ),
    tag = "assets", 
    security(("Authorization" = []))
)]
pub async fn list_asset_groups(state: web::Data<AppState>) -> HttpResponse {
    // TODO: Refine with a join to one shot this (inefficient because O(N + 1))
    let mut groups = match asset_repository::get_asset_groups(&state.sb_client).await {
        Ok(v) => v,
        Err(e) => return e.to_response(),
    };
    for group in groups.iter_mut() {
        match asset_repository::get_game_assets(&state.sb_client, group.id).await {
            Ok(assets) => group.assets = assets,
            Err(e) => return e.to_response(),
        }
    }
    HttpResponse::Ok().json(groups)
}

// TODO: Randomize when fetching
#[utoipa::path(
    get,
    path = "/api/asset-groups/{code}",
    params(
        ("code" = String, Path, description = "Game code"),
    ),
    responses(
        (status = 200, description = "Asset groups for game code", body = Vec<PublicAssetGroupRes>),
        (status = 500, description = "Internal error", body = Object),
    ),
    tag = "assets", 
    security(("Authorization" = []))
)]
pub async fn get_asset_groups_by_code(
    code: web::Path<String>,
    state: web::Data<AppState>,
) -> HttpResponse {
    // TODO: Refine with a join to one shot this (inefficient because O(N + 1))
    let game_code = code.into_inner();
    let mut asset_group =
        match asset_repository::get_asset_groups_by_code(&state.sb_client, game_code.clone()).await
        {
            Ok(v) => v,
            Err(e) => return e.to_response(),
        };

    for group in asset_group.iter_mut() {
        match asset_repository::get_game_assets(&state.sb_client, group.id).await {
            Ok(assets) => group.assets = assets,
            Err(e) => return e.to_response(),
        }
    }

    let public_groups: Vec<PublicAssetGroupRes> = asset_group
        .iter()
        .filter(|group| match game_code.as_str() {
            "gf" => is_playable_fluid_group(group),
            "gv" => is_playable_mental_rotation_group(group),
            _ => true,
        })
        .map(PublicAssetGroupRes::from)
        .collect();

    HttpResponse::Ok().json(public_groups)
}

#[utoipa::path(
    put,
    path = "/admin/asset-groups/{id}",
    params(
        ("id" = i32, Path, description = "id"),
    ),
    request_body = UpdateAssetGroupReq,
    responses(
        (status = 200, description = "Asset group updated"),
        (status = 400, description = "Validation error", body = Object),
        (status = 422, description = "Operation failed", body = Object),
        (status = 500, description = "Internal error", body = Object),
    ),
    tag = "assets", 
    security(("Authorization" = []))
)]
pub async fn update_asset_group(
    id: web::Path<i32>,
    body: web::Json<UpdateAssetGroupReq>,
    state: web::Data<AppState>,
) -> HttpResponse {
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(json!({"errors": errors}));
    }
    match asset_repository::update_asset_group(
        &state.sb_client,
        id.into_inner(),
        &body.game_code,
        &body.label,
    )
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => e.to_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/admin/asset-groups/{id}",
    params(
        ("id" = i32, Path, description = "id"),
    ),
    responses(
        (status = 204, description = "Asset group deleted"),
        (status = 500, description = "Internal error", body = Object),
    ),
    tag = "assets", 
    security(("Authorization" = []))
)]
pub async fn delete_asset_group(path: web::Path<i32>, state: web::Data<AppState>) -> HttpResponse {
    match asset_repository::delete_asset_group(&state.sb_client, path.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => e.to_response(),
    }
}

#[utoipa::path(
    post,
    path = "/admin/game-assets",
    request_body = CreateGameAssetReq,
    responses(
        (status = 201, description = "Game asset created", body = Object),
        (status = 400, description = "Validation error", body = Object),
        (status = 409, description = "Constraint violation", body = Object),
        (status = 422, description = "Operation failed", body = Object),
        (status = 500, description = "Internal error", body = Object),
    ),
    tag = "assets", 
    security(("Authorization" = []))
)]

pub async fn create_game_asset(
    body: web::Json<CreateGameAssetReq>,
    state: web::Data<AppState>,
) -> HttpResponse {
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(json!({"errors": errors}));
    }
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
        Err(e) => e.to_response(),
    }
}

#[utoipa::path(
    get,
    path = "/admin/game-assets/{group_id}",
    params(
        ("group_id" = i32, Path, description = "group_id"),
    ),
    responses(
        (status = 200, description = "Game assets for group", body = Vec<GameAssetRes>),
        (status = 500, description = "Internal error", body = Object),
    ),
    tag = "assets", 
    security(("Authorization" = []))
)]
pub async fn list_game_assets(path: web::Path<i32>, state: web::Data<AppState>) -> HttpResponse {
    match asset_repository::get_game_assets(&state.sb_client, path.into_inner()).await {
        Ok(assets) => HttpResponse::Ok().json(assets),
        Err(e) => e.to_response(),
    }
}

#[utoipa::path(
    put,
    path = "/admin/game-assets/{id}",
    params(
        ("id" = i32, Path, description = "id"),
    ),
    request_body = UpdateGameAssetReq,
    responses(
        (status = 200, description = "Game asset updated"),
        (status = 400, description = "Validation error", body = Object),
        (status = 422, description = "Operation failed", body = Object),
        (status = 500, description = "Internal error", body = Object),
    ),
    tag = "assets", 
    security(("Authorization" = []))
)]
pub async fn update_game_asset(
    id: web::Path<i32>,
    body: web::Json<UpdateGameAssetReq>,
    state: web::Data<AppState>,
) -> HttpResponse {
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(json!({"errors": errors}));
    }
    match asset_repository::update_game_asset(
        &state.sb_client,
        id.into_inner(),
        &body.label,
        &body.image_url,
        body.is_correct,
    )
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => e.to_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/admin/game-assets/{id}",
    params(
        ("id" = i32, Path, description = "id"),
    ),
    responses(
        (status = 204, description = "Game asset deleted"),
        (status = 500, description = "Internal error", body = Object),
    ),
    tag = "assets", 
    security(("Authorization" = []))
)]
pub async fn delete_game_asset(path: web::Path<i32>, state: web::Data<AppState>) -> HttpResponse {
    match asset_repository::delete_game_asset(&state.sb_client, path.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => e.to_response(),
    }
}
