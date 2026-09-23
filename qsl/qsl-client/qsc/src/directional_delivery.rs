//! Directional delivery transactions and exact-wire receipts.
//! External release requires the authoritative vault commit.
use crate::directional_core::{ectx, h, k, lp, Key, PROFILE, R};
use quantumshield_refimpl::crypto::stdcrypto::StdCrypto;
use quantumshield_refimpl::crypto::traits::Aead;
use serde::{Deserialize, Serialize};
use zeroize::Zeroize;

pub(crate) const INTEGRATION_PROFILE: &[u8] = b"NA0780-DIR-INTEGRATION-03";
const RECEIPT_PREFIX: usize = 4 + 16 + 1 + 8 + 32 + 4;
const RECEIPT_LEN: usize = RECEIPT_PREFIX + 48;

/// Retained separately from mutable traffic chains. Construct only at the
/// authenticated epoch activation; cloning is staging, not authority to reseal.
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ReceiptContext {
    sid: [u8; 16],
    direction: u8,
    epoch: u64,
    dh: Key,
    key: Key,
}
impl Drop for ReceiptContext {
    fn drop(&mut self) {
        self.key.zeroize();
    }
}
impl ReceiptContext {
    pub(crate) fn activated(
        sid: [u8; 16],
        direction: u8,
        epoch: u64,
        dh: Key,
        root: &Key,
    ) -> R<Self> {
        if direction > 1 {
            return Err("RECEIPT_DIRECTION");
        }
        let mut binding = lp(INTEGRATION_PROFILE);
        binding.extend(ectx(epoch, direction, &dh));
        let key = k(&sid, root, "RECEIPT_KEY", &binding);
        Ok(Self {
            sid,
            direction,
            epoch,
            dh,
            key,
        })
    }
    fn prefix(&self, slot: u32) -> Vec<u8> {
        let mut out = b"NDR1".to_vec();
        out.extend(self.sid);
        out.push(self.direction);
        out.extend(self.epoch.to_be_bytes());
        out.extend(self.dh);
        out.extend(slot.to_be_bytes());
        out
    }
    fn ad_nonce(&self, slot: u32) -> (Vec<u8>, [u8; 12]) {
        let mut ad = lp(INTEGRATION_PROFILE);
        ad.extend(lp(PROFILE));
        ad.extend(self.prefix(slot));
        let mut nonce = [0; 12];
        nonce[8..].copy_from_slice(&slot.to_be_bytes());
        (ad, nonce)
    }
    /// Only for a newly authenticated disposition. The durable controller must
    /// persist the returned bytes before release, and replay them on duplicates.
    /// This primitive alone does not provide that transaction or retry policy.
    pub(crate) fn construct(
        &self,
        receiver_role: u8,
        slot: u32,
        admitted_wire: &[u8],
    ) -> R<Vec<u8>> {
        if receiver_role > 1 || receiver_role == self.direction {
            return Err("RECEIPT_ROLE");
        }
        let (ad, nonce) = self.ad_nonce(slot);
        let ct = crate::directional_core::seal(&self.key, &nonce, &ad, &h(admitted_wire))?;
        if ct.len() != 48 {
            return Err("RECEIPT_SEAL");
        }
        let mut out = self.prefix(slot);
        out.extend(ct);
        Ok(out)
    }
    pub(crate) fn verify(
        &self,
        sender_role: u8,
        slot: u32,
        outstanding_wire: &[u8],
        receipt: &[u8],
    ) -> R<()> {
        if sender_role != self.direction {
            return Err("RECEIPT_ROLE");
        }
        if receipt.len() != RECEIPT_LEN || receipt[..RECEIPT_PREFIX] != self.prefix(slot) {
            return Err("RECEIPT_BINDING");
        }
        let (ad, nonce) = self.ad_nonce(slot);
        let pt = StdCrypto
            .open(&self.key, &nonce, &ad, &receipt[RECEIPT_PREFIX..])
            .map_err(|_| "RECEIPT_AUTH")?;
        let expected = h(outstanding_wire);
        if pt.len() != expected.len()
            || pt.iter().zip(expected).fold(0u8, |d, (a, b)| d | (*a ^ b)) != 0
        {
            return Err("RECEIPT_CONTENT");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn receipt_construction_and_binding() {
        // Fresh test keys. This tests the recorded primitive, not client
        // establishment, crash safety, retry sealing or root activation.
        use rand_core::{OsRng, RngCore};
        let mut root = [0; 32];
        OsRng.fill_bytes(&mut root);
        let ctx = ReceiptContext::activated([1; 16], 0, 0, [2; 32], &root).unwrap();
        let raw = b"fresh receipt primitive fixture";
        let receipt = ctx.construct(1, 7, raw).unwrap();
        assert_eq!(receipt.len(), RECEIPT_LEN);
        // Internal identical computation is deterministic; release requires the
        // controller commit, not permission from a process-global seal ledger.
        assert_eq!(ctx.construct(1, 7, raw).unwrap(), receipt);
        assert_eq!(ctx.verify(0, 7, raw, &receipt), Ok(()));
        // Exact stored-byte replay requires no new seal and no sending chain.
        assert_eq!(ctx.verify(0, 7, raw, &receipt.clone()), Ok(()));
        assert_eq!(ctx.verify(1, 7, raw, &receipt), Err("RECEIPT_ROLE"));
        assert_eq!(ctx.construct(0, 7, raw), Err("RECEIPT_ROLE"));
        assert_eq!(ctx.verify(0, 8, raw, &receipt), Err("RECEIPT_BINDING"));
        assert_eq!(
            ctx.verify(0, 7, b"different admitted wire", &receipt),
            Err("RECEIPT_CONTENT")
        );
        for index in [0, 4, 20, 21, 29, 61] {
            let mut changed = receipt.clone();
            changed[index] ^= 1;
            assert_eq!(ctx.verify(0, 7, raw, &changed), Err("RECEIPT_BINDING"));
        }
        let mut changed = receipt.clone();
        *changed.last_mut().unwrap() ^= 1;
        assert_eq!(ctx.verify(0, 7, raw, &changed), Err("RECEIPT_AUTH"));
        assert_eq!(
            ctx.verify(0, 7, raw, &receipt[..receipt.len() - 1]),
            Err("RECEIPT_BINDING")
        );
        let mut trailing = receipt.clone();
        trailing.push(0);
        assert_eq!(ctx.verify(0, 7, raw, &trailing), Err("RECEIPT_BINDING"));
        let different_root = ReceiptContext::activated([1; 16], 0, 0, [2; 32], &[9; 32]).unwrap();
        assert_eq!(
            different_root.verify(0, 7, raw, &receipt),
            Err("RECEIPT_AUTH")
        );
        let restored: ReceiptContext =
            serde_json::from_slice(&serde_json::to_vec(&ctx).unwrap()).unwrap();
        assert_eq!(restored.verify(0, 7, raw, &receipt), Ok(()));
        root.zeroize();
    }
}

use crate::directional_core::{Core, Wire};
use std::collections::{BTreeMap, BTreeSet};

const MAX_RECORD: usize = 16 * 1024 * 1024;
const MAX_BYTES: usize = 4 * 1024 * 1024;
const WINDOW: u32 = 16;

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct EpochReceipts {
    context: ReceiptContext,
    next: u32,
    prefix: u32,
    confirmed: u32,
    terminal: Option<u32>,
    holes: BTreeSet<u32>,
}
impl EpochReceipts {
    fn new(context: ReceiptContext) -> Self {
        Self {
            context,
            next: 0,
            prefix: 0,
            confirmed: 0,
            terminal: None,
            holes: BTreeSet::new(),
        }
    }
    fn admit(&mut self, n: u32) -> R<()> {
        if n < self.prefix || !self.holes.insert(n) {
            return Err("DISPOSITION_REPLAY");
        }
        while self.holes.remove(&self.prefix) {
            self.prefix = self.prefix.checked_add(1).ok_or("COUNTER_OVERFLOW")?;
        }
        self.next = self.next.max(n.checked_add(1).ok_or("COUNTER_OVERFLOW")?);
        Ok(())
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Flight {
    body_hash: Key,
    intent_hash: Key,
    epoch: u64,
    slot: u32,
    id: String,
    wire: Vec<u8>,
    accepted: bool,
    // REVIEW ONLY: new persisted field, never default/backfill from current state.
    // Exact bytes included at seal; relay acceptance does not confirm this proof.
    closure_proof: String,
}
fn response_default_pending()->bool {true}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Disposition {
    hash: Key,
    receipt: Vec<u8>,
    #[serde(default="response_default_pending")]
    response_pending: bool,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Application {
    id: String,
    body: Vec<u8>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Transaction {
    version: String,
    // Transient view hydrated from the fresh authoritative owner by paired update.
    // It is never a third persistent key or a fallback for missing owner state.
    #[serde(skip)]
    reserve: Option<crate::protocol_state::SessionControlReserve>,
    #[serde(skip)]
    received_reference: Option<(String,u64,u32,Key)>,
    #[serde(skip)]
    useful_send_closure: bool,
    pub(crate) generation: u64,
    pub(crate) core: Core,
    send: BTreeMap<u64, EpochReceipts>,
    recv: BTreeMap<u64, EpochReceipts>,
    flights: BTreeMap<String, Flight>,
    dispositions: BTreeMap<String, Disposition>,
    events: BTreeMap<String, Application>,
    completed: BTreeMap<String, Key>,
    request_sent: bool,
    recv_floor: Option<u64>,
    send_floor: Option<u64>,
    demand: bool,
    since_boundary: u32,
    last_boundary: u64,
}
#[derive(Clone)]
struct Closure {
    epoch: u64,
    dh: Key,
    count: u32,
    final_epoch: bool,
}
fn slot_key(epoch: u64, slot: u32) -> String {
    format!("{epoch}:{slot}")
}
/// Enqueue-time padding reservation. No peer state or clock participates in sizing.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Padding {
    pub(crate) profile: u8,
    pub(crate) maximum: u32,
    pub(crate) size: u32,
}
impl Padding {
    fn floor(profile: u8) -> R<usize> {
        match profile { 1 => Ok(1024), 2 => Ok(2048), 3 => Ok(4096), _ => Err("INTEGRATION_PADDING_PROFILE") }
    }
    pub(crate) fn resolve(id_len: usize, payload_len: usize, profile: u8,
                          maximum: Option<usize>, exact: Option<usize>) -> R<Self> {
        if id_len > 64 || payload_len > 60000 { return Err("INTEGRATION_LENGTH"); }
        let required = 16usize.checked_add(id_len).and_then(|n| n.checked_add(135))
            .and_then(|n| n.checked_add(payload_len)).ok_or("INTEGRATION_LENGTH")?
            .max(Self::floor(profile)?);
        let maximum = maximum.unwrap_or(4096);
        if maximum == 0 || maximum > 65536 { return Err("INTEGRATION_PADDING_LIMIT"); }
        let size = match exact { Some(n) => n, None => required.checked_next_power_of_two().ok_or("INTEGRATION_LENGTH")? };
        if size < required || size > maximum || size > 60000 { return Err("INTEGRATION_PADDING_SIZE"); }
        Ok(Self { profile, maximum: maximum as u32, size: size as u32 })
    }
    fn validate(&self, id_len: usize, payload_len: usize) -> R<()> {
        let expected = Self::resolve(id_len, payload_len, self.profile,
            Some(self.maximum as usize), Some(self.size as usize))?;
        if expected != *self { return Err("INTEGRATION_PADDING_SIZE"); }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct QueuedIntent {
    profile: String,
    kind: u8,
    id: String,
    body_hash: Key,
    pub(crate) padding: Padding,
}
impl QueuedIntent {
    pub(crate) fn message(id: &str, body: &[u8], padding: Padding) -> R<Self> {
        validate_typed_payload(0, id, body)?;
        padding.validate(id.len(), body.len())?;
        Ok(Self { profile: String::from_utf8(INTEGRATION_PROFILE.to_vec()).unwrap(),
            kind: 0, id: id.to_owned(), body_hash: h(body), padding })
    }
    pub(crate) fn encode(&self) -> R<Vec<u8>> {
        serde_json::to_vec(self).map_err(|_| "INTEGRATION_QUEUE_ENCODE")
    }
    pub(crate) fn decode(raw: &[u8], id: &str, body: &[u8]) -> R<Self> {
        if raw.len() > 1024 { return Err("INTEGRATION_QUEUE_SIZE"); }
        let value: Self = serde_json::from_slice(raw).map_err(|_| "INTEGRATION_QUEUE_INVALID")?;
        if value.profile.as_bytes() != INTEGRATION_PROFILE || value.kind != 0 || value.id != id || value.body_hash != h(body) {
            return Err("APPLICATION_ID_CONFLICT");
        }
        value.padding.validate(id.len(), body.len())?;
        Ok(value)
    }
}

fn validate_typed_payload(kind: u8, id: &str, payload: &[u8]) -> R<()> {
    if payload.len() > 60000 { return Err("INTEGRATION_LENGTH"); }
    match kind {
        0..=4 if id.is_empty() || id.len() > 64 || !id.is_ascii() => Err("INTEGRATION_ID"),
        0 => Ok(()),
        1..=4 => crate::store::directional_file_shape(kind, payload),
        5 if id.is_empty() && payload.len() == 1 && payload[0] <= 2 => Ok(()),
        5 => Err("INTEGRATION_BODY"),
        _ => Err("INTEGRATION_KIND"),
    }
}

fn typed_body_encode(kind: u8, id: &str, body: &[u8], closures: &[Closure], padding: &Padding) -> R<Vec<u8>> {
    validate_typed_payload(kind, id, body)?;
    padding.validate(id.len(), body.len())?;
    if closures.len() > 3 { return Err("CLOSURE_CAPACITY"); }
    if closures.windows(2).any(|p| p[0].epoch >= p[1].epoch) { return Err("CLOSURE_ORDER"); }
    let mut out = b"NDI2".to_vec();
    out.push(kind);
    out.push(id.len() as u8);
    out.extend(id.as_bytes());
    out.push(closures.len() as u8);
    for c in closures {
        out.extend(c.epoch.to_be_bytes()); out.extend(c.dh);
        out.extend(c.count.to_be_bytes()); out.push(u8::from(c.final_epoch));
    }
    out.push(padding.profile);
    out.extend((body.len() as u32).to_be_bytes());
    out.extend(padding.size.to_be_bytes());
    out.extend(body);
    if out.len() > padding.size as usize { return Err("INTEGRATION_PADDING_SIZE"); }
    out.resize(padding.size as usize, 0);
    Ok(out)
}

struct TypedBody {
    kind: u8,
    id: String,
    payload: Vec<u8>,
    closures: Vec<Closure>,
}
fn typed_body_decode(raw: &[u8]) -> R<TypedBody> {
    struct Reader<'a>(&'a [u8]);
    impl<'a> Reader<'a> {
        fn take(&mut self, n: usize) -> R<&'a [u8]> {
            if n > self.0.len() { return Err("INTEGRATION_LENGTH"); }
            let (a, b) = self.0.split_at(n); self.0 = b; Ok(a)
        }
        fn byte(&mut self) -> R<u8> { Ok(self.take(1)?[0]) }
        fn u32(&mut self) -> R<u32> { Ok(u32::from_be_bytes(self.take(4)?.try_into().unwrap())) }
    }
    if raw.len() > 60000 { return Err("INTEGRATION_LENGTH"); }
    let mut r = Reader(raw);
    if r.take(4)? != b"NDI2" { return Err("INTEGRATION_PROFILE"); }
    let kind = r.byte()?;
    let id_len = r.byte()? as usize;
    if id_len > 64 { return Err("INTEGRATION_ID"); }
    let id = std::str::from_utf8(r.take(id_len)?).map_err(|_| "INTEGRATION_ID")?.to_owned();
    let count = r.byte()?;
    if count > 3 { return Err("CLOSURE_CAPACITY"); }
    let mut closures: Vec<Closure> = Vec::new();
    for _ in 0..count {
        let epoch = u64::from_be_bytes(r.take(8)?.try_into().unwrap());
        if closures.last().is_some_and(|c| c.epoch >= epoch) { return Err("CLOSURE_ORDER"); }
        let dh = r.take(32)?.try_into().unwrap();
        let count = r.u32()?;
        let final_byte = r.byte()?;
        if final_byte > 1 { return Err("CLOSURE_FINAL"); }
        closures.push(Closure { epoch, dh, count, final_epoch: final_byte == 1 });
    }
    let profile = r.byte()?;
    let payload_len = r.u32()? as usize;
    let total = r.u32()? as usize;
    if total != raw.len() { return Err("INTEGRATION_LENGTH"); }
    // A sender reserves all three closures, even when the actual frame uses fewer.
    Padding::resolve(id.len(), payload_len, profile, Some(65536), Some(total))?;
    let payload = r.take(payload_len)?.to_vec();
    if r.0.iter().any(|b| *b != 0) { return Err("INTEGRATION_PADDING_NONZERO"); }
    validate_typed_payload(kind, &id, &payload)?;
    Ok(TypedBody { kind, id, payload, closures })
}
fn body_decode(raw: &[u8]) -> R<(String, Vec<u8>, bool, Vec<Closure>)> {
    let body = typed_body_decode(raw)?;
    if (1..=4).contains(&body.kind) { return Err("INTEGRATION_FILE_GATED"); }
    Ok((body.id, body.payload, body.kind == 5, body.closures))
}
impl Transaction {
    pub(crate) fn established(
        session: &quantumshield_refimpl::suite2::state::Suite2SessionState,
        now: u64,
    ) -> R<Self> {
        let core = Core::authenticated(session);
        let initial = if core.role == 0 {
            core.send.as_ref()
        } else {
            core.recv.get(&0)
        }
        .ok_or("INITIAL_EPOCH")?;
        let context = ReceiptContext::activated(core.sid, 0, 0, initial.dh, &core.root)?;
        let mut send = BTreeMap::new();
        let mut recv = BTreeMap::new();
        if core.role == 0 {
            send.insert(0, EpochReceipts::new(context));
        } else {
            recv.insert(0, EpochReceipts::new(context));
        }
        Ok(Self {
            version: String::from_utf8(INTEGRATION_PROFILE.to_vec()).unwrap(),
            generation: 0,
            reserve: None,
            received_reference: None,
            useful_send_closure: false,
            core,
            send,
            recv,
            flights: BTreeMap::new(),
            dispositions: BTreeMap::new(),
            events: BTreeMap::new(),
            completed: BTreeMap::new(),
            request_sent: false,
            recv_floor: None,
            send_floor: None,
            demand: false,
            since_boundary: 0,
            last_boundary: now,
        })
    }
    pub(crate) fn decode(raw: &str) -> R<Self> {
        crate::protocol_state::approved_directional_layout()?;
        if raw.len() > MAX_RECORD {
            return Err("TRANSACTION_CAPACITY");
        }
        let value: Self = serde_json::from_str(raw).map_err(|_| "TRANSACTION_TAMPERED")?;
        if value.version.as_bytes() != INTEGRATION_PROFILE {
            return Err("TRANSACTION_PROFILE");
        }
        value.bounds(0)?;
        Ok(value)
    }
    pub(crate) fn encode(&self) -> R<String> {
        crate::protocol_state::approved_directional_layout()?;
        self.bounds(0)?;
        serde_json::to_string(self).map_err(|_| "TRANSACTION_ENCODE")
    }
    fn bounds(&self, reserve: usize) -> R<()> {
        if self.send.len() + self.recv.len() > 3
            || self.flights.values().filter(|f| !f.id.is_empty()).count() > 64
            || self.flights.values().filter(|f| f.id.is_empty()).count() > 1
            || self.events.len() > 64
            || self.completed.len() > 64
            || self.flights.values().map(|f| f.wire.len()).sum::<usize>() + reserve > MAX_BYTES
            || self
                .dispositions
                .values()
                .map(|d| d.receipt.len())
                .sum::<usize>()
                + self.events.values().map(|e| e.body.len()).sum::<usize>()
                + reserve
                > MAX_BYTES
            || serde_json::to_vec(self)
                .map_err(|_| "TRANSACTION_ENCODE")?
                .len()
                + reserve.saturating_mul(8)
                > MAX_RECORD
        {
            return Err("TRANSACTION_CAPACITY");
        }
        Ok(())
    }
    fn promises(&self)->Vec<Closure> {
        let mut can_close=true;let mut out=Vec::new();
        for(g,e) in &self.send {
            let final_epoch=can_close && e.terminal==Some(e.prefix);
            if !final_epoch {can_close=false;}
            if e.prefix>e.confirmed || final_epoch {out.push(Closure{epoch:*g,dh:e.context.dh,count:e.prefix,final_epoch});}
        }
        out
    }
    fn confirm_carrier(&mut self, closures: &[Closure]) -> R<()> {
        for c in closures {
            // A delayed older carrier can repeat already confirmed coverage.
            if self.send_floor.is_some_and(|floor| c.epoch <= floor) { continue; }
            let e = self.send.get(&c.epoch).ok_or("CLOSURE_EPOCH")?;
            if c.dh != e.context.dh || c.count > e.prefix {
                return Err("CLOSURE_PREFIX");
            }
            if c.final_epoch {
                if self.send.keys().next().copied() != Some(c.epoch)
                    || e.terminal != Some(c.count) || e.prefix != c.count
                    || !e.holes.is_empty()
                    || self.flights.values().any(|f| f.epoch == c.epoch) {
                    return Err("CLOSURE_TERMINAL");
                }
                self.send.remove(&c.epoch);
                self.send_floor = Some(c.epoch);
            } else {
                let e = self.send.get_mut(&c.epoch).ok_or("CLOSURE_EPOCH")?;
                e.confirmed = e.confirmed.max(c.count);
            }
        }
        Ok(())
    }
    fn receive_promises(&mut self, closures: &[Closure]) -> R<()> {
        for c in closures {
            if self.recv_floor.is_some_and(|f| c.epoch <= f) {
                continue;
            }
            let e = self.recv.get_mut(&c.epoch).ok_or("CLOSURE_EPOCH")?;
            if c.dh != e.context.dh || c.count > e.prefix {
                return Err("CLOSURE_PREFIX");
            }
            if c.final_epoch
                && (e.terminal != Some(c.count)
                    || self.recv.keys().next().copied() != Some(c.epoch))
            {
                return Err("CLOSURE_TERMINAL");
            }
            self.dispositions.retain(|key,_| {
                !key.split_once(':').is_some_and(|(g,n)|g.parse::<u64>()==Ok(c.epoch) && n.parse::<u32>().is_ok_and(|n|n<c.count))
            });
            if c.final_epoch {
                self.recv.remove(&c.epoch);
                self.recv_floor = Some(c.epoch);
            } else {
                let e = self.recv.get_mut(&c.epoch).unwrap(); e.confirmed = e.confirmed.max(c.count);
            }
        }
        Ok(())
    }
    pub(crate) fn pending(&self)->Vec<Vec<u8>> {
        self.flights.values().map(|f|f.wire.clone()).chain(self.dispositions.values().filter(|d|d.response_pending).map(|d|d.receipt.clone())).collect()
    }
    pub(crate) fn prepare(
        &mut self,
        id: &str,
        body: &[u8],
        now: u64,
        advertise: bool,
        maintenance: bool,
    ) -> R<Vec<u8>> {
        let padding = Padding::resolve(id.len(), body.len(), 1, None, None)?;
        self.prepare_padded(id, body, now, advertise, maintenance, &padding)
    }
    pub(crate) fn prepare_padded(&mut self, id: &str, body: &[u8], now: u64,
        advertise: bool, maintenance: bool, padding: &Padding) -> R<Vec<u8>> {
        padding.validate(id.len(), body.len())?;
        let mut identity = vec![if maintenance { 5 } else { 0 }];
        identity.extend(lp(id.as_bytes())); identity.extend(lp(body));
        identity.extend(serde_json::to_vec(padding).map_err(|_| "INTEGRATION_QUEUE_ENCODE")?);
        let intent_hash = h(&identity);
        if let Some(f) = self.flights.values().find(|f| !id.is_empty() && f.id == id) {
            if f.intent_hash != intent_hash { return Err("APPLICATION_ID_CONFLICT"); }
            if f.body_hash != h(body) { return Err("APPLICATION_ID_CONFLICT"); }
            return Ok(f.wire.clone());
        }
        if self.completed.contains_key(id) { return Err("APPLICATION_ALREADY_DELIVERED"); }
        if body.len() > 60000 {
            return Err("APPLICATION_CAPACITY");
        }
        self.bounds(65536)?;
        if !maintenance && self.flights.values().filter(|f| !f.id.is_empty()).count() >= 64 {
            return Err("APPLICATION_CAPACITY");
        }
        if maintenance && self.flights.values().any(|f| f.id.is_empty()) {
            return Err("MAINTENANCE_CAPACITY");
        }
        let due = self.demand
            || self.since_boundary >= 4
            || now.saturating_sub(self.last_boundary) >= 900;
        let boundary =
            !(advertise || maintenance && body == [2]) && self.core.owner == self.core.role && (due || self.core.send.is_none());
        if !boundary {
            let e = self.core.send.as_ref().ok_or("WAIT_TURN")?;
            let base = self
                .flights
                .values()
                .filter(|f| f.epoch == e.id)
                .map(|f| f.slot)
                .min()
                .unwrap_or(e.next);
            if e.next.saturating_sub(base) >= WINDOW {
                return Err("SEND_WINDOW");
            }
        } else if self.send.len() + self.recv.len() >= 3 {
            return Err("RECEIPT_CONTEXT_CAPACITY");
        }
        if maintenance && !advertise {
            if let Some(e)=self.core.send.as_ref() {
                let confirmed=self.promises().iter().filter(|c|c.epoch==e.id)
                    .map(|c|c.count).max().unwrap_or(self.send.get(&e.id).ok_or("SEND_EPOCH")?.confirmed);
                // Receiver checks AFTER these carried promises. A stale local
                // confirmation must not block the very carrier that replenishes it.
                if !boundary && (e.next<confirmed || e.next-confirmed>=WINDOW) {
                    return Err("SEND_WINDOW");
                }
            }
            if body==[2] && !self.useful_send_closure
                && !self.promises().iter().any(|c|c.final_epoch) {
                return Err("TRANSACTION_CAPACITY");
            }
        }
        let promises = if advertise {
            Vec::new()
        } else {
            self.promises()
        };
        let payload = if advertise {
            Vec::new()
        } else {
            typed_body_encode(if maintenance { 5 } else { 0 }, id, body, &promises, padding)?
        };
        let prior = self.core.send.as_ref().map(|e| (e.id, e.next));
        let raw = if advertise {
            self.core.advertise()?.1
        } else if boundary {
            let target = self.core.peer.keys().next().copied();
            self.core.boundary(target, &payload)?
        } else {
            self.core.ordinary(0, &payload)?
        };
        let wire = Wire::parse(&raw)?;
        if boundary {
            if let Some((g, n)) = prior {
                self.send.get_mut(&g).ok_or("SEND_EPOCH")?.terminal = Some(n);
            }
            self.send.insert(
                wire.epoch,
                EpochReceipts::new(ReceiptContext::activated(
                    self.core.sid,
                    self.core.role,
                    wire.epoch,
                    wire.dh,
                    &self.core.root,
                )?),
            );
            self.demand = false;
            self.request_sent = false;
            self.since_boundary = 0;
            self.last_boundary = now;
        } else if due {
            self.demand = true;
        }
        self.send.get_mut(&wire.epoch).ok_or("SEND_EPOCH")?.next =
            wire.n.checked_add(1).ok_or("COUNTER_OVERFLOW")?;
        if maintenance && body==[0] {
            self.reserve.as_mut().ok_or("directional_reserve_missing")?
                .record_request(wire.epoch,wire.n,h(&raw))?;
        }
        if boundary {
            self.reserve.as_mut().ok_or("directional_reserve_missing")?
                .grant_peer_epoch=self.core.active_recv;
        }
        self.flights.insert(
            slot_key(wire.epoch, wire.n),
            Flight {
                body_hash: h(body),
                intent_hash,
                epoch: wire.epoch,
                slot: wire.n,
                id: id.to_owned(),
                wire: raw.clone(),
                accepted: false,
                closure_proof: carrier_proof_encode(&promises)?,
            },
        );
        // Only the exact carrier's authenticated receipt may confirm these promises.
        if !maintenance {
            self.since_boundary = self
                .since_boundary
                .checked_add(1)
                .ok_or("COUNTER_OVERFLOW")?;
        }
        self.generation = self
            .generation
            .checked_add(1)
            .ok_or("GENERATION_OVERFLOW")?;
        self.bounds(0)?;
        Ok(raw)
    }
    pub(crate) fn receive(&mut self, peer: &str, raw: &[u8]) -> R<Option<Vec<u8>>> {
        let mut staged = self.clone();
        let result = staged.receive_inner(peer, raw)?;
        *self = staged;
        Ok(result)
    }
    fn receive_inner(&mut self, peer: &str, raw: &[u8]) -> R<Option<Vec<u8>>> {
        if raw.starts_with(b"NDR1") {
            if raw.len() != RECEIPT_LEN {
                return Err("RECEIPT_BINDING");
            }
            let g = u64::from_be_bytes(raw[21..29].try_into().unwrap());
            let n = u32::from_be_bytes(raw[61..65].try_into().unwrap());
            let key = slot_key(g, n);
            let flight = self.flights.get(&key).ok_or("RECEIPT_NOT_OUTSTANDING")?.clone();
            let e = self.send.get_mut(&g).ok_or("RECEIPT_EPOCH")?;
            e.context.verify(self.core.role, n, &flight.wire, raw)?;
            e.admit(n)?;
            if !flight.id.is_empty() {
                if self.completed.len()>=64 { return Err("COMPLETION_CAPACITY"); }
                self.completed.insert(flight.id.clone(),flight.body_hash);
            }
            self.reserve.as_mut().ok_or("directional_reserve_missing")?
                .confirm_request(g,n,h(&flight.wire))?;
            let proof = carrier_proof_decode(&flight.closure_proof)?;
            // No demand/request/cause reset here: this receipt names only its Flight.
            // receive() stages the entire operation; persistence precedes release.
            self.confirm_carrier(&proof)?;
            self.flights.remove(&key);
            self.generation = self
                .generation
                .checked_add(1)
                .ok_or("GENERATION_OVERFLOW")?;
            return Ok(None);
        }
        let wire = Wire::parse(raw)?;
        let key = slot_key(wire.epoch, wire.n);
        if let Some(d) = self.dispositions.get_mut(&key) {
            if d.hash != h(raw) {
                return Err("DISPOSITION_CONFLICT");
            }
            d.response_pending=true; return Ok(Some(d.receipt.clone()));
        }
        if self.recv_floor.is_some_and(|g| wire.epoch <= g)
            || self
                .recv
                .get(&wire.epoch)
                .is_some_and(|e| wire.n < e.confirmed)
        {
            return Err("CLOSED_REPLAY");
        }
        self.bounds(65536)?;
        if wire.kind == 1 && self.send.len() + self.recv.len() >= 3 {
            return Err("RECEIPT_CONTEXT_CAPACITY");
        }
        let vacant_target=self.core.peer.is_empty();
        let (kind, payload) = self.core.receive(raw)?;
        if wire.kind == 1 {
            if wire.previous != u64::MAX {
                self.recv
                    .get_mut(&wire.previous)
                    .ok_or("RECEIPT_PREVIOUS")?
                    .terminal = Some(wire.terminal);
            }
            self.recv.insert(
                wire.epoch,
                EpochReceipts::new(ReceiptContext::activated(
                    self.core.sid,
                    wire.dir,
                    wire.epoch,
                    wire.dh,
                    &self.core.root,
                )?),
            );
        }
        if kind != 2 {
            if kind != 0 {
                return Err("INTEGRATION_KIND");
            }
            let (id, body, maintenance, closures) = body_decode(&payload)?;
            let useful=self.closures_free_nonclosure(&closures)?;
            self.receive_promises(&closures)?;
            if maintenance {
                // Canonical existing maintenance only, no application funding by
                // virtue of sharing its outer boundary or carrying a closure.
                if payload.len()!=1024 || payload.get(7+45*closures.len())!=Some(&1) {
                    return Err("TRANSACTION_CAPACITY");
                }
                let (class,effect)=match body.as_slice() {
                    [0] if wire.kind==0 => (1,true),
                    [1] if wire.kind==1 && wire.n==0 => (2,true),
                    [2] if wire.kind==0 => (3,useful),
                    _ => return Err("TRANSACTION_CAPACITY"),
                };
                self.retain_authenticated_control(class,&wire,raw,effect)?;
            }
            if !maintenance {
                if !self.events.contains_key(&id) && self.events.len() >= 64 {
                    return Err("EVENT_CAPACITY");
                }
                crate::timeline::timeline_validate_projection(peer,&body,&id)?;
                if let Some(old) = self.events.get(&id) {
                    if old.body != body {
                        return Err("APPLICATION_ID_CONFLICT");
                    }
                } else {
                    self.received_reference=Some((id.clone(),wire.epoch,wire.n,h(raw)));
                    self.events.insert(id.clone(), Application { id, body });
                }
            } else if body == [0] {
                if self.reserve.as_ref().ok_or("directional_reserve_missing")?
                    .grant_peer_epoch != Some(wire.epoch) {self.demand=true;}
            }
        }
        if kind==2 {
            self.retain_authenticated_control(4,&wire,raw,
                wire.kind==0 && vacant_target && self.core.peer.len()==1)?;
        }
        let e = self.recv.get_mut(&wire.epoch).ok_or("RECEIPT_EPOCH")?;
        e.admit(wire.n)?;
        let receipt = e.context.construct(self.core.role, wire.n, raw)?;
        self.dispositions.insert(
            key,
            Disposition {
                hash: h(raw),
                receipt: receipt.clone(),
                response_pending: true,
            },
        );
        self.generation = self
            .generation
            .checked_add(1)
            .ok_or("GENERATION_OVERFLOW")?;
        self.bounds(0)?;
        Ok(Some(receipt))
    }
}

impl Transaction {
    // Only recognized scheduler capacity refusals are retryable during receive.
    // Stage control preparation so deferral cannot consume a target, slot, promise
    // or demand. Exact already-sealed flights remain in the original transaction.
    pub(crate) fn control_before_receive(&mut self, now:u64)->R<Option<&'static str>> {
        let mut staged=self.clone();
        match staged.next_control(now) {
            Ok(_) => { *self=staged; Ok(None) },
            Err(error @ ("SEND_WINDOW" | "RECEIPT_CONTEXT_CAPACITY" | "TRANSACTION_CAPACITY" | "MAINTENANCE_CAPACITY")) => Ok(Some(error)),
            Err(error) => Err(error),
        }
    }
    pub(crate) fn next_control(&mut self, now:u64)->R<Option<Vec<u8>>> {
        if self.flights.values().any(|f|f.id.is_empty()){return Ok(None);}
        let can_close=self.useful_send_closure || self.promises().iter().any(|c|c.final_epoch);
        if self.send.len()+self.recv.len()>=3 && !self.promises().is_empty() && can_close {
            return self.prepare("", &[2], now, false, true).map(Some);
        }
        if self.core.owner==self.core.role && (self.core.send.is_none() || self.demand) {
            return self.prepare("", &[1], now, false, true).map(Some);
        }
        if self.core.send.is_none(){return Ok(None);}
        if self.core.local.is_empty(){return self.prepare("", &[], now, true, true).map(Some);}
        if can_close && !self.promises().is_empty() && (self.send.len()+self.recv.len()>=3 || self.send.values().any(|e|e.next.saturating_sub(e.confirmed)>=15)) {
            return self.prepare("", &[2], now, false, true).map(Some);
        }
        if self.demand && !self.request_sent {
            let raw=self.prepare("", &[0], now, false, true)?;self.request_sent=true;return Ok(Some(raw));
        }
        Ok(None)
    }
    pub(crate) fn project(&mut self,peer:&str)->R<()> {

        let mut done=Vec::new();
        for(id,hash) in &self.completed {
            if crate::msgqueue::directional_project_delivered(peer,id,hash)? {done.push(id.clone());}
        }
        for id in done {self.completed.remove(&id);}
        Ok(())
    }
    pub(crate) fn accepted(&mut self,raw:&[u8])->R<()> {
        if raw.starts_with(b"NDR1") {
            if raw.len()!=RECEIPT_LEN {return Err("RECEIPT_BINDING");}
            let g=u64::from_be_bytes(raw[21..29].try_into().unwrap());let n=u32::from_be_bytes(raw[61..65].try_into().unwrap());
            let d=self.dispositions.get_mut(&slot_key(g,n)).ok_or("RESPONSE_MISSING")?;
            if d.receipt!=raw{return Err("RESPONSE_CONFLICT");}d.response_pending=false;return Ok(());
        }
        let wire=Wire::parse(raw)?;
        let f=self.flights.get_mut(&slot_key(wire.epoch,wire.n)).ok_or("FLIGHT_MISSING")?;
        if f.wire!=raw{return Err("FLIGHT_CONFLICT");} f.accepted=true;Ok(())
    }
}

impl Transaction {
    pub(crate) fn project_received(&mut self,peer:&str,out:&std::path::Path,source:crate::model::ConfigSource)->R<usize>{
        let count=self.events.len();
        for event in self.events.values() {
            // Stable safe file identity, independent of untrusted message-id syntax.
            let mut identity=self.core.sid.to_vec();identity.extend(lp(peer.as_bytes()));identity.extend(lp(event.id.as_bytes()));
            let name=format!("recv_{}.bin",crate::hex_encode(&h(&identity)));
            let path=out.join(name);
            match std::fs::symlink_metadata(&path) {
                Ok(meta) => {
                    if !meta.is_file() || std::fs::read(&path).map_err(|_|"directional_output_read")?!=event.body {return Err("directional_output_conflict");}
                }
                Err(e) if e.kind()==std::io::ErrorKind::NotFound => {
                    crate::fs_store::write_atomic(&path,&event.body,source).map_err(|_|"directional_output_write")?;
                }
                Err(_)=>return Err("directional_output_read"),
            }
            crate::timeline::timeline_project_message(peer,"in",&event.body,&event.id)?;
            crate::directional_cut("after_timeline_projection");
        }
        self.events.clear();Ok(count)
    }
}

#[cfg(feature = "na0780-test-hooks")]
pub(crate) fn test_body(mode: &str) -> R<Vec<u8>> {
    if matches!(mode, "body_request" | "body_file_shape" | "body_file_gated") {
        let mut file=vec![0;16];file.extend([0,1,b'i']);file.extend(0u64.to_be_bytes());file.extend(0u32.to_be_bytes());
        file.push(1);file.extend([0;32]);
        let data=br#"{"handle":"synthetic","content_len":1}"#;
        file.extend((data.len() as u32).to_be_bytes());file.extend(data);
        let padding=Padding::resolve(5,file.len(),1,None,None)?;
        let mut body=typed_body_encode(4,"probe",&file,&[],&padding)?;
        match mode {
            "body_request" => body[16+5+31]=255,
            "body_file_shape" => body[16+5+16]=2,
            "body_file_gated" => {},
            _ => unreachable!(),
        }
        return Ok(body);
    }
    let maintenance = matches!(mode, "maintenance" | "body_maintenance");
    let id = if maintenance { "" } else { "probe" };
    let payload: &[u8] = if maintenance { &[2] } else { b"authenticated malformed fixture" };
    let padding = Padding::resolve(id.len(), payload.len(), 1, None, None)?;
    let closures = if mode == "body_closure" {
        vec![Closure { epoch: 0, dh: [0; 32], count: 0, final_epoch: false }]
    } else { Vec::new() };
    let mut body = typed_body_encode(if maintenance { 5 } else { 0 }, id, payload, &closures, &padding)?;
    let size_offset = 12 + id.len() + 45 * closures.len();
    match mode {
        "maintenance" => {},
        "body_profile" => body[3] = b'1',
        "body_kind" => body[4] = 255,
        "body_padding_profile" => body[7 + id.len()] = 0,
        "body_padding_size" => body[7 + id.len()] = 3,
        "body_maintenance" => body[16] = 3,
        "body_length" => body[size_offset..size_offset + 4].copy_from_slice(&0u32.to_be_bytes()),
        "body_payload_length" => body[8 + id.len()..12 + id.len()].copy_from_slice(&u32::MAX.to_be_bytes()),
        "body_padding" => *body.last_mut().unwrap() = 1,
        "body_closure" => body[7 + id.len() + 44] = 2,
        _ => return Err("test_mode"),
    }
    Ok(body)
}

#[cfg(test)]
mod successor_tests {
    use super::*;
    #[test]
    fn padding_reserves_closures_and_exact_size_limits() {
        for profile in 1..=3 {
            for count in 0..=3 {
                let pad = Padding::resolve(3, 900, profile, Some(65536), None).unwrap();
                let closures: Vec<_> = (0..count).map(|epoch| Closure { epoch, dh: [0;32], count: 2, final_epoch: false }).collect();
                let raw = typed_body_encode(0, "msg", &vec![7;900], &closures, &pad).unwrap();
                assert_eq!(raw.len(), pad.size as usize);
                let decoded = typed_body_decode(&raw).unwrap();
                assert_eq!(decoded.payload, vec![7;900]);
                assert_eq!(decoded.closures.len(), count as usize);
            }
        }
        // Hmax = 16+3+135+900 = 1054. The request is exact, never rounded.
        assert!(Padding::resolve(3,900,1,None,Some(1053)).is_err());
        assert_eq!(Padding::resolve(3,900,1,None,Some(1054)).unwrap().size,1054);
        assert_eq!(Padding::resolve(3,900,1,None,Some(1055)).unwrap().size,1055);
        assert!(Padding::resolve(3,900,2,None,Some(1054)).is_err());
        assert_eq!(Padding::resolve(64,59785,1,Some(65536),Some(60000)).unwrap().size,60000);
        assert!(Padding::resolve(64,59786,1,Some(65536),Some(60000)).is_err());
        assert!(Padding::resolve(64,59785,1,Some(65536),None).is_err());
        assert!(Padding::resolve(1,1,1,Some(65537),None).is_err());
        assert!(Padding::resolve(1,1,0,None,None).is_err());
        assert!(Padding::resolve(65,1,1,None,None).is_err());
        assert!(Padding::resolve(1,60001,1,None,None).is_err());
    }
    #[test]
    fn strict_body_rejects_malformed_and_old_shapes() {
        let p=Padding::resolve(1,3,1,None,None).unwrap();
        let raw=typed_body_encode(0,"x",b"abc",&[],&p).unwrap();
        assert_eq!(typed_body_decode(&raw).unwrap().payload,b"abc");
        // Codec input control: zero bytes may belong to the declared payload.
        // This does not model unauthenticated alteration of an AEAD-bound length.
        let mut valid = raw.clone();
        valid[12] = 255;
        let declared = u32::from_be_bytes(valid[9..13].try_into().unwrap()) as usize;
        assert_eq!(declared, 255);
        let mut expected = b"abc".to_vec();
        expected.resize(declared, 0);
        let decoded = typed_body_decode(&valid).unwrap();
        assert_eq!(decoded.kind, 0);
        assert_eq!(decoded.id, "x");
        assert!(decoded.closures.is_empty());
        assert_eq!(decoded.payload, expected);
        let remaining_padding = &valid[17 + declared..];
        assert_eq!(remaining_padding.len(), 752);
        assert!(remaining_padding.iter().all(|byte| *byte == 0));
        assert_eq!(typed_body_encode(0, "x", &decoded.payload, &[], &p).unwrap(), valid);
        for offset in [3,4,5,6,7,8,9,10,16,1023] {
            let mut bad=raw.clone();bad[offset]=255;
            if offset == 10 {
                // This exceeds both available bytes and the 60000-byte payload cap.
                // Padding::resolve rejects at the cap before the payload read;
                // this is not coverage of Reader::take's short-read branch.
                let declared = u32::from_be_bytes(bad[9..13].try_into().unwrap()) as usize;
                assert_eq!(declared, 16_711_683);
                assert!(declared > bad.len() - 17, "negative fixture must exceed available payload bytes");
            }
            assert!(typed_body_decode(&bad).is_err(),"malformed offset {offset}");
        }
        for len in [0,4,7,15,1023] { assert!(typed_body_decode(&raw[..len]).is_err()); }
        let mut trailing=raw.clone();trailing.push(0);assert!(typed_body_decode(&trailing).is_err());
        let mut old=raw.clone();old[3]=b'1';assert!(typed_body_decode(&old).is_err());
        for op in 0..=2 { let raw=typed_body_encode(5,"",&[op],&[],&Padding::resolve(0,1,1,None,None).unwrap()).unwrap(); assert!(body_decode(&raw).unwrap().2); }
        assert!(typed_body_encode(5,"",&[3],&[],&p).is_err());
        assert!(typed_body_encode(5,"x",&[1],&[],&p).is_err());
        assert!(typed_body_encode(0,"",b"abc",&[],&p).is_err());
        assert!(typed_body_encode(0,"\u{e9}",b"abc",&[],&p).is_err());
        let repeated=vec![Closure {epoch:1,dh:[0;32],count:0,final_epoch:false};2];
        assert!(matches!(typed_body_encode(0,"x",b"abc",&repeated,&p),Err("CLOSURE_ORDER")));
        let excess=vec![Closure {epoch:1,dh:[0;32],count:0,final_epoch:false};4];
        assert!(matches!(typed_body_encode(0,"x",b"abc",&excess,&p),Err("CLOSURE_CAPACITY")));
    }
    #[test]
    fn saved_intent_keeps_size_and_rejects_identity_changes() {
        let p=Padding::resolve(2,4,3,None,None).unwrap();
        let intent=QueuedIntent::message("id",b"body",p.clone()).unwrap();
        let raw=intent.encode().unwrap();
        assert_eq!(QueuedIntent::decode(&raw,"id",b"body").unwrap().padding,p);
        assert!(QueuedIntent::decode(&raw,"other",b"body").is_err());
        assert!(QueuedIntent::decode(&raw,"id",b"changed").is_err());
        let mut changed: serde_json::Value=serde_json::from_slice(&raw).unwrap();
        changed["profile"]="NA0780-DIR-INTEGRATION-01".into();
        assert!(QueuedIntent::decode(&serde_json::to_vec(&changed).unwrap(),"id",b"body").is_err());
        changed["profile"]="NA0780-DIR-INTEGRATION-03".into();changed["padding"]["size"]=1024.into();
        assert!(QueuedIntent::decode(&serde_json::to_vec(&changed).unwrap(),"id",b"body").is_err());
    }
}

#[cfg(test)]
pub(crate) fn test_file_body(kind:u8,payload:&[u8]) {
    for profile in 1..=3 {
        let padding=Padding::resolve(1,payload.len(),profile,Some(65536),None).unwrap();
        let raw=typed_body_encode(kind,"i",payload,&[],&padding).unwrap();
        assert_eq!(typed_body_decode(&raw).unwrap().kind,kind);
        assert!(matches!(body_decode(&raw),Err("INTEGRATION_FILE_GATED")));
    }
}

// R02 UNAPPLIED REVIEW: private saved coverage codec. No wire/profile identifier.
// 1 count + 3*(8 epoch +32 DH +4 count +1 final) =136 raw,184 base64 chars.
// The existing typed-body codec and receipt construction remain unchanged.
fn carrier_proof_encode(closures: &[Closure]) -> R<String> {
    use base64::Engine;
    if closures.len() > 3 { return Err("CLOSURE_CAPACITY"); }
    let mut bytes = [0u8; 136];
    bytes[0] = closures.len() as u8;
    let mut previous = None;
    for (i, c) in closures.iter().enumerate() {
        if previous.is_some_and(|g| g >= c.epoch) { return Err("CLOSURE_ORDER"); }
        previous = Some(c.epoch);
        let p = 1 + 45*i;
        bytes[p..p+8].copy_from_slice(&c.epoch.to_be_bytes());
        bytes[p+8..p+40].copy_from_slice(&c.dh);
        bytes[p+40..p+44].copy_from_slice(&c.count.to_be_bytes());
        bytes[p+44] = u8::from(c.final_epoch);
    }
    Ok(base64::engine::general_purpose::STANDARD.encode(bytes))
}
fn carrier_proof_decode(value: &str) -> R<Vec<Closure>> {
    use base64::Engine;
    if value.len() != 184 { return Err("CLOSURE_CAPACITY"); }
    let bytes = base64::engine::general_purpose::STANDARD.decode(value)
        .map_err(|_| "CLOSURE_PREFIX")?;
    if bytes.len()!=136 || bytes[0]>3 { return Err("CLOSURE_CAPACITY"); }
    let mut closures = Vec::new();
    for i in 0..bytes[0] as usize {
        let p=1+45*i;
        if bytes[p+44]>1 { return Err("CLOSURE_FINAL"); }
        closures.push(Closure {
            epoch: u64::from_be_bytes(bytes[p..p+8].try_into().unwrap()),
            dh: bytes[p+8..p+40].try_into().unwrap(),
            count: u32::from_be_bytes(bytes[p+40..p+44].try_into().unwrap()),
            final_epoch: bytes[p+44]!=0,
        });
    }
    if bytes[1+45*closures.len()..].iter().any(|b| *b!=0)
        || carrier_proof_encode(&closures)? != value { return Err("CLOSURE_PREFIX"); }
    Ok(closures)
}

impl Transaction {
    pub(crate) fn check_reserved_cost(&self, peer_future:u64)->R<()> {
        self.bounds(65536)?;
        let reserve=self.reserve.as_ref().ok_or("directional_reserve_missing")?;
        let controls=reserve.control_refs()?.len();
        let receipt_future=36usize.checked_sub(controls).ok_or("TRANSACTION_CAPACITY")?*RECEIPT_LEN;
        let receive_raw=self.dispositions.values().map(|d|d.receipt.len()).sum::<usize>()
            +self.events.values().map(|e|e.body.len()).sum::<usize>();
        let maintenance_future=if self.flights.values().any(|f|f.id.is_empty()) {0}else{2287};
        if receive_raw+receipt_future+65536>MAX_BYTES
            || self.flights.values().map(|f|f.wire.len()).sum::<usize>()+maintenance_future+65536>MAX_BYTES {
            return Err("TRANSACTION_CAPACITY");
        }
        let future=usize::try_from(peer_future).map_err(|_|"TRANSACTION_CAPACITY")?;
        let actual=serde_json::to_vec(self).map_err(|_|"TRANSACTION_ENCODE")?.len();
        // core_context_future already reserves the peer generation's full width.
        if actual.checked_add(future).and_then(|n|n.checked_add(524288))
            .is_none_or(|n|n>MAX_RECORD) {return Err("TRANSACTION_CAPACITY");}
        Ok(())
    }
}

impl Transaction {
    pub(crate) fn hydrate_reserve(&mut self,reserve:crate::protocol_state::SessionControlReserve)->R<()> {
        reserve.validate()?;
        self.reserve=Some(reserve);Ok(())
    }
    pub(crate) fn staged_reserve(&self)->R<crate::protocol_state::SessionControlReserve> {
        self.reserve.clone().ok_or("directional_reserve_missing")
    }
    fn closures_free_nonclosure(&self,closures:&[Closure])->R<bool> {
        let reserve=self.reserve.as_ref().ok_or("directional_reserve_missing")?;
        for c in closures {
            if let Some(e)=self.recv.get(&c.epoch) {
                if c.final_epoch && e.terminal==Some(c.count) {return Ok(true);}
                for key in self.dispositions.keys() {
                    let Some((g,n))=key.split_once(':') else{return Err("directional_owner_invariant");};
                    let g=g.parse::<u64>().map_err(|_|"directional_owner_invariant")?;
                    let n=n.parse::<u32>().map_err(|_|"directional_owner_invariant")?;
                    if g==c.epoch && n<c.count && reserve.control_class(g,n)?!=Some(3) {
                        return Ok(true);
                    }
                }
            }
        }
        Ok(false)
    }
    fn retain_authenticated_control(&mut self,class:u8,wire:&Wire,raw:&[u8],effect:bool)->R<()> {
        let mut reserve=self.reserve.take().ok_or("directional_reserve_missing")?;
        reserve.prune_refs(|g,n,hash|self.dispositions.get(&slot_key(g,n))
            .is_some_and(|d| &d.hash==hash))?;
        reserve.retire_epoch_witnesses(|g|self.recv.contains_key(&g))?;
        reserve.admit_epoch_control(class,wire.epoch,wire.dh,wire.n,h(raw),effect)?;
        let confirmed=self.recv.get(&wire.epoch).ok_or("RECEIPT_EPOCH")?.confirmed;
        reserve.retain_control(class,wire.epoch,wire.n,h(raw),confirmed)?;
        self.reserve=Some(reserve);
        Ok(())
    }
    // The candidate envelope is conditional on the class/span/witness induction.
    // Actual serialized costs, not the old paper constant, debit remaining credit.
    pub(crate) fn control_retained_encoded(&self)->R<usize> {
        let reserve=self.reserve.as_ref().ok_or("directional_reserve_missing")?;
        let mut used=serde_json::to_vec(reserve).map_err(|_|"TRANSACTION_ENCODE")?.len()+11;
        for (_,g,n,hash) in reserve.control_refs()? {
            let key=slot_key(g,n);
            let d=self.dispositions.get(&key).ok_or("directional_owner_invariant")?;
            if d.hash!=hash {return Err("directional_owner_invariant");}
            let one=BTreeMap::from([(key,d)]);
            used=used.checked_add(serde_json::to_vec(&one).map_err(|_|"TRANSACTION_ENCODE")?.len()-1+11)
                .ok_or("TRANSACTION_CAPACITY")?;
        }
        for (key,f) in self.flights.iter().filter(|(_,f)|f.id.is_empty()) {
            let one=BTreeMap::from([(key,f)]);
            used=used.checked_add(serde_json::to_vec(&one).map_err(|_|"TRANSACTION_ENCODE")?.len()-1)
                .ok_or("TRANSACTION_CAPACITY")?;
        }
        // Charging ALL current send holes is conservative; ordinary holes are not
        // allowed to disappear from the accounting when a control reference prunes.
        used=used.checked_add(self.send.values().map(|e|e.holes.len()*11).sum::<usize>())
            .ok_or("TRANSACTION_CAPACITY")?;
        Ok(used)
    }
}

impl Transaction {
    pub(crate) fn hydrate_owner(&mut self,peer:&str,owner:&crate::protocol_state::CapacityOwner)->R<()> {
        let p=owner.peer(peer,&self.core.sid)?;
        self.hydrate_reserve(p.control.clone())?;
        self.useful_send_closure=owner.entries.values().any(|e|e.peer==peer
            && e.sid==self.core.sid && e.direction==self.core.role
            && self.send.get(&e.epoch).is_some_and(|s|e.slot>=s.confirmed && e.slot<s.prefix));
        self.useful_send_closure|=p.control.request_covered(&self.promises()
            .iter().map(|c|(c.epoch,self.send[&c.epoch].confirmed,c.count)).collect::<Vec<_>>())?;
        Ok(())
    }
    pub(crate) fn reconcile_owner(&mut self,peer:&str,before:&Self,
        owner:&mut crate::protocol_state::CapacityOwner)->R<()> {
        use crate::protocol_state::{OwnerEntry,Charge};
        // Only new authenticated receive and locally sealed ordinary work can
        // create an owned entry. Retries find the same immutable operation.
        let mut new=Vec::new();
        if let Some((id,g,n,hash))=&self.received_reference {
            let event=self.events.get(id).ok_or("directional_owner_invariant")?;
            new.push((id.clone(),1-self.core.role,*g,*n,*hash,h(&event.body),event.body.len()));
        }
        for f in self.flights.values().filter(|f|!f.id.is_empty()) {
            // The bound depends on payload length only; Flight retains its exact
            // body hash/intent. Application max60000 remains unchanged.
            new.push((f.id.clone(),self.core.role,f.epoch,f.slot,h(&f.wire),f.body_hash,60000));
        }
        for (id,direction,epoch,slot,wire_hash,content,len) in new {
            let ticket=serde_json::to_string(&(peer,self.core.sid,direction,&id))
                .map_err(|_|"directional_owner_encode")?;
            if let Some(e)=owner.entries.get(&ticket) {
                if e.peer!=peer || e.sid!=self.core.sid || e.direction!=direction
                    || e.operation!=id || e.epoch!=epoch || e.slot!=slot
                    || e.content!=content || e.wire_hash!=wire_hash {
                    return Err("directional_owner_binding");
                }
                continue;
            }
            let projection=crate::timeline::directional_projection_bound(peer,&id,len)? as u64;
            owner.entries.insert(ticket.clone(),OwnerEntry {
                ticket,peer:peer.to_owned(),sid:self.core.sid,direction,operation:id,
                state:0,epoch,slot,reference_state:0,content,generation:0,
                projection,wire_hash,
                charge:Charge {vault_bytes:projection,..Charge::default()},
            });
        }
        for e in owner.entries.values_mut().filter(|e|e.peer==peer && e.sid==self.core.sid) {
            let old=(e.state,e.reference_state,e.projection,e.charge.vault_bytes);
            if e.direction==self.core.role && self.completed.contains_key(&e.operation) {e.state|=2;}
            let projected=if e.direction==self.core.role {
                before.completed.contains_key(&e.operation) && !self.completed.contains_key(&e.operation)
            } else {before.events.contains_key(&e.operation) && !self.events.contains_key(&e.operation)};
            if projected {
                e.charge.vault_bytes=e.charge.vault_bytes.checked_sub(e.projection)
                    .ok_or("directional_owner_invariant")?;
                e.projection=0;e.state|=1;
            }
            let closed=if e.direction==self.core.role {
                self.send_floor.is_some_and(|g|e.epoch<=g)
                    || self.send.get(&e.epoch).is_some_and(|s|e.slot<s.confirmed)
            } else {self.recv_floor.is_some_and(|g|e.epoch<=g)
                || self.recv.get(&e.epoch).is_some_and(|s|e.slot<s.confirmed)};
            if closed {e.reference_state=1;}
            if old!=(e.state,e.reference_state,e.projection,e.charge.vault_bytes) {
                e.generation=e.generation.checked_add(1).ok_or("GENERATION_OVERFLOW")?;
            }
        }
        owner.entries.retain(|_,e| !(e.peer==peer && e.sid==self.core.sid
            && e.state&1!=0 && e.reference_state==1 && e.charge.vault_bytes==0));
        let mut reserve=self.staged_reserve()?;
        reserve.generation=before.generation.checked_add(1).ok_or("GENERATION_OVERFLOW")?;
        reserve.prune_refs(|g,n,hash|self.dispositions.get(&slot_key(g,n)).is_some_and(|d|&d.hash==hash))?;
        reserve.retire_epoch_witnesses(|g|self.recv.contains_key(&g))?;
        let p=owner.peers.get_mut(peer).ok_or("directional_reserve_missing")?;
        p.control=reserve.clone();
        self.reserve=Some(reserve);
        // The authoritative writer refreshes both liabilities AFTER assigning
        // final peer/owner generations. No stale pre-generation future is stored.
        Ok(())
    }
}

impl Transaction {
    // Reuse the unchanged core/context schema calculation: maximum-width Core,
    // three receipt contexts,16 skipped keys, one local/peer target and65536-byte
    // last_out, plus fixed Transaction scalars =288920 encoded bytes. This remains
    // a conditional paper bound until serializer/inductive review, not a new cap.
    // Application/control rows and receipt holes are separately charged.
    pub(crate) fn core_context_future(&self)->R<u64> {
        let mut base=self.clone();
        base.flights.clear();base.dispositions.clear();base.events.clear();base.completed.clear();
        for e in base.send.values_mut().chain(base.recv.values_mut()) {e.holes.clear();}
        let actual=serde_json::to_vec(&base).map_err(|_|"TRANSACTION_ENCODE")?.len() as u64;
        288920u64.checked_sub(actual).ok_or("TRANSACTION_CAPACITY")
    }
}


impl Transaction {
    // Aggregate units are the bytes of JSON text embedded as a JSON string in
    // VaultPayload. This function uses the real serializer at both layers.
    fn outer_json_size<T: Serialize>(value:&T)->R<u64> {
        let text=serde_json::to_string(value).map_err(|_|"TRANSACTION_ENCODE")?;
        Ok(serde_json::to_vec(&text).map_err(|_|"TRANSACTION_ENCODE")?.len() as u64)
    }
    fn reserved_outer_bytes(&self,peer:&crate::protocol_state::PeerReserve)->R<u64> {
        let reserve=self.reserve.as_ref().ok_or("directional_reserve_missing")?;
        let controls: BTreeSet<String>=reserve.control_refs()?.into_iter()
            .map(|(_,g,n,_)|slot_key(g,n)).collect();
        // Pure measurement view, never encoded as a stored format. Remove only
        // reserved material; leave ordinary rows and ordinary receive-hole costs.
        // Existing field names are reused. No schema/profile identifier is chosen.
        let mut residual=serde_json::to_value(self).map_err(|_|"TRANSACTION_ENCODE")?;
        let fields=residual.as_object_mut().ok_or("directional_owner_invariant")?;
        fields.retain(|name,_|matches!(name.as_str(),"flights"|"dispositions"|"events"|"completed"|"recv"));
        fields.get_mut("flights").and_then(|v|v.as_object_mut())
            .ok_or("directional_owner_invariant")?
            .retain(|key,_|self.flights.get(key).is_some_and(|f|!f.id.is_empty()));
        fields.get_mut("dispositions").and_then(|v|v.as_object_mut())
            .ok_or("directional_owner_invariant")?.retain(|key,_|!controls.contains(key));
        let mut ordinary_recv=serde_json::Map::new();
        for (epoch,receipts) in &self.recv {
            let holes:Vec<u32>=receipts.holes.iter().copied()
                .filter(|n|!controls.contains(&slot_key(*epoch,*n))).collect();
            if !holes.is_empty() {
                ordinary_recv.insert(epoch.to_string(),serde_json::json!({"holes":holes}));
            }
        }
        fields.insert("recv".to_owned(),serde_json::Value::Object(ordinary_recv));
        let transaction=Self::outer_json_size(self)?.checked_sub(Self::outer_json_size(&residual)?)
            .ok_or("directional_owner_invariant")?;
        // Owner.control is embedded in the SAME outer string layer as the peer
        // JSON. Subtract complete before/after sizes to include its member/comma.
        let mut without_control=serde_json::to_value(peer).map_err(|_|"TRANSACTION_ENCODE")?;
        without_control.as_object_mut().ok_or("directional_owner_invariant")?.remove("control")
            .ok_or("directional_owner_invariant")?;
        let control=Self::outer_json_size(peer)?.checked_sub(Self::outer_json_size(&without_control)?)
            .ok_or("directional_owner_invariant")?;
        transaction.checked_add(control).ok_or("TRANSACTION_CAPACITY")
    }
    fn reservation_futures(&self,peer:&crate::protocol_state::PeerReserve)->R<(u64,u64)> {
        if peer.control_bound!=36_775 {return Err("directional_owner_invariant");}
        // Inner/raw admission stays independent. No increased control/core bound.
        let fixed=self.core_context_future()?;
        let inner=peer.control_bound.checked_sub(self.control_retained_encoded()? as u64)
            .and_then(|n|n.checked_add(fixed)).ok_or("TRANSACTION_CAPACITY")?;
        // The SAME conservative outer envelope is charged while empty and full:
        // retained contribution R plus remaining liability (C-R) always equals C.
        // 2x applies to the fixed envelope, NEVER to a replenishing inner remainder.
        let envelope=288920u64.checked_add(peer.control_bound).and_then(|n|n.checked_mul(2))
            .ok_or("TRANSACTION_CAPACITY")?;
        let remaining=envelope.checked_sub(self.reserved_outer_bytes(peer)?)
            .ok_or("TRANSACTION_CAPACITY")?;
        let outer=remaining.checked_add(2*524288).ok_or("TRANSACTION_CAPACITY")?;
        Ok((inner,outer))
    }
    pub(crate) fn refresh_reservation(&self,peer:&mut crate::protocol_state::PeerReserve)->R<()> {
        let (inner,outer)=self.reservation_futures(peer)?;
        peer.peer_future=inner;
        peer.vault_future=outer;
        self.check_reserved_cost(inner)
    }
    pub(crate) fn verify_reservation(&self,peer:&crate::protocol_state::PeerReserve)->R<()> {
        let expected=self.reservation_futures(peer)?;
        if peer.generation!=self.generation || peer.control.generation!=self.generation
            || (peer.peer_future,peer.vault_future)!=expected {
            return Err("directional_owner_invariant");
        }
        self.check_reserved_cost(peer.peer_future)
    }
}


#[cfg(test)]
mod r02_intent_profile_tests {
    use super::*;
    #[test]
    fn r02_saved_intent_is_exact_and_old_profile_refuses() {
        let id="0123456789abcdef0123456789abcdef";
        let body=b"ordinary unencrypted enqueue input";
        let intent=QueuedIntent::message(id,body,Padding::resolve(id.len(),body.len(),1,Some(16384),Some(1024)).unwrap()).unwrap();
        let current=intent.encode().unwrap();
        assert_eq!(QueuedIntent::decode(&current,id,body).unwrap().encode().unwrap(),current);
        let before=current.clone();
        let mut old:serde_json::Value=serde_json::from_slice(&current).unwrap();
        old["profile"]=serde_json::json!("NA0780-DIR-INTEGRATION-02");
        assert_eq!(QueuedIntent::decode(&serde_json::to_vec(&old).unwrap(),id,body),Err("APPLICATION_ID_CONFLICT"));
        assert_eq!(current,before);
    }
}
