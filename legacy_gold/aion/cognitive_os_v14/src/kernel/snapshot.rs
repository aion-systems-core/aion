//! Deterministic, hash-based snapshots of [`super::system_truth::SystemTruth`] + execution envelope metadata.

pub use crate::system_state::SystemSnapshot;

use super::envelope::ExecutionEnvelope;
use super::system_truth::SystemTruth;
use crate::config::Config;
use crate::evidence_engine::EvidenceIndex;
use crate::hardware::HardwareProfiles;
use crate::map_engine::MapModel;
use crate::policy::model::PolicySet;
use crate::process::id::ProcessId;
use crate::process::model::ProcessModel;
use crate::process::registry::ProcessRegistry;
use crate::state::State;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

fn sort_value_keys(v: &Value) -> Value {
    match v {
        Value::Object(map) => {
            let mut keys: Vec<String> = map.keys().cloned().collect();
            keys.sort();
            let mut out = serde_json::Map::new();
            for k in keys {
                if let Some(inner) = map.get(&k) {
                    out.insert(k, sort_value_keys(inner));
                }
            }
            Value::Object(out)
        }
        Value::Array(items) => Value::Array(items.iter().map(sort_value_keys).collect()),
        _ => v.clone(),
    }
}

fn canonical_json_string(v: &Value) -> Result<String> {
    Ok(sort_value_keys(v).to_string())
}

fn sha256_hex_of_canonical(v: &Value) -> Result<String> {
    let s = canonical_json_string(v)?;
    Ok(hex::encode(Sha256::digest(s.as_bytes())))
}

/// Snapshot files live next to the checkpoint DB: `<parent of checkpoint_path>/snapshots/`.
/// Uses existing [`Config::checkpoint_path`] only (no new config fields).
pub fn snapshots_directory(config: &Config) -> PathBuf {
    Path::new(&config.checkpoint_path)
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("snapshots")
}

/// On-disk process registry only (no in-memory `SystemTruth` overlays).
/// Public API per Phase 3; `cargo clippy` on the library target does not see `#[cfg(test)]` callers.
#[allow(dead_code)]
pub fn hash_processes(registry: &ProcessRegistry) -> String {
    let _ = registry;
    let empty: HashMap<ProcessId, ProcessModel> = HashMap::new();
    let mut ids = ProcessRegistry::list().unwrap_or_default();
    ids.sort_by(|a, b| a.as_str().cmp(b.as_str()));
    let arr = build_process_entries(&ids, HashMapLite::WithWrites(&empty));
    let v = Value::Array(arr);
    sha256_hex_of_canonical(&v).unwrap_or_else(|_| String::new())
}

enum HashMapLite<'a> {
    WithWrites(&'a HashMap<ProcessId, ProcessModel>),
}

fn build_process_entries(ids: &[ProcessId], writes: HashMapLite<'_>) -> Vec<Value> {
    let mut arr = Vec::new();
    for id in ids {
        let model: Option<ProcessModel> = match &writes {
            HashMapLite::WithWrites(m) => m.get(id).cloned().or_else(|| ProcessRegistry::load(id).ok()),
        };
        let Some(model) = model else {
            continue;
        };
        arr.push(serde_json::json!({
            "id": id.to_string(),
            "model": serde_json::to_value(&model).unwrap_or(Value::Null),
        }));
    }
    arr
}

fn process_ids_for_truth(truth: &SystemTruth) -> Vec<ProcessId> {
    let mut ids: Vec<ProcessId> = ProcessRegistry::list().unwrap_or_default();
    for s in &truth.evidence.processes {
        let pid = ProcessId::from_string(s);
        if !ids.iter().any(|p| p == &pid) {
            ids.push(pid);
        }
    }
    for k in truth.process_writes.keys() {
        if !ids.iter().any(|p| p == k) {
            ids.push(k.clone());
        }
    }
    ids.sort_by(|a, b| a.as_str().cmp(b.as_str()));
    ids
}

fn processes_hash_for_truth(truth: &SystemTruth) -> String {
    let ids = process_ids_for_truth(truth);
    let arr = build_process_entries(&ids, HashMapLite::WithWrites(&truth.process_writes));
    let v = Value::Array(arr);
    sha256_hex_of_canonical(&v).unwrap_or_else(|_| String::new())
}

pub fn hash_evidence(index: &EvidenceIndex) -> String {
    let v = serde_json::to_value(index).unwrap_or(Value::Null);
    sha256_hex_of_canonical(&v).unwrap_or_else(|_| String::new())
}

pub fn hash_map(map: &MapModel) -> String {
    let v = serde_json::to_value(map).unwrap_or(Value::Null);
    sha256_hex_of_canonical(&v).unwrap_or_else(|_| String::new())
}

