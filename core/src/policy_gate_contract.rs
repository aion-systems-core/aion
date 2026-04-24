use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum PolicyGateContext {
    Ci,
    Cd,
    Runtime,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PolicyGateDecision {
    Allow,
    Deny,
    Warn,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PolicyGateViolation {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PolicyGate {
    pub context: PolicyGateContext,
    pub decision: Option<PolicyGateDecision>,
    pub violations: Vec<PolicyGateViolation>,
    pub status: String,
}

pub fn evaluate_policy_gate(mut gate: PolicyGate) -> PolicyGate {
    gate.violations.sort_by(|a, b| a.code.cmp(&b.code));
    gate.status = if gate.decision.is_none() {
        "error".into()
    } else {
        "ok".into()
    };
    gate
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn gate_without_decision_negative() {
        let g = evaluate_policy_gate(PolicyGate {
            context: PolicyGateContext::Ci,
            decision: None,
            violations: vec![],
            status: String::new(),
        });
        assert_eq!(g.status, "error");
    }
}
