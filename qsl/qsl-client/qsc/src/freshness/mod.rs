//! NA-0787 F04-C07P S3: the freshness provider's PURE building blocks (C07 rollback and durability).
//!
//! - [`ProtectionMode`]: the C07-03 protection profile discriminator and its QSLFRESH profile byte.
//! - [`digest`] / [`anchor`]: the C8 vault digest and anchor chain (C07 T4.1 C8; the label is C07-06).
//! - [`checkpoint`]: the QSLFRESH v1 codec (C07 T6.5, DOC-CAN-003 C07-01).
//! - [`paths`]: the D31 checkpoint location and the D32 lock path (C07 T6.4).
//! - [`recover`]: the T4.3 LOCAL recovery classifier over the [`LineageOpen`] seam (S4).
//! - [`lock`]: the D32 lineage lock (FN1), with the inode re-check (S5).
//! - [`txn`]: begin / commit (C1-C3, C5) / genesis (C7) under the lineage lock (S5).
//! - [`codes`], [`select_provider`] and every typed failure's `code()`: DOC-CAN-009's client refusal codes (S6).
//!
//! NOTHING HERE IS WIRED. The recovery classifier (S4), the transaction (S5) and the provider
//! selector (S6) are the consumers, and S11 wires them into init/unlock/writes; until then every
//! item is reachable only from this module's tests, hence the one `dead_code` allowance below.
//! No client-code string lives here: the ER spellings are S6's (RULING_F04C07P R6).
//! S6 supersedes that last sentence: [`codes`] holds the spellings, and nothing outside it spells one.
#![allow(dead_code)]

pub(crate) mod checkpoint;
pub(crate) mod lock;
pub(crate) mod paths;
pub(crate) mod recover;
pub(crate) mod txn;

use crate::model::ErrorCode;
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

/// DOC-CAN-009 "QSC client refusal codes (normative registry)": one constant per registry row,
/// QRC-0001..QRC-0021 in order (RULING_NA0787_S6a R1-R3). The registry is append-only: a row is
/// never edited or reused. An evidence-side script compares these with its Code column (SR-20).
pub(crate) mod codes {
    /// QRC-0001, C07 ER2.
    pub(crate) const FRESHNESS_CHECKPOINT_MISSING: &str = "freshness_checkpoint_missing";
    /// QRC-0002, C07 ER3.
    pub(crate) const FRESHNESS_CHECKPOINT_CORRUPT: &str = "freshness_checkpoint_corrupt";
    /// QRC-0003, C07 ER4.
    pub(crate) const FRESHNESS_CHECKPOINT_UNSUPPORTED: &str = "freshness_checkpoint_unsupported";
    /// QRC-0004, C07 ER5.
    pub(crate) const FRESHNESS_GENERATION_REGRESSION: &str = "freshness_generation_regression";
    /// QRC-0005, C07 ER6.
    pub(crate) const FRESHNESS_DIGEST_CONFLICT: &str = "freshness_digest_conflict";
    /// QRC-0006, C07 ER7.
    pub(crate) const COMMITTED_STATE_MISSING: &str = "committed_state_missing";
    /// QRC-0007, C07 ER17.
    pub(crate) const STORAGE_DURABILITY_FAILED: &str = "storage_durability_failed";
    /// QRC-0008, C07 ER18.
    pub(crate) const FRESHNESS_GENERATION_EXHAUSTED: &str = "freshness_generation_exhausted";
    /// QRC-0009, C07 ER19.
    pub(crate) const VAULT_PROTECTION_MODE_UNSUPPORTED: &str = "vault_protection_mode_unsupported";
    /// QRC-0010, C07 ER21.
    pub(crate) const ANCHOR_UNQUALIFIED: &str = "anchor_unqualified";
    /// QRC-0011.
    pub(crate) const FRESHNESS_STATE_DIR_INVALID: &str = "freshness_state_dir_invalid";
    /// QRC-0012.
    pub(crate) const FRESHNESS_LINEAGE_LOCK_CONTENDED: &str = "freshness_lineage_lock_contended";
    /// QRC-0013.
    pub(crate) const FRESHNESS_LINEAGE_LOCK_UNSTABLE: &str = "freshness_lineage_lock_unstable";
    /// QRC-0014.
    pub(crate) const FRESHNESS_LINEAGE_CHANGED: &str = "freshness_lineage_changed";
    /// QRC-0015.
    pub(crate) const FRESHNESS_LINEAGE_MISMATCH: &str = "freshness_lineage_mismatch";
    /// QRC-0016.
    pub(crate) const FRESHNESS_CHECKPOINT_UNREADABLE: &str = "freshness_checkpoint_unreadable";
    /// QRC-0017.
    pub(crate) const FRESHNESS_HEAD_CHANGED: &str = "freshness_head_changed";
    /// QRC-0018.
    pub(crate) const FRESHNESS_CHECKPOINT_EXISTS: &str = "freshness_checkpoint_exists";
    /// QRC-0019.
    pub(crate) const FRESHNESS_SUCCESSOR_INVALID: &str = "freshness_successor_invalid";
    /// QRC-0020 (C01 AM-6.17 O14's distinct code for an oversized vault file).
    pub(crate) const VAULT_FILE_OVERSIZED: &str = "vault_file_oversized";
    /// QRC-0021.
    pub(crate) const VAULT_CAPACITY_EXCEEDED: &str = "vault_capacity_exceeded";

