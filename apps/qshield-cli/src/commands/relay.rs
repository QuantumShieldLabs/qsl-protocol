use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::Read;
use std::net::{SocketAddr, ToSocketAddrs};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use hex::encode as hex_encode;
use serde_json::json;
use sha2::{Digest, Sha256};
use tiny_http::{Header, Method, Response, Server};

use crate::config::{self, Config};
use crate::relay_client::{post_json, GenericOk, PollRequest, PollResponse, SendRequest};

const MAX_BODY_BYTES: usize = 64 * 1024;
const MAX_QUEUE_PER_RECIPIENT: usize = 256;
const MAX_TOTAL_QUEUE: usize = 10_000;
const MAX_QUEUE_PER_TOKEN: usize = 512;
const MAX_RELAY_ID_LEN: usize = 64;
const REGISTER_LIMIT: u32 = 50;
const POLL_LIMIT: u32 = 200;
const RETRY_AFTER_MS: u64 = 1000;

pub fn serve(listen: &str, allow_public: bool, unsafe_public: bool) -> Result<(), String> {
    let addr = resolve_addr(listen)?;
    if !addr.ip().is_loopback() && !allow_public {
        return Err("relay serve is local-only; use 127.0.0.1:<port>".to_string());
    }
    if !addr.ip().is_loopback() && !unsafe_public {
        return Err(
            "non-loopback bind requires --allow-public and --i-understand-this-is-unsafe"
                .to_string(),
        );
    }

    let server = Server::http(addr).map_err(|e| format!("start relay: {e}"))?;
    let token = load_or_generate_token()?;
    println!("qshield relay (demo) listening on http://{addr}");
    println!("DEMO ONLY: relay auth token required for /register /send /poll /bundle /consume /establish_record");
    println!("DEMO ONLY: token={token}");
    if !addr.ip().is_loopback() {
        eprintln!("warning: relay bound to non-loopback address (demo-only, unsafe)");
    }

    let state = Arc::new(Mutex::new(RelayState::default()));

    for request in server.incoming_requests() {
        let url = request.url().to_string();
        let method = request.method().clone();
        let response = handle_request(&state, &token, method, &url, request);
        let _ = response.respond();
    }

    Ok(())
}

pub fn send(store_path: &std::path::Path, to: &str, from: &str, msg: &str) -> Result<(), String> {
    let cfg_path = store_path.join(config::CONFIG_FILE_NAME);
    let cfg: Config = config::read_config(&cfg_path).map_err(|_| {
        format!(
            "config missing or invalid: {} (run: qshield init --store <path>)",
            cfg_path.display()
        )
    })?;

    let relay_token = config::resolve_relay_token(&cfg)?;
    let req = SendRequest {
        to: to.to_string(),
        from: from.to_string(),
        msg: msg.to_string(),
        pad_len: None,
        bucket: None,
    };
    let resp: GenericOk = post_json(&cfg.relay_url, "/send", &req, &relay_token)?;
    if !resp.ok {
        return Err("relay send failed".to_string());
    }
    println!("queued 1 message to {to}");
    Ok(())
}

pub fn poll(store_path: &std::path::Path, id: &str, max: u32) -> Result<(), String> {
    let cfg_path = store_path.join(config::CONFIG_FILE_NAME);
    let cfg: Config = config::read_config(&cfg_path).map_err(|_| {
        format!(
            "config missing or invalid: {} (run: qshield init --store <path>)",
            cfg_path.display()
        )
    })?;

    let relay_token = config::resolve_relay_token(&cfg)?;
    let req = PollRequest {
        id: id.to_string(),
        max,
    };
    let resp: PollResponse = post_json(&cfg.relay_url, "/poll", &req, &relay_token)?;
    if !resp.ok {
        return Err("relay poll failed".to_string());
    }
    let msgs = resp.msgs.unwrap_or_default();
    println!("polled {} message(s)", msgs.len());
    for m in msgs {
        println!("from {}: {}", m.from, m.msg);
    }
    Ok(())
}

fn resolve_addr(listen: &str) -> Result<SocketAddr, String> {
    let mut addrs = listen
        .to_socket_addrs()
        .map_err(|e| format!("invalid listen addr: {e}"))?;
    addrs
        .next()
        .ok_or_else(|| "unable to resolve listen address".to_string())
}

