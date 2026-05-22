use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::Read;
use std::net::{SocketAddr, ToSocketAddrs};
use std::process::Command;
use std::sync::{Arc, Mutex, MutexGuard};
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
const RELAY_LOCK_POISON: &str = "relay state lock poisoned";
const DEMO_BATCH_POLICY: &str = "qshield_demo_batching_v1";
const DEMO_BATCH_MAX_SIZE: usize = 4;
const DEMO_BATCH_MAX_WINDOW_MS: u64 = 750;
const DEMO_COVER_POLICY: &str = "qshield_demo_cover_traffic_v1";
const DEMO_COVER_MAX_PAYLOAD_BYTES: usize = 8192;
const DEMO_COVER_MAX_ITEMS_PER_MINUTE: usize = 4;
const DEMO_COVER_MAX_ITEMS_PER_HOUR: usize = 32;
const DEMO_COVER_MAX_ITEMS_PER_RUN: usize = 64;
const DEMO_COVER_MAX_PAYLOAD_BYTES_PER_RUN: usize = 512 * 1024;
const DEMO_COVER_MAX_REQUEST_BYTES_PER_RUN: usize = 1024 * 1024;
const DEMO_COVER_MAX_QUEUED_GLOBAL: usize = 16;
const DEMO_COVER_MAX_QUEUED_PER_ROUTE: usize = 4;
const DEMO_COVER_MAX_RETAINED_ARTIFACTS: usize = 4;
const DEMO_COVER_MAX_RETAINED_ARTIFACT_BYTES: usize = 1024 * 1024;
const DEMO_COVER_MIN_FREE_DISK_BYTES: u64 = 10 * 1024 * 1024 * 1024;
const DEMO_COVER_ROUTE_TAG_BYTES: usize = 6;
const DEMO_COVER_ARTIFACT_TAG_BYTES: usize = 8;
const DEMO_COVER_DEFAULT_FROM: &str = "qshield-demo-cover";

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
    println!("DEMO ONLY: relay auth token required for /register /send /poll /poll-candidate /ack /bundle /consume /establish_record");
    println!("DEMO ONLY: relay auth token is configured but not printed");
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
    cover: DemoCoverLedger,
    total_msgs: usize,
    next_msg_seq: u64,
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
    ack_id: String,
    from: String,
    msg: String,
    pad_len: u32,
    bucket: Option<u32>,
    token: String,
    cover: bool,
    cover_mode: Option<String>,
    cover_payload_len: Option<u32>,
}

#[derive(Clone, Debug)]
struct BatchSendMember {
    to: String,
    from: String,
    msg: String,
    pad_len: u32,
    bucket: Option<u32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DemoCoverMode {
    SyntheticLocal,
    ActiveSession,
    BatchFill,
}

impl DemoCoverMode {
    fn parse(value: &str) -> Option<Self> {
        match value {
            "synthetic_local" => Some(Self::SyntheticLocal),
            "active_session" => Some(Self::ActiveSession),
            "batch_fill" => Some(Self::BatchFill),
            _ => None,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::SyntheticLocal => "synthetic_local",
            Self::ActiveSession => "active_session",
            Self::BatchFill => "batch_fill",
        }
    }
}

#[derive(Default)]
struct DemoCoverLedger {
    generated_ms: Vec<u64>,
    run_items: usize,
    run_payload_bytes: usize,
    run_request_bytes: usize,
    retained_artifacts: VecDeque<DemoCoverArtifact>,
    retained_artifact_bytes: usize,
    purged_artifacts: usize,
    purged_items: usize,
    rejected_generations: usize,
}

struct DemoCoverArtifact {
    mode: String,
    payload_len: usize,
    route_tag: String,
    artifact_tag: String,
    generated_ms: u64,
    retained_bytes: usize,
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
        let state = match lock_relay_state(state) {
            Ok(guard) => guard,
            Err(err) => {
                return json_response(request, 500, json!({ "ok": false, "error": err }));
            }
        };
        let bundle = state.bundles.get(id).cloned();
        let body = json!({ "ok": bundle.is_some(), "bundle": bundle });
        return json_response(request, if bundle.is_some() { 200 } else { 404 }, body);
    }

    if method == Method::Post && url == "/consume" {
        let body = match read_json_body(&mut request, MAX_BODY_BYTES) {
            Ok(v) => v,
            Err(e) => {
                let status = json_body_error_status(&e);
                return json_response(request, status, json!({ "ok": false, "error": e }));
            }
        };
        let Some(id) = body
            .get("id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
        else {
            return json_response(request, 400, json!({ "ok": false, "error": "missing id" }));
        };
        let mut state = match lock_relay_state(state) {
            Ok(guard) => guard,
            Err(err) => {
                return json_response(request, 500, json!({ "ok": false, "error": err }));
            }
        };
        let removed = state.bundles.remove(&id).is_some();
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
                let status = json_body_error_status(&e);
                return json_response(request, status, json!({ "ok": false, "error": e }));
            }
        };
        let peer_id = body.get("peer_id").and_then(|v| v.as_str());
        let bundle_id = body.get("bundle_id").and_then(|v| v.as_str());
        let session_id_hex = body.get("session_id_hex").and_then(|v| v.as_str());
        let dh_init = body.get("dh_init").and_then(|v| v.as_str());
        let pq_init_ss = body.get("pq_init_ss").and_then(|v| v.as_str());
        let (Some(peer_id), Some(bundle_id), Some(session_id_hex), Some(dh_init), Some(pq_init_ss)) =
            (peer_id, bundle_id, session_id_hex, dh_init, pq_init_ss)
        else {
            return json_response(
                request,
                400,
                json!({ "ok": false, "error": "missing field" }),
            );
        };

        let mut h = Sha256::new();
        h.update(peer_id.as_bytes());
        h.update([0u8]);
        h.update(bundle_id.as_bytes());
        h.update([0u8]);
        h.update(session_id_hex.as_bytes());
        h.update([0u8]);
        h.update(dh_init.as_bytes());
        h.update([0u8]);
        h.update(pq_init_ss.as_bytes());
        let fp = hex::encode(h.finalize());

