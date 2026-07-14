// NA-0645 (D581): the relay URL-policy matrix, ported off the retired TUI.
//
// The policy under test is core (`adversarial::route::validate_relay_endpoint_url`,
// reached on every relay transport path via `normalize_relay_endpoint`): loopback
// http and any https endpoint are accepted; non-loopback http is rejected with the
// deterministic QSC_ERR_RELAY_TLS_REQUIRED marker; non-http(s) schemes are rejected
// with relay_endpoint_invalid_scheme. Before NA-0645 this matrix was exercised only
// through the TUI (`relay_url_policy.rs`, deleted with the TUI surface).
//
// Vehicle: `qsc receive --transport relay --relay <url>` validates the endpoint
// BEFORE any vault/contact/from handling, so an ACCEPTED endpoint falls through to
// the deterministic `recv_from_required` reject while a REJECTED endpoint dies at
// the policy gate with its own code — accept-vs-reject is observable hermetically,
// with no vault, no network, and no persisted state.
//
// The old TUI test's persisted-endpoint no-mutation half is NOT ported: the
// persisted relay endpoint (`tui.relay.endpoint`) was a TUI-only feature and is
// co-deleted with the TUI; the CLI passes endpoints per-invocation via --relay.

mod common;

use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

fn unique_cfg_dir(tag: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    std::env::current_dir()
        .expect("cwd")
        .join("target")
        .join("qsc-tests")
        .join(format!("{tag}-{}-{nonce}", std::process::id()))
}

fn ensure_dir_700(path: &Path) {
    std::fs::create_dir_all(path).expect("create dir");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o700)).expect("chmod");
    }
}

fn receive_with_relay(cfg: &Path, relay_url: &str) -> String {
    let mut cmd = common::qsc_std_command();
    let out = cmd
        .env("QSC_CONFIG_DIR", cfg)
        .env("QSC_DISABLE_KEYCHAIN", "1")
        .env("NO_COLOR", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args(["receive", "--transport", "relay", "--relay", relay_url])
        .output()
        .expect("run qsc receive");
    let mut combined = String::from_utf8_lossy(&out.stdout).to_string();
    combined.push_str(&String::from_utf8_lossy(&out.stderr));
    combined
}

#[test]
fn relay_url_policy_matrix_accepts_loopback_and_https() {
    let cfg = unique_cfg_dir("na0645_policy_allow");
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    for allowed in [
        "http://localhost:8080",
        "http://127.0.0.1:8080",
        "http://[::1]:8080",
        "https://example.com",
    ] {
        let out = receive_with_relay(&cfg, allowed);
        assert!(
            !out.contains("QSC_ERR_RELAY_TLS_REQUIRED")
                && !out.contains("relay_endpoint_invalid"),
            "allowed endpoint {allowed} must pass the URL policy gate: {out}"
        );
        assert!(
            out.contains("code=recv_from_required"),
            "allowed endpoint {allowed} must fall through to the next deterministic gate: {out}"
        );
    }
}

#[test]
fn relay_url_policy_rejects_non_loopback_http_and_bad_scheme() {
    let cfg = unique_cfg_dir("na0645_policy_reject");
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);

    let out = receive_with_relay(&cfg, "http://example.com");
    assert!(
        out.contains("code=QSC_ERR_RELAY_TLS_REQUIRED"),
        "non-loopback http must reject with the deterministic TLS-required marker: {out}"
    );
    assert!(
        !out.contains("recv_from_required"),
        "rejected endpoint must die at the policy gate, not fall through: {out}"
    );

    let out = receive_with_relay(&cfg, "ftp://example.com");
    assert!(
        out.contains("code=relay_endpoint_invalid_scheme"),
        "non-http(s) scheme must reject deterministically: {out}"
    );
}
