use aion_engine::enterprise::{
    self, evidence_query, lifecycle_purge, oidc_login, policy_evaluate, policy_validate, rbac_assign, rbac_check,
    tenant_capsule_list, tenant_capsule_register, tenant_create, tenant_delete, tenant_list, GovernanceEvalInput,
    GovernancePolicy, OidcConfig, Permission, Role, SinkConfig,
};
use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::PathBuf;
use std::sync::Mutex;

static ENV_LOCK: Mutex<()> = Mutex::new(());

fn temp_root(name: &str) -> PathBuf {
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    std::env::temp_dir().join(format!("sealrun-enterprise-{name}-{stamp}"))
}

fn with_enterprise_root<T>(name: &str, f: impl FnOnce(PathBuf) -> T) -> T {
    let _g = ENV_LOCK.lock().expect("env lock");
    let root = temp_root(name);
    std::env::set_var("SEALRUN_ENTERPRISE_ROOT", &root);
    let out = f(root.clone());
    let _ = std::fs::remove_dir_all(root);
    out
}

#[test]
fn tenant_isolation_and_listing_work() {
    with_enterprise_root("tenant-iso", |_| {
        tenant_create("t1").expect("create t1");
        tenant_create("t2").expect("create t2");
        let tenants = tenant_list().expect("list tenants");
        assert_eq!(tenants.len(), 2);

        let cap_path = std::env::temp_dir().join("cap-t1.aionai");
        std::fs::write(&cap_path, "{\"capsule\":\"x\"}\n").expect("write capsule");
        tenant_capsule_register("t1", &cap_path, None, vec!["prod".into()]).expect("register");
        let t1_caps = tenant_capsule_list("t1").expect("t1 caps");
        let t2_caps = tenant_capsule_list("t2").expect("t2 caps");
        assert_eq!(t1_caps.len(), 1);
        assert!(t2_caps.is_empty());
    });
}

#[test]
fn lifecycle_retention_and_purge_work() {
    with_enterprise_root("lifecycle", |_| {
        tenant_create("tenant-a").expect("create tenant");
        enterprise::retention_set("tenant-a", 1).expect("set retention");
        let cap_path = std::env::temp_dir().join("cap-old.aionai");
        std::fs::write(&cap_path, "{}\n").expect("write capsule");
        tenant_capsule_register("tenant-a", &cap_path, None, vec![]).expect("register");
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
            + 3 * 24 * 3600;
        let removed = lifecycle_purge("tenant-a", now).expect("purge");
        assert_eq!(removed.len(), 1);
    });
}

#[test]
fn rbac_evaluator_works() {
    with_enterprise_root("rbac", |_| {
        rbac_assign("alice", Role::Admin).expect("assign admin");
        rbac_assign("bob", Role::Viewer).expect("assign viewer");
        assert!(rbac_check("alice", Permission::TenantAdmin).expect("check alice"));
        assert!(rbac_check("bob", Permission::Replay).expect("check bob"));
        assert!(!rbac_check("bob", Permission::Purge).expect("check bob purge"));
    });
}

fn spawn_device_flow_server() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind listener");
    let addr = listener.local_addr().expect("listener addr");
    std::thread::spawn(move || {
        for idx in 0..2usize {
            let (mut stream, _) = listener.accept().expect("accept");
            let mut buf = [0u8; 4096];
            let _ = stream.read(&mut buf).expect("read request");
            let response_body = if idx == 0 {
                r#"{"device_code":"dev-code","user_code":"ABCD-EFGH","verification_uri":"https://example.test/verify","expires_in":60,"interval":1}"#
            } else {
                r#"{"access_token":"at","token_type":"Bearer","expires_in":3600,"refresh_token":"rt"}"#
            };
            let resp = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                response_body.len(),
                response_body
            );
            stream.write_all(resp.as_bytes()).expect("write response");
        }
    });
    format!("http://{}", addr)
}

