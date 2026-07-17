// NA-0649 (D585): library-level coverage for the qsc GUI-surface lane — B1
// `vault_init_with_passphrase`, B2 widened identity data accessors, B3
// `identity_ensure`. This file is the external-crate-shaped consumer D585 test 3 asks
// for: an integration test linking qsc as an external crate and touching ONLY the pub
// library surface (no compiled-binary invocation here; the CLI byte-identity
// spot-check — D585 test 4 — is lane-run evidence, the NA-0646 prover pattern reduced
// to the touched neighborhoods).
//
// Process-global state (env vars, the process passphrase, the unlocked flag, marker
// routing/queue) is shared across tests in this binary, so every test serializes on
// ENV_LOCK and resets that state up front.

use qsc::identity::{
    format_verification_code_from_fingerprint, identity_ensure, identity_fingerprint_from_identity,
    identity_read_self_public,
};
use qsc::model::ErrorCode;
use qsc::output::{marker_queue, set_marker_routing, MarkerRouting};
use qsc::vault::{
    has_process_passphrase, secret_get, set_process_passphrase, unlock_with_passphrase,
    vault_init_with_passphrase,
};
use qsc::{identity_show, set_vault_unlocked, vault_unlocked};
use std::fs;
use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard, OnceLock};

const PASS_A: &str = "na0649-lane-pass-a";
const DEFAULT_INBOX_ROUTE_TOKEN_KEY: &str = "tui.relay.inbox_token";

static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn env_lock() -> MutexGuard<'static, ()> {
    ENV_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn ensure_dir_700(path: &PathBuf) {
    fs::create_dir_all(path).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
    }
}

fn safe_test_root() -> PathBuf {
    let root = if let Ok(v) = std::env::var("QSC_TEST_ROOT") {
        PathBuf::from(v)
    } else if let Ok(v) = std::env::var("CARGO_TARGET_DIR") {
        PathBuf::from(v)
    } else {
        PathBuf::from("target")
    };
    let root = root.join("qsc-test-tmp").join("na0649-gui-surface");
    ensure_dir_700(&root);
    root
}

/// Point QSC_CONFIG_DIR at a FRESH per-test config dir and reset every piece of
/// process-global state the lane surface can touch. Returns the config dir path
/// (not yet created — `vault init` creates it, exactly like the CLI first run).
fn fresh_test_env(tag: &str) -> PathBuf {
    let case_root = safe_test_root().join(format!("{}_{}", tag, std::process::id()));
    if case_root.exists() {
        fs::remove_dir_all(&case_root).unwrap();
    }
    ensure_dir_700(&case_root);
    let cfg = case_root.join("cfg");
    std::env::set_var("QSC_CONFIG_DIR", &cfg);
    std::env::set_var("QSC_DISABLE_KEYCHAIN", "1");
    std::env::remove_var("QSC_MARK_FORMAT");
    set_process_passphrase(None);
    set_vault_unlocked(false);
    set_marker_routing(MarkerRouting::InApp);
    drain_markers();
    cfg
}

fn drain_markers() -> Vec<String> {
    marker_queue()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .drain(..)
        .collect()
}

fn marker_value<'a>(lines: &'a [String], event: &str, key: &str) -> Option<&'a str> {
    let event_kv = format!("event={}", event);
    let key_prefix = format!("{}=", key);
    lines
        .iter()
        .find(|l| l.contains(&event_kv))
        .and_then(|l| {
            l.split_whitespace()
                .find(|part| part.starts_with(&key_prefix))
        })
        .map(|part| &part[key.len() + 1..])
}

// ---------------------------------------------------------------------------
// D585 test group 1 — vault_init_with_passphrase
// ---------------------------------------------------------------------------

