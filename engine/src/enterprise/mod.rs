use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

fn enterprise_root() -> Result<PathBuf, String> {
    if let Ok(v) = std::env::var("SEALRUN_ENTERPRISE_ROOT") {
        let p = PathBuf::from(v);
        fs::create_dir_all(&p).map_err(|e| format!("create enterprise root: {e}"))?;
        return Ok(p);
    }
    let base = std::env::current_dir().map_err(|e| format!("cwd: {e}"))?;
    let p = base.join("sealrun_enterprise");
    fs::create_dir_all(&p).map_err(|e| format!("create enterprise root: {e}"))?;
    Ok(p)
}

fn tenants_root() -> Result<PathBuf, String> {
    let p = enterprise_root()?.join("tenants");
    fs::create_dir_all(&p).map_err(|e| format!("create tenants root: {e}"))?;
    Ok(p)
}

fn tenant_dir(tenant_id: &str) -> Result<PathBuf, String> {
    let p = tenants_root()?.join(tenant_id);
    Ok(p)
}

fn write_json_file<T: Serialize>(path: &Path, value: &T) -> Result<(), String> {
    let body = serde_json::to_string_pretty(value).map_err(|e| format!("json serialize: {e}"))?;
    fs::write(path, body).map_err(|e| format!("write {}: {e}", path.display()))
}

