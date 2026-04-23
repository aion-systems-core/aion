//! Typed tool platform contract — no execution logic, no kernel imports, no tool crates.

/// Immutable metadata for a registered AION tool (user-facing surface).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolSpec {
    pub name: String,
    pub description: String,
    pub version: String,
}

/// Contract every first-class tool implements. Dispatch is via the static registry.
pub trait AionTool {
    fn spec() -> ToolSpec;
    fn execute(args: Vec<String>) -> Result<(), String>;
}