    /// EXISTING qsc codes the provider reuses that have no constant anywhere else in the tree.
    /// NOT registry rows (DOC-CAN-009 sec 4). vault_locked and the model codes are referenced
    /// where they already live (`crate::msgqueue::MSGQUEUE_VAULT_LOCKED`, `ErrorCode::as_str`).
    pub(crate) mod existing {
        /// The unlock path's code for an absent vault (vault/mod.rs:964).
        pub(crate) const VAULT_MISSING: &str = "vault_missing";
        /// The unlock path's code for an unreadable vault file (vault/mod.rs:969).
        pub(crate) const VAULT_READ_FAILED: &str = "vault_read_failed";
        /// Init's code for a vault already present (vault/mod.rs:841).
        pub(crate) const VAULT_EXISTS: &str = "vault_exists";
    }
}

/// The anchor providers this build has: exactly one (C07 AM-1). The TPM provider is F05's.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Provider {
    LocalCheckpoint,
}

/// C07 AM-3: why no provider was selected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SelectError {
    /// ER21: `tpm` while the T2 PL8 list is empty. No TPM command is issued; nothing is written.
    AnchorUnqualified,
}

/// C07 AM-3, T6.3 (RULING_NA0787_S6a Q-12): the provider for a protection mode. The T2 PL8 list of
/// qualified TPMs is EMPTY in this build, so `tpm` always refuses ER21 -- never a silent fall-back
/// to the local provider (I01, QQ4). A pure function of one value: no path, no environment, no
/// I/O. A stored value that is neither spelling never gets here: `ProtectionMode::parse` refuses it.
pub(crate) fn select_provider(mode: ProtectionMode) -> Result<Provider, SelectError> {
    match mode {
        ProtectionMode::LocalCheckpoint => Ok(Provider::LocalCheckpoint),
        ProtectionMode::Tpm => Err(SelectError::AnchorUnqualified),
    }
}

// S6: every typed failure -> ONE client refusal code (MAPPING TABLE 1 as RULING_NA0787_S6a rules
// it; the row numbers below are that table's). Every match lists every variant, so a new kind is
// a compile error until it is mapped: `_` appears only on an io::ErrorKind payload, `..` only on
// the detail fields slot / kind / which. The provider exposes no counting rule (Q-10): S11 counts.

impl UnsupportedProtectionMode {
    /// Row 4, ER19.
    pub(crate) fn code(&self) -> &'static str {
        codes::VAULT_PROTECTION_MODE_UNSUPPORTED
    }
}

impl SelectError {
    /// Row 5, ER21.
    pub(crate) fn code(&self) -> &'static str {
        match self {
            Self::AnchorUnqualified => codes::ANCHOR_UNQUALIFIED,
        }
    }
}

