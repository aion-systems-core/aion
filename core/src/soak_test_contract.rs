use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SoakTestTarget {
    pub name: String,
    pub duration_hours: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SoakTestMetric {
    pub name: String,
    pub threshold: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SoakTestPlan {
    pub targets: Vec<SoakTestTarget>,
    pub metrics: Vec<SoakTestMetric>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SoakTestResult {
    pub plan: SoakTestPlan,
    pub status: String,
    pub notes: Vec<String>,
}

pub fn run_soak_test_plan(plan: SoakTestPlan, failed: bool) -> SoakTestResult {
    SoakTestResult {
        plan,
        status: if failed { "error" } else { "ok" }.into(),
        notes: if failed {
            vec!["soak:stability_degradation".into()]
        } else {
            vec!["soak:stable".into()]
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{run_soak_test_plan, SoakTestMetric, SoakTestPlan, SoakTestTarget};

    #[test]
    fn soak_ok_and_failed() {
        let plan = SoakTestPlan {
            targets: vec![SoakTestTarget {
                name: "replay_longrun".into(),
                duration_hours: 24,
            }],
            metrics: vec![SoakTestMetric {
                name: "memory_growth".into(),
                threshold: "<5%".into(),
            }],
        };
        assert_eq!(run_soak_test_plan(plan.clone(), false).status, "ok");
        assert_eq!(run_soak_test_plan(plan, true).status, "error");
    }
}

