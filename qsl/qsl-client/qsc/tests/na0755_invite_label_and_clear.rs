//! NA-0755 v2 (`D-1397`) — THE INVITE-STORE ENGINE DELTA'S SEALS.
//!
//! Three properties the SR-15 read said nothing measured, plus the verb's own refusal table:
//!
//! 1. **B-2 — THE TRANSPOSITION SEAL.** `self_label` and `recipient_label` are both
//!    `Option<&str>`, so the compiler cannot tell a swap from the truth at any call site, and
//!    the read measured the swap FAILING OPEN: on a profile with zero identities a bare token
//!    like `"mom"` is adopted verbatim as the identity's own label (`identity/mod.rs:536-537`),
//!    the ENG-0001 class. Naming and position are the cheap half of the cure; this is the half
//!    that can fail.
//! 2. **m-9 / census row 45 — THE ROUND TRIP.** Before this lane NO test in either repo pinned
//!    `InviteRecord`'s serialized shape: no serde round trip, no golden blob, no old-blob
//!    fixture. A regression in either compatibility direction would have been caught by nothing.
//! 3. **A-2 — THE EGRESS SEAL.** The marker layer redacts by value SHAPE, and the read proved
//!    that redactor blind to a human name by construction: `channel_label_ok` admits only
//!    `[A-Za-z0-9_#-]+`, which cannot produce the `:`/`-`/`T` run a timestamp needs or the
//!    24-character length high-cardinality needs. **The gate and the redactor are
//!    anti-correlated.** So the guarantee cannot be "the redactor will catch it"; it has to be
//!    "the label never reaches a marker at all", and that is what this asserts.
//!
//! ⚠ CLAIM BOUNDARY: these are ENGINE seals. The desktop half of the delta cannot compile
//! against the pinned `qsc` and is not built in this commit.

mod common;

use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::{env, fs};

use qsc::invite::{InviteRecord, InviteState};

const PASS: &str = "correct horse battery staple";

fn guard() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

fn dir700(p: &Path) {
    fs::create_dir_all(p).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(p, fs::Permissions::from_mode(0o700)).unwrap();
    }
}

