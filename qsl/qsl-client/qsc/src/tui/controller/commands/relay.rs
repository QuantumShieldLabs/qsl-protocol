use super::*;

pub(super) fn tui_msg_ensure_handshake(
    state: &mut TuiState,
    peer: &str,
) -> Result<(), &'static str> {
    let routing = resolve_peer_device_target(peer, false)?;
    let inbox_route_token = state.tui_relay_inbox_route_token()?;
    if protocol_active_or_reason_for_peer(routing.channel.as_str()).is_ok() {
        emit_tui_named_marker(
            "QSC_TUI_ORCH",
            &[("stage", "ensure_handshake"), ("status", "skip")],
        );
        return Ok(());
    }
    let relay = state
        .effective_relay_config()
        .ok_or("explicit_only_no_transport")?
        .relay;
    let self_label = env::var("QSC_SELF_LABEL").unwrap_or_else(|_| "self".to_string());
    let backoff_ms = [50u64, 100, 200];
    let _ = inbox_route_token;
    handshake_init(&self_label, peer, relay.as_str());
    for delay in backoff_ms {
        handshake_poll(&self_label, peer, relay.as_str(), 4);
        let routing = resolve_peer_device_target(peer, false)?;
        if protocol_active_or_reason_for_peer(routing.channel.as_str()).is_ok() {
            emit_tui_named_marker(
                "QSC_TUI_ORCH",
                &[("stage", "ensure_handshake"), ("status", "ok")],
            );
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(delay));
    }
    emit_tui_named_marker(
        "QSC_TUI_ORCH_FAIL",
        &[("stage", "handshake"), ("code", "handshake_incomplete")],
    );
    Err("handshake_incomplete")
}

pub(super) fn tui_msg_autotrust_first_use(
    state: &mut TuiState,
    peer: &str,
) -> Result<(), &'static str> {
    let mode = state.trust_onboarding_mode;
    let Some(rec) = state.contacts_records.get(peer) else {
        return Err("unknown_contact");
    };
    let Some(primary) = primary_device(rec) else {
        return Err("no_trusted_device");
    };
    let primary_state = canonical_device_state(primary.state.as_str());
    if primary_state == "CHANGED" {
        return Err("device_changed_reapproval_required");
    }
    if primary_state == "REVOKED" {
        return Err("device_revoked");
    }
    if primary_state != "UNVERIFIED" && primary_state != "VERIFIED" {
        return Err("no_trusted_device");
    }
    if mode == TrustOnboardingMode::Strict {
        emit_cli_trust_promotion("verified_only", "strict_mode", peer, None, mode);
        emit_tui_trust_promotion("verified_only", "strict_mode", peer, None, mode);
        return Err("no_trusted_device");
    }
    if tui_msg_ensure_handshake(state, peer).is_err() {
        return Err("handshake_incomplete");
    }
    let mut rec = match state.contacts_records.get(peer) {
        Some(v) => v.clone(),
        None => return Err("unknown_contact"),
    };
    normalize_contact_record(peer, &mut rec);
    let Some(primary) = primary_device_mut(&mut rec) else {
        return Err("no_trusted_device");
    };
    let state_now = canonical_device_state(primary.state.as_str());
    if state_now == "CHANGED" || state_now == "REVOKED" {
        return Err("device_changed_reapproval_required");
    }
    primary.state = "TRUSTED".to_string();
    if state.persist_contacts_cache_with(peer, rec).is_err() {
        emit_tui_named_marker(
            "QSC_TUI_ORCH_FAIL",
            &[
                ("stage", "auto_trust"),
                ("code", "contacts_store_unavailable"),
            ],
        );
        return Err("contacts_store_unavailable");
    }
    emit_cli_trust_promotion("trusted", "verified_match", peer, None, mode);
    emit_tui_trust_promotion("trusted", "verified_match", peer, None, mode);
    emit_tui_named_marker("QSC_TUI_ORCH", &[("stage", "auto_trust"), ("status", "ok")]);
    Ok(())
}

