use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn golden_dir() -> PathBuf {
    repo_root().join("tests").join("golden")
}

fn unique_output_base() -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before unix epoch")
        .as_nanos();
    repo_root()
        .join("target")
        .join("golden_tests")
        .join(format!("run_{}_{}", std::process::id(), nanos))
}

fn read_json(path: &Path) -> Value {
    let s = fs::read_to_string(path).expect("read json");
    serde_json::from_str(&s).expect("parse json")
}

fn parse_output_path(stdout: &str) -> PathBuf {
    let prefix = "Output written to: ";
    let line = stdout
        .lines()
        .find(|l| l.starts_with(prefix))
        .expect("output path line missing");
    PathBuf::from(line.trim_start_matches(prefix).trim())
}

fn run_sealrun(out_base: &Path, id: &str, args: &[&str]) -> (String, String, PathBuf) {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_sealrun"));
    cmd.arg("--id").arg(id);
    cmd.args(args);
    cmd.env("SEALRUN_OUTPUT_BASE", out_base);
    cmd.env("AION_OUTPUT_BASE", out_base);
    cmd.current_dir(repo_root());
    let output = cmd.output().expect("run sealrun");
    let stdout = String::from_utf8(output.stdout).expect("stdout utf8");
    let stderr = String::from_utf8(output.stderr).expect("stderr utf8");
    assert!(
        output.status.success(),
        "sealrun {:?} failed\nstdout:\n{}\nstderr:\n{}",
        args,
        stdout,
        stderr
    );
    let output_path = parse_output_path(&stdout);
    (stdout, stderr, output_path)
}

fn normalize_ai(ai_json: &Value) -> Value {
    let data = &ai_json["data"];
    let capsule = data.get("original_capsule").unwrap_or(data);
    json!({
        "model": capsule["model"],
        "prompt": capsule["prompt"],
        "seed": capsule["seed"],
        "tokens": capsule["tokens"],
        "drift_changed": capsule["drift"]["changed"]
    })
}

fn required_value<'a>(value: &'a Value, path: &[&str]) -> &'a Value {
    let mut current = value;
    for key in path {
        current = current
            .get(key)
            .unwrap_or_else(|| panic!("missing required field path: {}", path.join(".")));
    }
    current
}

