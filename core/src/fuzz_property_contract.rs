use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FuzzTarget {
    pub name: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FuzzFinding {
    pub id: String,
    pub severity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FuzzTestContract {
    pub targets: Vec<FuzzTarget>,
    pub findings: Vec<FuzzFinding>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PropertyTarget {
    pub name: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PropertyInvariant {
    pub id: String,
    pub statement: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PropertyTestContract {
    pub targets: Vec<PropertyTarget>,
    pub invariants: Vec<PropertyInvariant>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FuzzPropertyContract {
    pub fuzz: FuzzTestContract,
    pub property: PropertyTestContract,
    pub status: String,
}

pub fn evaluate_fuzz_property_contract(mut contract: FuzzPropertyContract) -> FuzzPropertyContract {
    contract.fuzz.targets.sort_by(|a, b| a.name.cmp(&b.name));
    contract.fuzz.findings.sort_by(|a, b| a.id.cmp(&b.id));
    contract.property.targets.sort_by(|a, b| a.name.cmp(&b.name));
    contract
        .property
        .invariants
        .sort_by(|a, b| a.id.cmp(&b.id));
    let has_gap = contract
        .fuzz
        .targets
        .iter()
        .any(|t| t.status == "gaps" || t.status == "planned")
        || contract
            .property
            .targets
            .iter()
            .any(|t| t.status == "gaps" || t.status == "planned");
    contract.status = if has_gap { "gaps".into() } else { "implemented".into() };
    contract
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fuzz_property_gaps_negative() {
        let c = evaluate_fuzz_property_contract(FuzzPropertyContract {
            fuzz: FuzzTestContract {
                targets: vec![FuzzTarget {
                    name: "replay_parser".into(),
                    status: "planned".into(),
                }],
                findings: vec![],
            },
            property: PropertyTestContract {
                targets: vec![PropertyTarget {
                    name: "evidence_chain".into(),
                    status: "implemented".into(),
                }],
                invariants: vec![PropertyInvariant {
                    id: "inv1".into(),
                    statement: "hash_stable".into(),
                }],
            },
            status: String::new(),
        });
        assert_eq!(c.status, "gaps");
    }
}

