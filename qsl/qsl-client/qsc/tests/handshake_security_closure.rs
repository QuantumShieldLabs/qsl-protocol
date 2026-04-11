use quantumshield_refimpl::crypto::stdcrypto::StdCrypto;
use quantumshield_refimpl::suite2::establish::init_from_base_handshake;
use quantumshield_refimpl::suite2::types::{SUITE2_PROTOCOL_VERSION, SUITE2_SUITE_ID};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

mod common;

const ROUTE_TOKEN_ALICE: &str = "route_token_alice_abcdefghijklmnop";
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

fn session_path(cfg: &Path, peer: &str) -> PathBuf {
    cfg.join("qsp_sessions").join(format!("{}.qsv", peer))
}

fn run_qsc(cfg: &Path, args: &[&str]) -> std::process::Output {
    let mut cmd = common::qsc_std_command();
    cmd.env("QSC_CONFIG_DIR", cfg)
        .args(args)
        .output()
        .expect("qsc command")
}

fn output_text(out: &std::process::Output) -> String {
    let mut text = String::from_utf8_lossy(&out.stdout).to_string();
    text.push_str(&String::from_utf8_lossy(&out.stderr));
    text
}

fn init_identity(cfg: &Path, label: &str) {
    let out = run_qsc(cfg, &["identity", "rotate", "--as", label, "--confirm"]);
    assert!(out.status.success(), "{}", output_text(&out));
}

fn identity_fp(cfg: &Path, label: &str) -> String {
    let out = run_qsc(cfg, &["identity", "show", "--as", label]);
    assert!(out.status.success(), "{}", output_text(&out));
    output_text(&out)
        .lines()
        .find_map(|line| line.strip_prefix("identity_fp="))
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| panic!("missing identity_fp in output: {}", output_text(&out)))
}

fn contacts_add_with_route(cfg: &Path, label: &str, token: &str) {
    let out = run_qsc(
        cfg,
        &[
            "contacts",
            "route-set",
            "--label",
            label,
            "--route-token",
            token,
        ],
    );
    assert!(
        out.status.success(),
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}

fn contacts_add_authenticated_with_route(cfg: &Path, label: &str, fp: &str, token: &str) {
    let out = run_qsc(
        cfg,
        &[
            "contacts",
            "add",
            "--label",
            label,
            "--fp",
            fp,
            "--route-token",
            token,
        ],
    );
    assert!(out.status.success(), "{}", output_text(&out));
}

fn seed_authenticated_pair(alice_cfg: &Path, bob_cfg: &Path) {
    init_identity(alice_cfg, "alice");
    init_identity(bob_cfg, "bob");
    let alice_fp = identity_fp(alice_cfg, "alice");
    let bob_fp = identity_fp(bob_cfg, "bob");
    contacts_add_authenticated_with_route(alice_cfg, "bob", bob_fp.as_str(), ROUTE_TOKEN_BOB);
    contacts_add_authenticated_with_route(bob_cfg, "alice", alice_fp.as_str(), ROUTE_TOKEN_ALICE);
}

fn relay_inbox_set(cfg: &Path, token: &str) {
    let out = run_qsc(cfg, &["relay", "inbox-set", "--token", token]);
    assert!(
        out.status.success(),
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn handshake_rejects_tampered_transcript_no_mutation() {
    let base = safe_test_root().join(format!("na0154_hs_tamper_{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let alice_cfg = base.join("alice");
    let bob_cfg = base.join("bob");
    ensure_dir_700(&alice_cfg);
    ensure_dir_700(&bob_cfg);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);
    seed_authenticated_pair(&alice_cfg, &bob_cfg);
    relay_inbox_set(&alice_cfg, ROUTE_TOKEN_ALICE);
    relay_inbox_set(&bob_cfg, ROUTE_TOKEN_BOB);

    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();

    let out_init = run_qsc(
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
        ],
    );
    assert!(out_init.status.success());

    let out_bob = run_qsc(
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
        ],
    );
    assert!(out_bob.status.success());

    // Tamper B1 transcript/MAC bytes before Alice consumes it.
    let mut items = server.drain_channel(ROUTE_TOKEN_ALICE);
    assert_eq!(items.len(), 1);
    if let Some(b) = items[0].get_mut(8) {
        *b = b.wrapping_add(1);
    }
    server.replace_channel(ROUTE_TOKEN_ALICE, items);

    let out_alice = run_qsc(
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
        ],
    );
    assert!(out_alice.status.success());
    assert!(!session_path(&alice_cfg, "bob").exists());

    let combined = String::from_utf8_lossy(&out_alice.stdout).to_string()
        + &String::from_utf8_lossy(&out_alice.stderr);
    assert!(combined.contains("handshake_reject"), "{}", combined);
}

#[test]
fn handshake_pinned_identity_mismatch_fails() {
    let base = safe_test_root().join(format!("na0154_hs_pin_mismatch_{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let alice_cfg = base.join("alice");
    let alice2_cfg = base.join("alice2");
    let bob_cfg = base.join("bob");
    ensure_dir_700(&alice_cfg);
    ensure_dir_700(&alice2_cfg);
    ensure_dir_700(&bob_cfg);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&alice2_cfg);
    common::init_mock_vault(&bob_cfg);
    init_identity(&alice_cfg, "alice");
    init_identity(&alice2_cfg, "alice");
    init_identity(&bob_cfg, "bob");
    let alice_fp = identity_fp(&alice_cfg, "alice");
    let bob_fp = identity_fp(&bob_cfg, "bob");
    contacts_add_authenticated_with_route(&alice_cfg, "bob", bob_fp.as_str(), ROUTE_TOKEN_BOB);
    contacts_add_authenticated_with_route(&alice2_cfg, "bob", bob_fp.as_str(), ROUTE_TOKEN_BOB);
    contacts_add_authenticated_with_route(&bob_cfg, "alice", alice_fp.as_str(), ROUTE_TOKEN_ALICE);
    relay_inbox_set(&alice_cfg, ROUTE_TOKEN_ALICE);
    relay_inbox_set(&alice2_cfg, ROUTE_TOKEN_ALICE);
    relay_inbox_set(&bob_cfg, ROUTE_TOKEN_BOB);

    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();

    // Establish first session and pin Alice at Bob.
    assert!(run_qsc(
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
        ],
    )
    .status
    .success());
    assert!(run_qsc(
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
        ],
    )
    .status
    .success());
    assert!(run_qsc(
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
        ],
    )
    .status
    .success());
    assert!(run_qsc(
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
        ],
    )
    .status
    .success());
    let session_before = fs::read(session_path(&bob_cfg, "alice")).expect("session before");

    // New Alice identity must be rejected against Bob's pin.
    let out_init2 = run_qsc(
        &alice2_cfg,
        &[
            "handshake",
            "init",
            "--as",
            "alice",
            "--peer",
            "bob",
            "--relay",
            &relay,
        ],
    );
    assert!(out_init2.status.success());
    let out_bob2 = run_qsc(
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
        ],
    );
    assert!(out_bob2.status.success());

    let session_after = fs::read(session_path(&bob_cfg, "alice")).expect("session after");
    assert_eq!(session_before, session_after);

    let combined = String::from_utf8_lossy(&out_bob2.stdout).to_string()
        + &String::from_utf8_lossy(&out_bob2.stderr);
    assert!(combined.contains("peer_mismatch"), "{}", combined);
}

