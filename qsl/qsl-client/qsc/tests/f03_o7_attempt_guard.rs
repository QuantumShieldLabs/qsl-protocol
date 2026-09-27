// NA-0785 F03 S8 (C01 O7): the attempt guard counts ONLY a passphrase-authentication
// failure. C01 T6 O7 makes it a precondition; T2 rows V1, V2, V7 and V8 name the cases.
//
// Every arm opens a product-initialised vault with the CORRECT passphrase (arm vi: the
// empty passphrase) after an on-disk change, with the wipe armed at 1, through ONE
// unlock_guarded_at call. A version or format refusal must return its own unchanged
// code and have no effect: no counter write, no delay, no wipe, vault bytes untouched
// (I03). The two COUNT controls (a wrong passphrase, and C01 V1's foreign ciphertext
// under intact-looking magic and KDF words) must stay counted, so the guard can still
// go red.
//
// Harness: the NA-0658 pattern -- the pub library surface only, every test serialised
// on ENV_LOCK, a fresh QSC_CONFIG_DIR per test under QSC_TEST_ROOT, the clock seam
// taking a fabricated reading. Arms iv and v re-encrypt a hand-built payload under
// Argon2id(PASS, the product vault's own salt) with the 53-byte header as AAD (the
// na0694 construction); a positive self-check proves the builder authenticates first.

use argon2::{Algorithm, Argon2, Params, Version};
use chacha20poly1305::aead::{Aead, KeyInit, Payload};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use qsc::output::{marker_queue, set_marker_routing, MarkerRouting};
use qsc::store::QSC_ERR_VAULT_WIPED_AFTER_FAILED_UNLOCKS;
use qsc::vault::protection::{
    protection_status_at, unlock_guarded_at, wipe_after_failed_unlocks_arm, GuardedUnlockOutcome,
};
use qsc::vault::{
    has_process_passphrase, open_session_with_passphrase, perf_snapshot, set_process_passphrase,
    vault_init_directional_with_passphrase,
};
use qsc::{set_vault_unlocked, vault_unlocked};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard, OnceLock};

const PASS: &str = "f03-o7-attempt-guard-pass";
const WRONG: &str = "f03-o7-attempt-guard-wrong";
const CURRENT_MAGIC: &[u8; 6] = b"QSCV03";
const CONFIG_FILE: &str = "vault_security.txt";
const COUNTER_FILE: &str = "vault_unlock_failures.txt";
const VAULT_FILE: &str = "vault.qsv";
const KDF_M_KIB: u32 = 19456;
const KDF_T: u32 = 2;
const KDF_P: u32 = 1;
const HEADER_LEN: usize = 53;
const T0: u64 = 1_000_000;

static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn env_lock() -> MutexGuard<'static, ()> {
    ENV_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn ensure_dir_700(path: &Path) {
    fs::create_dir_all(path).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
    }
}

fn test_root() -> PathBuf {
    let root = if let Ok(v) = std::env::var("QSC_TEST_ROOT") {
        PathBuf::from(v)
    } else if let Ok(v) = std::env::var("CARGO_TARGET_DIR") {
        PathBuf::from(v)
    } else {
        PathBuf::from("target")
    };
    let root = root.join("qsc-test-tmp").join("f03-o7-attempt-guard");
    ensure_dir_700(&root);
    root
}

fn point_at(cfg: &Path) {
    std::env::set_var("QSC_CONFIG_DIR", cfg);
    std::env::set_var("QSC_DISABLE_KEYCHAIN", "1");
    std::env::remove_var("QSC_MARK_FORMAT");
    set_process_passphrase(None);
    set_vault_unlocked(false);
}

/// A fresh per-test directory with QSC_CONFIG_DIR pointed at `<case>/cfg` and every
/// piece of process-global state the guard can touch reset.
fn fresh_case(tag: &str) -> PathBuf {
    let case = test_root().join(format!("{}_{}", tag, std::process::id()));
    if case.exists() {
        fs::remove_dir_all(&case).unwrap();
    }
    ensure_dir_700(&case);
    let cfg = case.join("cfg");
    point_at(&cfg);
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

fn read_opt(path: &Path) -> Option<Vec<u8>> {
    fs::read(path).ok()
}

fn write_vault(cfg: &Path, bytes: &[u8]) {
    let path = cfg.join(VAULT_FILE);
    fs::write(&path, bytes).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&path, fs::Permissions::from_mode(0o600)).unwrap();
    }
}

