//! D31 / D32 locations (C07 T6.4; DOC-CAN-003 C07-01, C07-01-V1, C07-05):
//!
//! ```text
//! <STATE>/qsl/freshness/<64 lowercase hex of vault_id>/head        the QSLFRESH checkpoint (D31)
//! <STATE>/qsl/freshness/<64 lowercase hex of vault_id>/head.tmp.<pid>.<n>   its temporary
//! <STATE>/qsl/freshness/<64 lowercase hex of vault_id>/lock        the lineage lock PATH (D32; the lock is S5's)
//! ```
//!
//! STATE = `$QSC_SUCC01_STATE_DIR` (the C07-01-V1 TEST override; honoured in every build, like its
//! A20 sibling, so CLI tests of the real binary can redirect it), else `$XDG_STATE_HOME` if set
//! and absolute, else `$HOME/.local/state`. Every root that is used must be absolute: a
//! cwd-relative root would give one lineage two checkpoint and lock locations (D32 / FN1).
//!
//! S7b X7 (F-10, OWED D40): the head temp is `head.tmp.<pid>.<n>`, n a per-process counter (txn's),
//! so no two writes of one process share a name; recovery's rule a removes a dead pid's head temp
//! of either shape (the `head.tmp.<pid>` of S5 included).

use crate::fs_store::{perms_group_or_world_writable, sync_dir_checked};
use std::env;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::io;
use std::os::unix::fs::DirBuilderExt;
use std::path::{Path, PathBuf};

pub(crate) const STATE_DIR_OVERRIDE_ENV: &str = "QSC_SUCC01_STATE_DIR";
const XDG_STATE_HOME_ENV: &str = "XDG_STATE_HOME";
const HOME_ENV: &str = "HOME";

