use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum IdentityDimension {
    KernelVersion,
    AbiVersion,
    ContractSpecVersion,
    Os,
    Arch,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IdentityEntry {
    pub kernel_version: String,
    pub abi_version: String,
    pub contract_spec_version: String,
    pub os: String,
    pub arch: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IdentityCompatibilityStatus {
    pub key: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IdentityMatrix {
    pub dimensions: Vec<IdentityDimension>,
    pub entries: Vec<IdentityEntry>,
    pub compatibility: Vec<IdentityCompatibilityStatus>,
    pub status: String,
}

pub fn evaluate_identity_matrix(mut entries: Vec<IdentityEntry>) -> IdentityMatrix {
    entries.sort_by(|a, b| {
        a.kernel_version
            .cmp(&b.kernel_version)
            .then(a.os.cmp(&b.os))
            .then(a.arch.cmp(&b.arch))
    });
    let compatibility = entries
        .iter()
        .map(|e| IdentityCompatibilityStatus {
            key: format!(
                "{}|{}|{}|{}|{}",
                e.kernel_version, e.abi_version, e.contract_spec_version, e.os, e.arch
            ),
            status: e.status.clone(),
        })
        .collect::<Vec<_>>();
    let status = if entries.iter().any(|e| e.status == "not_supported") {
        "error"
    } else {
        "ok"
    };
    IdentityMatrix {
        dimensions: vec![
            IdentityDimension::KernelVersion,
            IdentityDimension::AbiVersion,
            IdentityDimension::ContractSpecVersion,
            IdentityDimension::Os,
            IdentityDimension::Arch,
        ],
        entries,
        compatibility,
        status: status.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn invalid_combo_negative() {
        let m = evaluate_identity_matrix(vec![IdentityEntry {
            kernel_version: "1".into(),
            abi_version: "v1".into(),
            contract_spec_version: "x".into(),
            os: "linux".into(),
            arch: "x64".into(),
            status: "not_supported".into(),
        }]);
        assert_eq!(m.status, "error");
    }
}