/// A product-initialised vault under PASS; returns its bytes.
fn init_good_vault(cfg: &Path) -> Vec<u8> {
    vault_init_directional_with_passphrase(PASS).expect("product vault init");
    let bytes = fs::read(cfg.join(VAULT_FILE)).expect("vault written by init");
    assert_eq!(&bytes[..6], CURRENT_MAGIC, "product-written magic");
    assert_eq!(
        bytes[6], 1,
        "product wrote a passphrase envelope (key_source 1)"
    );
    bytes
}

/// Re-encrypt `plaintext` under Argon2id(PASS, salt of `product`) with the 53-byte
/// header as AAD, keeping the product header's magic, key_source, KDF words and salt.
fn authenticated_envelope(product: &[u8], plaintext: &[u8], nonce: [u8; 12]) -> Vec<u8> {
    let mut salt = [0u8; 16];
    salt.copy_from_slice(&product[25..41]);
    let params = Params::new(KDF_M_KIB, KDF_T, KDF_P, Some(32)).expect("argon2 params");
    let mut key = [0u8; 32];
    Argon2::new(Algorithm::Argon2id, Version::V0x13, params)
        .hash_password_into(PASS.as_bytes(), &salt, &mut key)
        .expect("vault key");
    let mut header = product[..HEADER_LEN].to_vec();
    header[21..25].copy_from_slice(&((plaintext.len() + 16) as u32).to_le_bytes());
    header[41..53].copy_from_slice(&nonce);
    let ciphertext = ChaCha20Poly1305::new(Key::from_slice(&key))
        .encrypt(
            Nonce::from_slice(&nonce),
            Payload {
                msg: plaintext,
                aad: &header,
            },
        )
        .expect("header-bound encrypt");
    key.fill(0);
    let mut out = header;
    out.extend_from_slice(&ciphertext);
    out
}

/// The builder's positive self-check: an admissible payload built the same way opens
/// through the read-only non-guard ingress, so a refusal of the arm's payload is
/// post-AEAD by construction (not a builder fault read as a wrong passphrase).
fn assert_builder_authenticates(cfg: &Path, product: &[u8]) {
    let admissible = br#"{"version":4,"protocol":"NA0780-OWNER-FREE-01","secrets":{}}"#;
    write_vault(
        cfg,
        &authenticated_envelope(product, admissible, [0x11; 12]),
    );
    assert!(
        open_session_with_passphrase(PASS).is_ok(),
        "builder self-check: an admissible hand-built payload must authenticate"
    );
    set_process_passphrase(None);
    set_vault_unlocked(false);
}

/// The REFUSE assertions, in the sealed order A1..A9 (SEALED_EXPECTATION_S8.md).
fn assert_refused_uncounted(arm: &str, cfg: &Path, passphrase: &str, code: &'static str) {
    let counter = cfg.join(COUNTER_FILE);
    let config = cfg.join(CONFIG_FILE);
    let vault = cfg.join(VAULT_FILE);
    let counter_before = read_opt(&counter);
    let config_before = read_opt(&config);
    let vault_before = read_opt(&vault);
    assert!(vault_before.is_some(), "O7 {arm}: setup wrote a vault file");
    let kdf_before = perf_snapshot().0;

    let outcome = unlock_guarded_at(passphrase, T0);
    let kdf_after = perf_snapshot().0;
    eprintln!(
        "F03_O7 arm={arm} outcome={outcome:?} kdf_calls_delta={}",
        kdf_after - kdf_before
    );

    assert_eq!(
        outcome,
        Err(code),
        "O7 {arm}: the refusal returns its own code, never Rejected/Wiped"
    );
    assert_eq!(
        read_opt(&counter),
        counter_before,
        "O7 {arm}: no counter write"
    );
    assert_eq!(
        read_opt(&config),
        config_before,
        "O7 {arm}: protection config untouched"
    );
    assert_eq!(
        read_opt(&vault),
        vault_before,
        "O7 {arm}: vault bytes untouched"
    );
    // Refused before any key is derived; iv and v are post-AEAD, so Argon2 runs there.
    let pre_kdf = [
        "arm i-a",
        "arm i-b",
        "arm ii",
        "arm iii",
        "arm vi",
        "arm vii-a",
        "arm vii-b",
    ];
    if pre_kdf.contains(&arm) {
        assert_eq!(kdf_after, kdf_before, "O7 {arm}: Argon2 not run");
    }
    assert!(!vault_unlocked(), "O7 {arm}: not unlocked");
    assert!(!has_process_passphrase(), "O7 {arm}: no process passphrase");
    let status = protection_status_at(T0).expect("protection status");
    assert_eq!(status.failed_unlocks, 0, "O7 {arm}: nothing counted");
    assert_eq!(status.retry_after_s, 0, "O7 {arm}: no delay armed");

    let again = unlock_guarded_at(passphrase, T0);
    assert_eq!(again, Err(code), "O7 {arm}: the next call is not Delayed");
    assert_eq!(
        read_opt(&counter),
        counter_before,
        "O7 {arm}: still no counter write"
    );
    assert_eq!(
        read_opt(&vault),
        vault_before,
        "O7 {arm}: vault still untouched"
    );

    assert_eq!(
        open_session_with_passphrase(passphrase).err(),
        Some(code),
        "O7 {arm}: the non-guard ingress returns the same unchanged code"
    );
}

