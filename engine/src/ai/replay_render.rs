//! HTML / SVG presentation for [`super::replay::ReplayReport`].

use super::graph::CausalGraphV2;
use super::replay::ReplayReport;
use std::collections::{BTreeMap, BTreeSet};

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

fn token_table(title: &str, tokens: &[String]) -> String {
    let rows: String = tokens
        .iter()
        .enumerate()
        .map(|(i, t)| format!("<tr><td>{i}</td><td><code>{}</code></td></tr>", esc(t)))
        .collect();
    let t = esc(title);
    format!(
        r#"<h2>{t}</h2><table><thead><tr><th>#</th><th>token</th></tr></thead><tbody>{rows}</tbody></table>"#,
        t = t,
        rows = rows
    )
}

fn graph_pre(g: &CausalGraphV2) -> String {
    serde_json::to_string_pretty(g).unwrap_or_else(|_| "{}".into())
}

fn mismatch_rows(rep: &ReplayReport) -> String {
    let c = &rep.comparison;
    let rows = [
        ("tokens_equal", c.tokens_equal),
        ("trace_equal", c.trace_equal),
        ("events_equal", c.events_equal),
        ("graph_equal", c.graph_equal),
        ("why_equal", c.why_equal),
        ("drift_equal", c.drift_equal),
        ("capsule_equal", c.capsule_equal),
        ("evidence_equal", c.evidence_equal),
        ("model_equal", c.model_equal),
        ("prompt_equal", c.prompt_equal),
        ("seed_equal", c.seed_equal),
        ("determinism_equal", c.determinism_equal),
    ];
    rows.iter()
        .map(|(name, ok)| {
            format!(
                "<tr><td><code>{}</code></td><td>{}</td></tr>",
                esc(name),
                if *ok { "ok" } else { "MISMATCH" }
            )
        })
        .collect::<String>()
}

/// Standalone HTML: original vs replay tokens, graphs, mismatch table, drift + determinism.
pub fn render_replay_report_html(rep: &ReplayReport) -> String {
    let diff_lines = rep
        .comparison
        .differences
        .iter()
        .map(|d| format!("<li>{}</li>", esc(d)))
        .collect::<String>();
    let drift_rows = rep
        .drift_report
        .details
        .iter()
        .map(|d| format!("<li>{}</li>", esc(d)))
        .collect::<String>();
    let det = &rep.original_capsule.determinism;
    let det_html = format!(
        "<ul><li>time_frozen: {}</li><li>time_epoch_secs: {}</li><li>random_seed: 0x{:x}</li><li>syscall_intercept: {}</li></ul>",
        det.time_frozen,
        det.time_epoch_secs,
        det.random_seed,
        det.syscall_intercept
    );
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head><meta charset="utf-8"><title>AI Replay v2</title>
<style>
body{{font-family:system-ui,sans-serif;margin:2rem;line-height:1.45}}
table{{border-collapse:collapse;margin-bottom:1.5rem}}
td,th{{border:1px solid #ccc;padding:.35rem .5rem}}
pre{{background:#f6f6f6;padding:.75rem;white-space:pre-wrap}}
.success{{color:rgb(30,120,60)}} .fail{{color:rgb(160,40,40)}}
</style>
</head>
<body>
<h1>AI Replay v2</h1>
<p class="{}"><strong>success</strong> = {}</p>
<p>replay_timestamp = {} · replay_duration_ms = {} · replay_aion_version = <code>{}</code></p>
<h2>Comparison flags</h2>
<table><thead><tr><th>check</th><th>status</th></tr></thead><tbody>{}</tbody></table>
<h2>First differing token</h2><p>{}</p>
<h2>Warnings</h2><ul>{}</ul>
<h2>Differences</h2><ul>{}</ul>
<h2>Drift summary</h2>
<p>changed = {}</p><ul>{}</ul>
<h2>Determinism (original)</h2> {}
<h2>Original graph (canonical)</h2><pre>{}</pre>
<h2>Replay graph (canonical)</h2><pre>{}</pre>
{}
{}
</body></html>"#,
        if rep.success { "success" } else { "fail" },
        rep.success,
        rep.replay_timestamp,
        rep.replay_duration_ms,
        esc(&rep.replay_aion_version),
        mismatch_rows(rep),
        rep.first_differing_token
            .map(|i| i.to_string())
            .unwrap_or_else(|| "none".into()),
        rep.warnings
            .iter()
            .map(|w| format!("<li>{}</li>", esc(w)))
            .collect::<String>(),
        diff_lines,
        rep.drift_report.changed,
        drift_rows,
        det_html,
        esc(&graph_pre(&rep.original_capsule.graph)),
        esc(&graph_pre(&rep.replay_capsule.graph)),
        token_table("Original tokens", &rep.original_capsule.tokens),
        token_table("Replay tokens", &rep.replay_capsule.tokens),
    )
}

fn node_labels(g: &CausalGraphV2) -> BTreeMap<String, String> {
    g.nodes
        .iter()
        .map(|n| (n.id.clone(), n.label.clone()))
        .collect()
}

/// SVG graph diff: nodes that differ in label (or missing on one side) highlighted in red; match in green.
pub fn render_replay_graph_svg(rep: &ReplayReport) -> String {
    let o = node_labels(&rep.original_capsule.graph);
    let r = node_labels(&rep.replay_capsule.graph);
    let ids: Vec<String> = o
        .keys()
        .chain(r.keys())
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let w = 520u32;
    let h = 40u32 + ids.len().saturating_mul(20) as u32;
    let mut y = 32u32;
    let mut body = String::new();
    for id in &ids {
        let lo = o.get(id).map(String::as_str).unwrap_or("<absent>");
        let lr = r.get(id).map(String::as_str).unwrap_or("<absent>");
        let ok = lo == lr;
        let fill = if ok {
            "rgb(220,245,220)"
        } else {
            "rgb(255,220,220)"
        };
        body.push_str(&format!(
            r#"<rect x="10" y="{y}" width="500" height="16" fill="{fill}" stroke="rgb(60,60,60)"/>"#,
            y = y,
            fill = fill
        ));
        body.push_str(&format!(
            r#"<text x="16" y="{ty}" font-size="10" font-family="sans-serif">{lab}</text>"#,
            lab = esc(&format!("{id}: orig='{lo}' replay='{lr}'")),
            ty = y + 12,
        ));
        y += 20;
    }
    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}">
<rect width="100%" height="100%" fill="rgb(255,255,255)"/>
<text x="10" y="20" font-size="12" font-family="sans-serif">Graph node diff (label by id)</text>
{body}
</svg>"#,
        w = w,
        h = h,
        body = body
    )
}
