//! C07 T4.1 THE TRANSACTION for the LOCAL profile (NA-0787 F04-C07P S5): begin a session under the
//! lineage lock, commit one successor at a time in the contract's order, and create a lineage
//! (genesis), so that a crash at any point leaves the old vault or the new one -- never a mix,
//! never a silent rollback.
//!
//! - [`begin`]: a QUARANTINED open learns only the vault_id; then the D31 directory and the D32
//!   lock (FN1: after quarantined decrypt, before reading the authority); then S4's T4.3
//!   `recover()` under the lock. OPEN / PROMOTE give a [`Session`]; FREEZE is returned, unlocked.
//! - [`Session::commit`] (C1-C3, C5): every refusal comes BEFORE any write; then B4 (a stale
//!   prepared slot removed durably), C2 the prepared slot, C3 the checkpoint head = THE LOCAL
//!   COMMIT POINT, C5 the rename over current and the CHECKED directory flush. [`Committed`] is
//!   returned only after that flush. Any write failure POISONS the session (IC3 :279). C4 is the
//!   TPM's; C6 -- releasing effects, keyed by the committed identity -- is the caller's.
//! - [`genesis`] (C7): generation 0 on 32 zero bytes through C2, C3, C5; a cut between C2 and C3
//!   freezes CheckpointMissing on the next begin (DD-5 LITERAL: never auto-discarded).
//!
//! The Session holds NO key material (RULING_F04C07P R8): the checkpoint key of a commit comes,
//! for that one operation, from the successor's authenticated fields. It keeps the exact head bytes
//! it authenticated (public data), so a MAC failure at commit tells a changed head from a
//! successor under another key (S7b X5). FN3: one successor per
//! commit; there is no batching API. The writes are exactly: the checkpoint directory and the lock
//! file (begin, genesis), B4, C2, C3 and C5. LOCK ORDER: the caller's store lock `.qsc.lock` FIRST,
//! then the lineage lock taken here; the reverse order is FORBIDDEN. No client-code string is
//! allocated here: the marker spellings are S6's (RULING_F04C07P R6).

use super::checkpoint::{self, Checkpoint, CheckpointError, CorruptKind};
use super::lock::{LineageLock, LockError};
use super::paths::{self, CheckpointDirError};
use super::recover::{
    self, current_path, prepared_path, prepared_temp_path, FreezeKind, RecoverError, Recovered,
    Slot, Unauthenticated,
};
use super::{anchor, digest, open_regular, LineageFields, LineageOpen, OpenError, ProtectionMode};
use crate::fs_store::{self, DurableWriteError};
use crate::model::ConfigSource;
use std::fmt;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::process;
use std::sync::atomic::{AtomicU64, Ordering};

/// S7b X7 (F-10): the per-process counter that makes every head temp name unique
/// (`head.tmp.<pid>.<n>`), so two writes of one process never meet each other's temp.
static HEAD_TEMP_SEQ: AtomicU64 = AtomicU64::new(0);

/// S7c DF-2: the prepared temp's own counter, by the same discipline
/// (`vault.qsv.prepared.tmp.<pid>.<n>`), so a leftover of a reused pid never blocks C2.
static PREPARED_TEMP_SEQ: AtomicU64 = AtomicU64::new(0);

/// The exact identity of a committed vault (I04): what the caller's C6 releases and retries under.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Committed {
    pub(crate) generation: u64,
    pub(crate) digest: [u8; 32],
    pub(crate) anchor: [u8; 32],
}

/// How the session's committed vault was reached. After `Promoted` the caller resumes the
/// promoted vault's committed outbox (T4.3 :265; C6 is the caller's).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Resumed {
    Opened,
    Promoted,
    Genesis,
}

#[derive(Debug)]
pub(crate) enum BeginError {
    /// Nothing in the store: no vault_id is known, so no lock was taken and nothing was written.
    NoLineage,
    /// The quarantined open (before the lock) found no vault it could learn a vault_id from.
    Quarantine(RecoverError),
    CheckpointDir(CheckpointDirError),
    Lock(LockError),
    /// T4.3 under the lock failed outside a verdict (S4's typed errors).
    Recover(RecoverError),
    Freeze(FreezeKind),
    /// The authority under the lock is not the lineage the lock was taken for.
    LineageChanged,
}

/// Why a successor does not extend the committed vault (C1). Refused before any write.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ChainBreak {
    VaultId,
    Mode,
    Generation,
    PredecessorAnchor,
    /// The successor's checkpoint key does not authenticate the committed head.
    CheckpointKey,
    /// The head no longer names the committed vault (FN2 :259, local form).
    HeadChanged,
    HeadRead(io::ErrorKind),
}

#[derive(Debug)]
pub(crate) enum CommitError {
    /// A write of this session failed earlier (IC3 :279): drop it and begin() again.
    Poisoned,
    /// ER18 (:232): g + 1 would exceed u64.
    GenerationExhausted,
    /// Longer than the caller's bound; never shown to the opener (I06).
    SuccessorTooLarge,
    SuccessorOpenFailed,
    /// S7b X1: the successor authenticated but its payload cannot be read (a client defect).
    SuccessorMalformed,
    /// S7b X11: the lineage lock no longer holds the inode at its path. Refused first.
    Lock(LockError),
    NotChained(ChainBreak),
    /// B4: the stale prepared slot could not be removed.
    StalePrepared(io::ErrorKind),
    /// ER17 (:231): a checked write or flush failed. The session is poisoned.
    DurableWrite(DurableWriteError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenesisBreak {
    Generation,
    PredecessorAnchor,
}

#[derive(Debug)]
pub(crate) enum GenesisError {
    B0TooLarge,
    B0OpenFailed,
    /// S7b X1: B0 authenticated but its payload cannot be read.
    B0Malformed,
    NotGenesis(GenesisBreak),
    NotLocalCheckpoint,
    CheckpointDir(CheckpointDirError),
    Lock(LockError),
    Recover(RecoverError),
    /// The store already holds a lineage (open, promotable or frozen): genesis never overwrites.
    LineagePresent,
    /// A checkpoint head already exists for this vault_id: a replayed B0 cannot reset a lineage.
    CheckpointExists,
    /// ER17 (:231).
    DurableWrite(DurableWriteError),
}

/// An unlocked lineage: holds the lineage lock for its whole lifetime (FN1) and the committed
/// identity -- never a key and never the vault's bytes.
pub(crate) struct Session {
    lock: LineageLock,
    store_dir: PathBuf,
    checkpoint_dir: PathBuf,
    max_blob_len: usize,
    vault_id: [u8; 32],
    mode: ProtectionMode,
    committed: Committed,
    /// The exact head bytes that name `committed` (read at begin, or written here). Public.
    head: [u8; checkpoint::LEN],
    resumed: Resumed,
    stale_prepared: bool,
    poisoned: bool,
}

/// S7c DF-6: X8's shape -- the head bytes are never formatted, only their length; the lock by its path.
impl fmt::Debug for Session {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Session")
            .field("vault_id", &crate::hex_encode(&self.vault_id))
            .field("mode", &self.mode)
            .field("committed", &self.committed)
            .field("head_len", &self.head.len())
            .field("resumed", &self.resumed)
            .field("stale_prepared", &self.stale_prepared)
            .field("poisoned", &self.poisoned)
            .field("lock", &self.lock.path())
            .finish_non_exhaustive()
    }
}

/// Unlock a lineage (B). PRECONDITION: the caller holds its store lock `.qsc.lock` (taken FIRST).
pub(crate) fn begin(
    store_dir: &Path,
    state_root: &Path,
    max_blob_len: usize,
    opener: &dyn LineageOpen,
) -> Result<Session, BeginError> {
    // 1. Quarantined open: it yields the vault_id ONLY; nothing from it is authority.
    let vault_id = quarantine_vault_id(store_dir, max_blob_len, opener)
        .map_err(BeginError::Quarantine)?
        .ok_or(BeginError::NoLineage)?;
    // 2. The D31 directory and the D32 lock: begin()'s only own write, and not authority.
    let checkpoint_dir =
        paths::ensure_checkpoint_dir(state_root, &vault_id).map_err(BeginError::CheckpointDir)?;
    let lock = LineageLock::acquire(&checkpoint_dir).map_err(BeginError::Lock)?;
    // 3. T4.3 under the lock, for the lineage locked (S7b X10). Every return below that is not Ok
    //    drops (releases) the lock.
    let recovered = recover::recover(store_dir, state_root, max_blob_len, opener, Some(&vault_id))
        .map_err(|e| match e {
            RecoverError::LineageChanged => BeginError::LineageChanged,
            e => BeginError::Recover(e),
        })?;
    let (current, resumed, stale_prepared) = match recovered {
        Recovered::NoLineage => return Err(BeginError::NoLineage),
        Recovered::Freeze(kind) => return Err(BeginError::Freeze(kind)),
        Recovered::Open {
            current,
            stale_prepared,
        } => (current, Resumed::Opened, stale_prepared),
        Recovered::Promoted { current } => (current, Resumed::Promoted, false),
    };
    // 4. The authority must belong to the lineage the lock was taken for.
    if current.fields.vault_id != vault_id {
        return Err(BeginError::LineageChanged);
    }
    Ok(Session {
        lock,
        store_dir: store_dir.to_path_buf(),
        checkpoint_dir,
        max_blob_len,
        vault_id,
        mode: current.fields.protection_mode,
        committed: Committed {
            generation: current.fields.generation,
            digest: current.digest,
            anchor: current.anchor,
        },
        head: current.head,
        resumed,
        stale_prepared,
        poisoned: false,
    })
}

impl Session {
    pub(crate) fn committed(&self) -> Committed {
        self.committed
    }

    pub(crate) fn vault_id(&self) -> [u8; 32] {
        self.vault_id
    }

    pub(crate) fn protection_mode(&self) -> ProtectionMode {
        self.mode
    }

    pub(crate) fn resumed(&self) -> Resumed {
        self.resumed
    }

    /// A prepared slot classified uncommitted at begin(); the next commit removes it first (B4).
    pub(crate) fn stale_prepared(&self) -> bool {
        self.stale_prepared
    }

    pub(crate) fn is_poisoned(&self) -> bool {
        self.poisoned
    }

