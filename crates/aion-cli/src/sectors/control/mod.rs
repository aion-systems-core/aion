use crate::KernelGateway;
use aion_core::PolicyProfile;
use aion_engine::ci;
use serde_json::json;
use std::fs;
use std::path::Path;

pub fn shell_argv(cmd: &str) -> Vec<String> {
    if cfg!(windows) {
        vec!["cmd".into(), "/C".into(), cmd.into()]
    } else {
        vec!["sh".into(), "-c".into(), cmd.into()]
    }
}

pub fn policy_list() -> String {
    "dev\nstage\nprod\n".to_string()
}

pub fn policy_show(name: &str) -> Result<String, String> {
    let p = match name {
        "stage" => PolicyProfile::stage(),
        "prod" => PolicyProfile::prod(),
        _ => PolicyProfile::dev(),
    };
    aion_engine::output::layout::canonical_json_from_serialize(&p).map_err(|e| e.to_string())
}

pub fn ci_run<G: KernelGateway>(g: &G, cmd: &str, baseline: &Path) -> Result<String, String> {
    if let Some(parent) = baseline.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let spec = json!({ "command": shell_argv(cmd) }).to_string();
    let run = g.run(&spec)?;
    let meta = ci::record_baseline(baseline, &run)?;
    Ok(json!({
        "baseline": baseline.to_string_lossy(),
        "run": serde_json::from_str::<serde_json::Value>(&run).map_err(|e| e.to_string())?,
        "meta": meta,
    })
    .to_string())
}

pub fn ci_drift<G: KernelGateway>(g: &G, cmd: &str, baseline: &Path) -> Result<String, String> {
    let spec = json!({ "command": shell_argv(cmd) }).to_string();
    let run = g.run(&spec)?;
    let rep = ci::check_baseline(baseline, &run)?;
    Ok(json!({ "drift": rep, "actual": serde_json::from_str::<serde_json::Value>(&run).map_err(|e| e.to_string())? }).to_string())
}

pub fn ci_replay<G: KernelGateway>(g: &G, artifact_path: &str) -> Result<String, String> {
    let s = std::fs::read_to_string(artifact_path).map_err(|e| format!("ci replay: {e}"))?;
    g.replay(&s)
}

pub fn sdk() -> String {
    "SealRun v2 SDK: Rust crates aion-core, aion-kernel, aion-engine; CLI binary `sealrun`.\n"
        .to_string()
}
