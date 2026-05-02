use crate::models::validate::Validate;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
pub struct FinalizeSessionReq {
    #[serde(default)]
    pub trials: Vec<TrialPayload>,
    pub client_ts: Option<String>,
    pub score: Option<f64>,
}

#[derive(Deserialize, Serialize, ToSchema, Clone)]
pub struct TrialPayload {
    pub ms: Option<f64>,
    pub span: Option<i32>,
    pub correct: Option<bool>,
    pub magnitude: Option<f64>,
    pub puzzle_id: Option<i32>,
    pub selected_option_id: Option<i32>,
}

#[derive(Serialize, ToSchema)]
pub struct FinalizeSessionResultRes {
    pub status: String,
    pub metric_value: Option<f64>,
    pub metrics: Value,
    pub scoring_version: i32,
}

impl Validate for FinalizeSessionReq {
    fn validate(&self) -> Result<(), Vec<&'static str>> {
        let mut errors = Vec::new();

        if self.trials.is_empty() && self.score.is_none() {
            errors.push("trials or legacy score is required");
        }

        if self.trials.len() > 1000 {
            errors.push("trials must not exceed 1000 entries");
        }

        if let Some(score) = self.score {
            if score < 0.0 {
                errors.push("score must be non-negative");
            }
        }

        if let Some(client_ts) = &self.client_ts {
            let ts = client_ts.trim();
            if ts.is_empty() {
                errors.push("client_ts must not be empty");
            } else if chrono::DateTime::parse_from_rfc3339(ts).is_err() {
                errors.push("client_ts must be a valid RFC3339 timestamp");
            }
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

#[derive(Serialize, ToSchema)]
pub struct GameEventAck {
    pub id: Uuid,
    pub round: i32,
}

#[derive(Serialize, ToSchema)]
pub struct GameEventWsAck {
    pub message_type: String,
    pub event: GameEventAck,
}

#[derive(Serialize, ToSchema)]
pub struct GameEventWsError {
    pub message_type: String,
    pub error: String,
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
    pub metric_value: Option<f64>,
    pub metrics: Option<Value>,
    pub scoring_version: Option<i32>,
    pub started_at: String,
    // Can be empty since game can be in progress
    pub completed_at: Option<String>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct LeaderboardEntry {
    pub game_code: String,
    pub user_id: Uuid,
    pub username: String,
    pub metric_value: f64,
    pub completed_at: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct PlayerStats {
    pub user_id: Uuid,
    pub username: String,
    pub game_code: String,
    pub latest_metric: f64,
    pub best_metric: f64,
    pub avg_metric: f64,
    pub session_count: i64,
    pub completed_at: Option<String>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct GameMetadata {
    pub game_code: String,
    pub display_name: String,
    pub chc_factor: String,
    pub cognitive_domain: String,
    pub description: String,
    pub scientific_basis: String,
    pub task_summary: String,
    pub metric_name: String,
    pub metric_direction: String,
    pub icon_url: Option<String>,
    pub sort_order: i32,
    pub is_active: bool,
}
