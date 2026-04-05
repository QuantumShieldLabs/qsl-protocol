use assert_cmd::Command;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

mod common;

const ROUTE_TOKEN_ALICE: &str = "route_token_alice_abcdefghijklmnop";
const ROUTE_TOKEN_BOB: &str = "route_token_bob_abcdefghijklmnopqr";

fn safe_test_root() -> PathBuf {
    let root = if let Ok(v) = env::var("QSC_TEST_ROOT") {
        PathBuf::from(v)
    } else if let Ok(v) = env::var("CARGO_TARGET_DIR") {
        PathBuf::from(v)
    } else {
        PathBuf::from("target")
    };
    let root = root.join("qsc-test-tmp");
    ensure_dir_700(&root);
    root
}

fn ensure_dir_700(path: &Path) {
    if let Ok(meta) = fs::symlink_metadata(path) {
        if meta.is_file() {
            let _ = fs::remove_file(path);
        }
    }
    fs::create_dir_all(path).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
    }
}

fn format_verification_code_from_fingerprint(fingerprint: &str) -> String {
    const CROCKFORD: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";
    let mut chars = fingerprint
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .map(|ch| ch.to_ascii_uppercase())
        .collect::<Vec<char>>();
    while chars.len() < 16 {
        chars.push('0');
    }
    let code = chars.into_iter().take(16).collect::<String>();
    let checksum_idx = code
        .bytes()
        .fold(0u32, |acc, byte| acc.saturating_add(byte as u32))
        % 32;
    let checksum = CROCKFORD[checksum_idx as usize] as char;
    format!(
        "{}-{}-{}-{}-{}",
        &code[0..4],
        &code[4..8],
        &code[8..12],
        &code[12..16],
        checksum
    )
}

fn session_path(cfg: &Path, peer: &str) -> PathBuf {
    cfg.join("qsp_sessions").join(format!("{}.qsv", peer))
}

fn run_qsc(cfg: &Path, args: &[&str]) -> std::process::Output {
    Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg)
        .args(args)
        .output()
        .expect("qsc command")
}

fn init_identity(cfg: &Path, label: &str) {
    let out = run_qsc(cfg, &["identity", "rotate", "--as", label, "--confirm"]);
    assert!(out.status.success(), "{}", output_text(&out));
}

fn output_text(out: &std::process::Output) -> String {
    let mut text = String::from_utf8_lossy(&out.stdout).to_string();
    text.push_str(&String::from_utf8_lossy(&out.stderr));
    text
}

fn identity_fp(cfg: &Path, label: &str) -> String {
    let out = run_qsc(cfg, &["identity", "show", "--as", label]);
    assert!(out.status.success(), "{}", output_text(&out));
    output_text(&out)
        .lines()
        .find_map(|line| line.strip_prefix("identity_fp="))
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| panic!("missing identity_fp in output: {}", output_text(&out)))
}

fn contacts_add_authenticated_with_route(cfg: &Path, label: &str, fp: &str, token: &str) {
    let out = run_qsc(
        cfg,
        &[
            "contacts",
            "add",
            "--label",
            label,
            "--fp",
            fp,
            "--route-token",
            token,
        ],
    );
    assert!(out.status.success(), "{}", output_text(&out));
}

fn contacts_route_set(cfg: &Path, label: &str, token: &str) {
    let out = run_qsc(
        cfg,
        &[
            "contacts",
            "route-set",
            "--label",
            label,
            "--route-token",
            token,
        ],
    );
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stdout)
    );
}

fn relay_inbox_set(cfg: &Path, token: &str) {
    let out = run_qsc(cfg, &["relay", "inbox-set", "--token", token]);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stdout)
    );
}

fn run_qsc_iso(iso: &common::TestIsolation, cfg: &Path, args: &[&str]) -> std::process::Output {
    let mut cmd = std::process::Command::new(assert_cmd::cargo::cargo_bin!("qsc"));
    iso.apply_to(&mut cmd);
    cmd.env("QSC_CONFIG_DIR", cfg)
        .args(args)
        .output()
        .expect("qsc command (isolated)")
}

