use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::actor::ActorClient;
use crate::config::{self, Config};
use crate::relay_client::{post_json, AckRequest, GenericOk, PollRequest, PollResponse, RelayMsg};
use crate::store::{SessionEntry, StoreState};
use crate::util::{load_or_init_state, state_path};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const DEMO_RETRY_LEDGER_FILE: &str = ".qshield_demo_retry_cadence_v1.json";
const DEMO_RETRY_POLICY: &str = "qshield_demo_retry_cadence_v1";
const DEMO_RETRY_WINDOW_MS: u64 = 60_000;
const DEMO_RETRY_MAX_INVALID_ATTEMPTS: u32 = 4;
const DEMO_RETRY_BACKOFF_MS: [u64; 4] = [0, 500, 1000, 2000];
const DEMO_JITTER_POLICY: &str = "qshield_demo_bounded_jitter_v1";
const DEMO_JITTER_MAX_MS: u64 = 250;
const DEMO_JITTER_COMPOSED_CAP_MS: u64 = 2250;

pub fn run(store_path: &Path, max: u32, demo_unauthenticated_override: bool) -> Result<(), String> {
    let cfg_path = store_path.join(config::CONFIG_FILE_NAME);
    let cfg: Config = config::read_config(&cfg_path).map_err(|_| {
        format!(
            "config missing or invalid: {} (run: qshield init --store <path>)",
            cfg_path.display()
        )
    })?;

    let state_path = state_path(store_path);
    let state: StoreState = load_or_init_state(&state_path)?;
    let my_id = state.my_id.clone().ok_or_else(|| {
        "identity missing; run: qshield register --store <path> --id <id>".to_string()
    })?;

    let relay_token = config::resolve_relay_token(&cfg)?;
    let poll = PollRequest {
        id: my_id.clone(),
        max,
    };
    let resp: PollResponse = post_json(&cfg.relay_url, "/poll-candidate", &poll, &relay_token)?;
    if !resp.ok {
        return Err("relay candidate poll failed".to_string());
    }
    let msgs = resp.msgs.unwrap_or_default();
    if msgs.is_empty() {
        apply_demo_retry_cadence(store_path, DemoRetryClass::EmptyPoll, None)?;
        println!("no messages");
        return Ok(());
    }

    let actor_path = std::env::var("QSHIELD_ACTOR")
        .unwrap_or_else(|_| "target/release/refimpl_actor".to_string());
    let mut actor: Option<ActorClient> = None;

    for msg in msgs {
        let ack_id = msg
            .ack_id
            .clone()
            .ok_or_else(|| "relay candidate missing ack".to_string())?;
        let candidate_tag = demo_retry_candidate_tag(&ack_id);
        apply_demo_retry_cadence(
            store_path,
            DemoRetryClass::InvalidCandidate,
            Some(&candidate_tag),
        )?;
        let sess: SessionEntry = state
            .sessions
            .get(&msg.from)
            .cloned()
            .ok_or_else(|| "no session for peer".to_string())?;

        let wire_hex = receive_wire_hex(&msg)?;

        if demo_unauthenticated_override {
            eprintln!("warning: unauthenticated establish override enabled (demo-only)");
        }
        if actor.is_none() {
            actor = Some(ActorClient::spawn(&actor_path)?);
        }
        let actor = actor
            .as_mut()
            .ok_or_else(|| "actor unavailable".to_string())?;

        let establish_params = serde_json::json!({
            "msg_type": { "u16": 1 },
            "negotiated": {
                "protocol_version": 1280,
                "suite_id": 2
            },
            "bound": {
                "protocol_version": 1280,
                "suite_id": 2,
                "pq_kem_pub_id": sess.pq_kem_pub_id_hex,
                "pq_prekey_id": sess.pq_prekey_id
            },
            "session_id": sess.session_id_hex,
            "dh_init": sess.dh_init_hex,
            "pq_init_ss": sess.pq_init_ss_hex,
            "pq_kem_pub_id": sess.pq_kem_pub_id_hex,
            "pq_prekey_id": { "u32": sess.pq_prekey_id },
            "dh_self_pub": sess.dh_self_pub_hex,
            "dh_peer_pub": sess.dh_peer_pub_hex,
            "authenticated": { "bool": demo_unauthenticated_override },
            "role": sess.role,
        });
        let _ = actor.call("suite2.establish.run", establish_params)?;

        let recv_params = serde_json::json!({
            "negotiated": {
                "protocol_version": 1280,
                "suite_id": 2
            },
            "session_id": sess.session_id_b64u,
            "wire_hex": wire_hex
        });
        let result = actor.call("suite2.e2e.recv", recv_params)?;
        let pt_hex = result
            .get("plaintext_hex")
            .and_then(|v| v.get("data"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| "actor response missing plaintext_hex".to_string())?;
        let pt = hex::decode(pt_hex).map_err(|e| format!("bad plaintext hex: {e}"))?;
        let text = String::from_utf8_lossy(&pt);
        let ack = AckRequest {
            id: my_id.clone(),
            ack_id,
        };
        let ack_resp: GenericOk = post_json(&cfg.relay_url, "/ack", &ack, &relay_token)?;
        if !ack_resp.ok {
            return Err("relay ack failed".to_string());
        }
        clear_demo_retry_cadence(store_path, DemoRetryClass::InvalidCandidate)?;
        println!("from {}: {}", msg.from, text);
    }

    Ok(())
}

fn receive_wire_hex(msg: &RelayMsg) -> Result<String, String> {
    let mut wire_bytes = hex::decode(&msg.msg).map_err(|_| "message decode reject".to_string())?;
    let pad_len = msg.pad_len.unwrap_or(0) as usize;
    if pad_len > wire_bytes.len() {
        return Err("padding reject".to_string());
    }
    if let Some(bucket) = msg.bucket {
        if wire_bytes.len() != bucket as usize {
            return Err("padding reject".to_string());
        }
    }
    if pad_len > 0 {
        let new_len = wire_bytes.len() - pad_len;
        if wire_bytes[new_len..].iter().any(|byte| *byte != 0) {
            return Err("padding reject".to_string());
        }
        wire_bytes.truncate(new_len);
    }
    Ok(hex::encode(&wire_bytes))
}

#[derive(Clone, Copy)]
enum DemoRetryClass {
    InvalidCandidate,
    EmptyPoll,
}

impl DemoRetryClass {
    fn key(self) -> &'static str {
        match self {
            DemoRetryClass::InvalidCandidate => "invalid_candidate",
            DemoRetryClass::EmptyPoll => "empty_poll",
        }
    }

    fn is_invalid_candidate(self) -> bool {
        matches!(self, DemoRetryClass::InvalidCandidate)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct DemoRetryLedger {
    policy: String,
    window_ms: u64,
    max_invalid_attempts: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    jitter_policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jitter_max_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jitter_composed_cap_ms: Option<u64>,
    entries: BTreeMap<String, DemoRetryEntry>,
}

impl Default for DemoRetryLedger {
    fn default() -> Self {
        Self {
            policy: DEMO_RETRY_POLICY.to_string(),
            window_ms: DEMO_RETRY_WINDOW_MS,
            max_invalid_attempts: DEMO_RETRY_MAX_INVALID_ATTEMPTS,
            jitter_policy: None,
            jitter_max_ms: None,
            jitter_composed_cap_ms: None,
            entries: BTreeMap::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct DemoRetryEntry {
    window_start_ms: u64,
    attempts: u32,
    last_delay_ms: u64,
    #[serde(default)]
    last_retry_delay_ms: u64,
    #[serde(default)]
    last_jitter_ms: u64,
    #[serde(default)]
    last_composed_delay_ms: u64,
    #[serde(default)]
    last_jitter_class: Option<String>,
    last_candidate_tag: Option<String>,
    capped: bool,
}

fn apply_demo_retry_cadence(
    store_path: &Path,
    class: DemoRetryClass,
    candidate_tag: Option<&str>,
) -> Result<(), String> {
    if !demo_timing_policy_enabled() {
        return Ok(());
    }

    let now_ms = demo_retry_now_ms()?;
    let mut ledger = load_demo_retry_ledger(store_path)?;
    refresh_demo_retry_policy(&mut ledger);

    let key = class.key().to_string();
    let entry = ledger
        .entries
        .entry(key)
        .or_insert_with(|| new_demo_retry_entry(now_ms, candidate_tag));
    let same_candidate = candidate_tag == entry.last_candidate_tag.as_deref();
    let window_expired = now_ms.saturating_sub(entry.window_start_ms) >= DEMO_RETRY_WINDOW_MS;
    if window_expired || (class.is_invalid_candidate() && !same_candidate) {
        *entry = new_demo_retry_entry(now_ms, candidate_tag);
    }

    let next_attempt = entry.attempts.saturating_add(1);
    if class.is_invalid_candidate() && next_attempt > DEMO_RETRY_MAX_INVALID_ATTEMPTS {
        entry.capped = true;
        entry.last_delay_ms = DEMO_RETRY_BACKOFF_MS[DEMO_RETRY_BACKOFF_MS.len() - 1];
        save_demo_retry_ledger(store_path, &ledger)?;
        return Err("retry cadence limit exceeded".to_string());
    }

    let retry_delay_ms = demo_retry_delay_ms(next_attempt);
    let jitter_class = demo_jitter_class(class, next_attempt);
    let jitter_ms = if let Some(jitter_class) = jitter_class {
        demo_jitter_delay_ms(jitter_class, next_attempt, candidate_tag, now_ms)
    } else {
        0
    };
    let composed_delay_ms = retry_delay_ms
        .saturating_add(jitter_ms)
        .min(DEMO_JITTER_COMPOSED_CAP_MS);

    entry.attempts = next_attempt;
    entry.last_delay_ms = composed_delay_ms;
    entry.last_retry_delay_ms = retry_delay_ms;
    entry.last_jitter_ms = jitter_ms;
    entry.last_composed_delay_ms = composed_delay_ms;
    entry.last_jitter_class = jitter_class.map(|class| class.key().to_string());
    entry.last_candidate_tag = candidate_tag.map(str::to_string);
    entry.capped = retry_delay_ms == DEMO_RETRY_BACKOFF_MS[DEMO_RETRY_BACKOFF_MS.len() - 1]
        || composed_delay_ms == DEMO_JITTER_COMPOSED_CAP_MS;
    let delay_ms = entry.last_delay_ms;
    save_demo_retry_ledger(store_path, &ledger)?;

    if delay_ms > 0 && !demo_retry_test_mode() && !demo_jitter_test_mode() {
        thread::sleep(Duration::from_millis(delay_ms));
    }

    Ok(())
}

fn clear_demo_retry_cadence(store_path: &Path, class: DemoRetryClass) -> Result<(), String> {
    if !demo_timing_policy_enabled() {
        return Ok(());
    }

    let mut ledger = load_demo_retry_ledger(store_path)?;
    ledger.entries.remove(class.key());
    save_demo_retry_ledger(store_path, &ledger)
}

fn new_demo_retry_entry(now_ms: u64, candidate_tag: Option<&str>) -> DemoRetryEntry {
    DemoRetryEntry {
        window_start_ms: now_ms,
        attempts: 0,
        last_delay_ms: 0,
        last_retry_delay_ms: 0,
        last_jitter_ms: 0,
        last_composed_delay_ms: 0,
        last_jitter_class: None,
        last_candidate_tag: candidate_tag.map(str::to_string),
        capped: false,
    }
}

fn demo_retry_delay_ms(attempts: u32) -> u64 {
    let idx = attempts.saturating_sub(1) as usize;
    DEMO_RETRY_BACKOFF_MS
        .get(idx)
        .copied()
        .unwrap_or(DEMO_RETRY_BACKOFF_MS[DEMO_RETRY_BACKOFF_MS.len() - 1])
}

fn load_demo_retry_ledger(store_path: &Path) -> Result<DemoRetryLedger, String> {
    let path = demo_retry_ledger_path(store_path);
    if !path.exists() {
        return Ok(DemoRetryLedger::default());
    }
    let data = fs::read(&path).map_err(|_| "retry cadence state read failed".to_string())?;
    serde_json::from_slice(&data).map_err(|_| "retry cadence state invalid".to_string())
}

fn save_demo_retry_ledger(store_path: &Path, ledger: &DemoRetryLedger) -> Result<(), String> {
    let path = demo_retry_ledger_path(store_path);
    let data = serde_json::to_vec_pretty(ledger)
        .map_err(|_| "retry cadence state serialize failed".to_string())?;
    fs::write(path, data).map_err(|_| "retry cadence state write failed".to_string())
}

fn demo_retry_ledger_path(store_path: &Path) -> PathBuf {
    store_path.join(DEMO_RETRY_LEDGER_FILE)
}

fn refresh_demo_retry_policy(ledger: &mut DemoRetryLedger) {
    ledger.policy = DEMO_RETRY_POLICY.to_string();
    ledger.window_ms = DEMO_RETRY_WINDOW_MS;
    ledger.max_invalid_attempts = DEMO_RETRY_MAX_INVALID_ATTEMPTS;
    if demo_jitter_enabled() {
        ledger.jitter_policy = Some(DEMO_JITTER_POLICY.to_string());
        ledger.jitter_max_ms = Some(DEMO_JITTER_MAX_MS);
        ledger.jitter_composed_cap_ms = Some(DEMO_JITTER_COMPOSED_CAP_MS);
    } else {
        ledger.jitter_policy = None;
        ledger.jitter_max_ms = None;
        ledger.jitter_composed_cap_ms = None;
    }
}

fn demo_retry_candidate_tag(ack_id: &str) -> String {
    let mut digest = Sha256::new();
    digest.update(b"qshield-demo-retry-candidate-v1");
    digest.update([0u8]);
    digest.update(ack_id.as_bytes());
    let tag = digest.finalize();
    hex::encode(&tag[..6])
}

fn demo_retry_now_ms() -> Result<u64, String> {
    if let Ok(value) = std::env::var("QSHIELD_DEMO_RETRY_CADENCE_NOW_MS") {
        return value
            .parse::<u64>()
            .map_err(|_| "retry cadence test clock invalid".to_string());
    }
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .map_err(|_| "retry cadence clock invalid".to_string())
}

fn demo_retry_enabled() -> bool {
    env_flag("QSHIELD_DEMO_RETRY_CADENCE")
}

fn demo_retry_test_mode() -> bool {
    env_flag("QSHIELD_DEMO_RETRY_CADENCE_TEST_MODE")
}

#[derive(Clone, Copy)]
enum DemoJitterClass {
    EmptyPoll,
    InvalidCandidateRetry,
}

impl DemoJitterClass {
    fn key(self) -> &'static str {
        match self {
            DemoJitterClass::EmptyPoll => "empty_poll",
            DemoJitterClass::InvalidCandidateRetry => "invalid_candidate_retry",
        }
    }
}

fn demo_timing_policy_enabled() -> bool {
    demo_retry_enabled() || demo_jitter_enabled()
}

fn demo_jitter_enabled() -> bool {
    env_flag("QSHIELD_DEMO_BOUNDED_JITTER")
}

fn demo_jitter_test_mode() -> bool {
    env_flag("QSHIELD_DEMO_BOUNDED_JITTER_TEST_MODE")
}

fn demo_jitter_class(class: DemoRetryClass, attempt: u32) -> Option<DemoJitterClass> {
    if !demo_jitter_enabled() {
        return None;
    }

    match class {
        DemoRetryClass::EmptyPoll => Some(DemoJitterClass::EmptyPoll),
        DemoRetryClass::InvalidCandidate if attempt > 1 => {
            Some(DemoJitterClass::InvalidCandidateRetry)
        }
        DemoRetryClass::InvalidCandidate => None,
    }
}

fn demo_jitter_delay_ms(
    class: DemoJitterClass,
    attempt: u32,
    candidate_tag: Option<&str>,
    now_ms: u64,
) -> u64 {
    let mut digest = Sha256::new();
    digest.update(b"qshield-demo-bounded-jitter-v1");
    digest.update([0u8]);
    digest.update(class.key().as_bytes());
    digest.update([0u8]);
    digest.update(attempt.to_le_bytes());
    digest.update([0u8]);
    digest.update(candidate_tag.unwrap_or("no-candidate").as_bytes());
    digest.update([0u8]);
    if demo_jitter_test_mode() {
        digest.update(b"deterministic-test-mode");
    } else {
        digest.update(now_ms.to_le_bytes());
    }
    let bytes = digest.finalize();
    let mut raw = [0u8; 8];
    raw.copy_from_slice(&bytes[..8]);
    u64::from_le_bytes(raw) % (DEMO_JITTER_MAX_MS + 1)
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
