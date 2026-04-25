use aion_cli::output_bundle;
use serde_json::Value;
use std::sync::Mutex;

static ENV_LOCK: Mutex<()> = Mutex::new(());

fn read_data(path: &std::path::Path) -> Value {
    let body = std::fs::read_to_string(path.join("result.json")).expect("read result");
    let envelope: Value = serde_json::from_str(&body).expect("parse json");
    envelope.get("data").cloned().expect("envelope data")
}

fn nested_data(v: &Value) -> &Value {
    v.get("data").unwrap_or(v)
}

#[test]
fn enterprise_outputs_are_structured() {
    let auth = output_bundle::write_enterprise_auth_output().expect("auth");
    let events = output_bundle::write_enterprise_audit_events_output().expect("events");
    let trust = output_bundle::write_enterprise_trust_center_output().expect("trust");
    let attestation =
        output_bundle::write_enterprise_release_attestation_output().expect("release attestation");
    let otel = output_bundle::write_enterprise_otel_output().expect("otel");
    let sinks = output_bundle::write_enterprise_sinks_output().expect("sinks");
    let policy_api = output_bundle::write_enterprise_policy_api_output().expect("policy api");
    let refs = output_bundle::write_enterprise_references_output().expect("refs");

    assert!(nested_data(&read_data(&auth)).get("sso").is_some());
    assert!(nested_data(&read_data(&auth)).get("rbac").is_some());
    assert!(nested_data(&read_data(&events))
        .get("governance_events")
        .is_some());
    assert!(nested_data(&read_data(&trust))
        .get("compliance_roadmap")
        .is_some());
    assert!(nested_data(&read_data(&attestation))
        .get("release_signature")
        .is_some());
    assert!(nested_data(&read_data(&attestation)).get("sbom").is_some());
    assert!(nested_data(&read_data(&otel)).get("otel_profile").is_some());
    assert!(nested_data(&read_data(&sinks)).get("supported_sinks").is_some());
    assert!(nested_data(&read_data(&policy_api)).get("endpoints").is_some());
    assert!(nested_data(&read_data(&policy_api))
        .get("governance_model")
        .is_some());
    assert!(nested_data(&read_data(&refs)).get("pilot_references").is_some());
}

#[test]
fn enterprise_tenant_lifecycle_and_rbac_outputs_are_structured() {
    let _g = ENV_LOCK.lock().expect("env lock");
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let root = std::env::temp_dir().join(format!("sealrun-cli-enterprise-{stamp}"));
    std::env::set_var("SEALRUN_ENTERPRISE_ROOT", &root);

    let create = output_bundle::write_enterprise_tenants_create_output("tenant1").expect("create");
    let list = output_bundle::write_enterprise_tenants_list_output().expect("list");
    let retention =
        output_bundle::write_enterprise_lifecycle_retention_set_output("tenant1", 7).expect("retention");
    let hold = output_bundle::write_enterprise_lifecycle_legal_hold_output("tenant1", true).expect("hold");
    let rbac = output_bundle::write_enterprise_rbac_assign_output("alice", "admin").expect("rbac assign");
    let check = output_bundle::write_enterprise_rbac_check_output("alice", "tenant-admin").expect("rbac check");
    let export = output_bundle::write_enterprise_rbac_export_output().expect("rbac export");

    assert!(nested_data(&read_data(&create)).get("tenant").is_some());
    assert!(nested_data(&read_data(&list)).get("tenants").is_some());
    assert!(nested_data(&read_data(&retention))
        .get("tenant_meta")
        .is_some());
    assert!(nested_data(&read_data(&hold)).get("tenant_meta").is_some());
    assert!(nested_data(&read_data(&rbac)).get("policy").is_some());
    assert!(nested_data(&read_data(&check)).get("allowed").is_some());
    assert!(nested_data(&read_data(&export)).get("policy").is_some());

    let _ = std::fs::remove_dir_all(root);
}
