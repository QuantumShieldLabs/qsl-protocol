// NA-0744 / D-1382 (ENG-0193) — THE PULL/ACK TRANSPORT-BOUNDARY DIAGNOSTIC.
//
// The push half of this boundary has explained itself since NA-0554
// (`relay_push_diagnostics.rs`, whose shape this file deliberately mirrors). The
// pull half was mute: `relay_inbox_pull_mode`'s 13 exits and `relay_inbox_ack`'s
// 11 published nothing at all, so every failure reached the operator as a bare
// `relay_inbox_pull_failed` — the name of the OPERATION, never the REASON.
//
// ⚠ THESE TESTS DRIVE THE SHIPPED BINARY AND NOTHING ELSE. That is what let them
// be written and run RED before the emission existed (E1 / R332.1): they compile
// against the pre-lane tree, because they reference no lane symbol.
//
// The fixture relay answers a FIXED status, which is the whole instrument: the
// pull's vocabulary is keyed on the status the server actually sent, and the
// point of the lane is that four of those statuses mean something different here
// than they do on the push.

mod common;

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

const MAILBOX_FIXTURE: &str = "rtNA0744PullDiagnosticMailboxAAA";
const PEER_ROUTE_FIXTURE: &str = "rtNA0744PullDiagnosticPeerBBBBB";
const BEARER_FIXTURE: &str = "bearerValueNA0744ShouldNotAppear123";
const RESPONSE_BODY_FIXTURE: &str = "response-body-fixture-NA0744-should-not-appear";
const PRIVATE_ENDPOINT_HOST_FRAGMENT: &str = "127.0.0.1";

// A port that refuses instantly. `127.0.0.1:1` is the dead endpoint §3.4 names:
// no relay, no secret, no network, so E4 runs on every PR and cannot rot.
const DEAD_ENDPOINT: &str = "http://127.0.0.1:1";

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
        204 => "No Content",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        413 => "Payload Too Large",
        429 => "Too Many Requests",
        _ => "Error",
    };
    // ⚠ A 204 carries NO body by the HTTP contract, and the client's 204 arm maps
    // to `Ok(Vec::new())`. Sending one would make the fixture a different server
    // from the one the lane measured.
    let response = if status == 204 {
        format!("HTTP/1.1 {} {}\r\nConnection: close\r\n\r\n", status, status_text)
    } else {
        format!(
            "HTTP/1.1 {} {}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            status,
            status_text,
            body.len(),
            body
        )
    };
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
        .env("QSC_QSP_SEED", "0744")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1");
    cmd
}

fn run_success(iso: &common::TestIsolation, cfg: &Path, args: &[&str]) {
    let out = qsc_base(iso, cfg)
        .args(args)
        .output()
        .expect("qsc setup command");
    assert!(out.status.success(), "{}", output_text(&out));
}

/// A vault, a self identity, a peer contact with a route token, and the account
/// inbox token — the smallest state from which both `receive` and
/// `handshake poll` reach the relay.
fn prepare_cfg(iso: &common::TestIsolation, tag: &str) -> std::path::PathBuf {
    let cfg = iso.root.join(format!("na0744-{tag}-cfg"));
    common::init_mock_vault(&cfg);
    run_success(iso, &cfg, &["identity", "rotate", "--confirm"]);
    run_success(
        iso,
        &cfg,
        &[
            "contacts",
            "add",
            "--label",
            "bob",
            "--fp",
            "fp-na0744-test",
            "--route-token",
            PEER_ROUTE_FIXTURE,
        ],
    );
    cfg
}

/// Every `QSC_MARK/1` line carrying `event=<event>`.
fn marker_lines<'a>(text: &'a str, event: &str) -> Vec<&'a str> {
    let needle = format!(" event={} ", event);
    let tail = format!(" event={}", event);
    text.lines()
        .filter(|line| line.starts_with("QSC_MARK/1 "))
        .filter(|line| line.contains(needle.as_str()) || line.ends_with(tail.as_str()))
        .collect()
}

