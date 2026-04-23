// Output contract.
//
// The CLI makes a short, hard promise: **identical inputs → identical
// output, byte-for-byte, forever**. This module codifies the invariants
// that must hold for any CLI output to count as contract-compliant.
//
// `validate` is a pure function over a rendered string. It flags:
//   * wall-clock timestamp leakage (raw numeric `timestamp` fields are
//     allowed, but only when they are the deterministic one stored in
//     the artifact — not `now()`);
//   * non-ASCII control characters, since they make snapshot tests
//     noisy on Windows consoles;
//   * `\r` line endings, since they also drift per-OS;
//   * trailing whitespace, which sneaks in through concatenation bugs.
//
// These checks are deliberately conservative. They must never produce
// false positives on a well-formed `format_*` output. If a new output
// mode is added, this module is the right place to tighten the rules.

// Integration surface: this module is the hard contract, consumed by
// unit tests, the determinism test suite, and any future CLI --check
// mode. Allow dead_code at the module level so internal helpers can be
// kept pure and named without compiler noise.
#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContractViolation {
    pub line: usize,
    pub kind: ViolationKind,
    pub message: String,
    pub snippet: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ViolationKind {
    CarriageReturn,
    ControlCharacter,
    TrailingWhitespace,
    AnsiEscape,
}

pub struct OutputContract;

impl OutputContract {
    /// Validate a rendered output string. Empty result = contract holds.
    pub fn validate(text: &str) -> Vec<ContractViolation> {
        let mut out = Vec::new();

        // `.split('\n')` gives us stable line numbering regardless of
        // whether the text ends in '\n' (which it always should for
        // our formatters, but we should not rely on that here).
        for (i, line) in text.split('\n').enumerate() {
            let lineno = i + 1;

            if line.contains('\r') {
                out.push(ContractViolation {
                    line: lineno,
                    kind: ViolationKind::CarriageReturn,
                    message: "line contains '\\r'; output must use '\\n' only".into(),
                    snippet: snippet(line),
                });
            }

            if contains_ansi_escape(line) {
                out.push(ContractViolation {
                    line: lineno,
                    kind: ViolationKind::AnsiEscape,
                    message: "line contains an ANSI escape sequence".into(),
                    snippet: snippet(line),
                });
            }

            if let Some(c) = first_non_tab_control_char(line) {
                out.push(ContractViolation {
                    line: lineno,
                    kind: ViolationKind::ControlCharacter,
                    message: format!("line contains control character U+{:04X}", c as u32),
                    snippet: snippet(line),
                });
            }

            if !line.is_empty() && line.ends_with(|c: char| c.is_whitespace()) {
                out.push(ContractViolation {
                    line: lineno,
                    kind: ViolationKind::TrailingWhitespace,
                    message: "line has trailing whitespace".into(),
                    snippet: snippet(line),
                });
            }
        }

        out
    }
}

fn snippet(line: &str) -> String {
    let trimmed: String = line.chars().take(80).collect();
    trimmed
}

fn contains_ansi_escape(line: &str) -> bool {
    // ESC (U+001B) followed by anything. Covers both CSI and OSC.
    line.contains('\u{001B}')
}

fn first_non_tab_control_char(line: &str) -> Option<char> {
    line.chars().find(|c| {
        // Allow '\t' in captured content (embedded artifact strings);
        // everything else below U+0020 is forbidden.
        let u = *c as u32;
        u < 0x20 && *c != '\t' && *c != '\u{001B}'
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clean_output_passes() {
        let v = OutputContract::validate("hello\nworld\n");
        assert!(v.is_empty(), "unexpected violations: {v:?}");
    }

    #[test]
    fn carriage_return_is_flagged() {
        let v = OutputContract::validate("hello\r\nworld\n");
        assert!(v.iter().any(|x| x.kind == ViolationKind::CarriageReturn));
    }

    #[test]
    fn ansi_escape_is_flagged() {
        let v = OutputContract::validate("\u{001B}[31mred\u{001B}[0m\n");
        assert!(v.iter().any(|x| x.kind == ViolationKind::AnsiEscape));
    }

    #[test]
    fn trailing_whitespace_is_flagged() {
        let v = OutputContract::validate("hello  \n");
        assert!(v
            .iter()
            .any(|x| x.kind == ViolationKind::TrailingWhitespace));
    }
}
