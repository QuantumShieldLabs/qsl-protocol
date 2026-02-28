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

fn token_hashes_are_12_hex(stdout: &str) -> bool {
    let mut saw_any = false;
    for line in stdout.lines() {
        if !line.contains("QSC_SOAK_DIAG_") || !line.contains(" hash=") {
            continue;
        }
        saw_any = true;
        let Some(value) = line.split(" hash=").nth(1) else {
            return false;
        };
        let hash = value.trim();
        if hash.len() != 12 || !hash.chars().all(|c| c.is_ascii_hexdigit()) {
            return false;
        }
    }
    saw_any
}

#[test]
fn remote_soak_diag_dry_run_mapping_and_substages_are_present_and_redacted() {
    let root = repo_root();
    let script = root.join("qsl/qsl-client/qsc/scripts/remote_soak.py");
    assert!(script.exists(), "missing script at {}", script.display());

    let tmp_home = root
        .join("target")
        .join("test-tmp")
        .join("na0168-diag-mapping-home");
    let _ = fs::remove_dir_all(&tmp_home);
    fs::create_dir_all(&tmp_home).expect("create temp HOME");
    fs::set_permissions(&tmp_home, fs::Permissions::from_mode(0o700)).expect("chmod 700 HOME");

    let out = Command::new("python3")
        .arg(&script)
        .args([
            "--diag",
            "--dry-run",
            "--relay-url",
            "https://example.invalid",
            "--clients",
            "2",
            "--duration-secs",
            "1",
        ])
        .env("HOME", &tmp_home)
        .current_dir(&root)
        .output()
        .expect("run remote_soak.py");

    assert!(
        out.status.success(),
        "status={:?} stderr={} stdout={}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr),
        String::from_utf8_lossy(&out.stdout)
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    for marker in [
        "QSC_SOAK_DIAG_OK",
        "QSC_SOAK_DIAG_MAPPING_OK match=yes",
        "QSC_SOAK_DIAG_SEND_READY stage=hs_a_send_to_b status=ready reason=ready attempts=1",
        "QSC_SOAK_STAGE stage=drain status=ok",
        "QSC_SOAK_STAGE stage=hs_pair_handshake_start status=ok",
        "QSC_SOAK_STAGE stage=hs_pair_handshake_a_init status=ok",
        "QSC_SOAK_STAGE stage=hs_pair_handshake_b_process status=ok",
        "QSC_SOAK_STAGE stage=hs_pair_handshake_a_finalize status=ok",
        "QSC_SOAK_STAGE stage=hs_pair_handshake_ok status=ok",
        "QSC_SOAK_STAGE stage=hs_a_send_to_b status=ok",
        "QSC_SOAK_STAGE stage=hs_b_recv_from_a status=ok",
        "QSC_SOAK_STAGE stage=hs_b_send_to_a status=ok",
        "QSC_SOAK_STAGE stage=hs_a_recv_from_b status=ok",
    ] {
        assert!(stdout.contains(marker), "missing marker {marker}: {stdout}");
    }

    let idx_setup = stdout.find("QSC_SOAK_STAGE stage=diag_setup status=ok");
    let idx_drain = stdout.find("QSC_SOAK_STAGE stage=drain status=ok");
    let idx_hs_start = stdout.find("QSC_SOAK_STAGE stage=hs_pair_handshake_start status=ok");
    let idx_hs_ok = stdout.find("QSC_SOAK_STAGE stage=hs_pair_handshake_ok status=ok");
    assert!(
        idx_setup.is_some() && idx_drain.is_some() && idx_hs_start.is_some() && idx_hs_ok.is_some(),
        "missing stage ordering anchors: {stdout}"
    );
    assert!(
        idx_setup.unwrap_or(usize::MAX) < idx_drain.unwrap_or(0)
            && idx_drain.unwrap_or(usize::MAX) < idx_hs_start.unwrap_or(0)
            && idx_hs_start.unwrap_or(usize::MAX) < idx_hs_ok.unwrap_or(0),
        "unexpected stage order: {stdout}"
    );

    assert!(
        !stdout.contains("/v1/"),
        "diag output leaked relay path: {stdout}"
    );
    assert!(
        !has_long_hex_run(&stdout),
        "diag output leaked token-like hex: {stdout}"
    );
    assert!(
        token_hashes_are_12_hex(&stdout),
        "diag output missing 12-hex token hashes: {stdout}"
    );
}
