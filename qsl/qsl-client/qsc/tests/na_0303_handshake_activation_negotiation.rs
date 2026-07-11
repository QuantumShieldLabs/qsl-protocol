mod common;

use std::fs;
use std::path::{Path, PathBuf};

const ROUTE_TOKEN_ALICE: &str = "route_token_alice_na0303_abcdefghijkl";
const ROUTE_TOKEN_BOB: &str = "route_token_bob_na0303_abcdefghijklmn";
const MALFORMED_SENTINEL: &str = "NA0303_HANDSHAKE_MALFORMED_SENTINEL_DO_NOT_ECHO";

fn safe_test_root() -> PathBuf {
    let root = if let Ok(v) = std::env::var("QSC_TEST_ROOT") {
        PathBuf::from(v)
    } else if let Ok(v) = std::env::var("CARGO_TARGET_DIR") {
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
    let mut cmd = common::qsc_std_command();
    cmd.env("QSC_CONFIG_DIR", cfg)
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

fn identity_kem_pk(cfg: &Path, label: &str) -> String {
    let out = run_qsc(cfg, &["identity", "show", "--as", label]);
    assert!(out.status.success(), "{}", output_text(&out));
    output_text(&out)
        .lines()
        .find_map(|line| line.strip_prefix("identity_kem_pk="))
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| panic!("missing identity_kem_pk in output: {}", output_text(&out)))
}

fn contacts_add_authenticated_with_route(cfg: &Path, label: &str, fp: &str, kem_pk: &str, token: &str) {
    let out = run_qsc(
        cfg,
        &[
            "contacts",
            "add",
            "--label",
            label,
            "--fp",
            fp,
            "--kem-pk",
            kem_pk,
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
    assert!(out.status.success(), "{}", output_text(&out));
}

fn relay_inbox_set(cfg: &Path, token: &str) {
    let out = run_qsc(cfg, &["relay", "inbox-set", "--token", token]);
    assert!(out.status.success(), "{}", output_text(&out));
}

fn seed_authenticated_pair(alice_cfg: &Path, bob_cfg: &Path) {
    init_identity(alice_cfg, "alice");
    init_identity(bob_cfg, "bob");
    let alice_fp = identity_fp(alice_cfg, "alice");
    let alice_kem = identity_kem_pk(alice_cfg, "alice");
    let bob_fp = identity_fp(bob_cfg, "bob");
    let bob_kem = identity_kem_pk(bob_cfg, "bob");
    contacts_add_authenticated_with_route(alice_cfg, "bob", bob_fp.as_str(), bob_kem.as_str(), ROUTE_TOKEN_BOB);
    contacts_add_authenticated_with_route(bob_cfg, "alice", alice_fp.as_str(), alice_kem.as_str(), ROUTE_TOKEN_ALICE);
    relay_inbox_set(alice_cfg, ROUTE_TOKEN_ALICE);
    relay_inbox_set(bob_cfg, ROUTE_TOKEN_BOB);
}

fn session_path(cfg: &Path, peer: &str) -> PathBuf {
    cfg.join("qsp_sessions").join(format!("{peer}.qsv"))
}

fn vault_path(cfg: &Path) -> PathBuf {
    cfg.join("vault.qsv")
}

fn assert_no_leak_or_panic(text: &str) {
    for forbidden in [
        ROUTE_TOKEN_ALICE,
        ROUTE_TOKEN_BOB,
        MALFORMED_SENTINEL,
        "panicked",
        "stack backtrace",
        "thread '",
        "QSC_DESKTOP_SESSION_PASSPHRASE",
    ] {
        assert!(
            !text.contains(forbidden),
            "forbidden output fragment leaked: {forbidden}: {text}"
        );
    }
}

fn assert_reject_no_commit(text: &str) {
    assert!(
        text.contains("event=handshake_reject"),
        "missing handshake reject marker: {text}"
    );
    assert!(
        !text.contains("event=handshake_complete"),
        "reject completed handshake: {text}"
    );
    assert!(
        !text.contains("event=recv_commit"),
        "recv commit on reject: {text}"
    );
    assert!(
        !text.contains("event=qsp_unpack ok=true"),
        "qsp output on reject: {text}"
    );
    assert_no_leak_or_panic(text);
}

fn mutate_handshake_version(mut msg: Vec<u8>, version: u16) -> Vec<u8> {
    assert!(msg.len() > 6, "handshake frame too short");
    msg[4..6].copy_from_slice(&version.to_be_bytes());
    msg
}

fn poll_bob(bob_cfg: &Path, relay: &str) -> std::process::Output {
    run_qsc(
        bob_cfg,
        &[
            "handshake",
            "poll",
            "--as",
            "bob",
            "--peer",
            "alice",
            "--relay",
            relay,
            "--max",
            "4",
        ],
    )
}

fn poll_alice(alice_cfg: &Path, relay: &str) -> std::process::Output {
    run_qsc(
        alice_cfg,
        &[
            "handshake",
            "poll",
            "--as",
            "alice",
            "--peer",
            "bob",
            "--relay",
            relay,
            "--max",
            "4",
        ],
    )
}

fn init_alice(alice_cfg: &Path, relay: &str) -> std::process::Output {
    run_qsc(
        alice_cfg,
        &[
            "handshake",
            "init",
            "--as",
            "alice",
            "--peer",
            "bob",
            "--relay",
            relay,
        ],
    )
}

fn new_pair(root: &Path, tag: &str) -> (PathBuf, PathBuf) {
    let alice_cfg = root.join(format!("{tag}-alice"));
    let bob_cfg = root.join(format!("{tag}-bob"));
    create_dir_700(&alice_cfg);
    create_dir_700(&bob_cfg);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);
    seed_authenticated_pair(&alice_cfg, &bob_cfg);
    (alice_cfg, bob_cfg)
}

#[test]
fn qsc_handshake_activation_admission_rejects_are_fail_closed() {
    let server = common::start_inbox_server(1024 * 1024, 32);
    let base = safe_test_root().join(format!("na0303_hs_activation_{}", std::process::id()));
    create_dir_700(&base);
    let relay = server.base_url().to_string();

    let (alice_valid, bob_valid) = new_pair(&base, "valid");
    let alice_init = init_alice(&alice_valid, &relay);
    assert!(alice_init.status.success(), "{}", output_text(&alice_init));
    assert!(output_text(&alice_init).contains("event=handshake_send msg=A1"));
    let bob_poll = poll_bob(&bob_valid, &relay);
    assert!(bob_poll.status.success(), "{}", output_text(&bob_poll));
    assert!(!session_path(&bob_valid, "alice").exists());
    let alice_poll = poll_alice(&alice_valid, &relay);
    assert!(alice_poll.status.success(), "{}", output_text(&alice_poll));
    assert!(session_path(&alice_valid, "bob").exists());
    let bob_confirm = poll_bob(&bob_valid, &relay);
    assert!(
        bob_confirm.status.success(),
        "{}",
        output_text(&bob_confirm)
    );
    assert!(session_path(&bob_valid, "alice").exists());
    let valid_text = [
        output_text(&alice_init),
        output_text(&bob_poll),
        output_text(&alice_poll),
        output_text(&bob_confirm),
    ]
    .join("\n");
    assert!(valid_text.contains("event=handshake_complete peer=bob role=initiator"));
    assert!(valid_text.contains("event=handshake_complete peer=alice role=responder"));
    assert_no_leak_or_panic(&valid_text);
    println!("NA0303_QSC_HANDSHAKE_CONTROL_OK");

    let mut rejected_outputs = Vec::new();

    let (alice_unsupported, bob_unsupported) = new_pair(&base, "unsupported-version");
    let init = init_alice(&alice_unsupported, &relay);
    assert!(init.status.success(), "{}", output_text(&init));
    let queued = server.drain_channel(ROUTE_TOKEN_BOB);
    assert_eq!(queued.len(), 1);
    server.replace_channel(
        ROUTE_TOKEN_BOB,
        vec![mutate_handshake_version(queued[0].clone(), 0xffff)],
    );
    let before_session = session_path(&bob_unsupported, "alice");
    assert!(!before_session.exists());
    let out = poll_bob(&bob_unsupported, &relay);
    assert!(out.status.success(), "{}", output_text(&out));
    let text = output_text(&out);
    assert_reject_no_commit(&text);
    assert!(!before_session.exists());
    assert!(server.drain_channel(ROUTE_TOKEN_ALICE).is_empty());
    rejected_outputs.push(text);
    println!("NA0303_UNSUPPORTED_SUITE_ADMISSION_REJECT_OK");

    let (alice_downgrade, bob_downgrade) = new_pair(&base, "downgrade-version");
    let init = init_alice(&alice_downgrade, &relay);
    assert!(init.status.success(), "{}", output_text(&init));
    let queued = server.drain_channel(ROUTE_TOKEN_BOB);
    assert_eq!(queued.len(), 1);
    server.replace_channel(
        ROUTE_TOKEN_BOB,
        vec![mutate_handshake_version(queued[0].clone(), 0x0000)],
    );
    let out = poll_bob(&bob_downgrade, &relay);
    assert!(out.status.success(), "{}", output_text(&out));
    let text = output_text(&out);
    assert_reject_no_commit(&text);
    assert!(!session_path(&bob_downgrade, "alice").exists());
    assert!(server.drain_channel(ROUTE_TOKEN_ALICE).is_empty());
    rejected_outputs.push(text);
    println!("NA0303_DOWNGRADE_ADMISSION_REJECT_OK");

    let (_alice_malformed, bob_malformed) = new_pair(&base, "malformed");
    server.replace_channel(
        ROUTE_TOKEN_BOB,
        vec![MALFORMED_SENTINEL.as_bytes().to_vec()],
    );
    let out = poll_bob(&bob_malformed, &relay);
    assert!(out.status.success(), "{}", output_text(&out));
    let text = output_text(&out);
    assert_reject_no_commit(&text);
    assert!(!session_path(&bob_malformed, "alice").exists());
    assert!(server.drain_channel(ROUTE_TOKEN_ALICE).is_empty());
    rejected_outputs.push(text);
    println!("NA0303_MALFORMED_ADMISSION_REJECT_OK");

    let alice_inactive = base.join("inactive-alice");
    let bob_inactive = base.join("inactive-bob");
    create_dir_700(&alice_inactive);
    create_dir_700(&bob_inactive);
    common::init_mock_vault(&alice_inactive);
    common::init_mock_vault(&bob_inactive);
    contacts_route_set(&alice_inactive, "bob", ROUTE_TOKEN_BOB);
    contacts_route_set(&bob_inactive, "alice", ROUTE_TOKEN_ALICE);
    relay_inbox_set(&alice_inactive, ROUTE_TOKEN_ALICE);
    relay_inbox_set(&bob_inactive, ROUTE_TOKEN_BOB);
    let out = init_alice(&alice_inactive, &relay);
    assert!(!out.status.success(), "{}", output_text(&out));
    let text = output_text(&out);
    assert_reject_no_commit(&text);
    assert!(text.contains("identity_unknown"), "{text}");
    assert!(server.drain_channel(ROUTE_TOKEN_BOB).is_empty());
    assert!(!session_path(&alice_inactive, "bob").exists());
    rejected_outputs.push(text);
    println!("NA0303_INACTIVE_ADMISSION_REJECT_OK");

    let (alice_replay, bob_replay) = new_pair(&base, "replay");
    let init = init_alice(&alice_replay, &relay);
    assert!(init.status.success(), "{}", output_text(&init));
    let queued = server.drain_channel(ROUTE_TOKEN_BOB);
    assert_eq!(queued.len(), 1);
    server.replace_channel(ROUTE_TOKEN_BOB, vec![queued[0].clone()]);
    let first_bob = poll_bob(&bob_replay, &relay);
    assert!(first_bob.status.success(), "{}", output_text(&first_bob));
    let first_b1 = server.drain_channel(ROUTE_TOKEN_ALICE);
    assert_eq!(first_b1.len(), 1, "first valid A1 should emit one B1");
    assert!(!session_path(&bob_replay, "alice").exists());
    let vault_before = fs::read(vault_path(&bob_replay)).expect("vault before replay");
    server.replace_channel(ROUTE_TOKEN_BOB, vec![queued[0].clone()]);
    let replay = poll_bob(&bob_replay, &relay);
    assert!(replay.status.success(), "{}", output_text(&replay));
    let text = output_text(&replay);
    assert_reject_no_commit(&text);
    assert!(!session_path(&bob_replay, "alice").exists());
    let vault_after = fs::read(vault_path(&bob_replay)).expect("vault after replay");
    assert_eq!(
        vault_before, vault_after,
        "replay mutated pending vault state"
    );
    assert!(server.drain_channel(ROUTE_TOKEN_ALICE).is_empty());
    rejected_outputs.push(text);
    println!("NA0303_REPLAY_ADMISSION_REJECT_OK");

    for text in &rejected_outputs {
        assert_reject_no_commit(text);
    }
    println!("NA0303_NO_MUTATION_ON_REJECT_OK");
    println!("NA0303_NO_RECV_COMMIT_ON_REJECT_OK");
    println!("NA0303_NO_PANIC_OK");
    println!("NA0303_NO_SECRET_LEAK_OK");
    println!("NA0303_QSC_HANDSHAKE_ACTIVATION_NEGOTIATION_OK");
}
