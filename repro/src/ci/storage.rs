// Local CI ledger under `./repro_ci_store/`. Append-only `INDEX.jsonl`.

use crate::ci::meta::CiExecutionContext;
use crate::ci::schema::CI_STORE_LAYOUT_VERSION;
use crate::core::artifact::ExecutionArtifact;
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

pub const CI_STORE_DIR: &str = "repro_ci_store";
const INDEX_FILE: &str = "INDEX.jsonl";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IndexEntry {
    pub layout_version: u32,
    pub run_id: String,
}

pub fn store_root() -> PathBuf {
    PathBuf::from(CI_STORE_DIR)
}

fn index_path(root: &Path) -> PathBuf {
    root.join(INDEX_FILE)
}

fn run_dir(root: &Path, run_id: &str) -> PathBuf {
    root.join(run_id)
}

/// Persist artifact + sidecars and append index row. `meta` is never read by diff engines.
pub fn save_ci_run(
    root: &Path,
    artifact: &ExecutionArtifact,
    meta: &CiExecutionContext,
) -> io::Result<PathBuf> {
    fs::create_dir_all(root)?;
    let dir = run_dir(root, &artifact.run_id);
    fs::create_dir_all(&dir)?;

    let json = serde_json::to_string_pretty(artifact)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    write_atomic(&dir.join("artifact.json"), json.as_bytes())?;

    write_atomic(&dir.join("stdout.txt"), artifact.stdout.as_bytes())?;
    write_atomic(&dir.join("stderr.txt"), artifact.stderr.as_bytes())?;

    let env_json = serde_json::to_string_pretty(&artifact.env_snapshot)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    write_atomic(&dir.join("env.json"), env_json.as_bytes())?;

    let meta_json = serde_json::to_string_pretty(meta)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    write_atomic(&dir.join("meta.json"), meta_json.as_bytes())?;

    append_index(root, &artifact.run_id)?;
    Ok(dir)
}

fn append_index(root: &Path, run_id: &str) -> io::Result<()> {
    let idx = index_path(root);
    let entry = IndexEntry {
        layout_version: CI_STORE_LAYOUT_VERSION,
        run_id: run_id.to_string(),
    };
    let line =
        serde_json::to_string(&entry).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let mut f = OpenOptions::new().create(true).append(true).open(idx)?;
    writeln!(f, "{line}")
}

fn write_atomic(path: &Path, bytes: &[u8]) -> io::Result<()> {
    let tmp = path.with_extension("tmp");
    let mut f = fs::File::create(&tmp)?;
    f.write_all(bytes)?;
    f.sync_all()?;
    drop(f);
    fs::rename(&tmp, path)
}

pub fn list_runs(root: &Path) -> io::Result<Vec<String>> {
    let p = index_path(root);
    if !p.exists() {
        return Ok(Vec::new());
    }
    let mut s = String::new();
    fs::File::open(&p)?.read_to_string(&mut s)?;
    let mut out = Vec::new();
    for line in s.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let e: IndexEntry = serde_json::from_str(line)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        out.push(e.run_id);
    }
    Ok(out)
}

/// Load all CI ledger artifacts in `INDEX.jsonl` append order (oldest first).
#[must_use]
pub fn list_ci_runs_in(root: &Path) -> Vec<ExecutionArtifact> {
    match list_runs(root) {
        Ok(ids) => ids
            .into_iter()
            .filter_map(|id| load_artifact(root, &id).ok())
            .collect(),
        Err(_) => Vec::new(),
    }
}

/// Same as [`list_ci_runs_in`] for the default store root ([`store_root`]).
#[must_use]
pub fn list_ci_runs() -> Vec<ExecutionArtifact> {
    list_ci_runs_in(&store_root())
}

pub fn resolve_alias(root: &Path, alias: &str) -> io::Result<String> {
    let runs = list_runs(root)?;
    match alias {
        "last" => runs
            .last()
            .cloned()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "no CI runs in index")),
        "prev" => {
            if runs.len() < 2 {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "no previous CI run (need at least 2)",
                ));
            }
            Ok(runs[runs.len() - 2].clone())
        }
        other => Ok(other.to_string()),
    }
}

pub fn load_artifact(root: &Path, run_id: &str) -> io::Result<ExecutionArtifact> {
    let p = run_dir(root, run_id).join("artifact.json");
    let mut s = String::new();
    fs::File::open(&p)?.read_to_string(&mut s)?;
    serde_json::from_str(&s).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

pub fn load_stdout(root: &Path, run_id: &str) -> io::Result<String> {
    let p = run_dir(root, run_id).join("stdout.txt");
    let mut s = String::new();
    fs::File::open(&p)?.read_to_string(&mut s)?;
    Ok(s)
}

pub fn load_stderr(root: &Path, run_id: &str) -> io::Result<String> {
    let p = run_dir(root, run_id).join("stderr.txt");
    let mut s = String::new();
    fs::File::open(&p)?.read_to_string(&mut s)?;
    Ok(s)
}
