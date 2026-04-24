//! SealRun **kernel** (`aion-kernel`): in-process subprocess execution and deterministic environment capture primitives consumed by the engine/CLI stack.

mod env;
mod envelope;
mod execute;
mod fs;
mod integrity;
mod network;
mod random;
mod time;

pub use env::{env_fingerprint, filtered_env_for_child};
pub use envelope::{
    capture_execution_envelope, capture_machine_fingerprint, freeze_cwd, freeze_env, freeze_random,
    freeze_time_ms, MachineFingerprint,
};
pub use execute::{cwd_string, join_command, path_exists, run_command};
pub use fs::{snapshot_cwd_stub, FsPolicy};
pub use integrity::{evaluate_and_enforce, full_report, self_integrity_hash, IntegrityReport};
pub use network::apply_net_policy_stub;
pub use random::DeterministicRng;
pub use time::{now_secs, FrozenClock};
