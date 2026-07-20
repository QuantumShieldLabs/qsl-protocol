// NA-0663 (D599, D-1286): client TLS trust — the OS trust store is honored, an explicit
// CA file is an ADDITIVE sanctioned escape, and a certificate-verification failure is a
// DISTINGUISHABLE typed outcome instead of the opaque relay_inbox_push_failed.
//
// THE HARD BOUNDARY (operator's words, absolute): no certificate-verification bypass of
// any kind exists in ANY form, for any reason, INCLUDING in this file — no option to skip
// verification, to trust every certificate, or to tolerate one that fails to verify. An
// explicit CA path is the sanctioned escape; a blanket bypass is not. Family 4 pins that
// as a test: it scans the product source AND this file, fail-closed, with no exemptions,
// which is why the needles below are assembled at runtime and never spelled in full.
//
// Rig (F3): certificates are generated at RUNTIME with rcgen and served over a loopback
// TLS listener with tokio-rustls. No key material is committed. No test reaches an
// external relay, host, or network endpoint; every endpoint is 127.0.0.1 except the
// RFC-2606 `.invalid` name used for the DNS-failure distinctness case, which is
// guaranteed never to resolve.

mod common;

use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use rcgen::{BasicConstraints, CertificateParams, DnType, IsCa, KeyPair};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_rustls::rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
use tokio_rustls::rustls::ServerConfig;
use tokio_rustls::TlsAcceptor;

const ROUTE_TOKEN_PEER: &str = "route_token_peer_abcdefghijklmnopq";

// ---------------------------------------------------------------------------
// runtime certificate material (F3): nothing on disk in the repo, ever
// ---------------------------------------------------------------------------

struct TestCa {
    cert: rcgen::Certificate,
    key: KeyPair,
    pem: String,
}

fn make_ca(common_name: &str) -> TestCa {
    let key = KeyPair::generate().expect("generate CA key");
    let mut params = CertificateParams::new(Vec::<String>::new()).expect("CA params");
    params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
    params
        .distinguished_name
        .push(DnType::CommonName, common_name);
    let cert = params.self_signed(&key).expect("self-sign CA");
    let pem = cert.pem();
    TestCa { cert, key, pem }
}

/// A leaf with a 127.0.0.1 IP SAN, signed by `ca`.
fn make_leaf(ca: &TestCa) -> (Vec<CertificateDer<'static>>, PrivateKeyDer<'static>) {
    let key = KeyPair::generate().expect("generate leaf key");
    let params = CertificateParams::new(vec!["127.0.0.1".to_string()]).expect("leaf params");
    let cert = params
        .signed_by(&key, &ca.cert, &ca.key)
        .expect("sign leaf with CA");
    let chain = vec![cert.der().clone()];
    let key_der = PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(key.serialize_der()));
    (chain, key_der)
}

// ---------------------------------------------------------------------------
// the loopback TLS listener
// ---------------------------------------------------------------------------

struct TlsServer {
    base_url: String,
    shutdown: Arc<AtomicBool>,
}

impl Drop for TlsServer {
    fn drop(&mut self) {
        self.shutdown.store(true, Ordering::SeqCst);
    }
}