const QSL_DIR: &str = "qsl";
const FRESHNESS_DIR: &str = "freshness";
const HEAD_FILE: &str = "head";
const LOCK_FILE: &str = "lock";
const HEAD_TEMP_PREFIX: &str = "head.tmp.";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StateRootError {
    /// Neither override nor XDG_STATE_HOME applies and HOME is unset or blank.
    NoHome,
    HomeNotAbsolute,
    OverrideNotAbsolute,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CheckpointDirError {
    RootNotAbsolute,
    Symlink,
    NotADirectory,
    GroupOrWorldWritable,
    Io(io::ErrorKind),
    /// S7b X6: a component this call created could not be flushed into its parent.
    DirFlush,
}

pub(crate) fn state_root() -> Result<PathBuf, StateRootError> {
    state_root_from(|name| env::var_os(name))
}

/// The resolver with the environment passed in, so tests never touch process variables.
/// A blank (empty or all-whitespace) override or HOME counts as unset (the QSC_CONFIG_DIR
/// precedent, fs_store::config_dir).
pub(crate) fn state_root_from(
    lookup: impl Fn(&str) -> Option<OsString>,
) -> Result<PathBuf, StateRootError> {
    if let Some(v) = lookup(STATE_DIR_OVERRIDE_ENV).filter(|v| !is_blank(v)) {
        let root = PathBuf::from(v);
        return if root.is_absolute() {
            Ok(root)
        } else {
            Err(StateRootError::OverrideNotAbsolute)
        };
    }
    if let Some(v) = lookup(XDG_STATE_HOME_ENV) {
        // "if set and absolute" (D31): a relative value is ignored, not refused.
        if !v.is_empty() && Path::new(&v).is_absolute() {
            return Ok(PathBuf::from(v));
        }
    }
    let home = lookup(HOME_ENV)
        .filter(|v| !is_blank(v))
        .ok_or(StateRootError::NoHome)?;
    let home = PathBuf::from(home);
    if !home.is_absolute() {
        return Err(StateRootError::HomeNotAbsolute);
    }
    Ok(home.join(".local").join("state"))
}

fn is_blank(v: &OsStr) -> bool {
    v.to_str().is_some_and(|s| s.trim().is_empty())
}

pub(crate) fn checkpoint_dir(root: &Path, vault_id: &[u8; 32]) -> PathBuf {
    root.join(QSL_DIR)
        .join(FRESHNESS_DIR)
        .join(crate::hex_encode(vault_id))
}

pub(crate) fn head_path(dir: &Path) -> PathBuf {
    dir.join(HEAD_FILE)
}

pub(crate) fn head_temp_path(dir: &Path, pid: u32, seq: u64) -> PathBuf {
    dir.join(format!("{HEAD_TEMP_PREFIX}{pid}.{seq}"))
}

/// The pid of a head temp name of either shape, `head.tmp.<pid>` (S5) or `head.tmp.<pid>.<n>`,
/// in CANONICAL decimal only (no sign, no leading zero, so never pid 0; within pid_t; n is "0" or
/// has no leading zero). Anything else is None and is never removed.
pub(crate) fn head_temp_pid(name: &OsStr) -> Option<i32> {
    let rest = name.to_str()?.strip_prefix(HEAD_TEMP_PREFIX)?;
    let (pid, seq) = match rest.split_once('.') {
        Some((pid, seq)) => (pid, Some(seq)),
        None => (rest, None),
    };
    let canonical = |d: &str| !d.is_empty() && d.bytes().all(|b| b.is_ascii_digit());
    if !canonical(pid) || pid.starts_with('0') {
        return None;
    }
    if let Some(seq) = seq {
        if !canonical(seq) || (seq.len() > 1 && seq.starts_with('0')) {
            return None;
        }
    }
    pid.parse().ok()
}

pub(crate) fn lock_path(dir: &Path) -> PathBuf {
    dir.join(LOCK_FILE)
}

/// Create the checkpoint directory (mode 0700 for every component this call creates) and
/// REFUSE a root or component that is a symlink, not a directory, or group/world-writable.
/// An existing private directory is accepted as it is (no chmod repair). S7b X6 (F-08): each
/// component this call CREATES is flushed into its parent (a hardening; no power-loss claim).
pub(crate) fn ensure_checkpoint_dir(
    root: &Path,
    vault_id: &[u8; 32],
) -> Result<PathBuf, CheckpointDirError> {
    if !root.is_absolute() {
        return Err(CheckpointDirError::RootNotAbsolute);
    }
    if fs::symlink_metadata(root).is_err() {
        fs::DirBuilder::new()
            .recursive(true)
            .mode(0o700)
            .create(root)
            .map_err(|e| CheckpointDirError::Io(e.kind()))?;
    }
    check_private_dir(root)?;
    let hex = crate::hex_encode(vault_id);
    let mut cur = root.to_path_buf();
    for comp in [QSL_DIR, FRESHNESS_DIR, hex.as_str()] {
        let parent = cur.clone();
        cur.push(comp);
        match fs::DirBuilder::new().mode(0o700).create(&cur) {
            Ok(()) => sync_dir_checked(&parent).map_err(|_| CheckpointDirError::DirFlush)?,
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {}
            Err(e) => return Err(CheckpointDirError::Io(e.kind())),
        }
        check_private_dir(&cur)?;
    }
    Ok(cur)
}

fn check_private_dir(dir: &Path) -> Result<(), CheckpointDirError> {
    let md = fs::symlink_metadata(dir).map_err(|e| CheckpointDirError::Io(e.kind()))?;
    if md.file_type().is_symlink() {
        return Err(CheckpointDirError::Symlink);
    }
    if !md.is_dir() {
        return Err(CheckpointDirError::NotADirectory);
    }
    if perms_group_or_world_writable(&md) {
        return Err(CheckpointDirError::GroupOrWorldWritable);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::test_support::syn_vault_id;
    use super::*;
    use std::collections::HashMap;
    use std::os::unix::fs::PermissionsExt;

    fn env_of(pairs: &[(&str, &str)]) -> impl Fn(&str) -> Option<OsString> {
        let map: HashMap<String, OsString> = pairs
            .iter()
            .map(|(k, v)| (k.to_string(), OsString::from(v)))
            .collect();
        move |name| map.get(name).cloned()
    }

    fn mode_of(p: &Path) -> u32 {
        fs::symlink_metadata(p).expect("stat").permissions().mode() & 0o777
    }

    /// Fixture directories at an EXPLICIT 0700: a plain create_dir takes the test process's
    /// umask (0002 on the seat's box gives 0775), which ensure_checkpoint_dir would refuse first.
    fn private_dir(p: &Path) {
        fs::DirBuilder::new()
            .recursive(true)
            .mode(0o700)
            .create(p)
            .expect("mkdir 0700");
    }

    const VID_HEX: &str = "030a11181f262d343b424950575e656c737a81888f969da4abb2b9c0c7ced5dc";

    #[test]
    fn path1_absolute_xdg_state_home_is_the_root() {
        let got = state_root_from(env_of(&[("XDG_STATE_HOME", "/x/state"), ("HOME", "/h")]));
        assert_eq!(got, Ok(PathBuf::from("/x/state")));
    }

    #[test]
    fn path2_relative_xdg_state_home_is_ignored() {
        let got = state_root_from(env_of(&[("XDG_STATE_HOME", "rel/state"), ("HOME", "/h")]));
        assert_eq!(got, Ok(PathBuf::from("/h/.local/state")));
    }

    #[test]
    fn path3_unset_or_empty_xdg_uses_home() {
        let unset = state_root_from(env_of(&[("HOME", "/h")]));
        assert_eq!(unset, Ok(PathBuf::from("/h/.local/state")));
        let empty = state_root_from(env_of(&[("XDG_STATE_HOME", ""), ("HOME", "/h")]));
        assert_eq!(empty, Ok(PathBuf::from("/h/.local/state")));
    }

    #[test]
    fn path3b_missing_or_relative_home_is_refused() {
        assert_eq!(state_root_from(env_of(&[])), Err(StateRootError::NoHome));
        assert_eq!(
            state_root_from(env_of(&[("HOME", "  ")])),
            Err(StateRootError::NoHome)
        );
        assert_eq!(
            state_root_from(env_of(&[("HOME", "h")])),
            Err(StateRootError::HomeNotAbsolute)
        );
    }

    #[test]
    fn path4_override_replaces_the_root() {
        let got = state_root_from(env_of(&[
            ("QSC_SUCC01_STATE_DIR", "/t/override"),
            ("XDG_STATE_HOME", "/x/state"),
            ("HOME", "/h"),
        ]));
        assert_eq!(got, Ok(PathBuf::from("/t/override")));
        assert_eq!(
            checkpoint_dir(&got.expect("root"), &syn_vault_id()),
            PathBuf::from(format!("/t/override/qsl/freshness/{VID_HEX}"))
        );
    }

    #[test]
    fn path4b_relative_override_refused_blank_override_ignored() {
        let relative = state_root_from(env_of(&[
            ("QSC_SUCC01_STATE_DIR", "t/override"),
            ("XDG_STATE_HOME", "/x/state"),
        ]));
        assert_eq!(relative, Err(StateRootError::OverrideNotAbsolute));
        let blank = state_root_from(env_of(&[
            ("QSC_SUCC01_STATE_DIR", " "),
            ("XDG_STATE_HOME", "/x/state"),
        ]));
        assert_eq!(blank, Ok(PathBuf::from("/x/state")));
    }

    #[test]
    fn path5_checkpoint_dir_is_64_lowercase_hex() {
        let dir = checkpoint_dir(Path::new("/s"), &syn_vault_id());
        let leaf = dir.file_name().and_then(|n| n.to_str()).expect("leaf");
        assert_eq!(leaf, VID_HEX);
        assert_eq!(leaf.len(), 64);
        assert!(leaf
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)));
        assert_eq!(dir, PathBuf::from(format!("/s/qsl/freshness/{VID_HEX}")));
    }

    #[test]
    fn path6_head_temp_and_lock_names() {
        let dir = checkpoint_dir(Path::new("/s"), &syn_vault_id());
        assert_eq!(head_path(&dir), dir.join("head"));
        assert_eq!(head_temp_path(&dir, 4242, 7), dir.join("head.tmp.4242.7"));
        assert_eq!(lock_path(&dir), dir.join("lock"));
    }

    #[test]
    fn dir1_ensure_creates_every_component_0700() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let root = tmp.path().join("state");
        let dir = ensure_checkpoint_dir(&root, &syn_vault_id()).expect("ensure");
        assert_eq!(dir, checkpoint_dir(&root, &syn_vault_id()));
        for p in [
            root.clone(),
            root.join("qsl"),
            root.join("qsl/freshness"),
            dir.clone(),
        ] {
            assert!(p.is_dir(), "{}", p.display());
            assert_eq!(mode_of(&p), 0o700, "{}", p.display());
        }
    }

    #[test]
    fn dir1b_ensure_refuses_group_or_world_writable_dir() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let root = tmp.path().join("state");
        let dir = checkpoint_dir(&root, &syn_vault_id());
        private_dir(&dir);
        fs::set_permissions(&dir, fs::Permissions::from_mode(0o777)).expect("chmod");
        assert_eq!(
            ensure_checkpoint_dir(&root, &syn_vault_id()),
            Err(CheckpointDirError::GroupOrWorldWritable)
        );
        // Non-vacuity: the 0777 leaf is what was refused -- at 0700 the same tree is accepted.
        fs::set_permissions(&dir, fs::Permissions::from_mode(0o700)).expect("chmod");
        assert_eq!(ensure_checkpoint_dir(&root, &syn_vault_id()), Ok(dir));
        let exposed_root = tmp.path().join("exposed");
        fs::create_dir(&exposed_root).expect("mkdir");
        fs::set_permissions(&exposed_root, fs::Permissions::from_mode(0o777)).expect("chmod");
        assert_eq!(
            ensure_checkpoint_dir(&exposed_root, &syn_vault_id()),
            Err(CheckpointDirError::GroupOrWorldWritable)
        );
        assert!(
            !exposed_root.join("qsl").exists(),
            "nothing created under a refused root"
        );
    }

    #[test]
    fn dir1c_ensure_refuses_symlinked_component() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let root = tmp.path().join("state");
        let elsewhere = tmp.path().join("elsewhere");
        private_dir(&root);
        private_dir(&elsewhere);
        std::os::unix::fs::symlink(&elsewhere, root.join("qsl")).expect("symlink");
        assert_eq!(
            ensure_checkpoint_dir(&root, &syn_vault_id()),
            Err(CheckpointDirError::Symlink)
        );
        assert!(
            !elsewhere.join("freshness").exists(),
            "nothing created through the link"
        );
    }

    #[test]
    fn dir1d_ensure_accepts_existing_private_dir() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let root = tmp.path().join("state");
        let first = ensure_checkpoint_dir(&root, &syn_vault_id()).expect("first");
        let second = ensure_checkpoint_dir(&root, &syn_vault_id()).expect("second");
        assert_eq!(first, second);
    }

    // ---- S7b (SEALED_EXPECTATION_S7b.md sec 1)

    #[test]
    fn x6_created_components_are_flushed_into_their_parents() {
        use crate::fs_store::{arm_durable_flush_fault, DurableFlushPoint};
        let fresh = || {
            let td = tempfile::tempdir().expect("tempdir");
            fs::set_permissions(td.path(), fs::Permissions::from_mode(0o700)).expect("chmod");
            td
        };
        for skip in 0..3 {
            let td = fresh();
            arm_durable_flush_fault(DurableFlushPoint::Dir, skip);
            let r = ensure_checkpoint_dir(td.path(), &syn_vault_id());
            assert_eq!(
                r,
                Err(CheckpointDirError::DirFlush),
                "flush {skip} is checked"
            );
            assert_eq!(r.unwrap_err().code(), "storage_durability_failed");
        }
        // Exactly three: the fourth flush is never reached, so the fault is still armed.
        let td = fresh();
        arm_durable_flush_fault(DurableFlushPoint::Dir, 3);
        crate::fs_store::take_dir_flushes();
        let dir = ensure_checkpoint_dir(td.path(), &syn_vault_id()).expect("three flushes pass");
        // S7c DF-3: each flush went to the created component's PARENT, in order.
        let qsl = td.path().join(QSL_DIR);
        assert_eq!(
            crate::fs_store::take_dir_flushes(),
            [
                td.path().to_path_buf(),
                qsl.clone(),
                qsl.join(FRESHNESS_DIR)
            ]
        );
        arm_durable_flush_fault(DurableFlushPoint::Dir, 0);
        assert_eq!(ensure_checkpoint_dir(td.path(), &syn_vault_id()), Ok(dir));
        assert!(
            sync_dir_checked(td.path()).is_err(),
            "an existing tree creates nothing and flushes nothing"
        );
    }

    #[test]
    fn x7_head_temp_names_are_unique_and_both_shapes_parse() {
        let dir = checkpoint_dir(Path::new("/s"), &syn_vault_id());
        assert_ne!(head_temp_path(&dir, 4242, 0), head_temp_path(&dir, 4242, 1));
        let pid = |s: &str| head_temp_pid(OsStr::new(s));
        assert_eq!(pid("head.tmp.4242"), Some(4242), "S5's shape");
        assert_eq!(pid("head.tmp.4242.0"), Some(4242));
        assert_eq!(pid("head.tmp.4242.17"), Some(4242));
        for bad in [
            "head.tmp.04242",
            "head.tmp.0",
            "head.tmp.0.1",
            "head.tmp.4242.",
            "head.tmp.4242.01",
            "head.tmp.4242.1.2",
            "head.tmp.4242.x",
            "head.tmp.x",
            "head.tmp.",
            "head.tmp.-1",
            "head.tmp.99999999999",
            "head",
            "head.tmp4242",
        ] {
            assert_eq!(pid(bad), None, "{bad:?} is not a head temp");
        }
    }
}
