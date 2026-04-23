//! Industrial CLI text helpers (deterministic unless `AION_COLOR=1`).

/// Dim ANSI (grey). Off by default for byte-stable logs.
pub fn dim(s: &str) -> String {
    if std::env::var("AION_COLOR").ok().as_deref() == Some("1") {
        format!("\x1b[90m{s}\x1b[0m")
    } else {
        s.to_string()
    }
}

pub fn err_prefix() -> &'static str {
    "ERR"
}

pub fn format_user_error(context: &str, detail: &str) -> String {
    format!("{}: {}", context, detail.replace('\n', " "))
}
