//! Evidence Layer v2 — encoding surface (stub).

use super::chain::EvidenceChainV2;
use super::record::EvidenceRecordV2;

pub struct EvidenceEncoderV2;

impl EvidenceEncoderV2 {
    pub fn encode_record(&self, _record: &EvidenceRecordV2) -> Result<String, String> {
        unimplemented!()
    }

    pub fn encode_chain(&self, _chain: &EvidenceChainV2) -> Result<String, String> {
        unimplemented!()
    }
}
