use super::version_info::VersionInfo;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VersionManifest {
    pub versions: Vec<VersionInfo>,
}
