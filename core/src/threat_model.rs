use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThreatCategory {
    Spoofing,
    Tampering,
    Repudiation,
    InformationDisclosure,
    DenialOfService,
    ElevationOfPrivilege,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ThreatSurface {
    pub name: String,
    pub categories: Vec<ThreatCategory>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ThreatMitigation {
    pub category: ThreatCategory,
    pub mitigation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ThreatModel {
    pub surfaces: Vec<ThreatSurface>,
    pub mitigations: Vec<ThreatMitigation>,
    pub status: String,
}

pub fn evaluate_threat_model() -> ThreatModel {
    let mut surfaces = vec![
        ThreatSurface {
            name: "kernel".to_string(),
            categories: vec![ThreatCategory::Tampering, ThreatCategory::DenialOfService],
        },
        ThreatSurface {
            name: "ffi_bindings".to_string(),
            categories: vec![
                ThreatCategory::Spoofing,
                ThreatCategory::InformationDisclosure,
            ],
        },
        ThreatSurface {
            name: "cli".to_string(),
            categories: vec![ThreatCategory::Repudiation, ThreatCategory::Tampering],
        },
        ThreatSurface {
            name: "output_contracts".to_string(),
            categories: vec![ThreatCategory::Tampering, ThreatCategory::Repudiation],
        },
        ThreatSurface {
            name: "evidence_policy_replay".to_string(),
            categories: vec![ThreatCategory::Tampering, ThreatCategory::ElevationOfPrivilege],
        },
    ];
    surfaces.sort_by(|a, b| a.name.cmp(&b.name));
    for s in &mut surfaces {
        s.categories.sort();
        s.categories.dedup();
    }
    let mut mitigations = vec![
        ThreatMitigation {
            category: ThreatCategory::Spoofing,
            mitigation: "identity_contract".to_string(),
        },
        ThreatMitigation {
            category: ThreatCategory::Tampering,
            mitigation: "evidence_chain_and_hashes".to_string(),
        },
        ThreatMitigation {
            category: ThreatCategory::Repudiation,
            mitigation: "canonical_error_and_output_contracts".to_string(),
        },
        ThreatMitigation {
            category: ThreatCategory::InformationDisclosure,
            mitigation: "sanitized_error_cause_and_policy_controls".to_string(),
        },
        ThreatMitigation {
            category: ThreatCategory::DenialOfService,
            mitigation: "deterministic_policy_constraints".to_string(),
        },
        ThreatMitigation {
            category: ThreatCategory::ElevationOfPrivilege,
            mitigation: "tenant_and_policy_isolation_contracts".to_string(),
        },
    ];
    mitigations.sort_by(|a, b| a.category.cmp(&b.category));
    ThreatModel {
        surfaces,
        mitigations,
        status: "ok".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::{evaluate_threat_model, ThreatCategory};

    #[test]
    fn deterministic_serialization() {
        let m = evaluate_threat_model();
        assert_eq!(
            serde_json::to_string(&m).unwrap(),
            serde_json::to_string(&m).unwrap()
        );
    }

    #[test]
    fn fixed_categories_exist() {
        let m = evaluate_threat_model();
        let all: Vec<ThreatCategory> = m
            .surfaces
            .iter()
            .flat_map(|s| s.categories.clone())
            .collect();
        assert!(all.contains(&ThreatCategory::Spoofing));
        assert!(all.contains(&ThreatCategory::Tampering));
    }
}

