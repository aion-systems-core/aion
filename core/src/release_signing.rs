use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReleaseSignature {
    pub artifact_hash: String,
    pub artifact_type: String,
    pub kernel_version: String,
    pub timestamp: u64,
    pub provenance_id: String,
    pub signature_hex: String,
    pub public_key_hex: String,
}

fn payload(
    artifact_hash: &str,
    artifact_type: &str,
    kernel_version: &str,
    timestamp: u64,
    provenance_id: &str,
) -> Vec<u8> {
    format!(
        "{artifact_hash}|{artifact_type}|{kernel_version}|{timestamp}|{provenance_id}"
    )
    .into_bytes()
}

pub fn sign_release_artifact(
    artifact_hash: &str,
    artifact_type: &str,
    kernel_version: &str,
    timestamp: u64,
    provenance_id: &str,
    signing_key_bytes: [u8; 32],
) -> ReleaseSignature {
    let sk = SigningKey::from_bytes(&signing_key_bytes);
    let vk: VerifyingKey = sk.verifying_key();
    let msg = payload(
        artifact_hash,
        artifact_type,
        kernel_version,
        timestamp,
        provenance_id,
    );
    let sig = sk.sign(&msg);
    ReleaseSignature {
        artifact_hash: artifact_hash.to_string(),
        artifact_type: artifact_type.to_string(),
        kernel_version: kernel_version.to_string(),
        timestamp,
        provenance_id: provenance_id.to_string(),
        signature_hex: hex::encode(sig.to_bytes()),
        public_key_hex: hex::encode(vk.to_bytes()),
    }
}

pub fn verify_release_signature(sig: &ReleaseSignature) -> Result<(), String> {
    if sig.signature_hex.is_empty() {
        return Err("release_signing:signature_missing".to_string());
    }
    let sig_bytes = hex::decode(&sig.signature_hex)
        .map_err(|_| "release_signing:signature_invalid".to_string())?;
    let pk_bytes = hex::decode(&sig.public_key_hex)
        .map_err(|_| "release_signing:signature_invalid".to_string())?;
    let pk_arr: [u8; 32] = pk_bytes
        .as_slice()
        .try_into()
        .map_err(|_| "release_signing:signature_invalid".to_string())?;
    let vk = VerifyingKey::from_bytes(&pk_arr)
        .map_err(|_| "release_signing:signature_invalid".to_string())?;
    let msg = payload(
        &sig.artifact_hash,
        &sig.artifact_type,
        &sig.kernel_version,
        sig.timestamp,
        &sig.provenance_id,
    );
    let signature = Signature::try_from(sig_bytes.as_slice())
        .map_err(|_| "release_signing:signature_invalid".to_string())?;
    vk.verify(&msg, &signature)
        .map_err(|_| "release_signing:signature_invalid".to_string())
}

#[cfg(test)]
mod tests {
    use super::{sign_release_artifact, verify_release_signature};

    #[test]
    fn deterministic_payload_signature() {
        let key = [7u8; 32];
        let a = sign_release_artifact("h", "kernel", "0.2.0+abc", 0, "p1", key);
        let b = sign_release_artifact("h", "kernel", "0.2.0+abc", 0, "p1", key);
        assert_eq!(a.signature_hex, b.signature_hex);
        assert!(verify_release_signature(&a).is_ok());
    }

    #[test]
    fn missing_signature_negative() {
        let key = [7u8; 32];
        let mut a = sign_release_artifact("h", "kernel", "0.2.0+abc", 0, "p1", key);
        a.signature_hex.clear();
        assert!(verify_release_signature(&a).is_err());
    }
}

