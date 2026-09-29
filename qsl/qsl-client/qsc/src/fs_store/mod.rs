use crate::model::{ConfigSource, ErrorCode, LockGuard, LockMode};
use crate::{ACK_MODE_KEY, LOCK_FILE_NAME, POLICY_KEY, STORE_META_NAME, STORE_META_TEMPLATE};
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process;
use std::time::{Duration, Instant};

pub(crate) fn config_dir() -> Result<(PathBuf, ConfigSource), ErrorCode> {
    if let Ok(v) = env::var("QSC_CONFIG_DIR") {
        if !v.trim().is_empty() {
            return Ok((PathBuf::from(v), ConfigSource::EnvOverride));
        }
    }
    if let Ok(v) = env::var("XDG_CONFIG_HOME") {
        if !v.trim().is_empty() {
            return Ok((PathBuf::from(v).join("qsc"), ConfigSource::XdgConfigHome));
        }
    }
    if let Ok(home) = env::var("HOME") {
        if !home.trim().is_empty() {
            return Ok((
                PathBuf::from(home).join(".config").join("qsc"),
                ConfigSource::DefaultHome,
            ));
        }
    }
    Err(ErrorCode::MissingHome)
}

pub(crate) fn normalize_profile(value: &str) -> Result<String, ErrorCode> {
    match value {
        "baseline" => Ok("baseline".to_string()),
        "strict" => Ok("strict".to_string()),
        _ => Err(ErrorCode::InvalidPolicyProfile),
    }
}

/// NA-0770 (D-1411): THE ACK-MODE CONFIG KEY IS TOMBSTONED, NOT DELETED.
///
/// `AckMode` no longer exists — delete-on-pull was retired — so there is no value to
/// normalise and no choice to express. The KEY is nevertheless retained here and at
/// [`read_ack_mode`] for one reason: a config file carrying `ack_mode` must be **detected
/// and announced**, never silently resolved to the surviving behaviour.
///
/// ⚠ **WHY A DELETION WOULD HAVE BEEN A DEFECT.** Before this lane the resolution path ended
/// `stored_ack_mode().unwrap_or(AckMode::Lease)`, and `stored_ack_mode` mapped both an `Err`
/// (via `.ok()?`) and an unknown value (via `_ => None`) to `None`. Removing this function,
/// or making it reject, would therefore have produced the *same* silent downgrade by a third
/// route. The cure is a THIRD STATE, not a rejection — see [`AckModeConfigState`].
///
/// ⚠ **THE USER-FACING CLAIM THIS PROTECTS.** A user who set `ack_mode=legacy` asked for
/// "the relay keeps nothing, even briefly". Lease holds an unacked frame for the lease
/// window. That is a privacy-relevant change to a setting the user chose, and it is not
/// made silently.
pub(crate) const ACK_MODE_RETIRED_KEY_STATE: &str = "ack_mode_retired_key_present";

/// The three states a config file can be in with respect to the retired `ack_mode` key.
///
/// ⚠ TWO states cannot express this. `Option<T>` collapses "absent" and "present but
/// retired" into one, and that collapse IS the defect this lane exists to remove.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum AckModeConfigState {
    /// No `ack_mode` key in the config file — the ordinary case.
    Nothing,
    /// The retired key IS present, carrying this raw value. Announced, never obeyed.
    RetiredKeyPresent(String),
}

/// NA-0688 C4 (D622 R7): parse `config.txt` as an ordered `key=value` list.
///
/// ⚠ **THIS FILE BECAME MULTI-KEY IN C4, AND THAT IS WHY THIS FUNCTION EXISTS.** It previously held
/// exactly one line, `policy_profile=…`, written by a writer that rewrote the whole file. R7 put
/// per-install preferences here (they are not secrets, and unlike the vault this store cannot
/// silently fail to apply when locked), so a second key had to coexist with the first.
///
/// ⚠ A MALFORMED LINE IS STILL AN ERROR. Corruption detection is not weakened by going multi-key:
/// any non-empty line without `=` fails the parse, which is what `doctor`'s `file_parseable` check
/// depends on. What changed is only that a *well-formed* file which happens not to mention a
/// particular key is no longer "corrupt" — it is simply a file that does not set that key.
fn read_config_kv(path: &Path) -> Result<Vec<(String, String)>, ErrorCode> {
    let mut f = File::open(path).map_err(|_| ErrorCode::IoReadFailed)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)
        .map_err(|_| ErrorCode::IoReadFailed)?;
    let mut out = Vec::new();
    for line in buf.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (k, v) = line.split_once('=').ok_or(ErrorCode::ParseFailed)?;
        let k = k.trim();
        if k.is_empty() {
            return Err(ErrorCode::ParseFailed);
        }
        out.push((k.to_string(), v.trim().to_string()));
    }
    Ok(out)
}

