#![allow(unexpected_cfgs)]

use super::*;

/// NA-0711 (D647 A4): the canonical single self-identity label, and the only default. It exists as
/// a constant so the CLI surface and the resolver cannot drift apart the way `--as` and
/// `--self-label` did.
pub(crate) const DEFAULT_SELF_LABEL: &str = "self";

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

/// NA-0749 (`D-1391`, ruled at `R362` §1) — the role of a fingerprint, carried in its domain string.
///
/// ⚠ The role is NEVER recoverable from a rendered fingerprint: all three roles render as 64
/// lowercase hex. It is fixed at the CALL SITE and must be passed, never sniffed.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum FpRole {
    /// The combined identity fingerprint — the only role with a voice tier.
    Identity,
    /// The signing-key fingerprint stored as a contact's `sig_fp`.
    Sig,
    /// The KEM-only legacy fingerprint (`contacts add --kem-pk` without `--sig-pk`).
    Kem,
}

/// The version-and-role-bearing domain. It is ASCII and contains NO NUL byte, so in
/// `DOMAIN || 0x00 || fields` the FIRST NUL is unambiguously the terminator whatever the fields
/// contain — role and version separation hold BY CONSTRUCTION, not because the field lengths
/// happen to differ.
fn fp_domain(role: FpRole) -> &'static [u8] {
    match role {
        FpRole::Identity => b"qsl-fp-v1:identity",
        FpRole::Sig => b"qsl-fp-v1:sig",
        FpRole::Kem => b"qsl-fp-v1:kem",
    }
}

/// `SHA-512( DOMAIN[role] || 0x00 || u64_be(len(f_1)) || f_1 || ... )`.
///
/// ⚠⚠ THE LENGTH PREFIX IS WHY THIS FUNCTION EXISTS. The construction this replaced hashed the bare
/// concatenation `kem_pk || sig_pk`, which is injective over the CONCATENATION and not over the
/// PAIR: every re-split of the same 3136 bytes is a different `(kem, sig)` producing an IDENTICAL
/// fingerprint — 3136 of them, one carrying an attacker-chosen signing key behind a byte-identical
/// read-aloud value. Nothing off the wire enforced the key lengths (`hex_decode` has no length bound
/// and `contacts_add` checks none), so the ambiguity was reachable. `tests/na0749_fingerprint_
/// conformance.rs` walks every split position and is the regression that keeps it closed.
///
/// u64 rather than the u16 that `invite::canonical_bundle_bytes` uses: a u16 prefix TRUNCATES above
/// 65535 bytes and truncation would reintroduce the ambiguity. `usize -> u64` is lossless on every
/// supported platform, so this cannot truncate and the function stays infallible.
fn fp_digest(role: FpRole, fields: &[&[u8]]) -> [u8; 64] {
    let c = StdCrypto;
    let domain = fp_domain(role);
    let mut buf = Vec::with_capacity(
        domain.len() + 1 + fields.iter().map(|f| 8 + f.len()).sum::<usize>(),
    );
    buf.extend_from_slice(domain);
    buf.push(0x00);
    for f in fields {
        buf.extend_from_slice(&(f.len() as u64).to_be_bytes());
        buf.extend_from_slice(f);
    }
    c.sha512(&buf)
}

/// The FULL form (C3): lowercase hex of the digest's first 32 bytes — 64 characters, 256-bit,
/// NO prefix. Grouping and case are presentation and belong to consumers.
fn fp_full(digest: &[u8; 64]) -> String {
    hex_encode(&digest[..32])
}

/// A single-key fingerprint under its own role domain.
///
/// Replaces the two retired single-key constructors — one in this module, one in `handshake` —
/// which were byte-identical constructions living in two modules with no domain separating them,
/// so the formal model's "distinct domain" assumption was true only by the accident that
/// `1184 != 1952`. It is now true by construction.
pub(super) fn identity_fingerprint_single(role: FpRole, pk: &[u8]) -> String {
    fp_full(&fp_digest(role, &[pk]))
}

