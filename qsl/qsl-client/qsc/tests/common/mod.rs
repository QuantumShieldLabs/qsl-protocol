#![allow(dead_code)]

use assert_cmd::Command;
use serde::Serialize;
use std::collections::{HashMap, VecDeque};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::Command as StdCommand;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread;
use std::time::{Duration, Instant};
use std::time::{SystemTime, UNIX_EPOCH};

#[allow(dead_code)]
pub fn init_mock_vault(cfg: &Path) {
    let out = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg)
        .args(["vault", "init", "--non-interactive", "--key-source", "mock"])
        .output()
        .expect("vault init mock");
    assert!(
        out.status.success(),
        "vault init failed: {}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}

fn ensure_dir_700(path: &Path) {
    std::fs::create_dir_all(path).expect("create dir");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o700)).expect("chmod 700");
    }
}

static TEST_NONCE_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

fn qsc_test_root_base() -> PathBuf {
    if let Ok(v) = std::env::var("QSC_TEST_ROOT") {
        return PathBuf::from(v);
    }
    if let Ok(v) = std::env::var("CARGO_TARGET_DIR") {
        return PathBuf::from(v);
    }
    PathBuf::from("target")
}

pub fn unique_test_root(tag: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let seq = TEST_NONCE_COUNTER.fetch_add(1, Ordering::Relaxed);
    qsc_test_root_base().join("qsc-test-tmp").join(format!(
        "{tag}_{}_{}_{}",
        std::process::id(),
        nonce,
        seq
    ))
}

#[derive(Clone, Debug)]
pub struct TestIsolation {
    pub root: PathBuf,
    home: PathBuf,
    xdg_config_home: PathBuf,
    tmpdir: PathBuf,
}

impl TestIsolation {
    pub fn new(tag: &str) -> Self {
        let root = unique_test_root(tag);
        let home = root.join("home");
        let xdg_config_home = home.join(".config");
        let tmpdir = root.join("tmp");
        ensure_dir_700(&root);
        ensure_dir_700(&home);
        ensure_dir_700(&xdg_config_home);
        ensure_dir_700(&tmpdir);
        Self {
            root,
            home,
            xdg_config_home,
            tmpdir,
        }
    }

    pub fn apply_to(&self, cmd: &mut StdCommand) {
        cmd.env("HOME", &self.home)
            .env("XDG_CONFIG_HOME", &self.xdg_config_home)
            .env("TMPDIR", &self.tmpdir);
    }
}

#[derive(Serialize)]
struct InboxPullItem {
    id: String,
    data: Vec<u8>,
}

#[derive(Serialize)]
struct InboxPullResp {
    items: Vec<InboxPullItem>,
}

struct InboxStore {
    queues: HashMap<String, VecDeque<(String, Vec<u8>)>>,
    next_id: u64,
    max_body: usize,
    max_queue: usize,
}

impl InboxStore {
    fn new(max_body: usize, max_queue: usize) -> Self {
        Self {
            queues: HashMap::new(),
            next_id: 1,
            max_body,
            max_queue,
        }
    }
}

#[allow(dead_code)]
pub struct InboxTestServer {
    base_url: String,
    store: Arc<Mutex<InboxStore>>,
    shutdown: Arc<AtomicBool>,
    handle: Option<thread::JoinHandle<()>>,
}

#[allow(dead_code)]
impl InboxTestServer {
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub fn drain_channel(&self, channel: &str) -> Vec<Vec<u8>> {
        let mut store = self.store.lock().unwrap();
        let mut out = Vec::new();
        if let Some(queue) = store.queues.get_mut(channel) {
            while let Some((_, data)) = queue.pop_front() {
                out.push(data);
            }
        }
        out
    }

