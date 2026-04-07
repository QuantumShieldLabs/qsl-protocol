use super::state::tui_file_display_state;
use super::*;

mod catalog;
mod locked;
mod relay;

use self::catalog::{
    focus_mode_for_fkey, focus_mode_for_target, inspector_for_fkey, inspector_pane_for_target,
};
pub(super) use self::catalog::{tui_help_items, TuiHelpItem};
pub(super) use self::locked::wipe_account_local_state_best_effort;
use self::locked::{
    handle_locked_reject, handle_tui_account_destroy_key, handle_tui_locked_command,
    handle_tui_locked_key, tui_alias_is_valid, tui_verification_code_is_valid,
};
pub(super) use self::relay::tui_receive_via_relay;
use self::relay::{
    tui_msg_autotrust_first_use, tui_msg_ensure_handshake, tui_msg_recv_poll_bounded,
    tui_send_via_relay,
};

pub(super) fn handle_tui_key(state: &mut TuiState, key: KeyEvent) -> bool {
    if state.is_locked() {
        return handle_tui_locked_key(state, key);
    }
    if state.account_destroy_active() {
        return handle_tui_account_destroy_key(state, key);
    }
    state.clear_command_feedback();
    if state.is_help_mode() {
        match key.code {
            KeyCode::Esc => state.exit_help_mode(),
            KeyCode::F(1) => state.toggle_help_mode(),
            KeyCode::Char('?') => state.toggle_help_mode(),
            KeyCode::Up => state.help_move(-1),
            KeyCode::Down => state.help_move(1),
            _ => {}
        }
        return false;
    }
    if state.is_focus_mode() {
        match key.code {
            KeyCode::Esc => state.exit_focus_mode(),
            KeyCode::Up => {
                if state.mode == TuiMode::FocusContacts {
                    state.contacts_move(-1);
                } else if state.mode == TuiMode::FocusFiles {
                    state.files_move(-1);
                } else {
                    let max_len = state.focus_max_len();
                    state.focus_scroll_move(-1, max_len);
                }
            }
            KeyCode::Down => {
                if state.mode == TuiMode::FocusContacts {
                    state.contacts_move(1);
                } else if state.mode == TuiMode::FocusFiles {
                    state.files_move(1);
                } else {
                    let max_len = state.focus_max_len();
                    state.focus_scroll_move(1, max_len);
                }
            }
            KeyCode::PageUp => {
                if state.mode == TuiMode::FocusContacts {
                    state.contacts_move(-(state.focus_view_rows() as i32));
                } else if state.mode == TuiMode::FocusFiles {
                    state.files_move(-(state.focus_view_rows() as i32));
                } else {
                    let max_len = state.focus_max_len();
                    state.focus_scroll_move(-(state.focus_view_rows() as i32), max_len);
                }
            }
            KeyCode::PageDown => {
                if state.mode == TuiMode::FocusContacts {
                    state.contacts_move(state.focus_view_rows() as i32);
                } else if state.mode == TuiMode::FocusFiles {
                    state.files_move(state.focus_view_rows() as i32);
                } else {
                    let max_len = state.focus_max_len();
                    state.focus_scroll_move(state.focus_view_rows() as i32, max_len);
                }
            }
            _ => {
                if let Some(mode) = focus_mode_for_fkey(key.code) {
                    state.enter_focus_mode(mode);
                }
            }
        }
        return false;
    }
    match key.code {
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => return true,
        KeyCode::Char('l') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            emit_marker("tui_cmd", None, &[("cmd", "lock_shortcut")]);
            state.set_locked_state(true, "ctrl_l_shortcut");
        }
        KeyCode::Esc => {
            state.home_focus = TuiHomeFocus::Nav;
            state.cmd_input_clear();
            emit_marker("tui_focus_home", None, &[("pane", state.home_focus_name())]);
        }
        KeyCode::Tab => {
            state.home_focus_cycle(1);
        }
        KeyCode::BackTab => {
            state.home_focus_cycle(-1);
        }
        KeyCode::F(1) | KeyCode::Char('?') => {
            if state.is_locked() {
                handle_locked_reject(state, "help", "locked_unlock_required");
            } else {
                state.toggle_help_mode();
            }
        }
        KeyCode::Enter => {
            if state.home_focus == TuiHomeFocus::Nav {
                state.nav_activate();
            } else if state.home_focus != TuiHomeFocus::Command {
                state.enter_focus_mode(state.focus_mode_for_inspector());
            } else if let Some(cmd) = parse_tui_command(state.cmd_input.as_str()) {
                let exit = handle_tui_command(&cmd, state);
                state.cmd_input_clear();
                return exit;
            } else if !state.cmd_input.is_empty() {
                emit_marker("tui_input_text", None, &[("kind", "plain")]);
            }
            state.cmd_input_clear();
        }
        KeyCode::Backspace => {
            if state.home_focus == TuiHomeFocus::Command {
                state.cmd_input_pop();
            }
        }
        KeyCode::Up => {
            if state.home_focus == TuiHomeFocus::Main {
                state.main_scroll_move(-1);
            } else {
                state.nav_move(-1);
            }
        }
        KeyCode::Down => {
            if state.home_focus == TuiHomeFocus::Main {
                state.main_scroll_move(1);
            } else {
                state.nav_move(1);
            }
        }
        KeyCode::PageUp => {
            if state.home_focus == TuiHomeFocus::Main {
                state.main_scroll_page(-1);
            }
        }
        KeyCode::PageDown => {
            if state.home_focus == TuiHomeFocus::Main {
                state.main_scroll_page(1);
            }
        }
        KeyCode::Home => {
            if state.home_focus == TuiHomeFocus::Main {
                state.main_scroll_home();
            }
        }
        KeyCode::End => {
            if state.home_focus == TuiHomeFocus::Main {
                state.main_scroll_end();
            }
        }
        KeyCode::Char(ch) => {
            if state.home_focus == TuiHomeFocus::Command {
                state.cmd_input_push(ch);
            } else if ch == '/' {
                state.home_focus = TuiHomeFocus::Command;
                emit_marker("tui_focus_home", None, &[("pane", state.home_focus_name())]);
                state.cmd_input_push(ch);
            }
        }
        _ => {
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                if let Some(mode) = focus_mode_for_fkey(key.code) {
                    state.enter_focus_mode(mode);
                }
            } else if let Some(pane) = inspector_for_fkey(key.code) {
                state.set_inspector(pane);
            }
        }
    }
    false
}

pub(super) fn format_message_transcript_line(
    peer: &str,
    state: &str,
    direction: &str,
    detail: &str,
) -> String {
    let prefix = if direction.eq_ignore_ascii_case("out") {
        "You".to_string()
    } else {
        peer.to_string()
    };
    let message = detail.trim();
    let semantic = message_delivery_semantic_from_state_str(direction, state).unwrap_or(state);
    if message.is_empty() {
        format!("{}:", prefix)
    } else if message.eq_ignore_ascii_case("source=test_harness") {
        format!("{}: (test message) [{}]", prefix, semantic)
    } else {
        format!("{}: {} [{}]", prefix, message, semantic)
    }
}