/// NA-0634 (D571 Decision 2a): the FULL-IDENTITY fingerprint binds BOTH identity public keys — the
/// ML-KEM identity key and the ML-DSA signing key — so the single out-of-band value a user compares
/// authenticates the whole identity, not just its KEM half (closing the ENG-0038 signing-key
/// asymmetry that C1 left open).
///
/// NA-0749: the ordered pair is now LENGTH-PREFIXED, so the pre-image is unambiguous for ANY pair of
/// field lengths rather than only for the fixed ones the parameter set happens to give.
pub fn identity_fingerprint_from_identity(kem_pk: &[u8], sig_pk: &[u8]) -> String {
    fp_full(&fp_digest(FpRole::Identity, &[kem_pk, sig_pk]))
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

/// NA-0711 (D647 as amended by A4, Δ33/Δ34; R238 §1): resolve the self label a path must use, so a
/// caller can consult NA-0616's property BEFORE it acts instead of meeting it afterwards with its
/// error discarded.
///
/// ⚠ This is the SAME ratified property `identity_self_kem_keypair` enforces below
/// ("a config dir is meant to hold one self-identity"), EXTRACTED rather than re-authored — the
/// house's own answer to this defect class, reused instead of a new one invented beside it.
///
/// | `requested` | `self_*.json` present | result |
/// |---|---|---|
/// | `None` | exactly one | **that label** — the derivation |
/// | `None` | none | `self`, the canonical default (first-run auto-create stays allowed) |
/// | `None` | two or more | `Err(IdentitySelfAmbiguous)` — derivation is ambiguous, so it refuses |
/// | `Some(l)` | a record for `l` exists, or none exist at all | `l` |
/// | `Some(l)` | records exist but none is `l` | `Err(IdentitySelfAmbiguous)` — the safety net |
///
/// ⚠ **RESIDUAL HOLE, STATED HERE RATHER THAN DISCOVERED LATER (R238 §1.1):** in a dir that
/// legitimately holds two or more identities, an explicit **wrong-but-existing** label passes this
/// check and the caller's lookup can still miss silently. That is the same class the "one
/// vocabulary" shape was refused for — narrowed to multi-identity dirs, **not closed**. The
/// whole-key `handshake_pending` marker is the compensating control.
pub(crate) fn identity_resolved_self_label(requested: Option<&str>) -> Result<String, ErrorCode> {
    if let Some(l) = requested {
        if !channel_label_ok(l) {
            return Err(ErrorCode::ParseFailed);
        }
    }
    let (dir, source) = config_dir()?;
    let identities = identities_dir(&dir);
    ensure_dir_secure(&identities, source)?;
    let mut existing: Vec<String> = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&identities) {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if let Some(stem) = name
                    .strip_prefix("self_")
                    .and_then(|stem| stem.strip_suffix(".json"))
                {
                    existing.push(stem.to_string());
                }
            }
        }
    }
    existing.sort();
    match (requested, existing.len()) {
        (Some(l), 0) => Ok(l.to_string()),
        (Some(l), _) => {
            if existing.iter().any(|e| e == l) {
                Ok(l.to_string())
            } else {
                emit_marker(
                    "identity_self_ambiguous",
                    None,
                    &[("existing", existing.join(",").as_str()), ("requested", l)],
                );
                Err(ErrorCode::IdentitySelfAmbiguous)
            }
        }
        (None, 0) => Ok(DEFAULT_SELF_LABEL.to_string()),
        (None, 1) => Ok(existing[0].clone()),
        (None, _) => {
            emit_marker(
                "identity_self_ambiguous",
                None,
                &[
                    ("existing", existing.join(",").as_str()),
                    ("requested", "<derive>"),
                ],
            );
            Err(ErrorCode::IdentitySelfAmbiguous)
        }
    }
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

