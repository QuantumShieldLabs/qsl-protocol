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
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("qsc"));
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_MARK_FORMAT", "plain");
    cmd
}

#[test]
fn cli_send_unknown_contact_blocked_no_mutation() {
    let cfg = unique_test_dir("na0177_cli_send_unknown");
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);

    let payload = cfg.join("msg.bin");
    fs::write(&payload, b"hello").expect("payload write");
    let before = snapshot_dir(&cfg);

    let out = qsc(&cfg)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            "http://127.0.0.1:9",
            "--to",
            "bob",
            "--file",
            payload.to_str().expect("payload utf8"),
        ])
        .output()
        .expect("send unknown");
    assert!(!out.status.success(), "send should fail-closed");
    let text = output_text(&out);
    assert!(
        text.contains("QSC_SEND_BLOCKED reason=unknown_contact peer=bob"),
        "missing block marker: {text}"
    );
    assert!(
        !text.contains("event=send_prepare")
            && !text.contains("event=relay_event action=push_fail"),
        "send attempt should not start: {text}"
    );
    assert_eq!(
        before,
        snapshot_dir(&cfg),
        "unknown-contact blocked send must not mutate state"
    );
    assert_no_leaks(&text);
}

#[test]
fn cli_file_send_unknown_contact_blocked_no_mutation() {
    let cfg = unique_test_dir("na0177_cli_file_unknown");
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);

    let payload = cfg.join("file.bin");
    fs::write(&payload, b"hello-file").expect("payload write");
    let before = snapshot_dir(&cfg);

    let out = qsc(&cfg)
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
            payload.to_str().expect("payload utf8"),
        ])
        .output()
        .expect("file send unknown");
    assert!(!out.status.success(), "file send should fail-closed");
    let text = output_text(&out);
    assert!(
        text.contains("QSC_SEND_BLOCKED reason=unknown_contact peer=bob"),
        "missing block marker: {text}"
    );
    assert!(
        !text.contains("event=send_prepare")
            && !text.contains("event=relay_event action=push_fail"),
        "file send attempt should not start: {text}"
    );
    assert_eq!(
        before,
        snapshot_dir(&cfg),
        "unknown-contact blocked file send must not mutate state"
    );
    assert_no_leaks(&text);
}

#[test]
fn cli_send_unpinned_contact_blocked() {
    let cfg = unique_test_dir("na0177_cli_send_unpinned");
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);

    let add = qsc(&cfg)
        .args([
            "contacts", "add", "--label", "bob", "--fp", "fp-bob-1", "--verify",
        ])
        .output()
        .expect("contacts add");
    assert!(add.status.success(), "contacts add should succeed");

    let payload = cfg.join("msg.bin");
    fs::write(&payload, b"hello").expect("payload write");
    let before = snapshot_dir(&cfg);

    let out = qsc(&cfg)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            "http://127.0.0.1:9",
            "--to",
            "bob",
            "--file",
            payload.to_str().expect("payload utf8"),
        ])
        .output()
        .expect("send unpinned");
    assert!(!out.status.success(), "unpinned send should fail-closed");
    let text = output_text(&out);
    assert!(
        text.contains("QSC_SEND_BLOCKED reason=trust_not_pinned peer=bob"),
        "missing trust_not_pinned marker: {text}"
    );
    assert!(
        !text.contains("event=send_prepare")
            && !text.contains("event=relay_event action=push_fail"),
        "send attempt should not start: {text}"
    );
    assert_eq!(
        before,
        snapshot_dir(&cfg),
        "unpinned blocked send must not mutate state"
    );
    assert_no_leaks(&text);
}
