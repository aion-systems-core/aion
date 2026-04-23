use crate::ai::AICapsuleV1;
use crate::ai::{ReplayComparison, ReplayReport};
use crate::governance::{DeterminismProfile, GovernanceReport, IntegrityProfile, PolicyProfile};
use crate::sdk;
use aion_core::DriftReport;

#[cfg(feature = "async")]
pub async fn replay_capsule_async(path: &str) -> Result<ReplayReport, String> {
    let p = path.to_string();
    tokio::task::spawn_blocking(move || {
        let c = sdk::load_capsule(std::path::Path::new(&p))?;
        Ok::<_, String>(sdk::replay_capsule(&c))
    })
    .await
    .map_err(|e| e.to_string())?
}

#[cfg(feature = "async")]
pub async fn compare_capsules_async(path_left: &str, path_right: &str) -> Result<ReplayComparison, String> {
    let l = path_left.to_string();
    let r = path_right.to_string();
    tokio::task::spawn_blocking(move || {
        let a = sdk::load_capsule(std::path::Path::new(&l))?;
        let b = sdk::load_capsule(std::path::Path::new(&r))?;
        Ok::<_, String>(sdk::compare_capsules(&a, &b))
    })
    .await
    .map_err(|e| e.to_string())?
}

#[cfg(feature = "async")]
pub async fn drift_between_async(path_a: &str, path_b: &str) -> Result<DriftReport, String> {
    let a = path_a.to_string();
    let b = path_b.to_string();
    tokio::task::spawn_blocking(move || {
        let ca = sdk::load_capsule(std::path::Path::new(&a))?;
        let cb = sdk::load_capsule(std::path::Path::new(&b))?;
        Ok::<_, String>(sdk::drift_between(&ca, &cb))
    })
    .await
    .map_err(|e| e.to_string())?
}

#[cfg(feature = "async")]
pub async fn why_explain_async(path: &str) -> Result<String, String> {
    let p = path.to_string();
    tokio::task::spawn_blocking(move || {
        let c = sdk::load_capsule(std::path::Path::new(&p))?;
        let e = sdk::explain_capsule(&c);
        serde_json::to_string(&e.why).map_err(|x| x.to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}

#[cfg(feature = "async")]
pub async fn graph_causal_async(path: &str) -> Result<String, String> {
    let p = path.to_string();
    tokio::task::spawn_blocking(move || {
        let c: AICapsuleV1 = sdk::load_capsule(std::path::Path::new(&p))?;
        serde_json::to_string(&c.graph).map_err(|x| x.to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}

#[cfg(feature = "async")]
pub async fn validate_async(path: &str, policy: &str) -> Result<GovernanceReport, String> {
    let p = path.to_string();
    let pol = policy.to_string();
    tokio::task::spawn_blocking(move || {
        let c = sdk::load_capsule(std::path::Path::new(&p))?;
        let profile: PolicyProfile = governance_profile_from_path(&pol)?;
        Ok::<_, String>(sdk::validate_capsule(
            &c,
            &profile,
            &DeterminismProfile::default(),
            &IntegrityProfile::default(),
        ))
    })
    .await
    .map_err(|e| e.to_string())?
}

#[cfg(feature = "async")]
fn governance_profile_from_path(path: &str) -> Result<PolicyProfile, String> {
    crate::governance::load_policy(std::path::Path::new(path))
}
