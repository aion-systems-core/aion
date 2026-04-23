use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum IncidentSeverity {
    P1,
    P2,
    P3,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IncidentTrigger {
    pub id: String,
    pub condition: String,
    pub severity: IncidentSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IncidentResponsePlan {
    pub owner: String,
    pub steps: Vec<String>,
    pub mttr_target_minutes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IncidentResolution {
    pub resolved: bool,
    pub resolution_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IncidentContract {
    pub triggers: Vec<IncidentTrigger>,
    pub response_plan: IncidentResponsePlan,
    pub resolution: IncidentResolution,
    pub status: String,
}

pub fn evaluate_incident_contract(
    mut triggers: Vec<IncidentTrigger>,
    mut response_plan: IncidentResponsePlan,
    resolution: IncidentResolution,
) -> IncidentContract {
    triggers.sort_by(|a, b| a.id.cmp(&b.id));
    response_plan.steps.sort();
    let status = if triggers.is_empty() || response_plan.steps.is_empty() || response_plan.owner.is_empty() {
        "error"
    } else if resolution.resolved {
        "ok"
    } else {
        "error"
    };
    IncidentContract {
        triggers,
        response_plan,
        resolution,
        status: status.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn incident_negative_incomplete_response() {
        let c = evaluate_incident_contract(
            vec![IncidentTrigger {
                id: "i1".into(),
                condition: "slo_violation".into(),
                severity: IncidentSeverity::P1,
            }],
            IncidentResponsePlan {
                owner: "oncall".into(),
                steps: vec![],
                mttr_target_minutes: 60,
            },
            IncidentResolution {
                resolved: false,
                resolution_code: "incident:open".into(),
            },
        );
        assert_eq!(c.status, "error");
    }
}

