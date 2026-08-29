//! NA-0690 (D624 / D-1329, ENG-0104) — **THE ACK IS GATED ON DURABLE CAPTURE.**
//!
//! ⚠ **WHAT THIS FILE PINS.** When the timeline row fails to store in
//! `transport::receive_pull_and_write`, the pulled item's id must **NOT** enter the ack set — so
//! the relay is never told to consume it, the lease expires, and the item **comes back**. Before
//! D-1329 the ack fired unconditionally: the relay dropped the item and the sender may already
//! have been told *delivered*. The comment at the site and the `not_stored_so_not_acked` marker
//! literal both already claimed this behaviour and were **false**; this lane moved the CODE to
//! them (D-1328 Ruling 4 forbade editing either).
//!
//! ⚠ **WHAT IT DOES NOT PIN, STATED FIRST SO NOBODY READS MORE INTO A GREEN THAN IS THERE.**
//! `commit_unpack_state` consumes the ratchet key ABOVE the store, so a store failure loses the
//! **plaintext** whether the failure is transient or permanent, and the redelivery can only be a
//! replay-reject. **REDELIVERY DOES NOT RECOVER THE MESSAGE, and nothing here asserts that it
//! does.** What the gate buys is narrower and is the whole point: the **ack — the sender's only
//! evidence of delivery — is WITHHELD**, so no false DELIVERED; the loss is **loud and witnessed**
//! rather than silent; what survives downstream is the **opaque envelope**, quarantined by NA-0689
//! with a witness, **not** the message; and the loop is **bounded to one cycle**.
//!
//! ⚠ **A ZERO ALONE WOULD BE THE VACUOUS HALF** — NA-0689's doctrine, adopted rather than
//! re-derived. *"The item was not acked"* is exactly what a run that never reached the site also
//! reports. So the positive and the negative are read by **the SAME INSTRUMENT in the same setup**,
//! a redelivery probe:
//!
//! - **acked**   ⇒ the relay consumed it ⇒ a later pull returns **NOTHING**;
//! - **unacked** ⇒ the lease expires     ⇒ a later pull **RETURNS IT AGAIN**.
//!
//! A broken instrument cannot green both arms, because they need it to answer *differently* about
//! the same relay, the same session and the same pair of identities.
//!
//! ⚠ **HOW THE STORE IS MADE TO FAIL, AND WHY IT IS THIS AND NOT A TEST SEAM.**
//! `timeline_store_load` maps an unparseable stored value to `timeline_tampered`, so poisoning the
//! recipient's timeline vault secret makes `timeline_append_entry` fail **deterministically at
//! exactly the target site** — an otherwise-good, fully decrypted message whose store fails, which
//! is ENG-0104's scenario. It needs **no production change** and asserts **in the default gate
//! build**. The tree's only fault seam, `qsc_rng_failure_test_seam`, compiles solely under a
//! non-default `--cfg` (D-0883: *"normal builds … must not read the seam selector"*), so a test
//! resting on it would assert **nothing** in the standard suite.
//!
//! ⚠ **THE POISON CANNOT DISPLACE, AND THAT WAS MEASURED, NOT ASSUMED.** The whole receive path
//! reads the timeline exactly **once**, at the target site: `receive_execute` does not touch it at
//! all, and the confirm arms that *do* read it are reachable only by control payloads, which
//! `qsc send --file` never produces.
//!
//! ⚠ **LEASE MODE ONLY, BY THE NATURE OF THE DEFECT.** `record_seen_and_queue_ack` is a **no-op**
//! when the seen store is absent, which is legacy mode — the property is *unobservable* there, not
//! merely untested. Lease is the default since NA-0688 C4; it is passed **explicitly** so this arm
//! cannot be silently retired by a future default flip.

mod common;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::thread;
use std::time::Duration;

