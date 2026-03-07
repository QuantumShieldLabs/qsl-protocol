mod common;

use assert_cmd::Command as AssertCommand;
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
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("qsc"));
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
    device_line
        .split_whitespace()
        .find_map(|tok| tok.strip_prefix("device="))
        .expect("device token")
        .to_string()
}

fn run_headless(cfg: &Path, script: &str) -> String {
    let mut cmd = AssertCommand::new(assert_cmd::cargo::cargo_bin!("qsc"));
    let out = cmd
        .env("QSC_CONFIG_DIR", cfg)
        .env("QSC_TUI_HEADLESS", "1")
        .env("QSC_TUI_SCRIPT", script)
        .env("QSC_TUI_COLS", "140")
        .env("QSC_TUI_ROWS", "40")
        .env("QSC_MARK_FORMAT", "plain")
        .env("QSC_TUI_TEST_UNLOCK", "1")
        .args(["tui"])
        .output()
        .expect("run tui");
    let combined = output_text(&out);
    assert!(out.status.success(), "tui failed: {combined}");
    combined
}

#[test]
fn unknown_contact_remediation_cli_send_and_file_send() {
    let cfg = unique_test_dir("na0178_unknown_contact");
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);

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
    let send_text = output_text(&send);
    assert!(
        send_text.contains("QSC_SEND_BLOCKED reason=unknown_contact peer=bob"),
        "missing blocked marker: {send_text}"
    );
    assert!(
        send_text
            .contains("QSC_TRUST_REMEDIATION reason=unknown_contact step=add_contact peer=bob")
            && send_text
                .contains("QSC_TRUST_REMEDIATION reason=unknown_contact step=learn_more peer=bob"),
        "missing remediation steps: {send_text}"
    );

    let file_send = qsc(&cfg)
        .args([
            "file",
            "send",
            "--transport",
            "relay",
            "--relay",
            "http://127.0.0.1:9",
            "--to",
            "bob",
            "--path",
            payload.to_str().expect("utf8"),
        ])
        .output()
        .expect("file send");
    assert!(!file_send.status.success(), "{}", output_text(&file_send));
    let file_text = output_text(&file_send);
    assert!(
        file_text.contains("QSC_SEND_BLOCKED reason=unknown_contact peer=bob"),
        "missing blocked marker for file send: {file_text}"
    );
    assert!(
        file_text
            .contains("QSC_TRUST_REMEDIATION reason=unknown_contact step=add_contact peer=bob")
            && file_text
                .contains("QSC_TRUST_REMEDIATION reason=unknown_contact step=learn_more peer=bob"),
        "missing remediation steps for file send: {file_text}"
    );
    assert_no_leaks(&send_text);
    assert_no_leaks(&file_text);
}

#[test]
fn no_trusted_device_remediation_cli_no_mutation() {
    let cfg = unique_test_dir("na0178_no_trusted");
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
        "missing blocked marker: {text}"
    );
    assert!(
        text.contains("QSC_TRUST_REMEDIATION reason=no_trusted_device step=list_devices peer=bob")
            && text.contains(
                "QSC_TRUST_REMEDIATION reason=no_trusted_device step=trust_device peer=bob"
            ),
        "missing remediation steps: {text}"
    );
    assert!(
        !text.contains("event=send_prepare") && !text.contains("QSC_ROUTING"),
        "blocked send must not mutate into send path: {text}"
    );
    assert_eq!(
        before,
        snapshot_dir(&cfg),
        "blocked send must not mutate state"
    );
    assert_no_leaks(&text);
}

#[test]
fn changed_device_remediation_cli_no_mutation() {
    let cfg = unique_test_dir("na0178_changed");
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
    assert!(!mismatch.status.success(), "{}", output_text(&mismatch));

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
        text.contains("QSC_SEND_BLOCKED reason=device_changed_reapproval_required peer=bob"),
        "missing changed blocked marker: {text}"
    );
    assert!(
        text.contains(
            "QSC_TRUST_REMEDIATION reason=device_changed_reapproval_required step=reapprove_changed_device peer=bob"
        ),
        "missing changed remediation marker: {text}"
    );
    assert!(
        !text.contains("event=send_prepare") && !text.contains("QSC_ROUTING"),
        "changed-device block must not mutate into send path: {text}"
    );
    assert_eq!(
        before,
        snapshot_dir(&cfg),
        "changed-device blocked send must not mutate state"
    );
    assert_no_leaks(&text);
}

#[test]
fn revoked_device_remediation_cli_no_mutation() {
    let cfg = unique_test_dir("na0178_revoked");
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
        text.contains("QSC_SEND_BLOCKED reason=device_revoked peer=bob"),
        "missing revoked blocked marker: {text}"
    );
    assert!(
        text.contains(
            "QSC_TRUST_REMEDIATION reason=device_revoked step=readd_revoked_device peer=bob"
        ),
        "missing revoked remediation marker: {text}"
    );
    assert!(
        !text.contains("event=send_prepare") && !text.contains("QSC_ROUTING"),
        "revoked block must not mutate into send path: {text}"
    );
    assert_eq!(
        before,
        snapshot_dir(&cfg),
        "revoked blocked send must not mutate state"
    );
    assert_no_leaks(&text);
}

#[test]
fn tui_blocked_msg_emits_remediation_markers() {
    let cfg = unique_test_dir("na0178_tui_blocked");
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    let out = run_headless(&cfg, "/msg unknown hello;/exit");
    assert!(
        out.contains("QSC_TUI_SEND_BLOCKED reason=unknown_contact peer=unknown"),
        "missing blocked marker: {out}"
    );
    assert!(
        out.contains(
            "QSC_TUI_TRUST_REMEDIATION reason=unknown_contact step=add_contact peer=unknown"
        ) && out.contains(
            "QSC_TUI_TRUST_REMEDIATION reason=unknown_contact step=learn_more peer=unknown"
        ),
        "missing tui remediation markers: {out}"
    );
    assert_no_leaks(&out);
}
