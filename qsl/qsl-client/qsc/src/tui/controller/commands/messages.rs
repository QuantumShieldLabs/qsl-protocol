use super::*;

pub(super) fn dispatch_messages_command(cmd: &TuiParsedCmd, state: &mut TuiState) -> bool {
    match cmd.cmd.as_str() {
        "messages" => handle_messages_command(cmd, state),
        "msg" => handle_msg_command(cmd, state),
        "files" => handle_files_command(cmd, state),
        "injectmsg" => handle_injectmsg_command(cmd, state),
        "injectevent" => handle_injectevent_command(cmd, state),
        "envelope" => {
            emit_marker("tui_cmd", None, &[("cmd", "envelope")]);
            state.refresh_envelope(state.last_payload_len());
            false
        }
        "export" => {
            emit_marker("tui_cmd", None, &[("cmd", "export")]);
            false
        }
        _ => {
            state.set_command_error(format!("unknown command: {}", cmd.cmd));
            emit_marker("tui_cmd", None, &[("cmd", cmd.cmd.as_str())]);
            false
        }
    }
}

fn handle_messages_command(cmd: &TuiParsedCmd, state: &mut TuiState) -> bool {
    emit_marker("tui_cmd", None, &[("cmd", "messages")]);
    let sub = cmd.args.first().map(|s| s.as_str()).unwrap_or("list");
    match sub {
        "list" => {
            let labels = state.conversation_labels();
            let count_s = labels.len().to_string();
            emit_marker("tui_messages_list", None, &[("count", count_s.as_str())]);
        }
        "select" => {
            let Some(peer) = cmd.args.get(1).map(|s| s.as_str()) else {
                state.set_command_error("messages: missing peer");
                emit_marker("tui_messages_invalid", None, &[("reason", "missing_peer")]);
                return false;
            };
            let labels = state.conversation_labels();
            let known_contact = state.contact_record_cached(peer).is_some();
            if labels.iter().any(|p| p == peer) || known_contact {
                state.focus_messages_thread(peer);
                emit_marker(
                    "tui_messages_select",
                    None,
                    &[("peer", peer), ("ok", "true")],
                );
            } else {
                emit_marker(
                    "tui_messages_select",
                    None,
                    &[("peer", peer), ("ok", "false")],
                );
            }
        }
        _ => {
            state.set_command_error("messages: unknown subcommand");
            emit_marker(
                "tui_messages_invalid",
                None,
                &[("reason", "unknown_subcmd")],
            );
        }
    }
    false
}

