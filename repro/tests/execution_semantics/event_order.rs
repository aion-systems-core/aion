//! Locked event order for real `repro run` captures.

use super::{assert_repro_ok, load_latest_artifact, run_repro, scratch_dir};
use repro::core::execution_trace::ExecutionEvent;

fn is_env_event(e: &ExecutionEvent) -> bool {
    matches!(e, ExecutionEvent::EnvResolved { .. })
}

#[test]
fn spawn_env_streams_exit_timing_order() {
    let cwd = scratch_dir("event_order");
    let out = run_repro(&cwd, &["run", "--", "echo", "hello"]);
    assert_repro_ok(&out, "repro run");

    let art = load_latest_artifact(&cwd);
    let ev = &art.trace.events;
    assert!(!ev.is_empty(), "trace must have events");

    assert!(
        matches!(ev.first(), Some(ExecutionEvent::Spawn { .. })),
        "event[0] must be Spawn, got {:?}",
        ev.first()
    );

    let env_positions: Vec<usize> = ev.iter().enumerate().filter(|(_, e)| is_env_event(e)).map(|(i, _)| i).collect();
    assert_eq!(
        env_positions.len(),
        1,
        "exactly one EnvResolved/EnvSnapshot event allowed, positions {env_positions:?}"
    );
    assert_eq!(env_positions[0], 1, "env event must be at index 1");

    let last = ev.len() - 1;
    let second_last = last.saturating_sub(1);
    assert!(
        matches!(ev[second_last], ExecutionEvent::Exit { .. }),
        "last-1 must be Exit, got {:?}",
        ev[second_last]
    );
    assert!(
        matches!(ev[last], ExecutionEvent::Timing { .. }),
        "last must be Timing when present, got {:?}",
        ev[last]
    );

    for i in 2..second_last {
        assert!(
            matches!(ev[i], ExecutionEvent::Stdout { .. } | ExecutionEvent::Stderr { .. }),
            "between env and exit only Stdout/Stderr allowed at index {i}: {:?}",
            ev[i]
        );
    }
}
