mod common;

use sha2::{Digest, Sha256};
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Mutex, MutexGuard, OnceLock};

const ROUTE_TOKEN_BOB: &str = "route_token_bob_abcdefghijklmnopqr";

fn attachment_test_guard() -> MutexGuard<'static, ()> {
    static TEST_GUARD: OnceLock<Mutex<()>> = OnceLock::new();
    TEST_GUARD
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

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
    let mut s = String::from_utf8_lossy(&out.stdout).to_string();
    s.push_str(&String::from_utf8_lossy(&out.stderr));
    s
}

fn write_repeated_file(path: &Path, size: usize, byte: u8) {
    let mut file = fs::File::create(path).unwrap();
    let chunk = vec![byte; 1024 * 1024];
    let mut remaining = size;
    while remaining > 0 {
        let step = remaining.min(chunk.len());
        file.write_all(&chunk[..step]).unwrap();
        remaining -= step;
    }
    file.sync_all().unwrap();
}

fn file_sha256_hex(path: &Path) -> String {
    let mut file = fs::File::open(path).unwrap();
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 8192];
    loop {
        let n = file.read(&mut buf).unwrap();
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    let digest = hasher.finalize();
    let mut out = String::with_capacity(digest.len() * 2);
    for byte in digest {
        out.push_str(format!("{byte:02x}").as_str());
    }
    out
}

fn qsc_base(cfg: &Path) -> Command {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("qsc"));
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1");
    cmd
}

fn contacts_add_with_route_token(cfg: &Path, label: &str, token: &str) {
    let out = qsc_base(cfg)
        .args([
            "contacts",
            "add",
            "--label",
            label,
            "--fp",
            "fp-test",
            "--route-token",
            token,
        ])
        .output()
        .expect("contacts add route token");
    assert!(out.status.success(), "{}", output_text(&out));
    let list = qsc_base(cfg)
        .args(["contacts", "device", "list", "--label", label])
        .output()
        .expect("contacts device list");
    assert!(list.status.success(), "{}", output_text(&list));
    let list_text = output_text(&list);
    let device_id = list_text
        .lines()
        .find(|line| line.starts_with("device="))
        .and_then(|line| {
            line.split_whitespace()
                .find_map(|tok| tok.strip_prefix("device="))
        })
        .unwrap_or_else(|| panic!("missing device id in output: {list_text}"));
    let trust = qsc_base(cfg)
        .args([
            "contacts",
            "device",
            "trust",
            "--label",
            label,
            "--device",
            device_id,
            "--confirm",
        ])
        .output()
        .expect("contacts device trust");
    assert!(trust.status.success(), "{}", output_text(&trust));
}

fn relay_set_inbox_token(cfg: &Path, token: &str) {
    let out = qsc_base(cfg)
        .args(["relay", "inbox-set", "--token", token])
        .output()
        .expect("relay inbox set");
    assert!(out.status.success(), "{}", output_text(&out));
}

fn run_attachment_send(
    cfg: &Path,
    relay: &str,
    service: &str,
    to: &str,
    path: &Path,
    with_receipt: bool,
) -> std::process::Output {
    let mut cmd = qsc_base(cfg);
    cmd.args([
        "file",
        "send",
        "--transport",
        "relay",
        "--relay",
        relay,
        "--attachment-service",
        service,
        "--to",
        to,
        "--path",
        path.to_str().unwrap(),
    ]);
    if with_receipt {
        cmd.args(["--receipt", "delivered"]);
    }
    cmd.output().expect("attachment send")
}

fn run_receive(
    cfg: &Path,
    relay: &str,
    mailbox: &str,
    out_dir: &Path,
    attachment_service: Option<&str>,
    emit_receipts: bool,
) -> std::process::Output {
    let mut cmd = qsc_base(cfg);
    cmd.args([
        "receive",
        "--transport",
        "relay",
        "--relay",
        relay,
        "--mailbox",
        mailbox,
        "--from",
        "bob",
        "--max",
        "64",
        "--max-file-size",
        "16777216",
        "--max-file-chunks",
        "512",
        "--out",
        out_dir.to_str().unwrap(),
    ]);
    if let Some(service) = attachment_service {
        cmd.args(["--attachment-service", service]);
    }
    if emit_receipts {
        cmd.args(["--emit-receipts", "delivered"]);
    }
    cmd.output().expect("receive")
}

struct ReceiveBounds {
    max_file_size: usize,
    max_file_chunks: usize,
}

