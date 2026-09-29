//! C07 T4.3 recovery for the LOCAL profile (NA-0787 F04-C07P S4). Given the store directory, the
//! D31 state root and the caller's [`LineageOpen`], decide -- from the files alone, before anything
//! is used -- whether the vault found is the last committed one (OPEN), the one that was about to
//! be committed (PROMOTE), or something else (FREEZE, never a guess).
//!
//! THE ORDERED RULES (SEALED_EXPECTATION_S4.md sec 3; the first rule that applies decides):
//! - a. any restart (:270): remove the prepared temps of DEAD pids and, when the caller names the
//!   lineage it locked, that lineage's head temps of DEAD pids (S7b X7), and nothing else;
//! - b. read current and prepared, each bounded by `max_blob_len`, before the opener sees either;
//! - c. both authenticated must agree on vault_id, checkpoint_mac_key and protection_mode, and on
//!   the lineage the caller locked, if it names one (S7b X10: refused before rule h can promote);
//! - d. nothing present -> NoLineage; blobs present and none authenticates -> Err(OpenFailed);
//! - e. the head: absent, corrupt, unsupported or foreign -> FREEZE, never rebuilt (:267);
//! - f-i. the head matches CURRENT -> open (:264); PREPARED -> promote durably (:265); neither ->
//!   FREEZE (:266), refined to ER5/ER6 only when no prepared slot exists;
//! - j. a FREEZE writes nothing: rule a's removals and rule h's promotion are the only writes.
//!
//! No lock is taken here: the caller holds the lineage lock (FN1; S5 adds it and calls this under
//! it). No client-code string is allocated: the marker spellings are S6's (RULING_F04C07P R6).
//! Every read open is `open_regular` (S7b X4): a symlink, FIFO or device is refused, never read.

use super::checkpoint::{self, Checkpoint, CheckpointError, CorruptKind, UnsupportedKind};
use super::ProtectionMode;
use super::{anchor, digest, open_regular, paths, LineageFields, LineageOpen, OpenError};
use crate::fs_store::{self, DurableWriteError};
use std::ffi::OsStr;
use std::fmt;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

/// C07 T6.4 D30 names, inside the successor store directory (the directory itself is S11's).
pub(crate) const CURRENT_FILE: &str = "vault.qsv";
pub(crate) const PREPARED_FILE: &str = "vault.qsv.prepared";
const PREPARED_TEMP_PREFIX: &str = "vault.qsv.prepared.tmp.";

pub(crate) fn current_path(store_dir: &Path) -> PathBuf {
    store_dir.join(CURRENT_FILE)
}

pub(crate) fn prepared_path(store_dir: &Path) -> PathBuf {
    store_dir.join(PREPARED_FILE)
}

pub(crate) fn prepared_temp_path(store_dir: &Path, pid: u32) -> PathBuf {
    store_dir.join(format!("{PREPARED_TEMP_PREFIX}{pid}"))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Slot {
    Current,
    Prepared,
}

/// Which present blobs failed to authenticate, when none did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Unauthenticated {
    Current,
    Prepared,
    Both,
}

/// An authenticated blob that the head names: its exact bytes, its lineage values, the head's
/// digest and anchor for it (equal to the recomputed ones, by the match), and the exact head bytes
/// that were read and authenticated (S7b X5: public, no key; commit compares against them).
pub(crate) struct CommittedBlob {
    pub(crate) bytes: Vec<u8>,
    pub(crate) fields: LineageFields,
    pub(crate) digest: [u8; 32],
    pub(crate) anchor: [u8; 32],
    pub(crate) head: [u8; checkpoint::LEN],
}

/// S7b X8 (F-12): the blob (the whole encrypted vault) is never formatted, only its length.
impl fmt::Debug for CommittedBlob {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CommittedBlob")
            .field("bytes_len", &self.bytes.len())
            .field("fields", &self.fields)
            .field("digest", &self.digest)
            .field("anchor", &self.anchor)
            .finish_non_exhaustive()
    }
}

pub(crate) enum Recovered {
    /// Nothing in the store: the empty store, init's case.
    NoLineage,
    /// T4.3 row 1 (:264). `stale_prepared`: a prepared slot exists, is classified uncommitted, is
    /// never opened, and is left for S5 to remove before its next preparation (B4 :318).
    Open {
        current: CommittedBlob,
        stale_prepared: bool,
    },
    /// T4.3 row 2 (:265): the prepared blob was renamed over current and the directory flushed.
    /// Resuming its committed outbox is the caller's (C6).
    Promoted { current: CommittedBlob },
    /// Fail closed. Permanent for these files: recovering again returns the same freeze.
    Freeze(FreezeKind),
}

/// S7b X8: through CommittedBlob's own Debug, so the blob bytes are never formatted.
impl fmt::Debug for Recovered {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoLineage => f.write_str("NoLineage"),
            Self::Open {
                current,
                stale_prepared,
            } => f
                .debug_struct("Open")
                .field("current", current)
                .field("stale_prepared", stale_prepared)
                .finish(),
            Self::Promoted { current } => f
                .debug_struct("Promoted")
                .field("current", current)
                .finish(),
            Self::Freeze(kind) => f.debug_tuple("Freeze").field(kind).finish(),
        }
    }
}

/// Typed freezes; the ER spellings are S6's.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FreezeKind {
    /// ER2 (:267).
    CheckpointMissing,
    /// ER3 (:267).
    CheckpointCorrupt(CorruptKind),
    /// ER4 (:218).
    CheckpointUnsupported(UnsupportedKind),
    /// An authentic head naming another vault_id: it names no blob of this lineage (:266).
    ForeignCheckpoint,
    /// Current and prepared both authenticate but disagree on vault_id, key or mode.
    LineageMismatch,
    /// ER5: the only retained blob is authentic and OLDER than the head (:139).
    GenerationRegression,
    /// ER6: the only retained blob is authentic, at the head's generation, and not its blob (:140).
    DigestConflict,
    /// ER7 (:266): the head matches neither retained blob.
    CommittedStateMissing,
}

/// Outcomes that are not freshness verdicts.
#[derive(Debug, Clone, Copy)]
pub(crate) enum RecoverError {
    /// Rule a could not list the store directory or remove a dead pid's temp.
    TempCleanup(io::ErrorKind),
    BlobRead {
        slot: Slot,
        kind: io::ErrorKind,
    },
    /// Longer than the caller's bound; never shown to the opener (I06).
    BlobTooLarge {
        slot: Slot,
    },
    /// Blobs are present and none authenticates (a wrong passphrase looks exactly like this).
    OpenFailed {
        which: Unauthenticated,
    },
    /// S7b X1: a blob authenticated but the opener cannot read its payload (OpenError::Malformed).
    /// Not a verdict and not a passphrase failure.
    BlobMalformed {
        slot: Slot,
    },
    /// S7b X10: the authenticated lineage is not the one the caller locked. Refused before rule h.
    LineageChanged,
    /// Not a local-checkpoint lineage: its authority is not this classifier's (AM-1).
    NotLocalCheckpoint,
    CheckpointRead(io::ErrorKind),
    /// The promotion's rename (nothing changed) or its directory flush (the rename HAS happened;
    /// recover again to reconcile, C07 IC3 :279).
    DurableWrite(DurableWriteError),
}

