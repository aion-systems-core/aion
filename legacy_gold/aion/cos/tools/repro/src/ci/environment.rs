//! Re-export of the core 15% environment slice (single source of truth).

pub use crate::core::execution_boundary::{
    capture_env_snapshot_15 as capture_snapshot, compute_env_hash as compute_environment_hash,
    EnvSnapshot15 as EnvironmentSnapshot,
};
