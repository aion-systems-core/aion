// `repro replay <run_id|last>`

use crate::core::replay;

pub fn handle(run_id: &str) -> Result<(), String> {
    let rendered = replay::replay_run(run_id).map_err(|e| format!("replay: {e}"))?;
    print!("{rendered}");
    Ok(())
}
