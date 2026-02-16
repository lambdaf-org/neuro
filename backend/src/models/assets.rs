use serde::Deserialize;
use serde::Serialize;

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
    pub id: i32,
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
pub struct CreateGameAssetReq {
    pub group_id: i32,
    pub label: String,
    pub image_url: String,
    pub is_correct: bool,
}

#[derive(Deserialize)]
pub struct UpdateGameAssetReq {
    pub label: String,
    pub image_url: String,
    pub is_correct: bool,
}