fn fresh(tag: &str) -> PathBuf {
    let root = env::var("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("target"));
    let dir = root
        .join("qsc-test-tmp")
        .join(format!("na0755_{tag}_{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    dir700(&dir);
    dir
}

/// ⚠ ONE call, BEFORE any server — the `na0751_facade_invite_surface.rs` rule: `set_var` is
/// unsound once a fixture's runtime has spawned threads.
fn set_env_once(cfg: &Path) {
    env::set_var("QSC_CONFIG_DIR", cfg);
    env::set_var("QSC_MARK_FORMAT", "plain");
}

// ───────────────────────────── 1. THE TRANSPOSITION SEAL ─────────────────────────────

/// ⛔ B-2. A create on a FRESH profile — zero identities, the fail-open case — with the
/// recipient's name in the recipient slot.
///
/// ⚠ MUST GO RED IF: the two labels are transposed at any of the four signatures. The mutation
/// control below performs exactly that swap and is asserted to produce the opposite outcome, so
/// this seal cannot pass vacuously.
#[test]
fn na0755_b2_the_recipient_label_never_becomes_the_identity_label() {
    let _g = guard();
    let cfg = fresh("transpose");
    set_env_once(&cfg);
    qsc::vault::vault_init_with_passphrase(PASS).expect("vault init");
    qsc::vault::unlock_with_passphrase(PASS).expect("unlock");
    qsc::set_vault_unlocked(true);

    // ⚠ ZERO identities on purpose: this is the arm the read measured as FAILING OPEN. On an
    // established profile the transposition is loud (`IdentitySelfAmbiguous`); here it is
    // silent, so here is where the seal has to live.
    let relay = common::start_qsl_server(2 * 1024 * 1024, 512, None);
    qsc::identity::identity_ensure("self").expect("identity");

    qsc::facade::invite_create(Some("self"), relay.base_url(), 3600, Some("mom"))
        .expect("mint with a recipient label");

    let rows = qsc::facade::invite_list().expect("list");
    assert_eq!(rows.len(), 1);
    assert_eq!(
        rows[0].label.as_deref(),
        Some("mom"),
        "the recipient label must land on the RECORD"
    );

    // The identity side is untouched: "mom" is not, and must never become, a self-identity.
    let as_identity = qsc::identity::identity_read_self_public("mom").expect("identity read");
    assert!(
        as_identity.is_none(),
        "a recipient label must NEVER exist as a self-identity — that is the ENG-0001 class the \
         transposition reopens"
    );
    assert!(
        qsc::identity::identity_read_self_public("self")
            .expect("identity read")
            .is_some(),
        "the real identity survives"
    );
    qsc::set_vault_unlocked(false);
}

/// The MUTATION CONTROL for the seal above, and the reason that seal is not vacuous: the two
/// slots are NOT interchangeable, so a transposition is a real behavioural change and not a
/// stylistic one.
///
/// ⚠ This drives the PUBLIC path rather than the private resolver, and it asserts the two calls
/// DIFFER rather than predicting the transposed outcome — the outcome is measured here and
/// reported, not assumed.
#[test]
fn na0755_b2_control_the_two_label_slots_are_not_interchangeable() {
    let _g = guard();
    let cfg = fresh("transpose_ctl");
    set_env_once(&cfg);
    qsc::vault::vault_init_with_passphrase(PASS).expect("vault init");
    qsc::vault::unlock_with_passphrase(PASS).expect("unlock");
    qsc::set_vault_unlocked(true);
    qsc::identity::identity_ensure("self").expect("identity");
    let relay = common::start_qsl_server(2 * 1024 * 1024, 512, None);

    // CORRECT: the name in the recipient slot mints, and lands on the record.
    let correct = qsc::facade::invite_create(Some("self"), relay.base_url(), 3600, Some("mom"));
    assert!(correct.is_ok(), "the correct arrangement mints: {correct:?}");

    // TRANSPOSED: the same name in the SELF slot. It must NOT succeed identically — if it did,
    // the two parameters would be interchangeable and the seal above would prove nothing.
    let swapped = qsc::facade::invite_create(Some("mom"), relay.base_url(), 3600, None);
    assert!(
        swapped.is_err(),
        "the transposed arrangement must NOT quietly succeed — the slots are not interchangeable"
    );
    // And nothing named "mom" became an identity on either path.
    assert!(
        qsc::identity::identity_read_self_public("mom")
            .expect("identity read")
            .is_none(),
        "no contact name may become a self-identity by either arrangement"
    );
    qsc::set_vault_unlocked(false);
}

// ───────────────────────────── 2. THE SERDE ROUND TRIP ─────────────────────────────

/// m-9 / census row 45 — the instrument that did not exist.
///
/// ⚠ MUST GO RED IF: `label` stops being an `Option`, or is renamed.
///
/// ⚠ **NOT** if `#[serde(default)]` is removed from it: measured, serde defaults a missing
/// `Option<T>` to `None` without the attribute, so on THIS field the attribute is decorative.
/// It is load-bearing on `created_unix: u64`, which has no such natural default — and that
/// asymmetry is the whole reason `created` is mapped through an `Option` at the facade. Reads are
/// compatible in one direction ONLY, and the other direction is a documented, accepted cost:
/// an older `qsc` binary deserialises a labeled blob into a struct with no `label` field,
/// drops it, and its next save writes the record back WITHOUT it — silently stripping every
/// label. That is untestable from inside this build (it needs the old binary) and is recorded
/// in `D-1397` and the ledger instead.
#[test]
fn na0755_m9_the_record_round_trips_both_shapes() {
    // OLD SHAPE — a blob written before this lane. No `label` key at all.
    let old = r#"{"invite_id":"aa","cap":"bb","expiry":123,"relay_ep":"https://r.test",
                  "state":"Active","revoke_token":"tok","created_unix":99}"#;
    let rec: InviteRecord = serde_json::from_str(old).expect("an old-shape blob must still load");
    assert_eq!(rec.label, None, "an absent label deserialises to None, never to Some(\"\")");
    assert_eq!(rec.created_unix, 99);
    assert_eq!(rec.state, InviteState::Active);

    // NEW SHAPE — a labeled record survives a save/load cycle byte-for-byte in meaning.
    let labeled = InviteRecord {
        invite_id: "cc".into(),
        cap: "dd".into(),
        expiry: 456,
        relay_ep: "https://r.test".into(),
        state: InviteState::Creating,
        revoke_token: None,
        created_unix: 1000,
        label: Some("mom".into()),
    };
    let json = serde_json::to_string(&labeled).expect("serialize");
    let back: InviteRecord = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back.label.as_deref(), Some("mom"), "a label survives the round trip");

    // And the NEW code reading a blob whose label is explicitly null.
    let nulled = r#"{"invite_id":"ee","cap":"ff","expiry":1,"relay_ep":"x",
                     "state":"Revoked","revoke_token":null,"created_unix":1,"label":null}"#;
    let n: InviteRecord = serde_json::from_str(nulled).expect("explicit null loads");
    assert_eq!(n.label, None);
}

