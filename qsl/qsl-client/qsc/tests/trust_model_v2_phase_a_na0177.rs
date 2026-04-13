mod common;

use argon2::{Algorithm, Argon2, Params, Version};
use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const CONTACTS_SECRET_KEY: &str = "contacts.json";
const ROUTE_TOKEN_BOB: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const VAULT_MAGIC: &[u8; 6] = b"QSCV01";

#[derive(Serialize, Deserialize)]
struct VaultPayload {
    version: u8,
    secrets: BTreeMap<String, String>,
}

#[derive(Clone)]
struct VaultEnvelope {
    key_source: u8,
    salt: [u8; 16],
    kdf_m_kib: u32,
    kdf_t: u32,
    kdf_p: u32,
    nonce: [u8; 12],
}

fn derive_vault_key(env: &VaultEnvelope) -> [u8; 32] {
    assert_eq!(env.key_source, 1, "expected passphrase vault");
    let params = Params::new(env.kdf_m_kib, env.kdf_t, env.kdf_p, Some(32)).expect("argon2 params");
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let mut key = [0u8; 32];
    argon2
        .hash_password_into(
            common::TEST_MOCK_VAULT_PASSPHRASE.as_bytes(),
            &env.salt,
            &mut key,
        )
        .expect("vault key");
    key
}

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

fn qsc(cfg: &Path) -> Command {
    let mut cmd = common::qsc_std_command();
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_MARK_FORMAT", "plain");
    cmd
}

fn output_text(out: &std::process::Output) -> String {
    let mut s = String::from_utf8_lossy(&out.stdout).to_string();
    s.push_str(&String::from_utf8_lossy(&out.stderr));
    s
}

fn leak_counts(s: &str) -> (usize, usize) {
    let v1_count = s.matches("/v1/").count();
    let mut hex_runs = 0usize;
    let mut run = 0usize;
    for ch in s.chars() {
        if ch.is_ascii_hexdigit() {
            run += 1;
        } else {
            if run >= 32 {
                hex_runs += 1;
            }
            run = 0;
        }
    }
    if run >= 32 {
        hex_runs += 1;
    }
    (v1_count, hex_runs)
}

fn parse_vault(path: &Path) -> (VaultEnvelope, VaultPayload) {
    let bytes = fs::read(path).expect("read vault");
    assert!(bytes.len() >= 6 + 1 + 1 + 1 + (4 * 4), "vault too small");
    assert_eq!(&bytes[..6], VAULT_MAGIC, "vault magic");
    let key_source = bytes[6];
    assert_eq!(bytes[7], 16, "salt len");
    assert_eq!(bytes[8], 12, "nonce len");
    let mut off = 9usize;
    let kdf_m_kib =
        u32::from_le_bytes([bytes[off], bytes[off + 1], bytes[off + 2], bytes[off + 3]]);
    off += 4;
    let kdf_t = u32::from_le_bytes([bytes[off], bytes[off + 1], bytes[off + 2], bytes[off + 3]]);
    off += 4;
    let kdf_p = u32::from_le_bytes([bytes[off], bytes[off + 1], bytes[off + 2], bytes[off + 3]]);
    off += 4;
    let ct_len =
        u32::from_le_bytes([bytes[off], bytes[off + 1], bytes[off + 2], bytes[off + 3]]) as usize;
    off += 4;
    let mut salt = [0u8; 16];
    salt.copy_from_slice(&bytes[off..off + 16]);
    off += 16;
    let mut nonce = [0u8; 12];
    nonce.copy_from_slice(&bytes[off..off + 12]);
    off += 12;
    let ciphertext = &bytes[off..off + ct_len];
    let key_bytes = derive_vault_key(&VaultEnvelope {
        key_source,
        salt,
        kdf_m_kib,
        kdf_t,
        kdf_p,
        nonce,
    });
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&key_bytes));
    let plaintext = cipher
        .decrypt(Nonce::from_slice(&nonce), ciphertext)
        .expect("decrypt vault");
    let payload: VaultPayload = serde_json::from_slice(&plaintext).expect("parse payload");
    (
        VaultEnvelope {
            key_source,
            salt,
            kdf_m_kib,
            kdf_t,
            kdf_p,
            nonce,
        },
        payload,
    )
}

fn write_vault(path: &Path, env: &VaultEnvelope, payload: &VaultPayload) {
    let key_bytes = derive_vault_key(env);
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&key_bytes));
    let plaintext = serde_json::to_vec(payload).expect("serialize payload");
    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&env.nonce), plaintext.as_slice())
        .expect("encrypt payload");
    let mut out = Vec::new();
    out.extend_from_slice(VAULT_MAGIC);
    out.push(env.key_source);
    out.push(16);
    out.push(12);
    out.extend_from_slice(&env.kdf_m_kib.to_le_bytes());
    out.extend_from_slice(&env.kdf_t.to_le_bytes());
    out.extend_from_slice(&env.kdf_p.to_le_bytes());
    out.extend_from_slice(&(ciphertext.len() as u32).to_le_bytes());
    out.extend_from_slice(&env.salt);
    out.extend_from_slice(&env.nonce);
    out.extend_from_slice(&ciphertext);
    fs::write(path, out).expect("write vault");
}

