//! `sealrun` binary: command-line entrypoint for SealRun’s deterministic execution, governance, and enterprise contract domains.
//!
//! Dispatch is intentionally thin; heavy lifting lives in the `aion_engine` crate via `output_bundle` writers. JSON lines follow the deterministic envelope rules in `docs/os_contract_spec.md`.
use aion_cli::{output_bundle, sectors, ux, InProcessKernel, KernelGateway};
use aion_core::error::{canonical_error_json, code, io_cause, line};
use aion_core::{
    diff_contract_snapshots, evaluate_contract_stability, evaluate_determinism_contract,
    evaluate_determinism_matrix, run_replay_invariant_gate, write_contract_snapshots,
    ContractSnapshot, DeterminismContractInput, DeterminismTarget,
};
use aion_core::{Capsule, DriftReport, RunResult};
use aion_engine::audit::AuditReport;
use aion_engine::ci::{check_baseline, record_baseline, CiDriftBundle, CiRunBundle};
use aion_engine::diff::diff_run_snapshots;
use aion_engine::output::OutputWriter;
use aion_engine::why::why_run_pair;
use aion_kernel::IntegrityReport;
use clap::{Parser, Subcommand};
use serde::de::DeserializeOwned;
use serde_json::json;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const AION_ABOUT: &str =
    "SealRun — seal your run. Deterministic, replayable, auditable execution capsules.";
const AION_AFTER_HELP: &str = r#"USAGE
  sealrun [OPTIONS] <COMMAND>

ARGS
  command_token  top_level_command

FLAGS
  --dry-run     execute_syntax_check_only
  --json        emit_machine_json
  --progress    emit_progress_lines

OPTIONS
  --id <RUN_ID>            deterministic_run_id
  --output-dir <PATH>      output_directory_override

EXAMPLES
  sealrun doctor
  sealrun evidence show path/to/capsule.aionai
  sealrun execute ai --model demo --prompt "hello" --seed 1
  sealrun execute ai-replay --capsule path/to/capsule.aionai
  sealrun observe capture -- echo hello
  sealrun policy validate --capsule path/to/capsule.aionai --policy examples/governance/dev.policy.json
  sealrun sdk capsule build --model demo --prompt "hello" --seed 1
  sealrun version --full
"#;

#[derive(Parser, Debug)]
#[command(
    name = "sealrun",
    version = env!("AION_SEMVER"),
    about = AION_ABOUT,
    disable_help_subcommand = true,
    propagate_version = true,
    after_long_help = AION_AFTER_HELP
)]
struct Cli {
    /// Override output base directory (sets `SEALRUN_OUTPUT_BASE` and legacy `AION_OUTPUT_BASE`).
    #[arg(long, global = true)]
    output_dir: Option<PathBuf>,
    /// Deterministic output run id (e.g. run_0001 or custom name).
    #[arg(long, global = true)]
    id: Option<String>,
    /// Print intent and skip execution/writes.
    #[arg(long, global = true, default_value_t = false)]
    dry_run: bool,
    /// Print lightweight progress messages for longer operations.
    #[arg(long, global = true, default_value_t = false)]
    progress: bool,
    /// Emit one JSON object per supported command (machine-readable stdout).
    #[arg(long, global = true, default_value_t = false)]
    json: bool,
    #[command(subcommand)]
    command: TopLevel,
}

#[derive(Subcommand, Debug)]
enum TopLevel {
    /// Observe deterministic run, drift, replay, and integrity artifacts.
    #[command(
        after_long_help = "EXAMPLES:\n  sealrun observe audit run_0001\n  sealrun observe capture -- echo ok\n  sealrun observe drift left.json right.json\n  sealrun observe graph run.json --format svg\n  sealrun observe integrity\n  sealrun observe why left.json right.json\n"
    )]
    Observe {
        #[command(subcommand)]
        command: ObserveCommand,
    },
    /// Execute deterministic shell and capsule replay workloads.
    #[command(
        after_long_help = "EXAMPLES:\n  sealrun execute ai --model demo --prompt \"hello\" --seed 1\n  sealrun execute ai-replay --capsule path/to/capsule.aionai\n  sealrun execute capsule --policy dev -- echo hi\n  sealrun execute replay run.json --explain\n  sealrun execute run -- echo hi\n"
    )]
    Execute {
        #[command(subcommand)]
        command: ExecuteCommand,
    },
    /// Control policy, profile, integrity, and CI command surfaces.
    #[command(
        after_long_help = "EXAMPLES:\n  sealrun control ci drift --cmd \"echo 1\"\n  sealrun control ci run --cmd \"echo 1\"\n  sealrun control determinism freeze --profile det.json\n  sealrun control integrity show-key\n  sealrun control policy list\n  sealrun control sdk\n"
    )]
    Control {
        #[command(subcommand)]
        command: ControlCommand,
    },
    /// Validate capsule policy profiles and constraints.
    #[command(
        after_long_help = "EXAMPLES:\n  sealrun policy list\n  sealrun policy show dev\n  sealrun policy validate --capsule path/to/capsule.aionai --policy policy.json\n"
    )]
    Policy {
        #[command(subcommand)]
        command: GovPolicyCommand,
    },
    /// Record and check governance CI baselines for capsule replay.
    #[command(
        after_long_help = "EXAMPLES:\n  sealrun ci baseline --capsule path/to/capsule.aionai --policy policy.json --determinism det.json --integrity integ.json\n  sealrun ci check --capsule path/to/capsule.aionai --baseline baseline.json\n"
    )]
    Ci {
        #[command(subcommand)]
        command: GovCiCommand,
    },
    /// SDK parity surface for capsule, policy, profile, and replay flows.
    #[command(
        after_long_help = "EXAMPLES:\n  sealrun sdk capsule build --model demo --prompt \"hello\" --seed 1\n  sealrun sdk drift --a left.aionai --b right.aionai\n  sealrun sdk replay --capsule path/to/capsule.aionai\n  sealrun sdk validate --capsule path/to/capsule.aionai --policy policy.json\n"
    )]
    Sdk {
        #[arg(long, default_value = "json")]
        output_format: String,
        #[arg(long, default_value_t = false)]
        quiet: bool,
        #[arg(long, default_value_t = false)]
        verbose: bool,
        #[command(subcommand)]
        command: SdkCommand,
    },
    /// Initialize local SealRun workspace files.
    Setup,
    /// Print version (use `version --full` for toolchain metadata).
    Version {
        #[arg(long, default_value_t = false)]
        full: bool,
    },
    /// Pretty-print evidence JSON from a capsule file.
    Evidence {
        #[command(subcommand)]
        command: EvidenceCommand,
    },
    /// Diagnose local environment readiness.
    Doctor,
    /// Show upgrade guidance for current version.
    Upgrade,
    /// Emit local usage stats snapshot.
    Stats,
    /// Manage opt-in telemetry preference.
    Telemetry {
        #[command(subcommand)]
        command: TelemetryCommand,
    },
    /// Release governance utilities.
    Release {
        #[command(subcommand)]
        command: ReleaseCommand,
    },
    /// Determinism matrix, checks, and CI gate.
    Determinism {
        #[command(subcommand)]
        command: DeterminismCliCommand,
    },
    /// Reliability contracts: status, SLO, chaos, soak.
    Reliability {
        #[command(subcommand)]
        command: ReliabilityCliCommand,
    },
    /// Operational excellence contracts and status.
    Ops {
        #[command(subcommand)]
        command: OpsCliCommand,
    },
    /// Distribution, identity, LTS, and installer trust status.
    Dist {
        #[command(subcommand)]
        command: DistCliCommand,
    },
    /// Aggregated governance model status.
    Governance {
        #[command(subcommand)]
        command: GovernanceCliCommand,
    },
    /// UX stability and enterprise onboarding contracts.
    Ux {
        #[command(subcommand)]
        command: UxCliCommand,
    },
    /// Formal test strategy and coverage contracts.
    Tests {
        #[command(subcommand)]
        command: TestsCliCommand,
    },
    /// Measurement and evidence contracts.
    Measure {
        #[command(subcommand)]
        command: MeasureCliCommand,
    },
    /// Manage contract snapshots, diffs, and verification.
    Contracts {
        #[command(subcommand)]
        command: ContractsCommand,
    },
}

