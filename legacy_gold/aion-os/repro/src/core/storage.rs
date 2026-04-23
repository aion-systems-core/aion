// Local-only run storage.
//
// Layout:
//   ./repro_runs/<run_id>.json         — one artifact per file, pretty JSON
//   ./repro_runs/<run_id>.events.json  — canonical event stream (Phase 8.2)
//   ./repro_runs/INDEX                 — ordered list of run_ids, oldest first

use crate::core::artifact::ExecutionArtifact;
use std::fs::{self, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

pub const RUNS_DIR: &str = "repro_runs";
/// Optional CI mirror of the same `ExecutionArtifact` JSON (no INDEX append).
pub const CI_RUNS_SUBDIR: &str = "ci";
const INDEX_FILE: &str = "INDEX";

pub fn runs_dir() -> PathBuf {
    PathBuf::from(RUNS_DIR)
}

fn index_path(dir: &Path) -> PathBuf {
    dir.join(INDEX_FILE)
}

fn artifact_path(dir: &Path, run_id: &str) -> PathBuf {
    dir.join(format!("{run_id}.json"))
}

pub fn save_run(run: &ExecutionArtifact) -> io::Result<PathBuf> {
    save_run_in(&runs_dir(), run)
}

/// Write `repro_runs/ci/<run_id>.json` (same schema as `save_run`, no index update).
pub fn save_ci_run_mirror(run: &ExecutionArtifact) -> io::Result<PathBuf> {
    let dir = runs_dir().join(CI_RUNS_SUBDIR);
    fs::create_dir_all(&dir)?;
    let path = dir.join(format!("{}.json", run.run_id));
    let json = serde_json::to_string_pretty(run)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    write_atomically(&path, json.as_bytes())?;
    crate::core::event_store::save_event_stream_in(&dir, &run.run_id, &run.trace)?;
    Ok(path)
}

pub fn load_run(run_id: &str) -> io::Result<ExecutionArtifact> {
    load_run_in(&runs_dir(), run_id)
}

pub fn list_runs() -> io::Result<Vec<String>> {
    list_runs_in(&runs_dir())
}

pub fn resolve_alias(alias: &str) -> io::Result<String> {
    resolve_alias_in(&runs_dir(), alias)
}

pub fn save_run_in(dir: &Path, run: &ExecutionArtifact) -> io::Result<PathBuf> {
    fs::create_dir_all(dir)?;

    let json = serde_json::to_string_pretty(run)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let path = artifact_path(dir, &run.run_id);
    write_atomically(&path, json.as_bytes())?;

    crate::core::event_store::save_event_stream_in(dir, &run.run_id, &run.trace)?;

    append_index(dir, &run.run_id)?;
    Ok(path)
}

pub fn load_run_in(dir: &Path, run_id: &str) -> io::Result<ExecutionArtifact> {
    let path = artifact_path(dir, run_id);
    let mut s = String::new();
    fs::File::open(&path)?.read_to_string(&mut s)?;
    serde_json::from_str(&s).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

pub fn list_runs_in(dir: &Path) -> io::Result<Vec<String>> {
    let idx = index_path(dir);
    if !idx.exists() {
        return Ok(Vec::new());
    }
    let mut s = String::new();
    fs::File::open(&idx)?.read_to_string(&mut s)?;
    Ok(s.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|l| l.to_string())
        .collect())
}

pub fn resolve_alias_in(dir: &Path, alias: &str) -> io::Result<String> {
    match alias {
        "last" => {
            let runs = list_runs_in(dir)?;
            runs.last()
                .cloned()
                .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "no runs recorded"))
        }
        "prev" => {
            let runs = list_runs_in(dir)?;
            if runs.len() < 2 {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "no previous run (need at least 2)",
                ));
            }
            Ok(runs[runs.len() - 2].clone())
        }
        literal => Ok(literal.to_string()),
    }
}

fn append_index(dir: &Path, run_id: &str) -> io::Result<()> {
    let existing = list_runs_in(dir)?;
    if existing.last().map(|s| s.as_str()) == Some(run_id) {
        return Ok(());
    }
    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .open(index_path(dir))?;
    writeln!(f, "{run_id}")
}

fn write_atomically(path: &Path, bytes: &[u8]) -> io::Result<()> {
    let tmp = path.with_extension("json.tmp");
    {
        let mut f = fs::File::create(&tmp)?;
        f.write_all(bytes)?;
        f.sync_all()?;
    }
    fs::rename(&tmp, path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::capture::{
        capture_command_with_clock, reset_counter_for_tests, FixedClock, CAPTURE_TEST_LOCK,
    };

    fn tmpdir(tag: &str) -> PathBuf {
        let base = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("repro-storage-tests");
        let _ = fs::create_dir_all(&base);
        base.join(format!(
            "{}-{}-{}",
            tag,
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ))
    }

    #[test]
    fn roundtrip_save_and_load() {
        let _g = CAPTURE_TEST_LOCK.lock().unwrap();
        let dir = tmpdir("roundtrip");
        reset_counter_for_tests();
        let run = capture_command_with_clock("echo hi".to_string(), &FixedClock(1));
        save_run_in(&dir, &run).unwrap();
        let loaded = load_run_in(&dir, &run.run_id).unwrap();
        assert_eq!(run, loaded);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn index_is_ordered_and_resolves_aliases() {
        let _g = CAPTURE_TEST_LOCK.lock().unwrap();
        let dir = tmpdir("index");
        reset_counter_for_tests();
        let a = capture_command_with_clock("echo a".to_string(), &FixedClock(1));
        let b = capture_command_with_clock("echo b".to_string(), &FixedClock(2));
        save_run_in(&dir, &a).unwrap();
        save_run_in(&dir, &b).unwrap();

        assert_eq!(
            list_runs_in(&dir).unwrap(),
            vec![a.run_id.clone(), b.run_id.clone()]
        );
        assert_eq!(resolve_alias_in(&dir, "last").unwrap(), b.run_id);
        assert_eq!(resolve_alias_in(&dir, "prev").unwrap(), a.run_id);
        assert_eq!(resolve_alias_in(&dir, "literal").unwrap(), "literal");
        let _ = fs::remove_dir_all(&dir);
    }

    struct CdGuard {
        back: PathBuf,
    }

    impl Drop for CdGuard {
        fn drop(&mut self) {
            let _ = std::env::set_current_dir(&self.back);
        }
    }

    #[test]
    fn ci_mirror_writes_event_stream() {
        let _g = CAPTURE_TEST_LOCK.lock().unwrap();
        let back = std::env::current_dir().unwrap();
        let tmp = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("ci-mirror-events-wd");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp).unwrap();
        std::env::set_current_dir(&tmp).unwrap();
        let _cd = CdGuard { back: back.clone() };

        reset_counter_for_tests();
        let run = capture_command_with_clock("echo z".to_string(), &FixedClock(42));
        save_ci_run_mirror(&run).unwrap();

        let ev = tmp
            .join(RUNS_DIR)
            .join(CI_RUNS_SUBDIR)
            .join(format!("{}.events.json", run.run_id));
        assert!(ev.is_file(), "expected event stream at {:?}", ev);

        drop(_cd);
        let _ = fs::remove_dir_all(&tmp);
    }
}
