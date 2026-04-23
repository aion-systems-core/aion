//! Capture subprocess output into [`RunResult`](aion_core::RunResult).

use aion_core::error::{code, line};
use aion_core::{DeterminismProfile, RunResult, EXECUTION_ARTIFACT_SCHEMA_VERSION};
use aion_kernel::{cwd_string, env_fingerprint, filtered_env_for_child, run_command};
use sha2::{Digest, Sha256};

fn run_id_for(command: &str, cwd: &str, ts: u64) -> String {
    let src = format!("{command}\x1f{cwd}\x1f{ts}");
    let full = format!("{:x}", Sha256::digest(src.as_bytes()));
    full.chars().take(16).collect()
}

/// Execute `command[0]` with args `command[1..]` using a filtered, sorted env map.
pub fn capture(command: &[String], det: &DeterminismProfile) -> Result<RunResult, String> {
    if command.is_empty() {
        return Err(line(code::CAPTURE_EMPTY, "capture", "empty_argv"));
    }
    let program = &command[0];
    let args = &command[1..];
    let env = filtered_env_for_child();
    let fp = env_fingerprint(&env);
    let cwd = cwd_string();
    let ts = det.time_epoch_secs;
    let cmdline = aion_kernel::join_command(program, args);
    let run_id = run_id_for(&cmdline, &cwd, ts);
    let child = run_command(program, args, &env)?;
    Ok(RunResult {
        schema_version: EXECUTION_ARTIFACT_SCHEMA_VERSION,
        run_id,
        command: cmdline,
        cwd,
        timestamp: ts,
        stdout: child.stdout,
        stderr: child.stderr,
        exit_code: child.code,
        duration_ms: child.duration_ms,
        env_fingerprint: fp,
    })
}
