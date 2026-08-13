// NA-0696 (D630 D6, D-1336; ENG-0111(a)): a non-Unix build fails AT COMPILE TIME where it
// previously built a silently succeeding no-op lock. The non-Unix stubs and lock-claim
// cfg masks are deleted with this refusal.
#[cfg(not(unix))]
compile_error!(
    "qsc requires a Unix target: the store lock is flock-based and has no non-Unix \
     implementation; a silently succeeding no-op lock is not acceptable (ENG-0111)."
);

use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy)]
// NA-0649 (D585, operator-approved scope amendment): visibility only — the widened
// identity accessors and `identity_ensure` carry this type in pub signatures, and an
// external (GUI) crate must be able to name and match it. No variant, body, or
// vocabulary change.
pub enum ErrorCode {
    MissingHome,
    InvalidPolicyProfile,
    UnsafePathSymlink,
    UnsafeParentPerms,
    LockOpenFailed,
    LockContended,
    LockFailed,
    // NA-0696 (D630 Q2, D-1336): a same-process EX request under a held SH lock — refused
    // fail-closed at the guard, the OS lock untouched. NOT contention (there is no other
    // holder) and NOT on the user-facing copy list: F4 proved no production flow reaches
    // it; the distinct name is what makes an appearance diagnosable as a defect.
    LockUpgradeRefused,
    IoWriteFailed,
    IoReadFailed,
    ParseFailed,
    IdentitySecretUnavailable,
    // NA-0616 (ENG-0001): refused to auto-mint a second, divergent self-identity
    // (a self-identity under a different label already exists in the config dir).
    IdentitySelfAmbiguous,
}

