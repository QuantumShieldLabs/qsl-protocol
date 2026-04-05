use super::{
    config_dir, emit_marker, enforce_peer_not_blocked, enforce_safe_parents, env, fs, hex_encode,
    identity_fingerprint_from_pk, identity_marker_display, identity_peer_status,
    identity_pin_matches_seen, identity_read_pin, identity_read_sig_pin, identity_self_kem_keypair,
    init_from_base_handshake, kmac_out, print_error_marker, qsp_send_ready_tuple, qsp_session_load,
    qsp_session_store, relay_peer_route_token, relay_self_inbox_route_token, require_unlocked,
    resolve_peer_device_target, runtime_pq_kem_ciphertext_bytes, runtime_pq_kem_keypair,
    runtime_pq_kem_public_key_bytes, runtime_pq_sig_keypair, runtime_pq_sig_public_key_bytes,
    runtime_pq_sig_signature_bytes, transport, vault, vault_unlocked, Deserialize, ErrorCode, Hash,
    IdentityKeypair, Kmac, OsRng, Path, PathBuf, PqKem768, PqSigMldsa65, RngCore, Serialize,
    StdCrypto, Suite2SessionState, X25519Dh, X25519Priv, X25519Pub, IDENTITY_FP_PREFIX,
    SUITE2_PROTOCOL_VERSION, SUITE2_SUITE_ID,
};

const HS_MAGIC: &[u8; 4] = b"QHSM";
const HS_VERSION: u16 = 1;
const HS_TYPE_INIT: u8 = 1;
const HS_TYPE_RESP: u8 = 2;
const HS_TYPE_CONFIRM: u8 = 3;

fn hs_kem_pk_len() -> usize {
    runtime_pq_kem_public_key_bytes()
}

fn hs_kem_ct_len() -> usize {
    runtime_pq_kem_ciphertext_bytes()
}

pub(crate) fn hs_kem_keypair() -> (Vec<u8>, Vec<u8>) {
    runtime_pq_kem_keypair()
}

fn hs_sig_pk_len() -> usize {
    runtime_pq_sig_public_key_bytes()
}

fn hs_sig_sig_len() -> usize {
    runtime_pq_sig_signature_bytes()
}

pub(crate) fn hs_sig_keypair() -> (Vec<u8>, Vec<u8>) {
    runtime_pq_sig_keypair()
}

fn hs_default_role() -> String {
    "initiator".to_string()
}

#[derive(Clone, Debug)]
struct HsInit {
    session_id: [u8; 16],
    kem_pk: Vec<u8>,
    sig_pk: Vec<u8>,
    dh_pub: [u8; 32],
}

#[derive(Clone, Debug)]
struct HsResp {
    session_id: [u8; 16],
    kem_ct: Vec<u8>,
    mac: [u8; 32],
    sig_pk: Vec<u8>,
    sig: Vec<u8>,
    dh_pub: [u8; 32],
}

#[derive(Clone, Debug)]
struct HsConfirm {
    session_id: [u8; 16],
    mac: [u8; 32],
    sig: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone)]
struct HandshakePending {
    self_label: String,
    peer: String,
    session_id: [u8; 16],
    kem_sk: Vec<u8>,
    kem_pk: Vec<u8>,
    #[serde(default)]
    dh_sk: Vec<u8>,
    #[serde(default)]
    dh_pub: Vec<u8>,
    #[serde(default)]
    sig_pk: Vec<u8>,
    #[serde(default)]
    peer_fp: Option<String>,
    #[serde(default)]
    peer_sig_fp: Option<String>,
    #[serde(default)]
    peer_sig_pk: Option<Vec<u8>>,
    #[serde(default = "hs_default_role")]
    role: String,
    #[serde(default)]
    confirm_key: Option<[u8; 32]>,
    #[serde(default)]
    transcript_hash: Option<[u8; 32]>,
    #[serde(default)]
    pending_session: Option<Vec<u8>>,
}

fn hs_encode_init(msg: &HsInit) -> Vec<u8> {
    let pk_len = hs_kem_pk_len();
    let sig_pk_len = hs_sig_pk_len();
    if msg.kem_pk.len() != pk_len || msg.sig_pk.len() != sig_pk_len {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(4 + 2 + 1 + 16 + pk_len + sig_pk_len + 32);
    out.extend_from_slice(HS_MAGIC);
    out.extend_from_slice(&HS_VERSION.to_be_bytes());
    out.push(HS_TYPE_INIT);
    out.extend_from_slice(&msg.session_id);
    out.extend_from_slice(&msg.kem_pk);
    out.extend_from_slice(&msg.sig_pk);
    out.extend_from_slice(&msg.dh_pub);
    out
}

fn hs_decode_init(bytes: &[u8]) -> Result<HsInit, &'static str> {
    let pk_len = hs_kem_pk_len();
    let sig_pk_len = hs_sig_pk_len();
    if bytes.len() != 4 + 2 + 1 + 16 + pk_len + sig_pk_len + 32 {
        return Err("handshake_init_len");
    }
    if &bytes[0..4] != HS_MAGIC {
        return Err("handshake_magic");
    }
    let ver = u16::from_be_bytes([bytes[4], bytes[5]]);
    if ver != HS_VERSION {
        return Err("handshake_version");
    }
    if bytes[6] != HS_TYPE_INIT {
        return Err("handshake_type");
    }
    let mut sid = [0u8; 16];
    sid.copy_from_slice(&bytes[7..23]);
    let kem_pk = bytes[23..(23 + pk_len)].to_vec();
    let sig_pk = bytes[(23 + pk_len)..(23 + pk_len + sig_pk_len)].to_vec();
    let mut dh_pub = [0u8; 32];
    dh_pub.copy_from_slice(&bytes[(23 + pk_len + sig_pk_len)..(23 + pk_len + sig_pk_len + 32)]);
    Ok(HsInit {
        session_id: sid,
        kem_pk,
        sig_pk,
        dh_pub,
    })
}

