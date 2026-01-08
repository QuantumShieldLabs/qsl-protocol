use std::collections::{BTreeSet, HashMap};
use std::fs::{self, OpenOptions};
use std::path::{Path, PathBuf};
use std::io::{self, BufRead, Write};

use base64::Engine;
use clap::Parser;
use rand_chacha::ChaCha20Rng;
use rand_core::{RngCore, SeedableRng};
use serde::{Deserialize, Serialize};
use sha3::{Digest as Sha3Digest, Sha3_256};

use ed25519_dalek::{Signer as _, SigningKey};

use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret};

use ml_dsa::{MlDsa65, Signature as MlDsaSig, SigningKey as MlDsaSk, VerifyingKey as MlDsaVk};
use ml_dsa::KeyGen as _;
use ml_dsa::signature::Verifier as _;

use ml_kem::{B32, EncapsulateDeterministic, Encoded, EncodedSizeUser, KemCore, MlKem768};
use ml_kem::kem::{DecapsulationKey as MlKemDk, EncapsulationKey as MlKemEk, Decapsulate as _};

use quantumshield_refimpl::codec::Writer;
use quantumshield_refimpl::crypto::stdcrypto::StdCrypto;

use quantumshield_refimpl::crypto::traits::Kmac as _;

// ---------------------------
// Suite-2 KDF helpers (QSP v5.0 / suite_id 0x0002)
// ---------------------------

fn hex_val(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(10 + (c - b'a')),
        b'A'..=b'F' => Some(10 + (c - b'A')),
        _ => None,
    }
}

fn hex_decode(s: &str) -> Result<Vec<u8>, ActorError> {
    let b = s.as_bytes();
    if b.len() % 2 != 0 {
        return Err(ActorError::Invalid("hex string must have even length".into()));
    }
    let mut out = Vec::with_capacity(b.len() / 2);
    let mut i = 0usize;
    while i < b.len() {
        let hi = hex_val(b[i]).ok_or_else(|| ActorError::Invalid("invalid hex".into()))?;
        let lo = hex_val(b[i + 1]).ok_or_else(|| ActorError::Invalid("invalid hex".into()))?;
        out.push((hi << 4) | lo);
        i += 2;
    }
    Ok(out)
}

fn get_bytes(params: &serde_json::Value, key: &str) -> Result<Vec<u8>, ActorError> {
    let v = params.get(key).ok_or_else(|| ActorError::Invalid(format!("missing params.{key}")))?;
    // Accept:
    // - string: hex
    // - object: { "type": "hex", "data": "<hex>" }
    if let Some(s) = v.as_str() {
        return hex_decode(s);
    }
    if let Some(obj) = v.as_object() {
        if let Some(t) = obj.get("type").and_then(|x| x.as_str()) {
            if t == "hex" {
                let d = obj.get("data").and_then(|x| x.as_str())
                    .ok_or_else(|| ActorError::Invalid(format!("params.{key}: hex object missing data")))?;
                return hex_decode(d);
            }
        }
    }
    Err(ActorError::Invalid(format!("params.{key}: expected hex string or {{type:'hex',data:'..'}}")))
}

fn get_u32(params: &serde_json::Value, key: &str) -> Result<u32, ActorError> {
    let v = params
        .get(key)
        .ok_or_else(|| ActorError::Invalid(format!("missing params.{key}")))?;

    // Direct number
    if let Some(n) = v.as_u64() {
        if n <= u32::MAX as u64 {
            return Ok(n as u32);
        }
    }

    if let Some(obj) = v.as_object() {
        // Direct object forms: {"u32": N} / {"value": N} / {"n": N}
        if let Some(n) = obj
            .get("u32")
            .and_then(|x| x.as_u64())
            .or_else(|| obj.get("value").and_then(|x| x.as_u64()))
            .or_else(|| obj.get("n").and_then(|x| x.as_u64()))
        {
            if n <= u32::MAX as u64 {
                return Ok(n as u32);
            }
        }

        // Typed wrapper: {"type":"json","data": ...}
        if obj.get("type").and_then(|x| x.as_str()) == Some("json") {
            if let Some(data) = obj.get("data") {
                // Legacy scalar form (kept for compatibility)
                if let Some(n) = data.as_u64() {
                    if n <= u32::MAX as u64 {
                        return Ok(n as u32);
                    }
                }

                // Schema-compliant object encoding: {"u32": N} / {"value": N} / {"n": N}
                if let Some(dobj) = data.as_object() {
                    if let Some(n) = dobj
                        .get("u32")
                        .and_then(|x| x.as_u64())
                        .or_else(|| dobj.get("value").and_then(|x| x.as_u64()))
                        .or_else(|| dobj.get("n").and_then(|x| x.as_u64()))
                    {
                        if n <= u32::MAX as u64 {
                            return Ok(n as u32);
                        }
                    }
                }

                // Array encoding: [N]
                if let Some(arr) = data.as_array() {
                    if arr.len() == 1 {
                        if let Some(n) = arr[0].as_u64() {
                            if n <= u32::MAX as u64 {
                                return Ok(n as u32);
                            }
                        }
                    }
                }
            }
        }
    }

    Err(ActorError::Invalid(format!(
        "params.{key}: expected u32 (number, {{u32:N}}, or {{type:'json',data:{{u32:N}}}})"
    )))
}

fn get_u16(params: &serde_json::Value, key: &str) -> Result<u16, ActorError> {
    let v = get_json_data(params, key)?;
    if let Some(obj) = v.as_object() {
        if let Some(n) = obj
            .get("u16")
            .and_then(|x| x.as_u64())
            .or_else(|| obj.get("value").and_then(|x| x.as_u64()))
            .or_else(|| obj.get("n").and_then(|x| x.as_u64()))
        {
            if n <= u16::MAX as u64 {
                return Ok(n as u16);
            }
        }
    }
    parse_u16(&v, &format!("params.{key}"))
}

fn parse_hex_list(v: &serde_json::Value, what: &str) -> Result<Vec<String>, ActorError> {
    let arr = v
        .as_array()
        .ok_or_else(|| ActorError::Invalid(format!("{what}: expected array")))?;
    let mut out = Vec::with_capacity(arr.len());
    for item in arr {
        if let Some(s) = item.as_str() {
            out.push(s.to_string());
            continue;
        }
        if let Some(obj) = item.as_object() {
            if obj.get("type").and_then(|x| x.as_str()) == Some("hex") {
                if let Some(d) = obj.get("data").and_then(|x| x.as_str()) {
                    out.push(d.to_string());
                    continue;
                }
            }
        }
        return Err(ActorError::Invalid(format!(
            "{what}: expected hex string or {{type:'hex',data:'..'}}"
        )));
    }
    Ok(out)
}

fn parse_u32_list(v: &serde_json::Value, what: &str) -> Result<Vec<u32>, ActorError> {
    let arr = v
        .as_array()
        .ok_or_else(|| ActorError::Invalid(format!("{what}: expected array")))?;
    let mut out = Vec::with_capacity(arr.len());
    for item in arr {
        if let Some(n) = item.as_u64() {
            if n <= u32::MAX as u64 {
                out.push(n as u32);
                continue;
            }
        }
        if let Some(obj) = item.as_object() {
            if let Some(n) = obj
                .get("u32")
                .and_then(|x| x.as_u64())
                .or_else(|| obj.get("value").and_then(|x| x.as_u64()))
                .or_else(|| obj.get("n").and_then(|x| x.as_u64()))
            {
                if n <= u32::MAX as u64 {
                    out.push(n as u32);
                    continue;
                }
            }
        }
        return Err(ActorError::Invalid(format!("{what}: expected u32 array")));
    }
    Ok(out)
}

fn parse_bool(v: &serde_json::Value, what: &str) -> Result<bool, ActorError> {
    if let Some(b) = v.as_bool() {
        return Ok(b);
    }
    if let Some(obj) = v.as_object() {
        if let Some(b) = obj.get("bool").and_then(|x| x.as_bool()) {
            return Ok(b);
        }
    }
    Err(ActorError::Invalid(format!("{what}: expected bool")))
}


fn get_json_data(params: &serde_json::Value, key: &str) -> Result<serde_json::Value, ActorError> {
    let v = params.get(key).ok_or_else(|| ActorError::Invalid(format!("missing params.{key}")))?;
    if let Some(obj) = v.as_object() {
        if obj.get("type").and_then(|x| x.as_str()) == Some("json") {
            return Ok(obj.get("data").cloned().unwrap_or(serde_json::Value::Null));
        }
    }
    Ok(v.clone())
}

fn parse_u16(v: &serde_json::Value, what: &str) -> Result<u16, ActorError> {
    if let Some(n) = v.as_u64() {
        if n <= u16::MAX as u64 { return Ok(n as u16); }
    }
    if let Some(s) = v.as_str() {
        let ss = s.trim();
        if let Some(hex) = ss.strip_prefix("0x").or_else(|| ss.strip_prefix("0X")) {
            if let Ok(n) = u16::from_str_radix(hex, 16) {
                return Ok(n);
            }
        }
        if let Ok(n) = ss.parse::<u16>() {
            return Ok(n);
        }
    }
    Err(ActorError::Invalid(format!("{what}: expected u16 (number or hex string)")))
}

fn parse_u32_value(v: &serde_json::Value, what: &str) -> Result<u32, ActorError> {
    if let Some(n) = v.as_u64() {
        if n <= u32::MAX as u64 { return Ok(n as u32); }
    }
    if let Some(s) = v.as_str() {
        let ss = s.trim();
        if let Some(hex) = ss.strip_prefix("0x").or_else(|| ss.strip_prefix("0X")) {
            if let Ok(n) = u32::from_str_radix(hex, 16) {
                return Ok(n);
            }
        }
        if let Ok(n) = ss.parse::<u32>() {
            return Ok(n);
        }
    }
    if let Some(obj) = v.as_object() {
        if let Some(n) = obj
            .get("u32")
            .and_then(|x| x.as_u64())
            .or_else(|| obj.get("value").and_then(|x| x.as_u64()))
            .or_else(|| obj.get("n").and_then(|x| x.as_u64()))
        {
            if n <= u32::MAX as u64 {
                return Ok(n as u32);
            }
        }
    }
    Err(ActorError::Invalid(format!("{what}: expected u32")))
}

fn parse_hex_value(v: &serde_json::Value, what: &str) -> Result<Vec<u8>, ActorError> {
    if let Some(s) = v.as_str() {
        return hex_decode(s);
    }
    if let Some(obj) = v.as_object() {
        if obj.get("type").and_then(|x| x.as_str()) == Some("hex") {
            if let Some(d) = obj.get("data").and_then(|x| x.as_str()) {
                return hex_decode(d);
            }
        }
    }
    Err(ActorError::Invalid(format!("{what}: expected hex string")))
}

fn suite_kind_str(suite: SuiteKind) -> &'static str {
    match suite {
        SuiteKind::Suite1 => "Suite-1",
        SuiteKind::Suite1B => "Suite-1B",
        SuiteKind::Suite2 => "Suite-2",
    }
}

fn parse_suite2_send_state(v: &serde_json::Value) -> Result<suite2_ratchet::Suite2SendState, ActorError> {
    let obj = v.as_object().ok_or_else(|| ActorError::Invalid("params.send_state: expected object".into()))?;
    let get_field = |k: &str| -> Result<&serde_json::Value, ActorError> {
        obj.get(k).ok_or_else(|| ActorError::Invalid(format!("params.send_state.{k} missing")))
    };

    let session_id = parse_hex_value(get_field("session_id")?, "params.send_state.session_id")?;
    if session_id.len() != 16 {
        return Err(ActorError::Invalid("params.send_state.session_id: expected 16 bytes".into()));
    }
    let mut session_id_arr = [0u8; 16];
    session_id_arr.copy_from_slice(&session_id);

    let dh_pub = parse_hex_value(get_field("dh_pub")?, "params.send_state.dh_pub")?;
    if dh_pub.len() != 32 {
        return Err(ActorError::Invalid("params.send_state.dh_pub: expected 32 bytes".into()));
    }
    let mut dh_pub_arr = [0u8; 32];
    dh_pub_arr.copy_from_slice(&dh_pub);

    let hk_s = parse_hex_value(get_field("hk_s")?, "params.send_state.hk_s")?;
    if hk_s.len() != 32 {
        return Err(ActorError::Invalid("params.send_state.hk_s: expected 32 bytes".into()));
    }
    let mut hk_s_arr = [0u8; 32];
    hk_s_arr.copy_from_slice(&hk_s);

    let ck_ec = parse_hex_value(get_field("ck_ec")?, "params.send_state.ck_ec")?;
    if ck_ec.len() != 32 {
        return Err(ActorError::Invalid("params.send_state.ck_ec: expected 32 bytes".into()));
    }
    let mut ck_ec_arr = [0u8; 32];
    ck_ec_arr.copy_from_slice(&ck_ec);

    let ck_pq = parse_hex_value(get_field("ck_pq")?, "params.send_state.ck_pq")?;
    if ck_pq.len() != 32 {
        return Err(ActorError::Invalid("params.send_state.ck_pq: expected 32 bytes".into()));
    }
    let mut ck_pq_arr = [0u8; 32];
    ck_pq_arr.copy_from_slice(&ck_pq);

    let ns = parse_u32_value(get_field("ns")?, "params.send_state.ns")?;
    let pn = parse_u32_value(get_field("pn")?, "params.send_state.pn")?;

    Ok(suite2_ratchet::Suite2SendState {
        session_id: session_id_arr,
        protocol_version: suite2_types::SUITE2_PROTOCOL_VERSION,
        suite_id: suite2_types::SUITE2_SUITE_ID,
        dh_pub: dh_pub_arr,
        hk_s: hk_s_arr,
        ck_ec: ck_ec_arr,
        ck_pq: ck_pq_arr,
        ns,
        pn,
    })
}