fn read_json_file<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, String> {
    let body = fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    serde_json::from_str(&body).map_err(|e| format!("parse {}: {e}", path.display()))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantRecord {
    pub id: String,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    pub days: u32,
}

impl Default for RetentionPolicy {
    fn default() -> Self {
        Self { days: 30 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantMeta {
    pub tenant: TenantRecord,
    pub retention: RetentionPolicy,
    pub legal_hold: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapsuleRecord {
    pub tenant_id: String,
    pub capsule_path: String,
    pub evidence_path: Option<String>,
    pub created_at: u64,
    pub labels: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceRecord {
    pub tenant_id: String,
    pub run_id: String,
    pub evidence_path: String,
    pub fields: BTreeMap<String, String>,
    pub created_at: u64,
}

fn now_secs() -> Result<u64, String> {
    let dur = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| format!("time: {e}"))?;
    Ok(dur.as_secs())
}

fn tenant_meta_path(tenant_id: &str) -> Result<PathBuf, String> {
    Ok(tenant_dir(tenant_id)?.join("tenant.json"))
}

fn capsules_index_path(tenant_id: &str) -> Result<PathBuf, String> {
    Ok(tenant_dir(tenant_id)?.join("capsules.index.json"))
}

fn evidence_index_path(tenant_id: &str) -> Result<PathBuf, String> {
    Ok(tenant_dir(tenant_id)?.join("evidence.index.json"))
}

fn ensure_tenant_exists(tenant_id: &str) -> Result<(), String> {
    let path = tenant_meta_path(tenant_id)?;
    if !path.exists() {
        return Err(format!("tenant_not_found:{tenant_id}"));
    }
    Ok(())
}

pub fn tenant_create(tenant_id: &str) -> Result<TenantMeta, String> {
    let dir = tenant_dir(tenant_id)?;
    if dir.exists() {
        return Err(format!("tenant_already_exists:{tenant_id}"));
    }
    fs::create_dir_all(&dir).map_err(|e| format!("create tenant dir: {e}"))?;
    fs::create_dir_all(dir.join("capsules")).map_err(|e| format!("create capsules dir: {e}"))?;
    fs::create_dir_all(dir.join("evidence")).map_err(|e| format!("create evidence dir: {e}"))?;
    let meta = TenantMeta {
        tenant: TenantRecord {
            id: tenant_id.to_string(),
            created_at: now_secs()?,
        },
        retention: RetentionPolicy::default(),
        legal_hold: false,
    };
    write_json_file(&tenant_meta_path(tenant_id)?, &meta)?;
    write_json_file(&capsules_index_path(tenant_id)?, &Vec::<CapsuleRecord>::new())?;
    write_json_file(&evidence_index_path(tenant_id)?, &Vec::<EvidenceRecord>::new())?;
    Ok(meta)
}

pub fn tenant_list() -> Result<Vec<TenantRecord>, String> {
    let mut out = Vec::new();
    let root = tenants_root()?;
    for entry in fs::read_dir(&root).map_err(|e| format!("read tenants root: {e}"))? {
        let entry = entry.map_err(|e| format!("read tenant entry: {e}"))?;
        if !entry.path().is_dir() {
            continue;
        }
        let meta_path = entry.path().join("tenant.json");
        if meta_path.exists() {
            let meta: TenantMeta = read_json_file(&meta_path)?;
            out.push(meta.tenant);
        }
    }
    out.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(out)
}

pub fn tenant_delete(tenant_id: &str) -> Result<(), String> {
    let meta: TenantMeta = read_json_file(&tenant_meta_path(tenant_id)?)?;
    if meta.legal_hold {
        return Err(format!("tenant_legal_hold_enabled:{tenant_id}"));
    }
    fs::remove_dir_all(tenant_dir(tenant_id)?).map_err(|e| format!("delete tenant: {e}"))
}

pub fn retention_get(tenant_id: &str) -> Result<RetentionPolicy, String> {
    let meta: TenantMeta = read_json_file(&tenant_meta_path(tenant_id)?)?;
    Ok(meta.retention)
}

pub fn retention_set(tenant_id: &str, days: u32) -> Result<TenantMeta, String> {
    let mut meta: TenantMeta = read_json_file(&tenant_meta_path(tenant_id)?)?;
    meta.retention.days = days;
    write_json_file(&tenant_meta_path(tenant_id)?, &meta)?;
    Ok(meta)
}

pub fn legal_hold_set(tenant_id: &str, enabled: bool) -> Result<TenantMeta, String> {
    let mut meta: TenantMeta = read_json_file(&tenant_meta_path(tenant_id)?)?;
    meta.legal_hold = enabled;
    write_json_file(&tenant_meta_path(tenant_id)?, &meta)?;
    Ok(meta)
}

pub fn tenant_capsule_register(
    tenant_id: &str,
    capsule_path: &Path,
    evidence_path: Option<&Path>,
    labels: Vec<String>,
) -> Result<CapsuleRecord, String> {
    ensure_tenant_exists(tenant_id)?;
    let mut index: Vec<CapsuleRecord> = read_json_file(&capsules_index_path(tenant_id)?)?;
    let rec = CapsuleRecord {
        tenant_id: tenant_id.to_string(),
        capsule_path: capsule_path.to_string_lossy().to_string(),
        evidence_path: evidence_path.map(|p| p.to_string_lossy().to_string()),
        created_at: now_secs()?,
        labels,
    };
    index.push(rec.clone());
    index.sort_by(|a, b| a.capsule_path.cmp(&b.capsule_path));
    write_json_file(&capsules_index_path(tenant_id)?, &index)?;
    Ok(rec)
}

pub fn tenant_capsule_list(tenant_id: &str) -> Result<Vec<CapsuleRecord>, String> {
    ensure_tenant_exists(tenant_id)?;
    read_json_file(&capsules_index_path(tenant_id)?)
}

pub fn tenant_replay_paths(tenant_id: &str, capsule_path: &str) -> Result<CapsuleRecord, String> {
    let index = tenant_capsule_list(tenant_id)?;
    index
        .into_iter()
        .find(|r| r.capsule_path == capsule_path)
        .ok_or_else(|| "capsule_not_registered_for_tenant".to_string())
}

pub fn evidence_register(
    tenant_id: &str,
    run_id: &str,
    evidence_path: &Path,
    fields: BTreeMap<String, String>,
) -> Result<EvidenceRecord, String> {
    ensure_tenant_exists(tenant_id)?;
    let mut index: Vec<EvidenceRecord> = read_json_file(&evidence_index_path(tenant_id)?)?;
    let rec = EvidenceRecord {
        tenant_id: tenant_id.to_string(),
        run_id: run_id.to_string(),
        evidence_path: evidence_path.to_string_lossy().to_string(),
        fields,
        created_at: now_secs()?,
    };
    index.push(rec.clone());
    index.sort_by(|a, b| a.run_id.cmp(&b.run_id));
    write_json_file(&evidence_index_path(tenant_id)?, &index)?;
    Ok(rec)
}

pub fn evidence_query(
    tenant_id: &str,
    field_key: Option<&str>,
    field_value: Option<&str>,
) -> Result<Vec<EvidenceRecord>, String> {
    ensure_tenant_exists(tenant_id)?;
    let mut rows: Vec<EvidenceRecord> = read_json_file(&evidence_index_path(tenant_id)?)?;
    if let (Some(k), Some(v)) = (field_key, field_value) {
        rows.retain(|r| r.fields.get(k).map(|x| x == v).unwrap_or(false));
    }
    Ok(rows)
}

pub fn lifecycle_purge(tenant_id: &str, now: u64) -> Result<Vec<String>, String> {
    let meta: TenantMeta = read_json_file(&tenant_meta_path(tenant_id)?)?;
    if meta.legal_hold {
        return Err("legal_hold_enabled".to_string());
    }
    let max_age = (meta.retention.days as u64) * 24 * 3600;
    let mut index: Vec<CapsuleRecord> = read_json_file(&capsules_index_path(tenant_id)?)?;
    let mut removed = Vec::new();
    index.retain(|row| {
        let expired = now.saturating_sub(row.created_at) > max_age;
        if expired {
            removed.push(row.capsule_path.clone());
            false
        } else {
            true
        }
    });
    write_json_file(&capsules_index_path(tenant_id)?, &index)?;
    Ok(removed)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum Permission {
    Replay,
    Diff,
    Purge,
    RetentionSet,
    LegalHold,
    TenantAdmin,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Admin,
    Auditor,
    Operator,
    Viewer,
}

fn role_permissions(role: &Role) -> BTreeSet<Permission> {
    match role {
        Role::Admin => BTreeSet::from([
            Permission::Replay,
            Permission::Diff,
            Permission::Purge,
            Permission::RetentionSet,
            Permission::LegalHold,
            Permission::TenantAdmin,
        ]),
        Role::Auditor => BTreeSet::from([Permission::Replay, Permission::Diff]),
        Role::Operator => BTreeSet::from([
            Permission::Replay,
            Permission::Diff,
            Permission::Purge,
            Permission::RetentionSet,
        ]),
        Role::Viewer => BTreeSet::from([Permission::Replay]),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RbacPolicyFile {
    pub assignments: BTreeMap<String, Role>,
}

fn rbac_policy_path() -> Result<PathBuf, String> {
    Ok(enterprise_root()?.join("rbac.policy.yaml"))
}

pub fn rbac_export() -> Result<RbacPolicyFile, String> {
    let p = rbac_policy_path()?;
    if !p.exists() {
        return Ok(RbacPolicyFile {
            assignments: BTreeMap::new(),
        });
    }
    let body = fs::read_to_string(&p).map_err(|e| format!("read rbac policy: {e}"))?;
    serde_yaml::from_str(&body).map_err(|e| format!("parse rbac policy yaml: {e}"))
}

pub fn rbac_assign(subject: &str, role: Role) -> Result<RbacPolicyFile, String> {
    let mut pol = rbac_export()?;
    pol.assignments.insert(subject.to_string(), role);
    let body = serde_yaml::to_string(&pol).map_err(|e| format!("yaml serialize: {e}"))?;
    fs::write(rbac_policy_path()?, body).map_err(|e| format!("write rbac policy: {e}"))?;
    Ok(pol)
}

pub fn rbac_check(subject: &str, permission: Permission) -> Result<bool, String> {
    let pol = rbac_export()?;
    let role = pol
        .assignments
        .get(subject)
        .ok_or_else(|| "subject_not_assigned".to_string())?;
    Ok(role_permissions(role).contains(&permission))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OidcConfig {
    pub client_id: String,
    pub device_authorization_endpoint: String,
    pub token_endpoint: String,
    pub scope: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OidcTokenStore {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub token_type: String,
    pub expires_in: u64,
    pub issued_at: u64,
}

fn auth_store_path() -> Result<PathBuf, String> {
    let p = enterprise_root()?.join("auth.tokens.json");
    Ok(p)
}

#[derive(Debug, Serialize, Deserialize)]
struct DeviceCodeResponse {
    device_code: String,
    user_code: String,
    verification_uri: String,
    expires_in: u64,
    interval: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u64,
    refresh_token: Option<String>,
}

pub fn oidc_login(config: &OidcConfig) -> Result<OidcTokenStore, String> {
    let client = reqwest::blocking::Client::new();
    let device: DeviceCodeResponse = client
        .post(&config.device_authorization_endpoint)
        .form(&[
            ("client_id", config.client_id.as_str()),
            ("scope", config.scope.as_str()),
        ])
        .send()
        .map_err(|e| format!("device auth request: {e}"))?
        .error_for_status()
        .map_err(|e| format!("device auth status: {e}"))?
        .json()
        .map_err(|e| format!("device auth decode: {e}"))?;

    println!(
        "Open {} and enter code {}",
        device.verification_uri, device.user_code
    );

    let interval = device.interval.unwrap_or(5).max(1);
    let start = now_secs()?;
    loop {
        let token_resp = client
            .post(&config.token_endpoint)
            .form(&[
                ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
                ("client_id", config.client_id.as_str()),
                ("device_code", device.device_code.as_str()),
            ])
            .send()
            .map_err(|e| format!("token request: {e}"))?;

        if token_resp.status().is_success() {
            let tok: TokenResponse = token_resp
                .json()
                .map_err(|e| format!("token decode: {e}"))?;
            let store = OidcTokenStore {
                access_token: tok.access_token,
                refresh_token: tok.refresh_token,
                token_type: tok.token_type,
                expires_in: tok.expires_in,
                issued_at: now_secs()?,
            };
            write_json_file(&auth_store_path()?, &store)?;
            return Ok(store);
        }

        let elapsed = now_secs()?.saturating_sub(start);
        if elapsed > device.expires_in {
            return Err("oidc_device_code_timeout".to_string());
        }
        std::thread::sleep(std::time::Duration::from_secs(interval));
    }
}

pub fn oidc_logout() -> Result<(), String> {
    let p = auth_store_path()?;
    if p.exists() {
        fs::remove_file(p).map_err(|e| format!("remove token file: {e}"))?;
    }
    Ok(())
}

pub fn oidc_status() -> Result<Value, String> {
    let p = auth_store_path()?;
    if !p.exists() {
        return Ok(serde_json::json!({"authenticated": false}));
    }
    let tok: OidcTokenStore = read_json_file(&p)?;
    let now = now_secs()?;
    let exp = tok.issued_at.saturating_add(tok.expires_in);
    Ok(serde_json::json!({
        "authenticated": true,
        "token_type": tok.token_type,
        "expires_at": exp,
        "expired": now >= exp
    }))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SinkConfig {
    pub endpoint: String,
    pub token: String,
}

fn post_json(endpoint: &str, token: &str, body: &Value) -> Result<u16, String> {
    let client = reqwest::blocking::Client::new();
    let status = client
        .post(endpoint)
        .bearer_auth(token)
        .json(body)
        .send()
        .map_err(|e| format!("send sink event: {e}"))?
        .status();
    Ok(status.as_u16())
}

pub fn send_splunk_hec(cfg: &SinkConfig, event: &Value) -> Result<u16, String> {
    let payload = serde_json::json!({ "event": event });
    post_json(&cfg.endpoint, &cfg.token, &payload)
}

pub fn send_datadog_logs(cfg: &SinkConfig, event: &Value) -> Result<u16, String> {
    let payload = serde_json::json!([event]);
    post_json(&cfg.endpoint, &cfg.token, &payload)
}

pub fn send_elastic_ingest(cfg: &SinkConfig, event: &Value) -> Result<u16, String> {
    let payload = serde_json::json!({ "docs": [event] });
    post_json(&cfg.endpoint, &cfg.token, &payload)
}

pub fn export_otel(endpoint: &str, event: &Value) -> Result<u16, String> {
    let body = serde_json::json!({
        "resourceLogs": [{
            "resource": {"attributes": [{"key": "service.name", "value": {"stringValue":"sealrun"}}]},
            "scopeLogs": [{
                "scope": {"name":"sealrun.enterprise"},
                "logRecords": [{
                    "severityText":"INFO",
                    "body":{"stringValue": serde_json::to_string(event).unwrap_or_else(|_| "{}".into())}
                }]
            }]
        }]
    });
    let client = reqwest::blocking::Client::new();
    let status = client
        .post(endpoint)
        .json(&body)
        .send()
        .map_err(|e| format!("otel export: {e}"))?
        .status();
    Ok(status.as_u16())
}

pub fn cosign_sign(artifact: &Path) -> Result<String, String> {
    let out = Command::new("cosign")
        .arg("sign-blob")
        .arg("--yes")
        .arg(artifact)
        .output()
        .map_err(|e| format!("spawn cosign: {e}"))?;
    if !out.status.success() {
        return Err(format!("cosign sign failed: {}", String::from_utf8_lossy(&out.stderr)));
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

pub fn cosign_verify(artifact: &Path, signature: &Path, pubkey: &Path) -> Result<String, String> {
    let out = Command::new("cosign")
        .arg("verify-blob")
        .arg("--key")
        .arg(pubkey)
        .arg("--signature")
        .arg(signature)
        .arg(artifact)
        .output()
        .map_err(|e| format!("spawn cosign verify: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "cosign verify failed: {}",
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

pub fn cargo_sbom_json() -> Result<Value, String> {
    let out = Command::new("cargo")
        .arg("sbom")
        .arg("--output-format")
        .arg("json")
        .output()
        .map_err(|e| format!("spawn cargo-sbom: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "cargo sbom failed: {}",
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    serde_json::from_slice(&out.stdout).map_err(|e| format!("parse cargo sbom json: {e}"))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernancePolicy {
    pub allowed_models: Vec<String>,
    pub allowed_seeds: Vec<u64>,
    pub allowed_external_calls: Vec<String>,
    pub required_evidence_fields: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceEvalInput {
    pub model: String,
    pub seed: u64,
    pub external_calls: Vec<String>,
    pub evidence_fields: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceEvalResult {
    pub ok: bool,
    pub violations: Vec<String>,
}

pub fn policy_validate(policy: &GovernancePolicy) -> GovernanceEvalResult {
    let mut violations = Vec::new();
    if policy.allowed_models.is_empty() {
        violations.push("allowed_models_empty".to_string());
    }
    if policy.required_evidence_fields.is_empty() {
        violations.push("required_evidence_fields_empty".to_string());
    }
    GovernanceEvalResult {
        ok: violations.is_empty(),
        violations,
    }
}

pub fn policy_evaluate(policy: &GovernancePolicy, input: &GovernanceEvalInput) -> GovernanceEvalResult {
    let mut violations = Vec::new();
    if !policy.allowed_models.iter().any(|m| m == &input.model) {
        violations.push("model_not_allowed".to_string());
    }
    if !policy.allowed_seeds.iter().any(|s| s == &input.seed) {
        violations.push("seed_not_allowed".to_string());
    }
    for call in &input.external_calls {
        if !policy.allowed_external_calls.iter().any(|x| x == call) {
            violations.push(format!("external_call_not_allowed:{call}"));
        }
    }
    for field in &policy.required_evidence_fields {
        if !input.evidence_fields.contains_key(field) {
            violations.push(format!("missing_evidence_field:{field}"));
        }
    }
    GovernanceEvalResult {
        ok: violations.is_empty(),
        violations,
    }
}

pub fn write_secure_token_file(path: &Path, token: &OidcTokenStore) -> Result<(), String> {
    let body = serde_json::to_vec(token).map_err(|e| format!("token serialize: {e}"))?;
    let mut file = fs::File::create(path).map_err(|e| format!("token create: {e}"))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        file.set_permissions(fs::Permissions::from_mode(0o600))
            .map_err(|e| format!("token chmod: {e}"))?;
    }
    file.write_all(&body)
        .map_err(|e| format!("token write: {e}"))
}

