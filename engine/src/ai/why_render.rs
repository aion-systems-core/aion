//! HTML for [`super::why::WhyReportV2`] (embeds causal graph SVG).

use super::graph::CausalGraphV2;
use super::graph_render::render_causal_graph_svg;
use super::why::WhyDiff;
use super::why::{WhyNodeKind, WhyReportV2};

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

fn rows_prompt(why: &WhyReportV2) -> String {
    why.nodes
        .iter()
        .filter(|n| matches!(n.kind, WhyNodeKind::Prompt))
        .map(|n| {
            format!(
                "<tr><td><code>{}</code></td><td>{}</td><td>{}</td></tr>",
                esc(&n.id),
                n.position
                    .map(|p| p.to_string())
                    .unwrap_or_else(|| "-".into()),
                esc(&n.text)
            )
        })
        .collect()
}

fn rows_token(why: &WhyReportV2) -> String {
    why.nodes
        .iter()
        .filter(|n| matches!(n.kind, WhyNodeKind::Token))
        .map(|n| {
            format!(
                "<tr><td><code>{}</code></td><td>{}</td><td>{}</td></tr>",
                esc(&n.id),
                n.position
                    .map(|p| p.to_string())
                    .unwrap_or_else(|| "-".into()),
                esc(&n.text)
            )
        })
        .collect()
}

fn rows_seed_det(why: &WhyReportV2) -> String {
    why.nodes
        .iter()
        .filter(|n| matches!(n.kind, WhyNodeKind::Seed | WhyNodeKind::Determinism))
        .map(|n| {
            let k = match n.kind {
                WhyNodeKind::Prompt => "prompt",
                WhyNodeKind::Token => "token",
                WhyNodeKind::Seed => "seed",
                WhyNodeKind::Determinism => "determinism",
            };
            format!(
                "<tr><td><code>{}</code></td><td>{}</td><td>{}</td></tr>",
                esc(&n.id),
                k,
                esc(&n.text)
            )
        })
        .collect()
}

fn rows_edges(why: &WhyReportV2) -> String {
    let mut e = why.edges.clone();
    e.sort_by(|a, b| (&a.from, &a.to).cmp(&(&b.from, &b.to)));
    e.iter()
        .map(|ed| {
            format!(
                "<tr><td><code>{}</code></td><td><code>{}</code></td><td>{:.4}</td><td>{}</td></tr>",
                esc(&ed.from),
                esc(&ed.to),
                ed.weight,
                esc(&ed.reason)
            )
        })
        .collect()
}

/// Standalone HTML: influence tables, summary, embedded causal graph SVG.
pub fn render_why_report_html(why: &WhyReportV2, graph: &CausalGraphV2) -> String {
    let svg = render_causal_graph_svg(graph);
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head><meta charset="utf-8"><title>Why report v2</title>
<style>
body{{font-family:system-ui,sans-serif;margin:2rem;line-height:1.45}}
table{{border-collapse:collapse;margin-bottom:1.2rem}}
td,th{{border:1px solid #ccc;padding:.35rem .5rem;font-size:.9rem}}
h2{{font-size:1.1rem}}
.embed svg{{max-width:100%;height:auto;border:1px solid #ddd}}
</style>
</head>
<body>
<h1>Why report v2</h1>
<p><code>why_schema_version={}</code> · <code>model_version={}</code> · <code>seed={}</code></p>
<p>{}</p>
<h2>Prompt influence</h2>
<table><thead><tr><th>id</th><th>pos</th><th>text</th></tr></thead><tbody>{}</tbody></table>
<h2>Token influence</h2>
<table><thead><tr><th>id</th><th>pos</th><th>text</th></tr></thead><tbody>{}</tbody></table>
<h2>Seed / determinism</h2>
<table><thead><tr><th>id</th><th>kind</th><th>text</th></tr></thead><tbody>{}</tbody></table>
<h2>Edges</h2>
<table><thead><tr><th>from</th><th>to</th><th>weight</th><th>reason</th></tr></thead><tbody>{}</tbody></table>
<h2>Causal graph (SVG)</h2>
<div class="embed">{}</div>
</body></html>"#,
        esc(&why.why_schema_version),
        esc(&why.model_version),
        why.seed,
        esc(&why.summary),
        rows_prompt(why),
        rows_token(why),
        rows_seed_det(why),
        rows_edges(why),
        svg
    )
}

/// HTML for replay why diff.
pub fn render_why_diff_html(
    diff: &WhyDiff,
    original: &WhyReportV2,
    replay: &WhyReportV2,
) -> String {
    let nd = diff
        .node_diffs
        .iter()
        .map(|d| format!("<li>{}</li>", esc(d)))
        .collect::<String>();
    let ed = diff
        .edge_diffs
        .iter()
        .map(|d| format!("<li>{}</li>", esc(d)))
        .collect::<String>();
    let os = esc(&original.summary);
    let rs = esc(&replay.summary);
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head><meta charset="utf-8"><title>Why diff</title>
<style>body{{font-family:system-ui,sans-serif;margin:2rem}} ul{{margin:.5rem 0}}</style>
</head>
<body>
<h1>Why diff (replay)</h1>
<p><strong>changed</strong> = {}</p>
<h2>Node diffs</h2><ul>{}</ul>
<h2>Edge diffs</h2><ul>{}</ul>
<h2>Original summary</h2><p>{}</p>
<h2>Replay summary</h2><p>{}</p>
</body></html>"#,
        diff.changed, nd, ed, os, rs
    )
}

/// Minimal SVG listing diff lines.
pub fn render_why_diff_svg(diff: &WhyDiff) -> String {
    let mut y = 28i32;
    let mut lines = String::new();
    for d in diff
        .node_diffs
        .iter()
        .chain(diff.edge_diffs.iter())
        .take(24)
    {
        lines.push_str(&format!(
            r#"<text x="12" y="{y}" font-size="11" font-family="sans-serif">{t}</text>"#,
            t = esc(d),
            y = y,
        ));
        y += 16;
    }
    let h = y + 20;
    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="720" height="{h}" viewBox="0 0 720 {h}">
<rect width="100%" height="100%" fill="rgb(255,255,255)"/>
<text x="12" y="18" font-size="13" font-family="sans-serif">Why diff (changed={ch})</text>
{lines}
</svg>"#,
        h = h,
        ch = diff.changed,
        lines = lines
    )
}
