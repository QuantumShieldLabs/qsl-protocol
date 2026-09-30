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
    #[serde(with = "crate::strict_json::b64")]
    sid: [u8; 16],
    direction: u8,
    epoch: u64,
    #[serde(with = "crate::strict_json::b64")]
    dh: Key,
    #[serde(with = "crate::strict_json::b64")]
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

// NA-0788 F04/S3b N3 (RULING_NA0788_S3_stop R5): the entry counts Transaction::bounds() states,
// enforced WHILE decoding -- the entry after the limit refuses before its value is built.
// bounds() keeps every check (the combined send + recv 3 and the named 64 / maintenance 1
// split of the flights stay there); dispositions have no stated count and are not counted.
fn at_most_3<'de, D, K, V>(deserializer: D) -> Result<BTreeMap<K, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    K: Deserialize<'de> + Ord,
    V: Deserialize<'de>,
{
    crate::strict_json::unique_map_at_most(deserializer, 3)
}
fn at_most_64<'de, D, K, V>(deserializer: D) -> Result<BTreeMap<K, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    K: Deserialize<'de> + Ord,
    V: Deserialize<'de>,
{
    crate::strict_json::unique_map_at_most(deserializer, 64)
}
fn at_most_65<'de, D, K, V>(deserializer: D) -> Result<BTreeMap<K, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    K: Deserialize<'de> + Ord,
    V: Deserialize<'de>,
{
    crate::strict_json::unique_map_at_most(deserializer, 64 + 1)
}
/// `at_most_64` for a map of byte fields (F04/S5: Transaction.completed), the same count.
fn at_most_64_bytes<'de, D, K, V>(deserializer: D) -> Result<BTreeMap<K, V>, D::Error>
where
    D: serde::Deserializer<'de>,
    K: Deserialize<'de> + Ord,
    V: crate::strict_json::b64::Bytes,
{
    crate::strict_json::b64::unique_map_at_most(deserializer, 64)
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct EpochReceipts {
    context: ReceiptContext,
    next: u32,
    prefix: u32,
    confirmed: u32,
    #[serde(deserialize_with = "crate::strict_json::required")]
    terminal: Option<u32>,
    #[serde(deserialize_with = "crate::strict_json::unique_set")]
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
#[derive(Clone)]
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
/// Flight's version-1 fields (NA-0788 F04/S5 E2): the strict definition behind the schema reader.
#[derive(Serialize, Deserialize)]
#[serde(remote = "Flight", deny_unknown_fields)]
struct FlightV1 {
    #[serde(with = "crate::strict_json::b64")]
    body_hash: Key,
    #[serde(with = "crate::strict_json::b64")]
    intent_hash: Key,
    epoch: u64,
    slot: u32,
    id: String,
    #[serde(with = "crate::strict_json::b64")]
    wire: Vec<u8>,
    accepted: bool,
    closure_proof: String,
}
crate::strict_json::versioned_record!(Flight, FlightV1, deserialize_nested);
#[derive(Clone)]
struct Disposition {
    hash: Key,
    receipt: Vec<u8>,
    response_pending: bool,
}
/// Disposition's version-1 fields (NA-0788 F04/S5 E2).
#[derive(Serialize, Deserialize)]
#[serde(remote = "Disposition", deny_unknown_fields)]
struct DispositionV1 {
    #[serde(with = "crate::strict_json::b64")]
    hash: Key,
    #[serde(with = "crate::strict_json::b64")]
    receipt: Vec<u8>,
    response_pending: bool,
}
crate::strict_json::versioned_record!(Disposition, DispositionV1, deserialize_nested);
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Application {
    id: String,
    #[serde(with = "crate::strict_json::b64")]
    body: Vec<u8>,
}
#[derive(Clone)]
pub(crate) struct Transaction {
    version: String,
    // Transient view hydrated from the fresh authoritative owner by paired update.
    // It is never a third persistent key or a fallback for missing owner state.
    reserve: Option<crate::protocol_state::SessionControlReserve>,
    received_reference: Option<(String, u64, u32, Key)>,
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
/// The Transaction's version-1 fields (NA-0788 F04/S5 E2): the strict definition behind the schema reader;
/// the profile field `version` and its TRANSACTION_PROFILE check are not the schema and stay as they are.
#[derive(Serialize, Deserialize)]
#[serde(remote = "Transaction", deny_unknown_fields)]
struct TransactionV1 {
    version: String,
    #[serde(skip)]
    reserve: Option<crate::protocol_state::SessionControlReserve>,
    #[serde(skip)]
    received_reference: Option<(String,u64,u32,Key)>,
    #[serde(skip)]
    useful_send_closure: bool,
    generation: u64,
    core: Core,
    #[serde(deserialize_with = "at_most_3")]
    send: BTreeMap<u64, EpochReceipts>,
    #[serde(deserialize_with = "at_most_3")]
    recv: BTreeMap<u64, EpochReceipts>,
    #[serde(deserialize_with = "at_most_65")]
    flights: BTreeMap<String, Flight>,
    #[serde(deserialize_with = "crate::strict_json::unique_map")]
    dispositions: BTreeMap<String, Disposition>,
    #[serde(deserialize_with = "at_most_64")]
    events: BTreeMap<String, Application>,
    #[serde(
        serialize_with = "crate::strict_json::b64::serialize_map",
        deserialize_with = "at_most_64_bytes"
    )]
    completed: BTreeMap<String, Key>,
    request_sent: bool,
    #[serde(deserialize_with = "crate::strict_json::required")]
    recv_floor: Option<u64>,
    #[serde(deserialize_with = "crate::strict_json::required")]
    send_floor: Option<u64>,
    demand: bool,
    since_boundary: u32,
    last_boundary: u64,
}
crate::strict_json::versioned_record!(Transaction, TransactionV1, deserialize_top);
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
    #[serde(with = "crate::strict_json::b64")]
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
        if value.profile.as_bytes() != INTEGRATION_PROFILE { return Err("INTENT_PROFILE"); }
        if value.kind != 0 || value.id != id || value.body_hash != h(body) {
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
// The typed-body prefix, named and read in ONE place (F03 S9b): the length cap, the NDI magic
// and the kind byte. typed_body_decode and body_decode's file gate both read it here, so the
// gate cannot drift from the decoder.
const TYPED_BODY_CAP: usize = 60000;
const TYPED_BODY_MAGIC: &[u8; 4] = b"NDI2";
const FILE_KINDS: std::ops::RangeInclusive<u8> = 1..=4;
fn typed_body_prefix(raw: &[u8]) -> R<u8> {
    if raw.len() > TYPED_BODY_CAP {
        return Err("INTEGRATION_LENGTH");
    }
    if raw.get(..4).ok_or("INTEGRATION_LENGTH")? != TYPED_BODY_MAGIC {
        return Err("INTEGRATION_MAGIC");
    }
    raw.get(4).copied().ok_or("INTEGRATION_LENGTH")
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
    let kind = typed_body_prefix(raw)?;
    let mut r = Reader(&raw[5..]);
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
    // Files are gated on the kind byte, before any file-shape validation (C01 T2 H, C06 GT1):
    // the gate reads the prefix through typed_body_prefix, exactly as typed_body_decode does.
    // The check after the decode stays as the backstop.
    if typed_body_prefix(raw).is_ok_and(|kind| FILE_KINDS.contains(&kind)) {
        return Err("INTEGRATION_FILE_GATED");
    }
    let body = typed_body_decode(raw)?;
    if FILE_KINDS.contains(&body.kind) {
        return Err("INTEGRATION_FILE_GATED");
    }
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
        // F04/S5 E2: the schema version is read from the whole text before any field is decoded.
        match crate::strict_json::schema_of(raw) {
            Ok(crate::strict_json::SCHEMA_VERSION) => {}
            Ok(_) => return Err(crate::strict_json::RECORD_VERSION_UNSUPPORTED),
            Err(_) => return Err("TRANSACTION_TAMPERED"),
        }
        // F04/S3b N3: a map over its count stops the decode with the capacity code.
        let value: Self = serde_json::from_str(raw).map_err(|e| {
            if crate::strict_json::is_over_limit(&e) {
                "TRANSACTION_CAPACITY"
            } else if crate::strict_json::is_version_unsupported(&e) {
                crate::strict_json::RECORD_VERSION_UNSUPPORTED
            } else {
                "TRANSACTION_TAMPERED"
            }
        })?;
        if value.version.as_bytes() != INTEGRATION_PROFILE {
            return Err("TRANSACTION_PROFILE");
        }
        value.check_bindings()?;
        value.bounds(0)?;
        Ok(value)
    }
    /// F04/S4b N1 (RULING_NA0788_S4_stop R3): an entry filed under a key that is not its own is
    /// refused. Bound only where every writer forms the key from the value it files: flights
    /// (slot_key of the flight's epoch and slot), events (the application id), send and recv (the
    /// receipt context's epoch) and core.recv (the epoch's id). Not bound: dispositions (the key
    /// is recoverable only from the receipt bytes), completed, core.local, core.peer and
    /// Epoch.skipped (the value does not carry the key).
    fn check_bindings(&self) -> R<()> {
        let bound = self
            .flights
            .iter()
            .all(|(k, f)| *k == slot_key(f.epoch, f.slot))
            && self.events.iter().all(|(k, e)| *k == e.id)
            && self
                .send
                .iter()
                .chain(&self.recv)
                .all(|(k, e)| *k == e.context.epoch)
            && self.core.recv.iter().all(|(k, e)| *k == e.id);
        if !bound {
            return Err("TRANSACTION_TAMPERED");
        }
        Ok(())
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
        // A wrong NDI magic: refused INTEGRATION_MAGIC since S9 (the mode keeps its name).
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
        assert_eq!(QueuedIntent::decode(&serde_json::to_vec(&old).unwrap(),id,body),Err("INTENT_PROFILE"));
        assert_eq!(current,before);
    }
}

// NA-0785 PLAN F03 / S9 -- C01 O9 distinct codes at the decode entry points. e2 lives here
// because no seam outside the crate reaches QueuedIntent::decode (tests/f03_distinct_codes.rs
// drives e1 and e5 through the real receive path, but only under the na0780-test-hooks
// feature: no CI job runs it, so this module is the CI guard for e1, e2 and e5).
#[cfg(test)]
mod f03_s9_distinct_codes_tests {
    use super::*;
    use crate::protocol_state::DirectionalUpdateError::Apply;
    /// A strict kind-0 body re-labelled `kind`: the file kinds then meet file-shape validation.
    fn frame(kind:u8,payload:&[u8])->Vec<u8> {
        let padding=Padding::resolve(1,payload.len(),1,None,None).unwrap();
        let mut raw=typed_body_encode(0,"i",payload,&[],&padding).unwrap();
        raw[4]=kind;raw
    }
    #[test]
    fn s9_e1_wrong_magic_is_integration_magic() {
        let raw=frame(0,b"abc");
        let got:Vec<_>=[*b"NDI1",*b"NDI3",*b"XXXX",[0;4]].iter().map(|magic| {
            let mut bad=raw.clone();bad[..4].copy_from_slice(magic);
            (typed_body_decode(&bad).err(),body_decode(&bad).err())
        }).collect();
        assert_eq!(got,vec![(Some("INTEGRATION_MAGIC"),Some("INTEGRATION_MAGIC"));4]);
        // The magic precedes the file gate; a short frame keeps its length code.
        let mut file=frame(4,b"not a file");file[..4].copy_from_slice(b"NDI1");
        assert_eq!(body_decode(&file).err(),Some("INTEGRATION_MAGIC"));
        assert_eq!(body_decode(&raw[..3]).err(),Some("INTEGRATION_LENGTH"));
        // Disposition kept: an expected non-admission, as INTEGRATION_PROFILE was.
        assert!(Apply("INTEGRATION_MAGIC").expected_non_admission());
    }
    #[test]
    fn s9_e2_foreign_intent_profile_is_intent_profile_checked_first() {
        let raw=QueuedIntent::message("id",b"body",Padding::resolve(2,4,3,None,None).unwrap()).unwrap().encode().unwrap();
        let edit=|change:&dyn Fn(&mut serde_json::Value)| {
            let mut value:serde_json::Value=serde_json::from_slice(&raw).unwrap();change(&mut value);
            serde_json::to_vec(&value).unwrap()
        };
        let foreign=edit(&|v| v["profile"]="NA0780-DIR-INTEGRATION-02".into());
        assert_eq!(QueuedIntent::decode(&foreign,"id",b"body"),Err("INTENT_PROFILE"));
        // Checked FIRST: every other identity fact wrong as well.
        let all=edit(&|v| {v["profile"]="NA0780-DIR-INTEGRATION-02".into();v["kind"]=1.into();});
        assert_eq!(QueuedIntent::decode(&all,"other",b"changed"),Err("INTENT_PROFILE"));
        // kind, id and body hash keep APPLICATION_ID_CONFLICT.
        assert_eq!(QueuedIntent::decode(&edit(&|v| v["kind"]=1.into()),"id",b"body"),Err("APPLICATION_ID_CONFLICT"));
        assert_eq!(QueuedIntent::decode(&raw,"other",b"body"),Err("APPLICATION_ID_CONFLICT"));
        assert_eq!(QueuedIntent::decode(&raw,"id",b"changed"),Err("APPLICATION_ID_CONFLICT"));
        assert!(QueuedIntent::decode(&raw,"id",b"body").is_ok());
    }
    #[test]
    fn s9_e5_file_kinds_are_gated_before_file_shape() {
        let mut request=vec![0u8;68];request[17]=1;request[18]=b'i';request[31]=255;
        let cases=[(b"not a file".to_vec(),"INTEGRATION_FILE_SHAPE"),(request,"INTEGRATION_FILE_REQUEST")];
        let (mut got,mut want)=(Vec::new(),Vec::new());
        for kind in 1..=4u8 {
            for (payload,shape) in &cases {
                let raw=frame(kind,payload);
                // typed_body_decode's other callers keep its file-shape result.
                assert_eq!(typed_body_decode(&raw).err(),Some(*shape),"kind {kind}");
                got.push((kind,body_decode(&raw).err()));want.push((kind,Some("INTEGRATION_FILE_GATED")));
            }
        }
        assert_eq!(got,want);
        // Any later structural refusal of a file-kind frame is gated too; kind 0 keeps its code.
        let mut gated=frame(3,b"not a file");*gated.last_mut().unwrap()=1;
        assert_eq!(body_decode(&gated).err(),Some("INTEGRATION_FILE_GATED"));
        let mut ordinary=frame(0,b"not a file");*ordinary.last_mut().unwrap()=1;
        assert_eq!(body_decode(&ordinary).err(),Some("INTEGRATION_PADDING_NONZERO"));
        assert_eq!(body_decode(&frame(6,b"x")).err(),Some("INTEGRATION_KIND"));
        assert!(body_decode(&frame(0,b"not a file")).is_ok());
        assert!(Apply("INTEGRATION_FILE_GATED").expected_non_admission());
    }
    /// S9b: the gate's edges -- no kind byte, the kind byte alone, and either side of the cap.
    #[test]
    fn s9b_body_decode_prefix_edge_lengths() {
        let prefix = |len: usize| {
            let mut raw = b"NDI2\x01".to_vec();
            raw.resize(len, 0);
            raw
        };
        let got: Vec<_> = [4, 5, 60000, 60001]
            .iter()
            .map(|&len| (len, body_decode(&prefix(len)).err()))
            .collect();
        let want = vec![
            (4, Some("INTEGRATION_LENGTH")),
            (5, Some("INTEGRATION_FILE_GATED")),
            (60000, Some("INTEGRATION_FILE_GATED")),
            (60001, Some("INTEGRATION_LENGTH")),
        ];
        assert_eq!(got, want);
    }
}

// NA-0788 F04/S1: the Transaction tree and the owner decode STRICTLY (C01 T1 rows 15 and 21,
// C07 T6). Each case goes through the real decoder with its existing code, after a control
// arm shows the unmodified record decodes. Key-shaped fixture bytes are drawn at run time.
#[cfg(test)]
mod f04_s1_strict_tests {
    use super::*;
    use crate::directional_core::{Epoch, LocalTarget};
    use crate::protocol_state::{
        CapacityOwner, Charge, OwnerEntry, PeerReserve, SessionControlReserve,
    };
    use rand_core::{OsRng, RngCore};
    use serde_json::Value;

    const TAMPERED: Option<&str> = Some("TRANSACTION_TAMPERED");
    const OWNER_TAMPERED: Option<&str> = Some("directional_owner_tampered");

    fn fresh<const N: usize>() -> [u8; N] {
        std::array::from_fn(|_| OsRng.next_u32() as u8)
    }
    fn opt<T>(none: bool, v: T) -> Option<T> {
        if none {
            None
        } else {
            Some(v)
        }
    }
    fn epoch(id: u64, terminal: Option<u32>) -> Epoch {
        Epoch {
            id,
            dir: 0,
            dh: fresh(),
            ec: fresh(),
            pq: fresh(),
            hk: fresh(),
            adv: fresh(),
            next: 2,
            terminal,
            skipped: BTreeMap::from([(1, fresh())]),
        }
    }
    fn receipts(sid: [u8; 16], epoch: u64, terminal: Option<u32>) -> EpochReceipts {
        let context = ReceiptContext {
            sid,
            direction: 0,
            epoch,
            dh: fresh(),
            key: fresh(),
        };
        EpochReceipts {
            context,
            next: 1,
            prefix: 1,
            confirmed: 0,
            terminal,
            holes: BTreeSet::from([3]),
        }
    }
    /// Every map holds one entry; every Option is Some, or None when `none` is set.
    fn sample(none: bool) -> Transaction {
        let sid = fresh();
        let core = Core {
            sid,
            role: 0,
            root: fresh(),
            seq: 1,
            digest: fresh(),
            owner: 0,
            own_priv: fresh(),
            own_pub: fresh(),
            peer_pub: fresh(),
            send: opt(none, epoch(1, Some(2))),
            recv: BTreeMap::from([(0, epoch(0, opt(none, 1)))]),
            active_recv: opt(none, 0),
            local: BTreeMap::from([(
                0,
                LocalTarget {
                    pk: vec![1],
                    sk: vec![2],
                },
            )]),
            local_next: 1,
            local_consumed_prefix: 0,
            peer: BTreeMap::from([(0, vec![3])]),
            peer_max: 1,
            peer_selected_prefix: 0,
            last_in: opt(none, fresh()),
            last_out: vec![4],
        };
        let flight = Flight {
            body_hash: fresh(),
            intent_hash: fresh(),
            epoch: 1,
            slot: 0,
            id: "m".into(),
            wire: vec![5],
            accepted: false,
            closure_proof: String::new(),
        };
        let disposition = Disposition {
            hash: fresh(),
            receipt: vec![6],
            response_pending: false,
        };
        let event = Application {
            id: "e".into(),
            body: vec![7],
        };
        Transaction {
            version: String::from_utf8(INTEGRATION_PROFILE.to_vec()).unwrap(),
            reserve: None,
            received_reference: None,
            useful_send_closure: false,
            generation: 1,
            core,
            send: BTreeMap::from([(1, receipts(sid, 1, opt(none, 1)))]),
            recv: BTreeMap::from([(0, receipts(sid, 0, opt(none, 0)))]),
            flights: BTreeMap::from([(slot_key(1, 0), flight)]),
            dispositions: BTreeMap::from([(slot_key(0, 0), disposition)]),
            events: BTreeMap::from([("e".into(), event)]),
            completed: BTreeMap::from([("c".into(), fresh())]),
            request_sent: false,
            recv_floor: opt(none, 0),
            send_floor: opt(none, 0),
            demand: false,
            since_boundary: 0,
            last_boundary: 0,
        }
    }
    fn owner(sid: [u8; 16], generation: u64, none: bool) -> CapacityOwner {
        let mut control = SessionControlReserve::fresh(sid);
        control.generation = generation;
        control.grant_peer_epoch = opt(none, 0);
        let peer = PeerReserve {
            peer: "bob".into(),
            sid,
            generation,
            control_bound: 0,
            peer_future: 0,
            vault_future: 0,
            control,
        };
        let entry = OwnerEntry {
            ticket: "t".into(),
            peer: "bob".into(),
            sid,
            direction: 0,
            operation: "o".into(),
            state: 0,
            epoch: 0,
            slot: 0,
            reference_state: 0,
            content: fresh(),
            generation,
            projection: 0,
            wire_hash: fresh(),
            charge: Charge { vault_bytes: 0 },
        };
        CapacityOwner {
            generation,
            peers: BTreeMap::from([("bob".into(), peer)]),
            entries: BTreeMap::from([("t".into(), entry)]),
        }
    }
    fn value<T: Serialize>(v: &T) -> Value {
        serde_json::to_value(v).unwrap()
    }
    fn tx_value() -> Value {
        let v = value(&sample(false));
        assert!(Transaction::decode(&v.to_string()).is_ok(), "control arm");
        v
    }
    fn owner_value() -> Value {
        let v = value(&owner(fresh(), 1, false));
        assert!(CapacityOwner::decode(&v.to_string()).is_ok(), "control arm");
        v
    }
    fn tx(raw: &str) -> Option<&'static str> {
        Transaction::decode(raw).err()
    }
    fn own(raw: &str) -> Option<&'static str> {
        CapacityOwner::decode(raw).err()
    }
    /// The node at `pointer` replaced by raw JSON text: a Value cannot hold a repeated key.
    fn splice(mut v: Value, pointer: &str, raw: &str) -> String {
        *v.pointer_mut(pointer).unwrap() = Value::String("F04S1_SPLICE".into());
        serde_json::to_string(&v)
            .unwrap()
            .replacen("\"F04S1_SPLICE\"", raw, 1)
    }
    /// The map at `pointer` with its entry written twice, identically.
    fn duplicated(v: Value, pointer: &str) -> String {
        let map = v.pointer(pointer).unwrap().as_object().unwrap();
        assert_eq!(map.len(), 1);
        let (k, e) = map.iter().next().unwrap();
        let raw = format!("{{{k}:{e},{k}:{e}}}", k = serde_json::to_string(k).unwrap());
        splice(v.clone(), pointer, &raw)
    }
    fn without(mut v: Value, pointer: &str, field: &str) -> String {
        let node = v.pointer_mut(pointer).unwrap().as_object_mut().unwrap();
        assert!(node.remove(field).is_some(), "{pointer}/{field}");
        v.to_string()
    }
    fn with_null(mut v: Value, pointer: &str, field: &str) -> String {
        let node = v.pointer_mut(pointer).unwrap().as_object_mut().unwrap();
        assert!(
            node.insert(field.into(), Value::Null).is_some(),
            "{pointer}/{field}"
        );
        v.to_string()
    }
    fn with_extra(mut v: Value, pointer: &str) -> String {
        let node = v.pointer_mut(pointer).unwrap().as_object_mut().unwrap();
        assert!(node.insert("extra".into(), Value::from(0)).is_none());
        v.to_string()
    }

    // T-A1: the removed default (response_default_pending) answered TRUE for an absent field.
    #[test]
    fn t_a1_disposition_without_response_pending_refused() {
        let raw = without(tx_value(), "/dispositions/0:0", "response_pending");
        assert_eq!(tx(&raw), TAMPERED);
    }

    // T-A2: one test per map (C01 T1 row 15), each through its real decoder.
    #[test]
    fn t_a2_duplicate_key_transaction_send() {
        assert_eq!(tx(&duplicated(tx_value(), "/send")), TAMPERED);
    }
    #[test]
    fn t_a2_duplicate_key_transaction_recv() {
        assert_eq!(tx(&duplicated(tx_value(), "/recv")), TAMPERED);
    }
    #[test]
    fn t_a2_duplicate_key_transaction_flights() {
        assert_eq!(tx(&duplicated(tx_value(), "/flights")), TAMPERED);
    }
    #[test]
    fn t_a2_duplicate_key_transaction_dispositions() {
        assert_eq!(tx(&duplicated(tx_value(), "/dispositions")), TAMPERED);
    }
    #[test]
    fn t_a2_duplicate_key_transaction_events() {
        assert_eq!(tx(&duplicated(tx_value(), "/events")), TAMPERED);
    }
    #[test]
    fn t_a2_duplicate_key_transaction_completed() {
        assert_eq!(tx(&duplicated(tx_value(), "/completed")), TAMPERED);
    }
    #[test]
    fn t_a2_duplicate_key_core_recv() {
        assert_eq!(tx(&duplicated(tx_value(), "/core/recv")), TAMPERED);
    }
    #[test]
    fn t_a2_duplicate_key_core_local() {
        assert_eq!(tx(&duplicated(tx_value(), "/core/local")), TAMPERED);
    }
    #[test]
    fn t_a2_duplicate_key_core_peer() {
        assert_eq!(tx(&duplicated(tx_value(), "/core/peer")), TAMPERED);
    }
    #[test]
    fn t_a2_duplicate_key_epoch_skipped() {
        assert_eq!(tx(&duplicated(tx_value(), "/core/send/skipped")), TAMPERED);
    }
    #[test]
    fn t_a2_duplicate_key_owner_peers() {
        assert_eq!(own(&duplicated(owner_value(), "/peers")), OWNER_TAMPERED);
    }
    #[test]
    fn t_a2_duplicate_key_owner_entries() {
        assert_eq!(own(&duplicated(owner_value(), "/entries")), OWNER_TAMPERED);
    }
    #[test]
    fn t_a2_escaped_spelling_of_one_key_refused() {
        let v = tx_value();
        let e = v["events"]["e"].to_string();
        let raw = splice(v, "/events", &format!("{{\"e\":{e},\"\\u0065\":{e}}}"));
        assert_eq!(tx(&raw), TAMPERED);
    }

    // T-A3: an unknown field inside core and each nested core type.
    #[test]
    fn t_a3_unknown_field_core() {
        assert_eq!(tx(&with_extra(tx_value(), "/core")), TAMPERED);
    }
    #[test]
    fn t_a3_unknown_field_core_send() {
        assert_eq!(tx(&with_extra(tx_value(), "/core/send")), TAMPERED);
    }
    #[test]
    fn t_a3_unknown_field_core_recv() {
        assert_eq!(tx(&with_extra(tx_value(), "/core/recv/0")), TAMPERED);
    }
    #[test]
    fn t_a3_unknown_field_core_local() {
        assert_eq!(tx(&with_extra(tx_value(), "/core/local/0")), TAMPERED);
    }

    // T-A4: each Option field ABSENT is refused; PRESENT as null decodes as None.
    fn absent_refused_null_accepted(pointer: &str, field: &str) -> Transaction {
        assert_eq!(tx(&without(tx_value(), pointer, field)), TAMPERED);
        Transaction::decode(&with_null(tx_value(), pointer, field)).unwrap()
    }
    #[test]
    fn t_a4_transaction_recv_floor_required_null_accepted() {
        let t = absent_refused_null_accepted("", "recv_floor");
        assert!(t.recv_floor.is_none());
    }
    #[test]
    fn t_a4_transaction_send_floor_required_null_accepted() {
        let t = absent_refused_null_accepted("", "send_floor");
        assert!(t.send_floor.is_none());
    }
    #[test]
    fn t_a4_receipts_terminal_required_null_accepted() {
        let t = absent_refused_null_accepted("/send/1", "terminal");
        assert!(t.send[&1].terminal.is_none());
    }
    #[test]
    fn t_a4_core_send_required_null_accepted() {
        let t = absent_refused_null_accepted("/core", "send");
        assert!(t.core.send.is_none());
    }
    #[test]
    fn t_a4_core_active_recv_required_null_accepted() {
        let t = absent_refused_null_accepted("/core", "active_recv");
        assert!(t.core.active_recv.is_none());
    }
    #[test]
    fn t_a4_core_last_in_required_null_accepted() {
        let t = absent_refused_null_accepted("/core", "last_in");
        assert!(t.core.last_in.is_none());
    }
    #[test]
    fn t_a4_epoch_terminal_required_null_accepted() {
        let t = absent_refused_null_accepted("/core/send", "terminal");
        assert!(t.core.send.unwrap().terminal.is_none());
    }
    #[test]
    fn t_a4_control_grant_peer_epoch_required_null_accepted() {
        let control = "/peers/bob/control";
        let field = "grant_peer_epoch";
        assert_eq!(own(&without(owner_value(), control, field)), OWNER_TAMPERED);
        let decoded = CapacityOwner::decode(&with_null(owner_value(), control, field)).unwrap();
        assert!(decoded.peers["bob"].control.grant_peer_epoch.is_none());
    }

    // T-RT: what the writers produce decodes and re-encodes to the same bytes, nothing lost.
    #[test]
    fn t_rt_records_round_trip_through_the_strict_decoders() {
        for none in [false, true] {
            let raw = sample(none).encode().unwrap();
            assert_eq!(Transaction::decode(&raw).unwrap().encode().unwrap(), raw);
            let raw = serde_json::to_string(&owner(fresh(), 1, none)).unwrap();
            let decoded = CapacityOwner::decode(&raw).unwrap();
            assert_eq!(serde_json::to_string(&decoded).unwrap(), raw);
        }
    }

    // N5 measured: the writers emit every Option field, as null when None.
    #[test]
    fn t_w_writers_emit_every_option_field() {
        let v = value(&sample(true));
        for (pointer, field) in [
            ("", "recv_floor"),
            ("", "send_floor"),
            ("/send/1", "terminal"),
            ("/recv/0", "terminal"),
            ("/core", "send"),
            ("/core", "active_recv"),
            ("/core", "last_in"),
            ("/core/recv/0", "terminal"),
        ] {
            let node = v.pointer(pointer).unwrap().as_object().unwrap();
            assert_eq!(node.get(field), Some(&Value::Null), "{pointer}/{field}");
        }
        let o = value(&owner(fresh(), 1, true));
        let grant = &o["peers"]["bob"]["control"]["grant_peer_epoch"];
        assert_eq!(grant, &Value::Null);
        assert!(Transaction::decode(&v.to_string()).is_ok());
        assert!(CapacityOwner::decode(&o.to_string()).is_ok());
    }

    // T-C3 and T-P8 through a REAL vault open: the refusal keeps its code and writes nothing.
    // Isolated in a child process (the timeline fixture pattern): it sets QSC_CONFIG_DIR.
    const CHILD: &str = "F04S1_VAULT_CHILD";
    #[test]
    fn t_c3_p8_vault_open_refusals_keep_codes_and_file() {
        if std::env::var_os(CHILD).is_none() {
            let name = "directional_delivery::f04_s1_strict_tests::\
                        t_c3_p8_vault_open_refusals_keep_codes_and_file";
            let status = std::process::Command::new(std::env::current_exe().unwrap())
                .args(["--exact", name, "--nocapture"])
                .env(CHILD, "1")
                .status()
                .unwrap();
            assert!(status.success(), "isolated vault fixture failed");
            return;
        }
        use argon2::{Algorithm, Argon2, Params, Version};
        use chacha20poly1305::aead::{Aead as _, KeyInit, Payload};
        use chacha20poly1305::{ChaCha20Poly1305, Nonce};
        let dir = tempfile::tempdir().unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = std::fs::Permissions::from_mode(0o700);
            std::fs::set_permissions(dir.path(), mode).unwrap();
        }
        std::env::set_var("QSC_CONFIG_DIR", dir.path());
        let pass: String = fresh::<16>().iter().map(|b| format!("{b:02x}")).collect();
        crate::vault::vault_init_directional_with_passphrase(&pass).unwrap();
        let path = dir.path().join("vault.qsv");
        let original = std::fs::read(&path).unwrap();
        assert!(crate::vault::open_session_with_passphrase(&pass).is_ok());
        let u32_at = |i: usize| u32::from_le_bytes(original[i..i + 4].try_into().unwrap());
        let params = Params::new(u32_at(9), u32_at(13), u32_at(17), Some(32)).unwrap();
        let mut key: Key = fresh();
        Argon2::new(Algorithm::Argon2id, Version::V0x13, params)
            .hash_password_into(pass.as_bytes(), &original[25..41], &mut key)
            .unwrap();
        let cipher = ChaCha20Poly1305::new(chacha20poly1305::Key::from_slice(&key));
        let sealed = Payload {
            msg: &original[53..],
            aad: &original[..53],
        };
        let plain = cipher
            .decrypt(Nonce::from_slice(&original[41..53]), sealed)
            .unwrap();
        let payload: Value = serde_json::from_slice(&plain).unwrap();
        let layout = crate::protocol_state::approved_directional_layout().unwrap();
        let t = sample(false);
        let sid = t.core.sid;
        let good = value(&t).to_string();
        let grant = "grant_peer_epoch";
        let cases = [
            (
                "stale-generation",
                value(&owner(sid, 2, false)).to_string(),
                good.clone(),
                "directional_stale_generation",
            ),
            (
                "transaction-duplicate-events",
                value(&owner(sid, 1, false)).to_string(),
                duplicated(value(&t), "/events"),
                "TRANSACTION_TAMPERED",
            ),
            (
                "owner-missing-grant_peer_epoch",
                without(value(&owner(sid, 1, false)), "/peers/bob/control", grant),
                good.clone(),
                "directional_owner_tampered",
            ),
            (
                "control-matching-generation",
                value(&owner(sid, 1, false)).to_string(),
                good.clone(),
                "directional_owner_invariant",
            ),
        ];
        for (name, owner_raw, tx_raw, expected) in cases {
            let mut p = payload.clone();
            p["secrets"][layout.owner_key] = Value::String(owner_raw);
            p["secrets"][format!("{}bob", layout.peer_prefix)] = Value::String(tx_raw);
            let bytes = serde_json::to_vec(&p).unwrap();
            let mut header = original[..53].to_vec();
            let ct_len = u32::try_from(bytes.len() + 16).unwrap();
            header[21..25].copy_from_slice(&ct_len.to_le_bytes());
            header[41..53].copy_from_slice(&fresh::<12>());
            let plain = Payload {
                msg: &bytes,
                aad: &header,
            };
            let sealed = cipher
                .encrypt(Nonce::from_slice(&header[41..53]), plain)
                .unwrap();
            let mut raw = header;
            raw.extend(sealed);
            std::fs::write(&path, &raw).unwrap();
            let mtime = std::fs::metadata(&path).unwrap().modified().unwrap();
            let got = crate::vault::open_session_with_passphrase(&pass).err();
            assert_eq!(got, Some(expected), "{name}");
            assert_eq!(std::fs::read(&path).unwrap(), raw, "{name}: file bytes");
            let after = std::fs::metadata(&path).unwrap().modified().unwrap();
            assert_eq!(after, mtime, "{name}: file mtime");
        }
    }
}

