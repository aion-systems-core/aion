//! Inject repo-root `VERSION` and rustc metadata for `aion version --full`.

use std::process::Command;

fn main() {
    let manifest_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let version_path = manifest_dir.join("../../VERSION");
    let ver = std::fs::read_to_string(&version_path)
        .unwrap_or_else(|e| panic!("read {}: {e}", version_path.display()))
        .trim()
        .to_string();
    if ver.is_empty() {
        panic!("VERSION file is empty");
    }
    println!("cargo:rustc-env=AION_SEMVER={ver}");
    println!("cargo:rerun-if-changed={}", version_path.display());

    let rustc = std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
    let rv = Command::new(rustc)
        .arg("--version")
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".into());
    println!("cargo:rustc-env=AION_RUSTC_VERSION={rv}");

    let mut h: u64 = 5381;
    for b in format!("{ver}|{rv}").bytes() {
        h = h.wrapping_mul(33).wrapping_add(b as u64);
    }
    println!("cargo:rustc-env=AION_BUILD_HASH={:016x}", h);
}