/// ⚠ ANCHORED AT BOTH ENDS. Unanchored, a `class=` needle also harvests
/// `status_class=`, `error_class=`, `diagnostic_class=` and
/// `timeout_phase_class=` — the same trap the shell helper documents (m8).
fn marker_field(line: &str, key: &str) -> Option<String> {
    let key_eq = format!("{}=", key);
    line.split(' ')
        .find(|tok| tok.starts_with(key_eq.as_str()))
        .map(|tok| tok[key_eq.len()..].to_string())
}

/// The diagnostic for ONE half of the boundary. ⚠ A single command can emit BOTH:
/// a lease-mode receive that commits an item goes on to ACK it, and the ack is its own
/// line with its own `op=`. Selecting by event alone would hand back whichever came
/// first — the same blending hazard the shell helper documents (D-1324).
fn pull_diagnostics_for_op<'a>(text: &'a str, op: &str) -> Vec<&'a str> {
    let sel = format!(" op={} ", op);
    marker_lines(text, "relay_pull_diagnostic")
        .into_iter()
        .filter(|line| line.contains(sel.as_str()))
        .collect()
}

/// ⚠ ASSERTS EXACTLY ONE, and the count is measured rather than assumed: each flow
/// below issues a single pull (a 401/400/dead-endpoint/CA-file failure ends the loop,
/// and a 204 ends it as `recv_none`). A future change that makes one of them pull twice
/// should say so here by name, not be silently read as its first line — the 200 arm,
/// which really does pull four times, uses `pull_diagnostics_for_op` instead.
fn sole_pull_diagnostic(text: &str) -> String {
    let lines = marker_lines(text, "relay_pull_diagnostic");
    assert!(
        !lines.is_empty(),
        "no relay_pull_diagnostic emitted (ENG-0193 is not repaired): {text}"
    );
    assert_eq!(
        lines.len(),
        1,
        "expected exactly one pull diagnostic for this flow, got {}: {text}",
        lines.len()
    );
    lines[0].to_string()
}

fn assert_no_fixture_secrets(text: &str) {
    for forbidden in [
        MAILBOX_FIXTURE,
        PEER_ROUTE_FIXTURE,
        BEARER_FIXTURE,
        RESPONSE_BODY_FIXTURE,
        "Authorization",
        "Bearer ",
        PRIVATE_ENDPOINT_HOST_FRAGMENT,
    ] {
        assert!(
            !text.contains(forbidden),
            "pull diagnostic leaked forbidden fixture {forbidden}: {text}"
        );
    }
}

fn run_receive(
    iso: &common::TestIsolation,
    cfg: &Path,
    relay: &str,
    mailbox: &str,
    diagnostic: Option<&str>,
    ack_mode: Option<&str>,
) -> Output {
    let out_dir = cfg.join("recv-out");
    let mut cmd = qsc_base(iso, cfg);
    cmd.env("RELAY_TOKEN", BEARER_FIXTURE).args([
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
        "4",
        "--out",
        out_dir.to_str().expect("out dir"),
    ]);
    if let Some(mode) = ack_mode {
        cmd.args(["--ack-mode", mode]);
    }
    if let Some(mode) = diagnostic {
        cmd.env("QSC_RELAY_PULL_DIAGNOSTIC", mode);
    }
    cmd.output().expect("qsc receive")
}

/// ⚠ THE NON-RECEIVE CALLER. `handshake poll` is one of the four pull callers
/// that publish NO `recv_start`, which is exactly why E5 is sealed on it: before
/// this lane it said nothing whatever about which mailbox it looked in.
fn run_handshake_poll(
    iso: &common::TestIsolation,
    cfg: &Path,
    relay: &str,
    diagnostic: Option<&str>,
) -> Output {
    let mut cmd = qsc_base(iso, cfg);
    cmd.env("RELAY_TOKEN", BEARER_FIXTURE)
        .args(["handshake", "poll", "--peer", "bob", "--relay", relay]);
    if let Some(mode) = diagnostic {
        cmd.env("QSC_RELAY_PULL_DIAGNOSTIC", mode);
    }
    cmd.output().expect("qsc handshake poll")
}

