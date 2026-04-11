mod common;

use std::collections::{HashMap, VecDeque};
use std::env;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread;
use std::time::Duration;

const ROUTE_TOKEN_BOB: &str = "route_token_bob_abcdefghijklmnopqr";
const ROUTE_TOKEN_BOB_ALT: &str = "route_token_bob_alt_abcdefghijklmnop";

struct Store {
    queues: HashMap<String, VecDeque<(String, Vec<u8>)>>,
    next_id: u64,
    required_token: String,
    last_target: Option<String>,
    last_route_token_header: Option<String>,
    last_auth: Option<String>,
}

impl Store {
    fn new(required_token: String) -> Self {
        Self {
            queues: HashMap::new(),
            next_id: 1,
            required_token,
            last_target: None,
            last_route_token_header: None,
            last_auth: None,
        }
    }
}

struct AuthRelayServer {
    base_url: String,
    store: Arc<Mutex<Store>>,
    shutdown: Arc<AtomicBool>,
    handle: Option<thread::JoinHandle<()>>,
}

impl AuthRelayServer {
    fn base_url(&self) -> &str {
        &self.base_url
    }

    fn queue_len(&self, channel: &str) -> usize {
        let store = self.store.lock().expect("store lock");
        store.queues.get(channel).map(|q| q.len()).unwrap_or(0)
    }

    fn last_target(&self) -> Option<String> {
        self.store.lock().expect("store lock").last_target.clone()
    }

    fn last_route_token_header(&self) -> Option<String> {
        self.store
            .lock()
            .expect("store lock")
            .last_route_token_header
            .clone()
    }
}

impl Drop for AuthRelayServer {
    fn drop(&mut self) {
        self.shutdown.store(true, Ordering::SeqCst);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

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

fn create_dir_700(path: &Path) {
    let _ = fs::remove_dir_all(path);
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

fn combined_output(output: &std::process::Output) -> String {
    let mut out = String::from_utf8_lossy(&output.stdout).to_string();
    out.push_str(&String::from_utf8_lossy(&output.stderr));
    out
}

fn qsc_base(cfg: &Path) -> Command {
    let mut cmd = common::qsc_std_command();
    cmd.env("QSC_CONFIG_DIR", cfg)
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
    assert!(out.status.success(), "{}", combined_output(&out));
}

fn contact_device_ids(cfg: &Path, label: &str) -> Vec<String> {
    let out = qsc_base(cfg)
        .args(["contacts", "device", "list", "--label", label])
        .output()
        .expect("contacts device list");
    assert!(out.status.success(), "{}", combined_output(&out));
    combined_output(&out)
        .lines()
        .filter_map(|line| {
            if !line.starts_with("device=") {
                return None;
            }
            line.split_whitespace()
                .find_map(|part| part.strip_prefix("device="))
                .map(ToOwned::to_owned)
        })
        .collect()
}

fn contacts_device_add_with_route_token(cfg: &Path, label: &str, fp: &str, token: &str) {
    let out = qsc_base(cfg)
        .args([
            "contacts",
            "device",
            "add",
            "--label",
            label,
            "--fp",
            fp,
            "--route-token",
            token,
        ])
        .output()
        .expect("contacts device add route token");
    assert!(out.status.success(), "{}", combined_output(&out));
}

fn contacts_device_trust(cfg: &Path, label: &str, device: &str) {
    let out = qsc_base(cfg)
        .args([
            "contacts",
            "device",
            "trust",
            "--label",
            label,
            "--device",
            device,
            "--confirm",
        ])
        .output()
        .expect("contacts device trust");
    assert!(out.status.success(), "{}", combined_output(&out));
}

fn contacts_device_primary_set(cfg: &Path, label: &str, device: &str) {
    let out = qsc_base(cfg)
        .args([
            "contacts",
            "device",
            "primary",
            "set",
            "--label",
            label,
            "--device",
            device,
            "--confirm",
        ])
        .output()
        .expect("contacts device primary set");
    assert!(out.status.success(), "{}", combined_output(&out));
}

fn relay_set_inbox_token(cfg: &Path, token: &str) {
    let out = qsc_base(cfg)
        .args(["relay", "inbox-set", "--token", token])
        .output()
        .expect("relay inbox set");
    assert!(out.status.success(), "{}", combined_output(&out));
}

fn init_mock_vault(cfg: &Path) {
    common::init_mock_vault(cfg);
}

fn tui_set_relay_token_file(cfg: &Path, token_file: &Path) {
    let script = format!("/relay set token-file {}\n/exit\n", token_file.display());
    let out = qsc_base(cfg)
        .env("QSC_TUI_HEADLESS", "1")
        .env("QSC_TUI_SCRIPT", script)
        .args(["tui"])
        .output()
        .expect("tui set relay token file");
    assert!(out.status.success(), "{}", combined_output(&out));
}

fn receive_once_with_token(
    cfg: &Path,
    token: &str,
    relay_url: &str,
    out_dir: &Path,
) -> std::process::Output {
    qsc_base(cfg)
        .env("RELAY_TOKEN", token)
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            relay_url,
            "--from",
            "bob",
            "--mailbox",
            ROUTE_TOKEN_BOB,
            "--max",
            "1",
            "--out",
            out_dir.to_str().expect("out path"),
        ])
        .output()
        .expect("recv with token")
}

fn start_auth_server(required_token: &str) -> AuthRelayServer {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind auth server");
    listener
        .set_nonblocking(true)
        .expect("auth listener nonblocking");
    let addr = listener.local_addr().expect("auth listener addr");
    let store = Arc::new(Mutex::new(Store::new(required_token.to_string())));
    let shutdown = Arc::new(AtomicBool::new(false));
    let shutdown_t = Arc::clone(&shutdown);
    let store_t = Arc::clone(&store);
    let handle = thread::spawn(move || {
        while !shutdown_t.load(Ordering::SeqCst) {
            match listener.accept() {
                Ok((stream, _)) => {
                    let s = Arc::clone(&store_t);
                    thread::spawn(move || handle_conn(stream, s));
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(10));
                }
                Err(_) => break,
            }
        }
    });
    AuthRelayServer {
        base_url: format!("http://{}", addr),
        store,
        shutdown,
        handle: Some(handle),
    }
}

