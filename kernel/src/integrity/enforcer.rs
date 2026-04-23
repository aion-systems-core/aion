//! Enforce integrity rule outcomes (fail-fast for hard gates).

use super::rules::{evaluate, RuleOutcome};

pub fn enforce(outcomes: &[RuleOutcome]) -> Result<(), String> {
    for o in outcomes {
        if !o.passed {
            return Err(format!("integrity rule {} failed: {}", o.rule_id, o.detail));
        }
    }
    Ok(())
}

pub fn evaluate_and_enforce(
    policy: &aion_core::PolicyProfile,
    det: &aion_core::DeterminismProfile,
) -> Result<Vec<RuleOutcome>, String> {
    let v = evaluate(policy, det);
    enforce(&v)?;
    Ok(v)
}
