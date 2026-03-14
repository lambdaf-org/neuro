use crate::models::validate::Validate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
pub struct FinalizeSessionReq {
    pub score: f64,
}
impl Validate for FinalizeSessionReq {
    fn validate(&self) -> Result<(), Vec<&'static str>> {
        let mut errors = Vec::new();
        if self.score < 0.0 {
            errors.push("score must be non-negative");
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[derive(Deserialize, ToSchema)]
pub struct CreateGameEventReq {
    pub round: i32,
    pub event_value: f64,
    pub client_ts: String,
}

impl Validate for CreateGameEventReq {
    fn validate(&self) -> Result<(), Vec<&'static str>> {
        let mut errors = Vec::new();
        if self.round < 1 {
            errors.push("round must be >= 1");
        }
        let ts = self.client_ts.trim();
        if ts.is_empty() {
            errors.push("client_ts is required");
        } else if chrono::DateTime::parse_from_rfc3339(ts).is_err() {
            errors.push("client_ts must be a valid RFC3339 timestamp");
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct GameEvent {
    pub id: Uuid,
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub round: i32,
    pub event_value: f64,
    pub client_ts: String,
    pub created_at: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct GameSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub game_code: String,
    pub status: String,
    // Can be empty since game can be in progress
    pub score: Option<f64>,
    pub started_at: String,
    // Can be empty since game can be in progress
    pub completed_at: Option<String>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct LeaderboardEntry {
    pub user_id: Uuid,
    pub username: String,
    pub score: f64,
    pub completed_at: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct PlayerStats {
    pub user_id: Uuid,
    pub username: String,
    pub game_code: String,
    pub best_score: f64,
    pub avg_score: f64,
    pub session_count: i64,
    pub completed_at: Option<String>,
}
