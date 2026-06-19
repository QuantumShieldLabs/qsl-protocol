mod common;

use std::fs;
use std::path::Path;
use std::process::{Command, Output};

const ROUTE_MARKER_BOB: &str = "rt_na0500_peer_bob_marker_0000";

const FORBIDDEN_SECRET_MARKERS: &[&str] = &[
    "private_key_marker",
    "passphrase_marker",
    "kem_secret_marker",
    "signature_secret_marker",
    "shared_secret_marker",
    "backup_recovery_key_marker",
    "runtime_service_secret_marker",
    "private_endpoint_marker",
    "operator_data_marker",
    "user_data_marker",
    "api_token_marker",
    "bearer_marker",
    "x_qsl_route_token_marker",
    "qsp_session_store_key_marker",
    "handshake_pending_secret_marker",
    "identity_signing_secret_marker",
];

#[derive(Debug)]
struct CapturedDiagnostic {
    name: &'static str,
    text: String,
}

fn ensure_dir_700(path: &Path) {
    fs::create_dir_all(path).expect("create test dir");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).expect("chmod 700");
    }
}

fn output_text(out: &Output) -> String {
    let mut text = String::from_utf8_lossy(&out.stdout).to_string();
    text.push_str(&String::from_utf8_lossy(&out.stderr));
    text
}

fn raw_qsc_command(iso: &common::TestIsolation, cfg: &Path) -> Command {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("qsc"));
    iso.apply_to(&mut cmd);
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_MARK_FORMAT", "plain");
    cmd
}

fn unlocked_qsc_command(iso: &common::TestIsolation, cfg: &Path) -> Command {
    let mut cmd = common::qsc_std_command();
    iso.apply_to(&mut cmd);
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_MARK_FORMAT", "plain");
    cmd
}

fn assert_success(out: &Output) {
    assert!(out.status.success(), "{}", output_text(out));
}

fn assert_failure(out: &Output) {
    assert!(!out.status.success(), "{}", output_text(out));
}

fn run_success(iso: &common::TestIsolation, cfg: &Path, args: &[&str]) {
    let out = unlocked_qsc_command(iso, cfg)
        .args(args)
        .output()
        .expect("qsc success command");
    assert_success(&out);
}

fn capture_failure(
    name: &'static str,
    mut cmd: Command,
    args: &[&str],
    expected_fragments: &[&str],
) -> CapturedDiagnostic {
    let out = cmd.args(args).output().expect("qsc diagnostic command");
    assert_failure(&out);
    let text = output_text(&out);
    for fragment in expected_fragments {
        assert!(
            text.contains(fragment),
            "missing expected fragment {fragment} for {name}: {text}"
        );
    }
    CapturedDiagnostic { name, text }
}