    pub fn replace_channel(&self, channel: &str, items: Vec<Vec<u8>>) {
        let mut store = self.store.lock().unwrap();
        let next_id = store.next_id;
        store.next_id = store.next_id.saturating_add(items.len() as u64);
        let queue = store.queues.entry(channel.to_string()).or_default();
        queue.clear();
        for (idx, data) in items.into_iter().enumerate() {
            let id = next_id.saturating_add(idx as u64).to_string();
            queue.push_back((id, data));
        }
    }

    pub fn enqueue_raw(&self, channel: &str, data: Vec<u8>) {
        let mut store = self.store.lock().unwrap();
        let id = store.next_id.to_string();
        store.next_id += 1;
        let queue = store.queues.entry(channel.to_string()).or_default();
        queue.push_back((id, data));
    }
}

impl Drop for InboxTestServer {
    fn drop(&mut self) {
        self.shutdown.store(true, Ordering::SeqCst);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

// Mock relay contract (tests-only):
// - Connection model: single-request-per-connection with "Connection: close" responses.
// - Timeout policy: bounded read timeout + request deadline to prevent CI hangs on partial/malformed input.
// - Readiness semantics: server is considered ready only after a bounded streak of successful push+pull probes.
pub fn start_inbox_server(max_body: usize, max_queue: usize) -> InboxTestServer {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind inbox server");
    let addr = listener.local_addr().expect("inbox addr");
    listener
        .set_nonblocking(true)
        .expect("nonblocking inbox listener");
    let store = Arc::new(Mutex::new(InboxStore::new(max_body, max_queue)));
    let shutdown = Arc::new(AtomicBool::new(false));
    let shutdown_thread = Arc::clone(&shutdown);
    let store_thread = Arc::clone(&store);
    let handle = thread::spawn(move || {
        while !shutdown_thread.load(Ordering::SeqCst) {
            match listener.accept() {
                Ok((stream, _)) => {
                    let store_conn = Arc::clone(&store_thread);
                    thread::spawn(move || handle_conn(stream, store_conn));
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(10));
                }
                Err(_) => break,
            }
        }
    });
    let server = InboxTestServer {
        base_url: format!("http://{}", addr),
        store,
        shutdown,
        handle: Some(handle),
    };
    wait_until_ready(server.base_url());
    server
}

fn wait_until_ready(base_url: &str) {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_millis(150))
        .build()
        .expect("build readiness client");
    let deadline = Instant::now() + Duration::from_secs(5);
    let mut attempt = 0u64;
    let mut consecutive_ok = 0u8;
    const READY_STREAK: u8 = 3;
    while Instant::now() < deadline {
        let probe_channel = format!("qsc_ready_probe_{}_{}", std::process::id(), attempt);
        let push_url = format!("{}/v1/push/{}", base_url, probe_channel);
        let mut ok = false;
        if let Ok(resp) = client.post(&push_url).body(vec![0x51]).send() {
            if resp.status().as_u16() == 200 {
                let pull_url = format!("{}/v1/pull/{}?max=1", base_url, probe_channel);
                if let Ok(resp) = client.get(&pull_url).send() {
                    if resp.status().as_u16() == 200 {
                        ok = true;
                    }
                }
            }
        }
        if ok {
            consecutive_ok = consecutive_ok.saturating_add(1);
            if consecutive_ok >= READY_STREAK {
                return;
            }
        } else {
            consecutive_ok = 0;
        }
        attempt = attempt.saturating_add(1);
        thread::sleep(Duration::from_millis(20));
    }
    panic!("inbox test server readiness probe timed out");
}

