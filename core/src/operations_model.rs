use serde::{Deserialize, Serialize};

use crate::{
    DisasterRecoveryContract, IncidentContract, RunbookContract, UpgradeMigrationContract,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OperationsModel {
    pub runbooks: RunbookContract,
    pub incident_model: IncidentContract,
    pub dr_status: DisasterRecoveryContract,
    pub upgrade_migration_status: UpgradeMigrationContract,
    pub status: String,
}

pub fn evaluate_operations_model(
    runbooks: RunbookContract,
    incident_model: IncidentContract,
    dr_status: DisasterRecoveryContract,
    upgrade_migration_status: UpgradeMigrationContract,
) -> OperationsModel {
    let status = if runbooks.status == "ok"
        && incident_model.status == "ok"
        && dr_status.status == "ok"
        && upgrade_migration_status.status == "ok"
    {
        "ok"
    } else {
        "error"
    };
    OperationsModel {
        runbooks,
        incident_model,
        dr_status,
        upgrade_migration_status,
        status: status.into(),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        evaluate_dr_contract, evaluate_incident_contract, evaluate_runbook_contract,
        evaluate_upgrade_migration_contract, BackupPolicy, DrTestResult, IncidentResolution,
        IncidentResponsePlan, IncidentSeverity, IncidentTrigger, RecoveryObjective, RestorePlan,
        RunbookResult, RunbookScenario, RunbookStep, UpgradePath,
    };

    use super::evaluate_operations_model;

    #[test]
    fn baseline_operations_model_fulfilled() {
        let runbooks = evaluate_runbook_contract(vec![RunbookResult {
            scenario: RunbookScenario::Incident,
            preconditions: vec!["alert".into()],
            steps: vec![RunbookStep {
                id: "01".into(),
                action: "acknowledge".into(),
                expected_outcome: "incident_opened".into(),
            }],
            status: "ok".into(),
        }]);
        let incident = evaluate_incident_contract(
            vec![IncidentTrigger {
                id: "i1".into(),
                condition: "slo_violation".into(),
                severity: IncidentSeverity::P1,
            }],
            IncidentResponsePlan {
                owner: "oncall".into(),
                steps: vec!["triage".into()],
                mttr_target_minutes: 60,
            },
            IncidentResolution {
                resolved: true,
                resolution_code: "incident:resolved".into(),
            },
        );
        let dr = evaluate_dr_contract(
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
                steps: vec!["restore".into()],
                last_tested_epoch: 0,
            },
            vec![DrTestResult {
                scenario: "restore".into(),
                status: "passed".into(),
            }],
        );
        let upgrade = evaluate_upgrade_migration_contract(
            vec![UpgradePath {
                from_version: "1.0.0".into(),
                to_version: "1.1.0".into(),
                steps: vec![crate::MigrationStep {
                    id: "01".into(),
                    scope: "contracts".into(),
                    action: "migrate".into(),
                }],
            }],
            vec![],
            vec![],
        );
        let model = evaluate_operations_model(runbooks, incident, dr, upgrade);
        assert_eq!(model.status, "ok");
    }
}

