use super::validate::Validate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct AssetGroupRes {
    pub id: i32,
    pub label: String,
    pub game_code: String,
    #[serde(skip_deserializing)]
    pub assets: Vec<GameAssetRes>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct GameAssetRes {
    pub id: i32,
    pub group_id: i32,
    pub label: String,
    pub image_url: String,
    pub is_correct: bool,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateAssetGroupReq {
    pub game_code: String,
    pub label: String,
}
impl Validate for CreateAssetGroupReq {
    fn validate(&self) -> Result<(), Vec<&'static str>> {
        let mut errors = Vec::new();
        if self.game_code.trim().is_empty() {
            errors.push("game_code is required");
        }
        if self.label.trim().is_empty() {
            errors.push("label is required");
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateAssetGroupReq {
    pub game_code: String,
    pub label: String,
}
impl Validate for UpdateAssetGroupReq {
    fn validate(&self) -> Result<(), Vec<&'static str>> {
        let mut errors = Vec::new();
        if self.game_code.trim().is_empty() {
            errors.push("game_code is required");
        }
        if self.label.trim().is_empty() {
            errors.push("label is required");
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[derive(Deserialize, ToSchema)]
pub struct CreateGameAssetReq {
    pub group_id: i32,
    pub label: String,
    pub image_url: String,
    pub is_correct: bool,
}
impl Validate for CreateGameAssetReq {
    fn validate(&self) -> Result<(), Vec<&'static str>> {
        let mut errors = Vec::new();
        if self.label.trim().is_empty() {
            errors.push("label is required");
        }
        if self.image_url.trim().is_empty() {
            errors.push("image_url is required");
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateGameAssetReq {
    pub label: String,
    pub image_url: String,
    pub is_correct: bool,
}

impl Validate for UpdateGameAssetReq {
    fn validate(&self) -> Result<(), Vec<&'static str>> {
        let mut errors = Vec::new();
        if self.label.trim().is_empty() {
            errors.push("label is required");
        }
        if self.image_url.trim().is_empty() {
            errors.push("image_url is required");
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