fn handle_msg_command(cmd: &TuiParsedCmd, state: &mut TuiState) -> bool {
    emit_marker("tui_cmd", None, &[("cmd", "msg")]);
    let (thread, text) = if cmd.args.len() >= 2 {
        let peer = cmd.args[0].clone();
        let body = cmd.args[1..].join(" ");
        (peer, body)
    } else {
        let Some(selected) = state.selected_messages_thread() else {
            state.set_command_error("msg: select a messages thread first");
            emit_marker("tui_msg_reject", None, &[("reason", "thread_not_selected")]);
            return false;
        };
        (selected, cmd.args.join(" "))
    };
    let trimmed = text.trim();
    if trimmed.is_empty() {
        state.set_command_error("msg: empty message");
        emit_marker("tui_msg_reject", None, &[("reason", "empty")]);
        return false;
    }
    if trimmed.chars().count() > TUI_MESSAGE_MAX_CHARS {
        state.set_command_error("msg: message too long");
        emit_marker("tui_msg_reject", None, &[("reason", "too_long")]);
        return false;
    }
    if thread.eq_ignore_ascii_case("Note to Self") || thread.eq_ignore_ascii_case("Note_to_Self") {
        let timeline_peer = TuiState::map_thread_to_timeline_peer(thread.as_str());
        if state
            .append_tui_timeline_entry(
                timeline_peer,
                "out",
                trimmed.len(),
                "msg",
                MessageState::Sent,
            )
            .is_err()
        {
            state.set_command_error("msg: timeline unavailable");
            emit_marker(
                "tui_msg_reject",
                None,
                &[("reason", "timeline_unavailable")],
            );
            return false;
        }
        state.record_message_line(thread.as_str(), "SENT", "out", trimmed);
        state.focus_messages_thread(thread.as_str());
        emit_tui_delivery_state(thread.as_str(), "accepted_by_relay");
        let len_s = trimmed.len().to_string();
        emit_marker(
            "tui_msg_send",
            None,
            &[
                ("peer", thread.as_str()),
                ("len", len_s.as_str()),
                ("ok", "true"),
            ],
        );
        return false;
    }
    if let Err(reason) = state.trust_allows_peer_send_strict(thread.as_str()) {
        if reason == "no_trusted_device"
            && tui_msg_autotrust_first_use(state, thread.as_str()).is_ok()
        {
            if state
                .trust_allows_peer_send_strict(thread.as_str())
                .is_err()
            {
                return false;
            }
        } else {
            return false;
        }
    }
    let effective_relay = state.effective_relay_config();
    if effective_relay.is_none() {
        state.emit_setup_required_marker_if_needed();
        state.set_command_error(
            "msg: relay not configured; use '/relay set endpoint <https://...>'",
        );
        emit_marker(
            "tui_msg_reject",
            None,
            &[("reason", "explicit_only_no_transport")],
        );
        return false;
    }
    if let Err(code) = tui_msg_ensure_handshake(state, thread.as_str()) {
        emit_marker(
            "tui_msg_reject",
            Some(code),
            &[("reason", code), ("peer", thread.as_str())],
        );
        state.set_command_error("msg: handshake failed");
        return false;
    }
    let relay = match effective_relay.as_ref() {
        Some(v) => v,
        None => return false,
    };
    let routing = match tui_resolve_peer_device_target(state, thread.as_str(), true) {
        Ok(v) => v,
        Err(code) => {
            emit_marker(
                "tui_msg_reject",
                Some(code),
                &[("reason", code), ("peer", thread.as_str())],
            );
            state.set_command_error("msg: send blocked");
            return false;
        }
    };
    if let Err(code) = tui_enforce_peer_not_blocked(state, routing.channel.as_str()) {
        emit_marker(
            "tui_msg_reject",
            Some(code),
            &[("reason", code), ("peer", thread.as_str())],
        );
        state.set_command_error("msg: send blocked");
        return false;
    }
    let outcome = transport::relay_send_with_payload(RelaySendPayloadArgs {
        to: thread.as_str(),
        payload: trimmed.as_bytes().to_vec(),
        relay: relay.relay.as_str(),
        injector: transport::fault_injector_from_tui(relay),
        pad_cfg: None,
        bucket_max: None,
        meta_seed: None,
        receipt: None,
        routing_override: Some(routing),
        tui_thread: Some(thread.as_str()),
    });
    if !outcome.delivered {
        state.set_command_error("msg: send failed");
        emit_tui_named_marker("QSC_TUI_ORCH", &[("stage", "send"), ("status", "fail")]);
        return false;
    }
    emit_tui_named_marker("QSC_TUI_ORCH", &[("stage", "send"), ("status", "ok")]);
    let timeline_peer = TuiState::map_thread_to_timeline_peer(thread.as_str());
    if state
        .append_tui_timeline_entry(
            timeline_peer,
            "out",
            trimmed.len(),
            "msg",
            MessageState::Sent,
        )
        .is_err()
    {
        state.set_command_error("msg: timeline unavailable");
        emit_marker(
            "tui_msg_reject",
            None,
            &[("reason", "timeline_unavailable")],
        );
        return false;
    }
    state.record_message_line(thread.as_str(), "SENT", "out", trimmed);
    state.focus_messages_thread(thread.as_str());
    tui_msg_recv_poll_bounded(state, thread.as_str());
    let len_s = trimmed.len().to_string();
    emit_marker(
        "tui_msg_send",
        None,
        &[
            ("peer", thread.as_str()),
            ("len", len_s.as_str()),
            ("ok", "true"),
        ],
    );
    false
}