fn run_receive_with_bounds(
    cfg: &Path,
    relay: &str,
    mailbox: &str,
    out_dir: &Path,
    attachment_service: Option<&str>,
    emit_receipts: bool,
    bounds: ReceiveBounds,
) -> std::process::Output {
    let mut cmd = qsc_base(cfg);
    cmd.args([
        "receive",
        "--transport",
        "relay",
        "--relay",
        relay,
        "--mailbox",
        mailbox,
        "--from",
        "bob",
        "--max",
        "64",
        "--max-file-size",
        &bounds.max_file_size.to_string(),
        "--max-file-chunks",
        &bounds.max_file_chunks.to_string(),
        "--out",
        out_dir.to_str().unwrap(),
    ]);
    if let Some(service) = attachment_service {
        cmd.args(["--attachment-service", service]);
    }
    if emit_receipts {
        cmd.args(["--emit-receipts", "delivered"]);
    }
    cmd.output().expect("receive")
}

fn run_raw_send(cfg: &Path, relay: &str, to: &str, file: &Path) -> std::process::Output {
    qsc_base(cfg)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            relay,
            "--to",
            to,
            "--file",
            file.to_str().unwrap(),
        ])
        .output()
        .expect("raw send")
}

fn assert_no_secretish_output(text: &str) {
    for forbidden in [
        "X-QATT-Resume-Token",
        "X-QATT-Fetch-Capability",
        "fetch_capability=",
        "resume_token=",
        "enc_ctx_b64u=",
        "/v1/attachments/",
    ] {
        assert!(
            !text.contains(forbidden),
            "forbidden attachment secret/url marker leaked: {forbidden}\n{text}"
        );
    }
}

fn setup_pair(base: &Path) -> (PathBuf, PathBuf, PathBuf, PathBuf) {
    let alice_cfg = base.join("alice_cfg");
    let bob_cfg = base.join("bob_cfg");
    let alice_out = base.join("alice_out");
    let bob_out = base.join("bob_out");
    create_dir_700(&alice_cfg);
    create_dir_700(&bob_cfg);
    create_dir_700(&alice_out);
    create_dir_700(&bob_out);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);
    contacts_add_with_route_token(&alice_cfg, "bob", ROUTE_TOKEN_BOB);
    contacts_add_with_route_token(&bob_cfg, "bob", ROUTE_TOKEN_BOB);
    relay_set_inbox_token(&alice_cfg, ROUTE_TOKEN_BOB);
    relay_set_inbox_token(&bob_cfg, ROUTE_TOKEN_BOB);
    (alice_cfg, bob_cfg, alice_out, bob_out)
}

fn forged_attachment_descriptor(path: &Path, bad_enc_ctx: bool) {
    let plaintext_len = 5_242_913_u64;
    let part_size_bytes = 65_536_u64;
    let part_plain_capacity = part_size_bytes - 16;
    let part_count = plaintext_len.div_ceil(part_plain_capacity) as u32;
    let ciphertext_len = plaintext_len + (part_count as u64 * 16);
    let enc_ctx = if bad_enc_ctx {
        "bad_ctx"
    } else {
        "AQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
    };
    let json = format!(
        concat!(
            "{{",
            "\"v\":1,",
            "\"t\":\"attachment_descriptor\",",
            "\"attachment_id\":\"{}\",",
            "\"plaintext_len\":{},",
            "\"ciphertext_len\":{},",
            "\"part_size_class\":\"p64k\",",
            "\"part_count\":{},",
            "\"integrity_alg\":\"sha512_merkle_v1\",",
            "\"integrity_root\":\"{}\",",
            "\"locator_kind\":\"service_ref_v1\",",
            "\"locator_ref\":\"locref1234567890\",",
            "\"fetch_capability\":\"fetchcapabilityplaceholder1234567890\",",
            "\"enc_ctx_alg\":\"chacha20poly1305_part_v1\",",
            "\"enc_ctx_b64u\":\"{}\",",
            "\"retention_class\":\"standard\",",
            "\"expires_at_unix_s\":4102444800,",
            "\"confirm_requested\":false,",
            "\"filename_hint\":\"forged.bin\"",
            "}}"
        ),
        "0".repeat(64),
        plaintext_len,
        ciphertext_len,
        part_count,
        "1".repeat(128),
        enc_ctx
    );
    fs::write(path, json).unwrap();
}