fn hs_encode_resp(msg: &HsResp) -> Vec<u8> {
    let ct_len = hs_kem_ct_len();
    let sig_pk_len = hs_sig_pk_len();
    let sig_len = hs_sig_sig_len();
    if msg.kem_ct.len() != ct_len || msg.sig_pk.len() != sig_pk_len || msg.sig.len() != sig_len {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(4 + 2 + 1 + 16 + ct_len + 32 + sig_pk_len + sig_len + 32);
    out.extend_from_slice(HS_MAGIC);
    out.extend_from_slice(&HS_VERSION.to_be_bytes());
    out.push(HS_TYPE_RESP);
    out.extend_from_slice(&msg.session_id);
    out.extend_from_slice(&msg.kem_ct);
    out.extend_from_slice(&msg.mac);
    out.extend_from_slice(&msg.sig_pk);
    out.extend_from_slice(&msg.sig);
    out.extend_from_slice(&msg.dh_pub);
    out
}

fn hs_decode_resp(bytes: &[u8]) -> Result<HsResp, &'static str> {
    let ct_len = hs_kem_ct_len();
    let sig_pk_len = hs_sig_pk_len();
    let sig_len = hs_sig_sig_len();
    if bytes.len() != 4 + 2 + 1 + 16 + ct_len + 32 + sig_pk_len + sig_len + 32 {
        return Err("handshake_resp_len");
    }
    if &bytes[0..4] != HS_MAGIC {
        return Err("handshake_magic");
    }
    let ver = u16::from_be_bytes([bytes[4], bytes[5]]);
    if ver != HS_VERSION {
        return Err("handshake_version");
    }
    if bytes[6] != HS_TYPE_RESP {
        return Err("handshake_type");
    }
    let mut sid = [0u8; 16];
    sid.copy_from_slice(&bytes[7..23]);
    let kem_ct = bytes[23..(23 + ct_len)].to_vec();
    let mut mac = [0u8; 32];
    let mac_off = 23 + ct_len;
    mac.copy_from_slice(&bytes[mac_off..(mac_off + 32)]);
    let sig_pk_off = mac_off + 32;
    let sig_off = sig_pk_off + sig_pk_len;
    let sig_pk = bytes[sig_pk_off..sig_off].to_vec();
    let sig = bytes[sig_off..(sig_off + sig_len)].to_vec();
    let mut dh_pub = [0u8; 32];
    dh_pub.copy_from_slice(&bytes[(sig_off + sig_len)..(sig_off + sig_len + 32)]);
    Ok(HsResp {
        session_id: sid,
        kem_ct,
        mac,
        sig_pk,
        sig,
        dh_pub,
    })
}

fn hs_encode_confirm(msg: &HsConfirm) -> Vec<u8> {
    let sig_len = hs_sig_sig_len();
    if msg.sig.len() != sig_len {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(4 + 2 + 1 + 16 + 32 + sig_len);
    out.extend_from_slice(HS_MAGIC);
    out.extend_from_slice(&HS_VERSION.to_be_bytes());
    out.push(HS_TYPE_CONFIRM);
    out.extend_from_slice(&msg.session_id);
    out.extend_from_slice(&msg.mac);
    out.extend_from_slice(&msg.sig);
    out
}

fn hs_decode_confirm(bytes: &[u8]) -> Result<HsConfirm, &'static str> {
    let sig_len = hs_sig_sig_len();
    if bytes.len() != 4 + 2 + 1 + 16 + 32 + sig_len {
        return Err("handshake_confirm_len");
    }
    if &bytes[0..4] != HS_MAGIC {
        return Err("handshake_magic");
    }
    let ver = u16::from_be_bytes([bytes[4], bytes[5]]);
    if ver != HS_VERSION {
        return Err("handshake_version");
    }
    if bytes[6] != HS_TYPE_CONFIRM {
        return Err("handshake_type");
    }
    let mut sid = [0u8; 16];
    sid.copy_from_slice(&bytes[7..23]);
    let mut mac = [0u8; 32];
    mac.copy_from_slice(&bytes[23..55]);
    let sig = bytes[55..(55 + sig_len)].to_vec();
    Ok(HsConfirm {
        session_id: sid,
        mac,
        sig,
    })
}

fn emit_peer_mismatch(peer: &str, pinned_fp: &str, seen_fp: &str) {
    let pinned_display = identity_marker_display(pinned_fp);
    let seen_display = identity_marker_display(seen_fp);
    emit_marker(
        "identity_mismatch",
        None,
        &[
            ("peer", peer),
            ("pinned_fp", pinned_display.as_str()),
            ("seen_fp", seen_display.as_str()),
        ],
    );
    emit_marker("error", Some("peer_mismatch"), &[("peer", peer)]);
}

fn hs_seed_from_env() -> Option<u64> {
    env::var("QSC_HANDSHAKE_SEED")
        .ok()?
        .trim()
        .parse::<u64>()
        .ok()
}

fn hs_rand_bytes(label: &str, len: usize) -> Vec<u8> {
    if let Some(seed) = hs_seed_from_env() {
        let c = StdCrypto;
        let seed_bytes = seed.to_le_bytes();
        let seed_hash = c.sha512(&seed_bytes);
        let mut seed_key = [0u8; 32];
        seed_key.copy_from_slice(&seed_hash[..32]);
        return c.kmac256(&seed_key, label, b"", len);
    }
    let mut out = vec![0u8; len];
    let mut rng = OsRng;
    rng.fill_bytes(&mut out);
    out
}