fn handle_conn(mut stream: TcpStream, store: Arc<Mutex<Store>>) {
    let mut buf = Vec::new();
    let mut tmp = [0u8; 1024];
    let mut header_end = None;
    loop {
        match stream.read(&mut tmp) {
            Ok(0) => break,
            Ok(n) => {
                buf.extend_from_slice(&tmp[..n]);
                if header_end.is_none() {
                    if let Some(pos) = find_header_end(&buf) {
                        header_end = Some(pos);
                        break;
                    }
                }
            }
            Err(_) => break,
        }
    }
    let header_end = match header_end {
        Some(v) => v,
        None => {
            let _ = write_plain(&mut stream, 400, "bad request");
            return;
        }
    };
    let header = String::from_utf8_lossy(&buf[..header_end]);
    let mut lines = header.split("\r\n");
    let req_line = match lines.next() {
        Some(v) => v,
        None => {
            let _ = write_plain(&mut stream, 400, "bad request");
            return;
        }
    };
    let mut req = req_line.split_whitespace();
    let method = req.next().unwrap_or("");
    let target = req.next().unwrap_or("");
    let mut content_len = 0usize;
    let mut auth = String::new();
    let mut route_token_header = None::<String>;
    for line in lines {
        let lower = line.to_ascii_lowercase();
        if let Some(v) = lower.strip_prefix("content-length:") {
            if let Ok(n) = v.trim().parse::<usize>() {
                content_len = n;
            }
        }
        if lower.starts_with("authorization:") {
            auth = line["authorization:".len()..].trim().to_string();
        }
        if lower.starts_with("x-qsl-route-token:") {
            route_token_header = Some(line["x-qsl-route-token:".len()..].trim().to_string());
        }
    }
    let mut body = Vec::new();
    if content_len > 0 {
        let mut body_buf = Vec::new();
        body_buf.extend_from_slice(&buf[(header_end + 4)..]);
        while body_buf.len() < content_len {
            let mut read_buf = [0u8; 1024];
            match stream.read(&mut read_buf) {
                Ok(0) => break,
                Ok(n) => body_buf.extend_from_slice(&read_buf[..n]),
                Err(_) => break,
            }
        }
        if body_buf.len() < content_len {
            let _ = write_plain(&mut stream, 400, "bad request");
            return;
        }
        body.extend_from_slice(&body_buf[..content_len]);
    }

    let required_token = { store.lock().expect("store lock").required_token.clone() };
    let expected_auth = format!("Bearer {}", required_token);
    if auth != expected_auth {
        let _ = write_plain(&mut stream, 401, "ERR_UNAUTHORIZED");
        return;
    }

    if method == "POST" && target == "/v1/push" {
        let channel = match resolve_route_token(route_token_header.clone()) {
            Ok(v) => v,
            Err(code) => {
                let _ = write_plain(&mut stream, 400, code);
                return;
            }
        };
        let mut s = store.lock().expect("store lock");
        s.last_target = Some(target.to_string());
        s.last_route_token_header = route_token_header.clone();
        s.last_auth = Some(auth.clone());
        let id = s.next_id.to_string();
        s.next_id = s.next_id.saturating_add(1);
        s.queues
            .entry(channel)
            .or_default()
            .push_back((id.clone(), body));
        let _ = write_json(&mut stream, 200, &format!("{{\"id\":\"{}\"}}", id));
        return;
    }
    if method == "GET" && target.starts_with("/v1/pull") {
        let (path, query) = match target.split_once('?') {
            Some((p, q)) => (p, Some(q)),
            None => (target, None),
        };
        if path != "/v1/pull" {
            let _ = write_plain(&mut stream, 404, "ERR_NOT_FOUND");
            return;
        }
        let mut max = 1usize;
        if let Some(q) = query {
            for part in q.split('&') {
                if let Some(v) = part.strip_prefix("max=") {
                    if let Ok(n) = v.parse::<usize>() {
                        max = n;
                    }
                }
            }
        }
        let channel = match resolve_route_token(route_token_header.clone()) {
            Ok(v) => v,
            Err(code) => {
                let _ = write_plain(&mut stream, 400, code);
                return;
            }
        };
        let mut s = store.lock().expect("store lock");
        s.last_target = Some(target.to_string());
        s.last_route_token_header = route_token_header;
        s.last_auth = Some(auth);
        let mut items = Vec::new();
        if let Some(queue) = s.queues.get_mut(channel.as_str()) {
            for _ in 0..max {
                if let Some((id, data)) = queue.pop_front() {
                    let data_json = data
                        .iter()
                        .map(|b| b.to_string())
                        .collect::<Vec<_>>()
                        .join(",");
                    items.push(format!("{{\"id\":\"{}\",\"data\":[{}]}}", id, data_json));
                } else {
                    break;
                }
            }
        }
        let body = format!("{{\"items\":[{}]}}", items.join(","));
        let _ = write_json(&mut stream, 200, &body);
        return;
    }
    let _ = write_plain(&mut stream, 404, "ERR_NOT_FOUND");
}

