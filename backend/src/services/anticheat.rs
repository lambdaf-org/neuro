// Anticheat rule proposal
//
// TIMING: response too fast to be human
//   T-001  reaction < 150ms
//   T-002  symbol match score > 120 in 90s
//   T-003  matrix solve < 2s
//   T-004  sequence recall < 200ms after shown
//   T-005  rotation answer < 500ms
//
// PATTERN: scores that don't happen naturally
//   P-001  20+ trials in a row all correct
//   P-002  every hard Gf item correct
//   P-003  scores only go up, never down
//   P-004  exact same metric_value 3+ sessions
//
// VARIANCE: too consistent, humans are noisy
//   V-001  timing std_dev < 5ms over 10+ trials
//   V-002  inter-event gap std_dev < 10ms
//   V-003  literally zero outliers in the distribution
//
// RATE: spamming the system
//   R-001  5+ sessions opened in under a minute
//   R-002  20+ events in 1 second
//   R-003  session done in < 3s
//
// TEMPORAL: messing with timestamps
//   X-001  client_ts ahead of server by > 2s
//   X-002  client_ts goes backward in same session
//   X-003  clock drift > 30s sustained
//
// Escalation:
//   one flag alone                      -> log it, keep going
//   two flags in session                -> ban
//   P-001, P-002, R-001, R-002          -> ban immediately
//   X-001 once                          -> flag + drop that trial
//   X-001 twice                         -> ban
//
// Per-round evaluation: this service is invoked from the events WebSocket
// after every round. `prior_event_ms` carries the reaction times stored for
// previous rounds in this session; `new_event_ms` is the round just received.

use crate::models::anticheat::{AnticheatAction, AnticheatFlag, AnticheatVerdict};

const SUPERHUMAN_REACTION_MS: f64 = 150.0;
const IMPOSSIBLY_FAST_MS: f64 = 80.0;
const MIN_TRIALS_FOR_VARIANCE: usize = 10;
const MIN_VARIANCE_STDDEV_MS: f64 = 5.0;

pub fn evaluate_round(
    _game_code: &str,
    prior_event_ms: &[f64],
    new_event_ms: f64,
) -> AnticheatVerdict {
    let mut flags: Vec<AnticheatFlag> = Vec::new();
    let mut force_ban = false;

    let valid = new_event_ms.is_finite() && new_event_ms >= 0.0;

    if valid && new_event_ms < IMPOSSIBLY_FAST_MS {
        // Below physiological floor — script-fast, ban on the spot.
        flags.push(AnticheatFlag {
            code: "T-001",
            reason: format!(
                "reaction {new_event_ms:.0}ms below physiological floor {IMPOSSIBLY_FAST_MS:.0}ms"
            ),
        });
        force_ban = true;
    } else if valid && new_event_ms < SUPERHUMAN_REACTION_MS {
        let prior_superfast = prior_event_ms
            .iter()
            .filter(|m| m.is_finite() && **m < SUPERHUMAN_REACTION_MS)
            .count();
        if prior_superfast >= 1 {
            flags.push(AnticheatFlag {
                code: "T-001",
                reason: format!(
                    "{} reactions below {SUPERHUMAN_REACTION_MS:.0}ms in this session",
                    prior_superfast + 1
                ),
            });
            force_ban = true;
        } else {
            flags.push(AnticheatFlag {
                code: "T-001",
                reason: format!("reaction {new_event_ms:.0}ms below {SUPERHUMAN_REACTION_MS:.0}ms"),
            });
        }
    }

    let all_events: Vec<f64> = prior_event_ms
        .iter()
        .copied()
        .filter(|m| m.is_finite() && *m >= 0.0)
        .chain(if valid { Some(new_event_ms) } else { None })
        .collect();

    if all_events.len() >= MIN_TRIALS_FOR_VARIANCE {
        let stddev = std_dev(&all_events);
        if stddev < MIN_VARIANCE_STDDEV_MS {
            flags.push(AnticheatFlag {
                code: "V-001",
                reason: format!(
                    "reaction std_dev {stddev:.2}ms over {} trials",
                    all_events.len()
                ),
            });
        }
    }

    let action = if force_ban || flags.len() >= 2 {
        AnticheatAction::Ban
    } else if !flags.is_empty() {
        AnticheatAction::Flag
    } else {
        AnticheatAction::Allow
    };

    AnticheatVerdict { action, flags }
}

fn std_dev(values: &[f64]) -> f64 {
    if values.len() < 2 {
        return 0.0;
    }
    let n = values.len() as f64;
    let mean = values.iter().sum::<f64>() / n;
    let variance = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / n;
    variance.sqrt()
}