fn hs_session_id(label: &str) -> [u8; 16] {
    let bytes = hs_rand_bytes(label, 16);
    let mut sid = [0u8; 16];
    sid.copy_from_slice(&bytes[..16]);
    sid
}

fn hs_transcript_mac(pq_init_ss: &[u8; 32], a1: &[u8], b1_no_mac: &[u8]) -> [u8; 32] {
    let c = StdCrypto;
    let mut data = Vec::with_capacity(a1.len() + b1_no_mac.len());
    data.extend_from_slice(a1);
    data.extend_from_slice(b1_no_mac);
    kmac_out::<32>(&c, pq_init_ss, "QSC.HS.TRANSCRIPT", &data)
}

fn hs_transcript_hash(pq_init_ss: &[u8; 32], a1: &[u8], b1_no_mac: &[u8]) -> [u8; 32] {
    let c = StdCrypto;
    let mut data = Vec::with_capacity(a1.len() + b1_no_mac.len());
    data.extend_from_slice(a1);
    data.extend_from_slice(b1_no_mac);
    kmac_out::<32>(&c, pq_init_ss, "QSC.HS.TRANSCRIPT.H", &data)
}

fn hs_pq_init_ss(ss_pq: &[u8], session_id: &[u8; 16]) -> [u8; 32] {
    let c = StdCrypto;
    let mut data = Vec::with_capacity(16 + 1);
    data.extend_from_slice(session_id);
    data.push(0x01);
    kmac_out::<32>(&c, ss_pq, "QSC.HS.PQ", &data)
}

fn hs_ephemeral_keypair() -> ([u8; 32], [u8; 32]) {
    let c = StdCrypto;
    let (sk, pk) = c.keypair();
    (sk.0, pk.0)
}

fn hs_dh_init_from_shared(dh_shared: &[u8; 32], session_id: &[u8; 16]) -> [u8; 32] {
    let c = StdCrypto;
    let mut data = Vec::with_capacity(16 + 1);
    data.extend_from_slice(session_id);
    data.push(0x02);
    kmac_out::<32>(&c, dh_shared, "QSC.HS.DHINIT", &data)
}

fn hs_dh_shared(self_sk: &[u8], peer_pub: &[u8]) -> Result<[u8; 32], &'static str> {
    if self_sk.len() != 32 || peer_pub.len() != 32 {
        return Err("handshake_dh_len");
    }
    let mut sk = [0u8; 32];
    sk.copy_from_slice(self_sk);
    let mut pk = [0u8; 32];
    pk.copy_from_slice(peer_pub);
    let c = StdCrypto;
    Ok(c.dh(&X25519Priv(sk), &X25519Pub(pk)))
}

fn hs_dh_pub_from_bytes(bytes: &[u8]) -> Result<[u8; 32], &'static str> {
    if bytes.len() != 32 {
        return Err("handshake_dh_len");
    }
    let mut out = [0u8; 32];
    out.copy_from_slice(bytes);
    Ok(out)
}

fn hs_dh_pub_is_all_zero(dh_pub: &[u8; 32]) -> bool {
    dh_pub.iter().all(|b| *b == 0)
}

fn hs_confirm_key(pq_init_ss: &[u8; 32], session_id: &[u8; 16], th: &[u8; 32]) -> [u8; 32] {
    let c = StdCrypto;
    let mut data = Vec::with_capacity(16 + 32);
    data.extend_from_slice(session_id);
    data.extend_from_slice(th);
    kmac_out::<32>(&c, pq_init_ss, "QSC.HS.CONFIRM", &data)
}

fn hs_confirm_mac(k_confirm: &[u8; 32], session_id: &[u8; 16], th: &[u8; 32]) -> [u8; 32] {
    let c = StdCrypto;
    let mut data = Vec::with_capacity(16 + 32 + 2);
    data.extend_from_slice(session_id);
    data.extend_from_slice(th);
    data.extend_from_slice(b"A2");
    kmac_out::<32>(&c, k_confirm, "QSC.HS.A2", &data)
}

fn hs_sig_fingerprint(sig_pk: &[u8]) -> String {
    let c = StdCrypto;
    let hash = c.sha512(sig_pk);
    format!("{}{}", IDENTITY_FP_PREFIX, hex_encode(&hash[..16]))
}

fn hs_sig_msg_b1(session_id: &[u8; 16], th: &[u8; 32]) -> Vec<u8> {
    let mut data = Vec::with_capacity(4 + 2 + 1 + 16 + 32);
    data.extend_from_slice(b"QSC.HS.SIG.B1");
    data.extend_from_slice(session_id);
    data.extend_from_slice(th);
    data
}

fn hs_sig_msg_a2(session_id: &[u8; 16], th: &[u8; 32], cmac: &[u8; 32]) -> Vec<u8> {
    let mut data = Vec::with_capacity(4 + 2 + 1 + 16 + 32 + 32);
    data.extend_from_slice(b"QSC.HS.SIG.A2");
    data.extend_from_slice(session_id);
    data.extend_from_slice(th);
    data.extend_from_slice(cmac);
    data
}

