//! Deterministic baseline selection for CI regression context (no scoring).

use crate::core::artifact::ExecutionArtifact;

/// Pick a baseline run for `current` from ledger candidates in **INDEX order**
/// (oldest first in `candidates`).
///
/// Rules (strict):
/// 1. Prefer candidates with the same `command` as `current`.
/// 2. Among those, prefer the same `cwd`.
/// 3. Among matches, prefer the **last** successful run (`exit_code == 0`) in index order.
/// 4. If none successful, use the **last** matching command+cwd run.
/// 5. If no command+cwd match, use the **last** candidate in index order (fallback).
///
/// The current run is excluded when its `run_id` appears in `candidates`.
#[must_use]
pub fn select_baseline_run(
    current: &ExecutionArtifact,
    candidates: &[ExecutionArtifact],
) -> Option<ExecutionArtifact> {
    let ordered: Vec<&ExecutionArtifact> = candidates
        .iter()
        .filter(|a| a.run_id != current.run_id)
        .collect();

    if ordered.is_empty() {
        return None;
    }

    let same_cmd_cwd: Vec<&ExecutionArtifact> = ordered
        .iter()
        .copied()
        .filter(|a| a.command == current.command && a.cwd == current.cwd)
        .collect();

    if !same_cmd_cwd.is_empty() {
        let successes: Vec<&ExecutionArtifact> = same_cmd_cwd
            .iter()
            .copied()
            .filter(|a| a.exit_code == 0)
            .collect();
        if let Some(b) = successes.last() {
            return Some((*b).clone());
        }
        return same_cmd_cwd.last().copied().cloned();
    }

    ordered.last().copied().cloned()
}
