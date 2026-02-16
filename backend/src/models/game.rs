use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct AssetGroupRes {
    pub id: i32,
    pub label: String,
    pub game_code: String,
    #[serde(skip_deserializing)]
    pub assets: Vec<GameAssetRes>,
}

#[derive(Deserialize, Serialize)]
pub struct GameAssetRes {
    pub group_id: i32,
    pub label: String,
    pub image_url: String,
    pub is_correct: bool,
}

#[derive(Deserialize)]
pub struct CreateAssetGroupReq {
    pub game_code: String,
    pub label: String,
}

#[derive(Deserialize)]
pub struct UpdateAssetGroupReq {
    pub game_code: String,
    pub label: String,
}

#[derive(Deserialize)]
pub struct FinalizeSessionReq {
    pub score: f64,
}

#[derive(Deserialize, Serialize)]
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
