use super::record::AuditRecord;

pub struct AuditValidator;

impl AuditValidator {
    pub fn new() -> Self {
        AuditValidator
    }

    pub fn validate(&self, _record: &AuditRecord) -> Result<(), String> {
        unimplemented!()
    }
}