impl paths::StateRootError {
    /// Rows 1-3.
    pub(crate) fn code(&self) -> &'static str {
        match self {
            Self::NoHome => ErrorCode::MissingHome.as_str(),
            Self::HomeNotAbsolute | Self::OverrideNotAbsolute => codes::FRESHNESS_STATE_DIR_INVALID,
        }
    }
}

impl paths::CheckpointDirError {
    /// Rows 10-14 (and 60).
    pub(crate) fn code(&self) -> &'static str {
        match self {
            Self::RootNotAbsolute | Self::NotADirectory => codes::FRESHNESS_STATE_DIR_INVALID,
            Self::Symlink => ErrorCode::UnsafePathSymlink.as_str(),
            Self::GroupOrWorldWritable => ErrorCode::UnsafeParentPerms.as_str(),
            Self::Io(_) => ErrorCode::IoWriteFailed.as_str(),
        }
    }
}

impl lock::LockError {
    /// Rows 15-19 (and 61).
    pub(crate) fn code(&self) -> &'static str {
        match self {
            Self::Open(_) => ErrorCode::LockOpenFailed.as_str(),
            Self::Contended => codes::FRESHNESS_LINEAGE_LOCK_CONTENDED,
            Self::Flock(_) | Self::Stat(_) => ErrorCode::LockFailed.as_str(),
            Self::InodeRetriesExhausted => codes::FRESHNESS_LINEAGE_LOCK_UNSTABLE,
        }
    }
}

impl recover::FreezeKind {
    /// Rows 28-35: the T4.3 freezes.
    pub(crate) fn code(&self) -> &'static str {
        use checkpoint::{CorruptKind as C, UnsupportedKind as U};
        match self {
            Self::CheckpointMissing => codes::FRESHNESS_CHECKPOINT_MISSING,
            Self::CheckpointCorrupt(C::Length | C::Trailing | C::Magic | C::Mac) => {
                codes::FRESHNESS_CHECKPOINT_CORRUPT
            }
            Self::CheckpointUnsupported(U::Version | U::Profile | U::ProfileDisagreesWithMode) => {
                codes::FRESHNESS_CHECKPOINT_UNSUPPORTED
            }
            Self::ForeignCheckpoint => codes::COMMITTED_STATE_MISSING,
            Self::LineageMismatch => codes::FRESHNESS_LINEAGE_MISMATCH,
            Self::GenerationRegression => codes::FRESHNESS_GENERATION_REGRESSION,
            Self::DigestConflict => codes::FRESHNESS_DIGEST_CONFLICT,
            Self::CommittedStateMissing => codes::COMMITTED_STATE_MISSING,
        }
    }
}

impl crate::fs_store::DurableWriteError {
    /// Rows 49-54 (Q-7): a path-safety refusal keeps the hygiene's own code; every other stage is
    /// a write or flush that did not complete, ER17.
    pub(crate) fn code(&self) -> &'static str {
        match self {
            Self::Hygiene(code) => code.as_str(),
            Self::TempNotSibling
            | Self::TempCreateOrWrite
            | Self::FileFlush
            | Self::Rename
            | Self::DirFlush => codes::STORAGE_DURABILITY_FAILED,
        }
    }
}

/// Where a RecoverError surfaced: two of its kinds mean different things at each (rows 9 / 23 /
/// 65 and 24 / 66).
#[derive(Debug, Clone, Copy)]
enum RecoverAt {
    /// begin()'s quarantined open, before the lock (it builds only BlobRead, BlobTooLarge and
    /// OpenFailed; the other kinds keep their own cause's code there).
    Quarantine,
    /// recover() under begin()'s lineage lock.
    Locked,
    /// recover() inside genesis(), where anything found means a vault is already here.
    Genesis,
}

