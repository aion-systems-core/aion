use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum RegressionArea {
    Kernel,
    Cli,
    Doctor,
    Replay,
    Policy,
    Governance,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RegressionCase {
    pub id: String,
    pub area: RegressionArea,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RegressionStatus {
    pub case_id: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RegressionMatrix {
    pub cases: Vec<RegressionCase>,
    pub results: Vec<RegressionStatus>,
    pub status: String,
}

pub fn evaluate_regression_matrix(
    mut cases: Vec<RegressionCase>,
    mut results: Vec<RegressionStatus>,
) -> RegressionMatrix {
    cases.sort_by(|a, b| a.id.cmp(&b.id));
    results.sort_by(|a, b| a.case_id.cmp(&b.case_id));
    let has_gap = results.iter().any(|r| r.status == "gap");
    RegressionMatrix {
        cases,
        results,
        status: if has_gap { "error" } else { "ok" }.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn regression_gap_negative() {
        let m = evaluate_regression_matrix(
            vec![RegressionCase {
                id: "reg_001".into(),
                area: RegressionArea::Kernel,
                label: "kernel_boot".into(),
            }],
            vec![RegressionStatus {
                case_id: "reg_001".into(),
                status: "gap".into(),
            }],
        );
        assert_eq!(m.status, "error");
    }
}
