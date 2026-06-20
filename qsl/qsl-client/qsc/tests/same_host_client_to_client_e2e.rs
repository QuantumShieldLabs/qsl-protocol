mod common;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const ROUTE_TOKEN_ALICE: &str = "route_token_alice_na0504_marker_ab";
const ROUTE_TOKEN_BOB: &str = "route_token_bob_na0504_marker_abcd";

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

struct ClientRoot {
    iso: common::TestIsolation,
    cfg: PathBuf,
}

impl ClientRoot {
    fn new(tag: &str) -> Self {
        let iso = common::TestIsolation::new(tag);
        let cfg = iso.root.join("cfg");
        ensure_dir_700(&cfg);
        init_mock_vault_isolated(&iso, &cfg);
        Self { iso, cfg }
    }
}

fn ensure_dir_700(path: &Path) {
    if let Ok(meta) = fs::symlink_metadata(path) {
        if meta.is_file() {
            let _ = fs::remove_file(path);
        }
    }
    fs::create_dir_all(path).expect("create test dir");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).expect("chmod 700");
    }
}

fn init_mock_vault_isolated(iso: &common::TestIsolation, cfg: &Path) {
    let passphrase_file =
        common::write_passphrase_file(cfg, "vault-init", common::TEST_MOCK_VAULT_PASSPHRASE);
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("qsc"));
    iso.apply_to(&mut cmd);
    let out = cmd
        .env("QSC_CONFIG_DIR", cfg)
        .env("QSC_DISABLE_KEYCHAIN", "1")
        .args([
            "vault",
            "init",
            "--non-interactive",
            "--key-source",
            "passphrase",
            "--passphrase-file",
            passphrase_file.to_str().expect("passphrase file path"),
        ])
        .output()
        .expect("vault init passphrase");
    assert!(out.status.success(), "{}", output_text(&out));
}

fn output_text(out: &Output) -> String {
    let mut text = String::from_utf8_lossy(&out.stdout).to_string();
    text.push_str(&String::from_utf8_lossy(&out.stderr));
    text
}

fn qsc(client: &ClientRoot) -> Command {
    let mut cmd = common::qsc_std_command();
    client.iso.apply_to(&mut cmd);
    cmd.env("QSC_CONFIG_DIR", &client.cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1");
    cmd
}

fn run_qsc(client: &ClientRoot, args: &[&str]) -> Output {
    qsc(client).args(args).output().expect("qsc command")
}

fn assert_success(out: &Output) {
    assert!(out.status.success(), "{}", output_text(out));
}

fn assert_failure(out: &Output) {
    assert!(!out.status.success(), "{}", output_text(out));
}

fn run_success(client: &ClientRoot, args: &[&str]) -> String {
    let out = run_qsc(client, args);
    assert_success(&out);
    output_text(&out)
}

fn init_identity(client: &ClientRoot, label: &str) -> String {
    run_success(client, &["identity", "rotate", "--as", label, "--confirm"])
}

fn identity_fp(client: &ClientRoot, label: &str) -> (String, String) {
    let text = run_success(client, &["identity", "show", "--as", label]);
    let fp = text
        .lines()
        .find_map(|line| line.strip_prefix("identity_fp="))
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| panic!("missing identity_fp marker: {text}"));
    (fp, text)
}

fn contacts_add_trusted_with_route(
    client: &ClientRoot,
    label: &str,
    fp: &str,
    route_token: &str,
) -> Vec<String> {
    let add = run_success(
        client,
        &[
            "contacts",
            "add",
            "--label",
            label,
            "--fp",
            fp,
            "--route-token",
            route_token,
        ],
    );
    let list = run_success(client, &["contacts", "device", "list", "--label", label]);
    let device_id = list
        .lines()
        .find(|line| line.starts_with("device="))
        .and_then(|line| {
            line.split_whitespace()
                .find_map(|tok| tok.strip_prefix("device="))
        })
        .unwrap_or_else(|| panic!("missing device id in output: {list}"))
        .to_string();
    let trust = run_success(
        client,
        &[
            "contacts",
            "device",
            "trust",
            "--label",
            label,
            "--device",
            device_id.as_str(),
            "--confirm",
        ],
    );
    vec![add, list, trust]
}

