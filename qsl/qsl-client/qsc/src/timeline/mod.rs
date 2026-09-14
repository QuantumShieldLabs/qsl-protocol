use crate::output::{CliError, CliResult};
use serde::{Deserialize, Serialize};

use crate::store::{FileTransferRecord, TimelineStore, TIMELINE_SECRET_KEY};
use crate::vault;

use super::{
    channel_label_ok,
    confirm_target_matches_channel, emit_cli_named_marker, emit_marker,
    file_xfer_store_key, require_unlocked, short_device_marker, short_peer_marker,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct TimelineEntry {
    pub(super) id: String,
    pub(super) peer: String,
    pub(super) direction: String,
    pub(super) byte_len: usize,
    pub(super) kind: String,
    pub(super) ts: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) target_device_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    content_commitment: Option<[u8; 32]>,
    #[serde(default)]
    pub(super) state: String,
    #[serde(default)]
    pub(super) status: String,
}

pub(crate) fn timeline_ts_default() -> u64 {
    1
}

pub(super) fn timeline_entry_default_state(direction: &str, status: &str) -> MessageState {
    if let Some(parsed) = MessageState::parse(status) {
        return parsed;
    }
    if direction == "out" {
        MessageState::Sent
    } else {
        MessageState::Received
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum MessageState {
    Created,
    Sent,
    Received,
    Delivered,
    Failed,
}

impl MessageState {
    pub(super) fn as_str(self) -> &'static str {
        match self {
            MessageState::Created => "CREATED",
            MessageState::Sent => "SENT",
            MessageState::Received => "RECEIVED",
            MessageState::Delivered => "DELIVERED",
            MessageState::Failed => "FAILED",
        }
    }

    pub(super) fn as_status(self) -> &'static str {
        match self {
            MessageState::Created => "created",
            MessageState::Sent => "sent",
            MessageState::Received => "received",
            MessageState::Delivered => "delivered",
            MessageState::Failed => "failed",
        }
    }

    pub(super) fn parse(s: &str) -> Option<Self> {
        match s {
            "CREATED" | "created" => Some(MessageState::Created),
            "SENT" | "sent" => Some(MessageState::Sent),
            "RECEIVED" | "received" => Some(MessageState::Received),
            "DELIVERED" | "delivered" => Some(MessageState::Delivered),
            "FAILED" | "failed" => Some(MessageState::Failed),
            _ => None,
        }
    }
}

fn message_delivery_semantic(direction: &str, state: MessageState) -> Option<&'static str> {
    if direction != "out" {
        return None;
    }
    match state {
        MessageState::Sent => Some("accepted_by_relay"),
        MessageState::Delivered => Some("peer_confirmed"),
        _ => None,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ConfirmPolicy {
    PrimaryOnly,
}

impl ConfirmPolicy {
    fn as_str(self) -> &'static str {
        match self {
            Self::PrimaryOnly => "primary_only",
        }
    }
}

const CONFIRM_POLICY: ConfirmPolicy = ConfirmPolicy::PrimaryOnly;

pub(super) fn emit_cli_confirm_policy() {
    emit_cli_named_marker("QSC_CONFIRM_POLICY", &[("policy", CONFIRM_POLICY.as_str())]);
}

pub(super) fn emit_cli_delivery_state_with_device(
    peer: &str,
    state: &'static str,
    device: Option<&str>,
) {
    let safe_peer = short_peer_marker(peer);
    let safe_device = short_device_marker(device.unwrap_or("unknown"));
    emit_cli_named_marker(
        "QSC_DELIVERY",
        &[
            ("state", state),
            ("policy", CONFIRM_POLICY.as_str()),
            ("peer", safe_peer.as_str()),
            ("device", safe_device.as_str()),
        ],
    );
}


pub(super) fn emit_cli_receipt_ignored_wrong_device(peer: &str, device: &str) {
    let safe_peer = short_peer_marker(peer);
    let safe_device = short_device_marker(device);
    emit_cli_named_marker(
        "QSC_RECEIPT_IGNORED",
        &[
            ("reason", "wrong_device"),
            ("peer", safe_peer.as_str()),
            ("device", safe_device.as_str()),
        ],
    );
}


