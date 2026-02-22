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

fn contacts_add_with_route(cfg: &Path, label: &str, token: &str) {
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
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}

fn relay_inbox_set(cfg: &Path, token: &str) {
    let out = run_qsc(cfg, &["relay", "inbox-set", "--token", token]);
    assert!(
        out.status.success(),
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn handshake_rejects_tampered_transcript_no_mutation() {
    let base = safe_test_root().join(format!("na0154_hs_tamper_{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let alice_cfg = base.join("alice");
    let bob_cfg = base.join("bob");
    ensure_dir_700(&alice_cfg);
    ensure_dir_700(&bob_cfg);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);
    contacts_add_with_route(&alice_cfg, "bob", ROUTE_TOKEN_BOB);
    contacts_add_with_route(&bob_cfg, "alice", ROUTE_TOKEN_ALICE);
    relay_inbox_set(&alice_cfg, ROUTE_TOKEN_ALICE);
    relay_inbox_set(&bob_cfg, ROUTE_TOKEN_BOB);

    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();

    let out_init = run_qsc(
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

    let out_bob = run_qsc(
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

    // Tamper B1 transcript/MAC bytes before Alice consumes it.
    let mut items = server.drain_channel(ROUTE_TOKEN_ALICE);
    assert_eq!(items.len(), 1);
    if let Some(b) = items[0].get_mut(8) {
        *b = b.wrapping_add(1);
    }
    server.replace_channel(ROUTE_TOKEN_ALICE, items);

    let out_alice = run_qsc(
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
    assert!(!session_path(&alice_cfg, "bob").exists());

    let combined = String::from_utf8_lossy(&out_alice.stdout).to_string()
        + &String::from_utf8_lossy(&out_alice.stderr);
    assert!(combined.contains("handshake_reject"), "{}", combined);
}

#[test]
fn handshake_pinned_identity_mismatch_fails() {
    let base = safe_test_root().join(format!("na0154_hs_pin_mismatch_{}", std::process::id()));
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
    contacts_add_with_route(&alice_cfg, "bob", ROUTE_TOKEN_BOB);
    contacts_add_with_route(&alice2_cfg, "bob", ROUTE_TOKEN_BOB);
    contacts_add_with_route(&bob_cfg, "alice", ROUTE_TOKEN_ALICE);
    relay_inbox_set(&alice_cfg, ROUTE_TOKEN_ALICE);
    relay_inbox_set(&alice2_cfg, ROUTE_TOKEN_ALICE);
    relay_inbox_set(&bob_cfg, ROUTE_TOKEN_BOB);

    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();

    // Establish first session and pin Alice at Bob.
    assert!(run_qsc(
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
    )
    .status
    .success());
    assert!(run_qsc(
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
    )
    .status
    .success());
    assert!(run_qsc(
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
    )
    .status
    .success());
    assert!(run_qsc(
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
    )
    .status
    .success());
    let session_before = fs::read(session_path(&bob_cfg, "alice")).expect("session before");

    let out_show = run_qsc(&alice_cfg, &["identity", "show", "--as", "alice"]);
    assert!(out_show.status.success());
    let show_text = String::from_utf8_lossy(&out_show.stdout).to_string()
        + &String::from_utf8_lossy(&out_show.stderr);
    let alice_fp = show_text
        .lines()
        .find_map(|line| line.strip_prefix("identity_fp="))
        .expect("identity fp");
    let out_pin = run_qsc(
        &bob_cfg,
        &[
            "contacts", "add", "--label", "alice", "--fp", alice_fp, "--verify",
        ],
    );
    assert!(out_pin.status.success());

    // New Alice identity must be rejected against Bob's pin.
    let out_init2 = run_qsc(
        &alice2_cfg,
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
    assert!(out_init2.status.success());
    let out_bob2 = run_qsc(
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
    assert!(out_bob2.status.success());

    let session_after = fs::read(session_path(&bob_cfg, "alice")).expect("session after");
    assert_eq!(session_before, session_after);

    let combined = String::from_utf8_lossy(&out_bob2.stdout).to_string()
        + &String::from_utf8_lossy(&out_bob2.stderr);
    assert!(combined.contains("peer_mismatch"), "{}", combined);
}
