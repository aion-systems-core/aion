//! AI capsule load / save / build (deterministic, no CLI).

use crate::ai::{ai_capsule_to_json, build_ai_capsule_v1, read_ai_capsule_v1, AICapsuleV1};
use crate::sdk::error::SdkError;
use aion_core::error::{code, io_cause, line};
use std::fs;
use std::path::Path;

/// Load a `.aionai` / JSON capsule from disk.
pub fn load_capsule(path: &Path) -> Result<AICapsuleV1, String> {
    read_ai_capsule_v1(path)
}

pub fn load_capsule_checked(path: &Path) -> Result<AICapsuleV1, SdkError> {
    let c = read_ai_capsule_v1(path).map_err(SdkError::Parse)?;
    if c.version != "1" {
        return Err(SdkError::VersionMismatch {
            expected: "1".into(),
            found: c.version,
        });
    }
    Ok(c)
}

/// Persist a capsule as pretty JSON (creates parent directories).
pub fn save_capsule(path: &Path, capsule: &AICapsuleV1) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| line(code::CAPSULE_SAVE_MKDIR, "save_capsule", &io_cause(&e)))?;
    }
    if path.exists() {
        return Err(line(code::CAPSULE_SAVE_EXISTS, "save_capsule", "exists"));
    }
    let body = ai_capsule_to_json(capsule)?;
    fs::write(path, body).map_err(|e| line(code::CAPSULE_SAVE_IO, "save_capsule", &io_cause(&e)))
}

/// Deterministic capsule construction from model / prompt / seed.
pub fn build_capsule(model: &str, prompt: &str, seed: u64) -> AICapsuleV1 {
    build_ai_capsule_v1(model.into(), prompt.into(), seed)
}