/// Serve TLS on 127.0.0.1 with `chain`/`key`, answering every request with
/// `status_line`. Enough HTTP for the relay push contract: qsc only needs the
/// status code back from /v1/push.
fn start_tls_server(
    chain: Vec<CertificateDer<'static>>,
    key: PrivateKeyDer<'static>,
    status_line: &'static str,
) -> TlsServer {
    let (tx, rx) = mpsc::channel::<u16>();
    let shutdown = Arc::new(AtomicBool::new(false));
    let stop = shutdown.clone();

    thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("tls test runtime");
        rt.block_on(async move {
            // An explicit provider keeps this independent of process-global state.
            let provider = tokio_rustls::rustls::crypto::ring::default_provider();
            let config = ServerConfig::builder_with_provider(Arc::new(provider))
                .with_safe_default_protocol_versions()
                .expect("protocol versions")
                .with_no_client_auth()
                .with_single_cert(chain, key)
                .expect("server config");
            let acceptor = TlsAcceptor::from(Arc::new(config));
            let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
                .await
                .expect("bind loopback");
            let port = listener.local_addr().expect("local addr").port();
            tx.send(port).expect("publish port");

            while !stop.load(Ordering::SeqCst) {
                let accepted =
                    tokio::time::timeout(Duration::from_millis(200), listener.accept()).await;
                let stream = match accepted {
                    Ok(Ok((stream, _))) => stream,
                    _ => continue,
                };
                let acceptor = acceptor.clone();
                tokio::spawn(async move {
                    // A failed handshake is the EXPECTED path for the untrusted
                    // cases: the client refuses, so there is nothing to serve.
                    let mut tls = match acceptor.accept(stream).await {
                        Ok(v) => v,
                        Err(_) => return,
                    };
                    let mut buf = [0u8; 4096];
                    // Drain what the client sent before replying, so the response is
                    // never written into a half-sent request.
                    let _ = tokio::time::timeout(Duration::from_millis(500), tls.read(&mut buf))
                        .await;
                    let body = b"{}";
                    let response = format!(
                        "HTTP/1.1 {status_line}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                        body.len()
                    );
                    let _ = tls.write_all(response.as_bytes()).await;
                    let _ = tls.write_all(body).await;
                    let _ = tls.flush().await;
                    let _ = tls.shutdown().await;
                });
            }
        });
    });

    let port = rx.recv().expect("tls server port");
    TlsServer {
        base_url: format!("https://127.0.0.1:{port}"),
        shutdown,
    }
}

// ---------------------------------------------------------------------------
// qsc driving helpers (per-Command env: the native-certs load seam)
// ---------------------------------------------------------------------------

