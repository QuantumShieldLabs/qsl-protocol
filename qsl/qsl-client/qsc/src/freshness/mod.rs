//! NA-0787 F04-C07P S3: the freshness provider's PURE building blocks (C07 rollback and durability).
//!
//! - [`ProtectionMode`]: the C07-03 protection profile discriminator and its QSLFRESH profile byte.
//! - [`digest`] / [`anchor`]: the C8 vault digest and anchor chain (C07 T4.1 C8; the label is C07-06).
//! - [`checkpoint`]: the QSLFRESH v1 codec (C07 T6.5, DOC-CAN-003 C07-01).
//! - [`paths`]: the D31 checkpoint location and the D32 lock path (C07 T6.4).
//!
//! NOTHING HERE IS WIRED. The recovery classifier (S4), the transaction (S5) and the provider
//! selector (S6) are the consumers, and S11 wires them into init/unlock/writes; until then every
//! item is reachable only from this module's tests, hence the one `dead_code` allowance below.
//! No client-code string lives here: the ER spellings are S6's (RULING_F04C07P R6).
#![allow(dead_code)]

pub(crate) mod checkpoint;
pub(crate) mod paths;

use sha2::{Digest, Sha256};

/// C07-06: the vault digest domain label. A CRYPTO INPUT -- every committed digest `d` of every
/// lineage depends on these exact 16 ASCII bytes, so changing them orphans every checkpoint.
pub(crate) const VAULT_DIGEST_LABEL: &[u8; 16] = b"QSL-C07-VAULT-v1";

/// C07-03: the protection profile, fixed at genesis. The payload spelling and the QSLFRESH
/// profile byte are two encodings of one value; neither has a default or a fallback (I01).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ProtectionMode {
    LocalCheckpoint,
    Tpm,
}

/// An unknown `protection_mode` value (C07 ER19). The spelling of the marker is S6's.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct UnsupportedProtectionMode;

impl ProtectionMode {
    /// EXACT match only: no case folding, no trimming. `"Tpm"` or `" tpm"` is an unknown value,
    /// and an unknown value refuses (C07-03, C07 T6.1 F2).
    pub(crate) fn parse(value: &str) -> Result<Self, UnsupportedProtectionMode> {
        match value {
            "local-checkpoint" => Ok(Self::LocalCheckpoint),
            "tpm" => Ok(Self::Tpm),
            _ => Err(UnsupportedProtectionMode),
        }
    }

    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::LocalCheckpoint => "local-checkpoint",
            Self::Tpm => "tpm",
        }
    }

    /// The QSLFRESH v1 profile byte (C07 T6.5: 1 local, 2 TPM).
    pub(crate) fn profile_byte(self) -> u8 {
        match self {
            Self::LocalCheckpoint => 1,
            Self::Tpm => 2,
        }
    }

    pub(crate) fn from_profile_byte(byte: u8) -> Option<Self> {
        match byte {
            1 => Some(Self::LocalCheckpoint),
            2 => Some(Self::Tpm),
            _ => None,
        }
    }
}

/// C8: `d = SHA-256("QSL-C07-VAULT-v1" || V || B)`, with `V` the 32-byte vault_id and `B` the exact
/// bytes of the complete encrypted vault (C07 T4.1 C8, citing the design's A2 :43-46).
pub(crate) fn digest(vault_id: &[u8; 32], blob: &[u8]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(VAULT_DIGEST_LABEL);
    h.update(vault_id);
    h.update(blob);
    h.finalize().into()
}

/// C8: `A_next = SHA-256(A || d)` -- the TPM NV_Extend rule (C07 NV5), so the local chain and the
/// hardware chain agree byte for byte. Genesis extends from 32 zero bytes (C07 T4.1 C7).
pub(crate) fn anchor(prev: &[u8; 32], d: &[u8; 32]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(prev);
    h.update(d);
    h.finalize().into()
}

