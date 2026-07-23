#![allow(unexpected_cfgs)]

use super::*;

#[derive(Serialize, Deserialize)]
pub(super) struct IdentityKeypair {
    pub(super) kem_pk: Vec<u8>,
    pub(super) kem_sk: Vec<u8>,
    pub(super) sig_pk: Vec<u8>,
    pub(super) sig_sk: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
// NA-0649 (D585 B2): pub visibility (type + fields) for the in-process GUI; the
// serialized shape is unchanged — the fingerprint stays DERIVED, never a stored field.
pub struct IdentityPublicRecord {
    pub kem_pk: Vec<u8>,
    #[serde(default)]
    pub sig_pk: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
struct IdentityLegacyRecord {
    kem_pk: Vec<u8>,
    kem_sk: Vec<u8>,
}

const IDENTITY_DIR: &str = "identities";
pub(super) const IDENTITY_FP_PREFIX: &str = "QSCFP-";

#[cfg(qsc_rng_failure_test_seam)]
const IDENTITY_LAZY_KEM_KEYPAIR_FAILURE_LABELS: &[&str] = &["QSC.IDENTITY.LAZY.KEM_KEYPAIR"];
#[cfg(qsc_rng_failure_test_seam)]
const IDENTITY_LAZY_SIG_KEYPAIR_FAILURE_LABELS: &[&str] = &["QSC.IDENTITY.LAZY.SIG_KEYPAIR"];
#[cfg(qsc_rng_failure_test_seam)]
const IDENTITY_LEGACY_MIGRATE_SIG_KEYPAIR_FAILURE_LABELS: &[&str] =
    &["QSC.IDENTITY.LEGACY_MIGRATE.SIG_KEYPAIR"];
#[cfg(qsc_rng_failure_test_seam)]
const IDENTITY_PUBLIC_RECORD_UPGRADE_SIG_KEYPAIR_FAILURE_LABELS: &[&str] =
    &["QSC.IDENTITY.PUBLIC_RECORD_UPGRADE.SIG_KEYPAIR"];
#[cfg(qsc_rng_failure_test_seam)]
const IDENTITY_ROTATE_KEM_KEYPAIR_FAILURE_LABELS: &[&str] = &["QSC.IDENTITY.ROTATE.KEM_KEYPAIR"];
#[cfg(qsc_rng_failure_test_seam)]
const IDENTITY_ROTATE_SIG_KEYPAIR_FAILURE_LABELS: &[&str] = &["QSC.IDENTITY.ROTATE.SIG_KEYPAIR"];
#[cfg(qsc_rng_failure_test_seam)]
fn identity_rng_failure_forced(labels: &[&str]) -> bool {
    std::env::var("QSC_RNG_FAILURE_TEST_SEAM")
        .ok()
        .map(|v| v == "all" || labels.iter().any(|label| v == *label))
        .unwrap_or(false)
}

#[cfg(qsc_rng_failure_test_seam)]
fn identity_lazy_kem_keypair() -> Result<(Vec<u8>, Vec<u8>), &'static str> {
    if identity_rng_failure_forced(IDENTITY_LAZY_KEM_KEYPAIR_FAILURE_LABELS) {
        return Err("rng_failure_forced");
    }
    crate::handshake::hs_kem_keypair_with_failure_label("QSC.KEM.KEYPAIR")
}

#[cfg(qsc_rng_failure_test_seam)]
fn identity_lazy_sig_keypair() -> Result<(Vec<u8>, Vec<u8>), &'static str> {
    if identity_rng_failure_forced(IDENTITY_LAZY_SIG_KEYPAIR_FAILURE_LABELS) {
        return Err("rng_failure_forced");
    }
    Ok(hs_sig_keypair())
}

#[cfg(qsc_rng_failure_test_seam)]
fn identity_legacy_migrate_sig_keypair() -> Result<(Vec<u8>, Vec<u8>), &'static str> {
    if identity_rng_failure_forced(IDENTITY_LEGACY_MIGRATE_SIG_KEYPAIR_FAILURE_LABELS) {
        return Err("rng_failure_forced");
    }
    Ok(hs_sig_keypair())
}

