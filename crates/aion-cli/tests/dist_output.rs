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
fn dist_outputs_are_structured() {
    let status = output_bundle::write_dist_status_output().expect("status");
    let identity = output_bundle::write_dist_identity_output().expect("identity");
    let lts = output_bundle::write_dist_lts_output().expect("lts");
    let installers = output_bundle::write_dist_installers_output().expect("installers");

    assert!(nested_data(&read_data(&status))
        .get("distribution_status")
        .is_some());
    assert!(nested_data(&read_data(&identity))
        .get("identity_matrix")
        .is_some());
    assert!(nested_data(&read_data(&lts)).get("lts_policy").is_some());
    assert!(nested_data(&read_data(&installers))
        .get("installer_trust_chain")
        .is_some());
}

