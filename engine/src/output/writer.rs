//! Typed writer for product artefacts under [`super::layout::OutputPath`].

use super::layout::{self, finalize_output_bundle, OutputPath};
use aion_core::error::{code, line};
use aion_core::{Capsule, EvidenceChain};
use serde::Serialize;
use std::path::PathBuf;

pub struct OutputWriter {
    path: OutputPath,
}

impl OutputWriter {
    pub fn new(command: &str) -> Result<Self, String> {
        Ok(Self {
            path: OutputPath::new(command)?,
        })
    }

    pub fn root(&self) -> &std::path::Path {
        &self.path.root
    }

    pub fn into_root(self) -> PathBuf {
        let _ = finalize_output_bundle(&self.path.root);
        self.path.root
    }

    pub fn write_json(&self, name: &str, value: &impl Serialize) -> Result<PathBuf, String> {
        layout::write_json(&self.path.root, name, value)
    }

    pub fn write_html(&self, name: &str, body: &str) -> Result<PathBuf, String> {
        layout::write_html(&self.path.root, name, body)
    }

    pub fn write_svg(&self, name: &str, body: &str) -> Result<PathBuf, String> {
        layout::write_svg(&self.path.root, name, body)
    }

    pub fn write_capsule(&self, name: &str, capsule: &Capsule) -> Result<PathBuf, String> {
        let body = layout::canonical_json_from_serialize(capsule).map_err(|_| {
            line(
                code::OUTPUT_JSON_SERIALIZE,
                "write_capsule",
                "invalid_json",
            )
        })?;
        let stem = name.trim_end_matches(".aion");
        layout::write_capsule(&self.path.root, stem, &body)
    }

    pub fn write_evidence(&self, name: &str, chain: &EvidenceChain) -> Result<PathBuf, String> {
        let body = layout::canonical_json_from_serialize(chain).map_err(|_| {
            line(
                code::OUTPUT_JSON_SERIALIZE,
                "write_evidence",
                "invalid_json",
            )
        })?;
        let stem = name.trim_end_matches(".aionevidence");
        layout::write_evidence(&self.path.root, stem, &body)
    }

    /// Pretty JSON for [`crate::ai::capsule::AICapsuleV1`] (`*.aionai`).
    pub fn write_aionai(&self, name: &str, body: &str) -> Result<PathBuf, String> {
        let stem = name.trim_end_matches(".aionai");
        let path = self.path.root.join(format!("{stem}.aionai"));
        if path.exists() {
            return Err(line(
                code::OUTPUT_AIONAI_EXISTS,
                "write_aionai",
                "exists",
            ));
        }
        layout::write_aionai(&self.path.root, stem, body)
    }
}
