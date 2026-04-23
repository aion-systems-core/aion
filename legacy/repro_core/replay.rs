// Replay engine.
//
// "Replay" here means: re-render a stored artifact to the terminal as
// if the run were happening again. We never re-execute the shell
// command — that would defeat determinism and is the whole reason
// `capture` persists stdout/stderr/exit_code in the first place.
//
// The replay path also constructs an `ExecutionReport` (with only the
// `identity` + `trace` sections populated) so downstream tooling has
// a single model to consume regardless of which CLI command invoked
// it. The rendered string still starts with `── replay ──` and still
// emits the `stdout | ` / `stderr | ` / `exit   |` prefixes that the
// e2e suite pins.

use crate::core::artifact::ExecutionArtifact;
use crate::core::event_store;
use crate::core::execution_trace::{ExecutionEvent, ExecutionTrace};
use crate::core::identity::ExecutionIdentity;
use crate::core::output;
use crate::core::report::{ExecutionReport, TraceSummary};
use crate::core::storage;
use std::io;

/// Concatenate all `Stdout` chunks from the trace in order (Phase 8.1 contract).
pub fn replay_stdout(trace: &ExecutionTrace) -> String {
    trace
        .events
        .iter()
        .filter_map(|e| match e {
            ExecutionEvent::Stdout { chunk } => Some(chunk.as_str()),
            _ => None,
        })
        .collect()
}

/// Load the canonical event stream when present; otherwise fall back to the artifact trace.
pub fn replay_run(run_id_or_alias: &str) -> io::Result<String> {
    let run_id = storage::resolve_alias(run_id_or_alias)?;
    let trace = match event_store::load_event_stream_in(&storage::runs_dir(), &run_id) {
        Ok(t) => t,
        Err(_) => storage::load_run(&run_id)?.trace,
    };
    Ok(replay_stdout(&trace))
}

/// Build an `ExecutionReport` for a single artifact — identity +
/// trace only. The report is the input for `output::format_report`,
/// but the replay command uses the lower-level pinned rendering
/// below instead, for output-contract stability.
#[allow(dead_code)] // public integration point; exercised by tests
pub fn report_for(a: &ExecutionArtifact) -> ExecutionReport {
    ExecutionReport {
        identity: ExecutionIdentity::from_artifact(a),
        trace: TraceSummary::from_artifact(a),
        diff: None,
        root_cause: None,
    }
}

pub fn format_replay(a: &ExecutionArtifact) -> String {
    let mut out = String::new();
    out.push_str("── replay ──\n");
    out.push_str(&output::format_artifact(a));
    out.push_str("── replay steps ──\n");
    // Phase-1 step model: the captured streams *are* the replay. We
    // render them with stable prefixes so diff tools (and humans) can
    // spot the boundary between stdout and stderr.
    for line in a.stdout.lines() {
        out.push_str("  stdout | ");
        out.push_str(line);
        out.push('\n');
    }
    for line in a.stderr.lines() {
        out.push_str("  stderr | ");
        out.push_str(line);
        out.push('\n');
    }
    out.push_str(&format!("  exit   | {}\n", a.exit_code));
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::capture::{
        capture_command_with_clock, reset_counter_for_tests, FixedClock, CAPTURE_TEST_LOCK,
    };

    #[test]
    fn format_is_deterministic() {
        let _g = CAPTURE_TEST_LOCK.lock().unwrap();
        reset_counter_for_tests();
        let a = capture_command_with_clock("echo hello".into(), &FixedClock(1));
        let s1 = format_replay(&a);
        let s2 = format_replay(&a);
        assert_eq!(s1, s2);
        assert!(s1.contains("stdout | hello"));
        assert!(s1.contains("exit   | 0"));
    }

    #[test]
    fn replay_stdout_matches_artifact_stdout() {
        let _g = CAPTURE_TEST_LOCK.lock().unwrap();
        reset_counter_for_tests();
        let a = capture_command_with_clock("echo hello".into(), &FixedClock(1));
        assert_eq!(replay_stdout(&a.trace), a.stdout);
    }

    #[test]
    fn report_for_artifact_round_trips_identity() {
        let _g = CAPTURE_TEST_LOCK.lock().unwrap();
        reset_counter_for_tests();
        let a = capture_command_with_clock("echo hi".into(), &FixedClock(1));
        let r1 = report_for(&a);
        let r2 = report_for(&a);
        assert_eq!(r1, r2);
    }
}
