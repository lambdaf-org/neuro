use serde_json::Value;
use serde_json::json;
use supabase_rs::SupabaseClient;

use crate::errors::custom_errors::RepoError;
use crate::models::game::AssetGroupRes;
use crate::models::game::GameAssetRes;

pub async fn create_asset_group(
    db: &SupabaseClient,
    game_code: &str,
    label: &str,
) -> Result<String, RepoError> {
    db.insert(
        "asset_groups",
        json!({
            "game_code": game_code,
            "label": label,
        }),
    )
    .await
    .map_err(|e| {
        log::error!("Failed inserting asset group: {e}");
        RepoError::InsertionError(String::from("Failed inserting asset group"))
    })
}

pub async fn get_game_assets(
    db: &SupabaseClient,
    group_id: i32,
) -> Result<Vec<GameAssetRes>, RepoError> {
    let rows = db
        .select("game_assets")
        .eq("group_id", &group_id.to_string())
        .execute()
        .await
        .map_err(|e| {
            log::error!("Failed fetching game assets: {e}");
            RepoError::ExtractionError(String::from("Failed fetching game assets"))
        })?;

    serde_json::from_value(Value::Array(rows))
        .map_err(|e| RepoError::ExtractionError(e.to_string()))
}

pub async fn get_asset_group_by_id(
    db: &SupabaseClient,
    id: i32,
) -> Result<AssetGroupRes, RepoError> {
    let rows = db
        .select("asset_groups")
        .eq("id", &id.to_string())
        .execute()
        .await
        .map_err(|e| {
            log::error!("Failed fetching asset group: {e}");
            RepoError::ExtractionError(String::from("Failed fetching asset group"))
        })?;

    let row = rows
        .into_iter()
        .next()
        .ok_or(RepoError::NotFound(String::from("Asset group not found")))?;

    serde_json::from_value(row).map_err(|e| RepoError::ExtractionError(e.to_string()))
}

// TODO: Create enum for game codes
// TODO: Randomize when fetching
pub async fn get_asset_groups_by_code(
    db: &SupabaseClient,
    code: String,
) -> Result<Vec<AssetGroupRes>, RepoError> {
    let rows = db
        .select("asset_groups")
        .eq("game_code", &code)
        .execute()
        .await
        .map_err(|e| {
            log::error!("Failed fetching asset group: {e}");
            RepoError::ExtractionError(String::from("Failed asset group"))
        })?;

    serde_json::from_value(Value::Array(rows))
        .map_err(|e| RepoError::ExtractionError(e.to_string()))
}

pub async fn get_asset_groups(db: &SupabaseClient) -> Result<Vec<AssetGroupRes>, RepoError> {
    let rows = db.select("asset_groups").execute().await.map_err(|e| {
        log::error!("Failed fetching asset group: {e}");
        RepoError::ExtractionError(String::from("Failed asset group"))
    })?;

    serde_json::from_value(Value::Array(rows))
        .map_err(|e| RepoError::ExtractionError(e.to_string()))
}

pub async fn update_asset_group(
    db: &SupabaseClient,
    id: i32,
    game_code: &str,
    label: &str,
) -> Result<(), RepoError> {
    db.update(
        "asset_groups",
        &id.to_string(),
        json!({
            "game_code": game_code,
            "label": label,
        }),
    )
    .await
    .map_err(|e| {
        log::error!("Failed updating asset group: {e}");
        RepoError::UpdateError(String::from("Failed updating asset group"))
    })?;
    Ok(())
}

pub async fn delete_asset_group(db: &SupabaseClient, id: i32) -> Result<(), RepoError> {
    db.delete("asset_groups", &id.to_string())
        .await
        .map_err(|e| {
            log::error!("Failed deleting asset group: {e}");
            RepoError::DeletionError(String::from("Failed deleting asset group"))
        })?;
    Ok(())
}