// NA-0788 F04/S3b N3: Transaction's entry counts are enforced WHILE decoding. The probe: an unknown
// field placed after every map. At the limit the decode reaches it (TRANSACTION_TAMPERED); one
// over, the decode stops at the map with TRANSACTION_CAPACITY before the probe is reached -- a
// check made only after the decode (bounds()) could not fire first.
#[cfg(test)]
mod f04_s3b_count_tests {
    use super::*;
    use crate::directional_core::{Epoch, LocalTarget};
    use rand_core::{OsRng, RngCore};
    use serde_json::Value;
    use std::cell::Cell;

    fn fresh<const N: usize>() -> [u8; N] {
        std::array::from_fn(|_| OsRng.next_u32() as u8)
    }
    fn epoch(id: u64) -> Epoch {
        Epoch {
            id,
            dir: 0,
            dh: fresh(),
            ec: fresh(),
            pq: fresh(),
            hk: fresh(),
            adv: fresh(),
            next: 2,
            terminal: Some(1),
            skipped: BTreeMap::from([(1, fresh())]),
        }
    }
    fn receipts(sid: [u8; 16], epoch: u64) -> EpochReceipts {
        let context = ReceiptContext {
            sid,
            direction: 0,
            epoch,
            dh: fresh(),
            key: fresh(),
        };
        EpochReceipts {
            context,
            next: 1,
            prefix: 1,
            confirmed: 0,
            terminal: Some(1),
            holes: BTreeSet::from([3]),
        }
    }
    /// A valid record with one entry in every map.
    fn sample() -> Value {
        let sid = fresh();
        let core = Core {
            sid,
            role: 0,
            root: fresh(),
            seq: 1,
            digest: fresh(),
            owner: 0,
            own_priv: fresh(),
            own_pub: fresh(),
            peer_pub: fresh(),
            send: Some(epoch(1)),
            recv: BTreeMap::from([(0, epoch(0))]),
            active_recv: Some(0),
            local: BTreeMap::from([(
                0,
                LocalTarget {
                    pk: vec![1],
                    sk: vec![2],
                },
            )]),
            local_next: 1,
            local_consumed_prefix: 0,
            peer: BTreeMap::from([(0, vec![3])]),
            peer_max: 1,
            peer_selected_prefix: 0,
            last_in: Some(fresh()),
            last_out: vec![4],
        };
        let flight = Flight {
            body_hash: fresh(),
            intent_hash: fresh(),
            epoch: 1,
            slot: 0,
            id: "m".into(),
            wire: vec![5],
            accepted: false,
            closure_proof: String::new(),
        };
        let disposition = Disposition {
            hash: fresh(),
            receipt: vec![6],
            response_pending: false,
        };
        let event = Application {
            id: "e".into(),
            body: vec![7],
        };
        let t = Transaction {
            version: String::from_utf8(INTEGRATION_PROFILE.to_vec()).unwrap(),
            reserve: None,
            received_reference: None,
            useful_send_closure: false,
            generation: 1,
            core,
            send: BTreeMap::from([(1, receipts(sid, 1))]),
            recv: BTreeMap::from([(0, receipts(sid, 0))]),
            flights: BTreeMap::from([(slot_key(1, 0), flight)]),
            dispositions: BTreeMap::from([(slot_key(0, 0), disposition)]),
            events: BTreeMap::from([("e".into(), event)]),
            completed: BTreeMap::from([("c".into(), fresh())]),
            request_sent: false,
            recv_floor: Some(0),
            send_floor: Some(0),
            demand: false,
            since_boundary: 0,
            last_boundary: 0,
        };
        let v = serde_json::to_value(&t).unwrap();
        assert!(Transaction::decode(&v.to_string()).is_ok(), "control arm");
        v
    }
    /// `map` holding `n` copies of its one entry, the i-th under `key(i)` and consistent with it
    /// the way every writer files it (F04/S4b A2): an event's id, a flight's epoch and slot, a
    /// receipt context's epoch. Completed and disposition copies carry no key and are unchanged.
    fn resized(mut v: Value, map: &str, n: usize, key: impl Fn(usize) -> String) -> Value {
        let entries = v[map].as_object_mut().unwrap();
        let entry = entries.values().next().unwrap().clone();
        entries.clear();
        for i in 0..n {
            let (k, mut e) = (key(i), entry.clone());
            match map {
                "events" => e["id"] = Value::from(k.clone()),
                "flights" => {
                    let (epoch, slot) = k.split_once(':').unwrap();
                    e["epoch"] = Value::from(epoch.parse::<u64>().unwrap());
                    e["slot"] = Value::from(slot.parse::<u32>().unwrap());
                }
                "send" | "recv" => e["context"]["epoch"] = Value::from(k.parse::<u64>().unwrap()),
                _ => {}
            }
            entries.insert(k, e);
        }
        v
    }
    fn decode(v: &Value) -> Option<&'static str> {
        Transaction::decode(&v.to_string()).err()
    }
    fn probed(mut v: Value) -> Value {
        let fields = v.as_object_mut().unwrap();
        assert!(fields
            .insert("zz_after_maps".into(), Value::from(0))
            .is_none());
        v
    }
    fn assert_limit(at: Value, over: Value) {
        assert_eq!(decode(&at), None, "at the limit");
        assert_eq!(
            decode(&probed(at)),
            Some("TRANSACTION_TAMPERED"),
            "probe reached"
        );
        assert_eq!(
            decode(&probed(over)),
            Some("TRANSACTION_CAPACITY"),
            "stopped at the map"
        );
    }
    fn named(i: usize) -> String {
        format!("n{i}")
    }

    #[test]
    fn t_n3_events_limit() {
        let v = sample();
        assert_limit(
            resized(v.clone(), "events", 64, named),
            resized(v, "events", 65, named),
        );
    }

    #[test]
    fn t_n3_completed_limit() {
        let v = sample();
        let over = resized(v.clone(), "completed", 65, named);
        assert_limit(resized(v, "completed", 64, named), over);
    }

    #[test]
    fn t_n3_flights_total_limit() {
        let v = sample();
        let with_maintenance = |n: usize| {
            let mut flights = resized(v.clone(), "flights", n, |i| slot_key(1, i as u32));
            flights["flights"]["1:0"]["id"] = Value::from("");
            flights
        };
        assert_limit(with_maintenance(64 + 1), with_maintenance(64 + 2));
    }

    #[test]
    fn t_n3_send_limit() {
        let mut v = sample();
        v["recv"] = Value::Object(Default::default());
        let epochs = |i: usize| (i + 1).to_string();
        assert_limit(
            resized(v.clone(), "send", 3, epochs),
            resized(v, "send", 4, epochs),
        );
    }

    #[test]
    fn t_n3_recv_limit() {
        let mut v = sample();
        v["send"] = Value::Object(Default::default());
        let epochs = |i: usize| i.to_string();
        assert_limit(
            resized(v.clone(), "recv", 3, epochs),
            resized(v, "recv", 4, epochs),
        );
    }

    thread_local! {
        static BUILT: Cell<usize> = const { Cell::new(0) };
    }
    /// A value that counts how many of it were built.
    struct Counted;
    impl<'de> Deserialize<'de> for Counted {
        fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            u8::deserialize(deserializer)?;
            BUILT.with(|b| b.set(b.get() + 1));
            Ok(Counted)
        }
    }
    fn counted(raw: &str, limit: usize) -> (Result<usize, serde_json::Error>, usize) {
        BUILT.with(|b| b.set(0));
        let mut de = serde_json::Deserializer::from_str(raw);
        let map = crate::strict_json::unique_map_at_most::<_, String, Counted>(&mut de, limit);
        (map.map(|m| m.len()), BUILT.with(|b| b.get()))
    }
    fn object(n: usize) -> String {
        let entries: Vec<String> = (0..n).map(|i| format!("\"k{i}\":0")).collect();
        format!("{{{}}}", entries.join(","))
    }

    #[test]
    fn t_n3_counted_map_builds_at_most_limit_values() {
        let (over, built) = counted(&object(65), 64);
        assert!(crate::strict_json::is_over_limit(&over.unwrap_err()));
        assert_eq!(built, 64);
        let (at, built) = counted(&object(64), 64);
        assert_eq!((at.unwrap(), built), (64, 64));
        let (duplicate, _) = counted(r#"{"k":0,"k":0}"#, 64);
        let duplicate = duplicate.unwrap_err();
        assert!(!crate::strict_json::is_over_limit(&duplicate));
        assert!(duplicate.to_string().starts_with("duplicate map key"));
    }

    // R5: dispositions have no stated count (a byte sum only), so none is enforced.
    #[test]
    fn t_n3_uncounted_maps_unchanged() {
        let v = resized(sample(), "dispositions", 200, |i| slot_key(0, i as u32));
        assert_eq!(decode(&v), None);
    }
}

