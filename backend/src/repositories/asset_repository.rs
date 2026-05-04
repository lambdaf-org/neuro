use serde_json::Value;
use serde_json::json;
use supabase_rs::SupabaseClient;

use crate::errors::custom_errors::RepoError;
use crate::models::assets::AssetGroupRes;
use crate::models::assets::GameAssetRes;

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

pub async fn get_game_assets_by_group_ids(
    db: &SupabaseClient,
    group_ids: &[i32],
) -> Result<Vec<GameAssetRes>, RepoError> {
    if group_ids.is_empty() {
        return Ok(Vec::new());
    }

    let rows = db
        .select("game_assets")
        .in_("group_id", group_ids)
        .execute()
        .await
        .map_err(|e| {
            log::error!("Failed fetching game assets: {e}");
            RepoError::ExtractionError(String::from("Failed fetching game assets"))
        })?;

    serde_json::from_value(Value::Array(rows))
        .map_err(|e| RepoError::ExtractionError(e.to_string()))
}

// TODO: This function is currently unused. Consider exposing it via a GET endpoint at /admin/asset-groups/{id}
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

pub async fn create_game_asset(
    db: &SupabaseClient,
    group_id: i32,
    label: &str,
    image_url: &str,
    is_correct: bool,
) -> Result<String, RepoError> {
    db.insert(
        "game_assets",
        json!({
            "group_id": group_id,
            "label": label,
            "image_url": image_url,
            "is_correct": is_correct,
        }),
    )
    .await
    .map_err(|e| {
        let err_msg = e.to_string();
        log::error!("Failed inserting game asset: {err_msg}");
        // Check for foreign key constraint violations
        if err_msg.contains("foreign key")
            || err_msg.contains("violates foreign")
            || err_msg.contains("fkey")
        {
            RepoError::ConstraintViolation(String::from("Foreign key constraint violation"))
        } else {
            RepoError::InsertionError(String::from("Failed inserting game asset"))
        }
    })
}

pub async fn update_game_asset(
    db: &SupabaseClient,
    id: i32,
    label: &str,
    image_url: &str,
    is_correct: bool,
) -> Result<(), RepoError> {
    db.update(
        "game_assets",
        &id.to_string(),
        json!({
            "label": label,
            "image_url": image_url,
            "is_correct": is_correct,
        }),
    )
    .await
    .map_err(|e| {
        log::error!("Failed updating game asset: {e}");
        RepoError::UpdateError(String::from("Failed updating game asset"))
    })?;
    Ok(())
}

pub async fn delete_game_asset(db: &SupabaseClient, id: i32) -> Result<(), RepoError> {
    db.delete("game_assets", &id.to_string())
        .await
        .map_err(|e| {
            log::error!("Failed deleting game asset: {e}");
            RepoError::DeletionError(String::from("Failed deleting game asset"))
        })?;
    Ok(())
}
