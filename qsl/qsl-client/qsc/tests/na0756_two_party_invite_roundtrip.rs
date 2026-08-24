//! NA-0756 (D-1398, desktop D-0037; ruled at `R387` §S7) — THE INVITE ROUND TRIP AT THE
//! FACADE, AND THE FIRST FIRING OF THE TWO SECURITY TELLS.
//!
//! ⚠⚠ WHAT IS NEW HERE, STATED PRECISELY, BECAUSE THE BRIEF'S CLAIM WAS WRONG.
//! The brief called this "the first time anyone has ever exercised invite_redeem /
//! invite_accept / invite_finish". That measured FALSE at STOP 002:
//! `NA_0681_two_party_handshake.rs` has driven create -> redeem -> accept -> finish GREEN
//! against this same in-process relay since NA-0681, over the CLI. `R387` §S7 corrected the
//! record and RE-AIMED item 10 rather than striking it. Genuinely first-ever, and measured
//! `rc 1` before being claimed:
//!   (1) the FACADE layer — `facade::invite_redeem` / `invite_finish` had ZERO call sites in
//!       any test; this is the layer the desktop actually calls, so it is the layer whose
//!       contract the GUI depends on;
//!   (2) the two SECURITY TELLS actually FIRING — `commitment_mismatch` and
//!       `signature_invalid` appear nowhere in the suite. NA-0755 wrote their copy and marked
//!       it "UNREACHABLE FROM LANE A — PREPARED FOR LANE B", explicitly declining to seal
//!       them because "a seal that cannot fail is not a seal". They can fail now;
//!   (3) the NOT-YET outcome — `invite_finish=none` / `not_yet` appear nowhere either.
//!
//! ⚠ THE TOPOLOGY, and its limit. Bob (the REDEEMER, Lane B's own side) runs IN-PROCESS
//! through `qsc::facade`. Alice (the inviter) runs as a `qsc` SUBPROCESS with her own config
//! dir. That split is FORCED, not chosen: `QSC_CONFIG_DIR` is process-global, and
//! `env::set_var` is unsound once the relay's runtime threads exist — so one process cannot
//! be two parties. It is also the tree's own idiom
//! (`na0751_facade_invite_surface.rs`: "The subprocess writes the session into the config
//! dir; the facade then reads it IN-PROCESS"). `invite_accept` is therefore driven over the
//! CLI here, as Alice's act; the desktop's own `invite_accept` wrapper is Lane C's approval
//! gate and is not this lane's to seal.
//!
//! ⚠ TWO VAULTS ON ONE HOST is the ruled acceptance topology, and it is NOT two machines. A
//! green here says nothing about NAT, real partitions, or two physical devices.

mod common;

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::{env, fs};

use qsc::facade::{self, FacadeError};

const ALICE_INBOX: &str = "na0756_alice_inbox_token_abcdefgh";
const BOB_INBOX: &str = "na0756_bob_inbox_token_ijklmnopq";

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
    let root = root.join("qsc-test-tmp").join(tag);
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

/// ⚠ ONE call, BEFORE any server starts. Everything downstream inherits it, and setting it
/// after the relay's threads exist is the unsoundness this file's header names.
fn set_env_once(cfg: &Path) {
    env::set_var("QSC_CONFIG_DIR", cfg);
    env::set_var("QSC_QSP_SEED", "1");
    env::set_var("QSC_ALLOW_SEED_FALLBACK", "1");
    env::set_var("QSC_UNSAFE_TEST_SEED_FALLBACK", "1");
    env::set_var("QSC_MARK_FORMAT", "plain");
}

/// A subprocess party: its own config dir, vault, identity and inbox token.
fn party(root: &Path, name: &str, inbox: &str) -> PathBuf {
    let cfg = root.join(name);
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    run_ok(&cfg, &["identity", "rotate", "--confirm"]);
    run_ok(&cfg, &["relay", "inbox-set", "--token", inbox]);
    cfg
}

/// Bob, the in-process party. Built with the subprocess for the two acts that need a CLI
/// (identity and inbox token), then UNLOCKED IN-PROCESS so the facade's own
/// `require_unlocked_here` gate passes — the capability pair `unlock_with_passphrase` +
/// `set_vault_unlocked` that `na0751_facade_invite_surface.rs:115-117` established.
fn bob_in_process(root: &Path) -> PathBuf {
    let cfg = root.join("bob");
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    // The two acts that need a CLI. Each subprocess carries its own `QSC_CONFIG_DIR`, so
    // these do not depend on the process env being set yet.
    run_ok(&cfg, &["identity", "rotate", "--confirm"]);
    run_ok(&cfg, &["relay", "inbox-set", "--token", BOB_INBOX]);
    // ⚠⚠ THE ENV MUST BE SET BEFORE THE IN-PROCESS UNLOCK, NOT AFTER IT. `unlock_with_passphrase`
    // resolves the vault through `QSC_CONFIG_DIR` at CALL time, so unlocking first and setting
    // the env afterwards fails with `vault_missing` — measured, not guessed: that is exactly how
    // this function failed on its first run. Setting it here also satisfies the file's other
    // ordering constraint, because every caller runs this BEFORE starting a server.
    set_env_once(&cfg);
    qsc::vault::unlock_with_passphrase(common::TEST_MOCK_VAULT_PASSPHRASE)
        .expect("in-process unlock of the same mock vault the subprocess used");
    qsc::set_vault_unlocked(true);
    cfg
}

