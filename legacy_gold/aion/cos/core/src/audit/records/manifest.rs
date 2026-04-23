#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AuditManifest {
    pub workflow_name: String,
    pub workflow_version: String,
    pub audit_chain: Vec<String>,
}