#[test]
fn first_contact_route_only_rejects_without_silent_tofu_or_state_mutation() {
    let iso = common::TestIsolation::new("na0100_identity_pin");
    let base = iso.root.clone();
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let alice_cfg = base.join("alice");
    let bob_cfg = base.join("bob");
    ensure_dir_700(&alice_cfg);
    ensure_dir_700(&bob_cfg);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);
    contacts_route_set(&alice_cfg, "bob", ROUTE_TOKEN_BOB);
    contacts_route_set(&bob_cfg, "alice", ROUTE_TOKEN_ALICE);
    assert!(run_qsc_iso(
        &iso,
        &alice_cfg,
        &["relay", "inbox-set", "--token", ROUTE_TOKEN_ALICE],
    )
    .status
    .success());
    assert!(run_qsc_iso(
        &iso,
        &bob_cfg,
        &["relay", "inbox-set", "--token", ROUTE_TOKEN_BOB],
    )
    .status
    .success());

    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();

    let out_init = run_qsc_iso(
        &iso,
        &alice_cfg,
        &[
            "handshake",
            "init",
            "--as",
            "alice",
            "--peer",
            "bob",
            "--relay",
            &relay,
        ],
    );
    assert!(!out_init.status.success());
    assert!(
        server.drain_channel(ROUTE_TOKEN_BOB).is_empty(),
        "route-only first contact must not emit A1"
    );
    assert!(!session_path(&alice_cfg, "bob").exists());
    assert!(!session_path(&bob_cfg, "alice").exists());

    let pin_path = bob_cfg.join("identities").join("peer_alice.fp");
    assert!(
        !pin_path.exists(),
        "silent TOFU pin file must not be created"
    );

    let combined = output_text(&out_init);
    assert!(combined.contains("identity_unknown"));
    assert!(combined.contains("handshake_reject"));

    for pat in [
        "TOKEN",
        "SECRET",
        "KEY",
        "PASS",
        "PRIVATE",
        "BEARER",
        "CREDENTIAL",
    ] {
        assert!(!combined.contains(pat));
    }
}

#[test]
fn pinned_mismatch_rejected_no_mutation() {
    let base = safe_test_root().join(format!("na0100_identity_mismatch_{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let alice_cfg = base.join("alice");
    let alice2_cfg = base.join("alice2");
    let bob_cfg = base.join("bob");
    ensure_dir_700(&alice_cfg);
    ensure_dir_700(&alice2_cfg);
    ensure_dir_700(&bob_cfg);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&alice2_cfg);
    common::init_mock_vault(&bob_cfg);
    init_identity(&alice_cfg, "alice");
    init_identity(&alice2_cfg, "alice");
    init_identity(&bob_cfg, "bob");
    let alice_fp = identity_fp(&alice_cfg, "alice");
    let bob_fp = identity_fp(&bob_cfg, "bob");
    contacts_add_authenticated_with_route(&alice_cfg, "bob", bob_fp.as_str(), ROUTE_TOKEN_BOB);
    contacts_add_authenticated_with_route(&alice2_cfg, "bob", bob_fp.as_str(), ROUTE_TOKEN_BOB);
    contacts_add_authenticated_with_route(&bob_cfg, "alice", alice_fp.as_str(), ROUTE_TOKEN_ALICE);
    relay_inbox_set(&alice_cfg, ROUTE_TOKEN_ALICE);
    relay_inbox_set(&alice2_cfg, ROUTE_TOKEN_ALICE);
    relay_inbox_set(&bob_cfg, ROUTE_TOKEN_BOB);

    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();

    let out_init = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &alice_cfg)
        .args([
            "handshake",
            "init",
            "--as",
            "alice",
            "--peer",
            "bob",
            "--relay",
            &relay,
        ])
        .output()
        .expect("handshake init");
    assert!(out_init.status.success());

    let out_bob = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &bob_cfg)
        .args([
            "handshake",
            "poll",
            "--as",
            "bob",
            "--peer",
            "alice",
            "--relay",
            &relay,
            "--max",
            "4",
        ])
        .output()
        .expect("handshake poll bob");
    assert!(out_bob.status.success());

    let out_alice = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &alice_cfg)
        .args([
            "handshake",
            "poll",
            "--as",
            "alice",
            "--peer",
            "bob",
            "--relay",
            &relay,
            "--max",
            "4",
        ])
        .output()
        .expect("handshake poll alice");
    assert!(out_alice.status.success());

    let out_bob_confirm = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &bob_cfg)
        .args([
            "handshake",
            "poll",
            "--as",
            "bob",
            "--peer",
            "alice",
            "--relay",
            &relay,
            "--max",
            "4",
        ])
        .output()
        .expect("handshake poll bob confirm");
    assert!(out_bob_confirm.status.success());

    let session_path = session_path(&bob_cfg, "alice");
    assert!(session_path.exists());
    let session_before = fs::read(&session_path).unwrap();

    let out_init2 = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &alice2_cfg)
        .args([
            "handshake",
            "init",
            "--as",
            "alice",
            "--peer",
            "bob",
            "--relay",
            &relay,
        ])
        .output()
        .expect("handshake init 2");
    assert!(out_init2.status.success());

    let out_bob2 = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &bob_cfg)
        .args([
            "handshake",
            "poll",
            "--as",
            "bob",
            "--peer",
            "alice",
            "--relay",
            &relay,
            "--max",
            "4",
        ])
        .output()
        .expect("handshake poll bob mismatch");
    assert!(out_bob2.status.success());

    let session_after = fs::read(&session_path).unwrap();
    assert_eq!(session_before, session_after);

    let mut combined = String::from_utf8_lossy(&out_init2.stdout).to_string()
        + &String::from_utf8_lossy(&out_init2.stderr);
    combined.push_str(&String::from_utf8_lossy(&out_bob2.stdout));
    combined.push_str(&String::from_utf8_lossy(&out_bob2.stderr));
    assert!(combined.contains("identity_mismatch"), "{}", combined);
    assert!(combined.contains("code=peer_mismatch"), "{}", combined);
    assert!(combined.contains("reason=peer_mismatch"), "{}", combined);

    for pat in [
        "TOKEN",
        "SECRET",
        "KEY",
        "PASS",
        "PRIVATE",
        "BEARER",
        "CREDENTIAL",
    ] {
        assert!(!combined.contains(pat));
    }
}

