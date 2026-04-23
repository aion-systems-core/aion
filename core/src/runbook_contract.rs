use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum RunbookScenario {
    Incident,
    Rollback,
    KeyRotation,
    PolicyFailure,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RunbookStep {
    pub id: String,
    pub action: String,
    pub expected_outcome: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RunbookResult {
    pub scenario: RunbookScenario,
    pub preconditions: Vec<String>,
    pub steps: Vec<RunbookStep>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RunbookContract {
    pub results: Vec<RunbookResult>,
    pub status: String,
}

pub fn evaluate_runbook_contract(mut results: Vec<RunbookResult>) -> RunbookContract {
    results.sort_by(|a, b| a.scenario.cmp(&b.scenario));
    for r in &mut results {
        r.steps.sort_by(|a, b| a.id.cmp(&b.id));
        r.preconditions.sort();
        if r.steps.is_empty() {
            r.status = "error".into();
        }
    }
    let status = if results.iter().all(|r| r.status == "ok") {
        "ok"
    } else {
        "error"
    };
    RunbookContract {
        results,
        status: status.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::{evaluate_runbook_contract, RunbookResult, RunbookScenario, RunbookStep};

    #[test]
    fn runbook_negative_missing_steps() {
        let c = evaluate_runbook_contract(vec![RunbookResult {
            scenario: RunbookScenario::Incident,
            preconditions: vec!["alert_received".into()],
            steps: vec![],
            status: "ok".into(),
        }]);
        assert_eq!(c.status, "error");
    }

    #[test]
    fn runbook_positive_ok() {
        let c = evaluate_runbook_contract(vec![RunbookResult {
            scenario: RunbookScenario::Rollback,
            preconditions: vec!["release_candidate_deployed".into()],
            steps: vec![RunbookStep {
                id: "01".into(),
                action: "switch_to_previous_release".into(),
                expected_outcome: "service_restored".into(),
            }],
            status: "ok".into(),
        }]);
        assert_eq!(c.status, "ok");
    }
}

