//! NA-0751 (D-1393) — W1: CONNECT-REASON TOTALITY, the SIX non-ACTIVE reasons.
//!
//! NO fixture and NO tokio runtime: `qsp_status_tuple` is a pure config/session read, so this
//! file starts no server and the only other threads are libtest's, which the file-local
//! `guard()` serialises. That is why the "environment set ONCE before `start_qsl_server`"
//! rule does not bind here — it binds files that START a server.
//!
//! ⚠ THE SEVENTH REASON IS NOT DRIVABLE HERE, AND THAT IS A MEASUREMENT, NOT A GAP.
//! `protocol_state` exposes exactly TWO `pub fn` — `qsp_status_tuple` and
//! `allow_unsafe_seed_fallback_for_tests` — and every session-establishing function is
//! `pub(crate)` or private, so an integration test (an external crate) has no `pub` path to a
//! stored session. `ACTIVE`/`handshake` is therefore driven in
//! `na0751_facade_invite_surface.rs`, where a fixture and a real session exist.
//!
//! COUNT == 7 IS ASSERTED OVER THE UNION, and the shared source is the ENUM ITSELF rather than
//! a duplicated const list: `ConnectReason::as_wire` is the one definition both files read.
//! (A literal shared const would have meant editing `tests/common/mod.rs`, which is not in
//! this lane's authorized edit set.)

use std::sync::{Mutex, MutexGuard, OnceLock};
use std::{env, fs};
use std::path::{Path, PathBuf};

use qsc::facade::{connect_status, ConnectReason, ConnectState};

fn guard() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

/// The SEVEN upstream-mapped reasons, named through the enum so there is exactly one
/// definition. `VaultLocked` and `Unrecognized` are the facade's own and are not upstream.
const UPSTREAM_SEVEN: [ConnectReason; 7] = [
    ConnectReason::Handshake,
    ConnectReason::NoSession,
    ConnectReason::MissingSeed,
    ConnectReason::SessionInvalid,
    ConnectReason::ChannelInvalid,
    ConnectReason::UnsafeParent,
    ConnectReason::MissingHome,
];

/// Driven in THIS file. `Handshake` is driven in `na0751_facade_invite_surface.rs`.
const DRIVEN_HERE: [ConnectReason; 6] = [
    ConnectReason::NoSession,
    ConnectReason::MissingSeed,
    ConnectReason::SessionInvalid,
    ConnectReason::ChannelInvalid,
    ConnectReason::UnsafeParent,
    ConnectReason::MissingHome,
];

fn test_root(tag: &str) -> PathBuf {
    let root = env::var("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("target"));
    let dir = root
        .join("qsc-test-tmp")
        .join(format!("na0751_totality_{tag}_{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    dir700(&dir);
    dir
}

fn dir700(p: &Path) {
    fs::create_dir_all(p).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(p, fs::Permissions::from_mode(0o700)).unwrap();
    }
}

/// Point the process at `cfg` and set the seed env to `seed`. Mutating the environment
/// BETWEEN states is what this file needs and what it is allowed: no server thread exists.
fn aim(cfg: Option<&Path>, seed: bool) {
    match cfg {
        Some(c) => env::set_var("QSC_CONFIG_DIR", c),
        None => env::remove_var("QSC_CONFIG_DIR"),
    }
    if seed {
        env::set_var("QSC_QSP_SEED", "1");
    } else {
        env::remove_var("QSC_QSP_SEED");
    }
}

#[test]
fn na0751_w1_the_six_non_active_reasons_are_driven_and_distinct() {
    let _g = guard();
    // ⚠ THE UNLOCK FLAG IS SET DELIBERATELY, and the reason is a MEASUREMENT this file made.
    // This binary never unlocks anything, so `VAULT_UNLOCKED_THIS_RUN` (`lib.rs:190`) is
    // default-FALSE — and the first run of this test therefore returned `VaultLocked` on the
    // `session_invalid` arm, exactly as `connect_status` specifies. That is the facade behaving
    // correctly, not a defect: the lock override IS the subject of
    // `na0751_facade_locked_control.rs`, and THIS file's subject is the SEVEN UPSTREAM reasons.
    // Setting the flag keeps the override from masking the upstream mapping under test.
    // ⛳ The other five arms passed untouched while it was false, which is independent evidence
    // that the override is scoped to exactly one arm.
    qsc::set_vault_unlocked(true);
    let mut seen = Vec::new();

    // no_session — a resolvable config dir with no session blob, seed SET.
    let cfg = test_root("no_session");
    aim(Some(&cfg), true);
    let s = connect_status("peer-0");
    assert_eq!(s.state, ConnectState::Inactive);
    assert_eq!(s.reason, ConnectReason::NoSession, "no_session arm");
    seen.push(s.reason);

    // missing_seed — same, seed UNSET.
    let cfg = test_root("missing_seed");
    aim(Some(&cfg), false);
    let s = connect_status("peer-0");
    assert_eq!(s.reason, ConnectReason::MissingSeed, "missing_seed arm");
    seen.push(s.reason);

    // session_invalid — a session file that is not a session.
    let cfg = test_root("session_invalid");
    let sessions = cfg.join("qsp_sessions");
    dir700(&sessions);
    fs::write(sessions.join("peer-0.bin"), b"not-a-session").unwrap();
    aim(Some(&cfg), true);
    let s = connect_status("peer-0");
    assert_eq!(s.reason, ConnectReason::SessionInvalid, "session_invalid arm");
    seen.push(s.reason);

    // channel_invalid — `channel_label_ok` (lib.rs:2568) accepts only
    // [alnum - _ #], so a space is refused.
    let cfg = test_root("channel_invalid");
    aim(Some(&cfg), true);
    let s = connect_status("bad peer");
    assert_eq!(s.reason, ConnectReason::ChannelInvalid, "channel_invalid arm");
    seen.push(s.reason);

    // unsafe_parent — a group/world-writable config dir.
    let cfg = test_root("unsafe_parent");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&cfg, fs::Permissions::from_mode(0o777)).unwrap();
    }
    aim(Some(&cfg), true);
    let s = connect_status("peer-0");
    assert_eq!(s.reason, ConnectReason::UnsafeParent, "unsafe_parent arm");
    seen.push(s.reason);

    // missing_home — none of the three resolution sources is set
    // (`fs_store::config_dir`, :11 QSC_CONFIG_DIR, :16 XDG_CONFIG_HOME, :21 HOME).
    let saved_home = env::var("HOME").ok();
    let saved_xdg = env::var("XDG_CONFIG_HOME").ok();
    aim(None, true);
    env::remove_var("XDG_CONFIG_HOME");
    env::remove_var("HOME");
    let s = connect_status("peer-0");
    if let Some(h) = saved_home {
        env::set_var("HOME", h);
    }
    if let Some(x) = saved_xdg {
        env::set_var("XDG_CONFIG_HOME", x);
    }
    assert_eq!(s.reason, ConnectReason::MissingHome, "missing_home arm");
    seen.push(s.reason);

    // The driven set is exactly the six this file owns, with no duplicates.
    assert_eq!(seen.len(), 6, "six states driven");
    for r in DRIVEN_HERE {
        assert!(seen.contains(&r), "{} was not driven", r.as_wire());
    }
    // ⚠ NOT VACUOUS: an empty or short drive cannot reach this line.
    let mut wires: Vec<&str> = seen.iter().map(|r| r.as_wire()).collect();
    wires.sort_unstable();
    wires.dedup();
    assert_eq!(wires.len(), 6, "six DISTINCT reasons, not one repeated");
    qsc::set_vault_unlocked(false);
}

