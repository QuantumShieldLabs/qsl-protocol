//! NA-0751 (D-1393) — W2 FACT FIDELITY and W7 ERROR-MAPPING TOTALITY.
//!
//! The file guard is MANDATORY here: marker routing and the marker queue are process-wide
//! statics (`output/mod.rs:26` `MARKER_ROUTING`, `:27` `MARKER_QUEUE`), and cargo runs a
//! binary's tests in parallel threads.

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

fn test_root(tag: &str) -> PathBuf {
    let root = env::var("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("target"));
    let dir = root
        .join("qsc-test-tmp")
        .join(format!("na0751_fidelity_{tag}_{}", std::process::id()));
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

// ── W2 ───────────────────────────────────────────────────────────────────────────────────

#[test]
fn na0751_w2_the_typed_connect_read_equals_the_string_route_on_the_same_state() {
    let _g = guard();
    // TWO INSTRUMENTS on one claim: the facade's typed read, and `qsp_status_tuple`'s own
    // strings, on the SAME driven state. Equality on EXTRACTED VALUES, never `contains`.
    let cfg = test_root("w2_no_session");
    env::set_var("QSC_CONFIG_DIR", &cfg);
    env::set_var("QSC_QSP_SEED", "1");

    let typed = connect_status("peer-0");
    let (raw_state, raw_reason) = qsc::protocol_state::qsp_status_tuple("peer-0");

    assert_eq!(typed.reason.as_wire(), raw_reason, "typed reason == string reason");
    assert_eq!(
        match typed.state {
            ConnectState::Active => "ACTIVE",
            ConnectState::Inactive => "INACTIVE",
        },
        raw_state,
        "typed state == string state"
    );
}

#[test]
fn na0751_w2_the_carve_out_is_exactly_one_arm() {
    let _g = guard();
    // The ONE written divergence: locked AND `session_invalid` -> `VaultLocked`. Every other
    // reason must agree with the string route even while locked, because the first five are
    // decided before any vault secret is touched.
    let cfg = test_root("w2_carveout");
    env::set_var("QSC_CONFIG_DIR", &cfg);
    env::set_var("QSC_QSP_SEED", "1");
    qsc::set_vault_unlocked(false);

    // no_session while locked: NOT overridden.
    let typed = connect_status("peer-0");
    let (_, raw) = qsc::protocol_state::qsp_status_tuple("peer-0");
    assert_eq!(typed.reason, ConnectReason::NoSession);
    assert_eq!(typed.reason.as_wire(), raw, "no divergence on this arm");

    // channel_invalid while locked: NOT overridden.
    let typed = connect_status("bad peer");
    let (_, raw) = qsc::protocol_state::qsp_status_tuple("bad peer");
    assert_eq!(typed.reason, ConnectReason::ChannelInvalid);
    assert_eq!(typed.reason.as_wire(), raw, "no divergence on this arm");
}

// ⚠ W7(a)(b)(c) — the ERROR-MAPPING seals — are NOT here. `map_code` is PRIVATE to the facade,
// and the honest instrument for a private function is a same-file `#[cfg(test)]` module (the
// tree's own `confirm_capture_reason_tests` precedent). Putting them here would have required
// widening the mapping to `pub`, which is public surface the sealed type surface does not
// carry. They live in `src/facade/mod.rs`'s test module and add ZERO test binaries, so the
// shard arithmetic stays +4.