fn mint(alice: &Path, base: &str) -> (String, String) {
    let out = run_ok(
        alice,
        &["invite", "create", "--relay", base, "--ttl-secs", "3600"],
    );
    let code = out
        .lines()
        .find(|l| l.starts_with("QSLI-1-"))
        .expect("invite code on stdout")
        .trim()
        .to_string();
    let listed = run_ok(alice, &["invite", "list"]);
    let id = listed
        .lines()
        .find_map(|l| l.strip_prefix("invite="))
        .and_then(|l| l.split_whitespace().next())
        .expect("invite id")
        .to_string();
    (code, id)
}

/// X1-X4 — THE WHOLE ROUND TRIP, WITH THE REDEEM AT THE FACADE AND THE FINISH MEASURED BOTH WAYS.
///
/// ⚠ MUST GO RED IF: `invite_redeem` stops provisioning inside the call, the finish stops
/// completing the handshake, or the post-handshake state stops being reachable from
/// `connect_status`. ⚠ X4 runs as a subprocess for a MEASURED reason — see `ENG-0239` at the
/// call site; the facade arm was driven and returned an error, and that error is FILED, not
/// asserted as if it were the expected behaviour. Every status comparison below is BY EQUALITY on the extracted value —
/// never `contains`, never a prefix (the `established` versus `established_recv_only` lesson).
#[test]
fn na0756_x1_x4_the_redeemer_drives_the_facade_through_a_real_handshake() {
    let _g = guard();
    let root = test_root("na0756_facade_roundtrip");

    // Bob's env FIRST, then his vault, and only THEN a server that spawns threads.
    // ⚠ Bob FIRST: `bob_in_process` sets the process env, and it must be set before the
    // relay's runtime threads exist.
    let bob = bob_in_process(&root);

    let relay = common::start_qsl_server(2 * 1024 * 1024, 512, None);
    let base = relay.base_url().to_string();
    let alice = party(&root, "alice", ALICE_INBOX);

    // X1 — Alice mints. The code is a side-channel-sized one-time capability.
    let (code, invite_id) = mint(&alice, &base);
    assert!(code.len() <= 250, "code must stay side-channel sized");

    // X2 — BOB REDEEMS, AT THE FACADE. First facade call to this verb in the tree's history.
    // The return IS the fingerprint, and the contact is provisioned INSIDE the call — which
    // is the engine fact that forced Lane B's single-view design.
    let fp = facade::invite_redeem(&code, "alice", None).expect("facade redeem succeeds");
    assert_eq!(fp.len(), 64, "the redeem returns a 64-hex fingerprint");
    assert!(
        fp.bytes().all(|b| b.is_ascii_hexdigit()),
        "and it is hex: {fp}"
    );

    // The contact EXISTS the moment redeem returns — there is no second, naming step for a
    // two-step UI to occupy, and this assertion is why mockup-15 is superseded.
    let rows = facade::contact_list().expect("contact_list");
    let alice_row = rows
        .iter()
        .find(|r| r.alias == "alice")
        .expect("redeem provisions the contact inside the call");
    assert_eq!(
        alice_row.fingerprint.as_ref().map(|f| f.full.as_str()),
        Some(fp.as_str()),
        "the provisioned contact carries the SAME fingerprint the redeem returned — one \
         source, two readings, and they must never disagree"
    );

    // X3 — Alice collects the handshake and answers it. Her side is the CLI, by the topology
    // constraint named in this file's header.
    let accept = run_ok(
        &alice,
        &["invite", "accept", "--invite-id", &invite_id, "--alias", "bob"],
    );
    assert!(
        accept.contains("status=pinned"),
        "Alice must end with a PENDING contact for Bob: {accept}"
    );

    // ⚠⚠ X4 — THE FINISH RUNS AS A SUBPROCESS, AND THAT IS A MEASURED CONSTRAINT RATHER THAN A
    // PREFERENCE. `ENG-0239`. Driven both ways against the SAME config dir, the SAME relay and
    // the SAME vault, with the process shape as the only variable:
    //     facade::invite_finish(..)  -> Err(Other("handshake_session_store_failed"))
    //     `qsc invite finish ..`     -> exit status 0
    // The handshake's session store discards the typed `ErrorCode` through `map_err(|_| ..)`
    // (`handshake/mod.rs:1929`, `:2156`), so the specific cause does not survive to this
    // surface — which is half of what the filing is about.
    //
    // ⚠ WHY THE IN-PROCESS ARM IS THE UNUSUAL ONE HERE: unlocking the vault in-process to
    // satisfy `require_unlocked_here` is exactly what takes `qsp_session_store_key_load`
    // (`protocol_state:168-181`) OFF the seed-fallback branch — that fallback fires only on
    // `Err("vault_missing" | "vault_locked")`, and an unlocked mock vault returns `Ok(None)`.
    // A subprocess runs with the vault LOCKED and therefore takes the fallback.
    //
    // ⚠⚠ WHAT THIS TEST DOES **NOT** SHOW, stated so its silence is not read as safety: whether
    // the DESKTOP hits the same wall. The desktop unlocks in-process too, but against a REAL
    // vault whose store key can genuinely be written, so its arm is UNMEASURED here — and it
    // cannot be measured in either harness, because neither can reach a relay (`ENG-0226`).
    // That is precisely what the operator's acceptance flight resolves.
    //
    // The shape below is the tree's own ruled idiom (`na0751_facade_invite_surface.rs`): the
    // SUBPROCESS writes the session into the config dir, and the FACADE then reads it
    // IN-PROCESS — which is what makes the assertion that follows a facade assertion rather
    // than a CLI one.
    let finish = run_ok(&bob, &["invite", "finish", "--alias", "alice", "--relay", &base]);
    assert!(
        finish.contains("invite_finish=ok"),
        "the reply was waiting, so finish reports FOUND: {finish}"
    );

    // And the post-handshake state reads BY EQUALITY off the typed surface the GUI consumes.
    let st = facade::connect_status("alice");
    assert_eq!(
        st.state,
        facade::ConnectState::Active,
        "after the handshake completes the peer reads ACTIVE"
    );
    assert_eq!(
        st.reason,
        facade::ConnectReason::Handshake,
        "and its reason is the handshake arm — the only Active producer"
    );
}

