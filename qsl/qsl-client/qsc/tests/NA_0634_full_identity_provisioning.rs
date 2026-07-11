// NA-0634 (D571 Decision 2a) security test: FULL-IDENTITY provisioning + the REQUIRED responder sig-pin.
//
// C1 (NA-0633) authenticated the responder's KEM identity to the initiator. NA-0634 completes the interim:
// the single verification code now binds BOTH the identity KEM key AND the signing key
// (fingerprint(kem_pk, sig_pk)), `sig_fp` is finally populated, and the initiator REQUIRES the responder's
// signing key to match the pinned `sig_fp` at B1 (fail-closed) — closing the ENG-0038 signing-key
// asymmetry that C1 left open. Both cases use the REAL qsc binary with NO forged frames.
//
// - positive: alice pins bob's REAL full identity; the genuine bob establishes (no regression).
// - wrong-signing-key: alice pins {bob.kem, mallory.sig} (correct KEM so C1 PASSES, but a signing key that
//   is not bob's). The genuine bob answers, passes C1 and the transcript MAC, but its signing key does not
//   match the pinned sig_fp, so the initiator REJECTS at B1 (responder_sig_mismatch) and commits no session.

mod common;

use sha2::{Digest, Sha512};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Output;

const ROUTE_TOKEN_ALICE: &str = "route_token_alice_na0634_fullid__";
const ROUTE_TOKEN_BOB: &str = "route_token_bob_na0634_fullid____";

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

fn hex_decode(s: &str) -> Vec<u8> {
    let s = s.trim();
    assert!(s.len() % 2 == 0, "odd-length hex");
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).expect("hex digit"))
        .collect()
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// Mirror of `identity_fingerprint_from_identity`: QSCFP- + hex(sha512(kem_pk || sig_pk)[..16]).
/// `contacts add` accepts the raw QSCFP- form (identity_pin_matches_seen), so no Crockford formatting.
fn combined_fp(kem_hex: &str, sig_hex: &str) -> String {
    let mut buf = hex_decode(kem_hex);
    buf.extend_from_slice(&hex_decode(sig_hex));
    let hash = Sha512::digest(&buf);
    format!("QSCFP-{}", hex_encode(&hash[..16]))
}

#[allow(clippy::too_many_arguments)]
fn contacts_add(
    iso: &common::TestIsolation,
    cfg: &Path,
    label: &str,
    fp: &str,
    kem_pk: &str,
    sig_pk: &str,
    token: &str,
) {
    assert_success(&run_qsc(
        iso,
        cfg,
        &[
            "contacts", "add", "--label", label, "--fp", fp, "--kem-pk", kem_pk, "--sig-pk", sig_pk,
            "--route-token", token,
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

struct Party {
    cfg: PathBuf,
    fp: String,
    kem: String,
    sig: String,
}

/// Init alice/bob/mallory identities and return each party's cfg + published full identity.
fn init_parties(iso: &common::TestIsolation, tag: &str) -> (Party, Party, Party) {
    let base = iso.root.join(tag);
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let mut out = Vec::new();
    for label in ["alice", "bob", "mallory"] {
        let cfg = base.join(label);
        ensure_dir_700(&cfg);
        common::init_mock_vault(&cfg);
        init_identity(iso, &cfg, label);
        out.push(Party {
            fp: identity_field(iso, &cfg, label, "identity_fp"),
            kem: identity_field(iso, &cfg, label, "identity_kem_pk"),
            sig: identity_field(iso, &cfg, label, "identity_sig_pk"),
            cfg,
        });
    }
    let mut it = out.into_iter();
    (it.next().unwrap(), it.next().unwrap(), it.next().unwrap())
}

#[test]
fn na0634_positive_full_identity_roundtrip() {
    let iso = common::TestIsolation::new("na0634_positive");
    let (alice, bob, _mallory) = init_parties(&iso, "positive");
    // Alice pins bob's REAL full identity (KEM + signing key vs the single combined code); bob pins alice.
    contacts_add(&iso, &alice.cfg, "bob", &bob.fp, &bob.kem, &bob.sig, ROUTE_TOKEN_BOB);
    contacts_add(&iso, &bob.cfg, "alice", &alice.fp, &alice.kem, &alice.sig, ROUTE_TOKEN_ALICE);
    relay_inbox_set(&iso, &alice.cfg, ROUTE_TOKEN_ALICE);
    relay_inbox_set(&iso, &bob.cfg, ROUTE_TOKEN_BOB);
    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();

    let out = drive(&iso, &alice.cfg, &bob.cfg, "bob", &server, &relay);
    let text = output_text(&out);

    assert!(
        !text.contains("handshake_reject"),
        "genuine full-identity responder was wrongly rejected:\n{text}"
    );
    assert!(
        session_path(&alice.cfg, "bob").exists(),
        "full-identity handshake did not establish a session:\n{text}"
    );
    eprintln!("NA0634_FULL_IDENTITY_POSITIVE_OK");
}

#[test]
fn na0634_wrong_signing_key_rejected() {
    let iso = common::TestIsolation::new("na0634_wrong_sig");
    let (alice, bob, mallory) = init_parties(&iso, "wrong_sig");
    // Alice pins bob's REAL KEM key (so C1 PASSES) but MALLORY's signing key — a full-identity pin whose
    // signing half is not bob's. The combined code is self-consistent for {bob.kem, mallory.sig}.
    let wrong_fp = combined_fp(&bob.kem, &mallory.sig);
    contacts_add(&iso, &alice.cfg, "bob", &wrong_fp, &bob.kem, &mallory.sig, ROUTE_TOKEN_BOB);
    // Bob pins alice correctly so the genuine bob answers on bob's channel.
    contacts_add(&iso, &bob.cfg, "alice", &alice.fp, &alice.kem, &alice.sig, ROUTE_TOKEN_ALICE);
    relay_inbox_set(&iso, &alice.cfg, ROUTE_TOKEN_ALICE);
    relay_inbox_set(&iso, &bob.cfg, ROUTE_TOKEN_BOB); // the genuine bob answers
    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();

    let out = drive(&iso, &alice.cfg, &bob.cfg, "bob", &server, &relay);
    let text = output_text(&out);

    // Bob passes C1 (holds bob.kem_sk) and its B1 signature is valid — but its signing key does not match
    // the pinned sig_fp, so the REQUIRED sig-pin rejects at B1. THE SIGNING-HALF FIX (ENG-0038).
    assert!(
        text.contains("handshake_reject"),
        "expected alice to reject the wrong-signing-key responder; got:\n{text}"
    );
    assert!(
        text.contains("responder_sig_mismatch"),
        "expected the reject reason to be the required sig-pin mismatch; got:\n{text}"
    );
    assert!(
        !session_path(&alice.cfg, "bob").exists(),
        "the signing-key asymmetry is NOT closed: alice committed a session to a wrong signing key:\n{text}"
    );
    eprintln!("NA0634_WRONG_SIGNING_KEY_REJECTED_OK");
}
