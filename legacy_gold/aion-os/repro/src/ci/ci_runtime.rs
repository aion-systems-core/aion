//! CI environment detection. Reads process environment (allowed under `src/ci/` only).

use crate::ci::meta::CiExecutionContext;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CiRunContext {
    pub ci: bool,
    pub job_id: Option<String>,
    pub repo: Option<String>,
    pub branch: Option<String>,
    pub commit: Option<String>,
}

/// Populate `meta` from detected CI signals (never mutates [`crate::core::artifact::ExecutionArtifact`]).
pub fn attach_ci_metadata(meta: &mut CiExecutionContext, ctx: &CiRunContext) {
    meta.ci = ctx.ci;
    meta.job_id = ctx.job_id.clone();
    meta.repo = ctx.repo.clone();
    meta.branch = ctx.branch.clone();
    meta.commit_sha = ctx.commit.clone();

    if !ctx.ci {
        return;
    }
    if std::env::var("GITHUB_ACTIONS").unwrap_or_default() == "true" {
        meta.runner = "github-actions".into();
    } else if std::env::var("GITLAB_CI").unwrap_or_default() == "true" {
        meta.runner = "gitlab".into();
    } else {
        meta.runner = "ci-generic".into();
    }
}

/// Detect common CI vendor environment variables.
pub fn detect_ci_context() -> CiRunContext {
    let github = std::env::var("GITHUB_ACTIONS").unwrap_or_default() == "true";
    let gitlab = std::env::var("GITLAB_CI").unwrap_or_default() == "true";
    let ci_flag = matches!(
        std::env::var("CI").unwrap_or_default().as_str(),
        "true" | "1" | "True" | "TRUE"
    );
    let ci = ci_flag || github || gitlab;

    let mut ctx = CiRunContext {
        ci,
        job_id: None,
        repo: None,
        branch: None,
        commit: None,
    };

    if github {
        ctx.job_id = std::env::var("GITHUB_RUN_ID").ok();
        ctx.repo = std::env::var("GITHUB_REPOSITORY").ok();
        ctx.branch = std::env::var("GITHUB_REF_NAME")
            .ok()
            .or_else(|| std::env::var("GITHUB_HEAD_REF").ok());
        ctx.commit = std::env::var("GITHUB_SHA").ok();
    } else if gitlab {
        ctx.job_id = std::env::var("CI_PIPELINE_ID")
            .ok()
            .or_else(|| std::env::var("CI_JOB_ID").ok());
        ctx.repo = std::env::var("CI_PROJECT_PATH").ok();
        ctx.branch = std::env::var("CI_COMMIT_REF_NAME").ok();
        ctx.commit = std::env::var("CI_COMMIT_SHA").ok();
    }

    ctx
}