struct Opened {
    bytes: Vec<u8>,
    fields: LineageFields,
}

enum SlotState {
    Absent,
    Unauthenticated,
    Authenticated(Opened),
}

impl SlotState {
    fn present(&self) -> bool {
        !matches!(self, Self::Absent)
    }

    fn opened(&self) -> Option<&Opened> {
        match self {
            Self::Authenticated(o) => Some(o),
            _ => None,
        }
    }

    fn into_opened(self) -> Option<Opened> {
        match self {
            Self::Authenticated(o) => Some(o),
            _ => None,
        }
    }
}

enum Verdict {
    Open(Opened),
    Promote(Opened),
    Freeze(FreezeKind),
}

/// Classify the store per C07 T4.3 (LOCAL) by the sealed ordered rules (module doc).
///
/// PRECONDITION (documented, not enforced here): the caller holds the lineage lock (C07 FN1 :258)
/// of `expected` when it names one (begin(): the quarantined vault_id); genesis() names none.
pub(crate) fn recover(
    store_dir: &Path,
    state_root: &Path,
    max_blob_len: usize,
    opener: &dyn LineageOpen,
    expected: Option<&[u8; 32]>,
) -> Result<Recovered, RecoverError> {
    // a. Any restart, before the rows are applied (:270).
    remove_dead_temps(store_dir, prepared_temp_pid)?;
    if let Some(vault_id) = expected {
        remove_dead_temps(
            &paths::checkpoint_dir(state_root, vault_id),
            paths::head_temp_pid,
        )?;
    }
    // b. Both reads are bounded and finish before the opener sees either blob.
    let current_bytes = read_slot(&current_path(store_dir), max_blob_len, Slot::Current)?;
    let prepared_bytes = read_slot(&prepared_path(store_dir), max_blob_len, Slot::Prepared)?;
    if current_bytes.is_none() && prepared_bytes.is_none() {
        return Ok(Recovered::NoLineage);
    }
    let current = open_slot(opener, current_bytes, Slot::Current)?;
    let prepared = open_slot(opener, prepared_bytes, Slot::Prepared)?;
    // c, d. The lineage identity comes from the authenticated blob(s) only.
    let lineage = match (current.opened(), prepared.opened()) {
        (Some(c), Some(p)) if !same_lineage(&c.fields, &p.fields) => {
            return Ok(Recovered::Freeze(FreezeKind::LineageMismatch));
        }
        (Some(c), _) => &c.fields,
        (None, Some(p)) => &p.fields,
        (None, None) => {
            let which = match (current.present(), prepared.present()) {
                (true, true) => Unauthenticated::Both,
                (true, false) => Unauthenticated::Current,
                _ => Unauthenticated::Prepared,
            };
            return Err(RecoverError::OpenFailed { which });
        }
    };
    // S7b X10 (F-19): the store changed since the caller's quarantined read; nothing below runs.
    if expected.is_some_and(|vault_id| *vault_id != lineage.vault_id) {
        return Err(RecoverError::LineageChanged);
    }
    if lineage.protection_mode != ProtectionMode::LocalCheckpoint {
        return Err(RecoverError::NotLocalCheckpoint);
    }
    // e. The head: never rebuilt from the supplied vault (:267).
    let (head, head_bytes) = match read_head(state_root, lineage)? {
        Ok(read) => read,
        Err(kind) => return Ok(Recovered::Freeze(kind)),
    };
    // f-i.
    let prepared_present = prepared.present();
    match classify(
        &head,
        current.into_opened(),
        prepared.into_opened(),
        prepared_present,
    ) {
        Verdict::Open(c) => Ok(Recovered::Open {
            current: committed(c, &head, head_bytes),
            stale_prepared: prepared_present,
        }),
        Verdict::Promote(p) => {
            promote(store_dir)?;
            Ok(Recovered::Promoted {
                current: committed(p, &head, head_bytes),
            })
        }
        Verdict::Freeze(kind) => Ok(Recovered::Freeze(kind)),
    }
}

/// Rules f-i over the authenticated blobs. Never the largest generation, never one step behind.
fn classify(
    head: &Checkpoint,
    current: Option<Opened>,
    prepared: Option<Opened>,
    prepared_present: bool,
) -> Verdict {
    // Row 1 (:264): the head names the current blob.
    let current = match current {
        Some(c) if matches(head, &c) => return Verdict::Open(c),
        other => other,
    };
    // Row 2 (:265): the head names the prepared blob.
    if let Some(p) = prepared.filter(|p| matches(head, p)) {
        return Verdict::Promote(p);
    }
    // Row 3 (:266): neither. ER5/ER6 only when NO prepared slot exists, so that the head can only
    // have described the current slot; with a prepared slot the head may describe that slot.
    Verdict::Freeze(match current {
        Some(c) if !prepared_present && c.fields.generation < head.generation => {
            FreezeKind::GenerationRegression
        }
        Some(c) if !prepared_present && c.fields.generation == head.generation => {
            FreezeKind::DigestConflict
        }
        _ => FreezeKind::CommittedStateMissing,
    })
}

/// Rule f: generation AND digest AND anchor. Generation alone never matches (C07 :140).
fn matches(head: &Checkpoint, blob: &Opened) -> bool {
    let d = digest(&blob.fields.vault_id, &blob.bytes);
    head.generation == blob.fields.generation
        && head.digest == d
        && head.anchor == anchor(&blob.fields.predecessor_anchor, &d)
}

fn committed(blob: Opened, head: &Checkpoint, head_bytes: [u8; checkpoint::LEN]) -> CommittedBlob {
    CommittedBlob {
        bytes: blob.bytes,
        fields: blob.fields,
        digest: head.digest,
        anchor: head.anchor,
        head: head_bytes,
    }
}

/// Rule h: rename the prepared slot over current, then the CHECKED directory flush (S2).
fn promote(store_dir: &Path) -> Result<(), RecoverError> {
    fs::rename(prepared_path(store_dir), current_path(store_dir))
        .map_err(|_| RecoverError::DurableWrite(DurableWriteError::Rename))?;
    fs_store::sync_dir_checked(store_dir).map_err(RecoverError::DurableWrite)
}

/// The authenticated head and its exact bytes, or the FREEZE it calls for.
type HeadOutcome = Result<(Checkpoint, [u8; checkpoint::LEN]), FreezeKind>;