    /// Commit ONE successor (C1-C3, C5). `Committed` is returned only after C5's checked
    /// directory flush; the caller releases effects only then, keyed by it (C6).
    pub(crate) fn commit(
        &mut self,
        successor: &[u8],
        opener: &dyn LineageOpen,
    ) -> Result<Committed, CommitError> {
        // S7b X11 (F-09): the lock must still be the path's before anything else.
        self.lock.recheck().map_err(CommitError::Lock)?;
        if self.poisoned {
            return Err(CommitError::Poisoned);
        }
        let next = self
            .committed
            .generation
            .checked_add(1)
            .ok_or(CommitError::GenerationExhausted)?;
        if successor.len() > self.max_blob_len {
            return Err(CommitError::SuccessorTooLarge);
        }
        let fields = opener.open(successor).map_err(|e| match e {
            OpenError::Unauthenticated => CommitError::SuccessorOpenFailed,
            OpenError::Malformed => CommitError::SuccessorMalformed,
        })?;
        self.check_chain(&fields, next)
            .map_err(CommitError::NotChained)?;
        let d = digest(&self.vault_id, successor);
        let committed = Committed {
            generation: next,
            digest: d,
            anchor: anchor(&self.committed.anchor, &d),
        };
        match self.write_transaction(successor, &fields, committed) {
            Ok(head) => {
                self.committed = committed;
                self.head = head;
                Ok(committed)
            }
            Err(e) => {
                self.poisoned = true;
                Err(e)
            }
        }
    }

    /// C1 and FN2 (local form), before any write.
    fn check_chain(&self, fields: &LineageFields, next: u64) -> Result<(), ChainBreak> {
        if fields.vault_id != self.vault_id {
            return Err(ChainBreak::VaultId);
        }
        if fields.protection_mode != self.mode {
            return Err(ChainBreak::Mode);
        }
        if fields.generation != next {
            return Err(ChainBreak::Generation);
        }
        if fields.predecessor_anchor != self.committed.anchor {
            return Err(ChainBreak::PredecessorAnchor);
        }
        // The head must still name the committed vault, under the successor's own key.
        let file = match open_regular(&paths::head_path(&self.checkpoint_dir)) {
            Ok(file) => file,
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Err(ChainBreak::HeadChanged),
            Err(e) => return Err(ChainBreak::HeadRead(e.kind())),
        };
        let bytes = checkpoint::read_bounded(file).map_err(|e| ChainBreak::HeadRead(e.kind()))?;
        let head = match checkpoint::decode(&bytes, fields.checkpoint_mac_key(), self.mode) {
            Ok(head) => head,
            // S7b X5 (F-06): the SAME bytes this session authenticated no longer verify, so the
            // successor carries another key; different bytes mean the head itself changed.
            Err(CheckpointError::Corrupt(CorruptKind::Mac)) => {
                return Err(if bytes[..] == self.head[..] {
                    ChainBreak::CheckpointKey
                } else {
                    ChainBreak::HeadChanged
                })
            }
            Err(CheckpointError::Read(kind)) => return Err(ChainBreak::HeadRead(kind)),
            Err(_) => return Err(ChainBreak::HeadChanged),
        };
        let c = &self.committed;
        if head.vault_id != self.vault_id
            || head.generation != c.generation
            || head.digest != c.digest
            || head.anchor != c.anchor
        {
            return Err(ChainBreak::HeadChanged);
        }
        Ok(())
    }