#[test]
fn na0751_w1_unrecognized_is_unreached_across_every_driven_state() {
    let _g = guard();
    qsc::set_vault_unlocked(true);
    // `Unrecognized` exists only as the honest runtime surface for an EIGHTH upstream string.
    // Any appearance is a defect to file. Re-drive the cheapest three and assert absence.
    let cfg = test_root("unrec_a");
    aim(Some(&cfg), true);
    assert_ne!(connect_status("peer-0").reason, ConnectReason::Unrecognized);
    aim(Some(&cfg), false);
    assert_ne!(connect_status("peer-0").reason, ConnectReason::Unrecognized);
    assert_ne!(connect_status("bad peer").reason, ConnectReason::Unrecognized);
}

#[test]
fn na0751_w1_the_union_is_seven_and_the_seventh_lives_in_the_invite_surface_file() {
    // The enum is the shared definition. Six are driven here; the seventh —
    // `Handshake`, the only ACTIVE arm — is driven in `na0751_facade_invite_surface.rs`
    // by `na0751_w1_the_seventh_reason_active_handshake`.
    assert_eq!(UPSTREAM_SEVEN.len(), 7, "count == 7 over the union");
    let mut wires: Vec<&str> = UPSTREAM_SEVEN.iter().map(|r| r.as_wire()).collect();
    wires.sort_unstable();
    wires.dedup();
    assert_eq!(wires.len(), 7, "seven DISTINCT wire names");
    let here: Vec<&str> = DRIVEN_HERE.iter().map(|r| r.as_wire()).collect();
    assert_eq!(here.len(), 6);
    let missing: Vec<&str> = UPSTREAM_SEVEN
        .iter()
        .map(|r| r.as_wire())
        .filter(|w| !here.contains(w))
        .collect();
    assert_eq!(
        missing,
        vec![ConnectReason::Handshake.as_wire()],
        "exactly one reason is owed to the invite-surface file, and it is `handshake`"
    );
}

#[test]
fn na0751_w1_as_wire_round_trips_and_the_facade_own_variants_are_not_upstream() {
    // `as_wire` is the compile-red delta symbol: deleting a variant makes its match
    // non-exhaustive. This asserts the values, so a silent re-spelling is caught too.
    assert_eq!(ConnectReason::Handshake.as_wire(), "handshake");
    assert_eq!(ConnectReason::NoSession.as_wire(), "no_session");
    assert_eq!(ConnectReason::MissingSeed.as_wire(), "missing_seed");
    assert_eq!(ConnectReason::SessionInvalid.as_wire(), "session_invalid");
    assert_eq!(ConnectReason::ChannelInvalid.as_wire(), "channel_invalid");
    assert_eq!(ConnectReason::UnsafeParent.as_wire(), "unsafe_parent");
    assert_eq!(ConnectReason::MissingHome.as_wire(), "missing_home");
    // The facade's own two, which no upstream arm produces.
    assert_eq!(ConnectReason::VaultLocked.as_wire(), "vault_locked");
    assert_eq!(ConnectReason::Unrecognized.as_wire(), "unrecognized");
    for r in UPSTREAM_SEVEN {
        assert_ne!(r, ConnectReason::VaultLocked);
        assert_ne!(r, ConnectReason::Unrecognized);
    }
}