fn resolve_route_token(header_token: Option<String>) -> Result<String, &'static str> {
    let header_token = match header_token {
        None => None,
        Some(raw) => {
            let token = raw.trim();
            if token.is_empty() {
                return Err("ERR_MISSING_ROUTE_TOKEN");
            }
            Some(token.to_string())
        }
    };
    header_token.ok_or("ERR_MISSING_ROUTE_TOKEN")
}

fn find_header_end(buf: &[u8]) -> Option<usize> {
    buf.windows(4).position(|w| w == b"\r\n\r\n")
}

fn write_plain(stream: &mut TcpStream, status: u16, body: &str) -> std::io::Result<()> {
    let status_text = match status {
        200 => "OK",
        400 => "Bad Request",
        401 => "Unauthorized",
        404 => "Not Found",
        _ => "Error",
    };
    let resp = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        status,
        status_text,
        body.len(),
        body
    );
    stream.write_all(resp.as_bytes())
}

fn write_json(stream: &mut TcpStream, status: u16, body: &str) -> std::io::Result<()> {
    let status_text = if status == 200 { "OK" } else { "Error" };
    let resp = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        status,
        status_text,
        body.len(),
        body
    );
    stream.write_all(resp.as_bytes())
}

#[test]
fn relay_auth_without_token_fails_no_mutation() {
    let server = start_auth_server("token-abc");
    let base = safe_test_root().join(format!("na0107_no_token_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    let payload = base.join("payload.txt");
    fs::write(&payload, b"hello").expect("payload write");
    init_mock_vault(&cfg);
    contacts_add_with_route_token(&cfg, "bob", ROUTE_TOKEN_BOB);

    let output = qsc_base(&cfg)
        .env_remove("QSC_RELAY_TOKEN")
        .env_remove("RELAY_TOKEN")
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "bob",
            "--file",
            payload.to_str().expect("payload path"),
        ])
        .output()
        .expect("send no token");

    let out = combined_output(&output);
    assert!(!output.status.success(), "send should fail without token");
    assert!(
        out.contains("code=relay_unauthorized"),
        "expected relay_unauthorized marker, got: {out}"
    );
    assert_eq!(
        server.queue_len(ROUTE_TOKEN_BOB),
        0,
        "unauthorized push mutated inbox"
    );
}

