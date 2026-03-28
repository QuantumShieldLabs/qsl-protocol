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

fn collect_files(root: &Path) -> Vec<(String, Vec<u8>)> {
    fn walk(base: &Path, cur: &Path, out: &mut Vec<(String, Vec<u8>)>) {
        let Ok(rd) = fs::read_dir(cur) else {
            return;
        };
        let mut ents: Vec<_> = rd.filter_map(Result::ok).collect();
        ents.sort_by_key(|e| e.path());
        for e in ents {
            let path = e.path();
            if path.is_dir() {
                walk(base, &path, out);
                continue;
            }
            if path.is_file() {
                let rel = path
                    .strip_prefix(base)
                    .unwrap_or(&path)
                    .to_string_lossy()
                    .to_string();
                let bytes = fs::read(&path).unwrap_or_default();
                out.push((rel, bytes));
            }
        }
    }

    let mut out = Vec::new();
    walk(root, root, &mut out);
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
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
    run_attachment_send_with_config(
        cfg,
        relay,
        to,
        path,
        AttachmentSendConfig {
            explicit_attachment_service: Some(service),
            with_receipt,
            ..AttachmentSendConfig::default()
        },
    )
}

#[derive(Clone, Copy, Default)]
struct AttachmentSendConfig<'a> {
    validated_attachment_service: Option<&'a str>,
    explicit_attachment_service: Option<&'a str>,
    env_stage: Option<&'a str>,
    arg_stage: Option<&'a str>,
    with_receipt: bool,
}

fn run_attachment_send_with_config(
    cfg: &Path,
    relay: &str,
    to: &str,
    path: &Path,
    send_cfg: AttachmentSendConfig<'_>,
) -> std::process::Output {
    let AttachmentSendConfig {
        validated_attachment_service,
        explicit_attachment_service,
        env_stage,
        arg_stage,
        with_receipt,
    } = send_cfg;
    let mut cmd = qsc_base(cfg);
    if let Some(service) = validated_attachment_service {
        cmd.env("QSC_ATTACHMENT_SERVICE", service);
    }
    if let Some(stage) = env_stage {
        cmd.env("QSC_LEGACY_IN_MESSAGE_STAGE", stage);
    }
    cmd.args([
        "file",
        "send",
        "--transport",
        "relay",
        "--relay",
        relay,
        "--to",
        to,
        "--path",
        path.to_str().unwrap(),
    ]);
    if let Some(service) = explicit_attachment_service {
        cmd.args(["--attachment-service", service]);
    }
    if let Some(stage) = arg_stage {
        cmd.args(["--legacy-in-message-stage", stage]);
    }
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
    run_receive_with_legacy_mode(
        cfg,
        relay,
        mailbox,
        out_dir,
        attachment_service,
        emit_receipts,
        None,
    )
}

fn run_receive_with_legacy_mode(
    cfg: &Path,
    relay: &str,
    mailbox: &str,
    out_dir: &Path,
    attachment_service: Option<&str>,
    emit_receipts: bool,
    legacy_receive_mode: Option<&str>,
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
    if let Some(mode) = legacy_receive_mode {
        cmd.args(["--legacy-receive-mode", mode]);
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

fn staged_outbound_attachment_id(cfg: &Path) -> String {
    let outbound = cfg.join("attachments").join("outbound");
    let mut ids = Vec::new();
    let entries = fs::read_dir(&outbound)
        .unwrap_or_else(|e| panic!("read outbound staging dir {}: {e}", outbound.display()));
    for entry in entries {
        let entry = entry.unwrap_or_else(|e| panic!("read outbound entry: {e}"));
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let Some(name) = path.file_name().and_then(|value| value.to_str()) else {
            continue;
        };
        if let Some(id) = name.strip_suffix(".cipher") {
            ids.push(id.to_string());
        }
    }
    ids.sort();
    assert_eq!(
        ids.len(),
        1,
        "expected one staged outbound attachment: {ids:?}"
    );
    ids.pop().unwrap()
}

fn assert_no_attachment_id_leak(text: &str, attachment_id: &str) {
    assert!(
        !text.contains(attachment_id),
        "forbidden attachment_id leaked\n{text}"
    );
}

fn assert_no_descriptor_identifier_leaks(text: &str, descriptor: &serde_json::Value) {
    for field in [
        "attachment_id",
        "locator_ref",
        "fetch_capability",
        "enc_ctx_b64u",
    ] {
        let value = descriptor[field]
            .as_str()
            .unwrap_or_else(|| panic!("missing descriptor field: {field}"));
        assert!(
            !text.contains(value),
            "forbidden descriptor field leaked: {field}\n{text}"
        );
    }
}

fn assert_file_send_policy(text: &str, stage: &str, size_class: &str) {
    let stage_marker = format!("stage={stage}");
    let size_class_marker = format!("size_class={size_class}");
    assert!(
        text.contains("event=file_send_policy")
            && text.contains(stage_marker.as_str())
            && text.contains(size_class_marker.as_str()),
        "{}",
        text
    );
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
    let attachment_id = staged_outbound_attachment_id(&alice_cfg);
    assert_no_attachment_id_leak(&send_text, &attachment_id);
    assert!(
        !send_text.contains(service.base_url()),
        "attachment service URL leaked in send output: {}",
        send_text
    );

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
    assert_no_attachment_id_leak(&bob_full_text, &attachment_id);
    assert!(
        !bob_full_text.contains(service.base_url()),
        "attachment service URL leaked in receive output: {}",
        bob_full_text
    );

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
    assert_no_secretish_output(&alice_after_confirm_text);
    assert_no_attachment_id_leak(&alice_after_confirm_text, &attachment_id);
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
    let forged_descriptor: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&forged).unwrap()).unwrap();
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
    assert_no_descriptor_identifier_leaks(&bad_ctx_text, &forged_descriptor);
    assert!(
        fs::read_dir(&bad_ctx_out).unwrap().next().is_none(),
        "malformed enc_ctx must not persist plaintext output"
    );
}