#[test]
fn handshake_unknown_peer_rejects_without_pending_or_session_state() {
    let base = safe_test_root().join(format!("na0221_hs_unknown_peer_{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let alice_cfg = base.join("alice");
    let bob_cfg = base.join("bob");
    ensure_dir_700(&alice_cfg);
    ensure_dir_700(&bob_cfg);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);
    contacts_add_with_route(&alice_cfg, "bob", ROUTE_TOKEN_BOB);
    contacts_add_with_route(&bob_cfg, "alice", ROUTE_TOKEN_ALICE);
    relay_inbox_set(&alice_cfg, ROUTE_TOKEN_ALICE);
    relay_inbox_set(&bob_cfg, ROUTE_TOKEN_BOB);

    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();

    let out_init = run_qsc(
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
        ],
    );
    assert!(!out_init.status.success(), "{}", output_text(&out_init));
    assert!(!session_path(&alice_cfg, "bob").exists());
    assert!(!session_path(&bob_cfg, "alice").exists());
    assert!(
        server.drain_channel(ROUTE_TOKEN_BOB).is_empty(),
        "unknown peer reject must not emit A1"
    );
    let combined = output_text(&out_init);
    assert!(combined.contains("identity_unknown"), "{}", combined);
    assert!(combined.contains("handshake_reject"), "{}", combined);
}

#[test]
fn handshake_initializer_rejects_missing_authenticated_establishment_commitment() {
    let c = StdCrypto;
    let err = match init_from_base_handshake(
        &c,
        true,
        SUITE2_PROTOCOL_VERSION,
        SUITE2_SUITE_ID,
        &[0x11; 16],
        &[0x22; 32],
        &[0x33; 32],
        &[0x44; 32],
        &[0x55; 32],
        false,
    ) {
        Ok(_) => panic!("unauthenticated establish must reject"),
        Err(err) => err,
    };
    assert_eq!(err, "REJECT_S2_ESTABLISH_UNAUTHENTICATED");
}
