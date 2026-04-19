// `repro ci …` — ledger persistence only; execution uses `core::capture` (same as `repro run`).

use crate::ci::ci_orchestrator::{ci_run_json_value, format_ci_result_text, process_ci_run};
use crate::ci::ci_runtime::{attach_ci_metadata, detect_ci_context};
use crate::ci::diff::{compare_ci_runs, CIComparisonReport};
use crate::ci::meta::CiExecutionContext;
use crate::ci::root_cause::{explain_ci_root_cause, format_root_cause_human};
use crate::ci::storage::{
    list_runs, load_artifact, load_stderr, load_stdout, resolve_alias, save_ci_run, store_root,
};
use crate::core::artifact::ExecutionArtifact;
use crate::core::capture;
use crate::core::storage as core_storage;
use clap::Subcommand;
use std::fs;
use std::io::{self, Read};

#[derive(Subcommand, Debug)]
pub enum CiCommand {
    /// Capture a subprocess in the CI ledger (`./repro_ci_store/`).
    Run {
        #[arg(long)]
        json: bool,
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, num_args = 1..)]
        command: Vec<String>,
    },
    /// Read a run-record JSON from a file or `-` (stdin) and append to the ledger.
    #[command(hide = true)]
    Ingest {
        /// Path to JSON, or `-` for stdin.
        artifact: String,
        #[arg(long)]
        json: bool,
    },
    /// Compare two CI runs (ids or `last` / `prev`).
    #[command(hide = true)]
    Diff {
        run_a: String,
        run_b: String,
        #[arg(long)]
        json: bool,
    },
    /// Explain divergence vs. the previous indexed run (or vs. `prev` when `last`).
    #[command(hide = true)]
    Why {
        run_id: String,
        #[arg(long)]
        json: bool,
    },
    /// List all CI run ids (oldest first).
    #[command(hide = true)]
    List {
        #[arg(long)]
        json: bool,
    },
}

pub fn handle(cmd: CiCommand) -> Result<(), String> {
    let root = store_root();
    match cmd {
        CiCommand::Run { json, command } => ci_run(&root, json, command),
        CiCommand::Ingest { artifact, json } => ci_ingest(&root, json, &artifact),
        CiCommand::Diff { run_a, run_b, json } => ci_diff(&root, json, &run_a, &run_b),
        CiCommand::Why { run_id, json } => ci_why(&root, json, &run_id),
        CiCommand::List { json } => ci_list(&root, json),
    }
}

fn join_argv(argv: &[String]) -> String {
    argv.iter()
        .map(|t| {
            if t.is_empty() || t.chars().any(|c| c.is_whitespace() || c == '"') {
                format!("\"{}\"", t.replace('"', "\\\""))
            } else {
                t.clone()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn ci_run(root: &std::path::Path, json: bool, command: Vec<String>) -> Result<(), String> {
    if command.is_empty() {
        return Err("ci run: missing command (usage: repro ci run -- <command>)".into());
    }
    let joined = join_argv(&command);
    let program = command[0].as_str();
    let args: Vec<String> = command[1..].to_vec();
    let ctx = detect_ci_context();
    let mut meta = CiExecutionContext::local_default();
    attach_ci_metadata(&mut meta, &ctx);

    let artifact = capture::capture_command_real(program, &args, &joined);
    let ci_result = process_ci_run(root, &artifact);

    save_ci_run(root, &artifact, &meta).map_err(|e| format!("ci run: save failed: {e}"))?;

    if ctx.ci {
        core_storage::save_ci_run_mirror(&artifact)
            .map_err(|e| format!("ci run: ci mirror: {e}"))?;
    }

    if json {
        let summary = ci_run_json_value(&ci_result);
        println!(
            "{}",
            serde_json::to_string_pretty(&summary).map_err(|e| e.to_string())?
        );
    } else {
        print!("{}", format_ci_result_text(&ci_result));
    }
    Ok(())
}

fn ci_ingest(root: &std::path::Path, json: bool, path: &str) -> Result<(), String> {
    let body = if path == "-" {
        let mut buf = String::new();
        io::stdin()
            .read_to_string(&mut buf)
            .map_err(|e| format!("ci ingest: stdin: {e}"))?;
        buf
    } else {
        fs::read_to_string(path).map_err(|e| format!("ci ingest: read {path}: {e}"))?
    };
    let artifact: ExecutionArtifact =
        serde_json::from_str(&body).map_err(|e| format!("ci ingest: parse json: {e}"))?;

    save_ci_run(root, &artifact, &CiExecutionContext::ingest_default())
        .map_err(|e| format!("ci ingest: save: {e}"))?;

    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "ingested_run_id": artifact.run_id,
                "schema_version": artifact.schema_version,
            }))
            .map_err(|e| e.to_string())?
        );
    } else {
        println!("ingested run_id : {}", artifact.run_id);
        println!("schema          : v{}", artifact.schema_version);
    }
    Ok(())
}

