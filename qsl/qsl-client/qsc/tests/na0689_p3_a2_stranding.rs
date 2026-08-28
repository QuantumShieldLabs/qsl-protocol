//! NA-0689 P3 (D623 §P3) — DOES A2 STRAND WHEN THE ACKING PATH PULLS IT COLLATERALLY?
//!
//! ⚠ WHAT THIS FILE MEASURES, AND WHY IT IS NOT THE QUESTION THE INTENT ASKED. The intent named
//! *"the inviter's puller"*. That role mapping was **measured, not adopted**, by bracketing A2's
//! producer and its consumer to their enclosing `fn`s — and then to their enclosing BRANCHES,
//! because both roles live in ONE function:
//!
//!   * A2 PRODUCER — `perform_handshake_poll_with_tokens` (`handshake/mod.rs`), branch
//!     `pending.role == "initiator"`: `hs_encode_confirm` then
//!     `transport::relay_inbox_push(relay, peer_route_token, …)`.
//!   * A2 CONSUMER — the SAME `fn`, branch `pending.role == "responder"`:
//!     `hs_decode_confirm_pending(&item.data, …)`.
//!
//! A census that bracketed only to the `fn` would have called producer and consumer the same site.
//! They are separated by a `pending.role` branch — D-1328 Ruling 6 applying to itself.
//!
//! Mapping those onto the INVITE roles, from the verbs rather than from the intent's sentence:
//! `invite_accept_at` reads the invite from the LOCAL store, so it is run by the party that minted
//! the invite — the INVITER — and it processes A1 and answers B1, making the inviter the handshake
//! RESPONDER. `invite_finish` is run by the REDEEMER (the handshake INITIATOR) and is the call that
//! PRODUCES A2, pushing it BARE to the inviter's ORDINARY inbox. **So the inviter is indeed A2's
//! consumer — but that is now derived, and the derivation is what exposes the real question.**
//!
//! ⚠ THE REAL QUESTION. A2 sits bare in a mailbox that TWO commands pull:
//!   * `handshake poll` → `transport::relay_inbox_pull`, the flag-less helper whose callers ack
//!     NOTHING (C4 SITE 2; §1a's forbidden side, deliberately not wired by NA-0689 P2);
//!   * `qsc receive` → `receive_pull_and_write`, **the acking path**, which carries all five
//!     censused discard sites.
//! If the inviter runs `receive` before `handshake poll`, `receive` pulls A2 collaterally. **What
//! the acking path does with a bare handshake frame is what decides whether A2 strands.**
//!
//! ⚠⚠ NA-0770 (D-1411): THE LEGACY CONTROL LEG IS RETIRED WITH THE MODE, AND THIS FILE'S METHOD IS
//! WEAKER FOR IT — said plainly rather than quietly. The original method, adopted wholesale from
//! `na0688_c4_collateral_arms.rs`, ran a legacy control FIRST so that a lease-only arm could not pass
//! vacuously: "the handshake completed" proves nothing about collateral pulls unless the same
//! topology is shown to STRAND. **A negative result is evidence only if the instrument could have
//! returned positive** — and the legacy leg was how this arm earned that.
//!
//! WHAT SURVIVES, AND WHY IT IS STILL WORTH RUNNING: LEG 0 is MODE-FREE and untouched. It shows the
//! scaffolding REACHES `peer_confirmed=yes` with no collateral pull, so a negative in the treatment
//! is still attributable to the collateral pull rather than to broken scaffolding — the specific
//! failure (`P1`'s green-for-the-wrong-reason) this file was built to exclude.
//!
//! ⚠ WHAT IS LOST, EXACTLY: the in-suite demonstration that NON-CONFIRMATION IS OBSERVABLE AT ALL.
//! Both remaining assertions on the handshake are POSITIVE. If a future change made
//! `inviter_polls_for_a2` incapable of ever returning `false`, LEG 0 would not notice and the
//! treatment would pass vacuously. That hole is REAL and is not filled by this lane: the only
//! honest filler is a mode-free way to strand A2, which does not exist in the tree (see loss L3 —
//! no `qsc` verb evicts a relay-mailbox head). It is recorded here so the next reader does not
//! mistake this file's green for the strength it had on 2026-08-01.
//!
//! ⚠ THE RELAY MUST BE THE REAL ONE. The test-local mock in `common` parses only `max=` and always
//! pops on pull, so it cannot express lease semantics and would make both arms vacuous.
//!
//! ⚠ WHAT A GREEN HERE DOES NOT ASSERT: nothing about two physically separate devices (the tested
//! topology is two vaults on one host), nothing about NAT or real network partitions, and no timing
//! claim of any kind.
//!
//! ## THE MEASURED RESULT (NA-0689 P3, 2026-08-01) — **A2 DOES NOT STRAND**
//!
//! D623 §P3 required both branches to be pre-stated before the run. They were, and **the second
//! branch fired**: *if it does not strand, record the negative and add nothing.* So this file adds
//! **no ENG, no witness carve, and no code change** — it is the measurement of record and nothing
//! more.
//!
//! What the three legs returned, and the mechanism behind them:
//!
//! | leg | `qsc receive` on the bare A2 | outcome |
//! |---|---|---|
//! | **positive control** (no collateral pull) | not run | `peer_confirmed=yes` — the scaffolding CAN confirm |
//! | **control** (legacy) — ⚠ NO LONGER EXECUTABLE, retired NA-0770 | `qsp_unpack code=qsp_env_decode_failed`, then `error` | relay had already deleted A2 at pull; poll sees `handshake_recv msg=none` — **STRANDED** |
//! | **treatment** (lease, shipped default) | same code, `recv_ack_mode mode=lease` | **no ack queued** → lease expires → redelivery → `handshake_recv msg=A2 ok=true` → `peer_confirmed=yes` |
//!
//! ⚠ **THE REASON IS STRUCTURAL, NOT LUCKY.** All five censused discard sites sit **downstream of a
//! successful unpack** — D2–D5 need a decrypted `InnerPayload`, and D1 (`qsp_replay_reject`) needs
//! THIS envelope's message key to have been consumed in an earlier run. A bare handshake frame is
//! not an envelope at all, so it dies at **envelope decode**, upstream of every one of them, and
//! reaches `return Err(CliError::code(code))` — which queues no ack. Under lease an unacked item is
//! redelivered; the quarantine store is never involved, and `quarantine list` reports **0**.
//!
//! ⚠ **THE ERROR CODE IS DELIBERATELY NOT ASSERTED.** `qsp_env_decode_failed` is the MECHANISM; the
//! assertions pin the PROPERTY — the handshake completes, and the acking path captured nothing. A
//! future change that routed bare frames somewhere else would still be caught by both, whereas a
//! pinned code would only pin today's spelling of today's route.

