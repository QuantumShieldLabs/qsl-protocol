mod common;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn ensure_dir_700(path: &Path) {
    fs::create_dir_all(path).expect("create dir");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).expect("chmod 700");
    }
}

fn unique_test_dir(tag: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    std::env::current_dir()
        .expect("cwd")
        .join("target")
        .join("qsc-tests")
        .join(format!("{tag}-{}-{nonce}", std::process::id()))
}

fn output_text(out: &std::process::Output) -> String {
    let mut s = String::from_utf8_lossy(&out.stdout).to_string();
    s.push_str(&String::from_utf8_lossy(&out.stderr));
    s
}

fn assert_no_leaks(s: &str) {
    assert!(!s.contains("/v1/"), "unexpected /v1/ output: {s}");
    let mut run = 0usize;
    for ch in s.chars() {
        if ch.is_ascii_hexdigit() {
            run = run.saturating_add(1);
            assert!(run < 32, "unexpected long-hex output: {s}");
        } else {
            run = 0;
        }
    }
}

fn qsc(cfg: &Path) -> Command {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("qsc"));
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_QSP_SEED", "7");
    cmd
}

fn device_ids_for(cfg: &Path, label: &str) -> Vec<String> {
    let out = qsc(cfg)
        .args(["contacts", "device", "list", "--label", label])
        .output()
        .expect("device list");
    assert!(out.status.success(), "{}", output_text(&out));
    let text = output_text(&out);
    let mut devices = Vec::new();
    for line in text.lines() {
        if let Some(tok) = line.split_whitespace().find(|v| v.starts_with("device=")) {
            devices.push(tok.trim_start_matches("device=").to_string());
        }
    }
    devices.sort();
    devices
}

fn mk_contact_with_two_trusted_devices(cfg: &Path) -> (String, String) {
    let add = qsc(cfg)
        .args([
            "contacts",
            "add",
            "--label",
            "bob",
            "--fp",
            "ABCD-EFGH-JKMP-QRST-V",
            "--route-token",
            "bobrouteprimary0000000001",
            "--verify",
        ])
        .output()
        .expect("contacts add");
    assert!(add.status.success(), "{}", output_text(&add));

    let add_second = qsc(cfg)
        .args([
            "contacts",
            "device",
            "add",
            "--label",
            "bob",
            "--fp",
            "BBBB-CCCC-DDDD-EEEE-F",
            "--route-token",
            "bobroutesecondary0000002",
        ])
        .output()
        .expect("add second");
    assert!(add_second.status.success(), "{}", output_text(&add_second));

    let devices = device_ids_for(cfg, "bob");
    assert_eq!(devices.len(), 2, "expected two devices: {devices:?}");
    let d1 = devices[0].clone();
    let d2 = devices[1].clone();

    for dev in [&d1, &d2] {
        let trust = qsc(cfg)
            .args([
                "contacts",
                "device",
                "trust",
                "--label",
                "bob",
                "--device",
                dev.as_str(),
                "--confirm",
            ])
            .output()
            .expect("trust device");
        assert!(trust.status.success(), "{}", output_text(&trust));
    }

    (d1, d2)
}

#[test]
fn primary_only_routing_marker_changes_after_primary_switch() {
    let cfg = unique_test_dir("na0177_phasec_primary_switch");
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    let (d1, d2) = mk_contact_with_two_trusted_devices(&cfg);

    let set_primary_1 = qsc(&cfg)
        .args([
            "contacts",
            "device",
            "primary",
            "set",
            "--label",
            "bob",
            "--device",
            d1.as_str(),
            "--confirm",
        ])
        .output()
        .expect("primary set d1");
    assert!(
        set_primary_1.status.success(),
        "{}",
        output_text(&set_primary_1)
    );

    let payload = cfg.join("msg.bin");
    fs::write(&payload, b"hello").expect("payload");

    let send_1 = qsc(&cfg)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            "http://127.0.0.1:9",
            "--to",
            "bob",
            "--file",
            payload.to_str().expect("utf8"),
        ])
        .output()
        .expect("send1");
    let text_1 = output_text(&send_1);
    assert!(
        text_1.contains(format!("QSC_ROUTING policy=primary_only peer=bob device={}", d1).as_str()),
        "missing primary device1 marker: {text_1}"
    );

    let set_primary_2 = qsc(&cfg)
        .args([
            "contacts",
            "device",
            "primary",
            "set",
            "--label",
            "bob",
            "--device",
            d2.as_str(),
            "--confirm",
        ])
        .output()
        .expect("primary set d2");
    assert!(
        set_primary_2.status.success(),
        "{}",
        output_text(&set_primary_2)
    );

    let send_2 = qsc(&cfg)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            "http://127.0.0.1:9",
            "--to",
            "bob",
            "--file",
            payload.to_str().expect("utf8"),
        ])
        .output()
        .expect("send2");
    let text_2 = output_text(&send_2);
    assert!(
        text_2.contains(format!("QSC_ROUTING policy=primary_only peer=bob device={}", d2).as_str()),
        "missing primary device2 marker: {text_2}"
    );
    assert_no_leaks(&text_1);
    assert_no_leaks(&text_2);
}

