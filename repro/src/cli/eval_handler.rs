// `repro eval`
//
// Run the full self-evaluation and print markdown followed by JSON.
// Both outputs are deterministic (pure function of embedded source +
// curated gap list), so this command is snapshottable.

use crate::analysis::system_evaluation;

pub fn handle() -> Result<(), String> {
    let report = system_evaluation::evaluate();
    print!(
        "{}",
        system_evaluation::sanitize_public_eval_output(&system_evaluation::render_markdown(
            &report
        ))
    );
    print!(
        "── json ──\n{}\n",
        system_evaluation::sanitize_public_eval_output(&system_evaluation::render_json(&report))
    );
    Ok(())
}
