#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VersionInfo {
    pub schema_version: String,
    pub output_version: String,
    pub contract_version: String,
}
