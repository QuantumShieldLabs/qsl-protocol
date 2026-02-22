mod common;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const ROUTE_TOKEN_PEER: &str = "route_token_peer_abcdefghijklmnopq";

fn safe_test_root() -> PathBuf {
    let root = if let Ok(v) = env::var("QSC_TEST_ROOT") {
        PathBuf::from(v)
    } else if let Ok(v) = env::var("CARGO_TARGET_DIR") {
        PathBuf::from(v)
    } else {
        PathBuf::from("target")
    };
    let root = root.join("qsc-test-tmp");
    ensure_dir_700(&root);
    root
}

fn ensure_dir_700(path: &Path) {
    if let Ok(meta) = fs::symlink_metadata(path) {
        if meta.is_file() {
            let _ = fs::remove_file(path);
        }
    }
    fs::create_dir_all(path).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
    }
}

fn combined_output(output: &std::process::Output) -> String {
    let mut combined = String::from_utf8_lossy(&output.stdout).to_string();
    combined.push_str(&String::from_utf8_lossy(&output.stderr));
    combined
}

fn relay_send(
    cfg: &Path,
    relay_addr: &str,
    scenario: &str,
    seed: &str,
    payload: &Path,
) -> std::process::Output {
    Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .env("QSC_SCENARIO", scenario)
        .env("QSC_SEED", seed)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            relay_addr,
            "--to",
            "peer",
            "--file",
            payload.to_str().unwrap(),
        ])
        .output()
        .expect("run send")
}

fn normalized_relay_events(output: &std::process::Output) -> Vec<String> {
    combined_output(output)
        .lines()
        .filter(|line| line.contains("event=relay_event"))
        .map(|line| line.trim().to_string())
        .collect()
}

fn init_cfg_with_peer_route_token(cfg: &Path) {
    common::init_mock_vault(cfg);
    let out = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .args([
            "contacts",
            "add",
            "--label",
            "peer",
            "--fp",
            "fp-test",
            "--route-token",
            ROUTE_TOKEN_PEER,
        ])
        .output()
        .expect("contacts add");
    assert!(out.status.success(), "{}", combined_output(&out));
}

#[test]
fn remote_scenario_happy_path_has_deliver_only() {
    let base = safe_test_root().join(format!("na0090_happy_{}", std::process::id()));
    ensure_dir_700(&base);
    let cfg = base.join("cfg");
    ensure_dir_700(&cfg);
    init_cfg_with_peer_route_token(&cfg);

    let payload = cfg.join("msg.bin");
    fs::write(&payload, b"hello").expect("write payload");

    let relay = common::start_inbox_server(1024 * 1024, 8);
    let relay_addr = relay.base_url().to_string();

    let output = relay_send(&cfg, relay_addr.as_str(), "happy-path", "1", &payload);

    if !output.status.success() {
        panic!("send failed: {}", combined_output(&output));
    }
    let combined = combined_output(&output);
    assert!(combined.contains("event=relay_event action=deliver"));
    assert!(!combined.contains("action=drop"));
    assert!(!combined.contains("action=reorder"));
    assert!(!combined.contains("action=dup"));
}

#[test]
fn remote_scenario_drop_reorder_emits_hostile_markers() {
    let base = safe_test_root().join(format!("na0090_drop_{}", std::process::id()));
    ensure_dir_700(&base);
    let cfg = base.join("cfg");
    ensure_dir_700(&cfg);
    init_cfg_with_peer_route_token(&cfg);

    let payload = cfg.join("msg.bin");
    fs::write(&payload, b"hello").expect("write payload");

    let relay = common::start_inbox_server(1024 * 1024, 8);
    let relay_addr = relay.base_url().to_string();

    let output = relay_send(&cfg, relay_addr.as_str(), "drop-reorder", "7", &payload);

    if !output.status.success() {
        panic!("send failed: {}", combined_output(&output));
    }
    let combined = combined_output(&output);
    assert!(combined.contains("event=relay_event action=reorder"));
    assert!(combined.contains("event=relay_event action=deliver"));
}

#[test]
fn remote_scenario_determinism_replay() {
    let base = safe_test_root().join(format!("na0090_replay_{}", std::process::id()));
    ensure_dir_700(&base);
    let cfg = base.join("cfg");
    ensure_dir_700(&cfg);
    init_cfg_with_peer_route_token(&cfg);

    let payload = cfg.join("msg.bin");
    fs::write(&payload, b"hello").expect("write payload");

    let relay = common::start_inbox_server(1024 * 1024, 8);
    let relay_addr = relay.base_url().to_string();

    let out1 = relay_send(&cfg, relay_addr.as_str(), "drop-reorder", "7", &payload);
    let out2 = relay_send(&cfg, relay_addr.as_str(), "drop-reorder", "7", &payload);

    if !out1.status.success() || !out2.status.success() {
        panic!(
            "send failed: {} || {}",
            combined_output(&out1),
            combined_output(&out2)
        );
    }

    let events1 = normalized_relay_events(&out1);
    let events2 = normalized_relay_events(&out2);
    assert_eq!(events1, events2, "relay_event lines must be deterministic");
    assert!(events1.iter().any(|l| l.contains("action=reorder")));
}
