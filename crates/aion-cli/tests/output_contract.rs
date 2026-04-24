use aion_engine::output::OutputWriter;
use serde_json::{json, Value};
use std::sync::Mutex;

static ENV_LOCK: Mutex<()> = Mutex::new(());

fn env_guard() -> std::sync::MutexGuard<'static, ()> {
    ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner())
}

fn read_result(path: &std::path::Path) -> (String, Value) {
    let body = std::fs::read_to_string(path.join("result.json")).expect("read result.json");
    let json: Value = serde_json::from_str(&body).expect("parse result.json");
    (body, json)
}

#[test]
fn output_contract_has_deterministic_key_order() {
    let _g = env_guard();
    let base = std::env::temp_dir().join("aion-cli-output-contract-keys");
    std::env::set_var("AION_OUTPUT_BASE", &base);
    std::env::set_var("AION_OUTPUT_ID", "run_keys");

    let w = OutputWriter::new("contract").expect("writer");
    w.write_json("result", &json!({"z": 1, "a": 2}))
        .expect("write json");
    let (body, _json) = read_result(w.root());
    let status_pos = body.find("\"status\"").expect("status key");
    let data_pos = body.find("\"data\"").expect("data key");
    assert!(status_pos < data_pos, "status must come before data");
}

#[test]
fn output_contract_sorts_known_arrays_deterministically() {
    let _g = env_guard();
    let base = std::env::temp_dir().join("aion-cli-output-contract-arrays");
    std::env::set_var("AION_OUTPUT_BASE", &base);
    std::env::set_var("AION_OUTPUT_ID", "run_arrays");

    let payload = json!({
        "labels": ["z_label", "a_label"],
        "categories": ["timing", "evidence", "model"],
        "violations": [{"k":"z"}, {"k":"a"}],
        "differences": ["diff:z", "diff:a"]
    });
    let w = OutputWriter::new("contract").expect("writer");
    w.write_json("result", &payload).expect("write json");
    let (_body, json) = read_result(w.root());
    let data = json.get("data").expect("data object");
    assert_eq!(data["labels"], json!(["a_label", "z_label"]));
    assert_eq!(data["differences"], json!(["diff:a", "diff:z"]));
}

#[test]
fn output_contract_is_identical_across_two_runs() {
    let _g = env_guard();
    let base = std::env::temp_dir().join("aion-cli-output-contract-stability");
    std::env::set_var("AION_OUTPUT_BASE", &base);

    std::env::set_var("AION_OUTPUT_ID", "run_one");
    let w1 = OutputWriter::new("contract").expect("writer1");
    w1.write_json("result", &json!({"labels":["b","a"],"ok":true}))
        .expect("write1");
    let (a, _) = read_result(w1.root());

    std::env::set_var("AION_OUTPUT_ID", "run_two");
    let w2 = OutputWriter::new("contract").expect("writer2");
    w2.write_json("result", &json!({"ok":true,"labels":["a","b"]}))
        .expect("write2");
    let (b, _) = read_result(w2.root());

    assert_eq!(a, b, "result.json must be byte-identical across runs");
}

#[test]
fn doctor_output_avoids_free_text_error_shapes() {
    let _g = env_guard();
    std::env::set_var("AION_DOCTOR_LIB_PATH", "C:/__aion_missing__/nope.dll");
    std::env::set_var("AION_LIB_PATH", "C:/__aion_missing__/nope.dll");
    std::env::set_var("AION_DOCTOR_PYTHON_BIN", "python_missing_for_doctor");

    let out = aion_cli::output_bundle::write_product_doctor_output().expect("doctor output");
    let (_body, json) = read_result(&out);
    let checks = json["data"]["checks"].as_array().expect("checks");
    for check in checks {
        let result = &check["result"];
        let code = result["code"].as_str().unwrap_or("");
        assert!(
            code == "AION_OK" || code.starts_with("AION_"),
            "doctor code must use stable machine contract codes"
        );
        assert!(
            result.get("message").is_none(),
            "no free-text message field"
        );
    }
}
