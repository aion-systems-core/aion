//! Reject OS introspection outside `core/execution_boundary.rs` (single chokepoint).

use std::fs;
use std::path::{Path, PathBuf};

fn rs_files_under(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    walk(dir, &mut out);
    out
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for e in entries.flatten() {
            let p = e.path();
            if p.is_dir() {
                walk(&p, out);
            } else if p.extension().and_then(|s| s.to_str()) == Some("rs") {
                out.push(p);
            }
        }
    }
}

fn is_boundary_file(p: &Path) -> bool {
    p.file_name().and_then(|s| s.to_str()) == Some("execution_boundary.rs")
}

fn is_ci_layer_file(p: &Path) -> bool {
    p.components().any(|c| c.as_os_str() == "ci")
}

fn allows_command_new(p: &Path) -> bool {
    matches!(
        p.file_name().and_then(|s| s.to_str()),
        Some("capture.rs") | Some("execution_boundary.rs")
    )
}

#[test]
fn no_os_introspection_outside_boundary() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    for p in rs_files_under(&root) {
        let text = fs::read_to_string(&p).unwrap_or_else(|e| panic!("read {}: {e}", p.display()));
        let boundary = is_boundary_file(&p);
        let ci_layer = is_ci_layer_file(&p);

        if !boundary && !ci_layer {
            for pat in [
                "std::env::vars",
                "env::vars",
                "std::env::var",
                "env::var",
                "env::consts::OS",
                "env::consts::ARCH",
            ] {
                assert!(
                    !text.contains(pat),
                    "OS introspection leak ({pat}) in {}",
                    p.display()
                );
            }
        }

        if !allows_command_new(&p) && !ci_layer && text.contains("Command::new") {
            panic!(
                "Command::new outside allowed files (capture / execution_boundary): {}",
                p.display()
            );
        }
    }
}
