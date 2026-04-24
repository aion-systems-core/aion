//! Drift between two AI capsules — deterministic semantic fields only (noise isolated).

use super::canon::{canonical_graph_v2_json, canonical_why_v2_json};
use super::events::{canonical_events_for_compare, token_events_semantic_equal};
use super::graph::CausalGraphV2;
use super::model::AICapsuleV1;
use super::why::WhyReportV2;
use aion_core::error::{canonical_error_json, code, line};
use aion_core::{DriftReport, DriftToleranceProfile, DriftToleranceViolation, EvidenceChain};

fn graph_edges_only_json(g: &CausalGraphV2) -> String {
    let mut edges: Vec<_> = g.edges.iter().map(|e| (&e.from, &e.to)).collect();
    edges.sort();
    format!("{edges:?}")
}

fn why_edges_only_json(w: &WhyReportV2) -> String {
    let mut edges: Vec<_> = w.edges.iter().map(|e| (&e.from, &e.to)).collect();
    edges.sort();
    format!("{edges:?}")
}

/// Compare capsules on tokens, token trace semantics, ordered events, graph edges, and why causal edges only.
pub fn drift_between_runs(a: &AICapsuleV1, b: &AICapsuleV1) -> DriftReport {
    let profile = DriftToleranceProfile::deterministic_default();
    let mut fields = Vec::new();
    let mut labels = Vec::new();
    let mut violations = Vec::new();
    if a.tokens != b.tokens {
        fields.push("tokens".into());
        labels.push("tokens:sequence_mismatch".into());
        labels.extend(token_delta_labels(&a.tokens, &b.tokens));
        let delta = a.tokens.len().abs_diff(b.tokens.len()) as u64;
        if delta > profile.max_token_delta as u64 {
            violations.push(DriftToleranceViolation {
                label: "tokens:delta_over_limit".into(),
                actual: delta,
                limit: profile.max_token_delta as u64,
            });
        }
    }
    if !token_events_semantic_equal(&a.token_trace, &b.token_trace) {
        fields.push("token_trace".into());
        labels.push("tokens:trace_mismatch".into());
    }
    if canonical_events_for_compare(&a.event_stream)
        != canonical_events_for_compare(&b.event_stream)
    {
        fields.push("event_stream".into());
        labels.push("shape:event_stream_mismatch".into());
        let delta = a.event_stream.len().abs_diff(b.event_stream.len()) as u64;
        if delta > profile.max_event_delta as u64 {
            violations.push(DriftToleranceViolation {
                label: "shape:event_delta_over_limit".into(),
                actual: delta,
                limit: profile.max_event_delta as u64,
            });
        }
    }
    if graph_edges_only_json(&a.graph) != graph_edges_only_json(&b.graph) {
        fields.push("graph_edges".into());
        labels.push("shape:graph_edges_mismatch".into());
    }
    if why_edges_only_json(&a.why) != why_edges_only_json(&b.why) {
        fields.push("why_causal_edges".into());
        labels.push("shape:why_edges_mismatch".into());
    }
    let mut overflow = false;
    fields.sort();
    fields.dedup();
    labels.sort();
    labels.dedup();
    if labels.len() > profile.max_labels {
        labels.truncate(profile.max_labels);
        overflow = true;
        labels.push("other:labels_overflow".into());
    }
    let categories = categories_from_labels(&labels);
    let changed = !labels.is_empty() || !violations.is_empty();
    violations.sort_by(|x, y| x.label.cmp(&y.label));
    let error = if overflow {
        Some(canonical_error_json(
            &line(
                code::DRIFT_OVERFLOW,
                "drift_between_runs",
                "labels_overflow",
            ),
            "drift",
        ))
    } else if !violations.is_empty() {
        Some(canonical_error_json(
            &line(
                code::DRIFT_TOLERANCE,
                "drift_between_runs",
                "tolerance_violation",
            ),
            "drift",
        ))
    } else {
        None
    };
    DriftReport {
        changed,
        categories,
        labels: labels.clone(),
        fields,
        details: labels,
        tolerance_profile: profile,
        tolerance_violations: violations,
        overflow,
        error,
    }
}

/// Replay-oriented drift: original vs replay capsule (deterministic fields only).
pub fn drift_against_original(original: &AICapsuleV1, replay: &AICapsuleV1) -> DriftReport {
    drift_between_runs(original, replay)
}

