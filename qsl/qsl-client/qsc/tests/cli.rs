use assert_cmd::Command;
use predicates::prelude::*;
use predicates::str::contains;
use std::env;
use std::fs::{self, OpenOptions};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn version_is_printable() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("qsc"));
    cmd.arg("--version");
    cmd.assert().success();
}

#[test]
fn status_is_deterministic_marker() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("qsc"));
    cmd.arg("status");
    cmd.assert()
        .success()
        .stdout(contains("QSC_MARK/1 event=status ok=true locked=unknown"));
}

#[test]
fn config_set_get_roundtrip_baseline() {
    let dir = safe_test_dir("roundtrip");
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("qsc"));
    cmd.env("QSC_CONFIG_DIR", &dir)
        .args(["config", "set", "policy-profile", "baseline"]);
    cmd.assert().success().stdout(predicate::eq(
        "QSC_MARK/1 event=config_set key=policy_profile value=<redacted> ok=true\n",
    ));

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("qsc"));
    cmd.env("QSC_CONFIG_DIR", &dir)
        .args(["config", "get", "policy-profile"]);
    cmd.assert().success().stdout(predicate::eq(
        "QSC_MARK/1 event=config_get key=policy_profile value=<redacted> ok=true\n",
    ));
}

#[test]
fn config_get_reveal_shows_value() {
    let dir = safe_test_dir("reveal");
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("qsc"));
    cmd.env("QSC_CONFIG_DIR", &dir)
        .args(["config", "set", "policy-profile", "baseline"]);
    cmd.assert().success();

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("qsc"));
    cmd.env("QSC_CONFIG_DIR", &dir)
        .arg("--reveal")
        .args(["config", "get", "policy-profile"]);
    cmd.assert()
        .success()
        .stdout(contains("value=baseline"))
        .stdout(contains("<redacted>").not());
}

#[cfg(unix)]
#[test]
fn store_meta_created_and_perms() {
    use std::os::unix::fs::PermissionsExt;

    let dir = safe_test_dir("store-meta");
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("qsc"));
    cmd.env("QSC_CONFIG_DIR", &dir)
        .args(["config", "set", "policy-profile", "baseline"]);
    cmd.assert().success();

    let meta = dir.join("store.meta");
    assert!(meta.exists(), "store.meta must exist");
    let content = fs::read_to_string(&meta).unwrap();
    assert_eq!(content, "store_version=1\nvmk_status=unset\nkeyslots=0\n");

    let dir_mode = fs::symlink_metadata(&dir).unwrap().permissions().mode() & 0o777;
    assert_eq!(dir_mode, 0o700);
    let file_mode = fs::symlink_metadata(&meta).unwrap().permissions().mode() & 0o777;
    assert_eq!(file_mode, 0o600);

    let tmp_left = fs::read_dir(&dir)
        .unwrap()
        .any(|e| e.unwrap().file_name().to_string_lossy().contains(".tmp."));
    assert!(!tmp_left, "temp files must not remain after atomic write");
}

#[test]
fn invalid_policy_profile_set_is_no_mutation() {
    let dir = fresh_dir("invalid");
    let cfg = config_file(&dir);
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("qsc"));
    cmd.env("QSC_CONFIG_DIR", &dir)
        .args(["config", "set", "policy-profile", "bad"]);
    cmd.assert().failure().stdout(predicate::eq(
        "QSC_MARK/1 event=error code=invalid_policy_profile\n",
    ));
    assert!(!dir.exists(), "config dir must not be created");
    assert!(!cfg.exists(), "config file must not be created");
}

#[test]
fn doctor_check_only_no_dir() {
    let base = safe_test_dir("doctor-nodir");
    let dir = base.join("missing");
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("qsc"));
    cmd.env("QSC_CONFIG_DIR", &dir)
        .args(["doctor", "--check-only"]);
    cmd.assert().success().stdout(predicate::eq(
        "QSC_MARK/1 event=doctor check_only=true ok=true dir_exists=false dir_writable=false file_parseable=true symlink_safe=true parent_safe=true\n",
    ));
    assert!(!dir.exists(), "doctor must not create the dir");
}

