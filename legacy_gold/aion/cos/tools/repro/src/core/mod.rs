// Core engine: pure, deterministic, filesystem-local.
// CLI layer depends on this; this layer depends on nothing above it.

pub mod artifact;
pub mod capture;
pub mod causal_graph;
pub mod causal_query;
pub mod contract;
pub mod diff;
pub mod event_store;
pub mod execution_boundary;
pub mod execution_trace;
pub mod identity;
pub mod output;
pub mod replay;
pub mod report;
pub mod root_cause;
pub mod storage;
pub mod why_engine;