#[cfg(qsc_rng_failure_test_seam)]
fn identity_public_record_upgrade_sig_keypair() -> Result<(Vec<u8>, Vec<u8>), &'static str> {
    if identity_rng_failure_forced(IDENTITY_PUBLIC_RECORD_UPGRADE_SIG_KEYPAIR_FAILURE_LABELS) {
        return Err("rng_failure_forced");
    }
    Ok(hs_sig_keypair())
}

#[cfg(qsc_rng_failure_test_seam)]
pub(super) fn identity_rotate_kem_keypair() -> Result<(Vec<u8>, Vec<u8>), &'static str> {
    if identity_rng_failure_forced(IDENTITY_ROTATE_KEM_KEYPAIR_FAILURE_LABELS) {
        return Err("rng_failure_forced");
    }
    Ok(hs_kem_keypair())
}

#[cfg(not(qsc_rng_failure_test_seam))]
pub(super) fn identity_rotate_kem_keypair() -> Result<(Vec<u8>, Vec<u8>), &'static str> {
    Ok(hs_kem_keypair())
}

#[cfg(qsc_rng_failure_test_seam)]
pub(super) fn identity_rotate_sig_keypair() -> Result<(Vec<u8>, Vec<u8>), &'static str> {
    if identity_rng_failure_forced(IDENTITY_ROTATE_SIG_KEYPAIR_FAILURE_LABELS) {
        return Err("rng_failure_forced");
    }
    Ok(hs_sig_keypair())
}

#[cfg(not(qsc_rng_failure_test_seam))]
pub(super) fn identity_rotate_sig_keypair() -> Result<(Vec<u8>, Vec<u8>), &'static str> {
    Ok(hs_sig_keypair())
}

pub(super) fn identities_dir(dir: &Path) -> PathBuf {
    dir.join(IDENTITY_DIR)
}

pub(super) fn identity_self_path(dir: &Path, self_label: &str) -> PathBuf {
    identities_dir(dir).join(format!("self_{}.json", self_label))
}

pub(super) fn identity_fingerprint_from_pk(pk: &[u8]) -> String {
    let c = StdCrypto;
    let hash = c.sha512(pk);
    let fp = &hash[..16];
    format!("{}{}", IDENTITY_FP_PREFIX, hex_encode(fp))
}

/// NA-0634 (D571 Decision 2a): the FULL-IDENTITY verification code binds BOTH identity public keys —
/// the ML-KEM identity key and the ML-DSA signing key — so the single out-of-band code a user compares
/// authenticates the whole identity, not just its KEM half (closing the ENG-0038 signing-key asymmetry
/// that C1 left open). Both keys are fixed-length, so the ordered concatenation `kem_pk || sig_pk` is an
/// unambiguous pre-image; both parties compute it identically.
pub fn identity_fingerprint_from_identity(kem_pk: &[u8], sig_pk: &[u8]) -> String {
    let c = StdCrypto;
    let mut buf = Vec::with_capacity(kem_pk.len() + sig_pk.len());
    buf.extend_from_slice(kem_pk);
    buf.extend_from_slice(sig_pk);
    let hash = c.sha512(&buf);
    format!("{}{}", IDENTITY_FP_PREFIX, hex_encode(&hash[..16]))
}

pub(super) fn identity_secret_name(self_label: &str) -> String {
    format!("identity.kem_sk.{}", self_label)
}

pub(super) fn identity_sig_secret_name(self_label: &str) -> String {
    format!("identity.sig_sk.{}", self_label)
}

pub(super) fn identity_secret_store(self_label: &str, kem_sk: &[u8]) -> Result<(), ErrorCode> {
    let key = identity_secret_name(self_label);
    let secret = hex_encode(kem_sk);
    if let Err(e) = vault::secret_set(&key, &secret) {
        let reason = match e {
            "vault_missing" => "vault_missing",
            "vault_locked" => "vault_locked",
            _ => "vault_write_failed",
        };
        emit_marker(
            "identity_secret_unavailable",
            Some(e),
            &[("reason", reason)],
        );
        return Err(match e {
            "vault_missing" => ErrorCode::IdentitySecretUnavailable,
            "vault_locked" => ErrorCode::IdentitySecretUnavailable,
            _ => ErrorCode::IoWriteFailed,
        });
    }
    emit_marker(
        "identity_secret_store",
        None,
        &[("ok", "true"), ("method", "vault")],
    );
    Ok(())
}