fn refuse_case(tag: &str, arm: &str, code: &'static str, tamper: impl FnOnce(&Path, Vec<u8>)) {
    let _g = env_lock();
    let cfg = fresh_case(tag);
    let product = init_good_vault(&cfg);
    tamper(&cfg, product);
    wipe_after_failed_unlocks_arm(1).expect("arm the wipe at 1");
    let passphrase = if arm == "arm vi" { "" } else { PASS };
    assert_refused_uncounted(arm, &cfg, passphrase, code);
}

fn with_magic(magic: &'static [u8; 6]) -> impl FnOnce(&Path, Vec<u8>) {
    move |cfg, mut bytes| {
        bytes[..6].copy_from_slice(magic);
        write_vault(cfg, &bytes);
    }
}

// ---------------------------------------------------------------------------
// REFUSE arms: returned with the unchanged code, no counter, no delay, no wipe
// ---------------------------------------------------------------------------

#[test]
fn o7_arm_i_known_old_magic_qscv01_refused_uncounted() {
    refuse_case(
        "arm_i_qscv01",
        "arm i-a",
        "vault_version_unsupported",
        with_magic(b"QSCV01"),
    );
}

#[test]
fn o7_arm_i_known_old_magic_qscv02_refused_uncounted() {
    refuse_case(
        "arm_i_qscv02",
        "arm i-b",
        "vault_version_unsupported",
        with_magic(b"QSCV02"),
    );
}

#[test]
fn o7_arm_ii_unknown_magic_refused_uncounted() {
    refuse_case(
        "arm_ii_qscv04",
        "arm ii",
        "vault_parse_failed",
        with_magic(b"QSCV04"),
    );
}

/// C01 T2 V2: the build's magic over a different KDF header refuses at the exact KDF
/// check, BEFORE Argon2 (perf_snapshot's KDF counter is the observable).
#[test]
fn o7_arm_iii_kdf_header_mismatch_refused_before_argon2_uncounted() {
    refuse_case(
        "arm_iii_kdf",
        "arm iii",
        "vault_parse_failed",
        |cfg, mut bytes| {
            bytes[9..13].copy_from_slice(&262_144u32.to_le_bytes());
            bytes[13..17].copy_from_slice(&3u32.to_le_bytes());
            bytes[17..21].copy_from_slice(&1u32.to_le_bytes());
            write_vault(cfg, &bytes);
        },
    );
}

#[test]
fn o7_arm_iv_authenticated_payload_version_refused_uncounted() {
    refuse_case(
        "arm_iv_version",
        "arm iv",
        "vault_version_unsupported",
        |cfg, bytes| {
            assert_builder_authenticates(cfg, &bytes);
            let old = br#"{"version":3,"protocol":"NA0780-OWNER-FREE-01","secrets":{}}"#;
            write_vault(cfg, &authenticated_envelope(&bytes, old, [0x44; 12]));
        },
    );
}

#[test]
fn o7_arm_v_authenticated_directional_schema_refused_uncounted() {
    refuse_case(
        "arm_v_schema",
        "arm v",
        "directional_schema_incompatible",
        |cfg, bytes| {
            assert_builder_authenticates(cfg, &bytes);
            let foreign = br#"{"version":4,"protocol":"NA0780-OWNER-FREE-01","secrets":{"na0780_directional_unknown_s8":"x"}}"#;
            write_vault(cfg, &authenticated_envelope(&bytes, foreign, [0x55; 12]));
        },
    );
}

/// The empty passphrase: no authentication is attempted, so nothing may count.
#[test]
fn o7_arm_vi_empty_passphrase_refused_uncounted() {
    refuse_case("arm_vi_empty", "arm vi", "vault_locked", |_, _| {});
}

/// C01 T2 V8, S8b: the encrypted part after the nonce is shorter than the 16-byte tag.
/// Magic, key_source, KDF words, salt and nonce are the product's; ct_len and the file
/// length agree, so the parser accepts it. It can never authenticate: a format defect.
fn with_short_tag(ct_len: u32) -> impl FnOnce(&Path, Vec<u8>) {
    move |cfg, mut bytes| {
        bytes[21..25].copy_from_slice(&ct_len.to_le_bytes());
        bytes.truncate(HEADER_LEN + ct_len as usize);
        write_vault(cfg, &bytes);
    }
}

