use repro::core::causal_graph::{build_causal_graph, CausalGraph};
use repro::core::causal_query::{
    first_divergent_causal_node, query_causes, query_effects, trace_path,
};
use repro::core::execution_trace::{ExecutionEvent, ExecutionTrace};

fn linear_three_node_graph() -> CausalGraph {
    let t = ExecutionTrace {
        run_id: "t".into(),
        events: vec![
            ExecutionEvent::Spawn {
                command: "a".into(),
            },
            ExecutionEvent::Stdout { chunk: "m".into() },
            ExecutionEvent::Stderr {
                chunk: String::new(),
            },
        ],
    };
    build_causal_graph(&t)
}

#[test]
fn query_causes_and_effects_match_incoming_outgoing_edges() {
    let g = linear_three_node_graph();
    let c = query_causes(&g, "n1");
    assert!(c.iter().any(|n| n.id == "n0"), "{c:?}");
    let e = query_effects(&g, "n1");
    assert!(e.iter().any(|n| n.id == "n2"), "{e:?}");
}

#[test]
fn trace_path_is_stable_across_repeated_calls() {
    let g = linear_three_node_graph();
    let p1 = trace_path(&g, "n0", "n2");
    let p2 = trace_path(&g, "n0", "n2");
    assert_eq!(p1, p2);
    assert_eq!(p1.len(), 3);
    assert_eq!(p1[0].id, "n0");
    assert_eq!(p1[2].id, "n2");
}

#[test]
fn first_divergent_causal_node_detects_stdout_payload_mismatch() {
    let ta = ExecutionTrace {
        run_id: "a".into(),
        events: vec![
            ExecutionEvent::Spawn {
                command: "x".into(),
            },
            ExecutionEvent::Stdout {
                chunk: "a\n".into(),
            },
            ExecutionEvent::Stderr {
                chunk: String::new(),
            },
            ExecutionEvent::Exit { code: 0 },
        ],
    };
    let mut tb = ta.clone();
    if let ExecutionEvent::Stdout { ref mut chunk } = tb.events[1] {
        *chunk = "b\n".into();
    }
    let ga = build_causal_graph(&ta);
    let gb = build_causal_graph(&tb);
    let n = first_divergent_causal_node(&ga, &gb).expect("divergence");
    assert_eq!(n.index, 1);
    assert_eq!(n.event_type, "Stdout");
}