fn parse_suite2_recv_state(v: &serde_json::Value) -> Result<suite2_ratchet::Suite2RecvWireState, ActorError> {
    let obj = v.as_object().ok_or_else(|| ActorError::Invalid("params.recv_state: expected object".into()))?;
    let get_field = |k: &str| -> Result<&serde_json::Value, ActorError> {
        obj.get(k).ok_or_else(|| ActorError::Invalid(format!("params.recv_state.{k} missing")))
    };

    let session_id = parse_hex_value(get_field("session_id")?, "params.recv_state.session_id")?;
    if session_id.len() != 16 {
        return Err(ActorError::Invalid("params.recv_state.session_id: expected 16 bytes".into()));
    }
    let mut session_id_arr = [0u8; 16];
    session_id_arr.copy_from_slice(&session_id);

    let dh_pub = parse_hex_value(get_field("dh_pub")?, "params.recv_state.dh_pub")?;
    if dh_pub.len() != 32 {
        return Err(ActorError::Invalid("params.recv_state.dh_pub: expected 32 bytes".into()));
    }
    let mut dh_pub_arr = [0u8; 32];
    dh_pub_arr.copy_from_slice(&dh_pub);

    let hk_r = parse_hex_value(get_field("hk_r")?, "params.recv_state.hk_r")?;
    if hk_r.len() != 32 {
        return Err(ActorError::Invalid("params.recv_state.hk_r: expected 32 bytes".into()));
    }
    let mut hk_r_arr = [0u8; 32];
    hk_r_arr.copy_from_slice(&hk_r);

    let rk = parse_hex_value(get_field("rk")?, "params.recv_state.rk")?;
    if rk.len() != 32 {
        return Err(ActorError::Invalid("params.recv_state.rk: expected 32 bytes".into()));
    }
    let mut rk_arr = [0u8; 32];
    rk_arr.copy_from_slice(&rk);

    let ck_ec = parse_hex_value(get_field("ck_ec")?, "params.recv_state.ck_ec")?;
    if ck_ec.len() != 32 {
        return Err(ActorError::Invalid("params.recv_state.ck_ec: expected 32 bytes".into()));
    }
    let mut ck_ec_arr = [0u8; 32];
    ck_ec_arr.copy_from_slice(&ck_ec);

    let ck_pq_send = parse_hex_value(get_field("ck_pq_send")?, "params.recv_state.ck_pq_send")?;
    if ck_pq_send.len() != 32 {
        return Err(ActorError::Invalid("params.recv_state.ck_pq_send: expected 32 bytes".into()));
    }
    let mut ck_pq_send_arr = [0u8; 32];
    ck_pq_send_arr.copy_from_slice(&ck_pq_send);

    let ck_pq_recv = parse_hex_value(get_field("ck_pq_recv")?, "params.recv_state.ck_pq_recv")?;
    if ck_pq_recv.len() != 32 {
        return Err(ActorError::Invalid("params.recv_state.ck_pq_recv: expected 32 bytes".into()));
    }
    let mut ck_pq_recv_arr = [0u8; 32];
    ck_pq_recv_arr.copy_from_slice(&ck_pq_recv);

    let nr = parse_u32_value(get_field("nr")?, "params.recv_state.nr")?;

    let role_v = get_field("role")?;
    let role_is_a = role_v.as_str().map(|s| s == "A").ok_or_else(|| ActorError::Invalid("params.recv_state.role: expected \"A\" or \"B\"".into()))?;

    let peer_max_adv_id_seen = parse_u32_value(get_field("peer_max_adv_id_seen")?, "params.recv_state.peer_max_adv_id_seen")?;

    let known_v = get_field("known_targets")?;
    let known_targets: BTreeSet<u32> = parse_u32_list(known_v, "params.recv_state.known_targets")?.into_iter().collect();
    let consumed_v = get_field("consumed_targets")?;
    let consumed_targets: BTreeSet<u32> = parse_u32_list(consumed_v, "params.recv_state.consumed_targets")?.into_iter().collect();
    let tomb_v = get_field("tombstoned_targets")?;
    let tombstoned_targets: BTreeSet<u32> = parse_u32_list(tomb_v, "params.recv_state.tombstoned_targets")?.into_iter().collect();

    let mks_v = get_field("mkskipped")?;
    let mks_arr = mks_v.as_array().ok_or_else(|| ActorError::Invalid("params.recv_state.mkskipped: expected array".into()))?;
    let mut mkskipped = Vec::new();
    for (idx, entry) in mks_arr.iter().enumerate() {
        let obj = entry.as_object().ok_or_else(|| ActorError::Invalid(format!("params.recv_state.mkskipped[{idx}]: expected object")))?;
        let dh_pub = parse_hex_value(obj.get("dh_pub").ok_or_else(|| ActorError::Invalid(format!("params.recv_state.mkskipped[{idx}].dh_pub missing")))?, "params.recv_state.mkskipped.dh_pub")?;
        if dh_pub.len() != 32 {
            return Err(ActorError::Invalid("params.recv_state.mkskipped.dh_pub: expected 32 bytes".into()));
        }
        let mut dh_pub_arr = [0u8; 32];
        dh_pub_arr.copy_from_slice(&dh_pub);
        let n = parse_u32_value(obj.get("n").ok_or_else(|| ActorError::Invalid(format!("params.recv_state.mkskipped[{idx}].n missing")))?, "params.recv_state.mkskipped.n")?;
        let mk = parse_hex_value(obj.get("mk").ok_or_else(|| ActorError::Invalid(format!("params.recv_state.mkskipped[{idx}].mk missing")))?, "params.recv_state.mkskipped.mk")?;
        if mk.len() != 32 {
            return Err(ActorError::Invalid("params.recv_state.mkskipped.mk: expected 32 bytes".into()));
        }
        let mut mk_arr = [0u8; 32];
        mk_arr.copy_from_slice(&mk);
        mkskipped.push(suite2_ratchet::MkSkippedEntry { dh_pub: dh_pub_arr, n, mk: mk_arr });
    }

    Ok(suite2_ratchet::Suite2RecvWireState {
        session_id: session_id_arr,
        protocol_version: suite2_types::SUITE2_PROTOCOL_VERSION,
        suite_id: suite2_types::SUITE2_SUITE_ID,
        dh_pub: dh_pub_arr,
        hk_r: hk_r_arr,
        rk: rk_arr,
        ck_ec: ck_ec_arr,
        ck_pq_send: ck_pq_send_arr,
        ck_pq_recv: ck_pq_recv_arr,
        nr,
        role_is_a,
        peer_max_adv_id_seen,
        known_targets,
        consumed_targets,
        tombstoned_targets,
        mkskipped,
    })
}

fn jhex(b: &[u8]) -> serde_json::Value {
    serde_json::json!({ "type": "hex", "data": to_hex(b) })
}

fn to_hex(b: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut s = String::with_capacity(b.len() * 2);
    for &x in b {
        s.push(HEX[(x >> 4) as usize] as char);
        s.push(HEX[(x & 0x0f) as usize] as char);
    }
    s
}

fn kmac32(std: &StdCrypto, key: &[u8], label: &str, data: &[u8]) -> Vec<u8> {
    std.kmac256(key, label, data, 32)
}

fn kmac64(std: &StdCrypto, key: &[u8], label: &str, data: &[u8]) -> Vec<u8> {
    std.kmac256(key, label, data, 64)
}
use quantumshield_refimpl::crypto::traits::{
    Aead, CryptoError, Hash, PqKem768, PqSigMldsa65, Rng12, SigEd25519, X25519Dh, X25519Priv,
    X25519Pub,
};
use quantumshield_refimpl::kt::{KtError, KtVerifier};
use quantumshield_refimpl::qsp::{
    initiator_build, initiator_finalize, responder_process, ratchet_decrypt, ratchet_encrypt,
    HandshakeDeps, HandshakeInit, HandshakeResp, InitiatorState, PrekeyBundle, ProtocolMessage,
    SessionState, SZ_ED25519_SIG, SZ_MLDSA65_SIG,
};
use quantumshield_refimpl::suite2::{binding, establish as suite2_establish, parse as suite2_parse, ratchet as suite2_ratchet, scka as suite2_scka, state as suite2_state, types as suite2_types};

// ---------------------------
// CLI
// ---------------------------

#[derive(Parser, Debug)]
#[command(name = "refimpl_actor", version)]
struct Cli {
    /// Actor name (e.g., impl_a / impl_b)
    #[arg(long)]
    name: String,

    /// CI mode: deterministic defaults
    #[arg(long)]
    ci: bool,
}

// ---------------------------
// Actor contract types (JSONL)
// ---------------------------

#[derive(Debug, Deserialize)]
struct Req {
    id: String,
    op: String,
    #[serde(default)]
    params: serde_json::Value,
}

#[derive(Debug, Serialize)]
struct RespOk {
    id: String,
    ok: bool,
    result: serde_json::Value,
}

#[derive(Debug, Serialize)]
struct RespErr {
    id: String,
    ok: bool,
    error: ErrObj,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
struct ErrObj {
    code: String,
    message: String,
}

// ---------------------------
// Errors
// ---------------------------

#[derive(thiserror::Error, Debug)]
enum ActorError {
    #[error("invalid request: {0}")]
    Invalid(String),
    #[error("invalid request: {0}")]
    InvalidWithResult(String, serde_json::Value),
    #[error("unsupported operation: {0}")]
    Unsupported(String),
    #[error("crypto error: {0}")]
    Crypto(String),
    #[error("internal error: {0}")]
    Internal(String),
}

impl ActorError {
    fn code(&self) -> &'static str {
        match self {
            ActorError::Invalid(_) => "INVALID",
            ActorError::InvalidWithResult(_, _) => "INVALID",
            ActorError::Unsupported(_) => "UNSUPPORTED",
            ActorError::Crypto(_) => "CRYPTO",
            ActorError::Internal(_) => "INTERNAL",
        }
    }


    fn result(&self) -> Option<&serde_json::Value> {
        match self {
            ActorError::InvalidWithResult(_, v) => Some(v),
            _ => None,
        }
    }
}


impl From<CryptoError> for ActorError {
    fn from(e: CryptoError) -> Self {
        ActorError::Crypto(format!("{:?}", e))
    }
}

// ---------------------------
// Deterministic seeding + encoding helpers
// ---------------------------

fn sha3_256(data: &[u8]) -> [u8; 32] {
    let mut h = Sha3_256::new();
    h.update(data);
    let out = h.finalize();
    let mut b = [0u8; 32];
    b.copy_from_slice(&out);
    b
}


fn hex_lower(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push_str(&format!("{:02x}", b));
    }
    s
}

fn sanitize_filename_component(s: &str) -> String {
    s.chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
        .collect()
}

fn dur_seen_path(base_dir: &Path, actor_name: &str, session_id_s: &str) -> PathBuf {
    let actor = sanitize_filename_component(actor_name);
    let sid = sanitize_filename_component(session_id_s);
    base_dir.join(actor).join(format!("{sid}.seen"))
}

fn dur_seen_contains(path: &Path, digest_hex: &str) -> Result<bool, ActorError> {
    match fs::read_to_string(path) {
        Ok(s) => Ok(s.lines().any(|l| l.trim() == digest_hex)),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                Ok(false)
            } else {
                Err(ActorError::Internal(format!("durability store read failed: {e}")))
            }
        }
    }
}

fn dur_seen_append(path: &Path, digest_hex: &str) -> Result<(), ActorError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| ActorError::Internal(format!("durability store mkdir failed: {e}")))?;
    }
    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|e| ActorError::Internal(format!("durability store open failed: {e}")))?;
    writeln!(f, "{digest_hex}")
        .map_err(|e| ActorError::Internal(format!("durability store write failed: {e}")))?;
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct DurSckaMonotonicV1 {
    version: u8,
    peer_max_adv_id_seen: u32,
    local_next_adv_id: u32,
    tombstones: Vec<u32>,
}

fn dur_scka_path(base_dir: &Path, actor_name: &str, session_id_s: &str) -> PathBuf {
    let actor = sanitize_filename_component(actor_name);
    let sid = sanitize_filename_component(session_id_s);
    base_dir.join(actor).join(format!("{sid}.scka.json"))
}

fn load_dur_scka(path: &Path) -> Result<Option<DurSckaMonotonicV1>, ActorError> {
    match fs::read_to_string(path) {
        Ok(s) => {
            let rec: DurSckaMonotonicV1 = serde_json::from_str(&s)
                .map_err(|e| ActorError::Invalid(format!("bad scka durability record: {e}")))?;
            if rec.version != 1 {
                return Err(ActorError::Invalid("bad scka durability record: version".into()));
            }
            Ok(Some(rec))
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                Ok(None)
            } else {
                Err(ActorError::Internal(format!("durability store read failed: {e}")))
            }
        }
    }
}

fn store_dur_scka(path: &Path, rec: &DurSckaMonotonicV1) -> Result<(), ActorError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| ActorError::Internal(format!("durability store mkdir failed: {e}")))?;
    }
    let tmp = path.with_extension(format!("tmp.{}", std::process::id()));
    let s = serde_json::to_string(rec)
        .map_err(|e| ActorError::Internal(format!("durability store encode failed: {e}")))?;
    fs::write(&tmp, s)
        .map_err(|e| ActorError::Internal(format!("durability store write failed: {e}")))?;
    fs::rename(&tmp, path)
        .map_err(|e| ActorError::Internal(format!("durability store rename failed: {e}")))?;
    Ok(())
}

fn suite2_local_next_adv_id(st: &suite2_ratchet::Suite2RecvWireState) -> u32 {
    let mut max_id = 0u32;
    if let Some(m) = st.known_targets.iter().max().copied() {
        max_id = max_id.max(m);
    }
    if let Some(m) = st.tombstoned_targets.iter().max().copied() {
        max_id = max_id.max(m);
    }
    max_id.saturating_add(1)
}

fn merge_dur_scka(prev: Option<DurSckaMonotonicV1>, st: &suite2_ratchet::Suite2RecvWireState) -> DurSckaMonotonicV1 {
    let mut tombstones: BTreeSet<u32> = st.tombstoned_targets.iter().copied().collect();
    let mut peer_max_adv_id_seen = st.peer_max_adv_id_seen;
    let mut local_next_adv_id = suite2_local_next_adv_id(st);

    if let Some(p) = prev {
        peer_max_adv_id_seen = peer_max_adv_id_seen.max(p.peer_max_adv_id_seen);
        local_next_adv_id = local_next_adv_id.max(p.local_next_adv_id);
        for t in p.tombstones {
            tombstones.insert(t);
        }
    }

    DurSckaMonotonicV1 {
        version: 1,
        peer_max_adv_id_seen,
        local_next_adv_id,
        tombstones: tombstones.iter().copied().collect(),
    }
}

fn check_dur_scka_rollback(
    rec: &DurSckaMonotonicV1,
    st: &suite2_ratchet::Suite2RecvWireState,
) -> Result<(), ActorError> {
    if st.peer_max_adv_id_seen < rec.peer_max_adv_id_seen {
        return Err(ActorError::Invalid("reject: REJECT_SCKA_ROLLBACK_DETECTED".into()));
    }
    let local_next = suite2_local_next_adv_id(st);
    if local_next < rec.local_next_adv_id {
        return Err(ActorError::Invalid("reject: REJECT_SCKA_ROLLBACK_DETECTED".into()));
    }
    let tombs: BTreeSet<u32> = st.tombstoned_targets.iter().copied().collect();
    for t in rec.tombstones.iter() {
        if !tombs.contains(t) {
            return Err(ActorError::Invalid("reject: REJECT_SCKA_ROLLBACK_DETECTED".into()));
        }
    }
    Ok(())
}

fn derive_seed32(label: &str, name: &str, seed: &str) -> [u8; 32] {
    // Domain-separated, deterministic across runs.
    let mut v = Vec::new();
    v.extend_from_slice(b"QSHIELD_ACTOR_SEED\0");
    v.extend_from_slice(label.as_bytes());
    v.push(0);
    v.extend_from_slice(name.as_bytes());
    v.push(0);
    v.extend_from_slice(seed.as_bytes());
    sha3_256(&v)
}

fn test_hooks_enabled() -> bool {
    match std::env::var("QSL_TEST_HOOKS") {
        Ok(v) => {
            let v = v.trim();
            v == "1" || v.eq_ignore_ascii_case("true") || v.eq_ignore_ascii_case("yes")
        }
        Err(_) => false,
    }
}


fn x25519_public_from_private(priv32: &[u8; 32]) -> [u8; 32] {
    let ss = StaticSecret::from(*priv32);
    let pk = X25519PublicKey::from(&ss);
    *pk.as_bytes()
}

fn b64u_encode(bytes: &[u8]) -> String {
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes)
}

fn b64u_decode(s: &str) -> Result<Vec<u8>, ActorError> {
    base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(s)
        .map_err(|e| ActorError::Invalid(format!("bad base64url: {e}")))
}

fn session_id_to_string(sid: &[u8; 16]) -> String {
    b64u_encode(sid)
}

fn session_id_from_string(s: &str) -> Result<[u8; 16], ActorError> {
    let b = b64u_decode(s)?;
    if b.len() != 16 {
        return Err(ActorError::Invalid("session_id must decode to 16 bytes".into()));
    }
    let mut sid = [0u8; 16];
    sid.copy_from_slice(&b);
    Ok(sid)
}

// ---------------------------
// Crypto adapters for the refimpl skeleton
// ---------------------------

/// Deterministic X25519 provider.
struct DhDet {
    rng: std::sync::Mutex<ChaCha20Rng>,
}

impl DhDet {
    fn new(seed32: [u8; 32]) -> Self {
        Self { rng: std::sync::Mutex::new(ChaCha20Rng::from_seed(seed32)) }
    }
}

impl X25519Dh for DhDet {
    fn keypair(&self) -> (X25519Priv, X25519Pub) {
        let mut sk = [0u8; 32];
        let mut rng = self.rng.lock().expect("rng mutex");
        rng.fill_bytes(&mut sk);
        let secret = x25519_dalek::StaticSecret::from(sk);
        let pubk = x25519_dalek::PublicKey::from(&secret);
        (X25519Priv(secret.to_bytes()), X25519Pub(pubk.to_bytes()))
    }

    fn dh(&self, privk: &X25519Priv, pubk: &X25519Pub) -> [u8; 32] {
        let sk = x25519_dalek::StaticSecret::from(privk.0);
        let pk = x25519_dalek::PublicKey::from(pubk.0);
        sk.diffie_hellman(&pk).to_bytes()
    }
}

/// Deterministic 12-byte nonce source.
///
/// For CI and test reproducibility we derive nonces from (seed32, counter).
/// This design is snapshot-friendly: restoring persists the counter, preventing reuse.
#[derive(Clone)]
struct Rng12Det {
    seed32: [u8; 32],
    ctr: u64,
}

impl Rng12Det {
    fn new(seed32: [u8; 32]) -> Self {
        Self { seed32, ctr: 0 }
    }

    fn snapshot(&self) -> Rng12DetSnap {
        Rng12DetSnap { seed32: self.seed32, ctr: self.ctr }
    }

    fn from_snapshot(s: Rng12DetSnap) -> Self {
        Self { seed32: s.seed32, ctr: s.ctr }
    }
}

