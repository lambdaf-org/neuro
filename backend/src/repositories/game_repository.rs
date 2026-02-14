use serde_json::Value;
use serde_json::json;
use supabase_rs::SupabaseClient;
use uuid::Uuid;

use crate::errors::custom_errors::RepoError;
use crate::models::game::AssetGroupRes;
use crate::models::game::GameAssetRes;
use crate::models::game::GameSession;

// TODO: Create enum for game codes
pub async fn create_session(
    db: &SupabaseClient,
    user_id: Uuid,
    game_code: String,
) -> Result<String, RepoError> {
    let db_result = db
        .insert(
            "game_sessions",
            json!({
                    "user_id": user_id,
                    "game_code": game_code,
            }),
        )
        .await;

    let handled_result = db_result.map_err(|e| {
        log::error!("Failed inserting new game session: {e}");
        RepoError::InsertionError(String::from("Failed inserting game"))
    })?;

    Ok(handled_result)
}

// TODO: Create enum for game codes
pub async fn get_asset_groups(
    db: &SupabaseClient,
    game_code: String,
) -> Result<Vec<AssetGroupRes>, RepoError> {
    let rows = db
        .select("asset_groups")
        .eq("game_code", &game_code)
        .execute()
        .await
        .map_err(|e| {
            log::error!("Failed fetching asset group: {e}");
            RepoError::ExtractionError(String::from("Failed asset group"))
        })?;

    serde_json::from_value(Value::Array(rows))
        .map_err(|e| RepoError::ExtractionError(e.to_string()))
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

pub async fn finalize_session(
    db: &SupabaseClient,
    session_id: Uuid,
    score: f64,
) -> Result<(), RepoError> {
    let db_result = db
        .update(
            "game_sessions",
            session_id.to_string().as_str(),
            json!({
                "score": score,
                // TODO: Make enumeration out of it
                "status": "completed",
                "completed_at": chrono::Utc::now().to_rfc3339(),
            }),
        )
        .await;

    db_result.map_err(|e| {
        log::error!("Failed updating game session: {e}");
        RepoError::UpdateError(String::from("Failed updating game session"))
    })?;

    Ok(())
}

pub async fn get_session(db: &SupabaseClient, session_id: Uuid) -> Result<GameSession, RepoError> {
    let rows = db
        .select("game_sessions")
        .eq("id", &session_id.to_string())
        .execute()
        .await
        .map_err(|e| {
            log::error!("Failed fetching game session: {e}");
            RepoError::ExtractionError(String::from("Failed fetching game session"))
        })?;

    let row = rows
        .into_iter()
        .next()
        .ok_or(RepoError::NotFound(String::from(
            "Session not found",
        )))?;

    serde_json::from_value(row).map_err(|e| RepoError::ExtractionError(e.to_string()))
}
