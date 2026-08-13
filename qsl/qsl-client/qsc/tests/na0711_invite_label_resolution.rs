//! NA-0711 (D647 as amended by A4; R237 §6, R238 §2.2) — THE SELF-LABEL RESOLUTION INSTRUMENT.
//!
//! Three rows, and ⚠ **NEITHER SUBSTITUTES FOR THE OTHER** (R237 §6.2):
//!
//! * **A — SUCCESS.** An identity under a NON-DEFAULT label, a two-party invite flow driven with
//!   **no label flag anywhere**, and the accepter must **reach a session**. ⚠ This is the outcome
//!   assertion; asserting an improved error message instead is the substitution A2 §F forbids.
//! * **B — REFUSAL.** An **explicit wrong** label on the accepter's poll must **fail closed** and the
//!   message must name **the whole key** and **which of the three states** the lookup saw.
//! * **C — THE SLOT.** A **mislabelled `invite accept`** must refuse **before the relay pull**, so the
//!   slot stays unredeemed and a second, correct call succeeds. ⚠ A PARTIAL on ENG-0175, never a
//!   closure: every other failure mode still burns the slot.
//!
//! ⚠ **WHY ROW B AND ROW C USE A DEFAULT-LABEL IDENTITY AND ROW A DOES NOT.** Row A needs a
//! non-default label or there is nothing to derive — with the identity under `self` the keys already
//! agree and the flow already passes at base (`NA_0681_two_party_handshake`), which is BLOCKER-2's
//! finding: an instrument that cannot go RED is not evidence. Rows B and C need their SETUP to
//! succeed at base so their RED lands on the asserted step rather than on an earlier failure.
//!
//! ⚠ **WHAT A GREEN HERE DOES NOT ASSERT:** anything about the real relay (the rig walk is a separate
//! gate), anything about messaging, and nothing about a config dir holding two or more identities —
//! ⚠ **in that case an explicit wrong-but-existing label still passes the gate and the lookup still
//! misses silently** (the residual hole of candidate #1, stated rather than discovered).

mod common;

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

/// A party whose identity lives under `label`. `None` = the canonical default (`self`).
fn party(root: &Path, name: &str, inbox: &str, label: Option<&str>) -> PathBuf {
    let cfg = root.join(name);
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    match label {
        Some(l) => run_ok(&cfg, &["identity", "rotate", "--as", l, "--confirm"]),
        None => run_ok(&cfg, &["identity", "rotate", "--confirm"]),
    };
    run_ok(&cfg, &["relay", "inbox-set", "--token", inbox]);
    cfg
}

fn mint_invite(alice: &Path, base: &str) -> (String, String) {
    let code = run_ok(
        alice,
        &["invite", "create", "--relay", base, "--ttl-secs", "3600"],
    );
    let code = code
        .lines()
        .find(|l| l.starts_with("QSLI-1-"))
        .expect("invite code on stdout")
        .trim()
        .to_string();
    let listed = run_ok(alice, &["invite", "list"]);
    let invite_id = listed
        .lines()
        .find_map(|l| l.strip_prefix("invite="))
        .and_then(|l| l.split_whitespace().next())
        .expect("invite id")
        .to_string();
    (code, invite_id)
}

const A_INBOX: &str = "na0711_alice_inbox_token_abcdefgh";
const B_INBOX: &str = "na0711_bob_inbox_token_ijklmnopq";

