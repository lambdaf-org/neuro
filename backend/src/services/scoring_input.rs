use std::collections::HashMap;

use supabase_rs::SupabaseClient;

use crate::errors::custom_errors::RepoError;
use crate::models::assets::GameAssetRes;
use crate::models::assets::playable_fluid_options;
use crate::models::assets::playable_mental_rotation_options;
use crate::models::game::TrialPayload;
use crate::repositories::asset_repository;

#[derive(Debug)]
pub enum TrialResolutionError {
    BadRequest(&'static str),
    Repository(RepoError),
}

impl From<RepoError> for TrialResolutionError {
    fn from(error: RepoError) -> Self {
        Self::Repository(error)
    }
}

pub async fn build_pattern_logic_trials(
    db: &SupabaseClient,
    trials: &[TrialPayload],
) -> Result<Vec<TrialPayload>, TrialResolutionError> {
    build_asset_choice_trials(
        db,
        "gf",
        trials,
        playable_fluid_options,
        "no playable Pattern Logic assets",
    )
    .await
}

pub async fn build_mental_rotation_trials(
    db: &SupabaseClient,
    trials: &[TrialPayload],
) -> Result<Vec<TrialPayload>, TrialResolutionError> {
    build_asset_choice_trials(
        db,
        "gv",
        trials,
        playable_mental_rotation_options,
        "no playable Mental Rotation assets",
    )
    .await
}

type PlayableOptionsFn = for<'a> fn(&'a [GameAssetRes]) -> Option<Vec<&'a GameAssetRes>>;

async fn build_asset_choice_trials(
    db: &SupabaseClient,
    game_code: &str,
    trials: &[TrialPayload],
    playable_options: PlayableOptionsFn,
    no_playable_assets_error: &'static str,
) -> Result<Vec<TrialPayload>, TrialResolutionError> {
    let mut answers_by_puzzle = HashMap::new();

    for trial in trials.iter() {
        let Some(puzzle_id) = trial.puzzle_id else {
            return Err(TrialResolutionError::BadRequest("puzzle_id is required"));
        };
        let Some(selected_option_id) = trial.selected_option_id else {
            return Err(TrialResolutionError::BadRequest(
                "selected_option_id is required",
            ));
        };

        if puzzle_id < 1 {
            return Err(TrialResolutionError::BadRequest("puzzle_id must be >= 1"));
        }
        if selected_option_id < 1 {
            return Err(TrialResolutionError::BadRequest(
                "selected_option_id must be >= 1",
            ));
        }
        if trial.ms.is_some_and(|ms| !ms.is_finite() || ms < 0.0) {
            return Err(TrialResolutionError::BadRequest(
                "ms must be a non-negative finite number",
            ));
        }

        if answers_by_puzzle.insert(puzzle_id, trial).is_some() {
            return Err(TrialResolutionError::BadRequest("duplicate puzzle answer"));
        }
    }

    let groups = asset_repository::get_asset_groups_by_code(db, String::from(game_code)).await?;
    let group_ids: Vec<_> = groups.iter().map(|group| group.id).collect();
    let assets = asset_repository::get_game_assets_by_group_ids(db, &group_ids).await?;
    let mut assets_by_group = HashMap::new();

    for asset in assets {
        assets_by_group
            .entry(asset.group_id)
            .or_insert_with(Vec::new)
            .push(asset);
    }

    let mut scoring_trials = Vec::new();

    for group in groups.iter() {
        let Some(assets) = assets_by_group.get(&group.id) else {
            continue;
        };
        let Some(options) = playable_options(assets) else {
            continue;
        };
        let correct_option = options
            .iter()
            .find(|asset| asset.is_correct)
            .expect("playable asset-choice options include exactly one correct option");

        let Some(answer) = answers_by_puzzle.get(&group.id) else {
            return Err(TrialResolutionError::BadRequest("missing puzzle answer"));
        };
        let selected_option_id = answer
            .selected_option_id
            .expect("selected_option_id was validated before insertion");

        if !options.iter().any(|asset| asset.id == selected_option_id) {
            return Err(TrialResolutionError::BadRequest(
                "selected option is invalid",
            ));
        }

        scoring_trials.push(TrialPayload {
            ms: answer.ms,
            span: None,
            correct: Some(selected_option_id == correct_option.id),
            magnitude: None,
            puzzle_id: answer.puzzle_id,
            selected_option_id: answer.selected_option_id,
        });
    }

    if scoring_trials.is_empty() {
        return Err(TrialResolutionError::BadRequest(no_playable_assets_error));
    }
    if answers_by_puzzle.len() != scoring_trials.len() {
        return Err(TrialResolutionError::BadRequest("answer count mismatch"));
    }

    Ok(scoring_trials)
}