/// A 1-second server-side pull lease so an unacked item becomes visible again quickly — the values
/// NA-0644 and NA-0689 use to prove lease redelivery against the real relay.
// ⚠⚠ NA-0770 (D-1411) WIDENED THESE 1s/2500ms -> 8s/20000ms. THE RECORDED REASON IS MARGIN
// AGAINST CONTENTION, and it is stated so a later reader does not "tidy" them back.
//
// Before this lane, arms that wanted a NEGATIVE result reached for delete-on-pull, which is
// instantaneous and needs no waiting. With the mode retired, every such arm must instead wait out a
// lease — so the suite's dependence on these two numbers went UP at the moment the mode went away.
// They are now load-bearing in shards that run twelve-wide on six cores, where a 2500ms wait
// against a 1s lease left almost no margin: an overrun does not merely slow the arm, it reports a
// perfectly intact message as lost. The pair is kept in step across the files that only ever WAIT
// OUT a lease: `NA_0644`, `na0689_capture`, `na0689_p3_a2`, `na0690`, `na0708`, `na0741`, and
// `na0742` under its own names — if you change one of THOSE, change all of them.
//
// ⚠⚠ `na0688_c4_collateral_arms` IS DELIBERATELY EXCLUDED AND CARRIES 45s/60000ms. It is the only
// file that does work INSIDE the lease window (its in-lease probe runs two full CLI invocations,
// each paying an Argon2id vault unlock, before the lease may expire) rather than merely waiting a
// lease out. Those are DIFFERENT REQUIREMENTS, and CI proved it: at 8s that file's self-asserting
// precondition measured the probe at 8.915s on a 2-core runner and REFUSED (PR #1802,
// `qsc-shard-10`). ⚠ DO NOT "HARMONISE" na0688 BACK TO THESE VALUES — it will start refusing
// again, correctly. A local 6-core run cannot reproduce the overrun.
const TEST_PULL_LEASE_SECS: usize = 8;
const LEASE_EXPIRY_WAIT: Duration = Duration::from_millis(20_000);

const ALICE_INBOX: &str = "na0690_alice_inbox_token_abcdefgh";
const BOB_INBOX: &str = "na0690_bob_inbox_token_ijklmnopq";

/// The vault secret the timeline store lives under. ⚠ The constant is `pub(crate)` in `qsc::store`,
/// so an integration test cannot import it and must repeat the literal — **a silent-vacuity
/// hazard**: if the name ever changed, this would poison an unrelated secret, the store would
/// succeed, and the file would quietly stop testing anything. It does not, because the negative arm
/// **asserts the store actually failed** before it asserts anything about the ack. A poison that
/// misses turns this file RED, never green.
const TIMELINE_SECRET_NAME: &str = "timeline.json";

/// Not parseable as a `TimelineStore`, and deliberately not parseable as JSON either.
const POISON: &str = "na0690 not a timeline store";

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
    fs::create_dir_all(path).expect("create dir");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut p = fs::metadata(path).expect("meta").permissions();
        p.set_mode(0o700);
        fs::set_permissions(path, p).expect("chmod");
    }
}

fn test_root(tag: &str) -> PathBuf {
    let root = common::unique_test_root(tag);
    ensure_dir_700(&root);
    root
}

fn qsc(cfg: &Path) -> Command {
    // ⚠ `qsc_std_command()` ALREADY applies the mock-vault unlock args; re-adding them makes clap
    // reject `--unlock-passphrase-env` as repeated and fails setup before any measurement runs.
    let mut c = common::qsc_std_command();
    c.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain");
    c
}

fn run_ok(cfg: &Path, args: &[&str]) -> String {
    let out = qsc(cfg).args(args).output().expect("run qsc");
    let text = output_text(&out);
    assert!(out.status.success(), "expected success: {args:?}\n{text}");
    text
}

/// A party: its own config dir, vault, identity and inbox token. Adopted from
/// `na0689_capture_boundary_arms.rs` rather than re-derived beside it.
fn party(root: &Path, name: &str, inbox: &str) -> PathBuf {
    let cfg = root.join(name);
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    run_ok(&cfg, &["identity", "rotate", "--confirm"]);
    run_ok(&cfg, &["relay", "inbox-set", "--token", inbox]);
    cfg
}

