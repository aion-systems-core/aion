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
fn governance_outputs_are_structured() {
    let packs = output_bundle::write_policy_packs_output().expect("packs");
    let gates = output_bundle::write_policy_gates_output().expect("gates");
    let evidence = output_bundle::write_policy_evidence_output().expect("evidence");
    let governance = output_bundle::write_governance_status_output().expect("governance");

    assert!(nested_data(&read_data(&packs))
        .get("policy_packs")
        .is_some());
    assert!(nested_data(&read_data(&gates))
        .get("policy_gates")
        .is_some());
    assert!(nested_data(&read_data(&evidence))
        .get("policy_evidence")
        .is_some());
    assert!(nested_data(&read_data(&governance))
        .get("governance_model")
        .is_some());
}