fn value_kind(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "bool",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

fn normalize_ai_full_contract(ai_json: &Value) -> Value {
    let data = required_value(ai_json, &["data"]);
    let capsule = data.get("original_capsule").unwrap_or(data);
    let evidence = required_value(capsule, &["evidence"]);
    let records = required_value(evidence, &["records"]);
    let first_record = records
        .as_array()
        .and_then(|a| a.first())
        .expect("missing required field path: evidence.records[0]");
    let drift = required_value(capsule, &["drift"]);
    let determinism = required_value(capsule, &["determinism"]);
    let env = required_value(capsule, &["execution_environment"]);
    let machine_fingerprint = required_value(env, &["machine_fingerprint"]);
    let runtime_fingerprint = required_value(env, &["runtime_fingerprint"]);

    json!({
        "status": value_kind(required_value(ai_json, &["status"])),
        "data_kind": value_kind(data),
        "capsule": {
            "model": value_kind(required_value(capsule, &["model"])),
            "prompt": value_kind(required_value(capsule, &["prompt"])),
            "seed": value_kind(required_value(capsule, &["seed"])),
            "tokens": value_kind(required_value(capsule, &["tokens"])),
            "token_trace": value_kind(required_value(capsule, &["token_trace"])),
            "graph": value_kind(required_value(capsule, &["graph"])),
            "why": value_kind(required_value(capsule, &["why"])),
            "determinism": {
                "kind": value_kind(determinism),
                "time_frozen": value_kind(required_value(determinism, &["time_frozen"])),
                "time_epoch_secs": value_kind(required_value(determinism, &["time_epoch_secs"])),
                "random_seed": value_kind(required_value(determinism, &["random_seed"]))
            },
            "drift": {
                "kind": value_kind(drift),
                "changed": value_kind(required_value(drift, &["changed"])),
                "categories": value_kind(required_value(drift, &["categories"]))
            },
            "evidence": {
                "kind": value_kind(evidence),
                "run_id": value_kind(required_value(evidence, &["run_id"])),
                "records": value_kind(records),
                "record_count_min": records.as_array().map(|a| a.len()).unwrap_or(0),
                "first_record": {
                    "kind": value_kind(required_value(first_record, &["kind"])),
                    "seq": value_kind(required_value(first_record, &["seq"])),
                    "payload_digest": value_kind(required_value(first_record, &["payload_digest"])),
                    "leaf_digest": value_kind(required_value(first_record, &["leaf_digest"]))
                }
            },
            "execution_environment": {
                "kind": value_kind(env),
                "machine_fingerprint_kind": value_kind(machine_fingerprint),
                "runtime_fingerprint_kind": value_kind(runtime_fingerprint),
                "os_version": value_kind(required_value(machine_fingerprint, &["os_version"])),
                "aion_version": value_kind(required_value(runtime_fingerprint, &["aion_version"]))
            }
        }
    })
}

fn write_golden_report(report: &Value) {
    let dir = repo_root().join("golden_reports");
    fs::create_dir_all(&dir).expect("create golden_reports dir");
    let path = dir.join("golden_test_report.json");
    let body = serde_json::to_string_pretty(report).expect("serialize golden report");
    fs::write(path, body).expect("write golden report");
}

#[test]
fn golden_execute_replay_and_drift() {
    let fixtures = golden_dir();
    let fixture1_input = read_json(&fixtures.join("fixture1_input.json"));
    let fixture1_expected = read_json(&fixtures.join("fixture1_output.json"));
    let fixture1_full_contract = read_json(&fixtures.join("fixture1_full_contract.json"));
    let fixture2_expected_drift = read_json(&fixtures.join("fixture2_drift.json"));

    let out_base = unique_output_base();
    fs::create_dir_all(&out_base).expect("create output base");

    let model = fixture1_input["model"].as_str().expect("fixture1 model");
    let prompt = fixture1_input["prompt"].as_str().expect("fixture1 prompt");
    let seed = fixture1_input["seed"].as_u64().expect("fixture1 seed");
    let seed_str = seed.to_string();

    let (_, _, exec_out_dir) = run_sealrun(
        &out_base,
        "golden_fixture1_execute",
        &[
            "execute", "ai", "--model", model, "--prompt", prompt, "--seed", &seed_str,
        ],
    );
    let execute_ai = read_json(&exec_out_dir.join("ai.json"));
    let execute_norm = normalize_ai(&execute_ai);
    let execute_full_contract = normalize_ai_full_contract(&execute_ai);
    assert_eq!(
        execute_norm, fixture1_expected,
        "execute output differs from fixture1_output.json"
    );
    assert_eq!(
        execute_full_contract, fixture1_full_contract,
        "execute full-contract shape differs from fixture1_full_contract.json"
    );

    let capsule_path = exec_out_dir.join("capsule.aionai");
    let capsule_path_str = capsule_path.to_string_lossy().to_string();
    let (_, _, replay_out_dir) = run_sealrun(
        &out_base,
        "golden_fixture1_replay",
        &["execute", "ai-replay", "--capsule", &capsule_path_str],
    );
    let replay_ai = read_json(&replay_out_dir.join("ai.json"));
    let replay_norm = normalize_ai(&replay_ai);
    let replay_full_contract = normalize_ai_full_contract(&replay_ai);
    assert_eq!(
        replay_norm, execute_norm,
        "ai-replay normalized output must match execute normalized output"
    );
    assert_eq!(
        replay_full_contract, fixture1_full_contract,
        "ai-replay full-contract shape differs from fixture1_full_contract.json"
    );

    let left = fixtures.join("fixture2_left.json");
    let right = fixtures.join("fixture2_right.json");
    let left_str = left.to_string_lossy().to_string();
    let right_str = right.to_string_lossy().to_string();
    let (_, _, drift_out_dir) = run_sealrun(
        &out_base,
        "golden_fixture2_drift",
        &["observe", "drift", &left_str, &right_str],
    );
    let drift_result = read_json(&drift_out_dir.join("result.json"));
    let drift_norm = drift_result["data"].clone();
    assert_eq!(
        drift_norm, fixture2_expected_drift,
        "drift output differs from fixture2_drift.json"
    );

    write_golden_report(&json!({
        "status": "PASS",
        "checks": {
            "normalized_subset": "PASS",
            "full_contract": "PASS",
            "drift_contract": "PASS"
        }
    }));
}