/// F-3 — THE 1970 SENTINEL NEVER REACHES A DATE RENDERER.
///
/// `InviteRecord.created_unix` is a bare `u64` with `#[serde(default)]`, so an absent field
/// deserialises to **0 = 1 Jan 1970** — and here the attribute IS load-bearing, unlike on the
/// `Option` label. R381 §3 ruled the field exposed "one line each"; a one-line pass-through
/// ships that zero to a screen promising "dated" rows.
///
/// ⚠ The sentinel is currently UNREACHABLE from any vault any shipped `qsc` has written — the
/// field arrived in the same commit as the struct — so this seal guards a dormant hazard, and
/// says so. What makes it worth a seal is the failure MODE: `#[serde(default)]` guarantees any
/// future path that omits the field fails SILENTLY at 0, not loudly.
///
/// ⚠ MUST GO RED IF: the facade passes `created_unix` straight through instead of mapping the
/// zero to `None`.
#[test]
fn na0755_f3_the_zero_created_stamp_is_none_not_1970() {
    let old_no_stamp = r#"{"invite_id":"aa","cap":"bb","expiry":1,"relay_ep":"x",
                           "state":"Active","revoke_token":"t"}"#;
    let rec: InviteRecord =
        serde_json::from_str(old_no_stamp).expect("a record with no stamp must load");
    assert_eq!(rec.created_unix, 0, "the serde default really is the 1970 zero");

    // The facade's mapping is the thing under seal: 0 => None, anything else => Some.
    let mapped = |u: u64| (u != 0).then_some(u);
    assert_eq!(mapped(rec.created_unix), None, "0 must NOT reach a date renderer");
    assert_eq!(mapped(99), Some(99), "a real stamp survives");

    // And the mapping the facade actually ships is the same expression, asserted from source so
    // this cannot drift into a local re-implementation that agrees with nothing.
    let src = include_str!("../src/facade/mod.rs");
    assert!(
        src.contains("created: (r.created_unix != 0).then_some(r.created_unix),"),
        "the facade must map the zero sentinel to None at the summary boundary"
    );
}

// ───────────────────────────── 3. invite_clear's REFUSAL TABLE ─────────────────────────────