/// ⚠ THE REACHABLE PRE-FLIGHT ARM, established by MEASUREMENT rather than by
/// assumption. The obvious candidate -- a malformed `--mailbox` -- never reaches
/// this wrapper at all: the receive path normalizes the route token at
/// `transport/mod.rs:258`, well before `recv_start` and before any pull. A
/// configured CA file that cannot be read DOES reach it, because
/// `relay_http_client()` is called inside the inner, after both normalizes.
fn run_handshake_poll_with_bad_ca(
    iso: &common::TestIsolation,
    cfg: &Path,
    relay: &str,
    ca_file: &Path,
) -> Output {
    let mut cmd = qsc_base(iso, cfg);
    cmd.env("RELAY_TOKEN", BEARER_FIXTURE)
        .env("QSC_RELAY_CA_FILE", ca_file)
        .env("QSC_RELAY_PULL_DIAGNOSTIC", "redacted")
        .args(["handshake", "poll", "--peer", "bob", "--relay", relay]);
    cmd.output().expect("qsc handshake poll (bad ca)")
}

// ---------------------------------------------------------------------------
// THE GATE
// ---------------------------------------------------------------------------

/// ⚠ CLASSIFIED FOR E1: this asserts an ABSENCE, so it passes on the pre-lane
/// tree BY CONSTRUCTION and is NOT part of the red-first set. It is still worth
/// having — it is the only thing that fails if a successor emits unconditionally.
#[test]
fn pull_default_mode_emits_no_relay_pull_diagnostic() {
    let iso = common::TestIsolation::new("na0744_pull_diagnostic_default");
    let server = start_fixed_relay(401, RESPONSE_BODY_FIXTURE);
    let cfg = prepare_cfg(&iso, "default");

    let out = run_receive(&iso, &cfg, server.base_url(), MAILBOX_FIXTURE, None, None);
    let text = output_text(&out);

    assert!(!out.status.success(), "{text}");
    assert!(
        marker_lines(&text, "relay_pull_diagnostic").is_empty(),
        "default mode emitted a pull diagnostic: {text}"
    );
    assert_no_fixture_secrets(&text);
    println!("NA0744_PULL_DIAGNOSTIC_DEFAULT_DISABLED_OK");
}

// ---------------------------------------------------------------------------
// THE STATUS ARMS — the pull's OWN vocabulary
// ---------------------------------------------------------------------------

/// Driven with an EXPLICIT `--ack-mode legacy` so the `ack_mode` field is pinned
/// to a value the caller chose. Its companion below pins the other value.
#[test]
fn pull_redacted_mode_publishes_unauthorized_with_full_field_set() {
    let iso = common::TestIsolation::new("na0744_pull_diagnostic_unauthorized");
    let server = start_fixed_relay(401, RESPONSE_BODY_FIXTURE);
    let cfg = prepare_cfg(&iso, "unauthorized");

    let out = run_receive(
        &iso,
        &cfg,
        server.base_url(),
        MAILBOX_FIXTURE,
        Some("redacted"),
        Some("legacy"),
    );
    let text = output_text(&out);
    assert!(!out.status.success(), "{text}");
    let line = sole_pull_diagnostic(&text);

    for (key, want) in [
        ("diagnostic", "QSC_RELAY_PULL_DIAGNOSTIC"),
        ("mode", "redacted"),
        ("api", "relay_pull_v1"),
        ("op", "pull"),
        ("ack_mode", "legacy"),
        ("max", "4"),
        ("status_class", "4xx"),
        ("status_code", "401"),
        ("error_class", "auth_rejected"),
        ("diagnostic_class", "bearer_auth_failed"),
        ("timeout_phase_class", "not_timeout"),
        ("qsc_error", "relay_unauthorized"),
        ("attempt", "1"),
    ] {
        assert_eq!(
            marker_field(&line, key).as_deref(),
            Some(want),
            "field {key} on the pull diagnostic: {line}"
        );
    }

    // ⚠ THE KEY IS `mailbox_hash`, NOT `route_token_hash8`: any key containing
    // `token` is blanked by `should_redact_value`. Proving the value is 8
    // lowercase hex proves it was neither redacted nor published raw. It must
    // also EQUAL what `recv_start` published for the same mailbox in the same
    // run — the two lines have to join up, or the field is useless for tracing.
    let hash = marker_field(&line, "mailbox_hash").expect("mailbox_hash present");
    assert_eq!(hash.len(), 8, "mailbox_hash is not 8 chars: {line}");
    assert!(
        hash.chars().all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()),
        "mailbox_hash is not lowercase hex: {line}"
    );
    let recv_start = marker_lines(&text, "recv_start");
    assert_eq!(recv_start.len(), 1, "expected one recv_start: {text}");
    assert_eq!(
        marker_field(recv_start[0], "mailbox_hash").as_deref(),
        Some(hash.as_str()),
        "the pull diagnostic's mailbox_hash disagrees with recv_start's for the same mailbox"
    );

    // `items_count` NEVER appears off the 200 arm; `acked_count` never on a pull.
    assert!(marker_field(&line, "items_count").is_none(), "{line}");
    assert!(marker_field(&line, "acked_count").is_none(), "{line}");

    assert_no_fixture_secrets(&text);
    println!("NA0744_PULL_DIAGNOSTIC_REDACTED_GATE_OK");
}

