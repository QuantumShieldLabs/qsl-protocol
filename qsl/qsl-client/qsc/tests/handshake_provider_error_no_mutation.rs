mod common;

use argon2::{Algorithm, Argon2, Params, Version};
use chacha20poly1305::aead::{Aead, AeadCore};
use chacha20poly1305::KeyInit;
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use rand_core::OsRng;
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

const ROUTE_TOKEN_ALICE: &str = "route_token_alice_na0436_abcdefghijkl";
const ROUTE_TOKEN_BOB: &str = "route_token_bob_na0436_abcdefghijklmn";

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

fn output_text(out: &std::process::Output) -> String {
    let mut text = String::from_utf8_lossy(&out.stdout).to_string();
    text.push_str(&String::from_utf8_lossy(&out.stderr));
    text
}

fn run_qsc(iso: &common::TestIsolation, cfg: &Path, args: &[&str]) -> std::process::Output {
    let mut cmd = common::qsc_std_command();
    iso.apply_to(&mut cmd);
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .args(args)
        .output()
        .expect("qsc command")
}

fn init_identity(iso: &common::TestIsolation, cfg: &Path, label: &str) {
    let out = run_qsc(
        iso,
        cfg,
        &["identity", "rotate", "--as", label, "--confirm"],
    );
    assert!(out.status.success(), "{}", output_text(&out));
}

fn identity_fp(iso: &common::TestIsolation, cfg: &Path, label: &str) -> String {
    let out = run_qsc(iso, cfg, &["identity", "show", "--as", label]);
    assert!(out.status.success(), "{}", output_text(&out));
    output_text(&out)
        .lines()
        .find_map(|line| line.strip_prefix("identity_fp="))
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| panic!("missing identity_fp in output: {}", output_text(&out)))
}

fn identity_kem_pk(iso: &common::TestIsolation, cfg: &Path, label: &str) -> String {
    let out = run_qsc(iso, cfg, &["identity", "show", "--as", label]);
    assert!(out.status.success(), "{}", output_text(&out));
    output_text(&out)
        .lines()
        .find_map(|line| line.strip_prefix("identity_kem_pk="))
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| panic!("missing identity_kem_pk in output: {}", output_text(&out)))
}

fn identity_sig_pk(iso: &common::TestIsolation, cfg: &Path, label: &str) -> String {
    let out = run_qsc(iso, cfg, &["identity", "show", "--as", label]);
    assert!(out.status.success(), "{}", output_text(&out));
    output_text(&out)
        .lines()
        .find_map(|line| line.strip_prefix("identity_sig_pk="))
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| panic!("missing identity_sig_pk in output: {}", output_text(&out)))
}

fn contacts_add_authenticated_with_route(
    iso: &common::TestIsolation,
    cfg: &Path,
    label: &str,
    fp: &str, kem_pk: &str, sig_pk: &str,
    token: &str,
) {
    let out = run_qsc(
        iso,
        cfg,
        &[
            "contacts",
            "add",
            "--label",
            label,
            "--fp",
            fp,
            "--kem-pk",
            kem_pk,
            "--sig-pk",
            sig_pk,
            "--route-token",
            token,
        ],
    );
    assert!(out.status.success(), "{}", output_text(&out));
}

fn relay_inbox_set(iso: &common::TestIsolation, cfg: &Path, token: &str) {
    let out = run_qsc(iso, cfg, &["relay", "inbox-set", "--token", token]);
    assert!(out.status.success(), "{}", output_text(&out));
}

fn seed_authenticated_pair(iso: &common::TestIsolation, alice_cfg: &Path, bob_cfg: &Path) {
    init_identity(iso, alice_cfg, "alice");
    init_identity(iso, bob_cfg, "bob");
    let alice_fp = identity_fp(iso, alice_cfg, "alice");
    let alice_kem = identity_kem_pk(iso, alice_cfg, "alice");
    let alice_sig = identity_sig_pk(iso, alice_cfg, "alice");
    let bob_fp = identity_fp(iso, bob_cfg, "bob");
    let bob_kem = identity_kem_pk(iso, bob_cfg, "bob");
    let bob_sig = identity_sig_pk(iso, bob_cfg, "bob");
    contacts_add_authenticated_with_route(iso, alice_cfg, "bob", bob_fp.as_str(), bob_kem.as_str(), bob_sig.as_str(), ROUTE_TOKEN_BOB);
    contacts_add_authenticated_with_route(
        iso,
        bob_cfg,
        "alice",
        alice_fp.as_str(),
        alice_kem.as_str(), alice_sig.as_str(),
        ROUTE_TOKEN_ALICE,
    );
    relay_inbox_set(iso, alice_cfg, ROUTE_TOKEN_ALICE);
    relay_inbox_set(iso, bob_cfg, ROUTE_TOKEN_BOB);
}