    /// The writes, in the contract's order. Every cut point is a cfg(test) statement. Returns the
    /// head it wrote.
    fn write_transaction(
        &mut self,
        successor: &[u8],
        fields: &LineageFields,
        committed: Committed,
    ) -> Result<[u8; checkpoint::LEN], CommitError> {
        let pid = process::id();
        // B4 (:318): the stale slot goes, durably, BEFORE the new preparation.
        if self.stale_prepared {
            remove_stale_prepared(&self.store_dir)?;
            self.stale_prepared = false;
        }
        let head = checkpoint::encode(
            &Checkpoint {
                mode: self.mode,
                vault_id: self.vault_id,
                generation: committed.generation,
                digest: committed.digest,
                anchor: committed.anchor,
            },
            fields.checkpoint_mac_key(),
        );
        let prepared_temp = next_prepared_temp(&self.store_dir, pid);
        let head_temp = next_head_temp(&self.checkpoint_dir, pid);
        #[cfg(test)]
        super::cut(super::CutPoint::KB2, None);
        #[cfg(test)]
        super::cut(
            super::CutPoint::KM2,
            Some((prepared_temp.as_path(), successor)),
        );
        // C2 (:248).
        write_prepared(&self.store_dir, successor, &prepared_temp)
            .map_err(CommitError::DurableWrite)?;
        #[cfg(test)]
        super::cut(super::CutPoint::KA2, None);
        #[cfg(test)]
        super::cut(super::CutPoint::KM3, Some((head_temp.as_path(), &head[..])));
        // C3 (:249): THE LOCAL COMMIT POINT.
        write_head(&self.checkpoint_dir, &head, &head_temp).map_err(CommitError::DurableWrite)?;
        #[cfg(test)]
        super::cut(super::CutPoint::KA34, None);
        // C5 (:251): promote, then the CHECKED directory flush.
        rename_prepared_over_current(&self.store_dir).map_err(CommitError::DurableWrite)?;
        #[cfg(test)]
        super::cut(super::CutPoint::KM5, None);
        fs_store::sync_dir_checked(&self.store_dir).map_err(CommitError::DurableWrite)?;
        #[cfg(test)]
        super::cut(super::CutPoint::KA5, None);
        Ok(head)
    }
}

/// Create a lineage (C7). PRECONDITION: the caller holds its store lock `.qsc.lock` (FIRST): an
/// empty store has no vault_id, so only that lock serializes two geneses on one store directory.
pub(crate) fn genesis(
    store_dir: &Path,
    state_root: &Path,
    max_blob_len: usize,
    b0: &[u8],
    opener: &dyn LineageOpen,
) -> Result<Session, GenesisError> {
    if b0.len() > max_blob_len {
        return Err(GenesisError::B0TooLarge);
    }
    let fields = opener.open(b0).map_err(|e| match e {
        OpenError::Unauthenticated => GenesisError::B0OpenFailed,
        OpenError::Malformed => GenesisError::B0Malformed,
    })?;
    if fields.generation != 0 {
        return Err(GenesisError::NotGenesis(GenesisBreak::Generation));
    }
    if fields.predecessor_anchor != [0u8; 32] {
        return Err(GenesisError::NotGenesis(GenesisBreak::PredecessorAnchor));
    }
    if fields.protection_mode != ProtectionMode::LocalCheckpoint {
        return Err(GenesisError::NotLocalCheckpoint);
    }
    let vault_id = fields.vault_id;
    let checkpoint_dir =
        paths::ensure_checkpoint_dir(state_root, &vault_id).map_err(GenesisError::CheckpointDir)?;
    let lock = LineageLock::acquire(&checkpoint_dir).map_err(GenesisError::Lock)?;
    // S7c DF-1: B0's authenticated vault_id, so another lineage is refused before rule h (X10).
    match recover::recover(store_dir, state_root, max_blob_len, opener, Some(&vault_id))
        .map_err(GenesisError::Recover)?
    {
        Recovered::NoLineage => {}
        _ => return Err(GenesisError::LineagePresent),
    }
    match fs::symlink_metadata(paths::head_path(&checkpoint_dir)) {
        Err(e) if e.kind() == io::ErrorKind::NotFound => {}
        _ => return Err(GenesisError::CheckpointExists),
    }
    let pid = process::id();
    let d = digest(&vault_id, b0);
    let committed = Committed {
        generation: 0,
        digest: d,
        anchor: anchor(&[0u8; 32], &d),
    };
    let head = checkpoint::encode(
        &Checkpoint {
            mode: fields.protection_mode,
            vault_id,
            generation: 0,
            digest: d,
            anchor: committed.anchor,
        },
        fields.checkpoint_mac_key(),
    );
    // C2.
    let prepared_temp = next_prepared_temp(store_dir, pid);
    write_prepared(store_dir, b0, &prepared_temp).map_err(GenesisError::DurableWrite)?;
    #[cfg(test)]
    super::cut(super::CutPoint::GI1, None);
    // C3: the genesis checkpoint.
    let head_temp = next_head_temp(&checkpoint_dir, pid);
    write_head(&checkpoint_dir, &head, &head_temp).map_err(GenesisError::DurableWrite)?;
    #[cfg(test)]
    super::cut(super::CutPoint::GI2, None);
    // C5.
    rename_prepared_over_current(store_dir).map_err(GenesisError::DurableWrite)?;
    fs_store::sync_dir_checked(store_dir).map_err(GenesisError::DurableWrite)?;
    #[cfg(test)]
    super::cut(super::CutPoint::GI3, None);
    Ok(Session {
        lock,
        store_dir: store_dir.to_path_buf(),
        checkpoint_dir,
        max_blob_len,
        vault_id,
        mode: fields.protection_mode,
        committed,
        head,
        resumed: Resumed::Genesis,
        stale_prepared: false,
        poisoned: false,
    })
}

/// B.1: current first, else prepared; each read bounded before the opener sees it. `Ok(None)`:
/// neither slot exists.
fn quarantine_vault_id(
    store_dir: &Path,
    max_blob_len: usize,
    opener: &dyn LineageOpen,
) -> Result<Option<[u8; 32]>, RecoverError> {
    let mut present = [false; 2];
    let slots = [
        (current_path(store_dir), Slot::Current),
        (prepared_path(store_dir), Slot::Prepared),
    ];
    for (i, (path, slot)) in slots.iter().enumerate() {
        let Some(bytes) = read_bounded(path, max_blob_len, *slot)? else {
            continue;
        };
        present[i] = true;
        match opener.open(&bytes) {
            Ok(fields) => return Ok(Some(fields.vault_id)),
            // S7b X1 (F-01): authentic but unreadable -- never "unauthenticated", never counted.
            Err(OpenError::Malformed) => return Err(RecoverError::BlobMalformed { slot: *slot }),
            Err(OpenError::Unauthenticated) => {}
        }
    }
    let which = match present {
        [false, false] => return Ok(None),
        [true, true] => Unauthenticated::Both,
        [true, false] => Unauthenticated::Current,
        [false, true] => Unauthenticated::Prepared,
    };
    Err(RecoverError::OpenFailed { which })
}

/// At most `max_blob_len + 1` bytes are read, so a longer file is refused unread (I06).
fn read_bounded(
    path: &Path,
    max_blob_len: usize,
    slot: Slot,
) -> Result<Option<Vec<u8>>, RecoverError> {
    let file = match open_regular(path) {
        Ok(file) => file,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(e) => {
            return Err(RecoverError::BlobRead {
                slot,
                kind: e.kind(),
            })
        }
    };
    let limit = u64::try_from(max_blob_len)
        .unwrap_or(u64::MAX)
        .saturating_add(1);
    let mut bytes = Vec::new();
    file.take(limit)
        .read_to_end(&mut bytes)
        .map_err(|e| RecoverError::BlobRead {
            slot,
            kind: e.kind(),
        })?;
    if bytes.len() > max_blob_len {
        return Err(RecoverError::BlobTooLarge { slot });
    }
    Ok(Some(bytes))
}

/// B4: remove the stale prepared slot and make the removal durable (checked directory flush).
fn remove_stale_prepared(store_dir: &Path) -> Result<(), CommitError> {
    match fs::remove_file(prepared_path(store_dir)) {
        Ok(()) => {}
        Err(e) if e.kind() == io::ErrorKind::NotFound => {}
        Err(e) => return Err(CommitError::StalePrepared(e.kind())),
    }
    fs_store::sync_dir_checked(store_dir).map_err(CommitError::DurableWrite)
}

/// C2 through S2's checked primitive, through the caller's `next_prepared_temp` (D30 :332).
fn write_prepared(store_dir: &Path, bytes: &[u8], temp: &Path) -> Result<(), DurableWriteError> {
    fs_store::write_file_durable(
        &prepared_path(store_dir),
        bytes,
        temp,
        ConfigSource::EnvOverride,
    )
}

/// This write's prepared temp: `vault.qsv.prepared.tmp.<pid>.<n>` (D30 :332, made unique by S7c DF-2).
fn next_prepared_temp(store_dir: &Path, pid: u32) -> PathBuf {
    let seq = PREPARED_TEMP_SEQ.fetch_add(1, Ordering::Relaxed);
    prepared_temp_path(store_dir, pid, seq)
}

/// This write's head temp: `head.tmp.<pid>.<n>` (D31 :333, made unique by S7b X7).
fn next_head_temp(checkpoint_dir: &Path, pid: u32) -> PathBuf {
    let seq = HEAD_TEMP_SEQ.fetch_add(1, Ordering::Relaxed);
    paths::head_temp_path(checkpoint_dir, pid, seq)
}

/// C3 through S2's checked primitive, through the caller's `next_head_temp`.
fn write_head(checkpoint_dir: &Path, head: &[u8], temp: &Path) -> Result<(), DurableWriteError> {
    fs_store::write_file_durable(
        &paths::head_path(checkpoint_dir),
        head,
        temp,
        ConfigSource::EnvOverride,
    )
}

/// C5's rename; its directory flush is the caller's next, CHECKED, step.
fn rename_prepared_over_current(store_dir: &Path) -> Result<(), DurableWriteError> {
    fs::rename(prepared_path(store_dir), current_path(store_dir))
        .map_err(|_| DurableWriteError::Rename)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::freshness::{cut, run_child, CutPoint, CUT_EXIT, FIXTURE_ENV};
    use crate::fs_store::{arm_durable_flush_fault, DurableFlushPoint};
    use crate::model::{LockGuard, LockMode};
    use chacha20poly1305::aead::{Aead, KeyInit};
    use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
    use rand_core::{OsRng, RngCore};
    use sha2::{Digest as _, Sha256};
    use std::cell::{Cell, RefCell};
    use std::fs::OpenOptions;
    use std::io::Write;
    use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
    use std::os::unix::io::AsRawFd;
    use zeroize::Zeroizing;

    /// The committed generation of the synthetic fixture; the successor is N + 1.
    const N: u64 = 2;
    const MAX: usize = 1 << 20;
    const ZERO: [u8; 32] = [0u8; 32];
    const FILLER: usize = 48;
    const PLAIN: usize = 32 + 8 + 32 + 32 + 1 + FILLER;
    const CHILD: &str = "freshness::txn::tests::cut_child";

    fn random32() -> [u8; 32] {
        let mut b = [0u8; 32];
        OsRng.fill_bytes(&mut b);
        b
    }

    /// THE TEST OPENER (S4's fixture, copied: recover.rs's test module is private to it). A fixture
    /// for the blob, not a provider: nonce(12) || ChaCha20-Poly1305(vault_id || generation BE ||
    /// predecessor_anchor || Kc || profile byte || random filler) under a random per-test key. The
    /// key is also written, 0600, into the test's own tempdir so a cut child can open the blobs.
    /// A tag failure -> Unauthenticated; an AUTHENTIC plaintext of the wrong length or with an
    /// unknown profile byte -> Malformed (S7b X1). `hook` (S7 read PR10) runs inside the n-th open.
    struct TestOpener {
        key: [u8; 32],
        aead: ChaCha20Poly1305,
        calls: Cell<u32>,
        hook: Hook,
    }

    /// A closure to run inside the n-th open() call.
    type Hook = RefCell<Option<(u32, Box<dyn Fn()>)>>;

    impl TestOpener {
        fn new() -> Self {
            Self::from_key(random32())
        }

        fn from_key(key: [u8; 32]) -> Self {
            Self {
                key,
                aead: ChaCha20Poly1305::new(Key::from_slice(&key)),
                calls: Cell::new(0),
                hook: RefCell::new(None),
            }
        }

        fn seal(
            &self,
            vault_id: &[u8; 32],
            generation: u64,
            pred: &[u8; 32],
            kc: &[u8; 32],
            profile: u8,
        ) -> Vec<u8> {
            let mut plain = Vec::with_capacity(PLAIN);
            plain.extend_from_slice(vault_id);
            plain.extend_from_slice(&generation.to_be_bytes());
            plain.extend_from_slice(pred);
            plain.extend_from_slice(kc);
            plain.push(profile);
            let mut filler = [0u8; FILLER];
            OsRng.fill_bytes(&mut filler);
            plain.extend_from_slice(&filler);
            let mut nonce = [0u8; 12];
            OsRng.fill_bytes(&mut nonce);
            let mut out = nonce.to_vec();
            out.extend(
                self.aead
                    .encrypt(Nonce::from_slice(&nonce), plain.as_slice())
                    .expect("seal"),
            );
            out
        }
    }

    impl LineageOpen for TestOpener {
        fn open(&self, blob: &[u8]) -> Result<LineageFields, OpenError> {
            self.calls.set(self.calls.get() + 1);
            if let Some((at, hook)) = self.hook.borrow().as_ref() {
                if *at == self.calls.get() {
                    hook();
                }
            }
            if blob.len() < 12 {
                return Err(OpenError::Unauthenticated);
            }
            let plain = self
                .aead
                .decrypt(Nonce::from_slice(&blob[..12]), &blob[12..])
                .map_err(|_| OpenError::Unauthenticated)?;
            if plain.len() != PLAIN {
                return Err(OpenError::Malformed);
            }
            let field = |at: usize| -> [u8; 32] { plain[at..at + 32].try_into().expect("32") };
            let mut generation = [0u8; 8];
            generation.copy_from_slice(&plain[32..40]);
            Ok(LineageFields {
                vault_id: field(0),
                generation: u64::from_be_bytes(generation),
                predecessor_anchor: field(40),
                checkpoint_mac_key: Zeroizing::new(field(72)),
                protection_mode: ProtectionMode::from_profile_byte(plain[104])
                    .ok_or(OpenError::Malformed)?,
            })
        }
    }

    struct Blob {
        bytes: Vec<u8>,
        generation: u64,
        digest: [u8; 32],
        anchor: [u8; 32],
    }

    impl Blob {
        fn id(&self) -> Committed {
            Committed {
                generation: self.generation,
                digest: self.digest,
                anchor: self.anchor,
            }
        }
    }

    struct Lineage {
        opener: TestOpener,
        vault_id: [u8; 32],
        kc: [u8; 32],
    }

    impl Lineage {
        fn new() -> Self {
            Self {
                opener: TestOpener::new(),
                vault_id: random32(),
                kc: random32(),
            }
        }

        /// A fresh authentic local blob; the random filler makes two calls give two blobs.
        fn blob(&self, generation: u64, pred: &[u8; 32]) -> Blob {
            self.blob_with(self.vault_id, generation, pred, self.kc, 1)
        }

        fn blob_with(
            &self,
            vault_id: [u8; 32],
            generation: u64,
            pred: &[u8; 32],
            kc: [u8; 32],
            profile: u8,
        ) -> Blob {
            let bytes = self.opener.seal(&vault_id, generation, pred, &kc, profile);
            let d = digest(&vault_id, &bytes);
            Blob {
                anchor: anchor(pred, &d),
                digest: d,
                bytes,
                generation,
            }
        }

        /// Generations 0..=n, each on its predecessor's anchor (genesis on 32 zero bytes).
        fn chain(&self, n: u64) -> Vec<Blob> {
            let mut out: Vec<Blob> = Vec::new();
            for g in 0..=n {
                let pred = out.last().map_or(ZERO, |b| b.anchor);
                out.push(self.blob(g, &pred));
            }
            out
        }

        fn head(&self, b: &Blob) -> Vec<u8> {
            let ck = Checkpoint {
                mode: ProtectionMode::LocalCheckpoint,
                vault_id: self.vault_id,
                generation: b.generation,
                digest: b.digest,
                anchor: b.anchor,
            };
            checkpoint::encode(&ck, &self.kc).to_vec()
        }
    }

    fn private_dir(path: &Path) {
        fs::create_dir_all(path).expect("mkdir");
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).expect("chmod 0700");
    }

    fn id_line(c: Committed) -> String {
        format!("{}:{}", c.generation, crate::hex_encode(&c.digest))
    }

    fn ledger_lines(ledger: &Path) -> Vec<String> {
        match fs::read_to_string(ledger) {
            Ok(s) => s.lines().map(str::to_owned).collect(),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Vec::new(),
            Err(e) => panic!("ledger: {e}"),
        }
    }

    fn append_line(ledger: &Path, line: &str) {
        let mut f = OpenOptions::new()
            .create(true)
            .append(true)
            .open(ledger)
            .expect("ledger open");
        writeln!(f, "{line}").expect("ledger append");
    }

    /// HARNESS C6: release the effect of a committed identity, exactly once per identity.
    fn release(ledger: &Path, c: Committed) {
        let line = id_line(c);
        if !ledger_lines(ledger).contains(&line) {
            append_line(ledger, &line);
        }
    }

