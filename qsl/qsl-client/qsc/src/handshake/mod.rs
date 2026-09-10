#![allow(unexpected_cfgs)]

use crate::output::{CliError, CliResult};
use super::{
    cmd::HandshakeSuiteMode, config_dir, emit_marker, enforce_peer_not_blocked,
    enforce_safe_parents, fs, identity_fingerprint_from_identity,
    identity_fingerprint_single, identity_peer_status, identity_pin_matches_seen,
    identity_pin_matches_seen_identity, identity_read_peer_kem_pk, identity_read_pin,
    identity_read_sig_pin, identity_self_kem_keypair, init_from_base_handshake, kmac_out,
        qsp_send_ready_tuple, qsp_session_load, qsp_session_store, relay_peer_route_token,
    relay_self_inbox_route_token, require_unlocked, resolve_peer_device_target,
    runtime_pq_kem_ciphertext_bytes, runtime_pq_kem_keypair, runtime_pq_kem_public_key_bytes,
    runtime_pq_sig_keypair, runtime_pq_sig_public_key_bytes, runtime_pq_sig_signature_bytes,
    transport, vault, vault_unlocked, Deserialize, ErrorCode, IdentityKeypair, OsRng, Path,
    PathBuf, PqKem768, PqSigMldsa65, RngCore, Serialize, StdCrypto, Suite2SessionState, X25519Dh,
    FpRole, X25519Priv, X25519Pub, SUITE2_PROTOCOL_VERSION, SUITE2_SUITE_ID,
};

// NA-0741 (D-1376): `pub(crate)` so `frameclass::classify` can reference the magic
// BY NAME instead of copying its bytes. Visibility only — no behaviour here changes,
// and `handshake` being `pub mod` means this adds no PUBLIC surface.
pub(crate) const HS_MAGIC: &[u8; 4] = b"QHSM";
const HS_VERSION_LEGACY: u16 = 1;
const HS_VERSION_V2: u16 = 2;
const HS_TYPE_INIT: u8 = 1;
const HS_TYPE_RESP: u8 = 2;
const HS_TYPE_CONFIRM: u8 = 3;
const HS_PARAM_BLOCK_MAX: usize = 64;
#[cfg(not(qsc_binding_fuzz_helper))]
const HS_PARAM_SUITE_CONTEXT: u16 = 0x0001;
#[cfg(not(qsc_binding_fuzz_helper))]
const HS_PARAM_FLAG_CRITICAL: u8 = 0x01;
const HS_SUITE_CONTEXT_BLOCK: [u8; 9] = [0x00, 0x01, 0x01, 0x00, 0x04, 0x05, 0x00, 0x00, 0x02];
const HS_SUITE2_PROTOCOL_VERSION_WIRE: u16 = 0x0500;
const HS_SUITE2_SUITE_ID_WIRE: u16 = 0x0002;
#[cfg(not(qsc_binding_fuzz_helper))]
const HS_LEGACY_PROTOCOL_VERSION_WIRE: u16 = 0x0403;
#[cfg(not(qsc_binding_fuzz_helper))]
const HS_LEGACY_SUITE_ID_WIRE: u16 = 0x0001;

fn hs_kem_pk_len() -> usize {
    runtime_pq_kem_public_key_bytes()
}

fn hs_kem_ct_len() -> usize {
    runtime_pq_kem_ciphertext_bytes()
}

pub(crate) fn hs_kem_keypair() -> (Vec<u8>, Vec<u8>) {
    runtime_pq_kem_keypair()
}

#[cfg(qsc_rng_failure_test_seam)]
pub(crate) fn hs_kem_keypair_with_failure_label(
    label: &str,
) -> Result<(Vec<u8>, Vec<u8>), &'static str> {
    if hs_rng_failure_forced(label) {
        return Err("rng_failure_forced");
    }
    Ok(runtime_pq_kem_keypair())
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

#[derive(Clone, Debug, PartialEq, Eq)]
enum HsSuiteContext {
    LegacyV1,
    ExplicitV2 {
        block: Vec<u8>,
        protocol_version: u16,
        suite_id: u16,
    },
}

impl HsSuiteContext {
    fn suite2() -> Self {
        Self::ExplicitV2 {
            block: HS_SUITE_CONTEXT_BLOCK.to_vec(),
            protocol_version: HS_SUITE2_PROTOCOL_VERSION_WIRE,
            suite_id: HS_SUITE2_SUITE_ID_WIRE,
        }
    }

    fn explicit_block(&self) -> Option<&[u8]> {
        match self {
            Self::LegacyV1 => None,
            Self::ExplicitV2 { block, .. } => Some(block.as_slice()),
        }
    }

    fn is_explicit(&self) -> bool {
        self.explicit_block().is_some()
    }

    fn as_pending_block(&self) -> Option<Vec<u8>> {
        self.explicit_block().map(|v| v.to_vec())
    }

    fn wire_version(&self) -> u16 {
        match self {
            Self::LegacyV1 => HS_VERSION_LEGACY,
            Self::ExplicitV2 { .. } => HS_VERSION_V2,
        }
    }

    fn mode_label(&self) -> &'static str {
        match self {
            Self::LegacyV1 => "legacy_v1",
            Self::ExplicitV2 { .. } => "v2_suite_context",
        }
    }
}

#[derive(Clone, Debug)]
struct HsInit {
    suite_context: HsSuiteContext,
    session_id: [u8; 16],
    kem_pk: Vec<u8>,
    sig_pk: Vec<u8>,
    dh_pub: [u8; 32],
    /// NA-0633 (ENG-0038, C1): the initiator's ML-KEM encapsulation to the RESPONDER's pinned identity
    /// KEM key. The responder must decapsulate it (proving identity-key possession) to derive the same
    /// `pq_init_ss`; a wrong responder cannot, and fails the transcript MAC. Length = ML-KEM-768 ct.
    resp_kem_ct: Vec<u8>,
}

#[derive(Clone, Debug)]
struct HsResp {
    suite_context: HsSuiteContext,
    session_id: [u8; 16],
    kem_ct: Vec<u8>,
    mac: [u8; 32],
    sig_pk: Vec<u8>,
    sig: Vec<u8>,
    dh_pub: [u8; 32],
}

#[derive(Clone, Debug)]
struct HsConfirm {
    suite_context: HsSuiteContext,
    session_id: [u8; 16],
    mac: [u8; 32],
    sig: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
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
    /// NA-0633 (ENG-0038, C1): the initiator's encapsulation to the responder's identity KEM key (kept
    /// so the B1-processing transcript reconstruction is byte-identical to the A1 that was sent).
    #[serde(default)]
    resp_kem_ct: Vec<u8>,
    /// NA-0633 (ENG-0038, C1): the shared secret from that encapsulation, mixed into pq_init_ss at B1
    /// processing. A wrong responder cannot reproduce it.
    #[serde(default)]
    resp_kem_ss: Vec<u8>,
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
    #[serde(default)]
    suite_context: Option<Vec<u8>>,
}

// NA-0780: one encrypted first-connection capsule is authoritative over its legacy
// pending mirror. Selection is written BEFORE route/session/pending effects. Recovery
// repeats those effects by generation and SID, never by alias/session presence alone.
const FIRST_CONNECTIONS_KEY: &str = "handshake.first_connections.v1";
const FIRST_CONNECTION_GROUPS: usize = 64;
const FIRST_CONNECTION_BYTES: usize = 256 * 1024;
const FIRST_CONNECTION_TOTAL_BYTES: usize = FIRST_CONNECTION_GROUPS * FIRST_CONNECTION_BYTES;
const FIRST_FRAME_BYTES: usize = 32 * 1024;

#[cfg(test)]
thread_local! {
    static HS_LIFECYCLE_CUT: std::cell::Cell<&'static str> = const { std::cell::Cell::new("") };
}

#[cfg(test)]
fn hs_lifecycle_test_cut(at: &'static str) -> Result<(), &'static str> {
    HS_LIFECYCLE_CUT.with(|cut| {
        if cut.get() == at {
            cut.set("");
            Err("test_process_interruption")
        } else {
            Ok(())
        }
    })
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct HsReceipt {
    // Owner is the mailbox that supplied this envelope, not the reply destination.
    mailbox: String,
    envelope_digest: [u8; 32],
    frame_digest: [u8; 32],
    route: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
struct HsDelivery {
    relay: String,
    route: String,
    bytes: Vec<u8>,
    ticket: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
struct HsCandidate {
    pending: HandshakePending,
    receipt: HsReceipt,
    reply: HsDelivery,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
struct HsSelection {
    session_id: [u8; 16],
    session: Option<Vec<u8>>,
    receipt: HsReceipt,
    reply: Option<HsDelivery>,
    route_before: String,
    applied: bool,
    reply_delivered: bool,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
struct FirstConnection {
    self_label: String,
    peer: String,
    self_fp: String,
    peer_fp: String,
    device: String,
    generation: [u8; 16],
    outgoing: Option<HsCandidate>,
    responder: Option<HsCandidate>,
    deferred: Option<HsReceipt>,
    selected: Option<HsSelection>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
struct FirstConnections {
    version: u8,
    entries: Vec<FirstConnection>,
}

fn hs_digest(bytes: &[u8]) -> [u8; 32] {
    use sha2::Digest;
    sha2::Sha256::digest(bytes).into()
}

fn hs_lifecycle_lock() -> Result<crate::model::LockGuard, &'static str> {
    let (dir, source) = config_dir().map_err(|_| "handshake_lifecycle_store")?;
    crate::fs_store::lock_store_exclusive(&dir, source).map_err(|_| "handshake_lifecycle_store")
}

fn hs_lifecycle_load() -> Result<FirstConnections, &'static str> {
    let Some(raw) =
        vault::secret_get(FIRST_CONNECTIONS_KEY).map_err(|_| "handshake_lifecycle_store")?
    else {
        return Ok(FirstConnections {
            version: 1,
            entries: Vec::new(),
        });
    };
    if raw.len() > FIRST_CONNECTION_TOTAL_BYTES {
        return Err("handshake_lifecycle_capacity");
    }
    let store: FirstConnections =
        serde_json::from_str(&raw).map_err(|_| "handshake_lifecycle_store")?;
    hs_lifecycle_validate(&store)?;
    Ok(store)
}

fn hs_lifecycle_validate(store: &FirstConnections) -> Result<(), &'static str> {
    if store.version != 1 {
        return Err("handshake_lifecycle_store");
    }
    if store.entries.iter().filter(|c| c.is_active()).count() > FIRST_CONNECTION_GROUPS {
        return Err("handshake_lifecycle_capacity");
    }
    let mut bindings = std::collections::BTreeSet::new();
    let mut aliases = std::collections::BTreeSet::new();
    for c in &store.entries {
        if !bindings.insert((&c.self_fp, &c.peer_fp, &c.device))
            || !aliases.insert(&c.peer)
            || c.responder.is_some() && c.deferred.is_some()
        {
            return Err("handshake_lifecycle_store");
        }
        if serde_json::to_vec(c)
            .map_err(|_| "handshake_lifecycle_store")?
            .len()
            > FIRST_CONNECTION_BYTES
        {
            return Err("handshake_lifecycle_capacity");
        }
        for candidate in [&c.outgoing, &c.responder].into_iter().flatten() {
            if candidate.pending.self_label != c.self_label || candidate.pending.peer != c.peer {
                return Err("handshake_lifecycle_store");
            }
            hs_delivery_validate(&candidate.reply)?;
        }
        if let Some(selection) = &c.selected {
            if let Some(reply) = &selection.reply {
                hs_delivery_validate(reply)?;
            }
            if !c.owns_sid(&selection.session_id) {
                return Err("handshake_lifecycle_store");
            }
            match (&selection.session, selection.applied) {
                (Some(bytes), false) => {
                    let st = Suite2SessionState::restore_bytes(bytes)
                        .map_err(|_| "handshake_lifecycle_store")?;
                    if st.send.session_id != selection.session_id
                        || st.recv.session_id != selection.session_id
                    {
                        return Err("handshake_lifecycle_store");
                    }
                }
                (None, true) => {}
                _ => return Err("handshake_lifecycle_store"),
            }
        }
    }
    Ok(())
}

// Charge every active record at its full serialized-record limit, including the
// store's JSON framing. Retained replay records consume only their actual bytes.
// This is admission accounting, not a new on-disk format or a load-time gate:
// older stores may lack this reservation and must remain readable/recoverable.
fn hs_lifecycle_reserved_bytes(store: &FirstConnections) -> Result<usize, &'static str> {
    let mut bytes = serde_json::to_vec(store)
        .map_err(|_| "handshake_lifecycle_store")?
        .len();
    for c in store.entries.iter().filter(|c| c.is_active()) {
        let actual = serde_json::to_vec(c)
            .map_err(|_| "handshake_lifecycle_store")?
            .len();
        bytes += FIRST_CONNECTION_BYTES.saturating_sub(actual);
    }
    Ok(bytes)
}

fn hs_lifecycle_admission_available(store: &FirstConnections) -> Result<(), &'static str> {
    if store.entries.iter().filter(|c| c.is_active()).count() >= FIRST_CONNECTION_GROUPS
        || hs_lifecycle_reserved_bytes(store)?
            + FIRST_CONNECTION_BYTES
            + usize::from(!store.entries.is_empty())
            > FIRST_CONNECTION_TOTAL_BYTES
    {
        return Err("handshake_lifecycle_capacity");
    }
    Ok(())
}

fn hs_delivery_validate(reply: &HsDelivery) -> Result<(), &'static str> {
    if reply.bytes.is_empty()
        || reply.bytes.len() > FIRST_FRAME_BYTES
        || reply.relay.len() > 4096
        || reply.route.len() > 256
        || reply.ticket.as_ref().is_some_and(|s| s.len() > 1024)
    {
        return Err("handshake_lifecycle_capacity");
    }
    Ok(())
}

fn hs_lifecycle_save(store: &FirstConnections) -> Result<(), &'static str> {
    hs_lifecycle_validate(store)?;
    let raw = serde_json::to_string(store).map_err(|_| "handshake_lifecycle_store")?;
    if raw.len() > FIRST_CONNECTION_TOTAL_BYTES {
        return Err("handshake_lifecycle_capacity");
    }
    vault::secret_set(FIRST_CONNECTIONS_KEY, &raw).map_err(|_| "handshake_lifecycle_store")
}

impl FirstConnection {
    fn is_active(&self) -> bool {
        // Local quiescence, not a claim that the peer has finished. An applied
        // initiator with an outstanding A2 delivery still occupies an active slot.
        !self
            .selected
            .as_ref()
            .is_some_and(|s| s.applied && s.reply_delivered)
    }

    fn owns_sid(&self, sid: &[u8; 16]) -> bool {
        [&self.outgoing, &self.responder]
            .into_iter()
            .flatten()
            .any(|c| &c.pending.session_id == sid)
    }

    fn check_binding(&self) -> Result<(), &'static str> {
        enforce_peer_not_blocked(&self.peer)?;
        let (self_fp, peer_fp, device) = hs_lifecycle_binding(&self.self_label, &self.peer)?;
        if (self_fp, peer_fp, device)
            != (
                self.self_fp.clone(),
                self.peer_fp.clone(),
                self.device.clone(),
            )
        {
            return Err("contacts_identity_changed");
        }
        Ok(())
    }

    fn check_session(&self) -> Result<(), &'static str> {
        match qsp_session_load(&self.peer).map_err(|_| "handshake_lifecycle_store")? {
            None if !self.selected.as_ref().is_some_and(|s| s.applied) => Ok(()),
            Some(st) => {
                let Some(s) = &self.selected else {
                    return Err("contacts_session_exists");
                };
                if st.send.session_id == s.session_id && st.recv.session_id == s.session_id {
                    Ok(())
                } else {
                    Err("contacts_session_exists")
                }
            }
            None => Err("handshake_lifecycle_conflict"),
        }
    }

    fn check_pending(&self) -> Result<(), &'static str> {
        let (pending, _) = hs_pending_load_state(&self.self_label, &self.peer)
            .map_err(|_| "handshake_lifecycle_store")?;
        if pending.as_ref().is_some_and(|p| {
            ![&self.outgoing, &self.responder]
                .into_iter()
                .flatten()
                .any(|candidate| &candidate.pending == p)
        }) {
            return Err("handshake_lifecycle_conflict");
        }
        Ok(())
    }
}

fn hs_lifecycle_binding(
    self_label: &str,
    peer: &str,
) -> Result<(String, String, String), &'static str> {
    // Read without contact migration writes, and reject alias/device ambiguity.
    let raw = vault::secret_get(crate::store::CONTACTS_SECRET_KEY)
        .map_err(|_| "handshake_lifecycle_store")?
        .ok_or("identity_unknown")?;
    let store: crate::store::ContactsStore =
        serde_json::from_str(&raw).map_err(|_| "handshake_lifecycle_store")?;
    let rec = store.peers.get(peer).ok_or("identity_unknown")?;
    let primary = crate::contacts::primary_device(rec).ok_or("contacts_identity_changed")?;
    if rec.devices.len() != 1
        || !rec.fp.eq_ignore_ascii_case(&primary.fp)
        || rec.sig_fp != primary.sig_fp
        || rec.kem_pk != primary.kem_pk
        || rec.sig_fp.is_none()
        || rec.kem_pk.is_none()
        || rec
            .primary_device_id
            .as_ref()
            .is_some_and(|id| !rec.devices.iter().any(|device| &device.device_id == id))
    {
        return Err("contacts_identity_changed");
    }
    if store
        .peers
        .iter()
        .any(|(alias, other)| alias != peer && other.fp.eq_ignore_ascii_case(&rec.fp))
    {
        return Err("handshake_lifecycle_conflict");
    }
    let keys = identity_self_kem_keypair(self_label).map_err(|_| "identity_secret_unavailable")?;
    let self_fp = identity_fingerprint_from_identity(&keys.kem_pk, &keys.sig_pk);
    let peer_fp = rec.fp.to_ascii_lowercase();
    if self_fp == peer_fp {
        return Err(crate::invite::INVITE_SELF);
    }
    Ok((
        self_fp,
        peer_fp,
        serde_json::to_string(&(&primary.device_id, &rec.sig_fp, &rec.kem_pk))
            .map_err(|_| "handshake_lifecycle_store")?,
    ))
}

fn hs_lifecycle_new(self_label: &str, peer: &str) -> Result<FirstConnection, &'static str> {
    if qsp_session_load(peer)
        .map_err(|_| "handshake_lifecycle_store")?
        .is_some()
    {
        return Err("contacts_session_exists");
    }
    if hs_pending_load_state(self_label, peer)
        .map_err(|_| "handshake_lifecycle_store")?
        .0
        .is_some()
    {
        return Err("handshake_lifecycle_conflict");
    }
    let (self_fp, peer_fp, device) = hs_lifecycle_binding(self_label, peer)?;
    let mut generation = [0; 16];
    OsRng
        .try_fill_bytes(&mut generation)
        .map_err(|_| "handshake_rng_failed")?;
    Ok(FirstConnection {
        self_label: self_label.into(),
        peer: peer.into(),
        self_fp,
        peer_fp,
        device,
        generation,
        outgoing: None,
        responder: None,
        deferred: None,
        selected: None,
    })
}

fn hs_lifecycle_find(
    self_label: &str,
    peer: &str,
) -> Result<Option<FirstConnection>, &'static str> {
    let store = hs_lifecycle_load()?;
    let found = store.entries.into_iter().find(|c| c.peer == peer);
    if let Some(c) = &found {
        if c.self_label != self_label {
            return Err("handshake_lifecycle_conflict");
        }
        c.check_binding()?;
        c.check_session()?;
        c.check_pending()?;
    }
    Ok(found)
}

// Caller holds the store lock. A capsule never replaces another generation.
fn hs_lifecycle_put(c: &FirstConnection) -> Result<(), &'static str> {
    let mut store = hs_lifecycle_load()?;
    let prior_charge = hs_lifecycle_reserved_bytes(&store)?;
    if let Some(old) = store.entries.iter_mut().find(|v| v.peer == c.peer) {
        if old.generation != c.generation {
            return Err("handshake_lifecycle_conflict");
        }
        *old = c.clone();
    } else {
        hs_lifecycle_admission_available(&store)?;
        store.entries.push(c.clone());
    }
    // Legacy stores that exceed the reservation budget can finish their admitted
    // work under the unchanged actual-byte limit, but cannot increase the deficit.
    // In particular, growth of retained records cannot steal active reservations.
    if hs_lifecycle_reserved_bytes(&store)? > FIRST_CONNECTION_TOTAL_BYTES.max(prior_charge) {
        return Err("handshake_lifecycle_capacity");
    }
    hs_lifecycle_save(&store)
}

fn hs_lifecycle_recover(c: &mut FirstConnection) -> Result<(), &'static str> {
    c.check_binding()?;
    c.check_session()?;
    c.check_pending()?;
    let Some(s) = &c.selected else {
        // Pending is only a compatibility/status mirror. A crash before this write
        // is recovered from the capsule, without regenerating keys or reply bytes.
        if let Some(candidate) = c.outgoing.as_ref().or(c.responder.as_ref()) {
            if hs_pending_load_state(&c.self_label, &c.peer)
                .map_err(|_| "handshake_lifecycle_store")?
                .0
                .is_none()
            {
                hs_pending_store(&candidate.pending).map_err(|_| "handshake_lifecycle_store")?;
                #[cfg(test)]
                hs_lifecycle_test_cut("pending_mirror")?;
            }
        }
        return Ok(());
    };
    if s.applied {
        return Ok(());
    }
    let st =
        Suite2SessionState::restore_bytes(s.session.as_deref().ok_or("handshake_lifecycle_store")?)
            .map_err(|_| "handshake_lifecycle_store")?;
    let route = relay_peer_route_token(&c.peer)?;
    if route != s.route_before && route != s.receipt.route {
        return Err("handshake_lifecycle_conflict");
    }
    // Every cut is recoverable from the earlier selection write. In particular,
    // never overwrite a same-SID session: it may already have advanced via send.
    if qsp_session_load(&c.peer)
        .map_err(|_| "handshake_lifecycle_store")?
        .is_none()
    {
        hs_session_store_reported(&c.peer, &st)?;
        #[cfg(test)]
        hs_lifecycle_test_cut("session")?;
    }
    crate::contacts::contacts_set_route_token(&c.peer, &s.receipt.route)?;
    #[cfg(test)]
    hs_lifecycle_test_cut("route")?;
    hs_pending_clear(&c.self_label, &c.peer).map_err(|_| "handshake_lifecycle_store")?;
    #[cfg(test)]
    hs_lifecycle_test_cut("pending_clear")?;
    // Once all recoverable effects are durable, retain only public replay data.
    // Keeping an epoch-zero session or handshake secrets here would defeat erasure
    // by the ordinary ratchet. This does not promise erasure from older backups.
    use zeroize::Zeroize;
    let selection = c.selected.as_mut().expect("selected");
    if let Some(mut snapshot) = selection.session.take() {
        snapshot.zeroize();
    }
    selection.applied = true;
    for candidate in [&mut c.outgoing, &mut c.responder].into_iter().flatten() {
        let p = &mut candidate.pending;
        p.kem_sk.zeroize();
        p.kem_sk.clear();
        p.dh_sk.zeroize();
        p.dh_sk.clear();
        p.resp_kem_ss.zeroize();
        p.resp_kem_ss.clear();
        p.confirm_key.zeroize();
        p.confirm_key = None;
        p.transcript_hash.zeroize();
        p.transcript_hash = None;
        if let Some(mut snapshot) = p.pending_session.take() {
            snapshot.zeroize();
        }
    }
    hs_lifecycle_put(c)?;
    #[cfg(test)]
    hs_lifecycle_test_cut("applied")?;
    Ok(())
}

