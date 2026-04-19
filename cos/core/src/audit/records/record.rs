#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AuditRecord {
    pub timestamp: String,
    pub actor: String,
    pub action: String,
    pub details: String,
}
