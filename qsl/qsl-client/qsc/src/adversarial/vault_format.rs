pub const VAULT_MAGIC: &[u8; 6] = b"QSCV01";

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
    if &bytes[..6] != VAULT_MAGIC {
        return Err("vault_parse_failed");
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
    if bytes.len() < need {
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
