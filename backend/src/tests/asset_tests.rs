use crate::models::assets::*;
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
