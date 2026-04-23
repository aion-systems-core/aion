//! Compile-time and feature-stable runtime identity for cross-machine replay checks.

use serde::{Deserialize, Serialize};

const RUSTC_VERSION: &str = env!("AION_ENGINE_RUSTC_VERSION");

/// Toolchain and build identity (strict equality on cross-machine replay).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeFingerprint {
    pub rustc_version: String,
    pub aion_version: String,
    pub build_hash: String,
    pub enabled_features: Vec<String>,
}

impl RuntimeFingerprint {
    pub fn capture() -> Self {
        let aion_version = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../VERSION"))
            .trim()
            .to_string();
        let mut features = Vec::new();
        if cfg!(feature = "ffi") {
            features.push("ffi".into());
        }
        if cfg!(feature = "async") {
            features.push("async".into());
        }
        features.sort();
        let build_hash = build_hash_lit(&aion_version);
        Self {
            rustc_version: RUSTC_VERSION.into(),
            aion_version,
            build_hash,
            enabled_features: features,
        }
    }
}

fn build_hash_lit(aion_version: &str) -> String {
    let src = format!("{aion_version}|{RUSTC_VERSION}");
    let h = blake3::hash(src.as_bytes());
    let b: [u8; 8] = h.as_bytes()[..8].try_into().unwrap_or([0u8; 8]);
    format!("{:016x}", u64::from_le_bytes(b))
}
