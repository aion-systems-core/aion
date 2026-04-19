// CLI surface. Only responsible for parsing args and dispatching to handlers.
// No core logic is allowed in this module tree.

pub mod ci_handler;
pub mod diff_handler;
pub mod eval_handler;
pub mod graph_handler;
pub mod replay_handler;
pub mod root_cause_handler;
pub mod run_handler;
pub mod why_handler;

use clap::{Parser, Subcommand};

/// Single-sentence product category. Locked in Phase 5 and referenced
/// by `--help`, the README, and `analysis::system_evaluation`. Must
/// stay byte-identical to `system_evaluation::CATEGORY_DEFINITION`.
///
/// Rather than duplicate the literal, we re-export it from the
/// analysis layer: one source, many call sites.
pub use crate::analysis::system_evaluation::CATEGORY_DEFINITION as CATEGORY_LINE;

#[derive(Parser, Debug)]
#[command(
    name = "repro",
    version,
    about = CATEGORY_LINE,
    long_about = CATEGORY_LINE,
    disable_help_subcommand = true,
    after_help = "AION Repro — use via `aion repro …`. Commands: run, replay, diff, why, root-cause, graph, ci. Pair why: `repro why <run_a> <run_b>`."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Capture a command into local storage under `./repro_runs/`.
    Run {
        /// Command and arguments after `--`.
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, num_args = 1..)]
        command: Vec<String>,
    },
    /// Replay a stored run by id, or `last` for the most recent one.
    Replay {
        /// Run id, or the literal `last`.
        run_id: String,
    },
    /// Diff two stored runs field-by-field.
    Diff {
        /// First run id, or `last` / `prev`.
        run_a: String,
        /// Second run id, or `last` / `prev`.
        run_b: String,
    },
    /// Print the deterministic causal graph projection for a stored run.
    Graph {
        /// Run id, or the literal `last`.
        run_id: String,
    },
    /// Causal query: immediate causes/effects for a focal node + graph divergence vs previous.
    #[command(name = "why")]
    Why {
        /// Focal run id, or `last`.
        run_id: String,
        /// Optional second run id for an explicit AION pair query.
        compare_to: Option<String>,
    },
    /// Report the primary semantic cause between a run and the previous run.
    #[command(name = "root-cause")]
    RootCause {
        /// Run id, or `last`.
        run_id: String,
    },
    /// Emit a full self-evaluation report (markdown + JSON).
    #[command(hide = true)]
    Eval,
    /// CI ledger: same capture engine as `run`, stored under `./repro_ci_store/`.
    Ci {
        #[command(subcommand)]
        command: ci_handler::CiCommand,
    },
}

/// Entry used by the `repro` binary (`argv` from the environment).
pub fn run() -> Result<(), String> {
    let cli = Cli::try_parse().unwrap_or_else(|e| e.exit());
    dispatch(cli)
}

/// Parse and dispatch as if `args_os` were `std::env::args_os()` for the `repro`
/// executable (first element is argv0 / binary name). Used by the AION router
/// to forward `aion repro …` without duplicating handler logic.
pub fn run_from_args_os<I>(args_os: I) -> Result<(), String>
where
    I: IntoIterator<Item = std::ffi::OsString>,
{
    let cli = Cli::try_parse_from(args_os).unwrap_or_else(|e| e.exit());
    dispatch(cli)
}

fn dispatch(cli: Cli) -> Result<(), String> {
    match cli.command {
        Command::Run { command } => run_handler::handle(command),
        Command::Replay { run_id } => replay_handler::handle(&run_id),
        Command::Diff { run_a, run_b } => diff_handler::handle(&run_a, &run_b),
        Command::Graph { run_id } => graph_handler::handle(&run_id),
        Command::Why {
            run_id,
            compare_to,
        } => why_handler::handle(&run_id, compare_to.as_deref()),
        Command::RootCause { run_id } => root_cause_handler::handle(&run_id),
        Command::Eval => eval_handler::handle(),
        Command::Ci { command } => ci_handler::handle(command),
    }
}