fn hs_delivery_send(reply: &HsDelivery) -> Result<(), &'static str> {
    hs_delivery_validate(reply)?;
    match &reply.ticket {
        Some(ticket) => transport::relay_inbox_push_with_ticket(
            &reply.relay,
            &reply.route,
            &reply.bytes,
            Some(ticket),
        ),
        None => transport::relay_inbox_push(&reply.relay, &reply.route, &reply.bytes),
    }
}

fn hs_receipt(
    mailbox: &str,
    envelope: &[u8],
    frame: &[u8],
    route: &str,
) -> Result<HsReceipt, &'static str> {
    if envelope.len() > FIRST_FRAME_BYTES
        || frame.len() > FIRST_FRAME_BYTES
        || mailbox.len() > 256
        || route.len() > 256
    {
        return Err("handshake_lifecycle_capacity");
    }
    let route = crate::adversarial::route::normalize_route_token(route)?;
    Ok(HsReceipt {
        mailbox: mailbox.into(),
        envelope_digest: hs_digest(envelope),
        frame_digest: hs_digest(frame),
        route,
    })
}

fn hs_lifecycle_after_send(
    c: &FirstConnection,
    sent: Option<&HsDelivery>,
) -> Result<(), &'static str> {
    let _lock = hs_lifecycle_lock()?;
    let mut current =
        hs_lifecycle_find(&c.self_label, &c.peer)?.ok_or("handshake_lifecycle_conflict")?;
    if current.generation != c.generation {
        return Err("handshake_lifecycle_conflict");
    }
    if let Some(selection) = current.selected.as_mut() {
        if let (Some(expected), Some(sent)) = (selection.reply.as_ref(), sent) {
            if expected.bytes == sent.bytes
                && expected.route == sent.route
                && expected.relay == sent.relay
                && !selection.reply_delivered
            {
                #[cfg(test)]
                hs_lifecycle_test_cut("reply_delivered")?;
                selection.reply_delivered = true;
                hs_lifecycle_put(&current)?;
            }
        }
    }
    Ok(())
}

pub(crate) fn hs_invite_recovery_pending(peer: &str) -> Result<bool, &'static str> {
    let _lock = hs_lifecycle_lock()?;
    let store = hs_lifecycle_load()?;
    let Some(c) = store.entries.iter().find(|c| c.peer == peer) else {
        return Ok(false);
    };
    c.check_binding()?;
    c.check_session()?;
    c.check_pending()?;
    Ok(c.selected
        .as_ref()
        .is_some_and(|s| !s.applied || !s.reply_delivered))
}

pub(crate) fn hs_invite_resume(self_label: &str, peer: &str) -> Result<(), &'static str> {
    let _lock = hs_lifecycle_lock()?;
    let Some(mut c) = hs_lifecycle_find(self_label, peer)? else {
        return Ok(());
    };
    hs_lifecycle_recover(&mut c)?;
    let reply = c
        .selected
        .as_ref()
        .filter(|s| !s.reply_delivered)
        .and_then(|s| s.reply.clone());
    drop(_lock);
    if let Some(reply) = reply {
        hs_delivery_send(&reply)?;
        hs_lifecycle_after_send(&c, Some(&reply))?;
    }
    Ok(())
}

fn hs_lifecycle_select(
    expected: &FirstConnection,
    pending: &HandshakePending,
    st: &Suite2SessionState,
    receipt: &HsReceipt,
    reply: Option<HsDelivery>,
) -> Result<Option<HsDelivery>, &'static str> {
    let _lock = hs_lifecycle_lock()?;
    let mut c = hs_lifecycle_find(&expected.self_label, &expected.peer)?
        .ok_or("handshake_lifecycle_conflict")?;
    if c.generation != expected.generation || !c.owns_sid(&pending.session_id) {
        return Err("handshake_lifecycle_conflict");
    }
    if let Some(selection) = &c.selected {
        // Same SID alone cannot authorize a different outer envelope/route.
        if &selection.receipt != receipt {
            return Err("handshake_lifecycle_conflict");
        }
    } else {
        c.selected = Some(HsSelection {
            session_id: st.send.session_id,
            session: Some(st.snapshot_bytes()),
            receipt: receipt.clone(),
            reply_delivered: reply.is_none(),
            reply,
            route_before: relay_peer_route_token(&c.peer)?,
            applied: false,
        });
        hs_lifecycle_put(&c)?;
        #[cfg(test)]
        hs_lifecycle_test_cut("selection")?;
    }
    hs_lifecycle_recover(&mut c)?;
    Ok(c.selected.as_ref().and_then(|s| s.reply.clone()))
}

// Used only after the invitation bundle has passed its existing commitment/signature
// gates. This recognizes THIS lifecycle's session on retry; provisioning's merged
// unrelated-session guard remains unchanged.
pub(crate) fn hs_invite_existing_binding(
    self_label: &str,
    peer: &str,
    kem: &[u8],
    sig: &[u8],
) -> Result<Option<String>, &'static str> {
    let _lock = hs_lifecycle_lock()?;
    let c = match hs_lifecycle_find(self_label, peer) {
        Ok(Some(c)) => c,
        Ok(None) | Err("identity_unknown") => return Ok(None),
        Err(code) => return Err(code),
    };
    if c.selected.is_none() {
        return Ok(None);
    }
    let fp = identity_fingerprint_from_identity(kem, sig);
    if fp != c.peer_fp {
        return Err("contacts_identity_changed");
    }
    Ok(Some(fp))
}

// Best-effort preflight before consuming a remote invitation. Admission is
// checked again under the same store lock when the outgoing capsule is saved;
// this local check does not make remote redemption and local storage atomic.
pub(crate) fn hs_invite_admission_preflight(
    self_label: &str,
    peer: &str,
    slot: &str,
) -> Result<(), &'static str> {
    let _lock = hs_lifecycle_lock()?;
    if let Some(c) = hs_lifecycle_find(self_label, peer)? {
        if c.selected.is_some() && c.outgoing.as_ref().is_none_or(|o| o.reply.route != slot) {
            return Err("contacts_session_exists");
        }
        // Exact retry or late-redeem coalescing uses the existing reservation.
        return Ok(());
    }
    hs_lifecycle_admission_available(&hs_lifecycle_load()?)
}

pub(crate) fn hs_invite_reserved_outgoing(
    self_label: &str,
    peer: &str,
    slot: &str,
) -> Result<bool, &'static str> {
    let _lock = hs_lifecycle_lock()?;
    Ok(hs_lifecycle_load()?.entries.iter().any(|c| {
        c.self_label == self_label
            && c.peer == peer
            && c.outgoing.as_ref().is_some_and(|o| o.reply.route == slot)
    }))
}

pub(crate) fn hs_invite_lifecycle_present(
    self_label: &str,
    peer: &str,
) -> Result<bool, &'static str> {
    let _lock = hs_lifecycle_lock()?;
    Ok(hs_lifecycle_find(self_label, peer)?.is_some())
}

// Recheck generation after network I/O. No locks are held across a relay call.
fn hs_lifecycle_retry(
    c: &FirstConnection,
    reply: Option<&HsDelivery>,
) -> Result<PollOutcome, &'static str> {
    if let Some(reply) = reply {
        hs_delivery_send(reply)?;
    }
    hs_lifecycle_after_send(c, reply)?;
    Ok(PollOutcome::AlreadyComplete)
}

struct HsFramePlan {
    lifecycle: Option<FirstConnection>,
    pending: Option<HandshakePending>,
    pending_state: HsPendingState,
    receipt: HsReceipt,
    immediate: Option<PollOutcome>,
    retry: Option<HsDelivery>,
}

fn hs_lifecycle_plan(
    self_label: &str,
    peer: &str,
    frame: &[u8],
    receipt: HsReceipt,
    invitation: bool,
    mode: HandshakeSuiteMode,
    speculative: bool,
) -> Result<HsFramePlan, &'static str> {
    let _lock = hs_lifecycle_lock()?;
    let mut plan = HsFramePlan {
        lifecycle: hs_lifecycle_find(self_label, peer)?,
        pending: None,
        pending_state: HsPendingState::Absent,
        receipt,
        immediate: None,
        retry: None,
    };
    if plan.lifecycle.is_none() && !invitation {
        (plan.pending, plan.pending_state) =
            hs_pending_load_state(self_label, peer).map_err(|e| e.as_str())?;
        return Ok(plan);
    }
    if let Some(c) = plan.lifecycle.as_mut() {
        hs_lifecycle_recover(c)?;
    }
    if let Ok(init) = hs_decode_init(frame, mode) {
        let fp = identity_fingerprint_from_identity(&init.kem_pk, &init.sig_pk);
        if hs_require_primary_identity_pin(peer, &fp, identity_read_pin, speculative).is_err() {
            plan.immediate = Some(PollOutcome::NotConsumed);
            return Ok(plan);
        }
        if plan.lifecycle.is_none() {
            plan.lifecycle = Some(hs_lifecycle_new(self_label, peer)?);
        }
        let c = plan.lifecycle.as_mut().expect("lifecycle");
        if let Some(responder) = &c.responder {
            if responder.receipt != plan.receipt {
                return Err("handshake_lifecycle_occupied");
            }
            plan.retry = Some(responder.reply.clone());
            plan.immediate = Some(PollOutcome::AlreadyComplete);
        } else if c.outgoing.is_some() && c.self_fp < c.peer_fp {
            // A1 matches pinned PUBLIC keys, but is not possession proof. Retain
            // just this exact observed envelope; do not ACK until B1 authenticates.
            if c.deferred.as_ref().is_some_and(|r| r != &plan.receipt) {
                return Err("handshake_lifecycle_occupied");
            }
            if c.deferred.is_none() {
                c.deferred = Some(plan.receipt.clone());
                hs_lifecycle_put(c)?;
            }
            plan.immediate = Some(if c.selected.is_some() {
                PollOutcome::AlreadyComplete
            } else {
                PollOutcome::NotConsumed
            });
        } else if c.selected.is_some() {
            plan.immediate = Some(PollOutcome::NotConsumed);
        }
        return Ok(plan);
    }
    let Some(c) = plan.lifecycle.as_ref() else {
        plan.immediate = Some(PollOutcome::NotConsumed);
        return Ok(plan);
    };
    if let Ok(confirm) = hs_decode_confirm(frame, mode) {
        if let Some(responder) = c
            .responder
            .as_ref()
            .filter(|r| r.pending.session_id == confirm.session_id)
        {
            // A2 carries no route. Use only the route in the admitted A1 envelope.
            plan.receipt.route = responder.receipt.route.clone();
            plan.pending = Some(responder.pending.clone());
        }
    } else if let Ok(resp) = hs_decode_resp_pending(frame, mode) {
        plan.pending = c
            .outgoing
            .as_ref()
            .filter(|o| o.pending.session_id == resp.session_id)
            .map(|o| o.pending.clone());
    }
    if let Some(s) = &c.selected {
        plan.pending = None;
        if s.receipt == plan.receipt {
            plan.retry = s.reply.clone();
            plan.immediate = Some(PollOutcome::AlreadyComplete);
        } else {
            plan.immediate = Some(PollOutcome::NotConsumed);
        }
    } else if plan.pending.is_none() {
        plan.immediate = Some(PollOutcome::NotConsumed);
    }
    if plan.pending.is_some() {
        plan.pending_state = HsPendingState::Present;
    }
    Ok(plan)
}

fn hs_suite_context_for_mode(mode: HandshakeSuiteMode) -> HsSuiteContext {
    match mode {
        HandshakeSuiteMode::LegacyCompat => HsSuiteContext::LegacyV1,
        HandshakeSuiteMode::SuiteRequired => HsSuiteContext::suite2(),
    }
}

// NA-0711 (D647 A4 Δ38): the collapse is GONE. Every reason now prints under its own name.
//
// ⚠ WHAT THIS DOES NOT DO, SAID PLAINLY: it does NOT make a REPLAY distinguishable from "I have no
// context". Those are the SAME branch emitting the SAME reason -- the no-pending arm decodes the
// frame as a confirm, cannot reach the replay guard (which needs an explicit suite context, and the
// invite path hardcodes LegacyCompat), and falls through to the init decoder, which rejects on the
// frame-type byte. Both print `handshake_type`. Separating them means touching the replay guard
// itself, which is a FOURTH change and is refused here; the reject-vocabulary normalisation lane
// NA-0708 filed owns it.
fn hs_decode_reason_label(reason: &'static str) -> &'static str {
    reason
}

fn hs_emit_suite_reject(reason: &'static str) {
    emit_marker(
        "handshake_suite_admission",
        Some(reason),
        &[("result", "reject"), ("reason", reason)],
    );
    emit_marker("handshake_reject", None, &[("reason", reason)]);
}

fn hs_emit_decode_reject(reason: &'static str) {
    if reason.starts_with("REJECT_QSC_HS_") {
        hs_emit_suite_reject(reason);
    } else {
        emit_marker(
            "handshake_reject",
            None,
            &[("reason", hs_decode_reason_label(reason))],
        );
    }
}

fn hs_emit_suite_accept(ctx: &HsSuiteContext, compatibility: bool) {
    if compatibility {
        emit_marker(
            "handshake_suite_admission",
            None,
            &[
                ("result", "compatibility_accept"),
                ("mode", "legacy_compat"),
                ("reason", "ACCEPT_QSC_HS_LEGACY_COMPATIBILITY"),
            ],
        );
        return;
    }
    if let HsSuiteContext::ExplicitV2 {
        protocol_version,
        suite_id,
        ..
    } = ctx
    {
        let protocol_s = format!("0x{protocol_version:04x}");
        let suite_s = format!("0x{suite_id:04x}");
        emit_marker(
            "handshake_suite_admission",
            None,
            &[
                ("result", "accept"),
                ("version", "v2"),
                ("protocol_version", protocol_s.as_str()),
                ("suite_id", suite_s.as_str()),
                ("reason", "ACCEPT_QSC_HS_SUITE2"),
            ],
        );
    }
}

fn hs_parse_parameter_block(block: &[u8]) -> Result<HsSuiteContext, &'static str> {
    #[cfg(qsc_binding_fuzz_helper)]
    {
        return hs_suite_context_from_fuzz(crate::adversarial::binding_fuzz::parse_suite_context(
            block,
        )?);
    }
    #[cfg(not(qsc_binding_fuzz_helper))]
    {
        if block.len() > HS_PARAM_BLOCK_MAX {
            return Err("REJECT_QSC_HS_MALFORMED_LENGTH");
        }
        let mut off = 0usize;
        let mut prior_id: Option<u16> = None;
        let mut suite_value: Option<[u8; 4]> = None;
        let mut unknown_critical = false;
        let mut unknown_parameter = false;

        while off < block.len() {
            if block.len().saturating_sub(off) < 5 {
                return Err("REJECT_QSC_HS_MALFORMED_LENGTH");
            }
            let param_id = u16::from_be_bytes([block[off], block[off + 1]]);
            let flags = block[off + 2];
            let value_len = u16::from_be_bytes([block[off + 3], block[off + 4]]) as usize;
            off += 5;
            if flags & !HS_PARAM_FLAG_CRITICAL != 0 {
                return Err("REJECT_QSC_HS_MALFORMED_LENGTH");
            }
            if let Some(prev) = prior_id {
                if param_id == prev {
                    return Err("REJECT_QSC_HS_DUPLICATE_PARAMETER");
                }
                if param_id < prev {
                    return Err("REJECT_QSC_HS_NONCANONICAL_ORDER");
                }
            }
            prior_id = Some(param_id);
            if block.len().saturating_sub(off) < value_len {
                return Err("REJECT_QSC_HS_MALFORMED_LENGTH");
            }
            let value = &block[off..off + value_len];
            off += value_len;

            if param_id == HS_PARAM_SUITE_CONTEXT {
                if suite_value.is_some() {
                    return Err("REJECT_QSC_HS_DUPLICATE_PARAMETER");
                }
                if flags != HS_PARAM_FLAG_CRITICAL || value_len != 4 {
                    return Err("REJECT_QSC_HS_MALFORMED_LENGTH");
                }
                let mut tuple = [0u8; 4];
                tuple.copy_from_slice(value);
                suite_value = Some(tuple);
                continue;
            }

            if flags & HS_PARAM_FLAG_CRITICAL != 0 {
                unknown_critical = true;
            } else {
                unknown_parameter = true;
            }
        }

        let Some(tuple) = suite_value else {
            return Err("REJECT_QSC_HS_SUITE_MISSING");
        };
        if unknown_critical {
            return Err("REJECT_QSC_HS_UNKNOWN_CRITICAL");
        }
        if unknown_parameter {
            return Err("REJECT_QSC_HS_UNKNOWN_PARAMETER");
        }

        let protocol_version = u16::from_be_bytes([tuple[0], tuple[1]]);
        let suite_id = u16::from_be_bytes([tuple[2], tuple[3]]);
        if protocol_version == HS_SUITE2_PROTOCOL_VERSION_WIRE
            && suite_id == HS_SUITE2_SUITE_ID_WIRE
        {
            return Ok(HsSuiteContext::ExplicitV2 {
                block: block.to_vec(),
                protocol_version,
                suite_id,
            });
        }
        if protocol_version == HS_SUITE2_PROTOCOL_VERSION_WIRE {
            return Err("REJECT_QSC_HS_SUITE_UNSUPPORTED");
        }
        if protocol_version == HS_LEGACY_PROTOCOL_VERSION_WIRE
            && suite_id == HS_LEGACY_SUITE_ID_WIRE
        {
            return Err("REJECT_QSC_HS_DOWNGRADE");
        }
        Err("REJECT_QSC_HS_INCONSISTENT_TUPLE")
    }
}

#[cfg(qsc_binding_fuzz_helper)]
fn hs_suite_context_from_fuzz(
    ctx: crate::adversarial::binding_fuzz::FuzzSuiteContext,
) -> Result<HsSuiteContext, &'static str> {
    match ctx {
        crate::adversarial::binding_fuzz::FuzzSuiteContext::LegacyV1 => {
            Ok(HsSuiteContext::LegacyV1)
        }
        crate::adversarial::binding_fuzz::FuzzSuiteContext::ExplicitV2 {
            block,
            protocol_version,
            suite_id,
        } => Ok(HsSuiteContext::ExplicitV2 {
            block,
            protocol_version,
            suite_id,
        }),
    }
}

#[cfg(qsc_binding_fuzz_helper)]
fn hs_fuzz_suite_mode(mode: HandshakeSuiteMode) -> crate::adversarial::binding_fuzz::FuzzSuiteMode {
    match mode {
        HandshakeSuiteMode::LegacyCompat => {
            crate::adversarial::binding_fuzz::FuzzSuiteMode::LegacyCompat
        }
        HandshakeSuiteMode::SuiteRequired => {
            crate::adversarial::binding_fuzz::FuzzSuiteMode::SuiteRequired
        }
    }
}

#[cfg(qsc_binding_fuzz_helper)]
fn hs_fuzz_frame_kind(
    frame_type: u8,
) -> Option<crate::adversarial::binding_fuzz::BindingFuzzFrameKind> {
    match frame_type {
        HS_TYPE_INIT => Some(crate::adversarial::binding_fuzz::BindingFuzzFrameKind::A1),
        HS_TYPE_RESP => Some(crate::adversarial::binding_fuzz::BindingFuzzFrameKind::B1),
        HS_TYPE_CONFIRM => Some(crate::adversarial::binding_fuzz::BindingFuzzFrameKind::A2),
        _ => None,
    }
}

fn hs_encode_header(out: &mut Vec<u8>, frame_type: u8, suite_context: &HsSuiteContext) -> bool {
    out.extend_from_slice(HS_MAGIC);
    out.extend_from_slice(&suite_context.wire_version().to_be_bytes());
    out.push(frame_type);
    if let Some(block) = suite_context.explicit_block() {
        if block.len() > HS_PARAM_BLOCK_MAX {
            return false;
        }
        out.extend_from_slice(&(block.len() as u16).to_be_bytes());
        out.extend_from_slice(block);
    }
    true
}

