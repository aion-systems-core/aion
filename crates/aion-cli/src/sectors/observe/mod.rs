use crate::KernelGateway;
use serde_json::{json, Value};
use std::fs;

pub fn capture<G: KernelGateway>(g: &G, command: Vec<String>) -> Result<String, String> {
    let spec = json!({ "command": command }).to_string();
    g.run(&spec)
}

pub fn drift<G: KernelGateway>(g: &G, left_path: &str, right_path: &str) -> Result<String, String> {
    let a = fs::read_to_string(left_path).map_err(|e| format!("drift read left: {e}"))?;
    let b = fs::read_to_string(right_path).map_err(|e| format!("drift read right: {e}"))?;
    g.diff(&a, &b)
}

pub fn why<G: KernelGateway>(g: &G, left_path: &str, right_path: &str) -> Result<String, String> {
    let a = fs::read_to_string(left_path).map_err(|e| format!("why read left: {e}"))?;
    let b = fs::read_to_string(right_path).map_err(|e| format!("why read right: {e}"))?;
    g.why(&a, &b)
}

pub fn graph<G: KernelGateway>(g: &G, run_id: &str) -> Result<String, String> {
    if let Ok(s) = fs::read_to_string(run_id) {
        return g.graph(&s);
    }
    let stub = json!({ "run_id": run_id }).to_string();
    g.graph(&stub)
}

pub fn audit<G: KernelGateway>(g: &G, run_id: &str) -> Result<String, String> {
    let integ: Value = serde_json::from_str(&g.integrity()?).map_err(|e| e.to_string())?;
    Ok(json!({
        "audit": "stub",
        "run_id": run_id,
        "integrity": integ,
    })
    .to_string())
}

pub fn integrity<G: KernelGateway>(g: &G) -> Result<String, String> {
    g.integrity()
}
