//! SealRun **engine** (`aion-engine`): builds capsules, replay/drift/evidence pipelines, SDK helpers, and filesystem artefact writers used by the `sealrun` CLI.
//!
//! The crate stays intentionally explicit about deterministic ordering for JSON and audit-facing outputs.

pub mod ai;
pub mod audit;
pub mod capsule;
pub mod capture;
pub mod ci;
pub mod diff;
pub mod events;
#[cfg(feature = "ffi")]
pub mod ffi;
pub mod governance;
pub mod graph;
pub mod output;
pub mod policy;
pub mod replay;
pub mod replay_debug;
pub mod runtime;
pub mod sdk;
pub mod syscall;
pub mod trace;
pub mod why;
