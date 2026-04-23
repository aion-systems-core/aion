// Deterministic field-by-field diff between two execution artifacts.
//
// Field list is the contract — it must stay in sync with
// `artifact::ExecutionArtifact` and with `root_cause.rs`.
//
// AION layer: full-env deltas (`ReproRun`), temporal epoch ordering (INDEX or
// deterministic fallback), and causal narration for `repro diff` output.

use crate::core::artifact::{ExecutionArtifact, ReproRun};
use crate::core::causal_graph::CausalGraph;
use crate::core::execution_trace::ExecutionTrace;
use aion_core::diff::{diff as core_diff, DiffOptions};
use aion_core::run::{sha256_prefixed, RunResult};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// Ordered list of fields compared. Order defines divergence semantics.
pub const FIELDS: &[&str] = &[
    "command",
    "environment_hash",
    "cwd",
    "exit_code",
    "stdout",
    "stderr",
    "duration_ms",
    "timestamp",
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldDiff {
    pub field: String,
    pub a: String,
    pub b: String,
}

/// One environment key whose value differs between two [`crate::core::artifact::ReproRun`] snapshots.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvVarDelta {
    pub key: String,
    /// Value on the `epoch_a` side (first argument to `repro diff`).
    pub value_a: String,
    /// Value on the `epoch_b` side (second argument to `repro diff`).
    pub value_b: String,
}

/// Which AION run is the older / newer epoch for causal narration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemporalContext {
    pub older_run_id: String,
    pub newer_run_id: String,
    /// True iff `run_a` from the diff invocation maps to the older epoch.
    pub run_a_is_older: bool,
}

/// AION-aware projection layered on top of the legacy [`DiffReport`] fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AionDiff {
    pub env_diff: Vec<EnvVarDelta>,
    /// Mirrors stdout divergence when present (execution state).
    pub stdout_diff: Option<FieldDiff>,
    /// Deterministic causal strings derived from env / execution deltas.
    pub causal_diff: Vec<String>,
    pub temporal_context: Option<TemporalContext>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffReport {
    pub run_a: String,
    pub run_b: String,
    pub differences: Vec<FieldDiff>,
    /// Field names that differ, in `FIELDS` order.
    pub changed_fields: Vec<String>,
    /// Field names that match, in `FIELDS` order.
    pub unchanged_fields: Vec<String>,
    /// Deterministic causal ordering: upstream (env, cwd, command) before
    /// downstream stream/exit fields when those also changed.
    pub causal_chain: Vec<String>,
    #[serde(default)]
    pub aion: AionDiff,
}

fn diff_repro_run_env(a: &ReproRun, b: &ReproRun) -> Vec<EnvVarDelta> {
    let keys: BTreeSet<String> = a.env.keys().chain(b.env.keys()).cloned().collect();
    let mut out = Vec::new();
    for k in keys {
        let va = a.env.get(&k).cloned().unwrap_or_default();
        let vb = b.env.get(&k).cloned().unwrap_or_default();
        if va != vb {
            out.push(EnvVarDelta {
                key: k,
                value_a: va,
                value_b: vb,
            });
        }
    }
    out
}

fn infer_temporal_context(run_a: &str, run_b: &str, index: &[String]) -> TemporalContext {
    let pos_a = index.iter().position(|s| s == run_a);
    let pos_b = index.iter().position(|s| s == run_b);
    match (pos_a, pos_b) {
        (Some(ia), Some(ib)) if ia < ib => TemporalContext {
            older_run_id: run_a.to_string(),
            newer_run_id: run_b.to_string(),
            run_a_is_older: true,
        },
        (Some(ia), Some(ib)) if ia > ib => TemporalContext {
            older_run_id: run_b.to_string(),
            newer_run_id: run_a.to_string(),
            run_a_is_older: false,
        },
        _ => {
            // Deterministic fallback when INDEX is missing or ambiguous: treat
            // lexicographic run_id order as canonical epoch rank (not wall clock).
            if run_a <= run_b {
                TemporalContext {
                    older_run_id: run_a.to_string(),
                    newer_run_id: run_b.to_string(),
                    run_a_is_older: true,
                }
            } else {
                TemporalContext {
                    older_run_id: run_b.to_string(),
                    newer_run_id: run_a.to_string(),
                    run_a_is_older: false,
                }
            }
        }
    }
}

