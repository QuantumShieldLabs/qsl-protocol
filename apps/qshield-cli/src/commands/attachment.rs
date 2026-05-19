use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::actor::ActorClient;
use crate::config::{self, Config};
use crate::relay_client::{
    post_json, AckRequest, GenericOk, PollRequest, PollResponse, RelayMsg, SendRequest,
};
use crate::store::{SessionEntry, StoreState};
use crate::util::{load_or_init_state, state_path};

const DESCRIPTOR_TYPE: &str = "qshield_demo_attachment_descriptor";
const ENC_CTX: &str = "suite2_e2e_demo_wire_v1";
const LOCATOR_KIND: &str = "qshield_demo_relay_poll_v1";

#[derive(Debug, Serialize, Deserialize)]
struct DemoAttachmentDescriptor {
    v: u8,
    t: String,
    attachment_id: String,
    filename_hint: String,
    ciphertext_len: usize,
    ciphertext_sha256: String,
    enc_ctx: String,
    locator_kind: String,
    non_production: bool,
}

struct DemoSession {
    cfg: Config,
    my_id: String,
    sess: SessionEntry,
}

pub fn send(
    store_path: &Path,
    peer_id: &str,
    path: &Path,
    demo_unauthenticated_override: bool,
    tamper_ciphertext: bool,
) -> Result<(), String> {
    let session = load_demo_session(store_path, peer_id)?;
    let plaintext = fs::read(path).map_err(|e| format!("read attachment payload: {e}"))?;
    if plaintext.is_empty() {
        return Err("attachment payload is empty".to_string());
    }
    let filename_hint = path
        .file_name()
        .and_then(|v| v.to_str())
        .map(safe_filename)
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| "attachment.bin".to_string());

    let mut actor = spawn_established_actor(&session.sess, demo_unauthenticated_override)?;
    let attachment_wire_hex =
        actor_send_wire(&mut actor, &session.sess.session_id_b64u, &plaintext)?;
    let attachment_wire =
        hex::decode(&attachment_wire_hex).map_err(|e| format!("bad attachment wire hex: {e}"))?;
    let descriptor = DemoAttachmentDescriptor {
        v: 1,
        t: DESCRIPTOR_TYPE.to_string(),
        attachment_id: attachment_id(&session.sess, &attachment_wire),
        filename_hint,
        ciphertext_len: attachment_wire.len(),
        ciphertext_sha256: sha256_hex(&attachment_wire),
        enc_ctx: ENC_CTX.to_string(),
        locator_kind: LOCATOR_KIND.to_string(),
        non_production: true,
    };
    let descriptor_json =
        serde_json::to_vec(&descriptor).map_err(|e| format!("encode descriptor: {e}"))?;
    let descriptor_wire_hex =
        actor_send_wire(&mut actor, &session.sess.session_id_b64u, &descriptor_json)?;

    let queued_attachment_wire_hex = if tamper_ciphertext {
        tamper_hex_wire(&attachment_wire_hex)?
    } else {
        attachment_wire_hex
    };

    let relay_token = config::resolve_relay_token(&session.cfg)?;
    relay_send(
        &session.cfg,
        &relay_token,
        peer_id,
        &session.my_id,
        &descriptor_wire_hex,
    )?;
    relay_send(
        &session.cfg,
        &relay_token,
        peer_id,
        &session.my_id,
        &queued_attachment_wire_hex,
    )?;

    println!(
        "DEMO_ATTACHMENT_DESCRIPTOR_OK id={} ciphertext_len={}",
        short_id(&descriptor.attachment_id),
        descriptor.ciphertext_len
    );
    println!("DEMO_ATTACHMENT_OPAQUE_BOUNDARY_OK");
    if tamper_ciphertext {
        println!("queued tampered demo attachment ciphertext for integrity proof");
    } else {
        println!("queued demo attachment for {peer_id}");
    }
    Ok(())
}

pub fn recv(
    store_path: &Path,
    out_dir: &Path,
    max: u32,
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
    if msgs.len() < 2 {
        return Err("attachment descriptor/ciphertext pair missing".to_string());
    }

    let descriptor_msg = &msgs[0];
    let ciphertext_msg = &msgs[1];
    let descriptor_ack = candidate_ack_id(descriptor_msg)?;
    let ciphertext_ack = candidate_ack_id(ciphertext_msg)?;
    if descriptor_msg.from != ciphertext_msg.from {
        return Err("attachment descriptor/ciphertext sender mismatch".to_string());
    }
    let sess: SessionEntry = state
        .sessions
        .get(&descriptor_msg.from)
        .cloned()
        .ok_or_else(|| "no session for peer".to_string())?;
    let mut actor = spawn_established_actor(&sess, demo_unauthenticated_override)?;

    let descriptor_plain = actor_recv_plain(&mut actor, &sess.session_id_b64u, &descriptor_msg.msg)
        .map_err(|_| "attachment_descriptor_reject".to_string())?;
    let descriptor: DemoAttachmentDescriptor = serde_json::from_slice(&descriptor_plain)
        .map_err(|_| "attachment_descriptor_reject".to_string())?;
    validate_descriptor(&descriptor)?;

    let ciphertext =
        hex::decode(&ciphertext_msg.msg).map_err(|_| "attachment_integrity_reject".to_string())?;
    if ciphertext.len() != descriptor.ciphertext_len
        || sha256_hex(&ciphertext) != descriptor.ciphertext_sha256
    {
        return Err("attachment_integrity_reject".to_string());
    }

    let plaintext = actor_recv_plain(&mut actor, &sess.session_id_b64u, &ciphertext_msg.msg)
        .map_err(|_| "attachment_decrypt_reject".to_string())?;
    ack_candidate(&cfg, &relay_token, &my_id, &descriptor_ack)?;
    ack_candidate(&cfg, &relay_token, &my_id, &ciphertext_ack)?;
    fs::create_dir_all(out_dir).map_err(|e| format!("create attachment output dir: {e}"))?;
    let output_name = safe_filename(&descriptor.filename_hint);
    let output_path = out_dir.join(output_name);
    fs::write(&output_path, &plaintext).map_err(|e| format!("write attachment output: {e}"))?;

    println!(
        "DEMO_ATTACHMENT_FETCH_DECRYPT_OK id={} path={}",
        short_id(&descriptor.attachment_id),
        output_path.display()
    );
    println!("DEMO_ATTACHMENT_OPAQUE_BOUNDARY_OK");
    Ok(())
}