/// ⚠ THE TEST THAT PROVES THE VOCABULARY SPLIT WAS NECESSARY. Reusing the push's
/// status map here would publish `route_token_auth_failed` for a 400 — sending an
/// operator to re-check a route token that was never the problem. The pull's own
/// arm is `relay_inbox_bad_request`: a malformed REQUEST.
#[test]
fn pull_bad_request_uses_pull_vocabulary_and_not_the_push_map() {
    let iso = common::TestIsolation::new("na0744_pull_diagnostic_bad_request");
    let server = start_fixed_relay(400, RESPONSE_BODY_FIXTURE);
    let cfg = prepare_cfg(&iso, "bad-request");

    let out = run_receive(
        &iso,
        &cfg,
        server.base_url(),
        MAILBOX_FIXTURE,
        Some("redacted"),
        None,
    );
    let text = output_text(&out);
    let line = sole_pull_diagnostic(&text);

    assert_eq!(marker_field(&line, "status_code").as_deref(), Some("400"));
    assert_eq!(
        marker_field(&line, "error_class").as_deref(),
        Some("request_rejected")
    );
    assert_eq!(
        marker_field(&line, "diagnostic_class").as_deref(),
        Some("bad_request_received")
    );
    assert_eq!(
        marker_field(&line, "qsc_error").as_deref(),
        Some("relay_inbox_bad_request")
    );

    // The push's values for this status, asserted ABSENT rather than merely
    // not-asserted: this is the false statement the split exists to prevent.
    assert!(
        !line.contains("route_token_auth_failed"),
        "the pull published the PUSH's 400 diagnosis: {line}"
    );
    assert!(
        !line.contains("error_class=route_rejected"),
        "the pull published the PUSH's 400 error class: {line}"
    );

    assert_no_fixture_secrets(&text);
    println!("NA0744_PULL_DIAGNOSTIC_VOCABULARY_IS_THE_PULLS_OWN_OK");
}

// ---------------------------------------------------------------------------
// E4 — THE DEAD-ENDPOINT ARM (§3.4)
// ---------------------------------------------------------------------------

/// ⚠ NAMED FAILURE: a line carrying only `qsc_error=relay_inbox_pull_failed` is a
/// MISS — that is the status quo wearing a new name. Measured on the pre-lane
/// tree, this exact flow produced `event=error code=relay_inbox_pull_failed` and
/// nothing else, which is what a live main-red event produced on 2026-08-18.
#[test]
fn pull_dead_endpoint_names_connection_refused() {
    let iso = common::TestIsolation::new("na0744_pull_diagnostic_dead_endpoint");
    let cfg = prepare_cfg(&iso, "dead-endpoint");

    let out = run_receive(
        &iso,
        &cfg,
        DEAD_ENDPOINT,
        MAILBOX_FIXTURE,
        Some("redacted"),
        None,
    );
    let text = output_text(&out);
    assert!(!out.status.success(), "{text}");
    let line = sole_pull_diagnostic(&text);

    for (key, want) in [
        ("error_class", "network_error"),
        ("diagnostic_class", "connection_refused"),
        ("status_class", "unknown"),
        ("status_code", "unknown"),
        ("timeout_phase_class", "not_timeout"),
        // The bare code is still published — it is simply no longer ALL there is.
        ("qsc_error", "relay_inbox_pull_failed"),
    ] {
        assert_eq!(
            marker_field(&line, key).as_deref(),
            Some(want),
            "field {key} on the dead-endpoint diagnostic: {line}"
        );
    }
    println!("NA0744_PULL_DIAGNOSTIC_DEAD_ENDPOINT_DIAGNOSED_OK");
}