mod common;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::thread;
use std::time::{Duration, Instant};

/// A 1-second server-side pull lease, so an unacked item becomes visible again quickly.
/// The same values NA-0644 uses to prove lease redelivery and NA-0688 C4 uses to prove survival.
// ⚠⚠ NA-0770 (D-1411) WIDENED THESE 1s/2500ms -> 8s/20000ms. THE RECORDED REASON IS MARGIN
// AGAINST CONTENTION, and it is stated so a later reader does not "tidy" them back.
//
// Before this lane, arms that wanted a NEGATIVE result reached for delete-on-pull, which is
// instantaneous and needs no waiting. With the mode retired, every such arm must instead wait out a
// lease — so the suite's dependence on these two numbers went UP at the moment the mode went away.
// They are now load-bearing in shards that run twelve-wide on six cores, where a 2500ms wait
// against a 1s lease left almost no margin: an overrun does not merely slow the arm, it reports a
// perfectly intact message as lost. The pair is kept in step across every file that defines it
// (`NA_0644`, `na0688`, `na0689_capture`, `na0689_p3_a2`, `na0690`, `na0708`, `na0741`, and
// `na0742` under its own names) — if you change one, change all of them.
const TEST_PULL_LEASE_SECS: usize = 8;
const LEASE_EXPIRY_WAIT: Duration = Duration::from_millis(20_000);

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

