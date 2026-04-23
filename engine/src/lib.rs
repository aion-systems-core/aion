//! AION engine: orchestrates kernel primitives into product operations.

pub mod ai;
pub mod audit;
pub mod capsule;
pub mod capture;
pub mod ci;
pub mod diff;
pub mod events;
#[cfg(feature = "ffi")]
pub mod ffi;
pub mod graph;
pub mod governance;
pub mod output;
pub mod policy;
pub mod replay;
pub mod replay_debug;
pub mod runtime;
pub mod sdk;
pub mod syscall;
pub mod trace;
pub mod why;
