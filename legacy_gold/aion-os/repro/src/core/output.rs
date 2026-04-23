// Terminal output formatting.
//
// All user-visible text flows through this module. That is what keeps
// output stable: if anyone wants to change how a run "looks", they
// have to change it here, and the determinism invariant tests in
// `core::contract` and `tests/e2e.rs` will catch accidental drift.
//
// Rules:
//   * no colors, no box-drawing, no unicode bullets — pure ASCII so
//     the output is identical on every terminal and easy to snapshot.
//   * every multi-line value is rendered with a fixed prefix so it
//     can never be confused with structural lines.
//   * field order mirrors `diff::FIELDS` where relevant.
//   * root-cause output is a single semantic block (no duplicate
//     divergence headlines).

use crate::core::artifact::ExecutionArtifact;
use crate::core::diff::DiffReport;
use crate::core::execution_trace::ExecutionTrace;
use crate::core::report::{ExecutionReport, RootCauseSummary};
use crate::core::root_cause::RootCause;

pub fn format_artifact(a: &ExecutionArtifact) -> String {
    let mut s = String::new();
    s.push_str(&format!("AION run_id      : {}\n", a.run_id));
    s.push_str(&format!("command          : {}\n", a.command));
    s.push_str(&format!("exit_code        : {}\n", a.exit_code));
    s.push_str(&format!("duration_ms      : {}\n", a.duration_ms));
    let env_n = a.repro_run.as_ref().map(|r| r.env.len()).unwrap_or(0);
    s.push_str(&format!(
        "env_captured     : {} variables (full map in repro_runs/{}.json)\n",
        env_n, a.run_id
    ));
    s.push_str("stdout           :\n");
    push_block(&mut s, &a.stdout);
    s.push_str("stderr           :\n");
    push_block(&mut s, &a.stderr);
    s
}

/// Pretty-print an [`ExecutionTrace`] (optional extra; not part of the pinned replay block).
pub fn format_trace(trace: &ExecutionTrace) -> String {
    let mut s = String::new();
    s.push_str("── execution trace ──\n");
    s.push_str(&format!("AION run_id: {}\n", trace.run_id));
    for (i, ev) in trace.events.iter().enumerate() {
        s.push_str(&format!("  [{i}] {ev:?}\n"));
    }
    s
}

pub fn format_diff(report: &DiffReport) -> String {
    let mut s = String::new();
    s.push_str("── AION DIFF ──\n");
    s.push_str(&format!("epoch_a AION run_id: {}\n", report.run_a));
    s.push_str(&format!("epoch_b AION run_id: {}\n", report.run_b));

    let has_semantic = !report.differences.is_empty() || !report.aion.env_diff.is_empty();
    if !has_semantic {
        s.push_str("no differences\n");
        return s;
    }

    if let Some(ref t) = report.aion.temporal_context {
        s.push_str("AION EPOCH context:\n");
        s.push_str(&format!(
            "  AION EPOCH (older): {}\n  AION EPOCH (newer): {}\n",
            t.older_run_id, t.newer_run_id
        ));
        s.push('\n');
    }

    for e in &report.aion.env_diff {
        s.push_str(&format!("~ {}\n", e.key));
        s.push_str(&format!("  epoch_a: {}\n", e.value_a));
        s.push_str(&format!("  epoch_b: {}\n", e.value_b));
        s.push_str("\n→ CAUSAL SHIFT:\n");
        s.push_str(&format!("  {} caused execution divergence\n", e.key));
        s.push('\n');
    }

    if !report.aion.env_diff.is_empty() {
        s.push_str("── AION CAUSAL LAYER ──\n");
        s.push_str("AGE SHIFT DETECTED:\n");
        for e in &report.aion.env_diff {
            s.push_str(&format!("  {} changed across AION epochs\n", e.key));
        }
        s.push('\n');
    }

    let skip_env_hash = !report.aion.env_diff.is_empty();
    for d in &report.differences {
        if d.field == "timestamp" || d.field == "duration_ms" {
            continue;
        }
        if skip_env_hash && d.field == "environment_hash" {
            continue;
        }
        s.push_str(&format!("~ {}\n", d.field));
        s.push_str("  a:\n");
        push_block_indented(&mut s, &d.a, "    ");
        s.push_str("  b:\n");
        push_block_indented(&mut s, &d.b, "    ");
    }

    if !report.causal_chain.is_empty() {
        s.push_str("causal chain:\n");
        for step in &report.causal_chain {
            s.push_str(&format!("  -> {}\n", step));
        }
    }
    s
}

/// Legacy formatter for the `RootCause` enum. Still used by any caller
/// that hasn't switched to `format_root_cause_summary`, and by the
/// `root-cause` CLI handler to preserve existing output strings.
#[allow(dead_code)] // kept for back-compat of the legacy RootCause enum
pub fn format_root_cause(rc: &RootCause) -> String {
    match rc {
        RootCause::NoPrevious => "no previous run; nothing to compare\n".to_string(),
        RootCause::Identical { previous } => {
            format!("no divergence from previous run {previous}\n")
        }
        RootCause::Divergence {
            previous,
            field,
            previous_value,
            current_value,
        } => {
            let mut s = String::new();
            s.push_str(&format!("divergence field: {field}\n"));
            s.push_str(&format!("previous run: {previous}\n"));
            s.push_str("previous value:\n");
            push_block_indented(&mut s, previous_value, "  ");
            s.push_str("current value:\n");
            push_block_indented(&mut s, current_value, "  ");
            s
        }
    }
}

