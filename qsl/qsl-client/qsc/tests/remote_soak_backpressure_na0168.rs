use std::path::PathBuf;
use std::process::Command;
use std::{fs, os::unix::fs::PermissionsExt};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(3)
        .unwrap_or_else(|| panic!("repo root"))
        .to_path_buf()
}

fn has_long_hex_run(s: &str) -> bool {
    let mut run = 0usize;
    for ch in s.chars() {
        if ch.is_ascii_digit() || (ch.is_ascii_hexdigit() && ch.is_ascii_lowercase()) {
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
fn remote_soak_dry_run_simulated_overload_retries_are_counted_and_redacted() {
    let root = repo_root();
    let script = root.join("qsl/qsl-client/qsc/scripts/remote_soak.py");
    assert!(script.exists(), "missing script at {}", script.display());
    let home = root.join("target").join("test-tmp").join("na0168-home");
    let workdir = root.join("target").join("test-tmp").join("na0168-workdir");
    let _ = fs::remove_dir_all(&home);
    let _ = fs::remove_dir_all(&workdir);
    fs::create_dir_all(&home).unwrap_or_else(|e| panic!("{e}"));
    fs::create_dir_all(&workdir).unwrap_or_else(|e| panic!("{e}"));
    fs::set_permissions(&home, fs::Permissions::from_mode(0o700)).unwrap_or_else(|e| panic!("{e}"));

    let out = Command::new("python3")
        .arg(script)
        .args([
            "--dry-run",
            "--relay-url",
            "https://example.invalid",
            "--clients",
            "2",
            "--duration-secs",
            "1",
            "--simulate-overload-attempts",
            "2",
            "--max-retries",
            "5",
            "--no-sleep",
            "--workdir",
            workdir.to_str().unwrap_or_else(|| panic!("workdir utf8")),
        ])
        .env("HOME", &home)
        .current_dir(&root)
        .output()
        .expect("run remote_soak.py");

    assert!(
        out.status.success(),
        "dry-run simulated overload should pass: status={:?} stderr={}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("QSC_SOAK_DRYRUN_OK"),
        "missing dry-run marker: {stdout}"
    );
    assert!(
        stdout.contains("QSC_SOAK_RESULT PASS"),
        "missing PASS marker: {stdout}"
    );
    assert!(
        !stdout.contains("/v1/"),
        "output leaked relay path: {stdout}"
    );
    let has_long_hex = has_long_hex_run(&stdout);
    assert!(!has_long_hex, "output leaked token-like hex: {stdout}");

    let summary_path = workdir.join("summary.json");
    let summary = fs::read_to_string(summary_path).unwrap_or_else(|e| panic!("{e}"));
    let parsed: serde_json::Value =
        serde_json::from_str(&summary).unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(parsed["overload_retries"], 2);
    assert_eq!(parsed["overload_failures"], 0);
}
