#![allow(dead_code)]
// ⚠ T8's driving arm sits behind `qsc_rng_failure_test_seam`, a NON-DEFAULT `--cfg` (D-0883).
// The allow is the tree's own idiom for the seam tests -- see `a2_signature_provider_rng_failure.rs:1`.
#![allow(unexpected_cfgs)]

//! NA-0768 (D-1409) -- THE INVITER REPAIR: THE MIXED-ROLE AND HYGIENE ARMS.
//!
//! The product's central promise is that two people who exchange an invite are CONNECTED on
//! BOTH sides. Before this lane the INVITER was not: the shipped tick pulled the shared inbox,
//! LEASED her handshake frame (A2), declined it, and handed it to nothing that could consume it
//! (`ENG-0250`, `ENG-0251`). `invite_finish` now offers the handshake-class frames its scan
//! already pulled to a bounded candidate set, within the call that pulled them, acking ONLY on
//! a witness that a durable commit occurred.
//!
//! ## EVERY ARM HERE IS RED-ARMED, AND THAT IS THE POINT
//! SR-15 cold read 4 found three arms printing nine properties and asserting NONE of them,
//! while a ruling had spent one of them as *the proof the defect is repaired*. **A green result
//! from a test with no red arm is not evidence.** Every property below is an `assert!`.
//!
//! ## WHAT EACH ARM PINS
//!  * `s1` RED-ARMED -- the inviter COMPLETES through `invite_finish`. Fails on the unrepaired tree.
//!  * `s2` RED-ARMED -- mixed role, plus the hygiene the rulings require:
//!         no `identity_mismatch`, no `identity_unknown`, no `handshake_reject(peer_mismatch)`,
//!         no `pinned_fp=`/`seen_fp=` from a SPECULATIVE offer; every `identity_ok` attributed
//!         to the RIGHT peer; and `caller_id` derived from the caller's OWN pinned fingerprint.
//!  * `s3` RED-ARMED -- the REDEEMER's shipped path is untouched.
//!  * `s5` RED-ARMED, BOTH HALVES -- a mismatch reached by ITERATION is silent; a mismatch on an
//!         ASSERTED peer still emits the security marker, exactly as before.
//!
//! ## ⚠ THE FINGERPRINT PROBE IS TOKEN-SCOPED ON PURPOSE
//! `pinned_fp=` and `seen_fp=` both END WITH `fp=`. An earlier probe searched for the MISMATCH
//! path's two keys and never for ` fp=`, the key the MATCH path uses -- and reported CLEAN on a
//! capture carrying two full 64-hex fingerprints. An instrument that cannot return the defect
//! is not a measurement.
//!
//! ## ⚠ THE RELAY IS THE REAL ONE
//! Every arm drives the REAL in-process `qsl-server` at the DEPLOYED lease. The test-local mock
//! always pops on pull and cannot express a lease, which would make every residue and ack
//! assertion here VACUOUS.

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
/// `na0688_c4_collateral_arms.rs`'s `TEST_PULL_LEASE_SECS`. ⚠ That file's value is DELIBERATELY
/// LARGER than this one's (45s vs 8s): it is the only file that does work INSIDE the lease window
/// rather than merely waiting one out. Do not "harmonise" them.
///
/// ⚠ NA-0770 (D-1411) re-pointed this citation from a LINE NUMBER to the SYMBOL. The line had
/// already moved once (:40 -> :68) when that file's constants were widened, so the cite named the
/// wrong bytes; a symbol survives edits above it.
const PRODUCTION_PULL_LEASE_SECS: usize = 60;

