//! Product output: timestamped directories, HTML/SVG/JSON artefacts.

pub mod governance_render;
pub mod html;
pub mod layout;
pub mod replay_ai;
pub mod svg;
pub mod writer;

pub use layout::{finalize_output_bundle, output_base_dir, OutputPath, OutputRunMeta};
pub use writer::OutputWriter;
