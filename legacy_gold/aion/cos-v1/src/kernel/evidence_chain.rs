use cos_core::evidence::record::EvidenceRecordV2;

/// Append-only evidence writer contract — **kernel proof rows** are always [`EvidenceRecordV2`].
pub trait EvidenceWriter {
    fn append(&mut self, record: EvidenceRecordV2) -> Result<(), String>;
}
