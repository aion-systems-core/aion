//! Replay symmetry invariants for deterministic capsule equivalence.
//!
//! This module defines replay-equality checks used by formal replay validation.
//! Invariant: canonical graph/why/drift and raw trace/events must match exactly.
//! Result errors identify which deterministic invariant failed.

use crate::ai::canon::{canonical_drift_json, canonical_graph_v2_json};
use crate::ai::{AICapsuleV1, WhyReportV2};
use crate::capsule::deterministic_capsule_hash;
use serde_json::json;

/// Canonicalize the deterministic subset of a `WhyReportV2`.
///
/// Purpose: compare replay explainability payload without serialization-order noise.
/// Invariant: only stable fields (`schema`, `seed`, `nodes`, `edges`, `summary`) are included.
/// I/O: `&WhyReportV2` -> canonical JSON `String`.
/// Determinism: fixed key set + canonical JSON stringification.
fn why_deterministic_slice(w: &WhyReportV2) -> String {
    let v = json!({
        "why_schema_version": w.why_schema_version,
        "seed": w.seed,
        "nodes": w.nodes,
        "edges": w.edges,
        "summary": w.summary,
    });
    crate::ai::canon::canonical_json_string(&v)
}

/// Assert replay symmetry across deterministic capsule payload fields.
///
/// Purpose: guard replay correctness before higher-level formal checks.
/// Invariant: token trace, events, canonical graph/why/drift, and determinism profile must be equal.
/// I/O: `(original, replayed)` capsule pair -> `Ok(())` or mismatch token in `Err(String)`.
/// Determinism: comparisons are pure and canonicalized where needed.
pub fn assert_replay_symmetry(original: &AICapsuleV1, replayed: &AICapsuleV1) -> Result<(), String> {
    if original.version != replayed.version {
        return Err("replay:invariant_failed".into());
    }
    if original.model != replayed.model || original.prompt != replayed.prompt {
        return Err("replay:invariant_failed".into());
    }
    if original.seed != replayed.seed {
        return Err("replay:invariant_failed".into());
    }
    if canonical_graph_v2_json(&original.graph) != canonical_graph_v2_json(&replayed.graph) {
        return Err("replay:invariant_failed".into());
    }
    if original.token_trace != replayed.token_trace {
        return Err("replay:symmetry_failed".into());
    }
    if original.event_stream != replayed.event_stream {
        return Err("replay:event_stream_mismatch".into());
    }
    if why_deterministic_slice(&original.why) != why_deterministic_slice(&replayed.why) {
        return Err("replay:why_slice_mismatch".into());
    }
    if canonical_drift_json(&original.drift) != canonical_drift_json(&replayed.drift) {
        return Err("replay:invariant_failed".into());
    }
    if original.determinism != replayed.determinism {
        return Err("replay:profile_mismatch".into());
    }
    Ok(())
}

/// Assert formal replay invariant including deterministic hash equality.
///
/// Purpose: extend symmetry checks with final digest identity proof.
/// Invariant: `assert_replay_symmetry` holds and `deterministic_capsule_hash` values are identical.
/// I/O: `(original, replayed)` capsule pair -> `Ok(())` or formal invariant failure token.
/// Determinism: hash input is canonical semantic payload, yielding stable digest comparison.
pub fn assert_formal_replay_invariant(original: &AICapsuleV1, replayed: &AICapsuleV1) -> Result<(), String> {
    assert_replay_symmetry(original, replayed)?;
    if deterministic_capsule_hash(original) != deterministic_capsule_hash(replayed) {
        return Err("replay:invariant_failed".into());
    }
    Ok(())
}
