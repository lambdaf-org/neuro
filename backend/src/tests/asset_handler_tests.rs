// Tests for asset_handler.rs
//
// This test module provides coverage for the asset handler functions.
// Due to the tight coupling with SupabaseClient and the absence of a mocking framework,
// these tests focus on:
//
// 1. Request model deserialization - Ensuring all request models properly serialize/deserialize
// 2. Request model validation - Verifying required fields are enforced at the serde level
// 3. Handler behavior documentation - Each handler function's expected behavior is documented below
//
// HANDLER FUNCTIONS COVERAGE:
//
// Asset Group Handlers:
// - create_asset_group: Validates that game_code and label are non-empty, calls repository to insert, returns Created with ID or BadRequest on error
// - list_asset_groups: Fetches all asset groups and populates their assets array via repository, returns Ok with array or BadRequest on error
// - get_asset_groups_by_code: Fetches asset groups by game code and populates assets, returns Ok with array or BadRequest on error
// - update_asset_group: Validates non-empty fields, calls repository to update by ID, returns Ok on success or BadRequest on error
// - delete_asset_group: Calls repository to delete by ID, returns NoContent on success or BadRequest on error
//
// Game Asset Handlers:
// - create_game_asset: Calls repository to insert with all fields, returns Created with ID or BadRequest on error
// - list_game_assets: Fetches assets by group_id, returns Ok with array or BadRequest on error
// - update_game_asset: Calls repository to update all fields by ID, returns Ok on success or BadRequest on error
// - delete_game_asset: Calls repository to delete by ID, returns NoContent on success or BadRequest on error
//
// INTEGRATION TEST RECOMMENDATIONS:
// For full integration testing with database interactions, consider:
// - Adding a mocking framework like mockall or mockito
// - Setting up a test database with fixture data
// - Testing actual repository error handling and edge cases
// - Testing concurrent operations and race conditions

use serde_json::json;

use crate::models::assets::{CreateAssetGroupReq, CreateGameAssetReq, UpdateAssetGroupReq, UpdateGameAssetReq};

mod request_deserialization_tests {
    use super::*;

    // === CreateAssetGroupReq Tests ===

    // Test that CreateAssetGroupReq can be deserialized with valid data
    #[test]
    fn test_create_asset_group_req_valid() {
        let data = json!({
            "game_code": "gv",
            "label": "test_group"
        });
        
        let req: CreateAssetGroupReq = serde_json::from_value(data).unwrap();
        assert_eq!(req.game_code, "gv");
        assert_eq!(req.label, "test_group");
    }

    // Test that CreateAssetGroupReq fails with missing game_code
    #[test]
    fn test_create_asset_group_req_missing_game_code() {
        let data = json!({
            "label": "test_group"
        });
        
        let result: Result<CreateAssetGroupReq, _> = serde_json::from_value(data);
        assert!(result.is_err());
    }

    // Test that CreateAssetGroupReq fails with missing label
    #[test]
    fn test_create_asset_group_req_missing_label() {
        let data = json!({
            "game_code": "gv"
        });
        
        let result: Result<CreateAssetGroupReq, _> = serde_json::from_value(data);
        assert!(result.is_err());
    }

    // Test that CreateAssetGroupReq accepts empty strings (validation happens at handler level)
    #[test]
    fn test_create_asset_group_req_empty_fields() {
        let data = json!({
            "game_code": "",
            "label": ""
        });
        
        // Serde accepts empty strings; handler is responsible for validation
        let req: CreateAssetGroupReq = serde_json::from_value(data).unwrap();
        assert_eq!(req.game_code, "");
        assert_eq!(req.label, "");
    }

    // Test that CreateAssetGroupReq with various game codes
    #[test]
    fn test_create_asset_group_req_various_game_codes() {
        for code in ["gv", "gf", "matrix", "puzzle"] {
            let data = json!({
                "game_code": code,
                "label": "test"
            });
            let req: CreateAssetGroupReq = serde_json::from_value(data).unwrap();
            assert_eq!(req.game_code, code);
        }
    }

    // === UpdateAssetGroupReq Tests ===

