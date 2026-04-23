//! Deterministic syscall policy and capture / replay.

use aion_core::DeterministicIOPolicy;
use aion_engine::ai::Event;
use aion_engine::syscall::{capture_syscall, replay_syscall_result, syscall_replay_matches};
use serde_json::json;

#[test]
fn syscall_whitelist_allows_read() {
    let mut violations = Vec::new();
    let ev = capture_syscall(
        DeterministicIOPolicy::Strict,
        1,
        "read",
        json!({"fd": 0, "len": 4}),
        json!({"bytes_read": 4}),
        &mut violations,
    )
    .expect("read allowed");
    assert!(violations.is_empty());
    assert_eq!(ev.name, "read");
}

#[test]
fn syscall_policy_blocks_write_open() {
    let mut violations = Vec::new();
    let err = capture_syscall(
        DeterministicIOPolicy::Deny,
        2,
        "open",
        json!({"path": "C:\\\\tmp\\\\x", "flags": "O_RDWR"}),
        json!({"fd": -1}),
        &mut violations,
    )
    .expect_err("write-capable open must be denied");
    assert!(!err.is_empty());
    assert!(
        violations
            .iter()
            .any(|e| matches!(e, Event::PolicyViolation { .. })),
        "expected policy_violation event"
    );
}

#[test]
fn syscall_replay_matches_original() {
    let mut v = Vec::new();
    let a = capture_syscall(
        DeterministicIOPolicy::Audit,
        1,
        "stat",
        json!({"path": "a//b"}),
        json!({"size": 10}),
        &mut v,
    )
    .unwrap();
    assert_eq!(
        a.args.get("path").and_then(|x| x.as_str()),
        Some("a/b"),
        "path canonicalized"
    );
    let b = a.clone();
    assert_eq!(replay_syscall_result(&a), json!({"size": 10}));
    assert!(syscall_replay_matches(&[a], &[b]));
}
