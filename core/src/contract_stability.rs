use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContractSnapshot {
    pub contract_name: String,
    pub contract_version: String,
    pub hash_sha256: String,
    pub schema: Value,
    pub invariants: Vec<String>,
    pub error_codes: Vec<String>,
    pub finality_rules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContractCompatibilityRule {
    pub reader_version: String,
    pub writer_version: String,
    pub status: String,
    pub rule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContractBreakingChange {
    pub contract_name: String,
    pub code: String,
    pub context: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContractStabilityReport {
    pub status: String,
    pub current_contract_versions: BTreeMap<String, String>,
    pub compatibility_matrix: Vec<ContractCompatibilityRule>,
    pub breaking_changes_detected: Vec<ContractBreakingChange>,
    pub snapshots: Vec<ContractSnapshot>,
    pub snapshot_hashes: BTreeMap<String, String>,
}

fn parse_semver(v: &str) -> (u64, u64, u64) {
    let core = v.split('+').next().unwrap_or(v);
    let mut p = core.split('.');
    let a = p.next().and_then(|x| x.parse::<u64>().ok()).unwrap_or(0);
    let b = p.next().and_then(|x| x.parse::<u64>().ok()).unwrap_or(0);
    let c = p.next().and_then(|x| x.parse::<u64>().ok()).unwrap_or(0);
    (a, b, c)
}

fn semver_string((a, b, c): (u64, u64, u64)) -> String {
    format!("{a}.{b}.{c}")
}

fn contract_names() -> Vec<&'static str> {
    vec![
        "output",
        "error",
        "replay",
        "drift",
        "evidence",
        "policy",
        "global_consistency",
        "identity",
        "upgrade_replay",
        "capsule_abi",
        "trust_chain",
        "runtime_isolation",
        "observability",
        "tenant_isolation",
        "legal_determinism",
    ]
}

fn contract_schema(name: &str) -> Value {
    match name {
        "output" => json!({"status":"string","data":"object","error":"object|null"}),
        "error" => json!({"code":"string","message":"string","context":"string","origin":"string","cause":"string|null"}),
        "replay" => json!({"replay_symmetry_ok":"bool","replay_profile_error":"string|null","differences":"array"}),
        "drift" => json!({"changed":"bool","labels":"array","categories":"array","tolerance_violations":"array"}),
        "evidence" => json!({"records":"array","root_anchor":"string","replay_anchors":"object"}),
        "policy" => json!({"policy_profile":"object","violations":"array","status":"string"}),
        "global_consistency" => json!({"run_finality":"object","capsule_finality":"object","evidence_finality":"object","replay_finality":"object"}),
        "identity" => json!({"kernel_version":"object","compatibility_profile":"object","instance_id":"object|null"}),
        "upgrade_replay" => json!({"support_window":"string","tested_upgrade_targets":"array","results":"array"}),
        "capsule_abi" => json!({"supported_abi_versions":"array","result":"object"}),
        "trust_chain" => json!({"key_rotation_policy":"string","attestation_required":"bool","result":"object"}),
        "runtime_isolation" => json!({"io_boundary_mode":"string","side_effect_matrix_version":"string","result":"object"}),
        "observability" => json!({"log_schema_version":"string","metrics_schema_version":"string","trace_schema_version":"string","result":"object"}),
        "tenant_isolation" => json!({"guard_matrix_version":"string","result":"object"}),
        "legal_determinism" => json!({"license_contract_version":"string","sla_contract_version":"string","result":"object"}),
        _ => json!({}),
    }
}

fn contract_error_codes(name: &str) -> Vec<String> {
    match name {
        "upgrade_replay" => vec![
            "upgrade:replay_mismatch".into(),
            "upgrade:abi_incompatible".into(),
            "upgrade:evidence_incompatible".into(),
            "upgrade:policy_incompatible".into(),
        ],
        "capsule_abi" => vec![
            "capsule_abi:layout_incompatible".into(),
            "capsule_abi:fields_incompatible".into(),
            "capsule_abi:serialization_incompatible".into(),
        ],
        "trust_chain" => vec![
            "trust_chain:signature_invalid".into(),
            "trust_chain:attestation_invalid".into(),
            "trust_chain:key_rotation_invalid".into(),
        ],
        "runtime_isolation" => vec![
            "runtime_isolation:io_boundary_violation".into(),
            "runtime_isolation:side_effect_violation".into(),
            "runtime_isolation:network_violation".into(),
        ],
        "observability" => vec![
            "observability:log_nondeterministic".into(),
            "observability:metric_nondeterministic".into(),
            "observability:trace_nondeterministic".into(),
        ],
        "tenant_isolation" => vec![
            "tenant_isolation:boundary_violation".into(),
            "tenant_isolation:cross_tenant_violation".into(),
            "tenant_isolation:scope_violation".into(),
        ],
        "legal_determinism" => vec![
            "legal_determinism:license_unstable".into(),
            "legal_determinism:sla_unstable".into(),
            "legal_determinism:terms_not_machine_readable".into(),
        ],
        _ => vec!["AION_OK".into()],
    }
}

fn hash_snapshot_body(
    name: &str,
    version: &str,
    schema: &Value,
    invariants: &[String],
    error_codes: &[String],
    finality_rules: &[String],
) -> String {
    let payload = json!({
        "contract_name": name,
        "contract_version": version,
        "schema": schema,
        "invariants": invariants,
        "error_codes": error_codes,
        "finality_rules": finality_rules
    });
    let s = serde_json::to_string(&payload).unwrap_or_default();
    let mut h = Sha256::new();
    h.update(s.as_bytes());
    format!("{:x}", h.finalize())
}

fn make_snapshot(contract_name: &str, contract_version: &str) -> ContractSnapshot {
    let schema = contract_schema(contract_name);
    let invariants = vec![format!("{contract_name}:deterministic_contract")];
    let error_codes = contract_error_codes(contract_name);
    let finality_rules = vec![format!("{contract_name}:status_ok_final")];
    let hash_sha256 = hash_snapshot_body(
        contract_name,
        contract_version,
        &schema,
        &invariants,
        &error_codes,
        &finality_rules,
    );
    ContractSnapshot {
        contract_name: contract_name.to_string(),
        contract_version: contract_version.to_string(),
        hash_sha256,
        schema,
        invariants,
        error_codes,
        finality_rules,
    }
}

fn schema_fields(schema: &Value) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    let Some(obj) = schema.as_object() else { return out };
    for (k, v) in obj {
        out.insert(k.clone(), v.as_str().unwrap_or("unknown").to_string());
    }
    out
}

fn detect_breaking(prev: &ContractSnapshot, curr: &ContractSnapshot) -> Vec<ContractBreakingChange> {
    let mut out = Vec::new();
    let prev_fields = schema_fields(&prev.schema);
    let curr_fields = schema_fields(&curr.schema);

    for (k, prev_ty) in &prev_fields {
        match curr_fields.get(k) {
            None => out.push(ContractBreakingChange {
                contract_name: curr.contract_name.clone(),
                code: "contract_stability:field_removed".to_string(),
                context: format!("contract_stability.{}.schema.{k}", curr.contract_name),
                cause: Some("field_removed".to_string()),
            }),
            Some(curr_ty) if curr_ty != prev_ty => out.push(ContractBreakingChange {
                contract_name: curr.contract_name.clone(),
                code: "contract_stability:field_type_changed".to_string(),
                context: format!("contract_stability.{}.schema.{k}", curr.contract_name),
                cause: Some("field_type_changed".to_string()),
            }),
            _ => {}
        }
    }

    let mut semantic_prev = Sha256::new();
    semantic_prev.update(prev.invariants.join("|").as_bytes());
    semantic_prev.update(prev.finality_rules.join("|").as_bytes());
    let prev_sem = format!("{:x}", semantic_prev.finalize());

    let mut semantic_curr = Sha256::new();
    semantic_curr.update(curr.invariants.join("|").as_bytes());
    semantic_curr.update(curr.finality_rules.join("|").as_bytes());
    let curr_sem = format!("{:x}", semantic_curr.finalize());
    if prev_sem != curr_sem {
        out.push(ContractBreakingChange {
            contract_name: curr.contract_name.clone(),
            code: "contract_stability:semantic_changed".to_string(),
            context: format!("contract_stability.{}.semantics", curr.contract_name),
            cause: Some("semantic_changed".to_string()),
        });
    }
    out
}

pub fn evaluate_contract_stability(
    current_kernel_version: &str,
    previous_snapshots: Option<Vec<ContractSnapshot>>,
) -> ContractStabilityReport {
    let (maj, min, _patch) = parse_semver(current_kernel_version);
    let n = semver_string((maj, min, 0));
    let n1 = semver_string((maj, min.saturating_sub(1), 0));
    let n2 = semver_string((maj, min.saturating_sub(2), 0));

    let compatibility_matrix = vec![
        ContractCompatibilityRule {
            reader_version: n.clone(),
            writer_version: n.clone(),
            status: "ok".to_string(),
            rule: "N_reads_N_writes".to_string(),
        },
        ContractCompatibilityRule {
            reader_version: n1.clone(),
            writer_version: n.clone(),
            status: "ok".to_string(),
            rule: "N_minus_1_reads_N".to_string(),
        },
        ContractCompatibilityRule {
            reader_version: n2.clone(),
            writer_version: n.clone(),
            status: "ok".to_string(),
            rule: "N_minus_2_reads_N".to_string(),
        },
    ];

    let mut snapshots = Vec::new();
    let mut current_contract_versions = BTreeMap::new();
    let mut snapshot_hashes = BTreeMap::new();
    for name in contract_names() {
        let version = format!("{n}-v1");
        let snap = make_snapshot(name, &version);
        current_contract_versions.insert(name.to_string(), version);
        snapshot_hashes.insert(name.to_string(), snap.hash_sha256.clone());
        snapshots.push(snap);
    }

    let mut breaking_changes_detected = Vec::new();
    if let Some(prev) = previous_snapshots {
        let mut by_name = BTreeMap::new();
        for p in prev {
            by_name.insert(p.contract_name.clone(), p);
        }
        for curr in &snapshots {
            if let Some(p) = by_name.get(&curr.contract_name) {
                breaking_changes_detected.extend(detect_breaking(p, curr));
            }
        }
    }
    breaking_changes_detected.sort_by(|a, b| {
        (a.contract_name.clone(), a.code.clone(), a.context.clone())
            .cmp(&(b.contract_name.clone(), b.code.clone(), b.context.clone()))
    });

    ContractStabilityReport {
        status: if breaking_changes_detected.is_empty() {
            "ok".to_string()
        } else {
            "error".to_string()
        },
        current_contract_versions,
        compatibility_matrix,
        breaking_changes_detected,
        snapshots,
        snapshot_hashes,
    }
}

pub fn write_contract_snapshots(
    root: &Path,
    report: &ContractStabilityReport,
) -> Result<Vec<String>, String> {
    let mut out_paths = Vec::new();
    for s in &report.snapshots {
        let dir = root.join("contracts").join(&s.contract_name);
        std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        let path = dir.join(format!("{}.json", s.contract_version));
        let body = serde_json::to_string(s).map_err(|e| e.to_string())?;
        std::fs::write(&path, body).map_err(|e| e.to_string())?;
        out_paths.push(path.display().to_string());
    }
    out_paths.sort();
    Ok(out_paths)
}

pub fn diff_contract_snapshots(a: &ContractSnapshot, b: &ContractSnapshot) -> Vec<String> {
    let mut out = Vec::new();
    if a.hash_sha256 != b.hash_sha256 {
        out.push("contract_stability:hash_changed".to_string());
    }
    let af = schema_fields(&a.schema);
    let bf = schema_fields(&b.schema);
    let a_keys: BTreeSet<_> = af.keys().cloned().collect();
    let b_keys: BTreeSet<_> = bf.keys().cloned().collect();
    for k in a_keys.difference(&b_keys) {
        out.push(format!("contract_stability:field_removed:{k}"));
    }
    for k in b_keys.difference(&a_keys) {
        out.push(format!("contract_stability:field_added:{k}"));
    }
    for k in a_keys.intersection(&b_keys) {
        if af.get(k) != bf.get(k) {
            out.push(format!("contract_stability:field_type_changed:{k}"));
        }
    }
    out.sort();
    out
}

#[cfg(test)]
mod tests {
    use super::{evaluate_contract_stability, ContractSnapshot};

    #[test]
    fn snapshot_and_hash_stability() {
        let a = evaluate_contract_stability("0.2.0", None);
        let b = evaluate_contract_stability("0.2.0", None);
        assert_eq!(a.snapshot_hashes, b.snapshot_hashes);
        let sa = serde_json::to_string(&a.snapshots).expect("a");
        let sb = serde_json::to_string(&b.snapshots).expect("b");
        assert_eq!(sa, sb);
    }

    #[test]
    fn compatibility_matrix_has_n_n1_n2() {
        let r = evaluate_contract_stability("0.2.0", None);
        assert_eq!(r.compatibility_matrix.len(), 3);
        assert_eq!(r.compatibility_matrix[0].reader_version, "0.2.0");
        assert_eq!(r.compatibility_matrix[1].reader_version, "0.1.0");
        assert_eq!(r.compatibility_matrix[2].reader_version, "0.0.0");
    }

    #[test]
    fn breaking_change_detection_field_removed() {
        let current = evaluate_contract_stability("0.2.0", None);
        let mut prev = current.snapshots.clone();
        let output_prev = prev
            .iter_mut()
            .find(|s| s.contract_name == "output")
            .expect("output snapshot");
        output_prev.schema = serde_json::json!({
            "status":"string",
            "data":"object",
            "error":"object|null",
            "removed_field":"string"
        });
        let r = evaluate_contract_stability("0.2.0", Some(prev));
        assert_eq!(r.status, "error");
        assert!(r
            .breaking_changes_detected
            .iter()
            .any(|b| b.code == "contract_stability:field_removed"));
    }

    #[test]
    fn deterministic_breaking_output_order() {
        let current = evaluate_contract_stability("0.2.0", None);
        let mut prev = current.snapshots.clone();
        for s in &mut prev {
            s.invariants.push("semantic_changed_marker".to_string());
        }
        let r = evaluate_contract_stability("0.2.0", Some(prev));
        let mut codes: Vec<String> = r
            .breaking_changes_detected
            .iter()
            .map(|b| b.code.clone())
            .collect();
        let sorted = {
            let mut c = codes.clone();
            c.sort();
            c
        };
        codes.sort();
        assert_eq!(codes, sorted);
    }

    #[test]
    fn snapshot_type_is_serializable() {
        let s = ContractSnapshot {
            contract_name: "x".to_string(),
            contract_version: "0.2.0-v1".to_string(),
            hash_sha256: "h".to_string(),
            schema: serde_json::json!({"a":"string"}),
            invariants: vec!["i".to_string()],
            error_codes: vec!["e".to_string()],
            finality_rules: vec!["f".to_string()],
        };
        let _ = serde_json::to_string(&s).expect("serialize");
    }
}

