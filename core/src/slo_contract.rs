use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum SloTarget {
    Availability,
    ReplayFidelity,
    ContractIntegrity,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SloObjective {
    pub target: SloTarget,
    pub threshold_bps: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SloWindow {
    pub name: String,
    pub duration_hours: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SloEvaluationResult {
    pub target: SloTarget,
    pub actual_bps: u64,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SloContract {
    pub window: SloWindow,
    pub objectives: Vec<SloObjective>,
    pub results: Vec<SloEvaluationResult>,
    pub status: String,
}

pub fn evaluate_slo_contract(
    availability_bps: u64,
    replay_fidelity_bps: u64,
    contract_integrity_bps: u64,
) -> SloContract {
    let objectives = vec![
        SloObjective {
            target: SloTarget::Availability,
            threshold_bps: 9950,
        },
        SloObjective {
            target: SloTarget::ReplayFidelity,
            threshold_bps: 9990,
        },
        SloObjective {
            target: SloTarget::ContractIntegrity,
            threshold_bps: 10000,
        },
    ];
    let mut results = vec![
        SloEvaluationResult {
            target: SloTarget::Availability,
            actual_bps: availability_bps,
            status: if availability_bps >= 9950 {
                "ok"
            } else {
                "error"
            }
            .into(),
        },
        SloEvaluationResult {
            target: SloTarget::ReplayFidelity,
            actual_bps: replay_fidelity_bps,
            status: if replay_fidelity_bps >= 9990 {
                "ok"
            } else {
                "error"
            }
            .into(),
        },
        SloEvaluationResult {
            target: SloTarget::ContractIntegrity,
            actual_bps: contract_integrity_bps,
            status: if contract_integrity_bps >= 10000 {
                "ok"
            } else {
                "error"
            }
            .into(),
        },
    ];
    results.sort_by(|a, b| a.target.cmp(&b.target));
    SloContract {
        window: SloWindow {
            name: "rolling_30d".into(),
            duration_hours: 720,
        },
        objectives,
        status: if results.iter().all(|r| r.status == "ok") {
            "ok".into()
        } else {
            "error".into()
        },
        results,
    }
}

#[cfg(test)]
mod tests {
    use super::evaluate_slo_contract;

    #[test]
    fn slo_ok_and_violation() {
        assert_eq!(evaluate_slo_contract(9960, 9995, 10000).status, "ok");
        assert_eq!(evaluate_slo_contract(9900, 9995, 10000).status, "error");
    }
}