    /// After an OPEN / PROMOTE: resume the committed outbox from committed state (keyed, TD-1).
    fn resume(ledger: &Path, c: Committed) {
        let line = id_line(c);
        if !ledger_lines(ledger).contains(&line) {
            append_line(ledger, &line);
        }
    }

    type Tree = Vec<(PathBuf, Option<[u8; 32]>)>;

    /// Every entry under `dir` (relative path; a file's sha256, None for a directory), sorted,
    /// without `excluded`.
    fn tree(dir: &Path, excluded: Option<&Path>) -> Tree {
        fn walk(root: &Path, dir: &Path, excluded: Option<&Path>, out: &mut Tree) {
            let Ok(entries) = fs::read_dir(dir) else {
                return;
            };
            for entry in entries {
                let path = entry.expect("entry").path();
                if Some(path.as_path()) == excluded {
                    continue;
                }
                let rel = path.strip_prefix(root).expect("under root").to_path_buf();
                if fs::symlink_metadata(&path).expect("stat").is_dir() {
                    out.push((rel, None));
                    walk(root, &path, excluded, out);
                } else {
                    let h: [u8; 32] = Sha256::digest(fs::read(&path).expect("read")).into();
                    out.push((rel, Some(h)));
                }
            }
        }
        let mut out = Vec::new();
        walk(dir, dir, excluded, &mut out);
        out.sort();
        out
    }

    struct Fx {
        root: tempfile::TempDir,
        store: PathBuf,
        state: PathBuf,
        lin: Lineage,
    }

    impl Fx {
        fn empty() -> Self {
            let root = tempfile::tempdir().expect("tempdir");
            private_dir(root.path());
            let store = root.path().join("store");
            let state = root.path().join("state");
            private_dir(&store);
            private_dir(&state);
            private_dir(&root.path().join("ledger"));
            Self {
                root,
                store,
                state,
                lin: Lineage::new(),
            }
        }

        /// SYNTHETIC committed state at generation n (S4 style): current = chain[n], its head, and
        /// the effects of generations 0..=n already released.
        fn at(n: u64) -> (Self, Vec<Blob>) {
            let fx = Self::empty();
            let chain = fx.lin.chain(n);
            let last = &chain[n as usize];
            fx.put_current(&last.bytes);
            fx.put_head(&fx.lin.head(last));
            for b in &chain {
                release(&fx.ledger(), b.id());
            }
            (fx, chain)
        }

        fn ledger(&self) -> PathBuf {
            self.root.path().join("ledger").join("effects")
        }

        fn ckdir(&self) -> PathBuf {
            paths::checkpoint_dir(&self.state, &self.lin.vault_id)
        }

        fn head_path(&self) -> PathBuf {
            paths::head_path(&self.ckdir())
        }

        fn head_bytes(&self) -> Option<Vec<u8>> {
            fs::read(self.head_path()).ok()
        }

        fn put_current(&self, bytes: &[u8]) {
            fs::write(current_path(&self.store), bytes).expect("write current");
        }

        fn put_prepared(&self, bytes: &[u8]) {
            fs::write(prepared_path(&self.store), bytes).expect("write prepared");
        }

        fn put_head(&self, head: &[u8]) {
            paths::ensure_checkpoint_dir(&self.state, &self.lin.vault_id).expect("checkpoint dir");
            fs::write(self.head_path(), head).expect("write head");
        }

        fn begin(&self) -> Result<Session, BeginError> {
            begin(&self.store, &self.state, MAX, &self.lin.opener)
        }

        fn trees(&self) -> (Tree, Tree) {
            (tree(&self.store, None), tree(&self.state, None))
        }

        /// Both trees without exactly the D32 lock file.
        fn trees_but_lock(&self) -> (Tree, Tree) {
            let lock = paths::lock_path(&self.ckdir());
            (
                tree(&self.store, Some(&lock)),
                tree(&self.state, Some(&lock)),
            )
        }

        fn write_fixture(&self, mode: &str, input: &[u8]) {
            let dir = self.root.path().join("fixture");
            private_dir(&dir);
            let mut key = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .mode(0o600)
                .open(dir.join("opener.key"))
                .expect("key file");
            key.write_all(&self.lin.opener.key).expect("key write");
            fs::write(dir.join("mode"), mode).expect("mode");
            fs::write(dir.join("input"), input).expect("input");
        }

        /// A cut row's child: exit 86 is asserted FIRST, before any outcome is read.
        fn cut_child(&self, mode: &str, input: &[u8], point: CutPoint) -> u32 {
            self.write_fixture(mode, input);
            let (status, pid) = run_child(CHILD, Some(point), self.root.path());
            assert_eq!(
                status.code(),
                Some(CUT_EXIT),
                "the child must die at {} with exit 86; got {status:?}",
                point.name()
            );
            pid
        }
    }

    #[derive(Clone, Copy)]
    enum Expect {
        Open {
            id: Committed,
            promoted_first: bool,
            stale: bool,
        },
        Freeze(FreezeKind),
    }

    fn open(b: &Blob) -> Expect {
        Expect::Open {
            id: b.id(),
            promoted_first: false,
            stale: false,
        }
    }

    fn promoted(b: &Blob) -> Expect {
        Expect::Open {
            id: b.id(),
            promoted_first: true,
            stale: false,
        }
    }

    fn stale(b: &Blob) -> Expect {
        Expect::Open {
            id: b.id(),
            promoted_first: false,
            stale: true,
        }
    }

    /// "Restart" twice: begin() in this process (new relative to any cut child), the sealed
    /// outcome each time, the harness's resume on OPEN / PROMOTE, and invariants (iv) and (v).
    fn restart_twice(fx: &Fx, expect: Expect) {
        for round in 0..2 {
            let head_before = fx.head_bytes();
            match (fx.begin(), expect) {
                (
                    Ok(s),
                    Expect::Open {
                        id,
                        promoted_first,
                        stale,
                    },
                ) => {
                    let want = if round == 0 && promoted_first {
                        Resumed::Promoted
                    } else {
                        Resumed::Opened
                    };
                    assert_eq!(s.resumed(), want, "round {round}");
                    assert_eq!(s.committed(), id, "round {round}: the committed identity");
                    assert_eq!(s.stale_prepared(), stale, "round {round}: stale prepared");
                    assert_eq!(s.vault_id(), fx.lin.vault_id);
                    assert_eq!(s.protection_mode(), ProtectionMode::LocalCheckpoint);
                    assert!(!s.is_poisoned());
                    resume(&fx.ledger(), s.committed());
                    // (iv): the head names exactly the committed vault now in current.
                    let head = checkpoint::decode(
                        &fx.head_bytes().expect("head"),
                        &fx.lin.kc,
                        ProtectionMode::LocalCheckpoint,
                    )
                    .expect("head decodes");
                    assert_eq!(
                        (head.vault_id, head.generation, head.digest, head.anchor),
                        (fx.lin.vault_id, id.generation, id.digest, id.anchor),
                        "(iv) round {round}"
                    );
                    let current = fs::read(current_path(&fx.store)).expect("current");
                    assert_eq!(digest(&fx.lin.vault_id, &current), id.digest, "(iv)");
                }
                (Err(BeginError::Freeze(kind)), Expect::Freeze(want)) => {
                    assert_eq!(kind, want, "round {round}");
                }
                (other, _) => panic!("round {round}: unexpected {other:?}"),
            }
            assert_eq!(
                fx.head_bytes(),
                head_before,
                "(v) begin never writes the head"
            );
        }
    }

    /// (ii) first, then the exact sealed list, which gives (i) and (iii).
    fn assert_ledger(fx: &Fx, want: &[&Blob]) {
        let got = ledger_lines(&fx.ledger());
        let mut unique = got.clone();
        unique.sort();
        unique.dedup();
        assert_eq!(unique.len(), got.len(), "(ii) an effect was released twice");
        let want: Vec<String> = want.iter().map(|b| id_line(b.id())).collect();
        assert_eq!(got, want, "(i)/(iii) the effects ledger");
    }

    fn upto(chain: &[Blob]) -> Vec<&Blob> {
        chain.iter().collect()
    }

    fn with<'a>(chain: &'a [Blob], more: &'a Blob) -> Vec<&'a Blob> {
        let mut v = upto(chain);
        v.push(more);
        v
    }

    fn kind<T: std::fmt::Debug, E: std::fmt::Debug>(r: &Result<T, E>) -> String {
        match r {
            Ok(v) => format!("Ok({v:?})"),
            Err(e) => format!("{e:?}"),
        }
    }

    fn prepared_family(store: &Path) -> usize {
        fs::read_dir(store)
            .expect("store")
            .filter(|e| {
                e.as_ref()
                    .expect("entry")
                    .file_name()
                    .to_string_lossy()
                    .starts_with("vault.qsv.prepared")
            })
            .count()
    }

    /// THE CUT CHILD. Runs only when re-executed by `run_child` (QSL_FRESHNESS_FIXTURE set);
    /// otherwise, e.g. under --include-ignored, it passes as a no-op.
    #[test]
    #[ignore = "a process-cut child: run only by run_child"]
    fn cut_child() {
        let Some(root) = std::env::var_os(FIXTURE_ENV).map(PathBuf::from) else {
            return;
        };
        let dir = root.join("fixture");
        let key: [u8; 32] = fs::read(dir.join("opener.key"))
            .expect("key")
            .try_into()
            .expect("32-byte key");
        let opener = TestOpener::from_key(key);
        let mode = fs::read_to_string(dir.join("mode")).expect("mode");
        let input = fs::read(dir.join("input")).expect("input");
        let (store, state) = (root.join("store"), root.join("state"));
        let ledger = root.join("ledger").join("effects");
        match mode.as_str() {
            "commit" => {
                let mut s = begin(&store, &state, MAX, &opener).expect("child begin");
                let c = s.commit(&input, &opener).expect("child commit");
                release(&ledger, c);
                cut(CutPoint::KA6, None);
            }
            "genesis" => {
                let s = genesis(&store, &state, MAX, &input, &opener).expect("child genesis");
                release(&ledger, s.committed());
            }
            "rc1" => {
                // S7b X6: begin() first CREATES the lock file (one checked flush), then promotes.
                arm_durable_flush_fault(DurableFlushPoint::Dir, 1);
                match begin(&store, &state, MAX, &opener) {
                    Err(BeginError::Recover(RecoverError::DurableWrite(
                        DurableWriteError::DirFlush,
                    ))) => cut(CutPoint::RC1, None),
                    other => panic!("rc1 child: {other:?}"),
                }
            }
            "lk1" => {
                let outcome = match begin(&root.join("store_copy"), &state, MAX, &opener) {
                    Err(BeginError::Lock(LockError::Contended)) => {
                        format!("contended calls={}", opener.calls.get())
                    }
                    Ok(s) => format!("open {}", s.committed().generation),
                    Err(e) => format!("other {e:?}"),
                };
                fs::write(dir.join("outcome"), outcome).expect("outcome");
            }
            other => panic!("unknown child mode {other}"),
        }
    }

