// ⚠ T8's driving arm sits behind `qsc_rng_failure_test_seam`, a NON-DEFAULT `--cfg` (D-0883:
// "normal builds must not read the seam selector"). The allow is the tree's own idiom for the
// seam tests — see `a2_signature_provider_rng_failure.rs:1`.
#![allow(unexpected_cfgs)]

//! NA-0742 (D-1378) LANE 2 — **THE INVITE-FINISH SCAN + THE PRODUCER ACKS**, the ENG-0196 repair.
//!
//! `invite finish` pulls the user's ORDINARY inbox at `--max 1` and processes only `.next()`. Any
//! frame that happens to sit at the head — a peer's ordinary message, another contact's handshake
//! frame — is fed to `decode_envelope_resp`, which fails, and the `?` at `invite/mod.rs:1168` turns
//! that into the command's exit. **The user is told their invite reply is malformed when it is
//! sitting perfectly intact one frame further down.**
//!
//! ## ⚠⚠ ENG-0196 HAS **TWO** FALSE-DIAGNOSIS SPELLINGS, AND ONLY ONE HAD EVER BEEN NAMED
//!
//! `decode_envelope_resp` checks byte 0 **before** byte 1 (`invite/mod.rs:747-753`):
//!
//! | head of the inbox | leading bytes | returns | named before? |
//! |---|---|---|---|
//! | QSP message envelope | `01 00` | `handshake_envelope_malformed` | yes — NA-0740 |
//! | **handshake frame** | `51 48` (`QHSM`) | **`handshake_envelope_version_newer`** | ⚠ **NO** |
//!
//! The second is reachable **without an adversary** — a bare A1 from any other contact lands in
//! exactly the mailbox `invite finish` pulls — and before this file it had **zero tests and zero
//! consumers tree-wide**. T1 and T2 drive one spelling each.
//!
//! ## THE REPAIR, AND WHAT THESE ARMS PIN
//!
//! Under `AckMode::Lease` **only**, `invite finish` scans DRAIN-FORWARD in capped batches, classifies
//! each frame by its leading bytes, and selects the **first `InviteResp`** — leaving everything else
//! untouched, unacked and undestroyed. The three callers that pull their own frames then **ack what
//! they consumed**, after that frame's LAST EFFECT.
//!
//! ## ⚠ WHY EVERY ARM IS LEASE-GATED, AND WHY LEGACY MUST BE BYTE-UNCHANGED
//!
//! Under `AckMode::Legacy` the relay DELETES ON PULL. A mode-blind scan would enlarge finish's pull
//! from 1 frame to as many as 128 **on a delete-on-pull server** — turning a one-frame loss into a
//! 16x-128x amplification. So the scan and every ack execute only under Lease, and T7 is the arm
//! that discriminates the two modes rather than asserting the bound.
//!
//! ## ⚠ THE RELAY IS THE REAL ONE
//!
//! Every arm drives the REAL in-process `qsl-server`. `common::start_inbox_server` — the test-local
//! mock — does not appear in this file: it always pops on pull and cannot express a lease, which
//! would make every residue and ack assertion here VACUOUS.

mod common;

use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, OnceLock};
use std::thread;
use std::time::{Duration, Instant};

/// ⚠ **PARITY.** The production relay runs `PULL_LEASE_SECS=60`, and that is the default for every
/// arm here. An arm that needs a lease to EXPIRE inside the test may set the short value below and
/// **must state it beside every figure it produces** — the committed precedent is
/// `na0688_c4_collateral_arms.rs:40`.
const PRODUCTION_PULL_LEASE_SECS: usize = 60;

/// A 1-second server-side pull lease, so an unacked frame becomes visible again quickly.
/// ⚠ **STATED BESIDE EVERY FIGURE THE ARMS USING IT PRODUCE.**
const SHORT_PULL_LEASE_SECS: usize = 1;
const LEASE_EXPIRY_WAIT: Duration = Duration::from_millis(2500);

fn guard() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

// ---------------------------------------------------------------------------
// HARNESS. Adopted wholesale from `na0688_c4_collateral_arms.rs` and
// `na0741_frame_class_dispatch.rs` rather than re-derived: these arms measure the scan and the
// acks, and bespoke scaffolding is a way to measure my own setup by mistake.
// ---------------------------------------------------------------------------

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
    // reject the repeated flag and the setup dies before any measurement runs.
    let mut c = common::qsc_std_command();
    c.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        // NA-0759 (`ENG-0243`, F4, FILE-SCOPED BY RULING): turn on the pull diagnostic that
        // `ENG-0193` built and nobody switched on. Without it a failed pull says only
        // `relay_inbox_pull_failed` — the OPERATION, never the REASON — which is exactly why the
        // `0b9d6967` red could not be localized from its own log. ⚠ Deliberately NOT suite-wide:
        // it adds one `event=relay_pull_diagnostic` line per pull, and this house has twice paid
        // for extra marker lines under consumers that count or equality-match. Safe HERE because
        // every consumer in this file (`has_marker_line`, `marker_lines`, `count_marker`) filters
        // on `event=<name>` first, and no assertion in this file reads that event.
        .env("QSC_RELAY_PULL_DIAGNOSTIC", "redacted");
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

/// ⚠ **LINE-SCOPED, ON PURPOSE.** Two independent `contains` calls over a whole capture are
/// satisfied by two DIFFERENT lines. Every marker assertion in this file reads its fields from the
/// SAME emission.
fn has_marker_line(text: &str, event: &str, needles: &[&str]) -> bool {
    let ev = format!("event={event}");
    text.lines()
        .any(|l| l.contains(&ev) && needles.iter().all(|n| l.contains(n)))
}

fn marker_lines<'a>(text: &'a str, event: &str) -> Vec<&'a str> {
    let ev = format!("event={event}");
    text.lines().filter(|l| l.contains(&ev)).collect()
}

fn count_marker(text: &str, event: &str) -> usize {
    marker_lines(text, event).len()
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

/// ⚠ Adding the contact is NOT enough to send: its device must also be TRUSTED.
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

/// ⚠ **THE MODE IS ALWAYS SET EXPLICITLY.** `invite finish`, `invite accept` and `handshake poll`
/// take no `--ack-mode` flag, so the per-install preference is the ONLY way to aim them — which
/// makes this config key part of the instrument rather than a convenience. An arm that inherited
/// the C4 default would stop measuring the moment that default moved.
fn set_ack_mode(cfg: &Path, mode: &str) {
    run_ok(cfg, &["config", "set", "ack-mode", mode]);
}

/// Put an arbitrary frame into a mailbox through the relay's OWN public push route — the same route
/// the client uses, and the same route a peer's handshake or invite frame arrives by.
fn push_raw(base: &str, route_token: &str, bytes: &[u8]) {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("push client");
    let resp = client
        .post(format!("{}/v1/push", base.trim_end_matches('/')))
        .header("X-QSL-Route-Token", route_token)
        .body(bytes.to_vec())
        .send()
        .expect("push frame");
    assert!(
        resp.status().is_success(),
        "the relay refused the frame, so the arm would measure nothing: {}",
        resp.status()
    );
}

/// Read a mailbox's CURRENTLY VISIBLE items straight off the wire.
///
/// ⚠ **`ack=lease` IS THE NON-DESTRUCTIVE CHOICE.** A legacy pull DELETES what it returns, so a
/// residency probe written that way would consume the very evidence it exists to report.
fn raw_pull_lease(base: &str, route_token: &str, max: usize) -> Vec<Vec<u8>> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("pull client");
    let resp = client
        .get(format!(
            "{}/v1/pull?max={}&ack=lease",
            base.trim_end_matches('/'),
            max
        ))
        .header("X-QSL-Route-Token", route_token)
        .send()
        .expect("raw pull");
    if resp.status().as_u16() == 204 {
        return Vec::new();
    }
    assert!(
        resp.status().is_success(),
        "raw pull failed: {}",
        resp.status()
    );
    let body: serde_json::Value = resp.json().expect("pull json");
    body.get("items")
        .and_then(|i| i.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|it| it.get("data").and_then(|d| d.as_array()))
                .map(|d| {
                    d.iter()
                        .filter_map(|n| n.as_u64())
                        .map(|n| n as u8)
                        .collect::<Vec<u8>>()
                })
                .collect()
        })
        .unwrap_or_default()
}

