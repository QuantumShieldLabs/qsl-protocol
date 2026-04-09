use super::state::tui_file_display_state;
use super::*;

mod catalog;
mod contacts;
mod dispatch;
mod key;
mod locked;
mod messages;
mod navigation;
mod relay;

use self::catalog::{
    focus_mode_for_fkey, focus_mode_for_target, inspector_for_fkey, inspector_pane_for_target,
};
pub(super) use self::catalog::{tui_help_items, TuiHelpItem};
use self::dispatch::dispatch_tui_command;
use self::locked::{
    handle_locked_reject, handle_tui_account_destroy_key, handle_tui_locked_command,
    handle_tui_locked_key, tui_alias_is_valid, tui_verification_code_is_valid,
};
use self::relay::{
    tui_msg_autotrust_first_use, tui_msg_ensure_handshake, tui_msg_recv_poll_bounded,
    tui_send_via_relay,
};

fn command_label_for_tracking(cmd: &TuiParsedCmd) -> String {
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
    command_label
}

pub(super) fn handle_tui_key(state: &mut TuiState, key: KeyEvent) -> bool {
    key::handle_tui_key(state, key)
}

pub(super) fn wipe_account_local_state_best_effort() {
    locked::wipe_account_local_state_best_effort()
}

pub(super) fn tui_receive_via_relay(state: &mut TuiState, from: &str) {
    relay::tui_receive_via_relay(state, from)
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
    let command_label = command_label_for_tracking(cmd);
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
    let exit = dispatch_tui_command(cmd, state);
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
