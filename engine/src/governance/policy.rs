//! Governance PolicyProfile v1 — AI capsule constraints (distinct from `aion_core::PolicyProfile`).

use crate::ai::AICapsuleV1;
use aion_core::error::{code, io_cause, line};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PolicyProfile {
    #[serde(default = "default_policy_version")]
    pub policy_version: String,
    pub name: String,
    pub allowed_models: Vec<String>,
    pub max_prompt_length: usize,
    pub allowed_seeds: Option<(u64, u64)>,
    pub max_drift_tokens: usize,
    pub require_evidence: bool,
    pub require_replay_success: bool,
}

impl Default for PolicyProfile {
    fn default() -> Self {
        Self {
            policy_version: default_policy_version(),
            name: "default".into(),
            allowed_models: vec![],
            max_prompt_length: 0,
            allowed_seeds: None,
            max_drift_tokens: 0,
            require_evidence: false,
            require_replay_success: false,
        }
    }
}

fn default_policy_version() -> String {
    "1".into()
}

/// Built-in governance presets (for `aion policy show` without a JSON file).
pub fn builtin_policy_profile(name: &str) -> PolicyProfile {
    match name {
        "strict" => PolicyProfile {
            policy_version: default_policy_version(),
            name: "strict".into(),
            allowed_models: vec!["*".into()],
            max_prompt_length: 65_536,
            allowed_seeds: None,
            max_drift_tokens: 1_000_000,
            require_evidence: true,
            require_replay_success: true,
        },
        "stage" => PolicyProfile {
            policy_version: default_policy_version(),
            name: "stage".into(),
            allowed_models: vec!["*".into()],
            max_prompt_length: 32_768,
            allowed_seeds: None,
            max_drift_tokens: 500_000,
            require_evidence: true,
            require_replay_success: false,
        },
        "prod" => PolicyProfile {
            policy_version: default_policy_version(),
            name: "prod".into(),
            allowed_models: vec!["*".into()],
            max_prompt_length: 16_384,
            allowed_seeds: None,
            max_drift_tokens: 200_000,
            require_evidence: true,
            require_replay_success: true,
        },
        _ => PolicyProfile {
            policy_version: default_policy_version(),
            name: "dev".into(),
            allowed_models: vec![],
            max_prompt_length: 0,
            allowed_seeds: None,
            max_drift_tokens: 0,
            require_evidence: false,
            require_replay_success: false,
        },
    }
}

