//! SDK error type with contextual variants.

use aion_core::error::{
    aion_error_from_line, code, error_to_json, is_packed_line, line, sanitize_cause, AionError,
};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SdkError {
    Io(String),
    Parse(String),
    Validation(String),
    VersionMismatch { expected: String, found: String },
    Other(String),
}

fn sdk_error_to_aion(e: &SdkError) -> AionError {
    match e {
        SdkError::Parse(s) if is_packed_line(s) => aion_error_from_line(s).unwrap_or_else(|| {
            aion_error_from_line(&line(code::SDK_PARSE, "sdk", &sanitize_cause(s))).expect("line")
        }),
        SdkError::Parse(s) => {
            aion_error_from_line(&line(code::SDK_PARSE, "sdk", &sanitize_cause(s))).expect("line")
        }
        SdkError::Io(s) => {
            aion_error_from_line(&line(code::SDK_IO, "sdk", &sanitize_cause(s))).expect("line")
        }
        SdkError::Validation(s) => {
            aion_error_from_line(&line(code::SDK_VALIDATION, "sdk", &sanitize_cause(s)))
                .expect("line")
        }
        SdkError::VersionMismatch { expected, found } => aion_error_from_line(&line(
            code::SDK_VERSION,
            "sdk",
            &format!(
                "expected:{}:found:{}",
                sanitize_cause(expected),
                sanitize_cause(found)
            ),
        ))
        .expect("line"),
        SdkError::Other(s) => {
            aion_error_from_line(&line(code::SDK_OTHER, "sdk", &sanitize_cause(s))).expect("line")
        }
    }
}

impl Display for SdkError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ae = sdk_error_to_aion(self);
        write!(
            f,
            "{}",
            error_to_json(&ae).map_err(|_| std::fmt::Error)?
        )
    }
}

impl std::error::Error for SdkError {}