#[test]
fn attachment_upload_resume_and_invalid_resume_token_fail_closed() {
    let _guard = attachment_test_guard();
    let relay = common::start_inbox_server(2 * 1024 * 1024, 512);
    let service = common::start_attachment_server(100 * 1024 * 1024);
    let base = safe_test_root().join(format!("na0197c_upload_resume_{}", std::process::id()));
    create_dir_700(&base);
    let (alice_cfg, bob_cfg, _alice_out, bob_out) = setup_pair(&base);
    let payload = base.join("large.bin");
    fs::write(&payload, vec![0x41; 5 * 1024 * 1024 + 8192]).unwrap();

    let first = qsc_base(&alice_cfg)
        .env("QSC_ATTACHMENT_TEST_ABORT_AFTER_UPLOAD_PARTS", "1")
        .args([
            "file",
            "send",
            "--transport",
            "relay",
            "--relay",
            relay.base_url(),
            "--attachment-service",
            service.base_url(),
            "--to",
            "bob",
            "--path",
            payload.to_str().unwrap(),
            "--receipt",
            "delivered",
        ])
        .output()
        .expect("attachment upload interrupt");
    assert!(!first.status.success(), "{}", output_text(&first));
    let first_text = output_text(&first);
    assert!(
        first_text.contains("attachment_upload_part"),
        "{}",
        first_text
    );
    assert!(
        first_text.contains("code=attachment_test_interrupt_upload")
            || first_text.contains("attachment_test_interrupt_upload"),
        "{}",
        first_text
    );

    let bob_peek = run_receive(
        &bob_cfg,
        relay.base_url(),
        ROUTE_TOKEN_BOB,
        &bob_out,
        Some(service.base_url()),
        false,
    );
    assert!(bob_peek.status.success(), "{}", output_text(&bob_peek));
    assert!(output_text(&bob_peek).contains("event=recv_none"));

    let second = qsc_base(&alice_cfg)
        .env("QSC_ATTACHMENT_RESUME_TOKEN_OVERRIDE", "badresume")
        .args([
            "file",
            "send",
            "--transport",
            "relay",
            "--relay",
            relay.base_url(),
            "--attachment-service",
            service.base_url(),
            "--to",
            "bob",
            "--path",
            payload.to_str().unwrap(),
            "--receipt",
            "delivered",
        ])
        .output()
        .expect("attachment bad resume");
    assert!(!second.status.success(), "{}", output_text(&second));
    let second_text = output_text(&second);
    assert!(
        second_text.contains("REJECT_QATTSVC_RESUME_TOKEN"),
        "{}",
        second_text
    );

    let bob_peek_again = run_receive(
        &bob_cfg,
        relay.base_url(),
        ROUTE_TOKEN_BOB,
        &bob_out,
        Some(service.base_url()),
        false,
    );
    assert!(
        bob_peek_again.status.success(),
        "{}",
        output_text(&bob_peek_again)
    );
    assert!(output_text(&bob_peek_again).contains("event=recv_none"));
}

