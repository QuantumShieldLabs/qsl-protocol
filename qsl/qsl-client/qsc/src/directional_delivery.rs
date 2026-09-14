//! Directional delivery transactions and exact-wire receipts.
//! External release requires the authoritative vault commit.
use crate::directional_core::{ectx, h, k, lp, Key, PROFILE, R};
use quantumshield_refimpl::crypto::stdcrypto::StdCrypto;
use quantumshield_refimpl::crypto::traits::Aead;
use serde::{Deserialize, Serialize};
use zeroize::Zeroize;

pub(crate) const INTEGRATION_PROFILE: &[u8] = b"NA0780-DIR-INTEGRATION-01";
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
    epoch: u64,
    slot: u32,
    id: String,
    wire: Vec<u8>,
    accepted: bool,
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
fn body_encode(id: &str, body: &[u8], maintenance: bool, closures: &[Closure]) -> R<Vec<u8>> {
    if closures.len() > 3
        || (!maintenance && (id.is_empty() || id.len() > 64 || !id.is_ascii()))
        || (maintenance && (!id.is_empty() || body.len() != 1 || body[0] > 2))
    {
        return Err("INTEGRATION_BODY");
    }
    let mut out = b"NDI1".to_vec();
    out.push(u8::from(maintenance));
    out.extend(lp(id.as_bytes()));
    out.push(closures.len() as u8);
    for c in closures {
        out.extend(c.epoch.to_be_bytes());
        out.extend(c.dh);
        out.extend(c.count.to_be_bytes());
        out.push(u8::from(c.final_epoch));
    }
    out.extend(lp(body));
    Ok(out)
}
fn body_decode(raw: &[u8]) -> R<(String, Vec<u8>, bool, Vec<Closure>)> {
    struct Reader<'a>(&'a [u8]);
    impl<'a> Reader<'a> {
        fn take(&mut self, n: usize) -> R<&'a [u8]> {
            if n > self.0.len() {
                return Err("INTEGRATION_LENGTH");
            }
            let (a, b) = self.0.split_at(n);
            self.0 = b;
            Ok(a)
        }
        fn byte(&mut self) -> R<u8> {
            Ok(self.take(1)?[0])
        }
        fn lp(&mut self) -> R<&'a [u8]> {
            let n = u32::from_be_bytes(self.take(4)?.try_into().unwrap()) as usize;
            self.take(n)
        }
    }
    let mut r = Reader(raw);
    if r.take(4)? != b"NDI1" {
        return Err("INTEGRATION_PROFILE");
    }
    let kind = r.byte()?;
    if kind > 1 {
        return Err("INTEGRATION_KIND");
    }
    let id = std::str::from_utf8(r.lp()?)
        .map_err(|_| "INTEGRATION_ID")?
        .to_owned();
    let count = r.byte()?;
    if count > 3 {
        return Err("CLOSURE_CAPACITY");
    }
    let mut closures: Vec<Closure> = Vec::new();
    for _ in 0..count {
        let epoch = u64::from_be_bytes(r.take(8)?.try_into().unwrap());
        if closures.last().is_some_and(|c| c.epoch >= epoch) {
            return Err("CLOSURE_ORDER");
        }
        let dh = r.take(32)?.try_into().unwrap();
        let count = u32::from_be_bytes(r.take(4)?.try_into().unwrap());
        let final_byte = r.byte()?;
        if final_byte > 1 {
            return Err("CLOSURE_FINAL");
        }
        closures.push(Closure {
            epoch,
            dh,
            count,
            final_epoch: final_byte == 1,
        });
    }
    let body = r.lp()?.to_vec();
    if !r.0.is_empty() {
        return Err("INTEGRATION_TRAILING");
    }
    // Reuse the exact shape rules; no interpretation of application bytes.
    body_encode(&id, &body, kind == 1, &closures)?;
    Ok((id, body, kind == 1, closures))
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
    fn sent_promises(&mut self, closures: &[Closure]) -> R<()> {
        for c in closures {
            if c.final_epoch {
                if self.send.keys().next().copied() != Some(c.epoch) {
                    return Err("CLOSURE_GAP");
                }
                self.send.remove(&c.epoch);
                self.send_floor = Some(c.epoch);
            } else {
                self.send
                    .get_mut(&c.epoch)
                    .ok_or("CLOSURE_EPOCH")?
                    .confirmed = c.count;
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
        if let Some(f) = self.flights.values().find(|f| !id.is_empty() && f.id == id) {
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
        let promises = if advertise {
            Vec::new()
        } else {
            self.promises()
        };
        let payload = if advertise {
            Vec::new()
        } else {
            body_encode(id, body, maintenance, &promises)?
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
        self.flights.insert(
            slot_key(wire.epoch, wire.n),
            Flight {
                body_hash: h(body),
                epoch: wire.epoch,
                slot: wire.n,
                id: id.to_owned(),
                wire: raw.clone(),
                accepted: false,
            },
        );
        self.sent_promises(&promises)?;
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
        if raw.starts_with(b"NDR1") {
            if raw.len() != RECEIPT_LEN {
                return Err("RECEIPT_BINDING");
            }
            let g = u64::from_be_bytes(raw[21..29].try_into().unwrap());
            let n = u32::from_be_bytes(raw[61..65].try_into().unwrap());
            let key = slot_key(g, n);
            let flight = self.flights.get(&key).ok_or("RECEIPT_NOT_OUTSTANDING")?;
            let e = self.send.get_mut(&g).ok_or("RECEIPT_EPOCH")?;
            e.context.verify(self.core.role, n, &flight.wire, raw)?;
            e.admit(n)?;
            if !flight.id.is_empty() {
                if self.completed.len()>=64 { return Err("COMPLETION_CAPACITY"); }
                self.completed.insert(flight.id.clone(),flight.body_hash);
            }
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
        if self.events.len() >= 64 {
            return Err("EVENT_CAPACITY");
        }
        if wire.kind == 1 && self.send.len() + self.recv.len() >= 3 {
            return Err("RECEIPT_CONTEXT_CAPACITY");
        }
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
            self.receive_promises(&closures)?;
            if !maintenance {
                crate::timeline::timeline_validate_projection(peer,&body,&id)?;
                if let Some(old) = self.events.get(&id) {
                    if old.body != body {
                        return Err("APPLICATION_ID_CONFLICT");
                    }
                } else {
                    self.events.insert(id.clone(), Application { id, body });
                }
            } else if body == [0] {
                self.demand = true;
            }
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
            Err(error @ ("SEND_WINDOW" | "RECEIPT_CONTEXT_CAPACITY")) => Ok(Some(error)),
            Err(error) => Err(error),
        }
    }
    pub(crate) fn next_control(&mut self, now:u64)->R<Option<Vec<u8>>> {
        if self.flights.values().any(|f|f.id.is_empty()){return Ok(None);}
        if self.send.len()+self.recv.len()>=3 && !self.promises().is_empty() {
            return self.prepare("", &[2], now, false, true).map(Some);
        }
        if self.core.owner==self.core.role && (self.core.send.is_none() || self.demand) {
            return self.prepare("", &[1], now, false, true).map(Some);
        }
        if self.core.send.is_none(){return Ok(None);}
        if self.core.local.is_empty(){return self.prepare("", &[], now, true, true).map(Some);}
        if !self.promises().is_empty() && (self.send.len()+self.recv.len()>=3 || self.send.values().any(|e|e.next.saturating_sub(e.confirmed)>=15)) {
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