fn inject_legacy_contacts(cfg: &Path, status: &str) {
    let vault_path = cfg.join("vault.qsv");
    let (env, mut payload) = parse_vault(&vault_path);
    let legacy_contacts = json!({
        "peers": {
            "bob": {
                "fp": "ABCD-EFGH-JKMP-QRST-V",
                "status": status,
                "blocked": false,
                "seen_at": null,
                "sig_fp": null,
                "route_token": ROUTE_TOKEN_BOB
            }
        }
    });
    payload
        .secrets
        .insert(CONTACTS_SECRET_KEY.to_string(), legacy_contacts.to_string());
    write_vault(&vault_path, &env, &payload);
}

fn read_contacts_secret(cfg: &Path) -> String {
    let vault_path = cfg.join("vault.qsv");
    let (_env, payload) = parse_vault(&vault_path);
    payload
        .secrets
        .get(CONTACTS_SECRET_KEY)
        .cloned()
        .expect("contacts secret")
}

#[test]
fn legacy_contacts_migrate_to_v2_devices_schema_deterministically() {
    let cfg = unique_test_dir("na0177_phasea_migrate");
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    inject_legacy_contacts(&cfg, "PINNED");

    let list = qsc(&cfg).args(["contacts", "list"]).output().expect("list");
    assert!(list.status.success(), "{}", output_text(&list));
    let show = qsc(&cfg)
        .args(["contacts", "show", "--label", "bob"])
        .output()
        .expect("show");
    assert!(show.status.success(), "{}", output_text(&show));

    let list_text = output_text(&list);
    let show_text = output_text(&show);
    assert!(
        list_text.contains("label=bob state=PINNED blocked=false device_count=1"),
        "list output missing v2 summary: {list_text}"
    );
    assert!(
        show_text.contains("label=bob state=PINNED blocked=false device_count=1")
            && show_text.contains("device=")
            && show_text.contains("state=TRUSTED"),
        "show output missing device details: {show_text}"
    );

    let persisted = read_contacts_secret(&cfg);
    let v: Value = serde_json::from_str(&persisted).expect("contacts json");
    let devices = &v["peers"]["bob"]["devices"];
    assert!(
        devices.is_array(),
        "expected devices[] in v2 store: {persisted}"
    );
    assert_eq!(devices.as_array().expect("devices").len(), 1);
    assert_eq!(devices[0]["state"], "TRUSTED");
    assert!(
        devices[0]["device_id"]
            .as_str()
            .map(|s| s.len() <= 12)
            .unwrap_or(false),
        "device_id must be short"
    );
}

#[test]
fn trust_gate_semantics_unchanged_after_migration() {
    let cfg = unique_test_dir("na0177_phasea_gate");
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);

    let payload = cfg.join("msg.bin");
    fs::write(&payload, b"hello").expect("payload");

    inject_legacy_contacts(&cfg, "VERIFIED");
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
    assert!(!blocked.status.success(), "verified should fail-closed");
    let blocked_text = output_text(&blocked);
    assert!(
        blocked_text.contains("QSC_SEND_BLOCKED reason=no_trusted_device peer=bob"),
        "missing no_trusted_device marker: {blocked_text}"
    );

    inject_legacy_contacts(&cfg, "PINNED");
    let allowed_gate = qsc(&cfg)
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
        .expect("send trusted");
    let allowed_text = output_text(&allowed_gate);
    assert!(
        !allowed_text.contains("QSC_SEND_BLOCKED reason=no_trusted_device"),
        "trusted migrated contact should pass trust gate: {allowed_text}"
    );
}

#[test]
fn contacts_outputs_remain_secret_safe_after_v2_migration() {
    let cfg = unique_test_dir("na0177_phasea_leaks");
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    inject_legacy_contacts(&cfg, "PINNED");

    let list = qsc(&cfg).args(["contacts", "list"]).output().expect("list");
    let show = qsc(&cfg)
        .args(["contacts", "show", "--label", "bob"])
        .output()
        .expect("show");
    assert!(list.status.success(), "{}", output_text(&list));
    assert!(show.status.success(), "{}", output_text(&show));
    let mut combined = output_text(&list);
    combined.push_str(&output_text(&show));
    let (v1_count, long_hex_count) = leak_counts(&combined);
    assert_eq!(v1_count, 0, "unexpected /v1/ output: {combined}");
    assert_eq!(long_hex_count, 0, "unexpected long-hex output: {combined}");
}