// ---------------------------------------------------------------------------
// E5 — THE STRONG FORM: `mailbox_hash` + `status_code=204` on a NON-RECEIVE caller
// ---------------------------------------------------------------------------

/// ⚠⚠ THIS IS THE LANE'S CENTRAL CASE, AND THE PRE-LANE MEASUREMENT IS WHAT MAKES
/// IT SHARP. Against a 204 relay, `handshake poll` exits **rc 0** and emits
/// exactly one marker: `handshake_recv msg=none ok=true`. It says nothing about
/// WHICH mailbox it polled — so a poll of the WRONG mailbox is indistinguishable
/// from a poll of an empty one, at rc 0, under an `ok=true` marker. That is the
/// failure ENG-0192 had to be hunted down by hand, and it is what this line ends.
///
/// ⚠ ANTECEDENT ASSERTED FIRST: if the 204 arm did not fire, nothing is concluded.
/// ⚠ `items_count=0` IS UNREACHABLE and must never be sealed on: the server
/// answers 204 to an empty pull at both measured revs, so a 200 body is non-empty
/// by construction. The 204 arm says `empty_mailbox` instead of counting nothing.
#[test]
fn non_receive_caller_publishes_mailbox_hash_and_204() {
    let iso = common::TestIsolation::new("na0744_pull_diagnostic_e5_204");
    let server = start_fixed_relay(204, "");
    let cfg = prepare_cfg(&iso, "e5-204");

    let out = run_handshake_poll(&iso, &cfg, server.base_url(), Some("redacted"));
    let text = output_text(&out);
    let line = sole_pull_diagnostic(&text);

    // ANTECEDENT.
    assert_eq!(
        marker_field(&line, "status_code").as_deref(),
        Some("204"),
        "the 204 arm did not fire; nothing is concluded: {line}"
    );

    for (key, want) in [
        ("op", "pull"),
        ("api", "relay_pull_v1"),
        // The flag-less default, MEASURED: `resolve_ack_mode(None)` is
        // `stored_ack_mode().unwrap_or(AckMode::Lease)` (lib.rs:942). Its
        // companion above pins `legacy` through an explicit flag, so between them
        // the field is proven to track the mode rather than to be a constant.
        ("ack_mode", "lease"),
        ("max", "4"),
        ("status_class", "2xx"),
        ("diagnostic_class", "empty_mailbox"),
        ("timeout_phase_class", "not_timeout"),
        ("qsc_error", "none"),
    ] {
        assert_eq!(
            marker_field(&line, key).as_deref(),
            Some(want),
            "field {key} on the 204 diagnostic: {line}"
        );
    }
    assert!(
        marker_field(&line, "items_count").is_none(),
        "items_count published on the 204 arm, where the server sent no count: {line}"
    );

    let hash = marker_field(&line, "mailbox_hash").expect("mailbox_hash present");
    assert_eq!(hash.len(), 8, "{line}");
    assert!(
        hash.chars().all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()),
        "{line}"
    );

    // ⚠ THE POINT OF E5, ASSERTED MECHANICALLY: this caller emits NO `recv_start`,
    // so before this lane it published nothing whatever about where it looked.
    assert!(
        marker_lines(&text, "recv_start").is_empty(),
        "handshake poll emitted recv_start; it is no longer a non-receive caller \
         and E5's premise must be re-derived: {text}"
    );
    // And its own behaviour is UNCHANGED: same marker, same success.
    assert!(
        text.contains("event=handshake_recv msg=none ok=true"),
        "the poll's own outcome changed: {text}"
    );
    assert!(out.status.success(), "{text}");

    assert_no_fixture_secrets(&text);
    println!("NA0744_PULL_DIAGNOSTIC_E5_MAILBOX_HASH_204_OK");
}

// ---------------------------------------------------------------------------
// PRE-FLIGHT — the outcomes v1 excluded and the wrapper includes
// ---------------------------------------------------------------------------

