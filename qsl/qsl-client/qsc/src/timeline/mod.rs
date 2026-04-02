use serde::{Deserialize, Serialize};

use crate::store::{FileTransferRecord, TimelineStore, TIMELINE_SECRET_KEY};
use crate::vault;

use super::{
    attachment_journal_load, attachment_journal_save, attachment_record_key, channel_label_ok,
    confirm_target_matches_channel, emit_cli_named_marker, emit_marker, emit_tui_named_marker,
    file_xfer_store_key, print_error_marker, require_unlocked, short_device_marker,
    short_peer_marker,
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

pub(super) fn message_delivery_semantic_from_state_str(
    direction: &str,
    state: &str,
) -> Option<&'static str> {
    MessageState::parse(state).and_then(|parsed| message_delivery_semantic(direction, parsed))
}

pub(super) fn file_delivery_semantic_from_state(state: &str) -> Option<&'static str> {
    match state.trim().to_ascii_uppercase().as_str() {
        "SENT" | "ACCEPTED_BY_RELAY" => Some("accepted_by_relay"),
        "AWAITING_CONFIRMATION" => Some("awaiting_confirmation"),
        "DELIVERED" | "PEER_CONFIRMED" => Some("peer_confirmed"),
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

pub(super) fn emit_tui_confirm_policy() {
    emit_tui_named_marker("QSC_CONFIRM_POLICY", &[("policy", CONFIRM_POLICY.as_str())]);
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

pub(super) fn emit_tui_delivery_state_with_device(
    thread: &str,
    state: &'static str,
    device: Option<&str>,
) {
    let safe_thread = short_peer_marker(thread);
    let safe_device = short_device_marker(device.unwrap_or("unknown"));
    emit_tui_named_marker(
        "QSC_TUI_DELIVERY",
        &[
            ("state", state),
            ("policy", CONFIRM_POLICY.as_str()),
            ("thread", safe_thread.as_str()),
            ("device", safe_device.as_str()),
        ],
    );
}

pub(super) fn emit_tui_delivery_state(thread: &str, state: &'static str) {
    emit_tui_delivery_state_with_device(thread, state, None);
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

pub(super) fn emit_tui_receipt_ignored_wrong_device(thread: &str, device: &str) {
    let safe_thread = short_peer_marker(thread);
    let safe_device = short_device_marker(device);
    emit_tui_named_marker(
        "QSC_TUI_RECEIPT_IGNORED",
        &[
            ("reason", "wrong_device"),
            ("thread", safe_thread.as_str()),
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

pub(super) fn emit_message_state_transition(id: &str, from: MessageState, to: MessageState) {
    emit_marker(
        "message_state_transition",
        None,
        &[
            ("from", from.as_str()),
            ("to", to.as_str()),
            ("id", id),
            ("ok", "true"),
        ],
    );
}

pub(super) fn emit_message_state_reject(id: &str, reason: &'static str) {
    emit_marker(
        "message_state_reject",
        Some(reason),
        &[("reason", reason), ("id", id)],
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

pub(super) fn emit_tui_file_delivery_with_device(
    thread: &str,
    state: &'static str,
    file_id: &str,
    device: Option<&str>,
) {
    let safe_thread = short_peer_marker(thread);
    let safe_file = file_delivery_short_id(file_id);
    let safe_device = short_device_marker(device.unwrap_or("unknown"));
    emit_tui_named_marker(
        "QSC_TUI_FILE_CONFIRM",
        &[
            ("state", state),
            ("policy", CONFIRM_POLICY.as_str()),
            ("thread", safe_thread.as_str()),
            ("device", safe_device.as_str()),
            ("file", safe_file.as_str()),
        ],
    );
}

pub(super) fn emit_tui_file_delivery(thread: &str, state: &'static str, file_id: &str) {
    emit_tui_file_delivery_with_device(thread, state, file_id, None);
}

pub(super) fn file_transfer_upsert_outbound_record(
    peer: &str,
    file_id: &str,
    rec: FileTransferRecord,
) -> Result<(), &'static str> {
    let key = file_xfer_store_key(peer, file_id);
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

fn attachment_transfer_apply_confirmation(
    peer: &str,
    attachment_id: &str,
    confirm_handle: &str,
    recv_channel: &str,
) -> Result<(), &'static str> {
    let key = attachment_record_key("out", peer, attachment_id);
    let mut journal = attachment_journal_load()?;
    let rec = journal
        .records
        .get_mut(&key)
        .ok_or("REJECT_ATT_CONFIRM_LINKAGE")?;
    if !rec.confirm_requested {
        return Err("REJECT_ATT_CONFIRM_LINKAGE");
    }
    if rec.state == "PEER_CONFIRMED" {
        return Err("REJECT_ATT_CONFIRM_LINKAGE");
    }
    if rec.confirm_handle.as_deref() != Some(confirm_handle) {
        return Err("REJECT_ATT_CONFIRM_LINKAGE");
    }
    if !confirm_target_matches_channel(rec.target_device_id.as_deref(), recv_channel) {
        return Err("confirm_wrong_device");
    }
    rec.state = "PEER_CONFIRMED".to_string();
    attachment_journal_save(&journal)?;
    Ok(())
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

fn attachment_transfer_timeline_id(
    peer: &str,
    attachment_id: &str,
) -> Result<String, &'static str> {
    let key = attachment_record_key("out", peer, attachment_id);
    let store = attachment_journal_load()?;
    let rec = store
        .records
        .get(&key)
        .ok_or("REJECT_ATT_CONFIRM_LINKAGE")?;
    rec.timeline_id.clone().ok_or("REJECT_ATT_CONFIRM_LINKAGE")
}

pub(super) fn timeline_store_load() -> Result<TimelineStore, &'static str> {
    let mut store = match vault::secret_get(TIMELINE_SECRET_KEY) {
        Ok(None) => Ok(TimelineStore::default()),
        Ok(Some(v)) => serde_json::from_str::<TimelineStore>(&v).map_err(|_| "timeline_tampered"),
        Err("vault_missing" | "vault_locked") => Err("timeline_unavailable"),
        Err(_) => Err("timeline_unavailable"),
    }?;
    if store.next_ts == 0 {
        store.next_ts = 1;
    }
    Ok(store)
}

pub(super) fn timeline_store_save(store: &TimelineStore) -> Result<(), &'static str> {
    let json = serde_json::to_string(store).map_err(|_| "timeline_unavailable")?;
    match vault::secret_set(TIMELINE_SECRET_KEY, &json) {
        Ok(()) => Ok(()),
        Err("vault_missing" | "vault_locked") => Err("timeline_unavailable"),
        Err(_) => Err("timeline_unavailable"),
    }
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

pub(super) fn timeline_append_entry_for_target(
    peer: &str,
    direction: &str,
    byte_len: usize,
    kind: &str,
    final_state: MessageState,
    forced_id: Option<&str>,
    target_device_id: Option<&str>,
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
    let mut store = timeline_store_load()?;
    let ts = store.next_ts;
    store.next_ts = store.next_ts.saturating_add(1);
    let id = forced_id
        .map(|v| v.to_string())
        .unwrap_or_else(|| format!("{}-{}", direction, ts));
    let entry = TimelineEntry {
        id: id.clone(),
        peer: peer.to_string(),
        direction: direction.to_string(),
        byte_len,
        kind: kind.to_string(),
        ts,
        target_device_id: target_device_id.map(short_device_marker),
        state: final_state.as_str().to_string(),
        status: final_state.as_status().to_string(),
    };
    store
        .peers
        .entry(peer.to_string())
        .or_default()
        .push(entry.clone());
    timeline_store_save(&store)?;
    emit_message_state_transition(id.as_str(), MessageState::Created, final_state);
    Ok(entry)
}

fn timeline_transition_entry_state(
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
    emit_message_state_transition(id, from, to);
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

pub(super) fn apply_attachment_peer_confirmation(
    peer: &str,
    attachment_id: &str,
    confirm_handle: &str,
    recv_channel: &str,
) -> Result<(ConfirmApplyOutcome, Option<String>), &'static str> {
    let key = attachment_record_key("out", peer, attachment_id);
    let store = attachment_journal_load()?;
    let Some(rec) = store.records.get(&key) else {
        return Err("REJECT_ATT_CONFIRM_LINKAGE");
    };
    let target = rec.target_device_id.clone();
    if !confirm_target_matches_channel(target.as_deref(), recv_channel) {
        return Ok((ConfirmApplyOutcome::IgnoredWrongDevice, target));
    }
    attachment_transfer_apply_confirmation(peer, attachment_id, confirm_handle, recv_channel)?;
    let timeline_id = attachment_transfer_timeline_id(peer, attachment_id)?;
    timeline_transition_entry_state(peer, timeline_id.as_str(), MessageState::Delivered)?;
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

pub(super) fn timeline_list(peer: &str, limit: Option<usize>) {
    if !require_unlocked("timeline_list") {
        return;
    }
    let mut entries =
        timeline_entries_for_peer(peer).unwrap_or_else(|code| print_error_marker(code));
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
}

pub(super) fn timeline_show(peer: &str, id: &str) {
    if !require_unlocked("timeline_show") {
        return;
    }
    let entries = timeline_entries_for_peer(peer).unwrap_or_else(|code| print_error_marker(code));
    let Some(entry) = entries.into_iter().find(|v| v.id == id) else {
        print_error_marker("timeline_item_missing");
    };
    timeline_emit_item(&entry);
}

pub(super) fn timeline_clear(peer: &str, confirm: bool) {
    if !require_unlocked("timeline_clear") {
        return;
    }
    if !confirm {
        emit_marker(
            "error",
            Some("timeline_clear_confirm_required"),
            &[("peer", peer), ("reason", "explicit_confirm_required")],
        );
        print_error_marker("timeline_clear_confirm_required");
    }
    if !channel_label_ok(peer) {
        print_error_marker("timeline_peer_invalid");
    }
    let mut store = timeline_store_load().unwrap_or_else(|code| print_error_marker(code));
    let removed = store.peers.remove(peer).map(|v| v.len()).unwrap_or(0usize);
    timeline_store_save(&store).unwrap_or_else(|code| print_error_marker(code));
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
}