fn identity_secret_load(self_label: &str) -> Result<Vec<u8>, ErrorCode> {
    let key = identity_secret_name(self_label);
    let Some(secret) = vault::secret_get(&key).map_err(|e| {
        let reason = match e {
            "vault_missing" => "vault_missing",
            "vault_locked" => "vault_locked",
            _ => "vault_read_failed",
        };
        emit_marker(
            "identity_secret_unavailable",
            Some(e),
            &[("reason", reason)],
        );
        match e {
            "vault_missing" => ErrorCode::IdentitySecretUnavailable,
            "vault_locked" => ErrorCode::IdentitySecretUnavailable,
            _ => ErrorCode::IoReadFailed,
        }
    })?
    else {
        emit_marker(
            "identity_secret_unavailable",
            Some("identity_secret_unavailable"),
            &[("reason", "missing_secret")],
        );
        return Err(ErrorCode::IdentitySecretUnavailable);
    };
    hex_decode(&secret)
}

pub(super) fn identity_sig_secret_store(self_label: &str, sig_sk: &[u8]) -> Result<(), ErrorCode> {
    let key = identity_sig_secret_name(self_label);
    let secret = hex_encode(sig_sk);
    if let Err(e) = vault::secret_set(&key, &secret) {
        let reason = match e {
            "vault_missing" => "vault_missing",
            "vault_locked" => "vault_locked",
            _ => "vault_write_failed",
        };
        emit_marker(
            "identity_secret_unavailable",
            Some(e),
            &[("reason", reason)],
        );
        return Err(match e {
            "vault_missing" | "vault_locked" => ErrorCode::IdentitySecretUnavailable,
            _ => ErrorCode::IoWriteFailed,
        });
    }
    emit_marker(
        "identity_secret_store",
        None,
        &[("ok", "true"), ("method", "vault")],
    );
    Ok(())
}

fn identity_sig_secret_load(self_label: &str) -> Result<Vec<u8>, ErrorCode> {
    let key = identity_sig_secret_name(self_label);
    let Some(secret) = vault::secret_get(&key).map_err(|e| {
        let reason = match e {
            "vault_missing" => "vault_missing",
            "vault_locked" => "vault_locked",
            _ => "vault_read_failed",
        };
        emit_marker(
            "identity_secret_unavailable",
            Some(e),
            &[("reason", reason)],
        );
        match e {
            "vault_missing" | "vault_locked" => ErrorCode::IdentitySecretUnavailable,
            _ => ErrorCode::IoReadFailed,
        }
    })?
    else {
        emit_marker(
            "identity_secret_unavailable",
            Some("identity_secret_unavailable"),
            &[("reason", "missing_secret")],
        );
        return Err(ErrorCode::IdentitySecretUnavailable);
    };
    hex_decode(&secret)
}

pub(super) fn identity_write_public_record(
    self_label: &str,
    kem_pk: &[u8],
    sig_pk: &[u8],
) -> Result<(), ErrorCode> {
    if !channel_label_ok(self_label) {
        return Err(ErrorCode::ParseFailed);
    }
    let (dir, source) = config_dir()?;
    let identities = identities_dir(&dir);
    ensure_dir_secure(&identities, source)?;
    let path = identity_self_path(&dir, self_label);
    let rec = IdentityPublicRecord {
        kem_pk: kem_pk.to_vec(),
        sig_pk: sig_pk.to_vec(),
    };
    let bytes = serde_json::to_vec(&rec).map_err(|_| ErrorCode::ParseFailed)?;
    write_atomic(&path, &bytes, source)?;
    Ok(())
}