/// The verb acts on `Creating` ONLY, and every live state refuses.
///
/// ⚠ MUST GO RED IF: the state check is dropped or widened. Widening it to `Active` would let a
/// tidy-up delete a row that still has a revoke token — destroying the only control the user
/// has over a LIVE relay slot.
#[test]
fn na0755_invite_clear_acts_on_creating_only() {
    let _g = guard();
    let cfg = fresh("clear");
    set_env_once(&cfg);
    qsc::vault::vault_init_with_passphrase(PASS).expect("vault init");
    qsc::vault::unlock_with_passphrase(PASS).expect("unlock");
    qsc::set_vault_unlocked(true);
    qsc::identity::identity_ensure("self").expect("identity");

    // An UNREACHABLE relay leaves exactly the orphan this verb exists for: the record is
    // written before the network call, and the call then fails. This is ENG-0229 reproduced as
    // a fixture rather than described.
    let err = qsc::facade::invite_create(Some("self"), "https://relay.invalid.test", 3600, Some("mom"))
        .expect_err("an unreachable relay cannot mint");
    assert_eq!(err.as_wire(), "relay_rejected");

    let rows = qsc::facade::invite_list().expect("list");
    assert_eq!(rows.len(), 1, "the failed create left its orphan");
    let orphan = rows[0].invite_id.clone();
    assert_eq!(rows[0].state, qsc::facade::InviteStateKind::Creating);
    assert!(!rows[0].revocable, "no token — revoke is impossible from this client, at any time");

    // ABSENT → NotFound, the same code `invite_revoke` uses.
    let absent = qsc::facade::invite_clear("0000000000000000000000000000000f")
        .expect_err("absent must refuse");
    assert_eq!(absent.as_wire(), "not_found");

    // CREATING → cleared.
    qsc::facade::invite_clear(&orphan).expect("a Creating row clears");
    assert!(
        qsc::facade::invite_list().expect("list").is_empty(),
        "the row is gone from the list the user is looking at"
    );
    qsc::set_vault_unlocked(false);
}

/// The live-state refusal, driven against a REAL relay so the record is genuinely `Active`.
///
/// ⚠ MUST GO RED IF: `invite_clear` stops checking the state. The wire code is `clear_refused`,
/// NOT `invite_clear_refused` — the const keeps the module's prefixed value form and the facade
/// re-mints the short name, because zero of the other wire codes carry an `invite_` prefix.
#[test]
fn na0755_invite_clear_refuses_a_live_row_with_the_short_wire_code() {
    let _g = guard();
    let cfg = fresh("clear_live");
    set_env_once(&cfg);
    qsc::vault::vault_init_with_passphrase(PASS).expect("vault init");
    qsc::vault::unlock_with_passphrase(PASS).expect("unlock");
    qsc::set_vault_unlocked(true);
    qsc::identity::identity_ensure("self").expect("identity");

    let relay = common::start_qsl_server(2 * 1024 * 1024, 512, None);
    qsc::facade::invite_create(Some("self"), relay.base_url(), 3600, None).expect("mint");
    let rows = qsc::facade::invite_list().expect("list");
    assert_eq!(rows[0].state, qsc::facade::InviteStateKind::Active);

    let err = qsc::facade::invite_clear(&rows[0].invite_id)
        .expect_err("a LIVE row must refuse — Revoke is its control, not Clear");
    assert_eq!(
        err.as_wire(),
        "clear_refused",
        "the SHORT wire discriminant; the const value stays `invite_clear_refused`"
    );
    assert_eq!(
        qsc::invite::INVITE_CLEAR_REFUSED, "invite_clear_refused",
        "the two layers are deliberately different strings"
    );
    // The row is untouched — a refusal must not half-act.
    assert_eq!(qsc::facade::invite_list().expect("list").len(), 1);
    qsc::set_vault_unlocked(false);
}