#[test]
fn validated_post_w0_send_rejects_explicit_w0_env_override() {
    let _guard = attachment_test_guard();
    let relay = common::start_inbox_server(2 * 1024 * 1024, 256);
    let service = common::start_attachment_server(100 * 1024 * 1024);
    let base = safe_test_root().join(format!("na0197c_legacy_{}", std::process::id()));
    create_dir_700(&base);
    let (alice_cfg, _bob_cfg, _alice_out, _bob_out) = setup_pair(&base);
    let payload = base.join("small.bin");
    fs::write(&payload, vec![0x77; 24_576]).unwrap();

    let send = run_attachment_send_with_config(
        &alice_cfg,
        relay.base_url(),
        "bob",
        &payload,
        AttachmentSendConfig {
            validated_attachment_service: Some(service.base_url()),
            env_stage: Some("w0"),
            ..AttachmentSendConfig::default()
        },
    );
    assert!(!send.status.success(), "{}", output_text(&send));
    let send_text = output_text(&send);
    assert!(
        send_text.contains("legacy_in_message_stage_retired_post_w0"),
        "{}",
        send_text
    );
    assert!(
        !send_text.contains("event=file_send_policy"),
        "{}",
        send_text
    );
    assert!(
        !send_text.contains("event=file_xfer_prepare"),
        "{}",
        send_text
    );
    assert!(
        !send_text.contains(service.base_url()),
        "attachment service URL leaked in legacy send output: {}",
        send_text
    );

    assert_no_secretish_output(&send_text);
}