fn hs_sig_verify(sig_pk: &[u8], msg: &[u8], sig: &[u8], reason: &str) -> Result<(), &'static str> {
    let c = StdCrypto;
    match c.verify(sig_pk, msg, sig) {
        Ok(true) => {
            emit_marker(
                "sig_status",
                None,
                &[("ok", "true"), ("alg", "ML-DSA-65"), ("reason", reason)],
            );
            Ok(())
        }
        Ok(false) => {
            emit_marker(
                "sig_status",
                Some("sig_invalid"),
                &[("ok", "false"), ("alg", "ML-DSA-65"), ("reason", reason)],
            );
            Err("sig_invalid")
        }
        Err(_) => {
            emit_marker(
                "sig_status",
                Some("sig_invalid"),
                &[("ok", "false"), ("alg", "ML-DSA-65"), ("reason", reason)],
            );
            Err("sig_invalid")
        }
    }
}

fn hs_require_identity_pin<F>(peer: &str, seen_fp: &str, read_pin: F) -> Result<(), &'static str>
where
    F: Fn(&str) -> Result<Option<String>, ErrorCode>,
{
    match read_pin(peer) {
        Ok(Some(pinned)) => {
            if !identity_pin_matches_seen(pinned.as_str(), seen_fp) {
                emit_peer_mismatch(peer, pinned.as_str(), seen_fp);
                emit_marker("handshake_reject", None, &[("reason", "peer_mismatch")]);
                return Err("peer_mismatch");
            }
            let fp_display = identity_marker_display(seen_fp);
            emit_marker(
                "identity_ok",
                None,
                &[("peer", peer), ("fp", fp_display.as_str())],
            );
            Ok(())
        }
        Ok(None) => {
            let fp_display = identity_marker_display(seen_fp);
            emit_marker(
                "identity_unknown",
                None,
                &[("peer", peer), ("seen_fp", fp_display.as_str())],
            );
            emit_marker("handshake_reject", None, &[("reason", "identity_unknown")]);
            Err("identity_unknown")
        }
        Err(_) => {
            emit_marker(
                "handshake_reject",
                None,
                &[("reason", "identity_pin_failed")],
            );
            Err("identity_pin_failed")
        }
    }
}

fn hs_require_authenticated_peer(
    peer: &str,
    peer_fp: Option<&str>,
    peer_sig_fp: Option<&str>,
) -> Result<(), &'static str> {
    if let Some(peer_fp) = peer_fp {
        hs_require_identity_pin(peer, peer_fp, identity_read_pin)?;
    }
    if let Some(peer_sig_fp) = peer_sig_fp {
        hs_require_identity_pin(peer, peer_sig_fp, identity_read_sig_pin)?;
    }
    Ok(())
}

fn hs_build_session(
    authenticated: bool,
    role_is_a: bool,
    session_id: [u8; 16],
    dh_init: [u8; 32],
    pq_init_ss: [u8; 32],
    dh_self_pub: [u8; 32],
    dh_peer_pub: [u8; 32],
) -> Result<Suite2SessionState, &'static str> {
    let c = StdCrypto;
    init_from_base_handshake(
        &c,
        role_is_a,
        SUITE2_PROTOCOL_VERSION,
        SUITE2_SUITE_ID,
        &session_id,
        &dh_init,
        &pq_init_ss,
        &dh_self_pub,
        &dh_peer_pub,
        authenticated,
    )
}

fn hs_pending_legacy_path(dir: &Path, self_label: &str, peer: &str) -> PathBuf {
    dir.join(format!("handshake_pending_{}_{}.json", self_label, peer))
}

fn hs_pending_secret_key(self_label: &str, peer: &str) -> String {
    format!("handshake.pending.{}.{}", self_label, peer)
}

fn hs_pending_load(self_label: &str, peer: &str) -> Result<Option<HandshakePending>, ErrorCode> {
    let secret_key = hs_pending_secret_key(self_label, peer);
    match vault::secret_get(&secret_key) {
        Ok(Some(v)) if !v.is_empty() => {
            let pending: HandshakePending =
                serde_json::from_str(&v).map_err(|_| ErrorCode::ParseFailed)?;
            return Ok(Some(pending));
        }
        Ok(_) => {}
        Err("vault_missing" | "vault_locked") => return Err(ErrorCode::IdentitySecretUnavailable),
        Err(_) => return Err(ErrorCode::IoReadFailed),
    }

    let (dir, source) = config_dir()?;
    let path = hs_pending_legacy_path(&dir, self_label, peer);
    if !path.exists() {
        return Ok(None);
    }
    enforce_safe_parents(&path, source)?;
    let bytes = fs::read(&path).map_err(|_| ErrorCode::IoReadFailed)?;
    let pending: HandshakePending =
        serde_json::from_slice(&bytes).map_err(|_| ErrorCode::ParseFailed)?;
    let v = serde_json::to_string(&pending).map_err(|_| ErrorCode::IoWriteFailed)?;
    match vault::secret_set(&secret_key, &v) {
        Ok(()) => {
            let _ = fs::remove_file(&path);
        }
        Err("vault_missing" | "vault_locked") => return Err(ErrorCode::IdentitySecretUnavailable),
        Err(_) => return Err(ErrorCode::IoWriteFailed),
    }
    Ok(Some(pending))
}

fn hs_pending_store(pending: &HandshakePending) -> Result<(), ErrorCode> {
    let key = hs_pending_secret_key(&pending.self_label, &pending.peer);
    let value = serde_json::to_string(pending).map_err(|_| ErrorCode::IoWriteFailed)?;
    match vault::secret_set(&key, &value) {
        Ok(()) => Ok(()),
        Err("vault_missing" | "vault_locked") => Err(ErrorCode::IdentitySecretUnavailable),
        Err(_) => Err(ErrorCode::IoWriteFailed),
    }
}