/// Rule e. `Ok(Err(kind))` is a FREEZE; `Err` is an I/O failure, not a verdict. The checkpoint
/// directory is never created here. The head is opened once, read once (bounded) and its exact
/// bytes are returned with it (S7b X5); a non-regular head is a read failure, never followed.
fn read_head(state_root: &Path, lineage: &LineageFields) -> Result<HeadOutcome, RecoverError> {
    let path = paths::head_path(&paths::checkpoint_dir(state_root, &lineage.vault_id));
    let file = match open_regular(&path) {
        Ok(file) => file,
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            return Ok(Err(FreezeKind::CheckpointMissing));
        }
        Err(e) => return Err(RecoverError::CheckpointRead(e.kind())),
    };
    let bytes =
        checkpoint::read_bounded(file).map_err(|e| RecoverError::CheckpointRead(e.kind()))?;
    let head = match checkpoint::decode(
        &bytes,
        lineage.checkpoint_mac_key(),
        lineage.protection_mode,
    ) {
        Ok(head) if head.vault_id != lineage.vault_id => {
            return Ok(Err(FreezeKind::ForeignCheckpoint))
        }
        Ok(head) => head,
        Err(CheckpointError::Corrupt(kind)) => return Ok(Err(FreezeKind::CheckpointCorrupt(kind))),
        Err(CheckpointError::Unsupported(kind)) => {
            return Ok(Err(FreezeKind::CheckpointUnsupported(kind)))
        }
        Err(CheckpointError::Read(kind)) => return Err(RecoverError::CheckpointRead(kind)),
    };
    // decode() accepted exactly LEN bytes; anything else cannot reach here.
    match <[u8; checkpoint::LEN]>::try_from(bytes.as_slice()) {
        Ok(head_bytes) => Ok(Ok((head, head_bytes))),
        Err(_) => Ok(Err(FreezeKind::CheckpointCorrupt(CorruptKind::Length))),
    }
}

fn same_lineage(a: &LineageFields, b: &LineageFields) -> bool {
    a.vault_id == b.vault_id
        && a.protection_mode == b.protection_mode
        && keys_equal(a.checkpoint_mac_key(), b.checkpoint_mac_key())
}

/// No early exit on the first differing byte (`subtle` is not a dependency of this crate).
fn keys_equal(a: &[u8; 32], b: &[u8; 32]) -> bool {
    a.iter()
        .zip(b.iter())
        .fold(0u8, |acc, (x, y)| acc | (x ^ y))
        == 0
}

/// Rule b: at most `max_blob_len + 1` bytes are read, so a longer file is seen without reading the
/// rest of it, and it never reaches the opener.
fn read_slot(
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

/// An authenticated-but-malformed blob is a typed non-verdict error (S7b X1), never "unauthenticated".
fn open_slot(
    opener: &dyn LineageOpen,
    bytes: Option<Vec<u8>>,
    slot: Slot,
) -> Result<SlotState, RecoverError> {
    Ok(match bytes {
        None => SlotState::Absent,
        Some(bytes) => match opener.open(&bytes) {
            Ok(fields) => SlotState::Authenticated(Opened { bytes, fields }),
            Err(OpenError::Unauthenticated) => SlotState::Unauthenticated,
            Err(OpenError::Malformed) => return Err(RecoverError::BlobMalformed { slot }),
        },
    })
}

/// Rule a (:270). Only a regular (non-directory) entry of `dir` whose name `pid_of` reads as a
/// canonical pid that is NOT alive is removed -- `vault.qsv.prepared.tmp.<pid>` in the store, and
/// (S7b X7) `head.tmp.<pid>[.<n>]` in the locked lineage's checkpoint directory; everything else is
/// left exactly as found. No directory flush follows: a resurrected dead temp is simply removed
/// again next time.
fn remove_dead_temps(dir: &Path, pid_of: fn(&OsStr) -> Option<i32>) -> Result<(), RecoverError> {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(()),
        Err(e) => return Err(RecoverError::TempCleanup(e.kind())),
    };
    for entry in entries {
        let entry = entry.map_err(|e| RecoverError::TempCleanup(e.kind()))?;
        let Some(pid) = pid_of(&entry.file_name()) else {
            continue;
        };
        if pid_alive(pid) {
            continue;
        }
        let file_type = entry
            .file_type()
            .map_err(|e| RecoverError::TempCleanup(e.kind()))?;
        if file_type.is_dir() {
            continue;
        }
        match fs::remove_file(entry.path()) {
            Ok(()) => {}
            Err(e) if e.kind() == io::ErrorKind::NotFound => {}
            Err(e) => return Err(RecoverError::TempCleanup(e.kind())),
        }
    }
    Ok(())
}

/// The pid of a prepared temp name, in CANONICAL decimal only: no sign, no leading zero (so never
/// pid 0, which kill() reads as "my process group"), within pid_t. Anything else is None.
fn prepared_temp_pid(name: &OsStr) -> Option<i32> {
    let digits = name.to_str()?.strip_prefix(PREPARED_TEMP_PREFIX)?;
    if digits.is_empty() || digits.starts_with('0') || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    digits.parse().ok()
}

extern "C" {
    fn kill(pid: i32, sig: i32) -> i32;
}

/// ESRCH, "no such process": 3 on Linux and on macOS.
const ESRCH: i32 = 3;