fn ensure_dir_700(path: &Path) {
    let _ = fs::create_dir_all(path);
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

fn test_root(tag: &str) -> PathBuf {
    let root = if let Ok(v) = std::env::var("QSC_TEST_ROOT") {
        PathBuf::from(v)
    } else if let Ok(v) = std::env::var("CARGO_TARGET_DIR") {
        PathBuf::from(v)
    } else {
        PathBuf::from("target")
    };
    let root = root
        .join("qsc-test-tmp")
        .join(format!("na0663_{tag}_{}", std::process::id()));
    create_dir_700(&root);
    root
}

/// A config dir with a mock vault and one contact, ready to `relay send`.
fn prepared_cfg(tag: &str) -> (PathBuf, PathBuf) {
    let base = test_root(tag);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    let payload = base.join("msg.bin");
    fs::write(&payload, b"na0663 tls trust payload").expect("write payload");

    common::init_mock_vault(&cfg);
    let added = common::qsc_std_command()
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
        .args([
            "contacts",
            "add",
            "--label",
            "peer",
            "--fp",
            "fp-test",
            "--route-token",
            ROUTE_TOKEN_PEER,
        ])
        .output()
        .expect("run contacts add");
    assert!(
        added.status.success(),
        "contacts add failed: {}",
        String::from_utf8_lossy(&added.stdout)
    );
    (cfg, payload)
}

fn base_send_command(cfg: &Path) -> std::process::Command {
    let mut cmd = common::qsc_std_command();
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain");
    cmd
}

fn run_send(mut cmd: std::process::Command, payload: &Path, relay: &str) -> (bool, String) {
    let out = cmd
        .args([
            "relay",
            "send",
            "--to",
            "peer",
            "--file",
            payload.to_str().expect("payload path"),
            "--relay",
            relay,
        ])
        .output()
        .expect("run relay send");
    let mut text = String::from_utf8_lossy(&out.stdout).to_string();
    text.push_str(&String::from_utf8_lossy(&out.stderr));
    (out.status.success(), text)
}

fn write_file(path: &Path, contents: &[u8]) {
    let mut f = fs::File::create(path).expect("create file");
    f.write_all(contents).expect("write file");
}

// ===========================================================================
// FAMILY 1 — a system-trusted CA is ACCEPTED (acceptance a)
// ===========================================================================

#[test]
fn family1_system_trusted_ca_is_accepted_with_no_explicit_ca() {
    let ca = make_ca("NA-0663 family1 CA");
    let (chain, key) = make_leaf(&ca);
    let server = start_tls_server(chain, key, "200 OK");

    let (cfg, payload) = prepared_cfg("family1");
    let ca_path = cfg.join("system_ca.pem");
    write_file(&ca_path, ca.pem.as_bytes());

    // The OS trust seam: rustls-native-certs/openssl-probe honor SSL_CERT_FILE.
    // NO explicit CA option is configured anywhere in this case.
    let mut cmd = base_send_command(&cfg);
    cmd.env("SSL_CERT_FILE", &ca_path);
    let (ok, text) = run_send(cmd, &payload, server.base_url.as_str());

    assert!(
        !text.contains("relay_tls_untrusted"),
        "a system-trusted CA must not report a trust failure: {text}"
    );
    assert!(
        ok,
        "push over a system-trusted CA should succeed, got: {text}"
    );
}

// ===========================================================================
// FAMILY 2 — an EXPLICIT CA file is ACCEPTED, and is purely ADDITIVE (b)
// ===========================================================================

#[test]
fn family2_explicit_ca_accepted_via_env_ingress() {
    let ca = make_ca("NA-0663 family2 env CA");
    let (chain, key) = make_leaf(&ca);
    let server = start_tls_server(chain, key, "200 OK");

    let (cfg, payload) = prepared_cfg("family2_env");
    let ca_path = cfg.join("explicit_ca.pem");
    write_file(&ca_path, ca.pem.as_bytes());

    // NO SSL_CERT_FILE here: the explicit option alone must carry it.
    let mut cmd = base_send_command(&cfg);
    cmd.env("QSC_RELAY_CA_FILE", &ca_path);
    let (ok, text) = run_send(cmd, &payload, server.base_url.as_str());

    assert!(
        !text.contains("relay_tls_untrusted"),
        "an explicit CA must be trusted: {text}"
    );
    assert!(ok, "push with an explicit CA should succeed, got: {text}");
}

#[test]
fn family2_explicit_ca_accepted_via_fallback_env_ingress() {
    let ca = make_ca("NA-0663 family2 fallback CA");
    let (chain, key) = make_leaf(&ca);
    let server = start_tls_server(chain, key, "200 OK");

    let (cfg, payload) = prepared_cfg("family2_fallback");
    let ca_path = cfg.join("explicit_ca.pem");
    write_file(&ca_path, ca.pem.as_bytes());

    let mut cmd = base_send_command(&cfg);
    cmd.env("RELAY_CA_FILE", &ca_path);
    let (ok, text) = run_send(cmd, &payload, server.base_url.as_str());

    assert!(!text.contains("relay_tls_untrusted"), "{text}");
    assert!(ok, "RELAY_CA_FILE ingress should succeed, got: {text}");
}

#[test]
fn family2_explicit_ca_accepted_via_cli_verb_ingress() {
    let ca = make_ca("NA-0663 family2 cli CA");
    let (chain, key) = make_leaf(&ca);
    let server = start_tls_server(chain, key, "200 OK");

    let (cfg, payload) = prepared_cfg("family2_cli");
    let ca_path = cfg.join("explicit_ca.pem");
    write_file(&ca_path, ca.pem.as_bytes());

    // Ingress: the CLI verb writes the vault secret; the later send reads it back
    // with NO env set at all.
    let set = base_send_command(&cfg)
        .args(["relay", "ca-set", "--path", ca_path.to_str().unwrap()])
        .output()
        .expect("run relay ca-set");
    let set_text = String::from_utf8_lossy(&set.stdout).to_string();
    assert!(set.status.success(), "relay ca-set failed: {set_text}");
    assert!(
        set_text.contains("relay_ca_file=set"),
        "expected the set marker, got: {set_text}"
    );
    // Redaction: the raw path never appears in the verb's output.
    assert!(
        !set_text.contains(ca_path.to_str().unwrap()),
        "the CA path must be redacted in output: {set_text}"
    );

    let show = base_send_command(&cfg)
        .args(["relay", "ca-show"])
        .output()
        .expect("run relay ca-show");
    let show_text = String::from_utf8_lossy(&show.stdout).to_string();
    assert!(show.status.success(), "relay ca-show failed: {show_text}");
    assert!(
        show_text.contains("relay_ca_file=true"),
        "ca-show should report configured: {show_text}"
    );
    assert!(
        !show_text.contains(ca_path.to_str().unwrap()),
        "the CA path must be redacted in ca-show: {show_text}"
    );

    let (ok, text) = run_send(base_send_command(&cfg), &payload, server.base_url.as_str());
    assert!(!text.contains("relay_tls_untrusted"), "{text}");
    assert!(ok, "the vault-stored CA should be honored, got: {text}");

    // And clearing it puts trust back where it was.
    let cleared = base_send_command(&cfg)
        .args(["relay", "ca-clear"])
        .output()
        .expect("run relay ca-clear");
    assert!(cleared.status.success());
    let show2 = base_send_command(&cfg)
        .args(["relay", "ca-show"])
        .output()
        .expect("run relay ca-show after clear");
    assert!(String::from_utf8_lossy(&show2.stdout).contains("relay_ca_file=false"));
}

#[test]
fn family2_explicit_ca_is_additive_and_never_narrows_existing_trust() {
    // ADDITIVITY PIN: with an explicit CA configured, the pre-existing plain-http
    // loopback path and the auth-reject path must behave exactly as at base.
    let ca = make_ca("NA-0663 additivity CA");
    let (cfg, payload) = prepared_cfg("family2_additive");
    let ca_path = cfg.join("explicit_ca.pem");
    write_file(&ca_path, ca.pem.as_bytes());

    // (i) plain-http loopback still works with the CA option set.
    let inbox = common::start_inbox_server(1024 * 1024, 64);
    let mut cmd = base_send_command(&cfg);
    cmd.env("QSC_RELAY_CA_FILE", &ca_path);
    let (ok, text) = run_send(cmd, &payload, inbox.base_url());
    assert!(
        ok,
        "an explicit CA must not disturb the plain-http loopback path: {text}"
    );
    assert!(!text.contains("relay_tls_untrusted"), "{text}");

    // (ii) the auth-reject path is still its own distinct outcome.
    let auth_ca = make_ca("NA-0663 additivity auth CA");
    let (chain, key) = make_leaf(&auth_ca);
    let server = start_tls_server(chain, key, "401 Unauthorized");
    let auth_ca_path = cfg.join("auth_ca.pem");
    write_file(&auth_ca_path, auth_ca.pem.as_bytes());

    let mut cmd = base_send_command(&cfg);
    cmd.env("QSC_RELAY_CA_FILE", &auth_ca_path);
    let (ok, text) = run_send(cmd, &payload, server.base_url.as_str());
    assert!(!ok, "a 401 must still fail: {text}");
    assert!(
        text.contains("relay_unauthorized"),
        "a 401 must stay relay_unauthorized, not a trust error: {text}"
    );
    assert!(
        !text.contains("relay_tls_untrusted"),
        "an auth rejection is NOT a trust failure: {text}"
    );
}

// ===========================================================================
// FAMILY 3 — an UNTRUSTED certificate yields the TYPED outcome (c)
// ===========================================================================

#[test]
fn family3_untrusted_cert_yields_typed_outcome_not_opaque_failure() {
    // The listener presents a leaf from CA-B, which is installed NOWHERE.
    let rogue_ca = make_ca("NA-0663 rogue CA");
    let (chain, key) = make_leaf(&rogue_ca);
    let server = start_tls_server(chain, key, "200 OK");

    let (cfg, payload) = prepared_cfg("family3_untrusted");
    let (ok, text) = run_send(base_send_command(&cfg), &payload, server.base_url.as_str());

    assert!(!ok, "an untrusted certificate must fail the push: {text}");
    assert!(
        text.contains("relay_tls_untrusted"),
        "expected the typed trust outcome, got: {text}"
    );
    // THE POINT OF THE LANE: no longer the opaque value.
    assert!(
        !text.contains("relay_inbox_push_failed"),
        "the trust failure must NOT collapse into the opaque value: {text}"
    );
}

#[test]
fn family3_trust_failure_is_distinct_from_refused_dns_and_auth() {
    let (cfg, payload) = prepared_cfg("family3_distinct");

    // (i) connection refused — a closed loopback port.
    let (_ok, refused) = run_send(base_send_command(&cfg), &payload, "https://127.0.0.1:1");
    assert!(
        !refused.contains("relay_tls_untrusted"),
        "connection-refused must not be reported as a trust failure: {refused}"
    );

    // (ii) DNS failure — RFC 2606 reserves `.invalid`; it never resolves, and no
    // external endpoint is contacted.
    let (_ok, dns) = run_send(
        base_send_command(&cfg),
        &payload,
        "https://na0663-nonexistent.invalid",
    );
    assert!(
        !dns.contains("relay_tls_untrusted"),
        "a DNS failure must not be reported as a trust failure: {dns}"
    );

    // (iii) auth rejection over a TRUSTED certificate.
    let ca = make_ca("NA-0663 distinct auth CA");
    let (chain, key) = make_leaf(&ca);
    let server = start_tls_server(chain, key, "401 Unauthorized");
    let ca_path = cfg.join("distinct_ca.pem");
    write_file(&ca_path, ca.pem.as_bytes());
    let mut cmd = base_send_command(&cfg);
    cmd.env("QSC_RELAY_CA_FILE", &ca_path);
    let (_ok, auth) = run_send(cmd, &payload, server.base_url.as_str());
    assert!(
        auth.contains("relay_unauthorized") && !auth.contains("relay_tls_untrusted"),
        "auth rejection must stay distinct from a trust failure: {auth}"
    );
}

#[test]
fn family3_ca_config_failures_are_each_enumerated_and_fail_closed() {
    let ca = make_ca("NA-0663 config-failure CA");
    let (chain, key) = make_leaf(&ca);
    let server = start_tls_server(chain, key, "200 OK");
    let (cfg, payload) = prepared_cfg("family3_config");

    // missing
    let mut cmd = base_send_command(&cfg);
    cmd.env("QSC_RELAY_CA_FILE", cfg.join("absent_ca.pem"));
    let (ok, text) = run_send(cmd, &payload, server.base_url.as_str());
    assert!(!ok, "a missing CA file must fail closed: {text}");
    assert!(
        text.contains("relay_ca_file_missing"),
        "expected relay_ca_file_missing, got: {text}"
    );

    // unreadable (a directory is not a readable file)
    let dir_path = cfg.join("ca_dir");
    ensure_dir_700(&dir_path);
    let mut cmd = base_send_command(&cfg);
    cmd.env("QSC_RELAY_CA_FILE", &dir_path);
    let (ok, text) = run_send(cmd, &payload, server.base_url.as_str());
    assert!(!ok, "an unreadable CA file must fail closed: {text}");
    assert!(
        text.contains("relay_ca_file_unreadable"),
        "expected relay_ca_file_unreadable, got: {text}"
    );

    // invalid (readable, but not PEM)
    let junk = cfg.join("not_a_cert.pem");
    write_file(&junk, b"this is definitely not a certificate\n");
    let mut cmd = base_send_command(&cfg);
    cmd.env("QSC_RELAY_CA_FILE", &junk);
    let (ok, text) = run_send(cmd, &payload, server.base_url.as_str());
    assert!(!ok, "an unparsable CA file must fail closed: {text}");
    assert!(
        text.contains("relay_ca_file_invalid"),
        "expected relay_ca_file_invalid, got: {text}"
    );
}

// ===========================================================================
// FAMILY 4 — NO BYPASS EXISTS ANYWHERE (acceptance d)
// ===========================================================================

/// The needles are assembled at RUNTIME from fragments so that this file can be
/// scanned by its own test without the literals appearing in it.
fn bypass_needles() -> Vec<String> {
    let danger = format!("danger_accept_{}", "invalid");
    let danger_hyphen = format!("danger-accept-{}", "invalid");
    vec![
        format!("{danger}_certs"),
        format!("{danger}_hostnames"),
        format!("{danger_hyphen}-certs"),
        format!("accept_{}_certs", "invalid"),
        format!("accept_{}_hostnames", "invalid"),
        format!("{}_skip_verify", "insecure"),
        format!("{}-skip-verify", "insecure"),
        format!("accept_{}_cert", "any"),
    ]
}

fn scan_dir_for(dir: &Path, needles: &[String], hits: &mut Vec<String>) {
    let entries = match fs::read_dir(dir) {
        Ok(v) => v,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            scan_dir_for(&path, needles, hits);
            continue;
        }
        let is_source = path
            .extension()
            .map(|e| e == "rs" || e == "toml")
            .unwrap_or(false);
        if !is_source {
            continue;
        }
        let body = match fs::read_to_string(&path) {
            Ok(v) => v,
            Err(_) => continue,
        };
        for needle in needles {
            if body.contains(needle.as_str()) {
                hits.push(format!("{} contains {}", path.display(), needle));
            }
        }
    }
}

