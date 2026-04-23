use crate::determinism_matrix::DeterminismMatrix;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReplayInvariantCheck {
    pub name: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReplayInvariantViolation {
    pub code: String,
    pub context: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReplayInvariantGate {
    pub checks: Vec<ReplayInvariantCheck>,
    pub violations: Vec<ReplayInvariantViolation>,
    pub status: String,
}

pub fn run_replay_invariant_gate(
    replay_invariant_ok: bool,
    matrix: &DeterminismMatrix,
    contract_snapshots_deterministic: bool,
) -> ReplayInvariantGate {
    let matrix_ok = matrix.results.iter().all(|r| r.status == "ok");
    let checks = vec![
        ReplayInvariantCheck {
            name: "replay_invariant".into(),
            status: if replay_invariant_ok { "ok" } else { "error" }.into(),
        },
        ReplayInvariantCheck {
            name: "determinism_matrix".into(),
            status: if matrix_ok { "ok" } else { "error" }.into(),
        },
        ReplayInvariantCheck {
            name: "contract_snapshots".into(),
            status: if contract_snapshots_deterministic { "ok" } else { "error" }.into(),
        },
    ];
    let mut violations = Vec::new();
    if !replay_invariant_ok {
        violations.push(ReplayInvariantViolation {
            code: "replay_gate:invariant_failed".into(),
            context: "replay_invariant_gate.replay".into(),
            cause: Some("replay_invariant_violated".into()),
        });
    }
    if !matrix_ok {
        violations.push(ReplayInvariantViolation {
            code: "replay_gate:matrix_failed".into(),
            context: "replay_invariant_gate.matrix".into(),
            cause: Some("supported_target_failed".into()),
        });
    }
    if !contract_snapshots_deterministic {
        violations.push(ReplayInvariantViolation {
            code: "replay_gate:snapshots_nondeterministic".into(),
            context: "replay_invariant_gate.snapshots".into(),
            cause: Some("snapshot_hash_mismatch".into()),
        });
    }
    ReplayInvariantGate {
        checks,
        status: if violations.is_empty() { "ok".into() } else { "error".into() },
        violations,
    }
}

#[cfg(test)]
mod tests {
    use super::run_replay_invariant_gate;
    use crate::{evaluate_determinism_matrix, DeterminismTarget};

    #[test]
    fn gate_positive() {
        let m = evaluate_determinism_matrix(vec![DeterminismTarget {
            os: "linux".into(),
            arch: "x64".into(),
            locale: "en_US.UTF-8".into(),
            timezone: "UTC".into(),
            seed: 42,
            env_profile: "frozen".into(),
        }]);
        let g = run_replay_invariant_gate(true, &m, true);
        assert_eq!(g.status, "ok");
    }

    #[test]
    fn gate_negative_matrix_replay() {
        let m = evaluate_determinism_matrix(vec![DeterminismTarget {
            os: "linux".into(),
            arch: "x64".into(),
            locale: "de_DE.UTF-8".into(),
            timezone: "Europe/Berlin".into(),
            seed: 99,
            env_profile: "dynamic".into(),
        }]);
        let g = run_replay_invariant_gate(false, &m, false);
        assert_eq!(g.status, "error");
        assert_eq!(g.violations.len(), 3);
    }
}

