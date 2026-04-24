use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum DistributionChannel {
    Source,
    Binary,
    Container,
    PackageManager,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DistributionArtifact {
    pub name: String,
    pub version: String,
    pub platform: String,
    pub channel: DistributionChannel,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DistributionSupportStatus {
    pub platform: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DistributionContract {
    pub artifacts: Vec<DistributionArtifact>,
    pub support: Vec<DistributionSupportStatus>,
    pub status: String,
}

pub fn evaluate_distribution_contract(
    mut artifacts: Vec<DistributionArtifact>,
    mut support: Vec<DistributionSupportStatus>,
) -> DistributionContract {
    artifacts.sort_by(|a, b| a.name.cmp(&b.name).then(a.platform.cmp(&b.platform)));
    support.sort_by(|a, b| a.platform.cmp(&b.platform));
    let status = if artifacts.is_empty()
        || artifacts.iter().any(|a| a.status == "unknown")
        || support.is_empty()
    {
        "error"
    } else {
        "ok"
    };
    DistributionContract {
        artifacts,
        support,
        status: status.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn artifact_missing_entry_negative() {
        let c = evaluate_distribution_contract(vec![], vec![]);
        assert_eq!(c.status, "error");
    }
}
