use super::*;

pub(super) fn dispatch_tui_command(cmd: &TuiParsedCmd, state: &mut TuiState) -> bool {
    match cmd.cmd.as_str() {
        "help" | "exithelp" | "focus" | "inspector" | "ins" | "back" | "unfocus" | "down"
        | "up" | "pgdn" | "pagedown" | "pgup" | "pageup" | "exit" | "quit" => {
            super::navigation::dispatch_navigation_command(cmd, state)
        }
        "send" => handle_send_command(state),
        "receive" => handle_receive_command(cmd, state),
        "handshake" => handle_handshake_command(cmd, state),
        "status" => handle_status_command(state),
        "autolock" => handle_autolock_command(cmd, state),
        "poll" | "polling" => handle_poll_command(cmd, state),
        "relay" | "server" => handle_relay_command(cmd, state),
        "vault" => handle_vault_command(cmd, state),
        "device" => handle_device_command(cmd, state),
        "account" => handle_account_command(cmd, state),
        "lock" => handle_lock_command(state),
        "unlock" => handle_unlock_command(state),
        "contacts" | "trust" | "requests" | "verify" => {
            super::contacts::dispatch_contacts_command(cmd, state)
        }
        "messages" | "msg" | "files" | "injectmsg" | "injectevent" | "envelope" | "export" => {
            super::messages::dispatch_messages_command(cmd, state)
        }
        other => {
            state.set_command_error(format!("unknown command: {}", other));
            emit_marker("tui_cmd", None, &[("cmd", other)]);
            false
        }
    }
}

fn handle_send_command(state: &mut TuiState) -> bool {
    emit_marker("tui_cmd", None, &[("cmd", "send")]);
    if state.is_locked() {
        emit_marker(
            "tui_send_blocked",
            None,
            &[("reason", "vault_locked"), ("hint", "run_qsc_vault_unlock")],
        );
        state.update_send_lifecycle("blocked");
        return false;
    }
    let selected_peer = state
        .selected_messages_thread()
        .unwrap_or_else(|| state.session.peer_label.to_string());
    if let Err(reason) = state.trust_allows_peer_send_strict(selected_peer.as_str()) {
        emit_marker("tui_send_blocked", Some(reason), &[("reason", reason)]);
        state.update_send_lifecycle("blocked");
        return false;
    }
    tui_send_via_relay(state, selected_peer.as_str());
    false
}

fn handle_receive_command(cmd: &TuiParsedCmd, state: &mut TuiState) -> bool {
    emit_marker("tui_cmd", None, &[("cmd", "receive")]);
    if state.is_locked() {
        emit_marker(
            "tui_receive_blocked",
            None,
            &[("reason", "vault_locked"), ("hint", "run_qsc_vault_unlock")],
        );
        return false;
    }
    let selected_peer = state.selected_messages_thread();
    let peer = cmd
        .args
        .first()
        .map(|s| s.as_str())
        .or(selected_peer.as_deref())
        .unwrap_or(state.session.peer_label);
    tui_receive_via_relay(state, peer);
    false
}

