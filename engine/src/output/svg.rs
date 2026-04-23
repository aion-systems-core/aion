//! Minimal deterministic SVG (no external fonts).

use aion_core::DriftReport;
use crate::events::EventStreamFile;
use crate::graph::CausalGraph;
use serde_json::Value;

fn xml_esc(s: &str) -> String {
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

fn graph_from_value(v: &Value) -> Option<(Vec<(String, String, usize)>, Vec<(String, String, String)>)> {
    let nodes = v.get("nodes")?.as_array()?;
    let edges = v.get("edges")?.as_array()?;
    let mut ns: Vec<(String, String, usize)> = Vec::new();
    for n in nodes {
        let id = n.get("id")?.as_str()?.to_string();
        let et = n.get("event_type")?.as_str()?.to_string();
        let idx = n.get("index")?.as_u64()? as usize;
        ns.push((id, et, idx));
    }
    ns.sort_by(|a, b| a.0.cmp(&b.0).then(a.2.cmp(&b.2)));
    let mut es: Vec<(String, String, String)> = Vec::new();
    for e in edges {
        let from = e.get("from")?.as_str()?.to_string();
        let to = e.get("to")?.as_str()?.to_string();
        let rel = e.get("relation")?.as_str()?.to_string();
        es.push((from, to, rel));
    }
    es.sort();
    Some((ns, es))
}

/// Render causal graph as a simple left-to-right layout (deterministic ordering).
pub fn render_graph_svg(graph_json: &Value) -> String {
    let (nodes, edges) = match graph_from_value(graph_json) {
        Some(x) => x,
        None => (Vec::new(), Vec::new()),
    };
    let w = 720u32;
    let h = 120u32.max(80 + (nodes.len() as u32).saturating_mul(36));
    let mut parts = vec![format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}">"#
    )];
    parts.push(
        r#"<rect x="0" y="0" width="100%" height="100%" fill="rgb(250,250,250)"/>"#.to_string(),
    );
    let n = nodes.len().max(1);
    for (i, (id, et, _)) in nodes.iter().enumerate() {
        let y = 40 + (i as f32 * ((h - 80) as f32 / n as f32)).max(0.0) as i32;
        let x = 40;
        parts.push(format!(
            r#"<rect x="{x}" y="{y}" rx="4" ry="4" width="200" height="28" fill="rgb(255,255,255)" stroke="rgb(51,51,51)"/>"#
        ));
        parts.push(format!(
            r#"<text x="{tx}" y="{ty}" font-size="12" font-family="sans-serif">{lab}</text>"#,
            tx = x + 8,
            ty = y + 19,
            lab = xml_esc(&format!("{id} · {et}"))
        ));
    }
    for (i, (from, to, rel)) in edges.iter().enumerate() {
        let y1 = 54 + (i * 14) as i32 % (h as i32 - 60);
        parts.push(format!(
            r#"<path d="M 260 {y1} L 420 {y1}" stroke="rgb(136,136,136)" fill="none" marker-end="url(#m)"/>"#,
            y1 = y1
        ));
        parts.push(format!(
            r#"<text x="300" y="{ty}" font-size="11" font-family="sans-serif">{t}</text>"#,
            ty = y1 - 4,
            t = xml_esc(&format!("{from}→{to} ({rel})"))
        ));
    }
    parts.push(
        r#"<defs><marker id="m" markerWidth="6" markerHeight="6" refX="5" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="rgb(136,136,136)"/></marker></defs>"#
            .to_string(),
    );
    parts.push("</svg>".into());
    parts.join("\n")
}

pub fn render_graph_svg_struct(g: &CausalGraph) -> String {
    let v = serde_json::to_value(g).unwrap_or_else(|_| Value::Null);
    render_graph_svg(&v)
}

/// Timeline of event categories by sequence.
pub fn render_trace_svg(event_stream: &EventStreamFile) -> String {
    let w = 720u32;
    let h = 160u32;
    let mut parts = vec![format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}">"#
    )];
    parts.push(
        r#"<rect x="0" y="0" width="100%" height="100%" fill="rgb(250,250,250)"/>"#.to_string(),
    );
    let max = event_stream.events.len().max(1);
    for e in &event_stream.events {
        let x = 20 + ((e.seq as f64 / max as f64) * (w as f64 - 40.0)) as i32;
        let cat = format!("{:?}", e.event.category());
        let fill = match cat.as_str() {
            "Process" => "rgb(74,144,217)",
            "Exit" => "rgb(217,74,74)",
            "Env" => "rgb(126,211,33)",
            _ => "rgb(136,136,136)",
        };
        parts.push(format!(
            r#"<rect x="{x}" y="60" width="6" height="40" fill="{fill}" stroke="rgb(34,34,34)"/>"#
        ));
    }
    parts.push(format!(
        r#"<text x="12" y="24" font-size="13" font-family="sans-serif">{}</text>"#,
        xml_esc(&format!("run_id {}", event_stream.run_id))
    ));
    parts.push("</svg>".into());
    parts.join("\n")
}

pub fn render_drift_svg(drift: &DriftReport) -> String {
    let w = 480u32;
    let h = 120u32;
    let bar_w = if drift.changed { 360 } else { 0 };
    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}">
<rect width="100%" height="100%" fill="rgb(250,250,250)"/>
<rect x="40" y="50" width="360" height="24" fill="rgb(238,238,238)" stroke="rgb(51,51,51)"/>
<rect x="40" y="50" width="{bar_w}" height="24" fill="{fill}" stroke="rgb(51,51,51)"/>
<text x="40" y="40" font-size="13" font-family="sans-serif">{label}</text>
</svg>"#,
        w = w,
        h = h,
        bar_w = bar_w,
        fill = if drift.changed {
            "rgb(192,57,43)"
        } else {
            "rgb(39,174,96)"
        },
        label = xml_esc(&format!(
            "changed={} fields={}",
            drift.changed,
            drift.fields.join(", ")
        ))
    )
}
