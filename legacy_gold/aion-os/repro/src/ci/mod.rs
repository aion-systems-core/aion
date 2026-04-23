// CI execution ledger: capture, fingerprint, store, diff, explain.
//
// **CI execution is immutable truth** — see `schema.rs` for versioned
// contracts. This module is intentionally separate from `core::` so CI
// layout (`./repro_ci_store/`) can evolve without breaking `repro_runs/`.

pub mod baseline;
pub mod ci_orchestrator;
pub mod ci_runtime;
pub mod diff;
pub mod environment;
pub mod failure_detector;
pub mod meta;
pub mod root_cause;
pub mod schema;
pub mod storage;