#[test]
fn vault_init_with_passphrase_roundtrip_no_unlock_side_effect() {
    let _g = env_lock();
    let cfg = fresh_test_env("roundtrip");

    assert!(!has_process_passphrase());
    assert!(!vault_unlocked());

    vault_init_with_passphrase(PASS_A).expect("in-process vault init");
    assert!(cfg.join("vault.qsv").is_file(), "vault envelope on disk");

    // NO unlock side effect: init alone must leave both unlock-state globals unset.
    assert!(
        !has_process_passphrase(),
        "init must not set the process passphrase"
    );
    assert!(!vault_unlocked(), "init must not set the unlocked flag");

    // The success path emits exactly the existing `vault_init` marker.
    let lines = drain_markers();
    assert!(
        lines.iter().any(|l| l.contains("event=vault_init")),
        "vault_init success marker expected, got: {:?}",
        lines
    );

    // A wrong passphrase must not open it; the real one must.
    assert_eq!(
        unlock_with_passphrase("not-the-passphrase"),
        Err("vault_locked")
    );
    assert!(!has_process_passphrase());
    unlock_with_passphrase(PASS_A).expect("unlock opens the lane-created vault");
    assert!(has_process_passphrase());

    // Same default inbox route-token seeding as the CLI path: present and non-empty.
    let token = secret_get(DEFAULT_INBOX_ROUTE_TOKEN_KEY)
        .expect("vault readable after unlock")
        .expect("default inbox route token seeded at init");
    assert!(!token.is_empty());

    set_process_passphrase(None);
}

#[test]
fn vault_init_with_passphrase_second_call_vault_exists() {
    let _g = env_lock();
    fresh_test_env("exists");

    vault_init_with_passphrase(PASS_A).expect("first init");
    assert_eq!(
        vault_init_with_passphrase(PASS_A),
        Err("vault_exists"),
        "second init must fail closed with the existing error code as a value"
    );
    assert!(!has_process_passphrase());
    assert!(!vault_unlocked());
}

#[test]
fn vault_init_with_passphrase_empty_passphrase_rejected() {
    let _g = env_lock();
    let cfg = fresh_test_env("empty");

    assert_eq!(
        vault_init_with_passphrase(""),
        Err("vault_passphrase_required")
    );
    assert!(!cfg.exists(), "no mutation on reject");
}

// ---------------------------------------------------------------------------
// D585 test group 2 — identity_ensure
// ---------------------------------------------------------------------------

#[test]
fn identity_ensure_creates_then_is_idempotent() {
    let _g = env_lock();
    let cfg = fresh_test_env("ensure");

    vault_init_with_passphrase(PASS_A).expect("init");
    unlock_with_passphrase(PASS_A).expect("unlock");
    drain_markers();

    let rec = identity_ensure("default").expect("fresh unlocked store creates");
    assert!(!rec.kem_pk.is_empty());
    assert!(!rec.sig_pk.is_empty());
    let fp = identity_fingerprint_from_identity(&rec.kem_pk, &rec.sig_pk);

    let record_path = cfg.join("identities").join("self_default.json");
    let bytes_after_create = fs::read(&record_path).expect("public record written");

    // The fingerprint matches a subsequent `identity_show` marker.
    drain_markers();
    identity_show("default").expect("identity_show");
    let lines = drain_markers();
    assert_eq!(
        marker_value(&lines, "identity_show", "fp"),
        Some(fp.as_str()),
        "identity_show marker fp must match the ensured record, got: {:?}",
        lines
    );

    // Second call returns the SAME record with no mutation.
    let rec2 = identity_ensure("default").expect("existing record returned");
    assert_eq!(rec2.kem_pk, rec.kem_pk);
    assert_eq!(rec2.sig_pk, rec.sig_pk);
    let bytes_after_second = fs::read(&record_path).expect("public record still there");
    assert_eq!(
        bytes_after_second, bytes_after_create,
        "store bytes stable across the idempotent second call"
    );

    set_process_passphrase(None);
}