pub(super) fn message_state_transition_allowed(
    from: MessageState,
    to: MessageState,
    direction: &str,
) -> Result<(), &'static str> {
    if from == MessageState::Failed {
        return Err("failed_terminal");
    }
    if from == to {
        return Err("state_duplicate");
    }
    if direction == "out" {
        return match (from, to) {
            (MessageState::Created, MessageState::Sent)
            | (MessageState::Created, MessageState::Failed)
            | (MessageState::Sent, MessageState::Delivered)
            | (MessageState::Sent, MessageState::Failed) => Ok(()),
            _ => Err("state_invalid_transition"),
        };
    }
    match (from, to) {
        (MessageState::Created, MessageState::Received)
        | (MessageState::Created, MessageState::Failed)
        | (MessageState::Received, MessageState::Failed) => Ok(()),
        _ => Err("state_invalid_transition"),
    }
}

/// NA-0686 / D-1325 (ENG-0084, operator-ruled Option C clause (b)) — the same
/// structural treatment as `emit_message_state_reject`, applied because the
/// precondition was MEASURED rather than assumed.
///
/// The clause was authorised "if and only if every consumer of this field is
/// gone". Measured across the whole surface, not just the files this lane
/// touched: **zero** consumers read `id` from this marker — not in
/// `qsl/qsl-client/qsc/tests/` (the only test naming this event,
/// `attachment_streaming_na0197c.rs`, asserts `contains("event=…")` and reads no
/// field), not in `qsc` itself, and not in `qsl-desktop`, which consumes qsc
/// markers and parses no `id=` at all. So the field was carrying an identifier
/// to nobody, at the cost of a live coupling to the shape-keyed redactor.
///
/// ⚠ ONE THING THE NAME HIDES, recorded because it is decision-bearing: despite
/// "message_state", this marker is **shared with the attachment path** —
/// `timeline_append_entry_for_target` and `timeline_transition_entry_state` both
/// serve file transfers, so an attachment's timeline id previously appeared here
/// too. Redacting it is therefore a (diagnostic-only) change to the attachment
/// surface as well. It is NOT the `attachments/mod.rs` `file_id` population that
/// the ruling placed out of scope — those sites are untouched — but the overlap
/// is real and is reported rather than absorbed.
pub(super) fn emit_message_state_transition(from: MessageState, to: MessageState) {
    emit_marker(
        "message_state_transition",
        None,
        &[
            ("from", from.as_str()),
            ("to", to.as_str()),
            ("id", "<redacted>"),
            ("ok", "true"),
        ],
    );
}

/// NA-0686 / D-1325 (ENG-0084, operator-ruled Option C) — this helper NO LONGER
/// ACCEPTS AN IDENTIFIER, and that is the fix.
///
/// ⚠ WHAT WAS ACTUALLY WRONG, because the ledger's premise turned out to be
/// false. ENG-0084 was filed as "`msg_id` is emitted UNREDACTED at one site" and
/// proposed a field-name-keyed redactor rule for the `msg_id` field.
/// MEASUREMENT: **every marker field literally named `msg_id` already carries
/// the literal string `"<redacted>"`** (eight sites). The field that carries
/// real message ids is this one — keyed `id` — so a rule on the name `msg_id`
/// would have been a **no-op**, and a rule on `id` would also have re-keyed the
/// attachment and timeline-listing markers, which this lane does not audit.
///
/// So the coupling is removed WITHOUT touching the redactor at all. Three of the
/// four call sites already passed `"<redacted>"` by hand; the fourth
/// (`transport/mod.rs`, the ack-reject path) passed `ctrl.msg_id` raw and was
/// defused only BY ACCIDENT — NA-0682's 32-hex id happens to cross the marker
/// layer's `len() >= 24` shape rule. **Any future identifier narrower than that
/// re-opened a raw emission here, with no test and no declaration connecting the
/// two.** A parameter that does not exist cannot be passed raw: the property is
/// now structural rather than a policy that happens to hold.
///
/// The emitted marker is BYTE-IDENTICAL to what the three correct sites produced
/// before, so no consumer moves. `message_state_model.rs` asserts that this
/// marker never echoes a raw id (the C17/F1 regression tripwire); that guard is
/// deliberately left untouched and is now structurally true rather than
/// shape-dependent.
///
/// Remaining population, named rather than silently left (ENG-0084 closure): the
/// attachment `file_id` asymmetry (`attachments/mod.rs`) and the `timeline_item`
/// entry id both still reach the marker layer as `id` and remain governed by the
/// shape-keyed redactor. Out of scope here by ruling; recorded for a later lane.
pub(super) fn emit_message_state_reject(reason: &'static str) {
    emit_marker(
        "message_state_reject",
        Some(reason),
        &[("reason", reason), ("id", "<redacted>")],
    );
}

