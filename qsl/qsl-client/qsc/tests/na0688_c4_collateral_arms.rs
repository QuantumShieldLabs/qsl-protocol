//! NA-0688 C4 (D622) — THE COLLATERAL ARMS: what a flag-less pull destroys, and what lease saves.
//!
//! ⚠ WHAT THIS FILE MEASURES. Three production commands reach `transport::relay_inbox_pull`, which
//! carried a hardcoded `AckMode::Legacy` until C4 (SITE 2 of 2). Under legacy the relay DELETES ON
//! PULL, so any item a command collected but did not process was destroyed — silently, at exit 0.
//! Under lease the relay holds each item until it is acked after a durable persist, and none of
//! these three commands acks, so a collaterally-pulled item is REDELIVERED instead of lost.
//!
//! The ratified caller set (D622 R1), measured from the call graph rather than from any directive:
//!   1. `invite accept`  — `invite_accept_at`, pulls the invite's OWN mailbox, `--max 1`
//!   2. `invite finish`  — `invite_finish`, ⚠ pulls the user's ORDINARY inbox, `--max 1`
//!   3. `handshake poll` — `--max 4`, iterates every pulled item and skips what it cannot parse
//!
//! ⚠ `invite redeem` IS NOT IN THIS FILE, and its absence is a measurement, not an oversight: it
//! reaches the relay only via `POST /v1/invite/redeem` and never pulls an inbox at all. C4's own
//! census wrongly listed it, having identified call sites by line number instead of bracketing them
//! to their enclosing functions.
//!
//! ⚠ EVERY ARM RUNS UNDER LEGACY FIRST, AND THAT IS THE WHOLE METHOD. An arm run only under lease
//! could pass vacuously: "the peer's message survived" proves nothing unless the same topology is
//! shown to DESTROY it under legacy. A negative result is only evidence if the instrument could
//! have returned positive. The legacy control is selected through `qsc config set ack-mode legacy`
//! — these three commands take no `--ack-mode` flag, so the per-install preference is the only way
//! to aim them, which makes the config key part of the instrument rather than a convenience.
//!
//! ⚠ THE RELAY MUST BE THE REAL ONE. The test-local mock in `common` parses only `max=` and always
//! pops on pull, so it cannot express lease semantics and would make every arm here vacuous.

mod common;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::thread;
use std::time::Duration;

/// A 1-second server-side pull lease, so an unacked item becomes visible again quickly.
/// Same values NA-0644 uses to prove lease redelivery.
const TEST_PULL_LEASE_SECS: usize = 1;
const LEASE_EXPIRY_WAIT: Duration = Duration::from_millis(2500);

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
    // ⚠ `qsc_std_command()` ALREADY applies the mock-vault unlock args. Adding them again makes
    // clap reject `--unlock-passphrase-env` as repeated, which fails setup before any measurement
    // runs — so do not "helpfully" re-add them here.
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

fn run_any(cfg: &Path, args: &[&str]) -> (bool, String) {
    let out = qsc(cfg).args(args).output().expect("run qsc");
    (out.status.success(), output_text(&out))
}

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

/// ⚠ Adding the contact is NOT enough to send: its device must also be TRUSTED. This is the
/// NA-0644 `setup_pair` sequence adopted wholesale rather than re-derived — the arms are a
/// measurement of ack-mode behaviour, and any bespoke setup here is a way to measure my own
/// scaffolding by mistake.
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
            "contacts", "device", "trust", "--label", label, "--device", device, "--confirm",
        ],
    );
}

/// Put ONE ordinary peer message into `peer_inbox`, from a real send over the real relay.
fn plant_ordinary_message(sender_cfg: &Path, relay: &str, base: &Path, body: &[u8]) {
    let f = base.join(format!("planted_{}.txt", body.len()));
    fs::write(&f, body).expect("write planted body");
    run_ok(
        sender_cfg,
        &[
            "send",
            "--transport",
            "relay",
            "--relay",
            relay,
            "--to",
            "peer",
            "--file",
            f.to_str().expect("path"),
        ],
    );
}

