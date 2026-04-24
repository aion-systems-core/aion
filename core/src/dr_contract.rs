use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecoveryObjective {
    pub rpo_minutes: u64,
    pub rto_minutes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BackupPolicy {
    pub cadence: String,
    pub retention_days: u64,
    pub immutable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RestorePlan {
    pub steps: Vec<String>,
    pub last_tested_epoch: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DrTestResult {
    pub scenario: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DisasterRecoveryContract {
    pub objective: RecoveryObjective,
    pub backup_policy: BackupPolicy,
    pub restore_plan: RestorePlan,
    pub tests: Vec<DrTestResult>,
    pub status: String,
}

pub fn evaluate_dr_contract(
    objective: RecoveryObjective,
    backup_policy: BackupPolicy,
    mut restore_plan: RestorePlan,
    mut tests: Vec<DrTestResult>,
) -> DisasterRecoveryContract {
    restore_plan.steps.sort();
    tests.sort_by(|a, b| a.scenario.cmp(&b.scenario));
    let has_restore_test = tests
        .iter()
        .any(|t| t.scenario == "restore" && t.status == "passed");
    let status = if restore_plan.steps.is_empty() || !has_restore_test {
        "error"
    } else {
        "ok"
    };
    DisasterRecoveryContract {
        objective,
        backup_policy,
        restore_plan,
        tests,
        status: status.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dr_negative_without_restore_test() {
        let c = evaluate_dr_contract(
            RecoveryObjective {
                rpo_minutes: 15,
                rto_minutes: 60,
            },
            BackupPolicy {
                cadence: "hourly".into(),
                retention_days: 30,
                immutable: true,
            },
            RestorePlan {
                steps: vec!["restore_snapshot".into()],
                last_tested_epoch: 0,
            },
            vec![DrTestResult {
                scenario: "backup".into(),
                status: "passed".into(),
            }],
        );
        assert_eq!(c.status, "error");
    }
}
