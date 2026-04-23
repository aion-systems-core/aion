//! Standalone HTML reports (inline CSS, deterministic markup).

use aion_core::{DriftReport, RunResult, WhyReport};
use aion_kernel::IntegrityReport;
use serde_json::Value;

fn esc(s: &str) -> String {
    let mut o = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => o.push_str("&amp;"),
            '<' => o.push_str("&lt;"),
            '>' => o.push_str("&gt;"),
            '"' => o.push_str("&quot;"),
            _ => o.push(c),
        }
    }
    o
}

fn shell(title: &str, body: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>{title}</title>
<style>
body{{font-family:system-ui,-apple-system,sans-serif;margin:2rem;line-height:1.45;color:#111}}
h1{{font-size:1.35rem;margin:0 0 1rem}}
table{{border-collapse:collapse;width:100%;max-width:56rem}}
th,td{{border:1px solid #ccc;padding:.4rem .55rem;text-align:left;vertical-align:top}}
th{{background:#f0f0f0}}
code,pre{{font-family:ui-monospace,monospace;font-size:.88rem;background:#f6f6f6;padding:.15rem .35rem}}
pre{{white-space:pre-wrap;word-break:break-word;padding:.75rem}}
.muted{{color:#555;font-size:.9rem}}
</style>
</head>
<body>
<h1>{title}</h1>
{body}
</body>
</html>"#,
        title = esc(title),
        body = body
    )
}

pub fn render_run_report(run: &RunResult) -> String {
    let body = format!(
        r#"<p class="muted">run_id <code>{rid}</code> · exit {exit} · {dur} ms</p>
<table>
<tr><th>command</th><td><code>{cmd}</code></td></tr>
<tr><th>cwd</th><td><code>{cwd}</code></td></tr>
<tr><th>timestamp</th><td>{ts}</td></tr>
<tr><th>env_fingerprint</th><td><code>{env}</code></td></tr>
</table>
<h2>stdout</h2><pre>{so}</pre>
<h2>stderr</h2><pre>{se}</pre>"#,
        rid = esc(&run.run_id),
        exit = run.exit_code,
        dur = run.duration_ms,
        cmd = esc(&run.command),
        cwd = esc(&run.cwd),
        ts = run.timestamp,
        env = esc(&run.env_fingerprint),
        so = esc(&run.stdout),
        se = esc(&run.stderr),
    );
    shell("Run report", &body)
}

pub fn render_why_report(report: &WhyReport) -> String {
    let ff = match &report.first_divergent_field {
        Some(f) => format!("<code>{}</code>", esc(f)),
        None => r#"<span class="muted">none</span>"#.to_string(),
    };
    let sug = report
        .suggestion
        .as_deref()
        .map(|s| format!("<p><strong>Suggestion</strong> {}</p>", esc(s)))
        .unwrap_or_default();
    let body = format!(
        r#"<p>{summary}</p>
<p><strong>First divergent field</strong> {ff}</p>
{sug}"#,
        summary = esc(&report.summary),
        ff = ff,
        sug = sug
    );
    shell("Why report", &body)
}

pub fn render_graph_report(graph_json: &Value) -> String {
    let pretty = serde_json::to_string_pretty(graph_json).unwrap_or_else(|_| "{}".into());
    let body = format!(r#"<pre>{}</pre>"#, esc(&pretty));
    shell("Graph report", &body)
}

pub fn render_drift_report(drift: &DriftReport) -> String {
    let fields = drift
        .fields
        .iter()
        .map(|f| format!("<li><code>{}</code></li>", esc(f)))
        .collect::<String>();
    let details = drift
        .details
        .iter()
        .map(|d| format!("<li>{}</li>", esc(d)))
        .collect::<String>();
    let body = format!(
        r#"<p><strong>Changed</strong>: {ch}</p>
<h2>Fields</h2><ul>{fields}</ul>
<h2>Details</h2><ul>{details}</ul>"#,
        ch = drift.changed,
        fields = fields,
        details = details
    );
    shell("Drift report", &body)
}

pub fn render_integrity_report(report: &IntegrityReport) -> String {
    let rows = report
        .rule_outcomes
        .iter()
        .map(|r| {
            format!(
                "<tr><td><code>{}</code></td><td>{}</td><td>{}</td></tr>",
                esc(&r.rule_id),
                if r.passed { "pass" } else { "fail" },
                esc(&r.detail)
            )
        })
        .collect::<String>();
    let body = format!(
        r#"<p><strong>kernel_build_hash</strong> <code>{h}</code></p>
<p><strong>evidence_root</strong> <code>{e}</code></p>
<table><thead><tr><th>rule</th><th>outcome</th><th>detail</th></tr></thead><tbody>{rows}</tbody></table>"#,
        h = esc(&report.kernel_build_hash),
        e = esc(&report.evidence_root),
        rows = rows
    );
    shell("Integrity report", &body)
}

pub fn render_replay_report(stdout: &str) -> String {
    let body = format!(r#"<h2>stdout</h2><pre>{}</pre>"#, esc(stdout));
    shell("Replay report", &body)
}

pub fn render_json_value(title: &str, v: &Value) -> String {
    let pretty = serde_json::to_string_pretty(v).unwrap_or_else(|_| "{}".into());
    let body = format!(r#"<pre>{}</pre>"#, esc(&pretty));
    shell(title, &body)
}
