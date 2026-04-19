// System evaluation.
//
// Full introspection of `repro`. Produces two deterministic outputs:
//
//   * `render_markdown(&SystemReport)` — human-readable markdown
//   * `render_json(&SystemReport)`     — machine-readable JSON
//
// The report is built by reading the source tree at compile time via
// `include_str!` and scanning for `pub fn`, `pub struct`, `pub enum`,
// `pub trait`, and `pub mod` declarations. That keeps the "feature
// inventory" section honest: it only shows what the binary actually
// ships.
//
// Every section is a pure function of the embedded source and of the
// curated product-analyzer entries. No I/O, no env vars, no clock:
// `repro eval` is byte-identical across runs and machines.

use crate::analysis::product_analyzer::{self, ProductGap};
use crate::core::report::Severity;
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Data model
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SystemReport {
    pub category_definition: String,
    pub feature_inventory: Vec<ModuleInventory>,
    pub architecture_map: Vec<ArchitectureNode>,
    pub data_flow: Vec<DataFlowEdge>,
    pub determinism_guarantees: Vec<String>,
    pub ux_flows: Vec<UxFlow>,
    pub missing_capabilities: Vec<ProductGap>,
    pub competitive_positioning: Vec<CompetitivePeer>,
    pub maturity: Maturity,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModuleInventory {
    pub path: String,
    pub lines: usize,
    pub pub_fns: Vec<String>,
    pub pub_structs: Vec<String>,
    pub pub_enums: Vec<String>,
    pub pub_traits: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ArchitectureNode {
    pub layer: String,
    pub module: String,
    pub purpose: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DataFlowEdge {
    pub from: String,
    pub to: String,
    pub carries: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UxFlow {
    pub command: String,
    pub steps: Vec<String>,
    pub determinism_notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CompetitivePeer {
    pub name: String,
    pub category: String,
    pub overlap: String,
    pub distinction: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Maturity {
    pub score: u8, // 0..=scale_max
    pub scale_max: u8,
    pub breakdown: Vec<MaturityComponent>,
    pub rationale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MaturityComponent {
    pub dimension: String,
    pub score: u8,
    pub max: u8,
    pub note: String,
}

// ---------------------------------------------------------------------------
// Category definition — the single sentence locked in by Phase-5.
// Change here, in the README, and in CLI help together.
// ---------------------------------------------------------------------------

pub const CATEGORY_DEFINITION: &str =
    "Deterministic execution debugging for reproducible systems.";

// ---------------------------------------------------------------------------
// Source tree, embedded at compile time.
// ---------------------------------------------------------------------------

struct SrcFile {
    path: &'static str,
    text: &'static str,
}

const SOURCES: &[SrcFile] = &[
    SrcFile {
        path: "src/main.rs",
        text: include_str!("../main.rs"),
    },
    SrcFile {
        path: "src/lib.rs",
        text: include_str!("../lib.rs"),
    },
    SrcFile {
        path: "src/cli/mod.rs",
        text: include_str!("../cli/mod.rs"),
    },
    SrcFile {
        path: "src/cli/ci_handler.rs",
        text: include_str!("../cli/ci_handler.rs"),
    },
    SrcFile {
        path: "src/cli/run_handler.rs",
        text: include_str!("../cli/run_handler.rs"),
    },
    SrcFile {
        path: "src/cli/replay_handler.rs",
        text: include_str!("../cli/replay_handler.rs"),
    },
    SrcFile {
        path: "src/cli/diff_handler.rs",
        text: include_str!("../cli/diff_handler.rs"),
    },
    SrcFile {
        path: "src/cli/root_cause_handler.rs",
        text: include_str!("../cli/root_cause_handler.rs"),
    },
    SrcFile {
        path: "src/cli/graph_handler.rs",
        text: include_str!("../cli/graph_handler.rs"),
    },
    SrcFile {
        path: "src/cli/why_handler.rs",
        text: include_str!("../cli/why_handler.rs"),
    },
    SrcFile {
        path: "src/cli/eval_handler.rs",
        text: include_str!("../cli/eval_handler.rs"),
    },
    SrcFile {
        path: "src/core/mod.rs",
        text: include_str!("../core/mod.rs"),
    },
    SrcFile {
        path: "src/core/artifact.rs",
        text: include_str!("../core/artifact.rs"),
    },
    SrcFile {
        path: "src/core/execution_boundary.rs",
        text: include_str!("../core/execution_boundary.rs"),
    },
    SrcFile {
        path: "src/core/capture.rs",
        text: include_str!("../core/capture.rs"),
    },
    SrcFile {
        path: "src/core/causal_graph.rs",
        text: include_str!("../core/causal_graph.rs"),
    },
    SrcFile {
        path: "src/core/causal_query.rs",
        text: include_str!("../core/causal_query.rs"),
    },
    SrcFile {
        path: "src/core/execution_trace.rs",
        text: include_str!("../core/execution_trace.rs"),
    },
    SrcFile {
        path: "src/core/storage.rs",
        text: include_str!("../core/storage.rs"),
    },
    SrcFile {
        path: "src/core/diff.rs",
        text: include_str!("../core/diff.rs"),
    },
    SrcFile {
        path: "src/core/replay.rs",
        text: include_str!("../core/replay.rs"),
    },
    SrcFile {
        path: "src/core/output.rs",
        text: include_str!("../core/output.rs"),
    },
    SrcFile {
        path: "src/core/root_cause.rs",
        text: include_str!("../core/root_cause.rs"),
    },
    SrcFile {
        path: "src/core/identity.rs",
        text: include_str!("../core/identity.rs"),
    },
    SrcFile {
        path: "src/core/report.rs",
        text: include_str!("../core/report.rs"),
    },
    SrcFile {
        path: "src/core/contract.rs",
        text: include_str!("../core/contract.rs"),
    },
    SrcFile {
        path: "src/analysis/mod.rs",
        text: include_str!("./mod.rs"),
    },
    SrcFile {
        path: "src/analysis/product_analyzer.rs",
        text: include_str!("./product_analyzer.rs"),
    },
    SrcFile {
        path: "src/analysis/system_evaluation.rs",
        text: include_str!("./system_evaluation.rs"),
    },
    SrcFile {
        path: "src/engine/mod.rs",
        text: include_str!("../engine/mod.rs"),
    },
    SrcFile {
        path: "src/engine/adapter.rs",
        text: include_str!("../engine/cos_adapter.rs"),
    },
    SrcFile {
        path: "src/ci/mod.rs",
        text: include_str!("../ci/mod.rs"),
    },
    SrcFile {
        path: "src/ci/baseline.rs",
        text: include_str!("../ci/baseline.rs"),
    },
    SrcFile {
        path: "src/ci/ci_orchestrator.rs",
        text: include_str!("../ci/ci_orchestrator.rs"),
    },
    SrcFile {
        path: "src/ci/schema.rs",
        text: include_str!("../ci/schema.rs"),
    },
    SrcFile {
        path: "src/ci/environment.rs",
        text: include_str!("../ci/environment.rs"),
    },
    SrcFile {
        path: "src/ci/meta.rs",
        text: include_str!("../ci/meta.rs"),
    },
    SrcFile {
        path: "src/ci/storage.rs",
        text: include_str!("../ci/storage.rs"),
    },
    SrcFile {
        path: "src/ci/diff.rs",
        text: include_str!("../ci/diff.rs"),
    },
    SrcFile {
        path: "src/ci/root_cause.rs",
        text: include_str!("../ci/root_cause.rs"),
    },
];

// ---------------------------------------------------------------------------
// Build the report.
// ---------------------------------------------------------------------------

pub fn evaluate() -> SystemReport {
    SystemReport {
        category_definition: CATEGORY_DEFINITION.to_string(),
        feature_inventory: feature_inventory(),
        architecture_map: architecture_map(),
        data_flow: data_flow(),
        determinism_guarantees: determinism_guarantees(),
        ux_flows: ux_flows(),
        missing_capabilities: product_analyzer::gaps(),
        competitive_positioning: competitive_positioning(),
        maturity: compute_maturity(),
    }
}

fn feature_inventory() -> Vec<ModuleInventory> {
    let mut out: Vec<ModuleInventory> = SOURCES.iter().map(scan_module).collect();
    out.sort_by(|a, b| a.path.cmp(&b.path));
    out
}

fn scan_module(src: &SrcFile) -> ModuleInventory {
    let mut pub_fns = Vec::new();
    let mut pub_structs = Vec::new();
    let mut pub_enums = Vec::new();
    let mut pub_traits = Vec::new();

    for raw_line in src.text.lines() {
        let line = raw_line.trim_start();

        // Strip `pub(crate)` / `pub(super)` etc. so we only count
        // genuinely crate-public items.
        let stripped = if let Some(rest) = line.strip_prefix("pub(") {
            if let Some(after) = rest.find(')') {
                rest[after + 1..].trim_start()
            } else {
                line
            }
        } else if let Some(rest) = line.strip_prefix("pub ") {
            rest
        } else {
            continue;
        };

        if let Some(name) = parse_name(stripped, "fn ") {
            pub_fns.push(name);
        } else if let Some(name) = parse_name(stripped, "struct ") {
            pub_structs.push(name);
        } else if let Some(name) = parse_name(stripped, "enum ") {
            pub_enums.push(name);
        } else if let Some(name) = parse_name(stripped, "trait ") {
            pub_traits.push(name);
        } else if let Some(name) = parse_name(stripped, "async fn ") {
            // Defensive: the project is explicitly no-async, so if this
            // ever fires it should also fail the determinism audit.
            pub_fns.push(format!("{name} (async!)"));
        }
    }

    pub_fns.sort();
    pub_structs.sort();
    pub_enums.sort();
    pub_traits.sort();

    ModuleInventory {
        path: src.path.to_string(),
        lines: src.text.lines().count(),
        pub_fns,
        pub_structs,
        pub_enums,
        pub_traits,
    }
}

fn parse_name(line: &str, keyword: &str) -> Option<String> {
    let rest = line.strip_prefix(keyword)?;
    // Name ends at the first of `(`, `<`, `{`, whitespace, `:`, `;`.
    let end = rest
        .find(|c: char| {
            c == '(' || c == '<' || c == '{' || c == ':' || c == ';' || c.is_whitespace()
        })
        .unwrap_or(rest.len());
    let name = rest[..end].trim();
    if name.is_empty() {
        None
    } else {
        Some(name.to_string())
    }
}

fn architecture_map() -> Vec<ArchitectureNode> {
    let mut v = vec![
        a(
            "cli",
            "cli::run_handler",
            "Parse `run` args; delegate capture + save.",
        ),
        a(
            "cli",
            "cli::replay_handler",
            "Load artifact by id/alias and print replay.",
        ),
        a(
            "cli",
            "cli::diff_handler",
            "Resolve both ids, load artifacts, print field diff.",
        ),
        a(
            "cli",
            "cli::graph_handler",
            "Load artifact; print causal graph projection for trace.",
        ),
        a(
            "cli",
            "cli::why_handler",
            "`repro why` — causes, effects, graph divergence vs previous.",
        ),
        a(
            "cli",
            "cli::root_cause_handler",
            "Build execution report; render root-cause summary.",
        ),
        a(
            "cli",
            "cli::eval_handler",
            "Run system_evaluation; print MD + JSON.",
        ),
        a(
            "cli",
            "cli::ci_handler",
            "`repro ci run|ingest|diff|why|list` → CI ledger.",
        ),
        a(
            "core",
            "core::artifact",
            "Single run-record schema (local + CI).",
        ),
        a(
            "core",
            "core::execution_boundary",
            "15% env slice + OS_EXPOSURE_RATIO budget.",
        ),
        a(
            "core",
            "core::capture",
            "Real subprocess capture → run record.",
        ),
        a(
            "core",
            "core::storage",
            "Local `./repro_runs/` JSON + INDEX persistence.",
        ),
        a(
            "core",
            "core::diff",
            "Ordered field-by-field diff between artifacts.",
        ),
        a(
            "core",
            "core::causal_graph",
            "Deterministic projection ExecutionTrace → CausalGraph.",
        ),
        a(
            "core",
            "core::causal_query",
            "Causes/effects/path queries + graph divergence on CausalGraph.",
        ),
        a(
            "core",
            "core::replay",
            "Re-render a stored artifact; never re-executes.",
        ),
        a(
            "core",
            "core::root_cause",
            "Semantic primary-cause engine (rule-based).",
        ),
        a(
            "core",
            "core::identity",
            "ExecutionIdentity: command/env/input/trace signatures.",
        ),
        a(
            "core",
            "core::report",
            "Unified ExecutionReport + semantic cause types.",
        ),
        a(
            "core",
            "core::contract",
            "OutputContract validator for deterministic UX.",
        ),
        a(
            "core",
            "core::output",
            "Single source of truth for printed text.",
        ),
        a(
            "analysis",
            "analysis::product_analyzer",
            "Honest gap / failure-mode inventory.",
        ),
        a(
            "analysis",
            "analysis::system_evaluation",
            "This module; full self-introspection.",
        ),
        a(
            "engine",
            "engine::integration_adapter",
            "Integration seam (stub) for future platform wiring.",
        ),
        a(
            "ci",
            "ci::environment",
            "EnvironmentSnapshot + SHA-256 fingerprint.",
        ),
        a(
            "ci",
            "ci::storage",
            "Immutable runs under `./repro_ci_store/` + INDEX.jsonl.",
        ),
        a(
            "ci",
            "ci::diff",
            "Semantic CIComparisonReport between runs.",
        ),
        a(
            "ci",
            "ci::root_cause",
            "RootCauseReport from env / exit / stream heuristics.",
        ),
    ];
    v.sort_by(|x, y| {
        (x.layer.as_str(), x.module.as_str()).cmp(&(y.layer.as_str(), y.module.as_str()))
    });
    v
}

fn a(layer: &str, module: &str, purpose: &str) -> ArchitectureNode {
    ArchitectureNode {
        layer: layer.to_string(),
        module: module.to_string(),
        purpose: purpose.to_string(),
    }
}

fn data_flow() -> Vec<DataFlowEdge> {
    let mut v = vec![
        e("user CLI args", "cli::mod::run", "clap-parsed Command enum"),
        e("cli::run_handler", "core::capture", "joined command string"),
        e("core::capture", "core::storage", "run record"),
        e(
            "core::storage",
            "disk: repro_runs/*.json + INDEX",
            "pretty JSON + id log",
        ),
        e("cli::replay_handler", "core::replay", "run_id or alias"),
        e(
            "core::replay",
            "core::output",
            "run record → rendered string",
        ),
        e("cli::diff_handler", "core::storage", "two run_ids"),
        e("cli::graph_handler", "core::storage", "run_id or alias"),
        e(
            "cli::graph_handler",
            "core::causal_graph",
            "ExecutionTrace → CausalGraph → text",
        ),
        e(
            "cli::why_handler",
            "core::causal_query",
            "CausalGraph → causes/effects/divergence text",
        ),
        e("core::diff", "core::output", "DiffReport → rendered string"),
        e(
            "cli::root_cause_handler",
            "core::root_cause",
            "run_id or alias",
        ),
        e(
            "core::root_cause",
            "core::report",
            "ExecutionReport { identity, trace, diff, root_cause }",
        ),
        e(
            "core::report",
            "core::output",
            "ExecutionReport → format_root_cause_summary",
        ),
        e(
            "cli::eval_handler",
            "analysis::system_evaluation",
            "() → SystemReport",
        ),
        e("analysis::system_evaluation", "stdout", "Markdown + JSON"),
        e(
            "cli::ci_handler",
            "core::capture",
            "same argv pipeline as `repro run`",
        ),
        e(
            "cli::ci_handler",
            "ci::storage",
            "ingested JSON → same INDEX as `ci run`",
        ),
        e(
            "core::capture",
            "ci::storage",
            "run record + CI execution metadata",
        ),
        e(
            "ci::storage",
            "disk: repro_ci_store/** + INDEX.jsonl",
            "artifact.json + streams",
        ),
        e(
            "cli::ci_handler",
            "ci::diff",
            "two run_ids → CIComparisonReport",
        ),
        e(
            "cli::ci_handler",
            "ci::root_cause",
            "run_id → RootCauseReport",
        ),
    ];
    v.sort_by(|x, y| (x.from.as_str(), x.to.as_str()).cmp(&(y.from.as_str(), y.to.as_str())));
    v
}

fn e(from: &str, to: &str, carries: &str) -> DataFlowEdge {
    DataFlowEdge {
        from: from.to_string(),
        to: to.to_string(),
        carries: carries.to_string(),
    }
}

fn determinism_guarantees() -> Vec<String> {
    vec![
        "Same (command, clock, counter) → byte-identical run record.".into(),
        "Run-record field order is fixed and drives JSON serialization and diff semantics.".into(),
        "JSON is pretty-printed with 2-space indent via serde_json; field order preserved by `preserve_order`.".into(),
        "INDEX is append-only and gates `last` / `prev` alias resolution; no reliance on filesystem mtime.".into(),
        "`diff::FIELDS` is a `const &[&str]` — reordering is a contract change and must be intentional.".into(),
        "`report::PRIORITY` is a `const &[CauseCategory]` — primary-cause selection is stable by construction.".into(),
        "`output::format_*` functions are pure string builders; no time, no env, no random sources.".into(),
        "`OutputContract::validate` rejects `\\r`, ANSI escapes, control chars (other than \\t), and trailing whitespace.".into(),
        "`system_evaluation::evaluate` is a pure function of embedded source text; no I/O.".into(),
        "No async, no threads, no network, no clock outside `capture::SystemClock`.".into(),
        "CI ledger: `INDEX.jsonl` append-only; `artifact.json` matches local schema v4; `meta.json` is non-diff metadata.".into(),
        "`execution_boundary`: only PATH, HOME, CI, SHELL, LANG (+ cwd) feed `environment_hash`.".into(),
        "`causal_graph::build_causal_graph` is a pure projection from `ExecutionTrace` (no I/O).".into(),
        "`causal_query` functions are pure graph walks (BFS queue is FIFO; no random tie-breaks).".into(),
    ]
}

fn ux_flows() -> Vec<UxFlow> {
    vec![
        UxFlow {
            command: "aion repro run -- <cmd>".into(),
            steps: vec![
                "CLI parses argv into the run subcommand".into(),
                "Run handler joins argv and invokes capture".into(),
                "Capture builds a run record using the system clock".into(),
                "Storage writes JSON and event stream atomically and appends INDEX".into(),
                "Summary formatting is printed to stdout".into(),
            ],
            determinism_notes: "Timestamp comes from the wall clock, so output differs between real executions. All other fields are a pure function of the command.".into(),
        },
        UxFlow {
            command: "aion repro replay <id|last>".into(),
            steps: vec![
                "Resolve id or alias to a concrete run id".into(),
                "Load event stream when present (else fall back to embedded trace)".into(),
                "Concatenate Stdout events and print raw stdout".into(),
            ],
            determinism_notes: "Pure transform of stored events: identical inputs → identical replay bytes.".into(),
        },
        UxFlow {
            command: "aion repro why <id|last>".into(),
            steps: vec![
                "resolve_alias, load_run for target artifact".into(),
                "build_causal_graph; pick deterministic focal (stdout > stderr > …)".into(),
                "causal_query::{query_causes, query_effects}; compare vs previous graph".into(),
            ],
            determinism_notes: "Pure transforms of stored traces; same artifacts → same CAUSE ANALYSIS block.".into(),
        },
        UxFlow {
            command: "aion repro graph <id|last>".into(),
            steps: vec![
                "resolve_alias then load_run".into(),
                "causal_graph::build_causal_graph(&artifact.trace)".into(),
                "format_causal_graph_text printed to stdout".into(),
            ],
            determinism_notes: "Pure transform of stored trace; identical artifact → identical graph text.".into(),
        },
        UxFlow {
            command: "aion repro diff <a> <b>".into(),
            steps: vec![
                "resolve_alias for both ids".into(),
                "load_run for both artifacts".into(),
                "diff::diff_runs walks FIELDS in fixed order".into(),
                "output::format_diff renders `~ <field>` sections".into(),
            ],
            determinism_notes: "Output ordered by FIELDS; two identical artifacts always produce `no differences`.".into(),
        },
        UxFlow {
            command: "aion repro root-cause <id|last>".into(),
            steps: vec![
                "resolve_alias, list_runs to locate previous".into(),
                "Build root-cause report from the run pair".into(),
                "classify each FieldDiff into a SemanticCause".into(),
                "select_primary picks by (severity rank, PRIORITY)".into(),
                "format_root_cause_summary renders primary + all causes".into(),
            ],
            determinism_notes: "Root-cause human output uses a single semantic block (previous run + primary + diverged fields).".into(),
        },
        UxFlow {
            command: "repro eval".into(),
            steps: vec![
                "Self-evaluation reads embedded sources (hidden command)".into(),
                "Markdown report is printed".into(),
                "JSON summary is printed after the divider".into(),
            ],
            determinism_notes: "Pure function; identical binary → identical report on every machine.".into(),
        },
    ]
}

fn competitive_positioning() -> Vec<CompetitivePeer> {
    let mut v = vec![
        c("rr (Mozilla)", "record & replay debugger",
          "Records process execution; supports deterministic replay.",
          "rr targets a single debugging session on Linux; AION Repro is artifact-centric, local-first, cross-command, and semantic (root cause + identity, not just replay)."),
        c("reprozip", "computational reproducibility",
          "Captures a command + its env to re-run elsewhere.",
          "reprozip targets re-execution on another host. AION Repro targets *truth about a specific execution*: compare, root-cause, explain — not re-run."),
        c("git bisect", "regression localization",
          "Finds the commit that introduced a failure.",
          "git bisect needs a repeatable test and N commits. AION Repro needs only two runs and produces a semantic cause immediately."),
        c("hyperfine", "benchmarking",
          "Runs a command N times and reports timing.",
          "hyperfine measures. AION Repro explains."),
        c("CI artifact logs", "post-hoc log archive",
          "Keep stdout/stderr per build.",
          "Raw logs are unstructured text. AION Repro stores a typed run record with identity and supports field-level diff + semantic root cause."),
        c("snapshot testing (insta, jest)", "output regression detection",
          "Detects that output drifted.",
          "Snapshots say *that* something changed. AION Repro says *which semantic axis* changed and ranks it."),
    ];
    v.sort_by(|a, b| a.name.cmp(&b.name));
    v
}

fn c(name: &str, category: &str, overlap: &str, distinction: &str) -> CompetitivePeer {
    CompetitivePeer {
        name: name.to_string(),
        category: category.to_string(),
        overlap: overlap.to_string(),
        distinction: distinction.to_string(),
    }
}

// ---------------------------------------------------------------------------
// Maturity score — weighted sum clamped to `scale_max` (feature + determinism
// + UX + causal graph + causal query completeness − honesty penalty).
// ---------------------------------------------------------------------------

fn compute_maturity() -> Maturity {
    // Feature coverage: how much of the canonical feature set is
    // present. We count modules whose public symbols are non-empty.
    let inv = feature_inventory();
    let modules_with_symbols = inv
        .iter()
        .filter(|m| {
            !m.pub_fns.is_empty()
                || !m.pub_structs.is_empty()
                || !m.pub_enums.is_empty()
                || !m.pub_traits.is_empty()
        })
        .count();
    let feature_score = scale(modules_with_symbols, 14, 3); // target ~14 public modules

    // Determinism: number of guarantees we can enumerate. 10+ = full.
    let guarantees = determinism_guarantees().len();
    let determinism_score = scale(guarantees, 10, 3);

    // UX: one point per covered command flow, capped.
    let flows = ux_flows().len();
    let ux_score = scale(flows, 7, 2);

    let causal_graph_completeness = crate::core::causal_graph::causal_graph_completeness_score();
    let causal_query_completeness = crate::core::causal_query::causal_query_completeness_score();

    // Honesty penalty: high-severity gaps reduce the score.
    let gaps = product_analyzer::gaps();
    let high_gaps = gaps.iter().filter(|g| g.severity == Severity::High).count();
    let honesty_penalty = high_gaps.min(3) as u8; // cap at 3

    let raw_total = feature_score
        + determinism_score
        + ux_score
        + causal_graph_completeness
        + causal_query_completeness;
    let score = raw_total.saturating_sub(honesty_penalty).min(15);
    let scale_max: u8 = 15;

    let breakdown = vec![
        MaturityComponent {
            dimension: "feature_coverage".into(),
            score: feature_score,
            max: 3,
            note: format!("{} public modules carry symbols.", modules_with_symbols),
        },
        MaturityComponent {
            dimension: "determinism".into(),
            score: determinism_score,
            max: 3,
            note: format!("{} enumerated guarantees.", guarantees),
        },
        MaturityComponent {
            dimension: "ux_coverage".into(),
            score: ux_score,
            max: 2,
            note: format!("{} modeled UX flows.", flows),
        },
        MaturityComponent {
            dimension: "causal_graph_completeness".into(),
            score: causal_graph_completeness,
            max: 3,
            note: "Node/edge coverage vs reference trace; rebuild is deterministic.".into(),
        },
        MaturityComponent {
            dimension: "causal_query_completeness".into(),
            score: causal_query_completeness,
            max: 3,
            note: "query_causes/effects, trace_path stability, graph-based divergence.".into(),
        },
        MaturityComponent {
            dimension: "honesty_penalty".into(),
            score: honesty_penalty,
            max: 3,
            note: format!(
                "{} high-severity gaps self-reported by product_analyzer.",
                high_gaps
            ),
        },
    ];

    let rationale = format!(
        "Score = feature({}) + determinism({}) + ux({}) + causal_graph({}) + causal_query({}) − honesty_penalty({}), clamped to [0, {scale_max}].",
        feature_score,
        determinism_score,
        ux_score,
        causal_graph_completeness,
        causal_query_completeness,
        honesty_penalty
    );

    Maturity {
        score,
        scale_max,
        breakdown,
        rationale,
    }
}

fn scale(value: usize, max_input: usize, max_output: u8) -> u8 {
    if max_input == 0 {
        return 0;
    }
    let capped = value.min(max_input);
    // Integer arithmetic only, rounded down — avoids floating point drift.
    ((capped as u32 * max_output as u32) / max_input as u32) as u8
}

// ---------------------------------------------------------------------------
// Rendering.
// ---------------------------------------------------------------------------

/// Strip internal-only platform tokens from `repro eval` output (markdown + JSON).
pub fn sanitize_public_eval_output(s: &str) -> String {
    let mut t = s.to_string();
    for (from, to) in [
        ("cos_core", "platform_types"),
        ("cos_governance", "platform_governance"),
        ("cos_runtime", "platform_runtime"),
        ("cos_adapter", "integration_adapter"),
        ("CosKernelInterface", "KernelBridge"),
        ("StubCosKernel", "StubKernelBridge"),
    ] {
        t = t.replace(from, to);
    }
    t.replace("ExecutionArtifact", "run record")
}

pub fn render_markdown(r: &SystemReport) -> String {
    let mut s = String::new();

    s.push_str("# AION Repro — system evaluation\n\n");
    s.push_str("**Category.** ");
    s.push_str(&r.category_definition);
    s.push_str("\n\n");

    s.push_str("## 1. Feature inventory\n\n");
    for m in &r.feature_inventory {
        s.push_str(&format!("### `{}` ({} lines)\n\n", m.path, m.lines));
        render_list(&mut s, "Public functions", &m.pub_fns);
        render_list(&mut s, "Public structs", &m.pub_structs);
        render_list(&mut s, "Public enums", &m.pub_enums);
        render_list(&mut s, "Public traits", &m.pub_traits);
        s.push('\n');
    }

    s.push_str("## 2. Architecture map\n\n");
    s.push_str("| layer | module | purpose |\n");
    s.push_str("| --- | --- | --- |\n");
    for n in &r.architecture_map {
        s.push_str(&format!(
            "| {} | `{}` | {} |\n",
            n.layer, n.module, n.purpose
        ));
    }
    s.push('\n');

    s.push_str("## 3. Data flow\n\n");
    for d in &r.data_flow {
        s.push_str(&format!("- `{}` → `{}`  — {}\n", d.from, d.to, d.carries));
    }
    s.push('\n');

    s.push_str("## 4. Determinism guarantees\n\n");
    for g in &r.determinism_guarantees {
        s.push_str(&format!("- {}\n", g));
    }
    s.push('\n');

    s.push_str("## 5. UX flows\n\n");
    for f in &r.ux_flows {
        s.push_str(&format!("### `{}`\n\n", f.command));
        for (i, step) in f.steps.iter().enumerate() {
            s.push_str(&format!("{}. {}\n", i + 1, step));
        }
        s.push_str(&format!("\n_Determinism:_ {}\n\n", f.determinism_notes));
    }

    s.push_str("## 6. Missing capabilities (honest gaps)\n\n");
    for g in &r.missing_capabilities {
        s.push_str(&format!(
            "- **[{} / {}]** {} — {}\n",
            g.category.as_str(),
            g.severity.as_str(),
            g.title,
            g.description
        ));
    }
    s.push('\n');

    s.push_str("## 7. Competitive positioning\n\n");
    s.push_str("| peer | category | overlap | distinction |\n");
    s.push_str("| --- | --- | --- | --- |\n");
    for p in &r.competitive_positioning {
        s.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            p.name, p.category, p.overlap, p.distinction
        ));
    }
    s.push('\n');

    s.push_str("## 8. Category definition\n\n");
    s.push_str(&r.category_definition);
    s.push_str("\n\n");

    s.push_str("## 9. Maturity score\n\n");
    s.push_str(&format!(
        "**{}/{}**.\n\n",
        r.maturity.score, r.maturity.scale_max
    ));
    s.push_str("| dimension | score | max | note |\n");
    s.push_str("| --- | --- | --- | --- |\n");
    for c in &r.maturity.breakdown {
        s.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            c.dimension, c.score, c.max, c.note
        ));
    }
    s.push('\n');
    s.push_str(&format!("_Rationale._ {}\n", r.maturity.rationale));
    s.push('\n');

    s.push_str("## 10. OS exposure & CI compatibility\n\n");
    s.push_str(&format!(
        "- **OS exposure ratio** (whitelist / budget): {:.4} (cap `OS_EXPOSURE_RATIO` = {})\n",
        crate::core::execution_boundary::os_exposure_ratio(),
        crate::core::execution_boundary::OS_EXPOSURE_RATIO
    ));
    s.push_str("- **CI ledger**: `repro ci run`, `repro ci ingest <file|->`, `diff`, `why`, `list` — same run record shape as `repro run`.\n\n");

    s
}

fn render_list(s: &mut String, title: &str, items: &[String]) {
    if items.is_empty() {
        return;
    }
    s.push_str(&format!("_{}_:\n", title));
    for it in items {
        s.push_str(&format!("- `{}`\n", it));
    }
    s.push('\n');
}

pub fn render_json(r: &SystemReport) -> String {
    // `serde_json::to_string_pretty` with `preserve_order` keeps struct
    // field order intact, which is what makes the JSON deterministic.
    serde_json::to_string_pretty(r).unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluation_is_deterministic() {
        let a = evaluate();
        let b = evaluate();
        assert_eq!(a, b);
        assert_eq!(render_markdown(&a), render_markdown(&b));
        assert_eq!(render_json(&a), render_json(&b));
    }

    #[test]
    fn every_source_is_scanned() {
        let r = evaluate();
        let paths: Vec<&str> = r
            .feature_inventory
            .iter()
            .map(|m| m.path.as_str())
            .collect();
        assert!(paths.contains(&"src/core/report.rs"));
        assert!(paths.contains(&"src/core/identity.rs"));
        assert!(paths.contains(&"src/core/artifact.rs"));
        assert!(paths.contains(&"src/core/execution_boundary.rs"));
        assert!(paths.contains(&"src/core/causal_graph.rs"));
        assert!(paths.contains(&"src/core/causal_query.rs"));
        assert!(paths.contains(&"src/analysis/system_evaluation.rs"));
        assert!(paths.contains(&"src/engine/adapter.rs"));
    }

    #[test]
    fn maturity_score_is_within_scale() {
        let m = evaluate().maturity;
        assert!(m.score <= m.scale_max);
        assert_eq!(m.scale_max, 15);
    }
}