/// Compose two policies (child overlays parent when set to non-default values).
pub fn compose_policies(parent: &PolicyProfile, child: &PolicyProfile) -> PolicyProfile {
    let mut out = parent.clone();
    out.policy_version = if child.policy_version.trim().is_empty() {
        parent.policy_version.clone()
    } else {
        child.policy_version.clone()
    };
    if !child.name.trim().is_empty() {
        out.name = child.name.clone();
    }
    if !child.allowed_models.is_empty() {
        out.allowed_models = child.allowed_models.clone();
    }
    if child.max_prompt_length > 0 {
        out.max_prompt_length = child.max_prompt_length;
    }
    if child.allowed_seeds.is_some() {
        out.allowed_seeds = child.allowed_seeds;
    }
    if child.max_drift_tokens > 0 {
        out.max_drift_tokens = child.max_drift_tokens;
    }
    out.require_evidence = child.require_evidence || out.require_evidence;
    out.require_replay_success = child.require_replay_success || out.require_replay_success;
    out
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PolicyViolation {
    pub ok: bool,
    pub messages: Vec<String>,
}

impl PolicyViolation {
    pub fn pass() -> Self {
        Self {
            ok: true,
            messages: vec![],
        }
    }
}

fn model_allowed(allowed: &[String], model: &str) -> bool {
    if allowed.is_empty() {
        return true;
    }
    allowed.iter().any(|m| m == "*" || m == model)
}

/// Load JSON policy from disk.
pub fn load_policy(path: &Path) -> Result<PolicyProfile, String> {
    let s = fs::read_to_string(path)
        .map_err(|e| line(code::GOVERNANCE_IO, "load_policy", &io_cause(&e)))?;
    let v: Value = serde_json::from_str(&s)
        .map_err(|_| line(code::GOVERNANCE_JSON, "load_policy", "json_parse_invalid"))?;
    parse_policy_profile(&v)
}

fn parse_policy_profile(v: &Value) -> Result<PolicyProfile, String> {
    // 1) JSON-shape
    let obj = v
        .as_object()
        .ok_or_else(|| line(code::GOVERNANCE_JSON, "policy_shape", "object_required"))?;

    // 2) Required fields
    require_field(obj, "policy_version")?;
    require_field(obj, "name")?;
    require_field(obj, "allowed_models")?;
    require_field(obj, "max_prompt_length")?;
    require_field(obj, "allowed_seeds")?;
    require_field(obj, "max_drift_tokens")?;
    require_field(obj, "require_evidence")?;
    require_field(obj, "require_replay_success")?;

    // 3) Type checks
    let policy_version = as_string(obj, "policy_version")?;
    let name = as_string(obj, "name")?;
    let allowed_models = as_string_array(obj, "allowed_models")?;
    let max_prompt_length = as_u64(obj, "max_prompt_length")? as usize;
    let allowed_seeds = as_u64_pair_or_null(obj, "allowed_seeds")?;
    let max_drift_tokens = as_u64(obj, "max_drift_tokens")? as usize;
    let require_evidence = as_bool(obj, "require_evidence")?;
    let require_replay_success = as_bool(obj, "require_replay_success")?;

    // 4) Value constraints
    if policy_version != "1" {
        return Err(line(
            code::GOVERNANCE_JSON,
            "policy_value",
            "policy_version_unsupported",
        ));
    }
    if name.trim().is_empty() {
        return Err(line(code::GOVERNANCE_JSON, "policy_value", "name_empty"));
    }
    if allowed_models.iter().any(|m| m.trim().is_empty()) {
        return Err(line(
            code::GOVERNANCE_JSON,
            "policy_value",
            "allowed_models_entry_empty",
        ));
    }

    // 5) Cross-field constraints
    if let Some((lo, hi)) = allowed_seeds {
        if lo > hi {
            return Err(line(
                code::GOVERNANCE_JSON,
                "policy_cross",
                "allowed_seeds_range_invalid",
            ));
        }
    }
    if require_replay_success && !require_evidence {
        return Err(line(
            code::GOVERNANCE_JSON,
            "policy_cross",
            "replay_requires_evidence",
        ));
    }

    Ok(PolicyProfile {
        policy_version,
        name,
        allowed_models,
        max_prompt_length,
        allowed_seeds,
        max_drift_tokens,
        require_evidence,
        require_replay_success,
    })
}

fn require_field(obj: &serde_json::Map<String, Value>, key: &'static str) -> Result<(), String> {
    if obj.contains_key(key) {
        Ok(())
    } else {
        Err(line(
            code::GOVERNANCE_JSON,
            "policy_required",
            &format!("field_missing:{key}"),
        ))
    }
}

fn as_string(obj: &serde_json::Map<String, Value>, key: &'static str) -> Result<String, String> {
    obj.get(key)
        .and_then(Value::as_str)
        .map(ToString::to_string)
        .ok_or_else(|| {
            line(
                code::GOVERNANCE_JSON,
                "policy_type",
                &format!("field_type:{key}:string"),
            )
        })
}

fn as_u64(obj: &serde_json::Map<String, Value>, key: &'static str) -> Result<u64, String> {
    obj.get(key).and_then(Value::as_u64).ok_or_else(|| {
        line(
            code::GOVERNANCE_JSON,
            "policy_type",
            &format!("field_type:{key}:u64"),
        )
    })
}

fn as_bool(obj: &serde_json::Map<String, Value>, key: &'static str) -> Result<bool, String> {
    obj.get(key).and_then(Value::as_bool).ok_or_else(|| {
        line(
            code::GOVERNANCE_JSON,
            "policy_type",
            &format!("field_type:{key}:bool"),
        )
    })
}

fn as_string_array(
    obj: &serde_json::Map<String, Value>,
    key: &'static str,
) -> Result<Vec<String>, String> {
    let arr = obj.get(key).and_then(Value::as_array).ok_or_else(|| {
        line(
            code::GOVERNANCE_JSON,
            "policy_type",
            &format!("field_type:{key}:array_string"),
        )
    })?;
    let mut out = Vec::with_capacity(arr.len());
    for v in arr {
        let s = v.as_str().ok_or_else(|| {
            line(
                code::GOVERNANCE_JSON,
                "policy_type",
                &format!("field_type:{key}:array_string_entry"),
            )
        })?;
        out.push(s.to_string());
    }
    Ok(out)
}

fn as_u64_pair_or_null(
    obj: &serde_json::Map<String, Value>,
    key: &'static str,
) -> Result<Option<(u64, u64)>, String> {
    let v = obj.get(key).ok_or_else(|| {
        line(
            code::GOVERNANCE_JSON,
            "policy_required",
            &format!("field_missing:{key}"),
        )
    })?;
    if v.is_null() {
        return Ok(None);
    }
    let arr = v.as_array().ok_or_else(|| {
        line(
            code::GOVERNANCE_JSON,
            "policy_type",
            &format!("field_type:{key}:null_or_pair_u64"),
        )
    })?;
    if arr.len() != 2 {
        return Err(line(
            code::GOVERNANCE_JSON,
            "policy_value",
            &format!("field_value:{key}:pair_len"),
        ));
    }
    let lo = arr[0].as_u64().ok_or_else(|| {
        line(
            code::GOVERNANCE_JSON,
            "policy_type",
            &format!("field_type:{key}:pair_u64_entry"),
        )
    })?;
    let hi = arr[1].as_u64().ok_or_else(|| {
        line(
            code::GOVERNANCE_JSON,
            "policy_type",
            &format!("field_type:{key}:pair_u64_entry"),
        )
    })?;
    Ok(Some((lo, hi)))
}

/// `replay_success`: pass `Some(true)` when replay has been executed and succeeded; `None` skips
/// `require_replay_success` (treated as unknown → violation if required).
pub fn validate_capsule_against_policy(
    capsule: &AICapsuleV1,
    policy: &PolicyProfile,
    replay_success: Option<bool>,
) -> PolicyViolation {
    let mut messages = Vec::new();

    if !model_allowed(&policy.allowed_models, &capsule.model) {
        messages.push("policy:model_not_allowed".into());
    }

    if policy.max_prompt_length > 0 && capsule.prompt.len() > policy.max_prompt_length {
        messages.push("policy:prompt_over_limit".into());
    }

    if let Some((lo, hi)) = policy.allowed_seeds {
        if capsule.seed < lo || capsule.seed > hi {
            messages.push("policy:seed_outside_allowed_range".into());
        }
    }

    if policy.max_drift_tokens > 0 && capsule.tokens.len() > policy.max_drift_tokens {
        messages.push("policy:token_count_over_limit".into());
    }

    if policy.require_evidence && capsule.evidence.records.is_empty() {
        messages.push("policy:evidence_required_missing".into());
    }

    if policy.require_replay_success {
        if replay_success == Some(false) {
            messages.push("policy:replay_required_failed".into());
        }
        if replay_success.is_none() {
            messages.push("policy:replay_required_unknown".into());
        }
    }

    messages.sort();
    PolicyViolation {
        ok: messages.is_empty(),
        messages,
    }
}
