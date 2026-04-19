use repro::core::causal_graph::{build_causal_graph, format_causal_graph_text};
use repro::core::causal_query::first_divergent_causal_node;
use repro::core::diff::diff_graph;
use repro::core::execution_trace::{ExecutionEvent, ExecutionTrace};

fn sample_trace(stdout: &str) -> ExecutionTrace {
    ExecutionTrace {
        run_id: "t".into(),
        events: vec![
            ExecutionEvent::Spawn {
                command: "cmd".into(),
            },
            ExecutionEvent::EnvResolved {
                keys: vec!["PATH=/p".into()],
            },
            ExecutionEvent::Stdout {
                chunk: stdout.into(),
            },
            ExecutionEvent::Stderr {
                chunk: String::new(),
            },
            ExecutionEvent::Exit { code: 0 },
            ExecutionEvent::Timing { duration_ms: 0 },
        ],
    }
}

#[test]
fn two_identical_traces_yield_byte_identical_graph_text() {
    let t = sample_trace("hi\n");
    let g1 = build_causal_graph(&t);
    let g2 = build_causal_graph(&t);
    let a = format_causal_graph_text(&g1);
    let b = format_causal_graph_text(&g2);
    assert_eq!(a, b);
    assert_eq!(diff_graph(&g1, &g2), "identical causal graphs\n");
}

#[test]
fn edge_i_connects_event_i_to_i_plus_one_with_expected_relation() {
    let t = sample_trace("out\n");
    let g = build_causal_graph(&t);
    assert_eq!(g.edges.len(), t.events.len() - 1);
    for i in 0..g.edges.len() {
        let e = &g.edges[i];
        assert_eq!(e.from, format!("n{i}"));
        assert_eq!(e.to, format!("n{}", i + 1));
        let from_ev = &t.events[i];
        let to_ev = &t.events[i + 1];
        let expected = match (from_ev, to_ev) {
            (ExecutionEvent::Spawn { .. }, ExecutionEvent::EnvResolved { .. }) => "next",
            (ExecutionEvent::EnvResolved { .. }, ExecutionEvent::Stdout { .. }) => "next",
            (ExecutionEvent::Stdout { .. }, ExecutionEvent::Stderr { .. }) => "next",
            (ExecutionEvent::Stderr { .. }, ExecutionEvent::Exit { .. }) => "next",
            (ExecutionEvent::Exit { .. }, ExecutionEvent::Timing { .. }) => "terminates",
            _ => panic!("unexpected pair at {i}: {from_ev:?} -> {to_ev:?}"),
        };
        assert_eq!(e.relation, expected);
    }

    let env_then_spawn = ExecutionTrace {
        run_id: "env-order".into(),
        events: vec![
            ExecutionEvent::EnvResolved {
                keys: vec!["PATH=/z".into()],
            },
            ExecutionEvent::Spawn {
                command: "sh".into(),
            },
        ],
    };
    let ge = build_causal_graph(&env_then_spawn);
    assert_eq!(ge.edges[0].relation, "enables");
}

#[test]
fn stdout_mismatch_maps_to_stdout_event_index() {
    let ta = sample_trace("a\n");
    let tb = sample_trace("b\n");
    let ga = build_causal_graph(&ta);
    let gb = build_causal_graph(&tb);
    let n = first_divergent_causal_node(&ga, &gb).expect("divergence");
    assert_eq!(n.index, 2);
}