/// Z7(i) — THE SECURITY TELLS FIRE, AND BOTH ARMS ARE DRIVEN.
///
/// ⚠⚠ THIS IS THE FIRST TIME EITHER TELL HAS EVER FIRED IN THIS SUITE. Both are produced only
/// inside `verify_redeemed_bundle`, which needs a relay that answers, so no lane before this
/// one could reach them.
///
/// BOTH vectors come from ONE technique — decode a REAL minted code, mutate ONE field,
/// re-encode — which is why `R387` §S7's "drive both if both are cheaply constructible"
/// resolved to BOTH rather than to one plus a filing:
///   · mutate `commit`  -> the commitment check at `invite/mod.rs:568` fails FIRST, so the
///     bundle's keys do not match what the invite committed to  => `commitment_mismatch`
///   · mutate `expiry`  -> `commit` and the bundle still agree, so the commitment check
///     PASSES and the signature over the payload fails at `:576-578` => `signature_invalid`
/// The second is the sharper control: it proves the two arms are genuinely separable and that
/// the first is not simply "any tamper at all".
///
/// ⚠ EACH ARM NEEDS ITS OWN FRESHLY MINTED INVITE. The capability burns at `:1081`, the
/// instant the relay answers — BEFORE the verification at `:1101` — which is exactly why the
/// shipped UI offers no Retry.
#[test]
fn na0756_z7i_a_tampered_invite_fires_the_security_tell_by_equality() {
    use qsc::invite::{decode_invite_code, encode_invite_code};

    let _g = guard();
    let root = test_root("na0756_facade_tamper");
    // ⚠ Bob FIRST: `bob_in_process` sets the process env, and it must be set before the
    // relay's runtime threads exist.
    let bob = bob_in_process(&root);

    let relay = common::start_qsl_server(2 * 1024 * 1024, 512, None);
    let base = relay.base_url().to_string();
    let alice = party(&root, "alice", ALICE_INBOX);

    // ── ARM 1: substituted commitment -> commitment_mismatch ──
    let (code, _id) = mint(&alice, &base);
    let mut p = decode_invite_code(&code).expect("the real code decodes");
    p.commit[0] ^= 0xFF; // ONE flipped byte, in the field the commitment check reads
    let tampered = encode_invite_code(&p).expect("re-encode");
    assert_ne!(tampered, code, "the tamper must actually change the code");

    let err = facade::invite_redeem(&tampered, "mallory1", None)
        .expect_err("a tampered commitment MUST be refused");
    assert_eq!(
        err.as_wire(),
        "commitment_mismatch",
        "the tell is observed BY EQUALITY on the extracted wire code, not by a substring"
    );
    assert!(
        matches!(err, FacadeError::CommitmentMismatch),
        "and the typed arm agrees with the wire code"
    );

    // ── ARM 2: tampered field, intact commitment -> signature_invalid ──
    let (code2, _id2) = mint(&alice, &base);
    let mut p2 = decode_invite_code(&code2).expect("the real code decodes");
    p2.expiry += 1; // covered by the signature, NOT by the commitment, and still in the future
    let tampered2 = encode_invite_code(&p2).expect("re-encode");
    assert_ne!(tampered2, code2, "the tamper must actually change the code");

    let err2 = facade::invite_redeem(&tampered2, "mallory2", None)
        .expect_err("an altered invite field MUST be refused");
    assert_eq!(
        err2.as_wire(),
        "signature_invalid",
        "the OTHER tell, reached by leaving the commitment intact — which is what proves the \
         two arms are separable rather than one catch-all"
    );

    // ⚠ NOTHING WAS PROVISIONED BY EITHER REFUSAL. The contact is created at `:1106`, AFTER
    // the verification at `:1101`, so a refused invite must leave no contact behind. This is
    // the claim the shipped copy makes in the user's words — "Nothing was set up" — and it is
    // measured here rather than asserted there.
    let rows = facade::contact_list().expect("contact_list");
    for alias in ["mallory1", "mallory2"] {
        assert!(
            !rows.iter().any(|r| r.alias == alias),
            "a refused invite must provision NOTHING, but `{alias}` exists"
        );
    }
}