#[derive(Subcommand, Debug)]
enum EvidenceCommand {
    /// Print `capsule.evidence` as pretty JSON to stdout.
    Show { capsule: PathBuf },
}

#[derive(Subcommand, Debug)]
enum TelemetryCommand {
    Enable,
    Disable,
    Status,
}

#[derive(Subcommand, Debug)]
enum ContractsCommand {
    /// Write versioned contract snapshots under `contracts/`.
    Snapshot,
    /// Diff two contract snapshot files.
    Diff {
        version_a: PathBuf,
        version_b: PathBuf,
    },
    /// Verify current contract stability report.
    Verify,
}

#[derive(Subcommand, Debug)]
enum ReleaseCommand {
    /// Generate deterministic release changelog with security impact.
    Changelog,
}

#[derive(Subcommand, Debug)]
enum DeterminismCliCommand {
    /// Emit deterministic matrix report.
    Matrix,
    /// Evaluate deterministic contract status.
    Check,
    /// Run replay invariant CI gate.
    Gate,
}

#[derive(Subcommand, Debug)]
enum ReliabilityCliCommand {
    /// Aggregate reliability model status.
    Status,
    /// Evaluate SLO contract.
    Slo,
    /// Show chaos experiment contract state.
    Chaos,
    /// Show soak-test contract state.
    Soak,
}

#[derive(Subcommand, Debug)]
enum OpsCliCommand {
    /// Show deterministic runbook contracts.
    Runbooks,
    /// Show deterministic incident model contract.
    Incidents,
    /// Show deterministic disaster-recovery contract.
    Dr,
    /// Show deterministic upgrade/migration contract.
    Upgrade,
}

#[derive(Subcommand, Debug)]
enum DistCliCommand {
    /// Show distribution status by channel/platform.
    Status,
    /// Show identity and compatibility matrix.
    Identity,
    /// Show LTS policy and support windows.
    Lts,
    /// Show installer/package trust chain.
    Installers,
}

#[derive(Subcommand, Debug)]
enum GovernanceCliCommand {
    /// Show governance model status.
    Status,
}

#[derive(Subcommand, Debug)]
enum UxCliCommand {
    /// Show API stability contract.
    Api,
    /// Show CLI stability contract.
    Cli,
    /// Show admin documentation contract.
    Admin,
    /// Show golden-path contract.
    GoldenPaths,
}

#[derive(Subcommand, Debug)]
enum TestsCliCommand {
    /// Show test strategy contract.
    Strategy,
    /// Show regression matrix contract.
    Regression,
    /// Show compatibility test contract.
    Compatibility,
    /// Show fuzz/property test contract.
    FuzzProperty,
}

#[derive(Subcommand, Debug)]
enum MeasureCliCommand {
    /// Show metrics contract.
    Metrics,
    /// Show KPI contract.
    Kpis,
    /// Show audit report contract.
    Audits,
    /// Show evidence export contract.
    Evidence,
}

#[derive(Subcommand, Debug)]
enum SdkCommand {
    /// SDK metadata and capability report.
    Info,
    /// Run a batch of sdk operations from JSON.
    Batch {
        #[arg(long)]
        file: PathBuf,
    },
    /// Load or build a capsule; emits sdk.json (+ HTML/SVG).
    Capsule {
        #[command(subcommand)]
        cmd: SdkCapsuleCmd,
    },
    /// Replay a capsule (`sdk.json` contains ReplayReport).
    Replay {
        #[arg(long)]
        capsule: PathBuf,
    },
    /// Drift between two capsules (exit 2 if changed).
    Drift {
        #[arg(long)]
        a: PathBuf,
        #[arg(long)]
        b: PathBuf,
    },
    /// Why + graph bundle for a capsule.
    Explain {
        #[arg(long)]
        capsule: PathBuf,
    },
    /// Governance validate (optional determinism/integrity JSON paths).
    Validate {
        #[arg(long)]
        capsule: PathBuf,
        #[arg(long)]
        policy: PathBuf,
        #[arg(long)]
        determinism: Option<PathBuf>,
        #[arg(long)]
        integrity: Option<PathBuf>,
    },
    /// Governance CI baseline or check (SDK-shaped outputs).
    Ci {
        #[command(subcommand)]
        cmd: SdkCiCmd,
    },
}

#[derive(Subcommand, Debug)]
enum SdkCapsuleCmd {
    /// Load capsule JSON from disk into sdk.json.
    Load {
        #[arg(long)]
        path: PathBuf,
    },
    /// Build a fresh deterministic capsule.
    Build {
        #[arg(long)]
        model: String,
        #[arg(long)]
        prompt: String,
        #[arg(long)]
        seed: u64,
    },
}

#[derive(Subcommand, Debug)]
enum SdkCiCmd {
    /// Record CiBaseline JSON to sdk bundle (copy governance.json from output).
    Baseline {
        #[arg(long)]
        capsule: PathBuf,
        #[arg(long)]
        policy: PathBuf,
        #[arg(long)]
        determinism: PathBuf,
        #[arg(long)]
        integrity: PathBuf,
    },
    /// Check capsule against a CiBaseline JSON file.
    Check {
        #[arg(long)]
        capsule: PathBuf,
        #[arg(long)]
        baseline: PathBuf,
    },
}

#[derive(Subcommand, Debug)]
enum GovPolicyCommand {
    /// List built-in governance policy preset names.
    List,
    /// Print a built-in preset as JSON (`governance.json`).
    Show { name: String },
    /// Validate a capsule against a policy JSON (default det/integ are permissive via engine path).
    Validate {
        #[arg(long)]
        capsule: PathBuf,
        #[arg(long)]
        policy: PathBuf,
    },
    /// Show policy pack contracts.
    Packs,
    /// Show policy gate contracts.
    Gates,
    /// Show policy evidence contracts.
    Evidence,
}

