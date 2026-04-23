// Execution capture layer.
//
// Production capture always executes a real subprocess via
// `std::process::Command`. Environment material is the 15% slice from
// `execution_boundary` only. The COS kernel stays out of scope;
// `engine::cos_adapter` is the integration seam for later.

use crate::core::artifact::{
    deterministic_run_id, ExecutionArtifact, ReproRun, EXECUTION_ARTIFACT_SCHEMA_VERSION,
};
use crate::core::execution_boundary::{
    assert_exposure_budget, capture_env_snapshot_15, capture_process_environment_full,
};
use crate::core::execution_trace::{ExecutionEvent, ExecutionTrace};
use aion_core::run::{execute as execute_run, RunSpec};
use std::time::Instant;

/// Source of timestamps. Real runs use wall-clock; tests inject a fixed clock.
pub trait Clock {
    fn now(&self) -> u64;
}

/// Default wall-clock source (seconds since UNIX epoch).
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    }
}

/// Fixed clock for deterministic tests / replay.
#[allow(dead_code)] // exercised by tests and by the COS adapter integration seam
pub struct FixedClock(pub u64);

impl Clock for FixedClock {
    fn now(&self) -> u64 {
        self.0
    }
}

/// Reset hook retained for tests that serialized capture; run ids are now
/// content-addressed and do not use a process-global counter.
#[doc(hidden)]
pub fn reset_counter_for_tests() {}

/// Serialize tests that spawn subprocesses from the capture layer so
/// parallel `cargo test` stays deterministic.
#[doc(hidden)]
pub static CAPTURE_TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// Subprocess outcome passed into [`build_execution_trace`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

/// Build the canonical event timeline for a completed subprocess capture.
///
/// Event order (locked in Phase 8.1 semantics tests): `Spawn` → `EnvResolved` → stream events
/// → `Exit` → `Timing`.
pub fn build_execution_trace(
    program: &str,
    args: &[String],
    output: &ProcessResult,
    duration_ms: u128,
    run_id: String,
    env_resolved_keys: Vec<String>,
) -> ExecutionTrace {
    ExecutionTrace {
        run_id,
        events: vec![
            ExecutionEvent::Spawn {
                command: format!("{} {}", program, args.join(" ")),
            },
            ExecutionEvent::EnvResolved {
                keys: env_resolved_keys,
            },
            ExecutionEvent::Stdout {
                chunk: output.stdout.clone(),
            },
            ExecutionEvent::Stderr {
                chunk: output.stderr.clone(),
            },
            ExecutionEvent::Exit {
                code: output.exit_code,
            },
            ExecutionEvent::Timing { duration_ms },
        ],
    }
}

/// Public entry used by `run_handler`: real subprocess, real clock.
pub fn capture_command_real(
    program: &str,
    args: &[String],
    joined_command: &str,
) -> ExecutionArtifact {
    capture_command_real_with_clock(program, args, joined_command, &SystemClock)
}

/// Real subprocess capture with an injected clock (tests only).
pub fn capture_command_real_with_clock(
    program: &str,
    args: &[String],
    joined_command: &str,
    clock: &dyn Clock,
) -> ExecutionArtifact {
    assert_exposure_budget();
    let timestamp = clock.now();

    let env_snapshot = capture_env_snapshot_15();
    let cwd = env_snapshot.cwd.clone();
    let env_full = capture_process_environment_full();
    let command_argv: Vec<String> = std::iter::once(program.to_string())
        .chain(args.iter().cloned())
        .collect();

    let (stdout, stderr, exit_code, duration_ms) = if program.is_empty() {
        let start = Instant::now();
        (
            String::new(),
            "empty program".to_string(),
            2,
            start.elapsed().as_millis() as u64,
        )
    } else {
        let start = Instant::now();
        match execute_run(&RunSpec {
            program: program.to_string(),
            args: args.to_vec(),
        }) {
            Ok(result) => {
                let stdout = result.stdout;
                let stderr = result.stderr;
                let exit_code = result.exit_code;
                let duration_ms = result.duration_ms;
                (stdout, stderr, exit_code, duration_ms)
            }
            Err(e) => {
                let duration_ms = start.elapsed().as_millis() as u64;
                (String::new(), format!("spawn error: {e}"), -1, duration_ms)
            }
        }
    };

    let run_id = deterministic_run_id(&command_argv, &env_full, &stdout);
    let repro_run = Some(ReproRun {
        id: run_id.clone(),
        command: command_argv,
        env: env_full,
        stdout: stdout.clone(),
    });

    let process_result = ProcessResult {
        stdout: stdout.clone(),
        stderr: stderr.clone(),
        exit_code,
    };
    let trace = build_execution_trace(
        program,
        args,
        &process_result,
        duration_ms as u128,
        run_id.clone(),
        crate::core::execution_boundary::env_resolved_trace_keys(),
    );
    // Canonical events are written next to the artifact JSON in `storage::save_run_in`
    // (`<run_id>.events.json`, schema `aion/event_stream/v1`).

    ExecutionArtifact {
        schema_version: EXECUTION_ARTIFACT_SCHEMA_VERSION,
        run_id,
        repro_run,
        command: joined_command.to_string(),
        cwd,
        timestamp,
        env_snapshot,
        stdout,
        stderr,
        exit_code,
        duration_ms,
        trace,
    }
}

