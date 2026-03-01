use assert_cmd::Command;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

mod common;
use common::start_inbox_server;

const ROUTE_TOKEN_BOB: &str = "route_token_bob_abcdefghijklmnopqr";

fn test_root() -> PathBuf {
    if let Ok(v) = env::var("QSC_TEST_ROOT") {
        return PathBuf::from(v);
    }
    if let Ok(v) = env::var("CARGO_TARGET_DIR") {
        return PathBuf::from(v);
    }
    PathBuf::from("target")
}

fn unique_dir(label: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    test_root().join("qsc-test-tmp").join(format!(
        "na0112_{}_{}_{}",
        label,
        std::process::id(),
        nonce
    ))
}

fn ensure_dir_700(path: &Path) {
    fs::create_dir_all(path).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
    }
}

fn qsc_cmd() -> Command {
    Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
}

fn qsc_cmd_iso(iso: &common::TestIsolation) -> std::process::Command {
    let mut cmd = std::process::Command::new(assert_cmd::cargo::cargo_bin!("qsc"));
    iso.apply_to(&mut cmd);
    cmd
}

fn combined_output(out: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    )
}

fn assert_no_secrets(text: &str) {
    for needle in [
        "TOKEN",
        "SECRET",
        "KEY",
        "PASS",
        "PRIVATE",
        "BEARER",
        "CREDENTIAL",
        "RELAY_TOKEN",
    ] {
        assert!(
            !text.contains(needle),
            "output leaked secret marker {needle}:\n{text}"
        );
    }
}

fn contacts_route_set(cfg: &Path, label: &str, token: &str) {
    let out = qsc_cmd()
        .env("QSC_CONFIG_DIR", cfg)
        .args([
            "contacts",
            "route-set",
            "--label",
            label,
            "--route-token",
            token,
        ])
        .output()
        .expect("contacts route set");
    assert!(out.status.success(), "{}", combined_output(&out));
}

