use super::record::AuditRecord;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AuditChain {
    pub records: Vec<AuditRecord>,
}