// ---------------------------------------------------------------------------
// FIXTURES.
// ---------------------------------------------------------------------------

/// A frame carrying the handshake magic. ⚠ The magic is spelled here rather than referenced because
/// `HS_MAGIC` is `pub(crate)` — deliberately NOT part of the public surface — so an integration test
/// cannot see it. T2's own RED outcome pins the bytes: a frame whose FIRST byte is not
/// `ENVELOPE_VER` is what produces `handshake_envelope_version_newer`, and no transcription error
/// could produce that code by accident.
const HS_MAGIC_WIRE: &[u8; 4] = b"QHSM";

fn handshake_frame() -> Vec<u8> {
    let mut v = HS_MAGIC_WIRE.to_vec();
    v.extend_from_slice(b"\x00\x01\x03na0742-handshake-body");
    v
}

// ---------------------------------------------------------------------------
// THE ARRANGEMENT.
//
// Three parties, because the ENG-0196 story is TRAFFIC FROM A DIFFERENT CONTACT: `carol` sends the
// redeemer an ordinary message, which lands in exactly the mailbox `invite finish` pulls. Using the
// inviter for that would mean giving one identity two labels on the same side and measuring the
// contact store instead of the scan.
// ---------------------------------------------------------------------------

const INVITER_INBOX: &str = "na0742-inviter-inbox-tok-aaaaaaaa";
const REDEEMER_INBOX: &str = "na0742-redeemer-inbox-tok-bbbbbbb";
const CAROL_INBOX: &str = "na0742-carol-inbox-token-ccccccccc";
/// ⚠ ONE shared label for the carol<->redeemer channel — see `setup_to_redeem`.
const CAROL_LABEL: &str = "carol";

struct Flow {
    inviter: PathBuf,
    redeemer: PathBuf,
    carol: PathBuf,
    invite_id: String,
}

/// `invite create` + `invite redeem`, stopping BEFORE `invite accept` so a caller can plant a frame
/// that will sit AHEAD of the invite reply in the redeemer's inbox.
fn setup_to_redeem(root: &Path, base: &str, mode: &str) -> Flow {
    let inviter = party(root, "inviter", INVITER_INBOX);
    let redeemer = party(root, "redeemer", REDEEMER_INBOX);
    let carol = party(root, "carol", CAROL_INBOX);
    for cfg in [&inviter, &redeemer, &carol] {
        set_ack_mode(cfg, mode);
    }
    // carol <-> redeemer, so carol can send a REAL ordinary message and the redeemer can receive it.
    //
    // ⚠⚠ **BOTH SIDES USE THE SAME LABEL, AND IT IS LOAD-BEARING RATHER THAN COSMETIC.** The label
    // is a shared CHANNEL name, not the other party's name — `na0708_ack_flush.rs`,
    // `na0688_c4_collateral_arms.rs` and `na0741_frame_class_dispatch.rs` all do this and lane 1
    // states why: `--from <label>` must resolve to the sender's identity. Measured the hard way
    // here: a first pass labelled the pair `redeemer`/`carol` and the delivery arm died with
    // `qsp_unpack code=qsp_hdr_auth_failed` — the message was intact on the wire and simply could
    // not be authenticated under a mismatched label. ⚠ The RED-FIRST arms did not catch it, because
    // they only need the message PRESENT at the head and never receive it.
    let carol_fp = fingerprint(&carol);
    let redeemer_fp = fingerprint(&redeemer);
    add_contact(&carol, CAROL_LABEL, &redeemer_fp, REDEEMER_INBOX);
    add_contact(&redeemer, CAROL_LABEL, &carol_fp, CAROL_INBOX);

    let code = run_ok(
        &inviter,
        &["invite", "create", "--relay", base, "--ttl-secs", "3600"],
    );
    let code = code
        .lines()
        .find(|l| l.starts_with("QSLI-1-"))
        .expect("invite code on stdout")
        .trim()
        .to_string();
    let listing = run_ok(&inviter, &["invite", "list"]);
    let invite_id = listing
        .lines()
        .find_map(|l| l.strip_prefix("invite="))
        .and_then(|l| l.split_whitespace().next())
        .expect("invite id")
        .to_string();
    run_ok(
        &redeemer,
        &["invite", "redeem", "--code", &code, "--alias", "inviter"],
    );
    Flow {
        inviter,
        redeemer,
        carol,
        invite_id,
    }
}

/// The inviter answers: pulls the invite slot, runs the poll as RESPONDER, pushes the wrapped B1
/// into the redeemer's ORDINARY inbox — behind whatever was planted first.
fn accept(flow: &Flow) -> String {
    run_ok(
        &flow.inviter,
        &[
            "invite",
            "accept",
            "--invite-id",
            &flow.invite_id,
            "--alias",
            "redeemer",
        ],
    )
}

fn finish(flow: &Flow, base: &str) -> (bool, String) {
    run_any(
        &flow.redeemer,
        &["invite", "finish", "--alias", "inviter", "--relay", base],
    )
}

/// carol -> redeemer, a REAL ordinary message over the real relay. ⚠ Real rather than synthetic:
/// the frame is produced by the product's OWN envelope encoder, so its `01 00` head cannot drift
/// out of agreement with the encoder the way a transcribed byte pair can.
fn plant_ordinary_message(flow: &Flow, base: &str, root: &Path, name: &str, body: &[u8]) {
    let f = root.join(name);
    fs::write(&f, body).expect("write planted body");
    let text = run_ok(
        &flow.carol,
        &[
            "send",
            "--transport",
            "relay",
            "--relay",
            base,
            "--to",
            CAROL_LABEL,
            "--file",
            f.to_str().expect("path"),
        ],
    );
    assert!(
        text.contains("QSC_DELIVERY state=accepted_by_relay"),
        "the planted message never reached the relay, so the arm would measure nothing:\n{text}"
    );
}

