use serde_json::json;
use supabase_rs::SupabaseClient;

use crate::errors::custom_errors::RepoError;
use crate::models::game::AssetGroupRes;

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

pub async fn get_asset_group(db: &SupabaseClient, id: i32) -> Result<AssetGroupRes, RepoError> {
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

pub async fn update_asset_group(
    db: &SupabaseClient,
    id: i32,
    game_code: &str,
    label: &str,
) -> Result<(), RepoError> {
    db.update(
        &id.to_string(),
        "asset_groups",
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