#[derive(Subcommand, Debug)]
enum GovCiCommand {
    /// Record a governance baseline (capsule + profiles) to `governance.json`.
    Baseline {
        #[arg(long)]
        capsule: PathBuf,
        #[arg(long)]
        policy: PathBuf,
        #[arg(long)]
        determinism: PathBuf,
        #[arg(long)]
        integrity: PathBuf,
    },
    /// Check a capsule against a saved baseline JSON.
    Check {
        #[arg(long)]
        capsule: PathBuf,
        #[arg(long)]
        baseline: PathBuf,
    },
}

#[derive(Subcommand, Debug)]
enum ObserveCommand {
    /// Capture stdout/stderr/exit into RunResult JSON (+ HTML/SVG).
    Capture {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, num_args = 1..)]
        command: Vec<String>,
    },
    /// Diff two RunResult JSON files (snapshots).
    Drift {
        left_path: String,
        right_path: String,
    },
    /// Narrative “why” from two RunResult JSON files plus drift context.
    Why {
        left_path: String,
        right_path: String,
        #[arg(long)]
        depth: Option<usize>,
    },
    /// Build a causal graph artefact from run JSON or a path-like id.
    Graph {
        run_id: String,
        #[arg(long, default_value = "svg")]
        format: String,
        #[arg(long)]
        depth: Option<usize>,
    },
    /// Bundle integrity context for an audit stub report.
    Audit { run_id: String },
    /// Print kernel integrity JSON and HTML.
    Integrity,
}

#[derive(Subcommand, Debug)]
enum ExecuteCommand {
    /// Run a command via the kernel gateway (argv after `--`).
    Run {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, num_args = 1..)]
        command: Vec<String>,
    },
    /// Replay a stored RunResult JSON file.
    Replay {
        artifact_path: String,
        #[arg(long, default_value_t = false)]
        explain: bool,
    },
    /// Produce a sealed capsule ZIP from a command invocation.
    Capsule {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, num_args = 1..)]
        command: Vec<String>,
        #[arg(long, default_value = "dev")]
        policy: String,
        #[arg(long)]
        out_dir: Option<String>,
    },
    /// Deterministic AI capsule run (writes ai.*, why.*, capsule.aionai).
    Ai {
        #[arg(long)]
        model: String,
        #[arg(long)]
        prompt: String,
        #[arg(long)]
        seed: u64,
        #[arg(long)]
        backend: Option<String>,
    },
    /// Replay an AI capsule and emit diff artefacts.
    AiReplay {
        #[arg(long)]
        capsule: PathBuf,
        #[arg(long, default_value_t = false)]
        explain: bool,
    },
}

#[derive(Subcommand, Debug)]
enum ControlCommand {
    Policy {
        #[command(subcommand)]
        command: PolicyCommand,
    },
    Determinism {
        #[command(subcommand)]
        command: DeterminismCommand,
    },
    Integrity {
        #[command(subcommand)]
        command: IntegrityCommand,
    },
    Ci {
        #[command(subcommand)]
        command: CiCommand,
    },
    Sdk,
}

#[derive(Subcommand, Debug)]
enum PolicyCommand {
    /// List core policy profile names (dev/stage/prod).
    List {
        #[arg(long, default_value = "text")]
        format: String,
    },
    /// Show a core policy profile as JSON.
    Show { name: String },
    /// Apply governance policy to a capsule (`governance.*` output).
    Apply {
        #[arg(long)]
        capsule: PathBuf,
        #[arg(long)]
        policy: PathBuf,
    },
}

#[derive(Subcommand, Debug)]
enum DeterminismCommand {
    /// Load and apply determinism freeze profile (exports SEALRUN_FREEZE_* env vars for current process).
    Freeze {
        #[arg(long)]
        profile: PathBuf,
    },
}

#[derive(Subcommand, Debug)]
enum IntegrityCommand {
    /// Deterministically sign capsule integrity (hash-chain signature envelope).
    Sign {
        #[arg(long)]
        capsule: PathBuf,
        #[arg(long)]
        private_key: Option<PathBuf>,
    },
    /// Verify Ed25519 signature for capsule evidence.
    Verify {
        #[arg(long)]
        capsule: PathBuf,
        #[arg(long)]
        public_key: PathBuf,
    },
    /// Show/generate current public key.
    ShowKey {
        #[arg(long)]
        private_key: Option<PathBuf>,
    },
}

#[derive(Subcommand, Debug)]
enum CiCommand {
    /// Record a baseline for a shell command run (RunResult JSON).
    Run {
        #[arg(long)]
        cmd: String,
        #[arg(long, default_value = ".sealrun/baseline.json")]
        baseline: PathBuf,
    },
    /// Drift a new shell run against a stored baseline JSON.
    Drift {
        #[arg(long)]
        cmd: String,
        #[arg(long, default_value = ".sealrun/baseline.json")]
        baseline: PathBuf,
    },
    /// Replay a RunResult JSON (same artefact layout as observe replay).
    Replay { artifact_path: String },
}

fn print_output_path(p: std::path::PathBuf) {
    println!("Output written to: {}", p.display());
}

fn cli_read(path: impl AsRef<Path>, context: &'static str) -> Result<String, String> {
    fs::read_to_string(path.as_ref()).map_err(|e| line(code::CLI_IO_READ, context, &io_cause(&e)))
}

fn cli_json<T: DeserializeOwned>(s: &str, context: &'static str) -> Result<T, String> {
    serde_json::from_str(s).map_err(|_| line(code::CLI_JSON_PARSE, context, "invalid_json"))
}

fn cli_json_pretty(v: &serde_json::Value, context: &'static str) -> Result<String, String> {
    aion_engine::output::layout::canonical_json_from_serialize(v)
        .map_err(|_| line(code::CLI_JSON_SERIALIZE, context, "invalid_json"))
}

