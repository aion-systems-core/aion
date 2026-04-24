use aion_cli::output_bundle;
use aion_core::{global_consistency_contract_version, os_contract_spec_version};
use serde_json::Value;
use std::path::PathBuf;
use std::sync::Mutex;

static ENV_LOCK: Mutex<()> = Mutex::new(());

fn read_doctor_result_json(out_root: &std::path::Path) -> Value {
    let body =
        std::fs::read_to_string(out_root.join("result.json")).expect("read doctor result.json");
    let envelope: Value = serde_json::from_str(&body).expect("parse doctor json");
    envelope.get("data").cloned().expect("doctor envelope data")
}

fn temp_file(name: &str) -> PathBuf {
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    std::env::temp_dir().join(format!("aion-cli-doctor-{name}-{stamp}.json"))
}

#[test]
fn doctor_reports_missing_kernel_library_as_ffi_error() {
    let _g = ENV_LOCK.lock().expect("env lock");
    std::env::set_var("AION_DOCTOR_LIB_PATH", "C:/__aion_missing__/nope.dll");
    std::env::set_var("AION_LIB_PATH", "C:/__aion_missing__/nope.dll");
    std::env::set_var("AION_DOCTOR_PYTHON_BIN", "python");

    let out = output_bundle::write_product_doctor_output().expect("doctor output");
    let v = read_doctor_result_json(&out);
    let checks = v
        .get("checks")
        .and_then(|x| x.as_array())
        .expect("checks array");
    let ffi = &checks[0]["result"];
    assert_eq!(ffi["status"], "error");
    assert_eq!(ffi["code"], "AION_FFI_IO");
}

#[test]
fn doctor_reports_python_bindings_failure() {
    let _g = ENV_LOCK.lock().expect("env lock");
    std::env::set_var("AION_DOCTOR_LIB_PATH", "C:/__aion_missing__/nope.dll");
    std::env::set_var("AION_LIB_PATH", "C:/__aion_missing__/nope.dll");
    std::env::set_var("AION_DOCTOR_PYTHON_BIN", "python_missing_for_doctor");

    let out = output_bundle::write_product_doctor_output().expect("doctor output");
    let v = read_doctor_result_json(&out);
    let checks = v
        .get("checks")
        .and_then(|x| x.as_array())
        .expect("checks array");
    let py = &checks[1]["result"];
    assert_eq!(py["status"], "error");
    assert_eq!(py["code"], "AION_BINDINGS_IO");
}

#[test]
fn doctor_reports_invalid_policy_file() {
    let _g = ENV_LOCK.lock().expect("env lock");
    let invalid_policy = temp_file("invalid-policy");
    std::fs::write(&invalid_policy, "{").expect("write invalid policy");
    std::env::set_var("AION_DOCTOR_POLICY", &invalid_policy);
    std::env::set_var("AION_DOCTOR_LIB_PATH", "C:/__aion_missing__/nope.dll");
    std::env::set_var("AION_LIB_PATH", "C:/__aion_missing__/nope.dll");
    std::env::set_var("AION_DOCTOR_PYTHON_BIN", "python_missing_for_doctor");

    let out = output_bundle::write_product_doctor_output().expect("doctor output");
    let v = read_doctor_result_json(&out);
    let checks = v
        .get("checks")
        .and_then(|x| x.as_array())
        .expect("checks array");
    let policy = &checks[2]["result"];
    assert_eq!(policy["status"], "error");
    assert_eq!(policy["code"], "AION_GOVERNANCE_JSON");
    let global = v
        .get("global_consistency")
        .expect("global_consistency block");
    assert_eq!(global["run_finality"]["status"], "error");
    assert_eq!(
        global["run_finality"]["cause"].as_str(),
        Some("run:policy_not_final")
    );
}