fn timeline_entry_state(entry: &TimelineEntry) -> MessageState {
    MessageState::parse(entry.state.as_str())
        .or_else(|| MessageState::parse(entry.status.as_str()))
        .unwrap_or_else(|| {
            timeline_entry_default_state(entry.direction.as_str(), entry.status.as_str())
        })
}

pub(super) fn file_delivery_short_id(raw: &str) -> String {
    let mut out = String::new();
    for ch in raw.chars() {
        if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
            out.push(ch.to_ascii_lowercase());
        }
        if out.len() >= 12 {
            break;
        }
    }
    if out.is_empty() {
        "unknown".to_string()
    } else {
        out
    }
}

pub(super) fn emit_cli_file_delivery_with_device(
    peer: &str,
    state: &'static str,
    file_id: &str,
    device: Option<&str>,
) {
    let safe_peer = short_peer_marker(peer);
    let safe_file = file_delivery_short_id(file_id);
    let safe_device = short_device_marker(device.unwrap_or("unknown"));
    emit_cli_named_marker(
        "QSC_FILE_DELIVERY",
        &[
            ("state", state),
            ("policy", CONFIRM_POLICY.as_str()),
            ("peer", safe_peer.as_str()),
            ("device", safe_device.as_str()),
            ("file", safe_file.as_str()),
        ],
    );
}


pub(super) fn file_transfer_upsert_outbound_record(
    peer: &str,
    file_id: &str,
    rec: FileTransferRecord,
) -> Result<(), &'static str> {
    let key = file_xfer_store_key(peer, file_id);
    let _lock = timeline_lock()?;
    let mut store = timeline_store_load().map_err(|_| "timeline_unavailable")?;
    store.file_transfers.insert(key, rec);
    timeline_store_save(&store).map_err(|_| "timeline_unavailable")
}

fn file_transfer_apply_confirmation(
    peer: &str,
    file_id: &str,
    confirm_id: &str,
    recv_channel: &str,
) -> Result<(), &'static str> {
    let key = file_xfer_store_key(peer, file_id);
    let _lock = timeline_lock()?;
    let mut store = timeline_store_load().map_err(|_| "timeline_unavailable")?;
    let rec = store.file_transfers.get_mut(&key).ok_or("state_unknown")?;
    if !rec.confirm_requested {
        return Err("confirm_not_requested");
    }
    if rec.state == "PEER_CONFIRMED" {
        return Err("state_duplicate");
    }
    if rec.confirm_id.as_deref().unwrap_or("") != confirm_id {
        return Err("confirm_id_mismatch");
    }
    if !confirm_target_matches_channel(rec.target_device_id.as_deref(), recv_channel) {
        return Err("confirm_wrong_device");
    }
    rec.state = "PEER_CONFIRMED".to_string();
    timeline_store_save(&store).map_err(|_| "timeline_unavailable")
}


fn file_transfer_target_device(peer: &str, file_id: &str) -> Result<Option<String>, &'static str> {
    let key = file_xfer_store_key(peer, file_id);
    let store = timeline_store_load().map_err(|_| "timeline_unavailable")?;
    let rec = store.file_transfers.get(&key).ok_or("state_unknown")?;
    Ok(rec.target_device_id.clone())
}

pub(super) fn file_transfer_confirm_id(peer: &str, file_id: &str) -> Result<String, &'static str> {
    let key = file_xfer_store_key(peer, file_id);
    let store = timeline_store_load().map_err(|_| "timeline_unavailable")?;
    let rec = store.file_transfers.get(&key).ok_or("state_unknown")?;
    rec.confirm_id.clone().ok_or("confirm_id_missing")
}


