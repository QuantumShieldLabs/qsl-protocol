//! NA-0681 (D616 §5.6) — TWO PARTIES, ONE INVITE, THE REAL SLICE-1 RELAY, NO MOCKS.
//!
//! This is the acceptance item the whole slice exists to satisfy: two strangers with
//! separate vaults and separate identities turn one invite code into a session. The relay
//! is qsl-server's actual router at the pinned Slice-1 commit, in process, and both parties
//! are the real `qsc` binary driven over its CLI.
//!
//! ⚠ THE TESTED TOPOLOGY IS TWO VAULTS ON ONE HOST. That is the ruled acceptance topology
//! (epic §4 Q2) and it is NOT two machines. A green here says nothing about NAT, about real
//! network partitions, or about two physically separate devices.
//!
//! ⚠ WHAT ELSE A GREEN DOES NOT ASSERT: nothing about messaging or delivery states
//! (Slice 3), nothing about the GUI (Slice 4), and no timing claim of any kind.

mod common;

use argon2::{Algorithm, Argon2, Params, Version};
use chacha20poly1305::aead::{Aead, KeyInit, Payload};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Mutex, MutexGuard, OnceLock};

fn guard() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

fn output_text(out: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    )
}

fn ensure_dir_700(path: &Path) {
    let _ = fs::remove_dir_all(path);
    fs::create_dir_all(path).expect("create dir");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).expect("chmod 700");
    }
}

fn test_root(tag: &str) -> PathBuf {
    let root = if let Ok(v) = std::env::var("CARGO_TARGET_DIR") {
        PathBuf::from(v)
    } else {
        PathBuf::from("target")
    };
    let root = root.join("qsc-test-tmp").join(tag);
    ensure_dir_700(&root);
    root
}

fn qsc(cfg: &Path) -> Command {
    let mut cmd = common::qsc_std_command();
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain");
    cmd
}

fn run_ok(cfg: &Path, args: &[&str]) -> String {
    let out = qsc(cfg).args(args).output().expect("run qsc");
    let text = output_text(&out);
    assert!(out.status.success(), "command failed {args:?}\n{text}");
    text
}

fn run_expect_fail(cfg: &Path, args: &[&str]) -> String {
    let out = qsc(cfg).args(args).output().expect("run qsc");
    let text = output_text(&out);
    assert!(
        !out.status.success(),
        "command unexpectedly SUCCEEDED {args:?}\n{text}"
    );
    text
}

/// A party: its own config dir, its own vault, its own identity, its own inbox token.
fn party(root: &Path, name: &str, inbox: &str) -> PathBuf {
    let cfg = root.join(name);
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    run_ok(&cfg, &["identity", "rotate", "--confirm"]);
    run_ok(&cfg, &["relay", "inbox-set", "--token", inbox]);
    cfg
}

const ALICE_INBOX: &str = "na0681_alice_inbox_token_abcdefgh";
const BOB_INBOX: &str = "na0681_bob_inbox_token_ijklmnopq";

