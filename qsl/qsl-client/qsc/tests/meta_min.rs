use assert_cmd::Command;
use quantumshield_refimpl::qse::EnvelopeProfile;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

mod common;
use common::start_inbox_server;

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

fn output_str(out: &std::process::Output) -> String {
    let mut s = String::new();
    s.push_str(&String::from_utf8_lossy(&out.stdout));
    s.push_str(&String::from_utf8_lossy(&out.stderr));
    s
}

fn assert_no_secrets(s: &str) {
    let needle = [
        "TOKEN",
        "SECRET",
        "KEY",
        "PASS",
        "PRIVATE",
        "BEARER",
        "CREDENTIAL",
    ];
    for n in needle.iter() {
        assert!(!s.contains(n), "unexpected secret marker: {n}");
    }
}

#[test]
fn poll_bounds_enforced() {
    let base = safe_test_root().join(format!("na0103_poll_bounds_{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let cfg = base.join("cfg");
    let out_dir = base.join("out");
    ensure_dir_700(&cfg);
    ensure_dir_700(&out_dir);
    let server = start_inbox_server(1024 * 1024, 4);

    let out = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_QSP_SEED", "1")
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--from",
            "bob",
            "--max",
            "1",
            "--out",
            out_dir.to_str().unwrap(),
            "--poll-interval-ms",
            "0",
            "--poll-ticks",
            "1",
            "--poll-max-per-tick",
            "1",
        ])
        .output()
        .expect("receive");
    assert!(!out.status.success());
    let s = output_str(&out);
    assert!(s.contains("meta_poll_invalid"));
    assert!(fs::read_dir(&out_dir).unwrap().next().is_none());
    assert_no_secrets(&s);
}

#[test]
fn poll_deterministic_schedule_headless() {
    let base = safe_test_root().join(format!("na0103_poll_determinism_{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let cfg = base.join("cfg");
    let out_dir = base.join("out");
    ensure_dir_700(&cfg);
    ensure_dir_700(&out_dir);
    let server = start_inbox_server(1024 * 1024, 4);

    let run = |suffix: &str| {
        let out = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
            .env("QSC_CONFIG_DIR", &cfg)
            .env("QSC_QSP_SEED", "1")
            .args([
                "receive",
                "--transport",
                "relay",
                "--relay",
                server.base_url(),
                "--from",
                "bob",
                "--max",
                "1",
                "--out",
                out_dir.to_str().unwrap(),
                "--poll-interval-ms",
                "1",
                "--poll-ticks",
                "2",
                "--poll-max-per-tick",
                "1",
                "--meta-seed",
                "7",
            ])
            .output()
            .expect("receive");
        let s = output_str(&out);
        let filtered: Vec<_> = s
            .lines()
            .filter(|l| l.contains("meta_poll_config") || l.contains("meta_poll_tick"))
            .map(|l| l.to_string())
            .collect();
        (suffix.to_string(), filtered.join("\n"))
    };

    let (_, first) = run("first");
    let (_, second) = run("second");
    assert_eq!(first, second);
}

#[test]
fn pad_bucket_applied_on_wire() {
    let base = safe_test_root().join(format!("na0103_pad_bucket_{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let cfg = base.join("cfg");
    ensure_dir_700(&cfg);
    let server = start_inbox_server(1024 * 1024, 8);

    let payload = base.join("msg.txt");
    fs::write(&payload, b"hello").unwrap();

    let out = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_QSP_SEED", "1")
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "bob",
            "--file",
            payload.to_str().unwrap(),
            "--pad-bucket",
            "enhanced",
            "--meta-seed",
            "3",
        ])
        .output()
        .expect("send");
    assert!(out.status.success());
    let s = output_str(&out);
    assert!(s.contains("meta_pad"));
    assert!(s.contains("bucket=enhanced"));
    assert_no_secrets(&s);

    let items = server.drain_channel("bob");
    assert!(!items.is_empty());
    let min_len = EnvelopeProfile::Enhanced.min_size_bytes();
    assert!(items[0].len() >= min_len, "expected padding to enhanced");
}

#[test]
fn pad_invalid_rejects_no_mutation() {
    let base = safe_test_root().join(format!("na0103_pad_invalid_{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let cfg = base.join("cfg");
    ensure_dir_700(&cfg);
    let payload = base.join("msg.txt");
    fs::write(&payload, b"hello").unwrap();

    let out = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_QSP_SEED", "1")
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            "http://127.0.0.1:9",
            "--to",
            "bob",
            "--file",
            payload.to_str().unwrap(),
            "--pad-to",
            "70000",
        ])
        .output()
        .expect("send");
    assert!(!out.status.success());
    let s = output_str(&out);
    assert!(s.contains("meta_pad_invalid"));
    assert!(!cfg.join("outbox.json").exists());
    assert_no_secrets(&s);
}
