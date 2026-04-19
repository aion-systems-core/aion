//! Deterministic replay — engine surface (stub).

use super::context::ReplayContext;

pub struct ReplayEngine;

impl ReplayEngine {
    pub fn new() -> Self {
        ReplayEngine
    }

    pub fn load_chain(&self, _path: &str) -> Result<ReplayContext, String> {
        unimplemented!()
    }

    pub fn replay(&self, _context: &ReplayContext) -> Result<(), String> {
        unimplemented!()
    }
}