// A caller may edit this snapshot, but can never save it over a different current
// record. Keep this guard at the shared API: attachment callers also use it.
pub(crate) struct TimelineSnapshot {
    store: TimelineStore,
    original: Option<String>,
}
impl std::ops::Deref for TimelineSnapshot {
    type Target = TimelineStore;
    fn deref(&self) -> &Self::Target {
        &self.store
    }
}
impl std::ops::DerefMut for TimelineSnapshot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.store
    }
}
fn timeline_lock() -> Result<crate::model::LockGuard, &'static str> {
    let (dir, source) = crate::fs_store::config_dir().map_err(|_| "timeline_unavailable")?;
    crate::fs_store::lock_store_exclusive(&dir, source).map_err(|_| "timeline_unavailable")
}
pub(super) fn timeline_store_load() -> Result<TimelineSnapshot, &'static str> {
    let original = vault::secret_get(TIMELINE_SECRET_KEY).map_err(|_| "timeline_unavailable")?;
    let mut store = match original.as_deref() {
        None => TimelineStore::default(),
        Some(v) => serde_json::from_str::<TimelineStore>(v).map_err(|_| "timeline_tampered")?,
    };
    if store.next_ts == 0 {
        store.next_ts = 1;
    }
    Ok(TimelineSnapshot { store, original })
}
pub(super) fn timeline_store_save(snapshot: &TimelineSnapshot) -> Result<(), &'static str> {
    let _lock = timeline_lock()?;
    // Reload after taking the SAME reentrant store lock used by vault writes.
    // Equality covers every persisted field, including unrelated peers/transfers.
    let current = vault::secret_get(TIMELINE_SECRET_KEY).map_err(|_| "timeline_unavailable")?;
    if current != snapshot.original {
        return Err("timeline_stale_snapshot");
    }
    let json = serde_json::to_string(&snapshot.store).map_err(|_| "timeline_unavailable")?;
    vault::secret_set(TIMELINE_SECRET_KEY, &json).map_err(|_| "timeline_unavailable")
}

pub(super) fn timeline_append_entry(
    peer: &str,
    direction: &str,
    byte_len: usize,
    kind: &str,
    final_state: MessageState,
    forced_id: Option<&str>,
) -> Result<TimelineEntry, &'static str> {
    timeline_append_entry_for_target(
        peer,
        direction,
        byte_len,
        kind,
        final_state,
        forced_id,
        None,
    )
}

// The existing reentrant store lock spans load, mutation and save. Nested
// transport transactions acquire no second lock or competing lock order.
pub(super) fn timeline_append_entry_for_target(
    peer: &str,
    direction: &str,
    byte_len: usize,
    kind: &str,
    final_state: MessageState,
    forced_id: Option<&str>,
    target_device_id: Option<&str>,
) -> Result<TimelineEntry, &'static str> {
    timeline_append_bound_entry(
        peer,
        direction,
        byte_len,
        kind,
        final_state,
        forced_id,
        target_device_id,
        None,
    )
}

// Candidate projections bind full immutable content, not just its length. The
// digest remains inside the encrypted timeline and is never emitted in markers.
pub(crate) fn timeline_project_message(
    peer: &str,
    direction: &str,
    body: &[u8],
    id: &str,
) -> Result<TimelineEntry, &'static str> {
    use sha2::Digest;
    let state = match direction {
        "in" => MessageState::Received,
        "out" => MessageState::Sent,
        _ => return Err("timeline_direction_invalid"),
    };
    let commitment: [u8; 32] = sha2::Sha256::digest(body).into();
    timeline_append_bound_entry(
        peer,
        direction,
        body.len(),
        "msg",
        state,
        Some(id),
        None,
        Some(commitment),
    )
}

