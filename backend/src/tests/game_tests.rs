use crate::models::assets::AssetGroupRes;
use crate::models::assets::CreateAssetGroupReq;
use crate::models::assets::GameAssetRes;
use serde_json::json;

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

#[test]
fn deserialize_asset_group_missing_field() {
    let data = json!({
        "id": 1,
        "label": "group_00"
    });
    let result: Result<AssetGroupRes, _> = serde_json::from_value(data);
    assert!(result.is_err());
}

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

#[test]
fn deserialize_create_asset_group_valid() {
    let data = json!({
        "game_code": "gf",
        "label": "matrix_01"
    });
    let req: CreateAssetGroupReq = serde_json::from_value(data).unwrap();
    assert_eq!(req.game_code, "gf");
}

#[test]
fn deserialize_create_asset_group_empty_code() {
    let data = json!({
        "game_code": "",
        "label": "matrix_01"
    });
    // Deserializes fine, empty string is valid serde, validation is handler responsibility
    let req: CreateAssetGroupReq = serde_json::from_value(data).unwrap();
    assert!(req.game_code.is_empty());
}
