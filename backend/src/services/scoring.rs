use serde_json::Value;
use serde_json::json;

use crate::models::game::TrialPayload;

// Will need some rethinking since there usually are outliers on the high and low end of the distribution
pub const SCORING_VERSION: i32 = 1;
const MIN_VALID_REACTION_MS: f64 = 150.0;
const MAX_VALID_REACTION_MS: f64 = 1500.0;
const MIN_REACTION_TRIALS: usize = 3;
const MIN_WORKING_MEMORY_SPAN: i32 = 1;
const MAX_WORKING_MEMORY_SPAN: i32 = 99;
const MIN_WORKING_MEMORY_TRIALS: usize = 3;
const MIN_ACCURACY_TRIALS: usize = 5;

pub enum ScoreOutcome {
    Valid { metric: f64, metrics: Value },
    Invalid { reason: &'static str },
}

pub trait Scorer {
    fn score(&self, trials: &[TrialPayload]) -> ScoreOutcome;
}

pub fn score_game(game_code: &str, trials: &[TrialPayload]) -> ScoreOutcome {
    match game_code {
        "gt" => ReactionScorer.score(trials),
        "gwm" => WorkingMemoryScorer.score(trials),
        "gs" => ProcessingSpeedScorer.score(trials),
        "gf" => PatternLogicScorer.score(trials),
        "gv" => MentalRotationScorer.score(trials),
        _ => ScoreOutcome::Invalid {
            reason: "no scorer registered for game",
        },
    }
}

struct ReactionScorer;
struct WorkingMemoryScorer;
struct ProcessingSpeedScorer;
struct PatternLogicScorer;
struct MentalRotationScorer;

impl Scorer for ReactionScorer {
    fn score(&self, trials: &[TrialPayload]) -> ScoreOutcome {
        let mut valid_times = trials
            .iter()
            .filter_map(|trial| trial.ms)
            .filter(|ms| ms.is_finite())
            .filter(|ms| *ms >= MIN_VALID_REACTION_MS && *ms <= MAX_VALID_REACTION_MS)
            .collect::<Vec<_>>();

        if valid_times.len() < MIN_REACTION_TRIALS {
            return ScoreOutcome::Invalid {
                reason: "not enough valid trials",
            };
        }

        valid_times
            .sort_by(|left, right| left.partial_cmp(right).unwrap_or(std::cmp::Ordering::Equal));

        let mid = valid_times.len() / 2;
        let metric = if valid_times.len() % 2 == 0 {
            (valid_times[mid - 1] + valid_times[mid]) / 2.0
        } else {
            valid_times[mid]
        };

        ScoreOutcome::Valid {
            metric,
            metrics: json!({
                "n_trials": trials.len(),
                "n_valid": valid_times.len(),
                "n_dropped": trials.len() - valid_times.len(),
            }),
        }
    }
}

impl Scorer for WorkingMemoryScorer {
    fn score(&self, trials: &[TrialPayload]) -> ScoreOutcome {
        let valid_spans = trials
            .iter()
            .filter_map(|trial| trial.span)
            .filter(|span| (*span >= MIN_WORKING_MEMORY_SPAN) && (*span <= MAX_WORKING_MEMORY_SPAN))
            .collect::<Vec<_>>();

        if valid_spans.len() < MIN_WORKING_MEMORY_TRIALS {
            return ScoreOutcome::Invalid {
                reason: "not enough valid trials",
            };
        }

        let max_span = valid_spans.iter().copied().max().unwrap_or(MIN_WORKING_MEMORY_SPAN);
        let invalid_count = trials.len().saturating_sub(valid_spans.len());

        ScoreOutcome::Valid {
            metric: f64::from(max_span),
            metrics: json!({
                "n_valid": valid_spans.len(),
                "n_fail": invalid_count,
                "retries": invalid_count,
            }),
        }
    }
}

impl Scorer for ProcessingSpeedScorer {
    fn score(&self, trials: &[TrialPayload]) -> ScoreOutcome {
        let valid_trials = trials
            .iter()
            .filter_map(|trial| trial.correct.map(|correct| (correct, trial.ms)))
            .collect::<Vec<_>>();

        if valid_trials.len() < MIN_ACCURACY_TRIALS {
            return ScoreOutcome::Invalid {
                reason: "not enough valid trials",
            };
        }

        let correct_count = valid_trials.iter().filter(|(correct, _)| *correct).count();
        let incorrect_count = valid_trials.len().saturating_sub(correct_count);
        let response_times = valid_trials
            .iter()
            .filter_map(|(_, ms)| *ms)
            .filter(|ms| ms.is_finite() && *ms >= 0.0)
            .collect::<Vec<_>>();
        let mean_rt = if response_times.is_empty() {
            0.0
        } else {
            response_times.iter().sum::<f64>() / response_times.len() as f64
        };

        ScoreOutcome::Valid {
            metric: correct_count as f64,
            metrics: json!({
                "n_trials": valid_trials.len(),
                "n_incorrect": incorrect_count,
                "mean_rt": mean_rt,
            }),
        }
    }
}

impl Scorer for PatternLogicScorer {
    fn score(&self, trials: &[TrialPayload]) -> ScoreOutcome {
        let valid_answers = trials
            .iter()
            .filter_map(|trial| trial.correct)
            .collect::<Vec<_>>();

        if valid_answers.len() < MIN_ACCURACY_TRIALS {
            return ScoreOutcome::Invalid {
                reason: "not enough valid trials",
            };
        }

        let right_count = valid_answers.iter().filter(|correct| **correct).count();
        let misses = valid_answers.len().saturating_sub(right_count);

        ScoreOutcome::Valid {
            metric: right_count as f64 / valid_answers.len() as f64,
            metrics: json!({
                "n_trials": valid_answers.len(),
                "misses": misses,
            }),
        }
    }
}

impl Scorer for MentalRotationScorer {
    fn score(&self, trials: &[TrialPayload]) -> ScoreOutcome {
        let valid_trials = trials
            .iter()
            .filter_map(|trial| trial.correct.map(|correct| (correct, trial.magnitude)))
            .collect::<Vec<_>>();

        if valid_trials.len() < MIN_ACCURACY_TRIALS {
            return ScoreOutcome::Invalid {
                reason: "not enough valid trials",
            };
        }

        let right_count = valid_trials.iter().filter(|(correct, _)| *correct).count();
        let misses = valid_trials.len().saturating_sub(right_count);
        let magnitudes = valid_trials
            .iter()
            .filter_map(|(_, magnitude)| *magnitude)
            .filter(|magnitude| magnitude.is_finite())
            .collect::<Vec<_>>();
        let mean_magnitude = if magnitudes.is_empty() {
            0.0
        } else {
            magnitudes.iter().sum::<f64>() / magnitudes.len() as f64
        };

        ScoreOutcome::Valid {
            metric: right_count as f64 / valid_trials.len() as f64,
            metrics: json!({
                "n_trials": valid_trials.len(),
                "misses": misses,
                "mean_magnitude": mean_magnitude,
            }),
        }
    }
}