fn handle_conn(mut stream: TcpStream, store: Arc<Mutex<InboxStore>>) {
    let _ = stream.set_read_timeout(Some(Duration::from_millis(200)));
    let deadline = Instant::now() + Duration::from_secs(2);
    let mut buf = Vec::with_capacity(1024);

    let header_end = match read_until_header_end(&mut stream, &mut buf, deadline) {
        Some(pos) => pos,
        None => {
            let _ = write_response(&mut stream, 400, "bad request");
            return;
        }
    };
    let header_bytes = &buf[..header_end];
    let header_str = String::from_utf8_lossy(header_bytes);
    let mut lines = header_str.split("\r\n");
    let request_line = match lines.next() {
        Some(line) => line,
        None => {
            let _ = write_response(&mut stream, 400, "bad request");
            return;
        }
    };
    let mut req_parts = request_line.split_whitespace();
    let method = req_parts.next().unwrap_or("");
    let target = req_parts.next().unwrap_or("");
    let mut content_len = 0usize;
    let mut seen_content_len = false;
    let mut has_chunked_transfer_encoding = false;
    for line in lines {
        let Some((name, value)) = line.split_once(':') else {
            continue;
        };
        if name.trim().eq_ignore_ascii_case("transfer-encoding") {
            has_chunked_transfer_encoding |= value
                .split(',')
                .any(|v| v.trim().eq_ignore_ascii_case("chunked"));
            continue;
        }
        if !name.trim().eq_ignore_ascii_case("content-length") {
            continue;
        }
        let Ok(n) = value.trim().parse::<usize>() else {
            let _ = write_response(&mut stream, 400, "bad request");
            return;
        };
        if seen_content_len && n != content_len {
            let _ = write_response(&mut stream, 400, "bad request");
            return;
        }
        seen_content_len = true;
        content_len = n;
    }
    let initial_body = &buf[(header_end + 4)..];
    let body = match read_body_exact(&mut stream, initial_body, content_len, deadline) {
        Some(b) => b,
        None => {
            let _ = write_response(&mut stream, 400, "bad request");
            return;
        }
    };

    if method == "POST" && target.starts_with("/v1/push/") {
        if has_chunked_transfer_encoding {
            let _ = write_response(&mut stream, 400, "ERR_UNSUPPORTED_TRANSFER_ENCODING");
            return;
        }
        let channel = &target["/v1/push/".len()..];
        if !channel_label_ok(channel) {
            let _ = write_response(&mut stream, 400, "ERR_BAD_CHANNEL");
            return;
        }
        let mut store = store.lock().unwrap();
        if body.len() > store.max_body {
            let _ = write_response(&mut stream, 413, "ERR_TOO_LARGE");
            return;
        }
        let queue_len = store.queues.get(channel).map(|q| q.len()).unwrap_or(0);
        if queue_len >= store.max_queue {
            let _ = write_response(&mut stream, 429, "ERR_QUEUE_FULL");
            return;
        }
        let id = store.next_id.to_string();
        store.next_id += 1;
        let queue = store.queues.entry(channel.to_string()).or_default();
        queue.push_back((id.clone(), body));
        let body = format!("{{\"id\":\"{}\"}}", id);
        let _ = write_response_json(&mut stream, 200, &body);
        return;
    }

    if method == "GET" && target.starts_with("/v1/pull/") {
        let mut path = &target["/v1/pull/".len()..];
        let mut max_n = 1usize;
        if let Some((base, query)) = path.split_once('?') {
            path = base;
            for part in query.split('&') {
                if let Some(v) = part.strip_prefix("max=") {
                    if let Ok(n) = v.parse::<usize>() {
                        max_n = n;
                    }
                }
            }
        }
        if !channel_label_ok(path) {
            let _ = write_response(&mut stream, 400, "ERR_BAD_CHANNEL");
            return;
        }
        let mut store = store.lock().unwrap();
        let queue = store.queues.entry(path.to_string()).or_default();
        if queue.is_empty() {
            let _ = write_response_empty(&mut stream, 204);
            return;
        }
        let mut items = Vec::new();
        let mut count = 0usize;
        while count < max_n {
            if let Some((id, data)) = queue.pop_front() {
                items.push(InboxPullItem { id, data });
                count += 1;
            } else {
                break;
            }
        }
        let resp = InboxPullResp { items };
        let body = serde_json::to_string(&resp).unwrap();
        let _ = write_response_json(&mut stream, 200, &body);
        return;
    }

    let _ = write_response(&mut stream, 404, "not found");
}

