use serde_json::json;
use supabase_rs::SupabaseClient;
use uuid::Uuid;

use crate::errors::custom_errors::RepoError;
use crate::models::anticheat::AnticheatFlag;

pub async fn log_flag(
    db: &SupabaseClient,
    user_id: Uuid,
    session_id: Option<Uuid>,
    action: &str,
    flag: &AnticheatFlag,
) -> Result<(), RepoError> {
    let reason = format!("{}: {}", flag.code, flag.reason);
    db.insert(
        "anticheat_log",
        json!({
            "user_id": user_id,
            "session_id": session_id,
            "action": action,
            "reason": reason,
        }),
    )
    .await
    .map_err(|e| {
        log::error!("Failed inserting anticheat log: {e}");
        RepoError::InsertionError(String::from("Failed inserting anticheat log"))
    })?;

    Ok(())
}

pub async fn ban_user(db: &SupabaseClient, user_id: Uuid) -> Result<(), RepoError> {
    db.update(
        "profiles",
        &user_id.to_string(),
        json!({ "is_banned": true }),
    )
    .await
    .map_err(|e| {
        log::error!("Failed banning user {user_id}: {e}");
        RepoError::UpdateError(String::from("Failed updating profile"))
    })?;

    Ok(())
}

pub async fn is_user_banned(db: &SupabaseClient, user_id: Uuid) -> Result<bool, RepoError> {
    let rows = db
        .select("profiles")
        .eq("id", &user_id.to_string())
        .execute()
        .await
        .map_err(|e| {
            log::error!("Failed fetching profile for ban check: {e}");
            RepoError::ExtractionError(String::from("Failed fetching profile"))
        })?;

    let Some(row) = rows.into_iter().next() else {
        return Ok(false);
    };

    Ok(row
        .get("is_banned")
        .and_then(|v| v.as_bool())
        .unwrap_or(false))
}
