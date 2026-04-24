//! Hash pipeline for deterministic capsule identity in SealRun.
//!
//! This module hashes canonical semantic payload only (no timestamp/path/envelope noise).
//! Invariant: equal deterministic semantics map to the same 32-byte digest.
//! Digest construction is byte-stable across machines and runs.

use crate::ai::canon::{canonical_drift_json, canonical_graph_v2_json, canonical_json_string};
use crate::ai::{canonical_events_for_hash, canonical_token_trace_for_hash, AICapsuleV1};
use aion_core::DeterminismProfile;
use serde::Serialize;
use serde_json::{json, Value};

#[derive(Serialize)]
struct HashPayload<'a> {
    prompt: &'a str,
    seed: u64,
    determinism_profile: &'a DeterminismProfile,
    token_trace: Value,
    events: Value,
    graph: Value,
    why_report: Value,
    drift_report: Value,
}

/// Compute the deterministic 32-byte capsule digest.
///
/// Purpose: provide replay/hash anchor for formal comparison and integrity checks.
/// Invariant: only semantic fields (prompt/seed/determinism/trace/events/graph/why/drift) contribute.
/// I/O: `&AICapsuleV1` -> `[u8; 32]` BLAKE3 digest bytes.
/// Determinism: canonical JSON payload + fixed field set + fixed byte encoding before hashing.
pub fn deterministic_capsule_hash(c: &AICapsuleV1) -> [u8; 32] {
    let token_trace = canonical_token_trace_for_hash(&c.token_trace);
    let events = canonical_events_for_hash(&c.event_stream, c.seed);
    let graph: Value =
        serde_json::from_str(&canonical_graph_v2_json(&c.graph)).unwrap_or(json!({}));
    let why_report: Value = json!({
        "why_schema_version": c.why.why_schema_version,
        "seed": c.why.seed,
        "nodes": c.why.nodes,
        "edges": c.why.edges,
        "summary": c.why.summary,
    });
    let drift: Value = serde_json::from_str(&canonical_drift_json(&c.drift)).unwrap_or(json!({}));
    let payload = HashPayload {
        prompt: &c.prompt,
        seed: c.seed,
        determinism_profile: &c.determinism,
        token_trace,
        events,
        graph,
        why_report,
        drift_report: drift,
    };
    let v = serde_json::to_value(&payload).unwrap_or(Value::Null);
    let s = canonical_json_string(&v);
    *blake3::hash(s.as_bytes()).as_bytes()
}