/// Back-compat: parse `command` as whitespace-separated argv and execute.
#[allow(dead_code)] // public API; `cos_adapter` uses `capture_command_with_clock`
pub fn capture_command(command: String) -> ExecutionArtifact {
    let (program, args) = split_cli_line(&command);
    capture_command_real(&program, &args, &command)
}

/// Capture with an injected clock (tests / COS stub). Still runs a real
/// subprocess when `program` is non-empty.
pub fn capture_command_with_clock(command: String, clock: &dyn Clock) -> ExecutionArtifact {
    let (program, args) = split_cli_line(&command);
    capture_command_real_with_clock(&program, &args, &command, clock)
}

fn split_cli_line(command: &str) -> (String, Vec<String>) {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.is_empty() {
        return (String::new(), vec![]);
    }
    let program = parts[0].to_string();
    let args = parts[1..].iter().map(|s| (*s).to_string()).collect();
    (program, args)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::execution_trace::ExecutionEvent;

    #[test]
    fn same_input_same_artifact_except_timing() {
        let _guard = CAPTURE_TEST_LOCK.lock().unwrap();
        reset_counter_for_tests();
        let a = capture_command_with_clock("echo hello".into(), &FixedClock(42));
        reset_counter_for_tests();
        let b = capture_command_with_clock("echo hello".into(), &FixedClock(42));
        assert_eq!(a.command, b.command);
        assert_eq!(a.stdout, b.stdout);
        assert_eq!(a.stderr, b.stderr);
        assert_eq!(a.exit_code, b.exit_code);
        assert_eq!(a.environment_hash(), b.environment_hash());
        assert_eq!(a.cwd, b.cwd);
        assert_eq!(a.run_id, b.run_id);
        assert_eq!(a.timestamp, b.timestamp);
        assert_eq!(a.trace.events.len(), b.trace.events.len());
        for (ea, eb) in a.trace.events.iter().zip(b.trace.events.iter()) {
            match (ea, eb) {
                (ExecutionEvent::Timing { .. }, ExecutionEvent::Timing { .. }) => {}
                _ => assert_eq!(ea, eb),
            }
        }
    }

    #[test]
    fn echo_round_trips() {
        let _guard = CAPTURE_TEST_LOCK.lock().unwrap();
        reset_counter_for_tests();
        let a = capture_command_with_clock("echo hello".into(), &FixedClock(0));
        assert_eq!(a.stdout, "hello\n");
        assert_eq!(a.stderr, "");
        assert_eq!(a.exit_code, 0);
    }

    #[test]
    fn different_commands_differ() {
        let _guard = CAPTURE_TEST_LOCK.lock().unwrap();
        reset_counter_for_tests();
        let a = capture_command_with_clock("echo a".into(), &FixedClock(0));
        let b = capture_command_with_clock("echo b".into(), &FixedClock(0));
        assert_ne!(a.run_id, b.run_id);
        assert_ne!(a.stdout, b.stdout);
    }
}
