//! Same command + env → identical trace event shapes (timing duration normalized).

use super::{assert_repro_ok, load_latest_artifact, run_repro_env, scratch_dir};
use repro::core::execution_trace::{ExecutionEvent, ExecutionTrace};
use serde_json::Value;

/// `Timing.duration_ms` is wall-measured per run; normalize for byte-stable JSON.
fn events_json_normalized(events: &[ExecutionEvent]) -> String {
    let v: Vec<Value> = events
        .iter()
        .map(|e| match e {
            ExecutionEvent::Timing { .. } => serde_json::json!({ "Timing": { "duration_ms": 0 } }),
            _ => serde_json::to_value(e).expect("serialize event"),
        })
        .collect();
    serde_json::to_string(&v).expect("stringify")
}

fn event_kind_name(e: &ExecutionEvent) -> &'static str {
    match e {
        ExecutionEvent::Spawn { .. } => "Spawn",
        ExecutionEvent::EnvResolved { .. } => "EnvResolved",
        ExecutionEvent::Stdout { .. } => "Stdout",
        ExecutionEvent::Stderr { .. } => "Stderr",
        ExecutionEvent::Exit { .. } => "Exit",
        ExecutionEvent::Timing { .. } => "Timing",
    }
}

fn assert_no_wall_clock_fields_in_events_json(s: &str) {
    assert!(
        !s.contains("\"timestamp\""),
        "events JSON must not contain timestamp fields: {s}"
    );
}

#[test]
fn two_identical_cli_runs_match_normalized_event_json() {
    // Same content-addressed `run_id` dedupes INDEX lines in one cwd; use two
    // trees so each run is stored independently (CLI behavior unchanged).
    let cwd_a = scratch_dir("event_det_a");
    let cwd_b = scratch_dir("event_det_b");
    let env = [("ENV_VAR", "foo")];
    let out1 = run_repro_env(&cwd_a, &env, &["run", "--", "echo", "hello"]);
    assert_repro_ok(&out1, "run 1");
    let out2 = run_repro_env(&cwd_b, &env, &["run", "--", "echo", "hello"]);
    assert_repro_ok(&out2, "run 2");

    let a = load_latest_artifact(&cwd_a);
    let b = load_latest_artifact(&cwd_b);

    let ea = &a.trace.events;
    let eb = &b.trace.events;
    assert_eq!(ea.len(), eb.len(), "event count mismatch");

    for (i, (xa, xb)) in ea.iter().zip(eb.iter()).enumerate() {
        assert_eq!(
            event_kind_name(xa),
            event_kind_name(xb),
            "event kind mismatch at index {i}"
        );
    }

    let sa = events_json_normalized(ea);
    let sb = events_json_normalized(eb);
    assert_no_wall_clock_fields_in_events_json(&sa);
    assert_no_wall_clock_fields_in_events_json(&sb);
    assert_eq!(sa, sb, "normalized events JSON must match");

    let stdout_a: String = ea
        .iter()
        .filter_map(|e| match e {
            ExecutionEvent::Stdout { chunk } => Some(chunk.as_str()),
            _ => None,
        })
        .collect();
    let stdout_b: String = eb
        .iter()
        .filter_map(|e| match e {
            ExecutionEvent::Stdout { chunk } => Some(chunk.as_str()),
            _ => None,
        })
        .collect();
    assert_eq!(stdout_a, stdout_b);
}

#[test]
fn trace_json_skips_timing_for_structural_identity() {
    // Library-level: two traces identical except Timing duration → same normalized form.
    let t1 = ExecutionTrace {
        run_id: "r".into(),
        events: vec![
            ExecutionEvent::Spawn {
                command: "echo hello".into(),
            },
            ExecutionEvent::EnvResolved {
                keys: vec!["PATH".into()],
            },
            ExecutionEvent::Stdout {
                chunk: "hello\n".into(),
            },
            ExecutionEvent::Stderr {
                chunk: String::new(),
            },
            ExecutionEvent::Exit { code: 0 },
            ExecutionEvent::Timing { duration_ms: 1 },
        ],
    };
    let t2 = ExecutionTrace {
        run_id: "r".into(),
        events: vec![
            ExecutionEvent::Spawn {
                command: "echo hello".into(),
            },
            ExecutionEvent::EnvResolved {
                keys: vec!["PATH".into()],
            },
            ExecutionEvent::Stdout {
                chunk: "hello\n".into(),
            },
            ExecutionEvent::Stderr {
                chunk: String::new(),
            },
            ExecutionEvent::Exit { code: 0 },
            ExecutionEvent::Timing { duration_ms: 999 },
        ],
    };
    assert_eq!(
        events_json_normalized(&t1.events),
        events_json_normalized(&t2.events)
    );
}