#[allow(clippy::too_many_arguments)]
fn timeline_append_bound_entry(
    peer: &str,
    direction: &str,
    byte_len: usize,
    kind: &str,
    final_state: MessageState,
    forced_id: Option<&str>,
    target_device_id: Option<&str>,
    content_commitment: Option<[u8; 32]>,
) -> Result<TimelineEntry, &'static str> {
    if !channel_label_ok(peer) {
        return Err("timeline_peer_invalid");
    }
    if let Some(v) = forced_id {
        if v.trim().is_empty() {
            return Err("state_id_invalid");
        }
    }
    message_state_transition_allowed(MessageState::Created, final_state, direction)?;
    let _lock = timeline_lock()?;
    let mut store = timeline_store_load()?;
    if let Some(id) = forced_id {
        let mut matches = store
            .peers
            .values()
            .flatten()
            .filter(|entry| entry.id == id);
        if let Some(entry) = matches.next() {
            if matches.next().is_some()
                || entry.peer != peer
                || entry.direction != direction
                || entry.byte_len != byte_len
                || entry.kind != kind
                || entry.target_device_id != target_device_id.map(short_device_marker)
                || entry.content_commitment != content_commitment
            {
                return Err("timeline_id_conflict");
            }
            // Replay is insertion only: never roll back an advanced/failed state.
            // Status changes go through timeline_transition_entry_state instead.
            return Ok(entry.clone());
        }
    }
    let ts = store.next_ts;
    store.next_ts = store.next_ts.checked_add(1).ok_or("timeline_capacity")?;
    let id = forced_id
        .map(|v| v.to_string())
        .unwrap_or_else(|| format!("{}-{}", direction, ts));
    if store.peers.values().flatten().any(|entry| entry.id == id) {
        return Err("timeline_id_conflict");
    }
    let entry = TimelineEntry {
        id: id.clone(),
        peer: peer.to_string(),
        direction: direction.to_string(),
        byte_len,
        kind: kind.to_string(),
        ts,
        target_device_id: target_device_id.map(short_device_marker),
        content_commitment,
        state: final_state.as_str().to_string(),
        status: final_state.as_status().to_string(),
    };
    store
        .peers
        .entry(peer.to_string())
        .or_default()
        .push(entry.clone());
    timeline_store_save(&store)?;
    emit_message_state_transition(MessageState::Created, final_state);
    Ok(entry)
}

pub(crate) fn timeline_transition_entry_state(
    peer: &str,
    id: &str,
    to: MessageState,
) -> Result<TimelineEntry, &'static str> {
    if !channel_label_ok(peer) {
        return Err("timeline_peer_invalid");
    }
    if id.trim().is_empty() {
        return Err("state_id_invalid");
    }
    let _lock = timeline_lock()?;
    let mut store = timeline_store_load()?;
    let Some(entries) = store.peers.get_mut(peer) else {
        return Err("state_unknown");
    };
    let Some(entry) = entries.iter_mut().find(|v| v.id == id) else {
        return Err("state_unknown");
    };
    let from = timeline_entry_state(entry);
    message_state_transition_allowed(from, to, entry.direction.as_str())?;
    entry.state = to.as_str().to_string();
    entry.status = to.as_status().to_string();
    let out = entry.clone();
    timeline_store_save(&store)?;
    emit_message_state_transition(from, to);
    Ok(out)
}

fn timeline_entries_for_peer(peer: &str) -> Result<Vec<TimelineEntry>, &'static str> {
    if !channel_label_ok(peer) {
        return Err("timeline_peer_invalid");
    }
    let store = timeline_store_load()?;
    Ok(store.peers.get(peer).cloned().unwrap_or_default())
}

fn timeline_outbound_target_device(peer: &str, id: &str) -> Result<Option<String>, &'static str> {
    if !channel_label_ok(peer) {
        return Err("timeline_peer_invalid");
    }
    if id.trim().is_empty() {
        return Err("state_id_invalid");
    }
    let store = timeline_store_load()?;
    let Some(entries) = store.peers.get(peer) else {
        return Err("state_unknown");
    };
    let Some(entry) = entries.iter().find(|v| v.id == id) else {
        return Err("state_unknown");
    };
    Ok(entry.target_device_id.clone())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ConfirmApplyOutcome {
    Confirmed,
    IgnoredWrongDevice,
}

pub(super) fn apply_message_peer_confirmation(
    peer: &str,
    msg_id: &str,
    recv_channel: &str,
) -> Result<(ConfirmApplyOutcome, Option<String>), &'static str> {
    let target = timeline_outbound_target_device(peer, msg_id)?;
    if !confirm_target_matches_channel(target.as_deref(), recv_channel) {
        return Ok((ConfirmApplyOutcome::IgnoredWrongDevice, target));
    }
    timeline_transition_entry_state(peer, msg_id, MessageState::Delivered)?;
    Ok((ConfirmApplyOutcome::Confirmed, target))
}