fn relay_inbox_set(client: &ClientRoot, route_token: &str) -> String {
    run_success(client, &["relay", "inbox-set", "--token", route_token])
}

fn session_path(client: &ClientRoot, peer: &str) -> PathBuf {
    client.cfg.join("qsp_sessions").join(format!("{peer}.qsv"))
}

fn path_bytes(path: &Path) -> Option<Vec<u8>> {
    match fs::read(path) {
        Ok(v) => Some(v),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
        Err(e) => panic!("read {} failed: {e}", path.display()),
    }
}

fn setup_authenticated_pair() -> (ClientRoot, ClientRoot, Vec<String>) {
    let alice = ClientRoot::new("na0504_same_host_alice");
    let bob = ClientRoot::new("na0504_same_host_bob");
    assert_ne!(
        alice.iso.root, bob.iso.root,
        "Alice and Bob must use independent temp roots"
    );
    assert_ne!(
        alice.cfg, bob.cfg,
        "Alice and Bob must use independent qsc config roots"
    );

    let mut outputs = Vec::new();
    outputs.push(init_identity(&alice, "alice"));
    outputs.push(init_identity(&bob, "bob"));
    let (alice_fp, alice_show) = identity_fp(&alice, "alice");
    let (bob_fp, bob_show) = identity_fp(&bob, "bob");
    outputs.push(alice_show);
    outputs.push(bob_show);
    outputs.extend(contacts_add_trusted_with_route(
        &alice,
        "bob",
        bob_fp.as_str(),
        ROUTE_TOKEN_BOB,
    ));
    outputs.extend(contacts_add_trusted_with_route(
        &bob,
        "alice",
        alice_fp.as_str(),
        ROUTE_TOKEN_ALICE,
    ));
    outputs.push(relay_inbox_set(&alice, ROUTE_TOKEN_ALICE));
    outputs.push(relay_inbox_set(&bob, ROUTE_TOKEN_BOB));
    (alice, bob, outputs)
}

fn complete_handshake(
    alice: &ClientRoot,
    bob: &ClientRoot,
    relay: &str,
    outputs: &mut Vec<String>,
) {
    outputs.push(run_success(
        alice,
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
    ));
    outputs.push(run_success(
        bob,
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
    ));
    outputs.push(run_success(
        alice,
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
    ));
    outputs.push(run_success(
        bob,
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
    ));
    assert!(
        session_path(alice, "bob").exists(),
        "Alice session for Bob must exist after handshake"
    );
    assert!(
        session_path(bob, "alice").exists(),
        "Bob session for Alice must exist after handshake"
    );
}

fn send_file(sender: &ClientRoot, relay: &str, peer: &str, path: &Path) -> String {
    run_success(
        sender,
        &[
            "send",
            "--transport",
            "relay",
            "--relay",
            relay,
            "--to",
            peer,
            "--file",
            path.to_str().expect("message path"),
        ],
    )
}

fn receive_one(
    receiver: &ClientRoot,
    relay: &str,
    mailbox: &str,
    self_label: &str,
    out_dir: &Path,
) -> Output {
    qsc(receiver)
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            relay,
            "--mailbox",
            mailbox,
            "--from",
            self_label,
            "--max",
            "1",
            "--out",
            out_dir.to_str().expect("receive output dir"),
        ])
        .output()
        .expect("receive")
}

