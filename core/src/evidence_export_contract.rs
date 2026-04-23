use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum EvidenceExportFormat {
    Json,
    NdJson,
    Parquet,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum EvidenceExportScope {
    Replay,
    Policy,
    Governance,
    Security,
    Ops,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EvidenceExportRequest {
    pub scope: EvidenceExportScope,
    pub format: EvidenceExportFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EvidenceExportResult {
    pub request: EvidenceExportRequest,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EvidenceExportContract {
    pub results: Vec<EvidenceExportResult>,
    pub status: String,
}

pub fn evaluate_evidence_export_contract(
    mut results: Vec<EvidenceExportResult>,
) -> EvidenceExportContract {
    results.sort_by(|a, b| {
        format!("{:?}{:?}", a.request.scope, a.request.format)
            .cmp(&format!("{:?}{:?}", b.request.scope, b.request.format))
    });
    let unsupported = results.iter().any(|r| r.status == "unsupported");
    let partial = results.iter().any(|r| r.status == "partial");
    let status = if unsupported {
        "unsupported"
    } else if partial {
        "partial"
    } else {
        "supported"
    };
    EvidenceExportContract {
        results,
        status: status.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn export_unsupported_negative() {
        let c = evaluate_evidence_export_contract(vec![EvidenceExportResult {
            request: EvidenceExportRequest {
                scope: EvidenceExportScope::Replay,
                format: EvidenceExportFormat::Parquet,
            },
            status: "unsupported".into(),
        }]);
        assert_eq!(c.status, "unsupported");
    }
}

