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
fn ux_outputs_are_structured() {
    let api = output_bundle::write_ux_api_output().expect("api");
    let cli = output_bundle::write_ux_cli_output().expect("cli");
    let admin = output_bundle::write_ux_admin_output().expect("admin");
    let gp = output_bundle::write_ux_golden_paths_output().expect("golden-paths");

    assert!(nested_data(&read_data(&api)).get("api_stability").is_some());
    assert!(nested_data(&read_data(&cli)).get("cli_stability").is_some());
    assert!(nested_data(&read_data(&admin)).get("admin_docs").is_some());
    assert!(nested_data(&read_data(&gp)).get("golden_paths").is_some());
}
