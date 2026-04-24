//! Deterministic SVG for [`super::graph::CausalGraphV2`].

use super::graph::{CausalGraphV2, GraphNodeKind};
use std::collections::BTreeMap;

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

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}…", &s[..max.saturating_sub(1)])
    }
}

/// Deterministic layout: seed/determinism left, prompt column, token row with arrows.
pub fn render_causal_graph_svg(graph: &CausalGraphV2) -> String {
    let mut pos: BTreeMap<String, (i32, i32)> = BTreeMap::new();
    let mut py = 40i32;
    for n in &graph.nodes {
        let p = match n.kind {
            GraphNodeKind::Seed => (40, 40),
            GraphNodeKind::Determinism => (40, 120),
            GraphNodeKind::Prompt => {
                let p = (200, py);
                py += 28;
                p
            }
            GraphNodeKind::Token => {
                let idx =
                    n.id.strip_prefix("token_")
                        .and_then(|s| s.parse::<usize>().ok())
                        .unwrap_or(0);
                (360 + (idx as i32) * 100, 100)
            }
        };
        pos.insert(n.id.clone(), p);
    }

    let w = 1200i32;
    let h = 220i32.max(py + 80);
    let mut parts = vec![format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}">"#
    )];
    parts.push(r#"<defs><marker id="arr" markerWidth="8" markerHeight="8" refX="7" refY="4" orient="auto"><path d="M0,0 L8,4 L0,8 Z" fill="rgb(60,60,60)"/></marker></defs>"#.into());
    parts.push(r#"<rect width="100%" height="100%" fill="rgb(252,252,252)"/>"#.into());

    for n in &graph.nodes {
        let (x, y) = pos.get(&n.id).copied().unwrap_or((0, 0));
        let fill = match n.kind {
            GraphNodeKind::Prompt => "rgb(230,240,255)",
            GraphNodeKind::Token => "rgb(240,255,240)",
            GraphNodeKind::Seed => "rgb(255,245,230)",
            GraphNodeKind::Determinism => "rgb(245,245,245)",
        };
        parts.push("<g>".into());
        parts.push(format!(
            r#"<title>{}</title>"#,
            esc(&format!("{} — {}", n.id, n.label))
        ));
        parts.push(format!(
            r#"<rect x="{x}" y="{y}" width="88" height="22" rx="3" fill="{fill}" stroke="rgb(80,80,80)"/>"#,
            x = x,
            y = y,
            fill = fill
        ));
        parts.push(format!(
            r#"<text x="{tx}" y="{ty}" font-size="9" font-family="sans-serif">{lab}</text>"#,
            lab = esc(&truncate(&n.id, 14)),
            tx = x + 4,
            ty = y + 15,
        ));
        parts.push("</g>".into());
    }

    for e in &graph.edges {
        let (x1, y1) = pos.get(&e.from).copied().unwrap_or((0, 0));
        let (x2, y2) = pos.get(&e.to).copied().unwrap_or((0, 0));
        let cx1 = x1 + 88;
        let cy1 = y1 + 11;
        let cx2 = x2;
        let cy2 = y2 + 11;
        parts.push("<g>".into());
        parts.push(format!(
            r#"<title>{} → {}</title>"#,
            esc(&e.from),
            esc(&e.to)
        ));
        parts.push(format!(
            r#"<path d="M {cx1} {cy1} L {cx2} {cy2}" stroke="rgb(70,70,70)" fill="none" marker-end="url(#arr)"/>"#,
            cx1 = cx1,
            cy1 = cy1,
            cx2 = cx2,
            cy2 = cy2
        ));
        parts.push("</g>".into());
    }
    parts.push("</svg>".into());
    parts.join("\n")
}