/// ⚠ NA-0688 C4: `Ok(None)` now means "this file does not set the profile", where it previously
/// meant `Err(ParseFailed)`. Both callers already handle `Ok(None)` — `config_get` prints "unset"
/// and `doctor` counts it parseable — so a config that sets only `ack_mode` no longer reports the
/// store as corrupt. **A genuinely malformed file still errors**, via `read_config_kv`.
pub(crate) fn read_policy_profile(path: &Path) -> Result<Option<String>, ErrorCode> {
    if !path.exists() {
        return Ok(None);
    }
    for (k, v) in read_config_kv(path)? {
        if k == POLICY_KEY {
            return match normalize_profile(v.as_str()) {
                Ok(v) => Ok(Some(v)),
                Err(_) => Err(ErrorCode::ParseFailed),
            };
        }
    }
    Ok(None)
}

/// NA-0770 (D-1411): DETECT PRESENCE, do not interpret.
///
/// ⚠ **THE VALUE IS RETURNED RAW AND UN-NORMALISED, AND THAT IS DELIBERATE.** Whatever the
/// file says — `legacy`, `lease`, or a typo — the key is retired, so every value is reported
/// identically as [`AckModeConfigState::RetiredKeyPresent`]. Normalising first would have
/// meant deciding which retired values are "valid", a distinction with no meaning once the
/// mode is gone, and it would have re-introduced a path where a malformed value and an
/// absent key look the same to the caller.
pub(crate) fn read_ack_mode_state(path: &Path) -> Result<AckModeConfigState, ErrorCode> {
    if !path.exists() {
        return Ok(AckModeConfigState::Nothing);
    }
    for (k, v) in read_config_kv(path)? {
        if k == ACK_MODE_KEY {
            return Ok(AckModeConfigState::RetiredKeyPresent(v));
        }
    }
    Ok(AckModeConfigState::Nothing)
}

pub(crate) fn ensure_dir_secure(dir: &Path, source: ConfigSource) -> Result<(), ErrorCode> {
    enforce_safe_parents(dir, source)?;
    if !dir.exists() {
        // NA-0757 (ENG-0239, R388 A1(a)): create the directory MODE-EXPLICITLY. A bare
        // `create_dir_all` takes its mode from the CALLING process's umask -- the `qsc`
        // BINARY sets one (`main.rs:59`), and every in-process library consumer does not.
        // A directory born group- or world-writable is then refused for good by
        // `enforce_safe_parents` (`perms_group_or_world_writable` = `mode & 0o022 != 0`),
        // which is the ENG-0239 field failure. `0o700` carries no group or world bits, so
        // NO umask can widen it and there is no window in which a wrong mode exists on
        // disk -- the chmod below could only narrow one after the fact.
        #[cfg(unix)]
        {
            use std::os::unix::fs::DirBuilderExt;
            fs::DirBuilder::new()
                .recursive(true)
                .mode(0o700)
                .create(dir)
                .map_err(|_| ErrorCode::IoWriteFailed)?;
        }
        #[cfg(not(unix))]
        fs::create_dir_all(dir).map_err(|_| ErrorCode::IoWriteFailed)?;
    }
    // ⚠ UNCHANGED, and deliberately: this still REPAIRS the mode of a directory that
    // already existed. R388 A1(d) keeps an INHERITED exposed directory refused; what the
    // block above changes is only the mode of a directory this process CREATES.
    #[cfg(unix)]
    {
        enforce_dir_perms(dir)?;
    }
    Ok(())
}

/// NA-0688 C4 (D622 R7): set one key in `config.txt`, **preserving every other key**.
///
/// ⚠ This REPLACED `write_config_atomic`, which is gone rather than kept as a wrapper: its only
/// purpose was to hide the single-key file format, and once the file is multi-key it hid nothing
/// while still being able to clobber the other key.
///
/// ⚠ **THE OLD WRITER CLOBBERED THE WHOLE FILE.** It emitted a single `policy_profile=…` line, which
/// was correct while that was the only key and becomes silent data loss the moment there are two:
/// setting the profile would have deleted the user's ack-mode preference, and vice versa. That is
/// why this is a read-modify-write rather than a second single-key writer.
///
/// The existing key order is preserved and a new key is appended, so the file stays diffable and a
/// rewrite does not churn unrelated lines.
pub(crate) fn write_config_key(
    path: &Path,
    key: &str,
    value: &str,
    source: ConfigSource,
) -> Result<(), ErrorCode> {
    let mut entries = if path.exists() {
        read_config_kv(path)?
    } else {
        Vec::new()
    };
    match entries.iter_mut().find(|(k, _)| k == key) {
        Some(slot) => slot.1 = value.to_string(),
        None => entries.push((key.to_string(), value.to_string())),
    }
    let mut content = String::new();
    for (k, v) in &entries {
        content.push_str(k);
        content.push('=');
        content.push_str(v);
        content.push('\n');
    }
    write_atomic(path, content.as_bytes(), source)
}

