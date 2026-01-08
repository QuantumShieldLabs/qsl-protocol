use std::path::Path;

use crate::actor::ActorClient;
use crate::config::{self, Config};
use crate::relay_client::{post_json, PollRequest, PollResponse};
use crate::store::{StoreState, SessionEntry};
use crate::util::{load_or_init_state, state_path};

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
    let my_id = state
        .my_id
        .clone()
        .ok_or_else(|| "identity missing; run: qshield register --store <path> --id <id>".to_string())?;

    let relay_token = config::resolve_relay_token(&cfg)?;
    let poll = PollRequest {
        id: my_id.clone(),
        max,
    };
    let resp: PollResponse = post_json(&cfg.relay_url, "/poll", &poll, &relay_token)?;
    if !resp.ok {
        return Err("relay poll failed".to_string());
    }
    let msgs = resp.msgs.unwrap_or_default();
    if msgs.is_empty() {
        println!("no messages");
        return Ok(());
    }

    let actor_path =
        std::env::var("QSHIELD_ACTOR").unwrap_or_else(|_| "target/release/refimpl_actor".to_string());
    let mut actor = ActorClient::spawn(&actor_path)?;

    for msg in msgs {
        let sess: SessionEntry = state
            .sessions
            .get(&msg.from)
            .cloned()
            .ok_or_else(|| format!("no session for peer {}", msg.from))?;

        if demo_unauthenticated_override {
            eprintln!("warning: unauthenticated establish override enabled (demo-only)");
        }

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

        let mut wire_bytes =
            hex::decode(&msg.msg).map_err(|e| format!("bad wire hex: {e}"))?;
        let pad_len = msg.pad_len.unwrap_or(0) as usize;
        if pad_len > wire_bytes.len() {
            return Err("pad_len exceeds message length".to_string());
        }
        if let Some(bucket) = msg.bucket {
            if wire_bytes.len() != bucket as usize {
                return Err("bucket size mismatch".to_string());
            }
        }
        if pad_len > 0 {
            let new_len = wire_bytes.len() - pad_len;
            wire_bytes.truncate(new_len);
        }
        let wire_hex = hex::encode(&wire_bytes);

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
        println!("from {}: {}", msg.from, text);
    }

    Ok(())
}