/// ROW A — the outcome assertion. Identity under a NON-DEFAULT label, **no label flag anywhere**,
/// and the accepter must reach a session.
#[test]
fn row_a_accepter_reaches_a_session_with_no_label_flag_anywhere() {
    let _g = guard();
    let relay = common::start_qsl_server(2 * 1024 * 1024, 512, None);
    let base = relay.base_url().to_string();
    let root = test_root("na0711_row_a");

    // ⚠ The accepter's identity is NOT under the default label. That is the whole point.
    let alice = party(&root, "alice", A_INBOX, Some("station"));
    let bob = party(&root, "bob", B_INBOX, None);

    let (code, invite_id) = mint_invite(&alice, &base);
    run_ok(
        &bob,
        &["invite", "redeem", "--code", &code, "--alias", "alice"],
    );
    run_ok(
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
    let finish = run_ok(
        &bob,
        &["invite", "finish", "--alias", "alice", "--relay", &base],
    );
    assert!(finish.contains("invite_finish=ok"), "{finish}");

    // ⭐ THE ASSERTED STEP — byte-identical before and after the fix: no `--as`.
    let poll = run_ok(
        &alice,
        &["handshake", "poll", "--peer", "bob", "--relay", &base],
    );
    assert!(
        poll.contains("handshake_complete") && poll.contains("role=responder"),
        "the accepter must COMPLETE the handshake, not merely report a better error:\n{poll}"
    );

    let status = run_ok(&alice, &["handshake", "status", "--peer", "bob"]);
    assert!(
        !status.contains("status=no_session"),
        "the accepter must hold a session:\n{status}"
    );
}

/// ROW B — the safety net. An EXPLICIT wrong label must fail closed, naming the whole key and the
/// state the lookup saw.
#[test]
fn row_b_explicit_wrong_label_fails_closed_naming_the_whole_key() {
    let _g = guard();
    let relay = common::start_qsl_server(2 * 1024 * 1024, 512, None);
    let base = relay.base_url().to_string();
    let root = test_root("na0711_row_b");

    // ⚠ Default-label parties, so the SETUP succeeds at base and the RED lands on the asserted step.
    let alice = party(&root, "alice", A_INBOX, None);
    let bob = party(&root, "bob", B_INBOX, None);

    let (code, invite_id) = mint_invite(&alice, &base);
    run_ok(
        &bob,
        &["invite", "redeem", "--code", &code, "--alias", "alice"],
    );
    run_ok(
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
    run_ok(
        &bob,
        &["invite", "finish", "--alias", "alice", "--relay", &base],
    );

    // ⭐ THE ASSERTED STEP — byte-identical before and after the fix.
    let out = run_expect_fail(
        &alice,
        &[
            "handshake",
            "poll",
            "--peer",
            "bob",
            "--relay",
            &base,
            "--as",
            "wrong",
        ],
    );
    assert!(
        out.contains("handshake.pending.wrong.bob"),
        "the refusal must name THE WHOLE KEY it looked under:\n{out}"
    );
    assert!(
        out.contains("self_label_unresolved") || out.contains("identity_self_ambiguous"),
        "the refusal must carry a reason of its own, not a swallowed one:\n{out}"
    );

    // ⚠ And the session must NOT have been established by a refused poll.
    let status = run_ok(&alice, &["handshake", "status", "--peer", "bob"]);
    assert!(status.contains("status=no_session"), "{status}");
}

/// ROW C — the slot. A mislabelled `invite accept` must refuse BEFORE the relay pull, leaving the
/// slot unredeemed so a second, correct call succeeds. ⚠ A PARTIAL on ENG-0175, never a closure.
#[test]
fn row_c_mislabelled_accept_leaves_the_slot_unredeemed() {
    let _g = guard();
    let relay = common::start_qsl_server(2 * 1024 * 1024, 512, None);
    let base = relay.base_url().to_string();
    let root = test_root("na0711_row_c");

    let alice = party(&root, "alice", A_INBOX, None);
    let bob = party(&root, "bob", B_INBOX, None);

    let (code, invite_id) = mint_invite(&alice, &base);
    run_ok(
        &bob,
        &["invite", "redeem", "--code", &code, "--alias", "alice"],
    );

    // ⭐ THE MISLABELLED CALL — it must refuse, and it must refuse BEFORE the pull.
    let refused = run_expect_fail(
        &alice,
        &[
            "invite",
            "accept",
            "--invite-id",
            &invite_id,
            "--alias",
            "bob",
            "--as",
            "wrong",
        ],
    );
    assert!(
        !refused.contains("handshake_send"),
        "a refusal must happen BEFORE any frame is sent:\n{refused}"
    );

    // ⚠ THE SLOT SURVIVED: the second, correct call succeeds where today it would read
    // `invite_already_redeemed`.
    let second = run_ok(
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
        second.contains("status=pinned"),
        "the retry must complete the accept:\n{second}"
    );
}
