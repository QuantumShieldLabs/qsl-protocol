#![allow(dead_code)]
// ⚠ T8's driving arm sits behind `qsc_rng_failure_test_seam`, a NON-DEFAULT `--cfg` (D-0883).
// The allow is the tree's own idiom for the seam tests -- see `a2_signature_provider_rng_failure.rs:1`.
#![allow(unexpected_cfgs)]

//! NA-0768 (D-1409) -- THE INVITER REPAIR: THE A1 ARM.
//!
//! ⚠ THE ARM A RULING ACCEPTS AS THE PROOF THE B1 DEFECT IS REPAIRED, AND IT IS RED-ARMED.
//!
//! The fan-out is handed every `QHSM`-prefixed frame the scan pulled. `frameclass::classify`
//! returns `Handshake` for ANY frame whose first four bytes are `QHSM` -- **that is a MATCH, not
//! an IDENTIFICATION** -- so a bare A1 from a pinned contact reaches the same offer path as the
//! inviter's own A2. An A1 does NOT commit a session; it commits a PENDING record and pushes a
//! B1. A witness that watches only for a session therefore reads FALSE for a frame that WAS
//! consumed: never acked, redelivered every lease to the 7-day TTL, the fan-out re-run per
//! redelivery. The witness observes BOTH durable commit shapes, and this arm is what proves it.
//!
//! ## THE FIVE PROPERTIES, ALL ASSERTED
//! consumed; acked EXACTLY ONCE; the pending STORED; the B1 PUSHED; not redelivered.
//! ⚠ `not_redelivered` does NOT carry itself: at the deployed lease an UNACKED frame is equally
//! invisible to an immediate re-pull, so that probe alone cannot separate "acked and deleted"
//! from "leased and due for redelivery" -- the exact pair the defect is about. The property is
//! carried by the ack count; the two together are the evidence, neither alone.

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


