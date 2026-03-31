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

fn prepare_dir(path: &PathBuf) {
    let _ = fs::remove_dir_all(path);
    fs::create_dir_all(path).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
    }
}

#[test]
fn plain_marker_contract_na0217a_preserves_bytes() {
    let mut timeout = qsc_cmd();
    timeout.args(["util", "timeout", "--wait-ms", "1", "--timeout-ms", "10"]);
    let timeout_out = timeout.assert().success().get_output().stdout.clone();
    assert_eq!(
        String::from_utf8(timeout_out).unwrap(),
        "QSC_MARK/1 event=timeout_ok elapsed_ms=1\n"
    );

    let cfg = test_root().join("na0217a_plain_marker_contract");
    prepare_dir(&cfg);
    let mut config = qsc_cmd();
    config
        .env("QSC_CONFIG_DIR", &cfg)
        .args(["config", "set", "policy-profile", "baseline"]);
    let config_out = config.assert().success().get_output().stdout.clone();
    assert_eq!(
        String::from_utf8(config_out).unwrap(),
        "QSC_MARK/1 event=config_set key=policy_profile value=<redacted> ok=true\n"
    );
}

#[test]
fn jsonl_marker_contract_na0217a_preserves_shape() {
    let cfg = test_root().join("na0217a_jsonl_marker_contract");
    prepare_dir(&cfg);

    let mut config = qsc_cmd();
    config
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_MARK_FORMAT", "jsonl")
        .args(["config", "set", "policy-profile", "baseline"]);
    let config_out = config.assert().success().get_output().stdout.clone();
    let config_line = String::from_utf8(config_out).unwrap();
    let config_json: serde_json::Value = serde_json::from_str(config_line.trim()).unwrap();
    assert_eq!(config_json["v"], 1);
    assert_eq!(config_json["event"], "config_set");
    assert_eq!(config_json["kv"]["key"], "policy_profile");
    assert_eq!(config_json["kv"]["value"], "<redacted>");
    assert_eq!(config_json["kv"]["ok"], "true");

    let mut timeout = qsc_cmd();
    timeout.env("QSC_MARK_FORMAT", "jsonl").args([
        "util",
        "timeout",
        "--wait-ms",
        "1",
        "--timeout-ms",
        "10",
    ]);
    let timeout_out = timeout.assert().success().get_output().stdout.clone();
    let timeout_line = String::from_utf8(timeout_out).unwrap();
    let timeout_json: serde_json::Value = serde_json::from_str(timeout_line.trim()).unwrap();
    assert_eq!(timeout_json["v"], 1);
    assert_eq!(timeout_json["event"], "timeout_ok");
    assert_eq!(timeout_json["kv"]["elapsed_ms"], "1");
}

#[test]
fn marker_log_contract_na0217a_redacts_secret_like_values() {
    let cfg = test_root().join("na0217a_log_contract");
    prepare_dir(&cfg);
    let log_path = cfg.join("qsc.log");

    let mut config = qsc_cmd();
    config
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_LOG", "1")
        .env("QSC_LOG_PATH", &log_path)
        .args(["config", "set", "policy-profile", "baseline"]);
    config.assert().success();

    let contents = fs::read_to_string(&log_path).unwrap();
    assert!(
        !contents.contains("baseline"),
        "log output must not reveal raw config values: {contents}"
    );
    let log_json: serde_json::Value = serde_json::from_str(contents.trim()).unwrap();
    assert_eq!(log_json["v"], 1);
    assert_eq!(log_json["event"], "config_set");
    assert_eq!(log_json["redacted"], true);
    assert_eq!(log_json["kv"]["key"], "policy_profile");
    assert_eq!(log_json["kv"]["value"], "<redacted>");
    assert_eq!(log_json["kv"]["ok"], "true");
}
