use serde_json::Value;

use crate::handlers::game_handler::build_finalize_session_result;
use crate::models::game::{FinalizeSessionReq, TrialPayload};
use crate::models::validate::Validate;
use crate::services::scoring::SCORING_VERSION;

fn trial(ms: Option<f64>, correct: Option<bool>) -> TrialPayload {
    TrialPayload {
        ms,
        span: None,
        correct,
        magnitude: None,
        puzzle_id: None,
        selected_option_id: None,
    }
}

fn finalize_req(trials: Vec<TrialPayload>, score: Option<f64>) -> FinalizeSessionReq {
    FinalizeSessionReq {
        trials,
        client_ts: None,
        score,
    }
}

fn string_metric_field(metrics: &Value, key: &str) -> String {
    metrics
        .get(key)
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string()
}

#[test]
fn finalize_request_requires_trials_or_legacy_score() {
    let request = finalize_req(Vec::new(), None);

    assert_eq!(
        request.validate(),
        Err(vec!["trials or legacy score is required"])
    );
}

#[test]
fn finalize_request_rejects_invalid_client_timestamp() {
    let request = FinalizeSessionReq {
        trials: Vec::new(),
        client_ts: Some(String::from("not-a-date")),
        score: Some(42.0),
    };

    assert_eq!(
        request.validate(),
        Err(vec!["client_ts must be a valid RFC3339 timestamp"])
    );
}

#[test]
fn finalize_request_accepts_rfc3339_client_timestamp() {
    let request = FinalizeSessionReq {
        trials: Vec::new(),
        client_ts: Some(String::from("2026-05-31T10:00:00Z")),
        score: Some(42.0),
    };

    assert!(request.validate().is_ok());
}

#[test]
fn finalize_result_completes_valid_reaction_trials() {
    let request = finalize_req(
        vec![
            trial(Some(200.0), None),
            trial(Some(300.0), None),
            trial(Some(400.0), None),
        ],
        None,
    );

    let result = build_finalize_session_result("gt", &request, &request.trials);

    assert_eq!(result.status, "completed");
    assert_eq!(result.metric_value, Some(300.0));
    assert_eq!(result.scoring_version, SCORING_VERSION);
    assert_eq!(result.metrics.get("n_valid").and_then(Value::as_i64), Some(3));
}

#[test]
fn finalize_result_marks_invalid_when_scoring_rejects_trials() {
    let request = finalize_req(vec![trial(Some(120.0), None), trial(Some(140.0), None)], None);

    let result = build_finalize_session_result("gt", &request, &request.trials);

    assert_eq!(result.status, "invalid");
    assert_eq!(result.metric_value, None);
    assert_eq!(result.scoring_version, SCORING_VERSION);
    assert_eq!(
        string_metric_field(&result.metrics, "reason"),
        "not enough valid trials"
    );
}

#[test]
fn finalize_result_supports_legacy_score_without_trials() {
    let request = finalize_req(Vec::new(), Some(42.0));

    let result = build_finalize_session_result("gt", &request, &request.trials);

    assert_eq!(result.status, "completed");
    assert_eq!(result.metric_value, Some(42.0));
    assert_eq!(result.scoring_version, 0);
    assert_eq!(result.metrics, serde_json::json!({}));
}