#[test]
fn primary_show_reports_explicit_primary_only_policy() {
    let cfg = unique_test_dir("na0177_phasec_primary_show");
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    let (d1, _) = mk_contact_with_two_trusted_devices(&cfg);

    let set = qsc(&cfg)
        .args([
            "contacts",
            "device",
            "primary",
            "set",
            "--label",
            "bob",
            "--device",
            d1.as_str(),
            "--confirm",
        ])
        .output()
        .expect("primary set");
    assert!(set.status.success(), "{}", output_text(&set));

    let show = qsc(&cfg)
        .args(["contacts", "device", "primary", "show", "--label", "bob"])
        .output()
        .expect("primary show");
    assert!(show.status.success(), "{}", output_text(&show));
    let text = output_text(&show);
    assert!(
        text.contains(
            format!(
                "label=bob primary_device={} selected=explicit policy=primary_only",
                d1
            )
            .as_str()
        ),
        "missing primary show line: {text}"
    );
    assert_no_leaks(&text);
}

#[test]
fn primary_state_changed_or_revoked_fails_closed_no_mutation() {
    let cfg = unique_test_dir("na0177_phasec_primary_state");
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    let (d1, d2) = mk_contact_with_two_trusted_devices(&cfg);

    let mismatch = qsc(&cfg)
        .args([
            "contacts",
            "device",
            "verify",
            "--label",
            "bob",
            "--device",
            d2.as_str(),
            "--fp",
            "WRNG-WRNG-WRNG-WRNG-W",
        ])
        .output()
        .expect("verify mismatch");
    assert!(!mismatch.status.success(), "{}", output_text(&mismatch));

    let set_changed = qsc(&cfg)
        .args([
            "contacts",
            "device",
            "primary",
            "set",
            "--label",
            "bob",
            "--device",
            d2.as_str(),
            "--confirm",
        ])
        .output()
        .expect("set changed primary");
    assert!(
        set_changed.status.success(),
        "{}",
        output_text(&set_changed)
    );

    let payload = cfg.join("msg.bin");
    fs::write(&payload, b"hello").expect("payload");
    let blocked_changed = qsc(&cfg)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            "http://127.0.0.1:9",
            "--to",
            "bob",
            "--file",
            payload.to_str().expect("utf8"),
        ])
        .output()
        .expect("send blocked changed");
    let changed_text = output_text(&blocked_changed);
    assert!(
        changed_text
            .contains("QSC_SEND_BLOCKED reason=device_changed_reapproval_required peer=bob"),
        "missing changed block marker: {changed_text}"
    );
    assert!(
        !changed_text.contains("event=send_prepare"),
        "must not mutate send path: {changed_text}"
    );

    let set_primary_1 = qsc(&cfg)
        .args([
            "contacts",
            "device",
            "primary",
            "set",
            "--label",
            "bob",
            "--device",
            d1.as_str(),
            "--confirm",
        ])
        .output()
        .expect("set primary d1");
    assert!(
        set_primary_1.status.success(),
        "{}",
        output_text(&set_primary_1)
    );

    let revoke = qsc(&cfg)
        .args([
            "contacts",
            "device",
            "revoke",
            "--label",
            "bob",
            "--device",
            d1.as_str(),
            "--confirm",
        ])
        .output()
        .expect("revoke d1");
    assert!(revoke.status.success(), "{}", output_text(&revoke));

    let blocked_revoked = qsc(&cfg)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            "http://127.0.0.1:9",
            "--to",
            "bob",
            "--file",
            payload.to_str().expect("utf8"),
        ])
        .output()
        .expect("send blocked revoked");
    let revoked_text = output_text(&blocked_revoked);
    assert!(
        revoked_text.contains("QSC_SEND_BLOCKED reason=device_revoked peer=bob"),
        "missing revoked block marker: {revoked_text}"
    );
    assert!(
        !revoked_text.contains("event=send_prepare"),
        "must not mutate send path: {revoked_text}"
    );

    assert_no_leaks(&changed_text);
    assert_no_leaks(&revoked_text);
}

#[test]
fn no_trusted_device_still_blocks_no_mutation() {
    let cfg = unique_test_dir("na0177_phasec_no_trusted");
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);

    let add = qsc(&cfg)
        .args([
            "contacts",
            "add",
            "--label",
            "bob",
            "--fp",
            "ABCD-EFGH-JKMP-QRST-V",
            "--verify",
        ])
        .output()
        .expect("contacts add");
    assert!(add.status.success(), "{}", output_text(&add));

    let payload = cfg.join("msg.bin");
    fs::write(&payload, b"hello").expect("payload");
    let send = qsc(&cfg)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            "http://127.0.0.1:9",
            "--to",
            "bob",
            "--file",
            payload.to_str().expect("utf8"),
        ])
        .output()
        .expect("send");
    assert!(!send.status.success(), "{}", output_text(&send));
    let text = output_text(&send);
    assert!(
        text.contains("QSC_SEND_BLOCKED reason=no_trusted_device peer=bob"),
        "missing no_trusted_device marker: {text}"
    );
    assert!(
        !text.contains("event=send_prepare"),
        "blocked send must not mutate: {text}"
    );
    assert_no_leaks(&text);
}
