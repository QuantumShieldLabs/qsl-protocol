use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(3)
        .expect("repo root")
        .to_path_buf()
}

fn contains_long_hex_like(s: &str) -> bool {
    let mut run = 0usize;
    for ch in s.chars() {
        if ch.is_ascii_hexdigit() {
            run += 1;
            if run >= 32 {
                return true;
            }
        } else {
            run = 0;
        }
    }
    false
}

#[test]
fn remote_soak_dry_run_uses_safe_state_root_defaults() {
    let root = repo_root();
    let script = root.join("qsl/qsl-client/qsc/scripts/remote_soak.py");
    assert!(script.exists(), "missing script at {}", script.display());

    let tmp_home = root.join("target").join("test-tmp").join("na0167-home");
    let _ = fs::remove_dir_all(&tmp_home);
    fs::create_dir_all(&tmp_home).expect("create temp HOME");
    fs::set_permissions(&tmp_home, fs::Permissions::from_mode(0o700)).expect("chmod 700 HOME");

    let out = Command::new("python3")
        .arg(&script)
        .args([
            "--relay-url",
            "https://example.invalid",
            "--clients",
            "1",
            "--duration-secs",
            "1",
            "--dry-run",
        ])
        .env("HOME", &tmp_home)
        .current_dir(&root)
        .output()
        .expect("run remote_soak.py");

    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("QSC_SOAK_DRYRUN_OK"),
        "missing dry-run marker: {stdout}"
    );
    assert!(
        stdout.contains("QSC_SOAK_STATE_ROOT_OK"),
        "missing safe state-root marker: {stdout}"
    );
    assert!(
        !stdout.contains("/v1/"),
        "dry-run output leaked relay path: {stdout}"
    );
    assert!(
        !contains_long_hex_like(&stdout),
        "dry-run output leaked token-like hex: {stdout}"
    );
}
