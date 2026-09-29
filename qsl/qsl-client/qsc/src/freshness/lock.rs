//! C07 FN1 (:258) / T6.4 D32 (:334): THE LINEAGE LOCK (NA-0787 F04-C07P S5).
//!
//! One exclusive, NON-BLOCKING flock on `<STATE>/qsl/freshness/<hex vault_id>/lock`. The path is
//! keyed by vault_id, so every pathname or copy of one lineage's store meets the same lock.
//! Contention is a typed refusal, never a wait. The lock file is created 0600 if absent and is
//! NEVER unlinked: dropping the lock only unlocks and closes it. After the flock the holder checks
//! that the inode it locked is still the one at the path (fstat(fd) against the path's own stat,
//! which does not follow a symlink) and retries a bounded number of times otherwise. The re-check
//! NARROWS the window in which an unlinked-and-recreated lock file gives two brokers two different
//! inodes; it does not close it (a swap after the re-check still can, S7 read PR19), so commit()
//! re-checks again before anything else ([`LineageLock::recheck`]) and the head re-verification
//! refuses the loser.
//!
//! S7b X3 (F-03, F-21): the lock path is opened with O_NOFOLLOW and O_NONBLOCK and must be a
//! REGULAR file: a symlink, FIFO or other file type there is refused, never followed or locked.
//! S7b X6 (F-08): when the open CREATES the lock file, its directory is flushed.
//! S7c DF-9: a HARD LINK at the lock path is a regular file and is accepted -- FN1's lock is an
//! inode at a path, not a name. A link to another file can only make brokers contend or lock that
//! file's inode (a refusal or confusion, never two holders of one inode); no nlink check is made,
//! since it would also refuse a benign link.
//!
//! LOCK ORDER (a precondition for every caller): the caller's store lock `.qsc.lock`
//! (model::LockGuard) FIRST, then this lineage lock. The reverse order is FORBIDDEN. Both locks are
//! non-blocking, so a reversal would surface as a contention refusal rather than a deadlock.
//!
//! This is its own flock (lib.rs's `flock` extern), not model::LockGuard: that guard has no inode
//! re-check and keeps a re-entrant per-thread registry, neither of which FN1 wants here.

use super::paths;
use crate::fs_store::sync_dir_checked;
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
    /// S7b X3: the lock path is not a regular file (a FIFO, device, socket or directory).
    NotRegular,
    /// S7b X6: the lock file was created but its directory could not be flushed.
    DirFlush,
    /// S7b X11: at commit, the held inode is no longer the one at the path.
    InodeChanged,
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
            let file = open_lock_file(checkpoint_dir, &path)?;
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
            if still_at_path(&file, &path)? {
                return Ok(Self { file, path });
            }
            // A stale inode: closing `file` here releases it; reopen the path.
        }
        Err(LockError::InodeRetriesExhausted)
    }

    pub(crate) fn path(&self) -> &Path {
        &self.path
    }

    /// S7b X11 (F-09): the held inode is still the one at the path. Narrows the window again at
    /// commit; it does not close it.
    pub(crate) fn recheck(&self) -> Result<(), LockError> {
        if still_at_path(&self.file, &self.path)? {
            Ok(())
        } else {
            Err(LockError::InodeChanged)
        }
    }
}

/// fstat(fd) against the path's own stat (not following a symlink): the same (dev, ino)?
fn still_at_path(file: &File, path: &Path) -> Result<bool, LockError> {
    let held = file.metadata().map_err(|e| LockError::Stat(e.kind()))?;
    match fs::symlink_metadata(path) {
        Ok(md) => Ok((md.dev(), md.ino()) == (held.dev(), held.ino())),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(e) => Err(LockError::Stat(e.kind())),
    }
}