fn new_pair(iso: &common::TestIsolation, root: &Path, tag: &str) -> (PathBuf, PathBuf) {
    let alice_cfg = root.join(format!("{tag}-alice"));
    let bob_cfg = root.join(format!("{tag}-bob"));
    ensure_dir_700(&alice_cfg);
    ensure_dir_700(&bob_cfg);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);
    seed_authenticated_pair(iso, &alice_cfg, &bob_cfg);
    (alice_cfg, bob_cfg)
}

fn session_path(cfg: &Path, peer: &str) -> PathBuf {
    cfg.join("qsp_sessions").join(format!("{peer}.qsv"))
}

fn path_bytes(path: &Path) -> Option<Vec<u8>> {
    match fs::read(path) {
        Ok(v) => Some(v),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
        Err(e) => panic!("read {} failed: {e}", path.display()),
    }
}

fn derive_mock_vault_key(bytes: &[u8]) -> ([u8; 32], [u8; 16], u8, u32, u32, u32, usize, usize) {
    assert!(bytes.len() > 25, "vault envelope too short");
    assert_eq!(&bytes[0..6], b"QSCV01");
    let key_source = bytes[6];
    assert_eq!(key_source, 1, "expected passphrase vault");
    let salt_len = bytes[7] as usize;
    let nonce_len = bytes[8] as usize;
    assert_eq!(salt_len, 16);
    assert_eq!(nonce_len, 12);
    let kdf_m_kib = u32::from_le_bytes([bytes[9], bytes[10], bytes[11], bytes[12]]);
    let kdf_t = u32::from_le_bytes([bytes[13], bytes[14], bytes[15], bytes[16]]);
    let kdf_p = u32::from_le_bytes([bytes[17], bytes[18], bytes[19], bytes[20]]);
    let mut salt = [0u8; 16];
    salt.copy_from_slice(&bytes[25..25 + salt_len]);
    let params = Params::new(kdf_m_kib, kdf_t, kdf_p, Some(32)).expect("argon2 params");
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let mut key = [0u8; 32];
    argon2
        .hash_password_into(
            common::TEST_MOCK_VAULT_PASSPHRASE.as_bytes(),
            &salt,
            &mut key,
        )
        .expect("vault key");
    (
        key, salt, key_source, kdf_m_kib, kdf_t, kdf_p, salt_len, nonce_len,
    )
}

fn read_mock_vault_json(cfg: &Path) -> Value {
    let bytes = fs::read(cfg.join("vault.qsv")).expect("vault read");
    let (key, _salt, _key_source, _m, _t, _p, salt_len, nonce_len) = derive_mock_vault_key(&bytes);
    let ct_len = u32::from_le_bytes([bytes[21], bytes[22], bytes[23], bytes[24]]) as usize;
    let mut off = 25 + salt_len;
    let nonce = &bytes[off..off + nonce_len];
    off += nonce_len;
    let ciphertext = &bytes[off..off + ct_len];
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&key));
    let plaintext = cipher
        .decrypt(Nonce::from_slice(nonce), ciphertext)
        .expect("vault decrypt");
    serde_json::from_slice(&plaintext).expect("vault json")
}

