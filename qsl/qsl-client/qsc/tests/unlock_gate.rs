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

fn qsc_with_unlock(cfg: &Path, passphrase: &str) -> Command {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("qsc"));
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_DISABLE_KEYCHAIN", "1")
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain");
    common::add_global_unlock_passphrase_file_arg(&mut cmd, cfg, "unlock-gate", passphrase);
    cmd
}

fn qsc_vault_unlock(cfg: &Path, passphrase: &str) -> Command {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("qsc"));
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_DISABLE_KEYCHAIN", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args(["vault", "unlock"]);
    common::add_vault_passphrase_file_arg(&mut cmd, cfg, "unlock-gate-vault", passphrase);
    cmd
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

    common::init_passphrase_vault(&cfg, "test-passphrase");
    let add_contact_out = qsc_with_unlock(&cfg, "test-passphrase")
        .args([
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

    let send_out = qsc_with_unlock(&cfg, "test-passphrase")
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

    let recv_out = qsc_with_unlock(&cfg, "test-passphrase")
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

    common::init_passphrase_vault(&cfg, secret);
    let out = qsc_vault_unlock(&cfg, secret)
        .output()
        .expect("vault unlock");
    assert!(out.status.success(), "vault unlock must succeed");
    let text = output_text(&out);
    assert!(text.contains("event=vault_unlock ok=true state=unlocked"));
    assert_no_secrets(&text, secret);
}

#[test]
fn retired_passphrase_env_and_argv_paths_fail_closed() {
    let base = safe_test_root().join(format!("na0216b_retired_env_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    common::init_passphrase_vault(&cfg, "test-passphrase");

    let retired_unlock = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_PASSPHRASE", "test-passphrase")
        .env("QSC_MARK_FORMAT", "plain")
        .args(["vault", "unlock", "--passphrase-env", "QSC_PASSPHRASE"])
        .output()
        .expect("retired unlock env");
    let retired_unlock_text = output_text(&retired_unlock);
    assert!(
        !retired_unlock.status.success(),
        "retired unlock env must fail"
    );
    assert!(
        retired_unlock_text.contains("event=error code=vault_passphrase_env_retired"),
        "{}",
        retired_unlock_text
    );

    let retired_init_env = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_PASSPHRASE", "test-passphrase")
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "vault",
            "init",
            "--non-interactive",
            "--passphrase-env",
            "QSC_PASSPHRASE",
        ])
        .output()
        .expect("retired init env");
    let retired_init_env_text = output_text(&retired_init_env);
    assert!(
        !retired_init_env.status.success(),
        "retired init env must fail"
    );
    assert!(
        retired_init_env_text.contains("event=error code=vault_passphrase_env_retired"),
        "{}",
        retired_init_env_text
    );

    let retired_init_argv = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "vault",
            "init",
            "--non-interactive",
            "--passphrase",
            "test-passphrase",
        ])
        .output()
        .expect("retired init argv");
    let retired_init_argv_text = output_text(&retired_init_argv);
    assert!(
        !retired_init_argv.status.success(),
        "retired init argv must fail"
    );
    assert!(
        retired_init_argv_text.contains("event=error code=vault_passphrase_argv_retired"),
        "{}",
        retired_init_argv_text
    );
}
