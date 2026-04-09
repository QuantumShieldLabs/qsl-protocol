use super::commands::format_message_transcript_line;
use super::*;

impl TuiState {
    pub(in super::super) fn selected_contact_label(&self) -> String {
        if self.contacts.is_empty() {
            "peer-0".to_string()
        } else {
            self.contacts[self
                .contacts_selected
                .min(self.contacts.len().saturating_sub(1))]
            .clone()
        }
    }

    pub(in super::super) fn selected_peer_trust_state(&self) -> &'static str {
        contact_state(self.contact_record_cached(self.session.peer_label))
    }

    pub(in super::super) fn trust_allows_peer_send_strict(
        &mut self,
        peer: &str,
    ) -> Result<(), &'static str> {
        let Some(rec) = self.contact_record_cached(peer) else {
            self.set_command_error("msg: unknown contact; add contact first");
            self.push_cmd_result("msg blocked", false, "unknown contact (add contact first)");
            emit_tui_named_marker(
                "QSC_TUI_SEND_BLOCKED",
                &[("reason", "unknown_contact"), ("peer", peer)],
            );
            emit_tui_trust_remediation("unknown_contact", peer, None);
            self.push_cmd_result(
                "trust remediation",
                false,
                trust_remediation_hint("unknown_contact"),
            );
            self.push_cmd_result(
                "trust remediation",
                false,
                trust_remediation_verify_vs_trusted_hint(),
            );
            emit_marker(
                "tui_msg_reject",
                Some("unknown_contact"),
                &[("reason", "unknown_contact"), ("peer", peer)],
            );
            return Err("unknown_contact");
        };
        let Some(primary) = primary_device(rec).cloned() else {
            self.set_command_error("msg: no trusted device; verify and trust a device first");
            self.push_cmd_result("msg blocked", false, "no trusted device");
            emit_tui_named_marker(
                "QSC_TUI_SEND_BLOCKED",
                &[("reason", "no_trusted_device"), ("peer", peer)],
            );
            emit_tui_trust_remediation("no_trusted_device", peer, None);
            self.push_cmd_result(
                "trust remediation",
                false,
                trust_remediation_hint("no_trusted_device"),
            );
            self.push_cmd_result(
                "trust remediation",
                false,
                trust_remediation_verify_vs_trusted_hint(),
            );
            emit_marker(
                "tui_msg_reject",
                Some("no_trusted_device"),
                &[("reason", "no_trusted_device"), ("peer", peer)],
            );
            return Err("no_trusted_device");
        };
        let primary_device_id = primary.device_id;
        let primary_state = canonical_device_state(primary.state.as_str());
        let has_trusted = contact_has_trusted_device(rec);
        match primary_state {
            "CHANGED" => {
                self.set_command_error(
                    "msg: primary device changed; explicit re-approval required",
                );
                self.push_cmd_result("msg blocked", false, "primary device changed");
                emit_tui_named_marker(
                    "QSC_TUI_SEND_BLOCKED",
                    &[
                        ("reason", "device_changed_reapproval_required"),
                        ("peer", peer),
                    ],
                );
                emit_tui_trust_remediation(
                    "device_changed_reapproval_required",
                    peer,
                    Some(primary_device_id.as_str()),
                );
                self.push_cmd_result(
                    "trust remediation",
                    false,
                    trust_remediation_hint("device_changed_reapproval_required"),
                );
                self.push_cmd_result(
                    "trust remediation",
                    false,
                    trust_remediation_verify_vs_trusted_hint(),
                );
                emit_marker(
                    "tui_msg_reject",
                    Some("device_changed_reapproval_required"),
                    &[
                        ("reason", "device_changed_reapproval_required"),
                        ("peer", peer),
                    ],
                );
                return Err("device_changed_reapproval_required");
            }
            "REVOKED" => {
                self.set_command_error(
                    "msg: primary device revoked; select/re-approve a trusted device",
                );
                self.push_cmd_result("msg blocked", false, "primary device revoked");
                emit_tui_named_marker(
                    "QSC_TUI_SEND_BLOCKED",
                    &[("reason", "device_revoked"), ("peer", peer)],
                );
                emit_tui_trust_remediation(
                    "device_revoked",
                    peer,
                    Some(primary_device_id.as_str()),
                );
                self.push_cmd_result(
                    "trust remediation",
                    false,
                    trust_remediation_hint("device_revoked"),
                );
                self.push_cmd_result(
                    "trust remediation",
                    false,
                    trust_remediation_verify_vs_trusted_hint(),
                );
                emit_marker(
                    "tui_msg_reject",
                    Some("device_revoked"),
                    &[("reason", "device_revoked"), ("peer", peer)],
                );
                return Err("device_revoked");
            }
            _ => {}
        }
        if !has_trusted {
            self.set_command_error("msg: no trusted device; verify and trust a device first");
            self.push_cmd_result("msg blocked", false, "no trusted device");
            emit_tui_named_marker(
                "QSC_TUI_SEND_BLOCKED",
                &[("reason", "no_trusted_device"), ("peer", peer)],
            );
            emit_tui_trust_remediation("no_trusted_device", peer, Some(primary_device_id.as_str()));
            self.push_cmd_result(
                "trust remediation",
                false,
                trust_remediation_hint("no_trusted_device"),
            );
            self.push_cmd_result(
                "trust remediation",
                false,
                trust_remediation_verify_vs_trusted_hint(),
            );
            emit_marker(
                "tui_msg_reject",
                Some("no_trusted_device"),
                &[("reason", "no_trusted_device"), ("peer", peer)],
            );
            return Err("no_trusted_device");
        }
        Ok(())
    }

    pub(in super::super) fn focus_messages_thread(&mut self, peer: &str) {
        self.ensure_conversation(peer);
        let labels = self.conversation_labels();
        if let Some(idx) = labels.iter().position(|p| p == peer) {
            self.conversation_selected = idx;
        }
        self.inspector = TuiInspectorPane::Events;
        self.mode = TuiMode::Normal;
        self.home_focus = TuiHomeFocus::Nav;
        self.set_active_peer(peer);
        self.sync_messages_if_main_focused();
        emit_tui_named_marker("QSC_TUI_NAV", &[("focus", "messages"), ("thread", peer)]);
    }

    pub(in super::super) fn selected_peer_identity_short(&self) -> String {
        self.contact_record_cached(self.session.peer_label)
            .map(|rec| short_identity_display(rec.fp.as_str()))
            .unwrap_or_else(|| "untrusted".to_string())
    }

    pub(crate) fn contact_record_cached(&self, label: &str) -> Option<&ContactRecord> {
        self.contacts_records.get(label)
    }

    pub(in super::super) fn contact_display_line_cached(&self, label: &str) -> String {
        label.to_string()
    }

    pub(in super::super) fn persist_contacts_cache(&mut self) -> Result<(), ErrorCode> {
        let mut store = ContactsStore {
            peers: self.contacts_records.clone(),
        };
        for (alias, rec) in store.peers.iter_mut() {
            normalize_contact_record(alias.as_str(), rec);
        }
        let json = serde_json::to_string(&store).map_err(|_| ErrorCode::ParseFailed)?;
        self.persist_account_secret(CONTACTS_SECRET_KEY, json.as_str())
            .map_err(|_| ErrorCode::IoWriteFailed)?;
        self.contacts_records = store.peers;
        Ok(())
    }

    pub(in super::super) fn persist_contacts_cache_with(
        &mut self,
        label: &str,
        mut rec: ContactRecord,
    ) -> Result<(), ErrorCode> {
        normalize_contact_record(label, &mut rec);
        self.contacts_records.insert(label.to_string(), rec);
        self.persist_contacts_cache()
    }

    pub(in super::super) fn tui_relay_inbox_route_token(&self) -> Result<String, &'static str> {
        // Reuse the shared vault helper so TUI and CLI resolve the persisted inbox
        // token through the same path.
        relay_self_inbox_route_token()
    }

    pub(in super::super) fn tui_timeline_store_load(&self) -> Result<TimelineStore, &'static str> {
        let raw = if let Some(session) = self.vault_session.as_ref() {
            vault::session_get(session, TIMELINE_SECRET_KEY).map_err(|_| "timeline_tampered")?
        } else {
            None
        };
        let mut store = raw
            .map(|encoded| {
                serde_json::from_str::<TimelineStore>(encoded.as_str())
                    .map_err(|_| "timeline_tampered")
            })
            .transpose()?
            .unwrap_or_default();
        if store.next_ts == 0 {
            store.next_ts = 1;
        }
        Ok(store)
    }

    pub(in super::super) fn tui_timeline_store_save(
        &mut self,
        store: &TimelineStore,
    ) -> Result<(), &'static str> {
        let json = serde_json::to_string(store).map_err(|_| "timeline_unavailable")?;
        self.persist_account_secret(TIMELINE_SECRET_KEY, json.as_str())
            .map_err(|_| "timeline_unavailable")
    }

    pub(in super::super) fn append_tui_timeline_entry(
        &mut self,
        peer: &str,
        direction: &str,
        byte_len: usize,
        kind: &str,
        final_state: MessageState,
    ) -> Result<(), &'static str> {
        if !channel_label_ok(peer) {
            return Err("timeline_peer_invalid");
        }
        message_state_transition_allowed(MessageState::Created, final_state, direction)?;
        let mut store = self.tui_timeline_store_load()?;
        let ts = store.next_ts;
        store.next_ts = store.next_ts.saturating_add(1);
        let id = format!("{}-{}", direction, ts);
        let entry = TimelineEntry {
            id: id.clone(),
            peer: peer.to_string(),
            direction: direction.to_string(),
            byte_len,
            kind: kind.to_string(),
            ts,
            target_device_id: None,
            state: final_state.as_str().to_string(),
            status: final_state.as_status().to_string(),
        };
        store.peers.entry(peer.to_string()).or_default().push(entry);
        self.tui_timeline_store_save(&store)?;
        emit_message_state_transition(id.as_str(), MessageState::Created, final_state);
        Ok(())
    }

    pub(in super::super) fn selected_file_id(&self) -> Option<&str> {
        self.files
            .get(self.file_selected.min(self.files.len().saturating_sub(1)))
            .map(|v| v.id.as_str())
    }

    pub(in super::super) fn refresh_file_selection_bounds(&mut self) {
        if self.file_selected >= self.files.len() {
            self.file_selected = self.files.len().saturating_sub(1);
        }
        if self.files.is_empty() {
            self.file_selected = 0;
            self.file_multi_selected.clear();
            return;
        }
        self.file_multi_selected
            .retain(|id| self.files.iter().any(|f| &f.id == id));
    }

    pub(in super::super) fn upsert_file_item(&mut self, item: TuiFileItem, from_update: bool) {
        let mut changed = false;
        if let Some(existing) = self.files.iter_mut().find(|v| v.id == item.id) {
            if existing != &item {
                *existing = item;
                changed = true;
            }
        } else {
            self.files.push(item);
            changed = true;
        }
        self.files.sort_by(|a, b| a.id.cmp(&b.id));
        self.refresh_file_selection_bounds();
        if from_update
            && changed
            && !(self.mode == TuiMode::Normal
                && self.inspector == TuiInspectorPane::Files
                && self.home_focus == TuiHomeFocus::Main)
        {
            self.file_unseen_updates = self.file_unseen_updates.saturating_add(1);
        }
    }

    pub(in super::super) fn refresh_files_from_timeline(&mut self) {
        for item in load_tui_files_snapshot() {
            self.upsert_file_item(item, true);
        }
    }

    pub(in super::super) fn files_select_by_id(&mut self, id: &str) -> bool {
        if let Some(idx) = self.files.iter().position(|v| v.id == id) {
            self.file_selected = idx;
            true
        } else {
            false
        }
    }

    pub(in super::super) fn files_toggle_selected(&mut self) -> bool {
        let Some(id) = self.selected_file_id().map(str::to_string) else {
            return false;
        };
        if self.file_multi_selected.contains(id.as_str()) {
            self.file_multi_selected.remove(id.as_str());
        } else {
            self.file_multi_selected.insert(id);
        }
        true
    }

    pub(in super::super) fn files_move(&mut self, delta: i32) {
        if self.files.is_empty() {
            self.file_selected = 0;
            return;
        }
        let max = (self.files.len() - 1) as i32;
        let mut idx = self.file_selected as i32 + delta;
        if idx < 0 {
            idx = 0;
        }
        if idx > max {
            idx = max;
        }
        self.file_selected = idx as usize;
    }

    pub(in super::super) fn set_active_peer(&mut self, peer: &str) {
        self.session.peer_label = Box::leak(peer.to_string().into_boxed_str());
        self.refresh_qsp_status();
    }

    pub(in super::super) fn sync_messages_if_main_focused(&mut self) {
        if self.mode != TuiMode::Normal
            || self.inspector != TuiInspectorPane::Events
            || self.home_focus != TuiHomeFocus::Main
        {
            return;
        }
        let peer = self.selected_conversation_label();
        let total = self
            .conversations
            .get(peer.as_str())
            .map(|v| v.len())
            .unwrap_or(0usize);
        self.visible_counts.insert(peer.clone(), total);
        self.unread_counts.insert(peer, 0);
    }

    pub(in super::super) fn sync_files_if_main_focused(&mut self) {
        if self.mode != TuiMode::Normal
            || self.inspector != TuiInspectorPane::Files
            || self.home_focus != TuiHomeFocus::Main
        {
            return;
        }
        self.file_unseen_updates = 0;
    }

    pub(in super::super) fn sync_activity_if_main_focused(&mut self) {
        if self.mode != TuiMode::Normal
            || self.inspector != TuiInspectorPane::Activity
            || self.home_focus != TuiHomeFocus::Main
        {
            return;
        }
        self.activity_visible_count = self.events.len();
        self.activity_unseen_updates = 0;
    }

    pub(in super::super) fn record_message_line(
        &mut self,
        peer: &str,
        state: &str,
        direction: &str,
        detail: &str,
    ) {
        self.ensure_conversation(peer);
        let line = format_message_transcript_line(peer, state, direction, detail);
        {
            let stream = self.conversations.entry(peer.to_string()).or_default();
            stream.push_back(line);
            if stream.len() > 128 {
                stream.pop_front();
            }
        }
        let selected = self.selected_conversation_label();
        let auto_append = self.mode == TuiMode::Normal
            && self.inspector == TuiInspectorPane::Events
            && self.home_focus == TuiHomeFocus::Main
            && selected == peer;
        let total = self
            .conversations
            .get(peer)
            .map(|v| v.len())
            .unwrap_or(0usize);
        if auto_append {
            self.visible_counts.insert(peer.to_string(), total);
            self.unread_counts.insert(peer.to_string(), 0);
        } else {
            let unread = self
                .unread_counts
                .get(peer)
                .copied()
                .unwrap_or(0usize)
                .saturating_add(1);
            self.unread_counts.insert(peer.to_string(), unread);
            self.visible_counts
                .entry(peer.to_string())
                .or_insert(total.saturating_sub(1));
        }
        let total_s = total.to_string();
        let unread_s = self
            .unread_counts
            .get(peer)
            .copied()
            .unwrap_or(0)
            .to_string();
        emit_marker(
            "tui_message_event",
            None,
            &[
                ("peer", peer),
                ("state", state),
                ("mode", if auto_append { "append" } else { "buffer" }),
                ("total", total_s.as_str()),
                ("unread", unread_s.as_str()),
            ],
        );
        if let Some(delivery) = message_delivery_semantic_from_state_str(direction, state) {
            emit_tui_delivery_state(peer, delivery);
        }
    }

    pub(in super::super) fn selected_messages_thread(&self) -> Option<String> {
        if self.inspector != TuiInspectorPane::Events {
            return None;
        }
        Some(self.selected_conversation_label())
    }

    pub(in super::super) fn map_thread_to_timeline_peer(thread: &str) -> &str {
        if thread == TUI_NOTE_TO_SELF_LABEL {
            TUI_NOTE_TO_SELF_TIMELINE_PEER
        } else {
            thread
        }
    }

    pub(in super::super) fn update_send_lifecycle(&mut self, value: &str) {
        self.send_lifecycle = value.to_string();
        self.status.send_lifecycle = Box::leak(self.send_lifecycle.clone().into_boxed_str());
        emit_marker(
            "tui_status_update",
            None,
            &[("field", "send_lifecycle"), ("value", value)],
        );
    }

    pub(in super::super) fn refresh_envelope(&mut self, payload_len: usize) {
        self.last_payload_len = payload_len;
        self.envelope = compute_envelope_status(payload_len);
        self.status.envelope = Box::leak(self.envelope.clone().into_boxed_str());
        emit_marker(
            "tui_status_update",
            None,
            &[("field", "envelope"), ("value", &self.envelope)],
        );
    }

    pub(in super::super) fn refresh_qsp_status(&mut self) {
        let peer = self.session.peer_label;
        self.qsp_status = qsp_status_string(peer);
        self.status.qsp = Box::leak(self.qsp_status.clone().into_boxed_str());
        let peer_fp = compute_peer_fingerprint(peer);
        self.status.peer_fp = Box::leak(peer_fp.into_boxed_str());
        self.refresh_contacts();
        emit_marker(
            "tui_status_update",
            None,
            &[("field", "qsp"), ("value", &self.qsp_status)],
        );
    }

    pub(in super::super) fn refresh_contacts(&mut self) {
        self.contacts = self.contacts_records.keys().cloned().collect::<Vec<_>>();
        self.contacts.sort();
        if self.contacts.is_empty() {
            self.contacts.push("peer-0".to_string());
        }
        if self.contacts_selected >= self.contacts.len() {
            self.contacts_selected = self.contacts.len().saturating_sub(1);
        }
        let labels = self.conversation_labels();
        if self.conversation_selected >= labels.len() {
            self.conversation_selected = labels.len().saturating_sub(1);
        }
    }

    pub(in super::super) fn push_event(&mut self, kind: &str, action: &str) {
        self.event_seq = self.event_seq.wrapping_add(1);
        let seq_s = self.event_seq.to_string();
        emit_marker(
            "tui_event",
            None,
            &[("kind", kind), ("action", action), ("seq", seq_s.as_str())],
        );
        let line = format!("{}:{} #{}", kind, action, self.event_seq);
        self.events.push_back(line);
        if self.events.len() > 64 {
            self.events.pop_front();
        }
        self.record_activity_update();
    }

    pub(in super::super) fn push_event_line(&mut self, line: String) {
        self.events.push_back(line);
        if self.events.len() > 64 {
            self.events.pop_front();
        }
        self.record_activity_update();
    }

    pub(in super::super) fn record_activity_update(&mut self) {
        let total = self.events.len();
        let auto_append = self.mode == TuiMode::Normal
            && self.inspector == TuiInspectorPane::Activity
            && self.home_focus == TuiHomeFocus::Main;
        if auto_append {
            self.activity_visible_count = total;
            self.activity_unseen_updates = 0;
            return;
        }
        self.activity_unseen_updates = self.activity_unseen_updates.saturating_add(1);
        if self.activity_visible_count == 0 && total > 0 {
            self.activity_visible_count = total.saturating_sub(1);
        } else if self.activity_visible_count > total {
            self.activity_visible_count = total;
        }
    }
}