fn recover_code(e: &recover::RecoverError, at: RecoverAt) -> &'static str {
    use recover::RecoverError as R;
    match e {
        R::TempCleanup(_) => ErrorCode::IoWriteFailed.as_str(),
        R::BlobRead { .. } => codes::existing::VAULT_READ_FAILED,
        R::BlobTooLarge { .. } => codes::VAULT_FILE_OVERSIZED,
        R::OpenFailed { .. } => match at {
            // ER1, the one code that may count -- and only under S11's opener condition (Q-10).
            RecoverAt::Quarantine => crate::msgqueue::MSGQUEUE_VAULT_LOCKED,
            // The quarantine authenticated a blob moments earlier: the slots changed.
            RecoverAt::Locked => codes::FRESHNESS_LINEAGE_CHANGED,
            // At init: a vault this opener cannot open is already in the store.
            RecoverAt::Genesis => codes::existing::VAULT_EXISTS,
        },
        R::NotLocalCheckpoint => match at {
            RecoverAt::Quarantine | RecoverAt::Locked => codes::ANCHOR_UNQUALIFIED,
            RecoverAt::Genesis => codes::existing::VAULT_EXISTS,
        },
        R::CheckpointRead(_) => codes::FRESHNESS_CHECKPOINT_UNREADABLE,
        R::DurableWrite(e) => e.code(),
    }
}

impl recover::RecoverError {
    /// Rows 20-27: recover() refusing under begin()'s lineage lock.
    pub(crate) fn code(&self) -> &'static str {
        recover_code(self, RecoverAt::Locked)
    }
}

impl txn::BeginError {
    /// Rows 6-36.
    pub(crate) fn code(&self) -> &'static str {
        match self {
            Self::NoLineage => codes::existing::VAULT_MISSING,
            Self::Quarantine(e) => recover_code(e, RecoverAt::Quarantine),
            Self::CheckpointDir(e) => e.code(),
            Self::Lock(e) => e.code(),
            Self::Recover(e) => e.code(),
            Self::Freeze(kind) => kind.code(),
            Self::LineageChanged => codes::FRESHNESS_LINEAGE_CHANGED,
        }
    }
}

impl txn::CommitError {
    /// Rows 37-54.
    pub(crate) fn code(&self) -> &'static str {
        use txn::ChainBreak as B;
        match self {
            Self::Poisoned | Self::StalePrepared(_) => codes::STORAGE_DURABILITY_FAILED,
            Self::GenerationExhausted => codes::FRESHNESS_GENERATION_EXHAUSTED,
            Self::SuccessorTooLarge => codes::VAULT_CAPACITY_EXCEEDED,
            Self::SuccessorOpenFailed
            | Self::NotChained(B::VaultId | B::Mode | B::Generation | B::PredecessorAnchor) => {
                codes::FRESHNESS_SUCCESSOR_INVALID
            }
            Self::NotChained(B::CheckpointKey | B::HeadChanged) => codes::FRESHNESS_HEAD_CHANGED,
            Self::NotChained(B::HeadRead(_)) => codes::FRESHNESS_CHECKPOINT_UNREADABLE,
            Self::DurableWrite(e) => e.code(),
        }
    }
}

