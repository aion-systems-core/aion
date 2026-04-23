//! Minimal CLI stub: prints a deterministic JSON envelope.

fn main() {
    println!(
        r#"{{"status":"ok","data":{{"crate":"aion-cli","note":"stub"}},"error":null}}"#
    );
}
