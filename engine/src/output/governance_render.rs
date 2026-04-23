//! HTML / SVG for [`crate::governance::validate::GovernanceReport`].

use crate::governance::validate::GovernanceReport;

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

fn ul_lines(lines: &[String]) -> String {
    if lines.is_empty() {
        return "<li>(none)</li>".into();
    }
    lines
        .iter()
        .map(|m| format!("<li>{}</li>", esc(m)))
        .collect()
}

pub fn render_governance_report_html(report: &GovernanceReport) -> String {
    let pol_ok = if report.policy.ok { "ok" } else { "violations" };
    let det_ok = if report.determinism.ok { "ok" } else { "violations" };
    let int_ok = if report.integrity.ok { "ok" } else { "violations" };
    let sum = if report.success {
        "Governance validation succeeded."
    } else {
        "Governance validation failed; see violations below."
    };
    let ci_block = match &report.ci {
        None => "<h2>CI baseline comparison</h2><p>(not applicable — run <code>aion ci check</code> for baseline comparison)</p>".into(),
        Some(c) => format!(
            r#"<h2>CI baseline comparison</h2>
<p>baseline: <code>{}</code></p>
<ul>
<li>drift.changed = {} ({})</li>
<li>replay_success = {}</li>
<li>ci.policy_ok = {} · ci.determinism_ok = {} · ci.integrity_ok = {}</li>
<li>ci.success = {}</li>
</ul>"#,
            esc(&c.baseline_name),
            c.drift_changed,
            esc(&c.drift_fields.join(", ")),
            c.replay_success,
            c.ci_policy_ok,
            c.ci_determinism_ok,
            c.ci_integrity_ok,
            c.ci_success,
        ),
    };
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head><meta charset="utf-8"><title>Governance report</title>
<style>
body{{font-family:system-ui,sans-serif;margin:2rem;line-height:1.45}}
.ok{{color:rgb(20,110,60)}} .bad{{color:rgb(160,40,40)}}
ul{{margin:.4rem 0}}
</style>
</head>
<body>
<h1>Governance v1</h1>
<p class="{}"><strong>{}</strong></p>
<h2>Summary</h2><p>{}</p>
<h2>Policy <span class="{}">{}</span></h2><ul>{}</ul>
<h2>Determinism <span class="{}">{}</span></h2><ul>{}</ul>
<h2>Integrity <span class="{}">{}</span></h2><ul>{}</ul>
{}
</body></html>"#,
        if report.success { "ok" } else { "bad" },
        if report.success { "SUCCESS" } else { "FAILURE" },
        esc(sum),
        if report.policy.ok { "ok" } else { "bad" },
        pol_ok,
        ul_lines(&report.policy.messages),
        if report.determinism.ok { "ok" } else { "bad" },
        det_ok,
        ul_lines(&report.determinism.messages),
        if report.integrity.ok { "ok" } else { "bad" },
        int_ok,
        ul_lines(&report.integrity.messages),
        ci_block,
    )
}

pub fn render_governance_graph_svg(report: &GovernanceReport) -> String {
    let node = |_id: &str, label: &str, x: i32, y: i32, ok: bool| {
        let fill = if ok {
            "rgb(220,245,220)"
        } else {
            "rgb(255,220,220)"
        };
        let stroke = if ok { "rgb(40,120,60)" } else { "rgb(180,40,40)" };
        format!(
            r#"<g><title>{title}</title>
<rect x="{x}" y="{y}" width="120" height="36" rx="4" fill="{fill}" stroke="{stroke}"/>
<text x="{tx}" y="{ty}" font-size="11" font-family="sans-serif">{lab}</text></g>"#,
            title = esc(label),
            x = x,
            y = y,
            fill = fill,
            stroke = stroke,
            tx = x + 8,
            ty = y + 22,
            lab = esc(label),
        )
    };
    let cx = 200;
    let cy = 120;
    let hub_ok = report.success;
    let hub_fill = if hub_ok {
        "rgb(230,240,255)"
    } else {
        "rgb(255,230,230)"
    };
    let mut edges = String::new();
    let pts = [
        ("policy", 40, 40, report.policy.ok),
        ("determinism", 280, 40, report.determinism.ok),
        ("integrity", 160, 200, report.integrity.ok),
    ];
    for (name, x, y, ok) in pts {
        let stroke = if ok { "rgb(90,90,90)" } else { "rgb(200,60,60)" };
        edges.push_str(&format!(
            r#"<path d="M {mx} {my} L {cx} {cy}" stroke="{stroke}" stroke-width="2" fill="none" marker-end="url(#arr)"><title>{name} → outcome</title></path>"#,
            mx = x + 60,
            my = y + 18,
            cx = cx,
            cy = cy,
            stroke = stroke,
            name = name,
        ));
    }
    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="400" height="260" viewBox="0 0 400 260">
<defs><marker id="arr" markerWidth="8" markerHeight="8" refX="6" refY="4" orient="auto"><path d="M0,0 L8,4 L0,8 z" fill="rgb(80,80,80)"/></marker></defs>
<rect width="100%" height="100%" fill="rgb(252,252,252)"/>
<text x="12" y="20" font-size="13" font-family="sans-serif">Governance pillars (edges = influence on outcome)</text>
{}
{}
{}
<g><title>combined outcome</title>
<rect x="{hx}" y="{hy}" width="100" height="40" rx="4" fill="{hub_fill}" stroke="rgb(60,60,120)"/>
<text x="{tx}" y="{ty}" font-size="11" font-family="sans-serif">outcome</text></g>
{edges}
</svg>"#,
        node("policy", "policy", 40, 40, report.policy.ok),
        node("determinism", "determinism", 280, 40, report.determinism.ok),
        node("integrity", "integrity", 160, 200, report.integrity.ok),
        hx = cx - 50,
        hy = cy - 20,
        hub_fill = hub_fill,
        tx = cx - 28,
        ty = cy + 5,
        edges = edges,
    )
}