    #[test]
    fn g_genesis_opens_generation_zero_effect_once() {
        let fx = Fx::empty();
        let b0 = fx.lin.blob(0, &ZERO);
        let s = genesis(&fx.store, &fx.state, MAX, &b0.bytes, &fx.lin.opener).expect("genesis");
        assert_eq!(s.resumed(), Resumed::Genesis);
        assert_eq!(s.committed(), b0.id());
        release(&fx.ledger(), s.committed());
        drop(s);
        restart_twice(&fx, open(&b0));
        assert_ledger(&fx, &[&b0]);
    }

    #[test]
    fn t1_commit_opens_next_effect_once() {
        let (fx, chain) = Fx::at(N);
        let s3 = fx.lin.blob(N + 1, &chain[N as usize].anchor);
        let mut s = fx.begin().expect("begin");
        assert_eq!(s.committed(), chain[N as usize].id());
        let c = s.commit(&s3.bytes, &fx.lin.opener).expect("commit");
        assert_eq!(c, s3.id());
        assert_eq!(s.committed(), s3.id());
        release(&fx.ledger(), c);
        drop(s);
        restart_twice(&fx, open(&s3));
        assert_ledger(&fx, &with(&chain, &s3));
    }

    #[test]
    fn t1b_second_commit_on_one_session_effects_once() {
        let (fx, chain) = Fx::at(N);
        let s3 = fx.lin.blob(N + 1, &chain[N as usize].anchor);
        let s4 = fx.lin.blob(N + 2, &s3.anchor);
        let mut s = fx.begin().expect("begin");
        let c3 = s.commit(&s3.bytes, &fx.lin.opener).expect("commit 3");
        release(&fx.ledger(), c3);
        let c4 = s.commit(&s4.bytes, &fx.lin.opener).expect("commit 4");
        release(&fx.ledger(), c4);
        assert_eq!((c3, c4), (s3.id(), s4.id()));
        drop(s);
        restart_twice(&fx, open(&s4));
        let mut want = with(&chain, &s3);
        want.push(&s4);
        assert_ledger(&fx, &want);
    }

    #[test]
    fn k_b2_cut_before_prepare_opens_n() {
        let (fx, chain) = Fx::at(N);
        let s3 = fx.lin.blob(N + 1, &chain[N as usize].anchor);
        let pid = fx.cut_child("commit", &s3.bytes, CutPoint::KB2);
        assert!(!prepared_path(&fx.store).exists());
        assert!(!prepared_temp_path(&fx.store, pid, 0).exists());
        restart_twice(&fx, open(&chain[N as usize]));
        assert_ledger(&fx, &upto(&chain));
    }

    #[test]
    fn k_m2_cut_inside_prepare_opens_n_dead_temp_removed() {
        let (fx, chain) = Fx::at(N);
        let s3 = fx.lin.blob(N + 1, &chain[N as usize].anchor);
        let pid = fx.cut_child("commit", &s3.bytes, CutPoint::KM2);
        // The child's first prepared write: its counter's first value (S7c DF-2).
        let temp = prepared_temp_path(&fx.store, pid, 0);
        assert_eq!(fs::read(&temp).expect("the dead child's temp"), s3.bytes);
        assert!(!prepared_path(&fx.store).exists());
        restart_twice(&fx, open(&chain[N as usize]));
        assert!(
            !temp.exists(),
            "(:270) a dead pid's prepared temp is removed"
        );
        assert_ledger(&fx, &upto(&chain));
    }

    #[test]
    fn k_a2_cut_after_prepare_opens_n_stale_reported() {
        let (fx, chain) = Fx::at(N);
        let s3 = fx.lin.blob(N + 1, &chain[N as usize].anchor);
        fx.cut_child("commit", &s3.bytes, CutPoint::KA2);
        assert_eq!(
            fs::read(prepared_path(&fx.store)).expect("prepared"),
            s3.bytes
        );
        restart_twice(&fx, stale(&chain[N as usize]));
        assert_eq!(
            fs::read(prepared_path(&fx.store)).expect("prepared kept"),
            s3.bytes,
            "reported, never opened, removed only by the next commit (B4)"
        );
        assert_ledger(&fx, &upto(&chain));
    }

    #[test]
    fn k_m3_cut_inside_checkpoint_opens_n_temp_not_authority() {
        let (fx, chain) = Fx::at(N);
        let s3 = fx.lin.blob(N + 1, &chain[N as usize].anchor);
        let pid = fx.cut_child("commit", &s3.bytes, CutPoint::KM3);
        // The child's first head write: its counter's first value (S7b X7).
        let temp = paths::head_temp_path(&fx.ckdir(), pid, 0);
        let bytes = fs::read(&temp).expect("the dead child's head temp");
        assert_eq!(bytes.len(), checkpoint::LEN);
        restart_twice(&fx, stale(&chain[N as usize]));
        assert!(
            !temp.exists(),
            "a checkpoint temp is never authority, and a dead pid's is removed (S7b X7, D40)"
        );
        assert_ledger(&fx, &upto(&chain));
    }

    #[test]
    fn k_a34_cut_after_checkpoint_promotes_effect_once() {
        let (fx, chain) = Fx::at(N);
        let s3 = fx.lin.blob(N + 1, &chain[N as usize].anchor);
        fx.cut_child("commit", &s3.bytes, CutPoint::KA34);
        restart_twice(&fx, promoted(&s3));
        assert!(!prepared_path(&fx.store).exists());
        assert_ledger(&fx, &with(&chain, &s3));
    }

    #[test]
    fn k_m5_cut_after_rename_opens_next() {
        let (fx, chain) = Fx::at(N);
        let s3 = fx.lin.blob(N + 1, &chain[N as usize].anchor);
        fx.cut_child("commit", &s3.bytes, CutPoint::KM5);
        restart_twice(&fx, open(&s3));
        assert_ledger(&fx, &with(&chain, &s3));
    }

    #[test]
    fn k_a5_cut_before_c6_opens_next_outbox_resumed() {
        let (fx, chain) = Fx::at(N);
        let s3 = fx.lin.blob(N + 1, &chain[N as usize].anchor);
        fx.cut_child("commit", &s3.bytes, CutPoint::KA5);
        assert!(!ledger_lines(&fx.ledger()).contains(&id_line(s3.id())));
        restart_twice(&fx, open(&s3));
        assert_ledger(&fx, &with(&chain, &s3));
    }

    #[test]
    fn k_a6_cut_after_c6_opens_next_no_rerelease() {
        let (fx, chain) = Fx::at(N);
        let s3 = fx.lin.blob(N + 1, &chain[N as usize].anchor);
        fx.cut_child("commit", &s3.bytes, CutPoint::KA6);
        assert!(ledger_lines(&fx.ledger()).contains(&id_line(s3.id())));
        restart_twice(&fx, open(&s3));
        assert_ledger(&fx, &with(&chain, &s3));
    }

    #[test]
    fn f1_prepared_flush_failure_aborts_and_poisons() {
        let (fx, chain) = Fx::at(N);
        let s3 = fx.lin.blob(N + 1, &chain[N as usize].anchor);
        let mut s = fx.begin().expect("begin");
        arm_durable_flush_fault(DurableFlushPoint::File, 0);
        let r = s.commit(&s3.bytes, &fx.lin.opener);
        assert_eq!(kind(&r), "DurableWrite(FileFlush)");
        let before = fx.trees();
        let r = s.commit(&s3.bytes, &fx.lin.opener);
        assert_eq!(
            kind(&r),
            "Poisoned",
            "IC3: no further commit on this handle"
        );
        assert_eq!(fx.trees(), before);
        drop(s);
        assert!(!prepared_path(&fx.store).exists());
        assert_eq!(
            prepared_family(&fx.store),
            0,
            "no temp of either shape is left"
        );
        restart_twice(&fx, open(&chain[N as usize]));
        assert_ledger(&fx, &upto(&chain));
    }

    #[test]
    fn f2_dir_flush_failure_after_promote_reconciles() {
        let (fx, chain) = Fx::at(N);
        let s3 = fx.lin.blob(N + 1, &chain[N as usize].anchor);
        let mut s = fx.begin().expect("begin");
        // Directory flushes of a commit: C2's, C3's, then C5's -- the third fails.
        arm_durable_flush_fault(DurableFlushPoint::Dir, 2);
        let r = s.commit(&s3.bytes, &fx.lin.opener);
        assert_eq!(kind(&r), "DurableWrite(DirFlush)");
        drop(s);
        restart_twice(&fx, open(&s3));
        assert_ledger(&fx, &with(&chain, &s3));
    }

    #[test]
    fn gi1_genesis_cut_before_checkpoint_freezes_missing() {
        let fx = Fx::empty();
        let b0 = fx.lin.blob(0, &ZERO);
        fx.cut_child("genesis", &b0.bytes, CutPoint::GI1);
        assert_eq!(fs::read(prepared_path(&fx.store)).expect("B0"), b0.bytes);
        assert!(!current_path(&fx.store).exists());
        assert!(!fx.head_path().exists());
        restart_twice(&fx, Expect::Freeze(FreezeKind::CheckpointMissing));
        assert_eq!(
            fs::read(prepared_path(&fx.store)).expect("B0 kept"),
            b0.bytes,
            "DD-5 LITERAL: never auto-discarded"
        );
        assert!(!current_path(&fx.store).exists());
        assert!(!fx.head_path().exists(), "never rebuilt");
        let before = fx.trees();
        let r = genesis(&fx.store, &fx.state, MAX, &b0.bytes, &fx.lin.opener);
        assert_eq!(kind(&r), "LineagePresent");
        assert_eq!(fx.trees(), before);
        assert_ledger(&fx, &[]);
    }

