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
pub(super) struct IdentityPublicRecord {
    pub(super) kem_pk: Vec<u8>,
    #[serde(default)]
    pub(super) sig_pk: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
struct IdentityLegacyRecord {
    kem_pk: Vec<u8>,
    kem_sk: Vec<u8>,
}

const IDENTITY_DIR: &str = "identities";
pub(super) const IDENTITY_FP_PREFIX: &str = "QSCFP-";

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

pub(super) fn identity_read_self_public(
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
    let (kem_pk, kem_sk) = hs_kem_keypair();
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

pub(super) fn identity_self_fingerprint(self_label: &str) -> Result<String, ErrorCode> {
    match identity_read_self_public(self_label)? {
        Some(rec) => Ok(identity_fingerprint_from_pk(&rec.kem_pk)),
        None => Ok("untrusted".to_string()),
    }
}

pub(super) fn format_verification_code_from_fingerprint(fingerprint: &str) -> String {
    const CROCKFORD: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";
    let mut chars = fingerprint
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
