//! `repro` library surface (same modules as the binary). Used by integration
//! tests and any embedder; the CLI binary is a thin `main` wrapper.
//!
//! **Boundary:** subprocess capture, diff, and root-cause live in [`core`].
//! Shared platform types used by deeper integrations are re-exported for
//! embedders that link the full stack.

pub use cos_core;

pub mod analysis;
pub mod ci;
pub mod cli;
pub mod core;
pub mod engine;
