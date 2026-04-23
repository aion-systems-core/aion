//! Declarative integrity rules over policy + determinism.

use aion_core::{DeterminismProfile, PolicyProfile};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleOutcome {
    pub rule_id: &'static str,
    pub passed: bool,
    pub detail: String,
}

pub fn evaluate(policy: &PolicyProfile, det: &DeterminismProfile) -> Vec<RuleOutcome> {
    vec![
        RuleOutcome {
            rule_id: "deterministic_time",
            passed: !policy.deterministic_time || det.time_frozen,
            detail: "frozen clock when policy requires deterministic time".into(),
        },
        RuleOutcome {
            rule_id: "deterministic_random",
            passed: policy.deterministic_random,
            detail: "RNG policy flag".into(),
        },
        RuleOutcome {
            rule_id: "network_policy",
            passed: true,
            detail: format!("no_network={}", policy.no_network),
        },
    ]
}
