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

pub fn is_fluid_matrix_asset(asset: &GameAssetRes) -> bool {
    asset.label.trim().eq_ignore_ascii_case("matrix")
}

pub fn is_mental_rotation_reference_asset(asset: &GameAssetRes) -> bool {
    asset.label.trim().eq_ignore_ascii_case("reference")
}

fn playable_options_with_prompt(
    assets: &[GameAssetRes],
    is_prompt_asset: fn(&GameAssetRes) -> bool,
) -> Option<Vec<&GameAssetRes>> {
    let has_prompt = assets.iter().any(is_prompt_asset);
    let options: Vec<_> = assets
        .iter()
        .filter(|asset| !is_prompt_asset(asset))
        .collect();
    let correct_options = options.iter().filter(|asset| asset.is_correct).count();

    if has_prompt && options.len() == 4 && correct_options == 1 {
        Some(options)
    } else {
        None
    }
}

pub fn playable_fluid_options(assets: &[GameAssetRes]) -> Option<Vec<&GameAssetRes>> {
    playable_options_with_prompt(assets, is_fluid_matrix_asset)
}

pub fn is_playable_fluid_group(group: &AssetGroupRes) -> bool {
    playable_fluid_options(&group.assets).is_some()
}

pub fn playable_mental_rotation_options(assets: &[GameAssetRes]) -> Option<Vec<&GameAssetRes>> {
    playable_options_with_prompt(assets, is_mental_rotation_reference_asset)
}

pub fn is_playable_mental_rotation_group(group: &AssetGroupRes) -> bool {
    playable_mental_rotation_options(&group.assets).is_some()
}

#[derive(Serialize, ToSchema)]
pub struct PublicAssetGroupRes {
    pub id: i32,
    pub label: String,
    pub game_code: String,
    pub assets: Vec<PublicGameAssetRes>,
}

#[derive(Serialize, ToSchema)]
pub struct PublicGameAssetRes {
    pub id: i32,
    pub group_id: i32,
    pub label: String,
    pub image_url: String,
}

impl From<&GameAssetRes> for PublicGameAssetRes {
    fn from(asset: &GameAssetRes) -> Self {
        Self {
            id: asset.id,
            group_id: asset.group_id,
            label: asset.label.clone(),
            image_url: asset.image_url.clone(),
        }
    }
}

impl From<&AssetGroupRes> for PublicAssetGroupRes {
    fn from(group: &AssetGroupRes) -> Self {
        Self {
            id: group.id,
            label: group.label.clone(),
            game_code: group.game_code.clone(),
            assets: group.assets.iter().map(PublicGameAssetRes::from).collect(),
        }
    }
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