fn handle_handshake_command(cmd: &TuiParsedCmd, state: &mut TuiState) -> bool {
    emit_marker("tui_cmd", None, &[("cmd", "handshake")]);
    if state.is_locked() {
        emit_marker(
            "tui_handshake_blocked",
            None,
            &[("reason", "vault_locked"), ("hint", "run_qsc_vault_unlock")],
        );
        return false;
    }
    let sub = cmd.args.first().map(|s| s.as_str()).unwrap_or("status");
    let selected_peer = state.selected_messages_thread();
    let peer = cmd
        .args
        .get(1)
        .map(|s| s.as_str())
        .or(selected_peer.as_deref())
        .unwrap_or(state.session.peer_label);
    let self_label = env::var("QSC_SELF_LABEL").unwrap_or_else(|_| "self".to_string());
    match sub {
        "status" => {
            handshake_status(Some(peer));
            state
                .events
                .push_back(format!("handshake status peer={}", peer));
        }
        "init" => {
            if let Some(r) = state.effective_relay_config() {
                if let Err(code) = resolve_peer_device_target(peer, false) {
                    emit_marker("tui_handshake_blocked", Some(code), &[("reason", code)]);
                    return false;
                }
                if let Err(code) = tui_enforce_peer_not_blocked(state, peer) {
                    emit_marker("tui_handshake_blocked", Some(code), &[("reason", code)]);
                    return false;
                }
                handshake_init(&self_label, peer, &r.relay);
                state
                    .events
                    .push_back(format!("handshake init peer={}", peer));
            } else {
                emit_marker(
                    "tui_handshake_blocked",
                    None,
                    &[("reason", "explicit_only_no_transport")],
                );
            }
        }
        "poll" => {
            if let Some(r) = state.effective_relay_config() {
                if let Err(code) = resolve_peer_device_target(peer, false) {
                    emit_marker("tui_handshake_blocked", Some(code), &[("reason", code)]);
                    return false;
                }
                if let Err(code) = relay_self_inbox_route_token() {
                    emit_marker("tui_handshake_blocked", Some(code), &[("reason", code)]);
                    return false;
                }
                if let Err(code) = tui_enforce_peer_not_blocked(state, peer) {
                    emit_marker("tui_handshake_blocked", Some(code), &[("reason", code)]);
                    return false;
                }
                handshake_poll(&self_label, peer, &r.relay, 4);
                state
                    .events
                    .push_back(format!("handshake poll peer={}", peer));
            } else {
                emit_marker(
                    "tui_handshake_blocked",
                    None,
                    &[("reason", "explicit_only_no_transport")],
                );
            }
        }
        _ => {
            emit_marker(
                "tui_handshake_invalid",
                None,
                &[("reason", "unknown_subcmd")],
            );
        }
    }
    false
}

fn handle_status_command(state: &mut TuiState) -> bool {
    emit_marker("tui_cmd", None, &[("cmd", "status")]);
    state.refresh_envelope(state.last_payload_len());
    state.refresh_qsp_status();
    state.push_cmd_result("status", true, "system overview refreshed");
    false
}

fn handle_autolock_command(cmd: &TuiParsedCmd, state: &mut TuiState) -> bool {
    emit_marker("tui_cmd", None, &[("cmd", "autolock")]);
    let sub = cmd.args.first().map(|s| s.as_str()).unwrap_or("show");
    match sub {
        "set" => {
            let Some(minutes_s) = cmd.args.get(1).map(|s| s.as_str()) else {
                state.set_command_error("autolock: missing minutes");
                emit_marker(
                    "tui_autolock_set",
                    Some("autolock_invalid_minutes"),
                    &[("ok", "false"), ("reason", "missing_minutes")],
                );
                return false;
            };
            let Ok(minutes) = minutes_s.parse::<u64>() else {
                state.set_command_error("autolock: invalid minutes");
                emit_marker(
                    "tui_autolock_set",
                    Some("autolock_invalid_minutes"),
                    &[("ok", "false"), ("reason", "invalid_minutes")],
                );
                return false;
            };
            if let Err(code) = state.set_autolock_minutes(minutes) {
                state.set_command_error(format!("autolock: {}", code));
                emit_marker("tui_autolock_set", Some(code), &[("ok", "false")]);
            } else {
                state.set_status_last_command_result(format!("autolock set {} min", minutes));
                state.push_cmd_result("autolock set", true, format!("timeout={} min", minutes));
            }
        }
        "show" => {
            let minutes_s = state.autolock_minutes().to_string();
            emit_marker(
                "tui_autolock_show",
                None,
                &[("ok", "true"), ("minutes", minutes_s.as_str())],
            );
            state.set_status_last_command_result(format!(
                "autolock {} min",
                state.autolock_minutes()
            ));
            state.push_cmd_result(
                "autolock show",
                true,
                format!("timeout={} min", state.autolock_minutes()),
            );
        }
        _ => {
            state.set_command_error("autolock: unknown subcommand");
            emit_marker(
                "tui_autolock_set",
                Some("autolock_invalid_subcmd"),
                &[("ok", "false"), ("reason", "unknown_subcmd")],
            );
        }
    }
    false
}