impl Rng12 for Rng12Det {
    fn random_nonce12(&mut self) -> [u8; 12] {
        let mut v = Vec::with_capacity(32 + 8);
        v.extend_from_slice(&self.seed32);
        v.extend_from_slice(&self.ctr.to_be_bytes());
        let h = sha3_256(&v);
        self.ctr = self.ctr.wrapping_add(1);
        let mut out = [0u8; 12];
        out.copy_from_slice(&h[..12]);
        out
    }
}

/// ML-KEM-768 adapter.
///
/// Uses deterministic encapsulation (m derived from a local counter) to ensure CI stability.
struct MlKemDet {
    ctr: std::sync::Mutex<u64>,
}

impl MlKemDet {
    fn new() -> Self {
        Self { ctr: std::sync::Mutex::new(0) }
    }

    fn next_m(&self, domain: &[u8]) -> B32 {
        let mut c = self.ctr.lock().expect("ctr mutex");
        let n = *c;
        *c = n.wrapping_add(1);

        let mut v = Vec::with_capacity(domain.len() + 8);
        v.extend_from_slice(domain);
        v.extend_from_slice(&n.to_be_bytes());
        B32::from(sha3_256(&v))
    }

    fn dk_from_bytes(privk: &[u8]) -> Result<MlKemDk<ml_kem::MlKem768Params>, CryptoError> {
        let enc = Encoded::<MlKemDk<ml_kem::MlKem768Params>>::try_from(privk)
            .map_err(|_| CryptoError::InvalidKey)?;
        Ok(MlKemDk::from_bytes(&enc))
    }

    fn ek_from_bytes(pubk: &[u8]) -> Result<MlKemEk<ml_kem::MlKem768Params>, CryptoError> {
        let enc = Encoded::<MlKemEk<ml_kem::MlKem768Params>>::try_from(pubk)
            .map_err(|_| CryptoError::InvalidKey)?;
        Ok(MlKemEk::from_bytes(&enc))
    }
}

impl PqKem768 for MlKemDet {
    fn encap(&self, pubk: &[u8]) -> Result<(Vec<u8>, Vec<u8>), CryptoError> {
        let ek = Self::ek_from_bytes(pubk)?;
        let m = self.next_m(b"QSHIELD_MLKEM_M");
        let (ct, ss) = ek.encapsulate_deterministic(&m).map_err(|_| CryptoError::InvalidKey)?;
        let ct_bytes: &[u8] = ct.as_ref();
        let ss_bytes: &[u8] = ss.as_slice();
        Ok((ct_bytes.to_vec(), ss_bytes.to_vec()))
    }

    fn decap(&self, privk: &[u8], ct: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let dk = Self::dk_from_bytes(privk)?;
        let ct_enc = ml_kem::Ciphertext::<MlKem768>::try_from(ct).map_err(|_| CryptoError::AuthFail)?;
        let ss = dk.decapsulate(&ct_enc).map_err(|_| CryptoError::AuthFail)?;
        Ok(ss.as_slice().to_vec())
    }
}

/// ML-DSA-65 adapter.
struct MlDsaDet;

impl PqSigMldsa65 for MlDsaDet {
    fn sign(&self, privk: &[u8], msg: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let enc = ml_dsa::EncodedSigningKey::<MlDsa65>::try_from(privk)
            .map_err(|_| CryptoError::InvalidKey)?;
        // `decode()` cannot fail once the input is the correct encoded length.
        let sk = MlDsaSk::<MlDsa65>::decode(&enc);
        let sig = sk.sign(msg);
        let enc_sig = sig.encode();
        Ok(AsRef::<[u8]>::as_ref(&enc_sig).to_vec())
    }

    fn verify(&self, pubk: &[u8], msg: &[u8], sig: &[u8]) -> Result<bool, CryptoError> {
        let enc = ml_dsa::EncodedVerifyingKey::<MlDsa65>::try_from(pubk)
            .map_err(|_| CryptoError::InvalidKey)?;
        // `decode()` cannot fail once the input is the correct encoded length.
        let vk = MlDsaVk::<MlDsa65>::decode(&enc);
        let sig = MlDsaSig::<MlDsa65>::try_from(sig).map_err(|_| CryptoError::AuthFail)?;
        Ok(vk.verify(msg, &sig).is_ok())
    }
}

/// Ed25519 adapter where the 32-byte private key is treated as an Ed25519 seed.
struct Ed25519Det;

impl SigEd25519 for Ed25519Det {
    fn sign(&self, privk: &[u8], msg: &[u8]) -> Vec<u8> {
        if privk.len() != 32 {
            return vec![]; // fail-closed by producing an unusable signature
        }
        let mut seed = [0u8; 32];
        seed.copy_from_slice(privk);
        let sk = ed25519_dalek::SigningKey::from_bytes(&seed);
        sk.sign(msg).to_bytes().to_vec()
    }

    fn verify(&self, pubk: &[u8], msg: &[u8], sig: &[u8]) -> bool {
        let pubk: [u8; 32] = match pubk.try_into() {
            Ok(x) => x,
            Err(_) => return false,
        };
        let vk = match ed25519_dalek::VerifyingKey::from_bytes(&pubk) {
            Ok(v) => v,
            Err(_) => return false,
        };
        let sig = match ed25519_dalek::Signature::from_slice(sig) {
            Ok(s) => s,
            Err(_) => return false,
        };
        vk.verify_strict(msg, &sig).is_ok()
    }
}

/// KT verifier for harness execution.
///
/// This implementation only accepts the "KT disabled" shape:
/// - kt_log_id == all-zero
/// - kt_sth / proofs are empty
///
/// If any KT material is present, it fail-closes with NotImplemented.
struct KtAllowEmptyOnly;

impl KtVerifier for KtAllowEmptyOnly {
    fn verify_bundle(
        &self,
        kt_log_id: &[u8; 32],
        kt_sth: &[u8],
        kt_inclusion_proof: &[u8],
        kt_consistency_proof: &[u8],
    ) -> Result<(), KtError> {
        let all_zero = kt_log_id.iter().all(|&b| b == 0);
        if all_zero && kt_sth.is_empty() && kt_inclusion_proof.is_empty() && kt_consistency_proof.is_empty() {
            Ok(())
        } else {
            Err(KtError::NotImplemented)
        }
    }
}

// ---------------------------
// Deterministic identity + prekey derivation
// ---------------------------


#[derive(Clone)]
struct StaticKeys {
    // Identity signature keys
    ik_sig_ec_seed: [u8; 32],
    ik_sig_ec_pub: [u8; 32],
    ik_sig_pq_priv: Vec<u8>,
    ik_sig_pq_pub: Vec<u8>,

    // Signed-prekeys (DH + PQ)
    spk_dh_priv: [u8; 32],
    spk_dh_pub: [u8; 32],
    spk_pq_priv: Vec<u8>,
    spk_pq_pub: Vec<u8>,

    // PQ receiver key (used for PQ boundary / service-edge delivery)
    pq_rcv_id: u32,
    pq_rcv_priv: Vec<u8>,
    pq_rcv_pub: Vec<u8>,
}

fn gen_static_keys(actor_name: &str, seed: &str) -> StaticKeys {
    // Ed25519 identity signing keys
    let ik_sig_ec_seed = derive_seed32("IK_SIG_EC", actor_name, seed);
    let ik_sig_ec = SigningKey::from_bytes(&ik_sig_ec_seed);
    let ik_sig_ec_pub = ik_sig_ec.verifying_key().to_bytes();

    // ML-DSA-65 identity signing keys
    let mut rng = ChaCha20Rng::from_seed(derive_seed32("IK_SIG_PQ", actor_name, seed));
    let kp = MlDsa65::key_gen(&mut rng);
    let enc_sk = kp.signing_key().encode();
    let enc_vk = kp.verifying_key().encode();
    let ik_sig_pq_priv = AsRef::<[u8]>::as_ref(&enc_sk).to_vec();
    let ik_sig_pq_pub = AsRef::<[u8]>::as_ref(&enc_vk).to_vec();

    // X25519 signed-prekey
    let spk_dh_priv = derive_seed32("SPK_DH", actor_name, seed);
    let spk_dh_pub = x25519_public_from_private(&spk_dh_priv);

    // ML-KEM-768 signed-prekey (deterministic keygen)
    let d = B32::from(derive_seed32("SPK_PQ_D", actor_name, seed));
    let z = B32::from(derive_seed32("SPK_PQ_Z", actor_name, seed));
    let (dk, ek) = MlKem768::generate_deterministic(&d, &z);
    let spk_pq_pub = ek.as_bytes().to_vec();
    let spk_pq_priv = dk.as_bytes().to_vec();

    // PQ receiver (deterministic keygen + deterministic id)
    let id_bytes = derive_seed32("PQ_RCV_ID", actor_name, seed);
    let mut pq_rcv_id = u32::from_be_bytes([id_bytes[0], id_bytes[1], id_bytes[2], id_bytes[3]]);
    if pq_rcv_id == 0 {
        pq_rcv_id = 1;
    }
    let d = B32::from(derive_seed32("PQ_RCV_D", actor_name, seed));
    let z = B32::from(derive_seed32("PQ_RCV_Z", actor_name, seed));
    let (dk, ek) = MlKem768::generate_deterministic(&d, &z);
    let pq_rcv_pub = ek.as_bytes().to_vec();
    let pq_rcv_priv = dk.as_bytes().to_vec();

    StaticKeys {
        ik_sig_ec_seed,
        ik_sig_ec_pub,
        ik_sig_pq_priv,
        ik_sig_pq_pub,
        spk_dh_priv,
        spk_dh_pub,
        spk_pq_priv,
        spk_pq_pub,
        pq_rcv_id,
        pq_rcv_priv,
        pq_rcv_pub,
    }
}

fn prekey_bundle_tbs(bundle: &PrekeyBundle) -> Vec<u8> {
    // Canonical “to-be-signed” encoding of a bundle: identical field order to PrekeyBundle::encode,
    // but *excluding* sig_ec and sig_pq.
    let mut w = Writer::new();

    w.write_varbytes_u16(&bundle.user_id);
    w.write_u32(bundle.device_id);
    w.write_u32(bundle.valid_from);
    w.write_u32(bundle.valid_to);

    w.write_bytes(&bundle.ik_sig_ec_pub);
    w.write_varbytes_u16(&bundle.ik_sig_pq_pub);

    w.write_bytes(&bundle.spk_dh_pub);
    w.write_varbytes_u16(&bundle.spk_pq_pub);

    w.write_u32(bundle.pq_rcv_id);
    w.write_varbytes_u16(&bundle.pq_rcv_pub);

    w.write_u16(if bundle.opk_dh.is_some() { 1 } else { 0 });
    if let Some((id, pk)) = &bundle.opk_dh {
        w.write_u32(*id);
        w.write_bytes(pk);
    }

    w.write_u16(if bundle.opk_pq.is_some() { 1 } else { 0 });
    if let Some((id, pk)) = &bundle.opk_pq {
        w.write_u32(*id);
        w.write_varbytes_u16(pk);
    }

    // KT material is also authenticated by the bundle signatures.
    w.write_bytes(&bundle.kt_log_id);
    w.write_varbytes_u32(&bundle.kt_sth);
    w.write_varbytes_u32(&bundle.kt_inclusion_proof);
    w.write_varbytes_u32(&bundle.kt_consistency_proof);

    w.into_vec()
}

fn build_prekey_bundle_for(
    peer: &StaticKeys,
    peer_name: &str,
    device_id: u32,
    std: &StdCrypto,
    sig_ec: &Ed25519Det,
    sig_pq: &MlDsaDet,
) -> Result<PrekeyBundle, CryptoError> {
    // NOTE: OPKs and KT proofs are not exercised by the current CI harness (it only runs baseline
    // handshake smoke tests). We still populate all fields and generate bundle signatures so that
    // future interop cases can be enabled without revisiting actor wiring.

    let mut bundle = PrekeyBundle {
        user_id: peer_name.as_bytes().to_vec(),
        device_id,
        valid_from: 0,
        valid_to: u32::MAX,

        ik_sig_ec_pub: peer.ik_sig_ec_pub,
        ik_sig_pq_pub: peer.ik_sig_pq_pub.clone(),

        spk_dh_pub: peer.spk_dh_pub,
        spk_pq_pub: peer.spk_pq_pub.clone(),

        pq_rcv_id: peer.pq_rcv_id,
        pq_rcv_pub: peer.pq_rcv_pub.clone(),

        opk_dh: None,
        opk_pq: None,

        sig_ec: vec![0u8; SZ_ED25519_SIG],
        sig_pq: vec![0u8; SZ_MLDSA65_SIG],

        kt_log_id: [0u8; 32],
        kt_sth: Vec::new(),
        kt_inclusion_proof: Vec::new(),
        kt_consistency_proof: Vec::new(),
    };

    let tbs = prekey_bundle_tbs(&bundle);
    let digest = std.sha512(&tbs);

    bundle.sig_ec = sig_ec.sign(&peer.ik_sig_ec_seed, &digest);
    bundle.sig_pq = sig_pq.sign(&peer.ik_sig_pq_priv, &digest)?;

    Ok(bundle)
}
fn derive_peer_name(self_name: &str) -> String {
    // Harness wires impl_a <-> impl_b.
    if self_name.ends_with("_a") {
        format!("{}_b", self_name.trim_end_matches("_a"))
    } else if self_name.ends_with("_b") {
        format!("{}_a", self_name.trim_end_matches("_b"))
    } else {
        // Fallback: deterministic but explicit.
        format!("{self_name}_peer")
    }
}

// ---------------------------
// Actor runtime state
// ---------------------------