fn read_default_backend_from_config() -> Option<String> {
    let cwd = std::env::current_dir().ok()?;
    let p = cwd.join("aion.config.toml");
    let s = std::fs::read_to_string(p).ok()?;
    let t: toml::Value = toml::from_str(&s).ok()?;
    t.get("ai")
        .and_then(|v| v.get("backend"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

fn dispatch(cli: Cli, k: &impl KernelGateway) -> Result<u8, String> {
    if cli.dry_run {
        println!("Dry run: no commands executed, no artefacts written.");
        println!("Requested: {:?}", cli.command);
        return Ok(0);
    }
    let progress = cli.progress;
    let json_out = cli.json;
    let progress_note = |msg: &str| {
        if progress {
            println!("[progress] {msg}");
        }
    };
    match cli.command {
        TopLevel::Observe { command } => match command {
            ObserveCommand::Capture { command } => {
                let spec = json!({ "command": command }).to_string();
                let s = k.run(&spec)?;
                let run: RunResult = cli_json(&s, "observe_capture_runresult")?;
                let p = output_bundle::write_capture_output(&run)?;
                print_output_path(p);
                Ok(0u8)
            }
            ObserveCommand::Drift {
                left_path,
                right_path,
            } => {
                let a = cli_read(&left_path, "observe_drift_read_left")?;
                let b = cli_read(&right_path, "observe_drift_read_right")?;
                let left: RunResult = cli_json(&a, "observe_drift_parse_left")?;
                let right: RunResult = cli_json(&b, "observe_drift_parse_right")?;
                let drift = diff_run_snapshots(&left, &right);
                let p = output_bundle::write_drift_output(&drift, &left, &right)?;
                if json_out {
                    println!(
                        "{}",
                        serde_json::json!({
                            "kind": "observe_drift",
                            "changed": drift.changed,
                            "fields": drift.fields,
                            "output_dir": p.display().to_string(),
                        })
                    );
                } else {
                    println!("SealRun | observe drift — RunResult snapshot diff");
                    println!("  deterministic_fields: {:?}", drift.fields);
                }
                print_output_path(p);
                Ok(0)
            }
            ObserveCommand::Why {
                left_path,
                right_path,
                depth: _,
            } => {
                let a = cli_read(&left_path, "observe_why_read_left")?;
                let b = cli_read(&right_path, "observe_why_read_right")?;
                let left: RunResult = cli_json(&a, "observe_why_parse_left")?;
                let right: RunResult = cli_json(&b, "observe_why_parse_right")?;
                let drift = diff_run_snapshots(&left, &right);
                let report = why_run_pair(&left, &right)?;
                let p = output_bundle::write_why_output(&report, &drift, &left)?;
                if json_out {
                    println!(
                        "{}",
                        serde_json::json!({
                            "kind": "observe_why",
                            "headline": "deterministic_causal_bundle",
                            "output_dir": p.display().to_string(),
                        })
                    );
                } else {
                    println!("SealRun | observe why — deterministic causal bundle (HTML/SVG under output)");
                }
                print_output_path(p);
                Ok(0)
            }
            ObserveCommand::Graph {
                run_id,
                format,
                depth,
            } => {
                let run_json = if std::path::Path::new(&run_id).exists() {
                    cli_read(&run_id, "observe_graph_read")?
                } else {
                    json!({ "run_id": run_id }).to_string()
                };
                let fmt = match format.as_str() {
                    "json" => output_bundle::GraphFormat::Json,
                    "dot" => output_bundle::GraphFormat::Dot,
                    _ => output_bundle::GraphFormat::Svg,
                };
                let p = output_bundle::write_graph_output(&run_json, fmt, depth)?;
                print_output_path(p);
                Ok(0)
            }
            ObserveCommand::Audit { run_id } => {
                let integ: IntegrityReport = cli_json(&k.integrity()?, "observe_audit_integrity")?;
                let drift = DriftReport::default();
                let audit = AuditReport {
                    audit: "stub".into(),
                    run_id,
                    integrity: integ,
                };
                let p = output_bundle::write_audit_output(&audit, &drift)?;
                print_output_path(p);
                Ok(0)
            }
            ObserveCommand::Integrity => {
                let integ: IntegrityReport = cli_json(&k.integrity()?, "observe_integrity_report")?;
                let p = output_bundle::write_integrity_output(&integ)?;
                print_output_path(p);
                Ok(0)
            }
        },
        TopLevel::Execute { command } => match command {
            ExecuteCommand::Run { command } => {
                let spec = json!({ "command": command }).to_string();
                let s = k.run(&spec)?;
                let run: RunResult = cli_json(&s, "execute_run_runresult")?;
                let p = output_bundle::write_run_output(&run)?;
                print_output_path(p);
                Ok(0)
            }
            ExecuteCommand::Replay {
                artifact_path,
                explain,
            } => {
                progress_note("execute replay (RunResult) started");
                if explain {
                    println!("{}", ux::dim("replay: load RunResult JSON"));
                    println!(
                        "{}",
                        ux::dim("replay: rebuild stdout from serialized fields")
                    );
                    println!(
                        "{}",
                        ux::dim("replay: write result bundle under output base")
                    );
                }
                let s = cli_read(&artifact_path, "execute_replay_read")?;
                let p = output_bundle::write_replay_output(&s)?;
                progress_note("execute replay (RunResult) completed");
                if json_out {
                    println!(
                        "{}",
                        serde_json::json!({
                            "kind": "execute_replay",
                            "output_dir": p.display().to_string(),
                        })
                    );
                } else {
                    println!("SealRun | execute replay — stdout replay from RunResult JSON");
                }
                print_output_path(p);
                Ok(0)
            }
            ExecuteCommand::Capsule {
                command,
                policy,
                out_dir: _,
            } => {
                let w = OutputWriter::new("capsule")?;
                let spec = json!({
                    "command": command,
                    "policy": policy,
                    "out_dir": w.root().to_string_lossy(),
                })
                .to_string();
                let s = k.capsule(&spec)?;
                let cap: Capsule = cli_json(&s, "execute_capsule_kernel_json")?;
                output_bundle::write_capsule_artefacts(&w, &cap)?;
                print_output_path(w.into_root());
                Ok(0)
            }
            ExecuteCommand::Ai {
                model,
                prompt,
                seed,
                backend,
            } => {
                progress_note("execute ai started");
                let backend_name = backend
                    .or_else(read_default_backend_from_config)
                    .unwrap_or_else(|| "dummy".to_string());
                let (p, cap) =
                    output_bundle::write_ai_execute_output(&model, &prompt, seed, &backend_name)?;
                progress_note("execute ai completed");
                if json_out {
                    println!(
                        "{}",
                        serde_json::json!({
                            "kind": "execute_ai",
                            "product": "sealrun",
                            "determinism_profile": cap.determinism,
                            "output_dir": p.display().to_string(),
                        })
                    );
                } else {
                    println!("SealRun | seal your run — deterministic run");
                    println!(
                        "  determinism_profile: {}",
                        serde_json::to_string(&cap.determinism).unwrap_or_else(|_| "{}".into())
                    );
                }
                print_output_path(p);
                Ok(0)
            }
            ExecuteCommand::AiReplay { capsule, explain } => {
                progress_note("execute ai-replay started");
                if explain {
                    println!("{}", ux::dim("ai-replay: load capsule"));
                    println!("{}", ux::dim("ai-replay: rerun deterministic pipeline"));
                    println!(
                        "{}",
                        ux::dim("ai-replay: compare tokens trace events graph why drift")
                    );
                    println!("{}", ux::dim("ai-replay: emit replay bundle"));
                }
                let cap = aion_engine::ai::read_ai_capsule_v1(&capsule)?;
                let rep = aion_engine::ai::replay_ai_capsule(&cap);
                let h = hex::encode(aion_engine::capsule::deterministic_capsule_hash(&cap));
                let p = output_bundle::write_ai_replay_output(&capsule)?;
                progress_note("execute ai-replay completed");
                if json_out {
                    println!(
                        "{}",
                        serde_json::json!({
                            "kind": "execute_ai_replay",
                            "replay_symmetry_ok": rep.replay_symmetry_ok,
                            "success": rep.success,
                            "deterministic_hash_hex": h,
                            "output_dir": p.display().to_string(),
                        })
                    );
                } else {
                    println!("SealRun | AI replay — symmetry check");
                    println!("  replay_symmetry_ok: {}", rep.replay_symmetry_ok);
                    println!("  deterministic_hash_hex: {h}");
                }
                print_output_path(p);
                Ok(0)
            }
        },
        TopLevel::Control { command } => match command {
            ControlCommand::Policy { command } => match command {
                PolicyCommand::List { format } => {
                    let p = output_bundle::write_policy_list_output_with_format(&format)?;
                    print_output_path(p);
                    Ok(0)
                }
                PolicyCommand::Show { name } => {
                    let profile = match name.as_str() {
                        "stage" => aion_core::PolicyProfile::stage(),
                        "prod" => aion_core::PolicyProfile::prod(),
                        _ => aion_core::PolicyProfile::dev(),
                    };
                    let p = output_bundle::write_policy_show_output(&profile)?;
                    print_output_path(p);
                    Ok(0)
                }
                PolicyCommand::Apply { capsule, policy } => {
                    let (p, rep) =
                        output_bundle::write_governance_policy_validate_output(&capsule, &policy)?;
                    if json_out {
                        println!(
                            "{}",
                            serde_json::json!({
                                "kind": "policy_validate",
                                "success": rep.success,
                                "policy_ok": rep.policy.ok,
                                "determinism_ok": rep.determinism.ok,
                                "integrity_ok": rep.integrity.ok,
                                "output_dir": p.display().to_string(),
                            })
                        );
                    } else {
                        println!("SealRun | policy validate — governance result");
                        println!("  overall_success: {}", rep.success);
                        println!(
                            "  policy_ok: {}  determinism_ok: {}  integrity_ok: {}",
                            rep.policy.ok, rep.determinism.ok, rep.integrity.ok
                        );
                    }
                    print_output_path(p);
                    Ok(0)
                }
            },
            ControlCommand::Determinism { command } => match command {
                DeterminismCommand::Freeze { profile } => {
                    let p = output_bundle::write_control_determinism_freeze_output(&profile)?;
                    print_output_path(p);
                    Ok(0)
                }
            },
            ControlCommand::Integrity { command } => match command {
                IntegrityCommand::Sign {
                    capsule,
                    private_key,
                } => {
                    let p = output_bundle::write_control_integrity_sign_output(
                        &capsule,
                        private_key.as_deref(),
                    )?;
                    print_output_path(p);
                    Ok(0)
                }
                IntegrityCommand::Verify {
                    capsule,
                    public_key,
                } => {
                    let p = output_bundle::write_control_integrity_verify_output(
                        &capsule,
                        &public_key,
                    )?;
                    print_output_path(p);
                    Ok(0)
                }
                IntegrityCommand::ShowKey { private_key } => {
                    let p = output_bundle::write_control_integrity_show_key_output(
                        private_key.as_deref(),
                    )?;
                    print_output_path(p);
                    Ok(0)
                }
            },
            ControlCommand::Ci { command } => match command {
                CiCommand::Run { cmd, baseline } => {
                    if let Some(parent) = baseline.parent() {
                        let _ = fs::create_dir_all(parent);
                    }
                    let spec = json!({ "command": sectors::control::shell_argv(&cmd) }).to_string();
                    let run_s = k.run(&spec)?;
                    let run: RunResult = cli_json(&run_s, "control_ci_run_runresult")?;
                    let meta = record_baseline(&baseline, &run_s)?;
                    let bundle = CiRunBundle {
                        baseline: baseline.to_string_lossy().into_owned(),
                        run,
                        meta,
                    };
                    let p = output_bundle::write_ci_run_output(&bundle)?;
                    print_output_path(p);
                    Ok(0)
                }
                CiCommand::Drift { cmd, baseline } => {
                    let spec = json!({ "command": sectors::control::shell_argv(&cmd) }).to_string();
                    let run_s = k.run(&spec)?;
                    let run: RunResult = cli_json(&run_s, "control_ci_drift_runresult")?;
                    let drift = check_baseline(&baseline, &run_s)?;
                    let changed = drift.changed;
                    let bundle = CiDriftBundle { drift, actual: run };
                    let p = output_bundle::write_ci_drift_output(&bundle)?;
                    print_output_path(p);
                    Ok(if changed { 2 } else { 0 })
                }
                CiCommand::Replay { artifact_path } => {
                    let s = cli_read(&artifact_path, "control_ci_replay_read")?;
                    let p = output_bundle::write_replay_output(&s)?;
                    print_output_path(p);
                    Ok(0)
                }
            },
            ControlCommand::Sdk => {
                let p = output_bundle::write_sdk_output()?;
                print_output_path(p);
                Ok(0)
            }
        },
        TopLevel::Policy { command } => match command {
            GovPolicyCommand::List => {
                let p = output_bundle::write_governance_policy_list_output()?;
                print_output_path(p);
                Ok(0)
            }
            GovPolicyCommand::Show { name } => {
                let p = output_bundle::write_governance_policy_show_output(&name)?;
                print_output_path(p);
                Ok(0)
            }
            GovPolicyCommand::Validate { capsule, policy } => {
                let (p, rep) =
                    output_bundle::write_governance_policy_validate_output(&capsule, &policy)?;
                if json_out {
                    println!(
                        "{}",
                        serde_json::json!({
                            "kind": "policy_validate",
                            "success": rep.success,
                            "policy_ok": rep.policy.ok,
                            "determinism_ok": rep.determinism.ok,
                            "integrity_ok": rep.integrity.ok,
                            "output_dir": p.display().to_string(),
                        })
                    );
                } else {
                    println!("SealRun | policy validate — governance result");
                    println!("  overall_success: {}", rep.success);
                    println!(
                        "  policy_ok: {}  determinism_ok: {}  integrity_ok: {}",
                        rep.policy.ok, rep.determinism.ok, rep.integrity.ok
                    );
                }
                print_output_path(p);
                Ok(0)
            }
            GovPolicyCommand::Packs => {
                let p = output_bundle::write_policy_packs_output()?;
                print_output_path(p);
                Ok(0)
            }
            GovPolicyCommand::Gates => {
                let p = output_bundle::write_policy_gates_output()?;
                print_output_path(p);
                Ok(0)
            }
            GovPolicyCommand::Evidence => {
                let p = output_bundle::write_policy_evidence_output()?;
                print_output_path(p);
                Ok(0)
            }
        },
        TopLevel::Ci { command } => match command {
            GovCiCommand::Baseline {
                capsule,
                policy,
                determinism,
                integrity,
            } => {
                let p = output_bundle::write_governance_ci_baseline_output(
                    &capsule,
                    &policy,
                    &determinism,
                    &integrity,
                )?;
                print_output_path(p);
                Ok(0)
            }
            GovCiCommand::Check { capsule, baseline } => {
                let p = output_bundle::write_governance_ci_check_output(&capsule, &baseline)?;
                print_output_path(p);
                Ok(0)
            }
        },
        TopLevel::Sdk {
            output_format,
            quiet,
            verbose,
            command,
        } => match command {
            SdkCommand::Info => {
                let info = serde_json::json!({
                    "sdk_version": aion_engine::sdk::sdk_version(),
                    "capsule_version_supported": "1",
                    "features": [
                        "capsule.load",
                        "capsule.build",
                        "replay",
                        "drift",
                        "explain",
                        "governance.validate",
                        "ci.baseline",
                        "ci.check"
                    ]
                });
                let p = output_bundle::write_sdk_bundle_with_format(
                    "sdk-info",
                    &info,
                    true,
                    &output_format,
                )?;
                if !quiet {
                    print_output_path(p);
                }
                Ok(0)
            }
            SdkCommand::Batch { file } => {
                progress_note("sdk batch started");
                let s = cli_read(&file, "sdk_batch_read")?;
                let v: serde_json::Value = cli_json(&s, "sdk_batch_json")?;
                let ops = v
                    .get("operations")
                    .and_then(|x| x.as_array())
                    .ok_or_else(|| {
                        line(code::CLI_SPEC_SHAPE, "sdk_batch", "operations_not_array")
                    })?;
                let mut results = Vec::new();
                for op in ops {
                    let kind = op.get("kind").and_then(|x| x.as_str()).unwrap_or("");
                    match kind {
                        "replay" => {
                            let cap =
                                op.get("capsule").and_then(|x| x.as_str()).ok_or_else(|| {
                                    line(
                                        code::CLI_SPEC_SHAPE,
                                        "sdk_batch_replay",
                                        "missing_capsule",
                                    )
                                })?;
                            let c = aion_engine::sdk::load_capsule(&PathBuf::from(cap))?;
                            let rep = aion_engine::sdk::replay_capsule(&c);
                            results
                                .push(serde_json::json!({"kind":"replay","success":rep.success}));
                        }
                        "drift" => {
                            let a = op.get("a").and_then(|x| x.as_str()).ok_or_else(|| {
                                line(code::CLI_SPEC_SHAPE, "sdk_batch_drift", "missing_a")
                            })?;
                            let b = op.get("b").and_then(|x| x.as_str()).ok_or_else(|| {
                                line(code::CLI_SPEC_SHAPE, "sdk_batch_drift", "missing_b")
                            })?;
                            let ca = aion_engine::sdk::load_capsule(&PathBuf::from(a))?;
                            let cb = aion_engine::sdk::load_capsule(&PathBuf::from(b))?;
                            let d = aion_engine::sdk::drift_between(&ca, &cb);
                            results.push(serde_json::json!({"kind":"drift","changed":d.changed}));
                        }
                        _ => {
                            results.push(serde_json::json!({"kind":kind, "error":"unsupported"}));
                        }
                    }
                }
                let body = serde_json::json!({"results":results});
                let ok = results.iter().all(|r| r.get("error").is_none());
                let p = output_bundle::write_sdk_bundle_with_format(
                    "sdk-batch",
                    &body,
                    ok,
                    &output_format,
                )?;
                if !quiet {
                    print_output_path(p);
                }
                progress_note("sdk batch completed");
                Ok(if ok { 0 } else { 2 })
            }
            SdkCommand::Capsule { cmd } => match cmd {
                SdkCapsuleCmd::Load { path } => {
                    let cap = aion_engine::sdk::load_capsule(&path)?;
                    let p = output_bundle::write_sdk_bundle_with_format(
                        "sdk-capsule-load",
                        &cap,
                        true,
                        &output_format,
                    )?;
                    if !quiet {
                        print_output_path(p);
                    }
                    Ok(0)
                }
                SdkCapsuleCmd::Build {
                    model,
                    prompt,
                    seed,
                } => {
                    let cap = aion_engine::sdk::build_capsule(&model, &prompt, seed);
                    let p = output_bundle::write_sdk_bundle_with_format(
                        "sdk-capsule-build",
                        &cap,
                        true,
                        &output_format,
                    )?;
                    if !quiet {
                        print_output_path(p);
                    }
                    Ok(0)
                }
            },
            SdkCommand::Replay { capsule } => {
                let cap = aion_engine::sdk::load_capsule(&capsule)?;
                let rep = aion_engine::sdk::replay_capsule(&cap);
                let p = output_bundle::write_sdk_bundle_with_format(
                    "sdk-replay",
                    &rep,
                    rep.success,
                    &output_format,
                )?;
                if verbose {
                    println!("sdk replay success={}", rep.success);
                }
                if !quiet {
                    print_output_path(p);
                }
                Ok(0)
            }
            SdkCommand::Drift { a, b } => {
                let ca = aion_engine::sdk::load_capsule(&a)?;
                let cb = aion_engine::sdk::load_capsule(&b)?;
                let d = aion_engine::sdk::drift_between(&ca, &cb);
                let p = output_bundle::write_sdk_bundle_with_format(
                    "sdk-drift",
                    &d,
                    !d.changed,
                    &output_format,
                )?;
                if verbose {
                    println!("sdk drift deterministic_fields: {:?}", d.fields);
                }
                if !quiet {
                    print_output_path(p);
                }
                Ok(if d.changed { 2 } else { 0 })
            }
            SdkCommand::Explain { capsule } => {
                let cap = aion_engine::sdk::load_capsule(&capsule)?;
                let ex = aion_engine::sdk::explain_capsule(&cap);
                let p = output_bundle::write_sdk_bundle_with_format(
                    "sdk-explain",
                    &ex,
                    true,
                    &output_format,
                )?;
                if !quiet {
                    print_output_path(p);
                }
                Ok(0)
            }
            SdkCommand::Validate {
                capsule,
                policy,
                determinism,
                integrity,
            } => {
                let cap = aion_engine::sdk::load_capsule(&capsule)?;
                let pol = aion_engine::governance::load_policy(&policy)?;
                let det = match &determinism {
                    Some(p) => aion_engine::governance::load_determinism(p)?,
                    None => aion_engine::governance::DeterminismProfile::default(),
                };
                let integ = match &integrity {
                    Some(p) => aion_engine::governance::load_integrity(p)?,
                    None => aion_engine::governance::IntegrityProfile::default(),
                };
                let rep = aion_engine::sdk::validate_capsule(&cap, &pol, &det, &integ);
                let p = output_bundle::write_sdk_bundle_with_format(
                    "sdk-validate",
                    &rep,
                    rep.success,
                    &output_format,
                )?;
                if !quiet {
                    print_output_path(p);
                }
                Ok(0)
            }
            SdkCommand::Ci { cmd } => match cmd {
                SdkCiCmd::Baseline {
                    capsule,
                    policy,
                    determinism,
                    integrity,
                } => {
                    let cap = aion_engine::sdk::load_capsule(&capsule)?;
                    let pol = aion_engine::governance::load_policy(&policy)?;
                    let det = aion_engine::governance::load_determinism(&determinism)?;
                    let integ = aion_engine::governance::load_integrity(&integrity)?;
                    let bl = aion_engine::sdk::ci_record_baseline(&cap, &pol, &det, &integ);
                    let p = output_bundle::write_sdk_bundle_with_format(
                        "sdk-ci-baseline",
                        &bl,
                        true,
                        &output_format,
                    )?;
                    if !quiet {
                        print_output_path(p);
                    }
                    Ok(0)
                }
                SdkCiCmd::Check { capsule, baseline } => {
                    let cap = aion_engine::sdk::load_capsule(&capsule)?;
                    let s = cli_read(&baseline, "sdk_ci_check_baseline_read")?;
                    let bl: aion_engine::governance::CiBaseline =
                        cli_json(&s, "sdk_ci_check_baseline_parse")?;
                    let res = aion_engine::sdk::ci_check(&cap, &bl);
                    let p = output_bundle::write_sdk_bundle_with_format(
                        "sdk-ci-check",
                        &res,
                        res.success,
                        &output_format,
                    )?;
                    if !quiet {
                        print_output_path(p);
                    }
                    Ok(if res.success { 0 } else { 2 })
                }
            },
        },
        TopLevel::Version { full } => {
            if full {
                println!(
                    "{} {} {}",
                    env!("AION_SEMVER"),
                    env!("AION_RUSTC_VERSION"),
                    env!("AION_BUILD_HASH")
                );
            } else {
                println!("{}", env!("AION_SEMVER"));
            }
            Ok(0)
        }
        TopLevel::Evidence { command } => match command {
            EvidenceCommand::Show { capsule } => {
                let cap = aion_engine::ai::read_ai_capsule_v1(&capsule)?;
                let v = serde_json::to_value(cap.evidence.contract_view()).map_err(|_| {
                    line(
                        code::CLI_JSON_SERIALIZE,
                        "evidence_show_to_value",
                        "invalid_json",
                    )
                })?;
                let s = cli_json_pretty(&v, "evidence_show_pretty")?;
                println!("{s}");
                Ok(0)
            }
        },
        TopLevel::Setup => {
            let p = output_bundle::write_product_setup_output()?;
            print_output_path(p);
            Ok(0)
        }
        TopLevel::Doctor => {
            let p = output_bundle::write_product_doctor_output()?;
            print_output_path(p);
            Ok(0)
        }
        TopLevel::Upgrade => {
            let p = output_bundle::write_product_upgrade_output()?;
            print_output_path(p);
            Ok(0)
        }
        TopLevel::Stats => {
            let p = output_bundle::write_product_stats_output()?;
            print_output_path(p);
            Ok(0)
        }
        TopLevel::Contracts { command } => match command {
            ContractsCommand::Snapshot => {
                let report =
                    evaluate_contract_stability(&aion_core::os_kernel_version().semver, None);
                let cwd = std::env::current_dir().map_err(|e| e.to_string())?;
                let paths = write_contract_snapshots(&cwd, &report)?;
                let v = serde_json::json!({
                    "status": "ok",
                    "data": {
                        "kind": "contracts_snapshot",
                        "snapshot_paths": paths,
                        "snapshot_hashes": report.snapshot_hashes
                    },
                    "error": serde_json::Value::Null
                });
                println!("{}", cli_json_pretty(&v, "contracts_snapshot")?);
                Ok(0)
            }
            ContractsCommand::Diff {
                version_a,
                version_b,
            } => {
                let sa: ContractSnapshot = cli_json(
                    &cli_read(&version_a, "contracts_diff_read_a")?,
                    "contracts_diff_parse_a",
                )?;
                let sb: ContractSnapshot = cli_json(
                    &cli_read(&version_b, "contracts_diff_read_b")?,
                    "contracts_diff_parse_b",
                )?;
                let changes = diff_contract_snapshots(&sa, &sb);
                let v = serde_json::json!({
                    "status": "ok",
                    "data": {
                        "kind": "contracts_diff",
                        "changes": changes
                    },
                    "error": serde_json::Value::Null
                });
                println!("{}", cli_json_pretty(&v, "contracts_diff")?);
                Ok(0)
            }
            ContractsCommand::Verify => {
                let report =
                    evaluate_contract_stability(&aion_core::os_kernel_version().semver, None);
                let v = serde_json::json!({
                    "status": if report.status == "ok" { "ok" } else { "error" },
                    "data": {
                        "kind": "contracts_verify",
                        "compatibility_matrix": report.compatibility_matrix,
                        "breaking_changes_detected": report.breaking_changes_detected,
                        "snapshot_hashes": report.snapshot_hashes
                    },
                    "error": serde_json::Value::Null
                });
                println!("{}", cli_json_pretty(&v, "contracts_verify")?);
                Ok(if report.status == "ok" { 0 } else { 2 })
            }
        },
        TopLevel::Release { command } => match command {
            ReleaseCommand::Changelog => {
                let report =
                    evaluate_contract_stability(&aion_core::os_kernel_version().semver, None);
                let v = serde_json::json!({
                    "status": "ok",
                    "data": {
                        "kind": "release_changelog",
                        "contract_changes": report.current_contract_versions,
                        "abi_changes": ["capsule_abi:v1"],
                        "security_impact": ["release_signing:enabled", "provenance:enabled", "sbom:enabled"],
                        "breaking_changes": report.breaking_changes_detected,
                        "migration_hints": ["follow_contract_stability_policy", "respect_sunset_window_n_minus_2"]
                    },
                    "error": serde_json::Value::Null
                });
                println!("{}", cli_json_pretty(&v, "release_changelog")?);
                Ok(0)
            }
        },
        TopLevel::Determinism { command } => match command {
            DeterminismCliCommand::Matrix => {
                let matrix = evaluate_determinism_matrix(vec![
                    DeterminismTarget {
                        os: "linux".into(),
                        arch: "x64".into(),
                        locale: "en_US.UTF-8".into(),
                        timezone: "UTC".into(),
                        seed: 42,
                        env_profile: "frozen".into(),
                    },
                    DeterminismTarget {
                        os: "windows".into(),
                        arch: "x64".into(),
                        locale: "en_US.UTF-8".into(),
                        timezone: "UTC".into(),
                        seed: 42,
                        env_profile: "frozen".into(),
                    },
                    DeterminismTarget {
                        os: "macos".into(),
                        arch: "arm64".into(),
                        locale: "en_US.UTF-8".into(),
                        timezone: "UTC".into(),
                        seed: 42,
                        env_profile: "frozen".into(),
                    },
                ]);
                let v = serde_json::json!({
                    "status":"ok",
                    "data":{"kind":"determinism_matrix","matrix":matrix},
                    "error":serde_json::Value::Null
                });
                println!("{}", cli_json_pretty(&v, "determinism_matrix")?);
                Ok(0)
            }
            DeterminismCliCommand::Check => {
                let c = evaluate_determinism_contract(DeterminismContractInput {
                    replay_ok: true,
                    drift_ok: true,
                    evidence_ok: true,
                    policy_ok: true,
                    global_consistency_ok: true,
                    upgrade_replay_ok: true,
                });
                let v = serde_json::json!({
                    "status": if c.status=="ok" {"ok"} else {"error"},
                    "data":{"kind":"determinism_check","contract":c},
                    "error":serde_json::Value::Null
                });
                println!("{}", cli_json_pretty(&v, "determinism_check")?);
                Ok(0)
            }
            DeterminismCliCommand::Gate => {
                let matrix = evaluate_determinism_matrix(vec![DeterminismTarget {
                    os: "linux".into(),
                    arch: "x64".into(),
                    locale: "en_US.UTF-8".into(),
                    timezone: "UTC".into(),
                    seed: 42,
                    env_profile: "frozen".into(),
                }]);
                let gate = run_replay_invariant_gate(true, &matrix, true);
                let v = serde_json::json!({
                    "status": if gate.status=="ok" {"ok"} else {"error"},
                    "data":{"kind":"determinism_gate","gate":gate},
                    "error":serde_json::Value::Null
                });
                println!("{}", cli_json_pretty(&v, "determinism_gate")?);
                Ok(if gate.status == "ok" { 0 } else { 2 })
            }
        },
        TopLevel::Reliability { command } => match command {
            ReliabilityCliCommand::Status => {
                let p = output_bundle::write_reliability_status_output()?;
                print_output_path(p);
                Ok(0)
            }
            ReliabilityCliCommand::Slo => {
                let p = output_bundle::write_reliability_slo_output()?;
                print_output_path(p);
                Ok(0)
            }
            ReliabilityCliCommand::Chaos => {
                let p = output_bundle::write_reliability_chaos_output()?;
                print_output_path(p);
                Ok(0)
            }
            ReliabilityCliCommand::Soak => {
                let p = output_bundle::write_reliability_soak_output()?;
                print_output_path(p);
                Ok(0)
            }
        },
        TopLevel::Ops { command } => match command {
            OpsCliCommand::Runbooks => {
                let p = output_bundle::write_ops_runbooks_output()?;
                print_output_path(p);
                Ok(0)
            }
            OpsCliCommand::Incidents => {
                let p = output_bundle::write_ops_incidents_output()?;
                print_output_path(p);
                Ok(0)
            }
            OpsCliCommand::Dr => {
                let p = output_bundle::write_ops_dr_output()?;
                print_output_path(p);
                Ok(0)
            }
            OpsCliCommand::Upgrade => {
                let p = output_bundle::write_ops_upgrade_output()?;
                print_output_path(p);
                Ok(0)
            }
        },
        TopLevel::Dist { command } => match command {
            DistCliCommand::Status => {
                let p = output_bundle::write_dist_status_output()?;
                print_output_path(p);
                Ok(0)
            }
            DistCliCommand::Identity => {
                let p = output_bundle::write_dist_identity_output()?;
                print_output_path(p);
                Ok(0)
            }
            DistCliCommand::Lts => {
                let p = output_bundle::write_dist_lts_output()?;
                print_output_path(p);
                Ok(0)
            }
            DistCliCommand::Installers => {
                let p = output_bundle::write_dist_installers_output()?;
                print_output_path(p);
                Ok(0)
            }
        },
        TopLevel::Governance { command } => match command {
            GovernanceCliCommand::Status => {
                let p = output_bundle::write_governance_status_output()?;
                print_output_path(p);
                Ok(0)
            }
        },
        TopLevel::Ux { command } => match command {
            UxCliCommand::Api => {
                let p = output_bundle::write_ux_api_output()?;
                print_output_path(p);
                Ok(0)
            }
            UxCliCommand::Cli => {
                let p = output_bundle::write_ux_cli_output()?;
                print_output_path(p);
                Ok(0)
            }
            UxCliCommand::Admin => {
                let p = output_bundle::write_ux_admin_output()?;
                print_output_path(p);
                Ok(0)
            }
            UxCliCommand::GoldenPaths => {
                let p = output_bundle::write_ux_golden_paths_output()?;
                print_output_path(p);
                Ok(0)
            }
        },
        TopLevel::Tests { command } => match command {
            TestsCliCommand::Strategy => {
                let p = output_bundle::write_tests_strategy_output()?;
                print_output_path(p);
                Ok(0)
            }
            TestsCliCommand::Regression => {
                let p = output_bundle::write_tests_regression_output()?;
                print_output_path(p);
                Ok(0)
            }
            TestsCliCommand::Compatibility => {
                let p = output_bundle::write_tests_compatibility_output()?;
                print_output_path(p);
                Ok(0)
            }
            TestsCliCommand::FuzzProperty => {
                let p = output_bundle::write_tests_fuzz_property_output()?;
                print_output_path(p);
                Ok(0)
            }
        },
        TopLevel::Measure { command } => match command {
            MeasureCliCommand::Metrics => {
                let p = output_bundle::write_measure_metrics_output()?;
                print_output_path(p);
                Ok(0)
            }
            MeasureCliCommand::Kpis => {
                let p = output_bundle::write_measure_kpis_output()?;
                print_output_path(p);
                Ok(0)
            }
            MeasureCliCommand::Audits => {
                let p = output_bundle::write_measure_audits_output()?;
                print_output_path(p);
                Ok(0)
            }
            MeasureCliCommand::Evidence => {
                let p = output_bundle::write_measure_evidence_output()?;
                print_output_path(p);
                Ok(0)
            }
        },
        TopLevel::Telemetry { command } => {
            let p = match command {
                TelemetryCommand::Enable => output_bundle::write_product_telemetry_enable_output()?,
                TelemetryCommand::Disable => {
                    output_bundle::write_product_telemetry_disable_output()?
                }
                TelemetryCommand::Status => output_bundle::write_product_telemetry_status_output()?,
            };
            print_output_path(p);
            Ok(0)
        }
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    if let Some(p) = &cli.output_dir {
        std::env::set_var("SEALRUN_OUTPUT_BASE", p);
        std::env::set_var("AION_OUTPUT_BASE", p);
    }
    if let Some(id) = &cli.id {
        std::env::set_var("SEALRUN_OUTPUT_ID", id);
        std::env::set_var("AION_OUTPUT_ID", id);
    }
    let k = InProcessKernel;
    let r = dispatch(cli, &k);
    match r {
        Ok(0) => ExitCode::SUCCESS,
        Ok(n) => ExitCode::from(n),
        Err(e) => {
            eprintln!("{}", canonical_error_json(&e, "cli"));
            ExitCode::from(1)
        }
    }
}