#[test]
fn family4_no_certificate_bypass_exists_in_source_or_tests() {
    let crate_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let needles = bypass_needles();
    let mut hits = Vec::new();
    scan_dir_for(&crate_root.join("src"), &needles, &mut hits);
    scan_dir_for(&crate_root.join("tests"), &needles, &mut hits);
    scan_dir_for(&crate_root.join("Cargo.toml"), &needles, &mut hits);
    // Cargo.toml is a file, not a dir; scan it directly too.
    if let Ok(manifest) = fs::read_to_string(crate_root.join("Cargo.toml")) {
        for needle in &needles {
            assert!(
                !manifest.contains(needle.as_str()),
                "manifest contains bypass needle {needle}"
            );
        }
    }
    assert!(
        hits.is_empty(),
        "certificate-bypass needles found (the hard boundary forbids all of these): {hits:?}"
    );
}

#[test]
fn family4_every_trust_knob_misset_still_refuses_an_untrusted_certificate() {
    // FAIL-CLOSED PROOF: the listener presents a rogue leaf, and BOTH trust knobs
    // are deliberately mis-set to an unrelated CA. No configuration accepts an
    // unverifiable certificate.
    let rogue_ca = make_ca("NA-0663 fail-closed rogue CA");
    let (chain, key) = make_leaf(&rogue_ca);
    let server = start_tls_server(chain, key, "200 OK");

    let (cfg, payload) = prepared_cfg("family4_failclosed");
    let wrong_ca = make_ca("NA-0663 fail-closed wrong CA");
    let wrong_path = cfg.join("wrong_ca.pem");
    write_file(&wrong_path, wrong_ca.pem.as_bytes());

    let another_wrong = make_ca("NA-0663 fail-closed other CA");
    let another_path = cfg.join("another_wrong_ca.pem");
    write_file(&another_path, another_wrong.pem.as_bytes());

    let mut cmd = base_send_command(&cfg);
    cmd.env("QSC_RELAY_CA_FILE", &wrong_path)
        .env("SSL_CERT_FILE", &another_path);
    let (ok, text) = run_send(cmd, &payload, server.base_url.as_str());

    assert!(
        !ok,
        "no combination of trust settings may accept an unverifiable certificate: {text}"
    );
    assert!(
        text.contains("relay_tls_untrusted"),
        "the refusal must still be the typed outcome: {text}"
    );
}

// ===========================================================================
// The pub library surface the GUI (slice B) will consume.
// ===========================================================================

#[test]
fn pub_library_surface_reports_presence_and_hash_without_the_path() {
    // Pure surface check: with nothing configured and no vault, the accessor
    // reports "not configured" rather than panicking or leaking.
    let status = qsc::transport::relay_ca_file_show();
    assert!(!status.configured);
    assert!(status.path_hash.is_none());
}
