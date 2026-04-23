//! Build [`Trace`](super::model::Trace) from a [`RunResult`](aion_core::RunResult) via EventStore v2.

use super::model::{Trace, TraceSpan};
use crate::events::{envelopes_from_run, EventEnvelope};
use aion_core::RunResult;
use serde_json::json;
use sha2::{Digest, Sha256};

fn surface(envelope: &EventEnvelope) -> String {
    let v = json!({ "event": &envelope.event, "attrs": &envelope.attrs });
    let bytes = serde_json::to_vec(&v).unwrap_or_default();
    let h = Sha256::digest(&bytes);
    let top: [u8; 8] = h[..8].try_into().unwrap_or([0u8; 8]);
    format!("{:016x}", u64::from_be_bytes(top))
}

pub fn trace_from_run(run: &RunResult) -> Trace {
    let envs = envelopes_from_run(run, None);
    let spans: Vec<TraceSpan> = envs
        .into_iter()
        .map(|e| TraceSpan {
            seq: e.seq,
            op: format!("{:?}", e.event.category()),
            surface: surface(&e),
        })
        .collect();
    Trace {
        run_id: run.run_id.clone(),
        spans,
    }
}