// ===========================================================================
// T1 — ENG-0196, SPELLING 1: a QSP message (`01 00`) at the head.
//
// CLASS: **RED-FIRST.** On the unrepaired tree this arm FAILS, and the failure is the defect.
// ===========================================================================

#[test]
fn t1_ordinary_message_at_the_head_does_not_break_invite_finish() {
    let _g = guard();
    let relay =
        common::start_qsl_server_with_store(2 * 1024 * 1024, 512, None, PRODUCTION_PULL_LEASE_SECS);
    let base = relay.base_url().to_string();
    let root = test_root("na0742_t1");
    let flow = setup_to_redeem(&root, &base, "lease");

    // The head: a REAL ordinary message from a DIFFERENT contact.
    plant_ordinary_message(
        &flow,
        &base,
        &root,
        "t1_msg.txt",
        b"na0742 t1 ordinary message",
    );
    // Behind it: the real invite reply.
    accept(&flow);

    let (ok, text) = finish(&flow, &base);
    assert!(
        ok,
        "ENG-0196 SPELLING 1: an ordinary message from another contact sat at the head of the \
         redeemer's inbox, and `invite finish` reported the invite reply malformed while the reply \
         was intact one frame behind it.\n{text}"
    );
    assert!(
        text.contains("invite_finish=ok"),
        "finish must report ok once it can see past the head:\n{text}"
    );
    assert!(
        has_marker_line(&text, "handshake_complete", &["role=initiator"]),
        "the handshake must actually complete, not merely exit 0:\n{text}"
    );
    assert!(
        !text.contains("handshake_envelope_malformed"),
        "the FALSE DIAGNOSIS must be gone, not merely survived:\n{text}"
    );
}

// ===========================================================================
// T2 — ENG-0196, SPELLING 2 (expectation E1b): a handshake frame (`51 48`) at the head.
//
// CLASS: **RED-FIRST.** ⚠ THE GREEN SIDE IS "NO ERROR AT ALL", NOT A DIFFERENT ERROR. After the
// repair `handshake_envelope_version_newer` is UNREACHABLE on the finish path: a classified
// `InviteResp` has `byte0 == 0x01`, so the version check cannot fire on anything the scan selects.
// It stays live on `invite accept` and in the decoder's own tests — this arm is not a claim that
// the code is dead everywhere.
// ===========================================================================

#[test]
fn t2_handshake_frame_at_the_head_does_not_break_invite_finish() {
    let _g = guard();
    let relay =
        common::start_qsl_server_with_store(2 * 1024 * 1024, 512, None, PRODUCTION_PULL_LEASE_SECS);
    let base = relay.base_url().to_string();
    let root = test_root("na0742_t2");
    let flow = setup_to_redeem(&root, &base, "lease");

    // The head: a bare handshake frame, as a bare A1 from any other contact would be.
    push_raw(&base, REDEEMER_INBOX, &handshake_frame());
    accept(&flow);

    let (ok, text) = finish(&flow, &base);
    assert!(
        ok,
        "ENG-0196 SPELLING 2: a handshake frame at the head made `decode_envelope_resp` reject on \
         BYTE 0 and report `handshake_envelope_version_newer` — a version complaint about a frame \
         that is not an envelope at all.\n{text}"
    );
    assert!(
        text.contains("invite_finish=ok"),
        "finish must report ok once it can see past the head:\n{text}"
    );
    assert!(
        !text.contains("handshake_envelope_version_newer"),
        "the SECOND spelling must be gone too — it is unreachable on this path after the repair:\n{text}"
    );
}

// ===========================================================================
// T3 — ZERO RESIDUE (discharges lane 1's owed E3(b)).
//
// CLASS: **BASELINE CONTROL — BOTH NUMBERS REPORTED.** The unrepaired figure is a deliverable in
// its own right, not a step on the way to green.
//
// ⚠ `PULL_LEASE_SECS = 1` **AND EVERY FIGURE BELOW IS STATED AT THAT VALUE.** The reason is
// mechanical, not stylistic: a leased-but-unacked row is INVISIBLE to a pull, so a residue probe
// run before expiry reports "empty" for a mailbox that is merely reserved. Waiting past expiry is
// what makes the probe able to tell ACKED (deleted, gone forever) from LEASED (coming back).
// ===========================================================================

/// Drive the whole flow to completion, including the inviter's own `handshake poll` — the ONLY
/// caller in this lane that reaches the poll through `HsPollSource::Relay`, and therefore the only
/// one whose ack the poll performs.
fn drive_full_flow(root: &Path, base: &str) -> Flow {
    let flow = setup_to_redeem(root, base, "lease");
    accept(&flow);
    let (ok, text) = finish(&flow, base);
    assert!(
        ok,
        "the flow must complete before residue means anything — `invite finish` failed:\n{text}"
    );
    // The inviter collects the A2 the redeemer just pushed. THIS is the poll's Relay arm.
    let (poll_ok, poll_text) = run_any(
        &flow.inviter,
        &[
            "handshake",
            "poll",
            "--peer",
            "redeemer",
            "--relay",
            base,
            "--max",
            "4",
        ],
    );
    assert!(poll_ok, "the inviter's poll must succeed:\n{poll_text}");
    assert!(
        has_marker_line(&poll_text, "handshake_complete", &["role=responder"]),
        "the inviter must actually complete as responder, or the A2 was never consumed and the \
         residue figure would be measuring a different thing:\n{poll_text}"
    );
    flow
}

#[test]
fn t3_a_completed_flow_leaves_zero_residue() {
    let _g = guard();
    let relay =
        common::start_qsl_server_with_store(2 * 1024 * 1024, 512, None, SHORT_PULL_LEASE_SECS);
    let base = relay.base_url().to_string();
    let root = test_root("na0742_t3");
    let flow = drive_full_flow(&root, &base);

    // ⚠ WAIT PAST THE LEASE (PULL_LEASE_SECS=1). Before expiry every unacked frame is held
    // invisible and the probe would report zero for a mailbox that is merely reserved.
    thread::sleep(LEASE_EXPIRY_WAIT);

    let mailboxes = [
        ("inviter inbox", INVITER_INBOX.to_string()),
        ("redeemer inbox", REDEEMER_INBOX.to_string()),
        ("invite slot", flow.invite_id.clone()),
    ];
    let mut residue = Vec::new();
    for (name, token) in &mailboxes {
        let items = raw_pull_lease(&base, token, 128);
        if !items.is_empty() {
            let heads: Vec<String> = items
                .iter()
                .map(|b| {
                    b.iter()
                        .take(4)
                        .map(|x| format!("{x:02x}"))
                        .collect::<Vec<_>>()
                        .join(" ")
                })
                .collect();
            residue.push(format!(
                "{name}: {} frame(s) head4=[{}]",
                items.len(),
                heads.join(" | ")
            ));
        }
    }
    assert!(
        residue.is_empty(),
        "E3: a completed invite + handshake flow must leave ZERO residue in every mailbox it \
         touched, at PULL_LEASE_SECS=1 measured past expiry. Surviving frames are each named here \
         rather than counted:\n  {}",
        residue.join("\n  ")
    );
}