#[test]
fn relay_auth_with_token_send_receive_ok_and_no_secret_leak() {
    let token = "token-abc";
    let server = start_auth_server(token);
    let base = safe_test_root().join(format!("na0107_with_token_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    let out_dir = base.join("out");
    create_dir_700(&cfg);
    create_dir_700(&out_dir);
    let payload = base.join("payload.txt");
    fs::write(&payload, b"hello").expect("payload write");
    init_mock_vault(&cfg);
    contacts_add_with_route_token(&cfg, "bob", ROUTE_TOKEN_BOB);
    relay_set_inbox_token(&cfg, ROUTE_TOKEN_BOB);

    let send_output = qsc_base(&cfg)
        .env("RELAY_TOKEN", token)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "bob",
            "--file",
            payload.to_str().expect("payload path"),
        ])
        .output()
        .expect("send with token");
    let send_out = combined_output(&send_output);
    assert!(
        send_output.status.success(),
        "send should succeed with token"
    );
    assert_eq!(
        server.last_target().as_deref(),
        Some("/v1/push"),
        "send should use canonical token-free push path"
    );
    assert_eq!(
        server.last_route_token_header().as_deref(),
        Some(ROUTE_TOKEN_BOB),
        "send should carry route token in X-QSL-Route-Token"
    );

    let mut recv_output = receive_once_with_token(&cfg, token, server.base_url(), &out_dir);
    let mut recv_out = combined_output(&recv_output);
    if !recv_output.status.success() {
        for _ in 0..4 {
            thread::sleep(Duration::from_millis(100));
            recv_output = receive_once_with_token(&cfg, token, server.base_url(), &out_dir);
            recv_out = combined_output(&recv_output);
            if recv_output.status.success() {
                break;
            }
        }
    }
    assert!(
        recv_output.status.success(),
        "receive should succeed with token: {recv_out}"
    );
    assert!(
        recv_out.contains("event=recv_commit"),
        "missing recv_commit marker"
    );
    assert!(out_dir.join("recv_1.bin").exists(), "expected recv file");
    assert_eq!(
        server.last_target().as_deref(),
        Some("/v1/pull?max=1"),
        "receive should use canonical token-free pull path"
    );
    assert_eq!(
        server.last_route_token_header().as_deref(),
        Some(ROUTE_TOKEN_BOB),
        "receive should carry route token in X-QSL-Route-Token"
    );

    assert!(!send_out.contains(token), "send output leaked token");
    assert!(!recv_out.contains(token), "recv output leaked token");
    assert!(
        !send_out.contains("Bearer") && !recv_out.contains("Bearer"),
        "output leaked bearer auth text"
    );
}