    #[test]
    fn gi2_genesis_cut_after_checkpoint_promotes_zero() {
        let fx = Fx::empty();
        let b0 = fx.lin.blob(0, &ZERO);
        fx.cut_child("genesis", &b0.bytes, CutPoint::GI2);
        restart_twice(&fx, promoted(&b0));
        assert_ledger(&fx, &[&b0]);
    }

    #[test]
    fn gi3_genesis_cut_after_promote_opens_zero() {
        let fx = Fx::empty();
        let b0 = fx.lin.blob(0, &ZERO);
        fx.cut_child("genesis", &b0.bytes, CutPoint::GI3);
        restart_twice(&fx, open(&b0));
        assert_ledger(&fx, &[&b0]);
    }

    #[test]
    fn rc1_recovery_promotion_cut_under_lock_opens_next() {
        let (fx, chain) = Fx::at(N);
        let s3 = fx.lin.blob(N + 1, &chain[N as usize].anchor);
        fx.put_prepared(&s3.bytes);
        fx.put_head(&fx.lin.head(&s3));
        fx.cut_child("rc1", &[], CutPoint::RC1);
        assert_eq!(
            fs::read(current_path(&fx.store)).expect("current"),
            s3.bytes
        );
        assert!(!prepared_path(&fx.store).exists());
        restart_twice(&fx, open(&s3));
        assert_ledger(&fx, &with(&chain, &s3));
    }

    #[test]
    fn ic3_l_poisoned_handle_refuses_before_any_write() {
        let (fx, chain) = Fx::at(N);
        let s3 = fx.lin.blob(N + 1, &chain[N as usize].anchor);
        let mut s = fx.begin().expect("begin");
        arm_durable_flush_fault(DurableFlushPoint::Dir, 2);
        let r = s.commit(&s3.bytes, &fx.lin.opener);
        assert_eq!(kind(&r), "DurableWrite(DirFlush)");
        let before = fx.trees();
        let r = s.commit(&s3.bytes, &fx.lin.opener);
        assert_eq!(kind(&r), "Poisoned");
        assert_eq!(fx.trees(), before, "refused before any write");
        drop(s);
        restart_twice(&fx, open(&s3));
        assert_ledger(&fx, &with(&chain, &s3));
    }

    #[test]
    fn b4_1_stale_prepared_removed_before_new_preparation() {
        let (fx, chain) = Fx::at(N);
        let a2 = chain[N as usize].anchor;
        let (x, y) = (fx.lin.blob(N + 1, &a2), fx.lin.blob(N + 1, &a2));
        fx.put_prepared(&x.bytes);
        fx.cut_child("commit", &y.bytes, CutPoint::KM2);
        assert!(
            !prepared_path(&fx.store).exists(),
            "B4: the stale slot is removed BEFORE the new preparation"
        );
        assert_eq!(prepared_family(&fx.store), 1, "at most one prepared file");
        restart_twice(&fx, open(&chain[N as usize]));
        assert_ledger(&fx, &upto(&chain));

        let (fx, chain) = Fx::at(N);
        let a2 = chain[N as usize].anchor;
        let (x, y) = (fx.lin.blob(N + 1, &a2), fx.lin.blob(N + 1, &a2));
        fx.put_prepared(&x.bytes);
        let mut s = fx.begin().expect("begin");
        assert!(s.stale_prepared());
        let c = s
            .commit(&y.bytes, &fx.lin.opener)
            .expect("commit over a stale slot");
        assert_eq!(c, y.id());
        assert!(!s.stale_prepared());
        assert!(!prepared_path(&fx.store).exists());
        assert_eq!(fs::read(current_path(&fx.store)).expect("current"), y.bytes);
        drop(s);
        let s = fx.begin().expect("restart");
        assert_eq!((s.resumed(), s.committed()), (Resumed::Opened, y.id()));
    }

    #[test]
    fn gen_max_refused_before_any_write() {
        let fx = Fx::empty();
        let top = fx.lin.blob(u64::MAX, &random32());
        fx.put_current(&top.bytes);
        fx.put_head(&fx.lin.head(&top));
        let mut s = fx.begin().expect("begin");
        assert_eq!(s.committed(), top.id());
        let next = fx.lin.blob(0, &top.anchor);
        let before = fx.trees();
        let r = s.commit(&next.bytes, &fx.lin.opener);
        assert_eq!(kind(&r), "GenerationExhausted");
        assert_eq!(fx.trees(), before);
        assert!(!s.is_poisoned());
    }

    #[test]
    fn succ_1_non_chaining_successors_refused_before_any_write() {
        let (fx, chain) = Fx::at(N);
        let lin = &fx.lin;
        let a2 = chain[N as usize].anchor;
        let s3 = lin.blob(N + 1, &a2);
        let mut s = fx.begin().expect("begin");
        let arms: Vec<(&str, Vec<u8>, &str)> = vec![
            ("a", lin.blob(N + 2, &a2).bytes, "NotChained(Generation)"),
            (
                "b",
                lin.blob(N + 1, &random32()).bytes,
                "NotChained(PredecessorAnchor)",
            ),
            (
                "c",
                lin.blob_with(random32(), N + 1, &a2, lin.kc, 1).bytes,
                "NotChained(VaultId)",
            ),
            (
                "d",
                lin.blob_with(lin.vault_id, N + 1, &a2, lin.kc, 2).bytes,
                "NotChained(Mode)",
            ),
            (
                "e",
                lin.blob_with(lin.vault_id, N + 1, &a2, random32(), 1).bytes,
                "NotChained(CheckpointKey)",
            ),
            (
                "f",
                Lineage::new().blob(N + 1, &a2).bytes,
                "SuccessorOpenFailed",
            ),
            ("g", vec![0u8; MAX + 1], "SuccessorTooLarge"),
        ];
        for (arm, bytes, want) in arms {
            let before = fx.trees();
            let calls = lin.opener.calls.get();
            let r = s.commit(&bytes, &lin.opener);
            assert_eq!(kind(&r), want, "arm {arm}");
            assert_eq!(fx.trees(), before, "arm {arm}: before any write");
            if arm == "g" {
                assert_eq!(lin.opener.calls.get(), calls, "never shown to the opener");
            }
        }
        let head = fx.head_bytes().expect("head");
        fx.put_head(&lin.head(&chain[1]));
        let before = fx.trees();
        let r = s.commit(&s3.bytes, &lin.opener);
        assert_eq!(kind(&r), "NotChained(HeadChanged)", "arm h");
        assert_eq!(fx.trees(), before, "arm h: before any write");
        fx.put_head(&head);
        assert!(!s.is_poisoned());
        let c = s
            .commit(&s3.bytes, &lin.opener)
            .expect("a chaining successor");
        assert_eq!(c, s3.id());
    }

    #[test]
    fn lk_1_copy_of_store_meets_the_same_lock_in_another_process() {
        let (fx, chain) = Fx::at(N);
        let s = fx.begin().expect("the parent holds the lineage lock");
        let copy = fx.root.path().join("store_copy");
        private_dir(&copy);
        fs::copy(current_path(&fx.store), current_path(&copy)).expect("copy the store");
        let copy_before = tree(&copy, None);
        fx.write_fixture("lk1", &[]);
        let outcome = fx.root.path().join("fixture").join("outcome");
        let (status, _) = run_child(CHILD, None, fx.root.path());
        assert!(status.success(), "lk1 child: {status:?}");
        assert_eq!(
            fs::read_to_string(&outcome).expect("outcome"),
            "contended calls=1",
            "refused at the lock, before recover() read the authority"
        );
        assert_eq!(tree(&copy, None), copy_before);
        drop(s);
        fs::remove_file(&outcome).expect("reset");
        let (status, _) = run_child(CHILD, None, fx.root.path());
        assert!(status.success(), "lk1 child: {status:?}");
        assert_eq!(
            fs::read_to_string(&outcome).expect("outcome"),
            format!("open {}", chain[N as usize].generation)
        );
    }

    #[test]
    fn lo_1_store_lock_then_lineage_lock_no_conflict() {
        let (fx, _chain) = Fx::at(N);
        let store_lock = fx.store.join(".qsc.lock");
        for _ in 0..2 {
            let guard = LockGuard::acquire(&fx.store, &store_lock, LockMode::Exclusive)
                .expect("the store lock FIRST");
            let s = fx.begin().expect("then the lineage lock");
            let probe = OpenOptions::new()
                .read(true)
                .write(true)
                .open(&store_lock)
                .expect("probe");
            // SAFETY: flock on a descriptor this test owns (LOCK_EX | LOCK_NB).
            let rc = unsafe { crate::flock(probe.as_raw_fd(), 2 | 4) };
            assert_ne!(rc, 0, "the store lock is held");
            assert_eq!(
                LineageLock::acquire(&fx.ckdir()).map(|_| ()),
                Err(LockError::Contended),
                "the D32 lineage lock is held"
            );
            drop(s);
            drop(guard);
        }
    }

