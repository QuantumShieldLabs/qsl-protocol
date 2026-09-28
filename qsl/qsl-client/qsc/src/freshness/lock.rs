//! C07 FN1 (:258) / T6.4 D32 (:334): THE LINEAGE LOCK (NA-0787 F04-C07P S5).
//!
//! One exclusive, NON-BLOCKING flock on `<STATE>/qsl/freshness/<hex vault_id>/lock`. The path is
//! keyed by vault_id, so every pathname or copy of one lineage's store meets the same lock.
//! Contention is a typed refusal, never a wait. The lock file is created 0600 if absent and is
//! NEVER unlinked: dropping the lock only unlocks and closes it. After the flock the holder checks
//! that the inode it locked is still the one at the path (fstat(fd) against the path's own stat,
//! which does not follow a symlink) and retries a bounded number of times otherwise, so an
//! unlinked-and-recreated lock file can never give two brokers two different inodes.
//!
//! LOCK ORDER (a precondition for every caller): the caller's store lock `.qsc.lock`
//! (model::LockGuard) FIRST, then this lineage lock. The reverse order is FORBIDDEN. Both locks are
//! non-blocking, so a reversal would surface as a contention refusal rather than a deadlock.
//!
//! This is its own flock (lib.rs's `flock` extern), not model::LockGuard: that guard has no inode
//! re-check and keeps a re-entrant per-thread registry, neither of which FN1 wants here.

use super::paths;
use std::fs::{self, File, OpenOptions};
use std::io;
use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
use std::os::unix::io::AsRawFd;
use std::path::{Path, PathBuf};

// The flock operations; the same values on Linux and on macOS (model/mod.rs:75-78).
const LOCK_EX: i32 = 2;
const LOCK_NB: i32 = 4;
const LOCK_UN: i32 = 8;

/// How many times the inode re-check may send the holder back to reopen the path (sealed).
pub(crate) const LOCK_ATTEMPTS: u32 = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LockError {
    /// The lock file could not be opened or created.
    Open(io::ErrorKind),
    /// Another holder has it. A refusal, never a wait.
    Contended,
    Flock(io::ErrorKind),
    Stat(io::ErrorKind),
    /// The path's inode changed after every one of the LOCK_ATTEMPTS flocks.
    InodeRetriesExhausted,
}

/// Held for the unlocked lifetime (FN1). Dropping it unlocks; the file stays.
#[derive(Debug)]
pub(crate) struct LineageLock {
    file: File,
    path: PathBuf,
}

impl LineageLock {
    pub(crate) fn acquire(checkpoint_dir: &Path) -> Result<Self, LockError> {
        let path = paths::lock_path(checkpoint_dir);
        for _ in 0..LOCK_ATTEMPTS {
            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(false)
                .mode(0o600)
                .open(&path)
                .map_err(|e| LockError::Open(e.kind()))?;
            #[cfg(test)]
            test_seam::between_open_and_flock(&path);
            // SAFETY: flock on a descriptor this function owns; no memory is passed.
            if unsafe { crate::flock(file.as_raw_fd(), LOCK_EX | LOCK_NB) } != 0 {
                let err = io::Error::last_os_error();
                return Err(if err.kind() == io::ErrorKind::WouldBlock {
                    LockError::Contended
                } else {
                    LockError::Flock(err.kind())
                });
            }
            let held = file.metadata().map_err(|e| LockError::Stat(e.kind()))?;
            match fs::symlink_metadata(&path) {
                Ok(md) if (md.dev(), md.ino()) == (held.dev(), held.ino()) => {
                    return Ok(Self { file, path });
                }
                Ok(_) => {}
                Err(e) if e.kind() == io::ErrorKind::NotFound => {}
                Err(e) => return Err(LockError::Stat(e.kind())),
            }
            // A stale inode: closing `file` here releases it; reopen the path.
        }
        Err(LockError::InodeRetriesExhausted)
    }

    pub(crate) fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for LineageLock {
    fn drop(&mut self) {
        // SAFETY: as in `acquire`. The file is closed right after; it is never unlinked (FN1).
        let _ = unsafe { crate::flock(self.file.as_raw_fd(), LOCK_UN) };
    }
}

#[cfg(test)]
pub(crate) mod test_seam {
    //! LK-2's seam: between the open and the flock, unlink the lock file and create a new one at
    //! the same path, `n` times on this thread. Compiled only under cfg(test).
    use std::cell::Cell;
    use std::fs::{self, OpenOptions};
    use std::os::unix::fs::OpenOptionsExt;
    use std::path::Path;

    thread_local! {
        static SWAPS: Cell<u32> = const { Cell::new(0) };
    }

    pub(crate) fn arm_lock_file_swaps(n: u32) {
        SWAPS.with(|c| c.set(n));
    }

    pub(super) fn between_open_and_flock(path: &Path) {
        SWAPS.with(|c| {
            let n = c.get();
            if n > 0 {
                c.set(n - 1);
                fs::remove_file(path).expect("seam: unlink the lock file");
                OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .mode(0o600)
                    .open(path)
                    .expect("seam: recreate the lock file");
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::test_seam::arm_lock_file_swaps;
    use super::*;
    use std::os::unix::fs::PermissionsExt;

    fn private_dir() -> tempfile::TempDir {
        let td = tempfile::tempdir().expect("tempdir");
        fs::set_permissions(td.path(), fs::Permissions::from_mode(0o700)).expect("chmod 0700");
        td
    }

    fn path_id(path: &Path) -> (u64, u64) {
        let md = fs::symlink_metadata(path).expect("lock file present");
        (md.dev(), md.ino())
    }

    fn held_id(lock: &LineageLock) -> (u64, u64) {
        let md = lock.file.metadata().expect("fstat");
        (md.dev(), md.ino())
    }

    #[test]
    fn lk_2_recreated_lock_file_detected_and_retried() {
        let td = private_dir();
        arm_lock_file_swaps(1);
        let lock = LineageLock::acquire(td.path()).expect("acquired after one retry");
        assert_eq!(
            held_id(&lock),
            path_id(&paths::lock_path(td.path())),
            "the holder must hold the inode that is at the path now"
        );
        assert_eq!(
            LineageLock::acquire(td.path()).map(|_| ()),
            Err(LockError::Contended),
            "a second broker must meet the same inode"
        );
    }

    #[test]
    fn lk_2x_inode_retries_are_bounded() {
        let td = private_dir();
        arm_lock_file_swaps(LOCK_ATTEMPTS);
        assert_eq!(
            LineageLock::acquire(td.path()).map(|_| ()),
            Err(LockError::InodeRetriesExhausted)
        );
        assert!(paths::lock_path(td.path()).exists(), "the lock file exists");
        let lock = LineageLock::acquire(td.path()).expect("an unarmed acquire succeeds");
        assert_eq!(held_id(&lock), path_id(&paths::lock_path(td.path())));
    }

    #[test]
    fn lk_3_lock_file_survives_release() {
        let td = private_dir();
        let path = paths::lock_path(td.path());
        let lock = LineageLock::acquire(td.path()).expect("acquire");
        assert_eq!(lock.path(), path.as_path());
        let id = held_id(&lock);
        drop(lock);
        assert_eq!(path_id(&path), id, "never unlinked: the same inode stays");
        let again = LineageLock::acquire(td.path()).expect("re-acquire after release");
        assert_eq!(held_id(&again), id);
    }
}