pub(super) fn apply_file_peer_confirmation(
    peer: &str,
    file_id: &str,
    confirm_id: &str,
    recv_channel: &str,
) -> Result<(ConfirmApplyOutcome, Option<String>), &'static str> {
    let target = file_transfer_target_device(peer, file_id)?;
    if !confirm_target_matches_channel(target.as_deref(), recv_channel) {
        return Ok((ConfirmApplyOutcome::IgnoredWrongDevice, target));
    }
    file_transfer_apply_confirmation(peer, file_id, confirm_id, recv_channel)?;
    timeline_transition_entry_state(peer, file_id, MessageState::Delivered)?;
    Ok((ConfirmApplyOutcome::Confirmed, target))
}


fn timeline_emit_item(entry: &TimelineEntry) {
    let len_s = entry.byte_len.to_string();
    let ts_s = entry.ts.to_string();
    let state = timeline_entry_state(entry);
    emit_marker(
        "timeline_item",
        None,
        &[
            ("id", entry.id.as_str()),
            ("dir", entry.direction.as_str()),
            ("len", len_s.as_str()),
            ("kind", entry.kind.as_str()),
            ("ts", ts_s.as_str()),
            ("state", state.as_str()),
        ],
    );
    if let Some(delivery) = message_delivery_semantic(entry.direction.as_str(), state) {
        if entry.kind == "file" {
            emit_cli_file_delivery_with_device(
                entry.peer.as_str(),
                delivery,
                entry.id.as_str(),
                entry.target_device_id.as_deref(),
            );
        } else {
            let safe_peer = short_peer_marker(entry.peer.as_str());
            emit_cli_named_marker(
                "QSC_DELIVERY",
                &[("state", delivery), ("peer", safe_peer.as_str())],
            );
        }
    }
}

pub(super) fn latest_outbound_file_id(peer: &str) -> Result<String, &'static str> {
    let entries = timeline_entries_for_peer(peer)?;
    let Some(entry) = entries
        .into_iter()
        .filter(|v| v.direction == "out" && v.kind == "file")
        .max_by(|a, b| a.ts.cmp(&b.ts).then_with(|| a.id.cmp(&b.id)))
    else {
        return Err("state_unknown");
    };
    Ok(entry.id)
}

pub fn timeline_list(peer: &str, limit: Option<usize>) -> CliResult {
    require_unlocked("timeline_list")?;
    let mut entries = timeline_entries_for_peer(peer).map_err(CliError::code)?;
    entries.sort_by(|a, b| b.ts.cmp(&a.ts).then_with(|| a.id.cmp(&b.id)));
    let take_n = limit.unwrap_or(entries.len()).min(entries.len());
    let count_s = take_n.to_string();
    emit_marker(
        "timeline_list",
        None,
        &[("count", count_s.as_str()), ("peer", peer)],
    );
    for entry in entries.into_iter().take(take_n) {
        timeline_emit_item(&entry);
    }
    Ok(())
}

pub fn timeline_show(peer: &str, id: &str) -> CliResult {
    require_unlocked("timeline_show")?;
    let entries = timeline_entries_for_peer(peer).map_err(CliError::code)?;
    let Some(entry) = entries.into_iter().find(|v| v.id == id) else {
        return Err(CliError::code("timeline_item_missing"));
    };
    timeline_emit_item(&entry);
    Ok(())
}

pub fn timeline_clear(peer: &str, confirm: bool) -> CliResult {
    require_unlocked("timeline_clear")?;
    if !confirm {
        emit_marker(
            "error",
            Some("timeline_clear_confirm_required"),
            &[("peer", peer), ("reason", "explicit_confirm_required")],
        );
        return Err(CliError::code("timeline_clear_confirm_required"));
    }
    if !channel_label_ok(peer) {
        return Err(CliError::code("timeline_peer_invalid"));
    }
    let _lock = timeline_lock().map_err(CliError::code)?;
    let mut store = timeline_store_load().map_err(CliError::code)?;
    #[cfg(test)]
    na0780_tests::after_clear_load();
    let removed = store.peers.remove(peer).map(|v| v.len()).unwrap_or(0usize);
    timeline_store_save(&store).map_err(CliError::code)?;
    let removed_s = removed.to_string();
    emit_marker(
        "timeline_clear",
        None,
        &[
            ("ok", "true"),
            ("peer", peer),
            ("removed", removed_s.as_str()),
        ],
    );
    Ok(())
}

