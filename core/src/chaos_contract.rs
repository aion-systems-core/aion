use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ChaosTarget {
    Io,
    Policy,
    Evidence,
    Replay,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ChaosFault {
    Timeout,
    Error,
    Corruption,
    Drop,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChaosExperiment {
    pub id: String,
    pub target: ChaosTarget,
    pub fault: ChaosFault,
    pub expected_behavior: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChaosResult {
    pub experiments: Vec<ChaosExperiment>,
    pub status: String,
}

pub fn run_chaos_experiments(mut experiments: Vec<ChaosExperiment>) -> ChaosResult {
    experiments.sort_by(|a, b| a.id.cmp(&b.id));
    let status = if experiments.iter().any(|e| e.status == "failed") {
        "error"
    } else if experiments.iter().any(|e| e.status == "planned") {
        "partial"
    } else {
        "ok"
    };
    ChaosResult {
        experiments,
        status: status.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::{run_chaos_experiments, ChaosExperiment, ChaosFault, ChaosTarget};

    #[test]
    fn chaos_statuses() {
        let ok = run_chaos_experiments(vec![ChaosExperiment {
            id: "c1".into(),
            target: ChaosTarget::Io,
            fault: ChaosFault::Timeout,
            expected_behavior: "retry_and_contract_error".into(),
            status: "passed".into(),
        }]);
        assert_eq!(ok.status, "ok");
        let err = run_chaos_experiments(vec![ChaosExperiment {
            id: "c2".into(),
            target: ChaosTarget::Replay,
            fault: ChaosFault::Corruption,
            expected_behavior: "fail_with_code".into(),
            status: "failed".into(),
        }]);
        assert_eq!(err.status, "error");
    }
}