// NA-0788 F04/S4b: semantic validation of the Transaction tree. N1 (RULING_NA0788_S4_stop R3): an entry
// filed under a key that is not its own is refused; the maps whose value does not carry the key stay
// unbound, named. N2: a repeated holes element is refused, not merged. N4: the aggregate check's
// refusals no test pinned, through a real vault open. N5 MV-2 (R5): every count maximum bounds()
// states in one record and one record per byte maximum decode; one more of each refuses.
#[cfg(test)]
mod f04_s4b_semantic_tests {
    use super::*;
    use crate::directional_core::{Epoch, LocalTarget};
    use crate::protocol_state::{
        CapacityOwner, Charge, OwnerEntry, PeerReserve, SessionControlReserve,
    };
    use rand_core::{OsRng, RngCore};
    use serde_json::Value;
    use std::time::Instant;

    const TAMPERED: Option<&str> = Some("TRANSACTION_TAMPERED");
    const CAPACITY: Option<&str> = Some("TRANSACTION_CAPACITY");

    fn fresh<const N: usize>() -> [u8; N] {
        std::array::from_fn(|_| OsRng.next_u32() as u8)
    }
    fn epoch(id: u64) -> Epoch {
        Epoch {
            id,
            dir: 0,
            dh: fresh(),
            ec: fresh(),
            pq: fresh(),
            hk: fresh(),
            adv: fresh(),
            next: 2,
            terminal: Some(1),
            skipped: BTreeMap::from([(1, fresh())]),
        }
    }
    fn receipts(sid: [u8; 16], epoch: u64) -> EpochReceipts {
        let context = ReceiptContext {
            sid,
            direction: 0,
            epoch,
            dh: fresh(),
            key: fresh(),
        };
        EpochReceipts {
            context,
            next: 1,
            prefix: 1,
            confirmed: 0,
            terminal: Some(1),
            holes: BTreeSet::from([3]),
        }
    }
    fn flight(epoch: u64, slot: u32, id: &str) -> Flight {
        Flight {
            body_hash: fresh(),
            intent_hash: fresh(),
            epoch,
            slot,
            id: id.into(),
            wire: vec![5],
            accepted: false,
            closure_proof: String::new(),
        }
    }
    fn event(id: &str) -> Application {
        Application {
            id: id.into(),
            body: vec![7],
        }
    }
    /// A valid record with one entry per map, every key formed the way the writers form it.
    fn sample() -> Transaction {
        let sid = fresh();
        let core = Core {
            sid,
            role: 0,
            root: fresh(),
            seq: 1,
            digest: fresh(),
            owner: 0,
            own_priv: fresh(),
            own_pub: fresh(),
            peer_pub: fresh(),
            send: Some(epoch(1)),
            recv: BTreeMap::from([(0, epoch(0))]),
            active_recv: Some(0),
            local: BTreeMap::from([(
                0,
                LocalTarget {
                    pk: vec![1],
                    sk: vec![2],
                },
            )]),
            local_next: 1,
            local_consumed_prefix: 0,
            peer: BTreeMap::from([(0, vec![3])]),
            peer_max: 1,
            peer_selected_prefix: 0,
            last_in: Some(fresh()),
            last_out: vec![4],
        };
        let disposition = Disposition {
            hash: fresh(),
            receipt: vec![6],
            response_pending: false,
        };
        Transaction {
            version: String::from_utf8(INTEGRATION_PROFILE.to_vec()).unwrap(),
            reserve: None,
            received_reference: None,
            useful_send_closure: false,
            generation: 1,
            core,
            send: BTreeMap::from([(1, receipts(sid, 1))]),
            recv: BTreeMap::from([(0, receipts(sid, 0))]),
            flights: BTreeMap::from([(slot_key(1, 0), flight(1, 0, "m"))]),
            dispositions: BTreeMap::from([(slot_key(0, 0), disposition)]),
            events: BTreeMap::from([("e".into(), event("e"))]),
            completed: BTreeMap::from([("c".into(), fresh())]),
            request_sent: false,
            recv_floor: Some(0),
            send_floor: Some(0),
            demand: false,
            since_boundary: 0,
            last_boundary: 0,
        }
    }
    fn value<T: Serialize>(v: &T) -> Value {
        serde_json::to_value(v).unwrap()
    }
    fn decode(v: &Value) -> Option<&'static str> {
        Transaction::decode(&v.to_string()).err()
    }
    fn control() -> Value {
        let v = value(&sample());
        assert_eq!(decode(&v), None, "control arm");
        v
    }
    /// The entry of the map at `pointer` moved from key `from` to key `to`, its value unchanged.
    fn rekeyed(mut v: Value, pointer: &str, from: &str, to: &str) -> Value {
        let map = v.pointer_mut(pointer).unwrap().as_object_mut().unwrap();
        let entry = map.remove(from).unwrap();
        assert!(map.insert(to.into(), entry).is_none());
        v
    }
    /// Re-keyed AND its value changed to name the new key: the key is its own again.
    fn refiled(v: Value, pointer: &str, from: &str, to: &str, field: &str, own: Value) -> Value {
        let mut v = rekeyed(v, pointer, from, to);
        *v.pointer_mut(&format!("{pointer}/{to}{field}")).unwrap() = own;
        v
    }

    // N1: each bound map -- a foreign key refused, the entry's own key accepted.
    #[test]
    fn t_n1_flights_key_bound() {
        let v = control();
        assert_eq!(
            decode(&rekeyed(v.clone(), "/flights", "1:0", "1:5")),
            TAMPERED
        );
        assert_eq!(
            decode(&rekeyed(v.clone(), "/flights", "1:0", "9:0")),
            TAMPERED
        );
        let own = refiled(v, "/flights", "1:0", "1:5", "/slot", Value::from(5));
        assert_eq!(decode(&own), None);
    }
    #[test]
    fn t_n1_events_key_bound() {
        let v = control();
        assert_eq!(decode(&rekeyed(v.clone(), "/events", "e", "x")), TAMPERED);
        assert_eq!(
            decode(&refiled(v, "/events", "e", "x", "/id", "x".into())),
            None
        );
    }
    #[test]
    fn t_n1_send_key_bound() {
        let v = control();
        assert_eq!(decode(&rekeyed(v.clone(), "/send", "1", "2")), TAMPERED);
        let own = refiled(v, "/send", "1", "2", "/context/epoch", Value::from(2));
        assert_eq!(decode(&own), None);
    }
    #[test]
    fn t_n1_recv_key_bound() {
        let v = control();
        assert_eq!(decode(&rekeyed(v.clone(), "/recv", "0", "2")), TAMPERED);
        let own = refiled(v, "/recv", "0", "2", "/context/epoch", Value::from(2));
        assert_eq!(decode(&own), None);
    }
    #[test]
    fn t_n1_core_recv_key_bound() {
        let v = control();
        assert_eq!(
            decode(&rekeyed(v.clone(), "/core/recv", "0", "5")),
            TAMPERED
        );
        let own = refiled(v, "/core/recv", "0", "5", "/id", Value::from(5));
        assert_eq!(decode(&own), None);
    }
    // R3: no binding where the value does not carry its key (named, not an oversight).
    #[test]
    fn t_n1_unbound_maps_named() {
        let v = control();
        for (pointer, from, to) in [
            ("/dispositions", "0:0", "7:9"),
            ("/completed", "c", "d"),
            ("/core/local", "0", "4"),
            ("/core/peer", "0", "4"),
            ("/core/send/skipped", "1", "9"),
        ] {
            assert_eq!(
                decode(&rekeyed(v.clone(), pointer, from, to)),
                None,
                "{pointer}"
            );
        }
    }

    // N2 (RULING_NA0788_S1 R3): a repeated element refused, distinct elements accepted.
    #[test]
    fn t_n2_holes_repeat_refused() {
        for pointer in ["/send/1/holes", "/recv/0/holes"] {
            let mut repeated = control();
            *repeated.pointer_mut(pointer).unwrap() = serde_json::json!([3, 3]);
            assert_eq!(decode(&repeated), TAMPERED, "{pointer}");
            let mut distinct = control();
            *distinct.pointer_mut(pointer).unwrap() = serde_json::json!([2, 3]);
            let t = Transaction::decode(&distinct.to_string()).unwrap();
            let holes = if pointer.starts_with("/send") {
                &t.send[&1]
            } else {
                &t.recv[&0]
            };
            assert_eq!(holes.holes, BTreeSet::from([2, 3]), "{pointer}");
        }
        let mut de = serde_json::Deserializer::from_str("[1,1]");
        let err = crate::strict_json::unique_set::<_, u32>(&mut de).unwrap_err();
        assert!(err.to_string().starts_with("duplicate set element"));
    }

    fn peer_reserve(peer: &str, sid: [u8; 16], generation: u64) -> PeerReserve {
        let mut control = SessionControlReserve::fresh(sid);
        control.generation = generation;
        control.grant_peer_epoch = Some(0);
        PeerReserve {
            peer: peer.into(),
            sid,
            generation,
            control_bound: 0,
            peer_future: 0,
            vault_future: 0,
            control,
        }
    }
    fn owner_entry(peer: &str, sid: [u8; 16], generation: u64) -> OwnerEntry {
        OwnerEntry {
            ticket: "t".into(),
            peer: peer.into(),
            sid,
            direction: 0,
            operation: "o".into(),
            state: 0,
            epoch: 0,
            slot: 0,
            reference_state: 0,
            content: fresh(),
            generation,
            projection: 0,
            wire_hash: fresh(),
            charge: Charge { vault_bytes: 0 },
        }
    }
    fn owner(peers: Vec<PeerReserve>, entries: Vec<OwnerEntry>) -> String {
        serde_json::to_string(&CapacityOwner {
            generation: 1,
            peers: peers.into_iter().map(|p| (p.peer.clone(), p)).collect(),
            entries: entries.into_iter().map(|e| (e.ticket.clone(), e)).collect(),
        })
        .unwrap()
    }

    // N4 and T-P8 through a REAL vault open: each refusal of check_directional_aggregate that no test
    // pinned keeps its code and writes nothing. Isolated in a child process (the S1 T-C3 pattern).
    const CHILD: &str = "F04S4B_VAULT_CHILD";
    #[test]
    fn t_n4_aggregate_pins_through_real_open() {
        if std::env::var_os(CHILD).is_none() {
            let name = "directional_delivery::f04_s4b_semantic_tests::\
                        t_n4_aggregate_pins_through_real_open";
            let status = std::process::Command::new(std::env::current_exe().unwrap())
                .args(["--exact", name, "--nocapture"])
                .env(CHILD, "1")
                .status()
                .unwrap();
            assert!(status.success(), "isolated vault fixture failed");
            return;
        }
        use argon2::{Algorithm, Argon2, Params, Version};
        use chacha20poly1305::aead::{Aead as _, KeyInit, Payload};
        use chacha20poly1305::{ChaCha20Poly1305, Nonce};
        let dir = tempfile::tempdir().unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = std::fs::Permissions::from_mode(0o700);
            std::fs::set_permissions(dir.path(), mode).unwrap();
        }
        std::env::set_var("QSC_CONFIG_DIR", dir.path());
        let pass: String = fresh::<16>().iter().map(|b| format!("{b:02x}")).collect();
        crate::vault::vault_init_directional_with_passphrase(&pass).unwrap();
        let path = dir.path().join("vault.qsv");
        let original = std::fs::read(&path).unwrap();
        assert!(crate::vault::open_session_with_passphrase(&pass).is_ok());
        let u32_at = |i: usize| u32::from_le_bytes(original[i..i + 4].try_into().unwrap());
        let params = Params::new(u32_at(9), u32_at(13), u32_at(17), Some(32)).unwrap();
        let mut key: Key = fresh();
        Argon2::new(Algorithm::Argon2id, Version::V0x13, params)
            .hash_password_into(pass.as_bytes(), &original[25..41], &mut key)
            .unwrap();
        let cipher = ChaCha20Poly1305::new(chacha20poly1305::Key::from_slice(&key));
        let sealed = Payload {
            msg: &original[53..],
            aad: &original[..53],
        };
        let plain = cipher
            .decrypt(Nonce::from_slice(&original[41..53]), sealed)
            .unwrap();
        let payload: Value = serde_json::from_slice(&plain).unwrap();
        let layout = crate::protocol_state::approved_directional_layout().unwrap();
        let t = sample();
        let sid = t.core.sid;
        let good = value(&t).to_string();
        let foreign = rekeyed(value(&t), "/flights", "1:0", "1:5").to_string();
        let bob = || peer_reserve("bob", sid, 1);
        let entry = || owner_entry("bob", sid, 1);
        let cases = [
            (
                "P1 peer name not a single channel",
                owner(vec![peer_reserve("b#c", sid, 1)], vec![]),
                None,
                "directional_single_channel_required",
            ),
            (
                "P2 peer name not a channel label",
                owner(vec![peer_reserve("b c", sid, 1)], vec![]),
                None,
                "directional_peer_invalid",
            ),
            (
                "P3 owned peer without its transaction",
                owner(vec![bob()], vec![]),
                None,
                "directional_reserve_missing",
            ),
            (
                "P4 owner's peer sid is not the transaction's",
                owner(vec![peer_reserve("bob", fresh(), 1)], vec![]),
                Some(good.clone()),
                "directional_owner_binding",
            ),
            (
                "P5 owner entry for a peer the owner does not hold",
                owner(vec![], vec![entry()]),
                None,
                "directional_reserve_missing",
            ),
            (
                "P6 a flight filed under a foreign key (S4b N1)",
                owner(vec![bob()], vec![entry()]),
                Some(foreign),
                "TRANSACTION_TAMPERED",
            ),
            (
                "P6c control: the same owner with the transaction's own keys",
                owner(vec![bob()], vec![entry()]),
                Some(good),
                "directional_owner_invariant",
            ),
        ];
        for (name, owner_raw, tx_raw, expected) in cases {
            let mut p = payload.clone();
            p["secrets"][layout.owner_key] = Value::String(owner_raw);
            if let Some(tx_raw) = tx_raw {
                p["secrets"][format!("{}bob", layout.peer_prefix)] = Value::String(tx_raw);
            }
            let bytes = serde_json::to_vec(&p).unwrap();
            let mut header = original[..53].to_vec();
            let ct_len = u32::try_from(bytes.len() + 16).unwrap();
            header[21..25].copy_from_slice(&ct_len.to_le_bytes());
            header[41..53].copy_from_slice(&fresh::<12>());
            let plain = Payload {
                msg: &bytes,
                aad: &header,
            };
            let sealed = cipher
                .encrypt(Nonce::from_slice(&header[41..53]), plain)
                .unwrap();
            let mut raw = header;
            raw.extend(sealed);
            std::fs::write(&path, &raw).unwrap();
            let mtime = std::fs::metadata(&path).unwrap().modified().unwrap();
            let got = crate::vault::open_session_with_passphrase(&pass).err();
            assert_eq!(got, Some(expected), "{name}");
            assert_eq!(std::fs::read(&path).unwrap(), raw, "{name}: file bytes");
            let after = std::fs::metadata(&path).unwrap().modified().unwrap();
            assert_eq!(after, mtime, "{name}: file mtime");
        }
    }

    // N5 MV-2 (R5): the limits are not a trap -- a record AT each maximum decodes and re-encodes to
    // itself; one more refuses with the capacity code. Build times are printed for the record.
    fn assert_round_trip(t: &Transaction) {
        let raw = t.encode().unwrap();
        assert_eq!(Transaction::decode(&raw).unwrap().encode().unwrap(), raw);
    }
    fn refused(t: &Transaction) -> Option<&'static str> {
        Transaction::decode(&serde_json::to_string(t).unwrap()).err()
    }
    fn timed(name: &str, started: Instant) {
        println!("F04S4B_FIXTURE {name} ms={}", started.elapsed().as_millis());
    }
    /// Every count bounds() states, at its maximum in ONE record: send + recv 3, flights 64 named +
    /// 1 maintenance, events 64, completed 64.
    fn count_maxima() -> Transaction {
        let mut t = sample();
        let sid = t.core.sid;
        t.send = BTreeMap::from([(1, receipts(sid, 1)), (2, receipts(sid, 2))]);
        t.recv = BTreeMap::from([(0, receipts(sid, 0))]);
        t.flights = (0..64)
            .map(|i| (slot_key(1, i), flight(1, i, &format!("m{i}"))))
            .collect();
        t.flights.insert(slot_key(2, 0), flight(2, 0, ""));
        t.events = (0..64)
            .map(|i| (format!("e{i}"), event(&format!("e{i}"))))
            .collect();
        t.completed = (0..64).map(|i| (format!("c{i}"), fresh())).collect();
        t
    }
    #[test]
    fn t_n5_mv2_count_maxima() {
        let started = Instant::now();
        let at = count_maxima();
        assert_round_trip(&at);
        let sid = at.core.sid;
        let mut send_recv = at.clone();
        send_recv.recv.insert(3, receipts(sid, 3));
        let mut named = at.clone();
        named.flights.get_mut("2:0").unwrap().id = "m64".into();
        let mut maintenance = at.clone();
        maintenance.flights.get_mut("1:0").unwrap().id = String::new();
        let mut flights = at.clone();
        flights
            .flights
            .insert(slot_key(1, 64), flight(1, 64, "m64"));
        let mut events = at.clone();
        events.events.insert("e64".into(), event("e64"));
        let mut completed = at.clone();
        completed.completed.insert("c64".into(), fresh());
        for (name, over) in [
            ("send + recv 4", send_recv),
            ("named flights 65", named),
            ("maintenance flights 2", maintenance),
            ("flights 66", flights),
            ("events 65", events),
            ("completed 65", completed),
        ] {
            assert_eq!(refused(&over), CAPACITY, "{name}");
        }
        timed("t_n5_mv2_count_maxima", started);
    }
    #[test]
    fn t_n5_mv2_wire_bytes_maximum() {
        let started = Instant::now();
        let mut at = sample();
        at.flights.get_mut("1:0").unwrap().wire = vec![0; MAX_BYTES];
        assert_round_trip(&at);
        let mut over = at.clone();
        over.flights.get_mut("1:0").unwrap().wire.push(0);
        assert_eq!(refused(&over), CAPACITY);
        timed("t_n5_mv2_wire_bytes_maximum", started);
    }
    #[test]
    fn t_n5_mv2_receipt_event_bytes_maximum() {
        let started = Instant::now();
        let mut at = sample();
        let body = at.events["e"].body.len();
        at.dispositions.get_mut("0:0").unwrap().receipt = vec![0; MAX_BYTES - body];
        assert_round_trip(&at);
        let mut over = at.clone();
        over.events.get_mut("e").unwrap().body.push(0);
        assert_eq!(refused(&over), CAPACITY);
        timed("t_n5_mv2_receipt_event_bytes_maximum", started);
    }
    #[test]
    fn t_n5_mv2_record_bytes_maximum() {
        let started = Instant::now();
        let mut at = sample();
        let base = at.encode().unwrap().len();
        at.flights.get_mut("1:0").unwrap().closure_proof = "p".repeat(MAX_RECORD - base);
        let raw = at.encode().unwrap();
        assert_eq!(raw.len(), MAX_RECORD);
        assert_eq!(Transaction::decode(&raw).unwrap().encode().unwrap(), raw);
        let mut over = at.clone();
        over.flights.get_mut("1:0").unwrap().closure_proof.push('p');
        assert_eq!(over.encode().err(), CAPACITY);
        assert_eq!(refused(&over), CAPACITY);
        timed("t_n5_mv2_record_bytes_maximum", started);
    }
}

