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

struct Store {
    queues: HashMap<String, VecDeque<(String, Vec<u8>)>>,
    next_id: u64,
    required_token: String,
}

impl Store {
    fn new(required_token: String) -> Self {
        Self {
            queues: HashMap::new(),
            next_id: 1,
            required_token,
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
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("qsc"));
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

fn relay_set_inbox_token(cfg: &Path, token: &str) {
    let out = qsc_base(cfg)
        .args(["relay", "inbox-set", "--token", token])
        .output()
        .expect("relay inbox set");
    assert!(out.status.success(), "{}", combined_output(&out));
}

fn init_mock_vault(cfg: &Path) {
    let out = qsc_base(cfg)
        .args(["vault", "init", "--non-interactive", "--key-source", "mock"])
        .output()
        .expect("vault init");
    assert!(out.status.success(), "{}", combined_output(&out));
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

    if method == "POST" && target.starts_with("/v1/push/") {
        let channel = &target["/v1/push/".len()..];
        if channel.is_empty() {
            let _ = write_plain(&mut stream, 400, "ERR_BAD_CHANNEL");
            return;
        }
        let mut s = store.lock().expect("store lock");
        let id = s.next_id.to_string();
        s.next_id = s.next_id.saturating_add(1);
        s.queues
            .entry(channel.to_string())
            .or_default()
            .push_back((id.clone(), body));
        let _ = write_json(&mut stream, 200, &format!("{{\"id\":\"{}\"}}", id));
        return;
    }
    if method == "GET" && target.starts_with("/v1/pull/") {
        let mut path = &target["/v1/pull/".len()..];
        let mut max = 1usize;
        if let Some((p, q)) = path.split_once('?') {
            path = p;
            for part in q.split('&') {
                if let Some(v) = part.strip_prefix("max=") {
                    if let Ok(n) = v.parse::<usize>() {
                        max = n;
                    }
                }
            }
        }
        if path.is_empty() {
            let _ = write_plain(&mut stream, 400, "ERR_BAD_CHANNEL");
            return;
        }
        let mut s = store.lock().expect("store lock");
        let mut items = Vec::new();
        if let Some(queue) = s.queues.get_mut(path) {
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

    let recv_output = qsc_base(&cfg)
        .env("RELAY_TOKEN", token)
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
        .expect("recv with token");
    let recv_out = combined_output(&recv_output);
    assert!(
        recv_output.status.success(),
        "receive should succeed with token"
    );
    assert!(
        recv_out.contains("event=recv_commit"),
        "missing recv_commit marker"
    );
    assert!(out_dir.join("recv_1.bin").exists(), "expected recv file");

    assert!(!send_out.contains(token), "send output leaked token");
    assert!(!recv_out.contains(token), "recv output leaked token");
    assert!(
        !send_out.contains("Bearer") && !recv_out.contains("Bearer"),
        "output leaked bearer auth text"
    );
}
