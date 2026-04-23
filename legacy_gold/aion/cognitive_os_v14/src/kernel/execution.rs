use crate::engine::Engine;
use crate::input::Workflow;
use crate::kernel::enforcement::{EnforcementEngine, EnforcementMode};
use crate::kernel::envelope::ExecutionEnvelope;
use crate::kernel::evidence_chain::EvidenceChain;
use crate::kernel::snapshot::{snapshots_directory, KernelSnapshot};
use crate::kernel::system_truth::SystemTruth;
use crate::kernel::transaction::KernelTransaction;
use crate::output::Report;
use crate::process::id::ProcessId;
use crate::process::model::ProcessModel;
use crate::process::ProcessManager;
use crate::process::state::ProcessState;
use crate::evidence_engine::EvidenceTimelineRow;
use crate::map_engine::MapBuilder;
use crate::state::State;
use anyhow::Result;
use std::fs;
use std::path::PathBuf;

pub struct ExecutionResult {
    pub report: Report,
}

pub struct KernelExecutionEngine;

impl KernelExecutionEngine {
    pub fn start_process(
        truth: &mut SystemTruth,
        spec_path: std::path::PathBuf,
        envelope: ExecutionEnvelope,
    ) -> Result<ProcessId> {
        let _ = envelope;
        if !spec_path.is_file() {
            anyhow::bail!("spec not found: {}", spec_path.display());
        }
        let wf_path = spec_path.clone();
        let model = ProcessModel::new(spec_path);
        let process_id = model.meta.id.clone();
        let pid_str = process_id.to_string();

        EnforcementEngine::check_or_block(truth, &model, EnforcementMode::HardwareOnly, None)?;

        let mut txn = KernelTransaction::begin(truth);
        {
            let truth = txn.truth_mut();
            truth
                .process_writes
                .insert(process_id.clone(), model.clone());
            truth.evidence.add_process(pid_str.clone());

            EvidenceChain::append_to_truth(truth, &pid_str, EvidenceTimelineRow::new(&pid_str, "created", None, None))?;
            {
                let m = truth
                    .process_writes
                    .get_mut(&process_id)
                    .expect("process just inserted");
                m.add_event("created".to_string());
                m.set_state(ProcessState::Running);
            }
            EvidenceChain::append_to_truth(truth, &pid_str, EvidenceTimelineRow::new(&pid_str, "running", None, None))?;

            let wf_path_inner = truth
                .process_writes
                .get(&process_id)
                .expect("process")
                .meta
                .spec_path
                .clone();
            let workflow = Workflow::from_file(&wf_path_inner)?;
            let mut engine = Engine::new(State::default(), &truth.config.checkpoint_path)?;
            let run_result = engine.execute_workflow(&workflow);

            truth.sync_checkpoint_from_disk()?;

            EvidenceChain::append_to_truth(
                truth,
                &pid_str,
                EvidenceTimelineRow::new(
                    &pid_str,
                    "workflow_executed",
                    Some(serde_json::json!({
                        "spec": wf_path.display().to_string(),
                    })),
                    Some(serde_json::json!({
                        "ok": run_result.is_ok(),
                    })),
                ),
            )?;

            match &run_result {
                Ok(_) => {
                    let m = truth
                        .process_writes
                        .get_mut(&process_id)
                        .expect("process");
                    m.set_state(ProcessState::Completed);
                    m.add_event("workflow_completed".to_string());
                    EvidenceChain::append_to_truth(truth, &pid_str, EvidenceTimelineRow::new(&pid_str, "completed", None, None))?;
                }
                Err(e) => {
                    let m = truth
                        .process_writes
                        .get_mut(&process_id)
                        .expect("process");
                    m.set_state(ProcessState::Failed);
                    m.add_event(format!("workflow_failed:{e}"));
                    EvidenceChain::append_to_truth(
                        truth,
                        &pid_str,
                        EvidenceTimelineRow::new(
                            &pid_str,
                            "failed",
                            None,
                            Some(serde_json::json!({ "error": e.to_string() })),
                        ),
                    )?;
                }
            }

            truth.map = MapBuilder::build_with_pending_models(&truth.process_writes);
        }
        txn.mark_dirty();
        txn.commit()?;
        Ok(process_id)
    }

    pub fn execute_process(
        truth: &mut SystemTruth,
        process_id: ProcessId,
        workflow: Workflow,
        envelope: ExecutionEnvelope,
    ) -> Result<ExecutionResult> {
        let _ = envelope;
        let model = ProcessManager::show_process(process_id.clone())?;
        let map_for_policy = MapBuilder::build_with_pending_models(&truth.process_writes);
        EnforcementEngine::check_or_block(truth, &model, EnforcementMode::Standard, Some(&map_for_policy))?;

        let mut txn = KernelTransaction::begin(truth);
        let report = {
            let truth = txn.truth_mut();
            truth.map = map_for_policy;
            let mut engine = Engine::new(State::default(), &truth.config.checkpoint_path)?;
            let report = engine.execute_workflow(&workflow)?;
            truth.sync_checkpoint_from_disk()?;
            EvidenceChain::append_to_truth(
                truth,
                process_id.as_str(),
                EvidenceTimelineRow::new(
                    process_id.as_str(),
                    "workflow_executed",
                    Some(serde_json::json!({
                        "workflow": workflow.name.clone(),
                    })),
                    Some(serde_json::json!({ "ok": true })),
                ),
            )?;
            truth.map = MapBuilder::build_with_pending_models(&truth.process_writes);
            report
        };
        txn.mark_dirty();
        txn.commit()?;
        Ok(ExecutionResult { report })
    }

    /// Builds a [`KernelSnapshot`], writes compact JSON under the configured snapshots directory, and returns it with the file path.
    /// Does not mutate `truth` or run transactions or the engine.
    pub fn take_snapshot(truth: &SystemTruth, envelope: &ExecutionEnvelope) -> Result<(KernelSnapshot, PathBuf)> {
        let snap = KernelSnapshot::from_truth(truth, envelope);
        let dir = snapshots_directory(&truth.config);
        fs::create_dir_all(&dir)?;
        let path = dir.join(format!("snapshot_{}.json", chrono::Utc::now().timestamp()));
        snap.save(&path)?;
        Ok((snap, path))
    }
}