fn handle_poll_command(cmd: &TuiParsedCmd, state: &mut TuiState) -> bool {
    emit_marker("tui_cmd", None, &[("cmd", "poll")]);
    let sub = cmd.args.first().map(|s| s.as_str()).unwrap_or("show");
    match sub {
        "show" => {
            state.emit_poll_show_marker();
            state.set_status_last_command_result(format!(
                "poll {} {}s",
                state.poll_mode().as_str(),
                state.poll_interval_seconds()
            ));
            state.push_cmd_result(
                "poll show",
                true,
                format!(
                    "mode={} interval={}s",
                    state.poll_mode().as_str(),
                    state.poll_interval_seconds()
                ),
            );
        }
        "set" => {
            let Some(mode) = cmd.args.get(1).map(|s| s.as_str()) else {
                state.set_command_error("poll: missing mode");
                emit_marker(
                    "tui_poll_set",
                    Some("poll_invalid_subcmd"),
                    &[("ok", "false"), ("reason", "missing_mode")],
                );
                return false;
            };
            match mode {
                "adaptive" => {
                    if let Err(code) = state.set_poll_mode_adaptive() {
                        state.set_command_error(format!("poll: {}", code));
                        emit_marker("tui_poll_set", Some(code), &[("ok", "false")]);
                    } else {
                        state.set_status_last_command_result("poll set adaptive");
                        state.push_cmd_result("poll set adaptive", true, "mode=adaptive");
                    }
                }
                "fixed" => {
                    let Some(seconds_s) = cmd.args.get(2).map(|s| s.as_str()) else {
                        state.set_command_error("poll: missing seconds");
                        emit_marker(
                            "tui_poll_set",
                            Some("poll_invalid_seconds"),
                            &[("ok", "false"), ("reason", "missing_seconds")],
                        );
                        return false;
                    };
                    let Ok(seconds) = seconds_s.parse::<u64>() else {
                        state.set_command_error("poll: invalid seconds");
                        emit_marker(
                            "tui_poll_set",
                            Some("poll_invalid_seconds"),
                            &[("ok", "false"), ("reason", "invalid_seconds")],
                        );
                        return false;
                    };
                    let now_ms = state.current_now_ms();
                    if let Err(code) = state.set_poll_mode_fixed(seconds, now_ms) {
                        state.set_command_error(format!("poll: {}", code));
                        emit_marker("tui_poll_set", Some(code), &[("ok", "false")]);
                    } else {
                        state
                            .set_status_last_command_result(format!("poll set fixed {}s", seconds));
                        state.push_cmd_result(
                            "poll set fixed",
                            true,
                            format!("interval={}s", seconds),
                        );
                    }
                }
                _ => {
                    state.set_command_error("poll: unknown mode");
                    emit_marker(
                        "tui_poll_set",
                        Some("poll_invalid_subcmd"),
                        &[("ok", "false"), ("reason", "unknown_mode")],
                    );
                }
            }
        }
        _ => {
            state.set_command_error("poll: unknown subcommand");
            emit_marker(
                "tui_poll_set",
                Some("poll_invalid_subcmd"),
                &[("ok", "false"), ("reason", "unknown_subcmd")],
            );
        }
    }
    false
}

