// NA-0688 / D622 C1 (R4a–R4e) — the ONE clock, and the negative control it exists to make
// possible.
//
// ⚠ THE POINT OF THIS FILE IS THAT IT CONTAINS NO SLEEPS. Before C1, forcing an invite past
// its expiry through the CLI meant either waiting out a real TTL or hand-forging a code with
// a dead timestamp — the first is slow and flaky, the second never exercises the create+check
// path at all. `crate::clock` makes "now" a single injectable value, so a test can put the
// client on the far side of an expiry deterministically and still drive the REAL create and
// the REAL redeem.
//
// The injection is an environment variable because the tests drive `qsc` as a SUBPROCESS: a
// thread-local or a compile-time hook cannot reach the code under test.

mod common;

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Mutex, MutexGuard, OnceLock};

const CLOCK_ENV: &str = "QSC_UNSAFE_TEST_CLOCK_UNIX_S";
const ALICE_INBOX: &str = "na0688_alice_inbox_token_abcdefgh";
const BOB_INBOX: &str = "na0688_bob_inbox_token_ijklmnopq";

fn guard() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

fn test_root(tag: &str) -> PathBuf {
    let root = std::env::var("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("target"))
        .join("qsc-test-tmp")
        .join(format!("{tag}_{}", std::process::id()));
    ensure_dir_700(&root);
    root
}

fn ensure_dir_700(p: &Path) {
    std::fs::create_dir_all(p).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(p, std::fs::Permissions::from_mode(0o700));
    }
}

fn output_text(out: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    )
}

/// `qsc` with the clock optionally PINNED. `at` is the whole point of this helper.
fn qsc_at(cfg: &Path, at: Option<u64>) -> Command {
    let mut c = common::qsc_std_command();
    c.env("QSC_CONFIG_DIR", cfg).env("QSC_MARK_FORMAT", "plain");
    match at {
        Some(t) => {
            c.env(CLOCK_ENV, t.to_string());
        }
        // ⚠ Explicitly REMOVE it. Inheriting a pinned clock from the parent process would
        // make an arm that believes it is on the real clock silently deterministic.
        None => {
            c.env_remove(CLOCK_ENV);
        }
    }
    c
}

fn run_ok_at(cfg: &Path, at: Option<u64>, args: &[&str]) -> String {
    let out = qsc_at(cfg, at).args(args).output().expect("run qsc");
    let text = output_text(&out);
    assert!(out.status.success(), "command failed {args:?}\n{text}");
    text
}

fn run_fail_at(cfg: &Path, at: Option<u64>, args: &[&str]) -> String {
    let out = qsc_at(cfg, at).args(args).output().expect("run qsc");
    let text = output_text(&out);
    assert!(
        !out.status.success(),
        "command unexpectedly SUCCEEDED {args:?}\n{text}"
    );
    text
}

fn party(root: &Path, name: &str, inbox: &str, at: Option<u64>) -> PathBuf {
    let cfg = root.join(name);
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    run_ok_at(&cfg, at, &["identity", "rotate", "--confirm"]);
    run_ok_at(&cfg, at, &["relay", "inbox-set", "--token", inbox]);
    cfg
}

fn real_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// R4b — THE NEGATIVE CONTROL, both arms driven by the injected clock, NO SLEEPS.
///
/// The single variable is the redeeming client's clock. Everything else — the relay, the
/// parties, the invite, the TTL — is identical between the arms, so a pass cannot be an
/// accident of setup.
///
///   arm LIVE    : redeem at T0 + 10          -> must NOT be refused as expired
///   arm EXPIRED : redeem at T0 + ttl + 60    -> must be refused with `invite_expired`
///
/// ⚠ The LIVE arm is what stops this from being vacuous. Asserting only that a far-future
/// clock refuses the invite would also pass if the invite were broken for some unrelated
/// reason, or if redemption never worked at all.
///
/// The base instant is taken from the REAL clock so that the relay — which sees invite
/// expiry, the only relay-visible time in v1 (R4d) — is never handed an absurd date.
#[test]
fn an_invite_expires_on_the_injected_clock_without_sleeping() {
    let _g = guard();
    let relay = common::start_qsl_server(2 * 1024 * 1024, 512, None);
    let base = relay.base_url().to_string();
    let root = test_root("na0688_c1_expiry");

    let t0 = real_now();
    let ttl: u64 = 3600;

    let alice = party(&root, "alice", ALICE_INBOX, Some(t0));
    let bob_live = party(&root, "bob_live", BOB_INBOX, Some(t0));

    // One invite, minted on a pinned clock.
    let create = run_ok_at(
        &alice,
        Some(t0),
        &[
            "invite",
            "create",
            "--relay",
            &base,
            "--ttl-secs",
            &ttl.to_string(),
        ],
    );
    let code = create
        .lines()
        .find(|l| l.starts_with("QSLI-1-"))
        .unwrap_or_else(|| panic!("no invite code minted:\n{create}"))
        .trim()
        .to_string();

    // ARM 1 — LIVE. Ten seconds after minting, the invite must be usable.
    let live = run_ok_at(
        &bob_live,
        Some(t0 + 10),
        &["invite", "redeem", "--code", &code, "--alias", "alice"],
    );
    assert!(
        !live.contains("invite_expired"),
        "an invite 10s old must NOT be expired — if this fires, the EXPIRED arm below \
         proves nothing:\n{live}"
    );

    // ARM 2 — EXPIRED. A fresh party redeems the same code with the clock past the TTL.
    // ⚠ A fresh config dir, so the refusal is expiry and not single-use replay.
    let bob_late = party(&root, "bob_late", BOB_INBOX, Some(t0));
    let expired = run_fail_at(
        &bob_late,
        Some(t0 + ttl + 60),
        &["invite", "redeem", "--code", &code, "--alias", "alice"],
    );
    assert!(
        expired.contains("invite_expired"),
        "past its TTL on the injected clock, the invite must die with `invite_expired` \
         BEFORE any network attempt:\n{expired}"
    );

    let _ = std::fs::remove_dir_all(&root);
}

/// The override is LOUD. An overridden clock that left no trace in the output would be a
/// silent behaviour change, and this repo's whole diagnostic discipline is that unusual
/// states announce themselves.
#[test]
fn a_pinned_clock_announces_itself_in_the_markers() {
    let _g = guard();
    let root = test_root("na0688_c1_marker");
    let cfg = root.join("solo");
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);

    // ⚠ `outbox status` is chosen because it ACTUALLY READS THE CLOCK
    // (`msgqueue::summarize_at(&dir, msgqueue::now_unix_s())`) and needs no relay. The first
    // draft of this test used `identity rotate`, which never consults the clock at all — so
    // it asserted the absence of a marker that could not have appeared either way. A test
    // whose subject is never invoked is the vacuous-pass shape, caught here by the assertion
    // failing rather than by luck.
    let pinned = run_ok_at(&cfg, Some(1_700_000_000), &["outbox", "status"]);
    assert!(
        pinned.contains("event=clock_override"),
        "a pinned clock must emit `clock_override`:\n{pinned}"
    );

    let unpinned = run_ok_at(&cfg, None, &["outbox", "status"]);
    assert!(
        !unpinned.contains("event=clock_override"),
        "an UNPINNED run must not claim an override — otherwise the marker says nothing:\n\
         {unpinned}"
    );

    let _ = std::fs::remove_dir_all(&root);
}