pub(super) fn handle_tui_command(cmd: &TuiParsedCmd, state: &mut TuiState) -> bool {
    let mut command_label = if cmd.args.is_empty() {
        cmd.cmd.clone()
    } else {
        format!("{} {}", cmd.cmd, cmd.args.join(" "))
    };
    if cmd.cmd == "relay" || cmd.cmd == "server" {
        if matches!(
            (
                cmd.args.first().map(|s| s.as_str()),
                cmd.args.get(1).map(|s| s.as_str())
            ),
            (Some("set"), Some("endpoint"))
        ) {
            command_label = "relay set endpoint <redacted>".to_string();
        } else if matches!(
            (
                cmd.args.first().map(|s| s.as_str()),
                cmd.args.get(1).map(|s| s.as_str())
            ),
            (Some("set"), Some("token"))
        ) {
            command_label = "relay set token <redacted>".to_string();
        } else if matches!(
            (
                cmd.args.first().map(|s| s.as_str()),
                cmd.args.get(1).map(|s| s.as_str())
            ),
            (Some("set"), Some("token-file"))
        ) {
            command_label = "relay set token-file <redacted>".to_string();
        } else if matches!(
            (
                cmd.args.first().map(|s| s.as_str()),
                cmd.args.get(1).map(|s| s.as_str())
            ),
            (Some("inbox"), Some("set"))
        ) {
            command_label = "relay inbox set <redacted>".to_string();
        }
    } else if cmd.cmd == "contacts"
        && matches!(
            (
                cmd.args.first().map(|s| s.as_str()),
                cmd.args.get(1).map(|s| s.as_str())
            ),
            (Some("route"), Some("set"))
        )
    {
        command_label = "contacts route set <redacted>".to_string();
    }
    let before_results_len = state.cmd_results.len();
    state.begin_command_tracking(command_label.clone());
    state.mark_input_activity(state.current_now_ms());
    state.clear_command_error();
    state.clear_command_feedback();
    if cmd.cmd == "key" {
        emit_marker("tui_cmd", None, &[("cmd", "key")]);
        let spec = cmd.args.first().map(|s| s.as_str()).unwrap_or("");
        if let Some(key) = parse_tui_script_key(spec) {
            let exit = handle_tui_key(state, key);
            if state.cmd_results.len() == before_results_len {
                state.push_cmd_result(command_label.as_str(), true, "ok");
            }
            if !exit && state.command_error.is_none() {
                state.set_command_feedback("ok: key event");
            }
            state.end_command_tracking();
            return exit;
        }
        state.set_command_error("key: unknown key");
        emit_marker("tui_key_invalid", None, &[("reason", "unknown_key")]);
        state.end_command_tracking();
        return false;
    }
    if state.is_locked() {
        if let Some(exit) = handle_tui_locked_command(cmd, state) {
            if state.cmd_results.len() == before_results_len {
                if let Some(err) = state.command_error.clone() {
                    state.push_cmd_result(command_label.as_str(), false, err);
                } else {
                    state.push_cmd_result(command_label.as_str(), true, "ok");
                }
            }
            state.end_command_tracking();
            return exit;
        }
    }
    let exit = match cmd.cmd.as_str() {
        "help" => {
            emit_marker("tui_cmd", None, &[("cmd", "help")]);
            state.enter_help_mode();
            false
        }
        "exithelp" => {
            emit_marker("tui_cmd", None, &[("cmd", "exithelp")]);
            state.exit_help_mode();
            false
        }
        "focus" => {
            emit_marker("tui_cmd", None, &[("cmd", "focus")]);
            let target = cmd.args.first().map(|s| s.as_str()).unwrap_or("");
            if let Some(mode) = focus_mode_for_target(target) {
                state.enter_focus_mode(mode);
            } else {
                state.set_command_error("focus: unknown pane");
                emit_marker("tui_focus_invalid", None, &[("reason", "unknown_pane")]);
            }
            false
        }
        "inspector" | "ins" => {
            emit_marker("tui_cmd", None, &[("cmd", "inspector")]);
            let target = cmd.args.first().map(|s| s.as_str()).unwrap_or("");
            if let Some(pane) = inspector_pane_for_target(target) {
                state.set_inspector(pane);
            } else {
                state.set_command_error("inspector: unknown pane");
                emit_marker("tui_inspector_invalid", None, &[("reason", "unknown_pane")]);
            }
            false
        }
        "back" | "unfocus" => {
            emit_marker("tui_cmd", None, &[("cmd", "back")]);
            state.exit_focus_mode();
            false
        }
        "down" => {
            emit_marker("tui_cmd", None, &[("cmd", "down")]);
            if state.is_focus_mode() {
                if state.mode == TuiMode::FocusContacts {
                    state.contacts_move(1);
                } else if state.mode == TuiMode::FocusFiles {
                    state.files_move(1);
                } else {
                    let max_len = state.focus_max_len();
                    state.focus_scroll_move(1, max_len);
                }
            }
            false
        }
        "up" => {
            emit_marker("tui_cmd", None, &[("cmd", "up")]);
            if state.is_focus_mode() {
                if state.mode == TuiMode::FocusContacts {
                    state.contacts_move(-1);
                } else if state.mode == TuiMode::FocusFiles {
                    state.files_move(-1);
                } else {
                    let max_len = state.focus_max_len();
                    state.focus_scroll_move(-1, max_len);
                }
            }
            false
        }
        "pgdn" | "pagedown" => {
            emit_marker("tui_cmd", None, &[("cmd", "pgdn")]);
            if state.is_focus_mode() {
                if state.mode == TuiMode::FocusContacts {
                    state.contacts_move(state.focus_view_rows() as i32);
                } else if state.mode == TuiMode::FocusFiles {
                    state.files_move(state.focus_view_rows() as i32);
                } else {
                    let max_len = state.focus_max_len();
                    state.focus_scroll_move(state.focus_view_rows() as i32, max_len);
                }
            }
            false
        }
        "pgup" | "pageup" => {
            emit_marker("tui_cmd", None, &[("cmd", "pgup")]);
            if state.is_focus_mode() {
                if state.mode == TuiMode::FocusContacts {
                    state.contacts_move(-(state.focus_view_rows() as i32));
                } else if state.mode == TuiMode::FocusFiles {
                    state.files_move(-(state.focus_view_rows() as i32));
                } else {
                    let max_len = state.focus_max_len();
                    state.focus_scroll_move(-(state.focus_view_rows() as i32), max_len);
                }
            }
            false
        }
        "exit" | "quit" => {
            emit_marker("tui_cmd", None, &[("cmd", "exit")]);
            true
        }
        "send" => {
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
        "receive" => {
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
        "handshake" => {
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
        "status" => {
            emit_marker("tui_cmd", None, &[("cmd", "status")]);
            state.refresh_envelope(state.last_payload_len());
            state.refresh_qsp_status();
            state.push_cmd_result("status", true, "system overview refreshed");
            false
        }
        "autolock" => {
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
                        state.set_status_last_command_result(format!(
                            "autolock set {} min",
                            minutes
                        ));
                        state.push_cmd_result(
                            "autolock set",
                            true,
                            format!("timeout={} min", minutes),
                        );
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
        "poll" | "polling" => {
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
                                state.set_status_last_command_result(format!(
                                    "poll set fixed {}s",
                                    seconds
                                ));
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
        "relay" | "server" => {
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
                                Err(code) => state.set_command_error(format!(
                                    "relay: {}",
                                    relay_user_reason_from_code(code)
                                )),
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
                                Err(code) => state.set_command_error(format!(
                                    "relay: {}",
                                    relay_user_reason_from_code(code)
                                )),
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
                                Err(code) => state.set_command_error(format!(
                                    "relay: {}",
                                    relay_user_reason_from_code(code)
                                )),
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
                            state.push_cmd_result(
                                "relay inbox clear",
                                true,
                                "relay inbox token cleared",
                            );
                            state.set_status_last_command_result("relay inbox token cleared");
                            state.set_command_feedback("ok: relay inbox token cleared");
                        }
                        _ => state.set_command_error("relay: unknown field"),
                    }
                }
                "clear" => {
                    let clear_token_only =
                        matches!(cmd.args.get(1).map(|s| s.as_str()), Some("token"));
                    let clear_token_file_only =
                        matches!(cmd.args.get(1).map(|s| s.as_str()), Some("token-file"));
                    let clear_inbox_only =
                        matches!(cmd.args.get(1).map(|s| s.as_str()), Some("inbox"));
                    if clear_token_only {
                        if let Err(code) =
                            state.persist_account_secret(TUI_RELAY_TOKEN_SECRET_KEY, "")
                        {
                            state.set_command_error(format!(
                                "relay: {}",
                                relay_user_reason_from_code(code)
                            ));
                            return false;
                        }
                        state.relay_token_set_cache = false;
                        state.push_cmd_result("relay clear token", true, "relay token cleared");
                        state.set_status_last_command_result("relay token cleared");
                        state.set_command_feedback("ok: relay token cleared");
                    } else if clear_token_file_only {
                        if let Err(code) =
                            state.persist_account_secret(TUI_RELAY_TOKEN_FILE_SECRET_KEY, "")
                        {
                            state.set_command_error(format!(
                                "relay: {}",
                                relay_user_reason_from_code(code)
                            ));
                            return false;
                        }
                        state.relay_token_file_cache = None;
                        state.relay_token_file_hash_cache = None;
                        state.push_cmd_result(
                            "relay clear token-file",
                            true,
                            "relay token file cleared",
                        );
                        state.set_status_last_command_result("relay token file cleared");
                        state.set_command_feedback("ok: relay token file cleared");
                    } else if clear_inbox_only {
                        if let Err(code) = state.clear_relay_inbox_token() {
                            state.set_command_error(format!(
                                "relay: {}",
                                relay_user_reason_from_code(code)
                            ));
                            return false;
                        }
                        state.push_cmd_result("relay clear inbox", true, "relay inbox cleared");
                        state.set_status_last_command_result("relay inbox cleared");
                        state.set_command_feedback("ok: relay inbox cleared");
                    } else if let Err(code) = state.clear_relay_config() {
                        state.set_command_error(format!(
                            "relay: {}",
                            relay_user_reason_from_code(code)
                        ));
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
                        state
                            .set_status_last_command_result(format!("relay test err ({})", reason));
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
                        let outcome =
                            run_relay_test_probe(endpoint.as_str(), token, token_file.as_deref());
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
        "vault" => {
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
                                Err(code) => {
                                    state.set_command_error(format!("vault: {}", code));
                                }
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
        "device" => {
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
        "account" => {
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
        "lock" => {
            emit_marker("tui_cmd", None, &[("cmd", "lock")]);
            state.set_locked_state(true, "explicit_command");
            false
        }
        "unlock" => {
            emit_marker("tui_cmd", None, &[("cmd", "unlock")]);
            if !state.is_locked() {
                emit_marker(
                    "tui_unlock",
                    None,
                    &[("ok", "true"), ("reason", "already_unlocked")],
                );
                return false;
            }
            let unlocked = if vault::unlock_if_mock_provider() || vault::has_process_passphrase() {
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
        "contacts" => {
            emit_marker("tui_cmd", None, &[("cmd", "contacts")]);
            let sub = cmd.args.first().map(|s| s.as_str()).unwrap_or("list");
            match sub {
                "list" => {
                    state.refresh_contacts();
                    let count_s = state.contacts.len().to_string();
                    emit_marker("tui_contacts_list", None, &[("count", count_s.as_str())]);
                }
                "device" => {
                    let Some(action) = cmd.args.get(1).map(|s| s.as_str()) else {
                        state.set_command_error("contacts device: missing action");
                        emit_marker(
                            "tui_contacts_invalid",
                            None,
                            &[("reason", "missing_device_action")],
                        );
                        return false;
                    };
                    match action {
                        "list" => {
                            let Some(label) = cmd.args.get(2).map(|s| s.as_str()) else {
                                state.set_command_error("contacts device list: missing alias");
                                emit_marker(
                                    "tui_contacts_invalid",
                                    None,
                                    &[("reason", "missing_label")],
                                );
                                return false;
                            };
                            let Some(rec) = state.contacts_records.get(label) else {
                                state.set_command_error("contacts device list: unknown alias");
                                emit_marker(
                                    "tui_contacts_device_list",
                                    Some("peer_unknown"),
                                    &[("label", label), ("ok", "false")],
                                );
                                return false;
                            };
                            let mut rec = rec.clone();
                            normalize_contact_record(label, &mut rec);
                            let count_s = rec.devices.len().to_string();
                            emit_marker(
                                "tui_contacts_device_list",
                                None,
                                &[("label", label), ("count", count_s.as_str())],
                            );
                            for dev in rec.devices.iter() {
                                emit_marker(
                                    "tui_contacts_device",
                                    None,
                                    &[
                                        ("label", label),
                                        ("device", dev.device_id.as_str()),
                                        ("state", canonical_device_state(dev.state.as_str())),
                                    ],
                                );
                            }
                        }
                        "status" => {
                            let Some(label) = cmd.args.get(2).map(|s| s.as_str()) else {
                                state.set_command_error("contacts device status: missing alias");
                                emit_marker(
                                    "tui_contacts_invalid",
                                    None,
                                    &[("reason", "missing_label")],
                                );
                                return false;
                            };
                            let Some(rec) = state.contacts_records.get(label) else {
                                state.set_command_error("contacts device status: unknown alias");
                                emit_marker(
                                    "tui_contacts_device_status",
                                    Some("peer_unknown"),
                                    &[("label", label), ("ok", "false")],
                                );
                                return false;
                            };
                            let mut rec = rec.clone();
                            normalize_contact_record(label, &mut rec);
                            if let Some(device_id) = cmd.args.get(3).map(|s| s.as_str()) {
                                let Some(idx) = contact_device_find_index(&rec, device_id) else {
                                    state.set_command_error(
                                        "contacts device status: unknown device id",
                                    );
                                    emit_marker(
                                        "tui_contacts_device_status",
                                        Some("device_unknown"),
                                        &[("label", label), ("device", device_id), ("ok", "false")],
                                    );
                                    return false;
                                };
                                let dev = &rec.devices[idx];
                                emit_marker(
                                    "tui_contacts_device_status",
                                    None,
                                    &[
                                        ("label", label),
                                        ("device", device_id),
                                        ("state", canonical_device_state(dev.state.as_str())),
                                    ],
                                );
                            } else {
                                let count_s = rec.devices.len().to_string();
                                emit_marker(
                                    "tui_contacts_device_status",
                                    None,
                                    &[("label", label), ("count", count_s.as_str())],
                                );
                            }
                        }
                        "add" => {
                            let Some(label) = cmd.args.get(2).map(|s| s.as_str()) else {
                                state.set_command_error("contacts device add: missing alias");
                                emit_marker(
                                    "tui_contacts_invalid",
                                    None,
                                    &[("reason", "missing_label")],
                                );
                                return false;
                            };
                            let Some(code) = cmd.args.get(3).map(|s| s.as_str()) else {
                                state.set_command_error(
                                    "contacts device add: missing verification code",
                                );
                                emit_marker(
                                    "tui_contacts_invalid",
                                    None,
                                    &[("reason", "missing_fp")],
                                );
                                return false;
                            };
                            if !tui_verification_code_is_valid(code) {
                                state.set_command_error(
                                    "contacts device add: invalid verification code format",
                                );
                                emit_marker(
                                    "tui_contacts_invalid",
                                    None,
                                    &[("reason", "invalid_code")],
                                );
                                return false;
                            }
                            let route_token = match cmd.args.get(4).map(|s| s.as_str()) {
                                Some(raw) => match normalize_route_token(raw) {
                                    Ok(token) => Some(token),
                                    Err(code) => {
                                        state.set_command_error(format!(
                                            "contacts device add: {}",
                                            code
                                        ));
                                        emit_marker("tui_contacts_invalid", Some(code), &[]);
                                        return false;
                                    }
                                },
                                None => None,
                            };
                            let Some(rec) = state.contacts_records.get_mut(label) else {
                                state.set_command_error("contacts device add: unknown alias");
                                emit_marker(
                                    "tui_contacts_device_add",
                                    Some("peer_unknown"),
                                    &[("label", label), ("ok", "false")],
                                );
                                return false;
                            };
                            normalize_contact_record(label, rec);
                            let device_id = device_id_short(label, None, code);
                            if contact_device_find_index(rec, device_id.as_str()).is_some() {
                                state.set_command_error(
                                    "contacts device add: device already exists",
                                );
                                emit_marker(
                                    "tui_contacts_device_add",
                                    Some("device_exists"),
                                    &[
                                        ("label", label),
                                        ("device", device_id.as_str()),
                                        ("ok", "false"),
                                    ],
                                );
                                return false;
                            }
                            rec.devices.push(ContactDeviceRecord {
                                device_id: device_id.clone(),
                                fp: code.to_ascii_uppercase(),
                                sig_fp: None,
                                state: "UNVERIFIED".to_string(),
                                route_token,
                                seen_at: None,
                                label: None,
                            });
                            normalize_contact_record(label, rec);
                            if state.persist_contacts_cache().is_err() {
                                state.set_command_error("contacts device add: store unavailable");
                                emit_marker(
                                    "tui_contacts_device_add",
                                    Some("contacts_store_unavailable"),
                                    &[("label", label), ("ok", "false")],
                                );
                                return false;
                            }
                            emit_marker(
                                "tui_contacts_device_add",
                                None,
                                &[
                                    ("label", label),
                                    ("device", device_id.as_str()),
                                    ("state", "UNVERIFIED"),
                                    ("ok", "true"),
                                ],
                            );
                            state.refresh_contacts();
                        }
                        "verify" => {
                            let Some(label) = cmd.args.get(2).map(|s| s.as_str()) else {
                                state.set_command_error("contacts device verify: missing alias");
                                emit_marker(
                                    "tui_contacts_invalid",
                                    None,
                                    &[("reason", "missing_label")],
                                );
                                return false;
                            };
                            let Some(device_id) = cmd.args.get(3).map(|s| s.as_str()) else {
                                state
                                    .set_command_error("contacts device verify: missing device id");
                                emit_marker(
                                    "tui_contacts_invalid",
                                    None,
                                    &[("reason", "missing_device")],
                                );
                                return false;
                            };
                            let Some(code) = cmd.args.get(4).map(|s| s.as_str()) else {
                                state.set_command_error(
                                    "contacts device verify: missing verification code",
                                );
                                emit_marker(
                                    "tui_contacts_invalid",
                                    None,
                                    &[("reason", "missing_fp")],
                                );
                                return false;
                            };
                            if !tui_verification_code_is_valid(code) {
                                state.set_command_error(
                                    "contacts device verify: invalid verification code format",
                                );
                                emit_marker(
                                    "tui_contacts_invalid",
                                    None,
                                    &[("reason", "invalid_code")],
                                );
                                return false;
                            }
                            let Some(rec) = state.contacts_records.get_mut(label) else {
                                state.set_command_error("contacts device verify: unknown alias");
                                emit_marker(
                                    "tui_contacts_device_verify",
                                    Some("peer_unknown"),
                                    &[("label", label), ("ok", "false")],
                                );
                                return false;
                            };
                            normalize_contact_record(label, rec);
                            let Some(idx) = contact_device_find_index(rec, device_id) else {
                                state
                                    .set_command_error("contacts device verify: unknown device id");
                                emit_marker(
                                    "tui_contacts_device_verify",
                                    Some("device_unknown"),
                                    &[("label", label), ("device", device_id), ("ok", "false")],
                                );
                                return false;
                            };
                            if rec.devices[idx].fp.eq_ignore_ascii_case(code) {
                                let mode = state.trust_onboarding_mode;
                                rec.devices[idx].state = "VERIFIED".to_string();
                                rec.status = "VERIFIED".to_string();
                                if mode == TrustOnboardingMode::Balanced {
                                    rec.devices[idx].state = "TRUSTED".to_string();
                                    rec.status = "PINNED".to_string();
                                }
                                if state.persist_contacts_cache().is_err() {
                                    state.set_command_error(
                                        "contacts device verify: store unavailable",
                                    );
                                    emit_marker(
                                        "tui_contacts_device_verify",
                                        Some("contacts_store_unavailable"),
                                        &[("label", label), ("ok", "false")],
                                    );
                                    return false;
                                }
                                emit_marker(
                                    "tui_contacts_device_verify",
                                    None,
                                    &[
                                        ("label", label),
                                        ("device", device_id),
                                        (
                                            "state",
                                            if mode == TrustOnboardingMode::Balanced {
                                                "TRUSTED"
                                            } else {
                                                "VERIFIED"
                                            },
                                        ),
                                        ("ok", "true"),
                                    ],
                                );
                                emit_tui_contact_flow(
                                    "verify",
                                    if mode == TrustOnboardingMode::Balanced {
                                        "TRUSTED"
                                    } else {
                                        "VERIFIED"
                                    },
                                    label,
                                    Some(device_id),
                                    mode,
                                );
                                if mode == TrustOnboardingMode::Balanced {
                                    emit_tui_trust_promotion(
                                        "trusted",
                                        "verified_match",
                                        label,
                                        Some(device_id),
                                        mode,
                                    );
                                    state.set_command_feedback(
                                        "ok: verification matched and device auto-trusted (balanced mode)",
                                    );
                                } else {
                                    emit_tui_trust_promotion(
                                        "verified_only",
                                        "strict_mode",
                                        label,
                                        Some(device_id),
                                        mode,
                                    );
                                    state.set_command_feedback(
                                        "ok: verification code matched identity (strict mode requires trust)",
                                    );
                                }
                            } else {
                                rec.devices[idx].state = "CHANGED".to_string();
                                rec.status = "CHANGED".to_string();
                                let _ = state.persist_contacts_cache();
                                state.set_command_error(
                                    "contacts device verify: verification code mismatch",
                                );
                                emit_marker(
                                    "tui_contacts_device_verify",
                                    Some("verification_mismatch"),
                                    &[
                                        ("label", label),
                                        ("device", device_id),
                                        ("state", "CHANGED"),
                                        ("ok", "false"),
                                    ],
                                );
                                emit_tui_contact_flow(
                                    "verify",
                                    "CHANGED",
                                    label,
                                    Some(device_id),
                                    state.trust_onboarding_mode,
                                );
                                return false;
                            }
                            state.refresh_contacts();
                        }
                        "trust" => {
                            let Some(label) = cmd.args.get(2).map(|s| s.as_str()) else {
                                state.set_command_error("contacts device trust: missing alias");
                                emit_marker(
                                    "tui_contacts_invalid",
                                    None,
                                    &[("reason", "missing_label")],
                                );
                                return false;
                            };
                            let Some(device_id) = cmd.args.get(3).map(|s| s.as_str()) else {
                                state.set_command_error("contacts device trust: missing device id");
                                emit_marker(
                                    "tui_contacts_invalid",
                                    None,
                                    &[("reason", "missing_device")],
                                );
                                return false;
                            };
                            let confirmed = cmd
                                .args
                                .get(4)
                                .map(|s| s.eq_ignore_ascii_case("confirm"))
                                .unwrap_or(false);
                            if !confirmed {
                                state.set_command_error(
                                    "contacts device trust: confirmation required",
                                );
                                emit_marker(
                                    "tui_contacts_device_trust",
                                    Some("confirm_required"),
                                    &[("label", label), ("device", device_id), ("ok", "false")],
                                );
                                return false;
                            }
                            let Some(rec) = state.contacts_records.get_mut(label) else {
                                state.set_command_error("contacts device trust: unknown alias");
                                emit_marker(
                                    "tui_contacts_device_trust",
                                    Some("peer_unknown"),
                                    &[("label", label), ("ok", "false")],
                                );
                                return false;
                            };
                            normalize_contact_record(label, rec);
                            let Some(idx) = contact_device_find_index(rec, device_id) else {
                                state.set_command_error("contacts device trust: unknown device id");
                                emit_marker(
                                    "tui_contacts_device_trust",
                                    Some("device_unknown"),
                                    &[("label", label), ("device", device_id), ("ok", "false")],
                                );
                                return false;
                            };
                            rec.devices[idx].state = "TRUSTED".to_string();
                            rec.status = "PINNED".to_string();
                            if state.persist_contacts_cache().is_err() {
                                state.set_command_error("contacts device trust: store unavailable");
                                emit_marker(
                                    "tui_contacts_device_trust",
                                    Some("contacts_store_unavailable"),
                                    &[("label", label), ("ok", "false")],
                                );
                                return false;
                            }
                            emit_marker(
                                "tui_contacts_device_trust",
                                None,
                                &[
                                    ("label", label),
                                    ("device", device_id),
                                    ("state", "TRUSTED"),
                                    ("ok", "true"),
                                ],
                            );
                            emit_tui_contact_flow(
                                "trust",
                                "TRUSTED",
                                label,
                                Some(device_id),
                                state.trust_onboarding_mode,
                            );
                            emit_tui_trust_promotion(
                                "trusted",
                                "explicit_operator_action",
                                label,
                                Some(device_id),
                                state.trust_onboarding_mode,
                            );
                            state.set_command_feedback("ok: device trusted (allowed to send)");
                            state.refresh_contacts();
                        }
                        "revoke" => {
                            let Some(label) = cmd.args.get(2).map(|s| s.as_str()) else {
                                state.set_command_error("contacts device revoke: missing alias");
                                emit_marker(
                                    "tui_contacts_invalid",
                                    None,
                                    &[("reason", "missing_label")],
                                );
                                return false;
                            };
                            let Some(device_id) = cmd.args.get(3).map(|s| s.as_str()) else {
                                state
                                    .set_command_error("contacts device revoke: missing device id");
                                emit_marker(
                                    "tui_contacts_invalid",
                                    None,
                                    &[("reason", "missing_device")],
                                );
                                return false;
                            };
                            let confirmed = cmd
                                .args
                                .get(4)
                                .map(|s| s.eq_ignore_ascii_case("confirm"))
                                .unwrap_or(false);
                            if !confirmed {
                                state.set_command_error(
                                    "contacts device revoke: confirmation required",
                                );
                                emit_marker(
                                    "tui_contacts_device_revoke",
                                    Some("confirm_required"),
                                    &[("label", label), ("device", device_id), ("ok", "false")],
                                );
                                return false;
                            }
                            let Some(rec) = state.contacts_records.get_mut(label) else {
                                state.set_command_error("contacts device revoke: unknown alias");
                                emit_marker(
                                    "tui_contacts_device_revoke",
                                    Some("peer_unknown"),
                                    &[("label", label), ("ok", "false")],
                                );
                                return false;
                            };
                            normalize_contact_record(label, rec);
                            let Some(idx) = contact_device_find_index(rec, device_id) else {
                                state
                                    .set_command_error("contacts device revoke: unknown device id");
                                emit_marker(
                                    "tui_contacts_device_revoke",
                                    Some("device_unknown"),
                                    &[("label", label), ("device", device_id), ("ok", "false")],
                                );
                                return false;
                            };
                            rec.devices[idx].state = "REVOKED".to_string();
                            if state.persist_contacts_cache().is_err() {
                                state
                                    .set_command_error("contacts device revoke: store unavailable");
                                emit_marker(
                                    "tui_contacts_device_revoke",
                                    Some("contacts_store_unavailable"),
                                    &[("label", label), ("ok", "false")],
                                );
                                return false;
                            }
                            emit_marker(
                                "tui_contacts_device_revoke",
                                None,
                                &[
                                    ("label", label),
                                    ("device", device_id),
                                    ("state", "REVOKED"),
                                    ("ok", "true"),
                                ],
                            );
                            state.refresh_contacts();
                        }
                        "primary" => {
                            let Some(primary_action) = cmd.args.get(2).map(|s| s.as_str()) else {
                                state.set_command_error("contacts device primary: missing action");
                                emit_marker(
                                    "tui_contacts_invalid",
                                    None,
                                    &[("reason", "missing_primary_action")],
                                );
                                return false;
                            };
                            match primary_action {
                                "set" => {
                                    let Some(label) = cmd.args.get(3).map(|s| s.as_str()) else {
                                        state.set_command_error(
                                            "contacts device primary set: missing alias",
                                        );
                                        emit_marker(
                                            "tui_contacts_invalid",
                                            None,
                                            &[("reason", "missing_label")],
                                        );
                                        return false;
                                    };
                                    let Some(device_id) = cmd.args.get(4).map(|s| s.as_str())
                                    else {
                                        state.set_command_error(
                                            "contacts device primary set: missing device id",
                                        );
                                        emit_marker(
                                            "tui_contacts_invalid",
                                            None,
                                            &[("reason", "missing_device")],
                                        );
                                        return false;
                                    };
                                    let confirmed = cmd
                                        .args
                                        .get(5)
                                        .map(|s| s.eq_ignore_ascii_case("confirm"))
                                        .unwrap_or(false);
                                    if !confirmed {
                                        state.set_command_error(
                                            "contacts device primary set: confirmation required",
                                        );
                                        emit_marker(
                                            "tui_contacts_device_primary_set",
                                            Some("confirm_required"),
                                            &[
                                                ("label", label),
                                                ("device", device_id),
                                                ("ok", "false"),
                                            ],
                                        );
                                        return false;
                                    }
                                    let Some(rec) = state.contacts_records.get_mut(label) else {
                                        state.set_command_error(
                                            "contacts device primary set: unknown alias",
                                        );
                                        emit_marker(
                                            "tui_contacts_device_primary_set",
                                            Some("peer_unknown"),
                                            &[("label", label), ("ok", "false")],
                                        );
                                        return false;
                                    };
                                    normalize_contact_record(label, rec);
                                    let Some(_) = contact_device_find_index(rec, device_id) else {
                                        state.set_command_error(
                                            "contacts device primary set: unknown device id",
                                        );
                                        emit_marker(
                                            "tui_contacts_device_primary_set",
                                            Some("device_unknown"),
                                            &[
                                                ("label", label),
                                                ("device", device_id),
                                                ("ok", "false"),
                                            ],
                                        );
                                        return false;
                                    };
                                    rec.primary_device_id = Some(device_id.to_string());
                                    normalize_contact_record(label, rec);
                                    if state.persist_contacts_cache().is_err() {
                                        state.set_command_error(
                                            "contacts device primary set: store unavailable",
                                        );
                                        emit_marker(
                                            "tui_contacts_device_primary_set",
                                            Some("contacts_store_unavailable"),
                                            &[("label", label), ("ok", "false")],
                                        );
                                        return false;
                                    }
                                    emit_marker(
                                        "tui_contacts_device_primary_set",
                                        None,
                                        &[
                                            ("label", label),
                                            ("device", device_id),
                                            ("selected", "explicit"),
                                            ("policy", "primary_only"),
                                            ("ok", "true"),
                                        ],
                                    );
                                    state.refresh_contacts();
                                }
                                "show" => {
                                    let Some(label) = cmd.args.get(3).map(|s| s.as_str()) else {
                                        state.set_command_error(
                                            "contacts device primary show: missing alias",
                                        );
                                        emit_marker(
                                            "tui_contacts_invalid",
                                            None,
                                            &[("reason", "missing_label")],
                                        );
                                        return false;
                                    };
                                    let Some(rec) = state.contacts_records.get(label) else {
                                        state.set_command_error(
                                            "contacts device primary show: unknown alias",
                                        );
                                        emit_marker(
                                            "tui_contacts_device_primary_show",
                                            Some("peer_unknown"),
                                            &[("label", label), ("ok", "false")],
                                        );
                                        return false;
                                    };
                                    let mut rec = rec.clone();
                                    let implicit = rec.primary_device_id.is_none();
                                    normalize_contact_record(label, &mut rec);
                                    let primary = primary_device(&rec)
                                        .map(|d| d.device_id.as_str())
                                        .unwrap_or("none")
                                        .to_string();
                                    emit_marker(
                                        "tui_contacts_device_primary_show",
                                        None,
                                        &[
                                            ("label", label),
                                            ("device", primary.as_str()),
                                            (
                                                "selected",
                                                if implicit { "implicit" } else { "explicit" },
                                            ),
                                            ("policy", "primary_only"),
                                            ("ok", "true"),
                                        ],
                                    );
                                }
                                _ => {
                                    state.set_command_error(
                                        "contacts device primary: unknown action",
                                    );
                                    emit_marker(
                                        "tui_contacts_invalid",
                                        None,
                                        &[("reason", "unknown_primary_action")],
                                    );
                                    return false;
                                }
                            }
                        }
                        _ => {
                            state.set_command_error("contacts device: unknown action");
                            emit_marker(
                                "tui_contacts_invalid",
                                None,
                                &[("reason", "unknown_device_action")],
                            );
                            return false;
                        }
                    }
                }
                "block" => {
                    let Some(label) = cmd.args.get(1).map(|s| s.as_str()) else {
                        state.set_command_error("contacts: missing label");
                        emit_marker("tui_contacts_invalid", None, &[("reason", "missing_label")]);
                        return false;
                    };
                    if let Some(rec) = state.contacts_records.get_mut(label) {
                        rec.blocked = true;
                        match state.persist_contacts_cache() {
                            Ok(()) => emit_marker(
                                "tui_contacts_block",
                                None,
                                &[("label", label), ("ok", "true")],
                            ),
                            Err(_) => {
                                state.set_command_error("contacts: store unavailable");
                                emit_marker(
                                    "tui_contacts_block",
                                    Some("contacts_store_unavailable"),
                                    &[("label", label), ("ok", "false")],
                                );
                                return false;
                            }
                        }
                    } else {
                        state.set_command_error("contacts: unknown alias");
                        emit_marker(
                            "tui_contacts_block",
                            Some("peer_unknown"),
                            &[("label", label), ("ok", "false")],
                        );
                        return false;
                    }
                    state.refresh_contacts();
                }
                "unblock" => {
                    let Some(label) = cmd.args.get(1).map(|s| s.as_str()) else {
                        state.set_command_error("contacts: missing label");
                        emit_marker("tui_contacts_invalid", None, &[("reason", "missing_label")]);
                        return false;
                    };
                    if let Some(rec) = state.contacts_records.get_mut(label) {
                        rec.blocked = false;
                        match state.persist_contacts_cache() {
                            Ok(()) => emit_marker(
                                "tui_contacts_unblock",
                                None,
                                &[("label", label), ("ok", "true")],
                            ),
                            Err(_) => {
                                state.set_command_error("contacts: store unavailable");
                                emit_marker(
                                    "tui_contacts_unblock",
                                    Some("contacts_store_unavailable"),
                                    &[("label", label), ("ok", "false")],
                                );
                                return false;
                            }
                        }
                    } else {
                        state.set_command_error("contacts: unknown alias");
                        emit_marker(
                            "tui_contacts_unblock",
                            Some("peer_unknown"),
                            &[("label", label), ("ok", "false")],
                        );
                        return false;
                    }
                    state.refresh_contacts();
                }
                "add" => {
                    let Some(label) = cmd.args.get(1).map(|s| s.as_str()) else {
                        state.set_command_error("contacts: missing label");
                        emit_marker("tui_contacts_invalid", None, &[("reason", "missing_label")]);
                        return false;
                    };
                    if !tui_alias_is_valid(label) {
                        state
                            .set_command_error("contacts: alias must be 2-32 chars [A-Za-z0-9._-]");
                        emit_marker("tui_contacts_invalid", None, &[("reason", "alias_invalid")]);
                        return false;
                    }
                    let Some(code) = cmd.args.get(2).map(|s| s.as_str()) else {
                        state.set_command_error("contacts: missing verification code");
                        emit_marker("tui_contacts_invalid", None, &[("reason", "missing_fp")]);
                        return false;
                    };
                    if !tui_verification_code_is_valid(code) {
                        state.set_command_error("contacts: invalid verification code format");
                        emit_marker("tui_contacts_invalid", None, &[("reason", "invalid_code")]);
                        return false;
                    }
                    let route_token = match cmd.args.get(3).map(|s| s.as_str()) {
                        Some(raw) => match normalize_route_token(raw) {
                            Ok(token) => Some(token),
                            Err(code) => {
                                state.set_command_error(format!("contacts: {}", code));
                                emit_marker(
                                    "tui_contacts_invalid",
                                    Some(code),
                                    &[("reason", "invalid_route_token")],
                                );
                                return false;
                            }
                        },
                        None => Some(generate_route_token()),
                    };
                    let rec = ContactRecord {
                        fp: code.to_ascii_uppercase(),
                        status: "UNVERIFIED".to_string(),
                        blocked: false,
                        seen_at: None,
                        sig_fp: None,
                        route_token: route_token.clone(),
                        primary_device_id: None,
                        devices: vec![ContactDeviceRecord {
                            device_id: device_id_short(label, None, code),
                            fp: code.to_ascii_uppercase(),
                            sig_fp: None,
                            state: "UNVERIFIED".to_string(),
                            route_token: route_token.clone(),
                            seen_at: None,
                            label: None,
                        }],
                    };
                    state.contacts_records.insert(label.to_string(), rec);
                    match state.persist_contacts_cache() {
                        Ok(()) => emit_marker(
                            "tui_contacts_add",
                            None,
                            &[("label", label), ("ok", "true"), ("status", "UNVERIFIED")],
                        ),
                        Err(_) => {
                            state.set_command_error("contacts: store unavailable");
                            emit_marker(
                                "tui_contacts_add",
                                Some("contacts_store_unavailable"),
                                &[("label", label), ("ok", "false")],
                            );
                            return false;
                        }
                    }
                    emit_tui_contact_flow(
                        "add",
                        "DISCOVERED",
                        label,
                        None,
                        state.trust_onboarding_mode,
                    );
                    state.refresh_contacts();
                }
                "route" => {
                    let Some(action) = cmd.args.get(1).map(|s| s.as_str()) else {
                        state.set_command_error("contacts: missing route subcommand");
                        emit_marker(
                            "tui_contacts_invalid",
                            None,
                            &[("reason", "missing_route_subcmd")],
                        );
                        return false;
                    };
                    if action != "set" {
                        state.set_command_error("contacts: unknown route subcommand");
                        emit_marker(
                            "tui_contacts_invalid",
                            None,
                            &[("reason", "unknown_route_subcmd")],
                        );
                        return false;
                    }
                    let Some(label) = cmd.args.get(2).map(|s| s.as_str()) else {
                        state.set_command_error("contacts: missing alias");
                        emit_marker("tui_contacts_invalid", None, &[("reason", "missing_label")]);
                        return false;
                    };
                    let Some(raw_token) = cmd.args.get(3).map(|s| s.as_str()) else {
                        state.set_command_error("contacts: missing route token");
                        emit_marker("tui_contacts_invalid", None, &[("reason", "missing_token")]);
                        return false;
                    };
                    let token = match normalize_route_token(raw_token) {
                        Ok(v) => v,
                        Err(code) => {
                            state.set_command_error(format!("contacts: {}", code));
                            emit_marker("tui_contacts_invalid", Some(code), &[]);
                            return false;
                        }
                    };
                    let rec =
                        state
                            .contacts_records
                            .entry(label.to_string())
                            .or_insert(ContactRecord {
                                fp: "UNSET".to_string(),
                                status: "UNVERIFIED".to_string(),
                                blocked: false,
                                seen_at: None,
                                sig_fp: None,
                                route_token: None,
                                primary_device_id: None,
                                devices: vec![ContactDeviceRecord {
                                    device_id: device_id_short(label, None, "UNSET"),
                                    fp: "UNSET".to_string(),
                                    sig_fp: None,
                                    state: "UNVERIFIED".to_string(),
                                    route_token: None,
                                    seen_at: None,
                                    label: None,
                                }],
                            });
                    rec.route_token = Some(token);
                    let primary_route_token = rec.route_token.clone();
                    if let Some(primary) = primary_device_mut(rec) {
                        primary.route_token = primary_route_token;
                    }
                    if state.persist_contacts_cache().is_err() {
                        state.set_command_error("contacts: store unavailable");
                        emit_marker(
                            "tui_contacts_route",
                            Some("contacts_store_unavailable"),
                            &[("label", label), ("ok", "false")],
                        );
                        return false;
                    }
                    emit_marker(
                        "tui_contacts_route",
                        None,
                        &[("label", label), ("ok", "true"), ("action", "set")],
                    );
                    state.push_cmd_result(
                        "contacts route set",
                        true,
                        "contact route token stored (redacted)",
                    );
                    state.set_status_last_command_result(format!("contact route set {}", label));
                    state.set_command_feedback("ok: contact route token set");
                    state.refresh_contacts();
                }
                _ => {
                    state.set_command_error("contacts: unknown subcommand");
                    emit_marker(
                        "tui_contacts_invalid",
                        None,
                        &[("reason", "unknown_subcmd")],
                    );
                }
            }
            false
        }
        "trust" => {
            emit_marker("tui_cmd", None, &[("cmd", "trust")]);
            let Some(action) = cmd.args.first().map(|s| s.as_str()) else {
                state.set_command_error("trust: missing action (use pin)");
                emit_marker("tui_trust_invalid", None, &[("reason", "missing_action")]);
                return false;
            };
            match action {
                "pin" => {
                    let Some(label) = cmd.args.get(1).map(|s| s.as_str()) else {
                        state.set_command_error("trust: missing alias");
                        emit_marker("tui_trust_invalid", None, &[("reason", "missing_label")]);
                        return false;
                    };
                    let confirmed = cmd
                        .args
                        .get(2)
                        .map(|s| s.eq_ignore_ascii_case("confirm"))
                        .unwrap_or(false);
                    if !confirmed {
                        state.set_command_error(
                            "trust: confirmation required (use '/trust pin <alias> confirm')",
                        );
                        emit_marker(
                            "tui_trust_pin",
                            Some("confirm_required"),
                            &[("label", label), ("ok", "false")],
                        );
                        return false;
                    }
                    let Some(rec) = state.contacts_records.get_mut(label) else {
                        state.set_command_error("trust: unknown alias");
                        emit_marker(
                            "tui_trust_pin",
                            Some("peer_unknown"),
                            &[("label", label), ("ok", "false")],
                        );
                        return false;
                    };
                    normalize_contact_record(label, rec);
                    rec.status = "PINNED".to_string();
                    if let Some(primary) = primary_device_mut(rec) {
                        primary.state = "TRUSTED".to_string();
                    }
                    if state.persist_contacts_cache().is_err() {
                        state.set_command_error("trust: store unavailable");
                        emit_marker(
                            "tui_trust_pin",
                            Some("contacts_store_unavailable"),
                            &[("label", label), ("ok", "false")],
                        );
                        return false;
                    }
                    state.push_cmd_result("trust pin", true, "contact pinned");
                    state.set_status_last_command_result(format!("trust pinned {}", label));
                    state.set_command_feedback("ok: contact trusted (allowed to send)");
                    emit_marker(
                        "tui_trust_pin",
                        None,
                        &[("label", label), ("ok", "true"), ("status", "PINNED")],
                    );
                    state.refresh_contacts();
                }
                "mode" => {
                    let mode_arg = cmd.args.get(1).map(|s| s.as_str());
                    match mode_arg {
                        None | Some("show") => {
                            let mode = state.trust_onboarding_mode.as_str();
                            emit_tui_named_marker("QSC_TUI_TRUST_MODE", &[("mode", mode)]);
                            state.push_cmd_result("trust mode", true, format!("mode={mode}"));
                        }
                        Some(raw) => {
                            let Some(mode) = TrustOnboardingMode::from_raw(raw) else {
                                state.set_command_error("trust mode: expected strict|balanced");
                                emit_tui_named_marker(
                                    "QSC_TUI_TRUST_MODE",
                                    &[("ok", "false"), ("reason", "invalid_mode")],
                                );
                                return false;
                            };
                            if state
                                .persist_account_secret(TUI_TRUST_MODE_SECRET_KEY, mode.as_str())
                                .is_err()
                            {
                                state.set_command_error("trust mode: store unavailable");
                                emit_tui_named_marker(
                                    "QSC_TUI_TRUST_MODE",
                                    &[("ok", "false"), ("reason", "contacts_store_unavailable")],
                                );
                                return false;
                            }
                            state.trust_onboarding_mode = mode;
                            emit_tui_named_marker(
                                "QSC_TUI_TRUST_MODE",
                                &[("mode", mode.as_str()), ("ok", "true")],
                            );
                            state.push_cmd_result(
                                "trust mode",
                                true,
                                format!("mode={}", mode.as_str()),
                            );
                        }
                    }
                }
                _ => {
                    state.set_command_error("trust: unknown action");
                    emit_marker("tui_trust_invalid", None, &[("reason", "unknown_action")]);
                }
            }
            false
        }
        "requests" => {
            emit_marker("tui_cmd", None, &[("cmd", "requests")]);
            let action = cmd.args.first().map(|s| s.as_str()).unwrap_or("list");
            match action {
                "list" => {
                    let items = contact_request_list().unwrap_or_default();
                    let count_s = items.len().to_string();
                    emit_tui_contact_request("list", "all", None);
                    state.push_cmd_result("requests list", true, format!("count={count_s}"));
                }
                "accept" => {
                    let Some(label) = cmd.args.get(1).map(|s| s.as_str()) else {
                        state.set_command_error("requests accept: missing alias");
                        return false;
                    };
                    let removed = contact_request_remove(label).unwrap_or(false);
                    if !removed {
                        state.set_command_error("requests accept: unknown request");
                        return false;
                    }
                    let mut rec =
                        state
                            .contacts_records
                            .get(label)
                            .cloned()
                            .unwrap_or(ContactRecord {
                                fp: "UNSET".to_string(),
                                status: "UNVERIFIED".to_string(),
                                blocked: false,
                                seen_at: None,
                                sig_fp: None,
                                route_token: None,
                                primary_device_id: None,
                                devices: vec![ContactDeviceRecord {
                                    device_id: device_id_short(label, None, "UNSET"),
                                    fp: "UNSET".to_string(),
                                    sig_fp: None,
                                    state: "UNVERIFIED".to_string(),
                                    route_token: None,
                                    seen_at: None,
                                    label: Some("request".to_string()),
                                }],
                            });
                    normalize_contact_record(label, &mut rec);
                    rec.status = "UNVERIFIED".to_string();
                    if state.persist_contacts_cache_with(label, rec).is_err() {
                        state.set_command_error("requests accept: store unavailable");
                        return false;
                    }
                    state.refresh_contacts();
                    emit_tui_contact_request("accept", label, None);
                    emit_tui_contact_flow(
                        "add",
                        "DISCOVERED",
                        label,
                        None,
                        state.trust_onboarding_mode,
                    );
                    state.push_cmd_result("requests accept", true, label.to_string());
                }
                "ignore" => {
                    let Some(label) = cmd.args.get(1).map(|s| s.as_str()) else {
                        state.set_command_error("requests ignore: missing alias");
                        return false;
                    };
                    let removed = contact_request_remove(label).unwrap_or(false);
                    if !removed {
                        state.set_command_error("requests ignore: unknown request");
                        return false;
                    }
                    emit_tui_contact_request("ignore", label, None);
                    state.push_cmd_result("requests ignore", true, label.to_string());
                }
                "block" => {
                    let Some(label) = cmd.args.get(1).map(|s| s.as_str()) else {
                        state.set_command_error("requests block: missing alias");
                        return false;
                    };
                    let _ = contact_request_remove(label);
                    let mut rec =
                        state
                            .contacts_records
                            .get(label)
                            .cloned()
                            .unwrap_or(ContactRecord {
                                fp: "UNSET".to_string(),
                                status: "REVOKED".to_string(),
                                blocked: true,
                                seen_at: None,
                                sig_fp: None,
                                route_token: None,
                                primary_device_id: None,
                                devices: vec![ContactDeviceRecord {
                                    device_id: device_id_short(label, None, "UNSET"),
                                    fp: "UNSET".to_string(),
                                    sig_fp: None,
                                    state: "REVOKED".to_string(),
                                    route_token: None,
                                    seen_at: None,
                                    label: Some("blocked_request".to_string()),
                                }],
                            });
                    normalize_contact_record(label, &mut rec);
                    rec.blocked = true;
                    if let Some(primary) = primary_device_mut(&mut rec) {
                        primary.state = "REVOKED".to_string();
                    }
                    if state.persist_contacts_cache_with(label, rec).is_err() {
                        state.set_command_error("requests block: store unavailable");
                        return false;
                    }
                    state.refresh_contacts();
                    emit_tui_contact_request("block", label, None);
                    state.push_cmd_result("requests block", true, label.to_string());
                }
                _ => {
                    state.set_command_error("requests: unknown action");
                    return false;
                }
            }
            false
        }
        "verify" => {
            emit_marker("tui_cmd", None, &[("cmd", "verify")]);
            let Some(label) = cmd.args.first().map(|s| s.as_str()) else {
                state.set_command_error("verify: missing alias");
                emit_marker("tui_contacts_invalid", None, &[("reason", "missing_label")]);
                return false;
            };
            let Some(code) = cmd.args.get(1).map(|s| s.as_str()) else {
                state.set_command_error("verify: missing verification code");
                emit_marker("tui_contacts_invalid", None, &[("reason", "missing_fp")]);
                return false;
            };
            if !tui_verification_code_is_valid(code) {
                state.set_command_error("verify: invalid verification code format");
                emit_marker("tui_contacts_invalid", None, &[("reason", "invalid_code")]);
                return false;
            }
            let Some(rec) = state.contacts_records.get_mut(label) else {
                state.set_command_error("verify: unknown alias");
                emit_marker(
                    "tui_contacts_verify",
                    Some("peer_unknown"),
                    &[("label", label), ("ok", "false")],
                );
                return false;
            };
            normalize_contact_record(label, rec);
            let expected = primary_device(rec)
                .map(|d| d.fp.to_ascii_uppercase())
                .unwrap_or_else(|| rec.fp.to_ascii_uppercase());
            let provided = code.to_ascii_uppercase();
            if expected == provided {
                let mode = state.trust_onboarding_mode;
                rec.status = "VERIFIED".to_string();
                if let Some(primary) = primary_device_mut(rec) {
                    primary.state = "VERIFIED".to_string();
                    if mode == TrustOnboardingMode::Balanced {
                        primary.state = "TRUSTED".to_string();
                        rec.status = "PINNED".to_string();
                    }
                }
                if state.persist_contacts_cache().is_err() {
                    state.set_command_error("verify: store unavailable");
                    emit_marker(
                        "tui_contacts_verify",
                        Some("contacts_store_unavailable"),
                        &[("label", label), ("ok", "false")],
                    );
                    return false;
                }
                emit_marker(
                    "tui_contacts_verify",
                    None,
                    &[
                        ("label", label),
                        ("ok", "true"),
                        (
                            "status",
                            if mode == TrustOnboardingMode::Balanced {
                                "TRUSTED"
                            } else {
                                "VERIFIED"
                            },
                        ),
                    ],
                );
                emit_tui_contact_flow(
                    "verify",
                    if mode == TrustOnboardingMode::Balanced {
                        "TRUSTED"
                    } else {
                        "VERIFIED"
                    },
                    label,
                    None,
                    mode,
                );
                if mode == TrustOnboardingMode::Balanced {
                    emit_tui_trust_promotion("trusted", "verified_match", label, None, mode);
                    state.set_command_feedback("ok: verification matched and contact auto-trusted");
                } else {
                    emit_tui_trust_promotion("verified_only", "strict_mode", label, None, mode);
                    state.set_command_feedback(
                        "ok: verification matched; strict mode requires explicit trust",
                    );
                }
            } else {
                rec.status = "CHANGED".to_string();
                if let Some(primary) = primary_device_mut(rec) {
                    primary.state = "CHANGED".to_string();
                }
                let _ = state.persist_contacts_cache();
                emit_marker(
                    "tui_contacts_verify",
                    Some("verification_mismatch"),
                    &[("label", label), ("ok", "false"), ("status", "CHANGED")],
                );
                emit_tui_contact_flow(
                    "verify",
                    "CHANGED",
                    label,
                    None,
                    state.trust_onboarding_mode,
                );
                state.set_command_error("verify: verification code mismatch");
                return false;
            }
            state.refresh_contacts();
            false
        }
        "messages" => {
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
        "msg" => {
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
            if thread.eq_ignore_ascii_case("Note to Self")
                || thread.eq_ignore_ascii_case("Note_to_Self")
            {
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
            } else {
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
            }
            false
        }
        "files" => {
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
        "injectmsg" => {
            emit_marker("tui_cmd", None, &[("cmd", "injectmsg")]);
            let peer = cmd.args.first().map(|s| s.as_str()).unwrap_or("peer-0");
            let state_name = cmd.args.get(1).map(|s| s.as_str()).unwrap_or("RECEIVED");
            let detail = if cmd.args.len() > 2 {
                cmd.args[2..].join(" ")
            } else {
                "source=test_harness".to_string()
            };
            let _ = state.append_tui_timeline_entry(
                peer,
                "in",
                detail.len(),
                "msg",
                MessageState::Received,
            );
            state.record_message_line(peer, state_name, "in", detail.as_str());
            false
        }
        "injectevent" => {
            emit_marker("tui_cmd", None, &[("cmd", "injectevent")]);
            let kind = cmd.args.first().map(|s| s.as_str()).unwrap_or("activity");
            let action = cmd.args.get(1).map(|s| s.as_str()).unwrap_or("test");
            state.push_event(kind, action);
            false
        }
        "envelope" => {
            emit_marker("tui_cmd", None, &[("cmd", "envelope")]);
            state.refresh_envelope(state.last_payload_len());
            false
        }
        "export" => {
            emit_marker("tui_cmd", None, &[("cmd", "export")]);
            false
        }
        other => {
            state.set_command_error(format!("unknown command: {}", other));
            emit_marker("tui_cmd", None, &[("cmd", other)]);
            false
        }
    };
    if state.cmd_results.len() == before_results_len {
        if let Some(err) = state.command_error.clone() {
            state.push_cmd_result(command_label.as_str(), false, err);
        } else {
            state.push_cmd_result(command_label.as_str(), true, "ok");
        }
    }
    if !exit && state.command_error.is_none() {
        if let Some(entry) = state.cmd_results.back() {
            if let Some(msg) = entry
                .strip_prefix("[ok] /")
                .and_then(|v| v.split_once(' '))
                .map(|(_, msg)| msg.to_string())
            {
                state.set_command_feedback(format!("ok: {}", msg));
            } else {
                state.set_command_feedback(format!("ok: /{}", command_label));
            }
        } else {
            state.set_command_feedback(format!("ok: /{}", command_label));
        }
    }
    state.end_command_tracking();
    exit
}
