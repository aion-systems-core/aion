//! Deterministic COS core surface.
//!
//! **Single source of truth** for kernel audit records, replay scaffolding, and
//! evidence chain types. Downstream crates must not duplicate these definitions.
//!
//! This crate is intentionally small and dependency-light: no `cognitive_os_v14`,
//! no embedded repro tool, and no I/O-heavy dependencies.

#![forbid(unsafe_code)]

pub mod audit;
pub mod evidence;
pub mod replay;