fn handle_files_command(cmd: &TuiParsedCmd, state: &mut TuiState) -> bool {
    emit_marker("tui_cmd", None, &[("cmd", "files")]);
    state.refresh_files_from_timeline();
    let sub = cmd.args.first().map(|s| s.as_str()).unwrap_or("list");
    match sub {
        "list" => {
            emit_marker(
                "tui_files_list",
                None,
                &[("count", state.files.len().to_string().as_str())],
            );
        }
        "select" => {
            let Some(id) = cmd.args.get(1).map(|s| s.as_str()) else {
                state.set_command_error("files: missing id");
                emit_marker("tui_files_invalid", None, &[("reason", "missing_id")]);
                return false;
            };
            let ok = state.files_select_by_id(id);
            emit_marker(
                "tui_files_select",
                None,
                &[("id", id), ("ok", if ok { "true" } else { "false" })],
            );
        }
        "toggle" => {
            if state.inspector != TuiInspectorPane::Files {
                state.set_command_error("files: multiselect only allowed in files");
                emit_marker(
                    "tui_files_multiselect_blocked",
                    None,
                    &[
                        ("reason", "domain_not_files"),
                        ("domain", state.inspector_name()),
                    ],
                );
                return false;
            }
            if let Some(id) = cmd.args.get(1).map(|s| s.as_str()) {
                if !state.files_select_by_id(id) {
                    state.set_command_error("files: unknown id");
                    emit_marker(
                        "tui_files_toggle",
                        None,
                        &[("id", id), ("ok", "false"), ("reason", "unknown_id")],
                    );
                    return false;
                }
            }
            let ok = state.files_toggle_selected();
            emit_marker(
                "tui_files_toggle",
                None,
                &[
                    ("id", state.selected_file_id().unwrap_or("none")),
                    ("ok", if ok { "true" } else { "false" }),
                    (
                        "selected",
                        state.file_multi_selected.len().to_string().as_str(),
                    ),
                ],
            );
        }
        "clear-selection" => {
            state.file_multi_selected.clear();
            emit_marker(
                "tui_files_clear_selection",
                None,
                &[("ok", "true"), ("selected", "0")],
            );
        }
        "inject" => {
            let Some(id) = cmd.args.get(1).map(|s| s.as_str()) else {
                state.set_command_error("files: missing id");
                emit_marker("tui_files_invalid", None, &[("reason", "missing_id")]);
                return false;
            };
            let state_name = cmd.args.get(2).map(|s| s.as_str()).unwrap_or("RECEIVING");
            let byte_len = cmd
                .args
                .get(3)
                .and_then(|v| v.parse::<usize>().ok())
                .unwrap_or(0usize);
            let filename = cmd
                .args
                .get(4)
                .map(|s| s.to_string())
                .unwrap_or_else(|| format!("{}.bin", id));
            let item = TuiFileItem {
                id: id.to_string(),
                peer: state.selected_conversation_label(),
                filename,
                byte_len,
                state: state_name.to_string(),
                display_state: tui_file_display_state(state_name),
            };
            state.upsert_file_item(item, true);
            emit_marker(
                "tui_files_inject",
                None,
                &[
                    ("id", id),
                    ("state", state_name),
                    ("display_state", tui_file_display_state(state_name).as_str()),
                ],
            );
        }
        _ => {
            state.set_command_error("files: unknown subcommand");
            emit_marker("tui_files_invalid", None, &[("reason", "unknown_subcmd")]);
        }
    }
    false
}

fn handle_injectmsg_command(cmd: &TuiParsedCmd, state: &mut TuiState) -> bool {
    emit_marker("tui_cmd", None, &[("cmd", "injectmsg")]);
    let peer = cmd.args.first().map(|s| s.as_str()).unwrap_or("peer-0");
    let state_name = cmd.args.get(1).map(|s| s.as_str()).unwrap_or("RECEIVED");
    let detail = if cmd.args.len() > 2 {
        cmd.args[2..].join(" ")
    } else {
        "source=test_harness".to_string()
    };
    let _ =
        state.append_tui_timeline_entry(peer, "in", detail.len(), "msg", MessageState::Received);
    state.record_message_line(peer, state_name, "in", detail.as_str());
    false
}

fn handle_injectevent_command(cmd: &TuiParsedCmd, state: &mut TuiState) -> bool {
    emit_marker("tui_cmd", None, &[("cmd", "injectevent")]);
    let kind = cmd.args.first().map(|s| s.as_str()).unwrap_or("activity");
    let action = cmd.args.get(1).map(|s| s.as_str()).unwrap_or("test");
    state.push_event(kind, action);
    false
}
