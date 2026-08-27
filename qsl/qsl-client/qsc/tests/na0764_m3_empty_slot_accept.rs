//! NA-0764 (`D-1405`) — **M3: THE EMPTY-SLOT ACCEPT IS FREE, AND IT IS NOW LOAD-BEARING.**
//!
//! Lane C's auto-connect scan class (`R1`) calls `invite_accept` for **every Active invite on
//! every tick**. At the blessed INSTANT tempo that is ~180 calls/hour per outstanding invite,
//! and at the `ACTIVE_INVITE_SOFT_CAP` of 10 it is ~1,800/hour. Almost all find an EMPTY slot.
//! Two properties therefore stop being incidental and become load-bearing:
//!
//!   1. an empty-slot accept **mutates nothing**, and
//!   2. it **leaves no lease** — a later real redemption is still immediately visible.
//!
//! Property 2 is the one with teeth. Under the default `AckMode::Lease` a pull takes a lease
//! and the relay hides the leased item for the lease window; if an empty accept took one, the
//! tick would be arming a hazard against its own next beat.
//!
//! ⚠⚠ **THE FIXTURE CHOICE IS PART OF THE MEASUREMENT, AND THE EASY ONE IS A VACUOUS PASS.**
//! This crate has two loopback relay fixtures. `common::start_inbox_server` is a `VecDeque`
//! with **no lease semantics whatsoever** — asking it "was a lease taken?" cannot return yes,
//! so a green there would prove nothing. This file uses `common::start_qsl_server_with_store`,
//! the **REAL `qsl-server` in-process**, with the lease pinned SHORT so a lease, had one been
//! taken, would still be in force at assertion time. (Ordered by `ORDER_NA0764_P1_20260826.md`
//! sec 2(e).)
//!
//! ⚠⚠ **AND THE FRAME IS PLANTED BY A REAL REDEMPTION, BECAUSE THE RELAY REFUSES ANY OTHER
//! WAY — MEASURED, NOT ASSUMED.** This file's first revision planted the frame with a bare
//! `POST /v1/push` carrying the invite id as the route token. **The relay answered `403
//! ERR_INVITE_TICKET_INVALID`.** `qsl-server`'s own enqueue path says why in its comment —
//! *"Reachable ONLY for a route the invite system created"* — so an invite slot is
//! **capability-gated at the relay** and cannot be written by anyone without the invite's
//! ticket. ⛳ That is a security property worth stating plainly: **the tick's every-beat empty
//! accepts cannot be poisoned by a third party stuffing the slot**, because there is no
//! unauthenticated way to stuff it. The refusal was the relay being right and the test being
//! wrong; the fix is a genuine two-party redemption, which is also the case the tick will
//! actually meet.
//!
//! ⚠ CLAIM BOUNDARY: this measures the ENGINE verb against a real relay on loopback. It says
//! nothing about NAT, partitions, or two physical machines — the operator's flight is that
//! gate.

mod common;

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::{env, fs};

use qsc::facade::{self, InviteStateKind};

const ALICE_INBOX: &str = "na0764_alice_inbox_token_abcdefgh";
const BOB_INBOX: &str = "na0764_bob_inbox_token_ijklmnopq";

/// Pinned SHORT but NOT zero: long enough to still be in force when the post-redemption
/// assertion runs, short enough that the test never waits on it.
const PULL_LEASE_SECS: usize = 60;

/// More than one, because a lease taken on the FIRST empty pull would be invisible to a test
/// that only ever calls once.
const EMPTY_ACCEPTS: usize = 3;

fn guard() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
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
    let root = env::var("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("target"));
    let root = root
        .join("qsc-test-tmp")
        .join(format!("na0764_{tag}_{}", std::process::id()));
    ensure_dir_700(&root);
    root
}

fn output_text(out: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    )
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

/// ⚠ ONE call, BEFORE any server starts — `set_var` is unsound once a fixture's runtime has
/// spawned threads (the `na0751_facade_invite_surface.rs` rule, carried by `na0756`).
fn set_env_once(cfg: &Path) {
    env::set_var("QSC_CONFIG_DIR", cfg);
    env::set_var("QSC_QSP_SEED", "1");
    env::set_var("QSC_ALLOW_SEED_FALLBACK", "1");
    env::set_var("QSC_UNSAFE_TEST_SEED_FALLBACK", "1");
    env::set_var("QSC_MARK_FORMAT", "plain");
}

