use crate::models::anticheat::AnticheatAction;
use crate::services::anticheat::{EventTuple, evaluate_round};

fn event(ms: f64, correct: Option<bool>) -> EventTuple {
    (ms, correct)
}

fn flag_codes(verdict_flags: &[crate::models::anticheat::AnticheatFlag]) -> Vec<&'static str> {
    verdict_flags.iter().map(|flag| flag.code).collect()
}

#[test]
fn normal_human_timing_is_allowed() {
    let history = [event(240.0, None), event(310.0, None), event(280.0, None)];

    let verdict = evaluate_round("gt", &history, event(260.0, None));

    assert_eq!(verdict.action, AnticheatAction::Allow);
    assert!(verdict.flags.is_empty());
}

#[test]
fn single_superhuman_reaction_is_flagged() {
    let verdict = evaluate_round("gt", &[], event(120.0, None));

    assert_eq!(verdict.action, AnticheatAction::Flag);
    assert_eq!(flag_codes(&verdict.flags), ["T-001"]);
}

#[test]
fn repeated_superhuman_reactions_are_banned() {
    let history = [event(120.0, None)];

    let verdict = evaluate_round("gt", &history, event(130.0, None));

    assert_eq!(verdict.action, AnticheatAction::Ban);
    assert_eq!(flag_codes(&verdict.flags), ["T-001"]);
}

#[test]
fn impossibly_fast_reaction_is_banned_immediately() {
    let verdict = evaluate_round("gt", &[], event(79.0, None));

    assert_eq!(verdict.action, AnticheatAction::Ban);
    assert_eq!(flag_codes(&verdict.flags), ["T-001"]);
}

#[test]
fn low_variance_after_ten_trials_is_flagged() {
    let history = [
        event(250.0, None),
        event(250.0, None),
        event(250.0, None),
        event(250.0, None),
        event(250.0, None),
        event(250.0, None),
        event(250.0, None),
        event(250.0, None),
        event(250.0, None),
    ];

    let verdict = evaluate_round("gt", &history, event(250.0, None));

    assert_eq!(verdict.action, AnticheatAction::Flag);
    assert_eq!(flag_codes(&verdict.flags), ["V-001"]);
}

#[test]
fn processing_speed_fast_correct_streak_is_banned() {
    let history = [
        event(220.0, Some(true)),
        event(230.0, Some(true)),
        event(240.0, Some(true)),
        event(250.0, Some(true)),
    ];

    let verdict = evaluate_round("gs", &history, event(260.0, Some(true)));

    assert_eq!(verdict.action, AnticheatAction::Ban);
    assert_eq!(flag_codes(&verdict.flags), ["P-001"]);
}