#[cfg(test)]
mod na0780_tests {
    use super::*;
    use std::cell::RefCell;
    thread_local! { static CLEAR_HOOK: RefCell<Option<Box<dyn FnOnce()>>> = RefCell::new(None); }
    pub(super) fn after_clear_load() {
        let hook = CLEAR_HOOK.with(|h| h.borrow_mut().take());
        if let Some(hook) = hook {
            hook();
        }
    }
    fn fixture(name: &str) -> Option<tempfile::TempDir> {
        if std::env::var("NA0780_TIMELINE_CHILD").ok().as_deref() != Some(name) {
            let status = std::process::Command::new(std::env::current_exe().unwrap())
                .args([
                    "--exact",
                    &format!("timeline::na0780_tests::{name}"),
                    "--nocapture",
                ])
                .env("NA0780_TIMELINE_CHILD", name)
                .status()
                .unwrap();
            assert!(status.success(), "isolated timeline fixture failed");
            return None;
        }
        let dir = tempfile::tempdir().unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o700)).unwrap();
        }
        std::env::set_var("QSC_CONFIG_DIR", dir.path());
        std::env::remove_var("QSC_QSP_SEED");
        std::env::remove_var("QSC_ALLOW_SEED_FALLBACK");
        vault::vault_init_with_passphrase("local timeline fixture only").unwrap();
        vault::protection::unlock_guarded("local timeline fixture only").unwrap();
        Some(dir)
    }
    fn append(peer: &str, id: &str) -> TimelineEntry {
        timeline_append_entry(peer, "in", 7, "msg", MessageState::Received, Some(id)).unwrap()
    }
    #[test]
    fn duplicate_projection() {
        let Some(_dir) = fixture("duplicate_projection") else {
            return;
        };
        let first = append("alice", "stable");
        let second = append("alice", "stable");
        assert_eq!(
            timeline_entries_for_peer("alice").unwrap().len(),
            1,
            "duplicate stable ID inserted"
        );
        assert_eq!(first.ts, second.ts);
    }
    #[test]
    fn stale_clear_projection() {
        let Some(_dir) = fixture("stale_clear_projection") else {
            return;
        };
        append("bob", "old");
        CLEAR_HOOK.with(|h| {
            *h.borrow_mut() = Some(Box::new(|| {
                append("alice", "new");
            }))
        });
        let result = timeline_clear("bob", true);
        assert_eq!(
            timeline_entries_for_peer("alice").unwrap().len(),
            1,
            "stale clear erased unrelated projection"
        );
        assert!(result.is_err(), "stale snapshot must refuse");
        timeline_clear("bob", true).unwrap();
        assert!(timeline_entries_for_peer("bob").unwrap().is_empty());
    }
    #[test]
    fn conflicts_status_and_stale_save() {
        let Some(_dir) = fixture("conflicts_status_and_stale_save") else {
            return;
        };
        timeline_append_entry("alice", "out", 7, "msg", MessageState::Sent, Some("sent")).unwrap();
        let before = vault::secret_get(TIMELINE_SECRET_KEY).unwrap();
        for (peer, dir, len, kind) in [
            ("bob", "out", 7, "msg"),
            ("alice", "in", 7, "msg"),
            ("alice", "out", 8, "msg"),
            ("alice", "out", 7, "file"),
        ] {
            let state = if dir == "in" {
                MessageState::Received
            } else {
                MessageState::Sent
            };
            assert_eq!(
                timeline_append_entry(peer, dir, len, kind, state, Some("sent")).unwrap_err(),
                "timeline_id_conflict"
            );
            assert_eq!(vault::secret_get(TIMELINE_SECRET_KEY).unwrap(), before);
        }
        let mut stale = timeline_store_load().unwrap();
        timeline_transition_entry_state("alice", "sent", MessageState::Delivered).unwrap();
        let replay =
            timeline_append_entry("alice", "out", 7, "msg", MessageState::Sent, Some("sent"))
                .unwrap();
        assert_eq!(timeline_entry_state(&replay), MessageState::Delivered);
        let advanced = vault::secret_get(TIMELINE_SECRET_KEY).unwrap();
        stale.peers.clear();
        assert_eq!(timeline_store_save(&stale), Err("timeline_stale_snapshot"));
        assert_eq!(vault::secret_get(TIMELINE_SECRET_KEY).unwrap(), advanced);
        assert_eq!(
            timeline_transition_entry_state("alice", "sent", MessageState::Sent).unwrap_err(),
            "state_invalid_transition"
        );
        assert_eq!(
            timeline_transition_entry_state("alice", "sent", MessageState::Delivered).unwrap_err(),
            "state_duplicate"
        );
    }
    #[test]
    fn concurrent_restart_projection() {
        let Some(dir) = fixture("concurrent_restart_projection") else {
            return;
        };
        // The existing store lock is nonblocking. Competing writers must refuse
        // without mutation, then exact projection retry succeeds after release.
        let before = vault::secret_get(TIMELINE_SECRET_KEY).unwrap();
        let lock = timeline_lock().unwrap();
        let barrier = std::sync::Arc::new(std::sync::Barrier::new(5));
        let workers: Vec<_> = (0..4)
            .map(|_| {
                let barrier = barrier.clone();
                std::thread::spawn(move || {
                    barrier.wait();
                    assert_eq!(
                        timeline_append_entry(
                            "alice",
                            "in",
                            7,
                            "msg",
                            MessageState::Received,
                            Some("replayed")
                        )
                        .unwrap_err(),
                        "timeline_unavailable"
                    );
                })
            })
            .collect();
        barrier.wait();
        for worker in workers {
            worker.join().unwrap();
        }
        assert_eq!(vault::secret_get(TIMELINE_SECRET_KEY).unwrap(), before);
        drop(lock);
        for i in 0..4 {
            append("alice", "replayed");
            append("bob", &format!("unique-{i}"));
        }
        assert_eq!(timeline_entries_for_peer("alice").unwrap().len(), 1);
        assert_eq!(timeline_entries_for_peer("bob").unwrap().len(), 4);
        // A new process authenticates normally and retries after a projection cut.
        let status = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "timeline::na0780_tests::restart_worker",
                "--nocapture",
            ])
            .env("NA0780_RESTART_DIR", dir.path())
            .status()
            .unwrap();
        assert!(status.success());
        assert_eq!(timeline_entries_for_peer("alice").unwrap().len(), 1);
        assert_eq!(timeline_entries_for_peer("bob").unwrap().len(), 4);
    }
    #[test]
    fn restart_worker() {
        let Ok(dir) = std::env::var("NA0780_RESTART_DIR") else {
            return;
        };
        std::env::set_var("QSC_CONFIG_DIR", dir);
        vault::protection::unlock_guarded("local timeline fixture only").unwrap();
        append("alice", "replayed");
        timeline_clear("carol", true).unwrap();
    }
    #[test]
    fn full_content_projection() {
        let Some(_dir) = fixture("full_content_projection") else {
            return;
        };
        let first = timeline_project_message("alice", "in", b"first", "bound").unwrap();
        let before = vault::secret_get(TIMELINE_SECRET_KEY).unwrap();
        let replay = timeline_project_message("alice", "in", b"first", "bound").unwrap();
        assert_eq!(first.ts, replay.ts);
        assert_eq!(vault::secret_get(TIMELINE_SECRET_KEY).unwrap(), before);
        // Equal-length content must not pass the metadata-only identity check.
        assert_eq!(
            timeline_project_message("alice", "in", b"other", "bound").unwrap_err(),
            "timeline_id_conflict"
        );
        assert_eq!(vault::secret_get(TIMELINE_SECRET_KEY).unwrap(), before);
        assert_eq!(
            timeline_append_entry(
                "alice",
                "in",
                5,
                "msg",
                MessageState::Received,
                Some("bound")
            )
            .unwrap_err(),
            "timeline_id_conflict"
        );
        assert_eq!(timeline_entries_for_peer("alice").unwrap().len(), 1);
    }
}

// Read-only admission check; the authoritative receive commit must precede
// projection, so validation must never insert a row here.
pub(crate) fn timeline_validate_projection(peer:&str,body:&[u8],id:&str)->Result<(),&'static str>{
    use sha2::Digest;
    let _lock=timeline_lock()?;
    let store=timeline_store_load()?;
    let mut matches=store.peers.values().flatten().filter(|e|e.id==id);
    if let Some(e)=matches.next() {
        let digest:[u8;32]=sha2::Sha256::digest(body).into();
        if matches.next().is_some() || e.peer!=peer || e.direction!="in" || e.kind!="msg"
            || e.byte_len!=body.len() || e.target_device_id.is_some() || e.content_commitment!=Some(digest) {
            return Err("timeline_id_conflict");
        }
    }
    Ok(())
}
