use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum PolicyPackLevel {
    Baseline,
    Strict,
    Regulatory,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PolicyPackEntry {
    pub id: String,
    pub use_case: String,
    pub rule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PolicyPackSignature {
    pub signature_id: String,
    pub algorithm: String,
    pub valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PolicyPack {
    pub name: String,
    pub version: String,
    pub level: PolicyPackLevel,
    pub entries: Vec<PolicyPackEntry>,
    pub signature: Option<PolicyPackSignature>,
    pub status: String,
}

pub fn evaluate_policy_pack(mut pack: PolicyPack) -> PolicyPack {
    pack.entries.sort_by(|a, b| a.id.cmp(&b.id));
    pack.status = if pack.entries.is_empty() {
        "invalid".into()
    } else if pack.signature.is_none() {
        "unsigned".into()
    } else if !pack.signature.as_ref().map(|s| s.valid).unwrap_or(false) {
        "invalid".into()
    } else {
        "valid".into()
    };
    pack
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsigned_pack_negative() {
        let p = evaluate_policy_pack(PolicyPack {
            name: "baseline".into(),
            version: "1".into(),
            level: PolicyPackLevel::Baseline,
            entries: vec![PolicyPackEntry {
                id: "p1".into(),
                use_case: "internal".into(),
                rule: "must_have_policy".into(),
            }],
            signature: None,
            status: String::new(),
        });
        assert_eq!(p.status, "unsigned");
    }
}