#[test]
fn legacy_path_roundtrip_rejects_then_confirms_without_false_peer_confirmed() {
    let _guard = attachment_test_guard();
    let relay = common::start_inbox_server(2 * 1024 * 1024, 512);
    let service = common::start_attachment_server(100 * 1024 * 1024);
    let base = safe_test_root().join(format!("na0199_legacy_roundtrip_{}", std::process::id()));
    create_dir_700(&base);
    let (alice_cfg, bob_cfg, alice_out, bob_out) = setup_pair(&base);
    let payload = base.join("legacy-small.bin");
    fs::write(&payload, vec![0x52; 24_576]).unwrap();

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
    assert!(
        send_text.contains("QSC_FILE_DELIVERY state=accepted_by_relay"),
        "{}",
        send_text
    );
    assert!(!send_text.contains("state=peer_confirmed"), "{}", send_text);
    assert_no_secretish_output(&send_text);

    let bob_reject = run_receive_with_bounds(
        &bob_cfg,
        relay.base_url(),
        ROUTE_TOKEN_BOB,
        &bob_out,
        None,
        true,
        ReceiveBounds {
            max_file_size: 8_192,
            max_file_chunks: 64,
        },
    );
    assert!(!bob_reject.status.success(), "{}", output_text(&bob_reject));
    let bob_reject_text = output_text(&bob_reject);
    assert!(
        bob_reject_text.contains("size_exceeds_max"),
        "{}",
        bob_reject_text
    );
    assert_no_secretish_output(&bob_reject_text);

    let bob_list_before = qsc_base(&bob_cfg)
        .args(["timeline", "list", "--peer", "bob", "--limit", "10"])
        .output()
        .expect("bob timeline list before success");
    assert!(
        bob_list_before.status.success(),
        "{}",
        output_text(&bob_list_before)
    );
    assert!(
        output_text(&bob_list_before).contains("event=timeline_list count=0 peer=bob"),
        "{}",
        output_text(&bob_list_before)
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
    let alice_before_confirm_text = output_text(&alice_before_confirm);
    assert!(
        alice_before_confirm_text.contains("event=recv_none"),
        "{}",
        alice_before_confirm_text
    );
    assert!(
        !alice_before_confirm_text.contains("state=peer_confirmed"),
        "{}",
        alice_before_confirm_text
    );

    let resend = run_attachment_send(
        &alice_cfg,
        relay.base_url(),
        service.base_url(),
        "bob",
        &payload,
        true,
    );
    assert!(resend.status.success(), "{}", output_text(&resend));
    assert_no_secretish_output(&output_text(&resend));

    let bob_ok = run_receive(
        &bob_cfg,
        relay.base_url(),
        ROUTE_TOKEN_BOB,
        &bob_out,
        None,
        true,
    );
    assert!(bob_ok.status.success(), "{}", output_text(&bob_ok));
    let bob_ok_text = output_text(&bob_ok);
    assert!(
        bob_ok_text.contains("event=file_xfer_complete"),
        "{}",
        bob_ok_text
    );
    assert!(
        bob_ok_text.contains("event=file_confirm_send"),
        "{}",
        bob_ok_text
    );
    assert_no_secretish_output(&bob_ok_text);

    let bob_list_after = qsc_base(&bob_cfg)
        .args(["timeline", "list", "--peer", "bob", "--limit", "10"])
        .output()
        .expect("bob timeline list after success");
    assert!(
        bob_list_after.status.success(),
        "{}",
        output_text(&bob_list_after)
    );
    let bob_list_after_text = output_text(&bob_list_after);
    assert!(
        bob_list_after_text.contains("event=timeline_list count=1 peer=bob"),
        "{}",
        bob_list_after_text
    );
    assert!(
        bob_list_after_text.contains("kind=file"),
        "{}",
        bob_list_after_text
    );
    assert!(
        bob_list_after_text.contains("state=RECEIVED"),
        "{}",
        bob_list_after_text
    );

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
fn w2_threshold_boundary_and_service_requirement_are_explicit() {
    let _guard = attachment_test_guard();
    let relay = common::start_inbox_server(4 * 1024 * 1024, 1024);
    let service = common::start_attachment_server(100 * 1024 * 1024);
    let base = safe_test_root().join(format!("na0199_threshold_{}", std::process::id()));
    create_dir_700(&base);
    let (alice_cfg, _bob_cfg, _alice_out, _bob_out) = setup_pair(&base);

    let threshold_payload = base.join("threshold.bin");
    write_repeated_file(&threshold_payload, 4 * 1024 * 1024, 0x61);
    let threshold_send = qsc_base(&alice_cfg)
        .env("QSC_ATTACHMENT_SERVICE", service.base_url())
        .args([
            "file",
            "send",
            "--transport",
            "relay",
            "--relay",
            relay.base_url(),
            "--to",
            "bob",
            "--path",
            threshold_payload.to_str().unwrap(),
            "--max-file-size",
            "4194304",
            "--max-chunks",
            "256",
        ])
        .output()
        .expect("threshold w2 send");
    assert!(
        threshold_send.status.success(),
        "{}",
        output_text(&threshold_send)
    );
    let threshold_text = output_text(&threshold_send);
    assert_file_send_policy(&threshold_text, "w2", "legacy_sized");
    assert!(
        threshold_text.contains("event=attachment_service_commit"),
        "{}",
        threshold_text
    );
    assert!(
        !threshold_text.contains("event=file_xfer_manifest"),
        "{}",
        threshold_text
    );
    assert!(
        !threshold_text.contains("state=peer_confirmed"),
        "{}",
        threshold_text
    );
    assert!(
        !threshold_text.contains(service.base_url()),
        "attachment service URL leaked in threshold legacy output: {}",
        threshold_text
    );
    assert_no_secretish_output(&threshold_text);

    let above_payload = base.join("threshold-plus-one.bin");
    write_repeated_file(&above_payload, 4 * 1024 * 1024 + 1, 0x62);

    let no_service = qsc_base(&alice_cfg)
        .args([
            "file",
            "send",
            "--transport",
            "relay",
            "--relay",
            relay.base_url(),
            "--to",
            "bob",
            "--path",
            above_payload.to_str().unwrap(),
        ])
        .output()
        .expect("above-threshold send without service");
    assert!(!no_service.status.success(), "{}", output_text(&no_service));
    let no_service_text = output_text(&no_service);
    assert!(
        no_service_text.contains("attachment_service_required"),
        "{}",
        no_service_text
    );
    assert_no_secretish_output(&no_service_text);

    let with_service = run_attachment_send_with_config(
        &alice_cfg,
        relay.base_url(),
        "bob",
        &above_payload,
        AttachmentSendConfig {
            validated_attachment_service: Some(service.base_url()),
            ..AttachmentSendConfig::default()
        },
    );
    assert!(
        with_service.status.success(),
        "{}",
        output_text(&with_service)
    );
    let with_service_text = output_text(&with_service);
    assert_file_send_policy(&with_service_text, "w2", "above_threshold");
    assert!(
        with_service_text.contains("event=attachment_service_commit"),
        "{}",
        with_service_text
    );
    assert!(
        !with_service_text.contains("event=file_xfer_manifest"),
        "{}",
        with_service_text
    );
    assert!(
        !with_service_text.contains("state=peer_confirmed"),
        "{}",
        with_service_text
    );
    assert!(
        !with_service_text.contains(service.base_url()),
        "attachment service URL leaked in default-selection output: {}",
        with_service_text
    );
    assert_no_secretish_output(&with_service_text);
}

#[test]
fn explicit_override_wins_and_attachment_rejects_do_not_fallback_to_legacy() {
    let _guard = attachment_test_guard();
    let relay = common::start_inbox_server(4 * 1024 * 1024, 1024);
    let rejecting_service = common::start_attachment_server(512 * 1024);
    let good_service = common::start_attachment_server(100 * 1024 * 1024);
    let base = safe_test_root().join(format!("na0202a_override_{}", std::process::id()));
    create_dir_700(&base);
    let (alice_cfg, _bob_cfg, _alice_out, _bob_out) = setup_pair(&base);

    let payload = base.join("default-plus-one.bin");
    write_repeated_file(&payload, 4 * 1024 * 1024 + 1, 0x63);

    let reject = run_attachment_send_with_config(
        &alice_cfg,
        relay.base_url(),
        "bob",
        &payload,
        AttachmentSendConfig {
            validated_attachment_service: Some(rejecting_service.base_url()),
            ..AttachmentSendConfig::default()
        },
    );
    assert!(!reject.status.success(), "{}", output_text(&reject));
    let reject_text = output_text(&reject);
    assert_file_send_policy(&reject_text, "w2", "above_threshold");
    assert!(
        reject_text.contains("event=file_xfer_reject"),
        "{}",
        reject_text
    );
    assert!(
        !reject_text.contains("event=file_xfer_manifest"),
        "{}",
        reject_text
    );
    assert!(
        !reject_text.contains("event=attachment_service_commit"),
        "{}",
        reject_text
    );
    assert!(
        !reject_text.contains("state=accepted_by_relay"),
        "{}",
        reject_text
    );
    assert!(
        !reject_text.contains("state=peer_confirmed"),
        "{}",
        reject_text
    );
    assert!(
        !reject_text.contains(rejecting_service.base_url()),
        "attachment service URL leaked in reject output: {}",
        reject_text
    );
    assert_no_secretish_output(&reject_text);

    let override_ok = run_attachment_send_with_config(
        &alice_cfg,
        relay.base_url(),
        "bob",
        &payload,
        AttachmentSendConfig {
            validated_attachment_service: Some(rejecting_service.base_url()),
            explicit_attachment_service: Some(good_service.base_url()),
            ..AttachmentSendConfig::default()
        },
    );
    assert!(
        override_ok.status.success(),
        "{}",
        output_text(&override_ok)
    );
    let override_text = output_text(&override_ok);
    assert_file_send_policy(&override_text, "w2", "above_threshold");
    assert!(
        override_text.contains("event=attachment_service_commit"),
        "{}",
        override_text
    );
    assert!(
        !override_text.contains("event=file_xfer_manifest"),
        "{}",
        override_text
    );
    assert!(
        !override_text.contains(rejecting_service.base_url()),
        "validated config URL leaked in override output: {}",
        override_text
    );
    assert!(
        !override_text.contains(good_service.base_url()),
        "override URL leaked in override output: {}",
        override_text
    );
    assert_no_secretish_output(&override_text);
}

#[test]
fn w2_legacy_sized_selection_is_default_for_small_and_threshold_files() {
    let _guard = attachment_test_guard();
    let relay = common::start_inbox_server(2 * 1024 * 1024, 256);
    let service = common::start_attachment_server(100 * 1024 * 1024);
    let base = safe_test_root().join(format!("na0205a_w2_select_{}", std::process::id()));
    create_dir_700(&base);
    let (alice_cfg, _bob_cfg, _alice_out, _bob_out) = setup_pair(&base);

    let small_payload = base.join("small-w2.bin");
    write_repeated_file(&small_payload, 1_048_576, 0x71);
    let small_send = run_attachment_send_with_config(
        &alice_cfg,
        relay.base_url(),
        "bob",
        &small_payload,
        AttachmentSendConfig {
            validated_attachment_service: Some(service.base_url()),
            ..AttachmentSendConfig::default()
        },
    );
    assert!(small_send.status.success(), "{}", output_text(&small_send));
    let small_text = output_text(&small_send);
    assert_file_send_policy(&small_text, "w2", "legacy_sized");
    assert!(
        small_text.contains("event=attachment_service_commit"),
        "{}",
        small_text
    );
    assert!(
        !small_text.contains("event=file_xfer_manifest"),
        "{}",
        small_text
    );
    assert!(
        !small_text.contains(service.base_url()),
        "attachment service URL leaked in W2 small-file output: {}",
        small_text
    );
    assert_no_secretish_output(&small_text);

    let threshold_payload = base.join("threshold-w2.bin");
    write_repeated_file(&threshold_payload, 4 * 1024 * 1024, 0x72);
    let threshold_send = run_attachment_send_with_config(
        &alice_cfg,
        relay.base_url(),
        "bob",
        &threshold_payload,
        AttachmentSendConfig {
            validated_attachment_service: Some(service.base_url()),
            ..AttachmentSendConfig::default()
        },
    );
    assert!(
        threshold_send.status.success(),
        "{}",
        output_text(&threshold_send)
    );
    let threshold_text = output_text(&threshold_send);
    assert_file_send_policy(&threshold_text, "w2", "legacy_sized");
    assert!(
        threshold_text.contains("event=attachment_service_commit"),
        "{}",
        threshold_text
    );
    assert!(
        !threshold_text.contains("event=file_xfer_manifest"),
        "{}",
        threshold_text
    );
    assert!(
        !threshold_text.contains(service.base_url()),
        "attachment service URL leaked in W2 threshold output: {}",
        threshold_text
    );
    assert_no_secretish_output(&threshold_text);
}

#[test]
fn w2_missing_service_fails_closed_without_legacy_fallback_when_selected_explicitly() {
    let _guard = attachment_test_guard();
    let relay = common::start_inbox_server(2 * 1024 * 1024, 256);
    let base = safe_test_root().join(format!("na0205a_w2_missing_{}", std::process::id()));
    create_dir_700(&base);
    let (alice_cfg, _bob_cfg, _alice_out, _bob_out) = setup_pair(&base);

    let payload = base.join("missing-service.bin");
    write_repeated_file(&payload, 262_144, 0x73);
    let reject = run_attachment_send_with_config(
        &alice_cfg,
        relay.base_url(),
        "bob",
        &payload,
        AttachmentSendConfig {
            arg_stage: Some("w2"),
            ..AttachmentSendConfig::default()
        },
    );
    assert!(!reject.status.success(), "{}", output_text(&reject));
    let reject_text = output_text(&reject);
    assert_file_send_policy(&reject_text, "w2", "legacy_sized");
    assert!(
        reject_text.contains("attachment_service_required"),
        "{}",
        reject_text
    );
    assert!(
        !reject_text.contains("event=file_xfer_manifest"),
        "{}",
        reject_text
    );
    assert!(
        !reject_text.contains("event=attachment_service_commit"),
        "{}",
        reject_text
    );
    assert!(
        !reject_text.contains("state=accepted_by_relay"),
        "{}",
        reject_text
    );
    assert_no_secretish_output(&reject_text);
}

#[test]
fn w2_attachment_rejects_do_not_fallback_to_legacy() {
    let _guard = attachment_test_guard();
    let relay = common::start_inbox_server(2 * 1024 * 1024, 256);
    let rejecting_service = common::start_attachment_server(512 * 1024);
    let base = safe_test_root().join(format!("na0205a_w2_reject_{}", std::process::id()));
    create_dir_700(&base);
    let (alice_cfg, _bob_cfg, _alice_out, _bob_out) = setup_pair(&base);

    let payload = base.join("reject.bin");
    write_repeated_file(&payload, 1_048_576, 0x74);
    let reject = run_attachment_send_with_config(
        &alice_cfg,
        relay.base_url(),
        "bob",
        &payload,
        AttachmentSendConfig {
            validated_attachment_service: Some(rejecting_service.base_url()),
            ..AttachmentSendConfig::default()
        },
    );
    assert!(!reject.status.success(), "{}", output_text(&reject));
    let reject_text = output_text(&reject);
    assert_file_send_policy(&reject_text, "w2", "legacy_sized");
    assert!(
        reject_text.contains("event=file_xfer_reject"),
        "{}",
        reject_text
    );
    assert!(
        !reject_text.contains("event=file_xfer_manifest"),
        "{}",
        reject_text
    );
    assert!(
        !reject_text.contains("event=attachment_service_commit"),
        "{}",
        reject_text
    );
    assert!(
        !reject_text.contains("state=accepted_by_relay"),
        "{}",
        reject_text
    );
    assert!(
        !reject_text.contains("state=peer_confirmed"),
        "{}",
        reject_text
    );
    assert!(
        !reject_text.contains(rejecting_service.base_url()),
        "attachment service URL leaked in W2 reject output: {}",
        reject_text
    );
    assert_no_secretish_output(&reject_text);
}

#[test]
fn explicit_send_attachment_service_override_does_not_activate_post_w0_defaults() {
    let _guard = attachment_test_guard();
    let relay = common::start_inbox_server(2 * 1024 * 1024, 256);
    let service = common::start_attachment_server(100 * 1024 * 1024);
    let base = safe_test_root().join(format!("na0209_send_arg_only_{}", std::process::id()));
    create_dir_700(&base);
    let (alice_cfg, _bob_cfg, _alice_out, _bob_out) = setup_pair(&base);

    let payload = base.join("legacy-small.bin");
    write_repeated_file(&payload, 196_608, 0x75);
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
    assert_file_send_policy(&send_text, "w0", "legacy_sized");
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
    assert!(!send_text.contains("state=peer_confirmed"), "{}", send_text);
    assert!(
        !send_text.contains(service.base_url()),
        "attachment service URL leaked in explicit send override output: {}",
        send_text
    );
    assert_no_secretish_output(&send_text);
}

#[test]
fn validated_post_w0_receive_rejects_explicit_coexistence_mode() {
    let _guard = attachment_test_guard();
    let relay = common::start_inbox_server(2 * 1024 * 1024, 256);
    let service = common::start_attachment_server(100 * 1024 * 1024);
    let base = safe_test_root().join(format!("na0209_recv_mode_reject_{}", std::process::id()));
    create_dir_700(&base);
    let (alice_cfg, bob_cfg, _alice_out, bob_out) = setup_pair(&base);

    let legacy_payload = base.join("legacy-small.bin");
    fs::write(&legacy_payload, vec![0x76; 24_576]).unwrap();
    let legacy_send = run_attachment_send(
        &alice_cfg,
        relay.base_url(),
        service.base_url(),
        "bob",
        &legacy_payload,
        true,
    );
    assert!(
        legacy_send.status.success(),
        "{}",
        output_text(&legacy_send)
    );
    let legacy_send_text = output_text(&legacy_send);
    assert!(
        legacy_send_text.contains("event=file_xfer_manifest"),
        "{}",
        legacy_send_text
    );

    let bob_recv = run_receive_with_legacy_mode(
        &bob_cfg,
        relay.base_url(),
        ROUTE_TOKEN_BOB,
        &bob_out,
        Some(service.base_url()),
        true,
        Some("coexistence"),
    );
    assert!(!bob_recv.status.success(), "{}", output_text(&bob_recv));
    let bob_text = output_text(&bob_recv);
    assert!(
        bob_text.contains("legacy_receive_mode_retired_post_w0"),
        "{}",
        bob_text
    );
    assert!(!bob_text.contains("event=recv_start"), "{}", bob_text);
    assert!(
        fs::read_dir(&bob_out).unwrap().next().is_none(),
        "{}",
        bob_text
    );
    assert!(
        !legacy_send_text.contains("state=peer_confirmed"),
        "{}",
        legacy_send_text
    );
}

#[test]
fn validated_post_w0_send_rejects_explicit_w0_arg_override() {
    let _guard = attachment_test_guard();
    let relay = common::start_inbox_server(2 * 1024 * 1024, 256);
    let service = common::start_attachment_server(100 * 1024 * 1024);
    let base = safe_test_root().join(format!("na0205a_w0_override_{}", std::process::id()));
    create_dir_700(&base);
    let (alice_cfg, _bob_cfg, _alice_out, _bob_out) = setup_pair(&base);

    let payload = base.join("override.bin");
    write_repeated_file(&payload, 196_608, 0x77);
    let send = run_attachment_send_with_config(
        &alice_cfg,
        relay.base_url(),
        "bob",
        &payload,
        AttachmentSendConfig {
            validated_attachment_service: Some(service.base_url()),
            env_stage: Some("w2"),
            arg_stage: Some("w0"),
            ..AttachmentSendConfig::default()
        },
    );
    assert!(!send.status.success(), "{}", output_text(&send));
    let send_text = output_text(&send);
    assert!(
        send_text.contains("legacy_in_message_stage_retired_post_w0"),
        "{}",
        send_text
    );
    assert!(
        !send_text.contains("event=file_send_policy"),
        "{}",
        send_text
    );
    assert_no_secretish_output(&send_text);
}

#[test]
fn validated_post_w0_receive_defaults_to_retired_and_keeps_attachment_descriptor_receive() {
    let _guard = attachment_test_guard();
    let relay = common::start_inbox_server(2 * 1024 * 1024, 512);
    let service = common::start_attachment_server(100 * 1024 * 1024);
    let base = safe_test_root().join(format!("na0205a_mixed_recv_{}", std::process::id()));
    create_dir_700(&base);
    let (alice_cfg, bob_cfg, alice_out, bob_out) = setup_pair(&base);

    let legacy_payload = base.join("legacy-small.bin");
    fs::write(&legacy_payload, vec![0x52; 24_576]).unwrap();
    let legacy_send = run_attachment_send(
        &alice_cfg,
        relay.base_url(),
        service.base_url(),
        "bob",
        &legacy_payload,
        true,
    );
    assert!(
        legacy_send.status.success(),
        "{}",
        output_text(&legacy_send)
    );
    let legacy_send_text = output_text(&legacy_send);
    assert_file_send_policy(&legacy_send_text, "w0", "legacy_sized");
    assert!(
        legacy_send_text.contains("event=file_xfer_manifest"),
        "{}",
        legacy_send_text
    );
    assert!(
        !legacy_send_text.contains("event=attachment_service_commit"),
        "{}",
        legacy_send_text
    );

    let bob_legacy_reject = run_receive(
        &bob_cfg,
        relay.base_url(),
        ROUTE_TOKEN_BOB,
        &bob_out,
        Some(service.base_url()),
        true,
    );
    assert!(
        !bob_legacy_reject.status.success(),
        "{}",
        output_text(&bob_legacy_reject)
    );
    let bob_legacy_reject_text = output_text(&bob_legacy_reject);
    assert!(
        bob_legacy_reject_text
            .contains("event=legacy_receive_reject code=legacy_receive_retired_post_w0"),
        "{}",
        bob_legacy_reject_text
    );
    assert!(
        !bob_legacy_reject_text.contains("event=file_confirm_send"),
        "{}",
        bob_legacy_reject_text
    );
    assert!(
        !bob_legacy_reject_text.contains("state=peer_confirmed"),
        "{}",
        bob_legacy_reject_text
    );
    assert!(
        fs::read_dir(&bob_out).unwrap().next().is_none(),
        "{}",
        bob_legacy_reject_text
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
    let alice_before_confirm_text = output_text(&alice_before_confirm);
    assert!(
        alice_before_confirm_text.contains("event=recv_none"),
        "{}",
        alice_before_confirm_text
    );
    assert!(
        !alice_before_confirm_text.contains("state=peer_confirmed"),
        "{}",
        alice_before_confirm_text
    );

    let attachment_payload = base.join("attachment.bin");
    write_repeated_file(&attachment_payload, 1_048_576, 0x79);
    let attachment_send = run_attachment_send_with_config(
        &alice_cfg,
        relay.base_url(),
        "bob",
        &attachment_payload,
        AttachmentSendConfig {
            validated_attachment_service: Some(service.base_url()),
            ..AttachmentSendConfig::default()
        },
    );
    assert!(
        attachment_send.status.success(),
        "{}",
        output_text(&attachment_send)
    );
    let attachment_send_text = output_text(&attachment_send);
    assert_file_send_policy(&attachment_send_text, "w2", "legacy_sized");
    assert!(
        attachment_send_text.contains("event=attachment_service_commit"),
        "{}",
        attachment_send_text
    );

    let bob_attachment_recv = run_receive(
        &bob_cfg,
        relay.base_url(),
        ROUTE_TOKEN_BOB,
        &bob_out,
        Some(service.base_url()),
        false,
    );
    assert!(
        bob_attachment_recv.status.success(),
        "{}",
        output_text(&bob_attachment_recv)
    );
    let bob_attachment_text = output_text(&bob_attachment_recv);
    assert!(
        bob_attachment_text.contains("event=qsp_unpack ok=true")
            && bob_attachment_text.contains("event=message_state_transition")
            && bob_attachment_text.contains("to=RECEIVED"),
        "{}",
        bob_attachment_text
    );
    assert_eq!(
        file_sha256_hex(&attachment_payload),
        file_sha256_hex(&bob_out.join("attachment.bin"))
    );

    let bob_list = qsc_base(&bob_cfg)
        .args(["timeline", "list", "--peer", "bob", "--limit", "10"])
        .output()
        .expect("bob timeline list after mixed compatibility");
    assert!(bob_list.status.success(), "{}", output_text(&bob_list));
    let bob_list_text = output_text(&bob_list);
    assert!(
        bob_list_text.contains("event=timeline_list count=1 peer=bob"),
        "{}",
        bob_list_text
    );
    assert!(bob_list_text.contains("kind=file"), "{}", bob_list_text);
    assert!(
        bob_list_text.contains("state=RECEIVED"),
        "{}",
        bob_list_text
    );
}

#[test]
fn post_w0_legacy_receive_retirement_fails_closed_without_mutation_or_false_peer_confirmed() {
    let _guard = attachment_test_guard();
    let relay = common::start_inbox_server(2 * 1024 * 1024, 512);
    let service = common::start_attachment_server(100 * 1024 * 1024);
    let base = safe_test_root().join(format!("na0207_post_w0_recv_{}", std::process::id()));
    create_dir_700(&base);
    let (alice_cfg, bob_cfg, alice_out, bob_out) = setup_pair(&base);

    let legacy_payload = base.join("legacy-post-w0.bin");
    fs::write(&legacy_payload, vec![0x57; 24_576]).unwrap();
    let legacy_send = run_attachment_send(
        &alice_cfg,
        relay.base_url(),
        service.base_url(),
        "bob",
        &legacy_payload,
        true,
    );
    assert!(
        legacy_send.status.success(),
        "{}",
        output_text(&legacy_send)
    );
    let legacy_send_text = output_text(&legacy_send);
    assert_file_send_policy(&legacy_send_text, "w0", "legacy_sized");
    assert!(
        legacy_send_text.contains("event=file_xfer_manifest"),
        "{}",
        legacy_send_text
    );

    let bob_before = collect_files(&bob_cfg);
    let bob_retired = run_receive_with_legacy_mode(
        &bob_cfg,
        relay.base_url(),
        ROUTE_TOKEN_BOB,
        &bob_out,
        Some(service.base_url()),
        true,
        Some("retired"),
    );
    assert!(
        !bob_retired.status.success(),
        "{}",
        output_text(&bob_retired)
    );
    let bob_retired_text = output_text(&bob_retired);
    assert!(
        bob_retired_text.contains("event=legacy_receive_reject"),
        "{}",
        bob_retired_text
    );
    assert!(
        bob_retired_text.contains("mode=retired"),
        "{}",
        bob_retired_text
    );
    assert!(
        bob_retired_text.contains("payload_type=file_chunk"),
        "{}",
        bob_retired_text
    );
    assert!(
        bob_retired_text
            .contains("event=legacy_receive_reject code=legacy_receive_retired_post_w0"),
        "{}",
        bob_retired_text
    );
    assert!(
        bob_retired_text.contains("event=file_xfer_reject code=legacy_receive_retired_post_w0"),
        "{}",
        bob_retired_text
    );
    assert!(
        bob_retired_text.contains("event=error code=legacy_receive_retired_post_w0"),
        "{}",
        bob_retired_text
    );
    assert!(
        !bob_retired_text.contains("event=file_xfer_complete"),
        "{}",
        bob_retired_text
    );
    assert!(
        !bob_retired_text.contains("event=file_confirm_send"),
        "{}",
        bob_retired_text
    );
    assert!(
        !bob_retired_text.contains("state=peer_confirmed"),
        "{}",
        bob_retired_text
    );
    assert_no_secretish_output(&bob_retired_text);
    assert_eq!(
        bob_before,
        collect_files(&bob_cfg),
        "cfg mutated on post-w0 legacy reject"
    );
    assert!(
        fs::read_dir(&bob_out).unwrap().next().is_none(),
        "post-w0 legacy reject must not persist output files"
    );

    let bob_list = qsc_base(&bob_cfg)
        .args(["timeline", "list", "--peer", "bob", "--limit", "10"])
        .output()
        .expect("bob timeline list after post-w0 reject");
    assert!(bob_list.status.success(), "{}", output_text(&bob_list));
    assert!(
        output_text(&bob_list).contains("event=timeline_list count=0 peer=bob"),
        "{}",
        output_text(&bob_list)
    );

    let alice_after_legacy_reject = run_receive(
        &alice_cfg,
        relay.base_url(),
        ROUTE_TOKEN_BOB,
        &alice_out,
        None,
        false,
    );
    assert!(
        alice_after_legacy_reject.status.success(),
        "{}",
        output_text(&alice_after_legacy_reject)
    );
    let alice_after_legacy_reject_text = output_text(&alice_after_legacy_reject);
    assert!(
        alice_after_legacy_reject_text.contains("event=recv_none"),
        "{}",
        alice_after_legacy_reject_text
    );
    assert!(
        !alice_after_legacy_reject_text.contains("state=peer_confirmed"),
        "{}",
        alice_after_legacy_reject_text
    );

    let attachment_payload = base.join("retired-attachment.bin");
    write_repeated_file(&attachment_payload, 1_048_576, 0x58);
    let attachment_send = run_attachment_send_with_config(
        &alice_cfg,
        relay.base_url(),
        "bob",
        &attachment_payload,
        AttachmentSendConfig {
            validated_attachment_service: Some(service.base_url()),
            with_receipt: true,
            ..AttachmentSendConfig::default()
        },
    );
    assert!(
        attachment_send.status.success(),
        "{}",
        output_text(&attachment_send)
    );
    let attachment_send_text = output_text(&attachment_send);
    assert_file_send_policy(&attachment_send_text, "w2", "legacy_sized");
    assert!(
        attachment_send_text.contains("event=attachment_service_commit"),
        "{}",
        attachment_send_text
    );

    let bob_attachment_recv = run_receive_with_legacy_mode(
        &bob_cfg,
        relay.base_url(),
        ROUTE_TOKEN_BOB,
        &bob_out,
        Some(service.base_url()),
        true,
        Some("retired"),
    );
    assert!(
        bob_attachment_recv.status.success(),
        "{}",
        output_text(&bob_attachment_recv)
    );
    let bob_attachment_text = output_text(&bob_attachment_recv);
    assert!(
        bob_attachment_text.contains("attachment_confirm_send"),
        "{}",
        bob_attachment_text
    );
    assert_eq!(
        file_sha256_hex(&attachment_payload),
        file_sha256_hex(&bob_out.join("retired-attachment.bin"))
    );

    let alice_after_attachment_confirm = run_receive(
        &alice_cfg,
        relay.base_url(),
        ROUTE_TOKEN_BOB,
        &alice_out,
        None,
        false,
    );
    assert!(
        alice_after_attachment_confirm.status.success(),
        "{}",
        output_text(&alice_after_attachment_confirm)
    );
    let alice_after_attachment_confirm_text = output_text(&alice_after_attachment_confirm);
    assert!(
        alice_after_attachment_confirm_text.contains("QSC_FILE_DELIVERY state=peer_confirmed"),
        "{}",
        alice_after_attachment_confirm_text
    );
}

#[test]
fn legacy_sized_w2_roundtrip_confirms_without_false_peer_confirmed() {
    let _guard = attachment_test_guard();
    let relay = common::start_inbox_server(2 * 1024 * 1024, 512);
    let service = common::start_attachment_server(100 * 1024 * 1024);
    let base = safe_test_root().join(format!("na0205a_w2_confirm_{}", std::process::id()));
    create_dir_700(&base);
    let (alice_cfg, bob_cfg, alice_out, bob_out) = setup_pair(&base);

    let payload = base.join("legacy-sized-attachment.bin");
    let payload_bytes = vec![0x7a; 1_048_576];
    fs::write(&payload, &payload_bytes).unwrap();

    let send = run_attachment_send_with_config(
        &alice_cfg,
        relay.base_url(),
        "bob",
        &payload,
        AttachmentSendConfig {
            validated_attachment_service: Some(service.base_url()),
            with_receipt: true,
            ..AttachmentSendConfig::default()
        },
    );
    assert!(send.status.success(), "{}", output_text(&send));
    let send_text = output_text(&send);
    assert_file_send_policy(&send_text, "w2", "legacy_sized");
    assert!(!send_text.contains("state=peer_confirmed"), "{}", send_text);
    assert!(
        send_text.contains("QSC_FILE_DELIVERY state=awaiting_confirmation"),
        "{}",
        send_text
    );

    let bob_recv = run_receive(
        &bob_cfg,
        relay.base_url(),
        ROUTE_TOKEN_BOB,
        &bob_out,
        Some(service.base_url()),
        true,
    );
    assert!(bob_recv.status.success(), "{}", output_text(&bob_recv));
    let bob_text = output_text(&bob_recv);
    assert!(bob_text.contains("attachment_confirm_send"), "{}", bob_text);
    assert_eq!(
        fs::read(bob_out.join("legacy-sized-attachment.bin")).unwrap(),
        payload_bytes
    );

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
fn validated_post_w0_send_rejects_deprecated_w1_alias() {
    let _guard = attachment_test_guard();
    let relay = common::start_inbox_server(2 * 1024 * 1024, 256);
    let service = common::start_attachment_server(100 * 1024 * 1024);
    let base = safe_test_root().join(format!("na0205a_w1_alias_{}", std::process::id()));
    create_dir_700(&base);
    let (alice_cfg, _bob_cfg, _alice_out, _bob_out) = setup_pair(&base);

    let env_payload = base.join("env-alias.bin");
    write_repeated_file(&env_payload, 262_144, 0x55);
    let env_send = run_attachment_send_with_config(
        &alice_cfg,
        relay.base_url(),
        "bob",
        &env_payload,
        AttachmentSendConfig {
            validated_attachment_service: Some(service.base_url()),
            env_stage: Some("w1"),
            ..AttachmentSendConfig::default()
        },
    );
    assert!(!env_send.status.success(), "{}", output_text(&env_send));
    let env_text = output_text(&env_send);
    assert!(
        env_text.contains("legacy_in_message_stage_retired_post_w0"),
        "{}",
        env_text
    );
    assert!(!env_text.contains("event=file_send_policy"), "{}", env_text);
    assert_no_secretish_output(&env_text);

    let arg_payload = base.join("arg-alias.bin");
    write_repeated_file(&arg_payload, 393_216, 0x56);
    let arg_send = run_attachment_send_with_config(
        &alice_cfg,
        relay.base_url(),
        "bob",
        &arg_payload,
        AttachmentSendConfig {
            validated_attachment_service: Some(service.base_url()),
            arg_stage: Some("w1"),
            ..AttachmentSendConfig::default()
        },
    );
    assert!(!arg_send.status.success(), "{}", output_text(&arg_send));
    let arg_text = output_text(&arg_send);
    assert!(
        arg_text.contains("legacy_in_message_stage_retired_post_w0"),
        "{}",
        arg_text
    );
    assert!(!arg_text.contains("event=file_send_policy"), "{}", arg_text);
    assert_no_secretish_output(&arg_text);
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
