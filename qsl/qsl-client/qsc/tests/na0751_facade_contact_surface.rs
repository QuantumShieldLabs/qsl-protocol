//! NA-0751 (D-1393) — `W8` FINGERPRINT ABSENCE and `W9` PIN/DISPLAY IDENTITY, plus the
//! contact-request verbs' typed `NotFound` arm.
//!
//! ⚠ WHY `W8` IS DRIVEN THROUGH `contacts_add` AND NOT THROUGH A FRESHLY-ACCEPTED REQUEST —
//! measured, and stated because `R373` §2(a) asks for the latter. `contacts_request_accept`
//! requires a request to ALREADY EXIST (`contacts/mod.rs:1567-1571`: `contact_request_remove`
//! must return true, else `request_unknown`), and the only writer of that store is
//! `contact_request_upsert` (`contacts/mod.rs:430`), which is `pub(super)` and is called from
//! exactly ONE place in the crate — `transport/mod.rs:1236`, on a relay RECEIVE. There is no
//! `pub` path and no CLI path, so "freshly accepted" cannot be driven from an integration test
//! without a second party — the same class `R373` §2(d) filed for redeem/accept/finish.
//! ⇒ The STATE `W8` is about is driven exactly: `ContactRecord.fp == "UNSET"`, which is the
//! literal the accept path writes (`contacts/mod.rs:1572`). `contacts_add` reaches it because
//! it applies NO length check to `fp` when no keys are supplied — which is `ENG-0209`, still
//! open, and is the reason this route exists at all. The PATH differs; the state does not.
//! The `contacts_request_accept` verb itself IS exercised below, on its `NotFound` arm.

use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::{env, fs};

use qsc::facade::{
    contact_list, contact_request_accept, contact_request_block, contact_request_ignore,
    contact_requests, ContactState, ContactSummary, FacadeError,
};