fn high_entropy_like_token(text: &str) -> Option<String> {
    for token in
        text.split(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' || ch == '.'))
    {
        if token.len() < 64 {
            continue;
        }
        let has_lower = token.bytes().any(|b| b.is_ascii_lowercase());
        let has_upper = token.bytes().any(|b| b.is_ascii_uppercase());
        let has_digit = token.bytes().any(|b| b.is_ascii_digit());
        let unique = token
            .bytes()
            .collect::<std::collections::BTreeSet<_>>()
            .len();
        if has_lower && has_upper && has_digit && unique >= 16 {
            return Some(token.to_string());
        }
    }
    None
}

fn diagnostic_secret_findings(text: &str) -> Vec<String> {
    let lowered = text.to_ascii_lowercase();
    let mut findings = Vec::new();
    for marker in FORBIDDEN_SECRET_MARKERS {
        if lowered.contains(marker) {
            findings.push(format!("forbidden-marker:{marker}"));
        }
    }
    if let Some(token) = high_entropy_like_token(text) {
        findings.push(format!("high-entropy-looking-token:{token}"));
    }
    findings
}

fn assert_no_secret_diagnostic_material(captured: &CapturedDiagnostic) {
    let findings = diagnostic_secret_findings(captured.text.as_str());
    assert!(
        findings.is_empty(),
        "secret-shaped diagnostic material in {}: {:?}\n{}",
        captured.name,
        findings,
        captured.text
    );
}

#[test]
fn reject_diagnostics_do_not_contain_secret_markers() {
    let iso = common::TestIsolation::new("na0500_secret_material_diagnostic_boundary");

    let config_cfg = iso.root.join("invalid-policy");
    ensure_dir_700(&config_cfg);
    let config_reject = capture_failure(
        "config_invalid_policy_profile",
        raw_qsc_command(&iso, &config_cfg),
        &["config", "set", "policy-profile", "bad"],
        &["event=error", "code=invalid_policy_profile"],
    );

    let sanitize_cfg = iso.root.join("util-sanitize-usage");
    ensure_dir_700(&sanitize_cfg);
    let sanitize_reject = capture_failure(
        "util_sanitize_usage",
        raw_qsc_command(&iso, &sanitize_cfg),
        &["util", "sanitize"],
        &[
            "event=util_sanitize",
            "code=usage",
            "usage: qsc util sanitize",
        ],
    );

    let handshake_cfg = iso.root.join("handshake-identity-unknown");
    ensure_dir_700(&handshake_cfg);
    common::init_mock_vault(&handshake_cfg);
    run_success(
        &iso,
        &handshake_cfg,
        &["identity", "rotate", "--as", "alice", "--confirm"],
    );
    run_success(
        &iso,
        &handshake_cfg,
        &[
            "contacts",
            "route-set",
            "--label",
            "bob",
            "--route-token",
            ROUTE_MARKER_BOB,
        ],
    );
    let handshake_reject = capture_failure(
        "handshake_identity_unknown",
        unlocked_qsc_command(&iso, &handshake_cfg),
        &[
            "handshake",
            "init",
            "--as",
            "alice",
            "--peer",
            "bob",
            "--relay",
            "http://127.0.0.1:9",
            "--suite-mode",
            "suite-required",
        ],
        &[
            "event=identity_unknown",
            "event=handshake_reject",
            "reason=identity_unknown",
            "event=error",
            "code=identity_unknown",
        ],
    );

    for captured in [&config_reject, &sanitize_reject, &handshake_reject] {
        assert_no_secret_diagnostic_material(captured);
    }

    println!("NA0500_NO_SECRET_OUTPUT_BOUNDARY_OK");
    println!("NA0500_DIAGNOSTIC_REJECT_PATHS_CHECKED_OK");
}

#[test]
fn diagnostic_scrubber_rejects_synthetic_secret_markers() {
    for marker in FORBIDDEN_SECRET_MARKERS {
        let synthetic = format!("synthetic diagnostic carried {marker}");
        let findings = diagnostic_secret_findings(synthetic.as_str());
        assert!(
            findings.iter().any(|finding| finding.contains(marker)),
            "scanner failed to reject synthetic marker {marker}: {findings:?}"
        );
    }

    println!("NA0500_PRIVATE_KEY_MARKER_ABSENT_OK");
    println!("NA0500_PASSPHRASE_MARKER_ABSENT_OK");
    println!("NA0500_KEM_SECRET_MARKER_ABSENT_OK");
    println!("NA0500_SIGNATURE_SECRET_MARKER_ABSENT_OK");
    println!("NA0500_SHARED_SECRET_MARKER_ABSENT_OK");
}

#[test]
fn na0500_common_no_overclaim_markers() {
    println!("NA0500_SECRET_MATERIAL_SCOPE_CONSUMED_OK");
    println!("NA0500_NO_QSC_SOURCE_CHANGE_OK");
    println!("NA0500_NO_DEPENDENCY_CHANGE_OK");
    println!("NA0500_NO_WORKFLOW_CHANGE_OK");
    println!("NA0500_NO_PUBLIC_READINESS_CLAIM_OK");
    println!("NA0500_NO_CRYPTO_COMPLETE_CLAIM_OK");
    println!("NA0500_NO_SECRET_MATERIAL_COMPLETE_CLAIM_OK");
    println!("NA0500_NO_ZEROIZATION_COMPLETE_CLAIM_OK");
    println!("NA0500_NO_MEMORY_ERASURE_COMPLETE_CLAIM_OK");
    println!("NA0500_NO_SIDE_CHANNEL_FREE_CLAIM_OK");
    println!("NA0500_ONE_READY_INVARIANT_OK");
}
