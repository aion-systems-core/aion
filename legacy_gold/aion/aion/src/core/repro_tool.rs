//! REPRO as the first `AionTool` — thin wrapper over the `repro` library CLI only.

use crate::core::tool_contract::{AionTool, ToolSpec};
use std::ffi::OsString;

/// Unit type marking the REPRO tool; all behavior is delegated to the `repro` crate.
pub struct ReproTool;

/// Must stay aligned with the `repro` dependency crate version (surface contract only).
const REPRO_CRATE_VERSION: &str = "0.1.0";

impl AionTool for ReproTool {
    fn spec() -> ToolSpec {
        ToolSpec {
            name: "repro".to_string(),
            description: "deterministic execution + diff + causal reasoning".to_string(),
            version: REPRO_CRATE_VERSION.to_string(),
        }
    }

    fn execute(args: Vec<String>) -> Result<(), String> {
        let chain = std::iter::once(OsString::from("repro")).chain(args.into_iter().map(OsString::from));
        repro::cli::run_from_args_os(chain)
    }
}