// ===========================================================================
// STOP 006 — THE A1 ARM (RULING_NA0768_005 sec 4; cold read 3's B1 and its N2).
// A bare A1 from a PINNED contact lands in the shared inbox concurrently with the
// inviter's finish. It must be: CONSUMED, ACKED ONCE, pending STORED, B1 PUSHED,
// and NOT REDELIVERED on the next pull.
// ===========================================================================
#[test]
fn s4_a1_arm_from_a_pinned_contact() {
    let _g = guard();
    let relay = common::start_qsl_server_with_store(2*1024*1024,512,None,PRODUCTION_PULL_LEASE_SECS);
    let base = relay.base_url().to_string();
    let root = test_root("na0768_s4");
    banner("S4 A1 ARM — a bare A1 from a PINNED contact, concurrent with the inviter's finish");

    // alpha (under test) and bravo (a pinned contact who will re-init).
    let alpha = party(&root,"alpha",ALPHA_INBOX);
    let bravo = party(&root,"bravo",BRAVO_INBOX);
    let a_fp = fingerprint(&alpha);
    let b_fp = fingerprint(&bravo);
    // ⚠ A BARE `handshake init` NEEDS THE PEER'S IDENTITY KEYS, not just a trusted device:
    // the invite flow carries them in the envelope, so na0742's `add_contact` (fp + token)
    // is enough there and NOT here. Measured the hard way -- the first run of this arm died
    // at `handshake_reject reason=peer_identity_key_missing`.
    let a_kem = identity_field(&alpha,"identity_kem_pk");
    let a_sig = identity_field(&alpha,"identity_sig_pk");
    let b_kem = identity_field(&bravo,"identity_kem_pk");
    let b_sig = identity_field(&bravo,"identity_sig_pk");
    add_contact_keyed(&alpha,"bravo",&b_fp,&b_kem,&b_sig,BRAVO_INBOX);
    add_contact_keyed(&bravo,"bravo",&a_fp,&a_kem,&a_sig,ALPHA_INBOX);

    // bravo re-inits: a BARE A1 lands in ALPHA_INBOX. No adversary — na0742's header says so.
    let (iok,it)=run_any(&bravo,&["handshake","init","--peer","bravo","--relay",&base]);
    assert!(iok,"bravo's init must push an A1:\n{it}");
    println!("S4 A1 planted by a pinned contact's own init: rc_ok={iok}");

    // the inbox before: exactly one frame
    println!("S4 pending BEFORE: {}", status_of(&alpha_m(&alpha),"bravo"));

    // alpha's finish for an UNRELATED alias — the A1 is fanned out to its pinned owner.
    let m = Mixed{ alpha: alpha.clone(), bravo: bravo.clone(), charlie: bravo.clone() };
    let (fok,ft)=finish_alias(&m,&base,"bravo");
    println!("S4 finish(bravo): rc_ok={fok}");
    for e in ["invite_scan_summary","invite_finish_hs_offer","invite_finish_hs_skip",
              "invite_finish_hs_offer_error","handshake_pending","producer_ack"] {
        for l in marker_lines(&ft,e){ println!("    {l}"); }
    }
    let consumed = has_marker_line(&ft,"invite_finish_hs_offer",&["consumed=true"]);
    let acks = count_marker(&ft,"producer_ack");
    println!("S4: consumed={consumed} producer_ack_count={acks}");
    println!("S4 status AFTER: {}", status_of(&m,"bravo"));

    // NOT REDELIVERED: a raw pull must not return the A1 again.
    let left = raw_pull_lease(&base, ALPHA_INBOX, 128);
    let heads: Vec<String> = left.iter()
        .map(|b| b.iter().take(4).map(|x| format!("{x:02x}")).collect::<Vec<_>>().join(" ")).collect();
    println!("S4 frames left in the inbox after the finish: {} head4={:?}", left.len(), heads);
    // ⚠ THE RULING NAMES FIVE PROPERTIES. `consumed`, `acked once` and `not redelivered`
    // are above; these are the other two, measured directly rather than inferred.
    // (4) THE PENDING WAS STORED: a second offer now sees a RESPONDER pending where the
    //     first saw `present=false role=none state=absent`.
    // ⚠⚠ MEASURE THE PROPERTY, NOT THE MECHANISM -- THE SECOND TIME THIS LANE HAS HAD TO
    // LEARN IT. A first version re-ran `invite_finish`, which found an EMPTY inbox, never
    // invoked the poll, and therefore never emitted `handshake_pending` at all -- reporting
    // `pending_stored=false` for a pending that exists. `handshake poll` invokes the poll
    // unconditionally and emits the record's state whether or not a frame is returned.
    // ⚠⚠ THIRD ITERATION ON THIS ONE PROBE, AND THE REASON IS WORTH RECORDING. The poll
    // EARLY-RETURNS on an empty mailbox (`items.is_empty()` -> `handshake_recv msg=none`)
    // BEFORE it loads the pending record, so it emits no `handshake_pending` at all. Both
    // earlier probes measured "did a marker appear in a run" -- the MECHANISM -- when the
    // property is "does a pending record exist". Planting one junk frame makes the poll
    // proceed far enough to REPORT the record's state.
    push_raw(&base, ALPHA_INBOX, &handshake_frame());
    let (_ok2,t2)=run_any(&m.alpha,&["handshake","poll","--peer","bravo","--relay",&base,"--max","1"]);
    let pending_stored = has_marker_line(&t2,"handshake_pending",&["present=true","role=responder"]);
    for l in marker_lines(&t2,"handshake_pending"){ println!("    2nd: {l}"); }
    // (5) THE B1 WAS PUSHED: it goes to the peer's route token, i.e. BRAVO_INBOX.
    let b1 = raw_pull_lease(&base, BRAVO_INBOX, 128);
    let b1_heads: Vec<String> = b1.iter()
        .map(|b| b.iter().take(4).map(|x| format!("{x:02x}")).collect::<Vec<_>>().join(" ")).collect();
    let b1_pushed = b1.iter().any(|f| f.len()>=4 && &f[0..4]==b"QHSM");
    println!("S4 frames in BRAVO's inbox (the B1 target): {} head4={:?}", b1.len(), b1_heads);
    println!("S4 VERDICT: consumed={consumed} acked_once={} not_redelivered={} pending_stored={pending_stored} b1_pushed={b1_pushed}",
             acks==1, left.is_empty());
    // ⚠⚠ RED-ARMED (R4-B3): `RULING_006` sec 1 accepts this arm as THE PROOF B1 IS REPAIRED,
    // and all five ruled properties were `println!` -- if every one were false the arm still
    // reported ok. `RULING_005` sec 4 names them: consumed, acked ONCE, pending stored, B1
    // pushed, NOT redelivered.
    assert!(consumed, "S4/1: the A1 must be CONSUMED by the fan-out:\n{ft}");
    assert_eq!(acks, 1, "S4/2: the consumed A1 must be acked EXACTLY ONCE:\n{ft}");
    assert!(pending_stored, "S4/3: the A1 must leave a PENDING record (witness shape 2):\n{ft}");
    assert!(b1_pushed, "S4/4: the A1 must cause a B1 push to the peer's inbox:\n{ft}");
    // ⚠ READ 4's SECOND INSTRUMENT DEFECT, NAMED RATHER THAN PAPERED OVER: at
    // PULL_LEASE_SECS=60 an unacked frame is ALSO invisible to this probe, so `left.is_empty()`
    // cannot by itself separate "acked and deleted" from "leased and due for redelivery" --
    // the exact pair B1 is about. It is asserted because it must hold, but the property is
    // CARRIED by `acks==1` above; the two together are the evidence, neither alone.
    assert!(left.is_empty(), "S4/5: no frame may remain visible to the next pull:\n{ft}");
}
// tiny shim so status_of can take a bare cfg path