#[cfg(unix)]
#[test]
fn unsafe_parent_perms_no_mutation() {
    use std::os::unix::fs::PermissionsExt;

    let dir = fresh_dir("unsafe_parent");
    fs::create_dir_all(&dir).unwrap();
    let cfg = config_file(&dir);
    fs::write(&cfg, "policy_profile=baseline\n").unwrap();
    fs::set_permissions(&cfg, fs::Permissions::from_mode(0o600)).unwrap();
    fs::set_permissions(&dir, fs::Permissions::from_mode(0o777)).unwrap();

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("qsc"));
    cmd.env("QSC_CONFIG_DIR", &dir)
        .args(["config", "set", "policy-profile", "strict"]);
    cmd.assert().failure().stdout(predicate::eq(
        "QSC_MARK/1 event=error code=unsafe_parent_perms\n",
    ));

    let contents = fs::read_to_string(&cfg).unwrap();
    assert_eq!(contents, "policy_profile=baseline\n");
}

#[cfg(unix)]
#[test]
fn symlink_path_rejected_no_mutation() {
    use std::os::unix::fs::symlink;

    let base = safe_test_dir("symlink");
    let real = base.join("real");
    fs::create_dir_all(&real).unwrap();
    let link = base.join("link");
    symlink(&real, &link).unwrap();

    let cfg = real.join("config.txt");
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("qsc"));
    cmd.env("QSC_CONFIG_DIR", &link)
        .args(["config", "set", "policy-profile", "baseline"]);
    cmd.assert().failure().stdout(predicate::eq(
        "QSC_MARK/1 event=error code=unsafe_path_symlink\n",
    ));

    assert!(!cfg.exists(), "config file must not be created");
}

#[cfg(unix)]
#[test]
fn lock_failure_no_mutation() {
    use std::os::unix::fs::PermissionsExt;
    use std::os::unix::io::AsRawFd;

    let dir = safe_test_dir("lock-fail");
    fs::create_dir_all(&dir).unwrap();
    fs::set_permissions(&dir, fs::Permissions::from_mode(0o700)).unwrap();

    let cfg = config_file(&dir);
    fs::write(&cfg, "policy_profile=baseline\n").unwrap();
    fs::set_permissions(&cfg, fs::Permissions::from_mode(0o600)).unwrap();

    let lock_path = dir.join(".qsc.lock");
    let lock_file = OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(&lock_path)
        .unwrap();
    fs::set_permissions(&lock_path, fs::Permissions::from_mode(0o600)).unwrap();
    let rc = unsafe { flock(lock_file.as_raw_fd(), LOCK_EX | LOCK_NB) };
    assert_eq!(rc, 0, "test setup must acquire lock");

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("qsc"));
    cmd.env("QSC_CONFIG_DIR", &dir)
        .args(["config", "set", "policy-profile", "strict"]);
    cmd.assert()
        .failure()
        .stdout(predicate::eq("QSC_MARK/1 event=error code=lock_failed\n"));

    let contents = fs::read_to_string(&cfg).unwrap();
    assert_eq!(contents, "policy_profile=baseline\n");
}

fn fresh_dir(label: &str) -> PathBuf {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let name = format!("qsc_test_{}_{}", label, now);
    env::temp_dir().join(name)
}

fn writable_test_base() -> PathBuf {
    if let Some(v) = env::var_os("QSC_TEST_ROOT") {
        return PathBuf::from(v);
    }
    if let Some(v) = env::var_os("CARGO_TARGET_DIR") {
        return PathBuf::from(v);
    }
    env::current_dir().unwrap().join("target")
}

fn safe_test_dir(label: &str) -> PathBuf {
    let base = writable_test_base();
    let root = base.join("qsc-test-tmp");
    fs::create_dir_all(&root).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&root, fs::Permissions::from_mode(0o700)).unwrap();
    }
    let n = NEXT_ID.fetch_add(1, Ordering::SeqCst);
    let dir = root.join(format!("{}-{}-{}", label, std::process::id(), n));
    fs::create_dir(&dir).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&dir, fs::Permissions::from_mode(0o700)).unwrap();
    }
    dir
}

fn config_file(dir: &Path) -> PathBuf {
    dir.join("config.txt")
}

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

#[cfg(unix)]
const LOCK_EX: i32 = 2;
#[cfg(unix)]
const LOCK_NB: i32 = 4;

#[cfg(unix)]
extern "C" {
    fn flock(fd: i32, operation: i32) -> i32;
}
