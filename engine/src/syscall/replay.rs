//! Replay syscall results from captured events (no host re-execution).

use super::capture::SyscallEvent;
use serde_json::Value;

/// Return the recorded syscall result for replay (deterministic substitution).
pub fn replay_syscall_result(ev: &SyscallEvent) -> Value {
    ev.result.clone()
}

/// Verify replayed capsule syscall stream matches originals (order + payload).
pub fn syscall_replay_matches(
    events_orig: &[SyscallEvent],
    events_replay: &[SyscallEvent],
) -> bool {
    events_orig == events_replay
}