#[test]
fn identity_ensure_locked_store_keeps_existing_vault_locked_behavior() {
    let _g = env_lock();
    let cfg = fresh_test_env("locked");

    vault_init_with_passphrase(PASS_A).expect("init");
    // Deliberately NOT unlocked: the underlying lazy path must fail exactly as today.
    drain_markers();

    let err = match identity_ensure("default") {
        Err(e) => e,
        Ok(_) => panic!("locked store must fail closed"),
    };
    assert!(
        matches!(err, ErrorCode::IdentitySecretUnavailable),
        "expected IdentitySecretUnavailable, got {:?}",
        err
    );
    assert_eq!(err.as_str(), "identity_secret_unavailable");

    let lines = drain_markers();
    assert!(
        lines
            .iter()
            .any(|l| l.contains("event=identity_secret_unavailable") && l.contains("vault_locked")),
        "the existing vault_locked marker behavior expected, got: {:?}",
        lines
    );
    assert!(
        !cfg.join("identities").join("self_default.json").exists(),
        "no identity record minted on the locked path"
    );
}

#[test]
fn identity_ensure_preserves_second_identity_guard() {
    let _g = env_lock();
    fresh_test_env("guard");

    vault_init_with_passphrase(PASS_A).expect("init");
    unlock_with_passphrase(PASS_A).expect("unlock");
    identity_ensure("default").expect("first identity");
    drain_markers();

    let err = match identity_ensure("other") {
        Err(e) => e,
        Ok(_) => panic!("second label must be refused"),
    };
    assert!(
        matches!(err, ErrorCode::IdentitySelfAmbiguous),
        "the NA-0616 guard must fire unchanged, got {:?}",
        err
    );
    let lines = drain_markers();
    assert!(
        lines
            .iter()
            .any(|l| l.contains("event=identity_self_ambiguous")),
        "identity_self_ambiguous marker expected, got: {:?}",
        lines
    );

    set_process_passphrase(None);
}

// ---------------------------------------------------------------------------
// D585 test group 3 — widened accessors as an external-crate consumer
// ---------------------------------------------------------------------------

#[test]
fn widened_accessors_expose_identity_as_data() {
    let _g = env_lock();
    fresh_test_env("accessors");

    vault_init_with_passphrase(PASS_A).expect("init");
    unlock_with_passphrase(PASS_A).expect("unlock");
    let created = identity_ensure("default").expect("identity present");

    // {fp, kem_pk, sig_pk} as values, straight from the widened accessor.
    let rec = identity_read_self_public("default")
        .expect("accessor readable")
        .expect("record present");
    assert_eq!(rec.kem_pk, created.kem_pk);
    assert_eq!(rec.sig_pk, created.sig_pk);
    let fp = identity_fingerprint_from_identity(&rec.kem_pk, &rec.sig_pk);
    assert!(fp.starts_with("QSCFP-"));

    // The verification code derives deterministically and keeps its shape.
    let code = format_verification_code_from_fingerprint(&fp);
    assert_eq!(code, format_verification_code_from_fingerprint(&fp));
    let groups: Vec<&str> = code.split('-').collect();
    assert_eq!(groups.len(), 5, "4 groups + checksum, got {}", code);
    assert!(groups[..4].iter().all(|g| g.len() == 4));
    assert_eq!(groups[4].len(), 1);
    assert!(code.chars().all(|c| c == '-' || c.is_ascii_alphanumeric()));

    // Cross-check the fp against the `identity_show` marker output.
    drain_markers();
    identity_show("default").expect("identity_show");
    let lines = drain_markers();
    assert_eq!(
        marker_value(&lines, "identity_show", "fp"),
        Some(fp.as_str())
    );

    // ErrorCode is nameable and matchable from an external crate (the D585 scope
    // amendment's purpose): an invalid label surfaces as a value we can match on.
    match identity_read_self_public("bad label!") {
        Err(ErrorCode::ParseFailed) => {}
        other => panic!("expected Err(ParseFailed), got {:?}", other.map(|_| ())),
    }

    set_process_passphrase(None);
}
