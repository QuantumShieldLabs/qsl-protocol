use std::path::Path;

use crate::actor::ActorClient;
use crate::config::{self, Config};
use crate::relay_client::{
    get_json, post_json, post_json_allow_status, BundleResponse, ConsumeRequest,
    EstablishRecordRequest, EstablishRecordResponse, GenericOk,
};
use crate::store::{SessionEntry, StoreState};
use crate::util::{
    demo_dh_init_bytes, demo_pq_init_bytes, demo_session_id_bytes, load_or_init_state, save_state,
    state_path,
};

pub fn run(
    store_path: &Path,
    peer_id: &str,
    role_override: Option<String>,
    demo_unauthenticated_override: bool,
    demo_identity_verified: bool,
) -> Result<(), String> {
    let cfg_path = store_path.join(config::CONFIG_FILE_NAME);
    let cfg: Config = config::read_config(&cfg_path).map_err(|_| {
        format!(
            "config missing or invalid: {} (run: qshield init --store <path>)",
            cfg_path.display()
        )
    })?;

    let state_path = state_path(store_path);
    let mut state: StoreState = load_or_init_state(&state_path)?;
    if state.sessions.is_empty() && !demo_identity_verified {
        eprintln!(
            "warning: verify peer identity out-of-band before first establish (demo-only); pass --demo-identity-verified to suppress"
        );
    }
    let my_id = state.my_id.clone().ok_or_else(|| {
        "identity missing; run: qshield register --store <path> --id <id>".to_string()
    })?;
    let dh_self_pub_hex = state.dh_pub_hex.clone().ok_or_else(|| {
        "identity missing; run: qshield register --store <path> --id <id>".to_string()
    })?;

    let relay_token = config::resolve_relay_token(&cfg)?;
    let bundle_resp: BundleResponse =
        get_json(&cfg.relay_url, &format!("/bundle/{peer_id}"), &relay_token)?;
    if !bundle_resp.ok {
        return Err(format!("relay bundle lookup failed for {peer_id}"));
    }
    let bundle = bundle_resp
        .bundle
        .ok_or_else(|| "peer bundle missing".to_string())?;
    let bundle_id = bundle
        .get("id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "peer bundle missing identity binding (bundle.id)".to_string())?;
    if bundle_id != peer_id {
        return Err(format!(
            "peer bundle identity mismatch: expected {peer_id} got {bundle_id}"
        ));
    }
    let dh_peer_pub_hex = bundle
        .get("dh_pub")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "peer bundle missing dh_pub".to_string())?;
    let pq_kem_pub_id_hex = bundle
        .get("pq_kem_pub_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "peer bundle missing pq_kem_pub_id".to_string())?;
    let pq_prekey_id = bundle
        .get("pq_prekey_id")
        .and_then(|v| v.as_u64())
        .ok_or_else(|| "peer bundle missing pq_prekey_id".to_string())?;
    if pq_prekey_id > u32::MAX as u64 {
        return Err("peer bundle pq_prekey_id out of range".to_string());
    }

    let dh_self_pub = hex::decode(&dh_self_pub_hex).map_err(|e| format!("bad dh_self_pub: {e}"))?;
    let dh_peer_pub = hex::decode(dh_peer_pub_hex).map_err(|e| format!("bad dh_peer_pub: {e}"))?;
    if dh_self_pub.len() != 32 || dh_peer_pub.len() != 32 {
        return Err("dh_pub must be 32 bytes".to_string());
    }

    let role = role_override.unwrap_or_else(|| {
        if my_id.as_str() <= peer_id {
            "A".to_string()
        } else {
            "B".to_string()
        }
    });
    if role != "A" && role != "B" {
        return Err("role must be A or B".to_string());
    }

    let session_id = demo_session_id_bytes(&my_id, peer_id);
    let session_id_hex = hex::encode(session_id);
    let dh_init = demo_dh_init_bytes(&my_id, peer_id, &dh_self_pub, &dh_peer_pub);
    let pq_init_ss = demo_pq_init_bytes(&my_id, peer_id, &dh_self_pub, &dh_peer_pub);

    let actor_path = std::env::var("QSHIELD_ACTOR")
        .unwrap_or_else(|_| "target/release/refimpl_actor".to_string());
    let mut actor = ActorClient::spawn(&actor_path)?;

    if demo_unauthenticated_override {
        eprintln!("warning: unauthenticated establish override enabled (demo-only)");
    }

    let params = serde_json::json!({
        "msg_type": { "u16": 1 },
        "negotiated": {
            "protocol_version": 1280,
            "suite_id": 2
        },
        "bound": {
            "protocol_version": 1280,
            "suite_id": 2,
            "pq_kem_pub_id": pq_kem_pub_id_hex,
            "pq_prekey_id": pq_prekey_id
        },
        "session_id": session_id_hex,
        "dh_init": hex::encode(dh_init),
        "pq_init_ss": hex::encode(pq_init_ss),
        "pq_kem_pub_id": pq_kem_pub_id_hex,
        "pq_prekey_id": { "u32": pq_prekey_id },
        "dh_self_pub": hex::encode(&dh_self_pub),
        "dh_peer_pub": hex::encode(&dh_peer_pub),
        "authenticated": { "bool": demo_unauthenticated_override },
        "role": role,
    });

    let result = actor.call("suite2.establish.run", params)?;
    let sid_b64u = result
        .get("session_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "missing session_id in actor response".to_string())?
        .to_string();

    let record_req = EstablishRecordRequest {
        peer_id: peer_id.to_string(),
        bundle_id: bundle_id.to_string(),
        session_id_hex: session_id_hex.clone(),
        dh_init: hex::encode(&dh_init),
        pq_init_ss: hex::encode(&pq_init_ss),
    };
    let (record_status, record_resp): (u16, EstablishRecordResponse) = post_json_allow_status(
        &cfg.relay_url,
        "/establish_record",
        &record_req,
        &relay_token,
    )?;
    if record_status == 409 || !record_resp.ok {
        let msg = record_resp
            .error
            .unwrap_or_else(|| "establish replay".to_string());
        return Err(format!("relay establish replay: {msg}"));
    }
    if record_status != 200 {
        return Err(format!(
            "relay establish record failed: status {record_status}"
        ));
    }

    let consume_req = ConsumeRequest {
        id: peer_id.to_string(),
    };
    let consume_resp: GenericOk =
        post_json(&cfg.relay_url, "/consume", &consume_req, &relay_token)?;
    if !consume_resp.ok {
        return Err("relay bundle consume failed".to_string());
    }

    state.sessions.insert(
        peer_id.to_string(),
        SessionEntry {
            session_id_b64u: sid_b64u.clone(),
            session_id_hex: hex::encode(session_id),
            role,
            dh_init_hex: hex::encode(dh_init),
            pq_init_ss_hex: hex::encode(pq_init_ss),
            pq_kem_pub_id_hex: pq_kem_pub_id_hex.to_string(),
            pq_prekey_id: pq_prekey_id as u32,
            dh_self_pub_hex,
            dh_peer_pub_hex: dh_peer_pub_hex.to_string(),
        },
    );
    save_state(&state_path, &state)?;

    println!("established session with {peer_id}: session_id={sid_b64u}");
    Ok(())
}
