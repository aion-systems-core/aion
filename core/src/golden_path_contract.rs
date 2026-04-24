use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum GoldenPathScenario {
    Pilot,
    Staging,
    ProductionRollout,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GoldenPathStep {
    pub id: String,
    pub precondition: String,
    pub action: String,
    pub expected_outcome: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GoldenPathResult {
    pub scenario: GoldenPathScenario,
    pub steps: Vec<GoldenPathStep>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GoldenPathContract {
    pub paths: Vec<GoldenPathResult>,
    pub status: String,
}

pub fn evaluate_golden_path_contract(mut contract: GoldenPathContract) -> GoldenPathContract {
    contract.paths.sort_by(|a, b| a.scenario.cmp(&b.scenario));
    for p in &mut contract.paths {
        p.steps.sort_by(|a, b| a.id.cmp(&b.id));
        if p.steps.is_empty() {
            p.status = "error".into();
        }
    }
    contract.status = if contract.paths.iter().all(|p| p.status == "ok") {
        "ok".into()
    } else {
        "error".into()
    };
    contract
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn golden_path_incomplete_negative() {
        let c = evaluate_golden_path_contract(GoldenPathContract {
            paths: vec![GoldenPathResult {
                scenario: GoldenPathScenario::Pilot,
                steps: vec![],
                status: "ok".into(),
            }],
            status: String::new(),
        });
        assert_eq!(c.status, "error");
    }
}
