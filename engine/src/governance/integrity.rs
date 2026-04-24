//! Governance IntegrityProfile v1 — required AI capsule artefacts.

use crate::ai::AICapsuleV1;
use aion_core::error::{code, io_cause, line};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct IntegrityProfile {
    pub require_manifest: bool,
    pub require_hashes: bool,
    pub require_evidence_chain: bool,
    pub require_graph: bool,
    pub require_why: bool,
    pub require_replay: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IntegrityViolation {
    pub ok: bool,
    pub messages: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IntegritySignature {
    pub capsule_hash: String,
    pub evidence_root: String,
    pub signature: String,
    pub previous_signature: Option<String>,
}

impl IntegrityViolation {
    pub fn pass() -> Self {
        Self {
            ok: true,
            messages: vec![],
        }
    }
}

pub fn load_integrity(path: &Path) -> Result<IntegrityProfile, String> {
    let s = fs::read_to_string(path)
        .map_err(|e| line(code::GOVERNANCE_IO, "load_integrity", &io_cause(&e)))?;
    serde_json::from_str(&s)
        .map_err(|_| line(code::GOVERNANCE_JSON, "load_integrity", "invalid_json"))
}

/// `replay_success`: `Some(true)` after replay; `None` when replay was not run (fails if `require_replay`).
pub fn validate_integrity(
    capsule: &AICapsuleV1,
    profile: &IntegrityProfile,
    replay_success: Option<bool>,
) -> IntegrityViolation {
    let mut messages = Vec::new();

    if profile.require_manifest && capsule.version.trim().is_empty() {
        messages.push("require_manifest: capsule.version is empty".into());
    }

    if profile.require_hashes {
        let ok = capsule
            .evidence
            .records
            .iter()
            .all(|r| !r.leaf_digest.is_empty() && !r.payload_digest.is_empty());
        if !ok || capsule.evidence.records.is_empty() {
            messages.push("require_hashes: evidence records missing digests or chain empty".into());
        }
    }

    if profile.require_evidence_chain && capsule.evidence.records.is_empty() {
        messages.push("require_evidence_chain: no evidence records".into());
    }

    if profile.require_graph && (capsule.graph.nodes.is_empty()) {
        messages.push("require_graph: causal graph has no nodes".into());
    }

    if profile.require_why && capsule.why.nodes.is_empty() {
        messages.push("require_why: why report has no nodes".into());
    }

    if profile.require_replay {
        match replay_success {
            Some(true) => {}
            Some(false) => messages.push("require_replay: replay did not succeed".into()),
            None => messages.push("require_replay: replay not attested (run ci check)".into()),
        }
    }

    messages.sort();
    IntegrityViolation {
        ok: messages.is_empty(),
        messages,
    }
}

/// Deterministic integrity signing (hash-chain style signature envelope).
pub fn sign_integrity(capsule: &AICapsuleV1) -> IntegritySignature {
    let body = serde_json::to_vec(capsule).unwrap_or_default();
    let capsule_hash = format!("{:x}", Sha256::digest(&body));
    let evidence_root = capsule.evidence.root_digest();
    let sig_src = format!("{}:{}", capsule_hash, evidence_root);
    let signature = format!("{:x}", Sha256::digest(sig_src.as_bytes()));
    IntegritySignature {
        capsule_hash,
        evidence_root,
        signature,
        previous_signature: None,
    }
}

pub fn aion_evidence_generate_keypair() -> (Vec<u8>, Vec<u8>) {
    let sk = SigningKey::from_bytes(&rand::random::<[u8; 32]>());
    let pk = sk.verifying_key();
    (sk.to_bytes().to_vec(), pk.to_bytes().to_vec())
}

pub fn aion_evidence_sign(evidence_data: &[u8], private_key: &[u8]) -> Result<Vec<u8>, String> {
    let key: [u8; 32] = private_key.try_into().map_err(|_| {
        line(
            code::EVIDENCE_HASH,
            "aion_evidence_sign",
            "evidence:key_invalid",
        )
    })?;
    let sk = SigningKey::from_bytes(&key);
    Ok(sk.sign(evidence_data).to_bytes().to_vec())
}

pub fn aion_evidence_verify_ed25519(
    evidence_data: &[u8],
    signature: &[u8],
    public_key: &[u8],
) -> Result<bool, String> {
    let pk_bytes: [u8; 32] = public_key.try_into().map_err(|_| {
        line(
            code::EVIDENCE_HASH,
            "aion_evidence_verify_ed25519",
            "evidence:pubkey_invalid",
        )
    })?;
    let sig_bytes: [u8; 64] = signature.try_into().map_err(|_| {
        line(
            code::EVIDENCE_HASH,
            "aion_evidence_verify_ed25519",
            "evidence:signature_invalid",
        )
    })?;
    let pk = VerifyingKey::from_bytes(&pk_bytes).map_err(|_| {
        line(
            code::EVIDENCE_HASH,
            "aion_evidence_verify_ed25519",
            "evidence:pubkey_decode_invalid",
        )
    })?;
    let sig = Signature::from_bytes(&sig_bytes);
    Ok(pk.verify(evidence_data, &sig).is_ok())
}