fn hs_pending_clear(self_label: &str, peer: &str) -> Result<(), ErrorCode> {
    let key = hs_pending_secret_key(self_label, peer);
    match vault::secret_set(&key, "") {
        Ok(()) => {}
        Err("vault_missing" | "vault_locked") => return Err(ErrorCode::IdentitySecretUnavailable),
        Err(_) => return Err(ErrorCode::IoWriteFailed),
    }
    let (dir, source) = config_dir()?;
    let path = hs_pending_legacy_path(&dir, self_label, peer);
    enforce_safe_parents(&path, source)?;
    let _ = fs::remove_file(path);
    Ok(())
}

pub(crate) fn handshake_status(peer: Option<&str>) {
    if !require_unlocked("handshake_status") {
        return;
    }
    let peer_label = peer.unwrap_or("peer-0");
    if let Err(code) = enforce_peer_not_blocked(peer_label) {
        print_error_marker(code);
    }
    let (peer_fp, pinned) = identity_peer_status(peer_label);
    let pinned_s = if pinned { "true" } else { "false" };
    let (send_ready, send_ready_reason) = qsp_send_ready_tuple(peer_label);
    let send_ready_s = if send_ready { "yes" } else { "no" };
    match qsp_session_load(peer_label) {
        Ok(Some(_)) => {
            if send_ready {
                emit_marker(
                    "handshake_status",
                    None,
                    &[
                        ("status", "established"),
                        ("peer", peer_label),
                        ("peer_fp", peer_fp.as_str()),
                        ("pinned", pinned_s),
                        ("send_ready", send_ready_s),
                    ],
                );
            } else {
                emit_marker(
                    "handshake_status",
                    None,
                    &[
                        ("status", "established_recv_only"),
                        ("peer", peer_label),
                        ("peer_fp", peer_fp.as_str()),
                        ("pinned", pinned_s),
                        ("send_ready", send_ready_s),
                        ("send_ready_reason", send_ready_reason),
                    ],
                );
            }
        }
        Ok(None) => {
            emit_marker(
                "handshake_status",
                None,
                &[
                    ("status", "no_session"),
                    ("peer", peer_label),
                    ("peer_fp", peer_fp.as_str()),
                    ("pinned", pinned_s),
                    ("send_ready", send_ready_s),
                    ("send_ready_reason", send_ready_reason),
                ],
            );
        }
        Err(_) => {
            emit_marker(
                "handshake_status",
                Some("handshake_status_failed"),
                &[
                    ("peer", peer_label),
                    ("peer_fp", peer_fp.as_str()),
                    ("pinned", pinned_s),
                    ("send_ready", send_ready_s),
                    ("send_ready_reason", send_ready_reason),
                ],
            );
        }
    }
}

fn perform_handshake_init_with_route(
    self_label: &str,
    peer: &str,
    relay: &str,
    route_token: &str,
) -> Result<(), &'static str> {
    enforce_peer_not_blocked(peer)?;
    let IdentityKeypair {
        kem_pk,
        kem_sk,
        sig_pk,
        sig_sk: _,
    } = identity_self_kem_keypair(self_label).map_err(|e| e.as_str())?;
    let sid = hs_session_id("QSC.HS.SID");
    let (dh_sk, dh_pub) = hs_ephemeral_keypair();
    let msg = HsInit {
        session_id: sid,
        kem_pk: kem_pk.clone(),
        sig_pk: sig_pk.clone(),
        dh_pub,
    };
    let bytes = hs_encode_init(&msg);
    if bytes.is_empty() {
        return Err("handshake_init_encode_failed");
    }
    let pending = HandshakePending {
        self_label: self_label.to_string(),
        peer: peer.to_string(),
        session_id: sid,
        kem_sk,
        kem_pk,
        dh_sk: dh_sk.to_vec(),
        dh_pub: dh_pub.to_vec(),
        sig_pk,
        peer_sig_fp: None,
        peer_sig_pk: None,
        peer_fp: None,
        role: "initiator".to_string(),
        confirm_key: None,
        transcript_hash: None,
        pending_session: None,
    };
    hs_pending_store(&pending).map_err(|_| "handshake_pending_store_failed")?;
    emit_marker(
        "handshake_start",
        None,
        &[("role", "initiator"), ("peer", peer)],
    );
    let size_s = bytes.len().to_string();
    let pk_len_s = hs_kem_pk_len().to_string();
    let sig_pk_len_s = hs_sig_pk_len().to_string();
    emit_marker(
        "handshake_send",
        None,
        &[
            ("msg", "A1"),
            ("size", size_s.as_str()),
            ("kem_pk_len", pk_len_s.as_str()),
            ("sig_pk_len", sig_pk_len_s.as_str()),
        ],
    );
    transport::relay_inbox_push(relay, route_token, &bytes)?;
    Ok(())
}

fn handshake_init_with_route(self_label: &str, peer: &str, relay: &str, route_token: &str) {
    if !require_unlocked("handshake_init") {
        return;
    }
    if let Err(code) = perform_handshake_init_with_route(self_label, peer, relay, route_token) {
        print_error_marker(code);
    }
}

pub(crate) fn handshake_init(self_label: &str, peer: &str, relay: &str) {
    if !vault_unlocked() {
        require_unlocked("handshake_init");
    }
    let peer_channel = resolve_peer_device_target(peer, false)
        .map(|v| v.channel)
        .unwrap_or_else(|_| peer.to_string());
    let route_token = relay_peer_route_token(peer).unwrap_or_else(|code| print_error_marker(code));
    handshake_init_with_route(
        self_label,
        peer_channel.as_str(),
        relay,
        route_token.as_str(),
    );
}

