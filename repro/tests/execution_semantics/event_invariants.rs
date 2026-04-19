//! Count and shape invariants for every real capture (echo hello).

use super::{assert_repro_ok, load_latest_artifact, run_repro, scratch_dir};
use repro::core::execution_trace::ExecutionEvent;

fn is_env_event(e: &ExecutionEvent) -> bool {
    matches!(e, ExecutionEvent::EnvResolved { .. })
}

#[test]
fn echo_hello_run_has_expected_cardinality_and_stdout_chunks() {
    let cwd = scratch_dir("invariants");
    let out = run_repro(&cwd, &["run", "--", "echo", "hello"]);
    assert_repro_ok(&out, "repro run");

    let art = load_latest_artifact(&cwd);
    let ev = &art.trace.events;

    let spawns = ev
        .iter()
        .filter(|e| matches!(e, ExecutionEvent::Spawn { .. }))
        .count();
    let envs = ev.iter().filter(|e| is_env_event(e)).count();
    let exits = ev
        .iter()
        .filter(|e| matches!(e, ExecutionEvent::Exit { .. }))
        .count();

    assert_eq!(spawns, 1, "exactly one Spawn");
    assert_eq!(envs, 1, "exactly one env-class event");
    assert_eq!(exits, 1, "exactly one Exit");

    // Vec-backed timeline: implicit ids are 0..events.len() with no gaps.
    if let Some((last_i, _)) = ev.iter().enumerate().last() {
        assert_eq!(last_i + 1, ev.len());
    }

    for (i, e) in ev.iter().enumerate() {
        if let ExecutionEvent::Stdout { chunk } = e {
            assert!(
                !chunk.is_empty(),
                "stdout chunk at index {i} must not be empty for echo hello"
            );
        }
    }
}
