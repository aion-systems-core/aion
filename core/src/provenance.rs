use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProvenanceSubject {
    pub name: String,
    pub digest_sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProvenancePredicate {
    pub build_environment: Vec<String>,
    pub build_steps: Vec<String>,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub signatures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProvenanceStatement {
    pub provenance_id: String,
    pub subjects: Vec<ProvenanceSubject>,
    pub predicate: ProvenancePredicate,
}

pub fn generate_provenance(
    subjects: Vec<ProvenanceSubject>,
    mut predicate: ProvenancePredicate,
) -> ProvenanceStatement {
    predicate.build_environment.sort();
    predicate.build_steps.sort();
    predicate.inputs.sort();
    predicate.outputs.sort();
    predicate.signatures.sort();
    let body = serde_json::to_string(&(subjects.clone(), predicate.clone())).unwrap_or_default();
    let mut h = Sha256::new();
    h.update(body.as_bytes());
    ProvenanceStatement {
        provenance_id: format!("{:x}", h.finalize()),
        subjects,
        predicate,
    }
}

pub fn verify_provenance(p: &ProvenanceStatement) -> Result<(), String> {
    if p.subjects.is_empty() {
        return Err("provenance:subject_missing".to_string());
    }
    if p.predicate.build_steps.is_empty() {
        return Err("provenance:build_steps_missing".to_string());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{generate_provenance, verify_provenance, ProvenancePredicate, ProvenanceSubject};

    #[test]
    fn deterministic_provenance() {
        let s = vec![ProvenanceSubject {
            name: "aion-cli".to_string(),
            digest_sha256: "h".to_string(),
        }];
        let p = ProvenancePredicate {
            build_environment: vec!["B".into(), "A".into()],
            build_steps: vec!["step1".into()],
            inputs: vec!["in".into()],
            outputs: vec!["out".into()],
            signatures: vec!["sig".into()],
        };
        let a = generate_provenance(s.clone(), p.clone());
        let b = generate_provenance(s, p);
        assert_eq!(a.provenance_id, b.provenance_id);
        assert!(verify_provenance(&a).is_ok());
    }

    #[test]
    fn invalid_provenance_is_rejected() {
        let p = generate_provenance(
            vec![],
            ProvenancePredicate {
                build_environment: vec![],
                build_steps: vec![],
                inputs: vec![],
                outputs: vec![],
                signatures: vec![],
            },
        );
        assert!(verify_provenance(&p).is_err());
    }
}