/// create → redeem → verify → handshake → both sides hold a PENDING contact carrying the
/// other's identity and route token.
#[test]
fn two_strangers_become_a_session_through_one_invite() {
    let _g = guard();
    let relay = common::start_qsl_server(2 * 1024 * 1024, 512, None);
    let base = relay.base_url().to_string();
    let root = test_root("na0681_two_party");

    let alice = party(&root, "alice", ALICE_INBOX);
    let bob = party(&root, "bob", BOB_INBOX);

    // --- Alice mints an invite -------------------------------------------------
    let code = run_ok(
        &alice,
        &["invite", "create", "--relay", &base, "--ttl-secs", "3600"],
    );
    let code = code
        .lines()
        .find(|l| l.starts_with("QSLI-1-"))
        .expect("invite code on stdout")
        .trim()
        .to_string();
    assert!(code.len() <= 250, "code must stay side-channel sized");

    let invite_id = run_ok(&alice, &["invite", "list"]);
    let invite_id = invite_id
        .lines()
        .find_map(|l| l.strip_prefix("invite="))
        .and_then(|l| l.split_whitespace().next())
        .expect("invite id")
        .to_string();

    // --- Bob redeems, verifies, and hand shakes into the slot -------------------
    let redeem = run_ok(
        &bob,
        &["invite", "redeem", "--code", &code, "--alias", "alice"],
    );
    assert!(
        redeem.contains("status=pinned"),
        "redemption must yield a PENDING contact, never a trusted one (I5): {redeem}"
    );

    // --- Alice collects the handshake and answers it ---------------------------
    let accept = run_ok(
        &alice,
        &[
            "invite",
            "accept",
            "--invite-id",
            &invite_id,
            "--alias",
            "bob",
        ],
    );
    assert!(
        accept.contains("status=pinned"),
        "Alice must end with a PENDING contact for Bob: {accept}"
    );

    // --- Bob finishes: learns Alice's real route token, completes the handshake --
    let finish = run_ok(
        &bob,
        &["invite", "finish", "--alias", "alice", "--relay", &base],
    );
    assert!(finish.contains("invite_finish=ok"), "{finish}");

    // --- Both sides hold the other, PENDING, with a usable route token ---------
    let alice_view = run_ok(&alice, &["contacts", "show", "--label", "bob"]);
    let bob_view = run_ok(&bob, &["contacts", "show", "--label", "alice"]);
    for (who, view) in [
        ("alice's view of bob", &alice_view),
        ("bob's view of alice", &bob_view),
    ] {
        // I5, asserted on the CONTACT-level state, which is what the verification ceremony
        // moves. ⚠ Do NOT read the device-level state here: `contacts show` also prints
        // `device=… state=TRUSTED`, because the shipped mapping sends "PINNED" -> "TRUSTED"
        // for every non-verified contact and that flag means "usable for routing", not
        // "the human checked the code". Reading it as trust would invert I5.
        assert!(
            view.contains("state=PINNED"),
            "{who} must be PENDING-VERIFICATION: {view}"
        );
        assert!(
            !view.contains("state=VERIFIED"),
            "{who} must NOT be verified — redemption never confers trust (I5): {view}"
        );
    }

    // The route tokens are read from the VAULT, not from CLI output: no command prints
    // them, deliberately -- they are routing secrets and the CLI redacts to `hash8`.
    //
    // This is the assertion the whole envelope design exists for. Bob reached Alice through
    // a one-shot invite slot whose ticket was burned by his own A1 push; unless the RESPONSE
    // envelope handed him her real inbox, he has no address for her and the session
    // dead-ends after B1.
    let bob_contacts = read_mock_vault_secret(&bob, "contacts.json").expect("bob contacts");
    let alice_contacts = read_mock_vault_secret(&alice, "contacts.json").expect("alice contacts");

    assert!(
        bob_contacts.contains(ALICE_INBOX),
        "Bob must have learned Alice's REAL route token from the response envelope: {bob_contacts}"
    );
    // Precisely: the ROUTE TOKEN must no longer be the invite slot. The `invite_id` itself
    // is expected to appear in the record -- §2f stores it deliberately, as provenance --
    // so a bare "does the id appear anywhere" check would assert the opposite of the
    // intent and fail on a correct implementation.
    assert!(
        !bob_contacts.contains(&format!("\"route_token\":\"{invite_id}\"")),
        "Bob must NOT still be ADDRESSING the burned invite slot: {bob_contacts}"
    );
    assert!(
        bob_contacts.contains(&format!("\"invite_id\":\"{invite_id}\"")),
        "the originating invite must be recorded as provenance: {bob_contacts}"
    );
    assert!(
        alice_contacts.contains(BOB_INBOX),
        "Alice must have learned Bob's route token from the request envelope: {alice_contacts}"
    );

    // P3, laid from contact #1: the plural endpoint list is populated and the pinning hook
    // exists and is DORMANT.
    assert!(
        bob_contacts.contains("relay_endpoints"),
        "the plural endpoint list must be laid from contact #1: {bob_contacts}"
    );

    // ⚠⚠ NA-0711 (D647 A4 Δ40): THE STEP THIS TEST WAS MISSING, AND ITS ABSENCE IS WHY A
    // BROKEN INVITE PATH SURVIVED TO A LIVE RELAY.
    //
    // Everything above asserts CONTACT state. Neither party was ever asked whether it holds a
    // SESSION, and the accepter's own `handshake poll` -- the step that ingests A2 and the step
    // that failed on the real rig for three walks across two client revs -- was never run here at
    // all. A green on contact state says two strangers exchanged identities; it does not say they
    // can talk.
    let poll = run_ok(
        &alice,
        &["handshake", "poll", "--peer", "bob", "--relay", &base],
    );
    assert!(
        poll.contains("handshake_complete") && poll.contains("role=responder"),
        "the accepter must COMPLETE the handshake, not merely hold a contact:\n{poll}"
    );
    let alice_status = run_ok(&alice, &["handshake", "status", "--peer", "bob"]);
    assert!(
        !alice_status.contains("status=no_session"),
        "the accepter must hold a session after its own poll:\n{alice_status}"
    );
    let bob_status = run_ok(&bob, &["handshake", "status", "--peer", "alice"]);
    assert!(
        !bob_status.contains("status=no_session"),
        "the redeemer must hold a session:\n{bob_status}"
    );
}

/// §5.3 arm (b): CLIENT-SIDE single use. The relay is not consulted — this is the arm that
/// survives a hostile relay, so it must hold even when the relay would happily serve the
/// invite again.
#[test]
fn a_second_redemption_is_refused_by_the_client_itself() {
    let _g = guard();
    let relay = common::start_qsl_server(2 * 1024 * 1024, 512, None);
    let base = relay.base_url().to_string();
    let root = test_root("na0681_client_single_use");

    let alice = party(&root, "alice", ALICE_INBOX);
    let bob = party(&root, "bob", BOB_INBOX);

    let code = run_ok(&alice, &["invite", "create", "--relay", &base])
        .lines()
        .find(|l| l.starts_with("QSLI-1-"))
        .expect("code")
        .trim()
        .to_string();

    run_ok(
        &bob,
        &["invite", "redeem", "--code", &code, "--alias", "alice"],
    );

    // The SAME client, the same code, a second time.
    let again = run_expect_fail(
        &bob,
        &["invite", "redeem", "--code", &code, "--alias", "alice2"],
    );
    assert!(
        again.contains("invite_already_redeemed"),
        "the client's own record must refuse this, without asking the relay: {again}"
    );
}

