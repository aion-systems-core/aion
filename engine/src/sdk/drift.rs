//! Drift between two AI capsules.

use crate::ai::{drift_between_runs, AICapsuleV1};
use aion_core::DriftReport;

pub fn drift_between(a: &AICapsuleV1, b: &AICapsuleV1) -> DriftReport {
    drift_between_runs(a, b)
}
