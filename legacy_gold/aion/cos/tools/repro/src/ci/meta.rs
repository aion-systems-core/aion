//! CI runner metadata — **not** part of diff comparison keys; append-only
//! sidecar for humans and downstream CI systems.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct CiExecutionContext {
    /// e.g. `github-actions`, `gitlab`, `local`, `file-ingest`
    #[serde(default)]
    pub runner: String,
    pub commit_sha: Option<String>,
    pub workflow_name: Option<String>,
    /// True when `detect_ci_context` saw a CI environment.
    #[serde(default)]
    pub ci: bool,
    #[serde(default)]
    pub job_id: Option<String>,
    #[serde(default)]
    pub repo: Option<String>,
    #[serde(default)]
    pub branch: Option<String>,
}

impl CiExecutionContext {
    pub fn local_default() -> Self {
        Self {
            runner: "local".into(),
            commit_sha: None,
            workflow_name: None,
            ci: false,
            job_id: None,
            repo: None,
            branch: None,
        }
    }

    pub fn ingest_default() -> Self {
        Self {
            runner: "file-ingest".into(),
            commit_sha: None,
            workflow_name: None,
            ci: false,
            job_id: None,
            repo: None,
            branch: None,
        }
    }
}
