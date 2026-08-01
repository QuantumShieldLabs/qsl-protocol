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

pub(crate) fn normalize_ack_mode(value: &str) -> Result<String, ErrorCode> {
    match value {
        "legacy" => Ok("legacy".to_string()),
        "lease" => Ok("lease".to_string()),
        _ => Err(ErrorCode::ParseFailed),
    }
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

pub(crate) fn read_ack_mode(path: &Path) -> Result<Option<String>, ErrorCode> {
    if !path.exists() {
        return Ok(None);
    }
    for (k, v) in read_config_kv(path)? {
        if k == ACK_MODE_KEY {
            return match normalize_ack_mode(v.as_str()) {
                Ok(v) => Ok(Some(v)),
                Err(_) => Err(ErrorCode::ParseFailed),
            };
        }
    }
    Ok(None)
}

pub(crate) fn ensure_dir_secure(dir: &Path, source: ConfigSource) -> Result<(), ErrorCode> {
    enforce_safe_parents(dir, source)?;
    if !dir.exists() {
        fs::create_dir_all(dir).map_err(|_| ErrorCode::IoWriteFailed)?;
    }
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
    #[cfg(unix)]
    {
        enforce_dir_perms(dir)?;
    }
    let lock_path = dir.join(LOCK_FILE_NAME);
    enforce_safe_parents(&lock_path, source)?;
    let file = OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(&lock_path)
        .map_err(|_| ErrorCode::LockOpenFailed)?;
    #[cfg(unix)]
    enforce_file_perms(&lock_path)?;
    #[cfg(unix)]
    LockGuard::lock(&file, LockMode::Exclusive)?;
    Ok(LockGuard { file })
}

pub(crate) fn lock_store_shared(
    dir: &Path,
    source: ConfigSource,
) -> Result<Option<LockGuard>, ErrorCode> {
    enforce_safe_parents(dir, source)?;
    if !dir.exists() {
        return Ok(None);
    }
    #[cfg(unix)]
    {
        enforce_dir_perms(dir)?;
    }
    let lock_path = dir.join(LOCK_FILE_NAME);
    enforce_safe_parents(&lock_path, source)?;
    let file = OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(&lock_path)
        .map_err(|_| ErrorCode::LockOpenFailed)?;
    #[cfg(unix)]
    enforce_file_perms(&lock_path)?;
    #[cfg(unix)]
    LockGuard::lock(&file, LockMode::Shared)?;
    Ok(Some(LockGuard { file }))
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

#[cfg(not(unix))]
pub(crate) fn fsync_dir_best_effort(_dir: &Path) {}

#[cfg(unix)]
pub(crate) fn fsync_dir_best_effort(dir: &Path) {
    let _ = File::open(dir).and_then(|d| d.sync_all());
}

#[cfg(not(unix))]
pub fn set_umask_077() {}

#[cfg(unix)]
pub fn set_umask_077() {
    unsafe {
        crate::umask(0o077);
    }
}
