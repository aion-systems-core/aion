use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeterminismGuarantee {
    pub scope: String,
    pub guarantee: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeterminismViolation {
    pub code: String,
    pub origin: String,
    pub context: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeterminismContract {
    pub guarantees: Vec<DeterminismGuarantee>,
    pub violations: Vec<DeterminismViolation>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeterminismContractInput {
    pub replay_ok: bool,
    pub drift_ok: bool,
    pub evidence_ok: bool,
    pub policy_ok: bool,
    pub global_consistency_ok: bool,
    pub upgrade_replay_ok: bool,
}

pub fn evaluate_determinism_contract(input: DeterminismContractInput) -> DeterminismContract {
    let guarantees = vec![
        DeterminismGuarantee {
            scope: "replay".into(),
            guarantee: "replay_deterministic_for_supported_matrix".into(),
        },
        DeterminismGuarantee {
            scope: "drift".into(),
            guarantee: "drift_contract_deterministic".into(),
        },
        DeterminismGuarantee {
            scope: "evidence".into(),
            guarantee: "evidence_chain_deterministic".into(),
        },
        DeterminismGuarantee {
            scope: "policy".into(),
            guarantee: "policy_decisions_deterministic".into(),
        },
        DeterminismGuarantee {
            scope: "global_consistency".into(),
            guarantee: "global_finality_deterministic".into(),
        },
        DeterminismGuarantee {
            scope: "upgrade_replay".into(),
            guarantee: "cross_version_replay_deterministic".into(),
        },
    ];
    let mut violations = Vec::new();
    if !input.replay_ok {
        violations.push(DeterminismViolation {
            code: "determinism:replay_failed".into(),
            origin: "determinism_contract".into(),
            context: "determinism_contract.replay".into(),
            cause: Some("replay_not_deterministic".into()),
        });
    }
    if !input.drift_ok {
        violations.push(DeterminismViolation {
            code: "determinism:drift_failed".into(),
            origin: "determinism_contract".into(),
            context: "determinism_contract.drift".into(),
            cause: Some("drift_not_deterministic".into()),
        });
    }
    if !input.evidence_ok {
        violations.push(DeterminismViolation {
            code: "determinism:evidence_failed".into(),
            origin: "determinism_contract".into(),
            context: "determinism_contract.evidence".into(),
            cause: Some("evidence_not_deterministic".into()),
        });
    }
    if !input.policy_ok {
        violations.push(DeterminismViolation {
            code: "determinism:policy_failed".into(),
            origin: "determinism_contract".into(),
            context: "determinism_contract.policy".into(),
            cause: Some("policy_not_deterministic".into()),
        });
    }
    if !input.global_consistency_ok {
        violations.push(DeterminismViolation {
            code: "determinism:global_consistency_failed".into(),
            origin: "determinism_contract".into(),
            context: "determinism_contract.global_consistency".into(),
            cause: Some("global_consistency_not_deterministic".into()),
        });
    }
    if !input.upgrade_replay_ok {
        violations.push(DeterminismViolation {
            code: "determinism:upgrade_replay_failed".into(),
            origin: "determinism_contract".into(),
            context: "determinism_contract.upgrade_replay".into(),
            cause: Some("upgrade_replay_not_deterministic".into()),
        });
    }
    DeterminismContract {
        guarantees,
        status: if violations.is_empty() {
            "ok".into()
        } else {
            "error".into()
        },
        violations,
    }
}

#[cfg(test)]
mod tests {
    use super::{evaluate_determinism_contract, DeterminismContractInput};

    #[test]
    fn positive_all_ok() {
        let c = evaluate_determinism_contract(DeterminismContractInput {
            replay_ok: true,
            drift_ok: true,
            evidence_ok: true,
            policy_ok: true,
            global_consistency_ok: true,
            upgrade_replay_ok: true,
        });
        assert_eq!(c.status, "ok");
    }

    #[test]
    fn negative_replay_drift() {
        let c = evaluate_determinism_contract(DeterminismContractInput {
            replay_ok: false,
            drift_ok: false,
            evidence_ok: true,
            policy_ok: true,
            global_consistency_ok: true,
            upgrade_replay_ok: true,
        });
        assert_eq!(c.status, "error");
        assert_eq!(c.violations[0].code, "determinism:replay_failed");
        assert_eq!(c.violations[1].code, "determinism:drift_failed");
    }
}