fn filter_meta_lines(text: &str) -> String {
    text.lines()
        .filter(|line| {
            line.contains("event=meta_plan")
                || line.contains("event=meta_tick")
                || line.contains("event=meta_bucket")
                || line.contains("event=meta_batch")
                || line.contains("event=meta_cover")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn meta_plan_is_deterministic() {
    let run = || {
        let out = qsc_cmd()
            .args([
                "meta",
                "plan",
                "--deterministic",
                "--tick-count",
                "3",
                "--interval-ms",
                "100",
                "--batch-max-count",
                "2",
                "--bucket-max",
                "4096",
            ])
            .output()
            .expect("meta plan output");
        assert!(out.status.success(), "meta plan must succeed");
        let text = combined_output(&out);
        let filtered = filter_meta_lines(&text);
        assert!(
            filtered.contains("event=meta_plan"),
            "meta plan marker missing:\n{filtered}"
        );
        assert!(filtered.contains("event=meta_tick tick=0"));
        assert!(filtered.contains("event=meta_tick tick=1"));
        assert!(filtered.contains("event=meta_tick tick=2"));
        assert!(filtered.contains("event=meta_batch count=2 bytes=0 planned=true"));
        assert!(filtered.contains("event=meta_bucket"));
        filtered
    };

    let first = run();
    let second = run();
    assert_eq!(first, second, "meta plan markers must be deterministic");
}

fn seed_inbox_two_items(iso: &common::TestIsolation, base: &Path, relay: &str) {
    let sender_cfg = base.join("sender_cfg");
    ensure_dir_700(&sender_cfg);
    common::init_mock_vault(&sender_cfg);
    contacts_route_set(&sender_cfg, "bob", ROUTE_TOKEN_BOB);
    let payload1 = base.join("msg1.bin");
    let payload2 = base.join("msg2.bin");
    fs::write(&payload1, b"phase2-msg-1").unwrap();
    fs::write(&payload2, b"phase2-msg-2").unwrap();

    for payload in [&payload1, &payload2] {
        let out = qsc_cmd_iso(iso)
            .env("QSC_CONFIG_DIR", &sender_cfg)
            .env("QSC_QSP_SEED", "7")
            .env("QSC_ALLOW_SEED_FALLBACK", "1")
            .args([
                "send",
                "--transport",
                "relay",
                "--relay",
                relay,
                "--to",
                "bob",
                "--file",
                payload.to_str().unwrap(),
            ])
            .output()
            .expect("send output");
        assert!(
            out.status.success(),
            "seed send failed:\n{}",
            combined_output(&out)
        );
    }
}

#[test]
fn receive_poll_emits_ticks_and_is_deterministic() {
    let iso = common::TestIsolation::new("na0112_recv_determinism");
    let root = iso.root.join("run");
    ensure_dir_700(&root);
    let server = start_inbox_server(1024 * 1024, 8);
    seed_inbox_two_items(&iso, &root, server.base_url());
    let seed_items = server.drain_channel(ROUTE_TOKEN_BOB);
    assert_eq!(seed_items.len(), 2, "expected two seeded inbox items");

    let run = |suffix: &str| {
        let cfg = root.join(format!("recv_cfg_{suffix}"));
        let out_dir = root.join(format!("out_{suffix}"));
        ensure_dir_700(&cfg);
        common::init_mock_vault(&cfg);
        let _ = fs::remove_dir_all(&out_dir);
        ensure_dir_700(&out_dir);
        server.replace_channel(ROUTE_TOKEN_BOB, seed_items.clone());

        let out = qsc_cmd_iso(&iso)
            .env("QSC_CONFIG_DIR", &cfg)
            .env("QSC_QSP_SEED", "7")
            .env("QSC_ALLOW_SEED_FALLBACK", "1")
            .args([
                "receive",
                "--transport",
                "relay",
                "--relay",
                server.base_url(),
                "--mailbox",
                ROUTE_TOKEN_BOB,
                "--from",
                "bob",
                "--max",
                "1",
                "--out",
                out_dir.to_str().unwrap(),
                "--poll-ticks",
                "2",
                "--deterministic-meta",
                "--batch-max-count",
                "1",
                "--interval-ms",
                "100",
                "--bucket-max",
                "4096",
            ])
            .output()
            .expect("receive output");
        assert!(
            out.status.success(),
            "receive must succeed:\n{}",
            combined_output(&out)
        );
        let text = combined_output(&out);
        let filtered = filter_meta_lines(&text);
        assert!(filtered.contains("event=meta_tick tick=0"));
        assert!(filtered.contains("event=meta_tick tick=1"));
        assert!(filtered.contains("deterministic=true"));
        assert!(filtered.contains("event=meta_batch count=1"));
        filtered
    };

    let first = run("first");
    let second = run("second");
    assert_eq!(first, second, "receive meta markers must be deterministic");
}

#[test]
fn bounds_reject_fail_closed_no_mutation() {
    let root = unique_dir("bounds");
    let cfg = root.join("cfg");
    let out = qsc_cmd()
        .env("QSC_CONFIG_DIR", &cfg)
        .args(["meta", "plan", "--tick-count", "999"])
        .output()
        .expect("meta plan reject output");
    assert!(!out.status.success(), "invalid bounds must fail");
    let text = combined_output(&out);
    assert!(text.contains("event=error code=meta_poll_invalid"));
    assert!(!cfg.exists(), "reject path must not mutate config state");
}

#[test]
fn no_secrets_in_meta_outputs() {
    let root = unique_dir("no_secrets");
    ensure_dir_700(&root);
    let cfg = root.join("cfg");
    ensure_dir_700(&cfg);

    let plan = qsc_cmd()
        .env("QSC_CONFIG_DIR", &cfg)
        .args([
            "meta",
            "plan",
            "--deterministic",
            "--tick-count",
            "2",
            "--interval-ms",
            "50",
            "--batch-max-count",
            "1",
            "--bucket-max",
            "4096",
            "--cover-enabled",
        ])
        .output()
        .expect("meta plan output");
    assert!(plan.status.success());
    let text = combined_output(&plan);
    assert_no_secrets(&text);
}