// ===========================================================================
// T4 — THE TAX ENDS.
//
// CLASS: **BASELINE CONTROL — BOTH NUMBERS REPORTED.**
//
// Lane 1 stopped the wedge by SKIPPING known-foreign frames, which converted an aborted batch into
// a per-lease-period skip tax: every `receive` steps over the same invite residue again. This lane
// removes the frames at their source, so the tax goes to zero. ⚠ `PULL_LEASE_SECS = 1`, stated
// beside the figure — before expiry the residue is invisible and the tax would read 0 VACUOUSLY.
// ===========================================================================

#[test]
fn t4_a_receive_after_a_completed_flow_skips_nothing() {
    let _g = guard();
    let relay =
        common::start_qsl_server_with_store(2 * 1024 * 1024, 512, None, SHORT_PULL_LEASE_SECS);
    let base = relay.base_url().to_string();
    let root = test_root("na0742_t4");
    let flow = drive_full_flow(&root, &base);

    thread::sleep(LEASE_EXPIRY_WAIT);

    let i_out = root.join("inviter_out");
    ensure_dir_700(&i_out);
    let (_i_ok, i_text) = run_any(
        &flow.inviter,
        &[
            "receive",
            "--transport",
            "relay",
            "--relay",
            &base,
            "--mailbox",
            INVITER_INBOX,
            "--from",
            "redeemer",
            "--max",
            "8",
            "--out",
            i_out.to_str().expect("out"),
            "--ack-mode",
            "lease",
        ],
    );
    let r_out = root.join("redeemer_out");
    ensure_dir_700(&r_out);
    let (_r_ok, r_text) = run_any(
        &flow.redeemer,
        &[
            "receive",
            "--transport",
            "relay",
            "--relay",
            &base,
            "--mailbox",
            REDEEMER_INBOX,
            "--from",
            "inviter",
            "--max",
            "8",
            "--out",
            r_out.to_str().expect("out"),
            "--ack-mode",
            "lease",
        ],
    );
    let skipped =
        count_marker(&i_text, "recv_frame_skipped") + count_marker(&r_text, "recv_frame_skipped");
    assert_eq!(
        skipped,
        0,
        "the skip tax must be ZERO after a completed flow (PULL_LEASE_SECS=1, measured past \
         expiry); the surviving skips are:\n  inviter: {:?}\n  redeemer: {:?}",
        marker_lines(&i_text, "recv_frame_skipped"),
        marker_lines(&r_text, "recv_frame_skipped")
    );
}

// ===========================================================================================
// THE T5 FAULT PROXY — TEST-OWNED, ZERO PRODUCT BYTES.
//
// A blocking HTTP proxy in front of the real in-process relay. Everything is forwarded verbatim
// EXCEPT `POST /v1/pull/ack`, which is answered **500**.
//
// ⚠⚠ **500 AND NEVER 404.** `transport/mod.rs` maps `NOT_FOUND` to
// `AckFlushOutcome::LegacyComplete` — the pre-durability-relay tolerance path. A 404 here would
// turn the INJECTED FAILURE INTO A SUCCESS PATH and every T5 arm would go vacuous while still
// reporting green. The status is asserted below so the trap cannot be reintroduced silently.
// ===========================================================================================

const ACK_FAULT_STATUS: u16 = 500;

struct AckFaultProxy {
    base_url: String,
    shutdown: Arc<AtomicBool>,
    faulted: Arc<AtomicUsize>,
    handle: Option<thread::JoinHandle<()>>,
}

impl AckFaultProxy {
    fn base_url(&self) -> &str {
        &self.base_url
    }
    /// How many ack attempts were answered with [`ACK_FAULT_STATUS`]. ⚠ Every arm asserts this is
    /// non-zero BEFORE scoring: an arm whose injected fault never fired is measuring the happy
    /// path under a fault's name.
    fn faulted(&self) -> usize {
        self.faulted.load(Ordering::SeqCst)
    }
}

impl Drop for AckFaultProxy {
    fn drop(&mut self) {
        self.shutdown.store(true, Ordering::SeqCst);
        let _ = TcpStream::connect(self.base_url.trim_start_matches("http://"));
        if let Some(h) = self.handle.take() {
            let _ = h.join();
        }
    }
}

/// NA-0759 (`ENG-0243`, F2): **THE HOUSE PATTERN, NOT A NEW ONE.** This is
/// `common/mod.rs`'s `read_until_header_end` shape — tolerate the three "nothing has arrived
/// yet" kinds and bound the wait with a deadline — applied to the one fixture in this tree that
/// lacked it. The old body answered EVERY `Err` with `None`, so a `WouldBlock` (and a
/// `SO_RCVTIMEO` expiry, which surfaces as `WouldBlock` too) meant "give up"; the caller answers
/// `None` by dropping the connection with **no response written at all**, which reaches the
/// client as a bare transport error and is collapsed by `transport::relay_send_outcome_for_error`
/// into `relay_inbox_pull_failed` — the marker that reddened macOS main at `0b9d6967`, with
/// `faulted()` still 0 because the request was never classified.
fn read_head(stream: &mut TcpStream) -> Option<(Vec<u8>, usize)> {
    let mut buf = Vec::new();
    let mut byte = [0u8; 1];
    let deadline = Instant::now() + Duration::from_secs(20);
    while Instant::now() < deadline {
        match stream.read(&mut byte) {
            Ok(0) => return None,
            Ok(_) => buf.push(byte[0]),
            Err(e)
                if matches!(
                    e.kind(),
                    std::io::ErrorKind::Interrupted
                        | std::io::ErrorKind::WouldBlock
                        | std::io::ErrorKind::TimedOut
                ) =>
            {
                continue
            }
            Err(_) => return None,
        }
        if buf.len() >= 4 && &buf[buf.len() - 4..] == b"\r\n\r\n" {
            return Some((buf.clone(), buf.len()));
        }
        if buf.len() > 64 * 1024 {
            return None;
        }
    }
    None
}

