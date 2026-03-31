use assert_cmd::Command;
use std::fs::{self, File, OpenOptions};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

fn test_root() -> PathBuf {
    let base = if let Ok(v) = std::env::var("QSC_TEST_ROOT") {
        PathBuf::from(v).join("qsc-test-tmp")
    } else if let Ok(v) = std::env::var("CARGO_TARGET_DIR") {
        PathBuf::from(v).join("qsc-test-tmp")
    } else {
        PathBuf::from("target").join("qsc-test-tmp")
    };
    if base.is_absolute() {
        base
    } else {
        std::env::current_dir().expect("cwd").join(base)
    }
}

fn unique_dir(tag: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    test_root().join(format!("{tag}_{}_{}", std::process::id(), nonce))
}

fn ensure_dir_700(path: &Path) {
    fs::create_dir_all(path).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
    }
}

fn qsc_cmd(cfg: &Path) -> Command {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("qsc");
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_MARK_FORMAT", "plain");
    cmd
}

fn output_text(out: &std::process::Output) -> String {
    let mut s = String::from_utf8_lossy(&out.stdout).to_string();
    s.push_str(&String::from_utf8_lossy(&out.stderr));
    s
}

fn tmp_residue(dir: &Path) -> Vec<String> {
    fs::read_dir(dir)
        .unwrap()
        .filter_map(Result::ok)
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|name| name.contains(".tmp."))
        .collect()
}

#[cfg(unix)]
fn mode(path: &Path) -> u32 {
    use std::os::unix::fs::PermissionsExt;
    fs::symlink_metadata(path).unwrap().permissions().mode() & 0o777
}

#[cfg(unix)]
fn hold_exclusive_lock(path: &Path) -> File {
    use std::os::unix::io::AsRawFd;
    const LOCK_EX: i32 = 2;
    const LOCK_NB: i32 = 4;

    let file = OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(path)
        .expect("open lock file");
    let rc = unsafe { flock(file.as_raw_fd(), LOCK_EX | LOCK_NB) };
    assert_eq!(rc, 0, "exclusive lock acquisition must succeed");
    file
}

#[cfg(unix)]
unsafe extern "C" {
    fn flock(fd: i32, operation: i32) -> i32;
}

#[test]
fn config_store_layout_preserves_secure_permissions_and_atomic_cleanup() {
    let base = unique_dir("na0217b_store_layout");
    let cfg = base.join("cfg");
    ensure_dir_700(&base);

    let first = qsc_cmd(&cfg)
        .args(["config", "set", "policy-profile", "baseline"])
        .output()
        .expect("config set baseline");
    assert!(first.status.success(), "{}", output_text(&first));
    assert_eq!(
        String::from_utf8_lossy(&first.stdout),
        "QSC_MARK/1 event=config_set key=policy_profile value=<redacted> ok=true\n"
    );

    let config_path = cfg.join("config.txt");
    let lock_path = cfg.join(".qsc.lock");
    let meta_path = cfg.join("store.meta");
    assert_eq!(
        fs::read_to_string(&config_path).unwrap(),
        "policy_profile=baseline\n"
    );
    assert_eq!(
        fs::read_to_string(&meta_path).unwrap(),
        "store_version=1\nvmk_status=unset\nkeyslots=0\n"
    );
    assert!(
        lock_path.exists(),
        "lock file must be created by config set"
    );
    #[cfg(unix)]
    {
        assert_eq!(mode(&cfg), 0o700);
        assert_eq!(mode(&config_path), 0o600);
        assert_eq!(mode(&lock_path), 0o600);
        assert_eq!(mode(&meta_path), 0o600);
    }

    let second = qsc_cmd(&cfg)
        .args(["config", "set", "policy-profile", "strict"])
        .output()
        .expect("config set strict");
    assert!(second.status.success(), "{}", output_text(&second));
    assert_eq!(
        fs::read_to_string(&config_path).unwrap(),
        "policy_profile=strict\n"
    );

    let tmp_files = tmp_residue(&cfg);
    assert!(
        tmp_files.is_empty(),
        "atomic writes must not leave temp files behind: {:?}",
        tmp_files
    );
}

#[cfg(unix)]
#[test]
fn symlinked_config_dir_rejects_fail_closed() {
    let base = unique_dir("na0217b_symlink_reject");
    ensure_dir_700(&base);
    let target = base.join("real_cfg");
    ensure_dir_700(&target);
    let link = base.join("cfg_link");
    std::os::unix::fs::symlink(&target, &link).unwrap();

    let out = qsc_cmd(&link)
        .args(["config", "set", "policy-profile", "baseline"])
        .output()
        .expect("config set via symlink");
    assert!(!out.status.success(), "symlinked config path must reject");
    assert!(
        output_text(&out).contains("event=error"),
        "{}",
        output_text(&out)
    );
    assert!(
        !target.join("config.txt").exists(),
        "symlink reject must not create config.txt"
    );
    assert!(
        !target.join("store.meta").exists(),
        "symlink reject must not create store.meta"
    );
    assert!(
        tmp_residue(&target).is_empty(),
        "symlink reject must not leave temp-file residue"
    );
}

#[cfg(unix)]
#[test]
fn lock_contention_remains_fail_closed_for_read_and_write_paths() {
    let base = unique_dir("na0217b_lock_contention");
    let cfg = base.join("cfg");
    ensure_dir_700(&base);

    let init = qsc_cmd(&cfg)
        .args(["config", "set", "policy-profile", "baseline"])
        .output()
        .expect("config set init");
    assert!(init.status.success(), "{}", output_text(&init));

    let lock_path = cfg.join(".qsc.lock");
    let _held = hold_exclusive_lock(&lock_path);

    let get_out = qsc_cmd(&cfg)
        .args(["config", "get", "policy-profile"])
        .output()
        .expect("config get under lock");
    assert!(
        !get_out.status.success(),
        "shared lock acquisition must fail while exclusive lock is held"
    );
    assert!(
        output_text(&get_out).contains("event=error code=lock_contended"),
        "{}",
        output_text(&get_out)
    );

    let set_out = qsc_cmd(&cfg)
        .args(["config", "set", "policy-profile", "strict"])
        .output()
        .expect("config set under lock");
    assert!(
        !set_out.status.success(),
        "exclusive lock acquisition must fail while exclusive lock is held"
    );
    assert!(
        output_text(&set_out).contains("event=error code=lock_contended"),
        "{}",
        output_text(&set_out)
    );
}
