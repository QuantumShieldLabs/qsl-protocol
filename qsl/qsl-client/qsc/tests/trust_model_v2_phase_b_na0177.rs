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

fn snapshot_dir(root: &Path) -> Vec<(String, Vec<u8>)> {
    let mut files = Vec::new();
    if !root.exists() {
        return files;
    }
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let entries = match fs::read_dir(&dir) {
            Ok(v) => v,
            Err(_) => continue,
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            if path.is_file() {
                let rel = path
                    .strip_prefix(root)
                    .unwrap_or(path.as_path())
                    .to_string_lossy()
                    .to_string();
                let bytes = fs::read(&path).unwrap_or_default();
                files.push((rel, bytes));
            }
        }
    }
    files.sort_by(|a, b| a.0.cmp(&b.0));
    files
}

fn qsc(cfg: &Path) -> Command {
    let mut cmd = common::qsc_std_command();
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_MARK_FORMAT", "plain");
    cmd
}

fn device_id_for(cfg: &Path, label: &str) -> String {
    let out = qsc(cfg)
        .args(["contacts", "device", "list", "--label", label])
        .output()
        .expect("device list");
    assert!(out.status.success(), "{}", output_text(&out));
    let text = output_text(&out);
    let device_line = text
        .lines()
        .find(|line| line.starts_with("device="))
        .expect("device line");
    let device = device_line
        .split_whitespace()
        .find_map(|tok| tok.strip_prefix("device="))
        .expect("device token");
    device.to_string()
}

#[test]
fn no_trusted_device_blocks_send_no_mutation() {
    let cfg = unique_test_dir("na0177_phaseb_no_trusted");
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
    let before = snapshot_dir(&cfg);
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
        "blocked send must not start send path: {text}"
    );
    assert_eq!(
        before,
        snapshot_dir(&cfg),
        "blocked send must not mutate state"
    );
    assert_no_leaks(&text);
}

#[test]
fn changed_device_blocks_until_reapproved() {
    let cfg = unique_test_dir("na0177_phaseb_changed");
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
    let device = device_id_for(&cfg, "bob");

    let mismatch = qsc(&cfg)
        .args([
            "contacts",
            "device",
            "verify",
            "--label",
            "bob",
            "--device",
            device.as_str(),
            "--fp",
            "WRNG-WRNG-WRNG-WRNG-W",
        ])
        .output()
        .expect("verify mismatch");
    assert!(
        !mismatch.status.success(),
        "mismatch should fail-closed: {}",
        output_text(&mismatch)
    );

    let payload = cfg.join("msg.bin");
    fs::write(&payload, b"hello").expect("payload");
    let blocked = qsc(&cfg)
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
        .expect("send blocked");
    assert!(!blocked.status.success(), "{}", output_text(&blocked));
    let blocked_text = output_text(&blocked);
    assert!(
        blocked_text
            .contains("QSC_SEND_BLOCKED reason=device_changed_reapproval_required peer=bob"),
        "missing changed marker: {blocked_text}"
    );

    let trust = qsc(&cfg)
        .args([
            "contacts",
            "device",
            "trust",
            "--label",
            "bob",
            "--device",
            device.as_str(),
            "--confirm",
        ])
        .output()
        .expect("trust device");
    assert!(trust.status.success(), "{}", output_text(&trust));

    let post = qsc(&cfg)
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
        .expect("send after trust");
    let post_text = output_text(&post);
    assert!(
        !post_text.contains("QSC_SEND_BLOCKED reason=device_changed_reapproval_required peer=bob")
            && !post_text.contains("QSC_SEND_BLOCKED reason=no_trusted_device peer=bob"),
        "trusted device should clear trust gate: {post_text}"
    );
    assert_no_leaks(&blocked_text);
    assert_no_leaks(&post_text);
}

#[test]
fn revoked_device_blocks_send() {
    let cfg = unique_test_dir("na0177_phaseb_revoked");
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
    let device = device_id_for(&cfg, "bob");

    let trust = qsc(&cfg)
        .args([
            "contacts",
            "device",
            "trust",
            "--label",
            "bob",
            "--device",
            device.as_str(),
            "--confirm",
        ])
        .output()
        .expect("trust");
    assert!(trust.status.success(), "{}", output_text(&trust));

    let revoke = qsc(&cfg)
        .args([
            "contacts",
            "device",
            "revoke",
            "--label",
            "bob",
            "--device",
            device.as_str(),
            "--confirm",
        ])
        .output()
        .expect("revoke");
    assert!(revoke.status.success(), "{}", output_text(&revoke));

    let payload = cfg.join("msg.bin");
    fs::write(&payload, b"hello").expect("payload");
    let blocked = qsc(&cfg)
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
        .expect("send blocked");
    assert!(!blocked.status.success(), "{}", output_text(&blocked));
    let text = output_text(&blocked);
    assert!(
        text.contains("QSC_SEND_BLOCKED reason=device_revoked peer=bob"),
        "missing revoked marker: {text}"
    );
    assert_no_leaks(&text);
}

#[test]
fn second_device_does_not_break_existing_trusted_primary() {
    let cfg = unique_test_dir("na0177_phaseb_second_device");
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
    let primary = device_id_for(&cfg, "bob");

    let trust = qsc(&cfg)
        .args([
            "contacts",
            "device",
            "trust",
            "--label",
            "bob",
            "--device",
            primary.as_str(),
            "--confirm",
        ])
        .output()
        .expect("trust");
    assert!(trust.status.success(), "{}", output_text(&trust));

    let add_second = qsc(&cfg)
        .args([
            "contacts",
            "device",
            "add",
            "--label",
            "bob",
            "--fp",
            "BBBB-CCCC-DDDD-EEEE-F",
        ])
        .output()
        .expect("add second");
    assert!(add_second.status.success(), "{}", output_text(&add_second));

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
    let text = output_text(&send);
    assert!(
        !text.contains("QSC_SEND_BLOCKED reason=no_trusted_device peer=bob")
            && !text
                .contains("QSC_SEND_BLOCKED reason=device_changed_reapproval_required peer=bob")
            && !text.contains("QSC_SEND_BLOCKED reason=device_revoked peer=bob"),
        "adding second device must not break trusted primary: {text}"
    );
    assert_no_leaks(&text);
}