fn start_ack_fault_proxy(upstream: &str) -> AckFaultProxy {
    let listener = TcpListener::bind("127.0.0.1:0").expect("proxy bind");
    let addr = listener.local_addr().expect("proxy addr");
    listener.set_nonblocking(true).expect("proxy nonblocking");
    let shutdown = Arc::new(AtomicBool::new(false));
    let faulted = Arc::new(AtomicUsize::new(0));
    let up = upstream.trim_end_matches('/').to_string();
    let sd = Arc::clone(&shutdown);
    let ft = Arc::clone(&faulted);
    let handle = thread::spawn(move || {
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(20))
            .build()
            .expect("proxy client");
        while !sd.load(Ordering::SeqCst) {
            let mut stream = match listener.accept() {
                Ok((s, _)) => s,
                Err(_) => {
                    thread::sleep(Duration::from_millis(10));
                    continue;
                }
            };
            // NA-0759 (`ENG-0243`, F1): CLEAR the flag the accepted socket may have INHERITED.
            // `set_nonblocking(true)` above is for the LISTENER. Linux does not pass that flag to
            // the accepted socket (measured: listener true / accepted false); BSD-derived kernels
            // are documented to. On a non-blocking socket the `set_read_timeout` below is INERT
            // and `read_head`'s first read returns `WouldBlock` before the request has landed —
            // which is one line of platform difference standing between a green shard and a red
            // one. Clearing it explicitly makes the arrangement the same on every runner.
            let _ = stream.set_nonblocking(false);
            let _ = stream.set_read_timeout(Some(Duration::from_secs(20)));
            let _ = stream.set_write_timeout(Some(Duration::from_secs(20)));
            let Some((head, _)) = read_head(&mut stream) else {
                continue;
            };
            let head_s = String::from_utf8_lossy(&head).to_string();
            let mut lines = head_s.split("\r\n");
            let request_line = lines.next().unwrap_or("").to_string();
            let mut parts = request_line.split_whitespace();
            let method = parts.next().unwrap_or("").to_string();
            let path = parts.next().unwrap_or("/").to_string();
            let mut headers: Vec<(String, String)> = Vec::new();
            let mut content_length = 0usize;
            for l in lines {
                if l.is_empty() {
                    continue;
                }
                if let Some((k, v)) = l.split_once(':') {
                    let (k, v) = (k.trim().to_string(), v.trim().to_string());
                    if k.eq_ignore_ascii_case("content-length") {
                        content_length = v.parse().unwrap_or(0);
                    }
                    headers.push((k, v));
                }
            }
            let mut body = vec![0u8; content_length];
            if content_length > 0 && stream.read_exact(&mut body).is_err() {
                continue;
            }

            // ⚠ THE INJECTED FAULT, AND THE ONLY THING THIS PROXY CHANGES.
            if method.eq_ignore_ascii_case("POST") && path.starts_with("/v1/pull/ack") {
                ft.fetch_add(1, Ordering::SeqCst);
                let _ = stream.write_all(
                    format!(
                        "HTTP/1.1 {} Internal Server Error\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                        ACK_FAULT_STATUS
                    )
                    .as_bytes(),
                );
                let _ = stream.flush();
                continue;
            }

            let url = format!("{}{}", up, path);
            let mut req = if method.eq_ignore_ascii_case("POST") {
                client.post(&url).body(body)
            } else {
                client.get(&url)
            };
            for (k, v) in &headers {
                let lk = k.to_ascii_lowercase();
                if lk == "host" || lk == "connection" || lk == "content-length" {
                    continue;
                }
                req = req.header(k.as_str(), v.as_str());
            }
            match req.send() {
                Ok(resp) => {
                    let status = resp.status().as_u16();
                    let ctype = resp
                        .headers()
                        .get("content-type")
                        .and_then(|v| v.to_str().ok())
                        .unwrap_or("application/octet-stream")
                        .to_string();
                    let bytes = resp.bytes().map(|b| b.to_vec()).unwrap_or_default();
                    let _ = stream.write_all(
                        format!(
                            "HTTP/1.1 {} X\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                            status,
                            ctype,
                            bytes.len()
                        )
                        .as_bytes(),
                    );
                    let _ = stream.write_all(&bytes);
                }
                Err(_) => {
                    let _ = stream.write_all(
                        b"HTTP/1.1 502 Bad Gateway\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                    );
                }
            }
            let _ = stream.flush();
        }
    });
    AckFaultProxy {
        base_url: format!("http://{}", addr),
        shutdown,
        faulted,
        handle: Some(handle),
    }
}

/// Leading-byte classification of a frame read straight off the wire, so a residency probe can say
/// WHAT survived rather than how many things did.
fn is_invite_resp(frame: &[u8]) -> bool {
    frame.len() >= 2
        && frame[0] == qsc::invite::ENVELOPE_VER
        && frame[1] == qsc::invite::ENVELOPE_TYPE_RESP
}

fn invite_init_frame(route_token: &str) -> Vec<u8> {
    qsc::invite::encode_envelope(&qsc::invite::HandshakeEnvelope {
        bundle: b"na0742-collateral-bundle".to_vec(),
        route_token: route_token.to_string(),
        a1: b"na0742-collateral-a1".to_vec(),
    })
    .expect("encode invite request envelope")
}

// ===========================================================================
// T5 — THE CRASH WINDOW. Three sub-arms, each asserting its ANTECEDENT with a named failure
// BEFORE it scores anything.
//
// CLASS: **POST-REPAIR, ANTECEDENT-CONTROLLED.** ⚠ `PULL_LEASE_SECS = 1`, stated beside every
// figure these arms produce.
// ===========================================================================

/// ⚠ THE TRAP THIS PINS. A 404 from the ack route is mapped to `LegacyComplete` — "the old relay
/// already delivered, nothing was lost". Injecting 404 instead of 500 would convert the fault into
/// a success and every T5 arm would pass while measuring nothing.
#[test]
fn t5_the_injected_ack_fault_is_500_and_never_404() {
    assert_eq!(
        ACK_FAULT_STATUS, 500,
        "the ack fault must be 500: a 404 is mapped to AckFlushOutcome::LegacyComplete and would \
         make every T5 arm vacuous while still reporting green"
    );
}

#[test]
fn t5f_finish_survives_a_lost_ack_and_the_frame_redelivers() {
    let _g = guard();
    let relay =
        common::start_qsl_server_with_store(2 * 1024 * 1024, 512, None, SHORT_PULL_LEASE_SECS);
    let base = relay.base_url().to_string();
    let proxy = start_ack_fault_proxy(&base);
    let root = test_root("na0742_t5f");
    let flow = setup_to_redeem(&root, &base, "lease");
    accept(&flow);

    // ---- finish THROUGH THE PROXY: the commit lands, the ack is 500'd. ----
    let (ok, text) = run_any(
        &flow.redeemer,
        &[
            "invite",
            "finish",
            "--alias",
            "inviter",
            "--relay",
            proxy.base_url(),
        ],
    );

    // ANTECEDENT 1 — the fault actually fired.
    assert!(
        proxy.faulted() > 0,
        "ANTECEDENT FAILED: the ack route was never reached, so this arm never exercised a lost \
         ack and proves nothing about the crash window. finish output:\n{text}"
    );
    // ANTECEDENT 2 — the ack was reported as failed, not silently succeeded.
    assert!(
        has_marker_line(&text, "producer_ack", &["caller=finish", "acked=0"]),
        "ANTECEDENT FAILED: the 500 did not surface as a failed producer ack:\n{text}"
    );

    // ---- THE ASSERTION: the lost ack is a NON-EVENT for the caller. ----
    assert!(
        ok,
        "THE LOST-ACK POSTURE IS THE ASSERTION, NOT A TOLERANCE: `invite finish` must still exit 0 \
         when the ack fails. The state it committed is already durable; turning a lost ack into a \
         failed command would be a worse defect than the one this lane fixes.\n{text}"
    );
    assert!(text.contains("invite_finish=ok"), "{text}");

    // ---- THE COST, MEASURED: one lease period of redelivery (PULL_LEASE_SECS=1). ----
    thread::sleep(LEASE_EXPIRY_WAIT);
    let redelivered = raw_pull_lease(&base, REDEEMER_INBOX, 16);
    assert!(
        redelivered.iter().any(|f| is_invite_resp(f)),
        "the unacked RESP must REDELIVER after the lease expires (PULL_LEASE_SECS=1) — that is \
         precisely what makes a lost ack recoverable rather than a loss"
    );
    thread::sleep(LEASE_EXPIRY_WAIT);

    // ---- THE RETRY IS A DONE NO-OP, AND ITS ACK LANDS (against the real relay). ----
    let (retry_ok, retry_text) = finish(&flow, &base);
    assert!(
        retry_ok,
        "the retry after a lost ack must be a done no-op, not a failure:\n{retry_text}"
    );
    assert!(
        has_marker_line(&retry_text, "producer_ack", &["caller=finish", "acked=1"]),
        "the retry's ack must LAND, retiring the frame the crashed run could not:\n{retry_text}"
    );
}

