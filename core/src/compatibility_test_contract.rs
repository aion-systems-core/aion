use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum CompatibilityDimension {
    Version,
    Os,
    Arch,
    Abi,
    ContractVersion,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CompatibilityCase {
    pub id: String,
    pub version: String,
    pub os: String,
    pub arch: String,
    pub abi: String,
    pub contract_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CompatibilityResult {
    pub case_id: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CompatibilityTestContract {
    pub dimensions: Vec<CompatibilityDimension>,
    pub cases: Vec<CompatibilityCase>,
    pub results: Vec<CompatibilityResult>,
    pub status: String,
}

pub fn evaluate_compatibility_test_contract(
    mut cases: Vec<CompatibilityCase>,
    mut results: Vec<CompatibilityResult>,
) -> CompatibilityTestContract {
    cases.sort_by(|a, b| a.id.cmp(&b.id));
    results.sort_by(|a, b| a.case_id.cmp(&b.case_id));
    let has_n = cases.iter().any(|c| c.version == "N");
    let has_n1 = cases.iter().any(|c| c.version == "N-1");
    let has_n2 = cases.iter().any(|c| c.version == "N-2");
    let has_gap = results.iter().any(|r| r.status != "ok");
    let status = if has_n && has_n1 && has_n2 && !has_gap {
        "ok"
    } else {
        "error"
    };
    CompatibilityTestContract {
        dimensions: vec![
            CompatibilityDimension::Version,
            CompatibilityDimension::Os,
            CompatibilityDimension::Arch,
            CompatibilityDimension::Abi,
            CompatibilityDimension::ContractVersion,
        ],
        cases,
        results,
        status: status.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn missing_n_minus_2_negative() {
        let c = evaluate_compatibility_test_contract(
            vec![
                CompatibilityCase {
                    id: "c1".into(),
                    version: "N".into(),
                    os: "linux".into(),
                    arch: "x64".into(),
                    abi: "v1".into(),
                    contract_version: "1".into(),
                },
                CompatibilityCase {
                    id: "c2".into(),
                    version: "N-1".into(),
                    os: "linux".into(),
                    arch: "x64".into(),
                    abi: "v1".into(),
                    contract_version: "1".into(),
                },
            ],
            vec![
                CompatibilityResult {
                    case_id: "c1".into(),
                    status: "ok".into(),
                },
                CompatibilityResult {
                    case_id: "c2".into(),
                    status: "ok".into(),
                },
            ],
        );
        assert_eq!(c.status, "error");
    }
}
