//! AION platform core — tool contracts only (no embedded execution in this crate).

pub mod repro_tool;
pub mod tool_contract;

pub use repro_tool::ReproTool;
pub use tool_contract::{AionTool, ToolSpec};
