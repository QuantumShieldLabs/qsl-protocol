// NA-0633 (ENG-0038) security regression test.
//
// This file began as the Phase-0 REPRODUCTION of ENG-0038 (the initiator committing a session to a
// wrong responder). With the C1 fix in place it now asserts the SECURITY PROPERTY: the initiator
// REJECTS a responder whose identity is not the one it pinned/verified out-of-band, and still
// establishes with the genuine responder. Both cases use the REAL qsc binary with NO forged frames.
//
// The fix (NA-0632 report §2 / NA-0633 design-lock C1): the contact carries the peer's full identity
// KEM public key (verified against the human code); the initiator encapsulates to it in A1 and mixes
// the shared secret into pq_init_ss, so a responder that cannot decapsulate (i.e. does not hold the
// pinned identity KEM secret) produces a B1 transcript MAC the initiator rejects.

mod common;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Output;

const ROUTE_TOKEN_ALICE: &str = "route_token_alice_na0633_repro__";
const ROUTE_TOKEN_BOB: &str = "route_token_bob_na0633_repro____";

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

fn output_text(out: &Output) -> String {
    let mut text = String::from_utf8_lossy(&out.stdout).to_string();
    text.push_str(&String::from_utf8_lossy(&out.stderr));
    text
}

fn assert_success(out: &Output) {
    assert!(out.status.success(), "{}", output_text(out));
}

fn run_qsc(iso: &common::TestIsolation, cfg: &Path, args: &[&str]) -> Output {
    let mut cmd = common::qsc_std_command();
    iso.apply_to(&mut cmd);
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .args(args)
        .output()
        .expect("qsc command")
}

fn init_identity(iso: &common::TestIsolation, cfg: &Path, label: &str) {
    assert_success(&run_qsc(
        iso,
        cfg,
        &["identity", "rotate", "--as", label, "--confirm"],
    ));
}

fn identity_field(iso: &common::TestIsolation, cfg: &Path, label: &str, key: &str) -> String {
    let out = run_qsc(iso, cfg, &["identity", "show", "--as", label]);
    assert_success(&out);
    let prefix = format!("{key}=");
    output_text(&out)
        .lines()
        .find_map(|line| line.strip_prefix(prefix.as_str()).map(ToOwned::to_owned))
        .unwrap_or_else(|| panic!("missing {key} in output: {}", output_text(&out)))
}

// NA-0633: provision the peer's full identity KEM key (verified against the fingerprint) — required now.
fn contacts_add(
    iso: &common::TestIsolation,
    cfg: &Path,
    label: &str,
    fp: &str,
    kem_pk: &str,
    token: &str,
) {
    assert_success(&run_qsc(
        iso,
        cfg,
        &[
            "contacts", "add", "--label", label, "--fp", fp, "--kem-pk", kem_pk, "--route-token",
            token,
        ],
    ));
}

fn relay_inbox_set(iso: &common::TestIsolation, cfg: &Path, token: &str) {
    assert_success(&run_qsc(iso, cfg, &["relay", "inbox-set", "--token", token]));
}

fn handshake_init(iso: &common::TestIsolation, alice_cfg: &Path, relay: &str) -> Output {
    run_qsc(
        iso,
        alice_cfg,
        &[
            "handshake", "init", "--as", "alice", "--peer", "bob", "--relay", relay,
            "--suite-mode", "suite-required",
        ],
    )
}

fn handshake_poll(
    iso: &common::TestIsolation,
    cfg: &Path,
    self_label: &str,
    peer: &str,
    relay: &str,
) -> Output {
    run_qsc(
        iso,
        cfg,
        &[
            "handshake", "poll", "--as", self_label, "--peer", peer, "--relay", relay,
            "--max", "4", "--suite-mode", "suite-required",
        ],
    )
}

fn session_path(cfg: &Path, peer: &str) -> PathBuf {
    cfg.join("qsp_sessions").join(format!("{peer}.qsv"))
}

