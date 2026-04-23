//! Expose rustc version string for [`RuntimeFingerprint`].

use std::process::Command;

fn main() {
    let ver = Command::new(std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into()))
        .arg("--version")
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                String::from_utf8(o.stdout).ok()
            } else {
                None
            }
        })
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".into());
    println!("cargo:rustc-env=AION_ENGINE_RUSTC_VERSION={ver}");
}
