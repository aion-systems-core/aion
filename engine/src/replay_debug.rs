//! Debug view: exit, duration, truncated streams.

use aion_core::RunResult;
use serde_json::json;

pub fn replay_debug_json(run_json: &str, max: usize) -> Result<String, String> {
    let r: RunResult = serde_json::from_str(run_json).map_err(|e| format!("replay_debug: {e}"))?;
    let trim = |s: &str| {
        let t = s.chars().take(max).collect::<String>();
        if s.chars().count() > max {
            format!("{t}…")
        } else {
            t
        }
    };
    Ok(json!({
        "run_id": r.run_id,
        "exit_code": r.exit_code,
        "duration_ms": r.duration_ms,
        "stdout_preview": trim(&r.stdout),
        "stderr_preview": trim(&r.stderr),
    })
    .to_string())
}
