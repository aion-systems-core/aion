use crate::KernelGateway;
use serde_json::json;

pub fn run<G: KernelGateway>(g: &G, command: Vec<String>) -> Result<String, String> {
    let spec = json!({ "command": command }).to_string();
    g.run(&spec)
}

pub fn replay<G: KernelGateway>(g: &G, artifact_path: &str) -> Result<String, String> {
    let s = std::fs::read_to_string(artifact_path).map_err(|e| format!("replay read: {e}"))?;
    g.replay(&s)
}

pub fn capsule<G: KernelGateway>(
    g: &G,
    command: Vec<String>,
    policy: &str,
    out_dir: Option<&str>,
) -> Result<String, String> {
    let mut spec = json!({ "command": command, "policy": policy });
    if let Some(d) = out_dir {
        spec["out_dir"] = json!(d);
    }
    g.capsule(&spec.to_string())
}