#[test]
fn t5a_accept_survives_a_lost_ack_on_the_slot_mailbox() {
    let _g = guard();
    let relay =
        common::start_qsl_server_with_store(2 * 1024 * 1024, 512, None, SHORT_PULL_LEASE_SECS);
    let base = relay.base_url().to_string();
    let proxy = start_ack_fault_proxy(&base);
    let root = test_root("na0742_t5a");
    // ⚠ The invite is created THROUGH THE PROXY so that `invite accept` — which takes its endpoint
    // from the stored invite record rather than from a flag — reaches the faulted ack route.
    let flow = setup_to_redeem(&root, proxy.base_url(), "lease");

    let out = qsc(&flow.inviter)
        .args([
            "invite",
            "accept",
            "--invite-id",
            &flow.invite_id,
            "--alias",
            "redeemer",
        ])
        .output()
        .expect("run qsc");
    let ok = out.status.success();
    let text = output_text(&out);

    assert!(
        proxy.faulted() > 0,
        "ANTECEDENT FAILED: the ack route was never reached:\n{text}"
    );
    assert!(
        has_marker_line(&text, "producer_ack", &["caller=accept", "acked=0"]),
        "ANTECEDENT FAILED: the 500 did not surface as a failed producer ack:\n{text}"
    );
    assert!(
        ok,
        "`invite accept` must still exit 0 when its ack is lost — the slot is already Redeemed \
         durably:\n{text}"
    );

    // The commit landed: the slot is burned client-side regardless of the ack.
    let listing = run_ok(&flow.inviter, &["invite", "list"]);
    assert!(
        listing.contains("redeemed") || listing.contains("Redeemed"),
        "the slot must read Redeemed — the durable commit is what the ack was supposed to \
         follow:\n{listing}"
    );

    // The A1 envelope redelivers on the SLOT mailbox (PULL_LEASE_SECS=1).
    thread::sleep(LEASE_EXPIRY_WAIT);
    let redelivered = raw_pull_lease(&base, &flow.invite_id, 16);
    assert!(
        !redelivered.is_empty(),
        "the unacked A1 must redeliver on the invite slot after the lease expires \
         (PULL_LEASE_SECS=1)"
    );
}

#[test]
fn t5p_the_poll_tolerates_a_redelivered_already_processed_frame() {
    let _g = guard();
    let relay =
        common::start_qsl_server_with_store(2 * 1024 * 1024, 512, None, SHORT_PULL_LEASE_SECS);
    let base = relay.base_url().to_string();
    let proxy = start_ack_fault_proxy(&base);
    let root = test_root("na0742_t5p");
    let flow = setup_to_redeem(&root, &base, "lease");
    accept(&flow);
    let (fin_ok, fin_text) = finish(&flow, &base);
    assert!(fin_ok, "setup: finish must succeed:\n{fin_text}");

    // ---- The poll's Relay arm, THROUGH THE PROXY. ----
    let (poll_ok, poll_text) = run_any(
        &flow.inviter,
        &[
            "handshake",
            "poll",
            "--peer",
            "redeemer",
            "--relay",
            proxy.base_url(),
            "--max",
            "4",
        ],
    );
    assert!(
        proxy.faulted() > 0,
        "ANTECEDENT FAILED: the poll never reached the ack route, so this arm is not exercising \
         the poll's producer ack at all:\n{poll_text}"
    );
    assert!(
        has_marker_line(&poll_text, "producer_ack", &["caller=poll", "acked=0"]),
        "ANTECEDENT FAILED: the 500 did not surface as a failed producer ack:\n{poll_text}"
    );
    assert!(
        poll_ok,
        "the poll must still exit 0 on a lost ack:\n{poll_text}"
    );
    assert!(
        has_marker_line(&poll_text, "handshake_complete", &["role=responder"]),
        "ANTECEDENT FAILED: the A2 was never consumed, so there is no crash window to \
         measure:\n{poll_text}"
    );

    // ---- The A2 redelivers, and the poll TOLERATES the already-processed frame. ----
    thread::sleep(LEASE_EXPIRY_WAIT);
    let (retry_ok, retry_text) = run_any(
        &flow.inviter,
        &[
            "handshake",
            "poll",
            "--peer",
            "redeemer",
            "--relay",
            &base,
            "--max",
            "4",
        ],
    );
    assert!(
        retry_ok,
        "a redelivered ALREADY-PROCESSED frame must be tolerated — the poll falls through to a \
         decode-reject and returns Ok(()). If this ever fails it is a FINDING, not a test to \
         retune:\n{retry_text}"
    );
    // The session the first poll built must still be intact: redelivery must not corrupt it.
    let status = run_ok(
        &flow.inviter,
        &["handshake", "status", "--peer", "redeemer"],
    );
    assert!(
        !status.contains("none"),
        "the redelivered frame must not have disturbed the completed session:\n{status}"
    );
    // ⚠⚠ **THE MEASURED RESIDUAL, PINNED RATHER THAN NARRATED — AND IT IS A DIRECTIVE EXPECTATION
    // THAT MISSED.** §5's T5p predicted *"the retry's consume+ack lands `acked=1`"*. It does not,
    // and the reason is structural: a redelivered ALREADY-PROCESSED A2 no longer decodes into a
    // consuming branch, so it reaches `hs_emit_decode_reject; continue` — a path that by design
    // never acks, because acking there would retire a frame this run did not consume.
    //
    // ⇒ **THE CRASH COST IS NOT THE SAME FOR ALL THREE CALLERS, and only executing it shows that:**
    //   * `invite finish` — the retry RE-CONSUMES and its ack lands (T5f measures `acked=1`), so a
    //     lost ack costs exactly one lease period and leaves nothing behind;
    //   * the **poll** — the retry cannot re-consume, so the frame is **never retired by any retry**
    //     and ages out only on the relay's retention TTL. Bounded and harmless (it is skipped by
    //     class on every `receive`, per lane 1), but it is a PERMANENT orphan, not a transient one.
    //
    // This arm pins both numbers so a successor that changes either has to say so.
    thread::sleep(LEASE_EXPIRY_WAIT);
    let left = raw_pull_lease(&base, INVITER_INBOX, 16);
    assert_eq!(
        count_marker(&retry_text, "producer_ack"),
        0,
        "the retry over an already-processed frame must NOT ack — it did not consume \
         anything:\n{retry_text}"
    );
    assert_eq!(
        left.len(),
        1,
        "the orphaned A2 must still be resident: no retry can retire it, so it ages out on the \
         relay's retention TTL. Measured frames in the inviter's inbox: {}",
        left.len()
    );
}

