// NA-0633 (ENG-0038) Phase 0 REPRODUCTION — PoC-first (directive D570).
//
// Demonstrates ENG-0038 end-to-end with the REAL qsc binary and NO forged frames: the initiator
// (alice) commits a Suite-2 session to a responder whose identity is NOT the one she pinned and
// verified out-of-band. A second real identity (mallory) — different KEM key AND different signing
// key from the pinned "bob" — occupies bob's channel, answers alice's handshake, and alice ACCEPTS
// it as "bob".
//
// Why alice accepts (the defect, NA-0632 report §2): the responder's only identity credential in B1
// is its ML-DSA signing key; alice verifies the signature under the key the responder SENT (mallory's,
// self-consistent); the OPTIONAL sig_fp pin that would catch a substituted key is structurally None on
// the shipped path; and alice's REQUIRED KEM pin is tautological/inert in the B->A direction (the
// responder sends no KEM key). Bob's real keys never touch this exchange.
//
// This test asserts the CURRENT (vulnerable) behavior so it is GREEN and documents the bug. The
// ENG-0038 fix (Phase 2) will make alice REJECT mallory; at that point this test's assertion is
// inverted (see the FIX marker below) — that inversion, committed alongside the fix, is the
// security-property regression test.

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

fn identity_fp(iso: &common::TestIsolation, cfg: &Path, label: &str) -> String {
    let out = run_qsc(iso, cfg, &["identity", "show", "--as", label]);
    assert_success(&out);
    output_text(&out)
        .lines()
        .find_map(|line| line.strip_prefix("identity_fp="))
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| panic!("missing identity_fp: {}", output_text(&out)))
}

fn contacts_add(iso: &common::TestIsolation, cfg: &Path, label: &str, fp: &str, token: &str) {
    assert_success(&run_qsc(
        iso,
        cfg,
        &[
            "contacts", "add", "--label", label, "--fp", fp, "--route-token", token,
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

#[test]
fn eng0038_initiator_accepts_wrong_responder_reproduction() {
    let iso = common::TestIsolation::new("na0633_eng0038_repro");
    let base = iso.root.join("repro");
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let alice_cfg = base.join("alice");
    let bob_cfg = base.join("bob"); // the REAL bob — only used to mint the verification code alice pins
    let mallory_cfg = base.join("mallory"); // the wrong responder: different KEM AND signing keys
    for c in [&alice_cfg, &bob_cfg, &mallory_cfg] {
        ensure_dir_700(c);
        common::init_mock_vault(c);
    }

    init_identity(&iso, &alice_cfg, "alice");
    init_identity(&iso, &bob_cfg, "bob");
    init_identity(&iso, &mallory_cfg, "mallory");
    let alice_fp = identity_fp(&iso, &alice_cfg, "alice");
    let bob_fp = identity_fp(&iso, &bob_cfg, "bob");
    let mallory_fp = identity_fp(&iso, &mallory_cfg, "mallory");
    assert_ne!(
        bob_fp, mallory_fp,
        "test setup: mallory must have a different identity than bob"
    );

    // Alice pins the REAL bob's out-of-band verification code (KEM fingerprint) and routes to bob's
    // channel. This is the diligent user who verified bob's code.
    contacts_add(&iso, &alice_cfg, "bob", &bob_fp, ROUTE_TOKEN_BOB);
    // Mallory pins alice (so mallory's responder accepts alice's A1) and routes replies to alice's
    // inbox. Mallory OCCUPIES bob's channel — the on-path position (e.g., the relay).
    contacts_add(&iso, &mallory_cfg, "alice", &alice_fp, ROUTE_TOKEN_ALICE);
    relay_inbox_set(&iso, &alice_cfg, ROUTE_TOKEN_ALICE);
    relay_inbox_set(&iso, &mallory_cfg, ROUTE_TOKEN_BOB);

    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();

    // Alice initiates to "bob"; A1 lands on bob's channel, which mallory occupies.
    assert_success(&handshake_init(&iso, &alice_cfg, &relay));
    let a1 = server
        .drain_channel(ROUTE_TOKEN_BOB)
        .pop()
        .expect("A1 on bob's channel");
    server.replace_channel(ROUTE_TOKEN_BOB, vec![a1]);

    // Mallory (NOT bob) answers: accepts alice's A1 (mallory pinned alice) and signs B1 with MALLORY's
    // key. No forged bytes — this is the real qsc responder path under mallory's identity.
    assert_success(&handshake_poll(&iso, &mallory_cfg, "mallory", "alice", &relay));
    let b1 = server
        .drain_channel(ROUTE_TOKEN_ALICE)
        .pop()
        .expect("B1 from mallory on alice's channel");
    server.replace_channel(ROUTE_TOKEN_ALICE, vec![b1]);

    // Alice processes mallory's B1 as "bob"'s response.
    let alice = handshake_poll(&iso, &alice_cfg, "alice", "bob", &relay);
    let text = output_text(&alice);

    // ===== ENG-0038 (the vulnerable behavior this lane exists to fix) =====
    // Alice ACCEPTS mallory as "bob": no peer_mismatch, and a Suite-2 session is committed — even
    // though mallory's identity != the pinned/verified bob. The out-of-band code gave no protection.
    assert!(
        !text.contains("peer_mismatch") && !text.contains("handshake_reject"),
        "ENG-0038 did NOT reproduce (alice rejected the wrong responder) — investigate before fixing:\n{text}"
    );
    assert!(
        session_path(&alice_cfg, "bob").exists(),
        "ENG-0038 did NOT reproduce (no session committed):\n{text}"
    );
    // FIX marker (Phase 2/3): once ENG-0038 is fixed, alice MUST reject mallory. Invert the two
    // assertions above (expect handshake_reject and assert_no_session) — that becomes the security
    // regression test. Kept as the CURRENT (vulnerable) behavior here to prove the bug reproduces.
    eprintln!("NA0633_ENG0038_REPRODUCED_OK bob_fp={bob_fp} mallory_fp={mallory_fp}");
}
