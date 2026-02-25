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
            if run >= 24 {
                return true;
            }
        } else {
            run = 0;
        }
    }
    false
}

#[test]
fn remote_soak_dry_run_marker_and_redaction() {
    let root = repo_root();
    let script = root.join("qsl/qsl-client/qsc/scripts/remote_soak.py");
    assert!(script.exists(), "missing script at {}", script.display());
    let out = Command::new("python3")
        .arg(&script)
        .args([
            "--relay-url",
            "https://relay.example.com",
            "--clients",
            "4",
            "--duration-secs",
            "1",
            "--dry-run",
        ])
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
        stdout.contains("QSC_SOAK_RESULT PASS"),
        "missing pass marker: {stdout}"
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