fn handle_relay_command(cmd: &TuiParsedCmd, state: &mut TuiState) -> bool {
    emit_marker("tui_cmd", None, &[("cmd", "relay")]);
    let sub = cmd.args.first().map(|s| s.as_str()).unwrap_or("show");
    match sub {
        "show" => {
            state.push_cmd_result("relay show", true, "relay page");
            state.set_status_last_command_result("relay show");
            state.route_show_to_system_nav(TuiInspectorPane::Relay);
        }
        "set" => {
            let Some(key) = cmd.args.get(1).map(|s| s.as_str()) else {
                state.set_command_error("relay: missing field");
                return false;
            };
            match key {
                "endpoint" => {
                    let Some(value) = cmd.args.get(2).map(|s| s.as_str()) else {
                        state.set_command_error("relay: missing endpoint");
                        return false;
                    };
                    match state.set_relay_endpoint(value) {
                        Ok(()) => {
                            let hash = state
                                .relay_endpoint_hash_cache
                                .as_deref()
                                .unwrap_or("none")
                                .to_string();
                            state.push_cmd_result(
                                "relay set endpoint",
                                true,
                                "relay endpoint set (redacted)",
                            );
                            state.set_status_last_command_result(format!(
                                "relay endpoint set (hash {})",
                                hash
                            ));
                            state.set_command_feedback("ok: relay endpoint set");
                        }
                        Err(code) => {
                            emit_marker("tui_relay_policy_reject", Some(code), &[]);
                            state.set_command_error(format!(
                                "relay: {}",
                                relay_user_reason_from_code(code)
                            ));
                        }
                    }
                }
                "token" => {
                    let Some(value) = cmd.args.get(2).map(|s| s.as_str()) else {
                        state.set_command_error("relay: missing token");
                        return false;
                    };
                    match state.set_relay_token(value) {
                        Ok(()) => {
                            state.push_cmd_result(
                                "relay set token",
                                true,
                                "relay token stored (redacted)",
                            );
                            state.set_status_last_command_result("relay token set");
                            state.set_command_feedback("ok: relay token set");
                        }
                        Err(code) => {
                            state.set_command_error(format!(
                                "relay: {}",
                                relay_user_reason_from_code(code)
                            ));
                        }
                    }
                }
                "token-file" => {
                    let Some(value) = cmd.args.get(2).map(|s| s.as_str()) else {
                        state.set_command_error("relay: missing token file path");
                        return false;
                    };
                    match state.set_relay_token_file(value) {
                        Ok(()) => {
                            state.push_cmd_result(
                                "relay set token-file",
                                true,
                                "relay token file stored (redacted)",
                            );
                            state.set_status_last_command_result("relay token file set");
                            state.set_command_feedback("ok: relay token file set");
                        }
                        Err(code) => {
                            state.set_command_error(format!(
                                "relay: {}",
                                relay_user_reason_from_code(code)
                            ));
                        }
                    }
                }
                _ => state.set_command_error("relay: unknown field"),
            }
        }
        "inbox" => {
            let Some(action) = cmd.args.get(1).map(|s| s.as_str()) else {
                state.set_command_error("relay: missing inbox subcommand");
                return false;
            };
            match action {
                "set" => {
                    let Some(value) = cmd.args.get(2).map(|s| s.as_str()) else {
                        state.set_command_error("relay: missing inbox token");
                        return false;
                    };
                    match state.set_relay_inbox_token(value) {
                        Ok(()) => {
                            state.push_cmd_result(
                                "relay inbox set",
                                true,
                                "relay inbox token stored (redacted)",
                            );
                            state.set_status_last_command_result("relay inbox token set");
                            state.set_command_feedback("ok: relay inbox token set");
                        }
                        Err(code) => {
                            state.set_command_error(format!(
                                "relay: {}",
                                relay_user_reason_from_code(code)
                            ));
                        }
                    }
                }
                "clear" => {
                    if let Err(code) = state.clear_relay_inbox_token() {
                        state.set_command_error(format!(
                            "relay: {}",
                            relay_user_reason_from_code(code)
                        ));
                        return false;
                    }
                    state.push_cmd_result("relay inbox clear", true, "relay inbox token cleared");
                    state.set_status_last_command_result("relay inbox token cleared");
                    state.set_command_feedback("ok: relay inbox token cleared");
                }
                _ => state.set_command_error("relay: unknown field"),
            }
        }
        "clear" => {
            let clear_token_only = matches!(cmd.args.get(1).map(|s| s.as_str()), Some("token"));
            let clear_token_file_only =
                matches!(cmd.args.get(1).map(|s| s.as_str()), Some("token-file"));
            let clear_inbox_only = matches!(cmd.args.get(1).map(|s| s.as_str()), Some("inbox"));
            if clear_token_only {
                if let Err(code) = state.persist_account_secret(TUI_RELAY_TOKEN_SECRET_KEY, "") {
                    state
                        .set_command_error(format!("relay: {}", relay_user_reason_from_code(code)));
                    return false;
                }
                state.relay_token_set_cache = false;
                state.push_cmd_result("relay clear token", true, "relay token cleared");
                state.set_status_last_command_result("relay token cleared");
                state.set_command_feedback("ok: relay token cleared");
            } else if clear_token_file_only {
                if let Err(code) = state.persist_account_secret(TUI_RELAY_TOKEN_FILE_SECRET_KEY, "")
                {
                    state
                        .set_command_error(format!("relay: {}", relay_user_reason_from_code(code)));
                    return false;
                }
                state.relay_token_file_cache = None;
                state.relay_token_file_hash_cache = None;
                state.push_cmd_result("relay clear token-file", true, "relay token file cleared");
                state.set_status_last_command_result("relay token file cleared");
                state.set_command_feedback("ok: relay token file cleared");
            } else if clear_inbox_only {
                if let Err(code) = state.clear_relay_inbox_token() {
                    state
                        .set_command_error(format!("relay: {}", relay_user_reason_from_code(code)));
                    return false;
                }
                state.push_cmd_result("relay clear inbox", true, "relay inbox cleared");
                state.set_status_last_command_result("relay inbox cleared");
                state.set_command_feedback("ok: relay inbox cleared");
            } else if let Err(code) = state.clear_relay_config() {
                state.set_command_error(format!("relay: {}", relay_user_reason_from_code(code)));
            } else {
                state.push_cmd_result("relay clear", true, "relay config cleared");
                state.set_status_last_command_result("relay config cleared");
                state.set_command_feedback("ok: relay config cleared");
            }
        }
        "test" => {
            if state.relay_endpoint_cache.is_none() {
                emit_tui_relay_test_event("err", "relay_endpoint_missing");
                state.push_cmd_result("relay test", false, "endpoint not configured");
                state.set_status_last_command_result("relay test err (endpoint missing)");
                state.set_command_error("relay: endpoint not configured");
                return false;
            }
            if state.relay_test_task.is_some() {
                emit_tui_relay_test_event("err", "relay_test_already_running");
                state.push_cmd_result("relay test", false, "test already running");
                state.set_status_last_command_result("relay test err (already running)");
                state.set_command_error("relay: test already running");
                return false;
            }
            let endpoint = state.relay_endpoint_cache.clone().unwrap_or_default();
            if let Err(code) = normalize_relay_endpoint(endpoint.as_str()) {
                emit_marker("tui_relay_policy_reject", Some(code), &[]);
                let reason = relay_user_reason_from_code(code);
                emit_tui_relay_test_event("err", code);
                state.push_cmd_result("relay test", false, reason);
                state.set_status_last_command_result(format!("relay test err ({})", reason));
                state.set_command_error(format!("relay: {}", reason));
                return false;
            }
            let token = if state.relay_token_set_cache {
                state.read_account_secret(TUI_RELAY_TOKEN_SECRET_KEY)
            } else {
                None
            };
            let token_file = state.relay_token_file_cache.clone();
            let (tx, rx) = mpsc::channel::<RelayTestOutcome>();
            std::thread::spawn(move || {
                let outcome = run_relay_test_probe(endpoint.as_str(), token, token_file.as_deref());
                let _ = tx.send(outcome);
            });
            state.relay_last_test_result = "pending".to_string();
            state.relay_test_task = Some(rx);
            emit_tui_relay_test_event("started", "pending");
            state.push_cmd_result("relay test", true, "started");
            state.set_status_last_command_result("relay test started");
            state.set_command_feedback("ok: relay test started");
        }
        _ => state.set_command_error("relay: unknown subcommand"),
    }
    false
}