/// An 8-second server-side pull lease, so an unacked frame becomes visible again inside a test.
/// ⚠ **STATED BESIDE EVERY FIGURE THE ARMS USING IT PRODUCE.**
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
const SHORT_PULL_LEASE_SECS: usize = 8;
const LEASE_EXPIRY_WAIT: Duration = Duration::from_millis(20_000);

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
/// ⚠ TOKEN-SCOPED, not `contains`: `pinned_fp=` and `seen_fp=` both END WITH `fp=`, so a
/// substring search for the MATCH path's key silently matches the MISMATCH path's keys. This
/// splits on whitespace and strips the WHOLE key, which is the discrimination R4-B2 needed.
fn marker_field(line: &str, key: &str) -> String {
    let pat = format!("{key}=");
    line.split_whitespace()
        .find_map(|t| t.strip_prefix(pat.as_str()))
        .unwrap_or("")
        .to_string()
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

// NA-0770 (D-1411): `set_ack_mode` removed. It existed because `invite finish`, `invite accept`
// and `handshake poll` take no `--ack-mode` flag, so the per-install config key was the ONLY way to
// aim them — which made the key part of the instrument rather than a convenience. There is nothing
// left to aim: one mode ships, and `config set ack-mode` now REFUSES the key by name, so this
// helper could only have failed at runtime.

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

const INVITER_INBOX: &str = "na0768-bravo-inbox-token-aaaaaaaa";
const REDEEMER_INBOX: &str = "na0768-alpha-inbox-token-bbbbbbbb";
const CAROL_INBOX: &str = "na0768-charlie-inbox-tok-ccccccccc";
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
fn setup_to_redeem(root: &Path, base: &str) -> Flow {
    let inviter = party(root, "inviter", INVITER_INBOX);
    let redeemer = party(root, "redeemer", REDEEMER_INBOX);
    let carol = party(root, "carol", CAROL_INBOX);
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
// NA-0768 MENU MEASUREMENT — THE MIXED-ROLE CASE (kickoff sec 2 axis (g); P3).
//
// ONE ACCOUNT, ONE SELF INBOX, TWO PENDING CONTACTS IN DIFFERENT ROLES:
//   alpha is the REDEEMER of bravo's invite   -> an InviteResp (wrapped B1) lands in ALPHA_INBOX
//   alpha is the INVITER of charlie           -> a real A2 (Handshake)    lands in ALPHA_INBOX
// Both frames are REAL, produced by the product's own encoders over a REAL relay
// (`common::start_qsl_server_with_store`, PULL_LEASE_SECS=60) — never a delete-on-pull mock (PR-6).
//
// THE QUESTION, from the kickoff: "Do the two fetchers lease each other's frames?"
// ===========================================================================

const ALPHA_INBOX: &str = "na0768-alpha-inbox-token-bbbbbbbb";
const BRAVO_INBOX: &str = "na0768-bravo-inbox-token-aaaaaaaa";
const CHARLIE_INBOX: &str = "na0768-charlie-inbox-tok-ccccccccc";

struct Mixed {
    alpha: PathBuf,
    #[allow(dead_code)]
    bravo: PathBuf,
    #[allow(dead_code)]
    charlie: PathBuf,
}

fn invite_code(text: &str) -> String {
    text.lines()
        .find(|l| l.starts_with("QSLI-1-"))
        .expect("invite code on stdout")
        .trim()
        .to_string()
}

fn newest_invite_id(cfg: &Path) -> String {
    let listing = run_ok(cfg, &["invite", "list"]);
    listing
        .lines()
        .filter_map(|l| l.strip_prefix("invite="))
        .filter_map(|l| l.split_whitespace().next())
        .last()
        .expect("invite id")
        .to_string()
}

/// Build the mixed-role inbox. `a2_first` controls which frame sits at the HEAD.
fn build_mixed(root: &Path, base: &str, a2_first: bool, want_bravo: bool) -> Mixed {
    let alpha = party(root, "alpha", ALPHA_INBOX);
    let bravo = party(root, "bravo", BRAVO_INBOX);
    let charlie = party(root, "charlie", CHARLIE_INBOX);

    // ---- LEG 2 (alpha as INVITER of charlie): ends with a REAL A2 in ALPHA_INBOX ----
    let mk_a2 = |alpha: &Path, charlie: &Path| {
        let code = invite_code(&run_ok(
            alpha,
            &["invite", "create", "--relay", base, "--ttl-secs", "3600"],
        ));
        let aid = newest_invite_id(alpha);
        run_ok(charlie, &["invite", "redeem", "--code", &code, "--alias", "alpha"]);
        run_ok(alpha, &["invite", "accept", "--invite-id", &aid, "--alias", "charlie"]);
        // charlie consumes B1 and pushes A2 into ALPHA_INBOX
        let (ok, t) = run_any(charlie, &["invite", "finish", "--alias", "alpha", "--relay", base]);
        assert!(ok, "charlie's finish must succeed or there is no A2 to measure:\n{t}");
    };

    // ---- LEG 1 (alpha as REDEEMER of bravo): ends with an InviteResp in ALPHA_INBOX ----
    let mk_resp = |alpha: &Path, bravo: &Path| {
        let code = invite_code(&run_ok(
            bravo,
            &["invite", "create", "--relay", base, "--ttl-secs", "3600"],
        ));
        let bid = newest_invite_id(bravo);
        run_ok(alpha, &["invite", "redeem", "--code", &code, "--alias", "bravo"]);
        run_ok(bravo, &["invite", "accept", "--invite-id", &bid, "--alias", "alpha"]);
    };

    if a2_first {
        mk_a2(&alpha, &charlie);
        if want_bravo { mk_resp(&alpha, &bravo); }
    } else {
        if want_bravo { mk_resp(&alpha, &bravo); }
        mk_a2(&alpha, &charlie);
    }

    Mixed { alpha, bravo, charlie }
}

fn poll_charlie(m: &Mixed, base: &str) -> (bool, String) {
    run_any(
        &m.alpha,
        &["handshake", "poll", "--peer", "charlie", "--relay", base, "--max", "4"],
    )
}

fn finish_bravo(m: &Mixed, base: &str) -> (bool, String) {
    run_any(&m.alpha, &["invite", "finish", "--alias", "bravo", "--relay", base])
}

fn banner(tag: &str) { println!("\n===== NA-0768 MENU MEASUREMENT :: {tag} ====="); }

fn report_poll(tag: &str, ok: bool, text: &str) -> bool {
    let complete = has_marker_line(text, "handshake_complete", &["role=responder"]);
    let sawnone = has_marker_line(text, "handshake_recv", &["msg=none"]);
    println!(
        "{tag}: rc_ok={ok} handshake_complete(role=responder)={complete} handshake_recv(msg=none)={sawnone}"
    );
    for l in marker_lines(text, "handshake_recv") { println!("    {l}"); }
    for l in marker_lines(text, "handshake_complete") { println!("    {l}"); }
    for l in marker_lines(text, "invite_scan_summary") { println!("    {l}"); }
    complete
}

// --- M1: CONTROL. Only the A2 is present. The poll MUST complete, or every other arm is void. ---

// ===========================================================================
// NA-0768 STOP 005 — SPECIFICATION ARMS FOR E4 (F1 shape (ii) prototype).
// ===========================================================================

fn finish_alias(m:&Mixed, base:&str, alias:&str)->(bool,String){
    run_any(&m.alpha,&["invite","finish","--alias",alias,"--relay",base])
}
fn status_of(m:&Mixed, alias:&str)->String{
    let (_ok,t)=run_any(&m.alpha,&["handshake","status","--peer",alias]);
    t.lines().filter(|l|l.contains("status=")).map(|s|s.to_string()).collect::<Vec<_>>().join(" | ")
}

// S1 — THE RED ARM. Pure inviter: only the A2 is present, contact charlie is inviter-role.

fn identity_field(cfg:&Path, key:&str)->String{
    run_ok(cfg,&["identity","show"]).lines()
        .find_map(|l| l.strip_prefix(&format!("{key}=")).map(|v| v.trim().to_string()))
        .unwrap_or_else(|| panic!("missing {key} in identity show"))
}
fn add_contact_keyed(cfg:&Path,label:&str,fp:&str,kem:&str,sig:&str,token:&str){
    run_ok(cfg,&["contacts","add","--label",label,"--fp",fp,"--kem-pk",kem,"--sig-pk",sig,
                 "--route-token",token]);
    let list=run_ok(cfg,&["contacts","device","list","--label",label]);
    let device=list.lines().find_map(|l| l.split_whitespace()
        .find_map(|t| t.strip_prefix("device="))).unwrap_or_else(|| panic!("no device: {list}"));
    run_ok(cfg,&["contacts","device","trust","--label",label,"--device",device,"--confirm"]);
}
fn alpha_m(p:&Path)->Mixed{ Mixed{alpha:p.to_path_buf(), bravo:p.to_path_buf(), charlie:p.to_path_buf()} }


#[test]
fn s1_red_arm_pure_inviter_completes_through_invite_finish() {
    let _g = guard();
    let relay = common::start_qsl_server_with_store(2*1024*1024,512,None,PRODUCTION_PULL_LEASE_SECS);
    let base = relay.base_url().to_string();
    let root = test_root("na0768_s1");
    banner("S1 RED ARM — pure inviter; invite_finish(charlie) must COMPLETE her");
    let m = build_mixed(&root,&base,true,false);
    println!("S1 status BEFORE: {}", status_of(&m,"charlie"));
    let (ok,t)=finish_alias(&m,&base,"charlie");
    let complete = has_marker_line(&t,"handshake_complete",&["role=responder"]);
    for l in marker_lines(&t,"invite_scan_summary"){println!("    {l}");}
    for l in marker_lines(&t,"invite_finish_hs_offer"){println!("    {l}");}
    for l in marker_lines(&t,"handshake_complete"){println!("    {l}");}
    for l in marker_lines(&t,"producer_ack"){println!("    {l}");}
    println!("S1: rc_ok={ok} handshake_complete(responder)={complete}");
    println!("S1 status AFTER : {}", status_of(&m,"charlie"));
    assert!(complete,"S1 RED ARM: the inviter must complete through invite_finish after E4:\n{t}");
}

// S2 — MIXED ROLE. finishScanClass loops contacts; bravo's call runs FIRST and leases the batch.

#[test]
fn s2_mixed_role_finish_bravo_first_then_charlie() {
    let _g = guard();
    let relay = common::start_qsl_server_with_store(2*1024*1024,512,None,PRODUCTION_PULL_LEASE_SECS);
    let base = relay.base_url().to_string();
    let root = test_root("na0768_s2");
    banner("S2 MIXED ROLE — invite_finish(bravo) FIRST (leases the batch), then invite_finish(charlie)");
    let m = build_mixed(&root,&base,true,true);
    let (bok,bt)=finish_alias(&m,&base,"bravo");
    println!("S2 finish(bravo): rc_ok={bok} invite_finish=ok present={}",bt.contains("invite_finish=ok"));
    for l in marker_lines(&bt,"invite_scan_summary"){println!("    {l}");}
    for l in marker_lines(&bt,"invite_finish_hs_offer"){println!("    {l}");}
    let (cok,ct)=finish_alias(&m,&base,"charlie");
    let ccomplete = has_marker_line(&ct,"handshake_complete",&["role=responder"]);
    println!("S2 finish(charlie) AFTER: rc_ok={cok} complete={ccomplete}");
    for l in marker_lines(&ct,"invite_scan_summary"){println!("    {l}");}
    for l in marker_lines(&ct,"invite_finish_hs_offer"){println!("    {l}");}
    // ⚠ THE VERDICT MUST TEST THE PROPERTY, NOT THE MECHANISM. A first version of this arm
    // read `ccomplete` -- whether CHARLIE'S OWN CALL emitted handshake_complete -- and
    // reported `false` while charlie was in fact CONNECTED, because the fan-out consumed her
    // A2 during BRAVO's call. The property is "is the inviter connected at the end", and the
    // ground truth is her status, by EQUALITY on the extracted value.
    let st = status_of(&m,"charlie");
    println!("S2 charlie status: {st}");
    let connected = st.contains("status=established_recv_only") || st.contains("status=established");
    // ⚠ RULING_006 ASK 2: the SPECULATIVE offer must emit NO security marker and NO foreign
    // fingerprint. bravo's call fans out to charlie -- a candidate bravo's command has no
    // business naming -- so any peer_mismatch / identity_unknown / fingerprint in bravo's
    // output is the defect.
    // ⚠⚠ **D1 (`RULING_007` sec 2): CLAUSE (b) DOES NOT COVER `identity_ok{peer, fp}` ON THE
    // ACCEPTANCE PATH.** That fingerprint is the matched candidate's OWN pinned identity,
    // attributed to the right peer, in the vocabulary the tree prints on every honest handshake
    // (`should_redact_value` exempts `fp` BY NAME). What (b) forbids is a FOREIGN fingerprint:
    // a sender's identity under a NON-OWNING candidate, or X's pinned value surfacing as a
    // MISMATCH inside Y's command. The three paths, precisely:
    //    Ok(Some)+!match, speculative -> hs_offer_not_addressee{peer}  SILENT, no fingerprint
    //    Ok(None),        speculative -> hs_offer_not_addressee{peer}  SILENT, no fingerprint
    //    Ok(Some)+match                -> identity_ok{peer, fp}        PRINTS -- D1 ALLOWS IT
    // ⚠⚠ **READ 4's R4-B2 IS WHY BOTH ARE MEASURED NOW.** The original probe looked ONLY for
    // `pinned_fp=`/`seen_fp=` -- the two keys the MISMATCH path uses -- and never for ` fp=`,
    // the key the MATCH path uses and the key the redaction rule exempts BY NAME. It reported
    // CLEAN on a capture carrying two full 64-hex fingerprints of a foreign contact. An
    // instrument that cannot return the defect is not a measurement.
    // ⚠ ` fp=` IS SPACE-PREFIXED ON PURPOSE: `pinned_fp=` and `seen_fp=` both CONTAIN `fp=`.
    let sec_mismatch = count_marker(&bt,"identity_mismatch");
    let sec_unknown  = count_marker(&bt,"identity_unknown");
    let rej_mismatch = has_marker_line(&bt,"handshake_reject",&["reason=peer_mismatch"]);
    let not_addressee= count_marker(&bt,"hs_offer_not_addressee");
    let foreign_fp   = bt.contains("pinned_fp=") || bt.contains("seen_fp=");
    let identity_ok_lines: Vec<&str> = marker_lines(&bt,"identity_ok");
    let acceptance_fp = identity_ok_lines.iter().filter(|l| l.contains(" fp=")).count();
    // D1's ATTRIBUTION property, made testable: every acceptance fingerprint names the peer
    // whose handshake this call actually completed. A fingerprint under the WRONG peer would
    // be clause (b) in terms, and this is the arm that would catch it.
    // ⚠⚠⚠ **THIS PREDICATE TOOK THREE CUTS AND THE ARM REFUTED ME TWICE. BOTH FAILURES WERE
    // MINE, NOT THE PRODUCT'S -- recorded because a green arm that never contradicted its
    // author is the thing this lane keeps paying for.**
    //  CUT 1 asserted `all(peer=charlie)`. Measured: bravo's OWN call emits THREE identity_ok
    //        lines -- two for charlie (the fan-out's acceptance) and one for BRAVO HERSELF from
    //        the legitimate RESP path at invite/mod.rs:1637, which is an ASSERTED `Provided`
    //        offer and nothing to do with the fan-out.
    //  CUT 2 compared each fp against that party's `identity show`. Measured: `identity show`
    //        prints PUBLIC KEYS (`identity_kem_pk=`, `identity_sig_pk=`), never a fingerprint,
    //        so the comparison could not match anything.
    //  ⇒ AND THE SHAPE WAS WRONG BOTH TIMES. A POSITIVE test ("every fp must be in this peer's
    //    set") needs a COMPLETE map of every fingerprint each peer has -- and each peer has at
    //    least two, the KEM and the signing one, of which any single command exposes one.
    //    **Clause (b) is a NEGATIVE: no fingerprint of contact X inside a command run for
    //    contact Y.** The negative form is sound against an incomplete map -- an UNKNOWN
    //    fingerprint is not evidence of misattribution -- and still fires on exactly the
    //    forbidden shape. That is what D1 preserves of clause (b).
    // The authority is ALPHA'S OWN PIN, because alpha is the process emitting the marker.
    let pin_of = |alias: &str| marker_field(&status_of(&m, alias), "peer_fp");
    let pin_bravo   = pin_of("bravo");
    let pin_charlie = pin_of("charlie");
    let misattributed: Vec<String> = identity_ok_lines.iter().filter_map(|l| {
        let peer = marker_field(l, "peer");
        let fp   = marker_field(l, "fp");
        if fp.is_empty() { return None; }
        let foreign = (peer != "bravo"   && !pin_bravo.is_empty()   && fp == pin_bravo)
                   || (peer != "charlie" && !pin_charlie.is_empty() && fp == pin_charlie);
        if foreign { Some((*l).to_string()) } else { None }
    }).collect();
    let attribution_ok = misattributed.is_empty();
    // ⚠ D2 (`RULING_007` sec 2), asserted in BOTH directions: `caller_id` is the first 8 hex of
    // the CALLER'S OWN pinned fingerprint, and is ABSENT -- not substituted -- with no pin.
    let offer_line = marker_lines(&bt,"invite_finish_hs_offer").first().map(|s| s.to_string()).unwrap_or_default();
    let caller_id_field = marker_field(&offer_line, "caller_id");
    let d2_ok = if caller_id_field.is_empty() {
        pin_bravo.is_empty()
    } else {
        caller_id_field.len() == 8 && pin_bravo.starts_with(caller_id_field.as_str())
    };
    println!("S2 D2 caller_id={caller_id_field} pin(bravo)={pin_bravo} derives_from_pin={d2_ok}");
    for l in &identity_ok_lines { println!("    {l}"); }
    for l in marker_lines(&bt,"hs_offer_not_addressee"){ println!("    {l}"); }
    println!("S2 SPECULATIVE-OFFER HYGIENE: identity_mismatch={sec_mismatch} identity_unknown={sec_unknown} \
reject(peer_mismatch)={rej_mismatch} hs_offer_not_addressee={not_addressee} any_fingerprint_field={foreign_fp}");
    println!("S2 marker-emitted-in-charlies-own-call={ccomplete} (mechanism, not the property)");
    println!("S2 VERDICT: inviter_CONNECTED_in_mixed_role={connected}");
    println!("S2 D1 ACCEPTANCE PATH: identity_ok lines={} of which carry ` fp=`={acceptance_fp} attribution_ok={attribution_ok}", identity_ok_lines.len());
    // ⚠⚠ RED-ARMED (R4-B3): this arm had ZERO assertions while sec 9.3 called it "THE ASK-2
    // HYGIENE ASSERTION". All five printed properties are assertions now, plus D1's attribution.
    assert!(connected, "S2/1: the inviter must be CONNECTED at the end of the mixed-role beat; status:\n{st}\n{bt}");
    assert_eq!(sec_mismatch, 0, "S2/2: a speculative offer must emit NO identity_mismatch:\n{bt}");
    assert_eq!(sec_unknown, 0, "S2/3: a speculative offer must emit NO identity_unknown:\n{bt}");
    assert!(!rej_mismatch, "S2/4: a speculative offer must emit NO handshake_reject(peer_mismatch):\n{bt}");
    assert!(!foreign_fp, "S2/5: a speculative REJECTION must print no pinned_fp=/seen_fp=:\n{bt}");
    assert!(attribution_ok, "S2/6 (D1): no identity_ok may pair a peer with ANOTHER contact's fingerprint; misattributed={misattributed:?}\n{bt}");
    assert!(d2_ok, "S2/7 (D2): caller_id must be the first 8 hex of the CALLER'S OWN pin, or absent when unpinned; caller_id={caller_id_field} pin={pin_bravo}\n{bt}");
}

// S3 — THE REDEEMER'S PATH, PROVEN UNTOUCHED BY RUN (ruling sec 4).

#[test]
fn s3_redeemer_path_untouched() {
    let _g = guard();
    let relay = common::start_qsl_server_with_store(2*1024*1024,512,None,PRODUCTION_PULL_LEASE_SECS);
    let base = relay.base_url().to_string();
    let root = test_root("na0768_s3");
    banner("S3 — the REDEEMER's shipped path must be untouched: invite_finish(bravo) completes");
    let m = build_mixed(&root,&base,false,true);
    let (ok,t)=finish_alias(&m,&base,"bravo");
    let done = t.contains("invite_finish=ok");
    for l in marker_lines(&t,"invite_scan_summary"){println!("    {l}");}
    for l in marker_lines(&t,"producer_ack"){println!("    {l}");}
    println!("S3: rc_ok={ok} invite_finish=ok={done}");
    println!("S3 bravo status: {}", status_of(&m,"bravo"));
    assert!(ok && done,"S3: E4 must not regress the redeemer's working path:\n{t}");
}


// ===========================================================================
// S5 — THE OWNING-CANDIDATE MISMATCH MUST STILL EMIT THE SECURITY MARKER.
// RULING_006 sec 2: "The owning candidate's real mismatch (a pinned contact whose A1
// carries a different identity) still emits the security marker exactly as today."
// alpha pins the contact `bravo` to CHARLIE's fingerprint; bravo then sends a real A1
// carrying BRAVO's identity. `invite finish --alias bravo` is the CALLER'S OWN alias, so
// the offer is NOT speculative and the mismatch is a genuine security event.
// ===========================================================================
#[test]
fn s5_owning_candidate_mismatch_still_emits() {
    let _g = guard();
    let relay = common::start_qsl_server_with_store(2*1024*1024,512,None,PRODUCTION_PULL_LEASE_SECS);
    let base = relay.base_url().to_string();
    let root = test_root("na0768_s5");
    banner("S5 — a mismatch reached by ITERATION is silent; a mismatch on an ASSERTED peer still emits");
    let alpha = party(&root,"alpha",ALPHA_INBOX);
    let bravo = party(&root,"bravo",BRAVO_INBOX);
    let charlie = party(&root,"charlie",CHARLIE_INBOX);
    let a_fp=fingerprint(&alpha); let b_fp=fingerprint(&bravo); let c_fp=fingerprint(&charlie);
    let a_kem=identity_field(&alpha,"identity_kem_pk"); let a_sig=identity_field(&alpha,"identity_sig_pk");
    let b_kem=identity_field(&bravo,"identity_kem_pk"); let b_sig=identity_field(&bravo,"identity_sig_pk");
    let c_kem=identity_field(&charlie,"identity_kem_pk"); let c_sig=identity_field(&charlie,"identity_sig_pk");
    let _ = (b_fp, b_kem, b_sig);
    // ⚠ THE MISPIN: alpha's contact `bravo` is pinned to CHARLIE's identity.
    add_contact_keyed(&alpha,"bravo",&c_fp,&c_kem,&c_sig,BRAVO_INBOX);
    add_contact_keyed(&bravo,"bravo",&a_fp,&a_kem,&a_sig,ALPHA_INBOX);
    let (iok,it)=run_any(&bravo,&["handshake","init","--peer","bravo","--relay",&base]);
    assert!(iok,"bravo's init must push an A1:\n{it}");
    let m = Mixed{ alpha: alpha.clone(), bravo: bravo.clone(), charlie: charlie.clone() };
    let (_fok,ft)=finish_alias(&m,&base,"bravo");
    for e in ["invite_finish_hs_offer","identity_mismatch","handshake_reject","hs_offer_not_addressee"] {
        for l in marker_lines(&ft,e){ println!("    {l}"); }
    }
    // (a) THE FAN-OUT OFFER MUST BE SILENT -- it reached this contact by ITERATION.
    let fanout_sec = count_marker(&ft,"identity_mismatch")>0
                  || has_marker_line(&ft,"handshake_reject",&["reason=peer_mismatch"]);
    let fanout_fp  = ft.contains("pinned_fp=") || ft.contains("seen_fp=");
    let not_addr   = count_marker(&ft,"hs_offer_not_addressee");
    println!("S5a FAN-OUT (iteration): security_marker={fanout_sec} fingerprint_field={fanout_fp} hs_offer_not_addressee={not_addr}");
    // (b) AN ASSERTED PEER MUST STILL EMIT IT -- `handshake poll --peer bravo` is a caller
    //     naming the peer, which is the shape a real mismatch has.
    // ⚠ A JUNK `QHSM` FRAME CANNOT MEASURE THIS: `hs_decode_init` rejects it on FRAME TYPE
    // (`handshake_type`) BEFORE the identity gate at :2272, so the probe never reaches the
    // code under test. A first version did exactly that and reported `false` for an emission
    // it never gave the poll a chance to make. A REAL, FRESH A1 is required -- the previous
    // one is still leased by the finish's own pull.
    let (_i2,_t2)=run_any(&bravo,&["handshake","init","--peer","bravo","--relay",&base]);
    let (_pk,pt)=run_any(&alpha,&["handshake","poll","--peer","bravo","--relay",&base,"--max","4"]);
    let asserted_sec = count_marker(&pt,"identity_mismatch")>0
                    || has_marker_line(&pt,"handshake_reject",&["reason=peer_mismatch"]);
    for l in marker_lines(&pt,"identity_mismatch"){ println!("    asserted: {l}"); }
    for l in marker_lines(&pt,"handshake_reject"){ println!("    asserted: {l}"); }
    println!("S5b ASSERTED (handshake poll --peer): security_marker={asserted_sec}");
    println!("S5 VERDICT: fanout_silent={} asserted_still_emits={asserted_sec}", !fanout_sec && !fanout_fp);
    assert!(!fanout_sec && !fanout_fp,
        "S5a: a fan-out offer reached by ITERATION must emit no security marker and no fingerprint:\n{ft}");
    // ⚠⚠ RED-ARMED (R4-B3): half (b) -- the clause `RULING_006` sec 2 PRESERVES -- was printed
    // and never asserted. A regression that silenced the ASSERTED poll as well as the fan-out
    // would have left this arm green while printing `asserted_still_emits=false` into a section
    // a Director reads as a pass.
    // ⚠ AND ITS OWN PRECONDITION IS ASSERTED TOO. If the second `init` pushed no frame,
    // `asserted_sec` would read false for want of a STIMULUS and nothing would say so -- which
    // is exactly the S5b probe failure this arm already paid for once.
    assert!(_i2, "S5b/precondition: the second init must push a REAL A1 for the asserted poll to judge:\n{_t2}");
    assert!(asserted_sec,
        "S5b: an ASSERTED peer's real mismatch MUST still emit the security marker (RULING_006 sec 2, the preserved clause):\n{pt}");
}
