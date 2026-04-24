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
fn measure_outputs_are_structured() {
    let metrics = output_bundle::write_measure_metrics_output().expect("metrics");
    let kpis = output_bundle::write_measure_kpis_output().expect("kpis");
    let audits = output_bundle::write_measure_audits_output().expect("audits");
    let evidence = output_bundle::write_measure_evidence_output().expect("evidence");

    assert!(nested_data(&read_data(&metrics))
        .get("metrics_contract")
        .is_some());
    assert!(nested_data(&read_data(&kpis)).get("kpi_contract").is_some());
    assert!(nested_data(&read_data(&audits))
        .get("audit_reports")
        .is_some());
    assert!(nested_data(&read_data(&evidence))
        .get("evidence_export")
        .is_some());
}
