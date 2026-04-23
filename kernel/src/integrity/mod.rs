//! Kernel integrity: build hash, policy/determinism rules, evidence binding.

mod enforcer;
mod report;
mod rules;

pub use enforcer::evaluate_and_enforce;
pub use report::{build_report, kernel_build_hash, IntegrityReport};
pub use rules::evaluate;

use aion_core::{DeterminismProfile, PolicyProfile, RunResult};

/// Full report (non-fatal: rules may fail but still returned).
pub fn full_report(
    policy: Option<&PolicyProfile>,
    det: Option<&DeterminismProfile>,
    run: Option<&RunResult>,
) -> IntegrityReport {
    let pol = policy.cloned().unwrap_or_default();
    let det = det.cloned().unwrap_or_default();
    let outcomes = evaluate(&pol, &det);
    let ev = run.map(|r| aion_core::seal_run(r, &pol, &det));
    build_report(&outcomes, ev.as_ref())
}

/// Legacy-compatible single hash string.
pub fn self_integrity_hash() -> String {
    kernel_build_hash()
}
