//! Run a subprocess with optional injected environment.

use aion_core::error::{code, io_cause, line};
use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;
use std::time::Instant;

#[derive(Debug)]
pub struct ChildOutput {
    pub stdout: String,
    pub stderr: String,
    pub code: i32,
    pub duration_ms: u64,
}

pub fn run_command(program: &str, args: &[String], env: &BTreeMap<String, String>) -> Result<ChildOutput, String> {
    let start = Instant::now();
    let mut cmd = Command::new(program);
    cmd.args(args);
    cmd.env_clear();
    for (k, v) in env {
        cmd.env(k, v);
    }
    if let Ok(cwd) = std::env::current_dir() {
        cmd.current_dir(&cwd);
    }
    let out = cmd
        .output()
        .map_err(|e| line(code::KERNEL_SPAWN, "run_command", &io_cause(&e)))?;
    let duration_ms = start.elapsed().as_millis() as u64;
    let code = out.status.code().unwrap_or(-1);
    Ok(ChildOutput {
        stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        code,
        duration_ms,
    })
}

pub fn cwd_string() -> String {
    std::env::current_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| ".".into())
}

pub fn join_command(program: &str, args: &[String]) -> String {
    let mut s = program.to_string();
    for a in args {
        s.push(' ');
        s.push_str(a);
    }
    s
}

pub fn path_exists(p: &str) -> bool {
    Path::new(p).exists()
}