/// kill(pid, 0) sends nothing; it only asks whether `pid` exists. ONLY ESRCH means dead: this
/// process, a live process of another user (EPERM) or any other answer counts as alive, so a temp
/// is never removed on a doubt. /proc is not used (the lib tests also run on macOS).
fn pid_alive(pid: i32) -> bool {
    // SAFETY: signal 0 performs the existence and permission checks only; no signal is delivered.
    if unsafe { kill(pid, 0) } == 0 {
        return true;
    }
    io::Error::last_os_error().raw_os_error() != Some(ESRCH)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fs_store::{arm_durable_flush_fault, DurableFlushPoint};
    use chacha20poly1305::aead::{Aead, KeyInit};
    use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
    use rand_core::{OsRng, RngCore};
    use sha2::{Digest as _, Sha256};
    use std::cell::Cell;
    use std::os::unix::fs::PermissionsExt;
    use zeroize::Zeroizing;

    /// The committed generation of most fixtures; the prepared successor is N + 1.
    const N: usize = 3;
    const MAX: usize = 1 << 20;
    const FILLER: usize = 48;
    const PLAIN: usize = 32 + 8 + 32 + 32 + 1 + FILLER;

    fn random32() -> [u8; 32] {
        let mut b = [0u8; 32];
        OsRng.fill_bytes(&mut b);
        b
    }

    /// THE TEST OPENER: a fixture for the blob, not a provider. An authenticated ChaCha20-Poly1305
    /// envelope under a random per-test key: nonce(12) || AEAD(vault_id || generation BE ||
    /// predecessor_anchor || Kc || profile byte || random filler). A tag failure -> Unauthenticated;
    /// an AUTHENTIC plaintext of the wrong length or with an unknown profile byte -> Malformed.
    struct TestOpener {
        aead: ChaCha20Poly1305,
        calls: Cell<u32>,
    }

    impl TestOpener {
        fn new() -> Self {
            let key = random32();
            Self {
                aead: ChaCha20Poly1305::new(Key::from_slice(&key)),
                calls: Cell::new(0),
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

    struct Lineage {
        opener: TestOpener,
        vault_id: [u8; 32],
        kc: [u8; 32],
        mode: ProtectionMode,
    }

    impl Lineage {
        fn new() -> Self {
            Self::with_mode(ProtectionMode::LocalCheckpoint)
        }

        fn with_mode(mode: ProtectionMode) -> Self {
            Self {
                opener: TestOpener::new(),
                vault_id: random32(),
                kc: random32(),
                mode,
            }
        }

        /// A fresh authentic blob; the random filler makes two calls give two different blobs.
        fn blob(&self, generation: u64, pred: &[u8; 32]) -> Blob {
            let bytes = self.opener.seal(
                &self.vault_id,
                generation,
                pred,
                &self.kc,
                self.mode.profile_byte(),
            );
            let d = digest(&self.vault_id, &bytes);
            Blob {
                anchor: anchor(pred, &d),
                digest: d,
                bytes,
                generation,
            }
        }

        /// Generations 0..len, each on its predecessor's anchor (genesis on 32 zero bytes, C7).
        fn chain(&self, len: usize) -> Vec<Blob> {
            let mut out: Vec<Blob> = Vec::new();
            for g in 0..len as u64 {
                let pred = out.last().map_or([0u8; 32], |b| b.anchor);
                out.push(self.blob(g, &pred));
            }
            out
        }

        /// An alternate authentic blob at `g`, on the same predecessor as `chain[g]`.
        fn alternate(&self, chain: &[Blob], g: usize) -> Blob {
            self.blob(g as u64, &chain[g - 1].anchor)
        }

        /// The same shape sealed under ANOTHER key: it never authenticates here.
        fn unauthentic(&self) -> Vec<u8> {
            TestOpener::new().seal(&self.vault_id, 1, &[0u8; 32], &self.kc, 1)
        }

        fn head(&self, b: &Blob) -> Vec<u8> {
            self.head_naming(self.vault_id, b)
        }

        fn head_naming(&self, vault_id: [u8; 32], b: &Blob) -> Vec<u8> {
            let ck = Checkpoint {
                mode: self.mode,
                vault_id,
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

    struct Store {
        root: tempfile::TempDir,
        store: PathBuf,
        state: PathBuf,
    }

    impl Store {
        fn new() -> Self {
            let root = tempfile::tempdir().expect("tempdir");
            private_dir(root.path());
            let store = root.path().join("store");
            let state = root.path().join("state");
            private_dir(&store);
            private_dir(&state);
            Self { root, store, state }
        }

        fn put_current(&self, bytes: &[u8]) {
            fs::write(current_path(&self.store), bytes).expect("write current");
        }

        fn put_prepared(&self, bytes: &[u8]) {
            fs::write(prepared_path(&self.store), bytes).expect("write prepared");
        }

        fn checkpoint_dir(&self, lin: &Lineage) -> PathBuf {
            paths::ensure_checkpoint_dir(&self.state, &lin.vault_id).expect("checkpoint dir")
        }

        fn put_head(&self, lin: &Lineage, head: &[u8]) {
            fs::write(paths::head_path(&self.checkpoint_dir(lin)), head).expect("write head");
        }

        fn head_path(&self, lin: &Lineage) -> PathBuf {
            paths::head_path(&paths::checkpoint_dir(&self.state, &lin.vault_id))
        }

        /// As begin() calls it: under the lineage's lock, naming the lineage (S7b X10).
        fn recover(&self, lin: &Lineage) -> Result<Recovered, RecoverError> {
            recover(
                &self.store,
                &self.state,
                MAX,
                &lin.opener,
                Some(&lin.vault_id),
            )
        }

        /// Every entry under the store and state roots: relative path -> sha256 (directories 0).
        fn trees(&self) -> Vec<(String, [u8; 32])> {
            let mut out = Vec::new();
            walk(self.root.path(), self.root.path(), &mut out);
            out.sort();
            out
        }
    }

    fn walk(root: &Path, dir: &Path, out: &mut Vec<(String, [u8; 32])>) {
        for entry in fs::read_dir(dir).expect("read_dir") {
            let path = entry.expect("entry").path();
            let rel = path
                .strip_prefix(root)
                .expect("prefix")
                .display()
                .to_string();
            if path.is_dir() {
                out.push((rel + "/", [0u8; 32]));
                walk(root, &path, out);
            } else {
                out.push((rel, Sha256::digest(fs::read(&path).expect("read")).into()));
            }
        }
    }

    fn summary(r: &Result<Recovered, RecoverError>) -> String {
        match r {
            Ok(Recovered::NoLineage) => "NoLineage".to_string(),
            Ok(Recovered::Open {
                current,
                stale_prepared,
            }) => format!(
                "Open({}, stale_prepared={stale_prepared})",
                current.fields.generation
            ),
            Ok(Recovered::Promoted { current }) => {
                format!("Promoted({})", current.fields.generation)
            }
            Ok(Recovered::Freeze(kind)) => format!("Freeze({kind:?})"),
            Err(RecoverError::OpenFailed { which }) => format!("Err(OpenFailed{{{which:?}}})"),
            Err(e) => format!("Err({e:?})"),
        }
    }

    /// A pid that is certainly not alive: a child that ran and was reaped. Guarded, so a reused pid
    /// fails the fixture loudly instead of passing for the wrong reason.
    fn dead_pid() -> u32 {
        let mut child = std::process::Command::new("true")
            .spawn()
            .expect("spawn true");
        let pid = child.id();
        child.wait().expect("wait");
        let as_pid_t = i32::try_from(pid).expect("pid fits pid_t");
        assert!(!pid_alive(as_pid_t), "fixture: pid {pid} is alive again");
        pid
    }

    // ---- the freeze fixtures (each individual test and PURE use the same builders)

    type Fixture = fn() -> (Store, Lineage);

    fn fx_r_ca() -> (Store, Lineage) {
        let (st, lin) = (Store::new(), Lineage::new());
        let chain = lin.chain(N + 1);
        st.put_current(&chain[N].bytes);
        st.checkpoint_dir(&lin);
        (st, lin)
    }

    fn fx_r_cc() -> (Store, Lineage) {
        let (st, lin) = (Store::new(), Lineage::new());
        let chain = lin.chain(N + 1);
        st.put_current(&chain[N].bytes);
        let mut head = lin.head(&chain[N]);
        head[50] ^= 1;
        st.put_head(&lin, &head);
        (st, lin)
    }

    fn fx_r_cs() -> (Store, Lineage) {
        let (st, lin) = (Store::new(), Lineage::new());
        let chain = lin.chain(N + 1);
        st.put_current(&chain[N].bytes);
        st.put_head(&lin, &lin.head(&chain[N - 1]));
        (st, lin)
    }

    fn fx_d1() -> (Store, Lineage) {
        let (st, lin) = (Store::new(), Lineage::new());
        let chain = lin.chain(N + 1);
        st.put_current(&chain[N - 2].bytes);
        st.put_head(&lin, &lin.head(&chain[N]));
        (st, lin)
    }

    fn fx_d1b() -> (Store, Lineage) {
        let (st, lin) = (Store::new(), Lineage::new());
        let chain = lin.chain(N + 1);
        let alternate = lin.alternate(&chain, N);
        assert_eq!(alternate.generation, chain[N].generation);
        assert_ne!(alternate.digest, chain[N].digest);
        st.put_current(&alternate.bytes);
        st.put_head(&lin, &lin.head(&chain[N]));
        (st, lin)
    }

    fn fx_fn2_l() -> (Store, Lineage) {
        let (st, lin) = (Store::new(), Lineage::new());
        let chain = lin.chain(N + 2);
        st.put_current(&chain[N].bytes);
        st.put_prepared(&chain[N + 1].bytes);
        st.put_head(&lin, &lin.head(&lin.alternate(&chain, N + 1)));
        (st, lin)
    }

    fn fx_gi1() -> (Store, Lineage) {
        let (st, lin) = (Store::new(), Lineage::new());
        st.put_prepared(&lin.blob(0, &[0u8; 32]).bytes);
        (st, lin)
    }

    fn fx_mismatch(which: &str) -> (Store, Lineage) {
        let (st, lin) = (Store::new(), Lineage::new());
        let chain = lin.chain(N + 1);
        st.put_current(&chain[N].bytes);
        st.put_head(&lin, &lin.head(&chain[N]));
        let pred = &chain[N].anchor;
        let next = N as u64 + 1;
        let other = match which {
            "vault_id" => lin.opener.seal(&random32(), next, pred, &lin.kc, 1),
            "kc" => lin.opener.seal(&lin.vault_id, next, pred, &random32(), 1),
            _ => lin.opener.seal(&lin.vault_id, next, pred, &lin.kc, 2),
        };
        st.put_prepared(&other);
        (st, lin)
    }

    fn fx_mismatch_vault_id() -> (Store, Lineage) {
        fx_mismatch("vault_id")
    }

    fn fx_unsupported() -> (Store, Lineage) {
        let (st, lin) = (Store::new(), Lineage::new());
        let chain = lin.chain(N + 1);
        st.put_current(&chain[N].bytes);
        let mut head = lin.head(&chain[N]);
        head[9] = 2;
        st.put_head(&lin, &head);
        (st, lin)
    }

    fn fx_foreign() -> (Store, Lineage) {
        let (st, lin) = (Store::new(), Lineage::new());
        let chain = lin.chain(N + 1);
        st.put_current(&chain[N].bytes);
        st.put_head(&lin, &lin.head_naming(random32(), &chain[N]));
        (st, lin)
    }

    // ---- the sealed tests

    #[test]
    fn r0_cur_head_matches_current_opens_and_changes_nothing() {
        let (st, lin) = (Store::new(), Lineage::new());
        let chain = lin.chain(N + 1);
        st.put_current(&chain[N].bytes);
        st.put_head(&lin, &lin.head(&chain[N]));
        let before = st.trees();
        let r = st.recover(&lin);
        assert_eq!(summary(&r), "Open(3, stale_prepared=false)");
        let Ok(Recovered::Open { current, .. }) = r else {
            unreachable!()
        };
        assert_eq!(current.bytes, chain[N].bytes);
        assert_eq!(
            (current.digest, current.anchor),
            (chain[N].digest, chain[N].anchor)
        );
        assert_eq!(st.trees(), before, "an OPEN changes no file");
    }

    #[test]
    fn r0_prep_head_matches_prepared_promotes() {
        let (st, lin) = (Store::new(), Lineage::new());
        let chain = lin.chain(N + 2);
        st.put_current(&chain[N].bytes);
        st.put_prepared(&chain[N + 1].bytes);
        st.put_head(&lin, &lin.head(&chain[N + 1]));
        let r = st.recover(&lin);
        assert_eq!(summary(&r), "Promoted(4)");
        assert!(
            !prepared_path(&st.store).exists(),
            "the prepared slot was renamed"
        );
        assert_eq!(
            fs::read(current_path(&st.store)).expect("current"),
            chain[N + 1].bytes
        );
        let Ok(Recovered::Promoted { current }) = r else {
            unreachable!()
        };
        assert_eq!(current.bytes, chain[N + 1].bytes);
    }

    #[test]
    fn k_a2_prepared_newer_but_head_names_current_opens_current() {
        let (st, lin) = (Store::new(), Lineage::new());
        let chain = lin.chain(N + 2);
        st.put_current(&chain[N].bytes);
        st.put_prepared(&chain[N + 1].bytes);
        st.put_head(&lin, &lin.head(&chain[N]));
        assert_eq!(
            summary(&st.recover(&lin)),
            "Open(3, stale_prepared=true)",
            "the largest generation is never chosen"
        );
        assert_eq!(
            fs::read(prepared_path(&st.store)).expect("prepared kept"),
            chain[N + 1].bytes,
            "the stale slot is reported, not deleted (B4: S5 removes it)"
        );
    }

    #[test]
    fn k_m3_head_temp_is_never_authority() {
        let (st, lin) = (Store::new(), Lineage::new());
        let chain = lin.chain(N + 2);
        st.put_current(&chain[N].bytes);
        st.put_prepared(&chain[N + 1].bytes);
        st.put_head(&lin, &lin.head(&chain[N]));
        let temp = paths::head_temp_path(&st.checkpoint_dir(&lin), dead_pid(), 0);
        let next_head = lin.head(&chain[N + 1]);
        fs::write(&temp, &next_head).expect("head temp");
        assert_eq!(summary(&st.recover(&lin)), "Open(3, stale_prepared=true)");
        assert!(
            !temp.exists(),
            "a checkpoint temp is never authority, and a dead pid's is removed (S7b X7, D40)"
        );
    }

    #[test]
    fn r_ca_absent_head_freezes_missing_never_rebuilt() {
        let (st, lin) = fx_r_ca();
        let current = fs::read(current_path(&st.store)).expect("current");
        assert_eq!(summary(&st.recover(&lin)), "Freeze(CheckpointMissing)");
        assert!(!st.head_path(&lin).exists(), "never rebuilt from the vault");
        assert_eq!(fs::read(current_path(&st.store)).expect("current"), current);
    }

    #[test]
    fn r_cc_flipped_head_byte_freezes_corrupt() {
        let (st, lin) = fx_r_cc();
        assert_eq!(summary(&st.recover(&lin)), "Freeze(CheckpointCorrupt(Mac))");
    }

    #[test]
    fn r_cs_older_authentic_head_freezes_committed_state_missing() {
        let (st, lin) = fx_r_cs();
        assert_eq!(
            summary(&st.recover(&lin)),
            "Freeze(CommittedStateMissing)",
            "never one step behind"
        );
    }

    #[test]
    fn d1_older_authentic_current_freezes_generation_regression() {
        let (st, lin) = fx_d1();
        assert_eq!(summary(&st.recover(&lin)), "Freeze(GenerationRegression)");
    }

    #[test]
    fn d1b_same_generation_alternate_freezes_digest_conflict() {
        let (st, lin) = fx_d1b();
        assert_eq!(
            summary(&st.recover(&lin)),
            "Freeze(DigestConflict)",
            "generation equality alone never matches"
        );
    }

    #[test]
    fn d2_store_and_head_restored_together_opens_not_detected() {
        let (st, lin) = (Store::new(), Lineage::new());
        let chain = lin.chain(N + 2);
        let k = N - 1;
        st.put_current(&chain[N + 1].bytes);
        st.put_head(&lin, &lin.head(&chain[N + 1]));
        assert_eq!(summary(&st.recover(&lin)), "Open(4, stale_prepared=false)");
        // The whole lineage restored together to the committed pair at k: indistinguishable from
        // a real state at k. C07 T1 D2: NOT COVERED -- a freeze here would be a false claim.
        st.put_current(&chain[k].bytes);
        st.put_head(&lin, &lin.head(&chain[k]));
        assert_eq!(summary(&st.recover(&lin)), "Open(2, stale_prepared=false)");
    }

    #[test]
    fn fn2_l_head_naming_a_third_blob_freezes_permanently() {
        let (st, lin) = fx_fn2_l();
        let before = st.trees();
        assert_eq!(summary(&st.recover(&lin)), "Freeze(CommittedStateMissing)");
        assert_eq!(
            summary(&st.recover(&lin)),
            "Freeze(CommittedStateMissing)",
            "FN2 local form: permanent"
        );
        assert_eq!(st.trees(), before);
    }

    #[test]
    fn gi1_genesis_prepared_without_head_freezes_missing() {
        let (st, lin) = fx_gi1();
        let b0 = fs::read(prepared_path(&st.store)).expect("B0");
        assert_eq!(
            summary(&st.recover(&lin)),
            "Freeze(CheckpointMissing)",
            "DD-5 LITERAL"
        );
        assert_eq!(
            fs::read(prepared_path(&st.store)).expect("B0 kept"),
            b0,
            "no auto-discard of B0"
        );
        assert!(!current_path(&st.store).exists());
        assert!(!st.head_path(&lin).exists());
    }

    #[test]
    fn r_tmp_only_dead_pid_prepared_temps_are_removed() {
        let (st, lin) = (Store::new(), Lineage::new());
        let dead = dead_pid();
        let dead_temp = prepared_temp_path(&st.store, dead);
        let kept = [
            prepared_temp_path(&st.store, std::process::id()),
            st.store.join(format!("{PREPARED_TEMP_PREFIX}0{dead}")),
            st.store.join(format!("{PREPARED_TEMP_PREFIX}x")),
            st.store.join(format!("{PREPARED_TEMP_PREFIX}0")),
            st.store.join(format!("{PREPARED_TEMP_PREFIX}99999999999")),
        ];
        for path in std::iter::once(&dead_temp).chain(kept.iter()) {
            fs::write(path, b"partial").expect("plant temp");
        }
        let dead_dir = prepared_temp_path(&st.store, dead_pid());
        fs::create_dir(&dead_dir).expect("plant dir");
        assert_eq!(summary(&st.recover(&lin)), "NoLineage");
        assert!(!dead_temp.exists(), "a dead pid's prepared temp is removed");
        for path in &kept {
            assert!(path.exists(), "{} must be untouched", path.display());
        }
        assert!(dead_dir.is_dir(), "a directory is not a temp file");
    }

    #[test]
    fn rc1_promotion_dir_flush_failure_then_recover_opens() {
        let (st, lin) = (Store::new(), Lineage::new());
        let chain = lin.chain(N + 2);
        st.put_current(&chain[N].bytes);
        st.put_prepared(&chain[N + 1].bytes);
        st.put_head(&lin, &lin.head(&chain[N + 1]));
        arm_durable_flush_fault(DurableFlushPoint::Dir, 0);
        let r = st.recover(&lin);
        assert!(
            matches!(
                r,
                Err(RecoverError::DurableWrite(DurableWriteError::DirFlush))
            ),
            "{}",
            summary(&r)
        );
        assert!(!prepared_path(&st.store).exists(), "the rename happened");
        assert_eq!(
            fs::read(current_path(&st.store)).expect("current"),
            chain[N + 1].bytes
        );
        assert_eq!(summary(&st.recover(&lin)), "Open(4, stale_prepared=false)");
    }

    #[test]
    fn pure_every_freeze_leaves_both_trees_identical() {
        // The verdicts are the individual tests'; this one pins that a freeze writes nothing and
        // that recovering again gives the same answer.
        let fixtures: [(&str, Fixture); 10] = [
            ("R-CA", fx_r_ca),
            ("R-CC", fx_r_cc),
            ("R-CS", fx_r_cs),
            ("D1", fx_d1),
            ("D1b", fx_d1b),
            ("FN2-L", fx_fn2_l),
            ("GI-1", fx_gi1),
            ("MISMATCH", fx_mismatch_vault_id),
            ("UNSUPPORTED", fx_unsupported),
            ("FOREIGN", fx_foreign),
        ];
        for (name, fixture) in fixtures {
            let (st, lin) = fixture();
            let before = st.trees();
            let first = summary(&st.recover(&lin));
            assert_eq!(st.trees(), before, "{name}: {first} changed a file");
            assert_eq!(summary(&st.recover(&lin)), first, "{name}: not permanent");
        }
    }

    #[test]
    fn nolin_empty_store_is_no_lineage() {
        let (st, lin) = (Store::new(), Lineage::new());
        assert_eq!(summary(&st.recover(&lin)), "NoLineage");
        let missing = st.state.join("missing");
        assert_eq!(
            summary(&recover(
                &missing,
                &st.state,
                MAX,
                &lin.opener,
                Some(&lin.vault_id)
            )),
            "NoLineage"
        );
        assert_eq!(lin.opener.calls.get(), 0);
    }

    #[test]
    fn unauth_nothing_authenticates_is_open_failed() {
        let (st, lin) = (Store::new(), Lineage::new());
        st.put_current(&lin.unauthentic());
        st.put_prepared(&lin.unauthentic());
        assert_eq!(summary(&st.recover(&lin)), "Err(OpenFailed{Both})");
        assert_eq!(lin.opener.calls.get(), 2);
        fs::remove_file(prepared_path(&st.store)).expect("rm prepared");
        assert_eq!(summary(&st.recover(&lin)), "Err(OpenFailed{Current})");
    }

    #[test]
    fn bound_oversized_blob_refused_before_the_opener() {
        let lin = Lineage::new();
        let chain = lin.chain(N + 2);
        let len = chain[N].bytes.len();
        let st = Store::new();
        st.put_current(&chain[N].bytes);
        st.put_head(&lin, &lin.head(&chain[N]));
        let r = recover(
            &st.store,
            &st.state,
            len - 1,
            &lin.opener,
            Some(&lin.vault_id),
        );
        assert!(
            matches!(
                r,
                Err(RecoverError::BlobTooLarge {
                    slot: Slot::Current
                })
            ),
            "{}",
            summary(&r)
        );
        assert_eq!(lin.opener.calls.get(), 0, "the opener never saw it");
        st.put_prepared(&[chain[N + 1].bytes.as_slice(), &[0u8]].concat());
        let r = recover(&st.store, &st.state, len, &lin.opener, Some(&lin.vault_id));
        assert!(
            matches!(
                r,
                Err(RecoverError::BlobTooLarge {
                    slot: Slot::Prepared
                })
            ),
            "{}",
            summary(&r)
        );
        assert_eq!(lin.opener.calls.get(), 0, "the opener never saw either");
        fs::remove_file(prepared_path(&st.store)).expect("rm prepared");
        assert_eq!(
            summary(&recover(
                &st.store,
                &st.state,
                len,
                &lin.opener,
                Some(&lin.vault_id)
            )),
            "Open(3, stale_prepared=false)",
            "exactly the bound is accepted"
        );
    }

    #[test]
    fn mismatch_disagreeing_blobs_freeze() {
        for which in ["vault_id", "kc", "mode"] {
            let (st, lin) = fx_mismatch(which);
            assert_eq!(
                summary(&st.recover(&lin)),
                "Freeze(LineageMismatch)",
                "{which}"
            );
        }
    }

    #[test]
    fn tpm_lineage_refused_before_the_head_is_read() {
        let (st, lin) = (Store::new(), Lineage::with_mode(ProtectionMode::Tpm));
        let chain = lin.chain(1);
        st.put_current(&chain[0].bytes);
        st.put_head(&lin, &lin.head(&chain[0]));
        let r = st.recover(&lin);
        assert!(
            matches!(r, Err(RecoverError::NotLocalCheckpoint)),
            "{}",
            summary(&r)
        );
    }

    #[test]
    fn read_error_is_typed_not_a_verdict() {
        let (st, lin) = (Store::new(), Lineage::new());
        fs::create_dir(current_path(&st.store)).expect("a directory at vault.qsv");
        let r = st.recover(&lin);
        assert!(
            matches!(
                r,
                Err(RecoverError::BlobRead {
                    slot: Slot::Current,
                    ..
                })
            ),
            "{}",
            summary(&r)
        );
    }

    // ---- S7b (SEALED_EXPECTATION_S7b.md sec 1)

    /// An AUTHENTIC blob with an unknown profile byte: the opener returns Malformed.
    fn malformed(lin: &Lineage, generation: u64, pred: &[u8; 32]) -> Vec<u8> {
        lin.opener.seal(&lin.vault_id, generation, pred, &lin.kc, 0)
    }

    #[test]
    fn x1_malformed_blob_is_a_typed_non_verdict() {
        let (st, lin) = (Store::new(), Lineage::new());
        let chain = lin.chain(N + 1);
        st.put_current(&malformed(&lin, N as u64, &chain[N - 1].anchor));
        st.put_head(&lin, &lin.head(&chain[N]));
        let before = st.trees();
        let r = st.recover(&lin);
        assert!(
            matches!(
                r,
                Err(RecoverError::BlobMalformed {
                    slot: Slot::Current
                })
            ),
            "{}",
            summary(&r)
        );
        let code = r.unwrap_err().code();
        assert_eq!(code, "vault_parse_failed");
        assert_ne!(code, crate::msgqueue::MSGQUEUE_VAULT_LOCKED);
        assert_eq!(st.trees(), before, "a non-verdict writes nothing");
        // An authentic current beside a Malformed prepared slot: refused, never classified.
        st.put_current(&chain[N].bytes);
        st.put_prepared(&malformed(&lin, N as u64 + 1, &chain[N].anchor));
        let r = st.recover(&lin);
        assert!(
            matches!(
                r,
                Err(RecoverError::BlobMalformed {
                    slot: Slot::Prepared
                })
            ),
            "{}",
            summary(&r)
        );
        // Unauthenticated keeps its own kind.
        fs::remove_file(prepared_path(&st.store)).expect("rm prepared");
        st.put_current(&lin.unauthentic());
        assert_eq!(summary(&st.recover(&lin)), "Err(OpenFailed{Current})");
    }

    #[test]
    fn x7_rule_a_removes_dead_pid_head_temps_both_shapes() {
        let (st, lin) = (Store::new(), Lineage::new());
        let ck = st.checkpoint_dir(&lin);
        let dead = dead_pid();
        let removed = [
            ck.join(format!("head.tmp.{dead}")),
            paths::head_temp_path(&ck, dead, 3),
        ];
        let me = std::process::id();
        let kept = [
            ck.join(format!("head.tmp.{me}")),
            paths::head_temp_path(&ck, me, 0),
            ck.join(format!("head.tmp.0{dead}")),
            ck.join(format!("head.tmp.{dead}.03")),
            ck.join(format!("head.tmp.{dead}.")),
            ck.join(format!("head.tmp.{dead}.1.2")),
            ck.join("head.tmp.x"),
            ck.join("head.tmp.99999999999"),
        ];
        for path in removed.iter().chain(kept.iter()) {
            fs::write(path, b"partial").expect("plant head temp");
        }
        let dead_dir = paths::head_temp_path(&ck, dead_pid(), 1);
        fs::create_dir(&dead_dir).expect("plant dir");
        let other = Lineage::new();
        let other_temp = paths::head_temp_path(&st.checkpoint_dir(&other), dead, 0);
        fs::write(&other_temp, b"partial").expect("plant other lineage's temp");
        assert_eq!(summary(&st.recover(&lin)), "NoLineage");
        for path in &removed {
            assert!(!path.exists(), "{} must be removed", path.display());
        }
        for path in &kept {
            assert!(path.exists(), "{} must be untouched", path.display());
        }
        assert!(dead_dir.is_dir(), "a directory is not a temp file");
        assert!(
            other_temp.exists(),
            "only the named lineage's directory is swept"
        );
    }

    #[test]
    fn x8_debug_never_formats_the_blob_bytes() {
        let lin = Lineage::new();
        let b = lin.blob(0, &[0u8; 32]);
        let blob = || CommittedBlob {
            bytes: vec![0xAB; 64],
            fields: lin.opener.open(&b.bytes).expect("authentic"),
            digest: b.digest,
            anchor: b.anchor,
            head: [0u8; checkpoint::LEN],
        };
        for text in [
            format!("{:?}", blob()),
            format!(
                "{:?}",
                Recovered::Open {
                    current: blob(),
                    stale_prepared: false
                }
            ),
            format!("{:?}", Recovered::Promoted { current: blob() }),
        ] {
            assert!(text.contains("bytes_len: 64"), "{text}");
            assert!(!text.contains("bytes: ["), "the blob was formatted: {text}");
            assert!(!text.contains("head:"), "{text}");
        }
    }

    // ---- THE TABLE: SEALED_EXPECTATION_S4.md sec 4, T01-T45, transcribed row for row.

    #[derive(Clone, Copy)]
    enum S {
        Auth,
        Unauth,
        Absent,
    }

    #[derive(Clone, Copy)]
    enum H {
        Absent,
        Corrupt,
        Unsupported,
        Foreign,
        MatchCurrent,
        MatchPrepared,
        /// Neither; the reference blob's generation (current's if it authenticates, else the
        /// prepared one's) is below, equal to, or above the head's.
        RefBelow,
        RefEqual,
        RefAbove,
    }

    use H::{
        Corrupt, Foreign, MatchCurrent, MatchPrepared, RefAbove, RefBelow, RefEqual, Unsupported,
    };
    use S::{Auth, Unauth};

    const TABLE: [(&str, S, S, H, &str); 45] = [
        ("T01", S::Absent, S::Absent, H::Absent, "NoLineage"),
        (
            "T02",
            Unauth,
            S::Absent,
            H::Absent,
            "Err(OpenFailed{Current})",
        ),
        (
            "T03",
            S::Absent,
            Unauth,
            H::Absent,
            "Err(OpenFailed{Prepared})",
        ),
        ("T04", Unauth, Unauth, H::Absent, "Err(OpenFailed{Both})"),
        ("T05", Auth, Auth, H::Absent, "Freeze(CheckpointMissing)"),
        ("T06", Auth, Auth, Corrupt, "Freeze(CheckpointCorrupt(Mac))"),
        (
            "T07",
            Auth,
            Auth,
            Unsupported,
            "Freeze(CheckpointUnsupported(Version))",
        ),
        ("T08", Auth, Auth, Foreign, "Freeze(ForeignCheckpoint)"),
        (
            "T09",
            Auth,
            Auth,
            MatchCurrent,
            "Open(3, stale_prepared=true)",
        ),
        ("T10", Auth, Auth, MatchPrepared, "Promoted(4)"),
        ("T11", Auth, Auth, RefBelow, "Freeze(CommittedStateMissing)"),
        ("T12", Auth, Auth, RefEqual, "Freeze(CommittedStateMissing)"),
        ("T13", Auth, Auth, RefAbove, "Freeze(CommittedStateMissing)"),
        ("T14", Auth, Unauth, H::Absent, "Freeze(CheckpointMissing)"),
        (
            "T15",
            Auth,
            Unauth,
            Corrupt,
            "Freeze(CheckpointCorrupt(Mac))",
        ),
        (
            "T16",
            Auth,
            Unauth,
            Unsupported,
            "Freeze(CheckpointUnsupported(Version))",
        ),
        ("T17", Auth, Unauth, Foreign, "Freeze(ForeignCheckpoint)"),
        (
            "T18",
            Auth,
            Unauth,
            MatchCurrent,
            "Open(3, stale_prepared=true)",
        ),
        (
            "T19",
            Auth,
            Unauth,
            RefBelow,
            "Freeze(CommittedStateMissing)",
        ),
        (
            "T20",
            Auth,
            Unauth,
            RefEqual,
            "Freeze(CommittedStateMissing)",
        ),
        (
            "T21",
            Auth,
            Unauth,
            RefAbove,
            "Freeze(CommittedStateMissing)",
        ),
        (
            "T22",
            Auth,
            S::Absent,
            H::Absent,
            "Freeze(CheckpointMissing)",
        ),
        (
            "T23",
            Auth,
            S::Absent,
            Corrupt,
            "Freeze(CheckpointCorrupt(Mac))",
        ),
        (
            "T24",
            Auth,
            S::Absent,
            Unsupported,
            "Freeze(CheckpointUnsupported(Version))",
        ),
        ("T25", Auth, S::Absent, Foreign, "Freeze(ForeignCheckpoint)"),
        (
            "T26",
            Auth,
            S::Absent,
            MatchCurrent,
            "Open(3, stale_prepared=false)",
        ),
        (
            "T27",
            Auth,
            S::Absent,
            RefBelow,
            "Freeze(GenerationRegression)",
        ),
        ("T28", Auth, S::Absent, RefEqual, "Freeze(DigestConflict)"),
        (
            "T29",
            Auth,
            S::Absent,
            RefAbove,
            "Freeze(CommittedStateMissing)",
        ),
        ("T30", Unauth, Auth, H::Absent, "Freeze(CheckpointMissing)"),
        (
            "T31",
            Unauth,
            Auth,
            Corrupt,
            "Freeze(CheckpointCorrupt(Mac))",
        ),
        (
            "T32",
            Unauth,
            Auth,
            Unsupported,
            "Freeze(CheckpointUnsupported(Version))",
        ),
        ("T33", Unauth, Auth, Foreign, "Freeze(ForeignCheckpoint)"),
        ("T34", Unauth, Auth, MatchPrepared, "Promoted(4)"),
        (
            "T35",
            Unauth,
            Auth,
            RefBelow,
            "Freeze(CommittedStateMissing)",
        ),
        (
            "T36",
            Unauth,
            Auth,
            RefEqual,
            "Freeze(CommittedStateMissing)",
        ),
        (
            "T37",
            Unauth,
            Auth,
            RefAbove,
            "Freeze(CommittedStateMissing)",
        ),
        (
            "T38",
            S::Absent,
            Auth,
            H::Absent,
            "Freeze(CheckpointMissing)",
        ),
        (
            "T39",
            S::Absent,
            Auth,
            Corrupt,
            "Freeze(CheckpointCorrupt(Mac))",
        ),
        (
            "T40",
            S::Absent,
            Auth,
            Unsupported,
            "Freeze(CheckpointUnsupported(Version))",
        ),
        ("T41", S::Absent, Auth, Foreign, "Freeze(ForeignCheckpoint)"),
        ("T42", S::Absent, Auth, MatchPrepared, "Promoted(4)"),
        (
            "T43",
            S::Absent,
            Auth,
            RefBelow,
            "Freeze(CommittedStateMissing)",
        ),
        (
            "T44",
            S::Absent,
            Auth,
            RefEqual,
            "Freeze(CommittedStateMissing)",
        ),
        (
            "T45",
            S::Absent,
            Auth,
            RefAbove,
            "Freeze(CommittedStateMissing)",
        ),
    ];

    /// current = chain[3], prepared = chain[4]. The reference generation r is 3 when current
    /// authenticates, else 4: RefBelow names an alternate blob at r + 1, RefEqual one at r, and
    /// RefAbove the true head at r - 1.
    fn table_fixture(current: S, prepared: S, head: H) -> (Store, Lineage) {
        let (st, lin) = (Store::new(), Lineage::new());
        let chain = lin.chain(N + 2);
        match current {
            Auth => st.put_current(&chain[N].bytes),
            Unauth => st.put_current(&lin.unauthentic()),
            S::Absent => {}
        }
        match prepared {
            Auth => st.put_prepared(&chain[N + 1].bytes),
            Unauth => st.put_prepared(&lin.unauthentic()),
            S::Absent => {}
        }
        let r = if matches!(current, Auth) { N } else { N + 1 };
        let bytes = match head {
            H::Absent => None,
            Corrupt => {
                let mut b = lin.head(&chain[N]);
                b[60] ^= 1;
                Some(b)
            }
            Unsupported => {
                let mut b = lin.head(&chain[N]);
                b[9] = 2;
                Some(b)
            }
            Foreign => Some(lin.head_naming(random32(), &chain[N])),
            MatchCurrent => Some(lin.head(&chain[N])),
            MatchPrepared => Some(lin.head(&chain[N + 1])),
            RefBelow => Some(lin.head(&lin.alternate(&chain, r + 1))),
            RefEqual => Some(lin.head(&lin.alternate(&chain, r))),
            RefAbove => Some(lin.head(&chain[r - 1])),
        };
        st.checkpoint_dir(&lin);
        if let Some(b) = bytes {
            st.put_head(&lin, &b);
        }
        (st, lin)
    }

    #[test]
    fn table_every_combination_matches_the_seal() {
        let mut differ = Vec::new();
        for (id, current, prepared, head, want) in TABLE {
            let (st, lin) = table_fixture(current, prepared, head);
            let got = summary(&st.recover(&lin));
            if got != want {
                differ.push(format!("{id}: sealed {want}, got {got}"));
            }
        }
        assert!(differ.is_empty(), "rows differ:\n{}", differ.join("\n"));
    }
}
