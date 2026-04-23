use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BuildFingerprint {
    pub kernel_version: String,
    pub contract_spec_version: String,
    pub global_consistency_contract_version: String,
    pub abi_version: String,
    pub build_sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeterministicBuildContract {
    pub deterministic_build_flags: Vec<String>,
    pub deterministic_env_keys: Vec<String>,
    pub output_hashes: Vec<String>,
    pub fingerprint: BuildFingerprint,
}

pub fn evaluate_deterministic_build_contract(
    kernel_version: &str,
    contract_spec_version: &str,
    global_consistency_contract_version: &str,
    abi_version: &str,
    build_outputs: &[String],
) -> DeterministicBuildContract {
    let mut output_hashes: Vec<String> = build_outputs
        .iter()
        .map(|o| {
            let mut h = Sha256::new();
            h.update(o.as_bytes());
            format!("{:x}", h.finalize())
        })
        .collect();
    output_hashes.sort();
    let mut h = Sha256::new();
    h.update(kernel_version.as_bytes());
    h.update(contract_spec_version.as_bytes());
    h.update(global_consistency_contract_version.as_bytes());
    h.update(abi_version.as_bytes());
    for oh in &output_hashes {
        h.update(oh.as_bytes());
    }
    let build_sha256 = format!("{:x}", h.finalize());
    DeterministicBuildContract {
        deterministic_build_flags: vec![
            "RUSTFLAGS=-Cdebuginfo=0".to_string(),
            "SOURCE_DATE_EPOCH=0".to_string(),
        ],
        deterministic_env_keys: vec![
            "SOURCE_DATE_EPOCH".to_string(),
            "RUSTFLAGS".to_string(),
            "CARGO_PROFILE_RELEASE_LTO".to_string(),
        ],
        output_hashes,
        fingerprint: BuildFingerprint {
            kernel_version: kernel_version.to_string(),
            contract_spec_version: contract_spec_version.to_string(),
            global_consistency_contract_version: global_consistency_contract_version.to_string(),
            abi_version: abi_version.to_string(),
            build_sha256,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::evaluate_deterministic_build_contract;

    #[test]
    fn deterministic_fingerprint() {
        let a = evaluate_deterministic_build_contract(
            "0.2.0+abc",
            "spec1",
            "gc1",
            "v1",
            &["a".into(), "b".into()],
        );
        let b = evaluate_deterministic_build_contract(
            "0.2.0+abc",
            "spec1",
            "gc1",
            "v1",
            &["b".into(), "a".into()],
        );
        assert_eq!(a.fingerprint.build_sha256, b.fingerprint.build_sha256);
    }
}

