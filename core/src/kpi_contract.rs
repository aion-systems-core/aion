use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum KpiDomain {
    Reliability,
    Security,
    Compliance,
    Operations,
    Adoption,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum KpiStatus {
    OnTrack,
    AtRisk,
    OffTrack,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KpiTarget {
    pub threshold: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KpiDefinition {
    pub id: String,
    pub domain: KpiDomain,
    pub target: Option<KpiTarget>,
    pub status: Option<KpiStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KpiContract {
    pub kpis: Vec<KpiDefinition>,
    pub status: String,
}

pub fn evaluate_kpi_contract(mut kpis: Vec<KpiDefinition>) -> KpiContract {
    kpis.sort_by(|a, b| a.id.cmp(&b.id));
    let invalid = kpis
        .iter()
        .any(|k| k.target.is_none() || k.status.is_none());
    KpiContract {
        kpis,
        status: if invalid { "error" } else { "ok" }.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn kpi_without_target_or_status_negative() {
        let c = evaluate_kpi_contract(vec![KpiDefinition {
            id: "kpi_mttr".into(),
            domain: KpiDomain::Reliability,
            target: None,
            status: Some(KpiStatus::AtRisk),
        }]);
        assert_eq!(c.status, "error");
    }
}
