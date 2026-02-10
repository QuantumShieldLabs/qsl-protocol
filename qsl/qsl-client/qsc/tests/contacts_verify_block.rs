mod common;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn safe_test_root() -> PathBuf {
    let root = if let Ok(v) = std::env::var("QSC_TEST_ROOT") {
        PathBuf::from(v)
    } else if let Ok(v) = std::env::var("CARGO_TARGET_DIR") {
        PathBuf::from(v)
    } else {
        PathBuf::from("target")
    };
    let root = root.join("qsc-test-tmp");
    ensure_dir_700(&root);
    root
}

fn ensure_dir_700(path: &Path) {
    let _ = fs::create_dir_all(path);
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(path, fs::Permissions::from_mode(0o700));
    }
}

fn create_dir_700(path: &Path) {
    let _ = fs::remove_dir_all(path);
    ensure_dir_700(path);
}

fn output_text(out: &std::process::Output) -> String {
    let mut s = String::from_utf8_lossy(&out.stdout).to_string();
    s.push_str(&String::from_utf8_lossy(&out.stderr));
    s
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

fn init_vault(cfg: &Path) {
    let out = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg)
        .env("QSC_PASSPHRASE", "test-passphrase")
        .env("QSC_DISABLE_KEYCHAIN", "1")
        .args([
            "vault",
            "init",
            "--non-interactive",
            "--passphrase-env",
            "QSC_PASSPHRASE",
        ])
        .output()
        .expect("vault init");
    assert!(out.status.success(), "vault init: {}", output_text(&out));
}

fn qsc_with_unlock(cfg: &Path) -> Command {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("qsc"));
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_PASSPHRASE", "test-passphrase")
        .env("QSC_DISABLE_KEYCHAIN", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_QSP_SEED", "7")
        .env("QSC_MARK_FORMAT", "plain")
        .arg("--unlock-passphrase-env")
        .arg("QSC_PASSPHRASE");
    cmd
}

fn assert_no_secrets(s: &str) {
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
        assert!(!s.contains(needle), "secret leaked");
    }
}