fn hs_decode_header(
    bytes: &[u8],
    frame_type: u8,
    payload_len: usize,
    mode: HandshakeSuiteMode,
    admit_context: bool,
) -> Result<(HsSuiteContext, usize), &'static str> {
    #[cfg(qsc_binding_fuzz_helper)]
    {
        let Some(frame_kind) = hs_fuzz_frame_kind(frame_type) else {
            return Err("handshake_type");
        };
        let header = crate::adversarial::binding_fuzz::decode_header(
            bytes,
            frame_kind,
            payload_len,
            hs_fuzz_suite_mode(mode),
            admit_context,
        )?;
        return Ok((
            hs_suite_context_from_fuzz(header.suite_context().clone())?,
            header.payload_offset(),
        ));
    }
    #[cfg(not(qsc_binding_fuzz_helper))]
    {
        if bytes.len() < 7 {
            return Err("handshake_len");
        }
        if &bytes[0..4] != HS_MAGIC {
            return Err("handshake_magic");
        }
        let ver = u16::from_be_bytes([bytes[4], bytes[5]]);
        if bytes[6] != frame_type {
            return Err("handshake_type");
        }
        match ver {
            HS_VERSION_LEGACY => {
                if mode == HandshakeSuiteMode::SuiteRequired {
                    return Err("REJECT_QSC_HS_LEGACY_REQUIRED");
                }
                if bytes.len() != 7 + payload_len {
                    return Err("handshake_len");
                }
                Ok((HsSuiteContext::LegacyV1, 7))
            }
            HS_VERSION_V2 => {
                if bytes.len() < 9 {
                    return Err("REJECT_QSC_HS_MALFORMED_LENGTH");
                }
                let block_len = u16::from_be_bytes([bytes[7], bytes[8]]) as usize;
                if block_len > HS_PARAM_BLOCK_MAX {
                    return Err("REJECT_QSC_HS_MALFORMED_LENGTH");
                }
                let payload_off = 9 + block_len;
                if bytes.len() != payload_off + payload_len {
                    return Err("REJECT_QSC_HS_MALFORMED_LENGTH");
                }
                let block = &bytes[9..payload_off];
                let suite_context = if admit_context {
                    hs_parse_parameter_block(block)?
                } else {
                    HsSuiteContext::ExplicitV2 {
                        block: block.to_vec(),
                        protocol_version: 0,
                        suite_id: 0,
                    }
                };
                Ok((suite_context, payload_off))
            }
            _ => Err("handshake_version"),
        }
    }
}

fn hs_encode_init(msg: &HsInit) -> Vec<u8> {
    let pk_len = hs_kem_pk_len();
    let sig_pk_len = hs_sig_pk_len();
    let ct_len = hs_kem_ct_len();
    if msg.kem_pk.len() != pk_len
        || msg.sig_pk.len() != sig_pk_len
        || msg.resp_kem_ct.len() != ct_len
    {
        return Vec::new();
    }
    let header_len = 4
        + 2
        + 1
        + msg
            .suite_context
            .explicit_block()
            .map_or(0, |b| 2 + b.len());
    let mut out = Vec::with_capacity(header_len + 16 + pk_len + sig_pk_len + 32 + ct_len);
    if !hs_encode_header(&mut out, HS_TYPE_INIT, &msg.suite_context) {
        return Vec::new();
    }
    out.extend_from_slice(&msg.session_id);
    out.extend_from_slice(&msg.kem_pk);
    out.extend_from_slice(&msg.sig_pk);
    out.extend_from_slice(&msg.dh_pub);
    out.extend_from_slice(&msg.resp_kem_ct);
    out
}

fn hs_decode_init(bytes: &[u8], mode: HandshakeSuiteMode) -> Result<HsInit, &'static str> {
    let pk_len = hs_kem_pk_len();
    let sig_pk_len = hs_sig_pk_len();
    let ct_len = hs_kem_ct_len();
    let payload_len = 16 + pk_len + sig_pk_len + 32 + ct_len;
    let (suite_context, off) = hs_decode_header(bytes, HS_TYPE_INIT, payload_len, mode, true)?;
    let mut sid = [0u8; 16];
    sid.copy_from_slice(&bytes[off..off + 16]);
    let pk_off = off + 16;
    let kem_pk = bytes[pk_off..(pk_off + pk_len)].to_vec();
    let sig_pk = bytes[(pk_off + pk_len)..(pk_off + pk_len + sig_pk_len)].to_vec();
    let mut dh_pub = [0u8; 32];
    let dh_off = pk_off + pk_len + sig_pk_len;
    dh_pub.copy_from_slice(&bytes[dh_off..dh_off + 32]);
    // NA-0633 (ENG-0038, C1): the initiator's encapsulation to the responder's identity KEM key.
    let ct_off = dh_off + 32;
    let resp_kem_ct = bytes[ct_off..(ct_off + ct_len)].to_vec();
    Ok(HsInit {
        suite_context,
        session_id: sid,
        kem_pk,
        sig_pk,
        dh_pub,
        resp_kem_ct,
    })
}

fn hs_encode_resp_no_auth(
    session_id: &[u8; 16],
    kem_ct: &[u8],
    sig_pk: &[u8],
    dh_pub: &[u8; 32],
    suite_context: &HsSuiteContext,
) -> Vec<u8> {
    let mut out = Vec::with_capacity(
        4 + 2
            + 1
            + suite_context.explicit_block().map_or(0, |b| 2 + b.len())
            + 16
            + kem_ct.len()
            + sig_pk.len()
            + 32,
    );
    if !hs_encode_header(&mut out, HS_TYPE_RESP, suite_context) {
        return Vec::new();
    }
    out.extend_from_slice(session_id);
    out.extend_from_slice(kem_ct);
    out.extend_from_slice(sig_pk);
    out.extend_from_slice(dh_pub);
    out
}

fn hs_encode_resp(msg: &HsResp) -> Vec<u8> {
    let ct_len = hs_kem_ct_len();
    let sig_pk_len = hs_sig_pk_len();
    let sig_len = hs_sig_sig_len();
    if msg.kem_ct.len() != ct_len || msg.sig_pk.len() != sig_pk_len || msg.sig.len() != sig_len {
        return Vec::new();
    }
    let header_len = 4
        + 2
        + 1
        + msg
            .suite_context
            .explicit_block()
            .map_or(0, |b| 2 + b.len());
    let mut out = Vec::with_capacity(header_len + 16 + ct_len + 32 + sig_pk_len + sig_len + 32);
    if !hs_encode_header(&mut out, HS_TYPE_RESP, &msg.suite_context) {
        return Vec::new();
    }
    out.extend_from_slice(&msg.session_id);
    out.extend_from_slice(&msg.kem_ct);
    out.extend_from_slice(&msg.mac);
    out.extend_from_slice(&msg.sig_pk);
    out.extend_from_slice(&msg.sig);
    out.extend_from_slice(&msg.dh_pub);
    out
}

fn hs_decode_resp_with_admission(
    bytes: &[u8],
    mode: HandshakeSuiteMode,
    admit_context: bool,
) -> Result<HsResp, &'static str> {
    let ct_len = hs_kem_ct_len();
    let sig_pk_len = hs_sig_pk_len();
    let sig_len = hs_sig_sig_len();
    let payload_len = 16 + ct_len + 32 + sig_pk_len + sig_len + 32;
    let (suite_context, off) =
        hs_decode_header(bytes, HS_TYPE_RESP, payload_len, mode, admit_context)?;
    let mut sid = [0u8; 16];
    sid.copy_from_slice(&bytes[off..off + 16]);
    let kem_off = off + 16;
    let kem_ct = bytes[kem_off..(kem_off + ct_len)].to_vec();
    let mut mac = [0u8; 32];
    let mac_off = kem_off + ct_len;
    mac.copy_from_slice(&bytes[mac_off..(mac_off + 32)]);
    let sig_pk_off = mac_off + 32;
    let sig_off = sig_pk_off + sig_pk_len;
    let sig_pk = bytes[sig_pk_off..sig_off].to_vec();
    let sig = bytes[sig_off..(sig_off + sig_len)].to_vec();
    let mut dh_pub = [0u8; 32];
    dh_pub.copy_from_slice(&bytes[(sig_off + sig_len)..(sig_off + sig_len + 32)]);
    Ok(HsResp {
        suite_context,
        session_id: sid,
        kem_ct,
        mac,
        sig_pk,
        sig,
        dh_pub,
    })
}

fn hs_decode_resp_pending(bytes: &[u8], mode: HandshakeSuiteMode) -> Result<HsResp, &'static str> {
    hs_decode_resp_with_admission(bytes, mode, false)
}

fn hs_encode_confirm(msg: &HsConfirm) -> Vec<u8> {
    let sig_len = hs_sig_sig_len();
    if msg.sig.len() != sig_len {
        return Vec::new();
    }
    let header_len = 4
        + 2
        + 1
        + msg
            .suite_context
            .explicit_block()
            .map_or(0, |b| 2 + b.len());
    let mut out = Vec::with_capacity(header_len + 16 + 32 + sig_len);
    if !hs_encode_header(&mut out, HS_TYPE_CONFIRM, &msg.suite_context) {
        return Vec::new();
    }
    out.extend_from_slice(&msg.session_id);
    out.extend_from_slice(&msg.mac);
    out.extend_from_slice(&msg.sig);
    out
}

fn hs_decode_confirm_with_admission(
    bytes: &[u8],
    mode: HandshakeSuiteMode,
    admit_context: bool,
) -> Result<HsConfirm, &'static str> {
    let sig_len = hs_sig_sig_len();
    let payload_len = 16 + 32 + sig_len;
    let (suite_context, off) =
        hs_decode_header(bytes, HS_TYPE_CONFIRM, payload_len, mode, admit_context)?;
    let mut sid = [0u8; 16];
    sid.copy_from_slice(&bytes[off..off + 16]);
    let mut mac = [0u8; 32];
    mac.copy_from_slice(&bytes[off + 16..off + 48]);
    let sig = bytes[off + 48..(off + 48 + sig_len)].to_vec();
    Ok(HsConfirm {
        suite_context,
        session_id: sid,
        mac,
        sig,
    })
}

fn hs_decode_confirm(bytes: &[u8], mode: HandshakeSuiteMode) -> Result<HsConfirm, &'static str> {
    hs_decode_confirm_with_admission(bytes, mode, true)
}

fn hs_decode_confirm_pending(
    bytes: &[u8],
    mode: HandshakeSuiteMode,
) -> Result<HsConfirm, &'static str> {
    hs_decode_confirm_with_admission(bytes, mode, false)
}

fn emit_peer_mismatch(peer: &str, pinned_fp: &str, seen_fp: &str) {
    // NA-0749: the marker fields `pinned_fp` / `seen_fp` now carry the CANONICAL FULL FORM.
    // The display helper that rendered the retired verification code is gone with it; a field
    // named `fp` carrying something that was not a fingerprint was the shape being repaired.
    let pinned_display = pinned_fp.to_string();
    let seen_display = seen_fp.to_string();
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

#[cfg(qsc_rng_failure_test_seam)]
fn hs_rng_failure_forced(label: &str) -> bool {
    std::env::var("QSC_RNG_FAILURE_TEST_SEAM")
        .ok()
        .map(|v| v == label || v == "all")
        .unwrap_or(false)
}

#[cfg(qsc_rng_failure_test_seam)]
fn hs_rand_bytes(label: &str, len: usize) -> Result<Vec<u8>, &'static str> {
    if hs_rng_failure_forced(label) {
        return Err("rng_failure_forced");
    }
    let mut out = vec![0u8; len];
    let mut rng = OsRng;
    rng.fill_bytes(&mut out);
    Ok(out)
}

#[cfg(not(qsc_rng_failure_test_seam))]
fn hs_rand_bytes(_label: &str, len: usize) -> Vec<u8> {
    let mut out = vec![0u8; len];
    let mut rng = OsRng;
    rng.fill_bytes(&mut out);
    out
}

#[cfg(qsc_rng_failure_test_seam)]
fn hs_session_id(label: &str) -> Result<[u8; 16], &'static str> {
    let bytes = hs_rand_bytes(label, 16)?;
    let mut sid = [0u8; 16];
    sid.copy_from_slice(&bytes[..16]);
    Ok(sid)
}

#[cfg(not(qsc_rng_failure_test_seam))]
fn hs_session_id(label: &str) -> [u8; 16] {
    let bytes = hs_rand_bytes(label, 16);
    let mut sid = [0u8; 16];
    sid.copy_from_slice(&bytes[..16]);
    sid
}

