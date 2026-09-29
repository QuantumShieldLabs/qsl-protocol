pub const VAULT_MAGIC: &[u8; 6] = b"QSCV03";

// NA-0694 (D628, D-1334): the ONE owner of vault-magic recognition, shared by the unlock
// parser below and `vault_status` — the same anti-divergence property as the single header
// builder. A recognized-but-old envelope must refuse with its own name at BOTH sites
// (Ruling A), never read as corrupt or wrong-passphrase; QSCV01 is a hard break (no
// migration, no dual-format read — Ruling 2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VaultMagicClass {
    Current,
    KnownOld,
    Unknown,
}

pub fn classify_vault_magic(magic: &[u8]) -> VaultMagicClass {
    if magic == VAULT_MAGIC {
        VaultMagicClass::Current
    } else if magic == b"QSCV01" || magic == b"QSCV02" {
        VaultMagicClass::KnownOld
    } else {
        VaultMagicClass::Unknown
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VaultEnvelopeView {
    pub key_source: u8,
    pub salt: [u8; 16],
    pub kdf_m_kib: u32,
    pub kdf_t: u32,
    pub kdf_p: u32,
    pub ciphertext: Vec<u8>,
}

pub fn parse_vault_envelope(bytes: &[u8]) -> Result<VaultEnvelopeView, &'static str> {
    let min = 6 + 1 + 1 + 1 + (4 * 4);
    if bytes.len() < min {
        return Err("vault_parse_failed");
    }
    // NA-0694 (D628 §5.4): the version distinction sits AT the existing magic-check
    // position, AFTER the min-length gate above — undersized inputs (including 6-byte
    // magic-only blobs) keep refusing as vault_parse_failed before any magic logic.
    match classify_vault_magic(&bytes[..6]) {
        VaultMagicClass::Current => {}
        VaultMagicClass::KnownOld => return Err("vault_version_unsupported"),
        VaultMagicClass::Unknown => return Err("vault_parse_failed"),
    }
    let key_source = bytes[6];
    let salt_len = bytes[7] as usize;
    let nonce_len = bytes[8] as usize;
    if salt_len != 16 || nonce_len != 12 {
        return Err("vault_parse_failed");
    }
    let mut off = 9usize;
    let kdf_m_kib =
        u32::from_le_bytes([bytes[off], bytes[off + 1], bytes[off + 2], bytes[off + 3]]);
    off += 4;
    let kdf_t = u32::from_le_bytes([bytes[off], bytes[off + 1], bytes[off + 2], bytes[off + 3]]);
    off += 4;
    let kdf_p = u32::from_le_bytes([bytes[off], bytes[off + 1], bytes[off + 2], bytes[off + 3]]);
    off += 4;
    let ct_len =
        u32::from_le_bytes([bytes[off], bytes[off + 1], bytes[off + 2], bytes[off + 3]]) as usize;
    off += 4;
    let need = off + salt_len + nonce_len + ct_len;
    // NA-0788 F04/S3b N2: the envelope is exactly its header and ct_len bytes. A short one and
    // one with trailing bytes after the ciphertext are both refused (bytes nothing authenticates).
    if bytes.len() != need {
        return Err("vault_parse_failed");
    }
    let mut salt = [0u8; 16];
    salt.copy_from_slice(&bytes[off..off + salt_len]);
    off += salt_len;
    let nonce = &bytes[off..off + nonce_len];
    off += nonce_len;
    let mut ciphertext = Vec::with_capacity(nonce_len + ct_len);
    ciphertext.extend_from_slice(nonce);
    ciphertext.extend_from_slice(&bytes[off..off + ct_len]);
    Ok(VaultEnvelopeView {
        key_source,
        salt,
        kdf_m_kib,
        kdf_t,
        kdf_p,
        ciphertext,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_input_rejects_cleanly() {
        assert_eq!(
            parse_vault_envelope(b"QSCV01").unwrap_err(),
            "vault_parse_failed"
        );
    }
}

// NA-0788 F04/S3b N2: an envelope is exactly its header and ct_len bytes.
#[cfg(test)]
mod f04_s3b_envelope_tests {
    use super::*;

    /// A structurally valid envelope (canonical header fields) carrying `ct_len` bytes.
    fn envelope(ct_len: u32) -> Vec<u8> {
        let mut out = VAULT_MAGIC.to_vec();
        out.extend_from_slice(&[1, 16, 12]);
        for field in [19_456u32, 2, 1, ct_len] {
            out.extend_from_slice(&field.to_le_bytes());
        }
        out.extend_from_slice(&[0x11; 16]);
        out.extend_from_slice(&[0x22; 12]);
        out.extend(std::iter::repeat_n(0x33, ct_len as usize));
        out
    }

    #[test]
    fn t_n2_envelope_trailing_bytes_refused() {
        let exact = envelope(44);
        let view = parse_vault_envelope(&exact).unwrap();
        assert_eq!(view.ciphertext.len(), 12 + 44);
        let mut one = exact.clone();
        one.push(0);
        let mut many = exact.clone();
        many.extend_from_slice(&[0u8; 4096]);
        for refused in [&one[..], &many[..], &exact[..exact.len() - 1]] {
            assert_eq!(
                parse_vault_envelope(refused).unwrap_err(),
                "vault_parse_failed"
            );
        }
    }
}
