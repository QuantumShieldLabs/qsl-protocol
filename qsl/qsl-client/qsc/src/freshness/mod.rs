//! NA-0787 F04-C07P S3: the freshness provider's PURE building blocks (C07 rollback and durability).
//!
//! - [`ProtectionMode`]: the C07-03 protection profile discriminator and its QSLFRESH profile byte.
//! - [`digest`] / [`anchor`]: the C8 vault digest and anchor chain (C07 T4.1 C8; the label is C07-06).
//! - [`checkpoint`]: the QSLFRESH v1 codec (C07 T6.5, DOC-CAN-003 C07-01).
//! - [`paths`]: the D31 checkpoint location and the D32 lock path (C07 T6.4).
//! - [`recover`]: the T4.3 LOCAL recovery classifier over the [`LineageOpen`] seam (S4).
//! - [`lock`]: the D32 lineage lock (FN1), with the inode re-check (S5).
//! - [`txn`]: begin / commit (C1-C3, C5) / genesis (C7) under the lineage lock (S5).
//!
//! NOTHING HERE IS WIRED. The recovery classifier (S4), the transaction (S5) and the provider
//! selector (S6) are the consumers, and S11 wires them into init/unlock/writes; until then every
//! item is reachable only from this module's tests, hence the one `dead_code` allowance below.
//! No client-code string lives here: the ER spellings are S6's (RULING_F04C07P R6).
#![allow(dead_code)]

pub(crate) mod checkpoint;
pub(crate) mod lock;
pub(crate) mod paths;
pub(crate) mod recover;
pub(crate) mod txn;

use sha2::{Digest, Sha256};
use std::fmt;
use zeroize::Zeroizing;

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

/// The five lineage values the provider needs from inside a vault blob (C07 T6.1 F1-F5), as the
/// caller's opener returns them AFTER authenticating the blob (step2/CIRCULARITY sec 2). Everything
/// else in the payload stays opaque to the provider.
pub(crate) struct LineageFields {
    pub(crate) vault_id: [u8; 32],
    pub(crate) generation: u64,
    pub(crate) predecessor_anchor: [u8; 32],
    /// Payload content (C07 F5), held only in this zeroizing container and only for one
    /// operation; never printed, logged or written except as the QSLFRESH MAC key
    /// (RULING_F04C07P R8).
    pub(crate) checkpoint_mac_key: Zeroizing<[u8; 32]>,
    pub(crate) protection_mode: ProtectionMode,
}

/// Deliberately partial: the key is never formatted.
impl fmt::Debug for LineageFields {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LineageFields")
            .field("generation", &self.generation)
            .field("mode", &self.protection_mode)
            .finish_non_exhaustive()
    }
}

/// "Did not authenticate", and nothing more: no reason and no string (RULING_F04C07P R6).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OpenFailed;

/// THE SEAM (step2/CIRCULARITY sec 2; RULING_F04C07P R3, R8). The CALLER authenticates the blob --
/// the vault AEAD, paying any Argon2id (C07 T4.3 :271-273, AM-4) -- and returns its lineage values.
/// The provider holds no vault key and derives none. S11 implements it over payload v5.
pub(crate) trait LineageOpen {
    fn open(&self, blob: &[u8]) -> Result<LineageFields, OpenFailed>;
}

/// THE PROCESS-CUT SEAM (S5; PROTOTYPE sec 4; RULING_F04C07P R9): a cut point of the transaction,
/// genesis or recovery. Compiled only under cfg(test): no shipping artifact contains it (I13).
#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CutPoint {
    KB2,
    KM2,
    KA2,
    KM3,
    KA34,
    KM5,
    KA5,
    KA6,
    GI1,
    GI2,
    GI3,
    RC1,
}

#[cfg(test)]
impl CutPoint {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::KB2 => "K-B2",
            Self::KM2 => "K-M2",
            Self::KA2 => "K-A2",
            Self::KM3 => "K-M3",
            Self::KA34 => "K-A34",
            Self::KM5 => "K-M5",
            Self::KA5 => "K-A5",
            Self::KA6 => "K-A6",
            Self::GI1 => "GI-1",
            Self::GI2 => "GI-2",
            Self::GI3 => "GI-3",
            Self::RC1 => "RC-1",
        }
    }
}

/// The variable naming the one cut point a child process dies at, and the child's exit code there.
#[cfg(test)]
pub(crate) const CUT_ENV: &str = "QSL_FRESHNESS_CUT";
#[cfg(test)]
pub(crate) const CUT_EXIT: i32 = 86;

/// THE CUT FUNCTION. In a process started with CUT_ENV naming `point`, end the process here with
/// exit 86 (no unwinding, no destructors: nothing is flushed or unlocked on the way out). When
/// `temp` is given, first leave that temp exactly as `fs_store::write_file_durable` leaves its own
/// before the rename -- created new, mode 0600, written, flushed -- because that cut point lies
/// inside the primitive, where no cut can be placed (K-M2, K-M3). Otherwise a no-op.
#[cfg(test)]
#[inline(never)]
pub(crate) fn cut(point: CutPoint, temp: Option<(&std::path::Path, &[u8])>) {
    if std::env::var(CUT_ENV).ok().as_deref() != Some(point.name()) {
        return;
    }
    if let Some((path, bytes)) = temp {
        use std::io::Write;
        use std::os::unix::fs::OpenOptionsExt;
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(path)
            .expect("cut: create the temp");
        f.write_all(bytes).expect("cut: write the temp");
        f.sync_all().expect("cut: flush the temp");
    }
    std::process::exit(CUT_EXIT);
}

/// The variable naming a cut child's fixture directory (the child reads its inputs there).
#[cfg(test)]
pub(crate) const FIXTURE_ENV: &str = "QSL_FRESHNESS_FIXTURE";

/// Re-execute this unit-test binary as a CHILD running exactly the ignored test `test`, with the
/// cut point (if any) and the fixture directory in its environment. Returns its exit status and
/// its pid. "Restart" is then the caller's own begin(): a new process relative to the child.
#[cfg(test)]
pub(crate) fn run_child(
    test: &str,
    cut: Option<CutPoint>,
    fixture: &std::path::Path,
) -> (std::process::ExitStatus, u32) {
    let exe = std::env::current_exe().expect("current_exe");
    let mut cmd = std::process::Command::new(exe);
    cmd.args(["--ignored", "--exact", test, "--test-threads=1"])
        .env(FIXTURE_ENV, fixture)
        .env_remove(CUT_ENV)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());
    if let Some(point) = cut {
        cmd.env(CUT_ENV, point.name());
    }
    let child = cmd.spawn().expect("spawn the child");
    let pid = child.id();
    let out = child.wait_with_output().expect("wait for the child");
    (out.status, pid)
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
