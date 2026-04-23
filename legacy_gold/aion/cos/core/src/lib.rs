//! Deterministic COS core surface (Phase 1).
//!
//! **Single source of truth** for kernel audit records, replay scaffolding, and
//! evidence chain v2 stubs (moved from `cognitive_os_v14`). Downstream crates
//! must not duplicate these types.
//!
//! See `WAVE1_SCOPE.md` for graph/proof status (still in `cos-v1` until a coordinated move).
//!
//! **Must not** depend on `cognitive_os_v14`, `cos/tools/repro`, or I/O-heavy crates.

#![forbid(unsafe_code)]

pub mod audit;
pub mod evidence;
pub mod replay;