/// ⚠ WHICH pre-flight arm is reachable was MEASURED, not assumed. A malformed
/// `--mailbox` never reaches this wrapper: the receive path normalizes the route
/// token at `transport/mod.rs:258`, before `recv_start` and before any pull. A
/// configured CA file that cannot be read DOES reach it — `relay_http_client()`
/// runs inside the inner, after both normalizes — and on the pre-lane tree it
/// produced the bare `code=relay_ca_file_invalid` and nothing more.
#[test]
fn pull_ca_file_preflight_says_so_before_any_socket() {
    let iso = common::TestIsolation::new("na0744_pull_diagnostic_preflight_ca");
    let cfg = prepare_cfg(&iso, "preflight-ca");
    let bad_ca = cfg.join("not-a-ca.pem");
    std::fs::write(&bad_ca, "this is not a certificate\n").expect("write bad ca");

    // The endpoint is never contacted; the client cannot even be built.
    let out = run_handshake_poll_with_bad_ca(&iso, &cfg, "https://127.0.0.1:1", &bad_ca);
    let text = output_text(&out);
    assert!(!out.status.success(), "{text}");
    let line = sole_pull_diagnostic(&text);

    for (key, want) in [
        ("status_class", "unknown"),
        ("status_code", "unknown"),
        ("error_class", "preflight_rejected"),
        ("diagnostic_class", "ca_file_unreadable"),
        ("timeout_phase_class", "not_timeout"),
        ("qsc_error", "relay_ca_file_invalid"),
    ] {
        assert_eq!(
            marker_field(&line, key).as_deref(),
            Some(want),
            "field {key} on the pre-flight diagnostic: {line}"
        );
    }
    println!("NA0744_PULL_DIAGNOSTIC_PREFLIGHT_NAMED_OK");
}

// ---------------------------------------------------------------------------
// E6 — REDACTION on a REAL emitted line (the synthetic half lives beside the
// NA-0554 original in `secret_material_diagnostic_boundary.rs`)
// ---------------------------------------------------------------------------

#[test]
fn pull_diagnostic_publishes_no_key_containing_token() {
    let iso = common::TestIsolation::new("na0744_pull_diagnostic_key_scan");
    let server = start_fixed_relay(401, RESPONSE_BODY_FIXTURE);
    let cfg = prepare_cfg(&iso, "key-scan");

    let out = run_receive(
        &iso,
        &cfg,
        server.base_url(),
        MAILBOX_FIXTURE,
        Some("redacted"),
        None,
    );
    let text = output_text(&out);
    let line = sole_pull_diagnostic(&text);

    let mut checked = 0usize;
    for token in line.split(' ').skip(1) {
        let Some((key, value)) = token.split_once('=') else {
            continue;
        };
        // ⚠ `should_redact_value` blanks ANY key containing `token`. A field so
        // named would render `<redacted>` and the diagnostic would silently lose
        // it — no error, no red, just a missing field.
        assert!(
            !key.to_ascii_lowercase().contains("token"),
            "field key {key} contains `token` and will be redacted away: {line}"
        );
        assert_ne!(value, "<redacted>", "field {key} was redacted: {line}");
        checked += 1;
    }
    assert!(checked >= 12, "scanned only {checked} fields: {line}");

    assert_no_fixture_secrets(&text);
    println!("NA0744_PULL_DIAGNOSTIC_VALUE_FREE_OK");
}

// ---------------------------------------------------------------------------
// E3 — ON/OFF STREAM IDENTITY (REQUIRED)
// ---------------------------------------------------------------------------