fn ci_diff(root: &std::path::Path, json: bool, run_a: &str, run_b: &str) -> Result<(), String> {
    let id_a = resolve_alias(root, run_a).map_err(|e| format!("ci diff: {e}"))?;
    let id_b = resolve_alias(root, run_b).map_err(|e| format!("ci diff: {e}"))?;
    let mut a = load_artifact(root, &id_a).map_err(|e| format!("ci diff: load a: {e}"))?;
    let mut b = load_artifact(root, &id_b).map_err(|e| format!("ci diff: load b: {e}"))?;
    a.stdout = load_stdout(root, &id_a).map_err(|e| format!("ci diff: stdout a: {e}"))?;
    a.stderr = load_stderr(root, &id_a).map_err(|e| format!("ci diff: stderr a: {e}"))?;
    b.stdout = load_stdout(root, &id_b).map_err(|e| format!("ci diff: stdout b: {e}"))?;
    b.stderr = load_stderr(root, &id_b).map_err(|e| format!("ci diff: stderr b: {e}"))?;
    let report = compare_ci_runs(&a, &b);

    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?
        );
    } else {
        print!("{}", format_comparison_human(&report));
    }
    Ok(())
}

fn format_comparison_human(r: &CIComparisonReport) -> String {
    let mut s = String::new();
    s.push_str("── CI diff ──\n");
    s.push_str(&format!("a: {}\n", r.run_a));
    s.push_str(&format!("b: {}\n", r.run_b));
    s.push_str(&format!("environment: {}\n", r.env_diff.summary));
    s.push_str(&format!("command:     {}\n", r.command_diff.summary));
    s.push_str(&format!("exit_code:   {}\n", r.exit_code_diff.summary));
    s.push_str(&format!("stdout:      {}\n", r.stdout_diff.summary));
    s.push_str(&format!("stderr:      {}\n", r.stderr_diff.summary));
    s.push_str("semantic:\n");
    for c in &r.semantic_classification {
        s.push_str(&format!("  - {:?}\n", c));
    }
    s.push('\n');
    s
}

fn ci_why(root: &std::path::Path, json: bool, run_id_or_alias: &str) -> Result<(), String> {
    let run_id = resolve_alias(root, run_id_or_alias).map_err(|e| format!("ci why: {e}"))?;
    let runs = list_runs(root).map_err(|e| format!("ci why: {e}"))?;
    let idx = runs
        .iter()
        .position(|r| r == &run_id)
        .ok_or_else(|| "ci why: run not in index".to_string())?;
    if idx == 0 {
        return Err("ci why: no previous run in index".into());
    }
    let prev_id = runs[idx - 1].clone();
    let mut cur = load_artifact(root, &run_id).map_err(|e| format!("ci why: {e}"))?;
    let mut prev = load_artifact(root, &prev_id).map_err(|e| format!("ci why: {e}"))?;
    cur.stdout = load_stdout(root, &run_id).map_err(|e| format!("ci why: {e}"))?;
    cur.stderr = load_stderr(root, &run_id).map_err(|e| format!("ci why: {e}"))?;
    prev.stdout = load_stdout(root, &prev_id).map_err(|e| format!("ci why: {e}"))?;
    prev.stderr = load_stderr(root, &prev_id).map_err(|e| format!("ci why: {e}"))?;
    let report = explain_ci_root_cause(&prev, &cur);

    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?
        );
    } else {
        print!("{}", format_root_cause_human(&report));
        println!("compare: {prev_id} -> {run_id}");
    }
    Ok(())
}

fn ci_list(root: &std::path::Path, json: bool) -> Result<(), String> {
    let runs = list_runs(root).map_err(|e| format!("ci list: {e}"))?;
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&runs).map_err(|e| e.to_string())?
        );
    } else {
        println!("── CI runs (oldest first) ──");
        for id in &runs {
            println!("{id}");
        }
        println!("total: {}", runs.len());
    }
    Ok(())
}