/// Can the victim still collect the planted message afterwards?
///
/// ⚠ Uses `--ack-mode legacy` DELIBERATELY so the probe itself is delete-on-pull and cannot leave a
/// leased copy behind that a later probe would see. The probe must not perturb what it measures.
///
/// ⚠⚠ CALLERS MUST WAIT PAST THE LEASE BEFORE PROBING A LEASE ARM, AND THE REASON IS THE WHOLE
/// POINT OF THE MECHANISM. Under lease a pulled-but-unacked item is held INVISIBLE until its lease
/// expires — it is not gone, it is reserved. A probe run immediately after the command under test
/// therefore reports "not recoverable" for an item that is perfectly intact, which measures
/// IMMEDIATE VISIBILITY when the claim under test is SURVIVAL. Those differ by exactly one lease
/// interval. Waiting past expiry also makes the arm prove something strictly stronger than
/// survival: REDELIVERY, which is the property that actually makes the collateral recoverable.
fn planted_message_still_recoverable(victim_cfg: &Path, relay: &str, inbox: &str, out: &Path) -> bool {
    let (_ok, text) = run_any(
        victim_cfg,
        &[
            "receive",
            "--transport",
            "relay",
            "--relay",
            relay,
            "--mailbox",
            inbox,
            "--from",
            "peer",
            "--max",
            "8",
            "--out",
            out.to_str().expect("out"),
            "--ack-mode",
            "legacy",
        ],
    );
    // A recovered message writes a recv file; an empty mailbox reports recv_none.
    let recovered = fs::read_dir(out)
        .map(|d| {
            d.filter_map(|e| e.ok())
                .any(|e| e.file_name().to_string_lossy().starts_with("recv_"))
        })
        .unwrap_or(false);
    recovered && !text.contains("event=recv_none")
}

fn set_ack_mode(cfg: &Path, mode: &str) {
    run_ok(cfg, &["config", "set", "ack-mode", mode]);
}

const VICTIM_INBOX: &str = "na0688c4_victim_inbox_tok_abcdefgh";
const PEER_INBOX: &str = "na0688c4_peer_inbox_token_ijklmnop";

/// Build a peer pair sharing the real relay, with `victim` reachable at [`VICTIM_INBOX`].
fn setup(root: &Path) -> (PathBuf, PathBuf) {
    let victim = party(root, "victim", VICTIM_INBOX);
    let peer = party(root, "peer", PEER_INBOX);
    let victim_fp = fingerprint(&victim);
    let peer_fp = fingerprint(&peer);
    add_contact(&victim, "peer", &peer_fp, PEER_INBOX);
    add_contact(&peer, "peer", &victim_fp, VICTIM_INBOX);
    (victim, peer)
}

// ---------------------------------------------------------------------------
// ARM Q4a — `handshake poll` against the ORDINARY inbox.
//
// The errand's original claim, now measured on both sides of the flip: `handshake poll` pulls up to
// `--max 4`, keeps only what parses as a handshake frame, and drops the rest. Under legacy the drop
// is a DESTRUCTION, because the relay already deleted the item at pull time — before any processing
// decision was reached.
// ---------------------------------------------------------------------------
#[test]
fn q4a_handshake_poll_destroys_under_legacy_and_preserves_under_lease() {
    let _g = guard();
    let relay =
        common::start_qsl_server_with_store(2 * 1024 * 1024, 512, None, TEST_PULL_LEASE_SECS);
    let base = relay.base_url().to_string();
    let root = test_root("na0688c4_q4a");
    let (victim, peer) = setup(&root);
    let out = root.join("out");
    ensure_dir_700(&out);

    // ---- CONTROL: legacy. The instrument MUST be able to return positive. ----
    set_ack_mode(&victim, "legacy");
    plant_ordinary_message(&peer, &base, &root, b"q4a legacy control message");
    let (_ok, poll_text) = run_any(
        &victim,
        &["handshake", "poll", "--peer", "peer", "--relay", &base, "--max", "4"],
    );
    let destroyed = !planted_message_still_recoverable(&victim, &base, VICTIM_INBOX, &out);
    assert!(
        destroyed,
        "CONTROL FAILED: under legacy the poll did not destroy the planted message, so this arm \
         cannot demonstrate anything about lease. poll output:\n{poll_text}"
    );

    // ---- TREATMENT: lease (the C4 default). ----
    let out2 = root.join("out2");
    ensure_dir_700(&out2);
    set_ack_mode(&victim, "lease");
    plant_ordinary_message(&peer, &base, &root, b"q4a lease treatment message");
    let (_ok2, poll_text2) = run_any(
        &victim,
        &["handshake", "poll", "--peer", "peer", "--relay", &base, "--max", "4"],
    );
    // ⚠ WAIT PAST THE LEASE. The poll pulled the item and never acked it, so the relay is holding
    // it INVISIBLE, not deleting it. Probing now would measure immediate visibility and report a
    // perfectly intact message as lost. After expiry the relay redelivers it -- which is the
    // property that actually makes the collateral recoverable.
    thread::sleep(LEASE_EXPIRY_WAIT);
    assert!(
        planted_message_still_recoverable(&victim, &base, VICTIM_INBOX, &out2),
        "under lease the collaterally-pulled message must survive and be recoverable. \
         poll output:\n{poll_text2}"
    );
}