#[test]
fn attachment_e2e_resume_and_peer_confirm_after_persistence() {
    let _guard = attachment_test_guard();
    let relay = common::start_inbox_server(2 * 1024 * 1024, 512);
    let service = common::start_attachment_server(100 * 1024 * 1024);
    let base = safe_test_root().join(format!("na0197c_e2e_{}", std::process::id()));
    create_dir_700(&base);
    let (alice_cfg, bob_cfg, alice_out, bob_out) = setup_pair(&base);
    let payload = base.join("stream.bin");
    let payload_bytes = vec![0x5a; 6 * 1024 * 1024 + 321];
    fs::write(&payload, &payload_bytes).unwrap();

    let send = run_attachment_send(
        &alice_cfg,
        relay.base_url(),
        service.base_url(),
        "bob",
        &payload,
        true,
    );
    assert!(send.status.success(), "{}", output_text(&send));
    let send_text = output_text(&send);
    assert!(
        send_text.contains("event=attachment_service_commit"),
        "{}",
        send_text
    );
    assert!(
        send_text.contains("QSC_FILE_DELIVERY state=accepted_by_relay"),
        "{}",
        send_text
    );
    assert!(!send_text.contains("state=peer_confirmed"), "{}", send_text);
    assert_no_secretish_output(&send_text);

    let bob_partial = qsc_base(&bob_cfg)
        .env("QSC_ATTACHMENT_TEST_ABORT_AFTER_FETCH_BYTES", "2000000")
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            relay.base_url(),
            "--attachment-service",
            service.base_url(),
            "--mailbox",
            ROUTE_TOKEN_BOB,
            "--from",
            "bob",
            "--max",
            "64",
            "--max-file-size",
            "16777216",
            "--max-file-chunks",
            "512",
            "--out",
            bob_out.to_str().unwrap(),
            "--emit-receipts",
            "delivered",
        ])
        .output()
        .expect("bob partial receive");
    assert!(
        !bob_partial.status.success(),
        "{}",
        output_text(&bob_partial)
    );
    let bob_partial_text = output_text(&bob_partial);
    assert!(
        bob_partial_text.contains("attachment_test_interrupt_download"),
        "{}",
        bob_partial_text
    );
    assert!(
        !bob_out.join("stream.bin").exists(),
        "plaintext must not be released before full verification"
    );

    let alice_before_confirm = run_receive(
        &alice_cfg,
        relay.base_url(),
        ROUTE_TOKEN_BOB,
        &alice_out,
        None,
        false,
    );
    assert!(
        alice_before_confirm.status.success(),
        "{}",
        output_text(&alice_before_confirm)
    );
    assert!(
        !output_text(&alice_before_confirm).contains("state=peer_confirmed"),
        "{}",
        output_text(&alice_before_confirm)
    );

    let bob_full = run_receive(
        &bob_cfg,
        relay.base_url(),
        ROUTE_TOKEN_BOB,
        &bob_out,
        Some(service.base_url()),
        true,
    );
    assert!(bob_full.status.success(), "{}", output_text(&bob_full));
    let bob_full_text = output_text(&bob_full);
    assert!(
        bob_full_text.contains("attachment_confirm_send"),
        "{}",
        bob_full_text
    );
    assert_eq!(fs::read(bob_out.join("stream.bin")).unwrap(), payload_bytes);
    assert_no_secretish_output(&bob_full_text);

    let alice_after_confirm = run_receive(
        &alice_cfg,
        relay.base_url(),
        ROUTE_TOKEN_BOB,
        &alice_out,
        None,
        false,
    );
    assert!(
        alice_after_confirm.status.success(),
        "{}",
        output_text(&alice_after_confirm)
    );
    let alice_after_confirm_text = output_text(&alice_after_confirm);
    assert!(
        alice_after_confirm_text.contains("QSC_FILE_DELIVERY state=peer_confirmed"),
        "{}",
        alice_after_confirm_text
    );
}

#[test]
fn attachment_fetch_capability_and_enc_ctx_reject_without_persistence() {
    let _guard = attachment_test_guard();
    let relay = common::start_inbox_server(2 * 1024 * 1024, 512);
    let service = common::start_attachment_server(100 * 1024 * 1024);
    let base = safe_test_root().join(format!("na0197c_rejects_{}", std::process::id()));
    create_dir_700(&base);
    let (alice_cfg, bob_cfg, _alice_out, bob_out) = setup_pair(&base);
    let payload = base.join("badfetch.bin");
    fs::write(&payload, vec![0x33; 5 * 1024 * 1024 + 123]).unwrap();

    let send = qsc_base(&alice_cfg)
        .args([
            "file",
            "send",
            "--transport",
            "relay",
            "--relay",
            relay.base_url(),
            "--attachment-service",
            service.base_url(),
            "--to",
            "bob",
            "--path",
            payload.to_str().unwrap(),
            "--chunk-size",
            "16384",
        ])
        .output()
        .expect("legacy coexist send");
    assert!(send.status.success(), "{}", output_text(&send));

    let bad_fetch = qsc_base(&bob_cfg)
        .env(
            "QSC_ATTACHMENT_FETCH_CAPABILITY_OVERRIDE",
            "badfetchcapability",
        )
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            relay.base_url(),
            "--attachment-service",
            service.base_url(),
            "--mailbox",
            ROUTE_TOKEN_BOB,
            "--from",
            "bob",
            "--max",
            "64",
            "--max-file-size",
            "16777216",
            "--max-file-chunks",
            "512",
            "--out",
            bob_out.to_str().unwrap(),
        ])
        .output()
        .expect("bad fetch receive");
    assert!(!bad_fetch.status.success(), "{}", output_text(&bad_fetch));
    let bad_fetch_text = output_text(&bad_fetch);
    assert!(
        bad_fetch_text.contains("REJECT_QATTSVC_FETCH_CAPABILITY"),
        "{}",
        bad_fetch_text
    );
    assert!(!bob_out.join("badfetch.bin").exists());

    let resume_ok = run_receive(
        &bob_cfg,
        relay.base_url(),
        ROUTE_TOKEN_BOB,
        &bob_out,
        Some(service.base_url()),
        false,
    );
    assert!(resume_ok.status.success(), "{}", output_text(&resume_ok));
    assert!(bob_out.join("badfetch.bin").exists());

    let forged = base.join("forged_attachment.json");
    forged_attachment_descriptor(&forged, true);
    let raw = run_raw_send(&alice_cfg, relay.base_url(), "bob", &forged);
    assert!(raw.status.success(), "{}", output_text(&raw));

    let bad_ctx_out = base.join("bad_ctx_out");
    create_dir_700(&bad_ctx_out);
    let bad_ctx = run_receive(
        &bob_cfg,
        relay.base_url(),
        ROUTE_TOKEN_BOB,
        &bad_ctx_out,
        Some(service.base_url()),
        false,
    );
    assert!(!bad_ctx.status.success(), "{}", output_text(&bad_ctx));
    let bad_ctx_text = output_text(&bad_ctx);
    assert!(
        bad_ctx_text.contains("REJECT_ATT_DESC_ENC_CTX"),
        "{}",
        bad_ctx_text
    );
    assert!(
        fs::read_dir(&bad_ctx_out).unwrap().next().is_none(),
        "malformed enc_ctx must not persist plaintext output"
    );
}