        let mut state = match lock_relay_state(state) {
            Ok(guard) => guard,
            Err(err) => {
                return json_response(request, 500, json!({ "ok": false, "error": err }));
            }
        };
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
                let status = json_body_error_status(&e);
                return json_response(request, status, json!({ "ok": false, "error": e }));
            }
        };
        let Some(id) = body
            .get("id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
        else {
            return json_response(
                request,
                400,
                json!({ "ok": false, "error": "missing id or bundle" }),
            );
        };
        let bundle = body.get("bundle").cloned();
        let Some(bundle) = bundle else {
            return json_response(
                request,
                400,
                json!({ "ok": false, "error": "missing id or bundle" }),
            );
        };
        if !valid_relay_id(&id) {
            return json_response(
                request,
                400,
                json!({ "ok": false, "error": "invalid id format" }),
            );
        }
        let mut state = match lock_relay_state(state) {
            Ok(guard) => guard,
            Err(err) => {
                return json_response(request, 500, json!({ "ok": false, "error": err }));
            }
        };
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
        state.bundles.insert(id, bundle);
        return json_response(request, 200, json!({ "ok": true }));
    }

    if method == Method::Post && url == "/cover-traffic" {
        if !demo_cover_enabled() {
            return json_response(
                request,
                404,
                json!({ "ok": false, "error": "cover traffic disabled" }),
            );
        }
        let body = match read_json_body(&mut request, MAX_BODY_BYTES) {
            Ok(v) => v,
            Err(e) => {
                let status = json_body_error_status(&e);
                return json_response(request, status, json!({ "ok": false, "error": e }));
            }
        };
        let cover_request = match parse_demo_cover_request(&body) {
            Ok(request) => request,
            Err(error) => {
                return json_response(request, 400, json!({ "ok": false, "error": error }));
            }
        };
        let now_ms = match demo_cover_now_ms(&body) {
            Ok(now_ms) => now_ms,
            Err(error) => {
                return json_response(request, 400, json!({ "ok": false, "error": error }));
            }
        };
        let free_disk_bytes = match demo_cover_free_disk_bytes() {
            Ok(bytes) => bytes,
            Err(error) => {
                return json_response(request, 507, json!({ "ok": false, "error": error }));
            }
        };
        if free_disk_bytes < DEMO_COVER_MIN_FREE_DISK_BYTES {
            return json_response(
                request,
                507,
                json!({ "ok": false, "error": "cover traffic disk floor not met" }),
            );
        }

        let request_bytes = body.to_string().len();
        let mut state = match lock_relay_state(state) {
            Ok(guard) => guard,
            Err(err) => {
                return json_response(request, 500, json!({ "ok": false, "error": err }));
            }
        };
        if let Err(error) =
            validate_demo_cover_caps(&state, &cover_request, now_ms, request_bytes, &token_value)
        {
            state.cover.rejected_generations = state.cover.rejected_generations.saturating_add(1);
            return json_response(request, 429, json!({ "ok": false, "error": error }));
        }
        purge_demo_cover_artifact_capacity(&mut state.cover, cover_request.items);

        let mut modes = Vec::with_capacity(cover_request.items);
        for idx in 0..cover_request.items {
            let seq = state.next_msg_seq;
            state.next_msg_seq = state.next_msg_seq.saturating_add(1);
            let ack_id = make_ack_id(&token_value, &cover_request.to, seq);
            let msg = make_demo_cover_payload(
                cover_request.mode,
                &cover_request.to,
                &cover_request.from,
                seq,
                cover_request.payload_len,
                now_ms,
            );
            let entry = state.queues.entry(cover_request.to.clone()).or_default();
            entry.push_back(QueuedMsg {
                ack_id,
                from: cover_request.from.clone(),
                msg,
                pad_len: 0,
                bucket: None,
                token: token_value.clone(),
                cover: true,
                cover_mode: Some(cover_request.mode.as_str().to_string()),
                cover_payload_len: Some(cover_request.payload_len as u32),
            });
            state.cover.generated_ms.push(now_ms);
            state.cover.run_items = state.cover.run_items.saturating_add(1);
            state.cover.run_payload_bytes = state
                .cover
                .run_payload_bytes
                .saturating_add(cover_request.payload_len);
            record_demo_cover_artifact(&mut state.cover, &cover_request, seq, idx, now_ms);
            modes.push(cover_request.mode.as_str());
        }
        state.cover.run_request_bytes = state.cover.run_request_bytes.saturating_add(request_bytes);
        *state.token_queued.entry(token_value).or_insert(0) += cover_request.items;
        state.total_msgs += cover_request.items;
        return json_response(
            request,
            200,
            json!({
                "ok": true,
                "queued": cover_request.items,
                "policy": DEMO_COVER_POLICY,
                "test_mode": demo_cover_test_mode(),
                "modes": modes,
                "max_payload_bytes": DEMO_COVER_MAX_PAYLOAD_BYTES,
                "max_items_per_minute": DEMO_COVER_MAX_ITEMS_PER_MINUTE,
                "max_items_per_hour": DEMO_COVER_MAX_ITEMS_PER_HOUR,
                "max_items_per_run": DEMO_COVER_MAX_ITEMS_PER_RUN,
                "max_queued_global": DEMO_COVER_MAX_QUEUED_GLOBAL,
                "max_queued_per_route": DEMO_COVER_MAX_QUEUED_PER_ROUTE,
                "retained_artifacts": state.cover.retained_artifacts.len(),
                "purged_artifacts": state.cover.purged_artifacts
            }),
        );
    }

    if method == Method::Post && url == "/cover-traffic/status" {
        if !demo_cover_enabled() {
            return json_response(
                request,
                404,
                json!({ "ok": false, "error": "cover traffic disabled" }),
            );
        }
        let state = match lock_relay_state(state) {
            Ok(guard) => guard,
            Err(err) => {
                return json_response(request, 500, json!({ "ok": false, "error": err }));
            }
        };
        let (queued_global, queued_by_route) = demo_cover_queue_counts(&state);
        return json_response(
            request,
            200,
            json!({
                "ok": true,
                "policy": DEMO_COVER_POLICY,
                "test_mode": demo_cover_test_mode(),
                "queued_global": queued_global,
                "queued_by_route": queued_by_route,
                "run_items": state.cover.run_items,
                "run_payload_bytes": state.cover.run_payload_bytes,
                "run_request_bytes": state.cover.run_request_bytes,
                "retained_artifacts": state.cover.retained_artifacts.len(),
                "retained_artifact_bytes": state.cover.retained_artifact_bytes,
                "retained_artifact_summaries": demo_cover_artifact_summaries(&state.cover),
                "purged_artifacts": state.cover.purged_artifacts,
                "purged_items": state.cover.purged_items,
                "rejected_generations": state.cover.rejected_generations,
                "max_retained_artifacts": DEMO_COVER_MAX_RETAINED_ARTIFACTS,
                "max_retained_artifact_bytes": DEMO_COVER_MAX_RETAINED_ARTIFACT_BYTES
            }),
        );
    }

    if method == Method::Post && url == "/cover-traffic/purge" {
        if !demo_cover_enabled() {
            return json_response(
                request,
                404,
                json!({ "ok": false, "error": "cover traffic disabled" }),
            );
        }
        let body = match read_json_body(&mut request, MAX_BODY_BYTES) {
            Ok(v) => v,
            Err(e) => {
                let status = json_body_error_status(&e);
                return json_response(request, status, json!({ "ok": false, "error": e }));
            }
        };
        let route = body.get("to").and_then(|v| v.as_str()).map(str::to_string);
        if route.as_deref().is_some_and(|to| !valid_relay_id(to)) {
            return json_response(
                request,
                400,
                json!({ "ok": false, "error": "invalid route" }),
            );
        }
        let mut state = match lock_relay_state(state) {
            Ok(guard) => guard,
            Err(err) => {
                return json_response(request, 500, json!({ "ok": false, "error": err }));
            }
        };
        let purged_items = purge_demo_cover_items(&mut state, route.as_deref());
        let purged_artifacts = state.cover.retained_artifacts.len();
        state.cover.retained_artifacts.clear();
        state.cover.retained_artifact_bytes = 0;
        state.cover.purged_artifacts = state
            .cover
            .purged_artifacts
            .saturating_add(purged_artifacts);
        return json_response(
            request,
            200,
            json!({
                "ok": true,
                "policy": DEMO_COVER_POLICY,
                "purged_cover_items": purged_items,
                "purged_artifacts": purged_artifacts
            }),
        );
    }

    if method == Method::Post && url == "/send" {
        let body = match read_json_body(&mut request, MAX_BODY_BYTES) {
            Ok(v) => v,
            Err(e) => {
                let status = json_body_error_status(&e);
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
        let (Some(to), Some(from), Some(msg)) = (to, from, msg) else {
            return json_response(
                request,
                400,
                json!({ "ok": false, "error": "missing to/from/msg" }),
            );
        };
        if invalid_padding_metadata(&msg, pad_len, bucket) {
            return json_response(
                request,
                400,
                json!({ "ok": false, "error": "invalid padding metadata" }),
            );
        }
        let mut state = match lock_relay_state(state) {
            Ok(guard) => guard,
            Err(err) => {
                return json_response(request, 500, json!({ "ok": false, "error": err }));
            }
        };
        if state.total_msgs >= MAX_TOTAL_QUEUE {
            return json_response(request, 429, json!({ "ok": false, "error": "queue full" }));
        }
        if !token_quota_available(&state, &token_value) {
            return quota_response(request, json_response);
        }
        if state.queues.get(&to).map(|q| q.len()).unwrap_or(0) >= MAX_QUEUE_PER_RECIPIENT {
            return json_response(
                request,
                429,
                json!({ "ok": false, "error": "recipient queue full" }),
            );
        }
        let seq = state.next_msg_seq;
        state.next_msg_seq = state.next_msg_seq.saturating_add(1);
        let ack_id = make_ack_id(&token_value, &to, seq);
        let entry = state.queues.entry(to).or_default();
        entry.push_back(QueuedMsg {
            ack_id,
            from,
            msg,
            pad_len,
            bucket,
            token: token_value.clone(),
            cover: false,
            cover_mode: None,
            cover_payload_len: None,
        });
        *state.token_queued.entry(token_value).or_insert(0) += 1;
        state.total_msgs += 1;
        return json_response(request, 200, json!({ "ok": true, "queued": 1 }));
    }

    if method == Method::Post && url == "/send-batch" {
        if !demo_batching_enabled() {
            return json_response(
                request,
                404,
                json!({ "ok": false, "error": "batching disabled" }),
            );
        }
        let body = match read_json_body(&mut request, MAX_BODY_BYTES) {
            Ok(v) => v,
            Err(e) => {
                let status = json_body_error_status(&e);
                return json_response(request, status, json!({ "ok": false, "error": e }));
            }
        };
        let Some(messages) = body.get("messages").and_then(|v| v.as_array()) else {
            return json_response(
                request,
                400,
                json!({ "ok": false, "error": "missing messages" }),
            );
        };
        if messages.is_empty() || messages.len() > DEMO_BATCH_MAX_SIZE {
            return json_response(
                request,
                400,
                json!({ "ok": false, "error": "invalid batch size" }),
            );
        }
        let mut batch = Vec::with_capacity(messages.len());
        for member in messages {
            let member = match parse_batch_send_member(member) {
                Ok(member) => member,
                Err(error) => {
                    return json_response(request, 400, json!({ "ok": false, "error": error }));
                }
            };
            batch.push(member);
        }

        let mut state = match lock_relay_state(state) {
            Ok(guard) => guard,
            Err(err) => {
                return json_response(request, 500, json!({ "ok": false, "error": err }));
            }
        };
        if state.total_msgs.saturating_add(batch.len()) > MAX_TOTAL_QUEUE {
            return json_response(request, 429, json!({ "ok": false, "error": "queue full" }));
        }
        if state
            .token_queued
            .get(&token_value)
            .copied()
            .unwrap_or(0)
            .saturating_add(batch.len())
            > MAX_QUEUE_PER_TOKEN
        {
            return quota_response(request, json_response);
        }
        let mut recipient_counts: HashMap<String, usize> = HashMap::new();
        for member in &batch {
            let base = state
                .queues
                .get(&member.to)
                .map(|queue| queue.len())
                .unwrap_or(0);
            let count = recipient_counts.entry(member.to.clone()).or_insert(base);
            *count = count.saturating_add(1);
            if *count > MAX_QUEUE_PER_RECIPIENT {
                return json_response(
                    request,
                    429,
                    json!({ "ok": false, "error": "recipient queue full" }),
                );
            }
        }

        let queued = batch.len();
        for member in batch {
            let seq = state.next_msg_seq;
            state.next_msg_seq = state.next_msg_seq.saturating_add(1);
            let ack_id = make_ack_id(&token_value, &member.to, seq);
            let entry = state.queues.entry(member.to).or_default();
            entry.push_back(QueuedMsg {
                ack_id,
                from: member.from,
                msg: member.msg,
                pad_len: member.pad_len,
                bucket: member.bucket,
                token: token_value.clone(),
                cover: false,
                cover_mode: None,
                cover_payload_len: None,
            });
        }
        *state.token_queued.entry(token_value).or_insert(0) += queued;
        state.total_msgs += queued;
        return json_response(
            request,
            200,
            json!({
                "ok": true,
                "queued": queued,
                "policy": DEMO_BATCH_POLICY,
                "test_mode": demo_batching_test_mode(),
                "max_batch_size": DEMO_BATCH_MAX_SIZE,
                "max_window_ms": DEMO_BATCH_MAX_WINDOW_MS
            }),
        );
    }

    if method == Method::Post && url == "/poll-candidate" {
        let body = match read_json_body(&mut request, MAX_BODY_BYTES) {
            Ok(v) => v,
            Err(e) => {
                let status = json_body_error_status(&e);
                return json_response(request, status, json!({ "ok": false, "error": e }));
            }
        };
        let Some(id) = body
            .get("id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
        else {
            return json_response(request, 400, json!({ "ok": false, "error": "missing id" }));
        };
        let max = body.get("max").and_then(|v| v.as_u64()).unwrap_or(1) as usize;
        let mut state = match lock_relay_state(state) {
            Ok(guard) => guard,
            Err(err) => {
                return json_response(request, 500, json!({ "ok": false, "error": err }));
            }
        };
        if !check_rate_limit(&mut state, &token_value, RateKind::Poll) {
            return rate_limit_response(request, json_response);
        }
        let msgs = state
            .queues
            .get(&id)
            .map(|queue| {
                queue
                    .iter()
                    .filter(|entry| !entry.cover)
                    .chain(queue.iter().filter(|entry| entry.cover))
                    .take(max)
                    .map(relay_candidate_json)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        return json_response(request, 200, json!({ "ok": true, "msgs": msgs }));
    }

    if method == Method::Post && url == "/ack" {
        let body = match read_json_body(&mut request, MAX_BODY_BYTES) {
            Ok(v) => v,
            Err(e) => {
                let status = json_body_error_status(&e);
                return json_response(request, status, json!({ "ok": false, "error": e }));
            }
        };
        let Some(id) = body
            .get("id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
        else {
            return json_response(request, 400, json!({ "ok": false, "error": "missing id" }));
        };
        let Some(ack_id) = body
            .get("ack_id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
        else {
            return json_response(request, 400, json!({ "ok": false, "error": "missing ack" }));
        };
        if !valid_ack_id(&ack_id) {
            return json_response(request, 400, json!({ "ok": false, "error": "invalid ack" }));
        }
        let mut state = match lock_relay_state(state) {
            Ok(guard) => guard,
            Err(err) => {
                return json_response(request, 500, json!({ "ok": false, "error": err }));
            }
        };
        let (removed_token, remove_queue) = {
            let Some(queue) = state.queues.get_mut(&id) else {
                return json_response(
                    request,
                    404,
                    json!({ "ok": false, "error": "ack not found" }),
                );
            };
            let Some(pos) = queue.iter().position(|entry| entry.ack_id == ack_id) else {
                return json_response(
                    request,
                    404,
                    json!({ "ok": false, "error": "ack not found" }),
                );
            };
            let Some(entry) = queue.remove(pos) else {
                return json_response(request, 500, json!({ "ok": false, "error": "ack failed" }));
            };
            (entry.token, queue.is_empty())
        };
        if remove_queue {
            state.queues.remove(&id);
        }
        if let Some(count) = state.token_queued.get_mut(&removed_token) {
            *count = count.saturating_sub(1);
            if *count == 0 {
                state.token_queued.remove(&removed_token);
            }
        }
        state.total_msgs = state.total_msgs.saturating_sub(1);
        return json_response(request, 200, json!({ "ok": true, "acked": 1 }));
    }

    if method == Method::Post && url == "/ack-batch" {
        if !demo_batching_enabled() {
            return json_response(
                request,
                404,
                json!({ "ok": false, "error": "batching disabled" }),
            );
        }
        let body = match read_json_body(&mut request, MAX_BODY_BYTES) {
            Ok(v) => v,
            Err(e) => {
                let status = json_body_error_status(&e);
                return json_response(request, status, json!({ "ok": false, "error": e }));
            }
        };
        let Some(id) = body
            .get("id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
        else {
            return json_response(request, 400, json!({ "ok": false, "error": "missing id" }));
        };
        let Some(ack_values) = body.get("ack_ids").and_then(|v| v.as_array()) else {
            return json_response(
                request,
                400,
                json!({ "ok": false, "error": "missing ack batch" }),
            );
        };
        if ack_values.is_empty() || ack_values.len() > DEMO_BATCH_MAX_SIZE {
            return json_response(
                request,
                400,
                json!({ "ok": false, "error": "invalid ack batch size" }),
            );
        }
        let mut ack_ids = Vec::with_capacity(ack_values.len());
        let mut unique_ack_ids = HashSet::new();
        for ack_value in ack_values {
            let Some(ack_id) = ack_value.as_str().map(|s| s.to_string()) else {
                return json_response(request, 400, json!({ "ok": false, "error": "invalid ack" }));
            };
            if !valid_ack_id(&ack_id) || !unique_ack_ids.insert(ack_id.clone()) {
                return json_response(request, 400, json!({ "ok": false, "error": "invalid ack" }));
            }
            ack_ids.push(ack_id);
        }

        let mut state = match lock_relay_state(state) {
            Ok(guard) => guard,
            Err(err) => {
                return json_response(request, 500, json!({ "ok": false, "error": err }));
            }
        };
        let (removed_tokens, remove_queue) = {
            let Some(queue) = state.queues.get_mut(&id) else {
                return json_response(
                    request,
                    404,
                    json!({ "ok": false, "error": "ack not found" }),
                );
            };
            if !ack_ids
                .iter()
                .all(|ack_id| queue.iter().any(|entry| entry.ack_id == *ack_id))
            {
                return json_response(
                    request,
                    404,
                    json!({ "ok": false, "error": "ack not found" }),
                );
            }
            let mut removed_tokens = Vec::with_capacity(ack_ids.len());
            queue.retain(|entry| {
                if unique_ack_ids.contains(&entry.ack_id) {
                    removed_tokens.push(entry.token.clone());
                    false
                } else {
                    true
                }
            });
            (removed_tokens, queue.is_empty())
        };
        let acked = removed_tokens.len();
        if remove_queue {
            state.queues.remove(&id);
        }
        for token_key in removed_tokens {
            if let Some(count) = state.token_queued.get_mut(&token_key) {
                *count = count.saturating_sub(1);
                if *count == 0 {
                    state.token_queued.remove(&token_key);
                }
            }
        }
        state.total_msgs = state.total_msgs.saturating_sub(acked);
        return json_response(
            request,
            200,
            json!({
                "ok": true,
                "acked": acked,
                "policy": DEMO_BATCH_POLICY,
                "test_mode": demo_batching_test_mode()
            }),
        );
    }

    if method == Method::Post && url == "/poll" {
        let body = match read_json_body(&mut request, MAX_BODY_BYTES) {
            Ok(v) => v,
            Err(e) => {
                let status = json_body_error_status(&e);
                return json_response(request, status, json!({ "ok": false, "error": e }));
            }
        };
        let Some(id) = body
            .get("id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
        else {
            return json_response(request, 400, json!({ "ok": false, "error": "missing id" }));
        };
        let max = body.get("max").and_then(|v| v.as_u64()).unwrap_or(1) as usize;
        let mut state = match lock_relay_state(state) {
            Ok(guard) => guard,
            Err(err) => {
                return json_response(request, 500, json!({ "ok": false, "error": err }));
            }
        };
        if !check_rate_limit(&mut state, &token_value, RateKind::Poll) {
            return rate_limit_response(request, json_response);
        }
        let (msgs, removed, removed_tokens, purged_cover_items) = {
            let queue = state.queues.entry(id).or_default();
            let mut msgs = Vec::new();
            let mut removed = 0usize;
            let mut removed_tokens: Vec<String> = Vec::new();
            let mut purged_cover_items = 0usize;
            while msgs.len() < max {
                if let Some(entry) = queue.pop_front() {
                    removed_tokens.push(entry.token);
                    if !entry.cover {
                        msgs.push(json!({
                            "from": entry.from,
                            "msg": entry.msg,
                            "pad_len": entry.pad_len,
                            "bucket": entry.bucket
                        }));
                    } else {
                        purged_cover_items = purged_cover_items.saturating_add(1);
                    }
                    removed += 1;
                } else {
                    break;
                }
            }
            (msgs, removed, removed_tokens, purged_cover_items)
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
        if purged_cover_items > 0 {
            state.cover.purged_items = state.cover.purged_items.saturating_add(purged_cover_items);
        }
        return json_response(request, 200, json!({ "ok": true, "msgs": msgs }));
    }

    let resp = Response::from_string("not found").with_status_code(404);
    ResponseWrapper {
        request,
        response: resp,
    }
}

struct DemoCoverRequest {
    to: String,
    from: String,
    mode: DemoCoverMode,
    payload_len: usize,
    items: usize,
}

fn parse_demo_cover_request(body: &serde_json::Value) -> Result<DemoCoverRequest, &'static str> {
    let Some(to) = body.get("to").and_then(|v| v.as_str()).map(str::to_string) else {
        return Err("missing cover route");
    };
    if !valid_relay_id(&to) {
        return Err("invalid cover route");
    }
    let mode = body
        .get("mode")
        .and_then(|v| v.as_str())
        .and_then(DemoCoverMode::parse)
        .ok_or("invalid cover mode")?;
    let from = body
        .get("from")
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .unwrap_or_else(|| DEMO_COVER_DEFAULT_FROM.to_string());
    if !valid_relay_id(&from) {
        return Err("invalid cover source");
    }
    if mode == DemoCoverMode::ActiveSession && from == DEMO_COVER_DEFAULT_FROM {
        return Err("active-session cover requires source peer");
    }
    let payload_len = body
        .get("payload_len")
        .and_then(|v| v.as_u64())
        .map(|n| n as usize)
        .unwrap_or(64);
    if payload_len == 0 || payload_len > DEMO_COVER_MAX_PAYLOAD_BYTES {
        return Err("cover payload cap exceeded");
    }
    let items = body
        .get("items")
        .and_then(|v| v.as_u64())
        .map(|n| n as usize)
        .unwrap_or(1);
    if items == 0 {
        return Err("invalid cover item count");
    }
    if mode != DemoCoverMode::BatchFill && items != 1 {
        return Err("cover item count requires batch-fill mode");
    }
    if items > DEMO_COVER_MAX_QUEUED_PER_ROUTE {
        return Err("cover route queue cap exceeded");
    }
    Ok(DemoCoverRequest {
        to,
        from,
        mode,
        payload_len,
        items,
    })
}

fn validate_demo_cover_caps(
    state: &RelayState,
    cover_request: &DemoCoverRequest,
    now_ms: u64,
    request_bytes: usize,
    token: &str,
) -> Result<(), &'static str> {
    let requested_payload = cover_request
        .payload_len
        .checked_mul(cover_request.items)
        .ok_or("cover payload cap exceeded")?;
    if state.cover.run_items.saturating_add(cover_request.items) > DEMO_COVER_MAX_ITEMS_PER_RUN {
        return Err("cover run item cap exceeded");
    }
    if state
        .cover
        .run_payload_bytes
        .saturating_add(requested_payload)
        > DEMO_COVER_MAX_PAYLOAD_BYTES_PER_RUN
    {
        return Err("cover run payload cap exceeded");
    }
    if state.cover.run_request_bytes.saturating_add(request_bytes)
        > DEMO_COVER_MAX_REQUEST_BYTES_PER_RUN
    {
        return Err("cover request byte cap exceeded");
    }
    let minute_floor = now_ms.saturating_sub(60_000);
    let hour_floor = now_ms.saturating_sub(3_600_000);
    let minute_count = state
        .cover
        .generated_ms
        .iter()
        .filter(|ts| **ts >= minute_floor)
        .count();
    let hour_count = state
        .cover
        .generated_ms
        .iter()
        .filter(|ts| **ts >= hour_floor)
        .count();
    if minute_count.saturating_add(cover_request.items) > DEMO_COVER_MAX_ITEMS_PER_MINUTE {
        return Err("cover minute quota exceeded");
    }
    if hour_count.saturating_add(cover_request.items) > DEMO_COVER_MAX_ITEMS_PER_HOUR {
        return Err("cover hour quota exceeded");
    }
    let (queued_global, queued_by_route) = demo_cover_queue_counts(state);
    if queued_global.saturating_add(cover_request.items) > DEMO_COVER_MAX_QUEUED_GLOBAL {
        return Err("cover global queue cap exceeded");
    }
    let route_count = queued_by_route.get(&cover_request.to).copied().unwrap_or(0);
    if route_count.saturating_add(cover_request.items) > DEMO_COVER_MAX_QUEUED_PER_ROUTE {
        return Err("cover route queue cap exceeded");
    }
    if state.total_msgs.saturating_add(cover_request.items) > MAX_TOTAL_QUEUE {
        return Err("queue full");
    }
    if state
        .token_queued
        .get(token)
        .copied()
        .unwrap_or(0)
        .saturating_add(cover_request.items)
        > MAX_QUEUE_PER_TOKEN
    {
        return Err("token quota exceeded");
    }
    if state
        .queues
        .get(&cover_request.to)
        .map(|q| q.len())
        .unwrap_or(0)
        .saturating_add(cover_request.items)
        > MAX_QUEUE_PER_RECIPIENT
    {
        return Err("recipient queue full");
    }
    let artifact_bytes = demo_cover_artifact_retained_bytes(cover_request);
    if state
        .cover
        .retained_artifact_bytes
        .saturating_add(artifact_bytes)
        > DEMO_COVER_MAX_RETAINED_ARTIFACT_BYTES
    {
        return Err("cover artifact retention cap exceeded");
    }
    Ok(())
}

fn make_demo_cover_payload(
    mode: DemoCoverMode,
    to: &str,
    from: &str,
    seq: u64,
    payload_len: usize,
    now_ms: u64,
) -> String {
    let mut payload = Vec::with_capacity(payload_len);
    let mut block_seq = 0u64;
    while payload.len() < payload_len {
        let mut h = Sha256::new();
        h.update(b"qshield-demo-cover-traffic-v1");
        h.update([0u8]);
        h.update(mode.as_str().as_bytes());
        h.update([0u8]);
        h.update(to.as_bytes());
        h.update([0u8]);
        h.update(from.as_bytes());
        h.update([0u8]);
        h.update(seq.to_be_bytes());
        h.update([0u8]);
        h.update(block_seq.to_be_bytes());
        h.update([0u8]);
        if demo_cover_test_mode() {
            h.update(b"deterministic-test-mode");
        } else {
            h.update(now_ms.to_be_bytes());
        }
        payload.extend_from_slice(&h.finalize());
        block_seq = block_seq.saturating_add(1);
    }
    payload.truncate(payload_len);
    hex::encode(payload)
}

fn record_demo_cover_artifact(
    ledger: &mut DemoCoverLedger,
    cover_request: &DemoCoverRequest,
    seq: u64,
    item_index: usize,
    now_ms: u64,
) {
    let route_tag = demo_cover_route_tag(&cover_request.to);
    let artifact_tag =
        demo_cover_artifact_tag(cover_request.mode, &route_tag, seq, item_index, now_ms);
    let retained_bytes = cover_request.mode.as_str().len()
        + route_tag.len()
        + artifact_tag.len()
        + std::mem::size_of::<usize>()
        + std::mem::size_of::<u64>();
    ledger.retained_artifact_bytes = ledger
        .retained_artifact_bytes
        .saturating_add(retained_bytes);
    ledger.retained_artifacts.push_back(DemoCoverArtifact {
        mode: cover_request.mode.as_str().to_string(),
        payload_len: cover_request.payload_len,
        route_tag,
        artifact_tag,
        generated_ms: now_ms,
        retained_bytes,
    });
}

fn purge_demo_cover_artifact_capacity(ledger: &mut DemoCoverLedger, incoming_items: usize) {
    while ledger.retained_artifacts.len() > DEMO_COVER_MAX_RETAINED_ARTIFACTS {
        if let Some(removed) = ledger.retained_artifacts.pop_front() {
            ledger.retained_artifact_bytes = ledger
                .retained_artifact_bytes
                .saturating_sub(removed.retained_bytes);
            ledger.purged_artifacts = ledger.purged_artifacts.saturating_add(1);
        }
    }
    while ledger
        .retained_artifacts
        .len()
        .saturating_add(incoming_items)
        > DEMO_COVER_MAX_RETAINED_ARTIFACTS
    {
        if let Some(removed) = ledger.retained_artifacts.pop_front() {
            ledger.retained_artifact_bytes = ledger
                .retained_artifact_bytes
                .saturating_sub(removed.retained_bytes);
            ledger.purged_artifacts = ledger.purged_artifacts.saturating_add(1);
        } else {
            break;
        }
    }
}

fn demo_cover_artifact_retained_bytes(cover_request: &DemoCoverRequest) -> usize {
    cover_request.items
        * (cover_request.mode.as_str().len()
            + (DEMO_COVER_ROUTE_TAG_BYTES * 2)
            + (DEMO_COVER_ARTIFACT_TAG_BYTES * 2)
            + std::mem::size_of::<usize>()
            + std::mem::size_of::<u64>())
}

fn demo_cover_route_tag(route: &str) -> String {
    let mut h = Sha256::new();
    h.update(b"qshield-demo-cover-route-v1");
    h.update([0u8]);
    h.update(route.as_bytes());
    let bytes = h.finalize();
    hex::encode(&bytes[..DEMO_COVER_ROUTE_TAG_BYTES])
}

fn demo_cover_artifact_tag(
    mode: DemoCoverMode,
    route_tag: &str,
    seq: u64,
    item_index: usize,
    now_ms: u64,
) -> String {
    let mut h = Sha256::new();
    h.update(b"qshield-demo-cover-artifact-v1");
    h.update([0u8]);
    h.update(mode.as_str().as_bytes());
    h.update([0u8]);
    h.update(route_tag.as_bytes());
    h.update([0u8]);
    h.update(seq.to_be_bytes());
    h.update([0u8]);
    h.update(item_index.to_be_bytes());
    h.update([0u8]);
    h.update(now_ms.to_be_bytes());
    let bytes = h.finalize();
    hex::encode(&bytes[..DEMO_COVER_ARTIFACT_TAG_BYTES])
}

fn demo_cover_artifact_summaries(ledger: &DemoCoverLedger) -> Vec<serde_json::Value> {
    ledger
        .retained_artifacts
        .iter()
        .map(|artifact| {
            json!({
                "mode": artifact.mode,
                "payload_len": artifact.payload_len,
                "route_tag": artifact.route_tag,
                "artifact_tag": artifact.artifact_tag,
                "generated_ms": artifact.generated_ms,
                "retained_bytes": artifact.retained_bytes
            })
        })
        .collect()
}

fn demo_cover_queue_counts(state: &RelayState) -> (usize, HashMap<String, usize>) {
    let mut total = 0usize;
    let mut by_route = HashMap::new();
    for (route, queue) in &state.queues {
        let count = queue.iter().filter(|entry| entry.cover).count();
        if count > 0 {
            total = total.saturating_add(count);
            by_route.insert(route.clone(), count);
        }
    }
    (total, by_route)
}

fn purge_demo_cover_items(state: &mut RelayState, route: Option<&str>) -> usize {
    let routes = state.queues.keys().cloned().collect::<Vec<_>>();
    let mut removed_tokens = Vec::new();
    let mut purged = 0usize;
    for key in routes {
        if route.is_some_and(|route| route != key) {
            continue;
        }
        let Some(queue) = state.queues.get_mut(&key) else {
            continue;
        };
        queue.retain(|entry| {
            if entry.cover {
                removed_tokens.push(entry.token.clone());
                purged = purged.saturating_add(1);
                false
            } else {
                true
            }
        });
        if queue.is_empty() {
            state.queues.remove(&key);
        }
    }
    for token_key in removed_tokens {
        if let Some(count) = state.token_queued.get_mut(&token_key) {
            *count = count.saturating_sub(1);
            if *count == 0 {
                state.token_queued.remove(&token_key);
            }
        }
    }
    state.total_msgs = state.total_msgs.saturating_sub(purged);
    state.cover.purged_items = state.cover.purged_items.saturating_add(purged);
    purged
}

fn relay_candidate_json(entry: &QueuedMsg) -> serde_json::Value {
    json!({
        "ack_id": entry.ack_id.clone(),
        "from": entry.from.clone(),
        "msg": entry.msg.clone(),
        "cover": entry.cover,
        "cover_mode": entry.cover_mode.clone(),
        "cover_payload_len": entry.cover_payload_len,
        "pad_len": entry.pad_len,
        "bucket": entry.bucket
    })
}

fn read_json_body(
    request: &mut tiny_http::Request,
    max_bytes: usize,
) -> Result<serde_json::Value, String> {
    if !has_json_content_type(request) {
        return Err("unsupported content type".to_string());
    }
    let mut buf = Vec::new();
    request
        .as_reader()
        .take((max_bytes + 1) as u64)
        .read_to_end(&mut buf)
        .map_err(|_| "read body failed".to_string())?;
    if buf.len() > max_bytes {
        return Err("body too large".to_string());
    }
    serde_json::from_slice(&buf).map_err(|_| "invalid json".to_string())
}

fn json_body_error_status(error: &str) -> u16 {
    match error {
        "body too large" => 413,
        "unsupported content type" => 415,
        _ => 400,
    }
}

fn has_json_content_type(request: &tiny_http::Request) -> bool {
    request.headers().iter().any(|header| {
        if !header.field.equiv("Content-Type") {
            return false;
        }
        let Ok(value) = std::str::from_utf8(header.value.as_ref()) else {
            return false;
        };
        value
            .split(';')
            .next()
            .map(|kind| kind.trim().eq_ignore_ascii_case("application/json"))
            .unwrap_or(false)
    })
}

fn invalid_padding_metadata(msg: &str, pad_len: u32, bucket: Option<u32>) -> bool {
    let Some(bucket) = bucket else {
        return pad_len != 0;
    };
    if bucket == 0 || !msg.len().is_multiple_of(2) {
        return true;
    }
    let wire_len = msg.len() / 2;
    wire_len != bucket as usize || pad_len > bucket
}

fn parse_batch_send_member(value: &serde_json::Value) -> Result<BatchSendMember, &'static str> {
    let Some(to) = value.get("to").and_then(|v| v.as_str()).map(str::to_string) else {
        return Err("missing to/from/msg");
    };
    let Some(from) = value
        .get("from")
        .and_then(|v| v.as_str())
        .map(str::to_string)
    else {
        return Err("missing to/from/msg");
    };
    let Some(msg) = value
        .get("msg")
        .and_then(|v| v.as_str())
        .map(str::to_string)
    else {
        return Err("missing to/from/msg");
    };
    let pad_len = value
        .get("pad_len")
        .and_then(|v| v.as_u64())
        .map(|n| n as u32)
        .unwrap_or(0);
    let bucket = value
        .get("bucket")
        .and_then(|v| v.as_u64())
        .map(|n| n as u32);
    if invalid_padding_metadata(&msg, pad_len, bucket) {
        return Err("invalid padding metadata");
    }
    Ok(BatchSendMember {
        to,
        from,
        msg,
        pad_len,
        bucket,
    })
}

fn lock_relay_state(
    state: &Arc<Mutex<RelayState>>,
) -> Result<MutexGuard<'_, RelayState>, &'static str> {
    state.lock().map_err(|_| RELAY_LOCK_POISON)
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

fn valid_ack_id(id: &str) -> bool {
    id.len() == 64 && id.bytes().all(|b| b.is_ascii_hexdigit())
}

fn make_ack_id(token: &str, recipient: &str, seq: u64) -> String {
    let mut h = Sha256::new();
    h.update(b"qshield-relay-ack-v1");
    h.update([0u8]);
    h.update(token.as_bytes());
    h.update([0u8]);
    h.update(recipient.as_bytes());
    h.update([0u8]);
    h.update(seq.to_be_bytes());
    hex::encode(h.finalize())
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

fn demo_batching_enabled() -> bool {
    env_flag("QSHIELD_DEMO_BATCHING")
}

fn demo_batching_test_mode() -> bool {
    env_flag("QSHIELD_DEMO_BATCHING_TEST_MODE")
}

fn demo_cover_enabled() -> bool {
    env_flag("QSHIELD_DEMO_COVER_TRAFFIC")
}

fn demo_cover_test_mode() -> bool {
    env_flag("QSHIELD_DEMO_COVER_TRAFFIC_TEST_MODE")
}

fn demo_cover_now_ms(body: &serde_json::Value) -> Result<u64, &'static str> {
    if let Some(value) = body.get("test_now_ms") {
        if !demo_cover_test_mode() {
            return Err("cover test clock requires test mode");
        }
        return value.as_u64().ok_or("cover test clock invalid");
    }
    if let Ok(value) = std::env::var("QSHIELD_DEMO_COVER_TRAFFIC_NOW_MS") {
        return value.parse::<u64>().map_err(|_| "cover test clock invalid");
    }
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .map_err(|_| "cover clock invalid")
}

fn demo_cover_free_disk_bytes() -> Result<u64, &'static str> {
    if let Ok(value) = std::env::var("QSHIELD_DEMO_COVER_TRAFFIC_DISK_FREE_BYTES") {
        return value
            .parse::<u64>()
            .map_err(|_| "cover disk floor check invalid");
    }
    let output = Command::new("df")
        .args(["-Pk", "/srv/qbuild"])
        .output()
        .map_err(|_| "cover disk floor check failed")?;
    if !output.status.success() {
        return Err("cover disk floor check failed");
    }
    let text = String::from_utf8(output.stdout).map_err(|_| "cover disk floor check failed")?;
    let Some(line) = text.lines().nth(1) else {
        return Err("cover disk floor check failed");
    };
    let Some(available_kib) = line
        .split_whitespace()
        .nth(3)
        .and_then(|value| value.parse::<u64>().ok())
    else {
        return Err("cover disk floor check failed");
    };
    available_kib
        .checked_mul(1024)
        .ok_or("cover disk floor check failed")
}

fn env_flag(name: &str) -> bool {
    std::env::var(name)
        .map(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(false)
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    fn poison_relay_state(state: &Arc<Mutex<RelayState>>) {
        let state = state.clone();
        let _ = thread::spawn(move || {
            let _guard = state.lock().unwrap();
            panic!("poison relay mutex");
        })
        .join();
    }

    #[test]
    fn relay_state_lock_poisoned_is_deterministic_and_no_mutation() {
        let mut baseline = RelayState::default();
        baseline.bundles.insert("id".to_string(), json!({ "b": 1 }));
        baseline.establish_fingerprints.insert("fp".to_string());
        baseline.rate.insert(
            "tok".to_string(),
            RateCounts {
                register: 2,
                poll: 3,
            },
        );
        baseline.queues.insert(
            "q".to_string(),
            VecDeque::from([QueuedMsg {
                ack_id: "a".repeat(64),
                from: "a".to_string(),
                msg: "b".to_string(),
                pad_len: 0,
                bucket: None,
                token: "tok".to_string(),
                cover: false,
                cover_mode: None,
                cover_payload_len: None,
            }]),
        );
        baseline.token_queued.insert("tok".to_string(), 1);
        baseline.total_msgs = 1;

        let state = Arc::new(Mutex::new(baseline));
        poison_relay_state(&state);

        let err1 = match lock_relay_state(&state) {
            Ok(_) => panic!("expected poisoned lock error"),
            Err(err) => err,
        };
        let err2 = match lock_relay_state(&state) {
            Ok(_) => panic!("expected poisoned lock error"),
            Err(err) => err,
        };
        assert_eq!(err1, RELAY_LOCK_POISON);
        assert_eq!(err2, RELAY_LOCK_POISON);

        let guard = state.lock().unwrap_or_else(|e| e.into_inner());
        assert_eq!(guard.total_msgs, 1);
        assert_eq!(guard.token_queued.get("tok"), Some(&1));
        assert_eq!(guard.bundles.get("id"), Some(&json!({ "b": 1 })));
        assert!(guard.establish_fingerprints.contains("fp"));
        assert_eq!(guard.rate.get("tok").map(|r| r.register), Some(2));
        assert_eq!(guard.rate.get("tok").map(|r| r.poll), Some(3));
        let queue_len = guard.queues.get("q").map(|q| q.len()).unwrap_or(0);
        assert_eq!(queue_len, 1);
    }

    #[test]
    fn relay_state_lock_poisoned_returns_err() {
        let state = Arc::new(Mutex::new(RelayState::default()));
        poison_relay_state(&state);
        let err = match lock_relay_state(&state) {
            Ok(_) => panic!("expected poisoned lock error"),
            Err(err) => err,
        };
        assert_eq!(err, RELAY_LOCK_POISON);
    }
}