/// ⚠ THE ANTECEDENT IS THE WHOLE TEST. If the ON run emits zero pull
/// diagnostics the streams are trivially identical and E3 is VACUOUS — which is
/// exactly its state on the pre-lane tree. So the non-zero count is asserted
/// FIRST, and that assertion is what makes this fail red before the emission
/// exists.
///
/// ⚠ AND THE COMPARISON ITSELF IS CONTROLLED: two OFF runs must agree before an
/// ON-vs-OFF comparison means anything. A flaky stream would otherwise let a real
/// behaviour change hide inside noise, or manufacture one that is not there.
#[test]
fn gate_on_and_off_streams_differ_only_by_pull_diagnostic_lines() {
    let iso = common::TestIsolation::new("na0744_pull_diagnostic_e3");
    let server = start_fixed_relay(204, "");
    let cfg = prepare_cfg(&iso, "e3");

    // Same config, same command, three runs: OFF, OFF, ON.
    let off_a = output_text(&run_handshake_poll(&iso, &cfg, server.base_url(), None));
    let off_b = output_text(&run_handshake_poll(&iso, &cfg, server.base_url(), None));
    let on = output_text(&run_handshake_poll(
        &iso,
        &cfg,
        server.base_url(),
        Some("redacted"),
    ));

    // CONTROL: the stream is deterministic across runs with the gate off.
    assert_eq!(
        off_a, off_b,
        "the OFF stream is not reproducible, so no ON/OFF comparison is meaningful"
    );

    // ANTECEDENT.
    let emitted = marker_lines(&on, "relay_pull_diagnostic").len();
    assert!(
        emitted > 0,
        "the ON run emitted no pull diagnostic, so E3 is VACUOUS: {on}"
    );
    assert!(
        marker_lines(&off_a, "relay_pull_diagnostic").is_empty(),
        "the OFF run emitted a pull diagnostic: {off_a}"
    );

    let stripped: Vec<&str> = on
        .lines()
        .filter(|line| !line.contains("event=relay_pull_diagnostic"))
        .collect();
    let baseline: Vec<&str> = off_b.lines().collect();
    assert_eq!(
        stripped, baseline,
        "the gate changed the stream beyond its own lines\nON(stripped):\n{stripped:#?}\nOFF:\n{baseline:#?}"
    );

    println!("NA0744_PULL_DIAGNOSTIC_E3_STREAM_IDENTITY_OK");
}

// ---------------------------------------------------------------------------
// THE 200 ARM — the only place `items_count` is published
// ---------------------------------------------------------------------------

/// ⚠ `items_count` IS `>= 1` BY CONSTRUCTION and this test must never seal on 0.
/// The server answers 204 to an empty pull at both measured revs, so a 200 body
/// always carries at least one item; the 204 arm says `empty_mailbox` instead of
/// counting nothing. That asymmetry is the whole reason the field is conditional.
#[test]
fn pull_ok_arm_publishes_items_count_of_at_least_one() {
    let iso = common::TestIsolation::new("na0744_pull_diagnostic_ok_items");
    // serde encodes `Vec<u8>` as a JSON array of numbers; the payload is garbage on
    // purpose — this arm is about the TRANSPORT line, not about unpacking.
    let server = start_fixed_relay(200, r#"{"items":[{"id":"na0744-item-1","data":[1,2,3]}]}"#);
    let cfg = prepare_cfg(&iso, "ok-items");

    let out = run_receive(
        &iso,
        &cfg,
        server.base_url(),
        MAILBOX_FIXTURE,
        Some("redacted"),
        None,
    );
    let text = output_text(&out);
    let pulls = pull_diagnostics_for_op(&text, "pull");
    assert!(
        !pulls.is_empty(),
        "no op=pull diagnostic emitted (ENG-0193 is not repaired): {text}"
    );
    let line = pulls[0];

    // ANTECEDENT: the 200 arm, not some other status.
    assert_eq!(
        marker_field(line, "status_code").as_deref(),
        Some("200"),
        "the 200 arm did not fire; nothing is concluded: {line}"
    );
    for (key, want) in [
        ("status_class", "2xx"),
        ("error_class", "unknown"),
        ("diagnostic_class", "http_status_received"),
        ("qsc_error", "none"),
        ("items_count", "1"),
    ] {
        assert_eq!(
            marker_field(line, key).as_deref(),
            Some(want),
            "field {key} on the 200 diagnostic: {line}"
        );
    }
    assert_ne!(
        marker_field(line, "items_count").as_deref(),
        Some("0"),
        "items_count=0 is UNREACHABLE at both measured server revs: {line}"
    );

    assert_no_fixture_secrets(&text);
    println!("NA0744_PULL_DIAGNOSTIC_OK_ARM_ITEMS_COUNT_OK");
}