#[test]
fn o7_arm_vii_short_tag_ct_len_0_refused_uncounted() {
    refuse_case(
        "arm_vii_ct0",
        "arm vii-a",
        "vault_parse_failed",
        with_short_tag(0),
    );
}

#[test]
fn o7_arm_vii_short_tag_ct_len_15_refused_uncounted() {
    refuse_case(
        "arm_vii_ct15",
        "arm vii-b",
        "vault_parse_failed",
        with_short_tag(15),
    );
}

// ---------------------------------------------------------------------------
// COUNT controls: must stay counted before AND after the fix
// ---------------------------------------------------------------------------

/// Part A: limit 1 -> Wiped with the restored marker, vault and both protection files
/// gone. Part B: limit 2 -> Rejected { 1, 0 } with the counter persisted.
fn assert_counted(arm: &str, tag: &str, passphrase: &str, build: impl Fn(&Path) -> Vec<u8>) {
    let _g = env_lock();
    let cfg = fresh_case(&format!("{tag}_limit1"));
    let bytes = build(&cfg);
    write_vault(&cfg, &bytes);
    wipe_after_failed_unlocks_arm(1).expect("arm the wipe at 1");
    drain_markers();
    let outcome = unlock_guarded_at(passphrase, T0);
    eprintln!("F03_O7 arm={arm} part=A outcome={outcome:?}");
    assert_eq!(
        outcome,
        Ok(GuardedUnlockOutcome::Wiped {
            marker: QSC_ERR_VAULT_WIPED_AFTER_FAILED_UNLOCKS
        }),
        "O7 {arm}: counted, and at limit 1 wiped"
    );
    assert!(
        !cfg.join(VAULT_FILE).exists(),
        "O7 {arm}: vault gone at the limit"
    );
    assert!(
        !cfg.join(CONFIG_FILE).exists() && !cfg.join(COUNTER_FILE).exists(),
        "O7 {arm}: both protection-state files cleared"
    );
    let lines = drain_markers();
    assert!(
        lines.iter().any(|l| l.contains("event=vault_unlock")
            && l.contains(QSC_ERR_VAULT_WIPED_AFTER_FAILED_UNLOCKS)
            && l.contains("reason=failed_unlock_limit_reached")),
        "O7 {arm}: the restored wipe marker is queued, got: {lines:?}"
    );

    let cfg = fresh_case(&format!("{tag}_limit2"));
    let bytes = build(&cfg);
    write_vault(&cfg, &bytes);
    wipe_after_failed_unlocks_arm(2).expect("arm the wipe at 2");
    let outcome = unlock_guarded_at(passphrase, T0);
    eprintln!("F03_O7 arm={arm} part=B outcome={outcome:?}");
    assert_eq!(
        outcome,
        Ok(GuardedUnlockOutcome::Rejected {
            failed_unlocks: 1,
            retry_after_s: 0
        }),
        "O7 {arm}: counted below the limit"
    );
    assert_eq!(
        read_opt(&cfg.join(COUNTER_FILE)),
        Some(format!("failed_unlocks=1\nlast_failure_unix_s={T0}\n").into_bytes()),
        "O7 {arm}: the counter is persisted"
    );
    assert_eq!(
        read_opt(&cfg.join(VAULT_FILE)),
        Some(bytes),
        "O7 {arm}: vault kept below the limit"
    );
}

#[test]
fn o7_control_w_wrong_passphrase_counted_and_wiped_at_limit() {
    assert_counted("control W", "control_w", WRONG, init_good_vault);
}

/// C01 T2 V1 (inherent): vault A's header -- magic, key_source 1, KDF words, salt --
/// over the nonce and ciphertext of an independently initialised vault B. Argon2 runs
/// under the passphrase and the AEAD fails exactly as a wrong passphrase does: counted.
#[test]
fn o7_control_v1_foreign_ciphertext_counted_and_wiped_at_limit() {
    assert_counted("control V1", "control_v1", PASS, |cfg| {
        let foreign_cfg = cfg.parent().unwrap().join("foreign");
        point_at(&foreign_cfg);
        let foreign = init_good_vault(&foreign_cfg);
        point_at(cfg);
        let own = init_good_vault(cfg);
        assert_ne!(own[25..41], foreign[25..41], "independent salts");
        let mut spliced = own[..21].to_vec();
        spliced.extend_from_slice(&foreign[21..25]);
        spliced.extend_from_slice(&own[25..41]);
        spliced.extend_from_slice(&foreign[41..]);
        spliced
    });
}