/// Z7(ii) — FINISH BEFORE ACCEPT IS THE NOT-YET OUTCOME: GREEN, AND NOTHING ON ALICE'S SIDE.
///
/// ⚠ MUST GO RED IF: a finish with no reply waiting starts returning an ERROR. "Not yet" is
/// the normal case — the earliest possible yes is after the other party manually approves —
/// and the source says so itself at `invite/mod.rs:1435-1439`: *"Not an error, because it is
/// not one — the reply has not arrived yet."* The shipped UI depends on that: the finish
/// triggers must be able to run at every unlock and stay SILENT.
///
/// ⚠ This is the first observation of the outcome anywhere in the suite: `not_yet` and
/// `invite_finish=none` both measured `rc 1` across `tests/` before this file existed.
#[test]
fn na0756_z7ii_finish_before_accept_is_not_yet_and_provisions_nothing() {
    let _g = guard();
    let root = test_root("na0756_facade_notyet");
    // ⚠ Bob FIRST: `bob_in_process` sets the process env, and it must be set before the
    // relay's runtime threads exist.
    let bob = bob_in_process(&root);

    let relay = common::start_qsl_server(2 * 1024 * 1024, 512, None);
    let base = relay.base_url().to_string();
    let alice = party(&root, "alice", ALICE_INBOX);

    let (code, _invite_id) = mint(&alice, &base);
    facade::invite_redeem(&code, "alice", None).expect("redeem succeeds");

    // ⚠ NO `invite accept` RUNS. Alice has not approved, so nothing is waiting for Bob.
    let done = facade::invite_finish(None, "alice", &base, 1).expect("finish must NOT error");
    assert_eq!(
        done, false,
        "with no reply waiting the outcome is NOT-YET — false, not an error"
    );

    // The contact is still there and still establishing: not-yet is not a rollback.
    let rows = facade::contact_list().expect("contact_list");
    assert!(
        rows.iter().any(|r| r.alias == "alice"),
        "the pending contact survives a not-yet finish"
    );
    let st = facade::connect_status("alice");
    assert_eq!(
        st.state,
        facade::ConnectState::Inactive,
        "and it still reads INACTIVE — which is exactly the predicate the GUI's finish \
         trigger scans on, compared by equality"
    );

    // ⚠ AND NOTHING WAS PROVISIONED ON ALICE'S SIDE. She never accepted, so she must hold no
    // contact for Bob. A redeem that reached across and created state on the inviter's side
    // without her approval would defeat the whole gate.
    let alice_view = qsc(&alice)
        .args(["contacts", "list"])
        .output()
        .expect("run qsc");
    let text = output_text(&alice_view);
    assert!(
        !text.contains("bob"),
        "nothing may be provisioned on the inviter's side before she accepts:\n{text}"
    );

    // The not-yet outcome is REPEATABLE, not a one-shot that consumed something.
    let again = facade::invite_finish(None, "alice", &base, 1).expect("finish must NOT error");
    assert_eq!(again, false, "still not yet, and still not an error");
}
