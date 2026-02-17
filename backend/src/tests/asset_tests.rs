// src/tests/asset_tests.rs
use crate::models::assets::*;
use serde_json::json;

// Verify that a valid asset group JSON maps correctly to AssetGroupRes
// and that the assets vec defaults to empty via skip_deserializing
#[test]
fn deserialize_asset_group_valid() {
    let data = json!({
        "id": 1,
        "label": "group_00",
        "game_code": "gv"
    });
    let group: AssetGroupRes = serde_json::from_value(data).unwrap();
    assert_eq!(group.id, 1);
    assert_eq!(group.game_code, "gv");
    assert!(group.assets.is_empty());
}

// Missing game_code should fail deserialization -> all fields except assets are required
#[test]
fn deserialize_asset_group_missing_field() {
    let data = json!({
        "id": 1,
        "label": "group_00"
    });
    let result: Result<AssetGroupRes, _> = serde_json::from_value(data);
    assert!(result.is_err());
}

// Valid game asset with is_correct=true should deserialize and preserve the flag
#[test]
fn deserialize_game_asset_valid() {
    let data = json!({
        "id": 10,
        "group_id": 1,
        "label": "option_0",
        "image_url": "https://example.com/img.svg",
        "is_correct": true
    });
    let asset: GameAssetRes = serde_json::from_value(data).unwrap();
    assert_eq!(asset.id, 10);
    assert!(asset.is_correct);
}

// Missing image_url and is_correct should fail -> partial assets are invalid
#[test]
fn deserialize_game_asset_missing_field() {
    let data = json!({
        "id": 10,
        "group_id": 1,
        "label": "option_0"
    });
    let result: Result<GameAssetRes, _> = serde_json::from_value(data);
    assert!(result.is_err());
}

// Valid create request should map cleanly
#[test]
fn deserialize_create_asset_group_valid() {
    let data = json!({
        "game_code": "gf",
        "label": "matrix_01"
    });
    let req: CreateAssetGroupReq = serde_json::from_value(data).unwrap();
    assert_eq!(req.game_code, "gf");
}

// Empty string is valid at the serde level -> handler is responsible for rejecting it
#[test]
fn deserialize_create_asset_group_empty_code() {
    let data = json!({
        "game_code": "",
        "label": "matrix_01"
    });
    let req: CreateAssetGroupReq = serde_json::from_value(data).unwrap();
    assert!(req.game_code.is_empty());
}

// Valid create game asset request with valid URL should deserialize
#[test]
fn deserialize_create_game_asset_valid_url() {
    let data = json!({
        "group_id": 1,
        "label": "asset_1",
        "image_url": "https://example.com/image.png",
        "is_correct": true
    });
    let req: CreateGameAssetReq = serde_json::from_value(data).unwrap();
    assert_eq!(req.image_url, "https://example.com/image.png");
}

// Invalid URL string should still deserialize at serde level -> handler validates URL format
#[test]
fn deserialize_create_game_asset_invalid_url() {
    let data = json!({
        "group_id": 1,
        "label": "asset_1",
        "image_url": "not-a-valid-url",
        "is_correct": true
    });
    let req: CreateGameAssetReq = serde_json::from_value(data).unwrap();
    assert_eq!(req.image_url, "not-a-valid-url");
}

// Valid update game asset request with valid URL should deserialize
#[test]
fn deserialize_update_game_asset_valid_url() {
    let data = json!({
        "label": "updated_asset",
        "image_url": "https://example.com/updated.jpg",
        "is_correct": false
    });
    let req: UpdateGameAssetReq = serde_json::from_value(data).unwrap();
    assert_eq!(req.image_url, "https://example.com/updated.jpg");
}

// Invalid URL string should still deserialize at serde level -> handler validates URL format
#[test]
fn deserialize_update_game_asset_invalid_url() {
    let data = json!({
        "label": "updated_asset",
        "image_url": "invalid url with spaces",
        "is_correct": false
    });
    let req: UpdateGameAssetReq = serde_json::from_value(data).unwrap();
    assert_eq!(req.image_url, "invalid url with spaces");
}
