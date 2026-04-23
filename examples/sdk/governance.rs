//! Validate a capsule against example governance profiles (SDK smoke).
use std::path::PathBuf;

fn example_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/governance")
        .join(name)
}

fn main() {
    let cap = aion_engine::sdk::build_capsule("demo", "gov example", 3);
    let pol = aion_engine::governance::load_policy(&example_path("dev.policy.json"))
        .expect("load policy");
    let det = aion_engine::governance::load_determinism(&example_path("dev.determinism.json"))
        .expect("load determinism");
    let integ = aion_engine::governance::load_integrity(&example_path("dev.integrity.json"))
        .expect("load integrity");
    let rep = aion_engine::sdk::validate_capsule(&cap, &pol, &det, &integ);
    println!("governance success = {}", rep.success);
}