#[test]
fn handshake_accepts_verification_code_pin_without_peer_mismatch() {
    let iso = common::TestIsolation::new("na0190_identity_verification_code_pin");
    let base = iso.root.clone();
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let alice_cfg = base.join("alice");
    let bob_cfg = base.join("bob");
    ensure_dir_700(&alice_cfg);
    ensure_dir_700(&bob_cfg);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);
    init_identity(&bob_cfg, "bob");
    assert!(run_qsc_iso(
        &iso,
        &alice_cfg,
        &["relay", "inbox-set", "--token", ROUTE_TOKEN_ALICE],
    )
    .status
    .success());
    assert!(run_qsc_iso(
        &iso,
        &bob_cfg,
        &["relay", "inbox-set", "--token", ROUTE_TOKEN_BOB],
    )
    .status
    .success());
    let out_rotate = run_qsc_iso(
        &iso,
        &alice_cfg,
        &["identity", "rotate", "--as", "alice", "--confirm"],
    );
    assert!(out_rotate.status.success());

    let out_show = run_qsc_iso(&iso, &alice_cfg, &["identity", "show", "--as", "alice"]);
    assert!(out_show.status.success());
    let show_text = String::from_utf8_lossy(&out_show.stdout).to_string()
        + &String::from_utf8_lossy(&out_show.stderr);
    let alice_fp = show_text
        .lines()
        .find_map(|line| line.strip_prefix("identity_fp="))
        .expect("alice identity fp line");
    let out_bob_show = run_qsc_iso(&iso, &bob_cfg, &["identity", "show", "--as", "bob"]);
    assert!(out_bob_show.status.success());
    let bob_show_text = output_text(&out_bob_show);
    let bob_fp = bob_show_text
        .lines()
        .find_map(|line| line.strip_prefix("identity_fp="))
        .expect("bob identity fp line");
    let out_pin_bob = run_qsc_iso(
        &iso,
        &alice_cfg,
        &[
            "contacts",
            "add",
            "--label",
            "bob",
            "--fp",
            bob_fp,
            "--route-token",
            ROUTE_TOKEN_BOB,
        ],
    );
    assert!(
        out_pin_bob.status.success(),
        "{}",
        output_text(&out_pin_bob)
    );
    let alice_code = format_verification_code_from_fingerprint(alice_fp);
    let out_add = run_qsc_iso(
        &iso,
        &bob_cfg,
        &[
            "contacts",
            "add",
            "--label",
            "alice",
            "--fp",
            &alice_code,
            "--route-token",
            ROUTE_TOKEN_ALICE,
            "--verify",
        ],
    );
    assert!(out_add.status.success(), "{}", output_text(&out_add));

    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();

    let out_init = run_qsc_iso(
        &iso,
        &alice_cfg,
        &[
            "handshake",
            "init",
            "--as",
            "alice",
            "--peer",
            "bob",
            "--relay",
            &relay,
        ],
    );
    assert!(out_init.status.success());

    let out_bob = run_qsc_iso(
        &iso,
        &bob_cfg,
        &[
            "handshake",
            "poll",
            "--as",
            "bob",
            "--peer",
            "alice",
            "--relay",
            &relay,
            "--max",
            "4",
        ],
    );
    assert!(out_bob.status.success());

    let out_alice = run_qsc_iso(
        &iso,
        &alice_cfg,
        &[
            "handshake",
            "poll",
            "--as",
            "alice",
            "--peer",
            "bob",
            "--relay",
            &relay,
            "--max",
            "4",
        ],
    );
    assert!(out_alice.status.success());

    let out_bob_confirm = run_qsc_iso(
        &iso,
        &bob_cfg,
        &[
            "handshake",
            "poll",
            "--as",
            "bob",
            "--peer",
            "alice",
            "--relay",
            &relay,
            "--max",
            "4",
        ],
    );
    assert!(out_bob_confirm.status.success());

    assert!(session_path(&alice_cfg, "bob").exists());
    assert!(session_path(&bob_cfg, "alice").exists());

    let mut combined = String::from_utf8_lossy(&out_init.stdout).to_string()
        + &String::from_utf8_lossy(&out_init.stderr);
    combined.push_str(&String::from_utf8_lossy(&out_bob.stdout));
    combined.push_str(&String::from_utf8_lossy(&out_bob.stderr));
    combined.push_str(&String::from_utf8_lossy(&out_alice.stdout));
    combined.push_str(&String::from_utf8_lossy(&out_alice.stderr));
    combined.push_str(&String::from_utf8_lossy(&out_bob_confirm.stdout));
    combined.push_str(&String::from_utf8_lossy(&out_bob_confirm.stderr));
    assert!(
        !combined.contains("peer_mismatch"),
        "verification-code pin should match handshake fingerprint: {}",
        combined
    );
    assert!(combined.contains("handshake_complete"), "{}", combined);
}
