//! Replay a freshly built capsule (SDK smoke).
fn main() {
    let c = aion_engine::sdk::build_capsule("demo", "replay example", 7);
    let r = aion_engine::sdk::replay_capsule(&c);
    println!("replay success = {}", r.success);
}