pub fn diff_environment(a: &ExecutionArtifact, b: &ExecutionArtifact) -> Vec<FieldDiff> {
    let mut v = Vec::new();
    let ha = a.environment_hash();
    let hb = b.environment_hash();
    if ha != hb {
        v.push(FieldDiff {
            field: "environment_hash".to_string(),
            a: ha,
            b: hb,
        });
    }
    if a.cwd != b.cwd {
        v.push(FieldDiff {
            field: "cwd".to_string(),
            a: a.cwd.clone(),
            b: b.cwd.clone(),
        });
    }
    v
}

pub fn diff_execution(a: &ExecutionArtifact, b: &ExecutionArtifact) -> Vec<FieldDiff> {
    let baseline = RunResult {
        stdout: a.stdout.clone(),
        stdout_hash: sha256_prefixed(&a.stdout),
        stderr: a.stderr.clone(),
        stderr_hash: sha256_prefixed(&a.stderr),
        exit_code: a.exit_code,
        duration_ms: a.duration_ms,
    };
    let actual = RunResult {
        stdout: b.stdout.clone(),
        stdout_hash: sha256_prefixed(&b.stdout),
        stderr: b.stderr.clone(),
        stderr_hash: sha256_prefixed(&b.stderr),
        exit_code: b.exit_code,
        duration_ms: b.duration_ms,
    };
    let core = core_diff(
        &baseline,
        &actual,
        &DiffOptions {
            ignore_duration: false,
            duration_tolerance: 0.0,
        },
    );
    let mut v = Vec::new();
    for field in core.fields {
        match field.field.as_str() {
            "stdout" => v.push(FieldDiff {
                field: "stdout".to_string(),
                a: a.stdout.clone(),
                b: b.stdout.clone(),
            }),
            "stderr" => v.push(FieldDiff {
                field: "stderr".to_string(),
                a: a.stderr.clone(),
                b: b.stderr.clone(),
            }),
            "exit_code" => v.push(FieldDiff {
                field: "exit_code".to_string(),
                a: a.exit_code.to_string(),
                b: b.exit_code.to_string(),
            }),
            "duration_ms" => v.push(FieldDiff {
                field: "duration_ms".to_string(),
                a: a.duration_ms.to_string(),
                b: b.duration_ms.to_string(),
            }),
            _ => {}
        }
    }
    v
}

pub fn diff_runs(a: &ExecutionArtifact, b: &ExecutionArtifact) -> DiffReport {
    diff_runs_with_index(a, b, &[])
}

/// Same as [`diff_runs`], but supplies the ordered `INDEX` run list so AION
/// temporal context can map **older epoch → newer epoch** without wall clock.
pub fn diff_runs_with_index(
    a: &ExecutionArtifact,
    b: &ExecutionArtifact,
    index: &[String],
) -> DiffReport {
    let env = diff_environment(a, b);
    let exe = diff_execution(a, b);
    let mut differences = Vec::new();
    for field in FIELDS {
        if let Some(d) = env
            .iter()
            .chain(exe.iter())
            .find(|d| d.field.as_str() == *field)
        {
            differences.push(d.clone());
            continue;
        }
        let (va, vb) = project(a, b, field);
        if va != vb {
            differences.push(FieldDiff {
                field: (*field).to_string(),
                a: va,
                b: vb,
            });
        }
    }

    let differences = filter_timing_noise_fields(differences);

    let changed_fields: Vec<String> = differences.iter().map(|d| d.field.clone()).collect();
    let changed_set: std::collections::HashSet<String> = changed_fields.iter().cloned().collect();
    let unchanged_fields: Vec<String> = FIELDS
        .iter()
        .map(|s| (*s).to_string())
        .filter(|f| !changed_set.contains(f))
        .collect();
    let causal_chain = build_causal_chain(&differences);

    let temporal_context = Some(infer_temporal_context(&a.run_id, &b.run_id, index));

    let env_diff = match (&a.repro_run, &b.repro_run) {
        (Some(ra), Some(rb)) => diff_repro_run_env(ra, rb),
        _ => Vec::new(),
    };

    let stdout_diff = differences.iter().find(|d| d.field == "stdout").cloned();

    let causal_diff: Vec<String> = env_diff
        .iter()
        .map(|e| format!("{} caused execution divergence", e.key))
        .collect();

    DiffReport {
        run_a: a.run_id.clone(),
        run_b: b.run_id.clone(),
        differences,
        changed_fields,
        unchanged_fields,
        causal_chain,
        aion: AionDiff {
            env_diff,
            stdout_diff,
            causal_diff,
            temporal_context,
        },
    }
}