fn identity_migrate_legacy(
    self_label: &str,
    source: ConfigSource,
    path: &Path,
    legacy: IdentityLegacyRecord,
) -> Result<IdentityKeypair, ErrorCode> {
    #[cfg(qsc_rng_failure_test_seam)]
    let (sig_pk, sig_sk) = match identity_legacy_migrate_sig_keypair() {
        Ok(v) => v,
        Err(e) => {
            emit_marker(
                "identity_secret_unavailable",
                Some(e),
                &[("reason", "rng_failure_forced")],
            );
            return Err(ErrorCode::IdentitySecretUnavailable);
        }
    };
    #[cfg(not(qsc_rng_failure_test_seam))]
    let (sig_pk, sig_sk) = hs_sig_keypair();
    if let Err(e) = identity_secret_store(self_label, &legacy.kem_sk) {
        emit_marker(
            "identity_secret_migrate",
            Some(e.as_str()),
            &[
                ("ok", "false"),
                ("action", "skipped"),
                ("reason", "vault_unavailable"),
            ],
        );
        return Err(e);
    }
    if let Err(e) = identity_sig_secret_store(self_label, &sig_sk) {
        emit_marker(
            "identity_secret_migrate",
            Some(e.as_str()),
            &[
                ("ok", "false"),
                ("action", "skipped"),
                ("reason", "vault_unavailable"),
            ],
        );
        return Err(e);
    }
    let rec = IdentityPublicRecord {
        kem_pk: legacy.kem_pk.clone(),
        sig_pk: sig_pk.clone(),
    };
    let bytes = serde_json::to_vec(&rec).map_err(|_| ErrorCode::ParseFailed)?;
    write_atomic(path, &bytes, source)?;
    emit_marker(
        "identity_secret_migrate",
        None,
        &[
            ("ok", "true"),
            ("action", "imported"),
            ("reason", "legacy_plaintext"),
        ],
    );
    Ok(IdentityKeypair {
        kem_pk: legacy.kem_pk,
        kem_sk: legacy.kem_sk,
        sig_pk,
        sig_sk,
    })
}

fn identity_read_self_kem_keypair(self_label: &str) -> Result<Option<IdentityKeypair>, ErrorCode> {
    if !channel_label_ok(self_label) {
        return Err(ErrorCode::ParseFailed);
    }
    let (dir, source) = config_dir()?;
    let identities = identities_dir(&dir);
    ensure_dir_secure(&identities, source)?;
    let path = identity_self_path(&dir, self_label);
    if !path.exists() {
        return Ok(None);
    }
    enforce_safe_parents(&path, source)?;
    let bytes = fs::read(&path).map_err(|_| ErrorCode::IoReadFailed)?;
    if let Ok(rec) = serde_json::from_slice::<IdentityPublicRecord>(&bytes) {
        let kem_sk = identity_secret_load(self_label)?;
        let (sig_pk, sig_sk) = if rec.sig_pk.is_empty() {
            #[cfg(qsc_rng_failure_test_seam)]
            let (sig_pk, sig_sk) = match identity_public_record_upgrade_sig_keypair() {
                Ok(v) => v,
                Err(e) => {
                    emit_marker(
                        "identity_secret_unavailable",
                        Some(e),
                        &[("reason", "rng_failure_forced")],
                    );
                    return Err(ErrorCode::IdentitySecretUnavailable);
                }
            };
            #[cfg(not(qsc_rng_failure_test_seam))]
            let (sig_pk, sig_sk) = hs_sig_keypair();
            identity_sig_secret_store(self_label, &sig_sk)?;
            identity_write_public_record(self_label, &rec.kem_pk, &sig_pk)?;
            (sig_pk, sig_sk)
        } else {
            (rec.sig_pk.clone(), identity_sig_secret_load(self_label)?)
        };
        return Ok(Some(IdentityKeypair {
            kem_pk: rec.kem_pk,
            kem_sk,
            sig_pk,
            sig_sk,
        }));
    }
    if let Ok(legacy) = serde_json::from_slice::<IdentityLegacyRecord>(&bytes) {
        let migrated = identity_migrate_legacy(self_label, source, &path, legacy)?;
        return Ok(Some(migrated));
    }
    Err(ErrorCode::ParseFailed)
}

