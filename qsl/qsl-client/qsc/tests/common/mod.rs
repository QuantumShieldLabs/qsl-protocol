use assert_cmd::Command;
use serde::Serialize;
use std::collections::{HashMap, VecDeque};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread;
use std::time::Duration;

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
    InboxTestServer {
        base_url: format!("http://{}", addr),
        store,
        shutdown,
        handle: Some(handle),
    }
}

fn handle_conn(mut stream: TcpStream, store: Arc<Mutex<InboxStore>>) {
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
    for line in lines {
        let lower = line.to_ascii_lowercase();
        if let Some(v) = lower.strip_prefix("content-length:") {
            if let Ok(n) = v.trim().parse::<usize>() {
                content_len = n;
            }
        }
    }
    let mut body = Vec::new();
    if content_len > 0 {
        let mut remaining = content_len;
        let mut body_buf = Vec::new();
        body_buf.extend_from_slice(&buf[(header_end + 4)..]);
        while body_buf.len() < content_len {
            let mut read_buf = vec![0u8; 1024];
            match stream.read(&mut read_buf) {
                Ok(0) => break,
                Ok(n) => body_buf.extend_from_slice(&read_buf[..n]),
                Err(_) => break,
            }
        }
        if body_buf.len() >= content_len {
            body.extend_from_slice(&body_buf[..content_len]);
        } else {
            body.extend_from_slice(&body_buf);
        }
        remaining = remaining.saturating_sub(body.len());
        if remaining > 0 {
            let _ = write_response(&mut stream, 400, "bad request");
            return;
        }
    }

    if method == "POST" && target.starts_with("/v1/push/") {
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