struct PendingInit {
    init: InitiatorState,
    dh0_a: (X25519Priv, X25519Pub),
    pq_rcv_a_priv: Vec<u8>,
    suite: SuiteKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SuiteKind {
    Suite1,
    Suite1B,
    Suite2,
}

struct SessionEntry {
    st: SessionState,
    rng12: Rng12Det,
    suite: SuiteKind,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
struct Rng12DetSnap {
    seed32: [u8; 32],
    ctr: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SessionSnapshotV1 {
    session_id: String,
    st_b64: String,
    rng: Rng12DetSnap,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ActorSnapshotV1 {
    seed: String,
    sessions: Vec<SessionSnapshotV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SessionSnapshotV2 {
    session_id: String,
    suite: String,
    #[serde(default)]
    st_b64: Option<String>,
    #[serde(default)]
    s2_b64: Option<String>,
    rng: Rng12DetSnap,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ActorSnapshotV2 {
    version: u8,
    seed: String,
    sessions: Vec<SessionSnapshotV2>,
}

struct Actor {
    name: String,
    ci: bool,
    seed: String,

    std: StdCrypto,
    dh: DhDet,
    pq_kem: MlKemDet,
    pq_sig: MlDsaDet,
    ed: Ed25519Det,
    kt: KtAllowEmptyOnly,

    sid_rng: ChaCha20Rng,
    static_keys: StaticKeys,

    pending: HashMap<[u8; 16], PendingInit>,
    sessions: HashMap<[u8; 16], SessionEntry>,
    suite2_sessions: HashMap<[u8; 16], suite2_state::Suite2SessionState>,
}

impl Actor {
    fn new(name: String, ci: bool) -> Result<Self, ActorError> {
        let seed = if ci { "ci-default".to_string() } else { "local-default".to_string() };

        let static_keys = gen_static_keys(&name, &seed);

        Ok(Self {
            dh: DhDet::new(derive_seed32("DH", &name, &seed)),
            sid_rng: ChaCha20Rng::from_seed(derive_seed32("SID", &name, &seed)),
            std: StdCrypto,
            pq_kem: MlKemDet::new(),
            pq_sig: MlDsaDet,
            ed: Ed25519Det,
            kt: KtAllowEmptyOnly,
            name,
            ci,
            seed,
            static_keys,
            pending: HashMap::new(),
            sessions: HashMap::new(),
            suite2_sessions: HashMap::new(),
        })
    }

    fn deps(&self) -> HandshakeDeps<'_> {
        HandshakeDeps {
            hash: &self.std,
            kmac: &self.std,
            aead: &self.std,
            dh: &self.dh,
            pq_kem: &self.pq_kem,
            pq_sig: &self.pq_sig,
            ed25519: &self.ed,
            kt: &self.kt,
        }
    }

    fn reset(&mut self, seed_opt: Option<String>) -> Result<(), ActorError> {
        if let Some(s) = seed_opt {
            self.seed = s;
        } else {
            self.seed = if self.ci { "ci-default".to_string() } else { "local-default".to_string() };
        }

        self.static_keys = gen_static_keys(&self.name, &self.seed);
        self.dh = DhDet::new(derive_seed32("DH", &self.name, &self.seed));
        self.pq_kem = MlKemDet::new();
        self.sid_rng = ChaCha20Rng::from_seed(derive_seed32("SID", &self.name, &self.seed));

        self.pending.clear();
        self.sessions.clear();
        self.suite2_sessions.clear();
        Ok(())
    }

    fn new_session_id(&mut self) -> [u8; 16] {
        let mut sid = [0u8; 16];
        self.sid_rng.fill_bytes(&mut sid);
        sid
    }

    fn derive_session_dh0(&self, session_id: &[u8; 16]) -> (X25519Priv, X25519Pub) {
        let mut v = Vec::new();
        v.extend_from_slice(b"DH0");
        v.extend_from_slice(session_id);
        v.extend_from_slice(self.name.as_bytes());
        v.extend_from_slice(self.seed.as_bytes());
        let sk32 = sha3_256(&v);
        let sk = x25519_dalek::StaticSecret::from(sk32);
        let pk = x25519_dalek::PublicKey::from(&sk);
        (X25519Priv(sk.to_bytes()), X25519Pub(pk.to_bytes()))
    }

    fn derive_session_pq_rcv(&self, session_id: &[u8; 16]) -> (u32, Vec<u8>, Vec<u8>) {
        // Deterministic keygen inputs (d,z) derived from session id.
        let mut d_in = Vec::new();
        d_in.extend_from_slice(b"PQ_RCV_D");
        d_in.extend_from_slice(session_id);
        d_in.extend_from_slice(self.name.as_bytes());
        d_in.extend_from_slice(self.seed.as_bytes());
        let d = B32::from(sha3_256(&d_in));

        let mut z_in = Vec::new();
        z_in.extend_from_slice(b"PQ_RCV_Z");
        z_in.extend_from_slice(session_id);
        z_in.extend_from_slice(self.name.as_bytes());
        z_in.extend_from_slice(self.seed.as_bytes());
        let z = B32::from(sha3_256(&z_in));

        let (dk, ek) = MlKem768::generate_deterministic(&d, &z);
        let id = 1; // deterministic placeholder
        (id, ek.as_bytes().as_slice().to_vec(), dk.as_bytes().as_slice().to_vec())
    }

    fn rng12_seed_for_session(&self, session_id: &[u8; 16]) -> [u8; 32] {
        let sid = session_id_to_string(session_id);
        derive_seed32("NONCE12", &format!("{}:{sid}", self.name), &self.seed)
    }

    fn handle_capabilities(&self) -> serde_json::Value {
        let mut ops = vec![
            "capabilities",
            "ping",
            "reset",
            "handshake_init",
            "handshake_respond",
            "handshake_finish",
            "encrypt",
            "decrypt",
        ];
        if test_hooks_enabled() {
            ops.push("debug_snapshot");
            ops.push("debug_restore");
        }
        serde_json::json!({
            "actor": "refimpl",
            "name": self.name,
            "mode": if self.ci { "ci" } else { "local" },
            "suites": ["Suite-1", "Suite-1B", "Suite-2"],
            "ops": ops,
        })
    }

    fn handle_handshake_init(&mut self, suite: &str) -> Result<serde_json::Value, ActorError> {
        let suite_kind = if suite == "Suite-1" {
            SuiteKind::Suite1
        } else if suite == "Suite-1B" {
            SuiteKind::Suite1B
        } else if suite == "Suite-2" {
            return Err(ActorError::Invalid("reject: REJECT_S2_NOT_IMPLEMENTED".into()));
        } else {
            return Err(ActorError::Unsupported(format!("unsupported suite: {suite}")));
        };

        let peer_name = derive_peer_name(&self.name);
        let peer_keys = gen_static_keys(&peer_name, &self.seed);
        let bundle_b = build_prekey_bundle_for(&peer_keys, &peer_name, 1, &self.std, &self.ed, &self.pq_sig)?;

        let session_id = self.new_session_id();

        // Session-specific A keys: DH0_A and PQ_RCV_A
        let dh0_a = self.derive_session_dh0(&session_id);
        let (pq_rcv_a_id, pq_rcv_a_pub, pq_rcv_a_priv) = self.derive_session_pq_rcv(&session_id);

        let deps = self.deps();
        let (hs1, init) = initiator_build(
            &deps,
            &bundle_b,
            // A identity
            self.name.as_bytes().to_vec(),
            1,
            session_id,
            // IK(A)
            self.static_keys.ik_sig_ec_pub,
            self.static_keys.ik_sig_ec_seed.to_vec(),
            self.static_keys.ik_sig_pq_pub.clone(),
            self.static_keys.ik_sig_pq_priv.clone(),
            // PQ receive for A
            pq_rcv_a_id,
            pq_rcv_a_pub.clone(),
        )
        .map_err(|e| ActorError::Crypto(format!("handshake_init failed: {e}")))?;

        self.pending.insert(session_id, PendingInit { init, dh0_a, pq_rcv_a_priv, suite: suite_kind });

        let msg1 = hs1.encode();
        Ok(serde_json::json!({
            "session_id": session_id_to_string(&session_id),
            "msg1_b64": b64u_encode(&msg1)
        }))
    }

    fn handle_handshake_respond(&mut self, msg1_b64: &str) -> Result<serde_json::Value, ActorError> {
        let msg1 = b64u_decode(msg1_b64)?;
        let hs1 = HandshakeInit::decode(&msg1).map_err(|e| ActorError::Invalid(format!("bad HS1: {e}")))?;

        // Session-specific B keys: DH0_B and PQ_RCV_B
        let dh0_b = self.derive_session_dh0(&hs1.session_id);
        let (pq_rcv_b_id, pq_rcv_b_pub, pq_rcv_b_priv) = self.derive_session_pq_rcv(&hs1.session_id);

        let deps = self.deps();
        let (hs2, st) = responder_process(
            &deps,
            &hs1,
            // IK(B)
            self.static_keys.ik_sig_ec_pub,
            self.static_keys.ik_sig_ec_seed.to_vec(),
            self.static_keys.ik_sig_pq_pub.clone(),
            self.static_keys.ik_sig_pq_priv.clone(),
            // SPK(B)
            X25519Priv(self.static_keys.spk_dh_priv),
            self.static_keys.spk_pq_priv.clone(),
            // OPK not used in this skeleton
            None,
            None,
            // DH0_B + PQ_RCV_B
            dh0_b,
            pq_rcv_b_id,
            pq_rcv_b_pub,
            pq_rcv_b_priv,
        )
        .map_err(|e| ActorError::Crypto(format!("handshake_respond failed: {e}")))?;

        let rng12 = Rng12Det::new(self.rng12_seed_for_session(&hs1.session_id));
        self.sessions.insert(hs1.session_id, SessionEntry { st, rng12, suite: SuiteKind::Suite1 });

        let msg2 = hs2.encode();
        Ok(serde_json::json!({ "msg2_b64": b64u_encode(&msg2) }))
    }

    fn handle_handshake_finish(&mut self, session_id_s: &str, msg2_b64: &str) -> Result<serde_json::Value, ActorError> {
        let session_id = session_id_from_string(session_id_s)?;
        let pending = self
            .pending
            .remove(&session_id)
            .ok_or_else(|| ActorError::Invalid("unknown session_id".into()))?;

        let msg2 = b64u_decode(msg2_b64)?;
        let hs2 = HandshakeResp::decode(&msg2).map_err(|e| ActorError::Invalid(format!("bad HS2: {e}")))?;

        let deps = self.deps();
        let st = initiator_finalize(&deps, pending.init, &hs2, pending.dh0_a, pending.pq_rcv_a_priv)
            .map_err(|e| ActorError::Crypto(format!("handshake_finish failed: {e}")))?;

        let rng12 = Rng12Det::new(self.rng12_seed_for_session(&session_id));
        self.sessions.insert(session_id, SessionEntry { st, rng12, suite: pending.suite });

        Ok(serde_json::json!({ "session_id": session_id_to_string(&session_id) }))
    }

    fn handle_encrypt(&mut self, session_id_s: &str, pt_b64: &str) -> Result<serde_json::Value, ActorError> {
        let session_id = session_id_from_string(session_id_s)?;
        let pt = b64u_decode(pt_b64)?;
        let entry = self
            .sessions
            .get_mut(&session_id)
            .ok_or_else(|| ActorError::Invalid("unknown session_id".into()))?;

        if entry.suite == SuiteKind::Suite2 {
            return Err(ActorError::Invalid("reject: REJECT_S2_NOT_IMPLEMENTED".into()));
        }

        let msg = ratchet_encrypt(
            &mut entry.st,
            &self.std,
            &self.std,
            &self.std,
            &self.dh,
            &self.pq_kem,
            &mut entry.rng12,
            &pt,
            false,
            false,
        )
        .map_err(|e| ActorError::Crypto(format!("encrypt failed: {e}")))?;

        Ok(serde_json::json!({ "ciphertext_b64": b64u_encode(&msg.encode()) }))
    }

    fn handle_decrypt(&mut self, session_id_s: &str, ct_b64: &str) -> Result<serde_json::Value, ActorError> {
        let session_id = session_id_from_string(session_id_s)?;
        let ct = b64u_decode(ct_b64)?;

        // Durability rollback-replay defense (test harness): persist seen ciphertext digests
        // across snapshot/restore so restoring an earlier snapshot cannot re-accept post-snapshot ciphertexts.
        //
        // Safety gating: this behavior MUST be test-only. It is enabled only when BOTH:
        //   - QSL_TEST_HOOKS=1 (or true/yes)
        //   - QSL_DUR_STORE_DIR is set and non-empty
        let dur_store_dir = if test_hooks_enabled() {
            std::env::var("QSL_DUR_STORE_DIR").ok().filter(|s| !s.is_empty())
        } else {
            None
        };
        let dur_digest = if dur_store_dir.is_some() {
            hex_lower(&sha3_256(&ct))
        } else {
            String::new()
        };
        let dur_seen = dur_store_dir.as_ref().map(|d| dur_seen_path(Path::new(d), &self.name, session_id_s));
        if let Some(p) = dur_seen.as_ref() {
            if dur_seen_contains(p, &dur_digest)? {
                return Err(ActorError::Crypto("replay (durable)".into()));
            }
        }
        let entry = self
            .sessions
            .get_mut(&session_id)
            .ok_or_else(|| ActorError::Invalid("unknown session_id".into()))?;

        if entry.suite == SuiteKind::Suite2 {
            return Err(ActorError::Invalid("reject: REJECT_S2_NOT_IMPLEMENTED".into()));
        }

        let msg = ProtocolMessage::decode(&ct).map_err(|e| ActorError::Invalid(format!("bad protocol message: {e}")))?;
        let pt = ratchet_decrypt(&mut entry.st, &self.std, &self.std, &self.std, &self.dh, &self.pq_kem, &msg)
            .map_err(|e| ActorError::Crypto(format!("decrypt failed: {e}")))?;

        if let Some(p) = dur_seen.as_ref() {
            dur_seen_append(p, &dur_digest)?;
        }

        Ok(serde_json::json!({ "plaintext_b64": b64u_encode(&pt) }))
    }

    fn handle_debug_snapshot(&self) -> Result<serde_json::Value, ActorError> {
        if !test_hooks_enabled() {
            return Err(ActorError::Unsupported("debug hooks disabled (set QSL_TEST_HOOKS=1)".into()));
        }

        let mut sessions: Vec<SessionSnapshotV2> = Vec::new();
        for (sid, entry) in &self.sessions {
            let st_bytes = entry.st.snapshot_bytes();
            sessions.push(SessionSnapshotV2 {
                session_id: session_id_to_string(sid),
                suite: suite_kind_str(entry.suite).to_string(),
                st_b64: Some(b64u_encode(&st_bytes)),
                s2_b64: None,
                rng: entry.rng12.snapshot(),
            });
        }
        for (sid, entry) in &self.suite2_sessions {
            let st_bytes = entry.snapshot_bytes();
            sessions.push(SessionSnapshotV2 {
                session_id: session_id_to_string(sid),
                suite: "Suite-2".to_string(),
                st_b64: None,
                s2_b64: Some(b64u_encode(&st_bytes)),
                rng: Rng12DetSnap { seed32: [0u8; 32], ctr: 0 },
            });
        }
        sessions.sort_by(|a, b| a.session_id.cmp(&b.session_id));

        let snap = ActorSnapshotV2 { version: 2, seed: self.seed.clone(), sessions };
        let raw = serde_json::to_vec(&snap)
            .map_err(|e| ActorError::Internal(format!("snapshot encode failed: {e}")))?;

        Ok(serde_json::json!({
            "snapshot_b64": b64u_encode(&raw),
            "sessions": snap.sessions.len(),
        }))
    }

    fn handle_debug_restore(&mut self, snapshot_b64: &str) -> Result<serde_json::Value, ActorError> {
        if !test_hooks_enabled() {
            return Err(ActorError::Unsupported("debug hooks disabled (set QSL_TEST_HOOKS=1)".into()));
        }

        let raw = b64u_decode(snapshot_b64)?;
        let snap_val: serde_json::Value = serde_json::from_slice(&raw)
            .map_err(|e| ActorError::Invalid(format!("bad snapshot json: {e}")))?;
        let version = snap_val.get("version").and_then(|v| v.as_u64());

        let mut new_sessions: HashMap<[u8; 16], SessionEntry> = HashMap::new();
        let mut new_suite2_sessions: HashMap<[u8; 16], suite2_state::Suite2SessionState> = HashMap::new();
        let mut seed = None;

        if version == Some(2) {
            let snap: ActorSnapshotV2 = serde_json::from_value(snap_val)
                .map_err(|e| ActorError::Invalid(format!("bad snapshot json: {e}")))?;
            seed = Some(snap.seed.clone());
            for s in snap.sessions.iter() {
                let sid = session_id_from_string(&s.session_id)?;
                match s.suite.as_str() {
                    "Suite-1" | "Suite-1B" => {
                        let st_b64 = s.st_b64.as_ref().ok_or_else(|| ActorError::Invalid("bad session snapshot: missing st_b64".into()))?;
                        let st_raw = b64u_decode(st_b64)?;
                        let st = SessionState::restore_bytes(&st_raw)
                            .map_err(|e| ActorError::Invalid(format!("bad session snapshot: {e}")))?;
                        let rng12 = Rng12Det::from_snapshot(s.rng);
                        let suite = if s.suite == "Suite-1B" { SuiteKind::Suite1B } else { SuiteKind::Suite1 };
                        new_sessions.insert(sid, SessionEntry { st, rng12, suite });
                    }
                    "Suite-2" => {
                        let s2_b64 = s.s2_b64.as_ref().ok_or_else(|| ActorError::Invalid("bad session snapshot: missing s2_b64".into()))?;
                        let st_raw = b64u_decode(s2_b64)?;
                        let st = suite2_state::Suite2SessionState::restore_bytes(&st_raw)
                            .map_err(|e| ActorError::Invalid(format!("bad session snapshot: {e}")))?;
                        new_suite2_sessions.insert(sid, st);
                    }
                    _ => return Err(ActorError::Invalid("bad session snapshot: unknown suite".into())),
                }
            }
        } else {
            let snap: ActorSnapshotV1 = serde_json::from_value(snap_val)
                .map_err(|e| ActorError::Invalid(format!("bad snapshot json: {e}")))?;
            seed = Some(snap.seed.clone());
            for s in snap.sessions.iter() {
                let sid = session_id_from_string(&s.session_id)?;
                let st_raw = b64u_decode(&s.st_b64)?;
                let st = SessionState::restore_bytes(&st_raw)
                    .map_err(|e| ActorError::Invalid(format!("bad session snapshot: {e}")))?;
                let rng12 = Rng12Det::from_snapshot(s.rng);
                new_sessions.insert(sid, SessionEntry { st, rng12, suite: SuiteKind::Suite1 });
            }
        }

        let dur_store_dir = if test_hooks_enabled() {
            std::env::var("QSL_DUR_STORE_DIR").ok().filter(|s| !s.is_empty())
        } else {
            None
        };
        if let Some(dir) = dur_store_dir.as_ref() {
            for (sid, st) in new_suite2_sessions.iter() {
                let sid_s = session_id_to_string(sid);
                let path = dur_scka_path(Path::new(dir), &self.name, &sid_s);
                if let Some(rec) = load_dur_scka(&path)? {
                    check_dur_scka_rollback(&rec, &st.recv)?;
                }
            }
        }

        // Reset deterministic key material to the snapshot seed, then restore session maps.
        if let Some(s) = seed {
            self.reset(Some(s))?;
        }
        self.pending.clear();
        self.sessions = new_sessions;
        self.suite2_sessions = new_suite2_sessions;

        Ok(serde_json::json!({
            "restored": true,
            "sessions": self.sessions.len() + self.suite2_sessions.len()
        }))
    }

    fn dispatch(&mut self, req: Req) -> Result<serde_json::Value, ActorError> {
        match req.op.as_str() {
            "capabilities" => Ok(self.handle_capabilities()),
            "ping" => Ok(serde_json::json!({ "name": self.name, "ci": self.ci })),
            "reset" => {
                let seed = req.params.get("seed").and_then(|v| v.as_str()).map(|s| s.to_string());
                self.reset(seed)?;
                Ok(serde_json::json!({ "reset": true }))
            }
            "handshake_init" => {
                let suite = req
                    .params
                    .get("suite")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| ActorError::Invalid("missing params.suite".into()))?;
                self.handle_handshake_init(suite)
            }
            "handshake_respond" => {
                let msg1 = req
                    .params
                    .get("msg1_b64")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| ActorError::Invalid("missing params.msg1_b64".into()))?;
                self.handle_handshake_respond(msg1)
            }
            "handshake_finish" => {
                let msg2 = req
                    .params
                    .get("msg2_b64")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| ActorError::Invalid("missing params.msg2_b64".into()))?;

                // The 4B harness does not pass session_id to handshake_finish; it expects
                // the actor to track the pending initiator state.
                //
                // If the caller provides session_id, use it. Otherwise, if there is exactly
                // one pending initiator session, infer it deterministically. Fail-closed if
                // ambiguous.
                let sid_s: String = if let Some(s) = req.params.get("session_id").and_then(|v| v.as_str()) {
                    s.to_string()
                } else if self.pending.len() == 1 {
                    let only = self.pending.keys().next().unwrap();
                    session_id_to_string(only)
                } else {
                    return Err(ActorError::Invalid("missing params.session_id".into()));
                };

                self.handle_handshake_finish(&sid_s, msg2)
            }
            "encrypt" => {
                let sid = req
                    .params
                    .get("session_id")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| ActorError::Invalid("missing params.session_id".into()))?;
                let pt = req
                    .params
                    .get("plaintext_b64")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| ActorError::Invalid("missing params.plaintext_b64".into()))?;
                self.handle_encrypt(sid, pt)
            }
            "decrypt" => {
                let sid = req
                    .params
                    .get("session_id")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| ActorError::Invalid("missing params.session_id".into()))?;
                let ct = req
                    .params
                    .get("ciphertext_b64")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| ActorError::Invalid("missing params.ciphertext_b64".into()))?;
                self.handle_decrypt(sid, ct)
            }
            "debug_snapshot" => self.handle_debug_snapshot(),
            "debug_restore" => {
                let snap_b64 = req
                    .params
                    .get("snapshot_b64")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| ActorError::Invalid("missing params.snapshot_b64".into()))?;
                self.handle_debug_restore(snap_b64)
            }
            
            // ---------------------------
            // Suite-2 KDF conformance ops
            // ---------------------------
            "suite2.transcript.check" => {
                let negotiated = get_json_data(&req.params, "negotiated")?;
                let pv = parse_u16(
                    negotiated
                        .get("protocol_version")
                        .ok_or_else(|| ActorError::Invalid("params.negotiated.protocol_version missing".into()))?,
                    "params.negotiated.protocol_version",
                )?;
                let sid = parse_u16(
                    negotiated
                        .get("suite_id")
                        .ok_or_else(|| ActorError::Invalid("params.negotiated.suite_id missing".into()))?,
                    "params.negotiated.suite_id",
                )?;
                if pv != 0x0500 || sid != 0x0002 {
                    return Err(ActorError::Invalid("reject: REJECT_S2_SUITE_MISMATCH".into()));
                }

                let session_id = get_bytes(&req.params, "session_id")?;
                let dh_pub = get_bytes(&req.params, "DH_pub")?;
                let flags = get_u16(&req.params, "flags")?;
                let pq_prefix = get_bytes(&req.params, "pq_prefix")?;
                let ad_hdr_in = get_bytes(&req.params, "ad_hdr")?;
                let ad_body_in = get_bytes(&req.params, "ad_body")?;

                let pq_bind = binding::pq_bind_sha512_32(&self.std, flags, &pq_prefix);
                let ad_hdr_calc = binding::ad_hdr(&session_id, pv, sid, &dh_pub, flags, &pq_bind);
                let ad_body_calc = binding::ad_body(&session_id, pv, sid, &pq_bind);

                if ad_hdr_calc != ad_hdr_in || ad_body_calc != ad_body_in {
                    return Err(ActorError::Invalid("reject: REJECT_S2_AD_MISMATCH".into()));
                }

                Ok(serde_json::json!({
                    "pq_bind": jhex(&pq_bind),
                    "ad_hdr": jhex(&ad_hdr_calc),
                    "ad_body": jhex(&ad_body_calc),
                }))
            }
            "suite2.mk_hybrid.check" => {
                let ck_ec = get_bytes(&req.params, "CK_ec")?;
                let ck_pq = get_bytes(&req.params, "CK_pq")?;
                if ck_ec.len() != 32 {
                    return Err(ActorError::Invalid("reject: REJECT_S2_MK_BAD_CK_EC".into()));
                }
                if ck_pq.len() != 32 {
                    return Err(ActorError::Invalid("reject: REJECT_S2_MK_BAD_CK_PQ".into()));
                }
                let mut ck_ec_arr: [u8; 32] = ck_ec.as_slice().try_into()
                    .map_err(|_| ActorError::Invalid("reject: REJECT_S2_MK_BAD_CK_EC".into()))?;
                let mut ck_pq_arr: [u8; 32] = ck_pq.as_slice().try_into()
                    .map_err(|_| ActorError::Invalid("reject: REJECT_S2_MK_BAD_CK_PQ".into()))?;

                let count = get_u32(&req.params, "count")?;
                let mut mk_list_hex: Vec<String> = Vec::new();
                let mut mk_list_json: Vec<serde_json::Value> = Vec::new();

                for _ in 0..count {
                    let (ck_ec_p, ck_pq_p, mk) =
                        suite2_ratchet::derive_mk_step(&self.std, &ck_ec_arr, &ck_pq_arr)
                            .map_err(|_| ActorError::Invalid("reject: REJECT_S2_MK_DERIVE_FAIL".into()))?;
                    let mk_hex = to_hex(&mk);
                    mk_list_hex.push(mk_hex.clone());
                    mk_list_json.push(serde_json::json!({ "type": "hex", "data": mk_hex }));
                    ck_ec_arr = ck_ec_p;
                    ck_pq_arr = ck_pq_p;
                }

                if req.params.get("expected_mk_list").is_some() {
                    let ev = get_json_data(&req.params, "expected_mk_list")?;
                    let expected = parse_hex_list(&ev, "params.expected_mk_list")?;
                    if expected.len() != mk_list_hex.len() {
                        return Err(ActorError::Invalid("reject: REJECT_S2_MK_MISMATCH".into()));
                    }
                    for (a, b) in expected.iter().zip(mk_list_hex.iter()) {
                        if a.to_ascii_lowercase() != b.to_ascii_lowercase() {
                            return Err(ActorError::Invalid("reject: REJECT_S2_MK_MISMATCH".into()));
                        }
                    }
                }

                Ok(serde_json::json!({
                    "mk_list": { "type": "json", "data": mk_list_json },
                    "CK_ec_final": jhex(&ck_ec_arr),
                    "CK_pq_final": jhex(&ck_pq_arr),
                }))
            }
            "suite2.pqreseed.apply" => {
                let role_v = get_json_data(&req.params, "role")?;
                let role_s = if let Some(s) = role_v.as_str() {
                    s
                } else if let Some(obj) = role_v.as_object() {
                    obj.get("role")
                        .and_then(|v| v.as_str())
                        .or_else(|| obj.get("value").and_then(|v| v.as_str()))
                        .ok_or_else(|| ActorError::Invalid("params.role: expected string".into()))?
                } else {
                    return Err(ActorError::Invalid("params.role: expected string".into()));
                };
                let role_is_a = match role_s {
                    "A" => true,
                    "B" => false,
                    _ => return Err(ActorError::Invalid("params.role: expected 'A' or 'B'".into())),
                };

                let rk = get_bytes(&req.params, "rk")?;
                if rk.len() != 32 {
                    return Err(ActorError::Invalid("params.rk: expected 32 bytes".into()));
                }
                let rk_arr: [u8; 32] = rk.as_slice().try_into()
                    .map_err(|_| ActorError::Invalid("params.rk: expected 32 bytes".into()))?;

                let pq_target_id = get_u32(&req.params, "pq_target_id")?;
                let pq_ct = get_bytes(&req.params, "pq_ct")?;
                let pq_epoch_ss = get_bytes(&req.params, "pq_epoch_ss")?;

                let peer_adv_id = get_u32(&req.params, "peer_adv_id")?;
                let peer_max_adv_id_seen = get_u32(&req.params, "peer_max_adv_id_seen")?;

                let known_v = get_json_data(&req.params, "known_targets")?;
                let consumed_v = get_json_data(&req.params, "consumed_targets")?;
                let tomb_v = get_json_data(&req.params, "tombstoned_targets")?;

                let known_targets: BTreeSet<u32> = parse_u32_list(&known_v, "params.known_targets")?.into_iter().collect();
                let consumed_targets: BTreeSet<u32> = parse_u32_list(&consumed_v, "params.consumed_targets")?.into_iter().collect();
                let tombstoned_targets: BTreeSet<u32> = parse_u32_list(&tomb_v, "params.tombstoned_targets")?.into_iter().collect();

                let commit_v = get_json_data(&req.params, "commit")?;
                let commit = parse_bool(&commit_v, "params.commit")?;

                let ck_pq_send = get_bytes(&req.params, "ck_pq_send")?;
                let ck_pq_recv = get_bytes(&req.params, "ck_pq_recv")?;
                if ck_pq_send.len() != 32 {
                    return Err(ActorError::Invalid("params.ck_pq_send: expected 32 bytes".into()));
                }
                if ck_pq_recv.len() != 32 {
                    return Err(ActorError::Invalid("params.ck_pq_recv: expected 32 bytes".into()));
                }
                let ck_pq_send_arr: [u8; 32] = ck_pq_send.as_slice().try_into()
                    .map_err(|_| ActorError::Invalid("params.ck_pq_send: expected 32 bytes".into()))?;
                let ck_pq_recv_arr: [u8; 32] = ck_pq_recv.as_slice().try_into()
                    .map_err(|_| ActorError::Invalid("params.ck_pq_recv: expected 32 bytes".into()))?;

                let out = suite2_scka::apply_pq_reseed(
                    &self.std,
                    &self.std,
                    role_is_a,
                    &rk_arr,
                    &pq_ct,
                    &pq_epoch_ss,
                    peer_adv_id,
                    peer_max_adv_id_seen,
                    &known_targets,
                    &consumed_targets,
                    &tombstoned_targets,
                    pq_target_id,
                    commit,
                    &ck_pq_send_arr,
                    &ck_pq_recv_arr,
                ).map_err(|e| match e {
                    suite2_scka::Suite2Reject::Code(c) => ActorError::Invalid(format!("reject: {c}")),
                })?;

                let consumed_after: Vec<u32> = out.consumed_targets_after.iter().cloned().collect();
                let tombstoned_after: Vec<u32> = out.tombstoned_targets_after.iter().cloned().collect();

                Ok(serde_json::json!({
                    "ck_pq_seed_a2b": jhex(&out.ck_pq_seed_a2b),
                    "ck_pq_seed_b2a": jhex(&out.ck_pq_seed_b2a),
                    "ck_pq_send_after": jhex(&out.ck_pq_send_after),
                    "ck_pq_recv_after": jhex(&out.ck_pq_recv_after),
                    "peer_max_adv_id_seen_after": { "type": "json", "data": { "u32": out.peer_max_adv_id_seen_after } },
                    "consumed_targets_after": { "type": "json", "data": consumed_after },
                    "tombstoned_targets_after": { "type": "json", "data": tombstoned_after }
                }))
            }
            "suite2.ooo_replay.run" => {
                let negotiated = get_json_data(&req.params, "negotiated")?;
                let pv = parse_u16(
                    negotiated
                        .get("protocol_version")
                        .ok_or_else(|| ActorError::Invalid("params.negotiated.protocol_version missing".into()))?,
                    "params.negotiated.protocol_version",
                )?;
                let sid = parse_u16(
                    negotiated
                        .get("suite_id")
                        .ok_or_else(|| ActorError::Invalid("params.negotiated.suite_id missing".into()))?,
                    "params.negotiated.suite_id",
                )?;
                if pv != 0x0500 || sid != 0x0002 {
                    return Err(ActorError::Invalid("reject: REJECT_S2_SUITE_MISMATCH".into()));
                }

                let session_id = get_bytes(&req.params, "session_id")?;
                if session_id.len() != 16 {
                    return Err(ActorError::Invalid("params.session_id: expected 16 bytes".into()));
                }
                let mut session_id_arr = [0u8; 16];
                session_id_arr.copy_from_slice(&session_id);

                let dh_pub = get_bytes(&req.params, "dh_pub")?;
                if dh_pub.len() != 32 {
                    return Err(ActorError::Invalid("params.dh_pub: expected 32 bytes".into()));
                }
                let mut dh_pub_arr = [0u8; 32];
                dh_pub_arr.copy_from_slice(&dh_pub);

                let hk_r = get_bytes(&req.params, "hk_r")?;
                if hk_r.len() != 32 {
                    return Err(ActorError::Invalid("params.hk_r: expected 32 bytes".into()));
                }
                let mut hk_r_arr = [0u8; 32];
                hk_r_arr.copy_from_slice(&hk_r);

                let ck_ec0 = get_bytes(&req.params, "ck_ec0")?;
                if ck_ec0.len() != 32 {
                    return Err(ActorError::Invalid("params.ck_ec0: expected 32 bytes".into()));
                }
                let mut ck_ec_arr = [0u8; 32];
                ck_ec_arr.copy_from_slice(&ck_ec0);

                let ck_pq0 = get_bytes(&req.params, "ck_pq0")?;
                if ck_pq0.len() != 32 {
                    return Err(ActorError::Invalid("params.ck_pq0: expected 32 bytes".into()));
                }
                let mut ck_pq_arr = [0u8; 32];
                ck_pq_arr.copy_from_slice(&ck_pq0);

                let nr0 = get_u32(&req.params, "nr0")?;

                let messages_v = get_json_data(&req.params, "messages")?;
                let msgs = messages_v
                    .as_array()
                    .ok_or_else(|| ActorError::Invalid("params.messages: expected array".into()))?;

                #[derive(Clone)]
                struct MsgSpec {
                    n: u32,
                    pn: u32,
                    body_pt: Vec<u8>,
                    tamper: String,
                }

                let mut specs: Vec<MsgSpec> = Vec::with_capacity(msgs.len());
                let mut max_n = 0u32;
                for (i, m) in msgs.iter().enumerate() {
                    let obj = m.as_object().ok_or_else(|| ActorError::Invalid(format!("params.messages[{i}]: expected object")))?;
                    let n_v = obj.get("n").ok_or_else(|| ActorError::Invalid(format!("params.messages[{i}].n missing")))?;
                    let pn_v = obj.get("pn").ok_or_else(|| ActorError::Invalid(format!("params.messages[{i}].pn missing")))?;
                    let pt_v = obj.get("body_pt_hex").ok_or_else(|| ActorError::Invalid(format!("params.messages[{i}].body_pt_hex missing")))?;
                    let t_v = obj.get("tamper").ok_or_else(|| ActorError::Invalid(format!("params.messages[{i}].tamper missing")))?;
                    let n = parse_u32_value(n_v, &format!("params.messages[{i}].n"))?;
                    let pn = parse_u32_value(pn_v, &format!("params.messages[{i}].pn"))?;
                    let body_pt = parse_hex_value(pt_v, &format!("params.messages[{i}].body_pt_hex"))?;
                    let tamper = t_v.as_str().unwrap_or("none").to_string();
                    max_n = max_n.max(n);
                    specs.push(MsgSpec { n, pn, body_pt, tamper });
                }

                // Precompute mk per N for sender-side ciphertext construction.
                let mut mk_map: Vec<[u8; 32]> = Vec::with_capacity((max_n + 1) as usize);
                let mut ck_ec_s = ck_ec_arr;
                let mut ck_pq_s = ck_pq_arr;
                for _ in 0..=max_n {
                    let (ck_ec_p, ck_pq_p, mk) =
                        suite2_ratchet::derive_mk_step(&self.std, &ck_ec_s, &ck_pq_s)
                            .map_err(|_| ActorError::Invalid("reject: REJECT_S2_OOO_DERIVE_FAIL".into()))?;
                    mk_map.push(mk);
                    ck_ec_s = ck_ec_p;
                    ck_pq_s = ck_pq_p;
                }

                let pq_bind = binding::pq_bind_sha512_32(&self.std, 0, &[]);
                let ad_hdr = binding::ad_hdr(&session_id_arr, pv, sid, &dh_pub_arr, 0, &pq_bind);
                let ad_body = binding::ad_body(&session_id_arr, pv, sid, &pq_bind);

                #[derive(Clone)]
                struct BuiltMsg {
                    n: u32,
                    hdr_ct: Vec<u8>,
                    body_ct: Vec<u8>,
                }
                let mut built: Vec<BuiltMsg> = Vec::with_capacity(specs.len());
                for spec in specs.iter() {
                    let mk = mk_map[spec.n as usize];
                    let mut hdr_pt = Vec::with_capacity(8);
                    hdr_pt.extend_from_slice(&spec.pn.to_be_bytes());
                    hdr_pt.extend_from_slice(&spec.n.to_be_bytes());
                    let nonce_hdr = suite2_ratchet::nonce_hdr(&self.std, &session_id_arr, &dh_pub_arr, spec.n);
                    let nonce_body = suite2_ratchet::nonce_body(&self.std, &session_id_arr, &dh_pub_arr, spec.n);
                    let mut hdr_ct = self.std.seal(&hk_r_arr, &nonce_hdr, &ad_hdr, &hdr_pt);
                    let mut body_ct = self.std.seal(&mk, &nonce_body, &ad_body, &spec.body_pt);

                    match spec.tamper.as_str() {
                        "none" => {}
                        "body" => {
                            if !body_ct.is_empty() {
                                body_ct[0] ^= 0x01;
                            }
                        }
                        "header" => {
                            if !hdr_ct.is_empty() {
                                hdr_ct[0] ^= 0x01;
                            }
                        }
                        _ => return Err(ActorError::Invalid("params.messages.tamper: expected none|body|header".into())),
                    }

                    built.push(BuiltMsg { n: spec.n, hdr_ct, body_ct });
                }

                let order_v = get_json_data(&req.params, "deliver_order")?;
                let order = parse_u32_list(&order_v, "params.deliver_order")?;

                let mut state = suite2_ratchet::Suite2RecvState {
                    session_id: session_id_arr,
                    protocol_version: pv,
                    suite_id: sid,
                    dh_pub: dh_pub_arr,
                    hk_r: hk_r_arr,
                    ck_ec: ck_ec_arr,
                    ck_pq: ck_pq_arr,
                    nr: nr0,
                    mkskipped: Vec::new(),
                };

                let mut results: Vec<serde_json::Value> = Vec::new();
                for idx in order {
                    let i = idx as usize;
                    if i >= built.len() {
                        return Err(ActorError::Invalid("params.deliver_order: index out of range".into()));
                    }
                    let msg = built[i].clone();
                    let out = suite2_ratchet::recv_nonboundary_ooo(&self.std, &self.std, &self.std, state, 0, &msg.hdr_ct, &msg.body_ct);
                    state = out.state;
                    if out.ok {
                        results.push(serde_json::json!({ "n": msg.n, "ok": true }));
                    } else {
                        results.push(serde_json::json!({ "n": msg.n, "ok": false, "reason_code": out.reason.unwrap_or("REJECT_S2_HDR_AUTH_FAIL") }));
                    }
                }

                Ok(serde_json::json!({
                    "results": { "type": "json", "data": results },
                    "final_state": { "type": "json", "data": {
                        "nr": { "u32": state.nr },
                        "ck_ec": to_hex(&state.ck_ec),
                        "ck_pq": to_hex(&state.ck_pq),
                        "mkskipped_len": { "u32": state.mkskipped.len() as u32 }
                    } }
                }))
            }
            "suite2.boundary.run" => {
                let negotiated = get_json_data(&req.params, "negotiated")?;
                let pv = parse_u16(
                    negotiated
                        .get("protocol_version")
                        .ok_or_else(|| ActorError::Invalid("params.negotiated.protocol_version missing".into()))?,
                    "params.negotiated.protocol_version",
                )?;
                let sid = parse_u16(
                    negotiated
                        .get("suite_id")
                        .ok_or_else(|| ActorError::Invalid("params.negotiated.suite_id missing".into()))?,
                    "params.negotiated.suite_id",
                )?;

                let role_v = get_json_data(&req.params, "role")?;
                let role_s = if let Some(s) = role_v.as_str() {
                    s
                } else if let Some(obj) = role_v.as_object() {
                    obj.get("role")
                        .and_then(|v| v.as_str())
                        .or_else(|| obj.get("value").and_then(|v| v.as_str()))
                        .ok_or_else(|| ActorError::Invalid("params.role: expected string".into()))?
                } else {
                    return Err(ActorError::Invalid("params.role: expected string".into()));
                };
                let role_is_a = match role_s {
                    "A" => true,
                    "B" => false,
                    _ => return Err(ActorError::Invalid("params.role: expected 'A' or 'B'".into())),
                };

                let session_id = get_bytes(&req.params, "session_id")?;
                if session_id.len() != 16 {
                    return Err(ActorError::Invalid("params.session_id: expected 16 bytes".into()));
                }
                let mut session_id_arr = [0u8; 16];
                session_id_arr.copy_from_slice(&session_id);

                let dh_pub = get_bytes(&req.params, "dh_pub")?;
                if dh_pub.len() != 32 {
                    return Err(ActorError::Invalid("params.dh_pub: expected 32 bytes".into()));
                }
                let mut dh_pub_arr = [0u8; 32];
                dh_pub_arr.copy_from_slice(&dh_pub);

                let hk_r = get_bytes(&req.params, "hk_r")?;
                if hk_r.len() != 32 {
                    return Err(ActorError::Invalid("params.hk_r: expected 32 bytes".into()));
                }
                let mut hk_r_arr = [0u8; 32];
                hk_r_arr.copy_from_slice(&hk_r);

                let rk = get_bytes(&req.params, "rk")?;
                if rk.len() != 32 {
                    return Err(ActorError::Invalid("params.rk: expected 32 bytes".into()));
                }
                let mut rk_arr = [0u8; 32];
                rk_arr.copy_from_slice(&rk);

                let ck_ec0 = get_bytes(&req.params, "ck_ec0")?;
                if ck_ec0.len() != 32 {
                    return Err(ActorError::Invalid("params.ck_ec0: expected 32 bytes".into()));
                }
                let mut ck_ec_arr = [0u8; 32];
                ck_ec_arr.copy_from_slice(&ck_ec0);

                let ck_pq_send0 = get_bytes(&req.params, "ck_pq_send0")?;
                if ck_pq_send0.len() != 32 {
                    return Err(ActorError::Invalid("params.ck_pq_send0: expected 32 bytes".into()));
                }
                let mut ck_pq_send_arr = [0u8; 32];
                ck_pq_send_arr.copy_from_slice(&ck_pq_send0);

                let ck_pq_recv0 = get_bytes(&req.params, "ck_pq_recv0")?;
                if ck_pq_recv0.len() != 32 {
                    return Err(ActorError::Invalid("params.ck_pq_recv0: expected 32 bytes".into()));
                }
                let mut ck_pq_recv_arr = [0u8; 32];
                ck_pq_recv_arr.copy_from_slice(&ck_pq_recv0);

                let nr0 = get_u32(&req.params, "nr0")?;

                let peer_max_adv_id_seen = get_u32(&req.params, "peer_max_adv_id_seen")?;
                let peer_adv_id = if req.params.get("peer_adv_id").is_some() {
                    get_u32(&req.params, "peer_adv_id")?
                } else {
                    peer_max_adv_id_seen.saturating_add(1)
                };

                let known_v = get_json_data(&req.params, "known_targets")?;
                let consumed_v = get_json_data(&req.params, "consumed_targets")?;
                let tomb_v = get_json_data(&req.params, "tombstoned_targets")?;

                let known_targets: BTreeSet<u32> = parse_u32_list(&known_v, "params.known_targets")?.into_iter().collect();
                let consumed_targets: BTreeSet<u32> = parse_u32_list(&consumed_v, "params.consumed_targets")?.into_iter().collect();
                let tombstoned_targets: BTreeSet<u32> = parse_u32_list(&tomb_v, "params.tombstoned_targets")?.into_iter().collect();

                let msg_v = get_json_data(&req.params, "message")?;
                let msg = msg_v
                    .as_object()
                    .ok_or_else(|| ActorError::Invalid("params.message: expected object".into()))?;

                let n_v = msg.get("n").ok_or_else(|| ActorError::Invalid("params.message.n missing".into()))?;
                let pn_v = msg.get("pn").ok_or_else(|| ActorError::Invalid("params.message.pn missing".into()))?;
                let flags_v = msg.get("flags").ok_or_else(|| ActorError::Invalid("params.message.flags missing".into()))?;
                let pq_prefix_v = msg
                    .get("pq_prefix_hex")
                    .ok_or_else(|| ActorError::Invalid("params.message.pq_prefix_hex missing".into()))?;
                let body_pt_v = msg
                    .get("body_pt_hex")
                    .ok_or_else(|| ActorError::Invalid("params.message.body_pt_hex missing".into()))?;
                let epoch_v = msg
                    .get("pq_epoch_ss")
                    .ok_or_else(|| ActorError::Invalid("params.message.pq_epoch_ss missing".into()))?;
                let tamper_v = msg.get("tamper");

                let n = parse_u32_value(n_v, "params.message.n")?;
                let pn = parse_u32_value(pn_v, "params.message.pn")?;
                let flags = if let Some(obj) = flags_v.as_object() {
                    if let Some(n) = obj
                        .get("u16")
                        .and_then(|x| x.as_u64())
                        .or_else(|| obj.get("value").and_then(|x| x.as_u64()))
                        .or_else(|| obj.get("n").and_then(|x| x.as_u64()))
                    {
                        if n <= u16::MAX as u64 {
                            n as u16
                        } else {
                            return Err(ActorError::Invalid("params.message.flags: expected u16".into()));
                        }
                    } else {
                        parse_u16(flags_v, "params.message.flags")?
                    }
                } else {
                    parse_u16(flags_v, "params.message.flags")?
                };

                if n < nr0 {
                    return Err(ActorError::Invalid("params.message.n: expected >= nr0".into()));
                }

                let pq_prefix = parse_hex_value(pq_prefix_v, "params.message.pq_prefix_hex")?;
                let body_pt = parse_hex_value(body_pt_v, "params.message.body_pt_hex")?;
                let pq_epoch_ss = parse_hex_value(epoch_v, "params.message.pq_epoch_ss")?;
                if pq_epoch_ss.len() != 32 {
                    return Err(ActorError::Invalid("params.message.pq_epoch_ss: expected 32 bytes".into()));
                }

                let tamper = tamper_v.and_then(|v| v.as_str()).unwrap_or("none").to_string();

                let mut mk = [0u8; 32];
                let mut ck_ec_s = ck_ec_arr;
                let mut ck_pq_s = ck_pq_recv_arr;
                for i in nr0..=n {
                    let (ck_ec_p, ck_pq_p, mk_i) =
                        suite2_ratchet::derive_mk_step(&self.std, &ck_ec_s, &ck_pq_s)
                            .map_err(|_| ActorError::Invalid("reject: REJECT_S2_BOUNDARY_DERIVE_FAIL".into()))?;
                    if i == n {
                        mk = mk_i;
                    }
                    ck_ec_s = ck_ec_p;
                    ck_pq_s = ck_pq_p;
                }

                let pq_bind = binding::pq_bind_sha512_32(&self.std, flags, &pq_prefix);
                let ad_hdr = binding::ad_hdr(&session_id_arr, pv, sid, &dh_pub_arr, flags, &pq_bind);
                let ad_body = binding::ad_body(&session_id_arr, pv, sid, &pq_bind);

                let mut hdr_pt = Vec::with_capacity(8);
                hdr_pt.extend_from_slice(&pn.to_be_bytes());
                hdr_pt.extend_from_slice(&n.to_be_bytes());
                let nonce_hdr = suite2_ratchet::nonce_hdr(&self.std, &session_id_arr, &dh_pub_arr, n);
                let nonce_body = suite2_ratchet::nonce_body(&self.std, &session_id_arr, &dh_pub_arr, n);

                let mut hdr_ct = self.std.seal(&hk_r_arr, &nonce_hdr, &ad_hdr, &hdr_pt);
                let mut body_ct = self.std.seal(&mk, &nonce_body, &ad_body, &body_pt);

                match tamper.as_str() {
                    "none" => {}
                    "body" => {
                        if !body_ct.is_empty() {
                            body_ct[0] ^= 0x01;
                        }
                    }
                    "header" => {
                        if !hdr_ct.is_empty() {
                            hdr_ct[0] ^= 0x01;
                        }
                    }
                    _ => return Err(ActorError::Invalid("params.message.tamper: expected none|body|header".into())),
                }

                let state = suite2_ratchet::Suite2BoundaryState {
                    session_id: session_id_arr,
                    protocol_version: pv,
                    suite_id: sid,
                    dh_pub: dh_pub_arr,
                    hk_r: hk_r_arr,
                    rk: rk_arr,
                    ck_ec: ck_ec_arr,
                    ck_pq_send: ck_pq_send_arr,
                    ck_pq_recv: ck_pq_recv_arr,
                    nr: nr0,
                    role_is_a,
                    peer_max_adv_id_seen,
                    known_targets,
                    consumed_targets,
                    tombstoned_targets,
                };

                let out = suite2_ratchet::recv_boundary_in_order(
                    &self.std,
                    &self.std,
                    &self.std,
                    state,
                    flags,
                    &pq_prefix,
                    &hdr_ct,
                    &body_ct,
                    &pq_epoch_ss,
                    peer_adv_id,
                );

                if !out.ok {
                    return Err(ActorError::Invalid(format!(
                        "reject: {}",
                        out.reason.unwrap_or("REJECT_S2_HDR_AUTH_FAIL")
                    )));
                }

                let consumed_after: Vec<u32> = out.state.consumed_targets.iter().cloned().collect();
                let tombstoned_after: Vec<u32> = out.state.tombstoned_targets.iter().cloned().collect();

                Ok(serde_json::json!({
                    "final_state": { "type": "json", "data": {
                        "nr": { "u32": out.state.nr },
                        "ck_ec": to_hex(&out.state.ck_ec),
                        "ck_pq_send": to_hex(&out.state.ck_pq_send),
                        "ck_pq_recv": to_hex(&out.state.ck_pq_recv),
                        "peer_max_adv_id_seen": { "u32": out.state.peer_max_adv_id_seen },
                        "consumed_targets": consumed_after,
                        "tombstoned_targets": tombstoned_after
                    } }
                }))
            }
            "suite2.parse.check" => {
                let msg = get_bytes(&req.params, "msg")?;
                let parsed = suite2_parse::decode_suite2_ratchet_message(&msg)
                    .map_err(|e| ActorError::Invalid(format!("reject: {e}")))?;

                Ok(serde_json::json!({
                    "flags": { "type": "json", "data": { "u16": parsed.flags } },
                    "pq_prefix_len": { "type": "json", "data": { "u32": parsed.pq_prefix.len() as u32 } },
                    "hdr_ct_len": { "type": "json", "data": { "u32": parsed.hdr_ct.len() as u32 } },
                    "body_ct_len": { "type": "json", "data": { "u32": parsed.body_ct.len() as u32 } },
                    "has_pq_adv": { "type": "json", "data": { "bool": parsed.pq_adv_id.is_some() } },
                    "has_pq_ctxt": { "type": "json", "data": { "bool": parsed.pq_target_id.is_some() } }
                }))
            }
            "suite2.establish.run" => {
                let msg_type = if req.params.get("msg_type").is_some() {
                    get_u16(&req.params, "msg_type")?
                } else {
                    return Err(ActorError::Invalid("reject: REJECT_S2_ESTABLISH_BAD_MSG_TYPE".into()));
                };
                if msg_type != 0x01 {
                    return Err(ActorError::Invalid("reject: REJECT_S2_ESTABLISH_BAD_MSG_TYPE".into()));
                }

                let negotiated = get_json_data(&req.params, "negotiated")?;
                let pv = parse_u16(
                    negotiated
                        .get("protocol_version")
                        .ok_or_else(|| ActorError::Invalid("params.negotiated.protocol_version missing".into()))?,
                    "params.negotiated.protocol_version",
                )?;
                let sid = parse_u16(
                    negotiated
                        .get("suite_id")
                        .ok_or_else(|| ActorError::Invalid("params.negotiated.suite_id missing".into()))?,
                    "params.negotiated.suite_id",
                )?;

                let session_id = get_bytes(&req.params, "session_id")?;
                let dh_init = get_bytes(&req.params, "dh_init")?;
                let pq_init_ss = get_bytes(&req.params, "pq_init_ss")?;
                let pq_kem_pub_id = if req.params.get("pq_kem_pub_id").is_some() {
                    get_bytes(&req.params, "pq_kem_pub_id")?
                } else {
                    return Err(ActorError::Invalid(
                        "reject: REJECT_S2_ESTABLISH_PQ_BIND_MISSING".into(),
                    ));
                };
                let pq_prekey_id = if req.params.get("pq_prekey_id").is_some() {
                    get_u32(&req.params, "pq_prekey_id")?
                } else {
                    return Err(ActorError::Invalid(
                        "reject: REJECT_S2_ESTABLISH_PQ_BIND_MISSING".into(),
                    ));
                };
                let dh_self_pub = get_bytes(&req.params, "dh_self_pub")?;
                let dh_peer_pub = get_bytes(&req.params, "dh_peer_pub")?;

                if session_id.len() != 16
                    || dh_init.len() != 32
                    || pq_init_ss.len() != 32
                    || pq_kem_pub_id.len() != 32
                    || dh_self_pub.len() != 32
                    || dh_peer_pub.len() != 32
                {
                    return Err(ActorError::Invalid("reject: REJECT_S2_ESTABLISH_BAD_INPUT_LEN".into()));
                }

                let authenticated = if let Some(v) = req.params.get("authenticated") {
                    parse_bool(v, "params.authenticated")?
                } else {
                    false
                };
                if !authenticated {
                    return Err(ActorError::Invalid(
                        "reject: REJECT_S2_ESTABLISH_UNAUTHENTICATED".into(),
                    ));
                }

                let role_v = req
                    .params
                    .get("role")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| ActorError::Invalid("params.role missing".into()))?;
                let role_is_a = match role_v {
                    "A" => true,
                    "B" => false,
                    _ => return Err(ActorError::Invalid("params.role: expected A or B".into())),
                };

                if let Some(policy_v) = req.params.get("policy") {
                    let policy = policy_v
                        .as_object()
                        .ok_or_else(|| ActorError::Invalid("params.policy: expected object".into()))?;
                    let local_supports = policy
                        .get("local_supports_suite2")
                        .and_then(|x| x.as_bool())
                        .unwrap_or(true);
                    if !local_supports {
                        return Err(ActorError::Invalid("reject: REJECT_S2_LOCAL_UNSUPPORTED".into()));
                    }
                    let peer_support = policy
                        .get("peer_support")
                        .and_then(|x| x.as_str())
                        .unwrap_or("true");
                    if peer_support != "true" {
                        return Err(ActorError::Invalid("reject: REJECT_S2_PEER_UNSUPPORTED".into()));
                    }
                }

                if pv != 0x0500 || sid != 0x0002 {
                    return Err(ActorError::Invalid("reject: REJECT_S2_SUITE_MISMATCH".into()));
                }

                let bound_v = req
                    .params
                    .get("bound")
                    .ok_or_else(|| {
                        ActorError::Invalid("reject: REJECT_S2_ESTABLISH_PQ_BIND_MISSING".into())
                    })?;
                let bound = bound_v
                    .as_object()
                    .ok_or_else(|| ActorError::Invalid("params.bound: expected object".into()))?;
                let bpv = parse_u16(
                    bound
                        .get("protocol_version")
                        .ok_or_else(|| ActorError::Invalid("params.bound.protocol_version missing".into()))?,
                    "params.bound.protocol_version",
                )?;
                let bsid = parse_u16(
                    bound
                        .get("suite_id")
                        .ok_or_else(|| ActorError::Invalid("params.bound.suite_id missing".into()))?,
                    "params.bound.suite_id",
                )?;
                if bpv != pv || bsid != sid {
                    return Err(ActorError::Invalid("reject: REJECT_S2_AD_MISMATCH".into()));
                }
                let bound_pq_kem_pub_id = get_bytes(bound_v, "pq_kem_pub_id")
                    .map_err(|_| ActorError::Invalid("reject: REJECT_S2_ESTABLISH_PQ_BIND_MISMATCH".into()))?;
                let bound_pq_prekey_id = get_u32(bound_v, "pq_prekey_id")
                    .map_err(|_| ActorError::Invalid("reject: REJECT_S2_ESTABLISH_PQ_BIND_MISMATCH".into()))?;
                if bound_pq_kem_pub_id.len() != 32
                    || bound_pq_kem_pub_id != pq_kem_pub_id
                    || bound_pq_prekey_id != pq_prekey_id
                {
                    return Err(ActorError::Invalid(
                        "reject: REJECT_S2_ESTABLISH_PQ_BIND_MISMATCH".into(),
                    ));
                }

                let state = suite2_establish::init_from_base_handshake(
                    &self.std,
                    role_is_a,
                    pv,
                    sid,
                    &session_id,
                    &dh_init,
                    &pq_init_ss,
                    &dh_self_pub,
                    &dh_peer_pub,
                    authenticated,
                )
                .map_err(|e| ActorError::Invalid(format!("reject: {e}")))?;

                let mut sid_arr = [0u8; 16];
                sid_arr.copy_from_slice(&session_id);
                self.suite2_sessions.insert(sid_arr, state);

                Ok(serde_json::json!({
                    "session_id": session_id_to_string(&sid_arr),
                }))
            }
            "suite2.e2e.recv" => {
                let negotiated = get_json_data(&req.params, "negotiated")?;
                let pv = parse_u16(
                    negotiated
                        .get("protocol_version")
                        .ok_or_else(|| ActorError::Invalid("params.negotiated.protocol_version missing".into()))?,
                    "params.negotiated.protocol_version",
                )?;
                let sid = parse_u16(
                    negotiated
                        .get("suite_id")
                        .ok_or_else(|| ActorError::Invalid("params.negotiated.suite_id missing".into()))?,
                    "params.negotiated.suite_id",
                )?;

                let session_id_opt = req.params.get("session_id").and_then(|v| v.as_str());
                let session_id_arr_opt = if let Some(s) = session_id_opt {
                    Some(session_id_from_string(s)?)
                } else {
                    None
                };

                let recv_state = if req.params.get("recv_state").is_some() {
                    let state_v = get_json_data(&req.params, "recv_state")?;
                    parse_suite2_recv_state(&state_v)?
                } else if let Some(sid) = session_id_arr_opt {
                    self.suite2_sessions
                        .get(&sid)
                        .map(|e| e.recv.clone())
                        .ok_or_else(|| ActorError::Invalid("params.recv_state missing".into()))?
                } else {
                    return Err(ActorError::Invalid("params.recv_state missing".into()));
                };

                if let Some(sid) = session_id_arr_opt {
                    if recv_state.session_id != sid {
                        return Err(ActorError::Invalid("params.session_id does not match recv_state.session_id".into()));
                    }
                }

                let send_state_for_store = if let Some(sid) = session_id_arr_opt {
                    if req.params.get("send_state").is_some() {
                        let send_v = get_json_data(&req.params, "send_state")?;
                        Some(parse_suite2_send_state(&send_v)?)
                    } else {
                        self.suite2_sessions.get(&sid).map(|e| e.send.clone())
                    }
                } else {
                    None
                };

                let wire = get_bytes(&req.params, "wire_hex")?;
                let pq_epoch_ss = if req.params.get("pq_epoch_ss").is_some() {
                    let v = get_bytes(&req.params, "pq_epoch_ss")?;
                    if v.len() != 32 {
                        return Err(ActorError::Invalid("params.pq_epoch_ss: expected 32 bytes".into()));
                    }
                    Some(v)
                } else {
                    None
                };
                let peer_adv_id = if req.params.get("peer_adv_id").is_some() {
                    Some(get_u32(&req.params, "peer_adv_id")?)
                } else {
                    None
                };

                let dur_store_dir = if test_hooks_enabled() {
                    std::env::var("QSL_DUR_STORE_DIR").ok().filter(|s| !s.is_empty())
                } else {
                    None
                };
                let dur_digest = if dur_store_dir.is_some() {
                    hex_lower(&sha3_256(&wire))
                } else {
                    String::new()
                };
                let dur_seen = dur_store_dir.as_ref().map(|d| {
                    let sid_s = session_id_to_string(&recv_state.session_id);
                    dur_seen_path(Path::new(d), &self.name, &sid_s)
                });
                if let Some(p) = dur_seen.as_ref() {
                    if dur_seen_contains(p, &dur_digest)? {
                        return Err(ActorError::Crypto("replay (durable)".into()));
                    }
                }

                let out = suite2_ratchet::recv_wire(
                    &self.std,
                    &self.std,
                    &self.std,
                    recv_state,
                    &wire,
                    pq_epoch_ss.as_deref(),
                    peer_adv_id,
                ).map_err(|e| ActorError::Invalid(format!("reject: {e}")))?;

                if let Some(dir) = dur_store_dir.as_ref() {
                    let sid_s = session_id_to_string(&out.state.session_id);
                    let path = dur_scka_path(Path::new(dir), &self.name, &sid_s);
                    let prev = load_dur_scka(&path)?;
                    let merged = merge_dur_scka(prev, &out.state);
                    store_dur_scka(&path, &merged)?;
                }
                if let Some(p) = dur_seen.as_ref() {
                    dur_seen_append(p, &dur_digest)?;
                }

                if let Some(sid) = session_id_arr_opt {
                    let send_state = send_state_for_store
                        .ok_or_else(|| ActorError::Invalid("params.send_state missing for new suite2 session".into()))?;
                    self.suite2_sessions.insert(sid, suite2_state::Suite2SessionState {
                        send: send_state,
                        recv: out.state.clone(),
                    });
                }

                let mkskipped_out: Vec<serde_json::Value> = out
                    .state
                    .mkskipped
                    .iter()
                    .map(|e| serde_json::json!({
                        "dh_pub": to_hex(&e.dh_pub),
                        "n": { "u32": e.n },
                        "mk": to_hex(&e.mk)
                    }))
                    .collect();

                Ok(serde_json::json!({
                    "plaintext_hex": { "type": "hex", "data": to_hex(&out.plaintext) },
                    "meta": { "type": "json", "data": {
                        "flags": { "u16": out.flags },
                        "pn": { "u32": out.pn },
                        "n": { "u32": out.n }
                    } },
                    "new_state": { "type": "json", "data": {
                        "session_id": to_hex(&out.state.session_id),
                        "dh_pub": to_hex(&out.state.dh_pub),
                        "hk_r": to_hex(&out.state.hk_r),
                        "rk": to_hex(&out.state.rk),
                        "ck_ec": to_hex(&out.state.ck_ec),
                        "ck_pq_send": to_hex(&out.state.ck_pq_send),
                        "ck_pq_recv": to_hex(&out.state.ck_pq_recv),
                        "nr": { "u32": out.state.nr },
                        "role": if out.state.role_is_a { "A" } else { "B" },
                        "peer_max_adv_id_seen": { "u32": out.state.peer_max_adv_id_seen },
                        "known_targets": out.state.known_targets.iter().cloned().collect::<Vec<u32>>(),
                        "consumed_targets": out.state.consumed_targets.iter().cloned().collect::<Vec<u32>>(),
                        "tombstoned_targets": out.state.tombstoned_targets.iter().cloned().collect::<Vec<u32>>(),
                        "mkskipped": mkskipped_out
                    } }
                }))
            }
            "suite2.e2e.send" => {
                let negotiated = get_json_data(&req.params, "negotiated")?;
                let pv = parse_u16(
                    negotiated
                        .get("protocol_version")
                        .ok_or_else(|| ActorError::Invalid("params.negotiated.protocol_version missing".into()))?,
                    "params.negotiated.protocol_version",
                )?;
                let sid = parse_u16(
                    negotiated
                        .get("suite_id")
                        .ok_or_else(|| ActorError::Invalid("params.negotiated.suite_id missing".into()))?,
                    "params.negotiated.suite_id",
                )?;
                let session_id_opt = req.params.get("session_id").and_then(|v| v.as_str());
                let session_id_arr_opt = if let Some(s) = session_id_opt {
                    Some(session_id_from_string(s)?)
                } else {
                    None
                };

                let send_state = if req.params.get("send_state").is_some() {
                    let state_v = get_json_data(&req.params, "send_state")?;
                    parse_suite2_send_state(&state_v)?
                } else if let Some(sid) = session_id_arr_opt {
                    self.suite2_sessions
                        .get(&sid)
                        .map(|e| e.send.clone())
                        .ok_or_else(|| ActorError::Invalid("params.send_state missing".into()))?
                } else {
                    return Err(ActorError::Invalid("params.send_state missing".into()));
                };

                if let Some(sid) = session_id_arr_opt {
                    if send_state.session_id != sid {
                        return Err(ActorError::Invalid("params.session_id does not match send_state.session_id".into()));
                    }
                }

                let recv_state_for_store = if let Some(sid) = session_id_arr_opt {
                    if req.params.get("recv_state").is_some() {
                        let recv_v = get_json_data(&req.params, "recv_state")?;
                        Some(parse_suite2_recv_state(&recv_v)?)
                    } else {
                        self.suite2_sessions.get(&sid).map(|e| e.recv.clone())
                    }
                } else {
                    None
                };

                let flags = if req.params.get("flags").is_some() {
                    get_u16(&req.params, "flags")?
                } else {
                    0u16
                };

                let plaintext = get_bytes(&req.params, "plaintext_hex")?;

                let out = suite2_ratchet::send_wire(
                    &self.std,
                    &self.std,
                    &self.std,
                    send_state,
                    flags,
                    &plaintext,
                ).map_err(|e| ActorError::Invalid(format!("reject: {e}")))?;

                if let Some(sid) = session_id_arr_opt {
                    let recv_state = recv_state_for_store
                        .ok_or_else(|| ActorError::Invalid("params.recv_state missing for new suite2 session".into()))?;
                    self.suite2_sessions.insert(sid, suite2_state::Suite2SessionState {
                        send: out.state.clone(),
                        recv: recv_state,
                    });
                }

                Ok(serde_json::json!({
                    "wire_hex": { "type": "hex", "data": to_hex(&out.wire) },
                    "meta": { "type": "json", "data": {
                        "flags": { "u16": out.flags },
                        "pn": { "u32": out.pn },
                        "n": { "u32": out.n }
                    } },
                    "new_state": { "type": "json", "data": {
                        "session_id": to_hex(&out.state.session_id),
                        "dh_pub": to_hex(&out.state.dh_pub),
                        "hk_s": to_hex(&out.state.hk_s),
                        "ck_ec": to_hex(&out.state.ck_ec),
                        "ck_pq": to_hex(&out.state.ck_pq),
                        "ns": { "u32": out.state.ns },
                        "pn": { "u32": out.state.pn }
                    } }
                }))
            }
            "suite2.kdf_ec_ck" => {
                let ck = get_bytes(&req.params, "CK_ec")?;
                let ck_p = kmac32(&self.std, &ck, "QSP5.0/CK", &[0x01]);
                let ec_mk = kmac32(&self.std, &ck, "QSP5.0/MK", &[0x02]);
                Ok(serde_json::json!({ "CK_ec_prime": jhex(&ck_p), "ec_mk": jhex(&ec_mk) }))
            }
            "suite2.kdf_pq_ck" => {
                let ck = get_bytes(&req.params, "CK_pq")?;
                let ck_p = kmac32(&self.std, &ck, "QSP5.0/PQCK", &[0x01]);
                let pq_mk = kmac32(&self.std, &ck, "QSP5.0/PQMK", &[0x02]);
                Ok(serde_json::json!({ "CK_pq_prime": jhex(&ck_p), "pq_mk": jhex(&pq_mk) }))
            }
            "suite2.kdf_hybrid" => {
                let ec_mk = get_bytes(&req.params, "ec_mk")?;
                let pq_mk = get_bytes(&req.params, "pq_mk")?;
                let mut data = Vec::with_capacity(pq_mk.len() + 1);
                data.extend_from_slice(&pq_mk);
                data.push(0x01);
                let mk = kmac32(&self.std, &ec_mk, "QSP5.0/HYBRID", &data);
                Ok(serde_json::json!({ "mk": jhex(&mk) }))
            }
            "suite2.kdf_rk_dh" => {
                let rk = get_bytes(&req.params, "RK")?;
                let dh = get_bytes(&req.params, "dh_out")?;
                let tmp = kmac64(&self.std, &rk, "QSP5.0/RKDH", &dh);
                let rk_p = &tmp[0..32];
                let ck0  = &tmp[32..64];
                Ok(serde_json::json!({ "RK_prime": jhex(rk_p), "CK_ec0": jhex(ck0) }))
            }
            "suite2.kdf_rk_pq" => {
                let rk = get_bytes(&req.params, "RK")?;
                let ss = get_bytes(&req.params, "pq_ss")?;
                let mut data = Vec::with_capacity(ss.len() + 1);
                data.extend_from_slice(&ss);
                data.push(0x01);
                let rk_p = kmac32(&self.std, &rk, "QSP5.0/RKPQ", &data);
                Ok(serde_json::json!({ "RK_prime": jhex(&rk_p) }))
            }
            "suite2.kdf_pq_reseed" => {
                let rk = get_bytes(&req.params, "RK")?;
                let tid = get_u32(&req.params, "pq_target_id")?;
                let ct  = get_bytes(&req.params, "pq_ct")?;
                let ss  = get_bytes(&req.params, "pq_epoch_ss")?;

                let h = self.std.sha512(&ct);
                let mut ctx = Vec::new();
                ctx.extend_from_slice(b"QSP5.0/SCKA/CTXT");
                ctx.extend_from_slice(&tid.to_be_bytes());
                ctx.extend_from_slice(&h[0..32]);
                ctx.extend_from_slice(&ss);

                let a2b = kmac32(&self.std, &rk, "QSP5.0/PQSEED/A->B", &ctx);
                let b2a = kmac32(&self.std, &rk, "QSP5.0/PQSEED/B->A", &ctx);

                Ok(serde_json::json!({ "CK_pq_seed_A2B": jhex(&a2b), "CK_pq_seed_B2A": jhex(&b2a) }))
            }