/// §5.4: an expired invite dies BEFORE any network attempt.
///
/// Asserted by pointing the invite at a port with NO LISTENER. If the client touched the
/// network first we would get a transport error; getting the local expiry code instead is
/// what proves the ordering.
///
/// ⚠ The control that makes this meaningful is the second half: with a FUTURE expiry and
/// the same dead port, the same command must fail differently — a transport error. Without
/// that, this test would pass against a client that simply failed at everything.
#[test]
fn an_expired_invite_dies_before_any_network_attempt() {
    let _g = guard();
    let root = test_root("na0681_expired_prenetwork");
    let bob = party(&root, "bob", BOB_INBOX);

    // Nothing is listening here. Any network attempt fails visibly.
    let dead = "http://127.0.0.1:9";

    let expired = make_code(dead, 1_000_000);
    let out = run_expect_fail(
        &bob,
        &["invite", "redeem", "--code", &expired, "--alias", "x"],
    );
    assert!(
        out.contains("invite_expired"),
        "an expired invite must die on the LOCAL clock, pre-network: {out}"
    );

    // THE CONTROL: same dead port, expiry in the future -> a DIFFERENT failure.
    let live = make_code(dead, 4_000_000_000);
    let out2 = run_expect_fail(&bob, &["invite", "redeem", "--code", &live, "--alias", "x"]);
    assert!(
        !out2.contains("invite_expired"),
        "with a valid expiry the failure must come from the network, not the clock: {out2}"
    );
}

/// Build a syntactically valid invite code with a chosen expiry. The signature is not valid
/// — it never gets that far, which is exactly the property under test.
fn make_code(relay_ep: &str, expiry: u64) -> String {
    use qsc::invite::*;
    let p = InvitePayload {
        ver: INVITE_VER,
        typ: INVITE_TYPE_CONTACT,
        invite_id: [0x11; ID_LEN],
        expiry,
        relay_ep: relay_ep.to_string(),
        cap: [0x22; ID_LEN],
        commit: [0x33; COMMIT_LEN],
    };
    encode_invite_code(&p).expect("encode")
}

fn derive_mock_vault_key(bytes: &[u8]) -> ([u8; 32], usize, usize) {
    assert!(bytes.len() > 25, "vault envelope too short");
    assert_eq!(&bytes[0..6], b"QSCV02");
    assert_eq!(bytes[6], 1, "expected passphrase vault");
    let salt_len = bytes[7] as usize;
    let nonce_len = bytes[8] as usize;
    assert_eq!(salt_len, 16);
    assert_eq!(nonce_len, 12);
    let kdf_m_kib = u32::from_le_bytes([bytes[9], bytes[10], bytes[11], bytes[12]]);
    let kdf_t = u32::from_le_bytes([bytes[13], bytes[14], bytes[15], bytes[16]]);
    let kdf_p = u32::from_le_bytes([bytes[17], bytes[18], bytes[19], bytes[20]]);
    let salt = &bytes[25..25 + salt_len];
    let params = Params::new(kdf_m_kib, kdf_t, kdf_p, Some(32)).expect("argon2 params");
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let mut key = [0u8; 32];
    argon2
        .hash_password_into(
            common::TEST_MOCK_VAULT_PASSPHRASE.as_bytes(),
            salt,
            &mut key,
        )
        .expect("vault key");
    (key, salt_len, nonce_len)
}
fn read_mock_vault_json(cfg: &Path) -> Value {
    let bytes = fs::read(cfg.join("vault.qsv")).expect("vault read");
    let (key, salt_len, nonce_len) = derive_mock_vault_key(&bytes);
    let ct_len = u32::from_le_bytes([bytes[21], bytes[22], bytes[23], bytes[24]]) as usize;
    let mut off = 25 + salt_len;
    let nonce = &bytes[off..off + nonce_len];
    off += nonce_len;
    let ciphertext = &bytes[off..off + ct_len];
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&key));
    let plaintext = cipher
        .decrypt(
            Nonce::from_slice(nonce),
            Payload {
                msg: ciphertext,
                // NA-0694 (D628 §2e F3): the product now binds the 53-byte header as
                // AEAD AAD; the header prefix of the file is that AAD verbatim.
                aad: &bytes[..53],
            },
        )
        .expect("vault decrypt");
    serde_json::from_slice(&plaintext).expect("vault json")
}
fn read_mock_vault_secret(cfg: &Path, name: &str) -> Option<String> {
    read_mock_vault_json(cfg)
        .get("secrets")
        .and_then(|v| v.get(name))
        .and_then(|v| v.as_str())
        .map(ToOwned::to_owned)
}