/// NA-0749 (`D-1391`) — the VOICE form (C4): EXACTLY 30 decimal digits, leading zeros legal.
///
/// `int(digest[0..20], big-endian) mod 10^30`, derived from the FULL form's own bytes so that any
/// holder of the rendered 64-hex string can compute it. That is load-bearing: `identity_pin_matches
/// _seen_identity` and every out-of-crate consumer receive the fingerprint ONLY as a string, and the
/// retired verification code had exactly this property.
///
/// ⚠⚠ PUBLISHED API. `identity` is a `pub mod`, so an out-of-crate caller can reach this. The `""`
/// sentinel below is only safe if the CALLER refuses it: an out-of-crate caller MUST NOT compare the
/// returned value against a user-supplied pin without first checking it is non-empty. In-crate the
/// guard is applied at the call site as well, belt-and-braces.
///
/// Returns `""` — a sentinel no legitimate pin equals — for any input that is not a well-formed FULL
/// form. Shape is checked FIRST, before any arithmetic.
pub fn identity_voice_form(fp: &str) -> String {
    // ⚠ Shape check FIRST. Two implementations are FORBIDDEN here and both are natural:
    //   - zero-padding a short input and continuing, which makes a fixed voice value that an
    //     attacker-pinned string equals for EVERY peer — a universal matcher;
    //   - panicking / unwrapping on non-hex, which turns an attacker-supplied contact record into
    //     a crash.
    if fp.len() != 64 || !fp.bytes().all(|b| b.is_ascii_hexdigit()) {
        return String::new();
    }
    let bytes = match hex_decode(fp) {
        Ok(b) => b,
        Err(_) => return String::new(),
    };
    if bytes.len() < 20 {
        return String::new();
    }
    // The 160-bit reduction, done without a 160-bit type: a u128 Horner loop over the first 20
    // bytes. The invariant is `acc < 10^30` after every step, and `10^30 < 2^100`, so the
    // intermediate `acc * 256 + 255 < 2^108 < 2^128` and cannot overflow.
    // ⚠ FORBIDDEN BY NAME: `u128::from_be_bytes(digest[0..16])` — it reads SIXTEEN bytes where this
    // derivation takes TWENTY, and would silently produce a different value that still looks like a
    // 30-digit number.
    const MODULUS: u128 = 1_000_000_000_000_000_000_000_000_000_000; // 10^30
    let mut acc: u128 = 0;
    for &b in &bytes[..20] {
        acc = (acc * 256 + b as u128) % MODULUS;
    }
    format!("{acc:030}")
}

/// The pin comparator for EVERY role. Tier 1 only: the FULL form.
///
/// ⚠ The voice tier is deliberately NOT here. `sig` and `kem` fingerprints have no voice form
/// sealed anywhere, and one is nonetheless derivable from any 64-hex string — so the defence is
/// ROUTING, not the shape of `identity_voice_form`. A single-key site that called the identity
/// comparator would silently accept a ~100-bit comparand for a role the design never sealed.
pub(super) fn identity_pin_matches_seen(pinned: &str, seen_fp: &str) -> bool {
    let pinned = pinned.trim();
    if pinned.is_empty() {
        return false;
    }
    pinned.eq_ignore_ascii_case(seen_fp)
}

