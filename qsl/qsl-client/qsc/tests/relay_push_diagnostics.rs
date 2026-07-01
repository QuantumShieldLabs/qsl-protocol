mod common;

use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::process::{Command, Output};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

const ROUTE_TOKEN_FIXTURE: &str = "rtNA0554PeerBobMarkerABCDEFGHIJK";
const BEARER_FIXTURE: &str = "bearerValueNA0554ShouldNotAppear123";
const PAYLOAD_FIXTURE: &str = "payload-fixture-NA0554-should-not-appear";
const RESPONSE_BODY_FIXTURE: &str = "response-body-fixture-NA0554-should-not-appear";
const PRIVATE_ENDPOINT_HOST_FRAGMENT: &str = "127.0.0.1";

struct FixedRelayServer {
    base_url: String,
    shutdown: Arc<AtomicBool>,
    handle: Option<thread::JoinHandle<()>>,
}

impl FixedRelayServer {
    fn base_url(&self) -> &str {
        &self.base_url
    }
}

impl Drop for FixedRelayServer {
    fn drop(&mut self) {
        self.shutdown.store(true, Ordering::SeqCst);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

fn start_fixed_relay(status: u16, body: &'static str) -> FixedRelayServer {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind fixture relay");
    listener
        .set_nonblocking(true)
        .expect("fixture relay nonblocking");
    let addr = listener.local_addr().expect("fixture relay addr");
    let shutdown = Arc::new(AtomicBool::new(false));
    let shutdown_t = Arc::clone(&shutdown);
    let handle = thread::spawn(move || {
        while !shutdown_t.load(Ordering::SeqCst) {
            match listener.accept() {
                Ok((stream, _)) => {
                    thread::spawn(move || handle_fixed_conn(stream, status, body));
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(10));
                }
                Err(_) => break,
            }
        }
    });
    FixedRelayServer {
        base_url: format!("http://{}", addr),
        shutdown,
        handle: Some(handle),
    }
}

fn handle_fixed_conn(mut stream: TcpStream, status: u16, body: &str) {
    let mut buf = Vec::new();
    let mut tmp = [0u8; 1024];
    while !buf.windows(4).any(|w| w == b"\r\n\r\n") {
        match stream.read(&mut tmp) {
            Ok(0) => break,
            Ok(n) => buf.extend_from_slice(&tmp[..n]),
            Err(_) => break,
        }
    }
    let status_text = match status {
        200 => "OK",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        413 => "Payload Too Large",
        429 => "Too Many Requests",
        _ => "Error",
    };
    let response = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        status,
        status_text,
        body.len(),
        body
    );
    let _ = stream.write_all(response.as_bytes());
}

fn output_text(out: &Output) -> String {
    let mut text = String::from_utf8_lossy(&out.stdout).to_string();
    text.push_str(&String::from_utf8_lossy(&out.stderr));
    text
}

fn qsc_base(iso: &common::TestIsolation, cfg: &Path) -> Command {
    let mut cmd = common::qsc_std_command();
    iso.apply_to(&mut cmd);
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .env("QSC_QSP_SEED", "0554")
        .env("QSC_ALLOW_SEED_FALLBACK", "1");
    cmd
}

fn run_success(iso: &common::TestIsolation, cfg: &Path, args: &[&str]) {
    let out = qsc_base(iso, cfg)
        .args(args)
        .output()
        .expect("qsc setup command");
    assert!(out.status.success(), "{}", output_text(&out));
}

fn prepare_relay_send_cfg(
    iso: &common::TestIsolation,
    tag: &str,
) -> (std::path::PathBuf, std::path::PathBuf) {
    let cfg = iso.root.join(format!("{tag}-cfg"));
    let payload = iso.root.join(format!("{tag}-payload.txt"));
    common::init_mock_vault(&cfg);
    run_success(
        iso,
        &cfg,
        &[
            "contacts",
            "add",
            "--label",
            "bob",
            "--fp",
            "fp-test",
            "--route-token",
            ROUTE_TOKEN_FIXTURE,
        ],
    );
    fs::write(&payload, PAYLOAD_FIXTURE).expect("write payload fixture");
    (cfg, payload)
}

fn run_relay_send(
    iso: &common::TestIsolation,
    cfg: &Path,
    relay: &str,
    payload: &Path,
    diagnostic: Option<&str>,
) -> Output {
    let mut cmd = qsc_base(iso, cfg);
    cmd.env("RELAY_TOKEN", BEARER_FIXTURE).args([
        "send",
        "--transport",
        "relay",
        "--relay",
        relay,
        "--to",
        "bob",
        "--file",
        payload.to_str().expect("payload path"),
    ]);
    if let Some(mode) = diagnostic {
        cmd.env("QSC_RELAY_PUSH_DIAGNOSTIC", mode);
    }
    cmd.output().expect("qsc relay send")
}