#[test]
fn attachment_path_coexists_with_legacy_below_threshold() {
    let _guard = attachment_test_guard();
    let relay = common::start_inbox_server(2 * 1024 * 1024, 256);
    let service = common::start_attachment_server(100 * 1024 * 1024);
    let base = safe_test_root().join(format!("na0197c_legacy_{}", std::process::id()));
    create_dir_700(&base);
    let (alice_cfg, _bob_cfg, _alice_out, _bob_out) = setup_pair(&base);
    let payload = base.join("small.bin");
    fs::write(&payload, vec![0x77; 24_576]).unwrap();

    let send = run_attachment_send(
        &alice_cfg,
        relay.base_url(),
        service.base_url(),
        "bob",
        &payload,
        false,
    );
    assert!(send.status.success(), "{}", output_text(&send));
    let send_text = output_text(&send);
    assert!(
        send_text.contains("event=file_xfer_prepare"),
        "{}",
        send_text
    );
    assert!(
        send_text.contains("event=file_xfer_manifest"),
        "{}",
        send_text
    );
    assert!(
        !send_text.contains("event=attachment_service_commit"),
        "{}",
        send_text
    );

    assert_no_secretish_output(&send_text);
}

#[test]
#[ignore = "local large-file proof only"]
fn attachment_large_local_roundtrip_proof() {
    let _guard = attachment_test_guard();
    let relay = common::start_inbox_server(4 * 1024 * 1024, 1024);
    let service = common::start_attachment_server(256 * 1024 * 1024);
    let base = safe_test_root().join(format!("na0197c_large_{}", std::process::id()));
    create_dir_700(&base);
    let (alice_cfg, bob_cfg, alice_out, bob_out) = setup_pair(&base);
    let size = std::env::var("QSC_ATTACHMENT_LARGE_BYTES")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(64 * 1024 * 1024);
    let payload = base.join("large-proof.bin");
    write_repeated_file(&payload, size, 0x6b);

    let send = run_attachment_send(
        &alice_cfg,
        relay.base_url(),
        service.base_url(),
        "bob",
        &payload,
        true,
    );
    assert!(send.status.success(), "{}", output_text(&send));
    let send_text = output_text(&send);
    assert!(
        send_text.contains("event=attachment_service_commit"),
        "{}",
        send_text
    );
    assert_no_secretish_output(&send_text);

    let max_file_size = size;
    let max_file_chunks = 4096usize;
    let bob_recv = run_receive_with_bounds(
        &bob_cfg,
        relay.base_url(),
        ROUTE_TOKEN_BOB,
        &bob_out,
        Some(service.base_url()),
        true,
        ReceiveBounds {
            max_file_size,
            max_file_chunks,
        },
    );
    assert!(bob_recv.status.success(), "{}", output_text(&bob_recv));
    let bob_text = output_text(&bob_recv);
    assert!(bob_text.contains("attachment_confirm_send"), "{}", bob_text);
    assert_no_secretish_output(&bob_text);

    let alice_recv = run_receive(
        &alice_cfg,
        relay.base_url(),
        ROUTE_TOKEN_BOB,
        &alice_out,
        None,
        false,
    );
    assert!(alice_recv.status.success(), "{}", output_text(&alice_recv));
    let alice_text = output_text(&alice_recv);
    assert!(
        alice_text.contains("QSC_FILE_DELIVERY state=peer_confirmed"),
        "{}",
        alice_text
    );

    let received = bob_out.join("large-proof.bin");
    assert!(received.exists(), "{}", bob_text);
    assert_eq!(file_sha256_hex(&payload), file_sha256_hex(&received));
}