fn run_any(cfg: &Path, args: &[&str]) -> (bool, String) {
    let out = qsc(cfg).args(args).output().expect("run qsc");
    (out.status.success(), output_text(&out))
}

/// A party: its own config dir, its own vault, its own identity, its own inbox token.
/// Adopted from `NA_0681_two_party_handshake.rs` rather than re-derived beside it.
fn party(root: &Path, name: &str, inbox: &str) -> PathBuf {
    let cfg = root.join(name);
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    run_ok(&cfg, &["identity", "rotate", "--confirm"]);
    run_ok(&cfg, &["relay", "inbox-set", "--token", inbox]);
    cfg
}

/// Drive a REAL invite handshake over the REAL relay to the exact moment A2 is in flight in the
/// INVITER's ordinary inbox and nothing has consumed it yet.
///
/// The sequence is `NA_0681_two_party_handshake.rs`'s, adopted wholesale: create → redeem →
/// accept → finish. ⚠ It stops one step short of that test: the inviter never polls, because the
/// unpolled A2 IS the state under measurement.
fn drive_until_a2_in_flight(inviter: &Path, redeemer: &Path, base: &str) {
    let code = run_ok(inviter, &["invite", "create", "--relay", base, "--ttl-secs", "3600"]);
    let code = code
        .lines()
        .find(|l| l.starts_with("QSLI-1-"))
        .expect("invite code on stdout")
        .trim()
        .to_string();

    let listing = run_ok(inviter, &["invite", "list"]);
    let invite_id = listing
        .lines()
        .find_map(|l| l.strip_prefix("invite="))
        .and_then(|l| l.split_whitespace().next())
        .expect("invite id")
        .to_string();

    run_ok(
        redeemer,
        &["invite", "redeem", "--code", &code, "--alias", "inviter"],
    );
    // The inviter collects A1 from the invite's OWN mailbox and answers B1: this is the call that
    // makes the inviter the handshake RESPONDER, holding a pending record that awaits A2.
    run_ok(
        inviter,
        &[
            "invite",
            "accept",
            "--invite-id",
            &invite_id,
            "--alias",
            "redeemer",
        ],
    );
    // The redeemer learns the inviter's real route token and PRODUCES A2, bare, into the inviter's
    // ORDINARY inbox. One-shot: nothing in the protocol re-sends it.
    let finish = run_ok(
        redeemer,
        &["invite", "finish", "--alias", "inviter", "--relay", base],
    );
    assert!(
        finish.contains("invite_finish=ok"),
        "A2 was never produced, so this arm would measure nothing: {finish}"
    );
}

/// The INVITER's ORDINARY puller — `qsc receive`, the acking path — reading the same mailbox A2
/// landed in.
///
/// NA-0770 (D-1411): the `ack_mode` parameter is gone with the mode. `receive` no longer takes an
/// `--ack-mode` selector, so this helper now exercises the ONE shipped behaviour (lease) and its
/// single remaining caller is the treatment.
fn inviter_receives_ordinary(cfg: &Path, base: &str, inbox: &str, out: &Path) -> (bool, String) {
    ensure_dir_700(out);
    run_any(
        cfg,
        &[
            "receive",
            "--transport",
            "relay",
            "--relay",
            base,
            "--mailbox",
            inbox,
            "--from",
            "redeemer",
            "--max",
            "8",
            "--out",
            out.to_str().expect("out"),
        ],
    )
}