#[test]
fn contacts_add_list_deterministic() {
    let base = safe_test_root().join(format!("na0116_contacts_list_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    init_vault(&cfg);

    let add_bob = qsc_with_unlock(&cfg)
        .args(["contacts", "add", "--label", "bob", "--fp", "fp-bob-1"])
        .output()
        .expect("contacts add bob");
    assert!(add_bob.status.success(), "{}", output_text(&add_bob));

    let add_alice = qsc_with_unlock(&cfg)
        .args(["contacts", "add", "--label", "alice", "--fp", "fp-alice-1"])
        .output()
        .expect("contacts add alice");
    assert!(add_alice.status.success(), "{}", output_text(&add_alice));

    let list = qsc_with_unlock(&cfg)
        .args(["contacts", "list"])
        .output()
        .expect("contacts list");
    assert!(list.status.success(), "{}", output_text(&list));
    let text = output_text(&list);
    assert!(text.contains("event=contacts_list count=2"));
    let idx_alice = text.find("label=alice").unwrap_or(usize::MAX);
    let idx_bob = text.find("label=bob").unwrap_or(usize::MAX);
    assert!(
        idx_alice < idx_bob,
        "contacts list not stable-sorted: {}",
        text
    );
}

#[test]
fn blocked_peer_refuses_handshake_no_mutation() {
    let base = safe_test_root().join(format!("na0116_blocked_hs_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    init_vault(&cfg);

    let add = qsc_with_unlock(&cfg)
        .args(["contacts", "add", "--label", "bob", "--fp", "fp-bob-1"])
        .output()
        .expect("contacts add");
    assert!(add.status.success(), "{}", output_text(&add));
    let block = qsc_with_unlock(&cfg)
        .args(["contacts", "block", "--label", "bob"])
        .output()
        .expect("contacts block");
    assert!(block.status.success(), "{}", output_text(&block));

    let before = snapshot_dir(&cfg);
    let hs = qsc_with_unlock(&cfg)
        .args([
            "handshake",
            "init",
            "--as",
            "alice",
            "--peer",
            "bob",
            "--relay",
            "http://127.0.0.1:9",
        ])
        .output()
        .expect("handshake blocked");
    assert!(!hs.status.success(), "blocked handshake must fail");
    let text = output_text(&hs);
    assert!(text.contains("event=error code=peer_blocked"), "{}", text);
    assert_eq!(
        before,
        snapshot_dir(&cfg),
        "blocked handshake mutated state"
    );
}

#[test]
fn pinned_mismatch_refuses_no_mutation() {
    let server = common::start_inbox_server(1024 * 1024, 16);
    let base = safe_test_root().join(format!("na0116_mismatch_hs_{}", std::process::id()));
    create_dir_700(&base);
    let alice_cfg = base.join("alice_cfg");
    let mallory_cfg = base.join("mallory_cfg");
    create_dir_700(&alice_cfg);
    create_dir_700(&mallory_cfg);
    init_vault(&alice_cfg);
    init_vault(&mallory_cfg);

    let add = qsc_with_unlock(&alice_cfg)
        .args([
            "contacts",
            "add",
            "--label",
            "mallory",
            "--fp",
            "fp-does-not-match",
            "--verify",
        ])
        .output()
        .expect("contacts add mismatch");
    assert!(add.status.success(), "{}", output_text(&add));

    let send_hs1 = qsc_with_unlock(&mallory_cfg)
        .args([
            "handshake",
            "init",
            "--as",
            "mallory",
            "--peer",
            "alice",
            "--relay",
            server.base_url(),
        ])
        .output()
        .expect("mallory hs init");
    assert!(send_hs1.status.success(), "{}", output_text(&send_hs1));

    let before = snapshot_dir(&alice_cfg);
    let poll = qsc_with_unlock(&alice_cfg)
        .args([
            "handshake",
            "poll",
            "--as",
            "alice",
            "--peer",
            "mallory",
            "--relay",
            server.base_url(),
            "--max",
            "1",
        ])
        .output()
        .expect("alice hs poll");
    assert!(
        poll.status.success(),
        "poll should reject message but not hard fail"
    );
    let text = output_text(&poll);
    assert!(text.contains("code=peer_mismatch"), "{}", text);
    assert_eq!(
        before,
        snapshot_dir(&alice_cfg),
        "mismatch poll mutated state"
    );
}

#[test]
fn verify_requires_confirm_no_mutation() {
    let base = safe_test_root().join(format!("na0116_verify_refuse_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    init_vault(&cfg);

    let add = qsc_with_unlock(&cfg)
        .args(["contacts", "add", "--label", "bob", "--fp", "fp-old"])
        .output()
        .expect("contacts add");
    assert!(add.status.success(), "{}", output_text(&add));

    let before = snapshot_dir(&cfg);
    let verify = qsc_with_unlock(&cfg)
        .args(["contacts", "verify", "--label", "bob", "--fp", "fp-new"])
        .output()
        .expect("contacts verify no confirm");
    assert!(!verify.status.success(), "verify without confirm must fail");
    let text = output_text(&verify);
    assert!(
        text.contains("event=error code=verify_requires_confirm"),
        "{}",
        text
    );
    assert_eq!(before, snapshot_dir(&cfg), "verify reject mutated state");
}

#[test]
fn no_plaintext_contacts_on_disk() {
    let base = safe_test_root().join(format!("na0116_no_plaintext_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    init_vault(&cfg);

    let add = qsc_with_unlock(&cfg)
        .args(["contacts", "add", "--label", "bob", "--fp", "fp-bob-1"])
        .output()
        .expect("contacts add");
    assert!(add.status.success(), "{}", output_text(&add));

    let mut stack = vec![cfg.clone()];
    while let Some(dir) = stack.pop() {
        let entries = fs::read_dir(&dir).expect("scan dir");
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            let name = path.file_name().and_then(|v| v.to_str()).unwrap_or("");
            assert_ne!(
                name, "contacts.json",
                "plaintext contacts file should not exist"
            );
        }
    }
}

#[test]
fn no_secrets_in_output() {
    let base = safe_test_root().join(format!("na0116_no_secrets_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    init_vault(&cfg);

    let add = qsc_with_unlock(&cfg)
        .args([
            "contacts", "add", "--label", "bob", "--fp", "fp-bob-1", "--verify",
        ])
        .output()
        .expect("contacts add");
    let list = qsc_with_unlock(&cfg)
        .args(["contacts", "list"])
        .output()
        .expect("contacts list");
    let show = qsc_with_unlock(&cfg)
        .args(["contacts", "show", "--label", "bob"])
        .output()
        .expect("contacts show");

    assert!(add.status.success(), "{}", output_text(&add));
    assert!(list.status.success(), "{}", output_text(&list));
    assert!(show.status.success(), "{}", output_text(&show));
    let mut combined = output_text(&add);
    combined.push_str(&output_text(&list));
    combined.push_str(&output_text(&show));
    assert_no_secrets(&combined);
}
