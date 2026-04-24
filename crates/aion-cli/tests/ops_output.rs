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
fn ops_outputs_are_structured() {
    let runbooks = output_bundle::write_ops_runbooks_output().expect("runbooks");
    let incidents = output_bundle::write_ops_incidents_output().expect("incidents");
    let dr = output_bundle::write_ops_dr_output().expect("dr");
    let upgrade = output_bundle::write_ops_upgrade_output().expect("upgrade");

    assert!(nested_data(&read_data(&runbooks)).get("runbooks").is_some());
    assert!(nested_data(&read_data(&incidents))
        .get("incident_model")
        .is_some());
    assert!(nested_data(&read_data(&dr)).get("dr_status").is_some());
    assert!(nested_data(&read_data(&upgrade))
        .get("upgrade_migration_status")
        .is_some());
}