impl ErrorCode {
    pub fn as_str(self) -> &'static str {
        match self {
            ErrorCode::MissingHome => "missing_home",
            ErrorCode::InvalidPolicyProfile => "invalid_policy_profile",
            ErrorCode::UnsafePathSymlink => "unsafe_path_symlink",
            ErrorCode::UnsafeParentPerms => "unsafe_parent_perms",
            ErrorCode::LockOpenFailed => "lock_open_failed",
            ErrorCode::LockContended => "lock_contended",
            ErrorCode::LockFailed => "lock_failed",
            ErrorCode::LockUpgradeRefused => "lock_upgrade_refused",
            ErrorCode::IoWriteFailed => "io_write_failed",
            ErrorCode::IoReadFailed => "io_read_failed",
            ErrorCode::ParseFailed => "parse_failed",
            ErrorCode::IdentitySecretUnavailable => "identity_secret_unavailable",
            ErrorCode::IdentitySelfAmbiguous => "identity_self_ambiguous",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum ConfigSource {
    EnvOverride,
    XdgConfigHome,
    DefaultHome,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum LockMode {
    Shared,
    Exclusive,
}

const LOCK_SH: i32 = 1;
const LOCK_EX: i32 = 2;
const LOCK_NB: i32 = 4;
const LOCK_UN: i32 = 8;

struct Held {
    file: File,
    mode: LockMode,
    depth: u32,
}

thread_local! {
    static LOCK_REGISTRY: RefCell<HashMap<PathBuf, Held>> = RefCell::new(HashMap::new());
}

// THREE-PHASE BORROW DISCIPLINE (D-1336, binding): no registry borrow is ever live across a
// syscall or any point where user code (including a Guard drop) can run. Phase 1:
// nested-check + increment under a short `borrow_mut` that ends before return — held EX +
// requested {EX,SH} → depth++ grant (held mode stays Exclusive; a grant may exceed the
// requested strength, never fall below); held SH + requested SH → depth++ grant; held SH +
// requested EX → `Err(LockUpgradeRefused)` with the OS lock UNTOUCHED — a forwarded same-fd
// conversion measurably LOSES the held lock (NA-0696 STOP 001 F3, replicated on a second
// kernel), so the registry never issues a conversion flock. Phase 2 (depth 0 only, NO
// borrow live): open the lock file (create/truncate(false)/read/write — the exact current
// flags) and `flock(op | LOCK_NB)`. Phase 3: insert `Held{file, mode, depth: 1}` under a
// fresh short borrow. Drop: one short `borrow_mut` — decrement; at 0 remove the entry,
// `flock(LOCK_UN)`, close. Panic-safety follows by construction (F2). Cross-thread
// posture: `thread_local` means a second thread sees its OWN empty registry, performs a
// REAL flock, and is denied EWOULDBLOCK by the first thread's held lock — fail-CLOSED,
// today's exact self-denial, never a false nested grant (zero `thread::spawn` in src,
// measured).
pub(crate) struct LockGuard {
    key: PathBuf,
}

impl LockGuard {
    pub(crate) fn acquire(
        dir: &Path,
        lock_path: &Path,
        requested: LockMode,
    ) -> Result<LockGuard, ErrorCode> {
        // Phase 1: the borrow ends when the closure returns — before the emit, before any
        // syscall, before either return path below.
        let nested_depth: Option<u32> = LOCK_REGISTRY.with(|registry| {
            let mut reg = registry.borrow_mut();
            match reg.get_mut(dir) {
                Some(held) => match (held.mode, requested) {
                    (LockMode::Exclusive, _) | (LockMode::Shared, LockMode::Shared) => {
                        held.depth += 1;
                        Ok(Some(held.depth))
                    }
                    (LockMode::Shared, LockMode::Exclusive) => Err(ErrorCode::LockUpgradeRefused),
                },
                None => Ok(None),
            }
        })?;
        if let Some(_depth) = nested_depth {
            // Q4 (D-1336): debug-only visibility on NESTED acquires only — a log line,
            // never an assert (nesting is legal by design) and never `emit_marker` (the
            // claim and diagnostic vocabularies stay separate).
            #[cfg(debug_assertions)]
            eprintln!(
                "qsc_lock_nested_acquire path={} depth={}",
                dir.display(),
                _depth
            );
            return Ok(LockGuard {
                key: dir.to_path_buf(),
            });
        }
        // Phase 2 (depth 0 only, no borrow live): the exact pre-registry open flags.
        let file = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(lock_path)
            .map_err(|_| ErrorCode::LockOpenFailed)?;
        let op = match requested {
            LockMode::Shared => LOCK_SH,
            LockMode::Exclusive => LOCK_EX,
        };
        let rc = {
            use std::os::unix::io::AsRawFd;
            unsafe { crate::flock(file.as_raw_fd(), op | LOCK_NB) }
        };
        if rc != 0 {
            let err = std::io::Error::last_os_error();
            if err.kind() == std::io::ErrorKind::WouldBlock {
                return Err(ErrorCode::LockContended);
            }
            return Err(ErrorCode::LockFailed);
        }
        // Phase 3: insert under a fresh short borrow.
        LOCK_REGISTRY.with(|registry| {
            registry.borrow_mut().insert(
                dir.to_path_buf(),
                Held {
                    file,
                    mode: requested,
                    depth: 1,
                },
            );
        });
        Ok(LockGuard {
            key: dir.to_path_buf(),
        })
    }
}

/// Q4 (D-1336): the always-compiled depth reader — 0 when this thread holds no lock for
/// `dir`. `pub(crate)`, both build profiles; the debug emit above is the log half.
// Inspection surface, not a production dependency: consumed by the registry units and by
// debugging; no live path is required to read it (dead_code allowance retained, the
// secret_set_with_passphrase precedent).
#[allow(dead_code)]
pub(crate) fn lock_depth(dir: &Path) -> u32 {
    LOCK_REGISTRY.with(|registry| registry.borrow().get(dir).map_or(0, |held| held.depth))
}

impl Drop for LockGuard {
    fn drop(&mut self) {
        // One short borrow_mut: decrement; at 0 remove the entry. The OS release runs
        // AFTER the borrow ends — no registry borrow is live across the flock syscall
        // (the three-phase discipline's Drop half).
        let released: Option<Held> = LOCK_REGISTRY.with(|registry| {
            let mut reg = registry.borrow_mut();
            match reg.get_mut(&self.key) {
                Some(held) if held.depth > 1 => {
                    held.depth -= 1;
                    None
                }
                Some(_) => reg.remove(&self.key),
                None => None,
            }
        });
        if let Some(held) = released {
            use std::os::unix::io::AsRawFd;
            let _ = unsafe { crate::flock(held.file.as_raw_fd(), LOCK_UN) };
        }
    }
}

// NA-0696 (D630 §4c, D-1336): the registry units — `LockGuard`/`LOCK_REGISTRY` are
// private to this module, so a same-file `#[cfg(test)] mod` is the only place that can
// drive the guard directly (the na0692 resolver-tests precedent). No env is touched:
// every test owns a unique directory and the raw-flock probes are separate open file
// descriptions in this same process, which is exactly the cross-OFD semantics `flock`
// contends on. These do NOT satisfy goal-lint (path-based); the gate instrument is
// `tests/na0696_vault_honesty.rs`.
#[cfg(test)]
mod na0696_lock_registry_tests {
    use super::*;
    use std::fs;
    use std::os::unix::io::AsRawFd;

    // Raw flock() denial errno, per platform: EWOULDBLOCK == EAGAIN is 11 on Linux and
    // 35 on Darwin (where 11 is EDEADLK). Production classifies via
    // ErrorKind::WouldBlock and stays portable; these tests probe the RAW value on a
    // separate open file description, so they must name the platform's own constant —
    // pinned here the way LOCK_* are, no libc dependency. An unlisted target_os fails
    // to compile at the use sites: fail-closed by construction.
    #[cfg(target_os = "linux")]
    const EWOULDBLOCK_RAW: i32 = 11;
    #[cfg(target_os = "macos")]
    const EWOULDBLOCK_RAW: i32 = 35;

    fn unit_dir(tag: &str) -> PathBuf {
        let root = if let Ok(v) = std::env::var("QSC_TEST_ROOT") {
            PathBuf::from(v)
        } else if let Ok(v) = std::env::var("CARGO_TARGET_DIR") {
            PathBuf::from(v)
        } else {
            PathBuf::from("target")
        };
        let dir = root
            .join("qsc-test-tmp")
            .join("na0696-lock-registry")
            .join(format!("{}_{}", tag, std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        // The Slice-B W1/X1 lesson: fixture-owned dirs are chmod 0700, or
        // `enforce_safe_parents` (correctly) refuses them under a group-writable root.
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&dir, fs::Permissions::from_mode(0o700)).unwrap();
        dir
    }

    fn acquire(dir: &Path, mode: LockMode) -> Result<LockGuard, ErrorCode> {
        LockGuard::acquire(dir, &dir.join(".qsc.lock"), mode)
    }

    /// Raw probe on a SEPARATE open file description: (rc, errno, the open file —
    /// keep it alive while the probe's lock matters).
    fn raw_flock(dir: &Path, op: i32) -> (i32, i32, fs::File) {
        let f = fs::OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(dir.join(".qsc.lock"))
            .unwrap();
        let rc = unsafe { crate::flock(f.as_raw_fd(), op | LOCK_NB) };
        let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(0);
        (rc, errno, f)
    }

    fn raw_unlock(f: &fs::File) {
        let _ = unsafe { crate::flock(f.as_raw_fd(), LOCK_UN) };
    }

    /// EX-under-EX and SH-under-EX are grants at held-EX strength; SH-under-SH is a
    /// grant; depth counts every live guard; the OS lock excludes a second OFD for the
    /// whole span and releases exactly at depth 0.
    #[test]
    fn nested_grants_and_depth() {
        let dir = unit_dir("nested_grants");
        assert_eq!(lock_depth(&dir), 0);
        let g1 = acquire(&dir, LockMode::Exclusive).expect("outer EX");
        assert_eq!(lock_depth(&dir), 1);
        let g2 = acquire(&dir, LockMode::Exclusive).expect("EX under EX");
        assert_eq!(lock_depth(&dir), 2);
        let g3 = acquire(&dir, LockMode::Shared).expect("SH under EX");
        assert_eq!(lock_depth(&dir), 3);
        // Held mode stays EXCLUSIVE: even a shared probe from another OFD is denied.
        let (rc, errno, _probe) = raw_flock(&dir, LOCK_SH);
        assert_eq!(
            (rc, errno),
            (-1, EWOULDBLOCK_RAW),
            "cross-OFD exclusion preserved"
        );
        drop(g3);
        drop(g2);
        assert_eq!(lock_depth(&dir), 1);
        let (rc, errno, _probe2) = raw_flock(&dir, LOCK_EX);
        assert_eq!((rc, errno), (-1, EWOULDBLOCK_RAW), "still held at depth 1");
        drop(g1);
        assert_eq!(lock_depth(&dir), 0);
        let (rc, _errno, probe3) = raw_flock(&dir, LOCK_EX);
        assert_eq!(rc, 0, "released exactly at depth 0");
        raw_unlock(&probe3);
        // SH-under-SH is a grant too.
        let s1 = acquire(&dir, LockMode::Shared).expect("outer SH");
        let s2 = acquire(&dir, LockMode::Shared).expect("SH under SH");
        assert_eq!(lock_depth(&dir), 2);
        drop(s2);
        drop(s1);
        assert_eq!(lock_depth(&dir), 0);
    }

    /// F1: guards are fungible decrements, not a LIFO discipline — dropping the OUTER
    /// guard first changes nothing observable; the OS lock holds until the LAST guard.
    #[test]
    fn drop_order_commutative() {
        let dir = unit_dir("drop_order");
        let g1 = acquire(&dir, LockMode::Exclusive).expect("g1");
        let g2 = acquire(&dir, LockMode::Exclusive).expect("g2");
        let g3 = acquire(&dir, LockMode::Exclusive).expect("g3");
        assert_eq!(lock_depth(&dir), 3);
        drop(g1); // the OUTER guard first
        let (rc, errno, _p1) = raw_flock(&dir, LOCK_EX);
        assert_eq!(
            (rc, errno),
            (-1, EWOULDBLOCK_RAW),
            "held after out-of-order outer drop"
        );
        drop(g2); // the middle guard next
        let (rc, errno, _p2) = raw_flock(&dir, LOCK_EX);
        assert_eq!((rc, errno), (-1, EWOULDBLOCK_RAW), "held at depth 1");
        drop(g3); // the last guard releases
        let (rc, _errno, p3) = raw_flock(&dir, LOCK_EX);
        assert_eq!(rc, 0, "released when the last guard dropped");
        raw_unlock(&p3);
    }

    /// F2's two arms via catch_unwind: unwind through a NESTED guard restores depth and
    /// keeps the OS lock; unwind through the LAST guard releases it. The registry stays
    /// usable afterwards (RefCell has no poisoning to trip).
    #[test]
    fn panic_unwind_restores_depth() {
        let dir = unit_dir("panic_unwind");
        // (a) panic while holding a NESTED guard.
        let outer = acquire(&dir, LockMode::Exclusive).expect("outer");
        let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _nested = acquire(&dir, LockMode::Exclusive).expect("nested");
            assert_eq!(lock_depth(&dir), 2);
            panic!("unwind through the nested guard");
        }));
        assert!(caught.is_err());
        assert_eq!(lock_depth(&dir), 1, "depth restored to the outer guard");
        let (rc, errno, _probe) = raw_flock(&dir, LOCK_EX);
        assert_eq!(
            (rc, errno),
            (-1, EWOULDBLOCK_RAW),
            "outer guard still holds the OS lock"
        );
        let renested = acquire(&dir, LockMode::Exclusive).expect("registry usable");
        assert_eq!(lock_depth(&dir), 2);
        drop(renested);
        drop(outer);
        // (b) panic while holding only the OUTER guard.
        let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _only = acquire(&dir, LockMode::Exclusive).expect("only guard");
            assert_eq!(lock_depth(&dir), 1);
            panic!("unwind through the last guard");
        }));
        assert!(caught.is_err());
        assert_eq!(lock_depth(&dir), 0, "depth zero after unwind");
        let (rc, _errno, probe) = raw_flock(&dir, LOCK_EX);
        assert_eq!(rc, 0, "OS lock released by the unwind");
        raw_unlock(&probe);
    }

    /// EX-under-SH refuses fail-closed AT THE GUARD with the OS lock untouched. The
    /// probe half observes the F3 hazard in-tree if the refusal is ever forwarded to a
    /// same-fd conversion (Control U): facts are collected first and asserted together,
    /// so a control run reports BOTH the wrong error AND the held SH lost.
    #[test]
    fn upgrade_refused_fail_closed() {
        let dir = unit_dir("upgrade_refused");
        let sh_guard = acquire(&dir, LockMode::Shared).expect("held SH");
        // A raw SH co-holder on a separate OFD (flock allows shared sharing) — the
        // ingredient that makes a forwarded same-fd conversion FAIL and drop the lock.
        let (rc, _errno, co_holder) = raw_flock(&dir, LOCK_SH);
        assert_eq!(rc, 0, "co-holder SH alongside");
        let refused = acquire(&dir, LockMode::Exclusive);
        let wrong_error = match &refused {
            Err(ErrorCode::LockUpgradeRefused) => None,
            Err(other) => Some(format!("Err({:?})", other)),
            Ok(_) => Some("Ok(granted)".to_string()),
        };
        let depth_after = lock_depth(&dir);
        // Release the co-holder, then probe EX from a fresh OFD: with our SH intact the
        // probe MUST be denied; if the conversion was forwarded and lost our SH, it
        // acquires — the loss observed.
        raw_unlock(&co_holder);
        drop(co_holder);
        let (probe_rc, probe_errno, probe) = raw_flock(&dir, LOCK_EX);
        if probe_rc == 0 {
            raw_unlock(&probe);
        }
        assert!(
            wrong_error.is_none()
                && depth_after == 1
                && probe_rc == -1
                && probe_errno == EWOULDBLOCK_RAW,
            "EX-under-SH must refuse fail-closed with the OS lock untouched: \
             wrong_error={:?} depth_after={} probe_rc={} probe_errno={} (probe acquiring \
             means the held SH was LOST — the F3 conversion hazard)",
            wrong_error,
            depth_after,
            probe_rc,
            probe_errno
        );
        assert_eq!(
            ErrorCode::LockUpgradeRefused.as_str(),
            "lock_upgrade_refused",
            "the distinct name (Q2)"
        );
        drop(sh_guard);
        assert_eq!(lock_depth(&dir), 0);
    }

    /// Q1: the shared helper's no-dir arm is a no-registry NON-entry — `Ok(None)`,
    /// nothing inserted, depth stays 0, and no directory is conjured.
    #[test]
    fn no_dir_shared_arm_takes_no_registry_entry() {
        let dir = unit_dir("no_dir_shared");
        let missing = dir.join("never_created");
        let got = crate::fs_store::lock_store_shared(&missing, ConfigSource::EnvOverride);
        assert!(
            matches!(got, Ok(None)),
            "no-dir shared arm must be Ok(None), got {:?}",
            got.map(|v| v.is_some())
        );
        assert_eq!(lock_depth(&missing), 0, "no registry entry");
        assert!(!missing.exists(), "the arm must not create the dir");
    }
}
