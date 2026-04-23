//! Drift between two deterministic capsules (SDK smoke).
fn main() {
    let a = aion_engine::sdk::build_capsule("m", "same prompt", 1);
    let b = aion_engine::sdk::build_capsule("m", "same prompt", 2);
    let d = aion_engine::sdk::drift_between(&a, &b);
    println!("drift changed = {}", d.changed);
}
