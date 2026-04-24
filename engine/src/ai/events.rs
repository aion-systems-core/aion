//! Deterministic ordering and canonical serialization for AI timeline events.

use super::trace::{AiTokenEvent, Event};
use aion_kernel::DeterministicRng;
use serde_json::{json, Value};

/// Sort key: logical timestamp, discriminant, tie-break index.
pub fn event_sort_key(e: &Event) -> (u64, u8, u32) {
    match e {
        Event::RunStart { .. } => (0, 0, 0),
        Event::PromptIngested { .. } => (1, 1, 0),
        Event::TokenGenerated { index, .. } => (2u64.saturating_add(*index as u64), 2, *index),
        Event::RunComplete { token_count } => (
            10_000u64.saturating_add(*token_count as u64),
            3,
            *token_count as u32,
        ),
        Event::SyscallCaptured { id, .. } => (50_000u64.saturating_add(*id), 4, 0),
        Event::PolicyViolation { .. } => (90_000, 5, 0),
    }
}

pub fn sort_events_deterministic(events: &[Event]) -> Vec<Event> {
    let mut tagged: Vec<((u64, u8, u32), Event)> = events
        .iter()
        .cloned()
        .map(|e| (event_sort_key(&e), e))
        .collect();
    tagged.sort_by(|a, b| a.0.cmp(&b.0));
    tagged.into_iter().map(|(_, e)| e).collect()
}

pub fn deterministic_event_id(seed: u64, index: u32) -> u64 {
    DeterministicRng::new(seed ^ (index as u64).rotate_left(23)).next_u64()
}

/// Namespace-separate from timeline [`deterministic_event_id`].
pub fn deterministic_token_id(seed: u64, index: u32) -> u64 {
    DeterministicRng::new(seed ^ 0x746f6b656e_u64 ^ (index as u64).rotate_left(11)).next_u64()
}

/// Token trace for hashing: index, token, token_id only (no logits, no logical timestamps).
pub fn canonical_token_trace_for_hash(trace: &[AiTokenEvent]) -> Value {
    let mut rows: Vec<Value> = trace
        .iter()
        .map(|t| {
            json!({
                "index": t.index,
                "token": t.token,
                "token_id": t.token_id,
            })
        })
        .collect();
    rows.sort_by(|a, b| {
        let ia = a.get("index").and_then(|x| x.as_u64()).unwrap_or(0);
        let ib = b.get("index").and_then(|x| x.as_u64()).unwrap_or(0);
        ia.cmp(&ib)
    });
    Value::Array(rows)
}

/// Events for capsule hash: sorted, stable ids from `seed` + stable index.
pub fn canonical_events_for_hash(events: &[Event], seed: u64) -> Value {
    let sorted = sort_events_deterministic(events);
    let mut rows = Vec::new();
    for (i, e) in sorted.iter().enumerate() {
        let id = deterministic_event_id(seed, i as u32);
        let row = match e {
            Event::RunStart { model } => json!({"id": id, "kind": "run_start", "model": model}),
            Event::PromptIngested { chars } => {
                json!({"id": id, "kind": "prompt_ingested", "chars": chars})
            }
            Event::TokenGenerated { index, token } => {
                json!({"id": id, "kind": "token_generated", "index": index, "token": token})
            }
            Event::RunComplete { token_count } => {
                json!({"id": id, "kind": "run_complete", "token_count": token_count})
            }
            Event::SyscallCaptured {
                name,
                args,
                result,
                deterministic,
                ..
            } => json!({
                "id": id,
                "kind": "syscall_captured",
                "name": name,
                "args": args,
                "result": result,
                "deterministic": deterministic,
            }),
            Event::PolicyViolation {
                syscall,
                reason,
                severity,
            } => json!({
                "id": id,
                "kind": "policy_violation",
                "syscall": syscall,
                "reason": reason,
                "severity": severity,
            }),
        };
        rows.push(row);
    }
    Value::Array(rows)
}

/// Structural event comparison for drift (no synthetic ids; order-normalized).
pub fn canonical_events_for_compare(events: &[Event]) -> String {
    let sorted = sort_events_deterministic(events);
    let v = Value::Array(
        sorted
            .iter()
            .map(|e| match e {
                Event::RunStart { model } => json!({"kind": "run_start", "model": model}),
                Event::PromptIngested { chars } => {
                    json!({"kind": "prompt_ingested", "chars": chars})
                }
                Event::TokenGenerated { index, token } => {
                    json!({"kind": "token_generated", "index": index, "token": token})
                }
                Event::RunComplete { token_count } => {
                    json!({"kind": "run_complete", "token_count": token_count})
                }
                Event::SyscallCaptured {
                    name,
                    args,
                    result,
                    deterministic,
                    ..
                } => json!({
                    "kind": "syscall_captured",
                    "name": name,
                    "args": args,
                    "result": result,
                    "deterministic": deterministic,
                }),
                Event::PolicyViolation {
                    syscall,
                    reason,
                    severity,
                } => json!({
                    "kind": "policy_violation",
                    "syscall": syscall,
                    "reason": reason,
                    "severity": severity,
                }),
            })
            .collect(),
    );
    crate::ai::canon::canonical_json_string(&v)
}

/// Strip nondeterministic fields from token events for drift / compare.
pub fn token_events_semantic_equal(a: &[AiTokenEvent], b: &[AiTokenEvent]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut va: Vec<_> = a
        .iter()
        .map(|t| (t.index, t.token.clone(), t.token_id))
        .collect();
    let mut vb: Vec<_> = b
        .iter()
        .map(|t| (t.index, t.token.clone(), t.token_id))
        .collect();
    va.sort_by(|x, y| x.0.cmp(&y.0));
    vb.sort_by(|x, y| x.0.cmp(&y.0));
    va == vb
}