pub(super) fn tui_msg_recv_poll_bounded(state: &mut TuiState, peer: &str) {
    let mut got_total = 0usize;
    let mut polls = 0usize;
    for delay in [0u64, 50, 100] {
        if delay > 0 {
            std::thread::sleep(Duration::from_millis(delay));
        }
        let before = state.session.recv_count;
        tui_receive_via_relay(state, peer);
        polls = polls.saturating_add(1);
        let after = state.session.recv_count;
        if after > before {
            got_total = got_total.saturating_add((after - before) as usize);
        }
        if got_total > 0 {
            break;
        }
    }
    let polls_s = polls.to_string();
    let got_s = got_total.to_string();
    emit_tui_named_marker(
        "QSC_TUI_ORCH",
        &[
            ("stage", "recv_poll"),
            ("status", "ok"),
            ("polls", polls_s.as_str()),
            ("got", got_s.as_str()),
        ],
    );
}

pub(super) fn tui_send_via_relay(state: &mut TuiState, to: &str) {
    let relay = match state.effective_relay_config() {
        Some(v) => v,
        None => {
            emit_marker(
                "tui_send_blocked",
                None,
                &[("reason", "explicit_only_no_transport")],
            );
            state.update_send_lifecycle("blocked");
            return;
        }
    };
    let routing = match tui_resolve_peer_device_target(state, to, true) {
        Ok(v) => v,
        Err(code) => {
            emit_marker("tui_send_blocked", Some(code), &[("reason", code)]);
            state.update_send_lifecycle("blocked");
            return;
        }
    };
    if let Err(code) = tui_enforce_peer_not_blocked(state, routing.channel.as_str()) {
        emit_marker("tui_send_blocked", Some(code), &[("reason", code)]);
        state.update_send_lifecycle("blocked");
        return;
    }
    if let Err(reason) = protocol_active_or_reason_for_peer(routing.channel.as_str()) {
        emit_protocol_inactive(reason.as_str());
        state.update_send_lifecycle("blocked");
        return;
    }
    let payload = tui_payload_bytes(state.send_seq);
    state.send_seq = state.send_seq.wrapping_add(1);
    let outcome = transport::relay_send_with_payload(RelaySendPayloadArgs {
        to,
        payload,
        relay: relay.relay.as_str(),
        injector: transport::fault_injector_from_tui(&relay),
        pad_cfg: None,
        bucket_max: None,
        meta_seed: None,
        receipt: None,
        routing_override: Some(routing),
        tui_thread: Some(to),
    });
    state.push_event("relay_event", outcome.action.as_str());
    if outcome.delivered {
        state.update_send_lifecycle("committed");
        state.session.sent_count = state.session.sent_count.saturating_add(1);
        state.record_message_line(to, "SENT", "out", "transport=relay");
    } else {
        state.update_send_lifecycle("failed");
    }
    state.refresh_qsp_status();
}

fn resolve_receive_out_dir() -> (PathBuf, ConfigSource) {
    if let Ok(dir) = env::var("QSC_RECEIVE_OUT") {
        return (PathBuf::from(dir), ConfigSource::EnvOverride);
    }
    let (dir, source) = match config_dir() {
        Ok(v) => v,
        Err(e) => print_error(e),
    };
    (dir.join("inbox"), source)
}