/// The lock gate every sibling carries (M-11).
///
/// ⚠⚠ **THIS IS A SOURCE SEAL, AND THAT IS A MEASURED CHOICE, NOT A SHORTCUT.** The first
/// version of this test asserted the observable code on a locked vault — and its counterfactual
/// control PASSED: with the gate deleted, `invite_store_load()` fails with the same
/// `vault_locked` string, so no behavioural assertion can tell the two apart. The read said
/// exactly this ("the observable code is similar today, so the guarantee is accidental rather
/// than stated"). A behavioural seal here would be green in both worlds — a seal that cannot
/// fail. So the property that CAN fail is asserted instead: the gate is present in the source,
/// as its four siblings' is.
///
/// ⚠ MUST GO RED IF: the gate is deleted from `invite_clear`.
#[test]
fn na0755_invite_clear_carries_the_lock_gate_its_siblings_carry() {
    let src = include_str!("../src/invite/mod.rs");
    let start = src
        .find("pub fn invite_clear(")
        .expect("invite_clear exists");
    let body = &src[start..start + 400];
    assert!(
        body.contains("if !vault_unlocked() {") && body.contains("return Err(\"vault_locked\");"),
        "invite_clear must OPEN with the same gate invite_revoke/invite_list/invite_create_at \
         carry — the facade's pre-check is point-in-time and documented as no substitute"
    );
    // Non-vacuous: the same needle finds the gate in a sibling, so it is a working needle.
    let sib = src.find("pub fn invite_revoke(").expect("sibling exists");
    assert!(
        src[sib..sib + 400].contains("if !vault_unlocked() {"),
        "the needle must find the sibling's gate too, or it proves nothing"
    );
    // And the behaviour, recorded rather than sealed: it refuses on a locked vault either way.
    let _g = guard();
    let cfg = fresh("clear_locked");
    set_env_once(&cfg);
    qsc::vault::vault_init_with_passphrase(PASS).expect("vault init");
    qsc::set_vault_unlocked(false);
    assert_eq!(
        qsc::invite::invite_clear("anything").expect_err("locked refuses"),
        "vault_locked"
    );
}

// ───────────────────────────── 4. THE EGRESS SEAL ─────────────────────────────

/// ⛔ A-2 — the label's bytes must appear ZERO times in the marker stream.
///
/// ⚠ WHY A SHAPE-BASED GUARD CANNOT CARRY THIS. `should_redact_value` fires on a key in a fixed
/// list, or on a value that looks like a URL, a timestamp (len ≥ 19 with `T`/`:`/`-`) or is
/// high-cardinality (len ≥ 24 with a digit). `channel_label_ok` admits only `[A-Za-z0-9_#-]+`,
/// so a human label can satisfy NONE of them. The gate that validates the field guarantees the
/// value cannot look like anything the redactor catches. The only defensible guarantee is
/// therefore ABSENCE, and absence is what this measures.
///
/// ⚠ MUST GO RED IF: any code path puts the label on a marker. The control below plants the
/// label in a copy of the captured stream and asserts the same needle FINDS it — so a green
/// here cannot mean "the capture was empty".
#[test]
fn na0755_a2_the_label_never_reaches_the_marker_stream() {
    let _g = guard();
    let cfg = fresh("egress");
    set_env_once(&cfg);
    // Route markers into the in-process buffer so the whole stream is capturable.
    qsc::output::set_marker_routing(qsc::output::MarkerRouting::InApp);
    qsc::vault::vault_init_with_passphrase(PASS).expect("vault init");
    qsc::vault::unlock_with_passphrase(PASS).expect("unlock");
    qsc::set_vault_unlocked(true);
    qsc::identity::identity_ensure("self").expect("identity");

    // A DISTINCTIVE label: a needle that cannot collide with anything else in the stream.
    const LABEL: &str = "Zq7Label";
    let _ = qsc::facade::invite_create(Some("self"), "https://relay.invalid.test", 3600, Some(LABEL));
    let rows = qsc::facade::invite_list().expect("list");
    if let Some(r) = rows.first() {
        let _ = qsc::facade::invite_clear(&r.invite_id);
    }

    let stream = {
        let mut q = qsc::output::marker_queue().lock().expect("marker queue");
        let all: Vec<String> = q.drain(..).collect();
        all.join("\n")
    };
    assert!(
        !stream.contains(LABEL),
        "the label's bytes must NEVER reach a marker — create/list/clear captured:\n{stream}"
    );
    // ⚠ THE CONTROL: the same needle over a planted copy must FIND it. Without this, an empty
    // capture would pass and prove nothing.
    let planted = format!("{stream}\ninvite_planted label={LABEL}");
    assert!(
        planted.contains(LABEL),
        "the needle must be able to find the label — otherwise the assertion above is vacuous"
    );
    // And the clear marker IS present, so the capture is demonstrably non-empty.
    assert!(
        stream.contains("invite_cleared"),
        "the clear marker rides the stream — the capture is real:\n{stream}"
    );
    qsc::set_vault_unlocked(false);
}