impl txn::GenesisError {
    /// Rows 55-71.
    pub(crate) fn code(&self) -> &'static str {
        use txn::GenesisBreak as B;
        match self {
            Self::B0TooLarge => codes::VAULT_CAPACITY_EXCEEDED,
            Self::B0OpenFailed | Self::NotGenesis(B::Generation | B::PredecessorAnchor) => {
                codes::FRESHNESS_SUCCESSOR_INVALID
            }
            Self::NotLocalCheckpoint => codes::ANCHOR_UNQUALIFIED,
            Self::CheckpointDir(e) => e.code(),
            Self::Lock(e) => e.code(),
            Self::Recover(e) => recover_code(e, RecoverAt::Genesis),
            Self::LineagePresent => codes::existing::VAULT_EXISTS,
            Self::CheckpointExists => codes::FRESHNESS_CHECKPOINT_EXISTS,
            Self::DurableWrite(e) => e.code(),
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

    #[test]
    fn sel1_local_checkpoint_selects_the_local_provider() {
        assert_eq!(
            select_provider(ProtectionMode::LocalCheckpoint),
            Ok(Provider::LocalCheckpoint)
        );
    }

    #[test]
    fn sel2_tpm_refuses_anchor_unqualified_with_no_io() {
        let refused = select_provider(ProtectionMode::Tpm);
        assert_eq!(refused, Err(SelectError::AnchorUnqualified));
        assert_eq!(refused.unwrap_err().code(), "anchor_unqualified");
        // The body census: no filesystem, process, environment or unsafe token, and no wildcard.
        let src = include_str!("mod.rs");
        let start = src
            .find("pub(crate) fn select_provider(")
            .expect("select_provider is defined");
        let len = src[start..].find("\n}\n").expect("select_provider ends");
        let body = &src[start..start + len];
        assert!(
            body.contains("match mode") && body.contains("ProtectionMode::Tpm"),
            "the census would be vacuous: {body}"
        );
        for token in [
            "fs::",
            "File",
            "OpenOptions",
            "Command",
            "env::",
            "unsafe",
            "_ =>",
        ] {
            assert!(!body.contains(token), "select_provider holds {token:?}");
        }
    }

    #[test]
    fn sel3_unknown_stored_mode_is_refused_by_parse() {
        for bad in ["none", "", "Tpm", "local_checkpoint"] {
            let refused = ProtectionMode::parse(bad);
            assert_eq!(
                refused,
                Err(UnsupportedProtectionMode),
                "{bad:?} must be refused"
            );
            assert_eq!(
                refused.unwrap_err().code(),
                "vault_protection_mode_unsupported"
            );
        }
    }

    /// CODE-2: (row, the constructed kind's code(), the RULED spelling). Every mismatch is printed,
    /// then the test fails naming them all, so a mutation's red set is visible row by row.
    fn code2(rows: &[(&str, &str, &str)]) {
        let red: Vec<&str> = rows
            .iter()
            .filter(|(_, got, want)| got != want)
            .map(|(id, got, want)| {
                eprintln!("CODE-2 row {id}: got {got} want {want}");
                *id
            })
            .collect();
        assert!(red.is_empty(), "CODE-2 red rows: {red:?}");
    }

    #[test]
    fn code2_rows_01_05_state_root_parse_select() {
        use super::paths::StateRootError as S;
        code2(&[
            ("1", S::NoHome.code(), "missing_home"),
            (
                "2",
                S::HomeNotAbsolute.code(),
                "freshness_state_dir_invalid",
            ),
            (
                "3",
                S::OverrideNotAbsolute.code(),
                "freshness_state_dir_invalid",
            ),
            (
                "4",
                UnsupportedProtectionMode.code(),
                "vault_protection_mode_unsupported",
            ),
            (
                "5",
                SelectError::AnchorUnqualified.code(),
                "anchor_unqualified",
            ),
        ]);
    }

    #[test]
    fn code2_rows_06_36_begin() {
        use super::checkpoint::{CorruptKind as C, UnsupportedKind as U};
        use super::lock::LockError as L;
        use super::paths::CheckpointDirError as D;
        use super::recover::{FreezeKind as F, RecoverError as R, Slot, Unauthenticated};
        use super::txn::BeginError as E;
        use crate::fs_store::DurableWriteError as W;
        use std::io::ErrorKind as K;
        let read = |slot| R::BlobRead {
            slot,
            kind: K::PermissionDenied,
        };
        code2(&[
            ("6", E::NoLineage.code(), "vault_missing"),
            (
                "7",
                E::Quarantine(read(Slot::Current)).code(),
                "vault_read_failed",
            ),
            (
                "8",
                E::Quarantine(R::BlobTooLarge {
                    slot: Slot::Current,
                })
                .code(),
                "vault_file_oversized",
            ),
            (
                "9",
                E::Quarantine(R::OpenFailed {
                    which: Unauthenticated::Current,
                })
                .code(),
                "vault_locked",
            ),
            (
                "10",
                E::CheckpointDir(D::RootNotAbsolute).code(),
                "freshness_state_dir_invalid",
            ),
            (
                "11",
                E::CheckpointDir(D::Symlink).code(),
                "unsafe_path_symlink",
            ),
            (
                "12",
                E::CheckpointDir(D::NotADirectory).code(),
                "freshness_state_dir_invalid",
            ),
            (
                "13",
                E::CheckpointDir(D::GroupOrWorldWritable).code(),
                "unsafe_parent_perms",
            ),
            (
                "14",
                E::CheckpointDir(D::Io(K::PermissionDenied)).code(),
                "io_write_failed",
            ),
            (
                "15",
                E::Lock(L::Open(K::PermissionDenied)).code(),
                "lock_open_failed",
            ),
            (
                "16",
                E::Lock(L::Contended).code(),
                "freshness_lineage_lock_contended",
            ),
            ("17", E::Lock(L::Flock(K::Other)).code(), "lock_failed"),
            ("18", E::Lock(L::Stat(K::NotFound)).code(), "lock_failed"),
            (
                "19",
                E::Lock(L::InodeRetriesExhausted).code(),
                "freshness_lineage_lock_unstable",
            ),
            (
                "20",
                E::Recover(R::TempCleanup(K::PermissionDenied)).code(),
                "io_write_failed",
            ),
            (
                "21",
                E::Recover(read(Slot::Prepared)).code(),
                "vault_read_failed",
            ),
            (
                "22",
                E::Recover(R::BlobTooLarge {
                    slot: Slot::Prepared,
                })
                .code(),
                "vault_file_oversized",
            ),
            (
                "23",
                E::Recover(R::OpenFailed {
                    which: Unauthenticated::Both,
                })
                .code(),
                "freshness_lineage_changed",
            ),
            (
                "24",
                E::Recover(R::NotLocalCheckpoint).code(),
                "anchor_unqualified",
            ),
            (
                "25",
                E::Recover(R::CheckpointRead(K::PermissionDenied)).code(),
                "freshness_checkpoint_unreadable",
            ),
            (
                "26",
                E::Recover(R::DurableWrite(W::Rename)).code(),
                "storage_durability_failed",
            ),
            (
                "27",
                E::Recover(R::DurableWrite(W::DirFlush)).code(),
                "storage_durability_failed",
            ),
            (
                "28",
                E::Freeze(F::CheckpointMissing).code(),
                "freshness_checkpoint_missing",
            ),
            (
                "29.Length",
                E::Freeze(F::CheckpointCorrupt(C::Length)).code(),
                "freshness_checkpoint_corrupt",
            ),
            (
                "29.Trailing",
                E::Freeze(F::CheckpointCorrupt(C::Trailing)).code(),
                "freshness_checkpoint_corrupt",
            ),
            (
                "29.Magic",
                E::Freeze(F::CheckpointCorrupt(C::Magic)).code(),
                "freshness_checkpoint_corrupt",
            ),
            (
                "29.Mac",
                E::Freeze(F::CheckpointCorrupt(C::Mac)).code(),
                "freshness_checkpoint_corrupt",
            ),
            (
                "30.Version",
                E::Freeze(F::CheckpointUnsupported(U::Version)).code(),
                "freshness_checkpoint_unsupported",
            ),
            (
                "30.Profile",
                E::Freeze(F::CheckpointUnsupported(U::Profile)).code(),
                "freshness_checkpoint_unsupported",
            ),
            (
                "30.ProfileDisagreesWithMode",
                E::Freeze(F::CheckpointUnsupported(U::ProfileDisagreesWithMode)).code(),
                "freshness_checkpoint_unsupported",
            ),
            (
                "31",
                E::Freeze(F::ForeignCheckpoint).code(),
                "committed_state_missing",
            ),
            (
                "32",
                E::Freeze(F::LineageMismatch).code(),
                "freshness_lineage_mismatch",
            ),
            (
                "33",
                E::Freeze(F::GenerationRegression).code(),
                "freshness_generation_regression",
            ),
            (
                "34",
                E::Freeze(F::DigestConflict).code(),
                "freshness_digest_conflict",
            ),
            (
                "35",
                E::Freeze(F::CommittedStateMissing).code(),
                "committed_state_missing",
            ),
            ("36", E::LineageChanged.code(), "freshness_lineage_changed"),
            // QX (sealed D-S1): kinds the quarantine never builds keep their own cause's code.
            (
                "QX.TempCleanup",
                E::Quarantine(R::TempCleanup(K::Other)).code(),
                "io_write_failed",
            ),
            (
                "QX.NotLocalCheckpoint",
                E::Quarantine(R::NotLocalCheckpoint).code(),
                "anchor_unqualified",
            ),
            (
                "QX.CheckpointRead",
                E::Quarantine(R::CheckpointRead(K::Other)).code(),
                "freshness_checkpoint_unreadable",
            ),
            (
                "QX.DurableWrite.Rename",
                E::Quarantine(R::DurableWrite(W::Rename)).code(),
                "storage_durability_failed",
            ),
        ]);
    }

    #[test]
    fn code2_rows_37_54_commit() {
        use super::txn::{ChainBreak as B, CommitError as E};
        use crate::fs_store::DurableWriteError as W;
        use crate::model::ErrorCode as M;
        use std::io::ErrorKind as K;
        code2(&[
            ("37", E::Poisoned.code(), "storage_durability_failed"),
            (
                "38",
                E::GenerationExhausted.code(),
                "freshness_generation_exhausted",
            ),
            ("39", E::SuccessorTooLarge.code(), "vault_capacity_exceeded"),
            (
                "40",
                E::SuccessorOpenFailed.code(),
                "freshness_successor_invalid",
            ),
            (
                "41",
                E::NotChained(B::VaultId).code(),
                "freshness_successor_invalid",
            ),
            (
                "42",
                E::NotChained(B::Mode).code(),
                "freshness_successor_invalid",
            ),
            (
                "43",
                E::NotChained(B::Generation).code(),
                "freshness_successor_invalid",
            ),
            (
                "44",
                E::NotChained(B::PredecessorAnchor).code(),
                "freshness_successor_invalid",
            ),
            (
                "45",
                E::NotChained(B::CheckpointKey).code(),
                "freshness_head_changed",
            ),
            (
                "46",
                E::NotChained(B::HeadChanged).code(),
                "freshness_head_changed",
            ),
            (
                "47",
                E::NotChained(B::HeadRead(K::PermissionDenied)).code(),
                "freshness_checkpoint_unreadable",
            ),
            (
                "48",
                E::StalePrepared(K::PermissionDenied).code(),
                "storage_durability_failed",
            ),
            (
                "49.IoWriteFailed",
                E::DurableWrite(W::Hygiene(M::IoWriteFailed)).code(),
                "io_write_failed",
            ),
            (
                "49.IoReadFailed",
                E::DurableWrite(W::Hygiene(M::IoReadFailed)).code(),
                "io_read_failed",
            ),
            (
                "49.UnsafePathSymlink",
                E::DurableWrite(W::Hygiene(M::UnsafePathSymlink)).code(),
                "unsafe_path_symlink",
            ),
            (
                "49.UnsafeParentPerms",
                E::DurableWrite(W::Hygiene(M::UnsafeParentPerms)).code(),
                "unsafe_parent_perms",
            ),
            (
                "50",
                E::DurableWrite(W::TempNotSibling).code(),
                "storage_durability_failed",
            ),
            (
                "51",
                E::DurableWrite(W::TempCreateOrWrite).code(),
                "storage_durability_failed",
            ),
            (
                "52",
                E::DurableWrite(W::FileFlush).code(),
                "storage_durability_failed",
            ),
            (
                "53",
                E::DurableWrite(W::Rename).code(),
                "storage_durability_failed",
            ),
            (
                "54",
                E::DurableWrite(W::DirFlush).code(),
                "storage_durability_failed",
            ),
        ]);
    }

    #[test]
    fn code2_rows_55_71_genesis() {
        use super::lock::LockError as L;
        use super::paths::CheckpointDirError as D;
        use super::recover::{RecoverError as R, Slot, Unauthenticated};
        use super::txn::{GenesisBreak as B, GenesisError as E};
        use crate::fs_store::DurableWriteError as W;
        use crate::model::ErrorCode as M;
        use std::io::ErrorKind as K;
        let durable = |e| E::DurableWrite(e).code();
        code2(&[
            ("55", E::B0TooLarge.code(), "vault_capacity_exceeded"),
            ("56", E::B0OpenFailed.code(), "freshness_successor_invalid"),
            (
                "57",
                E::NotGenesis(B::Generation).code(),
                "freshness_successor_invalid",
            ),
            (
                "58",
                E::NotGenesis(B::PredecessorAnchor).code(),
                "freshness_successor_invalid",
            ),
            ("59", E::NotLocalCheckpoint.code(), "anchor_unqualified"),
            (
                "60.RootNotAbsolute",
                E::CheckpointDir(D::RootNotAbsolute).code(),
                "freshness_state_dir_invalid",
            ),
            (
                "60.Symlink",
                E::CheckpointDir(D::Symlink).code(),
                "unsafe_path_symlink",
            ),
            (
                "60.NotADirectory",
                E::CheckpointDir(D::NotADirectory).code(),
                "freshness_state_dir_invalid",
            ),
            (
                "60.GroupOrWorldWritable",
                E::CheckpointDir(D::GroupOrWorldWritable).code(),
                "unsafe_parent_perms",
            ),
            (
                "60.Io",
                E::CheckpointDir(D::Io(K::Other)).code(),
                "io_write_failed",
            ),
            (
                "61.Open",
                E::Lock(L::Open(K::Other)).code(),
                "lock_open_failed",
            ),
            (
                "61.Contended",
                E::Lock(L::Contended).code(),
                "freshness_lineage_lock_contended",
            ),
            (
                "61.Flock",
                E::Lock(L::Flock(K::Other)).code(),
                "lock_failed",
            ),
            ("61.Stat", E::Lock(L::Stat(K::Other)).code(), "lock_failed"),
            (
                "61.InodeRetriesExhausted",
                E::Lock(L::InodeRetriesExhausted).code(),
                "freshness_lineage_lock_unstable",
            ),
            (
                "62",
                E::Recover(R::TempCleanup(K::Other)).code(),
                "io_write_failed",
            ),
            (
                "63",
                E::Recover(R::BlobRead {
                    slot: Slot::Current,
                    kind: K::Other,
                })
                .code(),
                "vault_read_failed",
            ),
            (
                "64",
                E::Recover(R::BlobTooLarge {
                    slot: Slot::Current,
                })
                .code(),
                "vault_file_oversized",
            ),
            (
                "65",
                E::Recover(R::OpenFailed {
                    which: Unauthenticated::Prepared,
                })
                .code(),
                "vault_exists",
            ),
            (
                "66",
                E::Recover(R::NotLocalCheckpoint).code(),
                "vault_exists",
            ),
            (
                "67",
                E::Recover(R::CheckpointRead(K::Other)).code(),
                "freshness_checkpoint_unreadable",
            ),
            (
                "68.Rename",
                E::Recover(R::DurableWrite(W::Rename)).code(),
                "storage_durability_failed",
            ),
            (
                "68.DirFlush",
                E::Recover(R::DurableWrite(W::DirFlush)).code(),
                "storage_durability_failed",
            ),
            ("69", E::LineagePresent.code(), "vault_exists"),
            (
                "70",
                E::CheckpointExists.code(),
                "freshness_checkpoint_exists",
            ),
            (
                "71.IoWriteFailed",
                durable(W::Hygiene(M::IoWriteFailed)),
                "io_write_failed",
            ),
            (
                "71.IoReadFailed",
                durable(W::Hygiene(M::IoReadFailed)),
                "io_read_failed",
            ),
            (
                "71.UnsafePathSymlink",
                durable(W::Hygiene(M::UnsafePathSymlink)),
                "unsafe_path_symlink",
            ),
            (
                "71.UnsafeParentPerms",
                durable(W::Hygiene(M::UnsafeParentPerms)),
                "unsafe_parent_perms",
            ),
            (
                "71.TempNotSibling",
                durable(W::TempNotSibling),
                "storage_durability_failed",
            ),
            (
                "71.TempCreateOrWrite",
                durable(W::TempCreateOrWrite),
                "storage_durability_failed",
            ),
            (
                "71.FileFlush",
                durable(W::FileFlush),
                "storage_durability_failed",
            ),
            ("71.Rename", durable(W::Rename), "storage_durability_failed"),
            (
                "71.DirFlush",
                durable(W::DirFlush),
                "storage_durability_failed",
            ),
        ]);
    }
}
