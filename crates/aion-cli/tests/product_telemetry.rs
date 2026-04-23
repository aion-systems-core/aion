use aion_cli::output_bundle;
use std::path::PathBuf;
use std::sync::Mutex;

static ENV_LOCK: Mutex<()> = Mutex::new(());

fn temp_home() -> PathBuf {
    let p = std::env::temp_dir().join(format!(
        "aion-cli-telemetry-test-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()
    ));
    let _ = std::fs::create_dir_all(&p);
    p
}

#[test]
fn test_telemetry_enable_disable_status() {
    let _g = ENV_LOCK.lock().expect("env lock");
    let home = temp_home();
    std::env::set_var("HOME", &home);
    std::env::set_var("USERPROFILE", &home);

    let _ = output_bundle::write_product_telemetry_disable_output().expect("disable");
    let _ = output_bundle::write_product_telemetry_status_output().expect("status after disable");
    let p = home.join(".aion").join("telemetry.toml");
    let body = std::fs::read_to_string(&p).expect("read telemetry file");
    assert!(body.contains("enabled = false"));

    let _ = output_bundle::write_product_telemetry_enable_output().expect("enable");
    let _ = output_bundle::write_product_telemetry_status_output().expect("status after enable");
    let body2 = std::fs::read_to_string(&p).expect("read telemetry file");
    assert!(body2.contains("enabled = true"));
}
