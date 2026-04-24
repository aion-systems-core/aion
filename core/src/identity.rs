use crate::os_contract_spec_version;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OsKernelVersion {
    pub semver: String,
    pub build_metadata: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OsInstanceId {
    pub value: String,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OsCompatibilityProfile {
    pub os_contract_spec_versions: Vec<String>,
    pub global_consistency_contract_versions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OsIdentity {
    pub kernel_version: OsKernelVersion,
    pub os_contract_spec_version: String,
    pub global_consistency_contract_version: String,
    pub compatibility_profile: OsCompatibilityProfile,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<OsInstanceId>,
}

pub fn os_kernel_version_from_inputs(semver: &str, commit: Option<&str>) -> OsKernelVersion {
    let build_metadata = commit
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("unknown")
        .to_string();
    OsKernelVersion {
        semver: semver.to_string(),
        value: format!("{semver}+{build_metadata}"),
        build_metadata,
    }
}

pub fn global_consistency_contract_version() -> String {
    let mut h = Sha256::new();
    h.update(b"global_consistency_contract:v1:run_finality|capsule_finality|evidence_finality|replay_finality");
    format!("{:x}", h.finalize())
}

fn derive_instance_id() -> Option<OsInstanceId> {
    if let Ok(explicit) = std::env::var("AION_INSTANCE_ID") {
        let v = explicit.trim();
        if !v.is_empty() {
            return Some(OsInstanceId {
                value: v.to_string(),
                source: "AION_INSTANCE_ID".to_string(),
            });
        }
    }
    let host = std::env::var("AION_HOST_ID")
        .ok()
        .or_else(|| std::env::var("HOSTNAME").ok())
        .or_else(|| std::env::var("COMPUTERNAME").ok())
        .unwrap_or_else(|| "unknown".to_string());
    if host == "unknown" {
        return None;
    }
    let scope = std::env::var("AION_INSTANCE_SCOPE").unwrap_or_else(|_| "unset".to_string());
    let env = std::env::var("AION_DEPLOYMENT_ENV").unwrap_or_else(|_| "unset".to_string());
    let mut h = Sha256::new();
    h.update(format!(
        "host={host}|scope={scope}|env={env}|os={}",
        std::env::consts::OS
    ));
    Some(OsInstanceId {
        value: format!("{:x}", h.finalize()),
        source: "derived_explicit_inputs".to_string(),
    })
}

pub fn os_kernel_version() -> OsKernelVersion {
    let semver = env!("CARGO_PKG_VERSION");
    let commit = std::env::var("AION_GIT_COMMIT").ok();
    os_kernel_version_from_inputs(semver, commit.as_deref())
}

pub fn os_compatibility_profile() -> OsCompatibilityProfile {
    OsCompatibilityProfile {
        os_contract_spec_versions: vec![os_contract_spec_version()],
        global_consistency_contract_versions: vec![global_consistency_contract_version()],
    }
}

pub fn os_identity() -> OsIdentity {
    OsIdentity {
        kernel_version: os_kernel_version(),
        os_contract_spec_version: os_contract_spec_version(),
        global_consistency_contract_version: global_consistency_contract_version(),
        compatibility_profile: os_compatibility_profile(),
        instance_id: derive_instance_id(),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        global_consistency_contract_version, os_compatibility_profile, os_contract_spec_version,
        os_identity, os_kernel_version_from_inputs,
    };

    #[test]
    fn os_identity_serialization_is_deterministic() {
        let id = os_identity();
        let a = serde_json::to_string(&id).expect("serialize identity a");
        let b = serde_json::to_string(&id).expect("serialize identity b");
        assert_eq!(a, b);
    }

    #[test]
    fn kernel_version_is_stable_for_identical_inputs() {
        let a = os_kernel_version_from_inputs("0.2.0", Some("abc123"));
        let b = os_kernel_version_from_inputs("0.2.0", Some("abc123"));
        assert_eq!(a, b);
        assert_eq!(a.value, "0.2.0+abc123");
    }

    #[test]
    fn compatibility_profile_references_current_spec_version() {
        let p = os_compatibility_profile();
        assert_eq!(
            p.os_contract_spec_versions,
            vec![os_contract_spec_version()]
        );
        assert_eq!(
            p.global_consistency_contract_versions,
            vec![global_consistency_contract_version()]
        );
    }
}