/// Open the lock file, creating it 0600 if absent: never through a final symlink, never blocking,
/// and only a REGULAR file is accepted. A creation is flushed into the checkpoint directory.
fn open_lock_file(checkpoint_dir: &Path, path: &Path) -> Result<File, LockError> {
    let open = |create: bool| {
        let mut options = OpenOptions::new();
        options
            .read(true)
            .write(true)
            .custom_flags(super::NO_FOLLOW_NO_BLOCK);
        if create {
            options.create_new(true).mode(0o600);
        }
        options.open(path)
    };
    let file = match open(false) {
        Ok(file) => file,
        Err(e) if e.kind() == io::ErrorKind::NotFound => match open(true) {
            Ok(file) => {
                sync_dir_checked(checkpoint_dir).map_err(|_| LockError::DirFlush)?;
                file
            }
            // Another broker created it between the two opens: open the one that is there.
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
                open(false).map_err(|e| LockError::Open(e.kind()))?
            }
            Err(e) => return Err(LockError::Open(e.kind())),
        },
        Err(e) => return Err(LockError::Open(e.kind())),
    };
    let md = file.metadata().map_err(|e| LockError::Stat(e.kind()))?;
    if !md.file_type().is_file() {
        return Err(LockError::NotRegular);
    }
    Ok(file)
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

    // ---- S7b (SEALED_EXPECTATION_S7b.md sec 1): the S7 read's probes, their outcomes INVERTED.

    #[test]
    fn x3_pr03_symlink_at_the_lock_path_is_refused_never_followed() {
        // PR03: a symlink to a file flocked elsewhere (was Contended, a misleading code).
        let td = private_dir();
        let victim = td.path().join("victim");
        let vf = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .mode(0o600)
            .open(&victim)
            .expect("victim");
        // SAFETY: flock on a descriptor this test owns (LOCK_EX | LOCK_NB).
        assert_eq!(
            unsafe { crate::flock(vf.as_raw_fd(), LOCK_EX | LOCK_NB) },
            0
        );
        let ck = td.path().join("ck");
        fs::create_dir(&ck).expect("ck");
        std::os::unix::fs::symlink(&victim, paths::lock_path(&ck)).expect("symlink");
        let r = LineageLock::acquire(&ck).map(|_| ());
        assert!(matches!(r, Err(LockError::Open(_))), "{r:?}");
        assert_eq!(r.unwrap_err().code(), "lock_open_failed");
        drop(vf);
        // PR01: a symlink to an unlocked regular file: refused, the file never written.
        let r = LineageLock::acquire(&ck).map(|_| ());
        assert!(matches!(r, Err(LockError::Open(_))), "{r:?}");
        assert_eq!(fs::metadata(&victim).expect("victim").len(), 0);
        // PR02: a DANGLING symlink: refused, and nothing is created at its target.
        let ck2 = td.path().join("ck2");
        fs::create_dir(&ck2).expect("ck2");
        let target = td.path().join("created_through_the_symlink");
        std::os::unix::fs::symlink(&target, paths::lock_path(&ck2)).expect("symlink");
        let r = LineageLock::acquire(&ck2).map(|_| ());
        assert!(matches!(r, Err(LockError::Open(_))), "{r:?}");
        assert!(
            !target.exists(),
            "O_NOFOLLOW: nothing created through the link"
        );
    }

    #[test]
    fn x3_pr05_fifo_at_the_lock_path_is_refused_not_regular() {
        let td = private_dir();
        let made = std::process::Command::new("mkfifo")
            .arg(paths::lock_path(td.path()))
            .status()
            .expect("mkfifo");
        assert!(made.success());
        let r = LineageLock::acquire(td.path()).map(|_| ());
        assert_eq!(r, Err(LockError::NotRegular));
        assert_eq!(r.unwrap_err().code(), "lock_open_failed");
    }

    #[test]
    fn x6_lock_file_creation_flushes_its_parent() {
        use crate::fs_store::{arm_durable_flush_fault, DurableFlushPoint};
        let td = private_dir();
        arm_durable_flush_fault(DurableFlushPoint::Dir, 0);
        crate::fs_store::take_dir_flushes();
        let r = LineageLock::acquire(td.path()).map(|_| ());
        assert_eq!(
            r,
            Err(LockError::DirFlush),
            "the creation's flush is checked"
        );
        // S7c DF-3: the flush went to the checkpoint DIRECTORY, not the lock file.
        assert_eq!(
            crate::fs_store::take_dir_flushes(),
            [td.path().to_path_buf()]
        );
        assert_eq!(r.unwrap_err().code(), "storage_durability_failed");
        assert!(
            paths::lock_path(td.path()).is_file(),
            "the lock file was created"
        );
        // An existing lock file is not created again, so nothing is flushed.
        arm_durable_flush_fault(DurableFlushPoint::Dir, 0);
        let lock = LineageLock::acquire(td.path()).expect("an existing lock file");
        drop(lock);
        assert!(
            sync_dir_checked(td.path()).is_err(),
            "the armed fault was not spent by the acquire"
        );
    }

    #[test]
    fn x11_recheck_sees_a_replaced_lock_file() {
        let td = private_dir();
        let lock = LineageLock::acquire(td.path()).expect("acquire");
        assert_eq!(lock.recheck(), Ok(()));
        let path = paths::lock_path(td.path());
        fs::remove_file(&path).expect("unlink");
        assert_eq!(lock.recheck(), Err(LockError::InodeChanged), "unlinked");
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&path)
            .expect("recreate");
        assert_eq!(lock.recheck(), Err(LockError::InodeChanged), "recreated");
        assert_eq!(
            LockError::InodeChanged.code(),
            "freshness_lineage_lock_unstable"
        );
    }
}