pub(crate) fn ensure_store_layout(dir: &Path, source: ConfigSource) -> Result<(), ErrorCode> {
    ensure_dir_secure(dir, source)?;
    let meta = dir.join(STORE_META_NAME);
    if meta.exists() {
        return Ok(());
    }
    write_atomic(&meta, STORE_META_TEMPLATE.as_bytes(), source)?;
    Ok(())
}

pub(crate) fn write_atomic(
    path: &Path,
    content: &[u8],
    source: ConfigSource,
) -> Result<(), ErrorCode> {
    let dir = path.parent().ok_or(ErrorCode::IoWriteFailed)?;
    enforce_safe_parents(path, source)?;
    #[cfg(unix)]
    if dir.exists() {
        enforce_dir_perms(dir)?;
    }
    let tmp_name = format!(
        "{}.tmp.{}",
        path.file_name().and_then(|v| v.to_str()).unwrap_or("tmp"),
        process::id()
    );
    let tmp_path = dir.join(tmp_name);
    let _ = fs::remove_file(&tmp_path);

    let mut f = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&tmp_path)
        .map_err(|_| ErrorCode::IoWriteFailed)?;
    #[cfg(unix)]
    enforce_file_perms(&tmp_path)?;
    f.write_all(content).map_err(|_| ErrorCode::IoWriteFailed)?;
    f.sync_all().map_err(|_| ErrorCode::IoWriteFailed)?;
    fs::rename(&tmp_path, path).map_err(|_| ErrorCode::IoWriteFailed)?;
    fsync_dir_best_effort(dir);
    Ok(())
}

pub(crate) fn enforce_safe_parents(path: &Path, source: ConfigSource) -> Result<(), ErrorCode> {
    let mut cur = PathBuf::new();
    for comp in path.components() {
        cur.push(comp);
        if cur.exists() {
            let md = fs::symlink_metadata(&cur).map_err(|_| ErrorCode::IoReadFailed)?;
            if md.file_type().is_symlink() {
                return Err(ErrorCode::UnsafePathSymlink);
            }
        } else {
            break;
        }
    }

    match source {
        ConfigSource::DefaultHome => {
            let mut cur = PathBuf::new();
            for comp in path.components() {
                cur.push(comp);
                if cur.exists() {
                    let md = fs::symlink_metadata(&cur).map_err(|_| ErrorCode::IoReadFailed)?;
                    #[cfg(unix)]
                    {
                        if md.is_dir() && perms_group_or_world_writable(&md) {
                            return Err(ErrorCode::UnsafeParentPerms);
                        }
                    }
                } else {
                    break;
                }
            }
        }
        ConfigSource::EnvOverride | ConfigSource::XdgConfigHome => {
            let root = if path.is_dir() {
                path
            } else {
                path.parent().unwrap_or(path)
            };
            if root.exists() {
                let md = fs::symlink_metadata(root).map_err(|_| ErrorCode::IoReadFailed)?;
                #[cfg(unix)]
                {
                    if md.is_dir() && perms_group_or_world_writable(&md) {
                        return Err(ErrorCode::UnsafeParentPerms);
                    }
                }
            }
        }
    }
    Ok(())
}

pub(crate) fn check_symlink_safe(path: &Path) -> bool {
    let mut cur = PathBuf::new();
    for comp in path.components() {
        cur.push(comp);
        if cur.exists() {
            match fs::symlink_metadata(&cur) {
                Ok(md) => {
                    if md.file_type().is_symlink() {
                        return false;
                    }
                }
                Err(_) => return false,
            }
        } else {
            break;
        }
    }
    true
}

pub(crate) fn check_parent_safe(path: &Path, source: ConfigSource) -> bool {
    let mut cur = PathBuf::new();
    match source {
        ConfigSource::DefaultHome => {
            for comp in path.components() {
                cur.push(comp);
                if cur.exists() {
                    match fs::symlink_metadata(&cur) {
                        Ok(md) => {
                            #[cfg(unix)]
                            {
                                if md.is_dir() && perms_group_or_world_writable(&md) {
                                    return false;
                                }
                            }
                        }
                        Err(_) => return false,
                    }
                } else {
                    break;
                }
            }
        }
        ConfigSource::EnvOverride | ConfigSource::XdgConfigHome => {
            let root = if path.is_dir() {
                path
            } else {
                path.parent().unwrap_or(path)
            };
            if root.exists() {
                match fs::symlink_metadata(root) {
                    Ok(md) => {
                        #[cfg(unix)]
                        {
                            if md.is_dir() && perms_group_or_world_writable(&md) {
                                return false;
                            }
                        }
                    }
                    Err(_) => return false,
                }
            }
        }
    }
    true
}