// ---------------------------------------------------------------------------
// ARM Q4b — `invite finish` against the ORDINARY inbox.
//
// ⚠ THE ARM THE CENSUS CORRECTION EXISTS FOR. `invite_finish` pulls
// `relay_self_inbox_route_token()` — the mailbox where a peer's ordinary messages sit — at
// `--max 1`, and processes only `.next()`. It is a command the user is REQUIRED to run to complete
// an invite.
// ---------------------------------------------------------------------------
#[test]
fn q4b_invite_finish_destroys_under_legacy_and_preserves_under_lease() {
    let _g = guard();
    let relay =
        common::start_qsl_server_with_store(2 * 1024 * 1024, 512, None, TEST_PULL_LEASE_SECS);
    let base = relay.base_url().to_string();
    let root = test_root("na0688c4_q4b");
    let (victim, peer) = setup(&root);
    let out = root.join("out");
    ensure_dir_700(&out);

    // ---- CONTROL: legacy ----
    set_ack_mode(&victim, "legacy");
    plant_ordinary_message(&peer, &base, &root, b"q4b legacy control message");
    let (_ok, finish_text) = run_any(
        &victim,
        &["invite", "finish", "--alias", "peer", "--relay", &base],
    );
    let destroyed = !planted_message_still_recoverable(&victim, &base, VICTIM_INBOX, &out);
    assert!(
        destroyed,
        "CONTROL FAILED: under legacy `invite finish` did not destroy the planted ordinary \
         message, so this arm proves nothing about lease. finish output:\n{finish_text}"
    );

    // ---- TREATMENT: lease ----
    let out2 = root.join("out2");
    ensure_dir_700(&out2);
    set_ack_mode(&victim, "lease");
    plant_ordinary_message(&peer, &base, &root, b"q4b lease treatment message");
    let (_ok2, finish_text2) = run_any(
        &victim,
        &["invite", "finish", "--alias", "peer", "--relay", &base],
    );
    // ⚠ WAIT PAST THE LEASE -- see the note on `planted_message_still_recoverable`.
    thread::sleep(LEASE_EXPIRY_WAIT);
    assert!(
        planted_message_still_recoverable(&victim, &base, VICTIM_INBOX, &out2),
        "under lease `invite finish` must leave the peer's ordinary message recoverable. \
         finish output:\n{finish_text2}"
    );
}

// ---------------------------------------------------------------------------
// ARM Q4c — `invite accept` against the invite's OWN mailbox: THE TOPOLOGY CLAIM.
//
// ⚠ THIS ARM EXISTS TO STOP AN UNMEASURED CLAIM. C4's census asserted that collateral was "less
// likely" in the dedicated invite mailbox, and the ruling correctly called that a topology claim to
// be measured or not made. `invite accept` pulls `invite_id_wire`, a mailbox addressed by the
// invite id rather than by the user's route token.
//
// ⚠ THE HONEST OUTCOME MAY BE "NOT DEMONSTRATED" RATHER THAN "NOT POSSIBLE", and this test is
// written to say so: if an ordinary message cannot be routed into an invite mailbox by the means
// available to a peer, that is recorded as a failure to construct the negative — NOT as proof that
// no such route exists anywhere.
// ---------------------------------------------------------------------------
#[test]
fn q4c_invite_accept_pulls_a_dedicated_mailbox_not_the_ordinary_inbox() {
    let _g = guard();
    let relay =
        common::start_qsl_server_with_store(2 * 1024 * 1024, 512, None, TEST_PULL_LEASE_SECS);
    let base = relay.base_url().to_string();
    let root = test_root("na0688c4_q4c");
    let (victim, peer) = setup(&root);
    let out = root.join("out");
    ensure_dir_700(&out);

    // Mint an invite so there is a real invite id, and a real dedicated mailbox behind it.
    run_ok(&victim, &["invite", "create", "--relay", &base, "--ttl-secs", "3600"]);
    let listing = run_ok(&victim, &["invite", "list"]);
    let invite_id = listing
        .lines()
        .find_map(|l| l.strip_prefix("invite="))
        .and_then(|l| l.split_whitespace().next())
        .expect("invite id")
        .to_string();

    // A peer's ordinary message goes to the ORDINARY inbox, which is a different mailbox.
    set_ack_mode(&victim, "legacy");
    plant_ordinary_message(&peer, &base, &root, b"q4c ordinary message");

    // `invite accept` pulls the invite's own mailbox. If the topology claim holds, this cannot
    // touch the ordinary inbox at all — so the planted message survives even under LEGACY, which
    // is the strongest form of the claim and the one worth pinning.
    let (_ok, accept_text) = run_any(
        &victim,
        &["invite", "accept", "--invite-id", &invite_id, "--alias", "peer"],
    );
    assert!(
        planted_message_still_recoverable(&victim, &base, VICTIM_INBOX, &out),
        "`invite accept` must pull the invite's OWN mailbox, never the ordinary inbox — the \
         planted message was consumed, which falsifies the topology claim. accept output:\n{accept_text}"
    );
}
