use crate::kernel::system_truth::{SystemTruth, SystemTruthSnapshot};
use anyhow::Result;

pub struct KernelTransaction<'a> {
    snapshot: Option<SystemTruthSnapshot>,
    truth: &'a mut SystemTruth,
    dirty: bool,
}

impl<'a> KernelTransaction<'a> {
    pub fn begin(truth: &'a mut SystemTruth) -> Self {
        Self {
            snapshot: Some(SystemTruthSnapshot::capture(truth)),
            truth,
            dirty: false,
        }
    }

    pub fn truth_mut(&mut self) -> &mut SystemTruth {
        self.truth
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn commit(mut self) -> Result<()> {
        let snap = self.snapshot.take();
        if self.dirty {
            if let Err(e) = self.truth.persist() {
                if let Some(s) = snap {
                    s.restore_into(self.truth);
                }
                return Err(e);
            }
        }
        Ok(())
    }

    pub fn rollback(mut self) {
        if let Some(snap) = self.snapshot.take() {
            snap.restore_into(self.truth);
        }
    }
}

impl Drop for KernelTransaction<'_> {
    fn drop(&mut self) {
        if let Some(snap) = self.snapshot.take() {
            snap.restore_into(self.truth);
        }
    }
}
