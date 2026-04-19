// AION — sole product CLI entrypoint; tools (e.g. repro) are routed, not embedded kernel logic.

use std::process::ExitCode;

fn main() -> ExitCode {
    match aion::cli::router::route() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("aion: error: {e}");
            ExitCode::from(1)
        }
    }
}
