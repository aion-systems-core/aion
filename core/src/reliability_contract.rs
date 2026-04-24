use serde::{Deserialize, Serialize};

use crate::chaos_contract::ChaosResult;
use crate::slo_contract::SloContract;
use crate::soak_test_contract::SoakTestResult;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ErrorBudget {
    pub slo_target: String,
    pub remaining_bps: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ErrorBudgetStatus {
    pub status: String,
    pub budgets: Vec<ErrorBudget>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IncidentCriteria {
    pub p1_trigger: String,
    pub p2_trigger: String,
    pub p3_trigger: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReliabilityEvaluation {
    pub error_budget_status: ErrorBudgetStatus,
    pub incident_criteria: IncidentCriteria,
    pub change_failure_rate_target_bps: u64,
    pub mttr_target_minutes: u64,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReliabilityContract {
    pub slo_status: SloContract,
    pub reliability_status: ReliabilityEvaluation,
    pub chaos_status: ChaosResult,
    pub soak_status: SoakTestResult,
}

pub fn evaluate_reliability_contract(
    slo: SloContract,
    chaos: ChaosResult,
    soak: SoakTestResult,
) -> ReliabilityContract {
    let mut budgets = slo
        .results
        .iter()
        .map(|r| ErrorBudget {
            slo_target: format!("{:?}", r.target),
            remaining_bps: 10000_i64 - r.actual_bps as i64,
        })
        .collect::<Vec<_>>();
    budgets.sort_by(|a, b| a.slo_target.cmp(&b.slo_target));
    let budget_status = if budgets.iter().any(|b| b.remaining_bps > 50) {
        "error"
    } else {
        "ok"
    };
    let reliability_status = ReliabilityEvaluation {
        error_budget_status: ErrorBudgetStatus {
            status: budget_status.into(),
            budgets,
        },
        incident_criteria: IncidentCriteria {
            p1_trigger: "availability_below_99_0_or_contract_break".into(),
            p2_trigger: "replay_fidelity_below_99_5".into(),
            p3_trigger: "transient_degradation_without_data_risk".into(),
        },
        change_failure_rate_target_bps: 500,
        mttr_target_minutes: 60,
        status: if slo.status == "ok"
            && chaos.status != "error"
            && soak.status == "ok"
            && budget_status == "ok"
        {
            "ok".into()
        } else {
            "error".into()
        },
    };
    ReliabilityContract {
        slo_status: slo,
        reliability_status,
        chaos_status: chaos,
        soak_status: soak,
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        evaluate_slo_contract, run_chaos_experiments, run_soak_test_plan, ChaosExperiment,
        ChaosFault, ChaosTarget, SoakTestMetric, SoakTestPlan, SoakTestTarget,
    };

    use super::evaluate_reliability_contract;

    #[test]
    fn reliability_ok_and_error_paths() {
        let slo_ok = evaluate_slo_contract(9980, 9995, 10000);
        let chaos_ok = run_chaos_experiments(vec![ChaosExperiment {
            id: "c1".into(),
            target: ChaosTarget::Io,
            fault: ChaosFault::Timeout,
            expected_behavior: "retry".into(),
            status: "executed".into(),
        }]);
        let soak_ok = run_soak_test_plan(
            SoakTestPlan {
                targets: vec![SoakTestTarget {
                    name: "longrun".into(),
                    duration_hours: 24,
                }],
                metrics: vec![SoakTestMetric {
                    name: "leak".into(),
                    threshold: "none".into(),
                }],
            },
            false,
        );
        assert_eq!(
            evaluate_reliability_contract(slo_ok, chaos_ok, soak_ok)
                .reliability_status
                .status,
            "ok"
        );

        let slo_bad = evaluate_slo_contract(9800, 9995, 10000);
        let chaos_bad = run_chaos_experiments(vec![ChaosExperiment {
            id: "c2".into(),
            target: ChaosTarget::Replay,
            fault: ChaosFault::Error,
            expected_behavior: "contract_error".into(),
            status: "failed".into(),
        }]);
        let soak_bad = run_soak_test_plan(
            SoakTestPlan {
                targets: vec![SoakTestTarget {
                    name: "longrun".into(),
                    duration_hours: 24,
                }],
                metrics: vec![SoakTestMetric {
                    name: "leak".into(),
                    threshold: "none".into(),
                }],
            },
            true,
        );
        assert_eq!(
            evaluate_reliability_contract(slo_bad, chaos_bad, soak_bad)
                .reliability_status
                .status,
            "error"
        );
    }
}
