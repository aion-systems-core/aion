use serde::{Deserialize, Serialize};

use crate::{AuditReport, EvidenceExportContract, KpiContract, MetricsContract};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MeasurementGap {
    pub area: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MeasurementStatus {
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MeasurementModel {
    pub metrics_contract: MetricsContract,
    pub kpi_contract: KpiContract,
    pub audit_reports: AuditReport,
    pub evidence_export: EvidenceExportContract,
    pub status: MeasurementStatus,
    pub gaps: Vec<MeasurementGap>,
}

pub fn evaluate_measurement_model(
    metrics_contract: MetricsContract,
    kpi_contract: KpiContract,
    audit_reports: AuditReport,
    evidence_export: EvidenceExportContract,
) -> MeasurementModel {
    let mut gaps = Vec::new();
    if metrics_contract.status != "ok" {
        gaps.push(MeasurementGap {
            area: "metrics".into(),
            reason: "metrics_missing_or_invalid".into(),
        });
    }
    if kpi_contract.status != "ok" {
        gaps.push(MeasurementGap {
            area: "kpi".into(),
            reason: "kpi_target_or_status_missing".into(),
        });
    }
    if audit_reports.status == "failed" {
        gaps.push(MeasurementGap {
            area: "audit".into(),
            reason: "audit_report_failed".into(),
        });
    }
    if evidence_export.status == "unsupported" {
        gaps.push(MeasurementGap {
            area: "evidence_export".into(),
            reason: "unsupported_export".into(),
        });
    }
    MeasurementModel {
        metrics_contract,
        kpi_contract,
        audit_reports,
        evidence_export,
        status: MeasurementStatus {
            status: if gaps.is_empty() {
                "ok".into()
            } else {
                "error".into()
            },
        },
        gaps,
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        evaluate_audit_report_contract, evaluate_evidence_export_contract, evaluate_kpi_contract,
        evaluate_metrics_contract, AuditFinding, AuditFindingSeverity, AuditScope,
        EvidenceExportFormat, EvidenceExportRequest, EvidenceExportResult, EvidenceExportScope,
        KpiDefinition, KpiDomain, MetricDefinition, MetricNamespace, MetricStatus, MetricType,
    };

    use super::evaluate_measurement_model;

    #[test]
    fn measurement_model_consistent_positive() {
        let m = evaluate_metrics_contract(vec![MetricDefinition {
            name: "aion_kpi_total".into(),
            namespace: MetricNamespace::Ops,
            metric_type: MetricType::Counter,
            status: MetricStatus::Defined,
        }]);
        let k = evaluate_kpi_contract(vec![KpiDefinition {
            id: "kpi_001".into(),
            domain: KpiDomain::Operations,
            target: Some(crate::KpiTarget {
                threshold: "<=5".into(),
            }),
            status: Some(crate::KpiStatus::OnTrack),
        }]);
        let a = evaluate_audit_report_contract(vec![AuditFinding {
            id: "audit_001".into(),
            scope: AuditScope::Compliance,
            severity: AuditFindingSeverity::Low,
            evidence_ref: "ev_001".into(),
        }]);
        let e = evaluate_evidence_export_contract(vec![EvidenceExportResult {
            request: EvidenceExportRequest {
                scope: EvidenceExportScope::Governance,
                format: EvidenceExportFormat::Json,
            },
            status: "supported".into(),
        }]);
        let model = evaluate_measurement_model(m, k, a, e);
        assert_eq!(model.status.status, "ok");
    }
}