    #[test]
    fn pure_begin_freezes_write_nothing_but_the_lock_file() {
        type Case = (&'static str, fn(&Fx, &[Blob]), FreezeKind);
        let cases: [Case; 4] = [
            (
                "head deleted",
                |fx, _| fs::remove_file(fx.head_path()).expect("rm head"),
                FreezeKind::CheckpointMissing,
            ),
            (
                "head flipped",
                |fx, _| {
                    let mut h = fx.head_bytes().expect("head");
                    let last = h.len() - 1;
                    h[last] ^= 1;
                    fs::write(fx.head_path(), h).expect("flip");
                },
                FreezeKind::CheckpointCorrupt(CorruptKind::Mac),
            ),
            (
                "current rolled back",
                |fx, chain| fx.put_current(&chain[1].bytes),
                FreezeKind::GenerationRegression,
            ),
            (
                "FN2-L",
                |fx, chain| {
                    let a2 = chain[N as usize].anchor;
                    fx.put_prepared(&fx.lin.blob(N + 1, &a2).bytes);
                    fx.put_head(&fx.lin.head(&fx.lin.blob(N + 1, &a2)));
                },
                FreezeKind::CommittedStateMissing,
            ),
        ];
        for (name, plant, want) in cases {
            let (fx, chain) = Fx::at(N);
            plant(&fx, &chain);
            let before = fx.trees_but_lock();
            for round in 0..2 {
                match fx.begin() {
                    Err(BeginError::Freeze(k)) => assert_eq!(k, want, "{name} round {round}"),
                    other => panic!("{name} round {round}: {other:?}"),
                }
            }
            assert_eq!(
                fx.trees_but_lock(),
                before,
                "{name}: a freeze writes nothing"
            );
            assert_ledger(&fx, &upto(&chain));
        }
    }

    #[test]
    fn begin_no_lineage_or_unauthenticated_writes_nothing() {
        let fx = Fx::empty();
        let before = fx.trees();
        let r = fx.begin();
        assert!(matches!(r, Err(BeginError::NoLineage)), "{r:?}");
        assert_eq!(fx.trees(), before);
        assert!(!fx.ckdir().exists());
        fx.put_current(&Lineage::new().blob(0, &ZERO).bytes);
        let before = fx.trees();
        let r = fx.begin();
        assert!(
            matches!(
                r,
                Err(BeginError::Quarantine(RecoverError::OpenFailed {
                    which: Unauthenticated::Current
                }))
            ),
            "{r:?}"
        );
        assert_eq!(fx.trees(), before);
        assert!(!fx.ckdir().exists(), "no checkpoint dir, no lock");
    }

    #[test]
    fn g_ref_genesis_refusals_before_any_write() {
        let fx = Fx::empty();
        let lin = &fx.lin;
        let arms: Vec<(&str, Vec<u8>, &str)> = vec![
            ("a", lin.blob(1, &ZERO).bytes, "NotGenesis(Generation)"),
            (
                "b",
                lin.blob(0, &random32()).bytes,
                "NotGenesis(PredecessorAnchor)",
            ),
            (
                "c",
                lin.blob_with(lin.vault_id, 0, &ZERO, lin.kc, 2).bytes,
                "NotLocalCheckpoint",
            ),
            ("d", Lineage::new().blob(0, &ZERO).bytes, "B0OpenFailed"),
            ("e", vec![0u8; MAX + 1], "B0TooLarge"),
        ];
        for (arm, bytes, want) in arms {
            let before = fx.trees();
            let r = genesis(&fx.store, &fx.state, MAX, &bytes, &lin.opener);
            assert_eq!(kind(&r), want, "arm {arm}");
            assert_eq!(fx.trees(), before, "arm {arm}: before any write");
            assert!(
                !fx.ckdir().exists(),
                "arm {arm}: no checkpoint dir, no lock"
            );
        }
        // f: the store already holds this lineage.
        let (fx, chain) = Fx::at(N);
        let before = fx.trees_but_lock();
        let r = genesis(&fx.store, &fx.state, MAX, &chain[0].bytes, &fx.lin.opener);
        assert_eq!(kind(&r), "LineagePresent", "arm f");
        assert_eq!(fx.trees_but_lock(), before, "arm f");
        // g: an empty store, but a head already exists for this vault_id.
        let fx = Fx::empty();
        let b0 = fx.lin.blob(0, &ZERO);
        fx.put_head(&fx.lin.head(&b0));
        let before = fx.trees_but_lock();
        let r = genesis(&fx.store, &fx.state, MAX, &b0.bytes, &fx.lin.opener);
        assert_eq!(kind(&r), "CheckpointExists", "arm g");
        assert_eq!(fx.trees_but_lock(), before, "arm g");
    }

    // ---- S7b (SEALED_EXPECTATION_S7b.md sec 1): the S7 read's probes, their outcomes INVERTED.

    /// An AUTHENTIC blob with an unknown profile byte: the opener returns Malformed.
    fn malformed(lin: &Lineage, generation: u64, pred: &[u8; 32]) -> Vec<u8> {
        lin.blob_with(lin.vault_id, generation, pred, lin.kc, 0)
            .bytes
    }

    #[test]
    fn x1_malformed_is_vault_parse_failed_never_vault_locked() {
        // a. The quarantine: refused as a parse failure, before any directory or lock.
        let fx = Fx::empty();
        fx.put_current(&malformed(&fx.lin, 0, &ZERO));
        let before = fx.trees();
        let r = fx.begin();
        assert!(
            matches!(
                r,
                Err(BeginError::Quarantine(RecoverError::BlobMalformed {
                    slot: Slot::Current
                }))
            ),
            "{r:?}"
        );
        let code = r.unwrap_err().code();
        assert_eq!(code, "vault_parse_failed");
        assert_ne!(
            code,
            crate::msgqueue::MSGQUEUE_VAULT_LOCKED,
            "never the counted code"
        );
        assert_eq!(fx.trees(), before);
        assert!(!fx.ckdir().exists(), "no checkpoint dir, no lock");
        // b. Under the lock: an authentic current beside a Malformed prepared slot.
        let (fx, chain) = Fx::at(N);
        fx.put_prepared(&malformed(&fx.lin, N + 1, &chain[N as usize].anchor));
        let r = fx.begin();
        assert!(
            matches!(
                r,
                Err(BeginError::Recover(RecoverError::BlobMalformed {
                    slot: Slot::Prepared
                }))
            ),
            "{r:?}"
        );
        assert_eq!(r.unwrap_err().code(), "vault_parse_failed");
        // c. A Malformed successor: refused before any write, the session not poisoned.
        let (fx, chain) = Fx::at(N);
        let mut s = fx.begin().expect("begin");
        let before = fx.trees();
        let r = s.commit(
            &malformed(&fx.lin, N + 1, &chain[N as usize].anchor),
            &fx.lin.opener,
        );
        assert_eq!(kind(&r), "SuccessorMalformed");
        assert_eq!(r.unwrap_err().code(), "vault_parse_failed");
        assert_eq!(fx.trees(), before);
        assert!(!s.is_poisoned());
        // d. A Malformed B0.
        let fx = Fx::empty();
        let before = fx.trees();
        let r = genesis(
            &fx.store,
            &fx.state,
            MAX,
            &malformed(&fx.lin, 0, &ZERO),
            &fx.lin.opener,
        );
        assert_eq!(kind(&r), "B0Malformed");
        assert_eq!(r.unwrap_err().code(), "vault_parse_failed");
        assert_eq!(fx.trees(), before);
    }

    #[test]
    fn x4_pr09_fifo_at_vault_qsv_is_refused_within_a_bound() {
        let (fx, _chain) = Fx::at(N);
        fs::remove_file(current_path(&fx.store)).expect("rm current");
        let made = std::process::Command::new("mkfifo")
            .arg(current_path(&fx.store))
            .status()
            .expect("mkfifo");
        assert!(made.success());
        let (store, state, key) = (fx.store.clone(), fx.state.clone(), fx.lin.opener.key);
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let opener = TestOpener::from_key(key);
            let r = begin(&store, &state, MAX, &opener);
            let _ = tx.send(r.map(|_| ()).map_err(|e| (format!("{e:?}"), e.code())));
        });
        let r = rx
            .recv_timeout(std::time::Duration::from_secs(10))
            .expect("begin() must return within the bound, never hang on a FIFO");
        let (kind, code) = r.expect_err("a FIFO is never a vault");
        assert!(
            kind.starts_with("Quarantine(BlobRead { slot: Current"),
            "{kind}"
        );
        assert_eq!(code, "vault_read_failed");
    }

    #[test]
    fn x4_pr17_symlinked_prepared_slot_is_refused_before_rule_h() {
        let (fx, chain) = Fx::at(N);
        fx.put_current(b"garbage");
        let elsewhere = fx.root.path().join("elsewhere.blob");
        fs::write(&elsewhere, &chain[N as usize].bytes).expect("elsewhere");
        std::os::unix::fs::symlink(&elsewhere, prepared_path(&fx.store)).expect("symlink");
        let r = fx.begin();
        assert!(r.is_err(), "{r:?}");
        assert_eq!(r.unwrap_err().code(), "vault_read_failed");
        let current = fs::symlink_metadata(current_path(&fx.store)).expect("current");
        assert!(
            current.file_type().is_file(),
            "current is still a regular file"
        );
        assert_eq!(
            fs::read(current_path(&fx.store)).expect("current"),
            b"garbage"
        );
        let prepared = fs::symlink_metadata(prepared_path(&fx.store)).expect("prepared");
        assert!(prepared.file_type().is_symlink(), "nothing was promoted");
        // S7c DF-11: rule b under the lock -- an AUTHENTIC current beside a symlinked prepared slot.
        let (fx, chain) = Fx::at(N);
        let elsewhere = fx.root.path().join("elsewhere.blob");
        fs::write(&elsewhere, &chain[N as usize].bytes).expect("elsewhere");
        std::os::unix::fs::symlink(&elsewhere, prepared_path(&fx.store)).expect("symlink");
        let r = fx.begin();
        assert!(
            matches!(
                r,
                Err(BeginError::Recover(RecoverError::BlobRead {
                    slot: Slot::Prepared,
                    ..
                }))
            ),
            "{r:?}"
        );
        assert_eq!(r.unwrap_err().code(), "vault_read_failed");
        let prepared = fs::symlink_metadata(prepared_path(&fx.store)).expect("prepared");
        assert!(
            prepared.file_type().is_symlink(),
            "never followed, never promoted"
        );
        assert_eq!(
            fs::read(current_path(&fx.store)).expect("current"),
            chain[N as usize].bytes
        );
    }

    #[test]
    fn x5_pr14_another_kc_is_successor_invalid_unless_the_head_changed() {
        let (fx, chain) = Fx::at(N);
        let lin = &fx.lin;
        let a2 = chain[N as usize].anchor;
        let mut s = fx.begin().expect("begin");
        // 1. Another key, the head bytes unchanged: the successor is at fault.
        let head = fx.head_bytes().expect("head");
        let bad = lin.blob_with(lin.vault_id, N + 1, &a2, random32(), 1);
        let r = s.commit(&bad.bytes, &lin.opener);
        assert_eq!(kind(&r), "NotChained(CheckpointKey)");
        assert_eq!(r.unwrap_err().code(), "freshness_successor_invalid");
        assert_eq!(
            fx.head_bytes().expect("head"),
            head,
            "the head did not change"
        );
        assert!(!s.is_poisoned());
        // 2. Another key AND another head on disk: the head changed.
        fx.put_head(&lin.head(&chain[1]));
        let r = s.commit(&bad.bytes, &lin.opener);
        assert_eq!(kind(&r), "NotChained(HeadChanged)");
        assert_eq!(r.unwrap_err().code(), "freshness_head_changed");
        // 3. Restored: a chaining successor commits, and the Session holds the NEW head's bytes.
        fx.put_head(&head);
        let s3 = lin.blob(N + 1, &a2);
        assert_eq!(s.commit(&s3.bytes, &lin.opener).expect("commit"), s3.id());
        let bad4 = lin.blob_with(lin.vault_id, N + 2, &s3.anchor, random32(), 1);
        let r = s.commit(&bad4.bytes, &lin.opener);
        assert_eq!(kind(&r), "NotChained(CheckpointKey)");
    }

    #[test]
    fn x7_pr11_own_pid_old_shape_head_temp_no_longer_blocks_c3() {
        let (fx, chain) = Fx::at(N);
        let s3 = fx.lin.blob(N + 1, &chain[N as usize].anchor);
        let planted = fx.ckdir().join(format!("head.tmp.{}", process::id()));
        fs::write(&planted, b"leftover-from-a-reused-pid").expect("plant");
        let mut s = fx.begin().expect("begin");
        let c = s
            .commit(&s3.bytes, &fx.lin.opener)
            .expect("C3 uses its own name");
        assert_eq!(c, s3.id());
        let head = checkpoint::decode(
            &fx.head_bytes().expect("head"),
            &fx.lin.kc,
            ProtectionMode::LocalCheckpoint,
        )
        .expect("head decodes");
        assert_eq!((head.generation, head.digest), (s3.generation, s3.digest));
        assert!(planted.exists(), "a live pid's temp is kept by rule a");
        // S7c DF-5: the counter advances -- two consecutive head temps of one process differ.
        let pid = process::id();
        assert_ne!(
            next_head_temp(&fx.ckdir(), pid),
            next_head_temp(&fx.ckdir(), pid)
        );
    }

    #[test]
    fn x10_pr10_swapped_store_is_refused_before_any_promotion() {
        // X is committed at N. Lineage Y (same opener, another vault_id and Kc) is promotable:
        // current Y0, prepared Y1, a head naming Y1. The store is swapped to Y INSIDE the
        // quarantine's open of X's current, after its bytes were read.
        let (fx, chain) = Fx::at(N);
        let (vid_y, kc_y) = (random32(), random32());
        let y0 = fx.lin.blob_with(vid_y, 0, &ZERO, kc_y, 1);
        let y1 = fx.lin.blob_with(vid_y, 1, &y0.anchor, kc_y, 1);
        let head_y1 = checkpoint::encode(
            &Checkpoint {
                mode: ProtectionMode::LocalCheckpoint,
                vault_id: vid_y,
                generation: y1.generation,
                digest: y1.digest,
                anchor: y1.anchor,
            },
            &kc_y,
        );
        let ck_y = paths::ensure_checkpoint_dir(&fx.state, &vid_y).expect("Y's dir");
        fs::write(paths::head_path(&ck_y), head_y1).expect("Y's head");
        let swap_to_y = || {
            let store = fx.store.clone();
            let (y0b, y1b) = (y0.bytes.clone(), y1.bytes.clone());
            fx.lin.opener.calls.set(0);
            *fx.lin.opener.hook.borrow_mut() = Some((
                1,
                Box::new(move || {
                    fs::write(current_path(&store), &y0b).expect("swap current");
                    fs::write(prepared_path(&store), &y1b).expect("swap prepared");
                }),
            ));
        };
        swap_to_y();
        let r = fx.begin();
        assert!(matches!(r, Err(BeginError::LineageChanged)), "{r:?}");
        assert_eq!(r.unwrap_err().code(), "freshness_lineage_changed");
        // S7c DF-10: the quarantine's open, then rule b's current and prepared -- nothing past rule c.
        assert_eq!(fx.lin.opener.calls.get(), 3);
        assert_eq!(
            fs::read(prepared_path(&fx.store)).expect("Y's prepared slot is still there"),
            y1.bytes
        );
        assert_eq!(
            fs::read(current_path(&fx.store)).expect("current"),
            y0.bytes
        );
        assert_eq!(
            fs::read(paths::head_path(&ck_y)).expect("Y's head"),
            head_y1
        );
        assert!(
            !paths::lock_path(&ck_y).exists(),
            "Y's lock was never taken"
        );
        // S7c DF-10 (D-S1's placement): Y's head is never OPENED. With a directory there, a read
        // before the compare would refuse it as a checkpoint read failure instead.
        fs::remove_file(paths::head_path(&ck_y)).expect("rm Y's head");
        fs::create_dir(paths::head_path(&ck_y)).expect("a directory at Y's head");
        fx.put_current(&chain[N as usize].bytes);
        fs::remove_file(prepared_path(&fx.store)).expect("rm prepared");
        swap_to_y();
        let r = fx.begin();
        assert!(matches!(r, Err(BeginError::LineageChanged)), "{r:?}");
        assert_eq!(fx.lin.opener.calls.get(), 3);
        assert_eq!(
            fs::read(prepared_path(&fx.store)).expect("Y's prepared slot is still there"),
            y1.bytes
        );
    }

    #[test]
    fn x11_commit_rechecks_the_lock_inode_first() {
        let (fx, chain) = Fx::at(N);
        let s3 = fx.lin.blob(N + 1, &chain[N as usize].anchor);
        let mut s = fx.begin().expect("begin");
        let lock = paths::lock_path(&fx.ckdir());
        fs::remove_file(&lock).expect("unlink the lock file");
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&lock)
            .expect("recreate it");
        let before = fx.trees();
        let calls = fx.lin.opener.calls.get();
        let r = s.commit(&s3.bytes, &fx.lin.opener);
        assert_eq!(kind(&r), "Lock(InodeChanged)");
        assert_eq!(r.unwrap_err().code(), "freshness_lineage_lock_unstable");
        assert_eq!(fx.trees(), before, "refused before any write");
        assert_eq!(
            fx.lin.opener.calls.get(),
            calls,
            "refused before the opener"
        );
        assert!(!s.is_poisoned());
    }

    // ---- S7c (SEALED_EXPECTATION_S7c.md sec 1): the delta read's findings, RULING R2.

    #[test]
    fn df1_genesis_refuses_a_foreign_promotable_store_before_promotion() {
        // p_a7 inverted: lineage Y (same opener, another vault_id and Kc) is promotable in the store.
        let fx = Fx::empty();
        let (vid_y, kc_y) = (random32(), random32());
        let y0 = fx.lin.blob_with(vid_y, 0, &ZERO, kc_y, 1);
        let y1 = fx.lin.blob_with(vid_y, 1, &y0.anchor, kc_y, 1);
        let head_y1 = checkpoint::encode(
            &Checkpoint {
                mode: ProtectionMode::LocalCheckpoint,
                vault_id: vid_y,
                generation: y1.generation,
                digest: y1.digest,
                anchor: y1.anchor,
            },
            &kc_y,
        );
        let ck_y = paths::ensure_checkpoint_dir(&fx.state, &vid_y).expect("Y's dir");
        fs::write(paths::head_path(&ck_y), head_y1).expect("Y's head");
        fx.put_current(&y0.bytes);
        fx.put_prepared(&y1.bytes);
        let b0 = fx.lin.blob(0, &ZERO);
        let r = genesis(&fx.store, &fx.state, MAX, &b0.bytes, &fx.lin.opener);
        assert_eq!(kind(&r), "Recover(LineageChanged)");
        assert_eq!(r.unwrap_err().code(), "vault_exists");
        assert_eq!(
            fs::read(prepared_path(&fx.store)).expect("Y's prepared slot is still there"),
            y1.bytes
        );
        assert_eq!(
            fs::read(current_path(&fx.store)).expect("current"),
            y0.bytes
        );
        assert_eq!(
            fs::read(paths::head_path(&ck_y)).expect("Y's head"),
            head_y1
        );
        assert!(
            !paths::lock_path(&ck_y).exists(),
            "Y's lock was never taken"
        );
    }

    #[test]
    fn df2_own_pid_old_shape_prepared_temp_no_longer_blocks_c2() {
        // p_a3 inverted: a leftover of a reused pid (this process's) in S5's old shape.
        let (fx, chain) = Fx::at(N);
        let planted = fx
            .store
            .join(format!("vault.qsv.prepared.tmp.{}", process::id()));
        fs::write(&planted, b"leftover-from-a-reused-pid").expect("plant");
        let mut s = fx.begin().expect("begin");
        let s3 = fx.lin.blob(N + 1, &chain[N as usize].anchor);
        let c = s
            .commit(&s3.bytes, &fx.lin.opener)
            .expect("C2 uses its own name");
        assert_eq!(c, s3.id());
        let s4 = fx.lin.blob(N + 2, &s3.anchor);
        assert_eq!(s.commit(&s4.bytes, &fx.lin.opener).expect("again"), s4.id());
        assert!(planted.exists(), "a live pid's temp is kept by rule a");
        let pid = process::id();
        assert_ne!(
            next_prepared_temp(&fx.store, pid),
            next_prepared_temp(&fx.store, pid)
        );
    }

    #[test]
    fn df4_genesis_session_head_classifies_a_wrong_key_successor() {
        // p_a1: the Session's head after genesis is the head genesis wrote.
        let fx = Fx::empty();
        let lin = &fx.lin;
        let b0 = lin.blob(0, &ZERO);
        let mut s = genesis(&fx.store, &fx.state, MAX, &b0.bytes, &lin.opener).expect("genesis");
        let bad = lin.blob_with(lin.vault_id, 1, &b0.anchor, random32(), 1);
        let r = s.commit(&bad.bytes, &lin.opener);
        assert_eq!(kind(&r), "NotChained(CheckpointKey)");
        assert_eq!(r.unwrap_err().code(), "freshness_successor_invalid");
        assert!(!s.is_poisoned());
        let s1 = lin.blob(1, &b0.anchor);
        assert_eq!(s.commit(&s1.bytes, &lin.opener).expect("commit"), s1.id());
    }

    #[test]
    fn df6_session_debug_never_formats_the_head_bytes() {
        let (fx, _chain) = Fx::at(N);
        let s = fx.begin().expect("begin");
        let text = format!("{s:?}");
        assert!(
            text.contains(&format!("head_len: {}", checkpoint::LEN)),
            "{text}"
        );
        assert!(!text.contains("head: ["), "the head was formatted: {text}");
        assert!(
            text.contains(&crate::hex_encode(&fx.lin.vault_id)),
            "{text}"
        );
        assert!(!text.contains("LineageLock"), "{text}");
    }
}