/// Alice, in-process, so the facade's own `require_unlocked_here` gate passes. The two acts
/// that need a CLI run first; the env is set BEFORE the in-process unlock because
/// `unlock_with_passphrase` resolves the vault through `QSC_CONFIG_DIR` at CALL time.
fn alice_in_process(root: &Path) -> PathBuf {
    let cfg = root.join("alice");
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    run_ok(&cfg, &["identity", "rotate", "--confirm"]);
    run_ok(&cfg, &["relay", "inbox-set", "--token", ALICE_INBOX]);
    set_env_once(&cfg);
    qsc::vault::unlock_with_passphrase(common::TEST_MOCK_VAULT_PASSPHRASE)
        .expect("in-process unlock of the same mock vault the subprocess used");
    qsc::set_vault_unlocked(true);
    cfg
}

/// Bob: a pure SUBPROCESS party. Every helper he needs sets `QSC_CONFIG_DIR` per call
/// (`init_passphrase_vault` spawns `qsc` with it explicitly), so building him AFTER
/// `set_env_once` cannot disturb Alice's in-process view.
fn party(root: &Path, name: &str, inbox: &str) -> PathBuf {
    let cfg = root.join(name);
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    run_ok(&cfg, &["identity", "rotate", "--confirm"]);
    run_ok(&cfg, &["relay", "inbox-set", "--token", inbox]);
    cfg
}

/// Every byte under the config dir, path-sorted. The invite store is a VAULT SECRET
/// (`invite_store_save` -> `vault::secret_set`), so **any** save re-encrypts the vault file and
/// changes these bytes. Byte-identity is therefore a proof that no save happened — a stronger
/// statement than "the row still reads Active".
fn snapshot(dir: &Path) -> Vec<(PathBuf, Vec<u8>)> {
    fn walk(dir: &Path, out: &mut Vec<(PathBuf, Vec<u8>)>) {
        let mut entries: Vec<_> = fs::read_dir(dir)
            .expect("read_dir")
            .map(|e| e.expect("dir entry").path())
            .collect();
        entries.sort();
        for p in entries {
            if p.is_dir() {
                walk(&p, out);
            } else {
                out.push((p.clone(), fs::read(&p).expect("read file")));
            }
        }
    }
    let mut out = Vec::new();
    walk(dir, &mut out);
    out
}

/// Mint IN-PROCESS through the facade, not the CLI.
///
/// ⚠ MEASURED, not assumed: the CLI's `invite create` at this pin takes only `--as`, `--relay`
/// and `--ttl-secs`. **There is no `--label`** — `recipient_label` is a FACADE parameter
/// (`invite_create`'s fourth argument, LAST and deliberately not `self_label`, per SR-15 B-2).
/// The label matters here because `R1`'s scan class passes `alias := the invite's own label`.
fn mint(base: &str) -> (String, String) {
    let code = facade::invite_create(None, base, 3600, Some("bob")).expect("facade mint succeeds");
    (code, only_row().invite_id)
}

/// The ONLY way a frame legitimately reaches an invite slot: its holder redeems it. See the
/// module header for the 403 that established this.
fn bob_redeems(bob: &Path, code: &str) {
    run_ok(
        bob,
        &["invite", "redeem", "--code", code, "--alias", "alice"],
    );
}

fn only_row() -> facade::InviteSummary {
    let rows = facade::invite_list().expect("invite_list succeeds while unlocked");
    assert_eq!(rows.len(), 1, "exactly one invite in this fixture");
    rows.into_iter().next().expect("the row")
}

// ─────────────────────────────────── M3, THE MEASUREMENT ───────────────────────────────────