// ===========================================================================
// T6 — THE MESSAGE SURVIVES.
//
// CLASS: **POST-REPAIR, ANTECEDENT-CONTROLLED.** The scanned-past ordinary message must be LEASED,
// never acked by finish, and delivered INTACT by the next `receive`.
//
// ⚠ The property is ALREADY GATED by `na0688_c4_collateral_arms.rs` at the OLD head-only boundary;
// this arm re-pins it at the NEW scan boundary, where finish now touches many frames instead of
// one. ⚠ `PULL_LEASE_SECS = 1`, stated beside every figure.
// ===========================================================================

#[test]
fn t6_the_scanned_past_message_is_delivered_intact() {
    let _g = guard();
    let relay =
        common::start_qsl_server_with_store(2 * 1024 * 1024, 512, None, SHORT_PULL_LEASE_SECS);
    let base = relay.base_url().to_string();
    let root = test_root("na0742_t6");
    let flow = setup_to_redeem(&root, &base, "lease");

    const BODY: &[u8] = b"na0742 t6 the message that must survive the scan";
    plant_ordinary_message(&flow, &base, &root, "t6_msg.txt", BODY);
    accept(&flow);

    let (ok, text) = finish(&flow, &base);
    assert!(ok, "ANTECEDENT FAILED: finish must succeed:\n{text}");
    // ANTECEDENT: the scan really did step OVER the message rather than never seeing it.
    assert!(
        has_marker_line(&text, "invite_scan_summary", &["selected=invite_resp"]),
        "ANTECEDENT FAILED: no scan summary, so the arm cannot claim the message was scanned \
         past:\n{text}"
    );
    assert!(
        has_marker_line(&text, "producer_ack", &["caller=finish", "sent=1"]),
        "ANTECEDENT FAILED: finish must ack EXACTLY ONE frame — the reply it consumed:\n{text}"
    );

    // ⚠ WAIT PAST THE LEASE (PULL_LEASE_SECS=1): the scanned-past message is held INVISIBLE, not
    // deleted. Probing before expiry would report a perfectly intact message as lost.
    thread::sleep(LEASE_EXPIRY_WAIT);

    let out = root.join("t6_out");
    ensure_dir_700(&out);
    let (_r_ok, r_text) = run_any(
        &flow.redeemer,
        &[
            "receive",
            "--transport",
            "relay",
            "--relay",
            &base,
            "--mailbox",
            REDEEMER_INBOX,
            "--from",
            CAROL_LABEL,
            "--max",
            "8",
            "--out",
            out.to_str().expect("out"),
            "--ack-mode",
            "lease",
        ],
    );
    let delivered: Vec<PathBuf> = fs::read_dir(&out)
        .expect("read out")
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .map(|n| n.to_string_lossy().starts_with("recv_"))
                .unwrap_or(false)
        })
        .collect();
    assert!(
        !delivered.is_empty(),
        "the scanned-past message must be DELIVERED by the next receive, not merely survive:\n{r_text}"
    );
    let got = fs::read(&delivered[0]).expect("read delivered");
    assert_eq!(
        got, BODY,
        "the scanned-past message must arrive BYTE-INTACT — survival is not enough if the bytes \
         changed"
    );
}

// ===========================================================================
// T7 — MODE DISCRIMINATION. ⚠⚠ THE ARM THAT MAKES THE LEASE-ONLY BOUND MEASURED RATHER THAN
// ASSERTED.
//
// CLASS: **POST-REPAIR, ANTECEDENT-CONTROLLED.**
//
// Three collateral frames sit ahead of a real invite reply. Under **Legacy** the relay deletes on
// pull, `invite finish` takes exactly one frame and destroys it, and the command fails exactly as
// it does today. Under **Lease** the scan steps past all three and consumes ONLY the reply.
//
// ⚠ This is what forbids the defect the ruling caught in the draft: a mode-blind scan would have
// pulled up to 128 frames under Legacy and destroyed every one of them — a 16x-128x amplification
// of a one-frame loss.
// ===========================================================================

fn plant_three_collateral(base: &str) {
    push_raw(base, REDEEMER_INBOX, &handshake_frame());
    push_raw(
        base,
        REDEEMER_INBOX,
        &invite_init_frame("na0742-collateral-route-token-xyz"),
    );
    push_raw(base, REDEEMER_INBOX, &handshake_frame());
}

#[test]
fn t7_legacy_destroys_exactly_one_and_lease_scans_past_all_three() {
    let _g = guard();

    // ---------- LEGACY: the control. It MUST destroy, or the lease arm proves nothing. ----------
    let legacy_relay =
        common::start_qsl_server_with_store(2 * 1024 * 1024, 512, None, SHORT_PULL_LEASE_SECS);
    let legacy_base = legacy_relay.base_url().to_string();
    let legacy_root = test_root("na0742_t7_legacy");
    let legacy_flow = setup_to_redeem(&legacy_root, &legacy_base, "legacy");
    plant_three_collateral(&legacy_base);
    accept(&legacy_flow);
    let (legacy_ok, legacy_text) = finish(&legacy_flow, &legacy_base);
    assert!(
        !legacy_ok,
        "CONTROL FAILED: under Legacy `invite finish` must still fail on the foreign head exactly \
         as it does today. If it now succeeds, the Legacy path was CHANGED and the byte-unchanged \
         bound is broken:\n{legacy_text}"
    );
    assert!(
        !legacy_text.contains("invite_scan_summary"),
        "CONTROL FAILED: the scan must not run at all under Legacy — a mode-blind scan on a \
         delete-on-pull server is a 16x-128x loss amplification:\n{legacy_text}"
    );
    thread::sleep(LEASE_EXPIRY_WAIT);
    let legacy_left = raw_pull_lease(&legacy_base, REDEEMER_INBOX, 32);
    assert_eq!(
        legacy_left.len(),
        3,
        "under Legacy exactly ONE of the four frames must be destroyed (delete-on-pull, --max 1), \
         leaving three. Measured survivors: {}",
        legacy_left.len()
    );
    assert_eq!(
        legacy_left.iter().filter(|f| is_invite_resp(f)).count(),
        1,
        "under Legacy the reply must still be sitting unconsumed behind the collateral"
    );

    // ---------- LEASE: the treatment. ----------
    let lease_relay =
        common::start_qsl_server_with_store(2 * 1024 * 1024, 512, None, SHORT_PULL_LEASE_SECS);
    let lease_base = lease_relay.base_url().to_string();
    let lease_root = test_root("na0742_t7_lease");
    let lease_flow = setup_to_redeem(&lease_root, &lease_base, "lease");
    plant_three_collateral(&lease_base);
    accept(&lease_flow);
    let (lease_ok, lease_text) = finish(&lease_flow, &lease_base);
    assert!(
        lease_ok,
        "under Lease the scan must reach the reply behind three collateral frames:\n{lease_text}"
    );
    assert!(
        has_marker_line(
            &lease_text,
            "invite_scan_summary",
            &["selected=invite_resp"]
        ),
        "the scan summary must report the selection:\n{lease_text}"
    );
    thread::sleep(LEASE_EXPIRY_WAIT);
    let lease_left = raw_pull_lease(&lease_base, REDEEMER_INBOX, 32);
    assert_eq!(
        lease_left.len(),
        3,
        "under Lease ALL THREE collateral frames must survive — none acked, none destroyed. \
         Measured survivors: {}",
        lease_left.len()
    );
    assert_eq!(
        lease_left.iter().filter(|f| is_invite_resp(f)).count(),
        0,
        "under Lease the reply must have been CONSUMED AND ACKED — and it must be the ONLY frame \
         that was"
    );
}

