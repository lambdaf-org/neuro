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