fn read_until_header_end(
    stream: &mut TcpStream,
    buf: &mut Vec<u8>,
    deadline: Instant,
) -> Option<usize> {
    let mut tmp = [0u8; 1024];
    while Instant::now() < deadline {
        if let Some(pos) = find_header_end(buf) {
            return Some(pos);
        }
        match stream.read(&mut tmp) {
            Ok(0) => return find_header_end(buf),
            Ok(n) => buf.extend_from_slice(&tmp[..n]),
            Err(e)
                if matches!(
                    e.kind(),
                    std::io::ErrorKind::Interrupted
                        | std::io::ErrorKind::WouldBlock
                        | std::io::ErrorKind::TimedOut
                ) =>
            {
                continue
            }
            Err(_) => return None,
        }
    }
    find_header_end(buf)
}

fn read_body_exact(
    stream: &mut TcpStream,
    initial: &[u8],
    content_len: usize,
    deadline: Instant,
) -> Option<Vec<u8>> {
    if content_len == 0 {
        return Some(Vec::new());
    }
    let mut body = Vec::with_capacity(content_len);
    body.extend_from_slice(&initial[..initial.len().min(content_len)]);
    while body.len() < content_len && Instant::now() < deadline {
        let mut tmp = [0u8; 1024];
        match stream.read(&mut tmp) {
            Ok(0) => break,
            Ok(n) => {
                let remaining = content_len - body.len();
                body.extend_from_slice(&tmp[..n.min(remaining)]);
            }
            Err(e)
                if matches!(
                    e.kind(),
                    std::io::ErrorKind::Interrupted
                        | std::io::ErrorKind::WouldBlock
                        | std::io::ErrorKind::TimedOut
                ) =>
            {
                continue
            }
            Err(_) => return None,
        }
    }
    if body.len() == content_len {
        Some(body)
    } else {
        None
    }
}

fn channel_label_ok(label: &str) -> bool {
    !label.is_empty()
        && label
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

fn find_header_end(buf: &[u8]) -> Option<usize> {
    buf.windows(4).position(|w| w == b"\r\n\r\n")
}

fn write_response(stream: &mut TcpStream, code: u16, body: &str) -> std::io::Result<()> {
    let body_bytes = body.as_bytes();
    let status = status_line(code);
    let header = format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\nContent-Type: text/plain\r\nConnection: close\r\n\r\n",
        status,
        body_bytes.len()
    );
    stream.write_all(header.as_bytes())?;
    stream.write_all(body_bytes)?;
    stream.flush()?;
    Ok(())
}

fn write_response_json(stream: &mut TcpStream, code: u16, body: &str) -> std::io::Result<()> {
    let body_bytes = body.as_bytes();
    let status = status_line(code);
    let header = format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\nContent-Type: application/json\r\nConnection: close\r\n\r\n",
        status,
        body_bytes.len()
    );
    stream.write_all(header.as_bytes())?;
    stream.write_all(body_bytes)?;
    stream.flush()?;
    Ok(())
}

fn write_response_empty(stream: &mut TcpStream, code: u16) -> std::io::Result<()> {
    let status = status_line(code);
    let header = format!(
        "HTTP/1.1 {}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        status
    );
    stream.write_all(header.as_bytes())?;
    stream.flush()?;
    Ok(())
}

fn status_line(code: u16) -> &'static str {
    match code {
        200 => "200 OK",
        204 => "204 No Content",
        400 => "400 Bad Request",
        401 => "401 Unauthorized",
        403 => "403 Forbidden",
        404 => "404 Not Found",
        413 => "413 Payload Too Large",
        429 => "429 Too Many Requests",
        _ => "500 Internal Server Error",
    }
}
