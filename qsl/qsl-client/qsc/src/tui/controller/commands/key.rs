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
            KeyCode::Up => move_focus_selection(state, -1),
            KeyCode::Down => move_focus_selection(state, 1),
            KeyCode::PageUp => page_focus_selection(state, -1),
            KeyCode::PageDown => page_focus_selection(state, 1),
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
                let exit = super::handle_tui_command(&cmd, state);
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
