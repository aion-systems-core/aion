// AION Repro — deterministic debugging for execution (diff, why, replay, CI ledger).

use std::process::ExitCode;

fn main() -> ExitCode {
    match repro::cli::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("repro: error: {e}");
            ExitCode::from(1)
        }
    }
}