/// Drop `timestamp` always. Drop `duration_ms` unless exit code or streams differ
/// (semantic diff present). Keeps [`DiffReport`] stable for root-cause and CI.
fn filter_timing_noise_fields(mut differences: Vec<FieldDiff>) -> Vec<FieldDiff> {
    let has_exit = differences.iter().any(|d| d.field == "exit_code");
    let has_streams = differences
        .iter()
        .any(|d| d.field == "stdout" || d.field == "stderr");
    differences.retain(|d| {
        if d.field == "timestamp" {
            return false;
        }
        if d.field == "duration_ms" && !has_exit && !has_streams {
            return false;
        }
        true
    });
    differences
}

fn push_unique(v: &mut Vec<String>, s: &str) {
    if !v.iter().any(|x| x == s) {
        v.push(s.to_string());
    }
}

/// Build a deterministic causal chain: when streams or exit differ, any
/// environment or command change is listed first as plausible upstream cause.
pub fn build_causal_chain(differences: &[FieldDiff]) -> Vec<String> {
    let changed: std::collections::HashSet<&str> =
        differences.iter().map(|d| d.field.as_str()).collect();
    let mut out: Vec<String> = Vec::new();
    for f in FIELDS {
        if !changed.contains(f) {
            continue;
        }
        if matches!(*f, "stdout" | "stderr" | "exit_code" | "duration_ms") {
            if changed.contains("environment_hash") {
                push_unique(&mut out, "environment_hash");
            }
            if changed.contains("cwd") {
                push_unique(&mut out, "cwd");
            }
            if changed.contains("command") {
                push_unique(&mut out, "command");
            }
        }
        if *f != "timestamp" && *f != "duration_ms" {
            push_unique(&mut out, f);
        }
    }
    out
}

pub fn diff_traces(a: &ExecutionTrace, b: &ExecutionTrace) -> Vec<String> {
    let mut diffs = vec![];

    let max = a.events.len().max(b.events.len());

    for i in 0..max {
        match (a.events.get(i), b.events.get(i)) {
            (Some(x), Some(y)) if x != y => {
                diffs.push(format!("event[{i}] changed: {x:?} → {y:?}"));
            }
            (Some(x), None) => diffs.push(format!("event[{i}] removed: {x:?}")),
            (None, Some(y)) => diffs.push(format!("event[{i}] added: {y:?}")),
            _ => {}
        }
    }

    diffs
}

pub fn diff_graph(a: &CausalGraph, b: &CausalGraph) -> String {
    let mut lines: Vec<String> = Vec::new();
    if a.nodes.len() != b.nodes.len() {
        lines.push(format!(
            "node count: {} vs {}\n",
            a.nodes.len(),
            b.nodes.len()
        ));
    }
    if a.edges.len() != b.edges.len() {
        lines.push(format!(
            "edge count: {} vs {}\n",
            a.edges.len(),
            b.edges.len()
        ));
    }
    let max = a.edges.len().max(b.edges.len());
    for i in 0..max {
        match (a.edges.get(i), b.edges.get(i)) {
            (Some(ea), Some(eb)) if ea != eb => lines.push(format!(
                "edge[{}]: {}|{}|{} vs {}|{}|{}\n",
                i, ea.from, ea.to, ea.relation, eb.from, eb.to, eb.relation
            )),
            (Some(ea), None) => lines.push(format!(
                "edge[{}]: {}|{}|{} vs <missing>\n",
                i, ea.from, ea.to, ea.relation
            )),
            (None, Some(eb)) => lines.push(format!(
                "edge[{}]: <missing> vs {}|{}|{}\n",
                i, eb.from, eb.to, eb.relation
            )),
            _ => {}
        }
    }
    if lines.is_empty() {
        "identical causal graphs\n".to_string()
    } else {
        lines.join("")
    }
}

