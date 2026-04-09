use super::*;

fn move_focus_selection(state: &mut TuiState, delta: i32) {
    if state.mode == TuiMode::FocusContacts {
        state.contacts_move(delta);
    } else if state.mode == TuiMode::FocusFiles {
        state.files_move(delta);
    } else {
        let max_len = state.focus_max_len();
        state.focus_scroll_move(delta, max_len);
    }
}

fn page_focus_selection(state: &mut TuiState, delta: i32) {
    let rows = state.focus_view_rows() as i32;
    move_focus_selection(state, rows.saturating_mul(delta));
}

pub(super) fn dispatch_navigation_command(cmd: &TuiParsedCmd, state: &mut TuiState) -> bool {
    match cmd.cmd.as_str() {
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
                move_focus_selection(state, 1);
            }
            false
        }
        "up" => {
            emit_marker("tui_cmd", None, &[("cmd", "up")]);
            if state.is_focus_mode() {
                move_focus_selection(state, -1);
            }
            false
        }
        "pgdn" | "pagedown" => {
            emit_marker("tui_cmd", None, &[("cmd", "pgdn")]);
            if state.is_focus_mode() {
                page_focus_selection(state, 1);
            }
            false
        }
        "pgup" | "pageup" => {
            emit_marker("tui_cmd", None, &[("cmd", "pgup")]);
            if state.is_focus_mode() {
                page_focus_selection(state, -1);
            }
            false
        }
        "exit" | "quit" => {
            emit_marker("tui_cmd", None, &[("cmd", "exit")]);
            true
        }
        _ => {
            state.set_command_error(format!("unknown command: {}", cmd.cmd));
            emit_marker("tui_cmd", None, &[("cmd", cmd.cmd.as_str())]);
            false
        }
    }
}
