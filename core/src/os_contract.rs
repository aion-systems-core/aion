use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::Path;

const OS_CONTRACT_SPEC_MARKDOWN: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../docs/os_contract_spec.md"
));

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OsInvariant {
    pub id: String,
    pub rule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OsErrorCode {
    pub code: String,
    pub scope: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OsFinalityRule {
    pub id: String,
    pub condition: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OsContractSection {
    pub section: String,
    pub input_form: Vec<String>,
    pub output_form: Vec<String>,
    pub invariants: Vec<OsInvariant>,
    pub determinism_guarantee: String,
    pub error_codes: Vec<OsErrorCode>,
    pub finality_rules: Vec<OsFinalityRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OsContractSpec {
    pub spec_id: String,
    pub spec_version: String,
    pub sections: Vec<OsContractSection>,
}

pub fn os_contract_spec_version() -> String {
    hash_os_contract_spec_markdown(OS_CONTRACT_SPEC_MARKDOWN)
}

pub fn hash_os_contract_spec_markdown(markdown: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(markdown.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn hash_os_contract_spec_file(path: &Path) -> Result<String, String> {
    let s = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    Ok(hash_os_contract_spec_markdown(&s))
}

pub fn os_contract_spec() -> OsContractSpec {
    OsContractSpec {
        spec_id: "aion-os-contract".to_string(),
        spec_version: os_contract_spec_version(),
        sections: vec![
            OsContractSection {
                section: "state_contract".to_string(),
                input_form: vec!["capsule".to_string(), "profile".to_string()],
                output_form: vec!["replay_state".to_string()],
                invariants: vec![
                    OsInvariant { id: "state.canonical_capsule".to_string(), rule: "capsule is canonicalized before replay".to_string() },
                    OsInvariant { id: "state.profile_stability".to_string(), rule: "profile fields are replay-stable".to_string() },
                ],
                determinism_guarantee: "canonical capsule serialization".to_string(),
                error_codes: vec![
                    OsErrorCode { code: "AION_REPLAY_MISMATCH".to_string(), scope: "state".to_string() },
                    OsErrorCode { code: "AION_REPLAY_PROFILE".to_string(), scope: "state".to_string() },
                ],
                finality_rules: vec![
                    OsFinalityRule { id: "state.finality.1".to_string(), condition: "capsule_complete && capsule_referencable".to_string() },
                ],
            },
            OsContractSection {
                section: "process_contract".to_string(),
                input_form: vec!["original_capsule".to_string(), "replay_capsule".to_string()],
                output_form: vec!["replay_report".to_string(), "replay_error_contract".to_string()],
                invariants: vec![
                    OsInvariant { id: "process.check_order".to_string(), rule: "shape->canonicalization->why_slice->event_stream->profile->evidence_anchors".to_string() },
                    OsInvariant { id: "process.tokenized_errors".to_string(), rule: "replay errors are tokenized".to_string() },
                ],
                determinism_guarantee: "fixed replay check order".to_string(),
                error_codes: vec![
                    OsErrorCode { code: "AION_REPLAY_MISMATCH".to_string(), scope: "process".to_string() },
                    OsErrorCode { code: "AION_REPLAY_SYMMETRY".to_string(), scope: "process".to_string() },
                    OsErrorCode { code: "AION_REPLAY_PROFILE".to_string(), scope: "process".to_string() },
                ],
                finality_rules: vec![
                    OsFinalityRule { id: "process.finality.1".to_string(), condition: "replay_invariant_ok && replay_symmetry_ok && replay_cross_machine_ok".to_string() },
                ],
            },
            OsContractSection {
                section: "map_contract".to_string(),
                input_form: vec!["left_capsule".to_string(), "right_capsule".to_string(), "tolerance_profile".to_string()],
                output_form: vec!["drift_report".to_string()],
                invariants: vec![
                    OsInvariant { id: "map.sorted_labels".to_string(), rule: "drift labels/categories are deterministically sorted".to_string() },
                    OsInvariant { id: "map.fixed_tolerance".to_string(), rule: "drift tolerance profile is explicit".to_string() },
                ],
                determinism_guarantee: "stable drift taxonomy".to_string(),
                error_codes: vec![
                    OsErrorCode { code: "AION_DRIFT_JSON".to_string(), scope: "map".to_string() },
                    OsErrorCode { code: "AION_DRIFT_TOLERANCE".to_string(), scope: "map".to_string() },
                    OsErrorCode { code: "AION_DRIFT_OVERFLOW".to_string(), scope: "map".to_string() },
                ],
                finality_rules: vec![
                    OsFinalityRule { id: "map.finality.1".to_string(), condition: "!drift_changed && tolerance_violations_empty".to_string() },
                ],
            },
            OsContractSection {
                section: "evidence_contract".to_string(),
                input_form: vec!["run_result".to_string(), "policy".to_string(), "determinism".to_string()],
                output_form: vec!["evidence_chain".to_string(), "evidence_contract".to_string()],
                invariants: vec![
                    OsInvariant { id: "evidence.linear_chain".to_string(), rule: "evidence chain is linear and hash-bound".to_string() },
                    OsInvariant { id: "evidence.anchor_closure".to_string(), rule: "replay anchors are closed".to_string() },
                ],
                determinism_guarantee: "stable rolling hash order".to_string(),
                error_codes: vec![
                    OsErrorCode { code: "AION_EVIDENCE_HASH".to_string(), scope: "evidence".to_string() },
                    OsErrorCode { code: "AION_EVIDENCE_ANCHOR".to_string(), scope: "evidence".to_string() },
                    OsErrorCode { code: "AION_EVIDENCE_IO".to_string(), scope: "evidence".to_string() },
                ],
                finality_rules: vec![
                    OsFinalityRule { id: "evidence.finality.1".to_string(), condition: "evidence_verified && !evidence_open_anchors".to_string() },
                ],
            },
            OsContractSection {
                section: "policy_contract".to_string(),
                input_form: vec!["policy_json".to_string(), "capsule".to_string()],
                output_form: vec!["policy_report".to_string(), "policy_error_contract".to_string()],
                invariants: vec![
                    OsInvariant { id: "policy.validation_order".to_string(), rule: "shape->required->type->value->cross_field".to_string() },
                    OsInvariant { id: "policy.tokenized_violations".to_string(), rule: "policy violations are deterministic tokens".to_string() },
                ],
                determinism_guarantee: "stable policy violation output".to_string(),
                error_codes: vec![
                    OsErrorCode { code: "AION_GOVERNANCE_JSON".to_string(), scope: "policy".to_string() },
                    OsErrorCode { code: "AION_GOVERNANCE_IO".to_string(), scope: "policy".to_string() },
                ],
                finality_rules: vec![
                    OsFinalityRule { id: "policy.finality.1".to_string(), condition: "policy_ok".to_string() },
                ],
            },
            OsContractSection {
                section: "global_consistency_contract".to_string(),
                input_form: vec!["replay_signals".to_string(), "drift_signals".to_string(), "policy_signals".to_string(), "evidence_signals".to_string()],
                output_form: vec!["global_consistency_contract".to_string()],
                invariants: vec![
                    OsInvariant { id: "global.fixed_order".to_string(), rule: "run/capsule/evidence/replay finality order is fixed".to_string() },
                    OsInvariant { id: "global.shared_shape".to_string(), rule: "all finality entries use status/code/context/origin/cause?".to_string() },
                ],
                determinism_guarantee: "same signals -> same finality contract".to_string(),
                error_codes: vec![
                    OsErrorCode { code: "AION_REPLAY_*".to_string(), scope: "global".to_string() },
                    OsErrorCode { code: "AION_EVIDENCE_*".to_string(), scope: "global".to_string() },
                    OsErrorCode { code: "AION_GOVERNANCE_*".to_string(), scope: "global".to_string() },
                ],
                finality_rules: vec![
                    OsFinalityRule { id: "global.finality.1".to_string(), condition: "run_finality_ok when replay/drift/policy/evidence are ok".to_string() },
                ],
            },
            OsContractSection {
                section: "output_contract".to_string(),
                input_form: vec!["command_payload".to_string(), "error_contract".to_string()],
                output_form: vec!["json_envelope".to_string()],
                invariants: vec![
                    OsInvariant { id: "output.envelope_shape".to_string(), rule: "envelope uses status/data/error?".to_string() },
                    OsInvariant { id: "output.stable_order".to_string(), rule: "json keys and known arrays are deterministic".to_string() },
                ],
                determinism_guarantee: "canonical envelope serialization".to_string(),
                error_codes: vec![
                    OsErrorCode { code: "AION_CLI_JSON_PARSE".to_string(), scope: "output".to_string() },
                    OsErrorCode { code: "AION_CLI_JSON_SERIALIZE".to_string(), scope: "output".to_string() },
                    OsErrorCode { code: "AION_OUTPUT_JSON_SERIALIZE".to_string(), scope: "output".to_string() },
                ],
                finality_rules: vec![
                    OsFinalityRule { id: "output.finality.1".to_string(), condition: "status == ok && data_contract_valid".to_string() },
                ],
            },
            OsContractSection {
                section: "error_contract".to_string(),
                input_form: vec!["aion_error_line".to_string(), "nested_error_contract".to_string()],
                output_form: vec!["aion_error_json".to_string()],
                invariants: vec![
                    OsInvariant { id: "error.fixed_fields".to_string(), rule: "code/message/context/origin/cause?".to_string() },
                    OsInvariant { id: "error.aion_namespace".to_string(), rule: "codes are in AION_* namespace".to_string() },
                ],
                determinism_guarantee: "same tuple -> same canonical json".to_string(),
                error_codes: vec![
                    OsErrorCode { code: "AION_REPLAY_*".to_string(), scope: "error".to_string() },
                    OsErrorCode { code: "AION_DRIFT_*".to_string(), scope: "error".to_string() },
                    OsErrorCode { code: "AION_EVIDENCE_*".to_string(), scope: "error".to_string() },
                    OsErrorCode { code: "AION_GOVERNANCE_*".to_string(), scope: "error".to_string() },
                    OsErrorCode { code: "AION_CLI_*".to_string(), scope: "error".to_string() },
                    OsErrorCode { code: "AION_FFI_*".to_string(), scope: "error".to_string() },
                    OsErrorCode { code: "AION_BINDINGS_*".to_string(), scope: "error".to_string() },
                ],
                finality_rules: vec![
                    OsFinalityRule { id: "error.finality.1".to_string(), condition: "canonical_json_valid && code_prefix_aion".to_string() },
                ],
            },
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::{
        hash_os_contract_spec_file, hash_os_contract_spec_markdown, os_contract_spec,
        os_contract_spec_version,
    };

    fn temp_file(name: &str) -> std::path::PathBuf {
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        std::env::temp_dir().join(format!("aion-os-contract-{name}-{stamp}.md"))
    }

    #[test]
    fn spec_serialization_is_deterministic() {
        let spec = os_contract_spec();
        let a = serde_json::to_string(&spec).expect("serialize spec a");
        let b = serde_json::to_string(&spec).expect("serialize spec b");
        assert_eq!(a, b);
    }

    #[test]
    fn spec_hash_id_is_deterministic() {
        let a = os_contract_spec_version();
        let b = os_contract_spec_version();
        assert_eq!(a, b);
    }

    #[test]
    fn spec_hash_changes_when_spec_content_changes() {
        let p = temp_file("hash");
        let original = "spec: aion\nrule: deterministic\n";
        std::fs::write(&p, original).expect("write original");
        let h1 = hash_os_contract_spec_file(&p).expect("hash original");
        std::fs::write(&p, format!("{original}extra: changed\n")).expect("write changed");
        let h2 = hash_os_contract_spec_file(&p).expect("hash changed");
        assert_ne!(h1, h2);
    }

    #[test]
    fn markdown_hash_function_is_stable() {
        let s = "aion_os_contract_spec_v1";
        let h1 = hash_os_contract_spec_markdown(s);
        let h2 = hash_os_contract_spec_markdown(s);
        assert_eq!(h1, h2);
    }
}