/// Did the inviter's designated consumer complete the responder side?
///
/// ⚠ THE OBSERVABLE IS NAMED BEFORE IT IS READ: `handshake_complete … role=responder
/// peer_confirmed=yes` is emitted in the responder branch only after `qsp_session_store` succeeds
/// and the pending record is cleared. It is the completion of the responder's side, not a report
/// about the wire.
fn inviter_polls_for_a2(cfg: &Path, base: &str) -> (bool, String) {
    let (_ok, text) = run_any(
        cfg,
        &["handshake", "poll", "--peer", "redeemer", "--relay", base, "--max", "4"],
    );
    let confirmed = text.contains("event=handshake_complete")
        && text.contains("role=responder")
        && text.contains("peer_confirmed=yes");
    (confirmed, text)
}

/// How many items the quarantine store holds, read through the shipped P4 verb rather than by
/// reaching into the store's files — so the arm measures the surface a user would meet.
fn quarantine_count(cfg: &Path) -> (usize, String) {
    let (_ok, text) = run_any(cfg, &["quarantine", "list"]);
    // ⚠ Marker lines are prefixed (`QSC_MARK/1 event=…`), so the event is matched by CONTAINMENT
    // and the count is read as a token. A `strip_prefix("event=…")` here silently matches nothing.
    let n = text
        .lines()
        .find(|l| l.contains("event=quarantine_list"))
        .and_then(|line| {
            line.split_whitespace()
                .find_map(|tok| tok.strip_prefix("count="))
        })
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or_else(|| panic!("no quarantine_list count in output:\n{text}"));
    (n, text)
}

