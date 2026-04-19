//! Deterministic, static tool registry — entries built via `register_tools()` only.

use crate::core::repro_tool::ReproTool;
use crate::core::tool_contract::AionTool;
use std::sync::OnceLock;

/// One registered tool: metadata + type-erased executor (`Vec<String>` after tool name).
pub struct ToolEntry {
    pub spec: crate::core::tool_contract::ToolSpec,
    pub executor: fn(Vec<String>) -> Result<(), String>,
}

fn dispatch_repro(args: Vec<String>) -> Result<(), String> {
    ReproTool::execute(args)
}

/// Static registration order is the contract order (deterministic, explicit).
pub fn register_tools() -> Vec<ToolEntry> {
    vec![ToolEntry {
        spec: ReproTool::spec(),
        executor: dispatch_repro,
    }]
}

static TOOLS: OnceLock<Vec<ToolEntry>> = OnceLock::new();

#[must_use]
pub fn registered_tools() -> &'static [ToolEntry] {
    TOOLS.get_or_init(register_tools)
}

#[must_use]
pub fn lookup_entry(tool_id: &str) -> Option<&'static ToolEntry> {
    registered_tools()
        .iter()
        .find(|e| e.spec.name == tool_id)
}

pub fn available_tool_names() -> String {
    registered_tools()
        .iter()
        .map(|e| e.spec.name.as_str())
        .collect::<Vec<_>>()
        .join(", ")
}