fn project(a: &ExecutionArtifact, b: &ExecutionArtifact, field: &str) -> (String, String) {
    match field {
        "command" => (a.command.clone(), b.command.clone()),
        "environment_hash" => (a.environment_hash(), b.environment_hash()),
        "cwd" => (a.cwd.clone(), b.cwd.clone()),
        "exit_code" => (a.exit_code.to_string(), b.exit_code.to_string()),
        "stdout" => (a.stdout.clone(), b.stdout.clone()),
        "stderr" => (a.stderr.clone(), b.stderr.clone()),
        "duration_ms" => (a.duration_ms.to_string(), b.duration_ms.to_string()),
        "timestamp" => (a.timestamp.to_string(), b.timestamp.to_string()),
        // Defensive: `changed_fields` is internal; never panic on the CLI path.
        other => (
            format!("<unsupported field: {other}>"),
            format!("<unsupported field: {other}>"),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::capture::{
        capture_command_with_clock, reset_counter_for_tests, FixedClock, CAPTURE_TEST_LOCK,
    };

    #[test]
    fn identical_runs_have_no_differences() {
        let _g = CAPTURE_TEST_LOCK.lock().unwrap();
        reset_counter_for_tests();
        let a = capture_command_with_clock("echo x".to_string(), &FixedClock(1));
        let b = a.clone();
        let r = diff_runs(&a, &b);
        assert!(r.differences.is_empty());
        assert!(r.changed_fields.is_empty());
        assert_eq!(r.unchanged_fields.len(), FIELDS.len());
        assert!(r.causal_chain.is_empty());
    }

    #[test]
    fn different_commands_surface_as_first_field() {
        let _g = CAPTURE_TEST_LOCK.lock().unwrap();
        reset_counter_for_tests();
        let a = capture_command_with_clock("echo a".to_string(), &FixedClock(1));
        let b = capture_command_with_clock("echo b".to_string(), &FixedClock(1));
        let r = diff_runs(&a, &b);
        assert!(!r.differences.is_empty());
        assert_eq!(r.differences[0].field, "command");
        assert_eq!(r.changed_fields[0], "command");
    }

    #[test]
    fn duration_only_differences_filtered_without_stream_or_exit_change() {
        let _g = CAPTURE_TEST_LOCK.lock().unwrap();
        reset_counter_for_tests();
        let a = capture_command_with_clock("echo x".to_string(), &FixedClock(1));
        let mut b = a.clone();
        b.duration_ms = a.duration_ms.saturating_add(999);
        let r = diff_runs(&a, &b);
        assert!(
            r.differences.is_empty(),
            "duration-only drift must be suppressed: {:?}",
            r.differences
        );
    }

    #[test]
    fn timestamp_only_differences_are_filtered_out() {
        let _g = CAPTURE_TEST_LOCK.lock().unwrap();
        reset_counter_for_tests();
        let a = capture_command_with_clock("echo x".to_string(), &FixedClock(1));
        let mut b = a.clone();
        b.timestamp = 99;
        let r = diff_runs(&a, &b);
        assert!(
            r.differences.is_empty(),
            "timestamp-only drift should not surface as a semantic diff: {:?}",
            r.differences
        );
        assert!(r.causal_chain.is_empty());
    }

    #[test]
    fn grouped_diff_subset_of_fields() {
        let _g = CAPTURE_TEST_LOCK.lock().unwrap();
        reset_counter_for_tests();
        let a = capture_command_with_clock("echo a".to_string(), &FixedClock(1));
        let b = capture_command_with_clock("echo b".to_string(), &FixedClock(1));
        let g = diff_environment(&a, &b).len() + diff_execution(&a, &b).len();
        let flat = diff_runs(&a, &b).differences.len();
        assert!(flat >= g);
    }
}
