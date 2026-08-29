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
//! ⚠⚠ THE METHOD, AND HOW NA-0770 (D-1411) CHANGED IT. Every arm here ONCE ran under legacy first,
//! selected through `qsc config set ack-mode legacy` — these three commands take no `--ack-mode`
//! flag, so the per-install preference was the only way to aim them. That control existed because
//! an arm run only under lease can pass VACUOUSLY: "the peer's message survived" proves nothing
//! unless the same topology is shown to DESTROY it. **A negative result is only evidence if the
//! instrument could have returned positive.**
//!
//! With delete-on-pull retired there is no destroying mode to run, and `config set ack-mode` now
//! REFUSES BY NAME. The requirement did not go away with the mode, so it is met a different way:
//!
//! **THE IN-LEASE PROBE.** Under lease a pulled-but-unacked item is held INVISIBLE until expiry.
//! So the arm probes TWICE against one plant — once INSIDE the lease window, where the message is
//! genuinely unrecoverable, and once AFTER expiry, where it must come back. The first probe is the
//! negative-capability control the legacy leg used to be, and it is MODE-FREE: it exercises only
//! shipped behaviour, needs no retired mode, and adds no test-only seam.
//!
//! ⚠ WHAT THE SUBSTITUTION DOES NOT PRESERVE, said plainly: the old control demonstrated
//! DESTRUCTION (the item was gone forever); the new one demonstrates INVISIBILITY (the item is
//! withheld, then returns). Both make the probe return `false`, which is what the arm needs — but
//! they are not the same fact about the world, and a reader must not cite this file as evidence
//! that a collateral pull can still destroy anything. It cannot; that is the point of the lane.
//!
//! ⚠ THE RELAY MUST BE THE REAL ONE. The test-local mock in `common` parses only `max=` and always
//! pops on pull, so it cannot express lease semantics and would make every arm here vacuous.

mod common;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::thread;
use std::time::{Duration, Instant};

/// A 45-second server-side pull lease.
///
/// ⚠⚠ THIS FILE DELIBERATELY NO LONGER MATCHES `NA_0644`'s 8s/20000ms, AND THE DIVERGENCE IS THE
/// POINT RATHER THAN DRIFT. The old comment here claimed parity with `NA_0644` and said "if you
/// change one, change both". That parity was true while both files only ever WAITED OUT a lease.
/// It stopped being true when NA-0770 gave this file the IN-LEASE PROBE: every other file needs
/// `LEASE_EXPIRY_WAIT > lease` and nothing more, but THIS file must fit TWO FULL CLI INVOCATIONS
/// — a command plus a `receive`, each paying an Argon2id vault unlock — INSIDE the lease window.
/// Those are different requirements and forcing one pair of numbers to serve both is how a
/// constant ends up wrong for everybody.
///
/// ⚠ THE NUMBERS ARE NOT GUESSED. CI measured the in-lease probe at **8.915s** on a 2-core runner
/// against the then-8s lease, and the self-asserting precondition below REFUSED rather than
/// reporting a false negative-capability result (PR #1802, `qsc-shard-10`). 45s is ~5x that
/// measured worst case; `LEASE_EXPIRY_WAIT` must exceed the lease for the redelivery half, so it
/// is 60s. ⚠ A local 6-core run CANNOT reproduce the overrun — the only instrument that found it
/// was a slower machine, which is why the precondition exists and must not be deleted.
///
/// ⚠⚠ NA-0770 (D-1411) WIDENED THESE 1s/2500ms -> 8s/20000ms, AND THE REASON IS LOAD-BEARING, NOT
/// FLAKE-CHASING. The retirement of `AckMode::Legacy` cost these arms their legacy control, and the
/// replacement (§ each arm below) probes INSIDE the lease window to show the instrument can still
/// return a negative. That probe is a full `qsc receive` — an Argon2id vault unlock among other
/// work — so the window must be wide enough to contain it on a CONTENDED box, where the suite runs
/// twelve shards on six cores. A 1-second window could not, and the arm would then report the
/// planted message as destroyed when it was merely still leased.
///
/// ⚠ THE WIDTH IS NOT ASSUMED — IT IS ASSERTED. Each arm times its in-lease probe against
/// [`LEASE_DURATION`] and FAILS LOUDLY if the probe overran, rather than silently converting a slow
/// box into a false negative-capability result. If that assertion ever fires, widen the lease; do
/// not delete the probe.
const TEST_PULL_LEASE_SECS: usize = 45;
const LEASE_EXPIRY_WAIT: Duration = Duration::from_millis(60_000);
/// The lease as a `Duration`, for the in-lease probe's self-check. Derived from the SAME constant
/// handed to the relay, so the two cannot drift apart.
const LEASE_DURATION: Duration = Duration::from_secs(TEST_PULL_LEASE_SECS as u64);

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

