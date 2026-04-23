use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpgradeReplayViolation {
    pub code: String,
    pub context: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpgradeReplayResult {
    pub target_version: String,
    pub status: String,
    pub violations: Vec<UpgradeReplayViolation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpgradeReplayContract {
    pub current_kernel_version: String,
    pub support_window: String,
    pub tested_upgrade_targets: Vec<String>,
    pub results: Vec<UpgradeReplayResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpgradeReplayInput {
    pub replay_deterministic: bool,
    pub abi_compatible: bool,
    pub evidence_compatible: bool,
    pub policy_compatible: bool,
}

fn parse_semver_triplet(v: &str) -> (u64, u64, u64) {
    let core = v.split('+').next().unwrap_or(v);
    let mut parts = core.split('.');
    let major = parts.next().and_then(|x| x.parse::<u64>().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|x| x.parse::<u64>().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|x| x.parse::<u64>().ok()).unwrap_or(0);
    (major, minor, patch)
}

fn next_minor(v: &str, step: u64) -> String {
    let (major, minor, _) = parse_semver_triplet(v);
    format!("{major}.{}.0", minor + step)
}

fn evaluate_target(target_version: &str, input: &UpgradeReplayInput) -> UpgradeReplayResult {
    let mut violations = Vec::new();
    if !input.replay_deterministic {
        violations.push(UpgradeReplayViolation {
            code: "upgrade:replay_mismatch".to_string(),
            context: "upgrade_replay.replay".to_string(),
            cause: Some("cross_version_replay_not_deterministic".to_string()),
        });
    }
    if !input.abi_compatible {
        violations.push(UpgradeReplayViolation {
            code: "upgrade:abi_incompatible".to_string(),
            context: "upgrade_replay.abi".to_string(),
            cause: Some("capsule_abi_break".to_string()),
        });
    }
    if !input.evidence_compatible {
        violations.push(UpgradeReplayViolation {
            code: "upgrade:evidence_incompatible".to_string(),
            context: "upgrade_replay.evidence".to_string(),
            cause: Some("evidence_chain_not_verifiable".to_string()),
        });
    }
    if !input.policy_compatible {
        violations.push(UpgradeReplayViolation {
            code: "upgrade:policy_incompatible".to_string(),
            context: "upgrade_replay.policy".to_string(),
            cause: Some("policy_decision_not_stable".to_string()),
        });
    }
    UpgradeReplayResult {
        target_version: target_version.to_string(),
        status: if violations.is_empty() {
            "ok".to_string()
        } else {
            "error".to_string()
        },
        violations,
    }
}

pub fn evaluate_upgrade_replay(
    current_kernel_version: &str,
    next_input: UpgradeReplayInput,
    n2_input: UpgradeReplayInput,
) -> UpgradeReplayContract {
    let next = next_minor(current_kernel_version, 1);
    let n2 = next_minor(current_kernel_version, 2);
    let r1 = evaluate_target(&next, &next_input);
    let r2 = evaluate_target(&n2, &n2_input);
    UpgradeReplayContract {
        current_kernel_version: current_kernel_version.to_string(),
        support_window: "N_to_Nplus2".to_string(),
        tested_upgrade_targets: vec![next, n2],
        results: vec![r1, r2],
    }
}

#[cfg(test)]
mod tests {
    use super::{evaluate_upgrade_replay, UpgradeReplayInput};

    fn ok_input() -> UpgradeReplayInput {
        UpgradeReplayInput {
            replay_deterministic: true,
            abi_compatible: true,
            evidence_compatible: true,
            policy_compatible: true,
        }
    }

    #[test]
    fn cross_version_replay_n_to_n_plus_1_and_n_plus_2_ok() {
        let c = evaluate_upgrade_replay("0.2.0", ok_input(), ok_input());
        assert_eq!(c.tested_upgrade_targets, vec!["0.3.0", "0.4.0"]);
        assert!(c.results.iter().all(|r| r.status == "ok"));
    }

    #[test]
    fn abi_compatibility_violation_is_reported() {
        let mut bad = ok_input();
        bad.abi_compatible = false;
        let c = evaluate_upgrade_replay("0.2.0", bad, ok_input());
        assert_eq!(c.results[0].status, "error");
        assert!(
            c.results[0]
                .violations
                .iter()
                .any(|v| v.code == "upgrade:abi_incompatible")
        );
    }

    #[test]
    fn evidence_compatibility_violation_is_reported() {
        let mut bad = ok_input();
        bad.evidence_compatible = false;
        let c = evaluate_upgrade_replay("0.2.0", ok_input(), bad);
        assert_eq!(c.results[1].status, "error");
        assert!(
            c.results[1]
                .violations
                .iter()
                .any(|v| v.code == "upgrade:evidence_incompatible")
        );
    }

    #[test]
    fn deterministic_violation_output_order() {
        let bad = UpgradeReplayInput {
            replay_deterministic: false,
            abi_compatible: false,
            evidence_compatible: false,
            policy_compatible: false,
        };
        let c = evaluate_upgrade_replay("0.2.0", bad, ok_input());
        let codes: Vec<&str> = c.results[0].violations.iter().map(|v| v.code.as_str()).collect();
        assert_eq!(
            codes,
            vec![
                "upgrade:replay_mismatch",
                "upgrade:abi_incompatible",
                "upgrade:evidence_incompatible",
                "upgrade:policy_incompatible",
            ]
        );
    }
}