// ---------------------------
// SCKA logic conformance ops
// ---------------------------
"scka.initial_epoch.map" => {
                let session_id = get_bytes(&req.params, "session_id")?;
                let dh_init = get_bytes(&req.params, "dh_init")?;
                let pq_init_ss = get_bytes(&req.params, "pq_init_ss")?;
                let dh_self_pub = get_bytes(&req.params, "dh_self_pub")?;
                let dh_peer_pub = get_bytes(&req.params, "dh_peer_pub")?;

                let authenticated = if req.params.get("authenticated").is_some() {
                    let auth_v = get_json_data(&req.params, "authenticated")?;
                    parse_bool(&auth_v, "params.authenticated")?
                } else {
                    false
                };

                let role_v = get_json_data(&req.params, "role")?;
                let role_s = if let Some(s) = role_v.as_str() {
                    s
                } else if let Some(obj) = role_v.as_object() {
                    obj.get("role")
                        .and_then(|v| v.as_str())
                        .or_else(|| obj.get("value").and_then(|v| v.as_str()))
                        .ok_or_else(|| ActorError::Invalid("params.role: expected string".into()))?
                } else {
                    return Err(ActorError::Invalid("params.role: expected string".into()));
                };
                let role_is_a = match role_s {
                    "A" => true,
                    "B" => false,
                    _ => return Err(ActorError::Invalid("params.role: expected A or B".into())),
                };

                let state = suite2_establish::init_from_base_handshake(
                    &self.std,
                    role_is_a,
                    0x0500,
                    0x0002,
                    &session_id,
                    &dh_init,
                    &pq_init_ss,
                    &dh_self_pub,
                    &dh_peer_pub,
                    authenticated,
                )
                .map_err(|e| ActorError::Invalid(format!("reject: {e}")))?;

                Ok(serde_json::json!({
                    "rk": jhex(&state.recv.rk),
                    "ck_pq_send": jhex(&state.recv.ck_pq_send),
                    "ck_pq_recv": jhex(&state.recv.ck_pq_recv),
                    "peer_max_adv_id_seen": { "type": "json", "data": { "u32": state.recv.peer_max_adv_id_seen } },
                    "known_targets": { "type": "json", "data": [] },
                    "consumed_targets": { "type": "json", "data": [] },
                    "tombstoned_targets": { "type": "json", "data": [] },
                    "local_next_adv_id": { "type": "json", "data": { "u32": 0 } }
                }))
            },