const PASS: &str = "correct horse battery staple";
/// A real 64-hex fingerprint — the shape `identity_voice_form` requires and the 64-hex guard admits.
const REAL_FP: &str = "4cb507ef6c16056799ef559de1998dd7e5e3f735e50659495495725c5b62ad98";
/// 22..=128 chars of [A-Za-z0-9_-] — `adversarial/route.rs:24`. The auto-mint path was RETIRED
/// (D616 §2m), so `contacts_add` now REQUIRES one: `contacts_route_token_required`.
const ROUTE_TOKEN: &str = "na0751-route-token-abcdefghij";

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
        .join(format!("na0751_contacts_{tag}_{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    dir700(&dir);
    dir
}

/// A real vault, created and unlocked in-process, pointed at a fresh config dir.
fn open_vault(tag: &str) -> PathBuf {
    let cfg = fresh(tag);
    env::set_var("QSC_CONFIG_DIR", &cfg);
    env::set_var("QSC_QSP_SEED", "1");
    qsc::vault::vault_init_with_passphrase(PASS).expect("vault init");
    qsc::vault::unlock_with_passphrase(PASS).expect("unlock");
    qsc::set_vault_unlocked(true);
    cfg
}

fn only(list: Vec<ContactSummary>, alias: &str) -> ContactSummary {
    list.into_iter()
        .find(|c| c.alias == alias)
        .unwrap_or_else(|| panic!("{alias} not in the contact list"))
}

// ── W8 ───────────────────────────────────────────────────────────────────────────────────

#[test]
fn na0751_w8_a_contact_without_a_64_hex_fingerprint_lists_as_typed_absence() {
    let _g = guard();
    let _cfg = open_vault("w8");

    // The exact state the accept path writes: `ContactRecord.fp == "UNSET"`.
    qsc::contacts::contacts_add("peer-unset", "UNSET", None, None, Some(ROUTE_TOKEN), false)
        .expect("contacts_add with the UNSET sentinel");

    let row = only(contact_list().expect("contact_list"), "peer-unset");
    // ⛳ THE SEAL: typed ABSENCE. Never `Some("")`, never the literal word on a pub surface.
    assert!(
        row.fingerprint.is_none(),
        "a non-64-hex fp must yield None, got {:?}",
        row.fingerprint
    );
    // ⚠ MEASURED, not assumed: `contacts_add(.., verify = false)` stores `status = "pinned"`
    // (`contacts/mod.rs:947`), so `contact_state()` reports PINNED — the trust STATE and the
    // FINGERPRINT are independent, and this row proves it: PINNED with no displayable
    // fingerprint at all. `pinned` here is the FP-resolution fact, which is false because
    // `identity_read_pin` refuses the UNSET sentinel.
    assert_eq!(row.state, ContactState::Pinned, "contacts_add pins by default");
    assert!(
        !row.pinned,
        "but the FP resolution refuses UNSET, so there is no pinned fingerprint"
    );
    assert!(!row.blocked);
    qsc::set_vault_unlocked(false);
}

#[test]
fn na0751_w8_mutation_control_the_guard_is_what_produces_the_absence() {
    let _g = guard();
    let _cfg = open_vault("w8ctl");
    qsc::contacts::contacts_add("peer-unset", "UNSET", None, None, Some(ROUTE_TOKEN), false).expect("add");

    // The facade's guard is `fp.len() == 64 && all ascii-hex`. Reproduce it here and show that
    // REMOVING it is what turns the honest `None` into the `Some` with an EMPTY voice that
    // `identity_voice_form`'s own doc warns a caller must refuse.
    let resolved = "UNSET";
    let with_guard = resolved.len() == 64 && resolved.bytes().all(|b| b.is_ascii_hexdigit());
    assert!(!with_guard, "the guard refuses UNSET");
    let voice_if_unguarded = qsc::identity::identity_voice_form(resolved);
    assert_eq!(
        voice_if_unguarded, "",
        "without the guard the published voice tier would be the empty sentinel"
    );
    // And the shipped surface does not do that.
    let row = only(contact_list().expect("contact_list"), "peer-unset");
    assert!(row.fingerprint.is_none());
    qsc::set_vault_unlocked(false);
}

// ── W9 ───────────────────────────────────────────────────────────────────────────────────

#[test]
fn na0751_w9_the_displayed_fingerprint_is_the_one_the_pin_comparison_consumes() {
    let _g = guard();
    let _cfg = open_vault("w9");
    qsc::contacts::contacts_add("peer-pinned", REAL_FP, None, None, Some(ROUTE_TOKEN), false)
        .expect("contacts_add with a real 64-hex fp");

    let row = only(contact_list().expect("contact_list"), "peer-pinned");
    let pair = row.fingerprint.as_ref().expect("a 64-hex fp yields Some");

    // ⛳ THE SEAL: the displayed full form EQUALS the exact string the pin comparison consumes.
    // `identity_peer_status` (`lib.rs:242`) is `identity_read_pin` plus an "untrusted"
    // placeholder, so its `.0` on a pinned contact IS that string.
    let (pin_consumes, pinned) = qsc::identity_peer_status("peer-pinned");
    assert!(pinned, "the contact is pinned");
    assert_eq!(
        pair.full, pin_consumes,
        "displayed full must BE the pinned string, not merely resemble it"
    );
    assert_eq!(pair.full, REAL_FP);
    assert_eq!(row.pinned, pinned, "one resolution feeds both fields");

    // The voice tier is derived from that same string, and is non-empty for a 64-hex input.
    assert_eq!(pair.voice, qsc::identity::identity_voice_form(REAL_FP));
    assert!(!pair.voice.is_empty(), "a 64-hex fp has a real voice form");

    // MUTATION CONTROL: a DIFFERENT fingerprint must not compare equal — proving the assertion
    // above is an identity check and not a tautology over two calls to the same function.
    //
    // ⚠ THE MUTATED BYTE MUST BE AN EARLY ONE, and finding that out is itself a result. Flipping
    // the LAST hex character leaves the voice form IDENTICAL: the voice tier is a 30-decimal-digit
    // reduction of the FIRST 20 BYTES (≈100 bits, `10^30 ≈ 2^99.7`), so the trailing 12 bytes of
    // the 32-byte fingerprint do not reach it. That is the ratified design, not a defect — the
    // FULL 64-hex form is what carries the whole value — but a control that mutates a byte
    // outside the voice form's input proves nothing about the voice form.
    let tail_only = format!("{}0", &REAL_FP[..63]);
    assert_ne!(tail_only, REAL_FP, "the tail-mutated value really differs");
    assert_eq!(
        qsc::identity::identity_voice_form(&tail_only),
        pair.voice,
        "a trailing-byte change does NOT reach the voice tier — stated, because it bounds what \
         the read-aloud form can distinguish"
    );
    let early = format!("0{}", &REAL_FP[1..]);
    assert_ne!(early, pin_consumes, "the control's other value really differs");
    assert_ne!(
        qsc::identity::identity_voice_form(&early),
        pair.voice,
        "an early-byte change DOES yield a different voice form"
    );
    qsc::set_vault_unlocked(false);
}

// ── the contact-request verbs ────────────────────────────────────────────────────────────

#[test]
fn na0751_the_request_verbs_accept_and_ignore_are_not_found_and_block_is_not() {
    let _g = guard();
    let _cfg = open_vault("verbs");

    // `contacts_request_accept` (:1565) and `_ignore` (:1613) both REMOVE the request first and
    // return `request_unknown` when there was none. The facade maps that to `NotFound`, NOT to
    // `StoreUnavailable` — which is the inversion the second read sustained as `D1`.
    for (name, r) in [
        ("accept", contact_request_accept("nobody")),
        ("ignore", contact_request_ignore("nobody")),
    ] {
        match r {
            Err(FacadeError::NotFound) => {}
            other => panic!("{name} on an unknown alias must be NotFound, got {other:?}"),
        }
    }

    // ⚠ `block` IS DIFFERENT, AND THIS TEST EXISTS PARTLY TO PIN THAT. It does NOT require an
    // existing request: on an unknown alias it CONSTRUCTS a record with `status: "REVOKED"` and
    // `blocked: true` (`contacts/mod.rs:1628-1646`) and returns Ok. Asserting it symmetrically
    // with the other two would have been asserting something the tree does not do.
    contact_request_block("stranger").expect("block succeeds on an unknown alias");
    let row = only(contact_list().expect("contact_list"), "stranger");
    assert!(row.blocked, "block sets the blocked flag");
    // ⛳ AND THE LEGACY-FIELD DIVERGENCE `M5` FOUND IS VISIBLE HERE: the record's raw `status`
    // is "REVOKED", but `contact_state()` — the value the CLI renders and the badge means —
    // maps REVOKED to CHANGED. The facade ships `contact_state`'s value, not the raw field.
    assert_eq!(
        row.state,
        ContactState::Changed,
        "the surface reports contact_state()'s CHANGED, not the raw REVOKED"
    );
    assert!(row.fingerprint.is_none(), "a blocked stranger has no pinned fingerprint");

    // `contact_requests`' SUCCESS payload, inspected rather than merely called.
    let reqs = contact_requests().expect("contact_requests");
    assert!(reqs.is_empty(), "a fresh store has no pending requests");
    qsc::set_vault_unlocked(false);
}
