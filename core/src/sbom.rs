use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SbomHash {
    pub alg: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SbomComponent {
    pub name: String,
    pub version: String,
    pub license: String,
    pub hashes: Vec<SbomHash>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SbomDocument {
    pub format: String,
    pub build_metadata: Vec<String>,
    pub components: Vec<SbomComponent>,
}

pub fn generate_sbom(mut doc: SbomDocument) -> SbomDocument {
    doc.build_metadata.sort();
    for c in &mut doc.components {
        c.hashes.sort_by(|a, b| a.alg.cmp(&b.alg).then(a.value.cmp(&b.value)));
    }
    doc.components
        .sort_by(|a, b| a.name.cmp(&b.name).then(a.version.cmp(&b.version)));
    doc
}

pub fn verify_sbom(doc: &SbomDocument) -> Result<(), String> {
    if doc.components.is_empty() {
        return Err("sbom:components_missing".to_string());
    }
    if doc.components.iter().any(|c| c.hashes.is_empty()) {
        return Err("sbom:hash_missing".to_string());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{generate_sbom, verify_sbom, SbomComponent, SbomDocument, SbomHash};

    #[test]
    fn sbom_stability() {
        let doc = SbomDocument {
            format: "spdx".to_string(),
            build_metadata: vec!["b".into(), "a".into()],
            components: vec![SbomComponent {
                name: "x".to_string(),
                version: "1".to_string(),
                license: "MIT".to_string(),
                hashes: vec![SbomHash {
                    alg: "sha256".to_string(),
                    value: "h".to_string(),
                }],
            }],
        };
        let a = generate_sbom(doc.clone());
        let b = generate_sbom(doc);
        assert_eq!(serde_json::to_string(&a).unwrap(), serde_json::to_string(&b).unwrap());
        assert!(verify_sbom(&a).is_ok());
    }

    #[test]
    fn sbom_mismatch_negative() {
        let doc = SbomDocument {
            format: "cyclonedx".to_string(),
            build_metadata: vec![],
            components: vec![SbomComponent {
                name: "x".to_string(),
                version: "1".to_string(),
                license: "MIT".to_string(),
                hashes: vec![],
            }],
        };
        assert!(verify_sbom(&doc).is_err());
    }
}