/// The pin comparator for the COMBINED IDENTITY fingerprint, and only for it. Tier 1 (the full
/// form) then tier 2 (the voice form) — the two renderings a user may have compared out of band.
///
/// ⚠ `trim()` binds ONCE, before BOTH tiers. Splitting the original single body is exactly how that
/// invariant was lost once already: a trim applied inside the tier-1 helper is a local binding that
/// never reaches tier 2, so a hand-typed voice form with a trailing space was refused while a padded
/// full form was accepted.
pub(super) fn identity_pin_matches_seen_identity(pinned: &str, seen_identity_fp: &str) -> bool {
    let pinned = pinned.trim();
    if pinned.is_empty() {
        return false;
    }
    if pinned.eq_ignore_ascii_case(seen_identity_fp) {
        return true;
    }
    let voice = identity_voice_form(seen_identity_fp);
    !voice.is_empty() && pinned == voice
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

#[cfg(test)]
mod na0749_fingerprint_tests {
    use super::*;

    fn kem_pk() -> Vec<u8> {
        (0..1184u32).map(|i| (i % 256) as u8).collect()
    }
    fn sig_pk() -> Vec<u8> {
        (0..1952u32).map(|i| (255 - (i % 256)) as u8).collect()
    }

    // The sealed independent vector (`/srv/qbuild/operator/NA-0749/vector_v2/`), computed outside
    // this codebase by a tool whose SHA-512 was validated against the published NIST vector before
    // any lane value was produced, and corroborated by four engines in total.
    const SEALED_IDENTITY_FULL: &str =
        "d67b4a10510394ca268c9e8cfde8980fd6280dc8c379d4ea8c8642ac9a750349";
    const SEALED_IDENTITY_VOICE: &str = "187363336018275058094178831816";
    const SEALED_SIG_FULL: &str =
        "c7251cb68ab0db6416e4ef3b3e9c372a6b63222587f22027ef12efbd75d58bab";
    const SEALED_KEM_FULL: &str =
        "f5f23dadc0acee52fb4da4528d7bbe49aa5e7ecd77b9ea6962b2981040246d98";

    #[test]
    fn na0749_single_key_roles_match_the_sealed_vector() {
        assert_eq!(
            identity_fingerprint_single(FpRole::Sig, &sig_pk()),
            SEALED_SIG_FULL
        );
        assert_eq!(
            identity_fingerprint_single(FpRole::Kem, &kem_pk()),
            SEALED_KEM_FULL
        );
    }

    /// The repair of the formal model's false premise: role separation must hold on IDENTICAL
    /// material, not merely because the parameter set gives the two keys different lengths.
    #[test]
    fn na0749_roles_are_separated_on_identical_material() {
        let m = sig_pk();
        assert_ne!(
            identity_fingerprint_single(FpRole::Sig, &m),
            identity_fingerprint_single(FpRole::Kem, &m),
            "role separation must not depend on the material differing"
        );
    }

    // ---- W6: the comparator arms. B, C and G ACCEPT and prove nothing alone; A, D, E, F and H
    // ---- are the seal, because a comparator that accepts everything passes every accepting arm.

    #[test]
    fn na0749_w6_a_old_format_pin_is_refused() {
        // The retired format. Fails closed against a new-format fingerprint.
        assert!(!identity_pin_matches_seen_identity(
            "QSCFP-9069d8689203a5a1576fbc88a44a525e",
            SEALED_IDENTITY_FULL
        ));
    }

    #[test]
    fn na0749_w6_b_c_correct_full_and_voice_forms_are_accepted() {
        assert!(identity_pin_matches_seen_identity(
            SEALED_IDENTITY_FULL,
            SEALED_IDENTITY_FULL
        ));
        assert!(identity_pin_matches_seen_identity(
            SEALED_IDENTITY_VOICE,
            SEALED_IDENTITY_FULL
        ));
    }

    #[test]
    fn na0749_w6_d_a_wrong_but_well_formed_voice_form_is_refused() {
        // Same shape, one digit different. This is the arm a comparator that accepts ANY 30-digit
        // string would fail, and the arm the first draft of these seals did not have.
        let wrong = "187363336018275058094178831817";
        assert_eq!(wrong.len(), 30);
        assert!(wrong.chars().all(|c| c.is_ascii_digit()));
        assert!(!identity_pin_matches_seen_identity(wrong, SEALED_IDENTITY_FULL));
    }

    #[test]
    fn na0749_w6_e_an_identity_voice_form_does_not_match_a_single_key_fingerprint() {
        // The voice tier belongs to the combined identity fingerprint and to nothing else.
        assert!(!identity_pin_matches_seen_identity(
            SEALED_IDENTITY_VOICE,
            SEALED_SIG_FULL
        ));
    }

    #[test]
    fn na0749_w6_f_malformed_seen_fingerprints_never_authenticate() {
        // Totality: the sentinel must be unreachable as a match, on every malformed shape.
        for bad in ["", "abc", "zz", &"z".repeat(64), &"a".repeat(63), &"a".repeat(65)] {
            assert!(identity_voice_form(bad).is_empty(), "voice form of {bad:?} must be the sentinel");
            assert!(!identity_pin_matches_seen_identity("", bad));
            assert!(!identity_pin_matches_seen_identity("   ", bad));
        }
        // And the sentinel itself must never be offered successfully as a pin.
        assert!(!identity_pin_matches_seen_identity("", SEALED_IDENTITY_FULL));
    }

    #[test]
    fn na0749_w6_g_whitespace_padding_is_accepted_on_BOTH_tiers() {
        // The trim binds ONCE, before both tiers. Splitting the original body once lost this: a
        // padded FULL form was accepted while a padded VOICE form was refused.
        let padded_full = format!("  {SEALED_IDENTITY_FULL}  ");
        let padded_voice = format!("  {SEALED_IDENTITY_VOICE}  ");
        assert!(identity_pin_matches_seen_identity(&padded_full, SEALED_IDENTITY_FULL));
        assert!(identity_pin_matches_seen_identity(&padded_voice, SEALED_IDENTITY_FULL));
    }

    #[test]
    fn na0749_w6_h_the_plain_comparator_refuses_its_own_derivable_voice_form() {
        // ROUTING is the defence, not the shape of `identity_voice_form`: a voice form IS derivable
        // for a single-key fingerprint, and the plain comparator must never accept it.
        let sig_voice = identity_voice_form(SEALED_SIG_FULL);
        assert_eq!(sig_voice.len(), 30, "a voice form IS derivable for the sig role");
        assert!(
            !identity_pin_matches_seen(&sig_voice, SEALED_SIG_FULL),
            "the plain comparator must have no voice tier"
        );
    }

    #[test]
    fn na0749_voice_form_is_case_insensitive_over_the_full_form() {
        let upper = SEALED_IDENTITY_FULL.to_ascii_uppercase();
        assert_eq!(identity_voice_form(&upper), SEALED_IDENTITY_VOICE);
    }
}