"scka.kem.check" => {
                // CAT-SCKA-KEM-001: ML-KEM-768 correctness checks for SCKA KEM operations.
                // Inputs:
                // - d_enc, z_enc: 32-byte deterministic keygen seeds
                // - m: 32-byte deterministic encapsulation message
                // - d_decap, z_decap: optional alternate seeds for decapsulation keygen (defaults to enc seeds)
                // - tamper_ct: optional {"bool": true|false} (if true, flips a bit in ciphertext before decapsulation)

                let d_enc = get_bytes(&req.params, "d_enc")?;
                if d_enc.len() != 32 {
                    return Err(ActorError::Invalid("reject: REJECT_SCKA_KEM_BAD_D".into()));
                }
                let z_enc = get_bytes(&req.params, "z_enc")?;
                if z_enc.len() != 32 {
                    return Err(ActorError::Invalid("reject: REJECT_SCKA_KEM_BAD_Z".into()));
                }
                let m = get_bytes(&req.params, "m")?;
                if m.len() != 32 {
                    return Err(ActorError::Invalid("reject: REJECT_SCKA_KEM_BAD_M".into()));
                }

                let d_decap = if req.params.get("d_decap").is_some() {
                    let v = get_bytes(&req.params, "d_decap")?;
                    if v.len() != 32 {
                        return Err(ActorError::Invalid("reject: REJECT_SCKA_KEM_BAD_D_DECAP".into()));
                    }
                    v
                } else {
                    d_enc.clone()
                };
                let z_decap = if req.params.get("z_decap").is_some() {
                    let v = get_bytes(&req.params, "z_decap")?;
                    if v.len() != 32 {
                        return Err(ActorError::Invalid("reject: REJECT_SCKA_KEM_BAD_Z_DECAP".into()));
                    }
                    v
                } else {
                    z_enc.clone()
                };

                let mut tamper_ct = false;
                if req.params.get("tamper_ct").is_some() {
                    let tv = get_json_data(&req.params, "tamper_ct")?;
                    if let Some(b) = tv.as_bool() {
                        tamper_ct = b;
                    } else if let Some(obj) = tv.as_object() {
                        if let Some(b) = obj.get("bool").and_then(|x| x.as_bool()) {
                            tamper_ct = b;
                        }
                    }
                }

                let d_enc_arr: [u8; 32] = d_enc.as_slice().try_into().map_err(|_| ActorError::Invalid("reject: REJECT_SCKA_KEM_BAD_D".into()))?;
                let z_enc_arr: [u8; 32] = z_enc.as_slice().try_into().map_err(|_| ActorError::Invalid("reject: REJECT_SCKA_KEM_BAD_Z".into()))?;
                let m_arr: [u8; 32] = m.as_slice().try_into().map_err(|_| ActorError::Invalid("reject: REJECT_SCKA_KEM_BAD_M".into()))?;
                let d_decap_arr: [u8; 32] = d_decap.as_slice().try_into().map_err(|_| ActorError::Invalid("reject: REJECT_SCKA_KEM_BAD_D_DECAP".into()))?;
                let z_decap_arr: [u8; 32] = z_decap.as_slice().try_into().map_err(|_| ActorError::Invalid("reject: REJECT_SCKA_KEM_BAD_Z_DECAP".into()))?;

                let (_dk_enc, ek_enc) = MlKem768::generate_deterministic(&B32::from(d_enc_arr), &B32::from(z_enc_arr));
                // NOTE: `encapsulate_deterministic()` returns (ct, ss).
                let (ct, ss_out) = ek_enc
                    .encapsulate_deterministic(&B32::from(m_arr))
                    .map_err(|_| ActorError::Invalid("reject: REJECT_SCKA_KEM_ENCAP_FAIL".into()))?;

				// NOTE: prefer `as_slice()` here to avoid ambiguous `AsRef` inference on hybrid_array::Array.
				let mut ct_bytes: Vec<u8> = ct.as_slice().to_vec();
                if tamper_ct && !ct_bytes.is_empty() {
                    ct_bytes[0] ^= 0x01;
                }

                let ct_enc = ml_kem::Ciphertext::<MlKem768>::try_from(ct_bytes.as_slice())
                    .map_err(|_| ActorError::Invalid("reject: REJECT_SCKA_KEM_BAD_CT".into()))?;

                let (dk_decap, _) = MlKem768::generate_deterministic(&B32::from(d_decap_arr), &B32::from(z_decap_arr));
                let ss_in = dk_decap
                    .decapsulate(&ct_enc)
                    .map_err(|_| ActorError::Invalid("reject: REJECT_SCKA_KEM_DECAP_FAIL".into()))?;

                let ss_out_slice: &[u8] = ss_out.as_slice();
                let ss_in_slice: &[u8] = ss_in.as_slice();
                let ss_match = ss_in_slice == ss_out_slice;

                Ok(serde_json::json!({
                    "pq_ct": jhex(&ct_bytes),
                    "pq_epoch_ss_out": jhex(ss_out_slice),
                    "pq_epoch_ss_in": jhex(ss_in_slice),
                    "ss_match": serde_json::json!({"type": "json", "data": {"bool": ss_match}})
                }))
            },
            "scka.peer_adv.process" => {
    let state_typed = req.params.get("state").cloned().unwrap_or(serde_json::Value::Null);
    let state = get_json_data(&req.params, "state")?;
    let msg = get_json_data(&req.params, "msg")?;

    let peer_max = state.get("peer_max_adv_id_seen")
        .and_then(|x| x.as_u64())
        .ok_or_else(|| ActorError::Invalid("params.state.peer_max_adv_id_seen: expected number".into()))?;
    let peer_adv = msg.get("peer_adv_id")
        .and_then(|x| x.as_u64())
        .ok_or_else(|| ActorError::Invalid("params.msg.peer_adv_id: expected number".into()))?;

    if peer_adv > peer_max {
        Ok(serde_json::json!({
            "peer_max_adv_id_seen": { "type": "json", "data": { "u32": peer_adv } }
        }))
    } else {
        Err(ActorError::InvalidWithResult(
            "reject: REJECT_SCKA_ADV_NONMONOTONIC".into(),
            serde_json::json!({ "state": state_typed })
        ))
    }
}

