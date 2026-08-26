//! NA-0764 (`D-1405`, ruling `R6` / `D-D`) — **THE TWO ADDED CONTACT-SUMMARY FIELDS, SEALED.**
//!
//! Lane C's blessed detail pane needs two facts the summary did not carry: what YOU call this
//! contact, and how many devices it has. Both already existed on `ContactRecord` and were
//! dropped by `ContactSummary`. This file seals the widening and, more importantly, seals the
//! BOUND on it — because the record also holds `invite_id`, `route_token`, `fp`, `sig_fp` and
//! `kem_pk`, and the reason to widen deliberately is that widening carelessly is how
//! identifiers and key material reach a surface a future messaging lane renders by default.
//!
//! ⚠⚠ **THE RENAME IS NOT A RE-KEY, AND THAT IS THE DESIGN.** `alias` keys
//! `ContactsStore.peers`, `identity_read_pin(peer)` AND `qsp_session_for_channel(channel)`. A
//! rename that moved the key would reach identity pins and live sessions. `display_name` sits
//! BESIDE the key: the UI renders it and passes `alias`, always.
//!
//! ⚠ The allowlist below is sealed against the **SOURCE**, not against a Debug rendering.
//! `na0751_facade_invite_surface.rs` seals `InviteSummary` by counting `": "` in `{:?}` — a
//! good seal for a FLAT struct, but `ContactSummary.fingerprint` is
//! `Option<FingerprintPair>`, whose `Some` arm nests two more `": "` and makes an exact count
//! depend on whether the fixture's pin happens to be 64-hex. Sealing the declaration itself is
//! immune to that, and it fails on a new field even when no fixture exercises it.

mod common;

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::{env, fs};

use qsc::facade::{self, FacadeError};

/// A well-formed 64-hex fingerprint. Its VALUE is irrelevant here; only its SHAPE matters,
/// because `ContactSummary.fingerprint` is `Some` only for exactly 64 ASCII-hex.
const FP_A: &str = "aa11bb22cc33dd44ee55ff6600778899aa11bb22cc33dd44ee55ff6600778899";
const FP_B: &str = "bb22cc33dd44ee55ff6600778899aa11bb22cc33dd44ee55ff6600778899aa11";

/// The field set ruled onto this boundary. **An allowlist, never a denylist** — a denylist of
/// known-bad names admits every new sensitive field by construction, which is exactly how one
/// would enter unnoticed. Growing this array is a deliberate act that must be ruled.
const RULED_FIELDS: [&str; 7] = [
    "alias",
    "fingerprint",
    "pinned",
    "blocked",
    "state",
    "display_name",
    "device_count",
];

/// Named so the failure message can say WHY each is refused, not merely that it is.
const REFUSED_FIELDS: [(&str, &str); 5] = [
    (
        "invite_id",
        "provenance identifier — R6 keeps it off the summary",
    ),
    ("route_token", "a routing credential"),
    (
        "seen_at",
        "presence disclosure at an unblessed precision (C-20)",
    ),
    ("sig_fp", "key material"),
    ("kem_pk", "key material"),
];

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

