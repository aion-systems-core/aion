use aion_cli::output_bundle::{write_graph_output, GraphFormat};

#[test]
fn test_graph_dot_output_and_depth() {
    let run_json = r#"{"run_id":"phase3-graph-test"}"#;
    let out = write_graph_output(run_json, GraphFormat::Dot, Some(3)).expect("graph output");
    assert!(out.join("result.json").exists());
    assert!(out.join("result.dot").exists());
}
