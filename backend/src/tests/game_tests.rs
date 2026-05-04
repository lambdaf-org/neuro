use serde_json::Value;

use crate::models::game::TrialPayload;
use crate::services::scoring::ScoreOutcome;
use crate::services::scoring::score_game;

fn metric_field(metrics: &Value, key: &str) -> f64 {
    metrics.get(key).and_then(Value::as_f64).unwrap_or_default()
}

fn integer_metric_field(metrics: &Value, key: &str) -> i64 {
    metrics.get(key).and_then(Value::as_i64).unwrap_or_default()
}

#[test]
fn reaction_scoring_uses_median_valid_ms() {
    let trials = vec![
        TrialPayload {
            ms: Some(120.0),
            span: None,
            correct: None,
            magnitude: None,
            puzzle_id: None,
            selected_option_id: None,
        },
        TrialPayload {
            ms: Some(200.0),
            span: None,
            correct: None,
            magnitude: None,
            puzzle_id: None,
            selected_option_id: None,
        },
        TrialPayload {
            ms: Some(300.0),
            span: None,
            correct: None,
            magnitude: None,
            puzzle_id: None,
            selected_option_id: None,
        },
        TrialPayload {
            ms: Some(400.0),
            span: None,
            correct: None,
            magnitude: None,
            puzzle_id: None,
            selected_option_id: None,
        },
    ];

    match score_game("gt", &trials) {
        ScoreOutcome::Valid { metric, metrics } => {
            assert_eq!(metric, 300.0);
            assert_eq!(integer_metric_field(&metrics, "n_valid"), 3);
        }
        ScoreOutcome::Invalid { reason } => panic!("unexpected invalid score: {reason}"),
    }
}

#[test]
fn working_memory_scoring_uses_max_valid_span() {
    let trials = vec![
        TrialPayload {
            ms: None,
            span: Some(2),
            correct: None,
            magnitude: None,
            puzzle_id: None,
            selected_option_id: None,
        },
        TrialPayload {
            ms: None,
            span: Some(5),
            correct: None,
            magnitude: None,
            puzzle_id: None,
            selected_option_id: None,
        },
        TrialPayload {
            ms: None,
            span: Some(101),
            correct: None,
            magnitude: None,
            puzzle_id: None,
            selected_option_id: None,
        },
        TrialPayload {
            ms: None,
            span: Some(4),
            correct: None,
            magnitude: None,
            puzzle_id: None,
            selected_option_id: None,
        },
    ];

    match score_game("gwm", &trials) {
        ScoreOutcome::Valid { metric, metrics } => {
            assert_eq!(metric, 5.0);
            assert_eq!(integer_metric_field(&metrics, "n_valid"), 3);
            assert_eq!(integer_metric_field(&metrics, "n_fail"), 1);
        }
        ScoreOutcome::Invalid { reason } => panic!("unexpected invalid score: {reason}"),
    }
}

#[test]
fn processing_speed_scoring_counts_correct_trials() {
    let trials = vec![
        TrialPayload {
            ms: Some(300.0),
            span: None,
            correct: Some(true),
            magnitude: None,
            puzzle_id: None,
            selected_option_id: None,
        },
        TrialPayload {
            ms: Some(400.0),
            span: None,
            correct: Some(false),
            magnitude: None,
            puzzle_id: None,
            selected_option_id: None,
        },
        TrialPayload {
            ms: Some(500.0),
            span: None,
            correct: Some(true),
            magnitude: None,
            puzzle_id: None,
            selected_option_id: None,
        },
        TrialPayload {
            ms: Some(250.0),
            span: None,
            correct: Some(true),
            magnitude: None,
            puzzle_id: None,
            selected_option_id: None,
        },
        TrialPayload {
            ms: Some(350.0),
            span: None,
            correct: Some(false),
            magnitude: None,
            puzzle_id: None,
            selected_option_id: None,
        },
    ];

    match score_game("gs", &trials) {
        ScoreOutcome::Valid { metric, metrics } => {
            assert_eq!(metric, 3.0);
            assert_eq!(integer_metric_field(&metrics, "n_trials"), 5);
            assert_eq!(integer_metric_field(&metrics, "n_incorrect"), 2);
            assert_eq!(metric_field(&metrics, "mean_rt"), 360.0);
        }
        ScoreOutcome::Invalid { reason } => panic!("unexpected invalid score: {reason}"),
    }
}

#[test]
fn pattern_logic_scoring_uses_accuracy() {
    let trials = vec![
        TrialPayload {
            ms: Some(1000.0),
            span: None,
            correct: Some(true),
            magnitude: None,
            puzzle_id: None,
            selected_option_id: None,
        },
        TrialPayload {
            ms: Some(1500.0),
            span: None,
            correct: Some(false),
            magnitude: None,
            puzzle_id: None,
            selected_option_id: None,
        },
        TrialPayload {
            ms: Some(2000.0),
            span: None,
            correct: Some(true),
            magnitude: None,
            puzzle_id: None,
            selected_option_id: None,
        },
        TrialPayload {
            ms: Some(2500.0),
            span: None,
            correct: Some(true),
            magnitude: None,
            puzzle_id: None,
            selected_option_id: None,
        },
        TrialPayload {
            ms: Some(3000.0),
            span: None,
            correct: Some(false),
            magnitude: None,
            puzzle_id: None,
            selected_option_id: None,
        },
    ];

    match score_game("gf", &trials) {
        ScoreOutcome::Valid { metric, metrics } => {
            assert_eq!(metric, 0.6);
            assert_eq!(integer_metric_field(&metrics, "n_trials"), 5);
            assert_eq!(integer_metric_field(&metrics, "misses"), 2);
            assert_eq!(metric_field(&metrics, "mean_rt"), 2000.0);
        }
        ScoreOutcome::Invalid { reason } => panic!("unexpected invalid score: {reason}"),
    }
}

#[test]
fn mental_rotation_scoring_uses_accuracy_and_mean_magnitude() {
    let trials = vec![
        TrialPayload {
            ms: None,
            span: None,
            correct: Some(true),
            magnitude: Some(45.0),
            puzzle_id: None,
            selected_option_id: None,
        },
        TrialPayload {
            ms: None,
            span: None,
            correct: Some(false),
            magnitude: Some(90.0),
            puzzle_id: None,
            selected_option_id: None,
        },
        TrialPayload {
            ms: None,
            span: None,
            correct: Some(true),
            magnitude: Some(135.0),
            puzzle_id: None,
            selected_option_id: None,
        },
        TrialPayload {
            ms: None,
            span: None,
            correct: Some(true),
            magnitude: Some(180.0),
            puzzle_id: None,
            selected_option_id: None,
        },
        TrialPayload {
            ms: None,
            span: None,
            correct: Some(false),
            magnitude: Some(90.0),
            puzzle_id: None,
            selected_option_id: None,
        },
    ];

    match score_game("gv", &trials) {
        ScoreOutcome::Valid { metric, metrics } => {
            assert_eq!(metric, 0.6);
            assert_eq!(integer_metric_field(&metrics, "n_trials"), 5);
            assert_eq!(integer_metric_field(&metrics, "misses"), 2);
            assert_eq!(metric_field(&metrics, "mean_magnitude"), 108.0);
        }
        ScoreOutcome::Invalid { reason } => panic!("unexpected invalid score: {reason}"),
    }
}

#[test]
fn unsupported_game_code_is_invalid() {
    match score_game("unknown", &[]) {
        ScoreOutcome::Invalid { reason } => assert_eq!(reason, "no scorer registered for game"),
        ScoreOutcome::Valid { .. } => panic!("unexpected valid score for unknown game"),
    }
}
