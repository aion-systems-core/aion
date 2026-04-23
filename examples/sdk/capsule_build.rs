//! Build a deterministic AI capsule and print JSON (SDK smoke).
fn main() {
    let c = aion_engine::sdk::build_capsule("demo", "sdk capsule example", 42);
    println!("{}", serde_json::to_string_pretty(&c).expect("serialize capsule"));
}