// ===========================================================================
// T7b — THE MARKERS, AS RENDERED.
//
// ⚠⚠ A CLAIM ABOUT RENDERED OUTPUT CANNOT BE CHECKED BY ANY SOURCE CENSUS. Lane 1 lost a whole
// marker diagnostic to the redactor (`len() >= 24` PLUS a digit) and only discovered it at runtime.
// So this arm reads the ACTUAL emitted line.
// ===========================================================================

#[test]
fn t7b_the_scan_marker_renders_without_redaction() {
    let _g = guard();
    let relay =
        common::start_qsl_server_with_store(2 * 1024 * 1024, 512, None, PRODUCTION_PULL_LEASE_SECS);
    let base = relay.base_url().to_string();
    let root = test_root("na0742_t7b");
    let flow = setup_to_redeem(&root, &base, "lease");
    plant_three_collateral(&base);
    accept(&flow);
    let (ok, text) = finish(&flow, &base);
    assert!(ok, "{text}");

    let scan: Vec<&str> = marker_lines(&text, "invite_scan_summary");
    assert_eq!(scan.len(), 1, "exactly one scan summary per run: {scan:?}");
    let line = scan[0];
    assert!(
        !line.contains("<redacted>"),
        "THE MARKER MUST RENDER WHOLE. `classes=` carries bare class names only — deduped, from a \
         fixed five-token vocabulary, NO DIGITS — so it is redaction-safe at any length by \
         construction rather than by staying under 24 characters. Rendered line:\n{line}"
    );
    for field in ["scanned=", "pulls=", "truncated=", "selected=", "classes="] {
        assert!(line.contains(field), "missing {field} in:\n{line}");
    }
    assert!(
        line.contains("classes=") && !line.contains("classes=<"),
        "the class list must be present and unredacted:\n{line}"
    );
    // The counts ride as their OWN fields, never inside `classes=`.
    let classes = line
        .split("classes=")
        .nth(1)
        .unwrap_or("")
        .split_whitespace()
        .next()
        .unwrap_or("");
    assert!(
        !classes.chars().any(|c| c.is_ascii_digit()),
        "`classes=` must contain NO DIGITS — a digit plus length >= 24 is exactly what the \
         redactor fires on. Measured: {classes:?}"
    );
    for name in classes.split(',') {
        assert!(
            [
                "handshake",
                "message",
                "invite_init",
                "invite_resp",
                "unknown"
            ]
            .contains(&name),
            "`classes=` must draw only from the classifier's fixed vocabulary; found {name:?}"
        );
    }
}

// ===========================================================================
// T8 — THE a2_sig-FAILURE EXIT DOES NOT ACK.
//
// ⚠⚠ The initiator branch commits the session and only THEN pushes A2. Between those two points
// sits a `return Ok(())` taken when A2 cannot be signed: **the session is stored but A2 never
// leaves.** Acking there would retire a frame whose effect never reached the peer — which is
// exactly why this lane's rule is "after the CONSUMED frame's LAST EFFECT", not "after the commit".
//
// ⚠ **THE SEAM COMPILES ONLY UNDER A NON-DEFAULT `--cfg` (D-0883): normal builds must not read the
// seam selector.** So the driving arm below is inert in the standard suite, and the `not(...)`
// companion asserts exactly that — the tree's own idiom for this seam, and the honest statement of
// what the default gate does and does not cover.
// ===========================================================================

#[cfg(qsc_rng_failure_test_seam)]
#[test]
fn t8_the_a2_sig_failure_exit_emits_no_producer_ack() {
    let _g = guard();
    let relay =
        common::start_qsl_server_with_store(2 * 1024 * 1024, 512, None, SHORT_PULL_LEASE_SECS);
    let base = relay.base_url().to_string();
    let root = test_root("na0742_t8");
    let flow = setup_to_redeem(&root, &base, "lease");
    accept(&flow);

    // Force A2 signing to fail, so the initiator branch takes its early `Ok(())` AFTER the session
    // commit and BEFORE the push.
    let out = qsc(&flow.redeemer)
        .env("QSC_RNG_FAILURE_TEST_SEAM", "QSC.SIG.A2")
        .args(["invite", "finish", "--alias", "inviter", "--relay", &base])
        .output()
        .expect("run qsc");
    let text = output_text(&out);

    // ANTECEDENT — the exit under test was actually taken.
    assert!(
        has_marker_line(&text, "handshake_reject", &["reason=sig_sign_failed"]),
        "ANTECEDENT FAILED: the a2_sig exit was not reached, so this arm proves nothing about \
         it:\n{text}"
    );
    assert!(
        !has_marker_line(&text, "handshake_complete", &[]),
        "ANTECEDENT FAILED: the handshake completed, so the early exit was not taken:\n{text}"
    );

    // THE ASSERTION.
    assert_eq!(
        count_marker(&text, "producer_ack"),
        0,
        "A FRAME WHOSE A2 NEVER LEFT MUST NOT BE ACKED. The session was committed, but the peer \
         received nothing; acking here would retire a frame whose effect never happened.\n{text}"
    );
}

/// ⚠ The default gate build must NOT read the seam selector (D-0883). This arm proves the seam is
/// genuinely absent rather than merely unused — so the `cfg` above cannot silently become a way to
/// change production behaviour.
#[cfg(not(qsc_rng_failure_test_seam))]
#[test]
fn t8_the_rng_seam_is_absent_from_the_default_build() {
    let _g = guard();
    let relay =
        common::start_qsl_server_with_store(2 * 1024 * 1024, 512, None, SHORT_PULL_LEASE_SECS);
    let base = relay.base_url().to_string();
    let root = test_root("na0742_t8_noseam");
    let flow = setup_to_redeem(&root, &base, "lease");
    accept(&flow);

    let out = qsc(&flow.redeemer)
        .env("QSC_RNG_FAILURE_TEST_SEAM", "QSC.SIG.A2")
        .args(["invite", "finish", "--alias", "inviter", "--relay", &base])
        .output()
        .expect("run qsc");
    let text = output_text(&out);
    assert!(
        out.status.success(),
        "the seam selector must have NO EFFECT in a default build:\n{text}"
    );
    assert!(
        has_marker_line(&text, "handshake_complete", &["role=initiator"]),
        "the flow must complete normally with the seam selector set — a default build does not \
         read it:\n{text}"
    );
    assert!(
        !text.contains("sig_sign_failed"),
        "a default build must not be able to force the signature failure:\n{text}"
    );
}
