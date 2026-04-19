//! Pure dispatch: parse argv → resolve [`ToolEntry`](crate::cli::registry::ToolEntry) → execute.

use crate::cli::error::AionError;
use crate::cli::registry;
use clap::error::ErrorKind;
use clap::Parser;

const ABOUT: &str = "AION — deterministic execution platform (tool shell)";

#[derive(Parser, Debug)]
#[command(
    name = "aion",
    version,
    about = ABOUT,
    long_about = ABOUT,
    disable_help_subcommand = true
)]
pub struct AionCli {
    /// Tool ID (e.g. repro).
    #[arg(value_name = "TOOL", required = true)]
    pub tool: String,
    /// Arguments forwarded to the tool (same as invoking `<tool> …` directly).
    #[arg(trailing_var_arg = true, allow_hyphen_values = true, num_args = 0..)]
    pub rest: Vec<String>,
}

pub fn route() -> Result<(), AionError> {
    let cli = match AionCli::try_parse() {
        Ok(cli) => cli,
        Err(e) if matches!(e.kind(), ErrorKind::DisplayHelp | ErrorKind::DisplayVersion) => {
            let _ = e.print();
            return Ok(());
        }
        Err(e) => return Err(AionError::InvalidArgs(e.to_string())),
    };

    let entry = registry::lookup_entry(cli.tool.as_str()).ok_or_else(|| {
        AionError::ToolNotFound {
            requested: cli.tool.clone(),
            available: registry::available_tool_names(),
        }
    })?;

    (entry.executor)(cli.rest).map_err(AionError::ExecutionFailed)
}