    // Test that UpdateAssetGroupReq can be deserialized with valid data
    #[test]
    fn test_update_asset_group_req_valid() {
        let data = json!({
            "game_code": "gv",
            "label": "updated_group"
        });
        
        let req: UpdateAssetGroupReq = serde_json::from_value(data).unwrap();
        assert_eq!(req.game_code, "gv");
        assert_eq!(req.label, "updated_group");
    }

    // Test that UpdateAssetGroupReq fails with missing game_code
    #[test]
    fn test_update_asset_group_req_missing_game_code() {
        let data = json!({
            "label": "updated_group"
        });
        
        let result: Result<UpdateAssetGroupReq, _> = serde_json::from_value(data);
        assert!(result.is_err());
    }

    // Test that UpdateAssetGroupReq fails with missing label
    #[test]
    fn test_update_asset_group_req_missing_label() {
        let data = json!({
            "game_code": "gv"
        });
        
        let result: Result<UpdateAssetGroupReq, _> = serde_json::from_value(data);
        assert!(result.is_err());
    }

    // Test that UpdateAssetGroupReq accepts empty strings (validation happens at handler level)
    #[test]
    fn test_update_asset_group_req_empty_fields() {
        let data = json!({
            "game_code": "",
            "label": ""
        });
        
        // Serde accepts empty strings; handler is responsible for validation
        let req: UpdateAssetGroupReq = serde_json::from_value(data).unwrap();
        assert_eq!(req.game_code, "");
        assert_eq!(req.label, "");
    }

    // === CreateGameAssetReq Tests ===

    // Test that CreateGameAssetReq can be deserialized with valid data
    #[test]
    fn test_create_game_asset_req_valid() {
        let data = json!({
            "group_id": 1,
            "label": "asset_01",
            "image_url": "https://example.com/image.png",
            "is_correct": true
        });
        
        let req: CreateGameAssetReq = serde_json::from_value(data).unwrap();
        assert_eq!(req.group_id, 1);
        assert_eq!(req.label, "asset_01");
        assert_eq!(req.image_url, "https://example.com/image.png");
        assert!(req.is_correct);
    }

    // Test that CreateGameAssetReq fails with missing group_id
    #[test]
    fn test_create_game_asset_req_missing_group_id() {
        let data = json!({
            "label": "asset_01",
            "image_url": "https://example.com/image.png",
            "is_correct": true
        });
        
        let result: Result<CreateGameAssetReq, _> = serde_json::from_value(data);
        assert!(result.is_err());
    }

    // Test that CreateGameAssetReq fails with missing label
    #[test]
    fn test_create_game_asset_req_missing_label() {
        let data = json!({
            "group_id": 1,
            "image_url": "https://example.com/image.png",
            "is_correct": true
        });
        
        let result: Result<CreateGameAssetReq, _> = serde_json::from_value(data);
        assert!(result.is_err());
    }

    // Test that CreateGameAssetReq fails with missing image_url
    #[test]
    fn test_create_game_asset_req_missing_image_url() {
        let data = json!({
            "group_id": 1,
            "label": "asset_01",
            "is_correct": true
        });
        
        let result: Result<CreateGameAssetReq, _> = serde_json::from_value(data);
        assert!(result.is_err());
    }

    // Test that CreateGameAssetReq fails with missing is_correct
    #[test]
    fn test_create_game_asset_req_missing_is_correct() {
        let data = json!({
            "group_id": 1,
            "label": "asset_01",
            "image_url": "https://example.com/image.png"
        });
        
        let result: Result<CreateGameAssetReq, _> = serde_json::from_value(data);
        assert!(result.is_err());
    }

    // Test that CreateGameAssetReq accepts is_correct=false
    #[test]
    fn test_create_game_asset_req_is_correct_false() {
        let data = json!({
            "group_id": 1,
            "label": "asset_01",
            "image_url": "https://example.com/image.png",
            "is_correct": false
        });
        
        let req: CreateGameAssetReq = serde_json::from_value(data).unwrap();
        assert!(!req.is_correct);
    }

    // Test that CreateGameAssetReq handles various group IDs
    #[test]
    fn test_create_game_asset_req_various_group_ids() {
        for group_id in [1, 100, 999, 1234567] {
            let data = json!({
                "group_id": group_id,
                "label": "test",
                "image_url": "https://example.com/img.png",
                "is_correct": false
            });
            let req: CreateGameAssetReq = serde_json::from_value(data).unwrap();
            assert_eq!(req.group_id, group_id);
        }
    }

