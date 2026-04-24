use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum AdminDocSection {
    Architecture,
    Operations,
    Security,
    Troubleshooting,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdminDocCoverage {
    pub section: AdminDocSection,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdminDocContract {
    pub coverage: Vec<AdminDocCoverage>,
    pub status: String,
}

pub fn evaluate_admin_docs_contract(mut contract: AdminDocContract) -> AdminDocContract {
    contract.coverage.sort_by(|a, b| a.section.cmp(&b.section));
    let required = vec![
        AdminDocSection::Architecture,
        AdminDocSection::Operations,
        AdminDocSection::Security,
        AdminDocSection::Troubleshooting,
    ];
    let all_present = required
        .into_iter()
        .all(|s| contract.coverage.iter().any(|c| c.section == s));
    let has_missing = contract.coverage.iter().any(|c| c.status == "missing");
    contract.status = if !all_present || has_missing {
        "missing".into()
    } else if contract.coverage.iter().any(|c| c.status == "partial") {
        "partial".into()
    } else {
        "complete".into()
    };
    contract
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn missing_admin_sections_negative() {
        let c = evaluate_admin_docs_contract(AdminDocContract {
            coverage: vec![AdminDocCoverage {
                section: AdminDocSection::Architecture,
                status: "complete".into(),
            }],
            status: String::new(),
        });
        assert_eq!(c.status, "missing");
    }
}