// Constant-time equality for fixed-length 32-byte MAC/tag values. Bit-for-bit
// equal to `a == b` for all inputs, but does not short-circuit on the first
// differing byte, closing the handshake MAC-comparison timing side-channel
// (ENG-0003). Accept/reject semantics and wire format are unchanged.
fn hs_ct_eq_32(a: &[u8; 32], b: &[u8; 32]) -> bool {
    let mut diff = 0u8;
    for i in 0..32 {
        diff |= a[i] ^ b[i];
    }
    diff == 0
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

fn hs_context_len(ctx: &HsSuiteContext) -> usize {
    ctx.explicit_block().map_or(0, |b| 2 + b.len())
}

fn hs_append_key_context(data: &mut Vec<u8>, ctx: &HsSuiteContext) {
    if let Some(block) = ctx.explicit_block() {
        data.extend_from_slice(&(block.len() as u16).to_be_bytes());
        data.extend_from_slice(block);
    }
}

/// NA-0634 (D571 Decision 2b): the canonical handshake key-schedule combiner. Derives a 32-byte
/// handshake secret from an ORDERED, length-prefixed list of labeled contributions through ONE KMAC,
/// replacing C1's incremental `resp_kem_ss` append. It is EXTENSIBLE: adding a contribution (e.g. the
/// X3DH DH products or a prekey KEM secret in NA-0635) is a list append, not a redesign. It stays in the
/// qsc handshake layer and feeds the UNCHANGED Suite-2 core (`init_from_base_handshake`); it introduces
/// NO prekey / three-DH structure here. The contribution count and per-item length prefixes keep the
/// encoding unambiguous; the fixed domain key provides domain separation (the secrecy is in the
/// contributions themselves — an HKDF-Extract-style construction). Both parties feed the identical
/// contributions in the identical order and derive the identical output.
const HS_ROOT_COMBINE_KEY: &[u8] = b"QSC.HS.ROOT.COMBINE.v1";

fn hs_root_combine(
    domain: &str,
    session_id: &[u8; 16],
    tag: u8,
    contributions: &[&[u8]],
    ctx: &HsSuiteContext,
) -> [u8; 32] {
    let c = StdCrypto;
    let mut data = Vec::with_capacity(16 + 1 + 1 + hs_context_len(ctx));
    data.extend_from_slice(session_id);
    data.push(tag);
    data.push(contributions.len() as u8);
    for contribution in contributions {
        data.extend_from_slice(&(contribution.len() as u16).to_be_bytes());
        data.extend_from_slice(contribution);
    }
    hs_append_key_context(&mut data, ctx);
    kmac_out::<32>(&c, HS_ROOT_COMBINE_KEY, domain, &data)
}

fn hs_pq_init_ss(
    ss_pq: &[u8],
    session_id: &[u8; 16],
    resp_kem_ss: &[u8],
    ctx: &HsSuiteContext,
) -> [u8; 32] {
    // NA-0634 (D571 Decision 2b): canonical ordered-list combiner over C1's ACTUAL PQ contributions —
    // the ephemeral handshake KEM secret (`ss_pq`) and the responder-identity KEM secret (`resp_kem_ss`,
    // C1). Binding `resp_kem_ss` here binds pq_init_ss — hence the transcript MAC AND the Suite-2 root —
    // to the responder's verified identity, so a wrong responder derives a different value and fails the
    // initiator's transcript-MAC check (explicit reject at B1). NO prekey / three-DH structure is added.
    hs_root_combine("QSC.HS.PQ", session_id, 0x01, &[ss_pq, resp_kem_ss], ctx)
}

fn hs_ephemeral_keypair() -> ([u8; 32], [u8; 32]) {
    let c = StdCrypto;
    let (sk, pk) = c.keypair();
    (sk.0, pk.0)
}

fn hs_dh_init_from_shared(
    dh_shared: &[u8; 32],
    session_id: &[u8; 16],
    ctx: &HsSuiteContext,
) -> [u8; 32] {
    // NA-0634 (D571 Decision 2b): the ephemeral DH contribution through the same canonical combiner,
    // structured so future X3DH DH products (DH1..DH4) extend this list without a redesign. The Suite-2
    // core keeps the DH and PQ inputs separate (the hybrid property) — this only makes the derivation
    // of each input canonical and extensible.
    hs_root_combine("QSC.HS.DHINIT", session_id, 0x02, &[dh_shared], ctx)
}

/// ENG-0034: the establishment DH rejected a non-contributory (small-subgroup) peer key. This is a
/// qsc-local log marker, not a canonical DOC-CAN-003 reason code.
const HS_DH_NONCONTRIBUTORY: &str = "dh_noncontributory";

fn hs_dh_shared(self_sk: &[u8], peer_pub: &[u8]) -> Result<[u8; 32], &'static str> {
    if self_sk.len() != 32 || peer_pub.len() != 32 {
        return Err("handshake_dh_len");
    }
    let mut sk = [0u8; 32];
    sk.copy_from_slice(self_sk);
    let mut pk = [0u8; 32];
    pk.copy_from_slice(peer_pub);
    let c = StdCrypto;
    let dh_out = c.dh(&X25519Priv(sk), &X25519Pub(pk));
    // RFC 7748 §6.1: X25519 accepts small-order points and returns the all-zero shared secret rather
    // than erroring. `hs_dh_pub_is_all_zero` screens only the all-zero ENCODING — one of the eight
    // low-order points. Checking the OUTPUT catches all eight, and is what the RFC prescribes for
    // protocols requiring contributory behaviour.
    if dh_out.iter().all(|b| *b == 0) {
        return Err(HS_DH_NONCONTRIBUTORY);
    }
    Ok(dh_out)
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

fn hs_confirm_key(
    pq_init_ss: &[u8; 32],
    session_id: &[u8; 16],
    th: &[u8; 32],
    ctx: &HsSuiteContext,
) -> [u8; 32] {
    let c = StdCrypto;
    let mut data = Vec::with_capacity(16 + 32 + hs_context_len(ctx));
    data.extend_from_slice(session_id);
    data.extend_from_slice(th);
    hs_append_key_context(&mut data, ctx);
    kmac_out::<32>(&c, pq_init_ss, "QSC.HS.CONFIRM", &data)
}

fn hs_confirm_mac(
    k_confirm: &[u8; 32],
    session_id: &[u8; 16],
    th: &[u8; 32],
    ctx: &HsSuiteContext,
) -> [u8; 32] {
    let c = StdCrypto;
    let mut data = Vec::with_capacity(16 + 32 + 2 + hs_context_len(ctx));
    data.extend_from_slice(session_id);
    data.extend_from_slice(th);
    data.extend_from_slice(b"A2");
    hs_append_key_context(&mut data, ctx);
    kmac_out::<32>(&c, k_confirm, "QSC.HS.A2", &data)
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

fn hs_require_primary_identity_pin<F>(
    peer: &str,
    seen_fp: &str,
    read_pin: F,
    speculative: bool,
) -> Result<(), &'static str>
where
    F: Fn(&str) -> Result<Option<String>, ErrorCode>,
{
    match read_pin(peer) {
        Ok(Some(pinned)) => {
            #[cfg(qsc_binding_fuzz_helper)]
            let pin_matches = {
                let _canonical_pin_matches = identity_pin_matches_seen_identity(pinned.as_str(), seen_fp);
                crate::adversarial::binding_fuzz::trusted_pin_matches_seen_identity(pinned.as_str(), seen_fp)
            };
            #[cfg(not(qsc_binding_fuzz_helper))]
            let pin_matches = identity_pin_matches_seen_identity(pinned.as_str(), seen_fp);
            if !pin_matches {
                // NA-0768 (`RULING_006` sec 2): a SPECULATIVE offer that is not this
                // candidate's is not a peer mismatch -- it is the fan-out's question
                // answered "no". ONE non-security marker, NO fingerprint, once per
                // (frame, candidate). The gate's DECISION is unchanged: still Err, still
                // before the first KEM.
                if speculative {
                    emit_marker("hs_offer_not_addressee", None, &[("peer", peer)]);
                    return Err("not_addressee");
                }
                emit_peer_mismatch(peer, pinned.as_str(), seen_fp);
                emit_marker("handshake_reject", None, &[("reason", "peer_mismatch")]);
                return Err("peer_mismatch");
            }
            let fp_display = seen_fp.to_string();
            emit_marker(
                "identity_ok",
                None,
                &[("peer", peer), ("fp", fp_display.as_str())],
            );
            Ok(())
        }
        Ok(None) => {
            // ⚠ THIS ARM PRINTS THE **SENDER'S** FINGERPRINT (`seen_fp`). Under a
            // speculative offer that is a FOREIGN fingerprint inside a command run for a
            // different contact, which `RULING_006` sec 2(b) forbids.
            if speculative {
                emit_marker("hs_offer_not_addressee", None, &[("peer", peer)]);
                return Err("not_addressee");
            }
            let fp_display = seen_fp.to_string();
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

/// NA-0634 (D571 Decision 2a): REQUIRE the responder's signing identity to be pinned AND to match.
/// Mirrors `hs_require_primary_identity_pin` but for the signing-key fingerprint (`sig_fp`), which
/// full-identity provisioning now populates. A missing pin (`None`) is a fail-closed REJECT — closing the
/// ENG-0038 never-populated-`sig_fp` weakness (the old OPTIONAL check passed on `None`, so the signing
/// key was never authenticated to the verified identity).
fn hs_require_sig_identity_pin<F>(
    peer: &str,
    seen_fp: &str,
    read_pin: F,
) -> Result<(), &'static str>
where
    F: Fn(&str) -> Result<Option<String>, ErrorCode>,
{
    match read_pin(peer) {
        Ok(Some(pinned)) => {
            #[cfg(qsc_binding_fuzz_helper)]
            let pin_matches = {
                let _canonical_pin_matches = identity_pin_matches_seen(pinned.as_str(), seen_fp);
                crate::adversarial::binding_fuzz::trusted_pin_matches_seen(pinned.as_str(), seen_fp)
            };
            #[cfg(not(qsc_binding_fuzz_helper))]
            let pin_matches = identity_pin_matches_seen(pinned.as_str(), seen_fp);
            if !pin_matches {
                emit_peer_mismatch(peer, pinned.as_str(), seen_fp);
                emit_marker(
                    "handshake_reject",
                    None,
                    &[("reason", "responder_sig_mismatch")],
                );
                return Err("responder_sig_mismatch");
            }
            Ok(())
        }
        Ok(None) => {
            emit_marker(
                "handshake_reject",
                None,
                &[("reason", "responder_sig_unpinned")],
            );
            Err("responder_sig_unpinned")
        }
        Err(_) => {
            emit_marker(
                "handshake_reject",
                None,
                &[("reason", "responder_sig_pin_failed")],
            );
            Err("responder_sig_pin_failed")
        }
    }
}

fn hs_check_optional_identity_pin<F>(
    peer: &str,
    seen_fp: &str,
    read_pin: F,
) -> Result<(), &'static str>
where
    F: Fn(&str) -> Result<Option<String>, ErrorCode>,
{
    match read_pin(peer) {
        Ok(Some(pinned)) => {
            #[cfg(qsc_binding_fuzz_helper)]
            let pin_matches = {
                let _canonical_pin_matches = identity_pin_matches_seen(pinned.as_str(), seen_fp);
                crate::adversarial::binding_fuzz::trusted_pin_matches_seen(pinned.as_str(), seen_fp)
            };
            #[cfg(not(qsc_binding_fuzz_helper))]
            let pin_matches = identity_pin_matches_seen(pinned.as_str(), seen_fp);
            if !pin_matches {
                emit_peer_mismatch(peer, pinned.as_str(), seen_fp);
                emit_marker("handshake_reject", None, &[("reason", "peer_mismatch")]);
                return Err("peer_mismatch");
            }
            let fp_display = seen_fp.to_string();
            emit_marker(
                "identity_ok",
                None,
                &[("peer", peer), ("fp", fp_display.as_str())],
            );
            Ok(())
        }
        Ok(None) => Ok(()),
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

#[allow(clippy::too_many_arguments)]
fn hs_build_session(
    authenticated: bool,
    role_is_a: bool,
    session_id: [u8; 16],
    dh_init: [u8; 32],
    pq_init_ss: [u8; 32],
    dh_self_pub: [u8; 32],
    dh_peer_pub: [u8; 32],
    dh_self_priv: [u8; 32],
) -> Result<Suite2SessionState, &'static str> {
    let c = StdCrypto;
    let mut st = init_from_base_handshake(
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
    )?;
    // NA-0620 (ENG-0012 Stage 1a): carry the local X25519 ephemeral private key into the
    // session's DH-ratchet state so the Stage 1b send-side ratchet can use it. Plumbing only —
    // no message-path code reads it in Stage 1a.
    st.set_dh_self_priv(dh_self_priv);
    Ok(st)
}

fn hs_pending_legacy_path(dir: &Path, self_label: &str, peer: &str) -> PathBuf {
    dir.join(format!("handshake_pending_{}_{}.json", self_label, peer))
}

fn hs_pending_secret_key(self_label: &str, peer: &str) -> String {
    format!("handshake.pending.{}.{}", self_label, peer)
}

/// NA-0711 (D647 A4 Δ35): what the lookup actually SAW, because "absent" and "cleared" are
/// different facts that the client collapsed into one. ⚠ Three eras of this program have now
/// produced the identical `present=false` + reject marker pair from three DIFFERENT causes -- a
/// vault clobber that destroyed the record, a lookup under the wrong key with the record intact,
/// and a completed handshake whose record was cleared on purpose (a test that is green in CI right
/// now). A marker that cannot tell them apart is what made each of those an investigation.
#[derive(Clone, Copy, PartialEq, Eq)]
enum HsPendingState {
    Absent,
    Cleared,
    Present,
}

impl HsPendingState {
    fn as_str(self) -> &'static str {
        match self {
            HsPendingState::Absent => "absent",
            HsPendingState::Cleared => "cleared",
            HsPendingState::Present => "present",
        }
    }
}

/// NA-0768 (`D-1409`) FORM (ii') -- THE SECOND COMMIT SHAPE'S WITNESS, MADE READABLE.
///
/// ⚠⚠ WHY THIS EXISTS. `perform_handshake_poll_with_tokens` has **TWO** durable-commit
/// shapes, not one: the responder branch commits a SESSION (`qsp_session_store`, then
/// clears the pending), and the no-pending branch commits a **PENDING** and pushes a B1.
/// A caller that witnesses consumption by "a session appeared" is UNSOUND for the second:
/// it records `consumed=false` for a frame that stored durable state and sent a frame.
/// This function is that branch's witness, and the poll ALREADY USES IT AS SUCH -- see the
/// guard at the end of the no-pending arm: *"the pending record this frame created must be
/// durably loadable before the frame may be acked"*.
///
/// ⚠ IT RETURNS `Result`, DELIBERATELY. An unreadable record is NOT "absent"; collapsing
/// the two is the defect `B3` names. The caller must distinguish them.
pub(crate) fn hs_pending_present(self_label: &str, peer: &str) -> Result<bool, ErrorCode> {
    let (rec, _state) = hs_pending_load_state(self_label, peer)?;
    Ok(rec.is_some())
}

fn hs_pending_load_state(
    self_label: &str,
    peer: &str,
) -> Result<(Option<HandshakePending>, HsPendingState), ErrorCode> {
    let secret_key = hs_pending_secret_key(self_label, peer);
    match vault::secret_get(&secret_key) {
        Ok(Some(v)) if !v.is_empty() => {
            let pending: HandshakePending =
                serde_json::from_str(&v).map_err(|_| ErrorCode::ParseFailed)?;
            return Ok((Some(pending), HsPendingState::Present));
        }
        Ok(Some(_)) => {
            // An EMPTY value is a record that was cleared on completion, not one that never
            // existed. `hs_pending_clear` writes "" rather than deleting.
            return Ok((None, HsPendingState::Cleared));
        }
        Ok(_) => {}
        Err("vault_missing" | "vault_locked") => return Err(ErrorCode::IdentitySecretUnavailable),
        Err(_) => return Err(ErrorCode::IoReadFailed),
    }

    // The legacy on-disk arm is keyed by the SAME label, so it diverges by the same mechanism.
    let (dir, source) = config_dir()?;
    let path = hs_pending_legacy_path(&dir, self_label, peer);
    if !path.exists() {
        return Ok((
            None,
            HsPendingState::Absent,
        ));
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
    Ok((Some(pending), HsPendingState::Present))
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

fn hs_pending_suite_context(pending: &HandshakePending) -> Result<HsSuiteContext, &'static str> {
    match pending.suite_context.as_deref() {
        Some(block) => hs_parse_parameter_block(block),
        None => Ok(HsSuiteContext::LegacyV1),
    }
}

fn hs_contexts_match(a: &HsSuiteContext, b: &HsSuiteContext) -> bool {
    match (a, b) {
        (HsSuiteContext::LegacyV1, HsSuiteContext::LegacyV1) => true,
        (
            HsSuiteContext::ExplicitV2 { block: a_block, .. },
            HsSuiteContext::ExplicitV2 { block: b_block, .. },
        ) => a_block == b_block,
        _ => false,
    }
}

fn hs_reject_context_mismatch() {
    hs_emit_suite_reject("REJECT_QSC_HS_CONTEXT_MISMATCH");
}

fn hs_reject_key_context() {
    hs_emit_suite_reject("REJECT_QSC_HS_KEY_CONTEXT");
}

fn hs_reject_replay() {
    hs_emit_suite_reject("REJECT_QSC_HS_REPLAY");
}

fn hs_zero32(v: &[u8; 32]) -> bool {
    v.iter().all(|b| *b == 0)
}

fn hs_send_ready_from_session(st: &Suite2SessionState) -> bool {
    !(hs_zero32(&st.send.ck_ec) || hs_zero32(&st.send.ck_pq))
}

fn hs_status_truth(st: &Suite2SessionState) -> (&'static str, &'static str, Option<&'static str>) {
    if !hs_send_ready_from_session(st) {
        return ("established_recv_only", "yes", Some("chainkey_unset"));
    }
    if st.recv.nr == 0 {
        return ("awaiting_peer_confirm", "no", None);
    }
    ("established", "yes", None)
}

pub fn handshake_status(peer: Option<&str>) -> CliResult {
    require_unlocked("handshake_status")?;
    let peer_label = peer.unwrap_or("peer-0");
    if let Err(code) = enforce_peer_not_blocked(peer_label) {
        return Err(CliError::code(code));
    }
    let (peer_fp, pinned) = identity_peer_status(peer_label);
    let pinned_s = if pinned { "true" } else { "false" };
    let (send_ready, send_ready_reason) = qsp_send_ready_tuple(peer_label);
    let send_ready_s = if send_ready { "yes" } else { "no" };
    match qsp_session_load(peer_label) {
        Ok(Some(st)) => {
            let (status, peer_confirmed, local_reason) = hs_status_truth(&st);
            if let Some(reason) = local_reason {
                emit_marker(
                    "handshake_status",
                    None,
                    &[
                        ("status", status),
                        ("peer", peer_label),
                        ("peer_fp", peer_fp.as_str()),
                        ("pinned", pinned_s),
                        ("peer_confirmed", peer_confirmed),
                        ("send_ready", send_ready_s),
                        ("send_ready_reason", reason),
                    ],
                );
            } else {
                emit_marker(
                    "handshake_status",
                    None,
                    &[
                        ("status", status),
                        ("peer", peer_label),
                        ("peer_fp", peer_fp.as_str()),
                        ("pinned", pinned_s),
                        ("peer_confirmed", peer_confirmed),
                        ("send_ready", send_ready_s),
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
                    ("peer_confirmed", "no"),
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
                    ("peer_confirmed", "unknown"),
                    ("send_ready", send_ready_s),
                    ("send_ready_reason", send_ready_reason),
                ],
            );
        }
    }
    Ok(())
}

/// NA-0681 (D616 F1): how the A1 frame is delivered.
///
/// `Direct` is the shipped behaviour, byte-identical. `InviteSlot` wraps the SAME A1 bytes
/// in a `QSLH-1` envelope alongside the initiator's bundle and route token, and pushes to
/// the invite slot with its one-shot ticket. The A1 frame itself is never re-encoded --
/// `hs_transcript_mac` binds those bytes, so the wrapper is transparent to the transcript,
/// which is the entire reason wrapping is safe where changing the frame would not be.
pub(crate) enum A1Delivery<'a> {
    Direct,
    InviteSlot {
        bundle: &'a [u8],
        self_route_token: &'a str,
        ticket: &'a str,
    },
}

fn hs_emit_a1(bytes: &[u8], suite_context: &HsSuiteContext) {
    let size_s = bytes.len().to_string();
    let pk_len_s = hs_kem_pk_len().to_string();
    let sig_pk_len_s = hs_sig_pk_len().to_string();
    // NA-0633 (ENG-0038, C1): A1 now also carries the initiator's encapsulation to the responder's
    // identity KEM key (one ML-KEM ciphertext); report its length so consumers can assert the layout.
    let resp_kem_ct_len_s = hs_kem_ct_len().to_string();
    let hs_version_s = suite_context.wire_version().to_string();
    emit_marker(
        "handshake_send",
        None,
        &[
            ("msg", "A1"),
            ("size", size_s.as_str()),
            ("kem_pk_len", pk_len_s.as_str()),
            ("sig_pk_len", sig_pk_len_s.as_str()),
            ("resp_kem_ct_len", resp_kem_ct_len_s.as_str()),
            ("hs_version", hs_version_s.as_str()),
            ("suite_context", suite_context.mode_label()),
        ],
    );
}

pub(crate) fn perform_handshake_init_with_route(
    self_label: &str,
    peer: &str,
    relay: &str,
    route_token: &str,
    suite_mode: HandshakeSuiteMode,
    delivery: A1Delivery<'_>,
) -> Result<(), &'static str> {
    enforce_peer_not_blocked(peer)?;
    {
        let _lock = hs_lifecycle_lock()?;
        if let Some(mut c) = hs_lifecycle_find(self_label, peer)? {
            if matches!(delivery, A1Delivery::Direct) {
                return Err("handshake_lifecycle_conflict");
            }
            if c.selected.is_some()
                && c.outgoing.as_ref().is_none_or(|o| o.reply.route != route_token)
            {
                return Err("contacts_session_exists");
            }
            hs_lifecycle_recover(&mut c)?;
            let retry = c
                .outgoing
                .as_ref()
                .filter(|o| o.reply.route == route_token)
                .filter(|_| c.selected.is_none())
                .map(|o| o.reply.clone());
            drop(_lock);
            if let Some(reply) = retry {
                hs_delivery_send(&reply)?;
                hs_lifecycle_after_send(&c, Some(&reply))?;
            }
            emit_marker("handshake_coalesced", None, &[("peer", peer)]);
            return Ok(());
        }
    }
    let peer_fp = match identity_read_pin(peer) {
        Ok(Some(peer_fp)) => peer_fp,
        Ok(None) => {
            emit_marker(
                "identity_unknown",
                None,
                &[("peer", peer), ("seen_fp", "unknown")],
            );
            emit_marker("handshake_reject", None, &[("reason", "identity_unknown")]);
            return Err("identity_unknown");
        }
        Err(_) => {
            emit_marker(
                "handshake_reject",
                None,
                &[("reason", "identity_pin_failed")],
            );
            return Err("identity_pin_failed");
        }
    };
    // NA-0633 (ENG-0038, C1): load the peer's PINNED identity KEM key and encapsulate to it, so the
    // responder must prove KEM-secret possession to complete the handshake. A contact without the full
    // key (legacy/incomplete) fails closed here — no fallback to the unauthenticated path.
    let peer_kem_pk = match identity_read_peer_kem_pk(peer) {
        Ok(Some(k)) => k,
        _ => {
            emit_marker(
                "handshake_reject",
                None,
                &[("reason", "peer_identity_key_missing")],
            );
            return Err("peer_identity_key_missing");
        }
    };
    let (resp_kem_ct, resp_kem_ss) = match StdCrypto.encap(&peer_kem_pk) {
        Ok(v) => v,
        Err(_) => {
            emit_marker(
                "handshake_reject",
                None,
                &[("reason", "peer_identity_key_invalid")],
            );
            return Err("peer_identity_key_invalid");
        }
    };
    let IdentityKeypair {
        kem_pk,
        kem_sk,
        sig_pk,
        sig_sk: _,
    } = identity_self_kem_keypair(self_label).map_err(|e| e.as_str())?;
    #[cfg(qsc_rng_failure_test_seam)]
    let sid = hs_session_id("QSC.HS.SID")?;
    #[cfg(not(qsc_rng_failure_test_seam))]
    let sid = hs_session_id("QSC.HS.SID");
    let (dh_sk, dh_pub) = hs_ephemeral_keypair();
    let suite_context = hs_suite_context_for_mode(suite_mode);
    let msg = HsInit {
        suite_context: suite_context.clone(),
        session_id: sid,
        kem_pk: kem_pk.clone(),
        sig_pk: sig_pk.clone(),
        dh_pub,
        resp_kem_ct: resp_kem_ct.clone(),
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
        resp_kem_ct: resp_kem_ct.clone(),
        resp_kem_ss: resp_kem_ss.clone(),
        peer_sig_fp: None,
        peer_sig_pk: None,
        peer_fp: Some(peer_fp),
        role: "initiator".to_string(),
        confirm_key: None,
        transcript_hash: None,
        pending_session: None,
        suite_context: suite_context.as_pending_block(),
    };
    if let A1Delivery::InviteSlot {
        bundle,
        self_route_token,
        ticket,
    } = &delivery
    {
        let wrapped = crate::invite::encode_envelope(&crate::invite::HandshakeEnvelope {
            bundle: bundle.to_vec(),
            route_token: self_route_token.to_string(),
            a1: bytes.clone(),
        })?;
        let reply = HsDelivery {
            relay: relay.into(),
            route: route_token.into(),
            bytes: wrapped.clone(),
            ticket: Some(ticket.to_string()),
        };
        let _lock = hs_lifecycle_lock()?;
        // Recheck after key generation: a responder might have reserved first.
        if let Some(mut c) = hs_lifecycle_find(self_label, peer)? {
            hs_lifecycle_recover(&mut c)?;
            emit_marker("handshake_coalesced", None, &[("peer", peer)]);
            return Ok(());
        }
        let mut c = hs_lifecycle_new(self_label, peer)?;
        c.outgoing = Some(HsCandidate {
            pending,
            receipt: hs_receipt(route_token, &wrapped, &bytes, self_route_token)?,
            reply: reply.clone(),
        });
        hs_lifecycle_put(&c)?;
        hs_lifecycle_recover(&mut c)?;
        drop(_lock);
        hs_emit_a1(&bytes, &suite_context);
        hs_delivery_send(&reply)?;
        hs_lifecycle_after_send(&c, Some(&reply))?;
        emit_marker(
            "handshake_start",
            None,
            &[("role", "initiator"), ("peer", peer)],
        );
        return Ok(());
    }
    {
        let _lock = hs_lifecycle_lock()?;
        if hs_lifecycle_find(self_label, peer)?.is_some() {
            return Err("handshake_lifecycle_conflict");
        }
        hs_pending_store(&pending).map_err(|_| "handshake_pending_store_failed")?;
    }
    emit_marker(
        "handshake_start",
        None,
        &[("role", "initiator"), ("peer", peer)],
    );
    hs_emit_a1(&bytes, &suite_context);
    match delivery {
        A1Delivery::Direct => transport::relay_inbox_push(relay, route_token, &bytes)?,
        A1Delivery::InviteSlot {
            bundle,
            self_route_token,
            ticket,
        } => {
            let env = crate::invite::HandshakeEnvelope {
                bundle: bundle.to_vec(),
                route_token: self_route_token.to_string(),
                // VERBATIM. Not re-encoded, not inspected.
                a1: bytes.clone(),
            };
            let wrapped = crate::invite::encode_envelope(&env)?;
            transport::relay_inbox_push_with_ticket(relay, route_token, &wrapped, Some(ticket))?
        }
    }
    Ok(())
}

fn handshake_init_with_route(
    self_label: &str,
    peer: &str,
    relay: &str,
    route_token: &str,
    suite_mode: HandshakeSuiteMode,
) -> CliResult {
    require_unlocked("handshake_init")?;
    if let Err(code) =
        // The shipped path: bare A1 to the peer's route token. Byte-identical to before.
        perform_handshake_init_with_route(
            self_label,
            peer,
            relay,
            route_token,
            suite_mode,
            A1Delivery::Direct,
        )
    {
        return Err(CliError::code(code));
    }
    Ok(())
}

pub fn handshake_init_with_suite_mode(
    self_label: &str,
    peer: &str,
    relay: &str,
    suite_mode: HandshakeSuiteMode,
) -> CliResult {
    if !vault_unlocked() {
        require_unlocked("handshake_init")?;
    }
    let peer_channel = resolve_peer_device_target(peer, false)
        .map(|v| v.channel)
        .unwrap_or_else(|_| peer.to_string());
    let route_token = relay_peer_route_token(peer).map_err(|code| CliError::code(code))?;
    handshake_init_with_route(
        self_label,
        peer_channel.as_str(),
        relay,
        route_token.as_str(),
        suite_mode,
    )?;
    Ok(())
}

// D581 KEEP -> NA-0646 (D582): part of the library's pub GUI surface, seeded for the GUI
// phase; dormant until the GUI consumes it (dead_code allowance retained meanwhile).
#[allow(dead_code)]
pub fn handshake_init(self_label: &str, peer: &str, relay: &str) -> CliResult {
    handshake_init_with_suite_mode(self_label, peer, relay, HandshakeSuiteMode::LegacyCompat)?;
    Ok(())
}

/// NA-0775 (`D-1418`): what the poll did with the frames it was handed.
///
/// ⚠⚠ A CALLER MAY ACK ONLY ON [`PollOutcome::Consumed`]. Every other value means the frame's
/// LAST EFFECT did not happen, so the frame must stay leased and redeliver. An `Err` means the
/// same and already reaches every caller through `?`.
///
/// ⚠ THE ENUM IS NOT A CONVENIENCE, IT IS THE FAIL-CLOSED PROPERTY. A twenty-second exit written
/// as `Ok(())` DOES NOT COMPILE, so a future exit cannot silently inherit "success" -- which is
/// exactly how `ENG-0269` came to exist beside a correctly-placed ack.
///
/// EXACTLY THREE exits are `Consumed`: the A2 push, the responder's durable commit (which pushes
/// nothing, and is the reason this is an enum and not "did we push"), and the B1 push.
/// NA-0775 (`D-1418`) `RULING_007`: THE THIRD VALUE. `AlreadyComplete` means THE WORK THIS
/// FRAME WOULD DO IS ALREADY DURABLY DONE -- re-processing it is a no-op and the frame should
/// be RETIRED. Callers ack on `Consumed` OR `AlreadyComplete`; never on `NotConsumed`.
///
/// ⚠⚠ IT IS NOT A WEAKER `Consumed` AND MUST NEVER BE RETURNED WHERE THE WORK IS MERELY
/// *BELIEVED* DONE. Every site that returns it compares the frame's own `session_id` against
/// the STORED session's, so "already done" is a measured fact about this frame, not an
/// inference from the peer having some session.
///
/// ⚠⚠ WHY IT EXISTS. A two-valued contract cannot express "already done", and that one gap
/// produced THREE defects in this lane in three different costumes: `ENG-0281` (a frame no
/// pass can CONSUME), `E-4` (state a redelivery RE-DERIVES wrongly), and `E-5` (a frame no
/// pass can RETIRE -- `t5f`, where a lost ack made the retirement unreachable forever).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[must_use]
pub(crate) enum PollOutcome {
    Consumed,
    AlreadyComplete,
    NotConsumed,
}

/// NA-0775 (`D-1418`): the `session_id` a frame carries, if it is a RESP or a CONFIRM.
///
/// Both decoders work from bytes and the suite mode alone -- neither needs a pending record --
/// which is what makes a POSITIVE "already complete" test possible in the no-pending branch.
/// Returns `None` for anything else, including an INIT: an A1 is not evidence that a handshake
/// finished.
fn hs_frame_session_id(bytes: &[u8], mode: HandshakeSuiteMode) -> Option<[u8; 16]> {
    if let Ok(resp) = hs_decode_resp_pending(bytes, mode) {
        return Some(resp.session_id);
    }
    if let Ok(confirm) = hs_decode_confirm(bytes, mode) {
        return Some(confirm.session_id);
    }
    None
}

/// NA-0681 (D616 F1): where the poll's frames come from.
///
/// The invite flow needs the responder logic to run over frames it has ALREADY pulled and
/// unwrapped from a `QSLH-1` envelope, because the peer's route token is inside that
/// envelope and is not known until it is opened -- so it cannot be passed in before the
/// pull. `Relay` is the shipped behaviour, byte-identical.
pub(crate) enum HsPollSource<'a> {
    Relay,
    Invitation {
        items: &'a [crate::InboxPullItem],
        envelope: &'a [u8],
        mailbox: &'a str,
    },
    /// NA-0768 (`D-1409`, `RULING_006` sec 2): a SPECULATIVE offer -- the fan-out asking
    /// "is this frame yours?" of a candidate it has no reason to believe owns it.
    /// ⚠ IT CHANGES NO GATE AND NO CONTROL FLOW. The identity pin still fails closed
    /// BEFORE the first KEM. What it changes is what a rejection SAYS: a speculative
    /// rejection is not a peer mismatch and must not fire the security marker or print a
    /// fingerprint (`PR-11`: a security marker fires only on a security event).
    ProvidedSpeculative(&'a [crate::InboxPullItem]),
}

/// NA-0681 (D616 F1): how the reply leaves.
///
/// `None` is the shipped behaviour -- B1 goes bare to the peer's route token. `Envelope`
/// wraps it, VERBATIM, in a `QSLH-1` response carrying the responder's own route token, so
/// the initiator learns where to send A2. Without this the invite handshake dead-ends after
/// B1: the initiator reached the responder through a one-shot invite slot whose ticket is
/// now burned, and has no other address for them.
///
/// This is the "authenticated in-session announcement" P3 already names as the mechanism by
/// which endpoints are exchanged -- never by re-invite.
pub(crate) struct HsReplyWrap<'a> {
    pub(crate) self_route_token: &'a str,
}

/// NA-0742 (D-1378): retire a frame THIS POLL pulled and consumed, and never fail the poll on a
/// lost ack — `receive`'s posture, for `receive`'s reason: the state is already durable, the lease
/// expires, and the redelivery is handled.
fn hs_emit_producer_ack(relay: &str, route_token: &str, id: &str) {
    let ids = [id.to_string()];
    match transport::producer_ack(relay, route_token, &ids) {
        Ok(crate::transport::AckFlushOutcome::Acked(n)) => {
            let acked = n.to_string();
            emit_marker(
                "producer_ack",
                None,
                &[("caller", "poll"), ("sent", "1"), ("acked", acked.as_str())],
            );
        }
        Ok(crate::transport::AckFlushOutcome::LegacyComplete) => {
            emit_marker(
                "ack_legacy_complete",
                None,
                &[("caller", "poll"), ("count", "1")],
            );
        }
        Err(code) => {
            emit_marker(
                "producer_ack",
                Some(code),
                &[("caller", "poll"), ("sent", "1"), ("acked", "0")],
            );
        }
    }
}


/// NA-0775 (`D-1418`) A1 -- COMMIT THE SESSION THIS FRAME PRODUCED, GUARDED AGAINST A LATE
/// LANDING. Called from the initiator A2 branch AFTER the push, never before it.
///
/// NA-0775 (`D-1418`) A1 -- THE COMMIT AND THE CLEAR, MOVED BELOW THE PUSH,
/// AND THE LATE-LANDING GUARD AROUND THEM. `STOP_NA0775_002_AMENDED` sec 1.
///
/// ⚠⚠ WHY THE COMMIT MOVED (E-4): with it above the signing, a failed A2
/// signature left a STORED session the peer never learned of, and the send
/// gate opens on blob existence alone (`qsp_status_tuple`,
/// protocol_state/mod.rs:90-91) -- so a redelivery would re-derive the
/// identical epoch-0 chain and overwrite an advanced one. Same key, same
/// counter, new plaintext.
///
/// ⚠⚠ WHY THE GUARD (cold read F-2 Variant A): moving the commit puts a
/// durable, unserialised write AFTER the longest-latency call in this
/// branch. A poll stalled between the push returning and this store can
/// wake a lease later -- after a SECOND poll completed the same handshake
/// and the user advanced the ratchet -- and write ck0/ns=0 over it.
///
/// ⚠ THE PREDICATE IS SESSION-ID-SCOPED ON PURPOSE. A bare "never move ns
/// backward" would refuse a LEGITIMATE re-handshake, whose session_id is
/// 128 fresh CSPRNG bits (`hs_session_id`; the only draw site is the A1
/// path). The SAME session_id means "another pass of THIS handshake
/// already committed", and its state is what we would have written.
///
/// ⚠⚠ G-8 -- A SPECIFIED CONSTRAINT, NOT A FORMATTING DETAIL. The nested
/// acquisitions below are safe ONLY because every one resolves the SAME
/// directory: this lock, `qsp_session_store_inner` (protocol_state:979)
/// and `qsp_session_store_key_get_or_create` (protocol_state:190) call
/// `config_dir()` directly, and `vault::secret_set` (vault:247) reaches
/// it INDIRECTLY via `vault_path_resolved()` (vault:1542), which returns
/// `config_dir()`'s dir unchanged. The constraint is that `config_dir()`
/// is STABLE for the duration of this guard: it re-reads the environment
/// on every call and nothing in production mutates those variables
/// mid-call. The registry carries the nesting by DEPTH rather than
/// issuing a second flock (model/mod.rs:95-105).
///
/// ⚠ LOCK_NB: a contended acquisition FAILS rather than waits, and that is
/// ORDINARY here rather than rare -- `qsc send` holds this same lock across
/// a network call today (transport/mod.rs:3748 -> :4005). The Err arms are
/// a path users will take, which is why they are marked (G-6).
fn hs_commit_session_guarded(
    self_label: &str,
    peer: &str,
    st: &Suite2SessionState,
) -> Result<(), &'static str> {
    // ⚠ `config_dir()` yields an `ErrorCode`, not a `&str`, so it is
    // matched rather than `?`-ed -- and its failure is MARKED like the
    // other two arms (G-6). A resolver failure here is the same class of
    // event as a denied lock: the caller must be able to tell them apart.
    let (lock_dir, lock_source) = match config_dir() {
        Ok(v) => v,
        Err(code) => {
            emit_marker(
                "error",
                Some("handshake_session_lock_failed"),
                &[
                    ("peer", peer),
                    ("store_code", code.as_str()),
                    ("stage", "config_dir"),
                ],
            );
            return Err("handshake_session_lock_failed");
        }
    };
    let _lock =
        match crate::fs_store::lock_store_exclusive(&lock_dir, lock_source)
        {
            Ok(v) => v,
            Err(code) => {
                emit_marker(
                    "error",
                    Some("handshake_session_lock_failed"),
                    &[("peer", peer), ("store_code", code.as_str())],
                );
                return Err("handshake_session_lock_failed");
            }
        };
    // ⚠ EXPLICIT ARMS, NO `_ =>` (cold read G-3). A catch-all swallowed a
    // load `Err` into a STORE, so the exclusion was stated
    // unconditionally and delivered conditionally.
    match qsp_session_load(peer) {
        Ok(Some(prior))
            if prior.send.session_id == st.send.session_id =>
        {
            emit_marker(
                "handshake_session_store_skipped",
                None,
                &[("peer", peer), ("reason", "already_committed")],
            );
            let _ = hs_pending_clear(self_label, peer);
        }
        Ok(Some(_)) | Ok(None) => {
        hs_session_store_reported(peer, st)?;
        let _ = hs_pending_clear(self_label, peer);
        }
        Err(code) => {
            // ⚠ FAIL-CLOSED (G-3): a session we cannot READ is not a
            // session we may OVERWRITE.
            emit_marker(
                "error",
                Some("handshake_session_load_failed"),
                &[("peer", peer), ("store_code", code.as_str())],
            );
            return Err("handshake_session_load_failed");
        }
    }
    Ok(())
}

// Both the legacy and lifecycle paths preserve the public outer code and the
// underlying store failure. An unsafe inherited directory remains a refusal.
fn hs_session_store_reported(peer: &str, st: &Suite2SessionState) -> Result<(), &'static str> {
    qsp_session_store(peer, st).map_err(|e| {
        emit_marker(
            "error",
            Some("handshake_session_store_failed"),
            &[("store_code", e.as_str())],
        );
        "handshake_session_store_failed"
    })
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn perform_handshake_poll_with_tokens(
    self_label: Option<&str>,
    peer: &str,
    relay: &str,
    inbox_route_token: &str,
    peer_route_token: &str,
    max: usize,
    suite_mode: HandshakeSuiteMode,
    source: HsPollSource<'_>,
    reply_wrap: Option<HsReplyWrap<'_>>,
) -> Result<PollOutcome, &'static str> {
    enforce_peer_not_blocked(peer)?;
    // NA-0711 (D647 A4 Δ36/Δ37, R238 §5.1): PRE-PULL, and it RETURNS Err.
    //
    // ⚠ Placement is the point. At the lookup below, the frame has already been pulled and leased;
    // a refusal there is a better error message, not a fix. Here it sits beside
    // `enforce_peer_not_blocked` -- this function's own precedent for a fail-closed precondition.
    //
    // ⚠ AND THE STYLE IS THE POINT TOO. The prevailing idiom in this function is
    // `emit_marker(...); continue;` and `return Ok(())` (22 of them against one `return Err`), and
    // NA-0616's ratified refusal is ALREADY defeated on this path by exactly that: the identity
    // gate IS called below, and both call sites discard its error. A gate written in the local
    // idiom would be swallowed identically. This one propagates.
    let self_label = match crate::identity::identity_resolved_self_label(self_label) {
        Ok(v) => v,
        Err(e) => {
            // ⚠ THE REFUSAL NAMES THE KEY IT WOULD HAVE LOOKED UNDER (D647 A4 Δ35, R238 §5.2).
            // A refusal that says only "no" reproduces the very defect this lane exists to fix:
            // three eras of `present=false` with nothing to attribute it to.
            emit_marker(
                "handshake_pending",
                None,
                &[
                    ("peer", peer),
                    ("present", "false"),
                    ("role", "none"),
                    (
                        "key",
                        hs_pending_secret_key(self_label.unwrap_or("<derive>"), peer).as_str(),
                    ),
                    ("state", "unresolved"),
                ],
            );
            return Err(e.as_str());
        }
    };
    let self_label = self_label.as_str();
    // NA-0742 (D-1378): ⚠⚠ **THE POLL ACKS ONLY WHAT IT PULLED ITSELF.** Under
    // `HsPollSource::Provided` the frames belong to the CALLER, which acks them.
    //
    // ⚠ THE GUARD'S HONEST SCOPE, because the two callers differ and implying otherwise would
    // overstate it: for `invite accept` the rule is MECHANICAL — it passes `""` as
    // `inbox_route_token`, so an ack there would have no route token to send to. For
    // `invite finish` it is LAYERING ONLY — a real token IS passed, and nothing mechanical would
    // stop a wrong implementation from acking the caller's frame here.
    //
    // NA-0770 (D-1411): UNCONDITIONAL ON ACK MODE. This read
    // `&& crate::resolve_ack_mode(None) == AckMode::Lease` because under the retired Legacy mode
    // the relay had already deleted the frame at pull time, so there was nothing to ack and
    // nothing left leased. With delete-on-pull gone from the client's intent every relay pull
    // leases, and the ack is always owed.
    // ⚠ THE `HsPollSource::Relay` HALF IS UNCHANGED AND IS THE LOAD-BEARING ONE: under
    // `Provided` the frames belong to the CALLER, which acks them. The honest scope above is
    // likewise unchanged — for `invite finish` this confinement is LAYERING ONLY.
    let acks_own_frames = matches!(source, HsPollSource::Relay);
    // NA-0768: a speculative offer carries its own items exactly as `Provided` does; the
    // flag governs EMISSION at the identity gate only.
    let speculative = matches!(source, HsPollSource::ProvidedSpeculative(_));
    let (envelope, mailbox, invitation) = match &source {
        HsPollSource::Invitation {
            envelope, mailbox, ..
        } => (Some(*envelope), *mailbox, true),
        _ => (None, inbox_route_token, false),
    };
    let items = match source {
        HsPollSource::Invitation { items, .. } => items.to_vec(),
        HsPollSource::ProvidedSpeculative(v) => v.to_vec(),
        HsPollSource::Relay => match transport::relay_inbox_pull(relay, inbox_route_token, max) {
            Ok(v) => v,
            Err(code) => {
                emit_marker("handshake_recv", Some(code), &[("ok", "false")]);
                return Err(code);
            }
        },
    };
    if items.is_empty() {
        emit_marker("handshake_recv", None, &[("msg", "none"), ("ok", "true")]);
        return Ok(PollOutcome::NotConsumed);
    }

    let mut result = PollOutcome::NotConsumed;
    for item in items {
        let receipt = hs_receipt(
            mailbox,
            envelope.unwrap_or(&item.data),
            &item.data,
            peer_route_token,
        )?;
        let outcome = hs_poll_item(
            self_label,
            peer,
            relay,
            inbox_route_token,
            peer_route_token,
            suite_mode,
            &item,
            speculative,
            invitation,
            receipt,
            reply_wrap.as_ref(),
        )?;
        if matches!(
            outcome,
            PollOutcome::Consumed | PollOutcome::AlreadyComplete
        ) {
            if acks_own_frames {
                hs_emit_producer_ack(relay, inbox_route_token, &item.id);
            }
            result = outcome;
        }
    }
    Ok(result)
}

#[allow(clippy::too_many_arguments)]
fn hs_poll_item(
    self_label: &str,
    peer: &str,
    relay: &str,
    inbox_route_token: &str,
    peer_route_token: &str,
    suite_mode: HandshakeSuiteMode,
    item: &crate::InboxPullItem,
    speculative: bool,
    invitation: bool,
    receipt: HsReceipt,
    reply_wrap: Option<&HsReplyWrap<'_>>,
) -> Result<PollOutcome, &'static str> {
    // Only the outer owner ACKs. Each frame is dispatched against its own SID.
    let acks_own_frames = false;
    let items = vec![item.clone()];
    let plan = hs_lifecycle_plan(
        self_label,
        peer,
        &item.data,
        receipt,
        invitation,
        suite_mode,
        speculative,
    )?;
    if let Some(outcome) = plan.immediate {
        if outcome == PollOutcome::NotConsumed {
            return Ok(outcome);
        }
        return hs_lifecycle_retry(
            plan.lifecycle
                .as_ref()
                .ok_or("handshake_lifecycle_conflict")?,
            plan.retry.as_ref(),
        );
    }
    let receipt = &plan.receipt;
    let lifecycle = plan.lifecycle.as_ref();
    let pending = plan.pending;
    let pending_state = plan.pending_state;
    let pending_key = hs_pending_secret_key(self_label, peer);
    if let Some(pending) = pending {
        emit_marker(
            "handshake_pending",
            None,
            &[
                ("peer", peer),
                ("present", "true"),
                ("role", pending.role.as_str()),
                ("key", pending_key.as_str()),
                ("state", pending_state.as_str()),
            ],
        );
        if pending.role == "initiator" {
            let pending_suite_context = match hs_pending_suite_context(&pending) {
                Ok(v) => v,
                Err(_) => {
                    // NA-0775 (`D-1418`) item 1.2, on cold-read F-3 and `RULING_006` sec 1:
                    // ⚠⚠ THE PENDING IS NO LONGER CLEARED HERE. Clearing it destroyed durable
                    // state on a path that pushes nothing and now acks nothing, which left the
                    // frame permanently un-consumable: cleared pending -> no-pending branch ->
                    // decode reject -> tail -> NotConsumed, forever. Measured: a surviving
                    // corrupt record does NOT loop -- it re-rejects once per pass, bounded by
                    // the relay TTL like any persistent failure.
                    // ⚠ THE RECOVERY IS ASYMMETRIC AND IS FILED, NOT REPAIRED: `:1537`
                    // overwrites a stale pending unconditionally, but the responder's own
                    // store sits behind the no-pending branch, so `invite accept` cannot
                    // overwrite a corrupt RESPONDER pending -- the user must initiate.
                    hs_reject_key_context();
                    return Ok(PollOutcome::NotConsumed);
                }
            };
            for item in items {
                match hs_decode_resp_pending(&item.data, suite_mode) {
                    Ok(resp) => {
                        if resp.session_id != pending.session_id {
                            emit_marker(
                                "handshake_reject",
                                None,
                                &[("reason", "session_id_mismatch")],
                            );
                            continue;
                        }
                        if suite_mode == HandshakeSuiteMode::SuiteRequired
                            && !pending_suite_context.is_explicit()
                        {
                            hs_reject_key_context();
                            continue;
                        }
                        if !hs_contexts_match(&pending_suite_context, &resp.suite_context) {
                            hs_reject_context_mismatch();
                            continue;
                        }
                        let active_suite_context = pending_suite_context.clone();
                        let c = StdCrypto;
                        let ss_pq = match c.decap(&pending.kem_sk, &resp.kem_ct) {
                            Ok(v) => v,
                            Err(_) => {
                                emit_marker(
                                    "handshake_reject",
                                    None,
                                    &[("reason", "pq_decap_failed")],
                                );
                                return Ok(PollOutcome::NotConsumed);
                            }
                        };
                        // NA-0633 (ENG-0038, C1): mix the responder-identity KEM secret the initiator
                        // encapsulated at init time; the transcript MAC below then only verifies if the
                        // responder decapsulated the SAME secret (i.e. holds the pinned identity key).
                        let pq_init_ss = hs_pq_init_ss(
                            &ss_pq,
                            &resp.session_id,
                            &pending.resp_kem_ss,
                            &active_suite_context,
                        );
                        if hs_dh_pub_is_all_zero(&resp.dh_pub) {
                            emit_marker("handshake_reject", None, &[("reason", "dh_pub_invalid")]);
                            return Ok(PollOutcome::NotConsumed);
                        }
                        let dh_self_pub = match hs_dh_pub_from_bytes(&pending.dh_pub) {
                            Ok(v) => v,
                            Err(_) => {
                                emit_marker("handshake_reject", None, &[("reason", "dh_missing")]);
                                return Ok(PollOutcome::NotConsumed);
                            }
                        };
                        let dh_shared = match hs_dh_shared(&pending.dh_sk, &resp.dh_pub) {
                            Ok(v) => v,
                            Err(e) => {
                                let reason = if e == HS_DH_NONCONTRIBUTORY {
                                    HS_DH_NONCONTRIBUTORY
                                } else {
                                    "dh_failed"
                                };
                                emit_marker("handshake_reject", None, &[("reason", reason)]);
                                return Ok(PollOutcome::NotConsumed);
                            }
                        };
                        let dh_init_arr = hs_dh_init_from_shared(
                            &dh_shared,
                            &resp.session_id,
                            &active_suite_context,
                        );
                        let dh_peer_pub = resp.dh_pub;
                        let a1 = hs_encode_init(&HsInit {
                            suite_context: pending_suite_context.clone(),
                            session_id: pending.session_id,
                            kem_pk: pending.kem_pk.clone(),
                            sig_pk: pending.sig_pk.clone(),
                            dh_pub: dh_self_pub,
                            resp_kem_ct: pending.resp_kem_ct.clone(),
                        });
                        let b1_no_auth = hs_encode_resp_no_auth(
                            &resp.session_id,
                            &resp.kem_ct,
                            &resp.sig_pk,
                            &resp.dh_pub,
                            &active_suite_context,
                        );
                        let mac = hs_transcript_mac(&pq_init_ss, &a1, &b1_no_auth);
                        if !hs_ct_eq_32(&mac, &resp.mac) {
                            if resp.suite_context.is_explicit() {
                                hs_emit_suite_reject("REJECT_QSC_HS_TRANSCRIPT_CONTEXT");
                            } else {
                                emit_marker(
                                    "handshake_reject",
                                    None,
                                    &[("reason", "bad_transcript")],
                                );
                            }
                            return Ok(PollOutcome::NotConsumed);
                        }
                        let th = hs_transcript_hash(&pq_init_ss, &a1, &b1_no_auth);
                        let sig_msg = hs_sig_msg_b1(&resp.session_id, &th);
                        if hs_sig_verify(&resp.sig_pk, &sig_msg, &resp.sig, "b1_verify").is_err() {
                            emit_marker("handshake_reject", None, &[("reason", "sig_invalid")]);
                            return Ok(PollOutcome::NotConsumed);
                        }
                        let sig_fp = identity_fingerprint_single(FpRole::Sig, &resp.sig_pk);
                        let Some(peer_fp) = pending.peer_fp.as_deref() else {
                            emit_marker(
                                "identity_unknown",
                                None,
                                &[("peer", peer), ("seen_fp", "unknown")],
                            );
                            emit_marker(
                                "handshake_reject",
                                None,
                                &[("reason", "identity_unknown")],
                            );
                            return Ok(PollOutcome::NotConsumed);
                        };
                        if hs_require_primary_identity_pin(peer, peer_fp, identity_read_pin, false)
                            .is_err()
                        {
                            return Ok(PollOutcome::NotConsumed);
                        }
                        // NA-0634 (D571 Decision 2a): the responder's signing identity is now REQUIRED-
                        // pinned against the populated sig_fp (was an inert OPTIONAL check — ENG-0038):
                        // fingerprint(resp.sig_pk) MUST equal the contact's sig_fp, else fail closed.
                        if hs_require_sig_identity_pin(peer, sig_fp.as_str(), identity_read_sig_pin)
                            .is_err()
                        {
                            return Ok(PollOutcome::NotConsumed);
                        }
                        // NA-0620 (Stage 1a): the initiator's local X25519 ephemeral private key
                        // (retained in the pending handshake) feeds the session DH-ratchet state.
                        let dh_self_priv: [u8; 32] = match pending.dh_sk.as_slice().try_into() {
                            Ok(v) => v,
                            Err(_) => {
                                emit_marker("handshake_reject", None, &[("reason", "dh_sk_len")]);
                                return Ok(PollOutcome::NotConsumed);
                            }
                        };
                        let st = match hs_build_session(
                            true,
                            true,
                            pending.session_id,
                            dh_init_arr,
                            pq_init_ss,
                            dh_self_pub,
                            dh_peer_pub,
                            dh_self_priv,
                        ) {
                            Ok(v) => v,
                            Err(_) => {
                                emit_marker(
                                    "handshake_reject",
                                    None,
                                    &[("reason", "session_init_failed")],
                                );
                                return Ok(PollOutcome::NotConsumed);
                            }
                        };
                        if active_suite_context.is_explicit() {
                            hs_emit_suite_accept(&active_suite_context, false);
                        } else {
                            hs_emit_suite_accept(&active_suite_context, true);
                        }
                        let k_confirm = hs_confirm_key(
                            &pq_init_ss,
                            &resp.session_id,
                            &th,
                            &active_suite_context,
                        );
                        let cmac = hs_confirm_mac(
                            &k_confirm,
                            &resp.session_id,
                            &th,
                            &active_suite_context,
                        );
                        let sig_sk = identity_self_kem_keypair(self_label)
                            .map_err(|e| e.as_str())?
                            .sig_sk;
                        let a2_sig_msg = hs_sig_msg_a2(&resp.session_id, &th, &cmac);
                        #[cfg(qsc_rng_failure_test_seam)]
                        let a2_sig_result = if hs_rng_failure_forced("QSC.SIG.A2") {
                            Err(())
                        } else {
                            c.sign(&sig_sk, &a2_sig_msg).map_err(|_| ())
                        };
                        #[cfg(not(qsc_rng_failure_test_seam))]
                        let a2_sig_result = c.sign(&sig_sk, &a2_sig_msg).map_err(|_| ());
                        let a2_sig = match a2_sig_result {
                            Ok(v) => v,
                            Err(_) => {
                                emit_marker(
                                    "handshake_reject",
                                    None,
                                    &[("reason", "sig_sign_failed")],
                                );
                                return Ok(PollOutcome::NotConsumed);
                            }
                        };
                        emit_marker(
                            "sig_status",
                            None,
                            &[("ok", "true"), ("alg", "ML-DSA-65"), ("reason", "a2_sign")],
                        );
                        let confirm = HsConfirm {
                            suite_context: active_suite_context,
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
                        if let Some(c) = lifecycle {
                            let reply = HsDelivery {
                                relay: relay.into(),
                                route: receipt.route.clone(),
                                bytes: cbytes,
                                ticket: None,
                            };
                            let reply =
                                hs_lifecycle_select(c, &pending, &st, receipt, Some(reply))?
                                    .ok_or("handshake_lifecycle_conflict")?;
                            hs_delivery_send(&reply)?;
                            hs_lifecycle_after_send(c, Some(&reply))?;
                            emit_marker(
                                "handshake_complete",
                                None,
                                &[
                                    ("peer", peer),
                                    ("role", "initiator"),
                                    ("peer_confirmed", "no"),
                                ],
                            );
                            return Ok(PollOutcome::Consumed);
                        }
                        transport::relay_inbox_push(relay, peer_route_token, &cbytes)?;
                        // NA-0775 (`D-1418`) A1 -- the late-landing guard. EXTRACTED to a named
                        // function so its property can be exercised DIRECTLY: the CLI cannot reach
                        // this state (after a successful pass the pending is cleared, so the branch
                        // cannot be re-entered with a stale epoch-0 `st`), and a guard whose absence
                        // was never shown to break the property is not evidence. Unit tests at
                        // `na0775_late_store_guard_tests`, foot of this file.
                        hs_commit_session_guarded(self_label, peer, &st)?;
                        // NA-0742: ⚠⚠ **AFTER THE PUSH, NOT AFTER THE COMMIT.** The session was
                        // stored above, but this frame's LAST effect is the A2 that just left. A
                        // frame whose reply never reached the peer is not consumed — which is
                        // exactly why the a2_sig-failure exit further up MUST NOT ack.
                        if acks_own_frames {
                            debug_assert!(
                                matches!(qsp_session_load(peer), Ok(Some(_))),
                                "NA-0742 guard 1 (poll/initiator): the session this frame created \
                                 must be durably loadable before the frame may be acked"
                            );
                            hs_emit_producer_ack(relay, inbox_route_token, &item.id);
                        }
                        emit_marker(
                            "handshake_complete",
                            None,
                            &[
                                ("peer", peer),
                                ("role", "initiator"),
                                ("peer_confirmed", "no"),
                            ],
                        );
                        return Ok(PollOutcome::Consumed);
                    }
                    Err(reason) => {
                        hs_emit_decode_reject(reason);
                        continue;
                    }
                }
            }
            return Ok(PollOutcome::NotConsumed);
        }
        if pending.role == "responder" {
            let pending_suite_context = match hs_pending_suite_context(&pending) {
                Ok(v) => v,
                Err(_) => {
                    // NA-0775 (`D-1418`) item 1.2, on cold-read F-3 and `RULING_006` sec 1:
                    // ⚠⚠ THE PENDING IS NO LONGER CLEARED HERE. Clearing it destroyed durable
                    // state on a path that pushes nothing and now acks nothing, which left the
                    // frame permanently un-consumable: cleared pending -> no-pending branch ->
                    // decode reject -> tail -> NotConsumed, forever. Measured: a surviving
                    // corrupt record does NOT loop -- it re-rejects once per pass, bounded by
                    // the relay TTL like any persistent failure.
                    // ⚠ THE RECOVERY IS ASYMMETRIC AND IS FILED, NOT REPAIRED: `:1537`
                    // overwrites a stale pending unconditionally, but the responder's own
                    // store sits behind the no-pending branch, so `invite accept` cannot
                    // overwrite a corrupt RESPONDER pending -- the user must initiate.
                    hs_reject_key_context();
                    return Ok(PollOutcome::NotConsumed);
                }
            };
            for item in items {
                match hs_decode_confirm_pending(&item.data, suite_mode) {
                    Ok(confirm) => {
                        if confirm.session_id != pending.session_id {
                            emit_marker(
                                "handshake_reject",
                                None,
                                &[("reason", "session_id_mismatch")],
                            );
                            continue;
                        }
                        if suite_mode == HandshakeSuiteMode::SuiteRequired
                            && !pending_suite_context.is_explicit()
                        {
                            hs_reject_key_context();
                            continue;
                        }
                        if !hs_contexts_match(&pending_suite_context, &confirm.suite_context) {
                            hs_reject_context_mismatch();
                            continue;
                        }
                        let active_suite_context = pending_suite_context.clone();
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
                        let expect = hs_confirm_mac(
                            &k_confirm,
                            &confirm.session_id,
                            &th,
                            &active_suite_context,
                        );
                        if !hs_ct_eq_32(&expect, &confirm.mac) {
                            emit_marker("handshake_recv", None, &[("msg", "A2"), ("ok", "false")]);
                            if active_suite_context.is_explicit() {
                                hs_emit_suite_reject("REJECT_QSC_HS_TRANSCRIPT_CONTEXT");
                            } else {
                                emit_marker("handshake_reject", None, &[("reason", "bad_confirm")]);
                            }
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
                        let Some(peer_sig_fp) = pending.peer_sig_fp.as_ref() else {
                            emit_marker(
                                "handshake_reject",
                                None,
                                &[("reason", "identity_missing")],
                            );
                            continue;
                        };
                        if hs_require_primary_identity_pin(
                            peer,
                            peer_fp.as_str(),
                            identity_read_pin,
                            // NOT speculative: this frame DECODED against this peer's own
                            // pending record, so a mismatch here is a real security event.
                            false,
                        )
                        .is_err()
                        {
                            continue;
                        }
                        if hs_check_optional_identity_pin(
                            peer,
                            peer_sig_fp.as_str(),
                            identity_read_sig_pin,
                        )
                        .is_err()
                        {
                            continue;
                        }
                        if let Some(c) = lifecycle {
                            hs_lifecycle_select(c, &pending, &st, receipt, None)?;
                            emit_marker(
                                "handshake_complete",
                                None,
                                &[
                                    ("peer", peer),
                                    ("role", "responder"),
                                    ("peer_confirmed", "yes"),
                                ],
                            );
                            return Ok(PollOutcome::Consumed);
                        }
                        qsp_session_store(peer, &st).map_err(|e| {
                            // NA-0757 (ENG-0239, R388 A1(b)): the typed code SURVIVES as a
                            // field. Seven distinct `ErrorCode`s reach this point and the
                            // flattening named none of them, which is why a field capture of
                            // the marker could not localize it. The outer string is unchanged ON
                            // PURPOSE: no new discriminant enters the facade's wire
                            // vocabulary, so an opaque error never becomes a WRONG one.
                            emit_marker(
                                "error",
                                Some("handshake_session_store_failed"),
                                &[("store_code", e.as_str())],
                            );
                            "handshake_session_store_failed"
                        })?;
                        let _ = hs_pending_clear(self_label, peer);
                        if active_suite_context.is_explicit() {
                            hs_emit_suite_accept(&active_suite_context, false);
                        } else {
                            hs_emit_suite_accept(&active_suite_context, true);
                        }
                        // NA-0742: the responder branch's last effect IS the durable commit —
                        // `qsp_session_store` then `hs_pending_clear` above, with NO push
                        // following. Verified from the bytes rather than assumed symmetric with
                        // the initiator, because in that branch a push DOES follow.
                        if acks_own_frames {
                            debug_assert!(
                                matches!(qsp_session_load(peer), Ok(Some(_))),
                                "NA-0742 guard 1 (poll/responder): the session this frame \
                                 completed must be durably loadable before the frame may be acked"
                            );
                            hs_emit_producer_ack(relay, inbox_route_token, &item.id);
                        }
                        emit_marker(
                            "handshake_complete",
                            None,
                            &[
                                ("peer", peer),
                                ("role", "responder"),
                                ("peer_confirmed", "yes"),
                            ],
                        );
                        return Ok(PollOutcome::Consumed);
                    }
                    Err(reason) => {
                        if pending_suite_context.is_explicit() {
                            #[cfg(qsc_binding_fuzz_helper)]
                            {
                                if crate::adversarial::binding_fuzz::replay_candidate_matches_pending_init(
                                    &item.data,
                                    &pending.session_id,
                                    pending_suite_context.explicit_block(),
                                    hs_fuzz_suite_mode(suite_mode),
                                ) {
                                    hs_reject_replay();
                                    continue;
                                }
                            }
                            #[cfg(not(qsc_binding_fuzz_helper))]
                            if let Ok(init) = hs_decode_init(&item.data, suite_mode) {
                                if init.session_id == pending.session_id
                                    && hs_contexts_match(
                                        &pending_suite_context,
                                        &init.suite_context,
                                    )
                                {
                                    hs_reject_replay();
                                    continue;
                                }
                            }
                        }
                        hs_emit_decode_reject(reason);
                        continue;
                    }
                }
            }
            return Ok(PollOutcome::NotConsumed);
        }
    }

    // NA-0711 (D647 A4 Δ35, R238 §5.2): NAME THE WHOLE KEY AND WHICH OF THE THREE STATES.
    // ⚠ `present=false` alone is what cost this program three investigations across three eras.
    emit_marker(
        "handshake_pending",
        None,
        &[
            ("peer", peer),
            ("present", "false"),
            ("role", "none"),
            ("key", pending_key.as_str()),
            ("state", pending_state.as_str()),
        ],
    );

    // NA-0775 (`D-1418`) `RULING_007` R2/`E-5`: did any frame in this batch turn out to be work
    // that is ALREADY DURABLY DONE? Tracked as a flag rather than an early `return` so a batch
    // keeps being processed -- under `HsPollSource::Relay` there can be many items, and returning
    // on the first would silently stop handling the rest.
    let mut already_complete = false;
    for item in items {
        if let Ok(confirm) = hs_decode_confirm(&item.data, suite_mode) {
            if confirm.suite_context.is_explicit() && matches!(qsp_session_load(peer), Ok(Some(_)))
            {
                // ⚠ `RULING_007` R2: this is `AlreadyComplete` on its own merits -- a redelivered
                // CONFIRM for a peer whose session is stored is work that already finished. It is
                // reached only when the frame's `session_id` did NOT match above, i.e. an
                // explicit-suite CONFIRM for some OTHER session of the same peer.
                hs_reject_replay();
                already_complete = true;
                continue;
            }
        }
        match hs_decode_init(&item.data, suite_mode) {
            Ok(init) => {
                if hs_dh_pub_is_all_zero(&init.dh_pub) {
                    emit_marker("handshake_reject", None, &[("reason", "dh_pub_invalid")]);
                    continue;
                }
                // NA-0634 (D571 Decision 2a): the responder pins the initiator's FULL identity (KEM +
                // signing) against the single combined verification code — binding init.sig_pk too.
                let peer_fp = identity_fingerprint_from_identity(&init.kem_pk, &init.sig_pk);
                let peer_sig_fp = identity_fingerprint_single(FpRole::Sig, &init.sig_pk);
                if hs_require_primary_identity_pin(
                    peer,
                    peer_fp.as_str(),
                    identity_read_pin,
                    speculative,
                )
                .is_err()
                {
                    continue;
                }
                if hs_check_optional_identity_pin(peer, peer_sig_fp.as_str(), identity_read_sig_pin)
                    .is_err()
                {
                    continue;
                }
                let c = StdCrypto;
                #[cfg(qsc_rng_failure_test_seam)]
                if hs_rng_failure_forced("QSC.KEM.ENCAP") {
                    emit_marker("handshake_reject", None, &[("reason", "pq_encap_failed")]);
                    continue;
                }
                let (kem_ct, ss_pq) = match c.encap(&init.kem_pk) {
                    Ok(v) => v,
                    Err(_) => {
                        emit_marker("handshake_reject", None, &[("reason", "pq_encap_failed")]);
                        continue;
                    }
                };
                // NA-0633 (ENG-0038, C1): decapsulate the initiator's encapsulation to OUR identity KEM
                // key using our identity KEM secret. Only the holder of that secret derives the same
                // `resp_kem_ss`; mixing it into pq_init_ss makes our transcript MAC verifiable ONLY by an
                // initiator who encapsulated to our REAL identity key — so a wrong responder (which cannot
                // decapsulate) produces a MAC the initiator rejects (the ENG-0038 fix, receiver half).
                let resp_kem_ss = match identity_self_kem_keypair(self_label) {
                    Ok(k) => match c.decap(&k.kem_sk, &init.resp_kem_ct) {
                        Ok(ss) => ss,
                        Err(_) => {
                            emit_marker(
                                "handshake_reject",
                                None,
                                &[("reason", "resp_kem_decap_failed")],
                            );
                            continue;
                        }
                    },
                    Err(_) => {
                        emit_marker("handshake_reject", None, &[("reason", "identity_missing")]);
                        continue;
                    }
                };
                let pq_init_ss =
                    hs_pq_init_ss(&ss_pq, &init.session_id, &resp_kem_ss, &init.suite_context);
                let (dh_sk, dh_self_pub) = hs_ephemeral_keypair();
                let dh_shared = match hs_dh_shared(&dh_sk, &init.dh_pub) {
                    Ok(v) => v,
                    Err(e) => {
                        let reason = if e == HS_DH_NONCONTRIBUTORY {
                            HS_DH_NONCONTRIBUTORY
                        } else {
                            "dh_failed"
                        };
                        emit_marker("handshake_reject", None, &[("reason", reason)]);
                        continue;
                    }
                };
                let dh_init_arr =
                    hs_dh_init_from_shared(&dh_shared, &init.session_id, &init.suite_context);
                let dh_peer_pub = init.dh_pub;
                let st = match hs_build_session(
                    true,
                    false,
                    init.session_id,
                    dh_init_arr,
                    pq_init_ss,
                    dh_self_pub,
                    dh_peer_pub,
                    dh_sk,
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
                let b1_no_auth = hs_encode_resp_no_auth(
                    &init.session_id,
                    &kem_ct,
                    &self_sig_pk,
                    &dh_self_pub,
                    &init.suite_context,
                );
                let mac = hs_transcript_mac(&pq_init_ss, &a1, &b1_no_auth);
                let th = hs_transcript_hash(&pq_init_ss, &a1, &b1_no_auth);
                let sig_msg = hs_sig_msg_b1(&init.session_id, &th);
                #[cfg(qsc_rng_failure_test_seam)]
                let sig_result = if hs_rng_failure_forced("QSC.SIG.B1") {
                    Err(())
                } else {
                    c.sign(&self_sig_sk, &sig_msg).map_err(|_| ())
                };
                #[cfg(not(qsc_rng_failure_test_seam))]
                let sig_result = c.sign(&self_sig_sk, &sig_msg).map_err(|_| ());
                let sig = match sig_result {
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
                let k_confirm =
                    hs_confirm_key(&pq_init_ss, &init.session_id, &th, &init.suite_context);
                let pending = HandshakePending {
                    self_label: self_label.to_string(),
                    peer: peer.to_string(),
                    session_id: init.session_id,
                    kem_sk: Vec::new(),
                    kem_pk: Vec::new(),
                    dh_sk: dh_sk.to_vec(),
                    dh_pub: dh_self_pub.to_vec(),
                    sig_pk: Vec::new(),
                    resp_kem_ct: Vec::new(),
                    resp_kem_ss: Vec::new(),
                    peer_fp: Some(peer_fp),
                    peer_sig_fp: Some(peer_sig_fp),
                    peer_sig_pk: Some(init.sig_pk.clone()),
                    role: "responder".to_string(),
                    confirm_key: Some(k_confirm),
                    transcript_hash: Some(th),
                    pending_session: Some(st.snapshot_bytes()),
                    suite_context: init.suite_context.as_pending_block(),
                };
                let resp = HsResp {
                    suite_context: init.suite_context.clone(),
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
                let hs_version_s = init.suite_context.wire_version().to_string();
                emit_marker(
                    "handshake_send",
                    None,
                    &[
                        ("msg", "B1"),
                        ("size", size_s.as_str()),
                        ("kem_ct_len", ct_len_s.as_str()),
                        ("sig_pk_len", sig_pk_len_s.as_str()),
                        ("hs_version", hs_version_s.as_str()),
                        ("suite_context", init.suite_context.mode_label()),
                    ],
                );
                if let Some(expected) = lifecycle {
                    let wire = match reply_wrap {
                        Some(w) => crate::invite::encode_envelope_resp(w.self_route_token, &bytes)?,
                        None => bytes.clone(),
                    };
                    let reply = HsDelivery {
                        relay: relay.into(),
                        route: receipt.route.clone(),
                        bytes: wire,
                        ticket: None,
                    };
                    let _lock = hs_lifecycle_lock()?;
                    let mut c =
                        hs_lifecycle_find(self_label, peer)?.unwrap_or_else(|| expected.clone());
                    if c.generation != expected.generation
                        || c.selected.is_some()
                        || c.responder.is_some()
                    {
                        return Err("handshake_lifecycle_conflict");
                    }
                    c.check_binding()?;
                    c.check_session()?;
                    c.check_pending()?;
                    c.responder = Some(HsCandidate {
                        pending,
                        receipt: receipt.clone(),
                        reply: reply.clone(),
                    });
                    hs_lifecycle_put(&c)?;
                    hs_lifecycle_recover(&mut c)?;
                    drop(_lock);
                    hs_delivery_send(&reply)?;
                    hs_lifecycle_after_send(&c, Some(&reply))?;
                    return Ok(PollOutcome::Consumed);
                }
                hs_pending_store(&pending).map_err(|_| "handshake_pending_store_failed")?;
                // NA-0681: the B1 FRAME is unchanged; only its framing on the wire differs.
                match &reply_wrap {
                    None => transport::relay_inbox_push(relay, peer_route_token, &bytes)?,
                    Some(w) => {
                        let wrapped =
                            crate::invite::encode_envelope_resp(w.self_route_token, &bytes)?;
                        transport::relay_inbox_push(relay, peer_route_token, &wrapped)?
                    }
                }
                // NA-0742: ⚠ **AFTER THE B1 PUSH.** The durable commit on this branch is the
                // `hs_pending_store` above; the frame's last effect is the B1 that just left.
                if acks_own_frames {
                    debug_assert!(
                        matches!(hs_pending_load_state(self_label, peer), Ok((Some(_), _))),
                        "NA-0742 guard 1 (poll/no-pending): the pending record this frame created \
                         must be durably loadable before the frame may be acked"
                    );
                    hs_emit_producer_ack(relay, inbox_route_token, &item.id);
                }
                return Ok(PollOutcome::Consumed);
            }
            Err(reason) => {
                // ⚠⚠ THE DECODE REJECT FIRES FIRST, AND THAT ORDER IS THE WHOLE OF CANDIDATE B
                // (`RULING_009` sec 1). A frame reaching here has, by construction, failed the
                // confirm/replay arm above AND `hs_decode_init` -- so "I could not read this" is
                // TRUE, and it is said first. Only then do we ask the SEPARATE question below:
                // is this frame's work already durably done? Both statements are true, in the
                // order they became true, and the marker stream now carries both.
                hs_emit_decode_reject(reason);

                // NA-0775 (`D-1418`): A REDELIVERED FRAME WHOSE HANDSHAKE ALREADY COMPLETED.
                //
                // ⚠⚠ WHY IT LIVES HERE AND NOT HIGHER UP -- `E-6`, and it cost a ruling to learn.
                // Placed above the decode, this test SWALLOWED the reject: it `continue`d before
                // `hs_emit_decode_reject` ever ran, and `handshake_mvp.rs:1145` caught the
                // vanished marker. ⚠ THE ARM IT SHADOWED WAS NOT THE REPLAY ARM: that one's
                // second conjunct is `confirm.suite_context.is_explicit()`, which is FALSE for
                // `LegacyV1`, and `hs_suite_context_for_mode(LegacyCompat)` returns `LegacyV1`
                // while the CLI defaults to `LegacyCompat` -- so the replay guard cannot fire on
                // this path AT ALL. `handshake/mod.rs:206-215` recorded that before this lane
                // existed; the reject a replayed A2 actually produces is this `handshake_type`
                // one. That unreachability is PRE-EXISTING and belongs to `NA-0708`'s
                // reject-vocabulary lane, not to this one.
                //
                // ⚠ IT COMPARES `session_id`, NOT MERE SESSION PRESENCE. Testing only that SOME
                // session exists for the peer would call a foreign frame "already done". This
                // asks whether THIS frame's handshake is the one that completed -- the same
                // predicate the late-landing guard uses, for the same reason.
                //
                // ⚠ THE ACK CONSEQUENCE IS ASYMMETRIC BY CALLER, AND THAT IS NOT AN OVERSIGHT.
                // Under `HsPollSource::Relay` the in-poll acks sit at the CONSUMED exits only and
                // `handshake_poll_with_tokens` discards the outcome, so a poll emits this marker
                // and acks NOTHING -- `t5p` pins that orphan and stays green. Under `Provided`
                // the caller acks on `Consumed | AlreadyComplete`, which is what retires `t5f`'s
                // frame after a lost ack.
                if let Some(sid) = hs_frame_session_id(&item.data, suite_mode) {
                    if matches!(
                        qsp_session_load(peer),
                        Ok(Some(prior)) if prior.send.session_id == sid
                    ) {
                        emit_marker(
                            "handshake_already_complete",
                            None,
                            &[("peer", peer), ("reason", "session_already_stored")],
                        );
                        already_complete = true;
                    }
                }
                continue;
            }
        }
    }
    // NA-0775 (`D-1418`) `RULING_007`: the tail stays `NotConsumed` UNLESS a frame in this batch
    // was measured already-done. A batch that decoded as nothing is NOT "already done" -- moving
    // the tail unconditionally would ack foreign litter, which is the defect this lane exists to
    // stop.
    if already_complete {
        Ok(PollOutcome::AlreadyComplete)
    } else {
        Ok(PollOutcome::NotConsumed)
    }
}

fn handshake_poll_with_tokens(
    self_label: Option<&str>,
    peer: &str,
    relay: &str,
    inbox_route_token: &str,
    peer_route_token: &str,
    max: usize,
    suite_mode: HandshakeSuiteMode,
) -> CliResult {
    require_unlocked("handshake_poll")?;
    // NA-0775 (`D-1418`): the poll now returns a `PollOutcome`. This caller pulls from the
    // relay itself (`HsPollSource::Relay`), so the poll acks its own frames internally and this
    // wrapper has nothing to decide -- the value is deliberately discarded here and NOWHERE else.
    if let Err(code) = perform_handshake_poll_with_tokens(
        self_label,
        peer,
        relay,
        inbox_route_token,
        peer_route_token,
        max,
        suite_mode,
        // The shipped path: pull from the relay, reply bare. Byte-identical to before.
        HsPollSource::Relay,
        None,
    ) {
        return Err(CliError::code(code));
    }
    Ok(())
}

pub fn handshake_poll_with_suite_mode(
    self_label: Option<&str>,
    peer: &str,
    relay: &str,
    max: usize,
    suite_mode: HandshakeSuiteMode,
) -> CliResult {
    let peer_channel = resolve_peer_device_target(peer, false)
        .map(|v| v.channel)
        .unwrap_or_else(|_| peer.to_string());
    let inbox_route_token =
        relay_self_inbox_route_token().map_err(|code| CliError::code(code))?;
    let peer_route_token =
        relay_peer_route_token(peer).map_err(|code| CliError::code(code))?;
    handshake_poll_with_tokens(
        self_label,
        peer_channel.as_str(),
        relay,
        inbox_route_token.as_str(),
        peer_route_token.as_str(),
        max,
        suite_mode,
    )?;
    Ok(())
}

// D581 KEEP -> NA-0646 (D582): part of the library's pub GUI surface, seeded for the GUI
// phase; dormant until the GUI consumes it (dead_code allowance retained meanwhile).
#[allow(dead_code)]
pub fn handshake_poll(
    self_label: Option<&str>,
    peer: &str,
    relay: &str,
    max: usize,
) -> CliResult {
    handshake_poll_with_suite_mode(
        self_label,
        peer,
        relay,
        max,
        HandshakeSuiteMode::LegacyCompat,
    )?;
    Ok(())
}

#[cfg(test)]
mod ct_eq_tests {
    use super::hs_ct_eq_32;

    // hs_ct_eq_32 must be bit-for-bit equivalent to `==` for all 32-byte inputs;
    // only the timing (no early-out) differs. These vectors assert that equivalence
    // so the ENG-0003 hardening cannot change accept/reject semantics.

    #[test]
    fn equal_arrays_are_equal() {
        let a = [0x5au8; 32];
        let b = [0x5au8; 32];
        assert!(hs_ct_eq_32(&a, &b));
        assert_eq!(hs_ct_eq_32(&a, &b), a == b);
    }

    #[test]
    fn single_byte_flip_is_unequal_at_every_position() {
        let base = {
            let mut v = [0u8; 32];
            for (i, b) in v.iter_mut().enumerate() {
                *b = i as u8;
            }
            v
        };
        for pos in 0..32 {
            let mut other = base;
            other[pos] ^= 0x01;
            assert!(!hs_ct_eq_32(&base, &other), "flip at {pos} must be unequal");
            assert_eq!(hs_ct_eq_32(&base, &other), base == other);
        }
    }

    #[test]
    fn high_bit_flip_and_all_different_are_unequal() {
        let base = [0x00u8; 32];
        let mut hi = base;
        hi[31] ^= 0x80;
        assert!(!hs_ct_eq_32(&base, &hi));
        let allff = [0xffu8; 32];
        assert!(!hs_ct_eq_32(&base, &allff));
        assert_eq!(hs_ct_eq_32(&base, &allff), base == allff);
    }

    #[test]
    fn matches_operator_over_mixed_vectors() {
        let vectors: [([u8; 32], [u8; 32]); 4] = [
            ([0u8; 32], [0u8; 32]),
            ([1u8; 32], [1u8; 32]),
            ([0u8; 32], [1u8; 32]),
            (
                {
                    let mut v = [7u8; 32];
                    v[0] = 9;
                    v
                },
                [7u8; 32],
            ),
        ];
        for (a, b) in vectors.iter() {
            assert_eq!(hs_ct_eq_32(a, b), a == b);
        }
    }
}

#[cfg(test)]
mod na0628_contributory_dh_tests {
    use super::{hs_dh_pub_is_all_zero, hs_dh_shared, HS_DH_NONCONTRIBUTORY};

    // NA-0628 (ENG-0034). RFC 7748 §6.1: X25519 accepts small-order points and returns the all-zero
    // shared secret rather than erroring. `hs_dh_pub_is_all_zero` screens the all-zero ENCODING —
    // exactly one of Curve25519's eight low-order points. The other seven reach the DH and produce a
    // degenerate shared secret, so the establishment DH must check the OUTPUT.

    /// The eight classical low-order encodings (RFC 7748 §6.1; the standard list). Every one of them
    /// drives `X25519(clamped_sk, .) -> 0^32`, independent of the scalar.
    const LOW_ORDER: [&str; 8] = [
        "0000000000000000000000000000000000000000000000000000000000000000",
        "0100000000000000000000000000000000000000000000000000000000000000",
        "e0eb7a7c3b41b8ae1656e3faf19fc46ada098deb9c32b1fd866205165f49b800",
        "5f9c95bca3508c24b1d0b1559c83ef5b04445cc4581c8e86d8224eddd09f1157",
        "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
        "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
        "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
        "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
    ];

    fn unhex(s: &str) -> [u8; 32] {
        let mut out = [0u8; 32];
        for (i, byte) in out.iter_mut().enumerate() {
            *byte = u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).unwrap();
        }
        out
    }

    #[test]
    fn establishment_dh_rejects_every_low_order_peer_key() {
        // Two structurally different scalars: the guard must not depend on the local key.
        for sk in [[0x11u8; 32], [0xa5u8; 32]] {
            for enc in LOW_ORDER {
                let peer = unhex(enc);
                assert_eq!(
                    hs_dh_shared(&sk, &peer),
                    Err(HS_DH_NONCONTRIBUTORY),
                    "low-order peer key {enc} must be rejected"
                );
            }
        }
    }

    #[test]
    fn seven_of_eight_low_order_keys_evade_the_encoding_check() {
        // Pins the reason the OUTPUT check is required: the ingress screen catches only one of them.
        let evade = LOW_ORDER
            .iter()
            .filter(|enc| !hs_dh_pub_is_all_zero(&unhex(enc)))
            .count();
        assert_eq!(evade, 7);
    }

    #[test]
    fn establishment_dh_accepts_an_honest_peer_key() {
        // Negative control: a real public key must still produce a shared secret, so the guard
        // cannot be passing by rejecting everything.
        let sk = [0x11u8; 32];
        let mut basepoint = [0u8; 32];
        basepoint[0] = 9;
        let peer = hs_dh_shared(&[0x77u8; 32], &basepoint).expect("honest pub derivation");
        let shared = hs_dh_shared(&sk, &peer).expect("honest peer key must be accepted");
        assert_ne!(shared, [0u8; 32]);
    }

    #[test]
    fn length_errors_keep_their_distinct_marker() {
        // The new marker must not swallow the pre-existing `handshake_dh_len` failure, whose callers
        // map it to the `dh_failed` log reason.
        assert_eq!(
            hs_dh_shared(&[0u8; 31], &[1u8; 32]),
            Err("handshake_dh_len")
        );
        assert_eq!(
            hs_dh_shared(&[1u8; 32], &[0u8; 31]),
            Err("handshake_dh_len")
        );
    }
}

// ===========================================================================================
// NA-0775 (`D-1418`) -- T10: THE LATE-LANDING GUARD, DRIVEN.
//
// ⚠⚠ WHY THIS IS A UNIT TEST AND NOT AN INTEGRATION ARM. The state the guard exists for is not
// reachable through the CLI: after a successful pass the pending is cleared, so the branch
// cannot be re-entered with a stale epoch-0 `st`. Reproducing cold-read F-2's Variant A for
// real would require suspending a process between the push returning and the store. These
// tests therefore prove the PREDICATE, not the SCHEDULING -- said here rather than discovered
// later.
//
// ⛳ AND ONE THING THEY BUY THAT `t8`/`t9` CANNOT: they are NOT behind
// `--cfg qsc_rng_failure_test_seam`, so unlike every arm that proves the rest of this repair,
// **these run in the ordinary suite and on every required check.** The guard's proof lives on
// the board; the seam's does not (`WF-0093`).
//
// `t10b` is the NEGATIVE CONTROL and it is permanent rather than a one-off tamper: it performs
// the SAME late store with the guard bypassed and asserts the rollback DOES happen. A guard
// whose absence was never shown to break the property is not evidence.
// ===========================================================================================
#[cfg(test)]
mod na0775_late_store_guard_tests {
    use super::*;
    use std::sync::{Mutex, MutexGuard, OnceLock};

    const TOUCHED_VARS: [&str; 4] = [
        "QSC_CONFIG_DIR",
        "QSC_ALLOW_SEED_FALLBACK",
        "QSC_UNSAFE_TEST_SEED_FALLBACK",
        // ⚠ The session-store key's test fallback derives from a SEED, not from the two
        // permission flags alone: `qsp_session_test_fallback_key` -> `qsp_seed_from_env`.
        // Measured, after the first run failed with `store_code=identity_secret_unavailable`.
        "QSC_QSP_SEED",
    ];

    static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

    fn env_lock() -> MutexGuard<'static, ()> {
        ENV_LOCK
            .get_or_init(|| Mutex::new(()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// Restores on `Drop`, including the unset case, so a panicking assertion cannot leak a
    /// mutated environment into the rest of this binary. Same shape as
    /// `na0692_config_resolver_tests`, which is this crate's precedent for env-mutating units.
    struct EnvSnapshot {
        vars: Vec<(&'static str, Option<String>)>,
    }

    impl EnvSnapshot {
        fn take() -> Self {
            Self {
                vars: TOUCHED_VARS
                    .iter()
                    .map(|k| (*k, std::env::var(k).ok()))
                    .collect(),
            }
        }
    }

    impl Drop for EnvSnapshot {
        fn drop(&mut self) {
            for (key, value) in &self.vars {
                match value {
                    Some(v) => std::env::set_var(key, v),
                    None => std::env::remove_var(key),
                }
            }
        }
    }

    /// A private 0700 directory. `QSC_CONFIG_DIR` makes the resolver report
    /// `ConfigSource::EnvOverride`, whose parent check inspects only the directory ITSELF
    /// (`fs_store::check_parent_safe`), so a world-writable `/tmp` above it is not walked.
    fn arena(tag: &str) -> std::path::PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let dir = std::env::temp_dir().join(format!("qsc-na0775-{tag}-{}-{nanos}", std::process::id()));
        fs::create_dir_all(&dir).expect("arena");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&dir, fs::Permissions::from_mode(0o700)).expect("perms");
        }
        dir
    }

    fn enter(tag: &str) -> (MutexGuard<'static, ()>, EnvSnapshot, std::path::PathBuf) {
        let g = env_lock();
        let snap = EnvSnapshot::take();
        let dir = arena(tag);
        std::env::set_var("QSC_CONFIG_DIR", &dir);
        std::env::set_var("QSC_ALLOW_SEED_FALLBACK", "1");
        std::env::set_var("QSC_UNSAFE_TEST_SEED_FALLBACK", "1");
        std::env::set_var("QSC_QSP_SEED", "775");
        (g, snap, dir)
    }

    /// A session derived exactly as the A2 branch derives one: deterministic in its inputs, so
    /// the "late" copy below is byte-identical to the first, which is the whole premise of the
    /// guard's session-id predicate.
    fn session(sid: [u8; 16]) -> Suite2SessionState {
        hs_build_session(true, true, sid, [1u8; 32], [2u8; 32], [3u8; 32], [4u8; 32], [5u8; 32])
            .expect("build session")
    }

    const SID_A: [u8; 16] = [0xA1; 16];
    const SID_B: [u8; 16] = [0xB2; 16];

    #[test]
    fn t10a_the_guard_refuses_an_epoch_zero_store_over_an_advanced_session() {
        let (_g, _snap, _dir) = enter("t10a");

        hs_commit_session_guarded("self", "peer", &session(SID_A)).expect("first commit");

        // The user sends: the ratchet advances and the send path commits it before pushing.
        let mut advanced = qsp_session_load("peer").expect("load").expect("present");
        advanced.send.ns = 7;
        qsp_session_store("peer", &advanced).expect("store advanced");

        // A poll stalled between its push and its store wakes a lease later and lands HERE,
        // holding the epoch-0 state it derived before the stall.
        hs_commit_session_guarded("self", "peer", &session(SID_A)).expect("late commit");

        let after = qsp_session_load("peer").expect("load").expect("present");
        assert_eq!(
            after.send.ns, 7,
            "THE LATE STORE MUST NOT ROLL THE SEND CHAIN BACK. A fresh derivation of the SAME \
             session_id is byte-identical at epoch 0 (ck0/ns=0), so writing it over an advanced \
             session reuses the same chain key at the same counter on different plaintext -- \
             the class the tree names at msgqueue/mod.rs. Measured ns={}",
            after.send.ns
        );
    }

    #[test]
    fn t10b_without_the_guard_the_same_store_rolls_the_session_back() {
        let (_g, _snap, _dir) = enter("t10b");

        hs_commit_session_guarded("self", "peer", &session(SID_A)).expect("first commit");
        let mut advanced = qsp_session_load("peer").expect("load").expect("present");
        advanced.send.ns = 7;
        qsp_session_store("peer", &advanced).expect("store advanced");

        // ⚠ THE NEGATIVE CONTROL: the identical late store WITHOUT the guard.
        qsp_session_store("peer", &session(SID_A)).expect("unguarded late store");

        let after = qsp_session_load("peer").expect("load").expect("present");
        assert_eq!(
            after.send.ns, 0,
            "THE PROPERTY MUST BE BREAKABLE OR t10a PROVES NOTHING. Unguarded, the late store \
             rolls ns 7 -> 0. Measured ns={}",
            after.send.ns
        );
    }

    #[test]
    fn t10c_a_different_session_id_still_stores_so_a_re_handshake_is_not_refused() {
        let (_g, _snap, _dir) = enter("t10c");

        hs_commit_session_guarded("self", "peer", &session(SID_A)).expect("first commit");
        let mut advanced = qsp_session_load("peer").expect("load").expect("present");
        advanced.send.ns = 7;
        qsp_session_store("peer", &advanced).expect("store advanced");

        // A genuine re-handshake: `hs_session_id` draws 128 fresh CSPRNG bits every time, so a
        // new handshake NEVER carries the old id. The guard must not refuse this.
        hs_commit_session_guarded("self", "peer", &session(SID_B)).expect("re-handshake commit");

        let after = qsp_session_load("peer").expect("load").expect("present");
        assert_eq!(
            after.send.session_id, SID_B,
            "A NEW session_id MUST REPLACE AN ADVANCED SESSION. A predicate that refused this \
             would break every re-invite, which is why the guard compares the id and not `ns`."
        );
        assert_eq!(after.send.ns, 0, "the replacement is a fresh session at epoch 0");
    }
}

#[cfg(test)]
mod na0780_lifecycle_tests {
    use super::*;

    fn isolated_process(test: &str) -> bool {
        let name = format!("handshake::na0780_lifecycle_tests::{test}");
        if std::env::var("QSC_NA0780_UNIT_CHILD").as_deref() == Ok(name.as_str()) {
            return false;
        }
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", &name, "--nocapture"])
            .env("QSC_NA0780_UNIT_CHILD", &name)
            .env_remove("QSC_CONFIG_DIR")
            .env_remove("QSC_ALLOW_SEED_FALLBACK")
            .env_remove("QSC_UNSAFE_TEST_SEED_FALLBACK")
            .env_remove("QSC_QSP_SEED")
            .output()
            .expect("start isolated lifecycle fixture");
        assert!(
            output.status.success(),
            "isolated fixture {test} failed:\n{}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        );
        assert!(String::from_utf8_lossy(&output.stdout).contains("1 passed; 0 failed"));
        true
    }

    struct Arena {
        _temp: tempfile::TempDir,
        prior: Option<std::ffi::OsString>,
    }
    impl Drop for Arena {
        fn drop(&mut self) {
            crate::set_vault_unlocked(false);
            match &self.prior {
                Some(v) => std::env::set_var("QSC_CONFIG_DIR", v),
                None => std::env::remove_var("QSC_CONFIG_DIR"),
            }
        }
    }
    fn setup() -> (Arena, FirstConnection, Suite2SessionState) {
        let temp = tempfile::tempdir().unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(temp.path(), fs::Permissions::from_mode(0o700)).unwrap();
        }
        let arena = Arena {
            prior: std::env::var_os("QSC_CONFIG_DIR"),
            _temp: temp,
        };
        std::env::set_var("QSC_CONFIG_DIR", arena._temp.path());
        vault::vault_init_with_passphrase("na0780-unit-fixture-passphrase").unwrap();
        vault::unlock_with_passphrase("na0780-unit-fixture-passphrase").unwrap();
        crate::set_vault_unlocked(true);
        identity_self_kem_keypair("self").unwrap();
        let (kem, _) = hs_kem_keypair();
        let (sig, _) = hs_sig_keypair();
        crate::contacts::contacts_provision_from_invite(
            "peer",
            &kem,
            &sig,
            "na0780-unit-original-route-00000001",
            "http://127.0.0.1:1",
            "na0780-unit-invite-slot-0000000004",
        )
        .unwrap();
        let mut c = hs_lifecycle_new("self", "peer").unwrap();
        let sid = [0x78; 16];
        let pending = HandshakePending {
            self_label: "self".into(),
            peer: "peer".into(),
            session_id: sid,
            kem_sk: vec![],
            kem_pk: vec![],
            dh_sk: vec![],
            dh_pub: vec![],
            sig_pk: vec![],
            resp_kem_ct: vec![],
            resp_kem_ss: vec![],
            peer_fp: Some(c.peer_fp.clone()),
            peer_sig_fp: None,
            peer_sig_pk: None,
            role: "initiator".into(),
            confirm_key: None,
            transcript_hash: None,
            pending_session: None,
            suite_context: None,
        };
        c.outgoing = Some(HsCandidate {
            pending,
            receipt: hs_receipt(
                "na0780-unit-invite-slot-0000000004",
                b"a1-envelope",
                b"a1",
                "na0780-unit-self-route-000000000003",
            )
            .unwrap(),
            reply: HsDelivery {
                relay: "http://127.0.0.1:1".into(),
                route: "na0780-unit-invite-slot-0000000004".into(),
                bytes: b"a1-envelope".to_vec(),
                ticket: Some("fixture".into()),
            },
        });
        hs_lifecycle_put(&c).unwrap();
        let st =
            hs_build_session(true, true, sid, [1; 32], [2; 32], [3; 32], [4; 32], [5; 32]).unwrap();
        (arena, c, st)
    }
    fn selection(c: &FirstConnection, st: &Suite2SessionState) -> Result<(), &'static str> {
        hs_lifecycle_select(
            c,
            &c.outgoing.as_ref().unwrap().pending,
            st,
            &hs_receipt(
                "na0780-unit-self-route-000000000003",
                b"b1-envelope",
                b"b1",
                "na0780-unit-selected-route-00000002",
            )
            .unwrap(),
            Some(HsDelivery {
                relay: "http://127.0.0.1:1".into(),
                route: "na0780-unit-selected-route-00000002".into(),
                bytes: b"exact-a2-reply".to_vec(),
                ticket: None,
            }),
        )
        .map(|_| ())
    }

    #[test]
    fn separate_write_interruptions_recover_and_preserve_advanced_same_session() {
        if isolated_process(
            "separate_write_interruptions_recover_and_preserve_advanced_same_session",
        ) {
            return;
        }
        // Real encrypted storage, reloaded after each injected process-interruption cut.
        // Authentication itself is covered by the two-party production-crypto regression.
        for cut in ["selection", "session", "route", "pending_clear", "applied"] {
            let (_arena, mut c, st) = setup();
            hs_lifecycle_recover(&mut c).unwrap();
            HS_LIFECYCLE_CUT.with(|v| v.set(cut));
            assert_eq!(
                selection(&c, &st),
                Err("test_process_interruption"),
                "cut {cut}"
            );
            // If a session became usable before interruption, simulate a real sender
            // advancing it. Recovery must preserve the entire stored snapshot.
            let advanced = if let Some(mut stored) = qsp_session_load("peer").unwrap() {
                stored.send.ns = 7;
                stored.recv.nr = 3;
                qsp_session_store("peer", &stored).unwrap();
                Some(stored.snapshot_bytes())
            } else {
                None
            };
            let mut reloaded = hs_lifecycle_find("self", "peer").unwrap().unwrap();
            hs_lifecycle_recover(&mut reloaded).unwrap();
            let restored = qsp_session_load("peer").unwrap().unwrap();
            assert_eq!(restored.send.session_id, st.send.session_id);
            if let Some(advanced) = advanced {
                assert!(restored.snapshot_bytes() == advanced, "ratchet cut {cut}");
            }
            assert_eq!(
                relay_peer_route_token("peer").unwrap(),
                "na0780-unit-selected-route-00000002"
            );
            assert!(hs_pending_load_state("self", "peer").unwrap().0.is_none());
            assert!(reloaded.selected.as_ref().unwrap().applied);
            assert!(
                reloaded.selected.as_ref().unwrap().session.is_none(),
                "retired initial ratchet snapshot"
            );
            for candidate in [&reloaded.outgoing, &reloaded.responder]
                .into_iter()
                .flatten()
            {
                assert!(
                    candidate.pending.kem_sk.is_empty()
                        && candidate.pending.dh_sk.is_empty()
                        && candidate.pending.resp_kem_ss.is_empty()
                        && candidate.pending.pending_session.is_none()
                );
            }
            assert_eq!(
                crate::facade::connect_status("peer").state,
                crate::facade::ConnectState::Inactive,
                "existing finish scan must remain eligible while A2 delivery is owed"
            );
            assert_eq!(
                reloaded
                    .selected
                    .as_ref()
                    .unwrap()
                    .reply
                    .as_ref()
                    .unwrap()
                    .bytes,
                b"exact-a2-reply"
            );
            let reply = reloaded.selected.as_ref().unwrap().reply.clone();
            HS_LIFECYCLE_CUT.with(|v| v.set("reply_delivered"));
            assert_eq!(
                hs_lifecycle_after_send(&reloaded, reply.as_ref()),
                Err("test_process_interruption")
            );
            assert!(hs_invite_recovery_pending("peer").unwrap());
            hs_lifecycle_after_send(&reloaded, reply.as_ref()).unwrap();
            assert_eq!(
                crate::facade::connect_status("peer").state,
                crate::facade::ConnectState::Active
            );
            hs_lifecycle_recover(&mut reloaded).unwrap();
            assert!(
                qsp_session_load("peer").unwrap().unwrap().snapshot_bytes()
                    == restored.snapshot_bytes(),
                "recovery changed session"
            );
        }
    }

    #[test]
    fn intent_and_pending_mirror_interruption_keeps_exact_generation_and_reply() {
        if isolated_process(
            "intent_and_pending_mirror_interruption_keeps_exact_generation_and_reply",
        ) {
            return;
        }
        let (_arena, c, _) = setup(); // Capsule persisted; mirror not yet written.
        assert!(hs_pending_load_state("self", "peer").unwrap().0.is_none());
        let mut reloaded = hs_lifecycle_find("self", "peer").unwrap().unwrap();
        HS_LIFECYCLE_CUT.with(|v| v.set("pending_mirror"));
        assert_eq!(
            hs_lifecycle_recover(&mut reloaded),
            Err("test_process_interruption")
        );
        let mut reloaded = hs_lifecycle_find("self", "peer").unwrap().unwrap();
        hs_lifecycle_recover(&mut reloaded).unwrap();
        assert_eq!(reloaded.generation, c.generation);
        assert_eq!(
            reloaded.outgoing.unwrap().reply.bytes,
            c.outgoing.unwrap().reply.bytes
        );
    }

    #[test]
    fn newer_pending_generation_and_unrelated_sessions_refuse_without_writes() {
        if isolated_process("newer_pending_generation_and_unrelated_sessions_refuse_without_writes")
        {
            return;
        }
        let (_arena, mut c, st) = setup();
        hs_lifecycle_recover(&mut c).unwrap();
        let mut newer = c.outgoing.as_ref().unwrap().pending.clone();
        newer.session_id = [0x79; 16];
        hs_pending_store(&newer).unwrap();
        let before = vault::secret_get(FIRST_CONNECTIONS_KEY).unwrap();
        assert_eq!(selection(&c, &st), Err("handshake_lifecycle_conflict"));
        assert_eq!(
            hs_pending_load_state("self", "peer")
                .unwrap()
                .0
                .unwrap()
                .session_id,
            newer.session_id
        );
        assert!(
            before == vault::secret_get(FIRST_CONNECTIONS_KEY).unwrap(),
            "capsule changed on refusal"
        );
        hs_pending_store(&c.outgoing.as_ref().unwrap().pending).unwrap();
        let mut other = st.clone();
        other.send.session_id = [0x79; 16];
        other.recv.session_id = [0x79; 16];
        qsp_session_store("peer", &other).unwrap();
        assert_eq!(selection(&c, &st), Err("contacts_session_exists"));
        assert!(
            qsp_session_load("peer").unwrap().unwrap().snapshot_bytes() == other.snapshot_bytes(),
            "unrelated session changed"
        );
        assert!(
            before == vault::secret_get(FIRST_CONNECTIONS_KEY).unwrap(),
            "capsule changed on refusal"
        );
        // Even a coincident SID is unrelated without a prior durable selection intent.
        qsp_session_store("peer", &st).unwrap();
        assert_eq!(selection(&c, &st), Err("contacts_session_exists"));
    }

    #[test]
    fn stale_capsule_and_changed_identity_binding_cannot_replace_current_generation() {
        if isolated_process(
            "stale_capsule_and_changed_identity_binding_cannot_replace_current_generation",
        ) {
            return;
        }
        let (_arena, c, _) = setup();
        let mut stale = c.clone();
        stale.generation[0] ^= 1;
        let before = vault::secret_get(FIRST_CONNECTIONS_KEY).unwrap();
        assert_eq!(
            hs_lifecycle_put(&stale),
            Err("handshake_lifecycle_conflict")
        );
        assert!(
            before == vault::secret_get(FIRST_CONNECTIONS_KEY).unwrap(),
            "capsule changed on refusal"
        );
        let mut contacts: crate::store::ContactsStore = serde_json::from_str(
            &vault::secret_get(crate::store::CONTACTS_SECRET_KEY)
                .unwrap()
                .unwrap(),
        )
        .unwrap();
        let record = contacts.peers.get_mut("peer").unwrap();
        record.sig_fp = Some("changed".into());
        record.devices[0].sig_fp = Some("changed".into());
        vault::secret_set(
            crate::store::CONTACTS_SECRET_KEY,
            &serde_json::to_string(&contacts).unwrap(),
        )
        .unwrap();
        assert_eq!(c.check_binding(), Err("contacts_identity_changed"));
        assert!(
            before == vault::secret_get(FIRST_CONNECTIONS_KEY).unwrap(),
            "capsule changed on refusal"
        );
    }

    #[test]
    fn count_serialized_bytes_and_saved_reply_caps_are_fail_closed() {
        if isolated_process("count_serialized_bytes_and_saved_reply_caps_are_fail_closed") {
            return;
        }
        let (_arena, c, _) = setup();
        let mut entries = Vec::new();
        for i in 0..FIRST_CONNECTION_GROUPS {
            let mut entry = c.clone();
            entry.peer = format!("peer{i}");
            entry.peer_fp = format!("identity{i}");
            entry.outgoing.as_mut().unwrap().pending.peer = entry.peer.clone();
            entries.push(entry);
        }
        let mut store = FirstConnections {
            version: 1,
            entries,
        };
        assert!(hs_lifecycle_validate(&store).is_ok());
        let mut excess = c.clone();
        excess.peer = "extra".into();
        excess.peer_fp = "extra".into();
        excess.outgoing.as_mut().unwrap().pending.peer = excess.peer.clone();
        store.entries.push(excess);
        assert_eq!(
            hs_lifecycle_validate(&store),
            Err("handshake_lifecycle_capacity")
        );
        store.entries.pop();
        store.entries[0].outgoing.as_mut().unwrap().reply.bytes = vec![0; FIRST_FRAME_BYTES + 1];
        assert_eq!(
            hs_lifecycle_validate(&store),
            Err("handshake_lifecycle_capacity")
        );
        store.entries[0].outgoing.as_mut().unwrap().reply.bytes = vec![0; FIRST_FRAME_BYTES];
        store.entries[0].outgoing.as_mut().unwrap().pending.kem_sk =
            vec![255; FIRST_CONNECTION_BYTES];
        assert_eq!(
            hs_lifecycle_validate(&store),
            Err("handshake_lifecycle_capacity")
        );
        assert!(hs_receipt("owned", &vec![0; FIRST_FRAME_BYTES + 1], b"frame", "route").is_err());
        assert!(
            hs_lifecycle_find("self", "peer").unwrap().is_some(),
            "capacity tests must not evict stored attempt"
        );
    }
    #[test]
    fn concurrent_selection_returns_the_already_saved_reply_not_a_new_signature() {
        if isolated_process(
            "concurrent_selection_returns_the_already_saved_reply_not_a_new_signature",
        ) {
            return;
        }
        let (_arena, mut c, st) = setup();
        hs_lifecycle_recover(&mut c).unwrap();
        selection(&c, &st).unwrap();
        let current = hs_lifecycle_find("self", "peer").unwrap().unwrap();
        let receipt = &current.selected.as_ref().unwrap().receipt;
        let stale_reply = HsDelivery {
            relay: "http://127.0.0.1:1".into(),
            route: receipt.route.clone(),
            bytes: b"different-late-signature".to_vec(),
            ticket: None,
        };
        let reply = hs_lifecycle_select(
            &c,
            &c.outgoing.as_ref().unwrap().pending,
            &st,
            receipt,
            Some(stale_reply),
        )
        .unwrap()
        .unwrap();
        assert_eq!(reply.bytes, b"exact-a2-reply");
        assert!(hs_invite_recovery_pending("peer").unwrap());
        hs_lifecycle_after_send(&current, Some(&reply)).unwrap();
        assert!(!hs_invite_recovery_pending("peer").unwrap());
    }

    #[test]
    fn occupied_responder_does_not_hide_original_b1_or_accept_unauthenticated_completion() {
        if isolated_process(
            "occupied_responder_does_not_hide_original_b1_or_accept_unauthenticated_completion",
        ) {
            return;
        }
        let (_arena, mut c, _) = setup();
        let mut provisional = c.outgoing.as_ref().unwrap().clone();
        provisional.pending.role = "responder".into();
        provisional.pending.session_id = [0x79; 16];
        c.responder = Some(provisional);
        hs_lifecycle_put(&c).unwrap();
        hs_lifecycle_recover(&mut c).unwrap();
        let b1 = hs_encode_resp(&HsResp {
            suite_context: HsSuiteContext::LegacyV1,
            session_id: c.outgoing.as_ref().unwrap().pending.session_id,
            kem_ct: vec![0; hs_kem_ct_len()],
            mac: [0; 32],
            sig_pk: vec![0; hs_sig_pk_len()],
            sig: vec![0; hs_sig_sig_len()],
            dh_pub: [3; 32],
        });
        let receipt = hs_receipt(
            "na0780-unit-self-route-000000000003",
            &b1,
            &b1,
            "na0780-unit-selected-route-00000002",
        )
        .unwrap();
        let plan = hs_lifecycle_plan(
            "self",
            "peer",
            &b1,
            receipt.clone(),
            true,
            HandshakeSuiteMode::LegacyCompat,
            false,
        )
        .unwrap();
        assert!(plan.immediate.is_none());
        assert_eq!(
            plan.pending.unwrap().session_id,
            c.outgoing.as_ref().unwrap().pending.session_id
        );
        let before = vault::secret_get(FIRST_CONNECTIONS_KEY).unwrap();
        let outcome = hs_poll_item(
            "self",
            "peer",
            "http://127.0.0.1:1",
            &receipt.mailbox,
            &receipt.route,
            HandshakeSuiteMode::LegacyCompat,
            &crate::InboxPullItem {
                id: "fixture".into(),
                data: b1,
            },
            false,
            true,
            receipt.clone(),
            None,
        )
        .unwrap();
        assert_eq!(outcome, PollOutcome::NotConsumed);
        assert!(qsp_session_load("peer").unwrap().is_none());
        assert!(
            before == vault::secret_get(FIRST_CONNECTIONS_KEY).unwrap(),
            "capsule changed on refusal"
        );
    }
    fn renamed(mut c: FirstConnection, name: &str) -> FirstConnection {
        c.peer = name.into();
        c.peer_fp = name.into();
        for candidate in [&mut c.outgoing, &mut c.responder].into_iter().flatten() {
            candidate.pending.peer = c.peer.clone();
        }
        c
    }

    #[test]
    fn completed_history_releases_only_delivered_applied_occupancy_and_preserves_replay() {
        if isolated_process(
            "completed_history_releases_only_delivered_applied_occupancy_and_preserves_replay",
        ) {
            return;
        }
        let (_arena, c, st) = setup();
        assert!(c.is_active());
        HS_LIFECYCLE_CUT.with(|v| v.set("selection"));
        assert_eq!(selection(&c, &st), Err("test_process_interruption"));
        let mut selected = hs_lifecycle_find("self", "peer").unwrap().unwrap();
        assert!(
            selected.is_active(),
            "unapplied intent still needs recovery"
        );
        hs_lifecycle_recover(&mut selected).unwrap();
        assert!(
            selected.is_active(),
            "applied with unsent A2 still occupies capacity"
        );
        let reply = selected.selected.as_ref().unwrap().reply.clone().unwrap();
        HS_LIFECYCLE_CUT.with(|v| v.set("reply_delivered"));
        assert_eq!(
            hs_lifecycle_after_send(&selected, Some(&reply)),
            Err("test_process_interruption")
        );
        assert!(hs_lifecycle_load().unwrap().entries[0].is_active());
        // Synthetic quota completion: real delivery is covered by the production-crypto arms.
        hs_lifecycle_after_send(&selected, Some(&reply)).unwrap();
        let completed = hs_lifecycle_find("self", "peer").unwrap().unwrap();
        assert!(!completed.is_active());
        let mut entries = vec![completed.clone()];
        for i in 1..FIRST_CONNECTION_GROUPS {
            entries.push(renamed(completed.clone(), &format!("history-{i}")));
        }
        let store = FirstConnections {
            version: 1,
            entries,
        };
        let history = serde_json::to_vec(&store.entries).unwrap();
        hs_lifecycle_save(&store).unwrap();
        hs_invite_admission_preflight("self", "new-peer", "new-slot").unwrap();
        let next = renamed(c, "new-peer");
        hs_lifecycle_put(&next).unwrap();
        let mut reloaded = hs_lifecycle_load().unwrap();
        assert_eq!(reloaded.entries.len(), 65);
        assert_eq!(reloaded.entries.iter().filter(|c| c.is_active()).count(), 1);
        reloaded.entries.pop();
        assert!(
            history == serde_json::to_vec(&reloaded.entries).unwrap(),
            "history changed on admission"
        );
        let mut advanced = st;
        advanced.send.ns = 7;
        advanced.recv.nr = 5;
        qsp_session_store("peer", &advanced).unwrap();
        let s = completed.selected.as_ref().unwrap();
        let plan = hs_lifecycle_plan(
            "self",
            "peer",
            b"b1",
            s.receipt.clone(),
            true,
            HandshakeSuiteMode::LegacyCompat,
            false,
        )
        .unwrap();
        assert!(plan.immediate == Some(PollOutcome::AlreadyComplete));
        assert!(
            plan.retry.unwrap().bytes == reply.bytes,
            "exact reply must survive capacity release"
        );
        assert!(
            qsp_session_load("peer").unwrap().unwrap().snapshot_bytes()
                == advanced.snapshot_bytes()
        );
    }

    #[test]
    fn legacy_active_store_remains_readable_and_refuses_new_redemption_before_writes() {
        if isolated_process(
            "legacy_active_store_remains_readable_and_refuses_new_redemption_before_writes",
        ) {
            return;
        }
        let (_arena, c, st) = setup();
        let mut entries = vec![c.clone()];
        for i in 1..FIRST_CONNECTION_GROUPS {
            entries.push(renamed(c.clone(), &format!("active-{i}")));
        }
        // Old v1 stores had no growth reservations. They remain structurally readable,
        // including 64 small active entries whose reserved JSON framing exceeds 16 MiB.
        hs_lifecycle_save(&FirstConnections {
            version: 1,
            entries,
        })
        .unwrap();
        let old = hs_lifecycle_load().unwrap();
        assert_eq!(old.entries.len(), 64);
        assert!(hs_lifecycle_reserved_bytes(&old).unwrap() > FIRST_CONNECTION_TOTAL_BYTES);
        let before = vault::secret_get(FIRST_CONNECTIONS_KEY).unwrap();
        assert_eq!(
            hs_lifecycle_put(&renamed(c.clone(), "excess")),
            Err("handshake_lifecycle_capacity")
        );
        assert!(before == vault::secret_get(FIRST_CONNECTIONS_KEY).unwrap());
        assert_capacity_precedes_redemption();
        // A retry needs no new slot; existing authentication and identity gates still run.
        hs_invite_admission_preflight("self", "peer", &c.outgoing.as_ref().unwrap().reply.route)
            .unwrap();
        selection(&c, &st).unwrap();
        let selected = hs_lifecycle_find("self", "peer").unwrap().unwrap();
        assert!(selected.is_active());
        hs_lifecycle_after_send(
            &selected,
            selected.selected.as_ref().unwrap().reply.as_ref(),
        )
        .unwrap();
        assert!(!hs_lifecycle_find("self", "peer")
            .unwrap()
            .unwrap()
            .is_active());
    }

    fn assert_capacity_precedes_redemption() {
        let payload = crate::invite::InvitePayload {
            ver: 1,
            typ: 1,
            invite_id: [0x45; 16],
            expiry: u64::MAX,
            relay_ep: "http://127.0.0.1:1".into(),
            cap: [0x46; 16],
            commit: [0x47; 32],
        };
        let code = crate::invite::encode_invite_code(&payload).unwrap();
        let before = vault::secret_get(crate::store::REDEMPTIONS_SECRET_KEY).unwrap();
        assert_eq!(
            crate::invite::invite_redeem_at(&code, "excess", Some("self"), 1),
            Err("handshake_lifecycle_capacity"),
        );
        assert!(
            before == vault::secret_get(crate::store::REDEMPTIONS_SECRET_KEY).unwrap(),
            "known capacity refusal must precede the redemption record and network call"
        );
    }

    #[test]
    fn retained_bytes_cannot_spend_active_growth_reservations() {
        if isolated_process("retained_bytes_cannot_spend_active_growth_reservations") {
            return;
        }
        let (_arena, c, st) = setup();
        selection(&c, &st).unwrap();
        let selected = hs_lifecycle_find("self", "peer").unwrap().unwrap();
        hs_lifecycle_after_send(
            &selected,
            selected.selected.as_ref().unwrap().reply.as_ref(),
        )
        .unwrap();
        let mut completed = hs_lifecycle_find("self", "peer").unwrap().unwrap();
        // Dense JSON byte encoding for both retained exact replies. Size the
        // fixture from its actual encoding, including identity/device metadata.
        completed.outgoing.as_mut().unwrap().reply.bytes = vec![255; FIRST_FRAME_BYTES / 2];
        completed
            .selected
            .as_mut()
            .unwrap()
            .reply
            .as_mut()
            .unwrap()
            .bytes = vec![255; FIRST_FRAME_BYTES / 2];
        let actual = serde_json::to_vec(&completed).unwrap().len();
        completed
            .device
            .push_str(&"x".repeat(FIRST_CONNECTION_BYTES - 1024 - actual));
        let mut entries = Vec::new();
        for i in 0..64 {
            let mut entry = renamed(completed.clone(), &format!("history-{i}"));
            let actual = serde_json::to_vec(&entry).unwrap().len();
            if actual < FIRST_CONNECTION_BYTES - 1024 {
                entry
                    .device
                    .push_str(&"x".repeat(FIRST_CONNECTION_BYTES - 1024 - actual));
            } else {
                entry
                    .device
                    .truncate(entry.device.len() - (actual - (FIRST_CONNECTION_BYTES - 1024)));
            }
            assert_eq!(
                serde_json::to_vec(&entry).unwrap().len(),
                FIRST_CONNECTION_BYTES - 1024
            );
            entries.push(entry);
        }
        let mut store = FirstConnections {
            version: 1,
            entries,
        };
        assert!(serde_json::to_vec(&store).unwrap().len() < FIRST_CONNECTION_TOTAL_BYTES);
        assert_eq!(store.entries.iter().filter(|c| c.is_active()).count(), 0);
        hs_lifecycle_save(&store).unwrap();
        assert_capacity_precedes_redemption();
        // One fewer retained record makes room to reserve a whole active record.
        store.entries.pop();
        hs_lifecycle_save(&store).unwrap();
        let active = renamed(c, "active");
        hs_lifecycle_put(&active).unwrap();
        let mut admitted = hs_lifecycle_load().unwrap();
        assert!(hs_lifecycle_reserved_bytes(&admitted).unwrap() <= FIRST_CONNECTION_TOTAL_BYTES);
        assert_eq!(admitted.entries.iter().filter(|c| c.is_active()).count(), 1);
        // Fill retained history to 128 bytes below the reservation budget. This is
        // a synthetic byte-boundary fixture, not cryptographic session evidence.
        let mut growth =
            FIRST_CONNECTION_TOTAL_BYTES - 128 - hs_lifecycle_reserved_bytes(&admitted).unwrap();
        for entry in admitted.entries.iter_mut().filter(|c| !c.is_active()) {
            let room = FIRST_CONNECTION_BYTES - serde_json::to_vec(entry).unwrap().len();
            let add = room.min(growth);
            entry.device.push_str(&"x".repeat(add));
            growth -= add;
        }
        assert_eq!(growth, 0);
        hs_lifecycle_save(&admitted).unwrap();
        let reserve = hs_lifecycle_reserved_bytes(&admitted).unwrap();
        assert_eq!(reserve, FIRST_CONNECTION_TOTAL_BYTES - 128);
        let mut growing = admitted
            .entries
            .iter()
            .find(|c| {
                !c.is_active()
                    && serde_json::to_vec(c).unwrap().len() + 129 <= FIRST_CONNECTION_BYTES
            })
            .unwrap()
            .clone();
        growing.device.push_str(&"x".repeat(129));
        let mut trial: FirstConnections =
            serde_json::from_slice(&serde_json::to_vec(&admitted).unwrap()).unwrap();
        *trial
            .entries
            .iter_mut()
            .find(|c| c.peer == growing.peer)
            .unwrap() = growing.clone();
        assert!(
            serde_json::to_vec(&trial).unwrap().len() < FIRST_CONNECTION_TOTAL_BYTES,
            "this must exercise reserved bytes, not the actual-byte cap"
        );
        let before = vault::secret_get(FIRST_CONNECTIONS_KEY).unwrap();
        assert_eq!(
            hs_lifecycle_put(&growing),
            Err("handshake_lifecycle_capacity")
        );
        assert!(before == vault::secret_get(FIRST_CONNECTIONS_KEY).unwrap());
        // An admitted active record can use its whole allowance without increasing
        // aggregate charge, even with almost no unreserved space left.
        let mut expanded = active;
        let actual = serde_json::to_vec(&expanded).unwrap().len();
        expanded
            .device
            .push_str(&"x".repeat(FIRST_CONNECTION_BYTES - actual));
        assert_eq!(
            serde_json::to_vec(&expanded).unwrap().len(),
            FIRST_CONNECTION_BYTES
        );
        hs_lifecycle_put(&expanded).unwrap();
        assert_eq!(
            hs_lifecycle_reserved_bytes(&hs_lifecycle_load().unwrap()).unwrap(),
            reserve
        );
    }
}