pub(crate) fn lock_store_exclusive(
    dir: &Path,
    source: ConfigSource,
) -> Result<LockGuard, ErrorCode> {
    enforce_safe_parents(dir, source)?;
    if !dir.exists() {
        fs::create_dir_all(dir).map_err(|_| ErrorCode::IoWriteFailed)?;
    }
    enforce_dir_perms(dir)?;
    let lock_path = dir.join(LOCK_FILE_NAME);
    enforce_safe_parents(&lock_path, source)?;
    // NA-0696 (D630 D1, D-1336): the lock file is created (never truncated) BEFORE the
    // perms check, preserving the pre-registry order; `LockGuard::acquire` re-opens it with
    // the same flags at depth 0 only (a nested acquisition opens nothing — the registry
    // carries it). The full enforce chain above and below runs on EVERY acquisition,
    // nested ones included. D6: the helper is straight-line — the lock-claim cfg masks
    // are deleted; non-Unix refuses at compile time.
    drop(
        OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(&lock_path)
            .map_err(|_| ErrorCode::LockOpenFailed)?,
    );
    enforce_file_perms(&lock_path)?;
    LockGuard::acquire(dir, &lock_path, LockMode::Exclusive)
}

pub(crate) fn lock_store_shared(
    dir: &Path,
    source: ConfigSource,
) -> Result<Option<LockGuard>, ErrorCode> {
    enforce_safe_parents(dir, source)?;
    if !dir.exists() {
        return Ok(None);
    }
    enforce_dir_perms(dir)?;
    let lock_path = dir.join(LOCK_FILE_NAME);
    enforce_safe_parents(&lock_path, source)?;
    // NA-0696 (D630 D1, D-1336): same create-before-perms order as the exclusive helper;
    // the no-dir `Ok(None)` arm above stays a no-registry non-entry (Q1, as ruled).
    drop(
        OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(&lock_path)
            .map_err(|_| ErrorCode::LockOpenFailed)?,
    );
    enforce_file_perms(&lock_path)?;
    LockGuard::acquire(dir, &lock_path, LockMode::Shared).map(Some)
}