"scka.decap.check" => {
    let state_typed = req.params.get("state").cloned().unwrap_or(serde_json::Value::Null);
    let state = get_json_data(&req.params, "state")?;
    let msg = get_json_data(&req.params, "msg")?;

    let target = msg.get("pq_target_id")
        .and_then(|x| x.as_u64())
        .ok_or_else(|| ActorError::Invalid("params.msg.pq_target_id: expected number".into()))?;
    let target_s = target.to_string();

    // tombstone check first
    if let Some(ts) = state.get("tombstones").and_then(|x| x.as_array()) {
        if ts.iter().any(|v| v.as_u64() == Some(target)) {
            return Err(ActorError::InvalidWithResult(
                "reject: REJECT_SCKA_TARGET_TOMBSTONED".into(),
                serde_json::json!({ "state": state_typed })
            ));
        }
    }

    let advkeys = state.get("advkeys")
        .and_then(|x| x.as_object())
        .ok_or_else(|| ActorError::Invalid("params.state.advkeys: expected object".into()))?;

    let ent = advkeys.get(&target_s).ok_or_else(|| ActorError::InvalidWithResult(
        "reject: REJECT_SCKA_TARGET_UNKNOWN".into(),
        serde_json::json!({ "state": state_typed })
    ))?;

    let consumed = ent.get("consumed").and_then(|x| x.as_bool()).unwrap_or(false);
    if consumed {
        Err(ActorError::InvalidWithResult(
            "reject: REJECT_SCKA_TARGET_CONSUMED".into(),
            serde_json::json!({ "state": state_typed })
        ))
    } else {
        Ok(serde_json::json!({
            "accepted": { "type": "json", "data": true },
            "pq_target_id": { "type": "json", "data": target }
        }))
    }
}

