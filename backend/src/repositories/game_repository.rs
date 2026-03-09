use serde_json::json;
use supabase_rs::SupabaseClient;
use uuid::Uuid;

use crate::errors::custom_errors::RepoError;
use crate::models::game::GameSession;
use crate::models::game::LeaderboardEntry;
use crate::models::game::PlayerStats;

// TODO: Create enum for game codes
pub async fn create_session(
    db: &SupabaseClient,
    user_id: Uuid,
    game_code: String,
) -> Result<Uuid, RepoError> {
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

    let id: Uuid = serde_json::from_str(&handled_result).map_err(|e| {
        log::error!("Invalid UUID returned from DB: {e}");
        RepoError::InsertionError(String::from("Invalid session id"))
    })?;

    Ok(id)
}

pub async fn finalize_session(
    db: &SupabaseClient,
    session_id: Uuid,
    score: f64,
) -> Result<(), RepoError> {
    let session_id = session_id.to_string();
    let db_result = db
        .update(
            "game_sessions",
            &session_id,
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
        .ok_or(RepoError::NotFound(String::from("Session not found")))?;

    serde_json::from_value(row).map_err(|e| RepoError::ExtractionError(e.to_string()))
}

pub async fn get_player_stats(
    db: &SupabaseClient,
    user_id: Uuid,
) -> Result<Vec<PlayerStats>, RepoError> {
    let rows = db
        .select("player_stats_view")
        .eq("user_id", &user_id.to_string())
        .order("game_code", true)
        .execute()
        .await
        .map_err(|e| {
            log::error!("Failed fetching player stats: {e}");
            RepoError::ExtractionError(String::from("Failed fetching player stats"))
        })?;

    rows.into_iter()
        .map(|r| serde_json::from_value(r).map_err(|e| RepoError::ExtractionError(e.to_string())))
        .collect()
}

pub async fn get_recent_sessions(
    db: &SupabaseClient,
    user_id: Uuid,
    game_code: &str,
    limit: usize,
) -> Result<Vec<LeaderboardEntry>, RepoError> {
    let rows = db
        .select("leaderboard_view")
        .eq("user_id", &user_id.to_string())
        .eq("game_code", game_code)
        .order("completed_at", false)
        .limit(limit)
        .execute()
        .await
        .map_err(|e| {
            log::error!("Failed fetching recent sessions: {e}");
            RepoError::ExtractionError(String::from("Failed fetching recent sessions"))
        })?;

    rows.into_iter()
        .map(|r| serde_json::from_value(r).map_err(|e| RepoError::ExtractionError(e.to_string())))
        .collect()
}

pub async fn get_leaderboard(
    db: &SupabaseClient,
    game_code: &str,
) -> Result<Vec<LeaderboardEntry>, RepoError> {
    let rows = db
        .select("leaderboard_view")
        .eq("game_code", game_code)
        .order("score", false)
        .execute()
        .await
        .map_err(|e| {
            log::error!("Failed fetching leaderboard: {e}");
            RepoError::ExtractionError(String::from("Failed fetching leaderboard"))
        })?;

    rows.into_iter()
        .map(|r| serde_json::from_value(r).map_err(|e| RepoError::ExtractionError(e.to_string())))
        .collect()
}