pub(crate) fn probe_dir_writable(dir: &Path, timeout_ms: u64) -> bool {
    let tmp = dir.join(format!("probe.tmp.{}", process::id()));
    let start = Instant::now();
    let timeout = Duration::from_millis(timeout_ms.max(1));
    loop {
        let res = OpenOptions::new().create_new(true).write(true).open(&tmp);
        if let Ok(mut f) = res {
            let _ = f.write_all(b"");
            let _ = f.sync_all();
            let _ = fs::remove_file(&tmp);
            return true;
        }
        if start.elapsed() >= timeout {
            return false;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

#[cfg(unix)]
pub(crate) fn perms_group_or_world_writable(md: &fs::Metadata) -> bool {
    use std::os::unix::fs::PermissionsExt;
    let mode = md.permissions().mode();
    (mode & 0o022) != 0
}

#[cfg(unix)]
pub(crate) fn enforce_dir_perms(dir: &Path) -> Result<(), ErrorCode> {
    use std::os::unix::fs::PermissionsExt;
    let md = fs::symlink_metadata(dir).map_err(|_| ErrorCode::IoReadFailed)?;
    if md.file_type().is_symlink() {
        return Err(ErrorCode::UnsafePathSymlink);
    }
    let perms = md.permissions().mode() & 0o777;
    if perms != 0o700 {
        fs::set_permissions(dir, fs::Permissions::from_mode(0o700))
            .map_err(|_| ErrorCode::IoWriteFailed)?;
    }
    Ok(())
}

#[cfg(unix)]
pub(crate) fn enforce_file_perms(path: &Path) -> Result<(), ErrorCode> {
    use std::os::unix::fs::PermissionsExt;
    let md = fs::symlink_metadata(path).map_err(|_| ErrorCode::IoReadFailed)?;
    if md.file_type().is_symlink() {
        return Err(ErrorCode::UnsafePathSymlink);
    }
    let perms = md.permissions().mode() & 0o777;
    if perms != 0o600 {
        fs::set_permissions(path, fs::Permissions::from_mode(0o600))
            .map_err(|_| ErrorCode::IoWriteFailed)?;
    }
    Ok(())
}

// NA-0696 (D630 D6): the non-Unix no-op stubs are deleted — a non-Unix build now refuses
// at compile time (`model/mod.rs`) instead of silently skipping durability and hygiene.
// The platform-API cfg-unix attributes below stay (the §0.12 boundary): vacuous under
// the compile refusal, and churning them is not this lane's scope.
#[cfg(unix)]
pub(crate) fn fsync_dir_best_effort(dir: &Path) {
    let _ = File::open(dir).and_then(|d| d.sync_all());
}

#[cfg(unix)]
pub fn set_umask_077() {
    unsafe {
        crate::umask(0o077);
    }
}

// NA-0787 F04-C07P/S2 (C07 T8.1 G-365; RULING_F04C07P_formalization R4, shape G2): the CHECKED
// durable write. `write_atomic` and `fsync_dir_best_effort` above are deliberately UNCHANGED --
// their repair is ENG-0365, F05's -- and nothing existing calls what follows. The provider's C2,
// C3 and C5 writes (S5) are its first callers; S6 maps `DurableWriteError` to
// storage_durability_failed. No client-code string is allocated here (RULING_F04C07P R6).

/// Why a checked durable write failed, by stage.
///
/// WHY STAGES AND NOT ONE CODE: `write_atomic` folds every stage into `IoWriteFailed` and
/// DISCARDS the directory flush, so its caller cannot tell "nothing was written" from "the new
/// file is in place but its directory entry is not known durable". Here the two are distinct:
/// every variant but `DirFlush` means the destination was NOT replaced (absent, or still its old
/// bytes) and no temp of this call is left behind; `DirFlush` from `write_file_durable` means the
/// new bytes ARE in place.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub(crate) enum DurableWriteError {
    /// The existing path hygiene refused (safe parents, directory or file perms), or the path has
    /// no parent directory. The code is the hygiene's own, unchanged.
    Hygiene(ErrorCode),
    /// `tmp_path` is not a sibling of `path` distinct from it. Refused before any write.
    TempNotSibling,
    /// Creating the temp (it must not already exist) or writing its bytes failed.
    TempCreateOrWrite,
    /// The temp's flush failed: its bytes are not known durable, so it was never renamed.
    FileFlush,
    /// The rename over the destination failed; the destination is unchanged.
    Rename,
    /// The directory flush failed. After `write_file_durable`'s rename this means the new file IS
    /// in place while its directory entry is not known durable.
    DirFlush,
}

/// Open `dir` and flush it, reporting ANY failure -- the open included -- as `DirFlush`. The
/// checked counterpart of `fsync_dir_best_effort`, which discards the same result.
#[allow(dead_code)]
pub(crate) fn sync_dir_checked(dir: &Path) -> Result<(), DurableWriteError> {
    let flushed = File::open(dir).and_then(|d| d.sync_all());
    #[cfg(test)]
    let flushed = durable_flush_fault_apply(DurableFlushPoint::Dir, flushed);
    flushed.map_err(|_| DurableWriteError::DirFlush)
}

/// Write `bytes` to `path` durably: a new temp at the caller-named `tmp_path` (a sibling of
/// `path`), its flush CHECKED, renamed over `path`, then the directory flush CHECKED. `Ok` means
/// every stage succeeded. The path hygiene is `write_atomic`'s, reused unchanged.
///
/// Two deliberate differences from `write_atomic`:
/// - an existing file at `tmp_path` is NOT removed first. It is not this call's (stale temps are
///   the recovery pass's to judge), so `create_new` refuses it as `TempCreateOrWrite` and it stays.
/// - on any failure after the temp is created, up to and including the rename, that temp is
///   removed: the primitive never leaves its own temp behind (`write_atomic` can; OWED L5).
#[allow(dead_code)]
pub(crate) fn write_file_durable(
    path: &Path,
    bytes: &[u8],
    tmp_path: &Path,
    source: ConfigSource,
) -> Result<(), DurableWriteError> {
    let dir = match path.parent() {
        Some(d) if !d.as_os_str().is_empty() => d,
        _ => return Err(DurableWriteError::Hygiene(ErrorCode::IoWriteFailed)),
    };
    if tmp_path == path || tmp_path.parent() != Some(dir) {
        return Err(DurableWriteError::TempNotSibling);
    }
    enforce_safe_parents(path, source).map_err(DurableWriteError::Hygiene)?;
    #[cfg(unix)]
    if dir.exists() {
        enforce_dir_perms(dir).map_err(DurableWriteError::Hygiene)?;
    }
    // NA-0787 S7b X2 (F-02): the temp is born 0600 under any umask; the chmod below stays.
    let mut f = {
        use std::os::unix::fs::OpenOptionsExt;
        OpenOptions::new()
            .create_new(true)
            .write(true)
            .mode(0o600)
            .open(tmp_path)
    }
    .map_err(|_| DurableWriteError::TempCreateOrWrite)?;
    // From here the temp exists and is this call's own.
    if let Err(e) = fill_flush_rename(&mut f, bytes, tmp_path, path) {
        drop(f);
        let _ = fs::remove_file(tmp_path);
        return Err(e);
    }
    // The temp's descriptor stays open through the directory flush, as in `write_atomic`: the
    // G365-1 instrument depends on it to make that flush's directory open the one that fails.
    let flushed = sync_dir_checked(dir);
    drop(f);
    flushed
}

fn fill_flush_rename(
    f: &mut File,
    bytes: &[u8],
    tmp_path: &Path,
    path: &Path,
) -> Result<(), DurableWriteError> {
    #[cfg(unix)]
    enforce_file_perms(tmp_path).map_err(DurableWriteError::Hygiene)?;
    f.write_all(bytes)
        .map_err(|_| DurableWriteError::TempCreateOrWrite)?;
    let flushed = f.sync_all();
    #[cfg(test)]
    let flushed = durable_flush_fault_apply(DurableFlushPoint::File, flushed);
    flushed.map_err(|_| DurableWriteError::FileFlush)?;
    fs::rename(tmp_path, path).map_err(|_| DurableWriteError::Rename)
}

// The test-only failure-injection seam for the two checked flushes (S5's F1/F2 cut rows use it).
// Every item is cfg(test): a non-test build has no seam at all. The seam REPLACES a flush's
// RESULT, so an injected failure travels the same checked path as a real one.

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DurableFlushPoint {
    File,
    Dir,
}

#[cfg(test)]
thread_local! {
    static DURABLE_FLUSH_FAULT: std::cell::Cell<Option<(DurableFlushPoint, u32)>> =
        const { std::cell::Cell::new(None) };
}

/// Arm ONE injected failure on this thread: after `skip` flushes at `point` pass through, the
/// next flush at `point` fails and the fault is spent.
#[cfg(test)]
pub(crate) fn arm_durable_flush_fault(point: DurableFlushPoint, skip: u32) {
    DURABLE_FLUSH_FAULT.with(|c| c.set(Some((point, skip))));
}

#[cfg(test)]
fn durable_flush_fault_apply(
    point: DurableFlushPoint,
    flushed: std::io::Result<()>,
) -> std::io::Result<()> {
    DURABLE_FLUSH_FAULT.with(|c| match c.get() {
        Some((p, 0)) if p == point => {
            c.set(None);
            Err(std::io::Error::other("injected durable flush fault"))
        }
        Some((p, n)) if p == point => {
            c.set(Some((p, n - 1)));
            flushed
        }
        _ => flushed,
    })
}

#[cfg(test)]
mod durable_write_tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;

    /// A fresh directory at 0700: `enforce_safe_parents` refuses a group-writable parent, and a
    /// tempdir's mode follows the session umask (the formalization trial's attempt-1 setup failure).
    fn private_dir() -> tempfile::TempDir {
        let td = tempfile::tempdir().expect("tempdir");
        fs::set_permissions(td.path(), fs::Permissions::from_mode(0o700)).expect("chmod 0700");
        td
    }

    fn names(dir: &Path) -> Vec<String> {
        let mut v: Vec<String> = fs::read_dir(dir)
            .expect("read_dir")
            .map(|e| e.expect("entry").file_name().to_string_lossy().into_owned())
            .collect();
        v.sort();
        v
    }

    #[test]
    fn durable_write_new_path_happy() {
        let td = private_dir();
        let path = td.path().join("f");
        let r = write_file_durable(
            &path,
            b"new",
            &td.path().join("f.tmp"),
            ConfigSource::EnvOverride,
        );
        assert!(r.is_ok(), "{r:?}");
        assert_eq!(fs::read(&path).expect("read"), b"new");
        let mode = fs::metadata(&path).expect("metadata").permissions().mode() & 0o777;
        assert_eq!(mode, 0o600);
        assert_eq!(names(td.path()), ["f"], "no temp may be left");
    }

    #[test]
    fn durable_write_replaces_existing() {
        let td = private_dir();
        let path = td.path().join("f");
        fs::write(&path, b"old").expect("seed");
        let r = write_file_durable(
            &path,
            b"new",
            &td.path().join("f.tmp"),
            ConfigSource::EnvOverride,
        );
        assert!(r.is_ok(), "{r:?}");
        assert_eq!(fs::read(&path).expect("read"), b"new");
        assert_eq!(names(td.path()), ["f"], "no temp may be left");
    }

    #[test]
    fn durable_write_refuses_unsafe_parent() {
        let td = private_dir();
        fs::set_permissions(td.path(), fs::Permissions::from_mode(0o777)).expect("chmod 0777");
        let path = td.path().join("f");
        let r = write_file_durable(
            &path,
            b"new",
            &td.path().join("f.tmp"),
            ConfigSource::EnvOverride,
        );
        assert!(
            matches!(
                r,
                Err(DurableWriteError::Hygiene(ErrorCode::UnsafeParentPerms))
            ),
            "{r:?}"
        );
        assert!(names(td.path()).is_empty(), "nothing may be written");
    }

    #[test]
    fn durable_write_refuses_tmp_in_other_dir() {
        let a = private_dir();
        let b = private_dir();
        let r = write_file_durable(
            &a.path().join("f"),
            b"new",
            &b.path().join("f.tmp"),
            ConfigSource::EnvOverride,
        );
        assert!(matches!(r, Err(DurableWriteError::TempNotSibling)), "{r:?}");
        assert!(
            names(a.path()).is_empty(),
            "nothing may be written in the destination dir"
        );
        assert!(
            names(b.path()).is_empty(),
            "nothing may be written in the temp's dir"
        );
    }

    #[test]
    fn durable_write_refuses_tmp_equal_to_path() {
        let td = private_dir();
        let path = td.path().join("f");
        let r = write_file_durable(&path, b"new", &path, ConfigSource::EnvOverride);
        assert!(matches!(r, Err(DurableWriteError::TempNotSibling)), "{r:?}");
        assert!(names(td.path()).is_empty(), "nothing may be written");
    }

    #[test]
    fn durable_write_keeps_foreign_existing_temp() {
        let td = private_dir();
        let path = td.path().join("f");
        let tmp = td.path().join("f.tmp");
        fs::write(&tmp, b"stale").expect("seed temp");
        let r = write_file_durable(&path, b"new", &tmp, ConfigSource::EnvOverride);
        assert!(
            matches!(r, Err(DurableWriteError::TempCreateOrWrite)),
            "{r:?}"
        );
        assert_eq!(
            fs::read(&tmp).expect("read temp"),
            b"stale",
            "a temp not ours stays"
        );
        assert!(!path.exists(), "the destination must stay absent");
    }

    #[test]
    fn durable_write_file_flush_failure_leaves_destination() {
        let td = private_dir();
        let path = td.path().join("f");
        let tmp = td.path().join("f.tmp");
        // Arm 1: no destination yet.
        arm_durable_flush_fault(DurableFlushPoint::File, 0);
        let r = write_file_durable(&path, b"new", &tmp, ConfigSource::EnvOverride);
        assert!(matches!(r, Err(DurableWriteError::FileFlush)), "{r:?}");
        assert!(!path.exists(), "the destination must stay absent");
        assert!(!tmp.exists(), "the temp must be removed");
        // Arm 2: an existing destination keeps its old bytes.
        fs::write(&path, b"old").expect("seed");
        arm_durable_flush_fault(DurableFlushPoint::File, 0);
        let r = write_file_durable(&path, b"new", &tmp, ConfigSource::EnvOverride);
        assert!(matches!(r, Err(DurableWriteError::FileFlush)), "{r:?}");
        assert_eq!(fs::read(&path).expect("read"), b"old");
        assert!(!tmp.exists(), "the temp must be removed");
    }

    #[test]
    fn durable_write_dir_flush_failure_after_landing() {
        let td = private_dir();
        let path = td.path().join("f");
        fs::write(&path, b"old").expect("seed");
        arm_durable_flush_fault(DurableFlushPoint::Dir, 0);
        let r = write_file_durable(
            &path,
            b"new",
            &td.path().join("f.tmp"),
            ConfigSource::EnvOverride,
        );
        assert!(matches!(r, Err(DurableWriteError::DirFlush)), "{r:?}");
        // The landed state is part of the claim: DirFlush means the new bytes ARE in place.
        assert_eq!(fs::read(&path).expect("read"), b"new");
        assert_eq!(names(td.path()), ["f"], "no temp may be left");
    }

    #[test]
    fn durable_flush_fault_skips_then_fires_once() {
        let td = private_dir();
        let path = td.path().join("f");
        let tmp = td.path().join("f.tmp");
        arm_durable_flush_fault(DurableFlushPoint::Dir, 1);
        let r = write_file_durable(&path, b"one", &tmp, ConfigSource::EnvOverride);
        assert!(r.is_ok(), "the skipped flush must pass through: {r:?}");
        let r = write_file_durable(&path, b"two", &tmp, ConfigSource::EnvOverride);
        assert!(matches!(r, Err(DurableWriteError::DirFlush)), "{r:?}");
        assert_eq!(fs::read(&path).expect("read"), b"two");
        let r = write_file_durable(&path, b"three", &tmp, ConfigSource::EnvOverride);
        assert!(r.is_ok(), "the fault must be spent: {r:?}");
        assert_eq!(fs::read(&path).expect("read"), b"three");
    }

    #[test]
    fn sync_dir_checked_reports_missing_dir() {
        let td = private_dir();
        let r = sync_dir_checked(td.path());
        assert!(r.is_ok(), "control: an existing directory flushes: {r:?}");
        let r = sync_dir_checked(&td.path().join("missing"));
        assert!(matches!(r, Err(DurableWriteError::DirFlush)), "{r:?}");
    }

    /// G365-1 (C07 T8.1 G-365; formalization trial g365_trial_module.rs, ported to the checked
    /// primitive). The child lowers RLIMIT_NOFILE so the temp takes the last permitted descriptor
    /// and the directory open inside the flush fails EMFILE. No test hook, no feature. Linux only:
    /// RLIMIT_NOFILE = 7 and a 64-bit rlim_t are Linux values; macOS does not run these.
    #[cfg(target_os = "linux")]
    mod g365 {
        use super::super::*;
        use super::private_dir;
        use std::os::unix::io::AsRawFd;

        #[repr(C)]
        struct RLimit {
            cur: u64,
            max: u64,
        }
        extern "C" {
            fn getrlimit(res: i32, rl: *mut RLimit) -> i32;
            fn setrlimit(res: i32, rl: *const RLimit) -> i32;
        }
        const RLIMIT_NOFILE: i32 = 7;
        const CHILD: &str = "fs_store::durable_write_tests::g365::child";

        fn lower_to_lowest_free() -> (RLimit, u64) {
            let probe = File::open("/dev/null").expect("probe");
            let n = probe.as_raw_fd() as u64;
            drop(probe);
            let mut old = RLimit { cur: 0, max: 0 };
            assert_eq!(unsafe { getrlimit(RLIMIT_NOFILE, &mut old) }, 0);
            let new = RLimit {
                cur: n + 1,
                max: old.max,
            };
            assert_eq!(unsafe { setrlimit(RLIMIT_NOFILE, &new) }, 0);
            (old, n)
        }

        #[test]
        #[ignore = "G365-1 child: runs only when re-executed by its parent tests"]
        fn child() {
            let Some(dir) = std::env::var_os("G365_DIR") else {
                return;
            };
            let dir = PathBuf::from(dir);
            let mode = std::env::var("G365_MODE").unwrap_or_default();
            if mode == "control" {
                // The mechanism is real: with the lowest free fd occupied, the directory open fails EMFILE.
                let (old, n) = lower_to_lowest_free();
                let hold = File::open("/dev/null").expect("hold");
                let r = File::open(&dir);
                let emfile = matches!(&r, Err(e) if e.raw_os_error() == Some(24));
                drop(r);
                drop(hold);
                unsafe { setrlimit(RLIMIT_NOFILE, &old) };
                println!(
                    "G365_CONTROL lowest_free_fd={n} nofile_before={} dir_open_emfile={emfile}",
                    old.cur
                );
                assert!(
                    emfile,
                    "control: the directory open must fail EMFILE under the lowered limit"
                );
                return;
            }
            let path = dir.join("f");
            let (old, n) = lower_to_lowest_free();
            let r = write_file_durable(&path, b"x", &dir.join("f.tmp"), ConfigSource::EnvOverride);
            unsafe { setrlimit(RLIMIT_NOFILE, &old) };
            let landed = fs::read(&path).map(|b| b == b"x").unwrap_or(false);
            println!("G365_CHILD lowest_free_fd={n} nofile_before={} write_file_durable={r:?} landed={landed}", old.cur);
            assert!(landed, "setup: the renamed file must be in place");
            assert!(
                matches!(r, Err(DurableWriteError::DirFlush)),
                "G-365: write_file_durable did not report the directory flush that could not run"
            );
        }

        fn run_child(mode: &str, marker: &str) {
            let td = private_dir();
            let out = std::process::Command::new(std::env::current_exe().expect("exe"))
                .args([
                    "--ignored",
                    "--exact",
                    CHILD,
                    "--nocapture",
                    "--test-threads=1",
                ])
                .env("G365_DIR", td.path())
                .env("G365_MODE", mode)
                .output()
                .expect("spawn child");
            let stdout = String::from_utf8_lossy(&out.stdout);
            println!(
                "child[{mode}] status={:?}\n{stdout}\n{}",
                out.status,
                String::from_utf8_lossy(&out.stderr)
            );
            // Vacuity guard: a child that matched zero tests exits 0 without printing its marker.
            assert!(stdout.contains(marker), "setup: the child test did not run");
            assert!(out.status.success(), "child[{mode}] failed: see its output");
        }

        #[test]
        fn mechanism_control() {
            run_child("control", "G365_CONTROL ");
        }

        #[test]
        fn durable_dir_flush_failure_is_reported() {
            run_child("red", "G365_CHILD ");
        }
    }
}