pub fn identity_read_self_public(
    self_label: &str,
) -> Result<Option<IdentityPublicRecord>, ErrorCode> {
    if !channel_label_ok(self_label) {
        return Err(ErrorCode::ParseFailed);
    }
    let (dir, source) = config_dir()?;
    let identities = identities_dir(&dir);
    ensure_dir_secure(&identities, source)?;
    let path = identity_self_path(&dir, self_label);
    if !path.exists() {
        return Ok(None);
    }
    enforce_safe_parents(&path, source)?;
    let bytes = fs::read(&path).map_err(|_| ErrorCode::IoReadFailed)?;
    if let Ok(rec) = serde_json::from_slice::<IdentityPublicRecord>(&bytes) {
        return Ok(Some(rec));
    }
    if let Ok(legacy) = serde_json::from_slice::<IdentityLegacyRecord>(&bytes) {
        return Ok(Some(IdentityPublicRecord {
            kem_pk: legacy.kem_pk,
            sig_pk: Vec::new(),
        }));
    }
    Err(ErrorCode::ParseFailed)
}

pub(super) fn identity_self_kem_keypair(self_label: &str) -> Result<IdentityKeypair, ErrorCode> {
    if !channel_label_ok(self_label) {
        return Err(ErrorCode::ParseFailed);
    }
    let (dir, source) = config_dir()?;
    let identities = identities_dir(&dir);
    ensure_dir_secure(&identities, source)?;
    let path = identity_self_path(&dir, self_label);
    if path.exists() {
        enforce_safe_parents(&path, source)?;
        if let Some(kp) = identity_read_self_kem_keypair(self_label)? {
            return Ok(kp);
        }
        return Err(ErrorCode::ParseFailed);
    }
    // NA-0616 (ENG-0001): fail closed rather than silently minting a SECOND, divergent
    // self-identity. A config dir is meant to hold one self; first-run auto-create (empty
    // dir) is allowed, but if a self-identity under a DIFFERENT label already exists the
    // operator most likely typo'd or used an inconsistent `--as`, so refuse. Explicit
    // `identity rotate --as <label>` bypasses this path and remains the intentional way
    // to create an additional identity.
    if let Ok(entries) = std::fs::read_dir(&identities) {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if let Some(existing) = name
                    .strip_prefix("self_")
                    .and_then(|stem| stem.strip_suffix(".json"))
                {
                    if existing != self_label {
                        emit_marker(
                            "identity_self_ambiguous",
                            None,
                            &[("existing", existing), ("requested", self_label)],
                        );
                        return Err(ErrorCode::IdentitySelfAmbiguous);
                    }
                }
            }
        }
    }
    #[cfg(qsc_rng_failure_test_seam)]
    let (kem_pk, kem_sk) = match identity_lazy_kem_keypair() {
        Ok(v) => v,
        Err(e) => {
            emit_marker(
                "identity_secret_unavailable",
                Some(e),
                &[("reason", "rng_failure_forced")],
            );
            return Err(ErrorCode::IdentitySecretUnavailable);
        }
    };
    #[cfg(not(qsc_rng_failure_test_seam))]
    let (kem_pk, kem_sk) = hs_kem_keypair();
    #[cfg(qsc_rng_failure_test_seam)]
    let (sig_pk, sig_sk) = match identity_lazy_sig_keypair() {
        Ok(v) => v,
        Err(e) => {
            emit_marker(
                "identity_secret_unavailable",
                Some(e),
                &[("reason", "rng_failure_forced")],
            );
            return Err(ErrorCode::IdentitySecretUnavailable);
        }
    };
    #[cfg(not(qsc_rng_failure_test_seam))]
    let (sig_pk, sig_sk) = hs_sig_keypair();
    identity_secret_store(self_label, &kem_sk)?;
    identity_sig_secret_store(self_label, &sig_sk)?;
    identity_write_public_record(self_label, &kem_pk, &sig_pk)?;
    Ok(IdentityKeypair {
        kem_pk,
        kem_sk,
        sig_pk,
        sig_sk,
    })
}

/// NA-0649 (D585 B3): deliberate identity creation for the in-process GUI. Returns the
/// existing identity's public record with no mutation; otherwise creates it via the
/// existing lazy path (`identity_self_kem_keypair`), so the NA-0616 second-identity
/// guard and the vault-level unlock requirement apply exactly as on that path.
/// Rotation stays the separate, explicit `identity_rotate` flow.
pub fn identity_ensure(self_label: &str) -> Result<IdentityPublicRecord, ErrorCode> {
    if let Some(rec) = identity_read_self_public(self_label)? {
        return Ok(rec);
    }
    let IdentityKeypair {
        kem_pk,
        mut kem_sk,
        sig_pk,
        mut sig_sk,
    } = identity_self_kem_keypair(self_label)?;
    kem_sk.zeroize();
    sig_sk.zeroize();
    Ok(IdentityPublicRecord { kem_pk, sig_pk })
}