#[test]
fn relay_auth_uses_account_token_file_when_env_missing() {
    let token = "token-abc";
    let server = start_auth_server(token);
    let base = safe_test_root().join(format!("na0183_token_file_auth_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    let out_dir = base.join("out");
    create_dir_700(&cfg);
    create_dir_700(&out_dir);
    let token_file = base.join("relay.token");
    fs::write(&token_file, token).expect("token file write");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&token_file, fs::Permissions::from_mode(0o600))
            .expect("chmod token 600");
    }
    let payload = base.join("payload.txt");
    fs::write(&payload, b"hello").expect("payload write");
    init_mock_vault(&cfg);
    contacts_add_with_route_token(&cfg, "bob", ROUTE_TOKEN_BOB);
    relay_set_inbox_token(&cfg, ROUTE_TOKEN_BOB);
    tui_set_relay_token_file(&cfg, &token_file);

    let send_output = qsc_base(&cfg)
        .env_remove("QSC_RELAY_TOKEN")
        .env_remove("RELAY_TOKEN")
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "bob",
            "--file",
            payload.to_str().expect("payload path"),
        ])
        .output()
        .expect("send with token file config");
    let send_out = combined_output(&send_output);
    assert!(send_output.status.success(), "{send_out}");

    let recv_output = qsc_base(&cfg)
        .env_remove("QSC_RELAY_TOKEN")
        .env_remove("RELAY_TOKEN")
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--from",
            "bob",
            "--mailbox",
            ROUTE_TOKEN_BOB,
            "--max",
            "1",
            "--out",
            out_dir.to_str().expect("out path"),
        ])
        .output()
        .expect("recv with token file config");
    let recv_out = combined_output(&recv_output);
    assert!(recv_output.status.success(), "{recv_out}");
    assert!(
        recv_out.contains("event=recv_commit"),
        "missing recv_commit marker"
    );
    assert!(!send_out.contains(token), "send output leaked token");
    assert!(!recv_out.contains(token), "recv output leaked token");
    let token_file_path = token_file.to_string_lossy();
    assert!(
        !send_out.contains(token_file_path.as_ref()),
        "send output leaked token-file path"
    );
    assert!(
        !recv_out.contains(token_file_path.as_ref()),
        "recv output leaked token-file path"
    );
    assert!(
        !send_out.contains("Authorization")
            && !recv_out.contains("Authorization")
            && !send_out.contains("Bearer")
            && !recv_out.contains("Bearer"),
        "output leaked auth header text"
    );
}

#[test]
fn relay_auth_uses_explicit_primary_device_route_token() {
    let token = "token-abc";
    let server = start_auth_server(token);
    let base = safe_test_root().join(format!("na0217e_primary_route_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    let payload = base.join("payload.txt");
    fs::write(&payload, b"hello").expect("payload write");
    init_mock_vault(&cfg);

    contacts_add_with_route_token(&cfg, "bob", ROUTE_TOKEN_BOB);
    let original_device = contact_device_ids(&cfg, "bob")
        .into_iter()
        .next()
        .expect("original device id");
    let spaced_alt = format!("  {}  ", ROUTE_TOKEN_BOB_ALT);
    contacts_device_add_with_route_token(&cfg, "bob", "fp-test-second", spaced_alt.as_str());
    let secondary_device = contact_device_ids(&cfg, "bob")
        .into_iter()
        .find(|device| device != &original_device)
        .expect("secondary device id");
    contacts_device_trust(&cfg, "bob", secondary_device.as_str());
    contacts_device_primary_set(&cfg, "bob", secondary_device.as_str());

    let send_output = qsc_base(&cfg)
        .env("RELAY_TOKEN", token)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "bob",
            "--file",
            payload.to_str().expect("payload path"),
        ])
        .output()
        .expect("send with explicit primary device");
    assert!(
        send_output.status.success(),
        "{}",
        combined_output(&send_output)
    );
    assert_eq!(
        server.last_target().as_deref(),
        Some("/v1/push"),
        "send should keep canonical token-free push path"
    );
    assert_eq!(
        server.last_route_token_header().as_deref(),
        Some(ROUTE_TOKEN_BOB_ALT),
        "send should use the explicit primary device route token after normalization"
    );
    assert_eq!(
        server.queue_len(ROUTE_TOKEN_BOB_ALT),
        1,
        "send should enqueue on the explicit primary device route token"
    );
    assert_eq!(
        server.queue_len(ROUTE_TOKEN_BOB),
        0,
        "send should not enqueue on the superseded primary route token"
    );
}