#[test]
fn oidc_login_mock_flow_works() {
    with_enterprise_root("oidc", |_| {
        let base = spawn_device_flow_server();
        let cfg = OidcConfig {
            client_id: "client".into(),
            device_authorization_endpoint: format!("{base}/device"),
            token_endpoint: format!("{base}/token"),
            scope: "openid profile email".into(),
        };
        let tok = oidc_login(&cfg).expect("oidc login");
        assert_eq!(tok.token_type, "Bearer");
        let status = enterprise::oidc_status().expect("oidc status");
        assert_eq!(status["authenticated"], true);
    });
}

#[test]
fn sinks_and_otel_exporters_send_http() {
    with_enterprise_root("sinks", |_| {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind sink listener");
        let addr = listener.local_addr().expect("addr");
        std::thread::spawn(move || {
            for _ in 0..4usize {
                let (mut stream, _) = listener.accept().expect("accept");
                let mut buf = [0u8; 4096];
                let _ = stream.read(&mut buf).expect("read req");
                let resp = "HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n";
                stream.write_all(resp.as_bytes()).expect("write");
            }
        });
        let cfg = SinkConfig {
            endpoint: format!("http://{}", addr),
            token: "token".into(),
        };
        let event = serde_json::json!({"kind":"test"});
        assert_eq!(enterprise::send_splunk_hec(&cfg, &event).expect("splunk"), 200);
        assert_eq!(enterprise::send_datadog_logs(&cfg, &event).expect("dd"), 200);
        assert_eq!(enterprise::send_elastic_ingest(&cfg, &event).expect("elastic"), 200);
        assert_eq!(
            enterprise::export_otel(&format!("http://{}", addr), &event).expect("otel"),
            200
        );
    });
}

#[test]
fn attestation_command_failure_is_reported() {
    let fake = std::env::temp_dir().join("fake-artifact.bin");
    std::fs::write(&fake, b"abc").expect("write fake artifact");
    let err = enterprise::cosign_sign(&fake).expect_err("cosign should fail without setup");
    assert!(!err.is_empty());
}

#[test]
fn policy_engine_validate_and_evaluate_work() {
    let policy = GovernancePolicy {
        allowed_models: vec!["gpt-4o-mini".into()],
        allowed_seeds: vec![1, 2, 42],
        allowed_external_calls: vec!["https://api.example.com".into()],
        required_evidence_fields: vec!["trace_id".into(), "policy_id".into()],
    };
    let valid = policy_validate(&policy);
    assert!(valid.ok);

    let mut fields = BTreeMap::new();
    fields.insert("trace_id".into(), "t-1".into());
    fields.insert("policy_id".into(), "p-1".into());
    let input = GovernanceEvalInput {
        model: "gpt-4o-mini".into(),
        seed: 42,
        external_calls: vec!["https://api.example.com".into()],
        evidence_fields: fields,
    };
    let eval = policy_evaluate(&policy, &input);
    assert!(eval.ok);
}

#[test]
fn tenant_evidence_query_filters_by_metadata() {
    with_enterprise_root("evidence", |_| {
        tenant_create("tenant-e").expect("create tenant");
        let ev_path = std::env::temp_dir().join("ev-tenant-e.aionevidence");
        std::fs::write(&ev_path, "{}\n").expect("write evidence");
        let mut fields = BTreeMap::new();
        fields.insert("model".into(), "gpt-4o-mini".into());
        enterprise::evidence_register("tenant-e", "run1", &ev_path, fields).expect("register ev");
        let rows = evidence_query("tenant-e", Some("model"), Some("gpt-4o-mini")).expect("query");
        assert_eq!(rows.len(), 1);
    });
}

#[test]
fn tenant_delete_is_blocked_by_legal_hold() {
    with_enterprise_root("tenant-delete-hold", |_| {
        tenant_create("tenant-h").expect("create tenant");
        enterprise::legal_hold_set("tenant-h", true).expect("set hold");
        let err = tenant_delete("tenant-h").expect_err("delete should be blocked");
        assert!(err.contains("legal_hold"));
    });
}
