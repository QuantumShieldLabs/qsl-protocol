use assert_cmd::Command;
use pqcrypto_traits::kem::{PublicKey as _, SecretKey as _};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

mod common;

const ROUTE_TOKEN_BOB: &str = "route_token_bob_abcdefghijklmnopqr";

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

fn contains_bytes(hay: &[u8], needle: &[u8]) -> bool {
    if needle.is_empty() || hay.len() < needle.len() {
        return false;
    }
    hay.windows(needle.len()).any(|w| w == needle)
}

fn legacy_identity_json() -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let (pk, sk) = pqcrypto_kyber::kyber768::keypair();
    let pkb = pk.as_bytes().to_vec();
    let skb = sk.as_bytes().to_vec();
    let json = serde_json::json!({
        "kem_pk": pkb,
        "kem_sk": skb,
    });
    (serde_json::to_vec(&json).unwrap(), pkb, skb)
}

#[test]
fn identity_secret_not_plaintext_on_disk() {
    let base = safe_test_root().join(format!("na0106_plaintext_absent_{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let cfg = base.join("cfg");
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);

    let out = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .args(["identity", "rotate", "--as", "self", "--confirm"])
        .output()
        .expect("identity rotate");
    assert!(out.status.success());
    let s = output_str(&out);
    assert!(s.contains("identity_secret_store"));

    let id_path = cfg.join("identities").join("self_self.json");
    let id_bytes = fs::read(&id_path).unwrap();
    assert!(!contains_bytes(&id_bytes, b"kem_sk"));
}

#[test]
fn migrate_legacy_identity_into_vault() {
    let base = safe_test_root().join(format!("na0106_migrate_legacy_{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let cfg = base.join("cfg");
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);

    let id_dir = cfg.join("identities");
    ensure_dir_700(&id_dir);
    let legacy_path = id_dir.join("self_alice.json");
    let (legacy_json, _pk, sk) = legacy_identity_json();
    fs::write(&legacy_path, &legacy_json).unwrap();
    let route = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .args([
            "contacts",
            "route-set",
            "--label",
            "bob",
            "--route-token",
            ROUTE_TOKEN_BOB,
        ])
        .output()
        .expect("contacts route set");
    assert!(route.status.success(), "{}", output_str(&route));

    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();
    let out = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .args([
            "handshake",
            "init",
            "--as",
            "alice",
            "--peer",
            "bob",
            "--relay",
            &relay,
        ])
        .output()
        .expect("handshake init");
    assert!(
        out.status.success(),
        "handshake init failed: {}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let s = output_str(&out);
    assert!(s.contains("identity_secret_migrate ok=true action=imported"));

    let migrated = fs::read(&legacy_path).unwrap();
    assert!(!contains_bytes(&migrated, b"kem_sk"));
    assert!(!contains_bytes(&migrated, &sk));
}

#[test]
fn migration_requires_vault_fail_closed_no_mutation() {
    let base = safe_test_root().join(format!("na0106_migrate_locked_{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let cfg = base.join("cfg");
    ensure_dir_700(&cfg);

    let id_dir = cfg.join("identities");
    ensure_dir_700(&id_dir);
    let legacy_path = id_dir.join("self_alice.json");
    let (legacy_json, _pk, _sk) = legacy_identity_json();
    fs::write(&legacy_path, &legacy_json).unwrap();
    let before = fs::read(&legacy_path).unwrap();

    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();
    let out = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .args([
            "handshake",
            "init",
            "--as",
            "alice",
            "--peer",
            "bob",
            "--relay",
            &relay,
        ])
        .output()
        .expect("handshake init");
    assert!(!out.status.success());
    let s = output_str(&out);
    assert!(s.contains("event=error code=vault_locked"));
    let after = fs::read(&legacy_path).unwrap();
    assert_eq!(before, after);
}

#[test]
fn no_secrets_in_identity_outputs() {
    let base = safe_test_root().join(format!("na0106_no_secrets_{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let cfg = base.join("cfg");
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);

    let out = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .args(["identity", "rotate", "--as", "self", "--confirm"])
        .output()
        .expect("identity rotate");
    assert!(out.status.success());
    let s = output_str(&out);
    for pat in [
        "TOKEN",
        "SECRET",
        "KEY",
        "PASS",
        "PRIVATE",
        "BEARER",
        "CREDENTIAL",
        "kem_sk",
    ] {
        assert!(!s.contains(pat), "unexpected secret pattern: {pat}");
    }
}