fn handle_vault_command(cmd: &TuiParsedCmd, state: &mut TuiState) -> bool {
    emit_marker("tui_cmd", None, &[("cmd", "vault")]);
    let sub = cmd.args.first().map(|s| s.as_str()).unwrap_or("");
    match sub {
        "where" => match config_dir() {
            Ok((cfg, _)) => {
                let path = cfg.join("vault.qsv");
                let path_s = path.display().to_string();
                emit_marker(
                    "tui_vault_where",
                    None,
                    &[("ok", "true"), ("path", path_s.as_str())],
                );
                state.set_status_last_command_result(format!("vault path {}", path_s));
                state.push_cmd_result("vault where", true, format!("path={}", path_s));
            }
            Err(code) => {
                state.set_command_error(format!("vault: {}", code.as_str()));
                emit_marker("tui_vault_where", Some(code.as_str()), &[("ok", "false")]);
            }
        },
        "attempt_limit" => {
            let action = cmd.args.get(1).map(|s| s.as_str()).unwrap_or("show");
            match action {
                "set" => {
                    let Some(limit_s) = cmd.args.get(2).map(|s| s.as_str()) else {
                        state.set_command_error("vault: attempt_limit missing value");
                        return false;
                    };
                    let Ok(limit) = limit_s.parse::<u32>() else {
                        state.set_command_error("vault: attempt_limit invalid value");
                        return false;
                    };
                    match state.set_unlock_attempt_limit(Some(limit)) {
                        Ok(()) => {
                            state.set_status_last_command_result(format!(
                                "vault attempt limit set {}",
                                limit
                            ));
                            state.push_cmd_result(
                                "vault attempt_limit set",
                                true,
                                format!(
                                    "limit={} (warning: enabling this can permanently destroy your vault after {} failed unlocks)",
                                    limit, limit
                                ),
                            );
                            state.set_command_feedback(format!(
                                "ok: vault attempt limit set {}",
                                limit
                            ));
                        }
                        Err(code) => state.set_command_error(format!("vault: {}", code)),
                    }
                }
                "clear" => match state.set_unlock_attempt_limit(None) {
                    Ok(()) => {
                        state.set_status_last_command_result(
                            "vault attempt limit cleared".to_string(),
                        );
                        state.push_cmd_result(
                            "vault attempt_limit clear",
                            true,
                            "attempt limit disabled",
                        );
                        state.set_command_feedback("ok: vault attempt limit cleared");
                    }
                    Err(code) => state.set_command_error(format!("vault: {}", code)),
                },
                "show" => {
                    let limit = state
                        .unlock_attempt_limit
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "off".to_string());
                    let failures = state.failed_unlock_attempts.to_string();
                    emit_marker(
                        "tui_vault_attempt_limit_show",
                        None,
                        &[
                            ("ok", "true"),
                            ("limit", limit.as_str()),
                            ("failed_unlocks", failures.as_str()),
                        ],
                    );
                    state.set_status_last_command_result(format!(
                        "vault attempt limit {} failures {}",
                        limit, failures
                    ));
                    state.push_cmd_result(
                        "vault attempt_limit show",
                        true,
                        format!("limit={} failed_unlocks={}", limit, failures),
                    );
                }
                _ => state.set_command_error("vault: attempt_limit unknown subcommand"),
            }
        }
        _ => {
            state.set_command_error("vault: unknown subcommand");
            emit_marker(
                "tui_vault_where",
                Some("vault_invalid_subcmd"),
                &[("ok", "false"), ("reason", "unknown_subcmd")],
            );
        }
    }
    false
}

