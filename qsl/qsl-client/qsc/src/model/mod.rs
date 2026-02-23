use std::fs::File;

#[derive(Debug, Clone, Copy)]
pub(crate) enum ErrorCode {
    MissingHome,
    InvalidPolicyProfile,
    UnsafePathSymlink,
    UnsafeParentPerms,
    LockOpenFailed,
    LockContended,
    LockFailed,
    IoWriteFailed,
    IoReadFailed,
    ParseFailed,
    IdentitySecretUnavailable,
}

impl ErrorCode {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            ErrorCode::MissingHome => "missing_home",
            ErrorCode::InvalidPolicyProfile => "invalid_policy_profile",
            ErrorCode::UnsafePathSymlink => "unsafe_path_symlink",
            ErrorCode::UnsafeParentPerms => "unsafe_parent_perms",
            ErrorCode::LockOpenFailed => "lock_open_failed",
            ErrorCode::LockContended => "lock_contended",
            ErrorCode::LockFailed => "lock_failed",
            ErrorCode::IoWriteFailed => "io_write_failed",
            ErrorCode::IoReadFailed => "io_read_failed",
            ErrorCode::ParseFailed => "parse_failed",
            ErrorCode::IdentitySecretUnavailable => "identity_secret_unavailable",
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

pub(crate) struct LockGuard {
    pub(crate) file: File,
}

impl LockGuard {
    #[cfg(unix)]
    pub(crate) fn lock(file: &File, mode: LockMode) -> Result<(), ErrorCode> {
        use std::io::ErrorKind;
        use std::os::unix::io::AsRawFd;
        const LOCK_SH: i32 = 1;
        const LOCK_EX: i32 = 2;
        const LOCK_NB: i32 = 4;
        let op = match mode {
            LockMode::Shared => LOCK_SH,
            LockMode::Exclusive => LOCK_EX,
        };
        let rc = unsafe { crate::flock(file.as_raw_fd(), op | LOCK_NB) };
        if rc != 0 {
            let err = std::io::Error::last_os_error();
            if err.kind() == ErrorKind::WouldBlock {
                return Err(ErrorCode::LockContended);
            }
            return Err(ErrorCode::LockFailed);
        }
        Ok(())
    }
}

impl Drop for LockGuard {
    fn drop(&mut self) {
        #[cfg(unix)]
        {
            use std::os::unix::io::AsRawFd;
            const LOCK_UN: i32 = 8;
            let _ = unsafe { crate::flock(self.file.as_raw_fd(), LOCK_UN) };
        }
    }
}