fn candidate_ack_id(msg: &RelayMsg) -> Result<String, String> {
    msg.ack_id
        .clone()
        .ok_or_else(|| "relay candidate missing ack".to_string())
}

fn ack_candidate(cfg: &Config, relay_token: &str, id: &str, ack_id: &str) -> Result<(), String> {
    let req = AckRequest {
        id: id.to_string(),
        ack_id: ack_id.to_string(),
    };
    let resp: GenericOk = post_json(&cfg.relay_url, "/ack", &req, relay_token)?;
    if !resp.ok {
        return Err("relay ack failed".to_string());
    }
    Ok(())
}

fn load_demo_session(store_path: &Path, peer_id: &str) -> Result<DemoSession, String> {
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
    let sess: SessionEntry = state.sessions.get(peer_id).cloned().ok_or_else(|| {
        "session missing; run: qshield establish --store <path> --peer <peer_id>".to_string()
    })?;
    Ok(DemoSession { cfg, my_id, sess })
}

fn spawn_established_actor(
    sess: &SessionEntry,
    demo_unauthenticated_override: bool,
) -> Result<ActorClient, String> {
    let actor_path = std::env::var("QSHIELD_ACTOR")
        .unwrap_or_else(|_| "target/release/refimpl_actor".to_string());
    let mut actor = ActorClient::spawn(&actor_path)?;
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
    Ok(actor)
}

fn actor_send_wire(
    actor: &mut ActorClient,
    session_id_b64u: &str,
    plaintext: &[u8],
) -> Result<String, String> {
    let send_params = serde_json::json!({
        "negotiated": {
            "protocol_version": 1280,
            "suite_id": 2
        },
        "session_id": session_id_b64u,
        "plaintext_hex": hex::encode(plaintext),
        "flags": { "u16": 0 }
    });
    let result = actor.call("suite2.e2e.send", send_params)?;
    result
        .get("wire_hex")
        .and_then(|v| v.get("data"))
        .and_then(|v| v.as_str())
        .map(|v| v.to_string())
        .ok_or_else(|| "actor response missing wire_hex".to_string())
}

fn actor_recv_plain(
    actor: &mut ActorClient,
    session_id_b64u: &str,
    wire_hex: &str,
) -> Result<Vec<u8>, String> {
    let recv_params = serde_json::json!({
        "negotiated": {
            "protocol_version": 1280,
            "suite_id": 2
        },
        "session_id": session_id_b64u,
        "wire_hex": wire_hex
    });
    let result = actor.call("suite2.e2e.recv", recv_params)?;
    let pt_hex = result
        .get("plaintext_hex")
        .and_then(|v| v.get("data"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| "actor response missing plaintext_hex".to_string())?;
    hex::decode(pt_hex).map_err(|e| format!("bad plaintext hex: {e}"))
}

fn relay_send(
    cfg: &Config,
    relay_token: &str,
    to: &str,
    from: &str,
    msg: &str,
) -> Result<(), String> {
    let req = SendRequest {
        to: to.to_string(),
        from: from.to_string(),
        msg: msg.to_string(),
        pad_len: None,
        bucket: None,
    };
    let resp: GenericOk = post_json(&cfg.relay_url, "/send", &req, relay_token)?;
    if !resp.ok {
        return Err("relay send failed".to_string());
    }
    Ok(())
}

fn validate_descriptor(desc: &DemoAttachmentDescriptor) -> Result<(), String> {
    if desc.v != 1
        || desc.t != DESCRIPTOR_TYPE
        || desc.enc_ctx != ENC_CTX
        || desc.locator_kind != LOCATOR_KIND
        || !desc.non_production
        || desc.attachment_id.len() != 64
        || desc.ciphertext_sha256.len() != 64
        || desc.ciphertext_len == 0
    {
        return Err("attachment_descriptor_reject".to_string());
    }
    Ok(())
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    hex::encode(h.finalize())
}

fn attachment_id(sess: &SessionEntry, wire: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(b"qshield-demo-attachment-v1");
    h.update(sess.session_id_hex.as_bytes());
    h.update(wire);
    hex::encode(h.finalize())
}

fn safe_filename(raw: &str) -> String {
    let mut out = String::new();
    for ch in raw.chars().take(96) {
        if ch.is_ascii_alphanumeric() || matches!(ch, '.' | '-' | '_') {
            out.push(ch);
        } else {
            out.push('_');
        }
    }
    if out.is_empty() || out == "." || out == ".." || out.starts_with('.') {
        "attachment.bin".to_string()
    } else {
        out
    }
}

fn short_id(id: &str) -> &str {
    id.get(..12).unwrap_or(id)
}

fn tamper_hex_wire(wire_hex: &str) -> Result<String, String> {
    let mut bytes = hex::decode(wire_hex).map_err(|e| format!("bad wire hex: {e}"))?;
    let Some(last) = bytes.last_mut() else {
        return Err("attachment ciphertext is empty".to_string());
    };
    *last ^= 0x01;
    Ok(hex::encode(bytes))
}
