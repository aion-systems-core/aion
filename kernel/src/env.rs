//! Environment fingerprinting for drift visibility (sorted keys).

use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::env;

/// Stable subset of environment for child process (deterministic key order).
pub fn filtered_env_for_child() -> BTreeMap<String, String> {
    let mut m = BTreeMap::new();
    for (k, v) in env::vars() {
        if k == "PATH" || k == "PATHEXT" || k == "SYSTEMROOT" || k == "USERPROFILE" {
            m.insert(k, v);
        }
    }
    m
}

pub fn env_fingerprint(vars: &BTreeMap<String, String>) -> String {
    let v: Value = serde_json::to_value(vars).unwrap_or(json!({}));
    let bytes = serde_json::to_vec(&v).unwrap_or_default();
    format!("{:x}", Sha256::digest(&bytes))
}