// ---------------------------------------------------------------------------
// THE ARM — control (legacy) then treatment (lease), one relay, separate parties.
//
// Separate parties per side because the invite handshake is ONE-SHOT: A2 cannot be re-produced for
// a second measurement on the same identities, so a shared pair would let the control's outcome
// decide the treatment's.
// ---------------------------------------------------------------------------
#[test]
fn a2_collateral_pull_by_the_acking_path_survives_under_lease() {
    let _g = guard();
    let started = Instant::now();
    let relay = common::start_qsl_server_with_store(2 * 1024 * 1024, 512, None, TEST_PULL_LEASE_SECS);
    let base = relay.base_url().to_string();
    let root = test_root("na0689_p3_a2");

    // ---- LEG 0 — POSITIVE CONTROL INSIDE THE SAME TEST: no collateral pull at all. ----
    //
    // ⚠ THIS LEG EXISTS BECAUSE THIS LANE ALREADY PAID FOR ITS ABSENCE. The P1 write-fail probe
    // went green for the WRONG REASON: a broken harness produced exactly the error code the test
    // asserted, and only a sibling's failure exposed it. "The inviter did not confirm" is the same
    // shape of negative — it is produced just as readily by scaffolding that could never confirm
    // under ANY conditions. So the scaffolding is shown to REACH `peer_confirmed=yes` first, and
    // only then is a failure to reach it attributable to the collateral pull.
    const BASE_INVITER_INBOX: &str = "na0689p3_base_inviter_tok_2345678";
    const BASE_REDEEMER_INBOX: &str = "na0689p3_base_redeemer_tok_9abcde";
    let baseline = root.join("baseline");
    ensure_dir_700(&baseline);
    let b_inviter = party(&baseline, "inviter", BASE_INVITER_INBOX);
    let b_redeemer = party(&baseline, "redeemer", BASE_REDEEMER_INBOX);
    drive_until_a2_in_flight(&b_inviter, &b_redeemer, &base);
    let (b_confirmed, b_poll) = inviter_polls_for_a2(&b_inviter, &base);
    assert!(
        b_confirmed,
        "POSITIVE CONTROL FAILED: with NO collateral pull the inviter still did not confirm, so \
         this scaffolding cannot reach `peer_confirmed=yes` at all and every negative below is \
         worthless — it would be measuring my own harness.\npoll:\n{b_poll}"
    );

    // ---- CONTROL (legacy) — RETIRED BY NA-0770 (D-1411). ----
    //
    // The leg ran `receive --ack-mode legacy` on the same topology and asserted `!c_confirmed`: the
    // relay deleted A2 at pull, the poll saw `handshake_recv msg=none`, and the handshake STRANDED.
    // That assertion was this file's NEGATIVE CAPABILITY — the proof that a non-confirmation is
    // observable here at all, and therefore that the treatment's `t_confirmed` is not vacuous.
    //
    // ⚠ IT IS NOT REPLACED, AND NO SUBSTITUTE IS SMUGGLED IN. Stranding A2 required deleting it at
    // pull; with delete-on-pull retired, nothing in the shipped CLI can strand it (loss L3). A
    // test-only seam to a retired mode is forbidden by this lane's brief, and faking the strand by
    // reaching into the relay's store would measure the fake, not the client. The hole is named in
    // the module header instead of being papered over.
    //
    // ⚠ LEG 0 IS A DIFFERENT CONTROL AND STILL RUNS: it is POSITIVE (the scaffolding CAN confirm),
    // mode-free, and untouched. It does not restore what this leg did.

    // ---- TREATMENT: lease (the C4 default, and the shipped behaviour). ----
    const TREAT_INVITER_INBOX: &str = "na0689p3_treat_inviter_tok_opqrstu";
    const TREAT_REDEEMER_INBOX: &str = "na0689p3_treat_redeemer_tok_vwxyz1";
    let treat = root.join("treatment");
    ensure_dir_700(&treat);
    let t_inviter = party(&treat, "inviter", TREAT_INVITER_INBOX);
    let t_redeemer = party(&treat, "redeemer", TREAT_REDEEMER_INBOX);
    drive_until_a2_in_flight(&t_inviter, &t_redeemer, &base);

    let (_t_recv_ok, t_recv) = inviter_receives_ordinary(
        &t_inviter,
        &base,
        TREAT_INVITER_INBOX,
        &treat.join("out"),
    );
    thread::sleep(LEASE_EXPIRY_WAIT);
    let (t_confirmed, t_poll) = inviter_polls_for_a2(&t_inviter, &base);
    let (t_quarantined, t_qlist) = quarantine_count(&t_inviter);

    // ⚠ THE TWO ASSERTIONS PIN DIFFERENT THINGS AND NEITHER IMPLIES THE OTHER. The first says the
    // handshake is not stranded; the second says WHY — the acking path never claimed A2, so there
    // was nothing to recover from quarantine. A pass on one alone would leave the mechanism
    // unmeasured.
    assert!(
        t_confirmed,
        "under lease a collateral `receive` must NOT consume A2: the handshake strands and the \
         inviter can never confirm. quarantine held {t_quarantined} item(s).\n\
         receive:\n{t_recv}\npoll:\n{t_poll}\nquarantine list:\n{t_qlist}"
    );
    assert_eq!(
        t_quarantined, 0,
        "A2 was CAPTURED by the acking path. That is the stranding branch: the frame reached a \
         censused discard site, was quarantined and acked, and quarantine is now the only \
         recovery vehicle for the handshake. This is a FINDING, not a flaky test.\n\
         receive:\n{t_recv}\npoll:\n{t_poll}\nquarantine list:\n{t_qlist}"
    );

    // ⚠ THE RUNTIME IS THE HONEST TELL. An all-clear on an impossible runtime is not reassurance:
    // NA-0688 saw `0 CONTROL FAILED` printed by runs that died before any control executed.
    //
    // NA-0770 (D-1411): RETUNED 2x -> 1x BECAUSE ONE WAIT REMAINS, not to make the arm easier. The
    // legacy control carried the other `LEASE_EXPIRY_WAIT`; with that leg retired, a 2x floor would
    // be an assertion this file can no longer satisfy on its own terms, and lowering it is
    // arithmetic on the waits actually executed — NOT a weakened tripwire. Two full invite
    // handshakes (LEG 0 and the treatment) plus the treatment's lease wait still cannot elapse
    // instantly, so the tell survives.
    let elapsed = started.elapsed();
    assert!(
        elapsed >= LEASE_EXPIRY_WAIT,
        "impossible runtime {elapsed:?}: the treatment's lease wait cannot have elapsed, so the \
         arms did not run as written"
    );
}
