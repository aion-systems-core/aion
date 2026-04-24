//! AION SDK v1 — minimal stable API over deterministic AI engine primitives (no CLI).

#[cfg(feature = "async")]
pub mod async_api;
pub mod capsule;
pub mod ci;
pub mod drift;
pub mod error;
pub mod explain;
pub mod governance;
pub mod output;
pub mod replay;

#[cfg(feature = "async")]
pub use async_api::{
    compare_capsules_async, drift_between_async, graph_causal_async, replay_capsule_async,
    validate_async, why_explain_async,
};
pub use capsule::{build_capsule, load_capsule, save_capsule};
pub use ci::{ci_check, ci_record_baseline};
pub use drift::drift_between;
pub use error::SdkError;
pub use explain::{explain_capsule, why_diff};
pub use governance::validate_capsule;
pub use output::{
    json_pretty, render_sdk_html, render_sdk_svg, sdk_output_dir, sdk_version, write_output_bundle,
    SdkMeta,
};
pub use replay::{compare_capsules, replay_capsule};
