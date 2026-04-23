use aion_engine::ai::{
    ai_causal_graph_v2, build_ai_capsule_v1, explain_capsule, replay_ai_capsule, why_diff,
    WhyNodeKind,
};

fn graph_is_acyclic(g: &aion_engine::ai::CausalGraphV2) -> bool {
    use std::collections::{HashMap, HashSet};
    let mut adj: HashMap<String, Vec<String>> = HashMap::new();
    for e in &g.edges {
        adj.entry(e.from.clone()).or_default().push(e.to.clone());
    }
    let mut perm = HashSet::new();
    let mut temp = HashSet::new();
    fn dfs(
        u: &str,
        adj: &HashMap<String, Vec<String>>,
        temp: &mut HashSet<String>,
        perm: &mut HashSet<String>,
    ) -> bool {
        if perm.contains(u) {
            return true;
        }
        if temp.contains(u) {
            return false;
        }
        temp.insert(u.to_string());
        for v in adj.get(u).into_iter().flatten() {
            if !dfs(v, adj, temp, perm) {
                return false;
            }
        }
        temp.remove(u);
        perm.insert(u.to_string());
        true
    }
    for n in &g.nodes {
        if !perm.contains(&n.id) && !dfs(&n.id, &adj, &mut temp, &mut perm) {
            return false;
        }
    }
    true
}

#[test]
fn test_why_structure() {
    let cap = build_ai_capsule_v1("model-x".into(), "one two".into(), 99);
    let w = &cap.why;
    let seg_count = cap.prompt.split_whitespace().count();
    let expected_prompt_nodes = if seg_count == 0 { 1 } else { seg_count };
    let prompt_nodes = w
        .nodes
        .iter()
        .filter(|n| n.kind == WhyNodeKind::Prompt)
        .count();
    assert_eq!(
        prompt_nodes, expected_prompt_nodes,
        "prompt segments → prompt nodes (empty prompt uses prompt_0)"
    );

    let token_nodes = w
        .nodes
        .iter()
        .filter(|n| n.kind == WhyNodeKind::Token)
        .count();
    assert_eq!(token_nodes, cap.tokens.len(), "one why node per token");

    assert_eq!(
        w.nodes
            .iter()
            .filter(|n| n.kind == WhyNodeKind::Seed)
            .count(),
        1
    );
    assert_eq!(
        w.nodes
            .iter()
            .filter(|n| n.kind == WhyNodeKind::Determinism)
            .count(),
        1
    );

    if !cap.tokens.is_empty() {
        let prompt_ids: Vec<String> = if cap.prompt.split_whitespace().next().is_none() {
            vec!["prompt_0".into()]
        } else {
            (0..seg_count).map(|i| format!("prompt_{i}")).collect()
        };
        for pid in &prompt_ids {
            assert!(
                w.edges.iter().any(|e| e.from == *pid && e.to == "token_0"),
                "prompt → token_0: {pid}"
            );
        }
        assert!(w.edges.iter().any(|e| e.from == "seed" && e.to == "token_0"));
        assert!(
            w
                .edges
                .iter()
                .any(|e| e.from == "determinism" && e.to == "token_0")
        );
        for i in 0..cap.tokens.len().saturating_sub(1) {
            let from = format!("token_{i}");
            let to = format!("token_{}", i + 1);
            assert!(
                w.edges.iter().any(|e| e.from == from && e.to == to),
                "token chain {from} → {to}"
            );
        }
    }

    let bundle = explain_capsule(&cap);
    assert_eq!(&bundle.why, w);
    assert_eq!(bundle.graph, cap.graph);
    assert_eq!(w.why_schema_version, "2");
    assert_eq!(w.model_version, cap.model);
    assert_eq!(w.seed, cap.seed);
    assert!(!w.determinism_profile.is_empty());
}

#[test]
fn test_graph_structure() {
    let cap = build_ai_capsule_v1("m".into(), "hello world".into(), 3);
    let g1 = ai_causal_graph_v2(&cap);
    let g2 = ai_causal_graph_v2(&cap);
    assert_eq!(g1, g2, "same capsule → identical graph");
    assert!(graph_is_acyclic(&g1), "causal graph must be acyclic");
    assert_eq!(g1.nodes.len(), cap.graph.nodes.len());
    assert_eq!(g1.edges.len(), cap.graph.edges.len());
}

#[test]
fn test_why_diff() {
    let a = build_ai_capsule_v1("m".into(), "z".into(), 1);
    let b = build_ai_capsule_v1("m".into(), "z".into(), 2);
    let same = why_diff(&a.why, &a.why);
    assert!(!same.changed, "identical why → no diff");

    let cross = why_diff(&a.why, &b.why);
    assert!(cross.changed, "different seeds → token / edge diff");

    let rep = replay_ai_capsule(&a);
    let replay_why = why_diff(&rep.original_capsule.why, &rep.replay_capsule.why);
    assert!(
        !replay_why.changed,
        "replay reconstruction should match why: {:?}",
        replay_why
    );
}