/// Compare evidence records only (`run_id` on the chain is ignored).
pub fn evidence_chains_equal_relaxed(a: &EvidenceChain, b: &EvidenceChain) -> bool {
    if a.records.len() != b.records.len() {
        return false;
    }
    if !a.records.iter().zip(b.records.iter()).all(|(x, y)| {
        x.seq == y.seq
            && x.kind == y.kind
            && x.leaf_digest == y.leaf_digest
            && x.payload_digest == y.payload_digest
            && x.parent_digest == y.parent_digest
    }) {
        return false;
    }
    match (a.formal_replay_invariant_ok, b.formal_replay_invariant_ok) {
        (Some(x), Some(y)) if x != y => return false,
        _ => {}
    }
    match (a.cross_machine_replay_ok, b.cross_machine_replay_ok) {
        (Some(x), Some(y)) if x != y => return false,
        _ => {}
    }
    true
}

/// Full structural drift (includes graph / why canonical JSON) for tooling that needs it.
pub fn drift_between_runs_full(a: &AICapsuleV1, b: &AICapsuleV1) -> DriftReport {
    let profile = DriftToleranceProfile::deterministic_default();
    let mut fields = Vec::new();
    let mut labels = Vec::new();
    let mut violations = Vec::new();
    if a.version != b.version {
        fields.push("version".into());
        labels.push("shape:version_mismatch".into());
    }
    if a.prompt != b.prompt {
        fields.push("prompt".into());
        labels.push("model:prompt_mismatch".into());
    }
    if a.model != b.model {
        fields.push("model".into());
        labels.push("model:model_name_mismatch".into());
    }
    if a.seed != b.seed {
        fields.push("seed".into());
        labels.push("model:seed_mismatch".into());
    }
    if a.determinism != b.determinism {
        fields.push("determinism".into());
        labels.push("timing:determinism_profile_mismatch".into());
    }
    let epoch_delta = a
        .determinism
        .time_epoch_secs
        .abs_diff(b.determinism.time_epoch_secs);
    if epoch_delta > profile.max_timing_delta_ms {
        violations.push(DriftToleranceViolation {
            label: "timing:epoch_delta_over_limit".into(),
            actual: epoch_delta,
            limit: profile.max_timing_delta_ms,
        });
    }
    let d = drift_between_runs(a, b);
    fields.extend(d.fields);
    labels.extend(d.labels);
    violations.extend(d.tolerance_violations);
    if canonical_why_v2_json(&a.why) != canonical_why_v2_json(&b.why) {
        fields.push("why".into());
        labels.push("shape:why_canonical_mismatch".into());
    }
    if canonical_graph_v2_json(&a.graph) != canonical_graph_v2_json(&b.graph) {
        fields.push("graph".into());
        labels.push("shape:graph_canonical_mismatch".into());
    }
    let mut overflow = d.overflow;
    fields.sort();
    fields.dedup();
    labels.sort();
    labels.dedup();
    if labels.len() > profile.max_labels {
        labels.truncate(profile.max_labels);
        overflow = true;
        labels.push("other:labels_overflow".into());
    }
    violations.sort_by(|x, y| x.label.cmp(&y.label));
    let categories = categories_from_labels(&labels);
    let changed = !labels.is_empty() || !violations.is_empty();
    let error = if overflow {
        Some(canonical_error_json(
            &line(
                code::DRIFT_OVERFLOW,
                "drift_between_runs_full",
                "labels_overflow",
            ),
            "drift",
        ))
    } else if !violations.is_empty() {
        Some(canonical_error_json(
            &line(
                code::DRIFT_TOLERANCE,
                "drift_between_runs_full",
                "tolerance_violation",
            ),
            "drift",
        ))
    } else {
        None
    };
    DriftReport {
        changed,
        categories,
        labels: labels.clone(),
        fields,
        details: labels,
        tolerance_profile: profile,
        tolerance_violations: violations,
        overflow,
        error,
    }
}

fn categories_from_labels(labels: &[String]) -> Vec<String> {
    let ordered = ["shape", "tokens", "timing", "model", "evidence", "other"];
    let mut out = Vec::new();
    for cat in ordered {
        if labels.iter().any(|l| l.starts_with(&format!("{cat}:"))) {
            out.push(cat.to_string());
        }
    }
    out
}

fn token_delta_labels(a: &[String], b: &[String]) -> Vec<String> {
    let n = a.len().max(b.len());
    let mut out = Vec::new();
    for i in 0..n {
        let l = a.get(i).map(String::as_str).unwrap_or("<missing>");
        let r = b.get(i).map(String::as_str).unwrap_or("<missing>");
        if l != r {
            out.push(format!(
                "tokens:token_at:{}:{}:{}",
                i,
                token_label(l),
                token_label(r)
            ));
        }
    }
    out
}

fn token_label(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect()
}
