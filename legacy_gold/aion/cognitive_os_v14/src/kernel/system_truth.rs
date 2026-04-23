use crate::checkpoint::CheckpointManager;
use crate::config::Config;
use crate::evidence_engine::{EvidenceIndex, EvidenceTimeline};
use crate::hardware::HardwareProfiles;
use crate::map_engine::{MapModel, MapSerializer};
use crate::policy::{self, PolicySet};
use crate::process::{ProcessId, ProcessModel, ProcessRegistry};
use crate::state::State;
use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;

fn map_json_path() -> PathBuf {
    PathBuf::from("map_state").join("map.json")
}

fn map_dot_path() -> PathBuf {
    PathBuf::from("map_state").join("map.dot")
}

#[derive(Clone)]
pub struct SystemTruthSnapshot {
    pub evidence: EvidenceIndex,
    pub map: MapModel,
    pub state: State,
    pub policies: PolicySet,
    pub hardware: HardwareProfiles,
    pub staged_timelines: HashMap<String, EvidenceTimeline>,
    pub process_writes: HashMap<ProcessId, ProcessModel>,
    pub dirty_timeline_ids: HashSet<String>,
}

impl SystemTruthSnapshot {
    pub fn capture(truth: &SystemTruth) -> Self {
        Self {
            evidence: truth.evidence.clone(),
            map: truth.map.clone(),
            state: truth.state.clone(),
            policies: truth.policies.clone(),
            hardware: truth.hardware.clone(),
            staged_timelines: truth.staged_timelines.clone(),
            process_writes: truth.process_writes.clone(),
            dirty_timeline_ids: truth.dirty_timeline_ids.clone(),
        }
    }

    pub fn restore_into(self, truth: &mut SystemTruth) {
        truth.evidence = self.evidence;
        truth.map = self.map;
        truth.state = self.state;
        truth.policies = self.policies;
        truth.hardware = self.hardware;
        truth.staged_timelines = self.staged_timelines;
        truth.process_writes = self.process_writes;
        truth.dirty_timeline_ids = self.dirty_timeline_ids;
    }
}

pub struct SystemTruth {
    pub processes: ProcessRegistry,
    pub evidence: EvidenceIndex,
    pub map: MapModel,
    pub policies: PolicySet,
    pub hardware: HardwareProfiles,
    pub state: State,
    pub config: Config,
    /// Process models staged for [`SystemTruth::persist`] (new processes and updates).
    pub(crate) process_writes: HashMap<ProcessId, ProcessModel>,
    pub(crate) staged_timelines: HashMap<String, EvidenceTimeline>,
    pub(crate) dirty_timeline_ids: HashSet<String>,
}

impl SystemTruth {
    pub fn load_from_disk(config: &Config) -> Result<Self> {
        let evidence = EvidenceIndex::load()?;
        let policies = policy::load();
        let hardware = HardwareProfiles::load()?;
        let map = if map_json_path().is_file() {
            let raw = fs::read_to_string(map_json_path())
                .with_context(|| format!("read {}", map_json_path().display()))?;
            serde_json::from_str(&raw).with_context(|| format!("parse {}", map_json_path().display()))?
        } else {
            MapModel::new()
        };
        let checkpoint = CheckpointManager::new(&config.checkpoint_path)?;
        let state = checkpoint.load()?.unwrap_or_default();
        Ok(Self {
            processes: ProcessRegistry,
            evidence,
            map,
            policies,
            hardware,
            state,
            config: config.clone(),
            process_writes: HashMap::new(),
            staged_timelines: HashMap::new(),
            dirty_timeline_ids: HashSet::new(),
        })
    }

    pub fn ensure_timeline(&mut self, process_id: &str) -> Result<&mut EvidenceTimeline> {
        if !self.staged_timelines.contains_key(process_id) {
            let t = EvidenceTimeline::load(process_id)?;
            self.staged_timelines.insert(process_id.to_string(), t);
        }
        Ok(self
            .staged_timelines
            .get_mut(process_id)
            .expect("inserted timeline"))
    }

    pub fn evidence_ready_for_policy(&self, process_id: &str) -> bool {
        let path = PathBuf::from("evidence").join(process_id).join("timeline.json");
        if path.is_file() {
            return true;
        }
        self.staged_timelines
            .get(process_id)
            .is_some_and(|t| !t.records.is_empty())
    }

    pub fn sync_checkpoint_from_disk(&mut self) -> Result<()> {
        let checkpoint = CheckpointManager::new(&self.config.checkpoint_path)?;
        if let Some(s) = checkpoint.load()? {
            self.state = s;
        }
        Ok(())
    }

    pub fn persist(&mut self) -> Result<()> {
        for model in self.process_writes.values() {
            ProcessRegistry::save(model)?;
        }
        self.evidence.save()?;
        for pid in &self.dirty_timeline_ids.clone() {
            if let Some(tl) = self.staged_timelines.get(pid) {
                tl.save()?;
            }
        }
        policy::save(&self.policies)?;
        self.hardware.save()?;
        let dir = PathBuf::from("map_state");
        fs::create_dir_all(&dir).with_context(|| format!("create_dir_all {}", dir.display()))?;
        let json_path = map_json_path();
        let v = MapSerializer::to_json(&self.map);
        fs::write(
            &json_path,
            serde_json::to_string_pretty(&v).with_context(|| format!("serialize {}", json_path.display()))?,
        )
        .with_context(|| format!("write {}", json_path.display()))?;
        let dot_path = map_dot_path();
        fs::write(&dot_path, MapSerializer::to_dot(&self.map))
            .with_context(|| format!("write {}", dot_path.display()))?;
        let checkpoint = CheckpointManager::new(&self.config.checkpoint_path)?;
        checkpoint.save(&self.state)?;
        self.process_writes.clear();
        self.staged_timelines.clear();
        self.dirty_timeline_ids.clear();
        Ok(())
    }
}