fn handle_device_command(cmd: &TuiParsedCmd, state: &mut TuiState) -> bool {
    emit_marker("tui_cmd", None, &[("cmd", "device")]);
    let sub = cmd.args.first().map(|s| s.as_str()).unwrap_or("");
    match sub {
        "show" => {
            let mode = "single-device";
            let device_id = "local-vault";
            emit_marker(
                "tui_device_show",
                None,
                &[("ok", "true"), ("mode", mode), ("id", device_id)],
            );
            state.set_status_last_command_result(format!("device {} {}", mode, device_id));
            state.push_cmd_result(
                "device show",
                true,
                format!("mode={} id={}", mode, device_id),
            );
        }
        _ => {
            state.set_command_error("device: unknown subcommand");
            emit_marker(
                "tui_device_show",
                Some("device_invalid_subcmd"),
                &[("ok", "false"), ("reason", "unknown_subcmd")],
            );
        }
    }
    false
}

fn handle_account_command(cmd: &TuiParsedCmd, state: &mut TuiState) -> bool {
    emit_marker("tui_cmd", None, &[("cmd", "account")]);
    let sub = cmd.args.first().map(|s| s.as_str()).unwrap_or("show");
    match sub {
        "show" => {
            state.set_inspector(TuiInspectorPane::Account);
            state.push_cmd_result("account show", true, "account page");
            state.set_status_last_command_result("account page");
        }
        "destroy" => {
            if state.is_locked() {
                state.set_command_error("locked: unlock required");
                return false;
            }
            if !state.has_vault() {
                state.set_command_error("account: no vault found");
                return false;
            }
            state.start_account_destroy_prompt();
            state.push_cmd_result("account destroy", true, "confirmation required");
            state.set_status_last_command_result("account destroy started");
        }
        _ => {
            state.set_command_error("account: unknown subcommand");
            emit_marker("tui_account_invalid", None, &[("reason", "unknown_subcmd")]);
        }
    }
    false
}

fn handle_lock_command(state: &mut TuiState) -> bool {
    emit_marker("tui_cmd", None, &[("cmd", "lock")]);
    state.set_locked_state(true, "explicit_command");
    false
}

fn handle_unlock_command(state: &mut TuiState) -> bool {
    emit_marker("tui_cmd", None, &[("cmd", "unlock")]);
    if !state.is_locked() {
        emit_marker(
            "tui_unlock",
            None,
            &[("ok", "true"), ("reason", "already_unlocked")],
        );
        return false;
    }
    let unlocked = if vault::has_process_passphrase() {
        state.open_vault_session(None).is_ok()
    } else {
        false
    };
    if unlocked {
        state.set_locked_state(false, "explicit_command");
        emit_marker("tui_unlock", None, &[("ok", "true")]);
    } else {
        emit_marker(
            "tui_unlock",
            Some("vault_locked"),
            &[("ok", "false"), ("reason", "passphrase_required")],
        );
    }
    false
}
