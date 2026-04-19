// COS integration boundary.
//
// `CosKernelInterface` is the contract the real COS kernel will implement
// once it is wired in. `StubCosKernel` delegates to the in-process core
// capture path (real subprocess execution).
//
// Rule: nothing in `core/` or `cli/` may refer to COS symbols directly —
// everything goes through this trait.

#![allow(dead_code)]

use crate::core::artifact::ExecutionArtifact;
use crate::core::capture::{capture_command_with_clock, Clock, SystemClock};
use crate::core::diff::{diff_runs, DiffReport};
use crate::core::replay;

pub trait CosKernelInterface {
    fn capture_execution(&self, command: String) -> ExecutionArtifact;
    fn replay_artifact(&self, artifact: &ExecutionArtifact) -> String;
    fn diff_artifacts(&self, a: &ExecutionArtifact, b: &ExecutionArtifact) -> DiffReport;
}

/// In-process stub. Identical behavior to the `core` modules.
pub struct StubCosKernel<C: Clock = SystemClock> {
    clock: C,
}

impl Default for StubCosKernel<SystemClock> {
    fn default() -> Self {
        Self { clock: SystemClock }
    }
}

impl<C: Clock> StubCosKernel<C> {
    pub fn with_clock(clock: C) -> Self {
        Self { clock }
    }
}

impl<C: Clock> CosKernelInterface for StubCosKernel<C> {
    fn capture_execution(&self, command: String) -> ExecutionArtifact {
        capture_command_with_clock(command, &self.clock)
    }

    fn replay_artifact(&self, artifact: &ExecutionArtifact) -> String {
        replay::format_replay(artifact)
    }

    fn diff_artifacts(&self, a: &ExecutionArtifact, b: &ExecutionArtifact) -> DiffReport {
        diff_runs(a, b)
    }
}
