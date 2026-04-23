use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum InstallerType {
    Homebrew,
    Apt,
    Rpm,
    Container,
    ManualBinary,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InstallerArtifact {
    pub name: String,
    pub installer_type: InstallerType,
    pub provenance_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InstallerSignature {
    pub signature_id: String,
    pub algorithm: String,
    pub trusted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InstallerTrustChain {
    pub artifacts: Vec<InstallerArtifact>,
    pub signatures: Vec<InstallerSignature>,
    pub status: String,
}

pub fn evaluate_installer_trust_chain(
    mut artifacts: Vec<InstallerArtifact>,
    mut signatures: Vec<InstallerSignature>,
) -> InstallerTrustChain {
    artifacts.sort_by(|a, b| a.name.cmp(&b.name));
    signatures.sort_by(|a, b| a.signature_id.cmp(&b.signature_id));
    let status = if artifacts.is_empty() || signatures.is_empty() || signatures.iter().any(|s| !s.trusted) {
        "untrusted"
    } else {
        "trusted"
    };
    InstallerTrustChain {
        artifacts,
        signatures,
        status: status.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn installer_without_signature_negative() {
        let c = evaluate_installer_trust_chain(
            vec![InstallerArtifact {
                name: "aion-cli".into(),
                installer_type: InstallerType::ManualBinary,
                provenance_ref: "prov1".into(),
            }],
            vec![],
        );
        assert_eq!(c.status, "untrusted");
    }
}

