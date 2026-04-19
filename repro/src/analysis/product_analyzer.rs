// Product analyzer.
//
// A rule-based, curated inventory of what the current `repro`
// implementation does *not* do yet. This is intentionally editorial:
// it is how the project tells the truth about itself in the `repro
// eval` report, and it is what stops the maturity score from being a
// vanity metric.
//
// Entries are grouped into four categories:
//   * MissingFeature   — capability the user would reasonably expect
//   * ArchitectureGap  — structural limitation that will block future work
//   * UxWeakness       — friction in the current command surface
//   * FailureMode      — a way the tool can silently give wrong answers
//
// Each entry carries a severity so downstream scoring can weight them.
// When a gap is closed, delete its entry. Do not edit it to say
// "done": the absence *is* the proof.

use crate::core::report::Severity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum GapCategory {
    MissingFeature,
    ArchitectureGap,
    UxWeakness,
    FailureMode,
}

impl GapCategory {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingFeature => "missing_feature",
            Self::ArchitectureGap => "architecture_gap",
            Self::UxWeakness => "ux_weakness",
            Self::FailureMode => "failure_mode",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProductGap {
    pub category: GapCategory,
    pub severity: Severity,
    pub title: String,
    pub description: String,
}

/// Deterministic, alphabetically-stable list of gaps. Order here is
/// the render order in the `repro eval` report.
pub fn gaps() -> Vec<ProductGap> {
    let mut v = vec![
        g(
            GapCategory::ArchitectureGap,
            Severity::Medium,
            "Environment snapshot is strict 15% slice",
            "`environment_hash` fingerprints cwd plus exactly PATH, HOME, CI, \
             SHELL, and LANG from a fixed allowlist. No OS/arch strings, no \
             tool probes, no full environment enumeration API.",
        ),
        g(
            GapCategory::ArchitectureGap,
            Severity::Medium,
            "No trace step model",
            "A run has `stdout`, `stderr`, `exit_code`, and nothing else. \
             There is no per-step trace, no timeline, no syscall log, no \
             file-system side-effect capture. Divergence inside a long \
             command is reduced to 'the bytes differ'.",
        ),
        g(
            GapCategory::FailureMode,
            Severity::Low,
            "POSIX vs Windows invocation edge cases",
            "Windows may execute bare `echo` via `cmd /C`; argv is not passed \
             through a full POSIX shell. Complex quoting should be validated \
             manually outside repro.",
        ),
        g(
            GapCategory::MissingFeature,
            Severity::Medium,
            "No `run list` / `runs` command",
            "The INDEX file is the canonical ordering but there is no way \
             to list it from the CLI. Users have to `cat repro_runs/INDEX` \
             manually.",
        ),
        g(
            GapCategory::MissingFeature,
            Severity::Medium,
            "No portable export bundle",
            "`repro ci ingest` accepts JSON into the ledger, but there is no \
             `repro export` to bundle runs + streams for offline handoff.",
        ),
    ];
    v.sort_by(|a, b| (a.category.as_str(), &a.title).cmp(&(b.category.as_str(), &b.title)));
    v
}

fn g(category: GapCategory, severity: Severity, title: &str, description: &str) -> ProductGap {
    ProductGap {
        category,
        severity,
        title: title.to_string(),
        description: description.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gaps_are_stable_across_calls() {
        assert_eq!(gaps(), gaps());
    }

    #[test]
    fn gaps_are_sorted() {
        let g = gaps();
        let mut sorted = g.clone();
        sorted
            .sort_by(|a, b| (a.category.as_str(), &a.title).cmp(&(b.category.as_str(), &b.title)));
        assert_eq!(g, sorted);
    }
}