#[cfg(test)]
pub(crate) mod test_support {
    //! The SYNTHETIC inputs of the reference script (lanes/NA-0787/evidence/s3/step2/
    //! reference_vectors.py), rebuilt here from the same formulas. No toy or production bytes.

    pub(crate) fn hex(s: &str) -> Vec<u8> {
        assert!(s.len().is_multiple_of(2), "odd hex length");
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16).expect("hex digit"))
            .collect()
    }

    pub(crate) fn hex32(s: &str) -> [u8; 32] {
        hex(s).try_into().expect("32 bytes")
    }

    /// V[i] = (7*i + 3) mod 256.
    pub(crate) fn syn_vault_id() -> [u8; 32] {
        std::array::from_fn(|i| (7 * i + 3) as u8)
    }

    /// B[i] = (31*i + 5) mod 256, 300 bytes.
    pub(crate) fn syn_blob() -> Vec<u8> {
        (0..300usize).map(|i| (31 * i + 5) as u8).collect()
    }

    /// P[i] = 0xA0 xor i.
    pub(crate) fn syn_prev() -> [u8; 32] {
        std::array::from_fn(|i| 0xA0 ^ i as u8)
    }

    /// K[i] = 0x40 + i. A synthetic test key, public by construction.
    pub(crate) fn syn_mac_key() -> [u8; 32] {
        std::array::from_fn(|i| 0x40 + i as u8)
    }

    /// Greater than 2^32 and not byte-palindromic, so byte order is observable.
    pub(crate) const SYN_GENERATION: u64 = 0x0102_0304_0506_0708;

    /// reference_vectors.out, "VEC-1 d" and "VEC-1 A".
    pub(crate) const VEC1_D: &str =
        "55a301aebdce4592501a1dd97da76d445a66d0bac8d84975e78a3c09e10d3b86";
    pub(crate) const VEC1_A: &str =
        "dd937666ccb22001dbd8d46bf6d5d12de2cab0fb6bfd8e9678f9dbb4b54a79c9";
}

#[cfg(test)]
mod tests {
    use super::test_support::*;
    use super::*;

    #[test]
    fn pm1_protection_mode_exact_strings() {
        for mode in [ProtectionMode::LocalCheckpoint, ProtectionMode::Tpm] {
            assert_eq!(ProtectionMode::parse(mode.as_str()), Ok(mode));
        }
        assert_eq!(
            ProtectionMode::parse("local-checkpoint"),
            Ok(ProtectionMode::LocalCheckpoint)
        );
        assert_eq!(ProtectionMode::parse("tpm"), Ok(ProtectionMode::Tpm));
        for bad in [
            "Tpm",
            "TPM",
            "local_checkpoint",
            " tpm",
            "tpm ",
            "",
            "none",
            "local-checkpoint\n",
        ] {
            assert_eq!(
                ProtectionMode::parse(bad),
                Err(UnsupportedProtectionMode),
                "{bad:?} must be refused"
            );
        }
    }

    #[test]
    fn pm2_profile_byte_mapping() {
        assert_eq!(ProtectionMode::LocalCheckpoint.profile_byte(), 1);
        assert_eq!(ProtectionMode::Tpm.profile_byte(), 2);
        assert_eq!(
            ProtectionMode::from_profile_byte(1),
            Some(ProtectionMode::LocalCheckpoint)
        );
        assert_eq!(
            ProtectionMode::from_profile_byte(2),
            Some(ProtectionMode::Tpm)
        );
        for bad in [0u8, 3, 255] {
            assert_eq!(ProtectionMode::from_profile_byte(bad), None);
        }
    }

    #[test]
    fn vec1_digest_and_anchor_match_reference() {
        let d = digest(&syn_vault_id(), &syn_blob());
        assert_eq!(
            d,
            hex32(VEC1_D),
            "C8 digest differs from the reference script"
        );
        let a = anchor(&syn_prev(), &d);
        assert_eq!(
            a,
            hex32(VEC1_A),
            "C8 anchor differs from the reference script"
        );
    }
}
