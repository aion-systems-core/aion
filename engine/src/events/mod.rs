//! Event store engine: deterministic-ordered timeline for runs and capsules.
//!
//! Used by replay (stdout/stderr projection), diff/why (canonical line projection),
//! and graph (sequential chain). No imports from legacy trees.

mod model;
mod reader;
mod store;

pub use model::{
    envelopes_from_run, EventCategory, EventEnvelope, EventStreamFile, RunEvent,
    EVENT_STREAM_SCHEMA_V2,
};
pub use reader::EventReader;
pub use store::EventStore;

use aion_core::{CapsuleManifest, DriftReport, RunResult, WhyReport};

/// Replay stdout strictly from ordered `StdoutChunk` events.
pub fn replay_stdout_from_store(store: &EventStore) -> String {
    EventReader::new(store).replay_stdout()
}

/// Replay stderr from ordered `StderrChunk` events.
pub fn replay_stderr_from_store(store: &EventStore) -> String {
    EventReader::new(store).replay_stderr()
}

/// Graph JSON (nodes + linear edges) for tooling.
pub fn graph_json_from_store(store: &EventStore) -> Result<String, String> {
    EventReader::new(store).to_graph_json()
}

/// Diff two stores using stable per-envelope JSON lines.
pub fn diff_event_stores(left: &EventStore, right: &EventStore) -> DriftReport {
    EventReader::diff_summaries(&EventReader::new(left), &EventReader::new(right))
}

/// Pair “why” hint from event stream shape alone.
pub fn why_event_stores(actual: &EventStore, baseline: &EventStore) -> WhyReport {
    EventReader::new(actual).why_against(&EventReader::new(baseline))
}

/// Build a store from a [`RunResult`] and optional capsule manifest tail.
pub fn store_from_run(run: &RunResult, manifest: Option<&CapsuleManifest>) -> EventStore {
    EventStore::from_run_result(run, manifest)
}