pub(in super::super) fn tui_receive_via_relay(state: &mut TuiState, from: &str) {
    let relay = match state.effective_relay_config() {
        Some(v) => v,
        None => {
            emit_marker(
                "tui_receive_blocked",
                None,
                &[("reason", "explicit_only_no_transport")],
            );
            return;
        }
    };
    if let Err(code) = normalize_relay_endpoint(relay.relay.as_str()) {
        emit_marker("tui_receive_blocked", None, &[("reason", code)]);
        return;
    }
    let (out_dir, source) = resolve_receive_out_dir();
    if let Err(e) = ensure_dir_secure(&out_dir, source) {
        print_error(e);
    }
    let (cfg_dir, cfg_source) = match config_dir() {
        Ok(v) => v,
        Err(e) => print_error(e),
    };
    if !check_symlink_safe(&cfg_dir) {
        print_error(ErrorCode::UnsafePathSymlink);
    }
    if !check_parent_safe(&cfg_dir, cfg_source) {
        print_error(ErrorCode::UnsafeParentPerms);
    }
    if let Err(reason) = protocol_active_or_reason_for_peer(from) {
        emit_protocol_inactive(reason.as_str());
        emit_marker("tui_receive", None, &[("from", from), ("count", "0")]);
        state.push_event("recv_blocked", reason.as_str());
        return;
    }
    let inbox_route_token = match state.tui_relay_inbox_route_token() {
        Ok(v) => v,
        Err(code) => {
            emit_marker("tui_receive_blocked", None, &[("reason", code)]);
            state.push_event("recv_blocked", code);
            return;
        }
    };
    let mailbox_hash = route_token_hash8(inbox_route_token.as_str());
    emit_marker(
        "recv_start",
        None,
        &[
            ("transport", "relay"),
            ("mailbox", "redacted"),
            ("mailbox_hash", mailbox_hash.as_str()),
            ("from", from),
            ("max", "1"),
        ],
    );
    let max = 1usize;
    let items = match transport::relay_inbox_pull(&relay.relay, inbox_route_token.as_str(), max) {
        Ok(v) => v,
        Err(code) => print_error_marker(code),
    };
    if items.is_empty() {
        emit_marker("recv_none", None, &[]);
        emit_marker("tui_receive", None, &[("from", from), ("count", "0")]);
        state.push_event("recv_none", from);
        return;
    }
    let mut count = 0usize;
    for item in items {
        match qsp_unpack_for_peer(from, &item.data) {
            Ok((outcome, channel)) => {
                record_qsp_status(&cfg_dir, cfg_source, true, "unpack_ok", false, true);
                emit_marker("qsp_unpack", None, &[("ok", "true"), ("version", "5.0")]);
                let msg_idx_s = outcome.msg_idx.to_string();
                emit_marker(
                    "ratchet_recv_advance",
                    None,
                    &[("msg_idx", msg_idx_s.as_str())],
                );
                if outcome.skip_delta > 0 {
                    let sd = outcome.skip_delta.to_string();
                    emit_marker("ratchet_skip_store", None, &[("count", sd.as_str())]);
                }
                if outcome.evicted > 0 {
                    let ev = outcome.evicted.to_string();
                    emit_marker("ratchet_skip_evict", None, &[("count", ev.as_str())]);
                }
                if qsp_session_store(channel.as_str(), &outcome.next_state).is_err() {
                    emit_marker("error", Some("qsp_session_store_failed"), &[]);
                    print_error_marker("qsp_session_store_failed");
                }
                count = count.saturating_add(1);
                let seq = state.session.recv_count.saturating_add(count as u64);
                let name = format!("recv_{}.bin", seq);
                let path = out_dir.join(&name);
                if write_atomic(&path, &outcome.plaintext, source).is_err() {
                    print_error_marker("recv_write_failed");
                }
                let size_s = outcome.plaintext.len().to_string();
                emit_marker(
                    "recv_item",
                    None,
                    &[
                        ("idx", count.to_string().as_str()),
                        ("size", size_s.as_str()),
                        ("id", item.id.as_str()),
                    ],
                );
                state.events.push_back(format!(
                    "recv: from={} size={} saved={}",
                    from, size_s, name
                ));
                let detail = format!("bytes={}", size_s);
                state.record_message_line(from, "RECEIVED", "in", detail.as_str());
            }
            Err(code) => {
                record_qsp_status(&cfg_dir, cfg_source, false, code, false, false);
                emit_marker("qsp_unpack", Some(code), &[("ok", "false")]);
                if code == "qsp_replay_reject" {
                    let msg_idx = qsp_session_for_channel(from)
                        .map(|st| st.recv.nr.to_string())
                        .unwrap_or_else(|_| "0".to_string());
                    emit_marker("ratchet_replay_reject", None, &[("msg_idx", &msg_idx)]);
                }
                print_error_marker(code);
            }
        }
    }
    state.session.recv_count = state.session.recv_count.saturating_add(count as u64);
    let count_s = count.to_string();
    emit_marker(
        "tui_receive",
        None,
        &[("from", from), ("count", count_s.as_str())],
    );
    emit_marker("recv_commit", None, &[("count", count_s.as_str())]);
    state.refresh_qsp_status();
}

fn tui_payload_bytes(seq: u64) -> Vec<u8> {
    // Deterministic, non-secret payload bytes.
    format!("tui_msg_seq={}", seq).into_bytes()
}
