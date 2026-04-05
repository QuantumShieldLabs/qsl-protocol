mod common;

use assert_cmd::Command;
use std::fs;
use std::path::{Path, PathBuf};

const ROUTE_TOKEN_ALICE: &str = "route_token_alice_abcdefghijklmnop";
const ROUTE_TOKEN_BOB: &str = "route_token_bob_abcdefghijklmnopqr";

fn safe_test_root() -> PathBuf {
    let root = std::env::temp_dir().join("qsc-test-tmp");
    ensure_dir_700(&root);
    root
}

fn ensure_dir_700(path: &Path) {
    let _ = fs::create_dir_all(path);
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(path, fs::Permissions::from_mode(0o700));
    }
}

fn create_dir_700(path: &Path) {
    let _ = fs::remove_dir_all(path);
    ensure_dir_700(path);
}

fn output_text(out: &std::process::Output) -> String {
    let mut text = String::from_utf8_lossy(&out.stdout).to_string();
    text.push_str(&String::from_utf8_lossy(&out.stderr));
    text
}

fn run_qsc(cfg: &Path, args: &[&str]) -> std::process::Output {
    Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .args(args)
        .output()
        .expect("qsc command")
}

fn init_identity(cfg: &Path, label: &str) {
    let out = run_qsc(cfg, &["identity", "rotate", "--as", label, "--confirm"]);
    assert!(out.status.success(), "{}", output_text(&out));
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

fn seed_authenticated_pair(alice_cfg: &Path, bob_cfg: &Path) {
    init_identity(alice_cfg, "alice");
    init_identity(bob_cfg, "bob");
    let alice_fp = identity_fp(alice_cfg, "alice");
    let bob_fp = identity_fp(bob_cfg, "bob");
    contacts_add_authenticated_with_route(alice_cfg, "bob", bob_fp.as_str(), ROUTE_TOKEN_BOB);
    contacts_add_authenticated_with_route(bob_cfg, "alice", alice_fp.as_str(), ROUTE_TOKEN_ALICE);
}

fn relay_inbox_set(cfg: &Path, token: &str) {
    let out = run_qsc(cfg, &["relay", "inbox-set", "--token", token]);
    assert!(out.status.success(), "{}", output_text(&out));
}

#[test]
fn handshake_status_tracks_establishment_after_full_exchange() {
    let base = safe_test_root().join(format!("na0217i_handshake_status_{}", std::process::id()));
    create_dir_700(&base);
    let alice_cfg = base.join("alice");
    let bob_cfg = base.join("bob");
    create_dir_700(&alice_cfg);
    create_dir_700(&bob_cfg);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);
    seed_authenticated_pair(&alice_cfg, &bob_cfg);
    relay_inbox_set(&alice_cfg, ROUTE_TOKEN_ALICE);
    relay_inbox_set(&bob_cfg, ROUTE_TOKEN_BOB);

    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();

    let initial = run_qsc(&alice_cfg, &["handshake", "status", "--peer", "bob"]);
    assert!(initial.status.success(), "{}", output_text(&initial));
    let initial_text = output_text(&initial);
    assert!(
        initial_text.contains("event=handshake_status status=no_session peer=bob"),
        "{}",
        initial_text
    );

    let alice_init = run_qsc(
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
    assert!(alice_init.status.success(), "{}", output_text(&alice_init));

    let bob_poll = run_qsc(
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
    assert!(bob_poll.status.success(), "{}", output_text(&bob_poll));

    let alice_poll = run_qsc(
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
    assert!(alice_poll.status.success(), "{}", output_text(&alice_poll));

    let alice_mid_status = run_qsc(&alice_cfg, &["handshake", "status", "--peer", "bob"]);
    assert!(
        alice_mid_status.status.success(),
        "{}",
        output_text(&alice_mid_status)
    );
    let alice_mid_status_text = output_text(&alice_mid_status);
    assert!(
        alice_mid_status_text
            .contains("event=handshake_status status=awaiting_peer_confirm peer=bob"),
        "{}",
        alice_mid_status_text
    );
    assert!(
        alice_mid_status_text.contains("peer_confirmed=no"),
        "{}",
        alice_mid_status_text
    );
    assert!(
        alice_mid_status_text.contains("send_ready=yes"),
        "{}",
        alice_mid_status_text
    );

    let bob_confirm = run_qsc(
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
    assert!(
        bob_confirm.status.success(),
        "{}",
        output_text(&bob_confirm)
    );

    let alice_status = run_qsc(&alice_cfg, &["handshake", "status", "--peer", "bob"]);
    assert!(
        alice_status.status.success(),
        "{}",
        output_text(&alice_status)
    );
    let alice_status_text = output_text(&alice_status);
    assert!(
        alice_status_text.contains("event=handshake_status status=awaiting_peer_confirm peer=bob"),
        "{}",
        alice_status_text
    );
    assert!(
        alice_status_text.contains("peer_confirmed=no"),
        "{}",
        alice_status_text
    );
    assert!(
        alice_status_text.contains("send_ready=yes"),
        "{}",
        alice_status_text
    );

    let bob_status = run_qsc(&bob_cfg, &["handshake", "status", "--peer", "alice"]);
    assert!(bob_status.status.success(), "{}", output_text(&bob_status));
    let bob_status_text = output_text(&bob_status);
    assert!(
        bob_status_text.contains("event=handshake_status status=established_recv_only peer=alice"),
        "{}",
        bob_status_text
    );
    assert!(
        bob_status_text.contains("peer_confirmed=yes"),
        "{}",
        bob_status_text
    );
    assert!(
        bob_status_text.contains("send_ready=no"),
        "{}",
        bob_status_text
    );
    assert!(
        bob_status_text.contains("send_ready_reason=chainkey_unset"),
        "{}",
        bob_status_text
    );
}