fn fingerprint(cfg: &Path) -> String {
    run_ok(cfg, &["identity", "show"])
        .lines()
        .find_map(|l| l.strip_prefix("identity_fp="))
        .expect("identity_fp")
        .trim()
        .to_string()
}

/// ⚠ Adding the contact is NOT enough to send: its device must also be TRUSTED. The NA-0644
/// `setup_pair` sequence, adopted wholesale — bespoke setup here would measure my own scaffolding.
fn add_contact(cfg: &Path, label: &str, fp: &str, route_token: &str) {
    run_ok(
        cfg,
        &[
            "contacts",
            "add",
            "--label",
            label,
            "--fp",
            fp,
            "--route-token",
            route_token,
        ],
    );
    let list = run_ok(cfg, &["contacts", "device", "list", "--label", label]);
    let device = list
        .lines()
        .find_map(|line| {
            line.split_whitespace()
                .find_map(|tok| tok.strip_prefix("device="))
        })
        .unwrap_or_else(|| panic!("missing device output: {list}"));
    run_ok(
        cfg,
        &[
            "contacts",
            "device",
            "trust",
            "--label",
            label,
            "--device",
            device,
            "--confirm",
        ],
    );
}

fn setup(root: &Path) -> (PathBuf, PathBuf) {
    let alice = party(root, "alice", ALICE_INBOX);
    let bob = party(root, "bob", BOB_INBOX);
    let alice_fp = fingerprint(&alice);
    let bob_fp = fingerprint(&bob);
    add_contact(&alice, "bob", &bob_fp, BOB_INBOX);
    add_contact(&bob, "bob", &alice_fp, ALICE_INBOX);
    (alice, bob)
}

fn send_message(alice: &Path, relay: &str, base: &Path, name: &str, bytes: &[u8]) {
    let msg = base.join(name);
    fs::write(&msg, bytes).expect("write msg");
    let text = run_ok(
        alice,
        &[
            "send",
            "--transport",
            "relay",
            "--relay",
            relay,
            "--to",
            "bob",
            "--file",
            msg.to_str().expect("msg path"),
        ],
    );
    assert!(
        text.contains("QSC_DELIVERY state=accepted_by_relay"),
        "{text}"
    );
}

/// ⚠ **LEASE IS PASSED EXPLICITLY.** It is the default since NA-0688 C4, but the defect is
/// unobservable in legacy mode (the ack accumulator is never populated there), so this arm states
/// the mode it needs instead of inheriting it.
fn receive_args<'a>(relay: &'a str, out: &'a str) -> Vec<&'a str> {
    vec![
        "receive",
        "--transport",
        "relay",
        "--relay",
        relay,
        "--mailbox",
        BOB_INBOX,
        "--from",
        "bob",
        "--max",
        "8",
        "--out",
        out,
    ]
}

fn recv_file_count(out: &Path) -> usize {
    fs::read_dir(out)
        .expect("read out dir")
        .filter_map(|e| e.ok())
        .filter(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            name.starts_with("recv_") && name.ends_with(".bin")
        })
        .count()
}

/// Make the recipient's timeline store unreadable, IN PROCESS, through the shipped library.
///
/// ⚠ The vault is a FILE, so a secret written here is what the `qsc receive` SUBPROCESS reads —
/// which is how an in-process poison gates an out-of-process receive.
fn poison_timeline(cfg: &Path) {
    std::env::set_var("QSC_CONFIG_DIR", cfg);
    std::env::set_var("QSC_DISABLE_KEYCHAIN", "1");
    qsc::vault::unlock_with_passphrase(common::TEST_MOCK_VAULT_PASSPHRASE).expect("unlock vault");
    qsc::vault::secret_set(TIMELINE_SECRET_NAME, POISON).expect("poison the timeline store");
}

