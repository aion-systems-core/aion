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
fn tests_outputs_are_structured() {
    let strategy = output_bundle::write_tests_strategy_output().expect("strategy");
    let regression = output_bundle::write_tests_regression_output().expect("regression");
    let compatibility =
        output_bundle::write_tests_compatibility_output().expect("compatibility");
    let fuzz_property =
        output_bundle::write_tests_fuzz_property_output().expect("fuzz_property");

    assert!(nested_data(&read_data(&strategy))
        .get("test_strategy")
        .is_some());
    assert!(nested_data(&read_data(&regression))
        .get("regression_matrix")
        .is_some());
    assert!(nested_data(&read_data(&compatibility))
        .get("compatibility_tests")
        .is_some());
    assert!(nested_data(&read_data(&fuzz_property))
        .get("fuzz_property_tests")
        .is_some());
}

