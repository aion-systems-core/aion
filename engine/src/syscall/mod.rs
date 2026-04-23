//! Deterministic syscall policy, capture, and replay helpers.

pub mod capture;
pub mod policy;
pub mod replay;

pub use capture::{capture_syscall, SyscallEvent};
pub use policy::{
    canonicalize_fs_path, evaluate_syscall, open_is_read_only, policy_violation_value,
    should_block, SyscallName,
};
pub use replay::{replay_syscall_result, syscall_replay_matches};