pub fn format_verification_code_from_fingerprint(fingerprint: &str) -> String {
    const CROCKFORD: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";
    // NA-0669 (C-1a): strip the constant `QSCFP-` prefix BEFORE the alphanumeric filter. The
    // filter drops the prefix's hyphen but KEEPS its five letters, so five of the sixteen
    // displayed characters were constant and every code ever shown began `QSCF-P`. Stripping
    // first takes the code from 11 varying hex characters (44 bits) to 16 (64 bits) at unchanged
    // target width and unchanged `4-4-4-4-checksum` grouping. This function is `pub` and
    // reachable from qsl-desktop, so a fingerprint that does NOT carry the prefix is left alone
    // rather than assumed; the match is case-sensitive, exact parity with the `starts_with`
    // guards at the two in-crate call sites.
    let body = fingerprint
        .strip_prefix(IDENTITY_FP_PREFIX)
        .unwrap_or(fingerprint);
    let mut chars = body
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .map(|ch| ch.to_ascii_uppercase())
        .collect::<Vec<char>>();
    while chars.len() < 16 {
        chars.push('0');
    }
    let code = chars.into_iter().take(16).collect::<String>();
    let checksum_idx = code
        .bytes()
        .fold(0u32, |acc, byte| acc.saturating_add(byte as u32))
        % 32;
    let checksum = CROCKFORD[checksum_idx as usize] as char;
    format!(
        "{}-{}-{}-{}-{}",
        &code[0..4],
        &code[4..8],
        &code[8..12],
        &code[12..16],
        checksum
    )
}

pub(super) fn identity_marker_display(fp: &str) -> String {
    if fp.starts_with(IDENTITY_FP_PREFIX) {
        format_verification_code_from_fingerprint(fp)
    } else {
        fp.to_string()
    }
}

#[allow(dead_code)]
pub(super) fn identity_pin_matches_seen(pinned: &str, seen_fp: &str) -> bool {
    let pinned = pinned.trim();
    if pinned.is_empty() {
        return false;
    }
    if pinned.eq_ignore_ascii_case(seen_fp) {
        return true;
    }
    if seen_fp.starts_with(IDENTITY_FP_PREFIX) {
        let seen_code = format_verification_code_from_fingerprint(seen_fp);
        return pinned.eq_ignore_ascii_case(seen_code.as_str());
    }
    false
}

pub(super) fn identity_read_pin(peer: &str) -> Result<Option<String>, ErrorCode> {
    let peer_alias = peer_alias_from_channel(peer);
    Ok(contacts_entry_read(peer_alias)?.and_then(|v| {
        let fp = primary_device(&v)
            .map(|d| d.fp.as_str())
            .unwrap_or(v.fp.as_str());
        if fp.is_empty() || fp.eq_ignore_ascii_case("UNSET") {
            None
        } else {
            Some(fp.to_string())
        }
    }))
}

pub(super) fn identity_read_sig_pin(peer: &str) -> Result<Option<String>, ErrorCode> {
    let peer_alias = peer_alias_from_channel(peer);
    Ok(contacts_entry_read(peer_alias)?.and_then(|v| {
        primary_device(&v)
            .and_then(|d| d.sig_fp.clone())
            .or(v.sig_fp)
    }))
}

/// NA-0633 (ENG-0038): the peer's full identity KEM public key (decoded), verified against the pinned
/// fingerprint at add-time. The initiator encapsulates to it so the responder must prove KEM-secret
/// possession (construction C1). `None` => a legacy/incomplete contact ⇒ the initiator fails closed.
pub(super) fn identity_read_peer_kem_pk(peer: &str) -> Result<Option<Vec<u8>>, ErrorCode> {
    let peer_alias = peer_alias_from_channel(peer);
    let hex = contacts_entry_read(peer_alias)?.and_then(|v| {
        primary_device(&v)
            .and_then(|d| d.kem_pk.clone())
            .or(v.kem_pk)
    });
    Ok(match hex {
        Some(h) => hex_decode(&h).ok(),
        None => None,
    })
}
