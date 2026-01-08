use std::path::Path;

use crate::actor::ActorClient;
use crate::config::{self, Config};
use crate::relay_client::{post_json, GenericOk, SendRequest};
use crate::store::{StoreState, SessionEntry};
use crate::util::{load_or_init_state, state_path};

pub fn run(
    store_path: &Path,
    peer_id: &str,
    text: &str,
    demo_unauthenticated_override: bool,
) -> Result<(), String> {
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
    let sess: SessionEntry = state
        .sessions
        .get(peer_id)
        .cloned()
        .ok_or_else(|| "session missing; run: qshield establish --store <path> --peer <peer_id>".to_string())?;

    let actor_path = std::env::var("QSHIELD_ACTOR").unwrap_or_else(|_| "target/release/refimpl_actor".to_string());
    let mut actor = ActorClient::spawn(&actor_path)?;

    if demo_unauthenticated_override {
        eprintln!("warning: unauthenticated establish override enabled (demo-only)");
    }

    // Re-establish session in actor for this command (demo-only).
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

    let send_params = serde_json::json!({
        "negotiated": {
            "protocol_version": 1280,
            "suite_id": 2
        },
        "session_id": sess.session_id_b64u,
        "plaintext_hex": hex::encode(text.as_bytes()),
        "flags": { "u16": 0 }
    });
    let result = actor.call("suite2.e2e.send", send_params)?;
    let wire_hex = result
        .get("wire_hex")
        .and_then(|v| v.get("data"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| "actor response missing wire_hex".to_string())?
        .to_string();

    let mut wire_bytes = hex::decode(&wire_hex).map_err(|e| format!("bad wire_hex: {e}"))?;
    let mut pad_len: Option<u32> = None;
    let mut bucket: Option<u32> = None;
    if cfg.padding_enabled {
        let buckets = cfg
            .padding_buckets
            .as_ref()
            .ok_or_else(|| "padding enabled but no buckets configured".to_string())?;
        let ct_len = wire_bytes.len() as u32;
        let mut chosen: Option<u32> = None;
        for b in buckets {
            if *b >= ct_len {
                chosen = Some(*b);
                break;
            }
        }
        let bucket_size = chosen.ok_or_else(|| "no padding bucket large enough for ciphertext".to_string())?;
        let pad = bucket_size.saturating_sub(ct_len);
        wire_bytes.extend(std::iter::repeat(0u8).take(pad as usize));
        pad_len = Some(pad);
        bucket = Some(bucket_size);
    }
    let wire_hex = hex::encode(&wire_bytes);

    let req = SendRequest {
        to: peer_id.to_string(),
        from: my_id,
        msg: wire_hex,
        pad_len,
        bucket,
    };
    let relay_token = config::resolve_relay_token(&cfg)?;
    let resp: GenericOk = post_json(&cfg.relay_url, "/send", &req, &relay_token)?;
    if !resp.ok {
        return Err("relay send failed".to_string());
    }

    println!("sent message to {peer_id}");
    Ok(())
}
