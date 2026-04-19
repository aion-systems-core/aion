// Phase 2: event timeline invariants (trace + diff_traces).

use repro::core::capture::{
    capture_command_with_clock, reset_counter_for_tests, FixedClock, CAPTURE_TEST_LOCK,
};
use repro::core::diff::diff_traces;
use repro::core::execution_trace::{ExecutionEvent, ExecutionTrace};

#[test]
fn two_identical_captures_have_identical_event_sequences() {
    let _g = CAPTURE_TEST_LOCK.lock().unwrap();
    reset_counter_for_tests();
    let a = capture_command_with_clock("echo hello".into(), &FixedClock(42));
    reset_counter_for_tests();
    let b = capture_command_with_clock("echo hello".into(), &FixedClock(42));
    assert_eq!(a.trace.events.len(), b.trace.events.len());
    for (ea, eb) in a.trace.events.iter().zip(b.trace.events.iter()) {
        match (ea, eb) {
            (ExecutionEvent::Timing { .. }, ExecutionEvent::Timing { .. }) => {}
            _ => assert_eq!(ea, eb),
        }
    }
}

#[test]
fn env_change_surfaces_at_env_resolved_event_index() {
    let base_tail = [
        ExecutionEvent::Stdout {
            chunk: "ok\n".into(),
        },
        ExecutionEvent::Stderr {
            chunk: String::new(),
        },
        ExecutionEvent::Exit { code: 0 },
        ExecutionEvent::Timing { duration_ms: 1 },
    ];

    let mut ev_a = vec![
        ExecutionEvent::Spawn {
            command: "echo hi".into(),
        },
        ExecutionEvent::EnvResolved {
            keys: vec!["PATH=/a".into(), "HOME=/h".into()],
        },
    ];
    ev_a.extend_from_slice(&base_tail);

    let mut ev_b = vec![
        ExecutionEvent::Spawn {
            command: "echo hi".into(),
        },
        ExecutionEvent::EnvResolved {
            keys: vec!["PATH=/b".into(), "HOME=/h".into()],
        },
    ];
    ev_b.extend_from_slice(&base_tail);

    let t_a = ExecutionTrace {
        run_id: "r1".into(),
        events: ev_a,
    };
    let t_b = ExecutionTrace {
        run_id: "r2".into(),
        events: ev_b,
    };

    let d = diff_traces(&t_a, &t_b);
    assert_eq!(d.len(), 1, "{d:?}");
    assert!(d[0].starts_with("event[1]"), "{}", d[0]);
    assert!(d[0].contains("EnvResolved"), "{}", d[0]);
}

#[test]
fn stdout_only_diff_points_at_stdout_event() {
    let tail = [
        ExecutionEvent::Stderr {
            chunk: String::new(),
        },
        ExecutionEvent::Exit { code: 0 },
        ExecutionEvent::Timing { duration_ms: 1 },
    ];

    let mut ev_a = vec![
        ExecutionEvent::Spawn {
            command: "x".into(),
        },
        ExecutionEvent::Stdout {
            chunk: "a\n".into(),
        },
    ];
    ev_a.extend_from_slice(&tail);

    let mut ev_b = vec![
        ExecutionEvent::Spawn {
            command: "x".into(),
        },
        ExecutionEvent::Stdout {
            chunk: "b\n".into(),
        },
    ];
    ev_b.extend_from_slice(&tail);

    let t_a = ExecutionTrace {
        run_id: "a".into(),
        events: ev_a,
    };
    let t_b = ExecutionTrace {
        run_id: "b".into(),
        events: ev_b,
    };

    let d = diff_traces(&t_a, &t_b);
    assert_eq!(d.len(), 1, "{d:?}");
    assert!(d[0].starts_with("event[1]"), "{}", d[0]);
    assert!(d[0].contains("Stdout"), "{}", d[0]);
}