fn fresh(tag: &str) -> PathBuf {
    let root = env::var("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("target"));
    let dir = root
        .join("qsc-test-tmp")
        .join(format!("na0764cs_{tag}_{}", std::process::id()));
    ensure_dir_700(&dir);
    dir
}

fn qsc(cfg: &Path) -> Command {
    let mut cmd = common::qsc_std_command();
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_MARK_FORMAT", "plain");
    cmd
}

fn run_ok(cfg: &Path, args: &[&str]) -> String {
    let out = qsc(cfg).args(args).output().expect("run qsc");
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(out.status.success(), "command failed {args:?}\n{text}");
    text
}

/// One unlocked profile with one contact. The contact is made over the CLI because
/// `contacts_provision_from_invite` is crate-private and a full invite round trip is not what
/// this file measures.
fn profile_with_contact(tag: &str, alias: &str) -> PathBuf {
    let cfg = fresh(tag);
    common::init_mock_vault(&cfg);
    // ⚠ `--route-token` is REQUIRED, measured: without it the CLI refuses with
    // `contacts_route_token_required`. The value is opaque here; only its presence matters.
    run_ok(
        &cfg,
        &[
            "contacts",
            "add",
            "--label",
            alias,
            "--fp",
            FP_A,
            "--route-token",
            "na0764_dana_route_token_abcdefgh",
        ],
    );
    env::set_var("QSC_CONFIG_DIR", &cfg);
    env::set_var("QSC_MARK_FORMAT", "plain");
    qsc::vault::unlock_with_passphrase(common::TEST_MOCK_VAULT_PASSPHRASE).expect("unlock");
    qsc::set_vault_unlocked(true);
    cfg
}

fn row(alias: &str) -> facade::ContactSummary {
    facade::contact_list()
        .expect("contact_list while unlocked")
        .into_iter()
        .find(|c| c.alias == alias)
        .unwrap_or_else(|| panic!("contact {alias} must be listed"))
}

// ───────────────────────────── P-3. THE BOUND, FIRST ─────────────────────────────

/// **P-3 / R6's guard sentence, as a test rather than a promise.**
///
/// R6 orders one sentence into the record: *"key material and route/invite identifiers never
/// leave the record onto summaries."* A sentence in a record cannot fail. This can.
///
/// ⚠ MUST GO RED IF: any field is added to `ContactSummary` without being ruled onto
/// [`RULED_FIELDS`] — including a field no fixture happens to populate, which a runtime seal
/// would miss entirely.
#[test]
fn the_summary_field_set_is_an_allowlist() {
    const FACADE_SRC: &str = include_str!("../src/facade/mod.rs");

    let decl = FACADE_SRC
        .split_once("pub struct ContactSummary {")
        .expect("ContactSummary must be declared in the facade")
        .1;
    let body = decl
        .split_once("\n}")
        .expect("the declaration must terminate")
        .0;

    // Field lines only: `pub <name>: <type>,` — doc comments and attributes are skipped, so a
    // field mentioned in prose cannot satisfy this and a field hidden behind an attribute
    // cannot escape it.
    let found: Vec<String> = body
        .lines()
        .filter_map(|l| l.trim().strip_prefix("pub "))
        .filter_map(|l| l.split_once(':'))
        .map(|(name, _)| name.trim().to_string())
        .collect();

    assert_eq!(
        found,
        RULED_FIELDS.to_vec(),
        "ContactSummary carries EXACTLY the ruled field set, in order. A new field must be \
         ruled onto this boundary before it can ship — that is R6's guard sentence, enforced. \
         found: {found:?}"
    );

    for (banned, why) in REFUSED_FIELDS {
        assert!(
            !found.iter().any(|f| f == banned),
            "`{banned}` must NOT be on the summary: {why}"
        );
    }
}

// ───────────────────────────── P-1. display_name ─────────────────────────────

/// **P-1 A3 + A2.** A rename reads back, and it moves NOTHING else — above all not the key.
///
/// ⚠ MUST GO RED IF: the setter re-keys the store, or the alias stops being the identity every
/// verb takes. The alias is asserted UNCHANGED and still resolvable after the rename, which is
/// the property that keeps identity pins and live sessions out of a rename's blast radius.
#[test]
fn rename_reads_back_and_changes_only_the_display_name() {
    let _g = guard();
    let _cfg = profile_with_contact("rename", "dana");

    let before = row("dana");
    assert_eq!(
        before.display_name, None,
        "a fresh contact has no local name"
    );

    facade::contact_set_display_name("dana", Some("Dana from work")).expect("rename succeeds");

    let after = row("dana");
    assert_eq!(
        after.display_name.as_deref(),
        Some("Dana from work"),
        "the display name reads back"
    );
    // THE KEY, AND EVERYTHING ELSE, UNMOVED.
    assert_eq!(after.alias, "dana", "the ALIAS KEY must not move");
    assert_eq!(after.fingerprint, before.fingerprint, "fingerprint moved");
    assert_eq!(after.pinned, before.pinned, "pinned moved");
    assert_eq!(after.blocked, before.blocked, "blocked moved");
    assert_eq!(after.state, before.state, "state moved");
    assert_eq!(
        after.device_count, before.device_count,
        "device_count moved"
    );

    // And clearing returns to typed absence, never Some("").
    facade::contact_set_display_name("dana", None).expect("clear succeeds");
    assert_eq!(row("dana").display_name, None, "clearing yields None");

    // Whitespace normalises at the boundary, so no consumer special-cases an empty string.
    facade::contact_set_display_name("dana", Some("   ")).expect("whitespace accepted");
    assert_eq!(
        row("dana").display_name,
        None,
        "an all-whitespace name normalises to None, never Some(\"\")"
    );
    qsc::set_vault_unlocked(false);
}

/// **P-1 A2, the other half.** Renaming an absent contact is a typed refusal, not a silent
/// no-op and not a panic.
///
/// ⚠ MUST GO RED IF: the setter starts creating contacts, or starts returning Ok for an alias
/// that does not exist. A rename that silently invents a contact is how a typo becomes a row.
#[test]
fn renaming_an_absent_contact_is_a_typed_refusal() {
    let _g = guard();
    let _cfg = profile_with_contact("absent", "dana");

    let err = facade::contact_set_display_name("nobody", Some("x"))
        .expect_err("an absent contact must refuse");
    assert!(
        matches!(err, FacadeError::NotFound),
        "the refusal is the typed NotFound, got {err:?}"
    );
    assert_eq!(
        facade::contact_list().expect("list").len(),
        1,
        "the refused rename must not have created a contact"
    );
    qsc::set_vault_unlocked(false);
}

/// **P-1 A1.** A record written BEFORE this lane — one with no `display_name` key at all —
/// loads unchanged and reads `None`.
///
/// ⚠⚠ THE FIXTURE IS A GENUINE LEGACY BLOB, NOT A FRESH RECORD. An earlier revision of this
/// test asserted `display_name == None` on a record this build had just written, and that
/// proved nothing about `#[serde(default)]`: the attribute carries no `skip_serializing_if`,
/// so a fresh record serialises `"display_name":null` and the key is PRESENT. The strip below
/// is what makes the arm real, and the assertion that the key was there to strip is what keeps
/// the strip from silently becoming a no-op.
///
/// ⚠⚠ **A RED ARM THIS FILE CLAIMED AND DID NOT HAVE, CORRECTED BY MEASUREMENT.** The first
/// version of this doc said *"MUST GO RED IF `#[serde(default)]` is dropped"*. It was RUN:
/// the attribute was removed from `display_name` and **all five tests still passed**.
/// ⇒ **serde already defaults a MISSING `Option<T>` field to `None`**; the attribute is
/// belt-and-braces on the `Option` fields of this struct and is load-bearing only on its
/// non-`Option` ones (`devices: Vec<_>`, `created_unix: u64`, …). Measured in-tree, not
/// recalled. The claim was wrong; the test is not.
///
/// ⚠ THE ARM IT ACTUALLY HAS, and it was run and observed RED: adding
/// `skip_serializing_if = "Option::is_none"` stops the key being written, the vacuity guard
/// below fires, and **exactly this test** goes red. That guard is the point — it is what stops
/// the strip from silently becoming a no-op and the arm from passing on an empty edit.
///
/// ⚠ MUST GO RED IF: the key stops being serialised, or a record lacking it stops loading.
/// The load half is the upgrade path for every existing user; nothing else in the suite
/// covers it.
#[test]
fn a_legacy_record_without_the_display_name_key_still_loads() {
    let _g = guard();
    let _cfg = profile_with_contact("legacy", "dana");

    // The key is written today — assert it, or the strip proves nothing.
    let raw = qsc::vault::secret_get("contacts.json")
        .expect("read the contacts secret")
        .expect("the contacts secret exists");
    assert!(
        raw.contains("display_name"),
        "this build must serialise the key, or stripping it is a no-op and this arm is          vacuous: {raw}"
    );

    // Strip it, structurally, to produce the blob a pre-NA-0764 build would have written.
    let mut doc: serde_json::Value = serde_json::from_str(&raw).expect("contacts json parses");
    let peers = doc
        .get_mut("peers")
        .and_then(|p| p.as_object_mut())
        .expect("the store has a peers map");
    for (_alias, rec) in peers.iter_mut() {
        rec.as_object_mut()
            .expect("each record is an object")
            .remove("display_name")
            .expect("the key was present to remove");
    }
    let stripped = serde_json::to_string(&doc).expect("re-serialise");
    assert!(
        !stripped.contains("display_name"),
        "the planted blob must carry no display_name key at all"
    );
    qsc::vault::secret_set("contacts.json", &stripped).expect("plant the legacy blob");

    // THE ARM: it must load, and present typed absence.
    let rows = facade::contact_list().expect(
        "a record with no display_name key must LOAD — if this errors, serde(default) is gone          and every upgrading user's contacts fail to deserialise",
    );
    assert_eq!(
        rows.len(),
        1,
        "the planted store still holds its one contact"
    );
    assert_eq!(
        rows[0].display_name, None,
        "an absent key presents as None, never Some(\"\")"
    );
    qsc::set_vault_unlocked(false);
}

// ───────────────────────────── P-2. device_count ─────────────────────────────

/// **P-2 B1 + B2.** The count is a real projection of the record's own device list, and the
/// ARRAY never reaches the surface.
///
/// ⚠ MUST GO RED IF: `device_count` becomes a constant, or the device array is exposed. The
/// count is proven non-constant by MOVING it — a second device is added and the number must
/// follow. A seal that only ever observes one value cannot tell a projection from a literal.
#[test]
fn device_count_is_a_live_projection_and_the_array_never_escapes() {
    let _g = guard();
    let cfg = profile_with_contact("devices", "dana");

    let one = row("dana").device_count;
    assert!(
        one >= 1,
        "a contact with a pin has at least its primary device, got {one}"
    );

    // Move it. A constant cannot follow.
    run_ok(
        &cfg,
        &[
            "contacts",
            "device",
            "add",
            "--label",
            "dana",
            "--fp",
            FP_B,
            "--route-token",
            "na0764_dana_device2_route_token_ij",
        ],
    );
    let two = row("dana").device_count;
    assert_eq!(
        two,
        one + 1,
        "device_count must FOLLOW the record's device list — {one} -> {two} after adding one \
         device. If this holds at a constant, the field is a literal, not a projection"
    );

    // And the identifiers stay off the surface.
    let rendered = format!("{:?}", row("dana"));
    assert!(
        !rendered.contains(FP_B),
        "a device fingerprint must never appear on the summary: {rendered}"
    );
    for (banned, why) in REFUSED_FIELDS {
        assert!(
            !rendered.contains(&format!("{banned}:")),
            "`{banned}` must not be rendered on the summary ({why}): {rendered}"
        );
    }
    qsc::set_vault_unlocked(false);
}
