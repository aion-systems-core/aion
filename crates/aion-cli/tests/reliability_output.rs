use aion_cli::output_bundle;
use serde_json::Value;

fn read_data(path: &std::path::Path) -> Value {
    let body = std::fs::read_to_string(path.join("result.json")).expect("read result");
    let envelope: Value = serde_json::from_str(&body).expect("parse json");
    envelope.get("data").cloned().expect("envelope data")
}

fn nested_data(v: &Value) -> &Value {
    v.get("data").unwrap_or(v)
}

#[test]
fn reliability_outputs_exist_and_are_structured() {
    let status = output_bundle::write_reliability_status_output().expect("status output");
    let slo = output_bundle::write_reliability_slo_output().expect("slo output");
    let chaos = output_bundle::write_reliability_chaos_output().expect("chaos output");
    let soak = output_bundle::write_reliability_soak_output().expect("soak output");

    let status_data = read_data(&status);
    let status_payload = nested_data(&status_data);
    assert!(status_payload.get("slo_status").is_some());
    assert!(status_payload.get("reliability_status").is_some());
    assert!(status_payload.get("chaos_status").is_some());
    assert!(status_payload.get("soak_status").is_some());

    let slo_data = read_data(&slo);
    assert!(nested_data(&slo_data).get("slo_status").is_some());

    let chaos_data = read_data(&chaos);
    assert!(nested_data(&chaos_data).get("chaos_status").is_some());

    let soak_data = read_data(&soak);
    assert!(nested_data(&soak_data).get("soak_status").is_some());
}

