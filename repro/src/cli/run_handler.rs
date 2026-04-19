// `repro run -- <command>`
//
// Captures the command into an artifact and persists it. The CLI layer
// is intentionally trivial: it splits argv into program + args, delegates
// to `core::capture::capture_command_real`, and writes through `core::storage`.

use crate::core::{capture, output, storage};

pub fn handle(command: Vec<String>) -> Result<(), String> {
    if command.is_empty() {
        return Err("run: missing command (usage: repro run -- <command>)".to_string());
    }
    let joined = join_argv(&command);
    let program = command[0].as_str();
    let args: Vec<String> = command[1..].to_vec();
    let artifact = capture::capture_command_real(program, &args, &joined);
    storage::save_run(&artifact).map_err(|e| format!("run: failed to save: {e}"))?;

    print!("{}", output::format_artifact(&artifact));
    Ok(())
}

/// Re-join argv into a single command string. We quote only tokens that
/// actually need it so the common `echo hello` case round-trips verbatim
/// (the success criteria explicitly uses unquoted `echo hello`).
fn join_argv(argv: &[String]) -> String {
    argv.iter()
        .map(|t| {
            if t.is_empty() || t.chars().any(|c| c.is_whitespace() || c == '"') {
                format!("\"{}\"", t.replace('"', "\\\""))
            } else {
                t.clone()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