fn perform_handshake_poll_with_tokens(
    self_label: &str,
    peer: &str,
    relay: &str,
    inbox_route_token: &str,
    peer_route_token: &str,
    max: usize,
) -> Result<(), &'static str> {
    enforce_peer_not_blocked(peer)?;
    let items = match transport::relay_inbox_pull(relay, inbox_route_token, max) {
        Ok(v) => v,
        Err(code) => {
            emit_marker("handshake_recv", Some(code), &[("ok", "false")]);
            return Err(code);
        }
    };
    if items.is_empty() {
        emit_marker("handshake_recv", None, &[("msg", "none"), ("ok", "true")]);
        return Ok(());
    }

    if let Some(pending) = hs_pending_load(self_label, peer).map_err(|e| e.as_str())? {
        emit_marker(
            "handshake_pending",
            None,
            &[
                ("peer", peer),
                ("present", "true"),
                ("role", pending.role.as_str()),
            ],
        );
        if pending.role == "initiator" {
            for item in items {
                match hs_decode_resp(&item.data) {
                    Ok(resp) => {
                        if resp.session_id != pending.session_id {
                            emit_marker(
                                "handshake_reject",
                                None,
                                &[("reason", "session_id_mismatch")],
                            );
                            continue;
                        }
                        let c = StdCrypto;
                        let ss_pq = match c.decap(&pending.kem_sk, &resp.kem_ct) {
                            Ok(v) => v,
                            Err(_) => {
                                emit_marker(
                                    "handshake_reject",
                                    None,
                                    &[("reason", "pq_decap_failed")],
                                );
                                return Ok(());
                            }
                        };
                        let pq_init_ss = hs_pq_init_ss(&ss_pq, &resp.session_id);
                        if hs_dh_pub_is_all_zero(&resp.dh_pub) {
                            emit_marker("handshake_reject", None, &[("reason", "dh_pub_invalid")]);
                            return Ok(());
                        }
                        let dh_self_pub = match hs_dh_pub_from_bytes(&pending.dh_pub) {
                            Ok(v) => v,
                            Err(_) => {
                                emit_marker("handshake_reject", None, &[("reason", "dh_missing")]);
                                return Ok(());
                            }
                        };
                        let dh_shared = match hs_dh_shared(&pending.dh_sk, &resp.dh_pub) {
                            Ok(v) => v,
                            Err(_) => {
                                emit_marker("handshake_reject", None, &[("reason", "dh_failed")]);
                                return Ok(());
                            }
                        };
                        let dh_init_arr = hs_dh_init_from_shared(&dh_shared, &resp.session_id);
                        let dh_peer_pub = resp.dh_pub;
                        let a1 = hs_encode_init(&HsInit {
                            session_id: pending.session_id,
                            kem_pk: pending.kem_pk.clone(),
                            sig_pk: pending.sig_pk.clone(),
                            dh_pub: dh_self_pub,
                        });
                        let b1_no_auth = {
                            let mut tmp = Vec::with_capacity(
                                4 + 2 + 1 + 16 + hs_kem_ct_len() + hs_sig_pk_len(),
                            );
                            tmp.extend_from_slice(HS_MAGIC);
                            tmp.extend_from_slice(&HS_VERSION.to_be_bytes());
                            tmp.push(HS_TYPE_RESP);
                            tmp.extend_from_slice(&resp.session_id);
                            tmp.extend_from_slice(&resp.kem_ct);
                            tmp.extend_from_slice(&resp.sig_pk);
                            tmp.extend_from_slice(&resp.dh_pub);
                            tmp
                        };
                        let mac = hs_transcript_mac(&pq_init_ss, &a1, &b1_no_auth);
                        if mac != resp.mac {
                            emit_marker("handshake_reject", None, &[("reason", "bad_transcript")]);
                            return Ok(());
                        }
                        let th = hs_transcript_hash(&pq_init_ss, &a1, &b1_no_auth);
                        let sig_msg = hs_sig_msg_b1(&resp.session_id, &th);
                        if hs_sig_verify(&resp.sig_pk, &sig_msg, &resp.sig, "b1_verify").is_err() {
                            emit_marker("handshake_reject", None, &[("reason", "sig_invalid")]);
                            return Ok(());
                        }
                        let sig_fp = hs_sig_fingerprint(&resp.sig_pk);
                        let authenticated_peer = match hs_require_authenticated_peer(
                            peer,
                            None,
                            Some(sig_fp.as_str()),
                        ) {
                            Ok(()) => true,
                            Err(_) => return Ok(()),
                        };
                        let st = match hs_build_session(
                            authenticated_peer,
                            true,
                            pending.session_id,
                            dh_init_arr,
                            pq_init_ss,
                            dh_self_pub,
                            dh_peer_pub,
                        ) {
                            Ok(v) => v,
                            Err(_) => {
                                emit_marker(
                                    "handshake_reject",
                                    None,
                                    &[("reason", "session_init_failed")],
                                );
                                return Ok(());
                            }
                        };
                        qsp_session_store(peer, &st)
                            .map_err(|_| "handshake_session_store_failed")?;
                        let _ = hs_pending_clear(self_label, peer);
                        let k_confirm = hs_confirm_key(&pq_init_ss, &resp.session_id, &th);
                        let cmac = hs_confirm_mac(&k_confirm, &resp.session_id, &th);
                        let sig_sk = identity_self_kem_keypair(self_label)
                            .map_err(|e| e.as_str())?
                            .sig_sk;
                        let a2_sig_msg = hs_sig_msg_a2(&resp.session_id, &th, &cmac);
                        let a2_sig = match c.sign(&sig_sk, &a2_sig_msg) {
                            Ok(v) => v,
                            Err(_) => {
                                emit_marker(
                                    "handshake_reject",
                                    None,
                                    &[("reason", "sig_sign_failed")],
                                );
                                return Ok(());
                            }
                        };
                        emit_marker(
                            "sig_status",
                            None,
                            &[("ok", "true"), ("alg", "ML-DSA-65"), ("reason", "a2_sign")],
                        );
                        let confirm = HsConfirm {
                            session_id: resp.session_id,
                            mac: cmac,
                            sig: a2_sig,
                        };
                        let cbytes = hs_encode_confirm(&confirm);
                        let size_s = cbytes.len().to_string();
                        emit_marker(
                            "handshake_send",
                            None,
                            &[("msg", "A2"), ("size", size_s.as_str())],
                        );
                        transport::relay_inbox_push(relay, peer_route_token, &cbytes)?;
                        emit_marker(
                            "handshake_complete",
                            None,
                            &[("peer", peer), ("role", "initiator")],
                        );
                        return Ok(());
                    }
                    Err(_) => {
                        emit_marker("handshake_reject", None, &[("reason", "decode_failed")]);
                        continue;
                    }
                }
            }
            return Ok(());
        }
        if pending.role == "responder" {
            for item in items {
                match hs_decode_confirm(&item.data) {
                    Ok(confirm) => {
                        if confirm.session_id != pending.session_id {
                            emit_marker(
                                "handshake_reject",
                                None,
                                &[("reason", "session_id_mismatch")],
                            );
                            continue;
                        }
                        let Some(k_confirm) = pending.confirm_key else {
                            emit_marker(
                                "handshake_reject",
                                None,
                                &[("reason", "missing_confirm_key")],
                            );
                            continue;
                        };
                        let Some(th) = pending.transcript_hash else {
                            emit_marker(
                                "handshake_reject",
                                None,
                                &[("reason", "missing_transcript")],
                            );
                            continue;
                        };
                        let expect = hs_confirm_mac(&k_confirm, &confirm.session_id, &th);
                        if expect != confirm.mac {
                            emit_marker("handshake_recv", None, &[("msg", "A2"), ("ok", "false")]);
                            emit_marker("handshake_reject", None, &[("reason", "bad_confirm")]);
                            continue;
                        }
                        let Some(peer_sig_pk) = pending.peer_sig_pk.as_ref() else {
                            emit_marker(
                                "handshake_reject",
                                None,
                                &[("reason", "identity_missing")],
                            );
                            continue;
                        };
                        let sig_msg = hs_sig_msg_a2(&confirm.session_id, &th, &confirm.mac);
                        if hs_sig_verify(peer_sig_pk, &sig_msg, &confirm.sig, "a2_verify").is_err()
                        {
                            emit_marker("handshake_reject", None, &[("reason", "sig_invalid")]);
                            continue;
                        }
                        emit_marker("handshake_recv", None, &[("msg", "A2"), ("ok", "true")]);
                        let Some(ref pending_bytes) = pending.pending_session else {
                            emit_marker("handshake_reject", None, &[("reason", "missing_session")]);
                            continue;
                        };
                        let st = match Suite2SessionState::restore_bytes(pending_bytes) {
                            Ok(v) => v,
                            Err(_) => {
                                emit_marker(
                                    "handshake_reject",
                                    None,
                                    &[("reason", "session_restore_failed")],
                                );
                                continue;
                            }
                        };
                        let Some(peer_fp) = pending.peer_fp.as_ref() else {
                            emit_marker(
                                "handshake_reject",
                                None,
                                &[("reason", "identity_missing")],
                            );
                            continue;
                        };
                        if hs_require_authenticated_peer(peer, Some(peer_fp.as_str()), None)
                            .is_err()
                        {
                            continue;
                        }
                        qsp_session_store(peer, &st)
                            .map_err(|_| "handshake_session_store_failed")?;
                        let _ = hs_pending_clear(self_label, peer);
                        emit_marker(
                            "handshake_complete",
                            None,
                            &[("peer", peer), ("role", "responder")],
                        );
                        return Ok(());
                    }
                    Err(_) => {
                        emit_marker("handshake_reject", None, &[("reason", "decode_failed")]);
                        continue;
                    }
                }
            }
            return Ok(());
        }
    }

    emit_marker(
        "handshake_pending",
        None,
        &[("peer", peer), ("present", "false"), ("role", "none")],
    );

    for item in items {
        match hs_decode_init(&item.data) {
            Ok(init) => {
                if hs_dh_pub_is_all_zero(&init.dh_pub) {
                    emit_marker("handshake_reject", None, &[("reason", "dh_pub_invalid")]);
                    continue;
                }
                let peer_fp = identity_fingerprint_from_pk(&init.kem_pk);
                let peer_sig_fp = hs_sig_fingerprint(&init.sig_pk);
                let authenticated_peer =
                    match hs_require_authenticated_peer(peer, Some(peer_fp.as_str()), None) {
                        Ok(()) => true,
                        Err(_) => continue,
                    };
                let c = StdCrypto;
                let (kem_ct, ss_pq) = match c.encap(&init.kem_pk) {
                    Ok(v) => v,
                    Err(_) => {
                        emit_marker("handshake_reject", None, &[("reason", "pq_encap_failed")]);
                        continue;
                    }
                };
                let pq_init_ss = hs_pq_init_ss(&ss_pq, &init.session_id);
                let (dh_sk, dh_self_pub) = hs_ephemeral_keypair();
                let dh_shared = match hs_dh_shared(&dh_sk, &init.dh_pub) {
                    Ok(v) => v,
                    Err(_) => {
                        emit_marker("handshake_reject", None, &[("reason", "dh_failed")]);
                        continue;
                    }
                };
                let dh_init_arr = hs_dh_init_from_shared(&dh_shared, &init.session_id);
                let dh_peer_pub = init.dh_pub;
                let st = match hs_build_session(
                    authenticated_peer,
                    false,
                    init.session_id,
                    dh_init_arr,
                    pq_init_ss,
                    dh_self_pub,
                    dh_peer_pub,
                ) {
                    Ok(v) => v,
                    Err(_) => {
                        emit_marker(
                            "handshake_reject",
                            None,
                            &[("reason", "session_init_failed")],
                        );
                        continue;
                    }
                };
                let a1 = hs_encode_init(&init);
                let self_sig = match identity_self_kem_keypair(self_label) {
                    Ok(k) => (k.sig_pk, k.sig_sk),
                    Err(_) => {
                        emit_marker("handshake_reject", None, &[("reason", "identity_missing")]);
                        continue;
                    }
                };
                let (self_sig_pk, self_sig_sk) = self_sig;
                let b1_no_auth = {
                    let mut tmp =
                        Vec::with_capacity(4 + 2 + 1 + 16 + hs_kem_ct_len() + hs_sig_pk_len());
                    tmp.extend_from_slice(HS_MAGIC);
                    tmp.extend_from_slice(&HS_VERSION.to_be_bytes());
                    tmp.push(HS_TYPE_RESP);
                    tmp.extend_from_slice(&init.session_id);
                    tmp.extend_from_slice(&kem_ct);
                    tmp.extend_from_slice(&self_sig_pk);
                    tmp.extend_from_slice(&dh_self_pub);
                    tmp
                };
                let mac = hs_transcript_mac(&pq_init_ss, &a1, &b1_no_auth);
                let th = hs_transcript_hash(&pq_init_ss, &a1, &b1_no_auth);
                let sig_msg = hs_sig_msg_b1(&init.session_id, &th);
                let sig = match c.sign(&self_sig_sk, &sig_msg) {
                    Ok(v) => v,
                    Err(_) => {
                        emit_marker("handshake_reject", None, &[("reason", "sig_sign_failed")]);
                        continue;
                    }
                };
                emit_marker(
                    "sig_status",
                    None,
                    &[("ok", "true"), ("alg", "ML-DSA-65"), ("reason", "b1_sign")],
                );
                let k_confirm = hs_confirm_key(&pq_init_ss, &init.session_id, &th);
                let pending = HandshakePending {
                    self_label: self_label.to_string(),
                    peer: peer.to_string(),
                    session_id: init.session_id,
                    kem_sk: Vec::new(),
                    kem_pk: Vec::new(),
                    dh_sk: dh_sk.to_vec(),
                    dh_pub: dh_self_pub.to_vec(),
                    sig_pk: Vec::new(),
                    peer_fp: Some(peer_fp),
                    peer_sig_fp: Some(peer_sig_fp),
                    peer_sig_pk: Some(init.sig_pk.clone()),
                    role: "responder".to_string(),
                    confirm_key: Some(k_confirm),
                    transcript_hash: Some(th),
                    pending_session: Some(st.snapshot_bytes()),
                };
                hs_pending_store(&pending).map_err(|_| "handshake_pending_store_failed")?;
                let resp = HsResp {
                    session_id: init.session_id,
                    kem_ct,
                    mac,
                    sig_pk: self_sig_pk,
                    sig,
                    dh_pub: dh_self_pub,
                };
                let bytes = hs_encode_resp(&resp);
                let size_s = bytes.len().to_string();
                let ct_len_s = hs_kem_ct_len().to_string();
                let sig_pk_len_s = hs_sig_pk_len().to_string();
                emit_marker(
                    "handshake_send",
                    None,
                    &[
                        ("msg", "B1"),
                        ("size", size_s.as_str()),
                        ("kem_ct_len", ct_len_s.as_str()),
                        ("sig_pk_len", sig_pk_len_s.as_str()),
                    ],
                );
                transport::relay_inbox_push(relay, peer_route_token, &bytes)?;
                return Ok(());
            }
            Err(_) => {
                emit_marker("handshake_reject", None, &[("reason", "decode_failed")]);
                continue;
            }
        }
    }
    Ok(())
}

fn handshake_poll_with_tokens(
    self_label: &str,
    peer: &str,
    relay: &str,
    inbox_route_token: &str,
    peer_route_token: &str,
    max: usize,
) {
    if !require_unlocked("handshake_poll") {
        return;
    }
    if let Err(code) = perform_handshake_poll_with_tokens(
        self_label,
        peer,
        relay,
        inbox_route_token,
        peer_route_token,
        max,
    ) {
        print_error_marker(code);
    }
}

pub(crate) fn handshake_poll(self_label: &str, peer: &str, relay: &str, max: usize) {
    let peer_channel = resolve_peer_device_target(peer, false)
        .map(|v| v.channel)
        .unwrap_or_else(|_| peer.to_string());
    let inbox_route_token =
        relay_self_inbox_route_token().unwrap_or_else(|code| print_error_marker(code));
    let peer_route_token =
        relay_peer_route_token(peer).unwrap_or_else(|code| print_error_marker(code));
    handshake_poll_with_tokens(
        self_label,
        peer_channel.as_str(),
        relay,
        inbox_route_token.as_str(),
        peer_route_token.as_str(),
        max,
    );
}
