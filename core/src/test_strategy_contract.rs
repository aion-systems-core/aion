use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum TestLayer {
    Unit,
    Integration,
    E2e,
    Regression,
    Compatibility,
    Fuzz,
    Property,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TestCoverageTarget {
    pub area: String,
    pub layers: Vec<TestLayer>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TestCoverageStatus {
    pub area: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TestStrategyContract {
    pub targets: Vec<TestCoverageTarget>,
    pub coverage: Vec<TestCoverageStatus>,
    pub status: String,
}

pub fn evaluate_test_strategy_contract(
    mut targets: Vec<TestCoverageTarget>,
    mut coverage: Vec<TestCoverageStatus>,
) -> TestStrategyContract {
    targets.sort_by(|a, b| a.area.cmp(&b.area));
    coverage.sort_by(|a, b| a.area.cmp(&b.area));
    for t in &mut targets {
        t.layers.sort();
    }
    let missing = coverage.iter().any(|c| c.status == "missing");
    let partial = coverage.iter().any(|c| c.status == "partial");
    let status = if missing {
        "missing"
    } else if partial {
        "partial"
    } else {
        "complete"
    };
    TestStrategyContract {
        targets,
        coverage,
        status: status.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn missing_layer_negative() {
        let c = evaluate_test_strategy_contract(
            vec![TestCoverageTarget {
                area: "kernel".into(),
                layers: vec![TestLayer::Unit],
            }],
            vec![TestCoverageStatus {
                area: "kernel".into(),
                status: "missing".into(),
            }],
        );
        assert_eq!(c.status, "missing");
    }
}