// ---------------------------------------------------------------------------
// THE PAIR. One test, one relay, one session, one pair of identities — so the ONLY thing that
// differs between the arms is whether the timeline row stored. Two separate tests would let a
// setup difference masquerade as the result.
// ---------------------------------------------------------------------------
#[test]
fn a_stored_message_is_acked_and_an_unstored_one_comes_back() {
    let _g = guard();
    let relay =
        common::start_qsl_server_with_store(2 * 1024 * 1024, 512, None, TEST_PULL_LEASE_SECS);
    let base = relay.base_url().to_string();
    let root = test_root("na0690_ack_gated_on_store");
    let (alice, bob) = setup(&root);

    // ---- POSITIVE CONTROL: a healthy message stores, IS acked, and does NOT come back. ----
    //
    // ⚠ This half is what makes the negative half mean anything. It proves the redelivery probe can
    // observe an ack at all — that a silent "nothing came back" is a real consumption, and not a
    // relay that was never going to redeliver, a lease that never expired, or a mailbox nobody
    // pulled.
    let out1 = root.join("out1");
    ensure_dir_700(&out1);
    send_message(
        &alice,
        &base,
        &root,
        "healthy.txt",
        b"na0690 a message that stores",
    );
    let healthy = run_ok(&bob, &receive_args(&base, out1.to_str().expect("out1")));
    assert_eq!(
        recv_file_count(&out1),
        1,
        "the healthy message must have been received: {healthy}"
    );

    thread::sleep(LEASE_EXPIRY_WAIT);
    let out1b = root.join("out1b");
    ensure_dir_700(&out1b);
    let after_healthy = run_ok(&bob, &receive_args(&base, out1b.to_str().expect("out1b")));
    assert_eq!(
        recv_file_count(&out1b),
        0,
        "AN ACKED ITEM MUST NOT COME BACK. If it does, the relay never consumed it, and the \
         negative arm below would report 'not acked' for every possible tree.\n\
         receive:\n{after_healthy}"
    );

    // ---- NEGATIVE: the store fails, so the item is NOT acked and the relay redelivers it. ----
    poison_timeline(&bob);

    let out2 = root.join("out2");
    ensure_dir_700(&out2);
    send_message(
        &alice,
        &base,
        &root,
        "doomed.txt",
        b"na0690 a message that cannot store",
    );
    let doomed = run_ok(&bob, &receive_args(&base, out2.to_str().expect("out2")));

    // ⚠ ASSERT THE STORE ACTUALLY FAILED **BEFORE** ASSERTING ANYTHING ABOUT THE ACK. A poison that
    // silently missed would leave a stored, acked message and a green "nothing came back" — the
    // vacuous pass this ordering exists to refuse.
    assert!(
        doomed.contains("op=timeline_receive_ingest"),
        "the timeline store must have FAILED, or this arm measures nothing: {doomed}"
    );

    // ⚠ The marker the fix makes HONEST. Its literal `not_stored_so_not_acked` was already in the
    // tree and was FALSE at the moment it was emitted — the very next statement acked the item
    // anyway. D-1328 Ruling 4 forbade editing it; this lane moved the code to it instead.
    assert!(
        doomed.contains("not_stored_so_not_acked"),
        "the receipt-suppressed marker must fire on the store failure: {doomed}"
    );

    thread::sleep(LEASE_EXPIRY_WAIT);
    let out3 = root.join("out3");
    ensure_dir_700(&out3);
    let redelivered = run_ok(&bob, &receive_args(&base, out3.to_str().expect("out3")));

    // ⚠ THE PROPERTY. The item was never acked, so the lease expired and the relay handed it back.
    //
    // ⚠ IT RETURNS AS A REPLAY REJECT, NOT AS A CLEAN SECOND DELIVERY, AND THAT BOUNDS WHAT THIS
    // FIX CLAIMS. `commit_unpack_state` consumes the ratchet key above the store, so a store
    // failure loses the plaintext either way and the redelivered envelope can never be decrypted
    // again. **Redelivery does not recover the message and this test does not assert that it
    // does.** It asserts the item was NOT CONSUMED BY THE RELAY — which is what makes the sender's
    // "delivered" evidence honest, and what turns a silent loss into a witnessed one.
    assert!(
        redelivered.contains("event=ack_replay_unrecoverable"),
        "AN UNSTORED ITEM MUST COME BACK. This is the whole property: a failed durable capture must \
         not let the relay consume the message.\nredelivery:\n{redelivered}"
    );
}
