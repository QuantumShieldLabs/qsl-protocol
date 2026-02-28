use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(3)
        .expect("repo root")
        .to_path_buf()
}

fn has_long_hex_run(s: &str) -> bool {
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

fn has_exact_12hex_peer_hash_marker(s: &str) -> bool {
    for line in s.lines() {
        if !line.contains("QSC_SOAK_DIAG_ACTIVE_PEER_HASH") {
            continue;
        }
        if let Some(hash) = line.split("peer_hash=").nth(1) {
            let token = hash.split_whitespace().next().unwrap_or("");
            if token.len() == 12 && token.chars().all(|c| c.is_ascii_hexdigit()) {
                return true;
            }
        }
    }
    false
}

#[cfg(unix)]
fn chmod_700(path: &std::path::Path) {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = std::fs::metadata(path).expect("metadata").permissions();
    perms.set_mode(0o700);
    std::fs::set_permissions(path, perms).expect("chmod 700");
}

fn fresh_test_home(root: &std::path::Path) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let base = root.join("target/test-homes");
    std::fs::create_dir_all(&base).expect("create test home base");
    #[cfg(unix)]
    chmod_700(&base);
    let home = base.join(format!("qsc-soak-mode-home-{}-{nanos}", std::process::id()));
    std::fs::create_dir_all(&home).expect("create test HOME");
    #[cfg(unix)]
    chmod_700(&home);
    home
}

#[test]
fn remote_soak_selftest_default_mode_is_session_only_and_redacted() {
    let root = repo_root();
    let script = root.join("qsl/qsl-client/qsc/scripts/remote_soak.py");
    let home = fresh_test_home(&root);
    let state_root = home.join("state");
    let out = Command::new("python3")
        .arg(&script)
        .args([
            "--selftest",
            "--relay-url",
            "https://example.invalid",
            "--clients",
            "2",
            "--duration-secs",
            "1",
            "--state-root",
            state_root.to_str().expect("state-root utf8"),
        ])
        .env("HOME", &home)
        .current_dir(&root)
        .output()
        .expect("run remote_soak selftest default");

    assert!(
        out.status.success(),
        "status={:?} stderr={} stdout={}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr),
        String::from_utf8_lossy(&out.stdout)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("QSC_SOAK_MODE=session_only"),
        "missing session_only marker: {stdout}"
    );
    assert!(
        stdout.contains(
            "QSC_SOAK_DIAG_SEND_READY stage=hs_a_send_to_b status=ready reason=ready attempts=1"
        ),
        "missing send-ready marker: {stdout}"
    );
    assert!(
        has_exact_12hex_peer_hash_marker(&stdout),
        "missing 12-hex peer hash marker: {stdout}"
    );
    assert!(
        stdout.contains("QSC_SOAK_SELFTEST_OK"),
        "missing selftest marker: {stdout}"
    );
    assert!(!stdout.contains("/v1/"), "leaked URI marker: {stdout}");
    assert!(
        !has_long_hex_run(&stdout),
        "leaked token-like hex: {stdout}"
    );
}

#[test]
fn remote_soak_selftest_seed_fallback_mode_and_redacted() {
    let root = repo_root();
    let script = root.join("qsl/qsl-client/qsc/scripts/remote_soak.py");
    let home = fresh_test_home(&root);
    let state_root = home.join("state");
    let out = Command::new("python3")
        .arg(&script)
        .args([
            "--selftest",
            "--seed-fallback",
            "--relay-url",
            "https://example.invalid",
            "--clients",
            "2",
            "--duration-secs",
            "1",
            "--state-root",
            state_root.to_str().expect("state-root utf8"),
        ])
        .env("HOME", &home)
        .current_dir(&root)
        .output()
        .expect("run remote_soak selftest fallback");

    assert!(
        out.status.success(),
        "status={:?} stderr={} stdout={}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr),
        String::from_utf8_lossy(&out.stdout)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("QSC_SOAK_MODE=seed_fallback"),
        "missing seed_fallback marker: {stdout}"
    );
    assert!(
        stdout.contains(
            "QSC_SOAK_DIAG_SEND_READY stage=hs_a_send_to_b status=ready reason=ready attempts=1"
        ),
        "missing send-ready marker: {stdout}"
    );
    assert!(
        has_exact_12hex_peer_hash_marker(&stdout),
        "missing 12-hex peer hash marker: {stdout}"
    );
    assert!(
        stdout.contains("QSC_SOAK_SELFTEST_OK"),
        "missing selftest marker: {stdout}"
    );
    assert!(!stdout.contains("/v1/"), "leaked URI marker: {stdout}");
    assert!(
        !has_long_hex_run(&stdout),
        "leaked token-like hex: {stdout}"
    );
}
