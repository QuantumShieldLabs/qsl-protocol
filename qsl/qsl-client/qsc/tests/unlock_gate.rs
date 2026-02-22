mod common;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const ROUTE_TOKEN_BOB: &str = "route_token_bob_abcdefghijklmnopqr";

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

fn assert_no_secrets(s: &str, secret: &str) {
    for needle in [
        "TOKEN",
        "SECRET",
        "KEY",
        "PASS",
        "PRIVATE",
        "BEARER",
        "CREDENTIAL",
        "RELAY_TOKEN",
        secret,
    ] {
        assert!(!s.contains(needle), "secret leaked");
    }
}

#[test]
fn locked_send_refuses_no_mutation() {
    let base = safe_test_root().join(format!("na0115_locked_send_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    fs::write(cfg.join("sentinel.txt"), b"baseline").unwrap();
    let payload = base.join("msg.bin");
    fs::write(&payload, b"locked-send").unwrap();
    let before = snapshot_dir(&cfg);

    let out = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_MARK_FORMAT", "plain")
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
        ])
        .output()
        .expect("locked send");

    assert!(!out.status.success(), "locked send must fail");
    let text = output_text(&out);
    assert!(text.contains("event=error code=vault_locked"));
    assert_eq!(before, snapshot_dir(&cfg), "locked send mutated state");
}

#[test]
fn locked_receive_refuses_no_mutation() {
    let base = safe_test_root().join(format!("na0115_locked_recv_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    fs::write(cfg.join("sentinel.txt"), b"baseline").unwrap();
    let out_dir = base.join("out");
    create_dir_700(&out_dir);
    let before_cfg = snapshot_dir(&cfg);
    let before_out = snapshot_dir(&out_dir);

    let out = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            "http://127.0.0.1:9",
            "--from",
            "bob",
            "--mailbox",
            "bob",
            "--max",
            "1",
            "--out",
            out_dir.to_str().unwrap(),
        ])
        .output()
        .expect("locked receive");

    assert!(!out.status.success(), "locked receive must fail");
    let text = output_text(&out);
    assert!(text.contains("event=error code=vault_locked"));
    assert_eq!(before_cfg, snapshot_dir(&cfg), "locked receive mutated cfg");
    assert_eq!(
        before_out,
        snapshot_dir(&out_dir),
        "locked receive mutated out"
    );
}

#[test]
fn locked_handshake_refuses_no_mutation() {
    let base = safe_test_root().join(format!("na0115_locked_hs_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    fs::write(cfg.join("sentinel.txt"), b"baseline").unwrap();
    let before = snapshot_dir(&cfg);

    let out = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_MARK_FORMAT", "plain")
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
        .expect("locked handshake");

    assert!(!out.status.success(), "locked handshake must fail");
    let text = output_text(&out);
    assert!(text.contains("event=error code=vault_locked"));
    assert_eq!(before, snapshot_dir(&cfg), "locked handshake mutated cfg");
}

#[test]
fn unlock_allows_send_receive_happy_path() {
    let server = common::start_inbox_server(1024 * 1024, 16);
    let base = safe_test_root().join(format!("na0115_unlock_happy_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    let out_dir = base.join("out");
    create_dir_700(&cfg);
    create_dir_700(&out_dir);
    let payload = base.join("msg.bin");
    fs::write(&payload, b"unlock-happy").unwrap();

    let init_out = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
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
    assert!(
        init_out.status.success(),
        "vault init failed: {}",
        output_text(&init_out)
    );
    let add_contact_out = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_PASSPHRASE", "test-passphrase")
        .env("QSC_DISABLE_KEYCHAIN", "1")
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .args([
            "--unlock-passphrase-env",
            "QSC_PASSPHRASE",
            "contacts",
            "add",
            "--label",
            "bob",
            "--fp",
            "fp-test",
            "--route-token",
            ROUTE_TOKEN_BOB,
        ])
        .output()
        .expect("contacts add");
    assert!(
        add_contact_out.status.success(),
        "contacts add failed: {}",
        output_text(&add_contact_out)
    );

    let send_out = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_PASSPHRASE", "test-passphrase")
        .env("QSC_DISABLE_KEYCHAIN", "1")
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "--unlock-passphrase-env",
            "QSC_PASSPHRASE",
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "bob",
            "--file",
            payload.to_str().unwrap(),
        ])
        .output()
        .expect("unlocked send");
    assert!(
        send_out.status.success(),
        "unlocked send failed: {}",
        output_text(&send_out)
    );
    let send_text = output_text(&send_out);
    assert!(send_text.contains("event=qsp_pack ok=true"));

    let recv_out = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_PASSPHRASE", "test-passphrase")
        .env("QSC_DISABLE_KEYCHAIN", "1")
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "--unlock-passphrase-env",
            "QSC_PASSPHRASE",
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
        ])
        .output()
        .expect("unlocked receive");
    assert!(
        recv_out.status.success(),
        "unlocked receive failed: {}",
        output_text(&recv_out)
    );
    let recv_text = output_text(&recv_out);
    assert!(recv_text.contains("event=qsp_unpack ok=true"));
}

#[test]
fn no_secrets_in_unlock_output() {
    let base = safe_test_root().join(format!("na0115_unlock_redact_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    let secret = "unlock-super-secret-passphrase";

    let init_out = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_PASSPHRASE", secret)
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
    assert!(init_out.status.success(), "vault init must succeed");

    let out = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_PASSPHRASE", secret)
        .env("QSC_DISABLE_KEYCHAIN", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args(["vault", "unlock", "--passphrase-env", "QSC_PASSPHRASE"])
        .output()
        .expect("vault unlock");
    assert!(out.status.success(), "vault unlock must succeed");
    let text = output_text(&out);
    assert!(text.contains("event=vault_unlock ok=true state=unlocked"));
    assert_no_secrets(&text, secret);
}