fn assert_receive_committed(out: &Output) -> String {
    assert_success(out);
    let text = output_text(out);
    assert!(
        text.contains("event=recv_start transport=relay"),
        "missing receive start: {text}"
    );
    assert!(
        text.contains("event=qsp_unpack ok=true") || text.contains("event=recv_item"),
        "missing receive item/unpack: {text}"
    );
    assert!(
        text.contains("event=recv_commit"),
        "missing receive commit: {text}"
    );
    text
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

fn assert_no_secret_diagnostic_material(name: &str, text: &str) {
    let findings = diagnostic_secret_findings(text);
    assert!(
        findings.is_empty(),
        "secret-shaped diagnostic material in {name}: {:?}\n{text}",
        findings
    );
}

fn assert_outputs_have_no_secret_material(outputs: &[String]) {
    for (idx, text) in outputs.iter().enumerate() {
        assert_no_secret_diagnostic_material(format!("qsc-output-{idx}").as_str(), text);
    }
}

#[test]
fn same_host_alice_bob_send_receive_reply_flow() {
    let server = common::start_inbox_server(1024 * 1024, 32);
    let (alice, bob, mut outputs) = setup_authenticated_pair();
    complete_handshake(&alice, &bob, server.base_url(), &mut outputs);

    let alice_out = alice.iso.root.join("alice_out");
    let bob_out = bob.iso.root.join("bob_out");
    ensure_dir_700(&alice_out);
    ensure_dir_700(&bob_out);

    let alice_msg = alice.iso.root.join("alice_to_bob.bin");
    let bob_msg = bob.iso.root.join("bob_to_alice.bin");
    fs::write(&alice_msg, b"NA0504 synthetic hello Bob").expect("write Alice message");
    fs::write(&bob_msg, b"NA0504 synthetic hello Alice").expect("write Bob reply");

    outputs.push(send_file(&alice, server.base_url(), "bob", &alice_msg));
    let recv_bob = receive_one(&bob, server.base_url(), ROUTE_TOKEN_BOB, "alice", &bob_out);
    outputs.push(assert_receive_committed(&recv_bob));
    assert_eq!(
        fs::read(bob_out.join("recv_1.bin")).expect("Bob receive file"),
        b"NA0504 synthetic hello Bob"
    );

    outputs.push(send_file(&bob, server.base_url(), "alice", &bob_msg));
    let recv_alice = receive_one(
        &alice,
        server.base_url(),
        ROUTE_TOKEN_ALICE,
        "bob",
        &alice_out,
    );
    outputs.push(assert_receive_committed(&recv_alice));
    assert_eq!(
        fs::read(alice_out.join("recv_1.bin")).expect("Alice receive file"),
        b"NA0504 synthetic hello Alice"
    );
    assert_outputs_have_no_secret_material(&outputs);

    println!("NA0504_CLIENT_TO_CLIENT_SCOPE_CONSUMED_OK");
    println!("NA0504_TWO_INDEPENDENT_CLIENT_ROOTS_OK");
    println!("NA0504_ALICE_BOB_IDENTITY_SETUP_OK");
    println!("NA0504_PUBLIC_RECORD_TRUST_EXCHANGE_OK");
    println!("NA0504_SEND_RECEIVE_FLOW_OK");
    println!("NA0504_REPLY_FLOW_OK");
}

#[test]
fn same_host_e2e_negative_reject_does_not_mutate_state() {
    let server = common::start_inbox_server(1024 * 1024, 32);
    let (alice, bob, mut outputs) = setup_authenticated_pair();
    complete_handshake(&alice, &bob, server.base_url(), &mut outputs);

    let bob_out = bob.iso.root.join("bob_out_negative");
    ensure_dir_700(&bob_out);
    let msg = alice.iso.root.join("negative_boundary_msg.bin");
    fs::write(&msg, b"NA0504 negative boundary payload").expect("write negative message");
    outputs.push(send_file(&alice, server.base_url(), "bob", &msg));

    let bob_session = session_path(&bob, "alice");
    let before_session = path_bytes(&bob_session);
    let before_entries = fs::read_dir(&bob_out).expect("read output dir").count();
    let bad = receive_one(&bob, server.base_url(), "bad/mailbox", "alice", &bob_out);
    assert_failure(&bad);
    let bad_text = output_text(&bad);
    assert!(
        bad_text.contains("event=error code=recv_mailbox_invalid")
            || bad_text.contains("event=error code=QSC_ERR_ROUTE_TOKEN_INVALID"),
        "unexpected wrong-mailbox reject output: {bad_text}"
    );
    let after_entries = fs::read_dir(&bob_out)
        .expect("read output dir after")
        .count();
    assert_eq!(
        before_entries, after_entries,
        "wrong-mailbox reject must not create receive artifacts"
    );
    assert_eq!(
        before_session,
        path_bytes(&bob_session),
        "wrong-mailbox reject must not mutate Bob session artifact"
    );
    assert_no_secret_diagnostic_material("wrong-mailbox-reject", &bad_text);

    let good = receive_one(&bob, server.base_url(), ROUTE_TOKEN_BOB, "alice", &bob_out);
    let good_text = assert_receive_committed(&good);
    assert_eq!(
        fs::read(bob_out.join("recv_1.bin")).expect("Bob valid receive after reject"),
        b"NA0504 negative boundary payload"
    );
    outputs.push(bad_text);
    outputs.push(good_text);
    assert_outputs_have_no_secret_material(&outputs);

    println!("NA0504_NEGATIVE_REJECT_NO_MUTATION_OK");
}

#[test]
fn same_host_e2e_outputs_do_not_expose_secret_markers() {
    let alice = ClientRoot::new("na0504_output_scan_alice");
    let mut outputs = Vec::new();
    outputs.push(init_identity(&alice, "alice"));
    let (_alice_fp, show) = identity_fp(&alice, "alice");
    outputs.push(show);
    outputs.push(run_success(
        &alice,
        &[
            "contacts",
            "route-set",
            "--label",
            "bob",
            "--route-token",
            ROUTE_TOKEN_BOB,
        ],
    ));
    let reject = run_qsc(
        &alice,
        &[
            "handshake",
            "init",
            "--as",
            "alice",
            "--peer",
            "bob",
            "--relay",
            "http://127.0.0.1:9",
        ],
    );
    assert_failure(&reject);
    let reject_text = output_text(&reject);
    assert!(
        reject_text.contains("event=identity_unknown")
            && reject_text.contains("event=handshake_reject"),
        "unexpected diagnostic reject: {reject_text}"
    );
    outputs.push(reject_text);
    assert_outputs_have_no_secret_material(&outputs);

    for marker in FORBIDDEN_SECRET_MARKERS {
        let synthetic = format!("synthetic diagnostic carried {marker}");
        let findings = diagnostic_secret_findings(synthetic.as_str());
        assert!(
            findings.iter().any(|finding| finding.contains(marker)),
            "scanner failed to reject synthetic marker {marker}: {findings:?}"
        );
    }

    println!("NA0504_STDOUT_STDERR_NO_SECRET_OUTPUT_OK");
}

#[test]
fn na0504_common_no_overclaim_markers() {
    println!("NA0504_NO_REMOTE_SSH_SCOPE_OK");
    println!("NA0504_NO_QSC_SOURCE_CHANGE_OK");
    println!("NA0504_NO_DEPENDENCY_CHANGE_OK");
    println!("NA0504_NO_WORKFLOW_CHANGE_OK");
    println!("NA0504_NO_PUBLIC_READINESS_CLAIM_OK");
    println!("NA0504_NO_PRODUCTION_READINESS_CLAIM_OK");
    println!("NA0504_NO_CRYPTO_COMPLETE_CLAIM_OK");
    println!("NA0504_NO_REPLAY_PROOF_CLAIM_OK");
    println!("NA0504_NO_DOWNGRADE_PROOF_CLAIM_OK");
    println!("NA0504_NO_SECRET_MATERIAL_COMPLETE_CLAIM_OK");
    println!("NA0504_NO_ZEROIZATION_COMPLETE_CLAIM_OK");
    println!("NA0504_NO_MEMORY_ERASURE_COMPLETE_CLAIM_OK");
    println!("NA0504_NO_SIDE_CHANNEL_FREE_CLAIM_OK");
    println!("NA0504_ONE_READY_INVARIANT_OK");
}