fn write_mock_vault_json(cfg: &Path, payload: &Value) {
    let path = cfg.join("vault.qsv");
    let bytes = fs::read(&path).expect("vault read for write");
    let (key, salt, key_source, kdf_m_kib, kdf_t, kdf_p, _salt_len, _nonce_len) =
        derive_mock_vault_key(&bytes);
    let plaintext = serde_json::to_vec(payload).expect("vault json serialize");
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&key));
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
    let ciphertext = cipher
        .encrypt(&nonce, plaintext.as_ref())
        .expect("vault encrypt");

    let mut out = Vec::with_capacity(25 + salt.len() + nonce.len() + ciphertext.len());
    out.extend_from_slice(b"QSCV01");
    out.push(key_source);
    out.push(16);
    out.push(12);
    out.extend_from_slice(&kdf_m_kib.to_le_bytes());
    out.extend_from_slice(&kdf_t.to_le_bytes());
    out.extend_from_slice(&kdf_p.to_le_bytes());
    out.extend_from_slice(&(ciphertext.len() as u32).to_le_bytes());
    out.extend_from_slice(&salt);
    out.extend_from_slice(nonce.as_slice());
    out.extend_from_slice(&ciphertext);
    fs::write(&path, out).expect("vault write");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&path, fs::Permissions::from_mode(0o600)).unwrap();
    }
}

fn read_mock_vault_secret(cfg: &Path, name: &str) -> Option<String> {
    read_mock_vault_json(cfg)
        .get("secrets")
        .and_then(|v| v.get(name))
        .and_then(|v| v.as_str())
        .map(ToOwned::to_owned)
}

fn write_mock_vault_secret(cfg: &Path, name: &str, value: String) {
    let mut payload = read_mock_vault_json(cfg);
    let secrets = payload
        .get_mut("secrets")
        .and_then(|v| v.as_object_mut())
        .expect("vault secrets object");
    secrets.insert(name.to_string(), Value::String(value));
    write_mock_vault_json(cfg, &payload);
}

fn corrupt_pending_kem_secret(cfg: &Path, self_label: &str, peer: &str) -> String {
    let key = format!("handshake.pending.{self_label}.{peer}");
    let pending_raw = read_mock_vault_secret(cfg, &key).expect("pending secret");
    let mut pending: Value = serde_json::from_str(&pending_raw).expect("pending json");
    assert_eq!(
        pending.get("role").and_then(|v| v.as_str()),
        Some("initiator")
    );

    pending["kem_sk"] = json!([0]);
    let mutated = serde_json::to_string(&pending).expect("pending serialize");
    write_mock_vault_secret(cfg, &key, mutated.clone());
    mutated
}

fn assert_reject_output(text: &str, reason: &str) {
    assert!(
        text.contains("event=handshake_reject"),
        "missing reject marker: {text}"
    );
    assert!(text.contains(reason), "missing reason {reason}: {text}");
    assert!(
        !text.contains("event=handshake_complete"),
        "reject completed handshake: {text}"
    );
    assert!(
        !text.contains("event=recv_commit"),
        "reject emitted recv_commit: {text}"
    );
    assert!(
        !text.contains("event=qsp_unpack ok=true"),
        "reject emitted qsp output: {text}"
    );
    for forbidden in [
        ROUTE_TOKEN_ALICE,
        ROUTE_TOKEN_BOB,
        common::TEST_MOCK_VAULT_PASSPHRASE,
        "QSC_DESKTOP_SESSION_PASSPHRASE",
        "panicked",
        "stack backtrace",
        "thread '",
    ] {
        assert!(
            !text.contains(forbidden),
            "forbidden output fragment leaked: {forbidden}: {text}"
        );
    }
}