// ---------------------------
// Suite-2 downgrade resistance ops
// ---------------------------
"suite2.downgrade.check" => {
    let local = get_json_data(&req.params, "local")?;
    let peer = get_json_data(&req.params, "peer")?;
    let negotiated = get_json_data(&req.params, "negotiated")?;
    let ad = get_json_data(&req.params, "ad")?;

    let local_supports = local.get("supports_suite2").and_then(|x| x.as_bool()).unwrap_or(false);
    let peer_supports = peer.get("supports_suite2").and_then(|x| x.as_bool()).unwrap_or(false);
    let policy_require = local.get("policy_require_suite2").and_then(|x| x.as_bool()).unwrap_or(false);

    // Determine if Suite-2 is required in this context (fail-closed bias).
    let suite2_required = policy_require || (local_supports && peer_supports);

    if policy_require && !peer_supports {
        return Err(ActorError::Invalid("reject: REJECT_S2_PEER_UNSUPPORTED".into()));
    }

    let n_pv = negotiated.get("protocol_version").ok_or_else(|| ActorError::Invalid("params.negotiated.protocol_version missing".into()))?;
    let n_sid = negotiated.get("suite_id").ok_or_else(|| ActorError::Invalid("params.negotiated.suite_id missing".into()))?;
    let pv = parse_u16(n_pv, "params.negotiated.protocol_version")?;
    let sid = parse_u16(n_sid, "params.negotiated.suite_id")?;

    // AD must match negotiated parameters
    let ad_pv = parse_u16(ad.get("protocol_version").ok_or_else(|| ActorError::Invalid("params.ad.protocol_version missing".into()))?, "params.ad.protocol_version")?;
    let ad_sid = parse_u16(ad.get("suite_id").ok_or_else(|| ActorError::Invalid("params.ad.suite_id missing".into()))?, "params.ad.suite_id")?;
    if ad_pv != pv || ad_sid != sid {
        return Err(ActorError::Invalid("reject: REJECT_S2_AD_MISMATCH".into()));
    }

    const PV_S2: u16 = 0x0500;
    const SID_S2: u16 = 0x0002;

    if suite2_required {
        if pv != PV_S2 || sid != SID_S2 {
            // If it looks like a Suite-1 fallback, call it downgrade explicitly.
            if pv == 0x0403 && sid == 0x0001 {
                return Err(ActorError::Invalid("reject: REJECT_S2_DOWNGRADE".into()));
            }
            return Err(ActorError::Invalid("reject: REJECT_S2_SUITE_MISMATCH".into()));
        }
    }

    Ok(serde_json::json!({
        "selected": { "type": "json", "data": { "protocol_version": "0x0500", "suite_id": "0x0002" } }
    }))
}
other => Err(ActorError::Unsupported(other.to_string())),
        }
    }
}

// ---------------------------
// main
// ---------------------------

fn main() {
    let cli = Cli::parse();

    let mut actor = match Actor::new(cli.name, cli.ci) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("failed to start actor: {e}");
            std::process::exit(2);
        }
    };

    let stdin = io::stdin();
    let mut stdout = io::BufWriter::new(io::stdout());

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };
        if line.trim().is_empty() {
            continue;
        }

        let parsed: Result<Req, _> = serde_json::from_str(&line);
        let req = match parsed {
            Ok(r) => r,
            Err(e) => {
                // If we can't parse the request, we can't extract an id. Emit a generic error.
                let out = serde_json::json!({
                    "id": "__parse__",
                    "ok": false,
                    "error": {"code": "INVALID", "message": format!("bad json: {e}")}
                });
                let _ = writeln!(stdout, "{}", out.to_string());
                let _ = stdout.flush();
                continue;
            }
        };

        let id = req.id.clone();
        let resp = match actor.dispatch(req) {
            Ok(result) => serde_json::to_string(&RespOk { id, ok: true, result }).unwrap(),
            Err(err) => {
                let out = RespErr {
                    id,
                    ok: false,
                    error: ErrObj {
                        code: err.code().to_string(),
                        message: err.to_string(),
                    },
                    result: err.result().cloned(),
                };
                serde_json::to_string(&out).unwrap()
            }
        };

        let _ = writeln!(stdout, "{}", resp);
        let _ = stdout.flush();
    }
}