pub fn hash_policies(policies: &PolicySet) -> String {
    let v = serde_json::to_value(policies).unwrap_or(Value::Null);
    sha256_hex_of_canonical(&v).unwrap_or_else(|_| String::new())
}

pub fn hash_hardware(hw: &HardwareProfiles) -> String {
    let v = serde_json::to_value(hw).unwrap_or(Value::Null);
    sha256_hex_of_canonical(&v).unwrap_or_else(|_| String::new())
}

pub fn hash_state(state: &State) -> String {
    let v = serde_json::to_value(state).unwrap_or(Value::Null);
    sha256_hex_of_canonical(&v).unwrap_or_else(|_| String::new())
}

pub fn hash_truth(truth: &SystemTruth) -> String {
    let processes_hash = processes_hash_for_truth(truth);
    let evidence_hash = hash_evidence(&truth.evidence);
    let map_hash = hash_map(&truth.map);
    let policies_hash = hash_policies(&truth.policies);
    let hardware_hash = hash_hardware(&truth.hardware);
    let state_hash = hash_state(&truth.state);
    let meta = serde_json::json!({
        "evidence_hash": evidence_hash,
        "hardware_hash": hardware_hash,
        "map_hash": map_hash,
        "policies_hash": policies_hash,
        "processes_hash": processes_hash,
        "state_hash": state_hash,
    });
    sha256_hex_of_canonical(&meta).unwrap_or_else(|_| String::new())
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KernelSnapshot {
    pub truth_hash: String,
    pub processes_hash: String,
    pub evidence_hash: String,
    pub map_hash: String,
    pub policies_hash: String,
    pub hardware_hash: String,
    pub state_hash: String,

    pub execution_seed: u64,
    pub kernel_version: String,
    pub policy_version: String,
    pub hardware_profile: String,
}

impl KernelSnapshot {
    pub fn from_truth(truth: &SystemTruth, envelope: &ExecutionEnvelope) -> Self {
        let processes_hash = processes_hash_for_truth(truth);
        let evidence_hash = hash_evidence(&truth.evidence);
        let map_hash = hash_map(&truth.map);
        let policies_hash = hash_policies(&truth.policies);
        let hardware_hash = hash_hardware(&truth.hardware);
        let state_hash = hash_state(&truth.state);
        let truth_hash = hash_truth(truth);
        Self {
            truth_hash,
            processes_hash,
            evidence_hash,
            map_hash,
            policies_hash,
            hardware_hash,
            state_hash,
            execution_seed: envelope.seed,
            kernel_version: envelope.kernel_version.clone(),
            policy_version: envelope.policy_version.clone(),
            hardware_profile: envelope.hardware_profile.clone(),
        }
    }

    /// Compact JSON with keys sorted lexicographically (deterministic on-disk encoding).
    pub fn to_json_bytes(&self) -> Result<Vec<u8>> {
        let mut pairs: Vec<(&str, Value)> = vec![
            ("evidence_hash", Value::String(self.evidence_hash.clone())),
            ("execution_seed", Value::Number(self.execution_seed.into())),
            ("hardware_hash", Value::String(self.hardware_hash.clone())),
            ("hardware_profile", Value::String(self.hardware_profile.clone())),
            ("kernel_version", Value::String(self.kernel_version.clone())),
            ("map_hash", Value::String(self.map_hash.clone())),
            ("policies_hash", Value::String(self.policies_hash.clone())),
            ("policy_version", Value::String(self.policy_version.clone())),
            ("processes_hash", Value::String(self.processes_hash.clone())),
            ("state_hash", Value::String(self.state_hash.clone())),
            ("truth_hash", Value::String(self.truth_hash.clone())),
        ];
        pairs.sort_by(|a, b| a.0.cmp(b.0));
        let obj: serde_json::Map<String, Value> = pairs
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();
        let body = serde_json::to_string(&Value::Object(obj)).context("serialize KernelSnapshot")?;
        Ok(body.into_bytes())
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let body = self.to_json_bytes()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).with_context(|| format!("create_dir_all {}", parent.display()))?;
        }
        let tmp = path.with_extension("json.tmp");
        fs::write(&tmp, &body).with_context(|| format!("write {}", tmp.display()))?;
        fs::rename(&tmp, path).with_context(|| format!("rename {} -> {}", tmp.display(), path.display()))?;
        Ok(())
    }

    pub fn load(path: &Path) -> Result<Self> {
        let raw = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
        let v: Value = serde_json::from_str(&raw).with_context(|| format!("parse {}", path.display()))?;
        let sorted = sort_value_keys(&v);
        serde_json::from_value(sorted).with_context(|| format!("decode KernelSnapshot {}", path.display()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_processes_empty_registry_is_stable_hex_len() {
        let h = hash_processes(&ProcessRegistry);
        assert_eq!(h.len(), 64, "{h}");
    }
}