/// Drive alice-initiates-to-"bob" where `responder_cfg`/`responder_label` answers on bob's channel.
/// Returns alice's B1-processing output.
fn drive(
    iso: &common::TestIsolation,
    alice_cfg: &Path,
    responder_cfg: &Path,
    responder_label: &str,
    server: &common::InboxTestServer,
    relay: &str,
) -> Output {
    assert_success(&handshake_init(iso, alice_cfg, relay));
    let a1 = server
        .drain_channel(ROUTE_TOKEN_BOB)
        .pop()
        .expect("A1 on bob's channel");
    server.replace_channel(ROUTE_TOKEN_BOB, vec![a1]);
    assert_success(&handshake_poll(iso, responder_cfg, responder_label, "alice", relay));
    let b1 = server
        .drain_channel(ROUTE_TOKEN_ALICE)
        .pop()
        .expect("B1 on alice's channel");
    server.replace_channel(ROUTE_TOKEN_ALICE, vec![b1]);
    handshake_poll(iso, alice_cfg, "alice", "bob", relay)
}

fn setup(iso: &common::TestIsolation, tag: &str) -> (PathBuf, PathBuf, PathBuf) {
    let base = iso.root.join(tag);
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let alice = base.join("alice");
    let bob = base.join("bob");
    let mallory = base.join("mallory");
    for c in [&alice, &bob, &mallory] {
        ensure_dir_700(c);
        common::init_mock_vault(c);
    }
    init_identity(iso, &alice, "alice");
    init_identity(iso, &bob, "bob");
    init_identity(iso, &mallory, "mallory");
    let alice_fp = identity_field(iso, &alice, "alice", "identity_fp");
    let alice_kem = identity_field(iso, &alice, "alice", "identity_kem_pk");
    let bob_fp = identity_field(iso, &bob, "bob", "identity_fp");
    let bob_kem = identity_field(iso, &bob, "bob", "identity_kem_pk");
    // Alice pins the REAL bob (fingerprint + full identity KEM key). Mallory (a different identity)
    // pins alice so its responder answers, and occupies bob's channel — the on-path position.
    contacts_add(iso, &alice, "bob", &bob_fp, &bob_kem, ROUTE_TOKEN_BOB);
    contacts_add(iso, &bob, "alice", &alice_fp, &alice_kem, ROUTE_TOKEN_ALICE);
    contacts_add(iso, &mallory, "alice", &alice_fp, &alice_kem, ROUTE_TOKEN_ALICE);
    relay_inbox_set(iso, &alice, ROUTE_TOKEN_ALICE);
    (alice, bob, mallory)
}

#[test]
fn eng0038_fixed_initiator_rejects_wrong_responder() {
    let iso = common::TestIsolation::new("na0633_eng0038_reject");
    let (alice_cfg, _bob_cfg, mallory_cfg) = setup(&iso, "reject");
    relay_inbox_set(&iso, &mallory_cfg, ROUTE_TOKEN_BOB); // mallory occupies bob's channel
    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();

    let alice = drive(&iso, &alice_cfg, &mallory_cfg, "mallory", &server, &relay);
    let text = output_text(&alice);

    // THE FIX: alice REJECTS mallory (its B1 transcript MAC cannot match — mallory could not
    // decapsulate the initiator's encapsulation to bob's identity KEM key), and commits NO session.
    assert!(
        text.contains("handshake_reject"),
        "expected alice to reject the wrong responder; got:\n{text}"
    );
    assert!(
        !session_path(&alice_cfg, "bob").exists(),
        "ENG-0038 NOT fixed: alice committed a session to a wrong responder:\n{text}"
    );
    eprintln!("NA0633_ENG0038_WRONG_RESPONDER_REJECTED_OK");
}

#[test]
fn eng0038_fixed_positive_roundtrip_with_genuine_responder() {
    let iso = common::TestIsolation::new("na0633_eng0038_positive");
    let (alice_cfg, bob_cfg, _mallory_cfg) = setup(&iso, "positive");
    relay_inbox_set(&iso, &bob_cfg, ROUTE_TOKEN_BOB); // the genuine bob answers
    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();

    let alice = drive(&iso, &alice_cfg, &bob_cfg, "bob", &server, &relay);
    let text = output_text(&alice);

    // The genuine bob holds its identity KEM secret, decapsulates correctly, and the transcript MAC
    // matches — so honest peers still establish (no regression of the initiator->responder direction).
    assert!(
        !text.contains("handshake_reject"),
        "genuine responder was wrongly rejected:\n{text}"
    );
    assert!(
        session_path(&alice_cfg, "bob").exists(),
        "genuine handshake did not establish a session:\n{text}"
    );
    eprintln!("NA0633_ENG0038_GENUINE_ROUNDTRIP_OK");
}