// NA-0788 F04/S5: every byte field at rest is canonical base64 (E1) and the four records carry a schema
// version read before field strictness (E2). Each case goes through the real decoder with its code.
#[cfg(test)]
mod f04_s5_encoding_tests {
    use super::*;
    use crate::directional_core::{Epoch, LocalTarget};
    use crate::protocol_state::{
        CapacityOwner, Charge, OwnerEntry, PeerReserve, SessionControlReserve,
    };
    use crate::strict_json::{RECORD_VERSION_UNSUPPORTED, SCHEMA_VERSION};
    use base64::Engine as _;
    use rand_core::{OsRng, RngCore};
    use serde_json::Value;

    const TAMPERED: Option<&str> = Some("TRANSACTION_TAMPERED");
    const OWNER_TAMPERED: Option<&str> = Some("directional_owner_tampered");
    const VERSION: Option<&str> = Some(RECORD_VERSION_UNSUPPORTED);
    const INTENT_INVALID: Option<&str> = Some("INTEGRATION_QUEUE_INVALID");

    fn fresh<const N: usize>() -> [u8; N] {
        std::array::from_fn(|_| OsRng.next_u32() as u8)
    }
    fn epoch(id: u64) -> Epoch {
        Epoch {
            id,
            dir: 0,
            dh: fresh(),
            ec: fresh(),
            pq: fresh(),
            hk: fresh(),
            adv: fresh(),
            next: 2,
            terminal: Some(1),
            skipped: BTreeMap::from([(1, fresh())]),
        }
    }
    fn receipts(sid: [u8; 16], epoch: u64) -> EpochReceipts {
        let context = ReceiptContext {
            sid,
            direction: 0,
            epoch,
            dh: fresh(),
            key: fresh(),
        };
        EpochReceipts {
            context,
            next: 1,
            prefix: 1,
            confirmed: 0,
            terminal: Some(1),
            holes: BTreeSet::from([3]),
        }
    }
    fn flight(epoch: u64, slot: u32, id: &str) -> Flight {
        Flight {
            body_hash: fresh(),
            intent_hash: fresh(),
            epoch,
            slot,
            id: id.into(),
            wire: vec![5],
            accepted: false,
            closure_proof: String::new(),
        }
    }
    fn event(id: &str) -> Application {
        Application {
            id: id.into(),
            body: vec![7],
        }
    }
    /// A valid record with one entry per map, every key formed the way the writers form it.
    fn sample() -> Transaction {
        let sid = fresh();
        let core = Core {
            sid,
            role: 0,
            root: fresh(),
            seq: 1,
            digest: fresh(),
            owner: 0,
            own_priv: fresh(),
            own_pub: fresh(),
            peer_pub: fresh(),
            send: Some(epoch(1)),
            recv: BTreeMap::from([(0, epoch(0))]),
            active_recv: Some(0),
            local: BTreeMap::from([(
                0,
                LocalTarget {
                    pk: vec![1],
                    sk: vec![2],
                },
            )]),
            local_next: 1,
            local_consumed_prefix: 0,
            peer: BTreeMap::from([(0, vec![3])]),
            peer_max: 1,
            peer_selected_prefix: 0,
            last_in: Some(fresh()),
            last_out: vec![4],
        };
        let disposition = Disposition {
            hash: fresh(),
            receipt: vec![6],
            response_pending: false,
        };
        Transaction {
            version: String::from_utf8(INTEGRATION_PROFILE.to_vec()).unwrap(),
            reserve: None,
            received_reference: None,
            useful_send_closure: false,
            generation: 1,
            core,
            send: BTreeMap::from([(1, receipts(sid, 1))]),
            recv: BTreeMap::from([(0, receipts(sid, 0))]),
            flights: BTreeMap::from([(slot_key(1, 0), flight(1, 0, "m"))]),
            dispositions: BTreeMap::from([(slot_key(0, 0), disposition)]),
            events: BTreeMap::from([("e".into(), event("e"))]),
            completed: BTreeMap::from([("c".into(), fresh())]),
            request_sent: false,
            recv_floor: Some(0),
            send_floor: Some(0),
            demand: false,
            since_boundary: 0,
            last_boundary: 0,
        }
    }
    fn owner() -> CapacityOwner {
        let sid = fresh();
        let mut control = SessionControlReserve::fresh(sid);
        control.generation = 1;
        control.grant_peer_epoch = Some(0);
        let peer = PeerReserve {
            peer: "bob".into(),
            sid,
            generation: 1,
            control_bound: 0,
            peer_future: 0,
            vault_future: 0,
            control,
        };
        let entry = OwnerEntry {
            ticket: "t".into(),
            peer: "bob".into(),
            sid,
            direction: 0,
            operation: "o".into(),
            state: 0,
            epoch: 0,
            slot: 0,
            reference_state: 0,
            content: fresh(),
            generation: 1,
            projection: 0,
            wire_hash: fresh(),
            charge: Charge { vault_bytes: 0 },
        };
        CapacityOwner {
            generation: 1,
            peers: BTreeMap::from([("bob".into(), peer)]),
            entries: BTreeMap::from([("t".into(), entry)]),
        }
    }
    fn value<T: Serialize>(v: &T) -> Value {
        serde_json::to_value(v).unwrap()
    }
    fn decode(v: &Value) -> Option<&'static str> {
        Transaction::decode(&v.to_string()).err()
    }
    fn own(v: &Value) -> Option<&'static str> {
        CapacityOwner::decode(&v.to_string()).err()
    }
    fn control() -> Value {
        let v = value(&sample());
        assert_eq!(decode(&v), None, "control arm");
        v
    }
    fn owner_control() -> Value {
        let v = value(&owner());
        assert_eq!(own(&v), None, "control arm");
        v
    }
    fn set(mut v: Value, pointer: &str, to: Value) -> Value {
        *v.pointer_mut(pointer).unwrap() = to;
        v
    }
    fn canonical(text: &str) -> Option<Vec<u8>> {
        let engine = base64::engine::general_purpose::STANDARD;
        let bytes = engine.decode(text).ok()?;
        (engine.encode(&bytes) == text).then_some(bytes)
    }
    fn as_array(bytes: &[u8]) -> Value {
        Value::Array(bytes.iter().map(|b| Value::from(*b)).collect())
    }

    /// The byte-field census of the Transaction tree, by JSON pointer, with the fixed length
    /// (None = variable); every writer-produced value must be a canonical base64 string here.
    const TX_BYTE_FIELDS: &[(&str, Option<usize>)] = &[
        ("/core/sid", Some(16)),
        ("/core/root", Some(32)),
        ("/core/digest", Some(32)),
        ("/core/own_priv", Some(32)),
        ("/core/own_pub", Some(32)),
        ("/core/peer_pub", Some(32)),
        ("/core/last_in", Some(32)),
        ("/core/last_out", None),
        ("/core/send/dh", Some(32)),
        ("/core/send/ec", Some(32)),
        ("/core/send/pq", Some(32)),
        ("/core/send/hk", Some(32)),
        ("/core/send/adv", Some(32)),
        ("/core/send/skipped/1", Some(32)),
        ("/core/recv/0/dh", Some(32)),
        ("/core/recv/0/ec", Some(32)),
        ("/core/recv/0/pq", Some(32)),
        ("/core/recv/0/hk", Some(32)),
        ("/core/recv/0/adv", Some(32)),
        ("/core/recv/0/skipped/1", Some(32)),
        ("/core/local/0/pk", None),
        ("/core/local/0/sk", None),
        ("/core/peer/0", None),
        ("/send/1/context/sid", Some(16)),
        ("/send/1/context/dh", Some(32)),
        ("/send/1/context/key", Some(32)),
        ("/recv/0/context/sid", Some(16)),
        ("/recv/0/context/dh", Some(32)),
        ("/recv/0/context/key", Some(32)),
        ("/flights/1:0/body_hash", Some(32)),
        ("/flights/1:0/intent_hash", Some(32)),
        ("/flights/1:0/wire", None),
        ("/dispositions/0:0/hash", Some(32)),
        ("/dispositions/0:0/receipt", None),
        ("/events/e/body", None),
        ("/completed/c", Some(32)),
    ];
    const OWNER_BYTE_FIELDS: &[(&str, Option<usize>)] = &[
        ("/peers/bob/sid", Some(16)),
        ("/entries/t/sid", Some(16)),
        ("/entries/t/content", Some(32)),
        ("/entries/t/wire_hash", Some(32)),
    ];

    fn assert_fields(
        v: &Value,
        fields: &[(&str, Option<usize>)],
        refuse: impl Fn(&Value) -> Option<&'static str>,
        code: Option<&'static str>,
    ) {
        for (pointer, fixed) in fields {
            let text = v
                .pointer(pointer)
                .and_then(Value::as_str)
                .unwrap_or_else(|| panic!("{pointer}: not a string"));
            let bytes = canonical(text).unwrap_or_else(|| panic!("{pointer}: not canonical"));
            if let Some(n) = fixed {
                assert_eq!(bytes.len(), *n, "{pointer}");
            }
            assert_eq!(
                refuse(&set(v.clone(), pointer, as_array(&bytes))),
                code,
                "{pointer} as a number array"
            );
        }
    }

    // E1: every census field is a canonical base64 string; the same bytes as a number array are refused.
    #[test]
    fn t_e1_transaction_byte_fields_canonical_and_number_arrays_refused() {
        assert_fields(&control(), TX_BYTE_FIELDS, decode, TAMPERED);
    }
    #[test]
    fn t_e1_owner_byte_fields_canonical_and_number_arrays_refused() {
        assert_fields(&owner_control(), OWNER_BYTE_FIELDS, own, OWNER_TAMPERED);
    }
    #[test]
    fn t_e1_queued_intent_body_hash_canonical_and_number_array_refused() {
        let padding = Padding::resolve(2, 4, 3, None, None).unwrap();
        let raw = QueuedIntent::message("id", b"body", padding)
            .unwrap()
            .encode()
            .unwrap();
        let v: Value = serde_json::from_slice(&raw).unwrap();
        let bytes = canonical(v["body_hash"].as_str().unwrap()).unwrap();
        assert_eq!(bytes.len(), 32);
        assert!(QueuedIntent::decode(&raw, "id", b"body").is_ok());
        let arr = set(v, "/body_hash", as_array(&bytes)).to_string();
        assert_eq!(
            QueuedIntent::decode(arr.as_bytes(), "id", b"body").err(),
            INTENT_INVALID
        );
        assert!(raw.len() <= 1024);
    }

    // E1: the non-canonical forms, on a fixed field (16 bytes, 24 characters), a variable field and the option.
    #[test]
    fn t_e1_non_canonical_forms_refused() {
        let v = control();
        let sid = v["core"]["sid"].as_str().unwrap().to_owned();
        assert_eq!(sid.len(), 24);
        let fixed_forms = [
            ("no padding", sid[..22].to_owned()),
            ("one pad short", sid[..23].to_owned()),
            ("extra pad", format!("{sid}=")),
            ("pad inside", format!("{}={}", &sid[..10], &sid[11..])),
            ("trailing garbage", format!("{sid}A")),
            (
                "wrong length: 32-byte text on a 16-byte field",
                v["core"]["root"].as_str().unwrap().to_owned(),
            ),
            ("non-alphabet", format!("{}-{}", &sid[..10], &sid[11..])),
            ("space inside", format!("{} {}", &sid[..10], &sid[11..])),
            ("non-zero trailing bits", format!("{}B==", &sid[..21])),
            ("empty", String::new()),
            ("escaped null", "null".to_owned()),
        ];
        for (name, form) in fixed_forms {
            let bad = if name == "escaped null" {
                Value::Null
            } else {
                Value::from(form)
            };
            assert_eq!(
                decode(&set(v.clone(), "/core/sid", bad)),
                TAMPERED,
                "{name}"
            );
        }
        let wire = v["flights"]["1:0"]["wire"].as_str().unwrap().to_owned();
        assert_eq!(wire, "BQ==");
        for (name, form) in [
            ("no padding", "BQ"),
            ("one pad", "BQ="),
            ("three pads", "BQ==="),
            ("trailing", "BQ==A"),
            ("trailing bits", "BR=="),
            ("space", "B Q=="),
            ("url-safe", "B_=="),
            ("number", "5"),
        ] {
            let bad = if name == "number" {
                Value::from(5)
            } else {
                Value::from(form)
            };
            assert_eq!(
                decode(&set(v.clone(), "/flights/1:0/wire", bad)),
                TAMPERED,
                "{name}"
            );
        }
        // the option: null is None; a number array and a bad string are refused; absent is refused (S1's rule)
        let t =
            Transaction::decode(&set(v.clone(), "/core/last_in", Value::Null).to_string()).unwrap();
        assert!(t.core.last_in.is_none());
        assert_eq!(
            decode(&set(v.clone(), "/core/last_in", as_array(&[1; 32]))),
            TAMPERED
        );
        assert_eq!(
            decode(&set(v.clone(), "/core/last_in", Value::from("AQ=="))),
            TAMPERED
        );
        let mut absent = v.clone();
        absent["core"].as_object_mut().unwrap().remove("last_in");
        assert_eq!(decode(&absent), TAMPERED);
    }

    // E4: lengths 0, 1 and 2 are legitimate variable-field values and round-trip.
    #[test]
    fn t_e4_variable_field_lengths_zero_one_two_round_trip() {
        for (bytes, text) in [(vec![], ""), (vec![9], "CQ=="), (vec![9, 8], "CQg=")] {
            let mut t = sample();
            t.core.last_out = bytes.clone();
            t.flights.get_mut("1:0").unwrap().wire = bytes.clone();
            t.events.get_mut("e").unwrap().body = bytes.clone();
            let raw = t.encode().unwrap();
            let v: Value = serde_json::from_str(&raw).unwrap();
            assert_eq!(v["core"]["last_out"], text);
            assert_eq!(v["flights"]["1:0"]["wire"], text);
            assert_eq!(v["events"]["e"]["body"], text);
            let back = Transaction::decode(&raw).unwrap();
            assert_eq!(back.core.last_out, bytes);
            assert_eq!(back.encode().unwrap(), raw);
        }
    }

    // E2: schema 0 and 2 refuse with the new code on each of the four records.
    fn versions(v: &Value, pointer: &str, refuse: impl Fn(&Value) -> Option<&'static str>) {
        assert_eq!(
            v.pointer(pointer).unwrap(),
            &Value::from(SCHEMA_VERSION),
            "{pointer} is written as 1"
        );
        for bad in [0u64, 2, u64::MAX] {
            assert_eq!(
                refuse(&set(v.clone(), pointer, Value::from(bad))),
                VERSION,
                "{pointer} = {bad}"
            );
        }
    }
    #[test]
    fn t_e2_transaction_schema_0_and_2_refused() {
        versions(&control(), "/schema", decode);
    }
    #[test]
    fn t_e2_owner_schema_0_and_2_refused() {
        versions(&owner_control(), "/schema", own);
    }
    #[test]
    fn t_e2_flight_schema_0_and_2_refused() {
        versions(&control(), "/flights/1:0/schema", decode);
    }
    #[test]
    fn t_e2_disposition_schema_0_and_2_refused() {
        versions(&control(), "/dispositions/0:0/schema", decode);
    }

    // E2 DETECTION ORDER: a version-2 record with an extra field is a version, not tampering; the same
    // extra field at version 1 is tampering. serde_json's Value orders members alphabetically, so the
    // extra member ("extra" < "schema") precedes the version in the text: the version must still win.
    fn extra(v: &Value, pointer: &str) -> Value {
        let mut v = v.clone();
        v.pointer_mut(pointer)
            .unwrap()
            .as_object_mut()
            .unwrap()
            .insert("extra".into(), Value::from(0));
        v
    }
    fn version_two_with_extra(
        v: &Value,
        pointer: &str,
        refuse: impl Fn(&Value) -> Option<&'static str>,
        tampered: Option<&'static str>,
    ) {
        let with_extra = extra(v, pointer);
        assert_eq!(
            refuse(&with_extra),
            tampered,
            "{pointer}: extra field at version 1"
        );
        let v2 = set(with_extra, &format!("{pointer}/schema"), Value::from(2));
        let text = v2.to_string();
        // the only `"schema":2` is this object's; its `"extra":0` precedes it in the text
        assert!(
            text.find("\"extra\":0").unwrap() < text.find("\"schema\":2").unwrap(),
            "the extra member precedes the version in the text"
        );
        assert_eq!(
            refuse(&v2),
            VERSION,
            "{pointer}: version 2 with an extra field"
        );
    }
    #[test]
    fn t_e2_transaction_version_2_with_extra_field_is_a_version() {
        version_two_with_extra(&control(), "", decode, TAMPERED);
    }
    #[test]
    fn t_e2_owner_version_2_with_extra_field_is_a_version() {
        version_two_with_extra(&owner_control(), "", own, OWNER_TAMPERED);
    }
    #[test]
    fn t_e2_flight_version_2_with_extra_field_is_a_version() {
        version_two_with_extra(&control(), "/flights/1:0", decode, TAMPERED);
    }
    #[test]
    fn t_e2_disposition_version_2_with_extra_field_is_a_version() {
        version_two_with_extra(&control(), "/dispositions/0:0", decode, TAMPERED);
    }

    // E2: a missing, repeated, non-integer or negative schema keeps the existing code (no default).
    fn malformed_schema(
        v: &Value,
        pointer: &str,
        refuse: impl Fn(&Value) -> Option<&'static str>,
        tampered: Option<&'static str>,
    ) {
        let mut absent = v.clone();
        absent
            .pointer_mut(pointer)
            .unwrap()
            .as_object_mut()
            .unwrap()
            .remove("schema")
            .unwrap();
        assert_eq!(refuse(&absent), tampered, "{pointer}: absent");
        for (name, bad) in [
            ("string", Value::from("1")),
            ("float", Value::from(1.5)),
            ("negative", Value::from(-1)),
            ("null", Value::Null),
            ("array", Value::from(vec![1])),
        ] {
            assert_eq!(
                refuse(&set(v.clone(), &format!("{pointer}/schema"), bad)),
                tampered,
                "{pointer}: {name}"
            );
        }
    }
    fn duplicated_schema(v: &Value, pointer: &str) -> String {
        // the object at `pointer` re-spelled with its schema member twice (a Value cannot hold a repeat)
        let mut marked = v.clone();
        *marked.pointer_mut(&format!("{pointer}/schema")).unwrap() = Value::from("F04S5_DUP");
        marked
            .to_string()
            .replacen("\"schema\":\"F04S5_DUP\"", "\"schema\":1,\"schema\":1", 1)
    }
    #[test]
    fn t_e2_transaction_malformed_schema_keeps_existing_code() {
        let v = control();
        malformed_schema(&v, "", decode, TAMPERED);
        assert_eq!(
            Transaction::decode(&duplicated_schema(&v, "")).err(),
            TAMPERED,
            "repeated"
        );
    }
    #[test]
    fn t_e2_owner_malformed_schema_keeps_existing_code() {
        let v = owner_control();
        malformed_schema(&v, "", own, OWNER_TAMPERED);
        assert_eq!(
            CapacityOwner::decode(&duplicated_schema(&v, "")).err(),
            OWNER_TAMPERED,
            "repeated"
        );
    }
    #[test]
    fn t_e2_flight_and_disposition_malformed_schema_keep_existing_code() {
        let v = control();
        malformed_schema(&v, "/flights/1:0", decode, TAMPERED);
        malformed_schema(&v, "/dispositions/0:0", decode, TAMPERED);
        assert_eq!(
            Transaction::decode(&duplicated_schema(&v, "/flights/1:0")).err(),
            TAMPERED,
            "repeated"
        );
        assert_eq!(
            Transaction::decode(&duplicated_schema(&v, "/dispositions/0:0")).err(),
            TAMPERED,
            "repeated"
        );
    }

    // E2: the writers put the version first in every record and nested record.
    #[test]
    fn t_e2_schema_is_the_first_member_written() {
        let raw = sample().encode().unwrap();
        assert!(
            raw.starts_with("{\"schema\":1,\"version\":\"NA0780-DIR-INTEGRATION-03\","),
            "{}",
            &raw[..64]
        );
        assert!(raw.contains("\"flights\":{\"1:0\":{\"schema\":1,\"body_hash\":\""));
        assert!(raw.contains("\"dispositions\":{\"0:0\":{\"schema\":1,\"hash\":\""));
        let owner = serde_json::to_string(&owner()).unwrap();
        assert!(
            owner.starts_with("{\"schema\":1,\"generation\":1,"),
            "{}",
            &owner[..40]
        );
    }

    // I04: the encoding round-trips byte-exact over the MV-2 count maxima and the one-entry record.
    fn count_maxima() -> Transaction {
        let mut t = sample();
        let sid = t.core.sid;
        t.send = BTreeMap::from([(1, receipts(sid, 1)), (2, receipts(sid, 2))]);
        t.recv = BTreeMap::from([(0, receipts(sid, 0))]);
        t.flights = (0..64)
            .map(|i| (slot_key(1, i), flight(1, i, &format!("m{i}"))))
            .collect();
        t.flights.insert(slot_key(2, 0), flight(2, 0, ""));
        t.events = (0..64)
            .map(|i| (format!("e{i}"), event(&format!("e{i}"))))
            .collect();
        t.completed = (0..64).map(|i| (format!("c{i}"), fresh())).collect();
        t
    }
    #[test]
    fn t_i04_round_trip_byte_exact_over_the_count_maxima() {
        for t in [sample(), count_maxima()] {
            let raw = t.encode().unwrap();
            assert_eq!(Transaction::decode(&raw).unwrap().encode().unwrap(), raw);
            // no census field is a number array anywhere in the tree
            let v: Value = serde_json::from_str(&raw).unwrap();
            fn walk(v: &Value, path: &str) {
                match v {
                    Value::Object(m) => m.iter().for_each(|(k, x)| walk(x, &format!("{path}/{k}"))),
                    Value::Array(a) => {
                        assert!(path.ends_with("/holes"), "{path}: an array outside holes");
                        a.iter().for_each(|x| walk(x, path));
                    }
                    _ => {}
                }
            }
            walk(&v, "");
        }
        let owner_raw = serde_json::to_string(&owner()).unwrap();
        assert_eq!(
            serde_json::to_string(&CapacityOwner::decode(&owner_raw).unwrap()).unwrap(),
            owner_raw
        );
    }

    // E4: the count-maxima record shrinks (base 36,403 bytes, step1/size_probe_base.txt) and the paper
    // bound of core_context_future (288,920) holds at the maximum-width core with margin.
    #[test]
    fn t_e4_sizes_shrink_and_the_paper_bound_holds() {
        let after = count_maxima().encode().unwrap().len();
        println!("F04S5_SIZE count_maxima_record {after}");
        assert!(after < 36_403, "{after}");
        let mut t = sample();
        let sid = t.core.sid;
        let e = |id: u64, skipped: u32| {
            let mut e = epoch(id);
            e.next = u32::MAX;
            e.terminal = Some(u32::MAX);
            e.skipped = (0..skipped).map(|i| (u32::MAX - i, fresh())).collect();
            e
        };
        t.core.seq = u64::MAX;
        t.core.local_next = u32::MAX;
        t.core.local_consumed_prefix = u32::MAX;
        t.core.peer_max = u32::MAX;
        t.core.peer_selected_prefix = u32::MAX;
        t.core.active_recv = Some(u64::MAX);
        t.core.send = Some(e(u64::MAX, 0));
        t.core.recv = BTreeMap::from([
            (u64::MAX - 1, e(u64::MAX - 1, 8)),
            (u64::MAX, e(u64::MAX, 8)),
        ]);
        t.core.local = BTreeMap::from([(
            u32::MAX,
            LocalTarget {
                pk: vec![255; 1184],
                sk: vec![255; 2400],
            },
        )]);
        t.core.peer = BTreeMap::from([(u32::MAX, vec![255; 1184])]);
        t.core.last_out = vec![255; 65536];
        let r = |g: u64| {
            let mut r = receipts(sid, g);
            r.next = u32::MAX;
            r.prefix = u32::MAX;
            r.confirmed = u32::MAX;
            r.terminal = Some(u32::MAX);
            r.holes.clear();
            r
        };
        t.send = BTreeMap::from([(u64::MAX - 1, r(u64::MAX - 1)), (u64::MAX, r(u64::MAX))]);
        t.recv = BTreeMap::from([(u64::MAX - 2, r(u64::MAX - 2))]);
        t.flights.clear();
        t.dispositions.clear();
        t.events.clear();
        t.completed.clear();
        t.generation = u64::MAX;
        t.recv_floor = Some(u64::MAX);
        t.send_floor = Some(u64::MAX);
        t.since_boundary = u32::MAX;
        t.last_boundary = u64::MAX;
        let actual = serde_json::to_vec(&t).unwrap().len();
        let future = t.core_context_future().unwrap();
        println!("F04S5_SIZE max_width_core_context_actual {actual} core_context_future {future}");
        assert!(future > 671, "the base's margin was 671 bytes");
    }

    // E5: the ticket (a bare [u8;16] in a tuple) and the intent-hash input (Padding) serialize as before.
    #[test]
    fn t_e5_ticket_and_padding_serialization_unchanged() {
        let sid: [u8; 16] = std::array::from_fn(|i| i as u8 + 1);
        let ticket = serde_json::to_string(&("peer", sid, 0u8, "id")).unwrap();
        assert_eq!(
            ticket,
            "[\"peer\",[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16],0,\"id\"]"
        );
        let padding = Padding {
            profile: 1,
            maximum: 4096,
            size: 1024,
        };
        assert_eq!(
            serde_json::to_vec(&padding).unwrap(),
            b"{\"profile\":1,\"maximum\":4096,\"size\":1024}"
        );
    }
}
