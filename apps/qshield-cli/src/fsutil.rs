use std::fs;
use std::io::Write;
use std::path::Path;

#[cfg(unix)]
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};

pub fn write_secure_file(path: &Path, data: &[u8]) -> Result<(), String> {
    let tmp = path.with_extension("tmp");
    let mut opts = fs::OpenOptions::new();
    opts.write(true).create(true).truncate(true);
    #[cfg(unix)]
    {
        opts.mode(0o600);
    }
    let mut f = opts.open(&tmp).map_err(|e| format!("open tmp: {e}"))?;
    f.write_all(data).map_err(|e| format!("write tmp: {e}"))?;
    f.sync_all().map_err(|e| format!("sync tmp: {e}"))?;
    fs::rename(&tmp, path).map_err(|e| format!("rename tmp: {e}"))?;
    Ok(())
}

pub fn ensure_dir_permissions(path: &Path) -> Result<(), String> {
    #[cfg(unix)]
    {
        let perms = fs::Permissions::from_mode(0o700);
        fs::set_permissions(path, perms).map_err(|e| format!("set store permissions: {e}"))?;
    }
    Ok(())
}

pub fn secure_delete_file(path: &Path) -> Result<(), String> {
    if !path.exists() {
        return Ok(());
    }
    #[cfg(unix)]
    {
        let perms = fs::Permissions::from_mode(0o600);
        fs::set_permissions(path, perms).map_err(|e| format!("set file permissions: {e}"))?;
    }
    let len = fs::metadata(path)
        .map_err(|e| format!("stat file: {e}"))?
        .len();
    if len > 0 {
        let mut f = fs::OpenOptions::new()
            .write(true)
            .open(path)
            .map_err(|e| format!("open file: {e}"))?;
        let buf = [0u8; 8192];
        let mut remaining = len;
        while remaining > 0 {
            let n = std::cmp::min(remaining, buf.len() as u64) as usize;
            f.write_all(&buf[..n])
                .map_err(|e| format!("wipe file: {e}"))?;
            remaining -= n as u64;
        }
        f.sync_all().map_err(|e| format!("sync file: {e}"))?;
    }
    fs::remove_file(path).map_err(|e| format!("remove file: {e}"))?;
    Ok(())
}