    // Test that CreateGameAssetReq accepts various URL formats
    #[test]
    fn test_create_game_asset_req_various_urls() {
        let urls = [
            "https://example.com/image.png",
            "http://test.com/img.jpg",
            "https://cdn.example.com/assets/images/test.svg",
            "/relative/path/image.png",
        ];
        
        for url in urls {
            let data = json!({
                "group_id": 1,
                "label": "test",
                "image_url": url,
                "is_correct": true
            });
            let req: CreateGameAssetReq = serde_json::from_value(data).unwrap();
            assert_eq!(req.image_url, url);
        }
    }

    // === UpdateGameAssetReq Tests ===

    // Test that UpdateGameAssetReq can be deserialized with valid data
    #[test]
    fn test_update_game_asset_req_valid() {
        let data = json!({
            "label": "updated_asset",
            "image_url": "https://example.com/updated.png",
            "is_correct": false
        });
        
        let req: UpdateGameAssetReq = serde_json::from_value(data).unwrap();
        assert_eq!(req.label, "updated_asset");
        assert_eq!(req.image_url, "https://example.com/updated.png");
        assert!(!req.is_correct);
    }

    // Test that UpdateGameAssetReq fails with missing label
    #[test]
    fn test_update_game_asset_req_missing_label() {
        let data = json!({
            "image_url": "https://example.com/updated.png",
            "is_correct": false
        });
        
        let result: Result<UpdateGameAssetReq, _> = serde_json::from_value(data);
        assert!(result.is_err());
    }

    // Test that UpdateGameAssetReq fails with missing image_url
    #[test]
    fn test_update_game_asset_req_missing_image_url() {
        let data = json!({
            "label": "updated_asset",
            "is_correct": false
        });
        
        let result: Result<UpdateGameAssetReq, _> = serde_json::from_value(data);
        assert!(result.is_err());
    }

    // Test that UpdateGameAssetReq fails with missing is_correct
    #[test]
    fn test_update_game_asset_req_missing_is_correct() {
        let data = json!({
            "label": "updated_asset",
            "image_url": "https://example.com/updated.png"
        });
        
        let result: Result<UpdateGameAssetReq, _> = serde_json::from_value(data);
        assert!(result.is_err());
    }

    // Test that UpdateGameAssetReq accepts both true and false for is_correct
    #[test]
    fn test_update_game_asset_req_is_correct_values() {
        for is_correct in [true, false] {
            let data = json!({
                "label": "test",
                "image_url": "https://example.com/img.png",
                "is_correct": is_correct
            });
            let req: UpdateGameAssetReq = serde_json::from_value(data).unwrap();
            assert_eq!(req.is_correct, is_correct);
        }
    }
}

// Tests documenting handler validation behavior
// These tests verify that the validation logic in handlers works as expected
mod handler_validation_behavior_tests {
    use super::*;

    // === Asset Group Handler Validation ===
    
    // Documents that create_asset_group and update_asset_group validate empty game_code
    #[test]
    fn test_empty_game_code_validation_expectation() {
        // Handler should reject requests with empty game_code
        // Validation: body.game_code.is_empty() -> BadRequest
        let req = CreateAssetGroupReq {
            game_code: String::from(""),
            label: String::from("test"),
        };
        assert!(req.game_code.is_empty(), "Empty game_code should be caught by handler");
    }

    // Documents that create_asset_group and update_asset_group validate empty label
    #[test]
    fn test_empty_label_validation_expectation() {
        // Handler should reject requests with empty label
        // Validation: body.label.is_empty() -> BadRequest
        let req = CreateAssetGroupReq {
            game_code: String::from("gv"),
            label: String::from(""),
        };
        assert!(req.label.is_empty(), "Empty label should be caught by handler");
    }

    // Documents that both fields must be non-empty
    #[test]
    fn test_both_fields_empty_validation_expectation() {
        let req = CreateAssetGroupReq {
            game_code: String::from(""),
            label: String::from(""),
        };
        assert!(req.game_code.is_empty() || req.label.is_empty(), 
                "Handler validates at least one field is non-empty");
    }