fn assert_no_fixture_secrets(text: &str) {
    for forbidden in [
        ROUTE_TOKEN_FIXTURE,
        BEARER_FIXTURE,
        PAYLOAD_FIXTURE,
        RESPONSE_BODY_FIXTURE,
        "Authorization",
        "Bearer ",
        PRIVATE_ENDPOINT_HOST_FRAGMENT,
    ] {
        assert!(
            !text.contains(forbidden),
            "diagnostic output leaked forbidden fixture {forbidden}: {text}"
        );
    }
}

#[test]
fn default_mode_emits_no_relay_push_diagnostics() {
    let iso = common::TestIsolation::new("na0554_relay_push_diagnostic_default");
    let server = start_fixed_relay(401, RESPONSE_BODY_FIXTURE);
    let (cfg, payload) = prepare_relay_send_cfg(&iso, "default");

    let out = run_relay_send(&iso, &cfg, server.base_url(), &payload, None);
    let text = output_text(&out);

    assert!(!out.status.success(), "{text}");
    assert!(
        !text.contains("event=relay_push_diagnostic"),
        "default mode emitted relay push diagnostic: {text}"
    );
    assert_no_fixture_secrets(&text);
    println!("NA0554_DIAGNOSTIC_DEFAULT_DISABLED_OK");
}

#[test]
fn redacted_mode_reports_status_body_and_presence_without_values() {
    let iso = common::TestIsolation::new("na0554_relay_push_diagnostic_redacted");
    let server = start_fixed_relay(401, RESPONSE_BODY_FIXTURE);
    let (cfg, payload) = prepare_relay_send_cfg(&iso, "redacted");

    let out = run_relay_send(&iso, &cfg, server.base_url(), &payload, Some("redacted"));
    let text = output_text(&out);

    assert!(!out.status.success(), "{text}");
    assert!(text.contains("event=relay_push_diagnostic"), "{text}");
    assert!(
        text.contains("diagnostic=QSC_RELAY_PUSH_DIAGNOSTIC"),
        "{text}"
    );
    assert!(text.contains("mode=redacted"), "{text}");
    assert!(text.contains("api=relay_push_v1"), "{text}");
    assert!(text.contains("status_class=4xx"), "{text}");
    assert!(text.contains("status_code=401"), "{text}");
    assert!(text.contains("error_class=auth_rejected"), "{text}");
    assert!(
        text.contains("diagnostic_class=bearer_auth_failed"),
        "{text}"
    );
    assert!(text.contains("timeout_phase_class=not_timeout"), "{text}");
    assert!(text.contains("response_body_present=true"), "{text}");
    assert!(
        text.contains(format!("response_body_len={}", RESPONSE_BODY_FIXTURE.len()).as_str()),
        "{text}"
    );
    assert!(text.contains("route_header_present=true"), "{text}");
    assert!(text.contains("auth_present=true"), "{text}");
    assert!(text.contains("qsc_error=relay_unauthorized"), "{text}");
    assert_no_fixture_secrets(&text);
    println!("NA0554_REDACTED_DIAGNOSTIC_GATE_OK");
    println!("NA0554_SECRET_MATERIAL_REDACTION_TESTS_OK");
}

#[test]
fn redacted_mode_maps_payload_rejected_without_response_content() {
    let iso = common::TestIsolation::new("na0554_relay_push_diagnostic_payload_rejected");
    let server = start_fixed_relay(413, RESPONSE_BODY_FIXTURE);
    let (cfg, payload) = prepare_relay_send_cfg(&iso, "payload-rejected");

    let out = run_relay_send(&iso, &cfg, server.base_url(), &payload, Some("redacted"));
    let text = output_text(&out);

    assert!(!out.status.success(), "{text}");
    assert!(text.contains("status_class=4xx"), "{text}");
    assert!(text.contains("status_code=413"), "{text}");
    assert!(text.contains("error_class=payload_rejected"), "{text}");
    assert!(
        text.contains("diagnostic_class=http_status_received"),
        "{text}"
    );
    assert!(text.contains("timeout_phase_class=not_timeout"), "{text}");
    assert!(text.contains("qsc_error=relay_inbox_too_large"), "{text}");
    assert_no_fixture_secrets(&text);
    println!("NA0554_RELAY_PUSH_DIAGNOSTIC_STATUS_CLASS_OK");
}
