use actix_web::web;
use serde_json::json;
use uuid::Uuid;

use crate::errors::custom_errors::RepoError;
use crate::models::anticheat::AnticheatVerdict;
use crate::models::app_state::AppState;
use crate::repositories::anticheat_repository;
use crate::repositories::game_repository;

pub async fn apply_verdict(
    state: &web::Data<AppState>,
    user_id: Uuid,
    session_id: Option<Uuid>,
    verdict: &AnticheatVerdict,
) -> Result<(), RepoError> {
    if verdict.flags.is_empty() {
        return Ok(());
    }

    let action = verdict.action.as_str();
    for flag in &verdict.flags {
        if let Err(e) =
            anticheat_repository::log_flag(&state.sb_client, user_id, session_id, action, flag)
                .await
        {
            // Log failures shouldn't block the ban — note and continue.
            log::error!("anticheat log_flag failed: {e}");
        }
    }

    if verdict.is_ban() {
        anticheat_repository::ban_user(&state.sb_client, user_id).await?;
    }

    Ok(())
}

pub async fn invalidate_session_for_ban(
    state: &web::Data<AppState>,
    session_id: Uuid,
    verdict: &AnticheatVerdict,
) {
    let metrics = json!({
        "reason": "anticheat_ban",
        "anticheat": verdict.flags_summary(),
    });

    if let Err(e) =
        game_repository::finalize_session(&state.sb_client, session_id, "invalid", None, metrics, 0)
            .await
    {
        log::error!("failed marking session {session_id} invalid after ban: {e}");
    }
}