#[test]
fn pq_decap_failed_reject_does_not_mutate_sessions_or_pending_state() {
    // NA-0436 is intentionally decap-only. D278/NA-0435 leave
    // pq_encap_failed as a defensive branch caveat under current provider/API
    // reachability, so this test must not claim coverage for that branch.
    let iso = common::TestIsolation::new("na0436_pq_decap_failed_no_mutation");
    let base = iso.root.join("run");
    ensure_dir_700(&base);
    let (alice_cfg, bob_cfg) = new_pair(&iso, &base, "decap");

    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();

    let alice_init = run_qsc(
        &iso,
        &alice_cfg,
        &[
            "handshake",
            "init",
            "--as",
            "alice",
            "--peer",
            "bob",
            "--relay",
            &relay,
            "--suite-mode",
            "suite-required",
        ],
    );
    assert!(alice_init.status.success(), "{}", output_text(&alice_init));
    let a1 = server
        .drain_channel(ROUTE_TOKEN_BOB)
        .pop()
        .expect("A1 queued");
    server.replace_channel(ROUTE_TOKEN_BOB, vec![a1]);

    let bob_poll = run_qsc(
        &iso,
        &bob_cfg,
        &[
            "handshake",
            "poll",
            "--as",
            "bob",
            "--peer",
            "alice",
            "--relay",
            &relay,
            "--max",
            "4",
            "--suite-mode",
            "suite-required",
        ],
    );
    assert!(bob_poll.status.success(), "{}", output_text(&bob_poll));
    let b1 = server
        .drain_channel(ROUTE_TOKEN_ALICE)
        .pop()
        .expect("B1 queued");
    server.replace_channel(ROUTE_TOKEN_ALICE, vec![b1]);

    let alice_pending_key = "handshake.pending.alice.bob";
    let bob_pending_key = "handshake.pending.bob.alice";
    let mutated_pending = corrupt_pending_kem_secret(&alice_cfg, "alice", "bob");
    assert_eq!(
        read_mock_vault_secret(&alice_cfg, alice_pending_key).as_deref(),
        Some(mutated_pending.as_str())
    );

    let alice_session_before = path_bytes(&session_path(&alice_cfg, "bob"));
    let bob_session_before = path_bytes(&session_path(&bob_cfg, "alice"));
    assert!(
        alice_session_before.is_none(),
        "alice session existed before reject"
    );
    assert!(
        bob_session_before.is_none(),
        "bob session existed before confirm"
    );

    let alice_pending_before =
        read_mock_vault_secret(&alice_cfg, alice_pending_key).expect("alice pending before");
    let bob_pending_before =
        read_mock_vault_secret(&bob_cfg, bob_pending_key).expect("bob pending before");
    let alice_vault_before = fs::read(alice_cfg.join("vault.qsv")).expect("alice vault before");
    let bob_vault_before = fs::read(bob_cfg.join("vault.qsv")).expect("bob vault before");

    let alice_reject = run_qsc(
        &iso,
        &alice_cfg,
        &[
            "handshake",
            "poll",
            "--as",
            "alice",
            "--peer",
            "bob",
            "--relay",
            &relay,
            "--max",
            "4",
            "--suite-mode",
            "suite-required",
        ],
    );
    assert!(
        alice_reject.status.success(),
        "{}",
        output_text(&alice_reject)
    );
    let reject_text = output_text(&alice_reject);
    assert_reject_output(&reject_text, "pq_decap_failed");

    assert_eq!(
        path_bytes(&session_path(&alice_cfg, "bob")),
        alice_session_before,
        "alice session store changed on pq_decap_failed"
    );
    assert_eq!(
        path_bytes(&session_path(&bob_cfg, "alice")),
        bob_session_before,
        "bob session store changed during alice reject"
    );
    assert_eq!(
        read_mock_vault_secret(&alice_cfg, alice_pending_key).as_deref(),
        Some(alice_pending_before.as_str()),
        "alice initiator pending state changed on pq_decap_failed"
    );
    assert_eq!(
        read_mock_vault_secret(&bob_cfg, bob_pending_key).as_deref(),
        Some(bob_pending_before.as_str()),
        "bob responder pending state changed during alice reject"
    );
    assert_eq!(
        fs::read(alice_cfg.join("vault.qsv")).expect("alice vault after"),
        alice_vault_before,
        "alice vault bytes changed on pq_decap_failed"
    );
    assert_eq!(
        fs::read(bob_cfg.join("vault.qsv")).expect("bob vault after"),
        bob_vault_before,
        "bob vault bytes changed during alice reject"
    );
    assert!(
        server.drain_channel(ROUTE_TOKEN_BOB).is_empty(),
        "pq_decap_failed emitted A2"
    );

    println!("NA0436_PQ_DECAP_FAILED_MARKER_OK");
    println!("NA0436_NO_SESSION_MUTATION_ON_DECAP_REJECT_OK");
    println!("NA0436_PENDING_STORE_NO_MUTATION_ON_DECAP_REJECT_OK");
    println!("NA0436_PQ_ENCAP_FAILED_CAVEAT_PRESERVED_OK");
    println!("NA0436_NO_RUNTIME_HOOK_USED_OK");
}