#[derive(Default)]
struct RelayState {
    bundles: HashMap<String, serde_json::Value>,
    establish_fingerprints: HashSet<String>,
    rate: HashMap<String, RateCounts>,
    queues: HashMap<String, VecDeque<QueuedMsg>>,
    token_queued: HashMap<String, usize>,
    total_msgs: usize,
}

#[derive(Default)]
struct RateCounts {
    register: u32,
    poll: u32,
}

enum RateKind {
    Register,
    Poll,
}
#[derive(Clone, Debug)]
struct QueuedMsg {
    from: String,
    msg: String,
    pad_len: u32,
    bucket: Option<u32>,
    token: String,
}

struct ResponseWrapper {
    request: tiny_http::Request,
    response: Response<std::io::Cursor<Vec<u8>>>,
}

impl ResponseWrapper {
    fn respond(self) -> Result<(), String> {
        self.request
            .respond(self.response)
            .map_err(|e| format!("respond: {e}"))
    }
}

fn handle_request(
    state: &Arc<Mutex<RelayState>>,
    token: &str,
    method: Method,
    url: &str,
    mut request: tiny_http::Request,
) -> ResponseWrapper {
    let json_response = |request: tiny_http::Request, status: u16, body: serde_json::Value| {
        let data = serde_json::to_vec(&body).unwrap_or_else(|_| b"{\"ok\":false}".to_vec());
        let mut resp = Response::from_data(data).with_status_code(status);
        let _ = Header::from_bytes("Content-Type", "application/json").map(|h| resp.add_header(h));
        ResponseWrapper {
            request,
            response: resp,
        }
    };

    if method == Method::Get && url == "/health" {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        return json_response(
            request,
            200,
            json!({ "ok": true, "name": "qshield-relay", "mode": "demo", "ts": ts }),
        );
    }

    let token_value = match auth_token(&request, token) {
        Some(value) => value,
        None => {
            return json_response(
                request,
                401,
                json!({ "ok": false, "error": "missing or invalid relay token" }),
            )
        }
    };

    if method == Method::Get && url.starts_with("/bundle/") {
        let id = url.trim_start_matches("/bundle/");
        let state = state.lock().unwrap();
        let bundle = state.bundles.get(id).cloned();
        let body = json!({ "ok": bundle.is_some(), "bundle": bundle });
        return json_response(request, if bundle.is_some() { 200 } else { 404 }, body);
    }

    if method == Method::Post && url == "/consume" {
        let body = match read_json_body(&mut request, MAX_BODY_BYTES) {
            Ok(v) => v,
            Err(e) => {
                let status = if e == "body too large" { 413 } else { 400 };
                return json_response(request, status, json!({ "ok": false, "error": e }));
            }
        };
        let id = body
            .get("id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        if id.is_none() {
            return json_response(request, 400, json!({ "ok": false, "error": "missing id" }));
        }
        let mut state = state.lock().unwrap();
        let removed = state.bundles.remove(id.as_ref().unwrap()).is_some();
        if !removed {
            return json_response(
                request,
                404,
                json!({ "ok": false, "error": "bundle missing" }),
            );
        }
        return json_response(request, 200, json!({ "ok": true }));
    }

    if method == Method::Post && url == "/establish_record" {
        let body = match read_json_body(&mut request, MAX_BODY_BYTES) {
            Ok(v) => v,
            Err(e) => {
                let status = if e == "body too large" { 413 } else { 400 };
                return json_response(request, status, json!({ "ok": false, "error": e }));
            }
        };
        let peer_id = body.get("peer_id").and_then(|v| v.as_str());
        let bundle_id = body.get("bundle_id").and_then(|v| v.as_str());
        let session_id_hex = body.get("session_id_hex").and_then(|v| v.as_str());
        let dh_init = body.get("dh_init").and_then(|v| v.as_str());
        let pq_init_ss = body.get("pq_init_ss").and_then(|v| v.as_str());
        if peer_id.is_none()
            || bundle_id.is_none()
            || session_id_hex.is_none()
            || dh_init.is_none()
            || pq_init_ss.is_none()
        {
            return json_response(
                request,
                400,
                json!({ "ok": false, "error": "missing field" }),
            );
        }

        let mut h = Sha256::new();
        h.update(peer_id.unwrap().as_bytes());
        h.update([0u8]);
        h.update(bundle_id.unwrap().as_bytes());
        h.update([0u8]);
        h.update(session_id_hex.unwrap().as_bytes());
        h.update([0u8]);
        h.update(dh_init.unwrap().as_bytes());
        h.update([0u8]);
        h.update(pq_init_ss.unwrap().as_bytes());
        let fp = hex::encode(h.finalize());

        let mut state = state.lock().unwrap();
        if state.establish_fingerprints.contains(&fp) {
            return json_response(
                request,
                409,
                json!({ "ok": false, "error": "establish replay" }),
            );
        }
        state.establish_fingerprints.insert(fp);
        return json_response(request, 200, json!({ "ok": true }));
    }

    if method == Method::Post && url == "/register" {
        let body = match read_json_body(&mut request, MAX_BODY_BYTES) {
            Ok(v) => v,
            Err(e) => {
                let status = if e == "body too large" { 413 } else { 400 };
                return json_response(request, status, json!({ "ok": false, "error": e }));
            }
        };
        let id = body
            .get("id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let bundle = body.get("bundle").cloned();
        if id.is_none() || bundle.is_none() {
            return json_response(
                request,
                400,
                json!({ "ok": false, "error": "missing id or bundle" }),
            );
        }
        let id = id.unwrap();
        if !valid_relay_id(&id) {
            return json_response(
                request,
                400,
                json!({ "ok": false, "error": "invalid id format" }),
            );
        }
        let mut state = state.lock().unwrap();
        if !check_rate_limit(&mut state, &token_value, RateKind::Register) {
            return rate_limit_response(request, json_response);
        }
        if state.bundles.contains_key(&id) {
            return json_response(
                request,
                409,
                json!({ "ok": false, "error": "id already registered" }),
            );
        }
        state.bundles.insert(id, bundle.unwrap());
        return json_response(request, 200, json!({ "ok": true }));
    }

    if method == Method::Post && url == "/send" {
        let body = match read_json_body(&mut request, MAX_BODY_BYTES) {
            Ok(v) => v,
            Err(e) => {
                let status = if e == "body too large" { 413 } else { 400 };
                return json_response(request, status, json!({ "ok": false, "error": e }));
            }
        };
        let to = body
            .get("to")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let from = body
            .get("from")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let msg = body
            .get("msg")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let pad_len = body
            .get("pad_len")
            .and_then(|v| v.as_u64())
            .map(|n| n as u32)
            .unwrap_or(0);
        let bucket = body
            .get("bucket")
            .and_then(|v| v.as_u64())
            .map(|n| n as u32);
        if to.is_none() || from.is_none() || msg.is_none() {
            return json_response(
                request,
                400,
                json!({ "ok": false, "error": "missing to/from/msg" }),
            );
        }
        let mut state = state.lock().unwrap();
        if state.total_msgs >= MAX_TOTAL_QUEUE {
            return json_response(request, 429, json!({ "ok": false, "error": "queue full" }));
        }
        if !token_quota_available(&state, &token_value) {
            return quota_response(request, json_response);
        }
        let entry = state.queues.entry(to.unwrap()).or_default();
        if entry.len() >= MAX_QUEUE_PER_RECIPIENT {
            return json_response(
                request,
                429,
                json!({ "ok": false, "error": "recipient queue full" }),
            );
        }
        entry.push_back(QueuedMsg {
            from: from.unwrap(),
            msg: msg.unwrap(),
            pad_len,
            bucket,
            token: token_value.clone(),
        });
        *state.token_queued.entry(token_value).or_insert(0) += 1;
        state.total_msgs += 1;
        return json_response(request, 200, json!({ "ok": true, "queued": 1 }));
    }

    if method == Method::Post && url == "/poll" {
        let body = match read_json_body(&mut request, MAX_BODY_BYTES) {
            Ok(v) => v,
            Err(e) => {
                let status = if e == "body too large" { 413 } else { 400 };
                return json_response(request, status, json!({ "ok": false, "error": e }));
            }
        };
        let id = body
            .get("id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let max = body.get("max").and_then(|v| v.as_u64()).unwrap_or(1) as usize;
        if id.is_none() {
            return json_response(request, 400, json!({ "ok": false, "error": "missing id" }));
        }
        let mut state = state.lock().unwrap();
        if !check_rate_limit(&mut state, &token_value, RateKind::Poll) {
            return rate_limit_response(request, json_response);
        }
        let (msgs, removed, removed_tokens) = {
            let queue = state.queues.entry(id.unwrap()).or_default();
            let mut msgs = Vec::new();
            let mut removed = 0usize;
            let mut removed_tokens: Vec<String> = Vec::new();
            for _ in 0..max {
                if let Some(entry) = queue.pop_front() {
                    removed_tokens.push(entry.token);
                    msgs.push(json!({
                        "from": entry.from,
                        "msg": entry.msg,
                        "pad_len": entry.pad_len,
                        "bucket": entry.bucket
                    }));
                    removed += 1;
                } else {
                    break;
                }
            }
            (msgs, removed, removed_tokens)
        };
        for token_key in removed_tokens {
            if let Some(count) = state.token_queued.get_mut(&token_key) {
                *count = count.saturating_sub(1);
                if *count == 0 {
                    state.token_queued.remove(&token_key);
                }
            }
        }
        if removed > 0 {
            state.total_msgs = state.total_msgs.saturating_sub(removed);
        }
        return json_response(request, 200, json!({ "ok": true, "msgs": msgs }));
    }

    let resp = Response::from_string("not found").with_status_code(404);
    ResponseWrapper {
        request,
        response: resp,
    }
}

fn read_json_body(
    request: &mut tiny_http::Request,
    max_bytes: usize,
) -> Result<serde_json::Value, String> {
    let mut buf = Vec::new();
    request
        .as_reader()
        .take((max_bytes + 1) as u64)
        .read_to_end(&mut buf)
        .map_err(|e| format!("read body: {e}"))?;
    if buf.len() > max_bytes {
        return Err("body too large".to_string());
    }
    serde_json::from_slice(&buf).map_err(|e| format!("parse json: {e}"))
}

fn auth_token(request: &tiny_http::Request, token: &str) -> Option<String> {
    let expected = format!("Bearer {token}");
    for header in request.headers() {
        if header.field.equiv("Authorization") {
            if let Ok(value) = std::str::from_utf8(header.value.as_ref()) {
                if value.trim() == expected {
                    return Some(token.to_string());
                }
            }
        }
    }
    None
}

fn valid_relay_id(id: &str) -> bool {
    if id.is_empty() || id.len() > MAX_RELAY_ID_LEN {
        return false;
    }
    id.bytes()
        .all(|b| matches!(b, b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_'))
}

fn check_rate_limit(state: &mut RelayState, token: &str, kind: RateKind) -> bool {
    let entry = state.rate.entry(token.to_string()).or_default();
    match kind {
        RateKind::Register => {
            entry.register = entry.register.saturating_add(1);
            entry.register <= REGISTER_LIMIT
        }
        RateKind::Poll => {
            entry.poll = entry.poll.saturating_add(1);
            entry.poll <= POLL_LIMIT
        }
    }
}

fn rate_limit_response(
    request: tiny_http::Request,
    json_response: impl Fn(tiny_http::Request, u16, serde_json::Value) -> ResponseWrapper,
) -> ResponseWrapper {
    json_response(
        request,
        429,
        json!({ "ok": false, "error": "rate limited", "retry_after_ms": RETRY_AFTER_MS }),
    )
}

fn token_quota_available(state: &RelayState, token: &str) -> bool {
    state.token_queued.get(token).copied().unwrap_or(0) < MAX_QUEUE_PER_TOKEN
}

fn quota_response(
    request: tiny_http::Request,
    json_response: impl Fn(tiny_http::Request, u16, serde_json::Value) -> ResponseWrapper,
) -> ResponseWrapper {
    json_response(
        request,
        429,
        json!({ "ok": false, "error": "token quota exceeded", "retry_after_ms": RETRY_AFTER_MS }),
    )
}

fn load_or_generate_token() -> Result<String, String> {
    if let Ok(token) = std::env::var("QSHIELD_RELAY_TOKEN") {
        if !token.trim().is_empty() {
            return Ok(token);
        }
    }
    let mut buf = [0u8; 32];
    let mut f = File::open("/dev/urandom").map_err(|e| format!("read /dev/urandom: {e}"))?;
    f.read_exact(&mut buf)
        .map_err(|e| format!("read /dev/urandom: {e}"))?;
    Ok(hex_encode(buf))
}