/// Structured root-cause formatter: previous run, primary semantic
/// cause, and all diverging fields (single narrative).
pub fn format_root_cause_summary(rc: &RootCauseSummary) -> String {
    let mut s = String::new();
    s.push_str("── root cause ──\n");

    match (&rc.previous_run, &rc.primary, &rc.first_diverging_field) {
        (None, _, _) => {
            s.push_str("no previous run; nothing to compare\n");
            s
        }
        (Some(prev), None, None) => {
            s.push_str(&format!("no divergence from previous run {prev}\n"));
            s
        }
        (Some(prev), primary, _first) => {
            s.push_str(&format!("previous run: {prev}\n"));
            if let Some(p) = primary {
                s.push_str(&format!("primary cause: {}\n", p.field));
                s.push_str(&format!("category     : {}\n", p.category.as_str()));
                s.push_str(&format!("severity     : {}\n", p.severity.as_str()));
                s.push_str("explanation  :\n");
                push_block_indented(&mut s, &p.explanation, "  ");
                s.push_str("previous value:\n");
                push_block_indented(&mut s, &p.previous_value, "  ");
                s.push_str("current value:\n");
                push_block_indented(&mut s, &p.current_value, "  ");
            }
            if !rc.causes.is_empty() {
                s.push_str("~ diverged fields:\n");
                for c in &rc.causes {
                    s.push_str(&format!(
                        "  - {:<16} ({}, {})\n",
                        c.field,
                        c.category.as_str(),
                        c.severity.as_str()
                    ));
                }
            }
            s
        }
    }
}

/// Render an `ExecutionReport` to a single deterministic string.
/// Sections are emitted in the order they appear on the struct so the
/// rendering is a straight projection of the data. Available for
/// tooling that wants to render a full report; individual handlers
/// still use the targeted `format_*` functions above.
#[allow(dead_code)] // public integration point; used by downstream tooling
pub fn format_report(r: &ExecutionReport) -> String {
    let mut s = String::new();

    s.push_str("── execution report ──\n");
    s.push_str(&format!("AION run_id          : {}\n", r.trace.run_id));
    s.push_str(&format!("command              : {}\n", r.trace.command));
    s.push_str(&format!("exit_code            : {}\n", r.trace.exit_code));
    s.push_str(&format!("duration_ms          : {}\n", r.trace.duration_ms));
    s.push_str(&format!(
        "stdout_bytes         : {}\n",
        r.trace.stdout_bytes
    ));
    s.push_str(&format!(
        "stderr_bytes         : {}\n",
        r.trace.stderr_bytes
    ));

    s.push_str("── identity ──\n");
    s.push_str(&format!(
        "command_signature    : {}\n",
        r.identity.command_signature
    ));
    s.push_str(&format!(
        "environment_signature: {}\n",
        r.identity.environment_signature
    ));
    s.push_str(&format!(
        "input_signature      : {}\n",
        r.identity.input_signature
    ));
    s.push_str(&format!(
        "trace_signature      : {}\n",
        r.identity.trace_signature
    ));
    s.push_str(&format!(
        "composite            : {}\n",
        r.identity.composite
    ));

    if let Some(d) = &r.diff {
        s.push_str("── diff summary ──\n");
        s.push_str(&format!("a: {}\n", d.run_a));
        s.push_str(&format!("b: {}\n", d.run_b));
        s.push_str(&format!(
            "identity_delta       : command={} environment={} input={} trace={}\n",
            d.identity_delta.command_changed,
            d.identity_delta.environment_changed,
            d.identity_delta.input_changed,
            d.identity_delta.trace_changed,
        ));
        if d.differences.is_empty() {
            s.push_str("no differences\n");
        } else {
            for diff in &d.differences {
                if diff.field == "timestamp" || diff.field == "duration_ms" {
                    continue;
                }
                s.push_str(&format!("~ {}\n", diff.field));
            }
            if !d.causal_chain.is_empty() {
                s.push_str("causal_chain:\n");
                for step in &d.causal_chain {
                    s.push_str(&format!("  -> {}\n", step));
                }
            }
        }
    }

    if let Some(rc) = &r.root_cause {
        s.push_str(&format_root_cause_summary(rc));
    }

    s
}

fn push_block(s: &mut String, value: &str) {
    push_block_indented(s, value, "  | ")
}

fn push_block_indented(s: &mut String, value: &str, prefix: &str) {
    if value.is_empty() {
        s.push_str(prefix);
        s.push_str("<empty>\n");
        return;
    }
    for line in value.lines() {
        s.push_str(prefix);
        s.push_str(line);
        s.push('\n');
    }
    // Preserve the trailing-newline distinction without letting it mutate
    // the last rendered line: if the source ended with '\n' we already
    // rendered every logical line; if it didn't, mark it explicitly so
    // two artifacts that differ only in trailing newline still diff.
    if !value.ends_with('\n') {
        s.push_str(prefix);
        s.push_str("<no trailing newline>\n");
    }
}
