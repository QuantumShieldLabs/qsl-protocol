use predicates::prelude::*;
use predicates::str::contains;
use std::fs;
use std::path::PathBuf;

fn test_root() -> PathBuf {
    if let Ok(v) = std::env::var("QSC_TEST_ROOT") {
        return PathBuf::from(v);
    }
    if let Ok(v) = std::env::var("CARGO_TARGET_DIR") {
        return PathBuf::from(v).join("qsc-test-tmp");
    }
    PathBuf::from("target").join("qsc-test-tmp")
}

fn qsc_cmd() -> assert_cmd::Command {
    assert_cmd::cargo::cargo_bin_cmd!("qsc")
}

#[test]
fn diagnostics_no_secrets() {
    let base = test_root().join("na0064_doctor_no_secrets");
    let _ = fs::remove_dir_all(&base);
    fs::create_dir_all(&base).unwrap();

    let secret = "super-secret-passphrase";
    let mut cmd = qsc_cmd();
    cmd.env("QSC_PASSPHRASE", secret)
        .env("QSC_CONFIG_DIR", &base)
        .args(["doctor", "--check-only"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(secret).not());
}

#[test]
fn default_output_no_endpoint_or_time() {
    let mut cmd = qsc_cmd();
    cmd.args([
        "util",
        "sanitize",
        "--print",
        "https://example.com/2026-01-01T00:00:00Z",
    ]);
    cmd.assert()
        .success()
        .stdout(contains("https://").not())
        .stdout(contains("2026-01-01T00:00:00Z").not())
        .stdout(contains("<redacted>"));
}

#[test]
fn redact_is_enforced() {
    let base = test_root().join("na0065_redact");
    let _ = fs::remove_dir_all(&base);
    fs::create_dir_all(&base).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&base, fs::Permissions::from_mode(0o700)).unwrap();
    }

    let mut cmd = qsc_cmd();
    cmd.env("QSC_CONFIG_DIR", &base)
        .args(["config", "set", "policy-profile", "baseline"]);
    cmd.assert()
        .success()
        .stdout(contains("value=<redacted>"))
        .stdout(contains("value=baseline").not());

    let mut cmd = qsc_cmd();
    cmd.env("QSC_CONFIG_DIR", &base)
        .args(["config", "get", "policy-profile"]);
    cmd.assert()
        .success()
        .stdout(contains("value=<redacted>"))
        .stdout(contains("value=baseline").not());
}

#[test]
fn markers_schema_stable_jsonl() {
    let mut cmd = qsc_cmd();
    cmd.env("QSC_MARK_FORMAT", "jsonl").arg("status");
    let out = cmd.assert().success().get_output().stdout.clone();
    let line = String::from_utf8_lossy(&out);
    let v: serde_json::Value = serde_json::from_str(line.trim_end()).unwrap();
    assert_eq!(v["v"], 1);
    assert_eq!(v["event"], "status");
}

#[test]
fn logs_off_by_default() {
    let base = test_root().join("na0064_logs_off");
    let _ = fs::remove_dir_all(&base);
    fs::create_dir_all(&base).unwrap();

    let log_path = base.join("qsc.log");
    let _ = fs::remove_file(&log_path);

    let mut cmd = qsc_cmd();
    cmd.env("QSC_LOG_PATH", &log_path).arg("status");
    cmd.assert().success();

    assert!(
        !log_path.exists(),
        "log file must not be created by default"
    );
}

#[test]
fn doctor_export_redacted() {
    let base = test_root().join("na0064_doctor_export");
    let _ = fs::remove_dir_all(&base);
    fs::create_dir_all(&base).unwrap();

    let export = base.join("doctor.json");
    let secret = "top-secret-value";

    let mut cmd = qsc_cmd();
    cmd.env("QSC_PASSPHRASE", secret)
        .env("QSC_CONFIG_DIR", &base)
        .args([
            "doctor",
            "--check-only",
            "--export",
            export.to_str().unwrap(),
        ]);

    cmd.assert().success();

    let contents = fs::read_to_string(&export).unwrap();
    assert!(contents.contains("\"redacted\":true"));
    assert!(contents.contains("<redacted>"));
    assert!(!contents.contains(secret));
}
