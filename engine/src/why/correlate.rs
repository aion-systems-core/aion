//! Correlate environment drift with stdout drift (deterministic heuristic).

use aion_core::DriftReport;

pub fn env_stdout_linked(d: &DriftReport) -> bool {
    d.fields.iter().any(|f| f == "env_fingerprint") && d.fields.iter().any(|f| f == "stdout")
}
