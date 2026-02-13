use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize)]
pub struct AssetGroupRes {
    pub id: i32,
    pub label: String,
    pub game_code: String,
    #[serde(skip_deserializing)]
    pub assets: Vec<GameAsset>,
}

#[derive(Deserialize, Serialize)]
pub struct GameAssetRes {
    pub group_id: i32,
    pub label: String,
    pub image_url: String,
    pub is_correct: bool,
}