// NA-0770 (D-1411): `set_ack_mode` removed. `config set ack-mode` no longer sets anything — the
// key is a tombstone and the writer refuses it by name, so this helper could only ever have failed.

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
fn q4a_handshake_poll_preserves_the_collateral_it_pulls_under_lease() {
    let _g = guard();
    let relay =
        common::start_qsl_server_with_store(2 * 1024 * 1024, 512, None, TEST_PULL_LEASE_SECS);
    let base = relay.base_url().to_string();
    let root = test_root("na0688c4_q4a");
    let (victim, peer) = setup(&root);
    let out = root.join("out");
    ensure_dir_700(&out);

    // ---- ONE PLANT, ONE COLLATERAL POLL, THEN TWO PROBES OF THE SAME ITEM. ----
    //
    // ⚠ THE CLOCK STARTS BEFORE THE COMMAND, DELIBERATELY. The relay's lease begins when it SERVES
    // the pull, somewhere inside that command's runtime. Timing from before it therefore OVERSTATES
    // how much of the lease has elapsed, so the in-lease assertion below is conservative in the
    // safe direction: it can fail early, never pass late.
    plant_ordinary_message(&peer, &base, &root, b"q4a collateral message");
    let lease_clock = Instant::now();
    let (_ok, poll_text) = run_any(
        &victim,
        &["handshake", "poll", "--peer", "peer", "--relay", &base, "--max", "4"],
    );

    // ---- PROBE 1 — INSIDE THE LEASE. The negative-capability control (see the header). ----
    let recoverable_inside = planted_message_still_recoverable(&victim, &base, VICTIM_INBOX, &out);
    let probe1_finished_at = lease_clock.elapsed();
    assert!(
        probe1_finished_at < LEASE_DURATION,
        "PRECONDITION UNMET, NOT A RESULT: the in-lease probe finished at {probe1_finished_at:?}, \
         past the {LEASE_DURATION:?} lease, so it cannot tell 'withheld' from 'redelivered' and the \
         control below would be meaningless. Widen TEST_PULL_LEASE_SECS; do not delete this probe."
    );
    assert!(
        !recoverable_inside,
        "NEGATIVE-CAPABILITY CONTROL FAILED: `handshake poll` collaterally pulled the planted message, so \
         inside the lease it must be INVISIBLE. It was recoverable at {probe1_finished_at:?} — so \
         either the command never pulled it, and this arm measures nothing about collateral pulls, \
         or the relay is not honouring the lease. output:\n{poll_text}"
    );

    // ---- PROBE 2 — AFTER EXPIRY. The claim itself. ----
    let out2 = root.join("out2");
    ensure_dir_700(&out2);
    thread::sleep(LEASE_EXPIRY_WAIT);
    assert!(
        planted_message_still_recoverable(&victim, &base, VICTIM_INBOX, &out2),
        "under lease the collaterally-pulled message must survive and be recoverable \
         once the lease expires. output:\n{poll_text}"
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
fn q4b_invite_finish_preserves_the_ordinary_message_it_pulls_under_lease() {
    let _g = guard();
    let relay =
        common::start_qsl_server_with_store(2 * 1024 * 1024, 512, None, TEST_PULL_LEASE_SECS);
    let base = relay.base_url().to_string();
    let root = test_root("na0688c4_q4b");
    let (victim, peer) = setup(&root);
    let out = root.join("out");
    ensure_dir_700(&out);

    // ---- ONE PLANT, ONE COLLATERAL `invite finish`, THEN TWO PROBES OF THE SAME ITEM. ----
    //
    // ⚠ THE CLOCK STARTS BEFORE THE COMMAND, DELIBERATELY. The relay's lease begins when it SERVES
    // the pull, somewhere inside that command's runtime. Timing from before it therefore OVERSTATES
    // how much of the lease has elapsed, so the in-lease assertion below is conservative in the
    // safe direction: it can fail early, never pass late.
    plant_ordinary_message(&peer, &base, &root, b"q4b collateral message");
    let lease_clock = Instant::now();
    let (_ok, finish_text) = run_any(
        &victim,
        &["invite", "finish", "--alias", "peer", "--relay", &base],
    );

    // ---- PROBE 1 — INSIDE THE LEASE. The negative-capability control (see the header). ----
    let recoverable_inside = planted_message_still_recoverable(&victim, &base, VICTIM_INBOX, &out);
    let probe1_finished_at = lease_clock.elapsed();
    assert!(
        probe1_finished_at < LEASE_DURATION,
        "PRECONDITION UNMET, NOT A RESULT: the in-lease probe finished at {probe1_finished_at:?}, \
         past the {LEASE_DURATION:?} lease, so it cannot tell 'withheld' from 'redelivered' and the \
         control below would be meaningless. Widen TEST_PULL_LEASE_SECS; do not delete this probe."
    );
    assert!(
        !recoverable_inside,
        "NEGATIVE-CAPABILITY CONTROL FAILED: `invite finish` collaterally pulled the planted message, so \
         inside the lease it must be INVISIBLE. It was recoverable at {probe1_finished_at:?} — so \
         either the command never pulled it, and this arm measures nothing about collateral pulls, \
         or the relay is not honouring the lease. output:\n{finish_text}"
    );

    // ---- PROBE 2 — AFTER EXPIRY. The claim itself. ----
    let out2 = root.join("out2");
    ensure_dir_700(&out2);
    thread::sleep(LEASE_EXPIRY_WAIT);
    assert!(
        planted_message_still_recoverable(&victim, &base, VICTIM_INBOX, &out2),
        "under lease `invite finish` must leave the peer's ordinary message \
         recoverable once the lease expires. output:\n{finish_text}"
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
    plant_ordinary_message(&peer, &base, &root, b"q4c ordinary message");

    // `invite accept` pulls the invite's own mailbox. If the topology claim holds, this cannot
    // touch the ordinary inbox at all — so the planted message survives.
    //
    // ⚠ NA-0770 (D-1411) DELETED THIS ARM'S `set_ack_mode(&victim, "legacy")` AND NOTHING ELSE.
    // Unlike q4a/q4b, legacy was never this arm's CONTROL — it was a STRESS SETTING. The claim is
    // TOPOLOGICAL (accept reads a different mailbox), and it was pinned under the most destructive
    // mode available so that a pass could not be explained by lease's forgiveness. The vacuity
    // ground that justifies q4a/q4b's in-lease probe therefore does not apply here, and no probe
    // is added: there is no second outcome for this arm to distinguish.
    //
    // ⚠ WHAT IS LOST: TIME-INDEPENDENCE. Under legacy a wrong mailbox meant the message was gone
    // permanently, so a pass held no matter when the probe ran. Under lease a wrong mailbox leaves
    // it merely leased, so a probe run late enough would find it redelivered and pass anyway. The
    // probe here is immediate and therefore still inside the lease — the assertion holds — but it
    // is now ORDER-DEPENDENT where it used to be unconditional. Do not move it after a sleep.
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
