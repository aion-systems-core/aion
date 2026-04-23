//! `KernelGateway`: every product operation flows through this trait.

use aion_core::error::{code, line};
use aion_core::{write_capsule_v1, Capsule, DeterminismProfile, PolicyProfile};
use aion_engine::{capture, diff, events, graph, policy, replay, replay_debug, trace, why};
use aion_kernel::{apply_net_policy_stub, evaluate_and_enforce, full_report, now_secs};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::path::Path;

pub mod output_bundle;
pub mod sectors;
pub mod ux;

/// In-process kernel + engine surface (no external `libloading`).
pub trait KernelGateway {
    fn run(&self, spec_json: &str) -> Result<String, String>;
    fn capsule(&self, spec_json: &str) -> Result<String, String>;
    fn replay(&self, run_json: &str) -> Result<String, String>;
    fn diff(&self, left_json: &str, right_json: &str) -> Result<String, String>;
    fn why(&self, left_json: &str, right_json: &str) -> Result<String, String>;
    fn graph(&self, run_json: &str) -> Result<String, String>;
    fn integrity(&self) -> Result<String, String>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct InProcessKernel;

fn seed_from_policy(name: &str) -> u64 {
    let h = Sha256::digest(name.as_bytes());
    let mut b = [0u8; 8];
    b.copy_from_slice(&h[..8]);
    u64::from_be_bytes(b)
}

fn parse_command(v: &Value) -> Result<Vec<String>, String> {
    let arr = v
        .get("command")
        .and_then(Value::as_array)
        .ok_or_else(|| line(code::CLI_SPEC_SHAPE, "parse_command", "missing_command_array"))?;
    let mut out = Vec::new();
    for x in arr {
        let s = x.as_str().ok_or_else(|| {
            line(
                code::CLI_SPEC_SHAPE,
                "parse_command",
                "command_entry_not_string",
            )
        })?;
        out.push(s.to_string());
    }
    if out.is_empty() {
        return Err(line(
            code::CLI_SPEC_SHAPE,
            "parse_command",
            "empty_command",
        ));
    }
    Ok(out)
}

fn determinism_from_spec(v: &Value) -> DeterminismProfile {
    let mut det = DeterminismProfile::default();
    if let Some(t) = v.get("time_epoch_secs").and_then(Value::as_u64) {
        det.time_epoch_secs = t;
    }
    if let Some(s) = v.get("random_seed").and_then(Value::as_u64) {
        det.random_seed = s;
    }
    if det.time_epoch_secs == 0 {
        det.time_epoch_secs = now_secs();
    }
    det
}

impl KernelGateway for InProcessKernel {
    fn run(&self, spec_json: &str) -> Result<String, String> {
        let v: Value = serde_json::from_str(spec_json).map_err(|_| {
            line(
                code::CLI_JSON_PARSE,
                "kernel_run_spec",
                "invalid_json",
            )
        })?;
        let cmd = parse_command(&v)?;
        let mut det = determinism_from_spec(&v);
        if v.get("time_epoch_secs").is_none() {
            det.time_epoch_secs = now_secs();
        }
        let rr = capture::capture(&cmd, &det)?;
        if v
            .get("include_telemetry")
            .and_then(Value::as_bool)
            .unwrap_or(false)
        {
            let store = events::store_from_run(&rr, None);
            let tr = trace::trace_from_run(&rr);
            return serde_json::to_string(&json!({
                "run": rr,
                "events": store.into_file(),
                "trace": tr,
            }))
            .map_err(|_| {
                line(
                    code::CLI_JSON_SERIALIZE,
                    "kernel_run_telemetry_bundle",
                    "invalid_json",
                )
            });
        }
        serde_json::to_string(&rr).map_err(|_| {
            line(
                code::CLI_JSON_SERIALIZE,
                "kernel_run",
                "invalid_json",
            )
        })
    }

    fn capsule(&self, spec_json: &str) -> Result<String, String> {
        let v: Value = serde_json::from_str(spec_json).map_err(|_| {
            line(
                code::CLI_JSON_PARSE,
                "kernel_capsule_spec",
                "invalid_json",
            )
        })?;
        let cmd = parse_command(&v)?;
        let policy_name = v.get("policy").and_then(Value::as_str).unwrap_or("dev");
        let pol = policy::resolve(policy_name);
        apply_net_policy_stub(&policy::net_policy_for(&pol))?;
        let mut det = DeterminismProfile {
            time_frozen: pol.deterministic_time,
            time_epoch_secs: now_secs(),
            random_seed: seed_from_policy(policy_name),
            syscall_intercept: false,
            ..Default::default()
        };
        if let Some(t) = v.get("time_epoch_secs").and_then(Value::as_u64) {
            det.time_epoch_secs = t;
        }
        if let Some(s) = v.get("random_seed").and_then(Value::as_u64) {
            det.random_seed = s;
        }
        let rr = capture::capture(&cmd, &det)?;
        if policy_name == "prod" {
            evaluate_and_enforce(&pol, &det)?;
        }
        let run_json = serde_json::to_string(&rr).map_err(|_| {
            line(
                code::CLI_JSON_SERIALIZE,
                "kernel_capsule_run",
                "invalid_json",
            )
        })?;
        let out_dir = v
            .get("out_dir")
            .and_then(Value::as_str)
            .unwrap_or("capsules");
        let path = write_capsule_v1(&run_json, Path::new(out_dir), pol.clone(), det)?;
        let zip_file = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| {
                line(
                    code::CLI_SPEC_SHAPE,
                    "capsule_zip",
                    "missing_zip_file_name",
                )
            })?
            .to_string();
        let capsule = Capsule {
            capsule_schema_version: aion_core::CAPSULE_SCHEMA_VERSION,
            zip_file,
            run: rr,
            policy: pol,
            determinism: det,
        };
        serde_json::to_string(&capsule).map_err(|_| {
            line(
                code::CLI_JSON_SERIALIZE,
                "kernel_capsule",
                "invalid_json",
            )
        })
    }

    fn replay(&self, run_json: &str) -> Result<String, String> {
        let out = replay::replay_stdout(run_json)?;
        Ok(json!({ "stdout": out }).to_string())
    }

    fn diff(&self, left_json: &str, right_json: &str) -> Result<String, String> {
        let d = diff::diff_runs(left_json, right_json)?;
        serde_json::to_string(&d).map_err(|_| {
            line(
                code::CLI_JSON_SERIALIZE,
                "kernel_diff",
                "invalid_json",
            )
        })
    }

    fn why(&self, left_json: &str, right_json: &str) -> Result<String, String> {
        let w = why::why_pair(left_json, right_json)?;
        serde_json::to_string(&w).map_err(|_| {
            line(
                code::CLI_JSON_SERIALIZE,
                "kernel_why",
                "invalid_json",
            )
        })
    }

    fn graph(&self, run_json: &str) -> Result<String, String> {
        graph::graph_json_v2(run_json)
    }

    fn integrity(&self) -> Result<String, String> {
        let pol = PolicyProfile::dev();
        let det = DeterminismProfile::default();
        let rep = full_report(Some(&pol), Some(&det), None);
        serde_json::to_string(&rep).map_err(|_| {
            line(
                code::CLI_JSON_SERIALIZE,
                "kernel_integrity",
                "invalid_json",
            )
        })
    }
}

/// Replay with stderr/duration previews (debug).
pub fn replay_debug_gateway(run_json: &str, max: usize) -> Result<String, String> {
    replay_debug::replay_debug_json(run_json, max)
}