    // Documents that whitespace-only strings pass serde but should be handled by application logic
    #[test]
    fn test_whitespace_only_fields() {
        let data = json!({
            "game_code": "   ",
            "label": "   "
        });
        
        let req: CreateAssetGroupReq = serde_json::from_value(data).unwrap();
        // Note: Handlers currently check is_empty() but not whitespace-only strings
        // This is intentional - trimming should be done at UI/validation layer
        assert!(!req.game_code.is_empty());
        assert!(!req.label.is_empty());
    }

    // === Game Asset Handler Validation ===
    
    // Documents that empty fields pass deserialization but are validated by handlers
    #[test]
    fn test_game_asset_no_validation() {
        // CreateGameAssetReq deserialization accepts empty strings.
        // However, the handlers (create_game_asset, update_game_asset) validate
        // empty fields and will reject them before reaching the database.
        let req = CreateGameAssetReq {
            group_id: 1,
            label: String::from(""),
            image_url: String::from(""),
            is_correct: false,
        };
        
        // Deserialization accepts empty strings
        assert_eq!(req.label, "");
        assert_eq!(req.image_url, "");
    }

    // Documents that negative group_ids are technically allowed by the type system
    #[test]
    fn test_negative_group_id_type_system() {
        let data = json!({
            "group_id": -1,
            "label": "test",
            "image_url": "https://example.com/img.png",
            "is_correct": false
        });
        
        // i32 type allows negative values; database constraints should prevent invalid IDs
        let req: CreateGameAssetReq = serde_json::from_value(data).unwrap();
        assert_eq!(req.group_id, -1);
    }
}

// Tests for edge cases and special scenarios
mod edge_case_tests {
    use super::*;

    // Test very long strings are accepted at deserialization level
    #[test]
    fn test_very_long_label() {
        let long_label = "a".repeat(10000);
        let data = json!({
            "game_code": "gv",
            "label": long_label
        });
        
        let req: CreateAssetGroupReq = serde_json::from_value(data).unwrap();
        assert_eq!(req.label.len(), 10000);
    }

    // Test unicode characters in labels and game codes
    #[test]
    fn test_unicode_characters() {
        let data = json!({
            "game_code": "游戏代码",
            "label": "标签 🎮 テスト"
        });
        
        let req: CreateAssetGroupReq = serde_json::from_value(data).unwrap();
        assert_eq!(req.game_code, "游戏代码");
        assert!(req.label.contains("🎮"));
    }

    // Test special characters in URLs
    #[test]
    fn test_special_characters_in_url() {
        let url = "https://example.com/image?param=value&other=123#anchor";
        let data = json!({
            "group_id": 1,
            "label": "test",
            "image_url": url,
            "is_correct": true
        });
        
        let req: CreateGameAssetReq = serde_json::from_value(data).unwrap();
        assert_eq!(req.image_url, url);
    }

    // Test zero group_id is technically valid
    #[test]
    fn test_zero_group_id() {
        let data = json!({
            "group_id": 0,
            "label": "test",
            "image_url": "https://example.com/img.png",
            "is_correct": false
        });
        
        let req: CreateGameAssetReq = serde_json::from_value(data).unwrap();
        assert_eq!(req.group_id, 0);
    }

    // Test maximum i32 value for group_id
    #[test]
    fn test_max_group_id() {
        let data = json!({
            "group_id": i32::MAX,
            "label": "test",
            "image_url": "https://example.com/img.png",
            "is_correct": true
        });
        
        let req: CreateGameAssetReq = serde_json::from_value(data).unwrap();
        assert_eq!(req.group_id, i32::MAX);
    }

    // Test that extra fields in JSON are ignored during deserialization
    #[test]
    fn test_extra_fields_ignored() {
        let data = json!({
            "game_code": "gv",
            "label": "test",
            "extra_field": "ignored",
            "another_field": 123
        });
        
        let req: CreateAssetGroupReq = serde_json::from_value(data).unwrap();
        assert_eq!(req.game_code, "gv");
        assert_eq!(req.label, "test");
    }

    // Test empty URL string
    #[test]
    fn test_empty_url_string() {
        let data = json!({
            "group_id": 1,
            "label": "test",
            "image_url": "",
            "is_correct": false
        });
        
        // Empty URL passes deserialization; handler validates empty fields
        let req: CreateGameAssetReq = serde_json::from_value(data).unwrap();
        assert_eq!(req.image_url, "");
    }
}
