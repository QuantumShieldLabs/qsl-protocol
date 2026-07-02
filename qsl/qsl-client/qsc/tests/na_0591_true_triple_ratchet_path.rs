mod common;

use quantumshield_refimpl::qse::Envelope;
use std::fs;
use std::path::{Path, PathBuf};

const ROUTE_TOKEN_ALICE: &str = "route_token_alice_na0591_abcdefghijkl";
const ROUTE_TOKEN_BOB: &str = "route_token_bob_na0591_abcdefghijklmn";

fn ensure_dir_700(path: &Path) {
    if let Ok(meta) = fs::symlink_metadata(path) {
        if meta.is_file() {
            let _ = fs::remove_file(path);
        }
    }
    fs::create_dir_all(path).expect("create dir");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).expect("chmod 700");
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

fn run_qsc(iso: &common::TestIsolation, cfg: &Path, args: &[&str]) -> std::process::Output {
    let mut cmd = common::qsc_std_command();
    iso.apply_to(&mut cmd);
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .env_remove("QSC_QSP_SEED")
        .env_remove("QSC_ALLOW_SEED_FALLBACK")
        .env_remove("QSC_UNSAFE_TEST_SEED_FALLBACK")
        .args(args)
        .output()
        .expect("qsc command")
}

fn init_identity(iso: &common::TestIsolation, cfg: &Path, label: &str) {
    let out = run_qsc(
        iso,
        cfg,
        &["identity", "rotate", "--as", label, "--confirm"],
    );
    assert!(out.status.success(), "{}", output_text(&out));
}

fn identity_fp(iso: &common::TestIsolation, cfg: &Path, label: &str) -> String {
    let out = run_qsc(iso, cfg, &["identity", "show", "--as", label]);
    assert!(out.status.success(), "{}", output_text(&out));
    output_text(&out)
        .lines()
        .find_map(|line| line.strip_prefix("identity_fp="))
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| panic!("missing identity_fp in output: {}", output_text(&out)))
}

fn contacts_add_authenticated_with_route(
    iso: &common::TestIsolation,
    cfg: &Path,
    label: &str,
    fp: &str,
    token: &str,
) {
    let out = run_qsc(
        iso,
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

fn relay_inbox_set(iso: &common::TestIsolation, cfg: &Path, token: &str) {
    let out = run_qsc(iso, cfg, &["relay", "inbox-set", "--token", token]);
    assert!(out.status.success(), "{}", output_text(&out));
}

fn complete_handshake(iso: &common::TestIsolation, alice_cfg: &Path, bob_cfg: &Path, relay: &str) {
    let init = run_qsc(
        iso,
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
    );
    assert!(init.status.success(), "{}", output_text(&init));

    let bob_poll_1 = run_qsc(
        iso,
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
    );
    assert!(bob_poll_1.status.success(), "{}", output_text(&bob_poll_1));

    let alice_poll_2 = run_qsc(
        iso,
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
    );
    assert!(
        alice_poll_2.status.success(),
        "{}",
        output_text(&alice_poll_2)
    );

    let bob_poll_3 = run_qsc(
        iso,
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
    );
    let bob_poll_3_text = output_text(&bob_poll_3);
    assert!(bob_poll_3.status.success(), "{bob_poll_3_text}");
    assert!(
        bob_poll_3_text.contains("event=handshake_complete"),
        "missing handshake completion marker: {bob_poll_3_text}"
    );
}

fn contains_subslice(haystack: &[u8], needle: &[u8]) -> bool {
    !needle.is_empty()
        && haystack
            .windows(needle.len())
            .any(|window| window == needle)
}

#[test]
fn handshake_backed_send_receive_uses_qsp_without_seed_fallback() {
    let server = common::start_inbox_server(1024 * 1024, 32);
    let iso = common::TestIsolation::new("na0591_true_triple_ratchet_path");
    let base: PathBuf = iso.root.join("case");
    create_dir_700(&base);

    let alice_cfg = base.join("alice_cfg");
    let bob_cfg = base.join("bob_cfg");
    let bob_out = base.join("bob_out");
    create_dir_700(&alice_cfg);
    create_dir_700(&bob_cfg);
    create_dir_700(&bob_out);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);

    init_identity(&iso, &alice_cfg, "alice");
    init_identity(&iso, &bob_cfg, "bob");
    let alice_fp = identity_fp(&iso, &alice_cfg, "alice");
    let bob_fp = identity_fp(&iso, &bob_cfg, "bob");
    contacts_add_authenticated_with_route(&iso, &alice_cfg, "bob", &bob_fp, ROUTE_TOKEN_BOB);
    contacts_add_authenticated_with_route(&iso, &bob_cfg, "alice", &alice_fp, ROUTE_TOKEN_ALICE);
    relay_inbox_set(&iso, &alice_cfg, ROUTE_TOKEN_ALICE);
    relay_inbox_set(&iso, &bob_cfg, ROUTE_TOKEN_BOB);

    complete_handshake(&iso, &alice_cfg, &bob_cfg, server.base_url());

    let plaintext: Vec<u8> = (0..96).map(|i| ((i * 17 + 3) % 251) as u8).collect();
    let msg = base.join("msg.bin");
    fs::write(&msg, &plaintext).expect("write message");

    let send = run_qsc(
        &iso,
        &alice_cfg,
        &[
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "bob",
            "--file",
            msg.to_str().expect("msg path"),
        ],
    );
    let send_text = output_text(&send);
    assert!(send.status.success(), "{send_text}");
    assert!(
        send_text.contains("event=qsp_pack ok=true")
            && send_text.contains("event=ratchet_send_advance"),
        "send did not report qsp pack and ratchet advance: {send_text}"
    );
    assert!(
        !send_text.contains("QSC_ALLOW_SEED_FALLBACK") && !send_text.contains("QSC_QSP_SEED"),
        "seed fallback surface leaked into send output: {send_text}"
    );

    let queued = server.drain_channel(ROUTE_TOKEN_BOB);
    assert_eq!(queued.len(), 1, "expected exactly one relay item");
    let envelope = Envelope::decode(&queued[0]).expect("decode qse envelope");
    assert!(envelope.payload.len() > plaintext.len());
    assert!(!contains_subslice(&queued[0], &plaintext));
    assert!(!contains_subslice(&envelope.payload, &plaintext));
    server.replace_channel(ROUTE_TOKEN_BOB, queued);

    let recv = run_qsc(
        &iso,
        &bob_cfg,
        &[
            "receive",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--mailbox",
            ROUTE_TOKEN_BOB,
            "--from",
            "alice",
            "--max",
            "1",
            "--out",
            bob_out.to_str().expect("out path"),
        ],
    );
    let recv_text = output_text(&recv);
    assert!(recv.status.success(), "{recv_text}");
    assert!(
        recv_text.contains("event=qsp_unpack ok=true")
            && recv_text.contains("event=ratchet_recv_advance")
            && recv_text.contains("event=recv_commit"),
        "receive did not report qsp unpack, ratchet advance, and commit: {recv_text}"
    );
    assert!(
        !recv_text.contains("QSC_ALLOW_SEED_FALLBACK") && !recv_text.contains("QSC_QSP_SEED"),
        "seed fallback surface leaked into receive output: {recv_text}"
    );

    let received = fs::read(bob_out.join("recv_1.bin")).expect("read received message");
    assert_eq!(received, plaintext);
}
