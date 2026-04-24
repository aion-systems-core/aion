use serde::{Deserialize, Serialize};

use crate::{DistributionContract, IdentityMatrix, InstallerTrustChain, LtsPolicy};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DistributionModel {
    pub distribution_status: DistributionContract,
    pub identity_matrix: IdentityMatrix,
    pub lts_policy: LtsPolicy,
    pub installer_trust_chain: InstallerTrustChain,
    pub status: String,
}

pub fn evaluate_distribution_model(
    distribution_status: DistributionContract,
    identity_matrix: IdentityMatrix,
    lts_policy: LtsPolicy,
    installer_trust_chain: InstallerTrustChain,
) -> DistributionModel {
    let status = if distribution_status.status == "ok"
        && identity_matrix.status == "ok"
        && lts_policy.status == "ok"
        && installer_trust_chain.status == "trusted"
    {
        "ok"
    } else {
        "error"
    };
    DistributionModel {
        distribution_status,
        identity_matrix,
        lts_policy,
        installer_trust_chain,
        status: status.into(),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        evaluate_distribution_contract, evaluate_identity_matrix, evaluate_installer_trust_chain,
        evaluate_lts_policy, DistributionArtifact, DistributionChannel, DistributionSupportStatus,
        EolPolicy, IdentityEntry, InstallerArtifact, InstallerSignature, InstallerType, LtsChannel,
        SupportWindow,
    };

    use super::evaluate_distribution_model;

    #[test]
    fn distribution_model_consistent_positive() {
        let d = evaluate_distribution_contract(
            vec![DistributionArtifact {
                name: "aion-cli".into(),
                version: "1.0.0".into(),
                platform: "windows-x64".into(),
                channel: DistributionChannel::Binary,
                status: "supported".into(),
            }],
            vec![DistributionSupportStatus {
                platform: "windows-x64".into(),
                status: "supported".into(),
            }],
        );
        let i = evaluate_identity_matrix(vec![IdentityEntry {
            kernel_version: "1.0.0".into(),
            abi_version: "v1".into(),
            contract_spec_version: "spec1".into(),
            os: "windows".into(),
            arch: "x64".into(),
            status: "supported".into(),
        }]);
        let l = evaluate_lts_policy(
            LtsChannel::Lts12,
            Some(SupportWindow {
                months: 12,
                starts_at: "2026-01-01".into(),
            }),
            EolPolicy {
                status: "supported".into(),
                eol_date: "2027-01-01".into(),
            },
        );
        let t = evaluate_installer_trust_chain(
            vec![InstallerArtifact {
                name: "aion-cli".into(),
                installer_type: InstallerType::ManualBinary,
                provenance_ref: "prov".into(),
            }],
            vec![InstallerSignature {
                signature_id: "sig1".into(),
                algorithm: "ed25519".into(),
                trusted: true,
            }],
        );
        assert_eq!(evaluate_distribution_model(d, i, l, t).status, "ok");
    }
}