/// **M3.** An empty-slot `invite_accept` mutates nothing AND takes no lease.
///
/// ⚠ MUST GO RED IF: the empty path gains a store write, or a pull on an empty mailbox starts
/// consuming lease/rate state that hides a subsequent real redemption. The second half is what
/// the tick's every-beat cadence makes load-bearing.
///
/// The lease arm is **non-vacuous by construction**: the fixture is the real `qsl-server` with
/// `PULL_LEASE_SECS` still in force at assertion time, and the redemption is proven visible
/// through the SAME production verb that did the empty pulls.
#[test]
fn m3_empty_slot_accept_takes_no_lease_and_leaves_the_store_untouched() {
    let _g = guard();
    let root = test_root("m3");
    let alice = alice_in_process(&root);
    let bob = party(&root, "bob", BOB_INBOX);
    let relay = common::start_qsl_server_with_store(2 * 1024 * 1024, 512, None, PULL_LEASE_SECS);
    let base = relay.base_url().to_string();

    let (code, invite_id) = mint(&base);
    let before_row = only_row();
    assert_eq!(
        before_row.state,
        InviteStateKind::Active,
        "the fixture must start from an ACTIVE invite, or nothing below is about the tick's case"
    );

    let before = snapshot(&alice);

    // ── HALF ONE: the empty accepts mutate nothing. ──
    for i in 0..EMPTY_ACCEPTS {
        let got = facade::invite_accept(None, &invite_id, "bob", 1)
            .unwrap_or_else(|e| panic!("empty-slot accept #{i} must not error: {e:?}"));
        assert!(
            got.is_none(),
            "empty-slot accept #{i} must return the EMPTY sentinel, got {got:?}"
        );
    }

    let after = snapshot(&alice);
    assert_eq!(
        before, after,
        "the config dir must be BYTE-IDENTICAL after {EMPTY_ACCEPTS} empty accepts — the invite \
         store is a vault secret, so any save at all would re-encrypt the vault and move these \
         bytes"
    );

    // Row-level accounting, as ordered: untouched in every field a screen reads.
    let after_row = only_row();
    assert_eq!(after_row.invite_id, before_row.invite_id, "invite_id moved");
    assert_eq!(after_row.state, InviteStateKind::Active, "state moved");
    assert_eq!(after_row.expiry, before_row.expiry, "expiry moved");
    assert_eq!(after_row.revocable, before_row.revocable, "revocable moved");
    assert_eq!(after_row.label, before_row.label, "label moved");
    assert_eq!(after_row.created, before_row.created, "created moved");

    // ── HALF TWO: no lease was taken, proven through the production verb. ──
    // If any empty pull above had taken a lease, the relay would hide Bob's frame for
    // PULL_LEASE_SECS and the accept below would report the slot EMPTY.
    bob_redeems(&bob, &code);

    let seen = facade::invite_accept(None, &invite_id, "bob", 1);
    assert!(
        !matches!(seen, Ok(None)),
        "THE LEASE PROPERTY: after {EMPTY_ACCEPTS} empty accepts a real redemption must still be \
         VISIBLE to the very next accept. Ok(None) here means the empty pulls left a lease that \
         hides a real redemption. Got: {seen:?}"
    );
    assert!(
        matches!(seen, Ok(Some(_))),
        "R1's PROMISE: the accept that finds the redemption must COMPLETE it and return the \
         peer fingerprint — this is the hands-off provisioning the tick exists to perform. \
         Got: {seen:?}"
    );

    // And the completion is durable: the row leaves the Active state exactly once.
    assert_eq!(
        only_row().state,
        InviteStateKind::Redeemed,
        "after a completed accept the invite row must read Redeemed"
    );
}

/// **THE INSTRUMENT'S OWN CAN-FAIL PROOF.** Every assertion above is an equality that could
/// pass by being constant. This proves the two load-bearing ones are not.
///
/// ⚠ MUST GO RED IF: the snapshot comparison stops discriminating, or the empty sentinel stops
/// distinguishing an empty slot from a used one. A green arm that cannot be made red is not a
/// measurement.
#[test]
fn m3_control_the_snapshot_and_the_sentinel_both_discriminate() {
    let _g = guard();
    let root = test_root("m3ctl");
    let alice = alice_in_process(&root);
    let bob = party(&root, "bob", BOB_INBOX);
    let relay = common::start_qsl_server_with_store(2 * 1024 * 1024, 512, None, PULL_LEASE_SECS);
    let base = relay.base_url().to_string();

    let (code, invite_id) = mint(&base);
    let before = snapshot(&alice);

    // (i) THE SENTINEL DISCRIMINATES — a redeemed slot must NOT report empty.
    bob_redeems(&bob, &code);
    let redeemed = facade::invite_accept(None, &invite_id, "bob", 1);
    assert!(
        !matches!(redeemed, Ok(None)),
        "a NON-EMPTY slot must not report the empty sentinel — otherwise the main test's \
         Ok(None) is a constant and proves nothing. Got: {redeemed:?}"
    );

    // (ii) THE SNAPSHOT DISCRIMINATES — that accept wrote the store, so the bytes MUST move.
    let after = snapshot(&alice);
    assert_ne!(
        before, after,
        "a completed accept writes the invite store, so the config-dir bytes MUST differ — if \
         they do not, the byte comparison in the main test cannot fail and its green is worthless"
    );
}