#[test]
fn doctor_output_has_deterministic_check_order() {
    let _g = ENV_LOCK.lock().expect("env lock");
    std::env::set_var("AION_DOCTOR_LIB_PATH", "C:/__aion_missing__/nope.dll");
    std::env::set_var("AION_LIB_PATH", "C:/__aion_missing__/nope.dll");
    std::env::set_var("AION_DOCTOR_PYTHON_BIN", "python_missing_for_doctor");

    let out = output_bundle::write_product_doctor_output().expect("doctor output");
    let v = read_doctor_result_json(&out);
    let checks = v
        .get("checks")
        .and_then(|x| x.as_array())
        .expect("checks array");
    let names: Vec<&str> = checks
        .iter()
        .map(|c| c.get("check").and_then(|v| v.as_str()).unwrap_or(""))
        .collect();
    assert_eq!(
        names,
        vec!["ffi_check", "python_bindings_check", "policy_schema_check"]
    );
    let global = v
        .get("global_consistency")
        .expect("global_consistency block");
    assert!(global.get("run_finality").is_some());
    assert!(global.get("capsule_finality").is_some());
    assert!(global.get("evidence_finality").is_some());
    assert!(global.get("replay_finality").is_some());
    assert_eq!(
        v.get("os_contract_spec_version").and_then(|x| x.as_str()),
        Some(os_contract_spec_version().as_str())
    );
    let identity = v.get("os_identity").expect("os_identity block");
    assert!(identity.get("kernel_version").is_some());
    assert_eq!(
        identity
            .get("os_contract_spec_version")
            .and_then(|x| x.as_str()),
        Some(os_contract_spec_version().as_str())
    );
    assert_eq!(
        identity
            .get("global_consistency_contract_version")
            .and_then(|x| x.as_str()),
        Some(global_consistency_contract_version().as_str())
    );
    let compat = identity
        .get("compatibility_profile")
        .expect("compatibility_profile");
    let versions = compat
        .get("os_contract_spec_versions")
        .and_then(|x| x.as_array())
        .expect("os_contract_spec_versions");
    assert!(versions
        .iter()
        .any(|x| x.as_str() == Some(os_contract_spec_version().as_str())));
    let upgrade = v.get("upgrade_replay").expect("upgrade_replay block");
    assert!(upgrade.get("current_kernel_version").is_some());
    let targets = upgrade
        .get("tested_upgrade_targets")
        .and_then(|x| x.as_array())
        .expect("tested_upgrade_targets");
    assert_eq!(targets.len(), 2);
    let results = upgrade
        .get("results")
        .and_then(|x| x.as_array())
        .expect("upgrade results");
    assert_eq!(results.len(), 2);
    assert!(v.get("capsule_abi").is_some());
    assert!(v.get("trust_chain").is_some());
    assert!(v.get("runtime_isolation").is_some());
    assert!(v.get("observability").is_some());
    assert!(v.get("tenant_isolation").is_some());
    assert!(v.get("legal_determinism").is_some());
    assert!(v.get("contract_stability").is_some());
    assert!(v.get("current_contract_versions").is_some());
    assert!(v.get("compatibility_matrix").is_some());
    assert!(v.get("breaking_changes_detected").is_some());
    assert!(v.get("snapshot_hashes").is_some());
    assert!(v.get("build_fingerprint").is_some());
    assert!(v.get("release_signatures").is_some());
    assert!(v.get("provenance").is_some());
    assert!(v.get("sbom").is_some());
    assert!(v.get("vulnerability_status").is_some());
    assert!(v.get("security_model").is_some());
    assert!(v.get("threat_model").is_some());
    assert!(v.get("compliance_status").is_some());
    assert!(v.get("security_scanning").is_some());
    assert!(v.get("logging_policy").is_some());
    assert!(v.get("determinism_matrix").is_some());
    assert!(v.get("determinism_contract").is_some());
    assert!(v.get("replay_invariant_gate").is_some());
    assert!(v.get("slo_status").is_some());
    assert!(v.get("reliability_status").is_some());
    assert!(v.get("chaos_status").is_some());
    assert!(v.get("soak_status").is_some());
    assert!(v.get("runbooks").is_some());
    assert!(v.get("incident_model").is_some());
    assert!(v.get("dr_status").is_some());
    assert!(v.get("upgrade_migration_status").is_some());
    assert!(v.get("operations_model").is_some());
    assert!(v.get("distribution_status").is_some());
    assert!(v.get("identity_matrix").is_some());
    assert!(v.get("lts_policy").is_some());
    assert!(v.get("installer_trust_chain").is_some());
    assert!(v.get("distribution_model").is_some());
    assert!(v.get("policy_packs").is_some());
    assert!(v.get("policy_gates").is_some());
    assert!(v.get("policy_evidence").is_some());
    assert!(v.get("governance_model").is_some());
    assert!(v.get("api_stability").is_some());
    assert!(v.get("cli_stability").is_some());
    assert!(v.get("admin_docs").is_some());
    assert!(v.get("golden_paths").is_some());
    assert!(v.get("test_strategy").is_some());
    assert!(v.get("regression_matrix").is_some());
    assert!(v.get("compatibility_tests").is_some());
    assert!(v.get("fuzz_property_tests").is_some());
    assert!(v.get("metrics_contract").is_some());
    assert!(v.get("kpi_contract").is_some());
    assert!(v.get("audit_reports").is_some());
    assert!(v.get("evidence_export").is_some());
    assert!(v.get("measurement_model").is_some());
}
