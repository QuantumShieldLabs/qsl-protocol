use super::state::tui_file_display_state;
use super::*;

fn focus_mode_for_fkey(code: KeyCode) -> Option<TuiMode> {
    match code {
        KeyCode::F(2) => Some(TuiMode::FocusEvents),
        KeyCode::F(3) => Some(TuiMode::FocusStatus),
        KeyCode::F(4) => Some(TuiMode::FocusSession),
        KeyCode::F(5) => Some(TuiMode::FocusContacts),
        _ => None,
    }
}

fn inspector_for_fkey(code: KeyCode) -> Option<TuiInspectorPane> {
    match code {
        KeyCode::F(2) => Some(TuiInspectorPane::Events),
        KeyCode::F(3) => Some(TuiInspectorPane::Status),
        KeyCode::F(4) => Some(TuiInspectorPane::Session),
        KeyCode::F(5) => Some(TuiInspectorPane::Contacts),
        _ => None,
    }
}

fn locked_cmd_input_value(input: &str, command: &str) -> String {
    let trimmed = input.trim();
    let prefix = format!("/{}", command);
    if trimmed == prefix {
        return String::new();
    }
    if let Some(rest) = trimmed.strip_prefix(&(prefix.clone() + " ")) {
        return rest.trim().to_string();
    }
    trimmed.to_string()
}

fn handle_locked_prompt_submit(state: &mut TuiState) -> bool {
    match state.locked_flow.clone() {
        LockedFlow::None => {
            if let Some(cmd) = parse_tui_command(state.cmd_input.as_str()) {
                let exit = handle_tui_command(&cmd, state);
                state.cmd_input_clear();
                return exit;
            }
            state.cmd_input_clear();
            state.locked_clear_error();
            false
        }
        LockedFlow::UnlockPassphrase => {
            let mut passphrase = locked_cmd_input_value(state.cmd_input.as_str(), "unlock");
            state.cmd_input_clear();
            if passphrase.is_empty() {
                state.locked_set_error("passphrase required");
                emit_marker(
                    "tui_unlock",
                    Some("vault_locked"),
                    &[("ok", "false"), ("reason", "passphrase_required")],
                );
                return false;
            }
            match state.unlock_with_policy(passphrase.as_str()) {
                UnlockAttemptOutcome::Unlocked => {
                    state.set_locked_state(false, "explicit_command");
                    state.locked_clear_error();
                    emit_marker("tui_unlock", None, &[("ok", "true")]);
                }
                UnlockAttemptOutcome::Wiped => {
                    state.locked_set_error(
                        "vault wiped after failed unlock attempts; run /init to rebuild local state",
                    );
                    state.command_error = Some(format!(
                        "vault: {} (run /init to rebuild local state)",
                        QSC_ERR_VAULT_WIPED_AFTER_FAILED_UNLOCKS
                    ));
                }
                UnlockAttemptOutcome::Rejected => {
                    state
                        .locked_set_error("unlock failed: passphrase did not open the local vault");
                    emit_marker(
                        "tui_unlock",
                        Some("vault_locked"),
                        &[("ok", "false"), ("reason", "passphrase_invalid")],
                    );
                }
            }
            passphrase.zeroize();
            false
        }
        LockedFlow::InitAlias => {
            let alias = locked_cmd_input_value(state.cmd_input.as_str(), "init");
            if !tui_alias_is_valid(alias.as_str()) {
                state.locked_set_error("alias must be 2-32 chars [A-Za-z0-9._-]");
                emit_marker(
                    "tui_init_reject",
                    Some("alias_invalid"),
                    &[("ok", "false"), ("reason", "alias_invalid")],
                );
                state.cmd_input_clear();
                return false;
            }
            state.locked_flow = LockedFlow::InitPassphrase { alias };
            state.cmd_input_clear();
            state.locked_clear_error();
            emit_marker("tui_init_wizard", None, &[("step", "passphrase")]);
            false
        }
        LockedFlow::InitPassphrase { alias } => {
            let passphrase = state.cmd_input.clone();
            if !tui_passphrase_is_strong(passphrase.as_str()) {
                state.locked_set_error("passphrase must be 16+ chars and not weak/common");
                emit_marker(
                    "tui_init_reject",
                    Some("passphrase_weak"),
                    &[("ok", "false"), ("reason", "passphrase_weak")],
                );
                state.cmd_input_clear();
                return false;
            }
            state.locked_flow = LockedFlow::InitConfirm { alias, passphrase };
            state.cmd_input_clear();
            state.locked_clear_error();
            emit_marker("tui_init_wizard", None, &[("step", "confirm")]);
            false
        }
        LockedFlow::InitConfirm { alias, passphrase } => {
            let confirm = state.cmd_input.clone();
            state.cmd_input_clear();
            if confirm != passphrase {
                state.locked_set_error("passphrase confirmation does not match");
                emit_marker(
                    "tui_init_reject",
                    Some("passphrase_mismatch"),
                    &[("ok", "false"), ("reason", "passphrase_mismatch")],
                );
                state.locked_flow = LockedFlow::InitPassphrase { alias };
                emit_marker("tui_init_wizard", None, &[("step", "passphrase")]);
                return false;
            }
            state.locked_flow = LockedFlow::InitDecision { alias, passphrase };
            state.locked_clear_error();
            emit_marker("tui_init_wizard", None, &[("step", "confirm_decision")]);
            false
        }
        LockedFlow::InitDecision { alias, passphrase } => {
            let decision = state.cmd_input.trim().to_ascii_uppercase();
            if decision == "N" || decision == "NO" {
                state.locked_flow = LockedFlow::None;
                state.cmd_input_clear();
                state.locked_clear_error();
                emit_marker(
                    "tui_init_reject",
                    Some("confirm_cancelled"),
                    &[("ok", "false"), ("reason", "confirm_cancelled")],
                );
                return false;
            }
            if decision != "Y"
                && decision != "YES"
                && decision != "I AGREE"
                && decision != "I UNDERSTAND"
            {
                state.locked_set_error("confirm with I AGREE or N");
                emit_marker(
                    "tui_init_reject",
                    Some("confirm_required"),
                    &[("ok", "false"), ("reason", "confirm_required")],
                );
                state.cmd_input_clear();
                return false;
            }
            match tui_try_vault_init(passphrase.as_str()) {
                Ok(()) => {}
                Err(code) => {
                    state.locked_set_error("vault init failed");
                    emit_marker(
                        "tui_init_reject",
                        Some(code.as_str()),
                        &[("ok", "false"), ("reason", "vault_init_failed")],
                    );
                    state.locked_flow = LockedFlow::InitAlias;
                    emit_marker("tui_init_wizard", None, &[("step", "alias")]);
                    return false;
                }
            }
            if let Err(code) = initialize_account_after_init(alias.as_str(), passphrase.as_str()) {
                emit_marker(
                    "tui_init_reject",
                    Some(code.as_str()),
                    &[("ok", "false"), ("reason", "account_init_failed")],
                );
                state.locked_flow = LockedFlow::InitAlias;
                state.locked_set_error("failed to initialize account");
                emit_marker("tui_init_wizard", None, &[("step", "alias")]);
                return false;
            }
            let _ = vault_security_state_clear_files();
            state.unlock_attempt_limit = None;
            state.failed_unlock_attempts = 0;
            state.mark_vault_present();
            state.set_locked_state(true, "post_init_locked");
            state.locked_flow = LockedFlow::None;
            state.locked_clear_error();
            emit_marker(
                "tui_init",
                None,
                &[("ok", "true"), ("alias", "stored_local_only")],
            );
            false
        }
    }
}

fn handle_tui_locked_key(state: &mut TuiState, key: KeyEvent) -> bool {
    let no_ctrl_alt = !key
        .modifiers
        .intersects(KeyModifiers::CONTROL | KeyModifiers::ALT | KeyModifiers::SUPER);
    match key.code {
        KeyCode::Up => {
            state.nav_move(-1);
            false
        }
        KeyCode::Down => {
            state.nav_move(1);
            false
        }
        KeyCode::Enter => {
            if state.home_focus == TuiHomeFocus::Nav {
                state.locked_nav_activate()
            } else if state.home_focus == TuiHomeFocus::Command {
                handle_locked_prompt_submit(state)
            } else {
                false
            }
        }
        KeyCode::Tab => {
            state.locked_focus_toggle();
            false
        }
        KeyCode::Esc => {
            state.home_focus = TuiHomeFocus::Nav;
            state.locked_flow = LockedFlow::None;
            state.locked_clear_error();
            state.cmd_input_clear();
            emit_marker("tui_focus_home", None, &[("pane", state.home_focus_name())]);
            false
        }
        KeyCode::Char('/') => {
            state.home_focus = TuiHomeFocus::Command;
            state.cmd_input_push('/');
            state.locked_clear_error();
            emit_marker("tui_focus_home", None, &[("pane", state.home_focus_name())]);
            false
        }
        KeyCode::Backspace | KeyCode::Delete => {
            if state.home_focus == TuiHomeFocus::Command {
                if !state.cmd_input.is_empty() {
                    state.cmd_input_pop();
                    state.locked_clear_error();
                } else {
                    match state.locked_flow.clone() {
                        LockedFlow::InitPassphrase { alias: _ } => {
                            state.locked_flow = LockedFlow::InitAlias;
                            state.locked_clear_error();
                            emit_marker("tui_init_wizard", None, &[("step", "alias")]);
                        }
                        LockedFlow::InitConfirm { alias, .. } => {
                            state.locked_flow = LockedFlow::InitPassphrase { alias };
                            state.locked_clear_error();
                            emit_marker("tui_init_wizard", None, &[("step", "passphrase")]);
                        }
                        LockedFlow::InitDecision { alias, passphrase } => {
                            state.locked_flow = LockedFlow::InitConfirm { alias, passphrase };
                            state.locked_clear_error();
                            emit_marker("tui_init_wizard", None, &[("step", "confirm")]);
                        }
                        _ => {}
                    }
                }
            }
            false
        }
        KeyCode::Char(ch) => {
            if no_ctrl_alt && state.home_focus == TuiHomeFocus::Command && !ch.is_control() {
                state.cmd_input_push(ch);
                state.locked_clear_error();
            }
            false
        }
        _ => false,
    }
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

fn handle_tui_account_destroy_key(state: &mut TuiState, key: KeyEvent) -> bool {
    let no_ctrl_alt = !key
        .modifiers
        .intersects(KeyModifiers::CONTROL | KeyModifiers::ALT);
    match key.code {
        KeyCode::Esc => {
            state.cancel_account_destroy_prompt();
            false
        }
        KeyCode::Enter => {
            if state.home_focus != TuiHomeFocus::Command {
                state.home_focus = TuiHomeFocus::Command;
                emit_marker("tui_focus_home", None, &[("pane", state.home_focus_name())]);
                return false;
            }
            match state.account_destroy_flow.clone() {
                AccountDestroyFlow::None => false,
                AccountDestroyFlow::Passphrase => {
                    if state.cmd_input.is_empty() {
                        state.account_destroy_set_error("passphrase required");
                        state.push_cmd_result("account destroy", false, "passphrase required");
                        return false;
                    }
                    if vault::unlock_with_passphrase(state.cmd_input.as_str()).is_err() {
                        state.account_destroy_set_error("current passphrase invalid");
                        state.push_cmd_result("account destroy", false, "passphrase invalid");
                        state.cmd_input_clear();
                        return false;
                    }
                    let passphrase = state.cmd_input.clone();
                    state.account_destroy_flow = AccountDestroyFlow::ConfirmDecision { passphrase };
                    state.account_destroy_clear_error();
                    state.cmd_input_clear();
                    emit_marker("tui_account_destroy", None, &[("step", "confirm")]);
                    false
                }
                AccountDestroyFlow::ConfirmDecision { passphrase } => {
                    let mut passphrase = passphrase;
                    let decision = state.cmd_input.trim().to_ascii_uppercase();
                    if decision == "N" || decision == "NO" {
                        state.account_destroy_set_error("destroy cancelled");
                        state.push_cmd_result("account destroy", false, "cancelled");
                        state.cmd_input_clear();
                        state.cancel_account_destroy_prompt();
                        return false;
                    }
                    if decision != "Y" && decision != "YES" {
                        state.account_destroy_set_error("confirm with Y or N");
                        state.push_cmd_result("account destroy", false, "confirmation required");
                        state.cmd_input_clear();
                        return false;
                    }
                    match vault::destroy_with_passphrase(passphrase.as_str()) {
                        Ok(()) => {
                            wipe_account_local_state_best_effort();
                            let _ = vault_security_state_clear_files();
                            state.close_vault_session();
                            state.mark_vault_absent();
                            state.unlock_attempt_limit = None;
                            state.failed_unlock_attempts = 0;
                            state.account_destroy_flow = AccountDestroyFlow::None;
                            state.account_destroy_clear_error();
                            state.apply_default_account_settings();
                            state.cmd_results.clear();
                            state.status_last_command_result = None;
                            state.command_feedback = None;
                            state.push_cmd_result("account destroy", true, "vault destroyed");
                            state.set_locked_state(true, "account_destroy");
                            passphrase.zeroize();
                            false
                        }
                        Err(code) => {
                            state.account_destroy_set_error(format!("destroy failed: {}", code));
                            state.push_cmd_result(
                                "account destroy",
                                false,
                                format!("destroy failed ({})", code),
                            );
                            state.cmd_input_clear();
                            passphrase.zeroize();
                            false
                        }
                    }
                }
            }
        }
        KeyCode::Backspace => {
            if state.home_focus == TuiHomeFocus::Command {
                state.cmd_input_pop();
                state.account_destroy_clear_error();
            }
            false
        }
        KeyCode::Tab => {
            state.home_focus_cycle(1);
            false
        }
        KeyCode::BackTab => {
            state.home_focus_cycle(-1);
            false
        }
        KeyCode::Up => {
            if state.home_focus == TuiHomeFocus::Main {
                state.main_scroll_move(-1);
            } else {
                state.nav_move(-1);
            }
            false
        }
        KeyCode::Down => {
            if state.home_focus == TuiHomeFocus::Main {
                state.main_scroll_move(1);
            } else {
                state.nav_move(1);
            }
            false
        }
        KeyCode::PageUp => {
            if state.home_focus == TuiHomeFocus::Main {
                state.main_scroll_page(-1);
            }
            false
        }
        KeyCode::PageDown => {
            if state.home_focus == TuiHomeFocus::Main {
                state.main_scroll_page(1);
            }
            false
        }
        KeyCode::Home => {
            if state.home_focus == TuiHomeFocus::Main {
                state.main_scroll_home();
            }
            false
        }
        KeyCode::End => {
            if state.home_focus == TuiHomeFocus::Main {
                state.main_scroll_end();
            }
            false
        }
        KeyCode::Char(ch) => {
            if state.home_focus == TuiHomeFocus::Command && no_ctrl_alt && !ch.is_control() {
                state.cmd_input_push(ch);
                state.account_destroy_clear_error();
            } else if ch == '/' {
                state.home_focus = TuiHomeFocus::Command;
                emit_marker("tui_focus_home", None, &[("pane", state.home_focus_name())]);
                state.cmd_input_push(ch);
                state.account_destroy_clear_error();
            }
            false
        }
        _ => false,
    }
}

fn tui_alias_is_valid(alias: &str) -> bool {
    let len = alias.chars().count();
    if !(2..=32).contains(&len) {
        return false;
    }
    alias
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' || ch == '.')
}

fn tui_verification_code_is_valid(code: &str) -> bool {
    const CROCKFORD: &str = "0123456789ABCDEFGHJKMNPQRSTVWXYZ";
    let upper = code.trim().to_ascii_uppercase();
    if upper.len() != 21 {
        return false;
    }
    for idx in [4usize, 9, 14, 19] {
        if upper.as_bytes().get(idx).copied() != Some(b'-') {
            return false;
        }
    }
    for (idx, ch) in upper.chars().enumerate() {
        if [4usize, 9, 14, 19].contains(&idx) {
            continue;
        }
        if !CROCKFORD.contains(ch) {
            return false;
        }
    }
    true
}

fn tui_passphrase_is_strong(passphrase: &str) -> bool {
    if passphrase.chars().count() < 16 {
        return false;
    }
    let lowered = passphrase.to_ascii_lowercase();
    if lowered.contains("password")
        || lowered.contains("qwerty")
        || lowered.contains("letmein")
        || lowered.contains("123456")
    {
        return false;
    }
    let all_same = passphrase
        .chars()
        .next()
        .map(|first| passphrase.chars().all(|ch| ch == first))
        .unwrap_or(true);
    !all_same
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

fn tui_try_vault_init(passphrase: &str) -> Result<(), String> {
    let exe = env::current_exe().map_err(|_| "spawn_failed".to_string())?;
    let mut child = Command::new(exe)
        .args([
            "vault",
            "init",
            "--non-interactive",
            "--passphrase-stdin",
            "--key-source",
            "passphrase",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|_| "spawn_failed".to_string())?;
    {
        let Some(mut stdin) = child.stdin.take() else {
            return Err("spawn_failed".to_string());
        };
        stdin
            .write_all(passphrase.as_bytes())
            .map_err(|_| "spawn_failed".to_string())?;
    }
    let out = child
        .wait_with_output()
        .map_err(|_| "spawn_failed".to_string())?;
    if out.status.success() {
        return Ok(());
    }
    let mut text = String::from_utf8_lossy(&out.stdout).to_string();
    text.push_str(&String::from_utf8_lossy(&out.stderr));
    for line in text.lines() {
        if let Some(code) = line.split("code=").nth(1) {
            return Err(code
                .split_whitespace()
                .next()
                .unwrap_or("vault_init_failed")
                .to_string());
        }
    }
    Err("vault_init_failed".to_string())
}

fn init_account_defaults_with_passphrase(passphrase: &str) -> Result<(), &'static str> {
    let autolock = TUI_AUTOLOCK_DEFAULT_MINUTES.to_string();
    let poll_interval = TUI_POLL_DEFAULT_INTERVAL_SECONDS.to_string();
    vault::secret_set_with_passphrase(TUI_AUTOLOCK_SECRET_KEY, autolock.as_str(), passphrase)?;
    vault::secret_set_with_passphrase(
        TUI_POLL_MODE_SECRET_KEY,
        TuiPollMode::Adaptive.as_str(),
        passphrase,
    )?;
    vault::secret_set_with_passphrase(
        TUI_POLL_INTERVAL_SECRET_KEY,
        poll_interval.as_str(),
        passphrase,
    )?;
    vault::secret_set_with_passphrase(
        TUI_RECEIPT_MODE_SECRET_KEY,
        ReceiptEmitMode::Off.as_str(),
        passphrase,
    )?;
    vault::secret_set_with_passphrase(
        TUI_RECEIPT_BATCH_WINDOW_MS_SECRET_KEY,
        RECEIPT_BATCH_WINDOW_MS_DEFAULT.to_string().as_str(),
        passphrase,
    )?;
    vault::secret_set_with_passphrase(
        TUI_RECEIPT_JITTER_MS_SECRET_KEY,
        RECEIPT_JITTER_MS_DEFAULT.to_string().as_str(),
        passphrase,
    )?;
    vault::secret_set_with_passphrase(
        TUI_FILE_CONFIRM_MODE_SECRET_KEY,
        FileConfirmEmitMode::CompleteOnly.as_str(),
        passphrase,
    )?;
    let mut seed = [0u8; 16];
    OsRng.fill_bytes(&mut seed);
    vault::secret_set_with_passphrase(
        ACCOUNT_VERIFICATION_SEED_SECRET_KEY,
        hex_encode(&seed).as_str(),
        passphrase,
    )?;
    vault::secret_set_with_passphrase(TUI_RELAY_ENDPOINT_SECRET_KEY, "", passphrase)?;
    vault::secret_set_with_passphrase(TUI_RELAY_TOKEN_SECRET_KEY, "", passphrase)?;
    let inbox_token = generate_route_token();
    vault::secret_set_with_passphrase(
        TUI_RELAY_INBOX_TOKEN_SECRET_KEY,
        inbox_token.as_str(),
        passphrase,
    )?;
    Ok(())
}

fn init_identity_with_passphrase(passphrase: &str) -> Result<(), ErrorCode> {
    let self_label = "self";
    if !channel_label_ok(self_label) {
        return Err(ErrorCode::ParseFailed);
    }
    let (dir, source) = config_dir()?;
    let identities = identities_dir(&dir);
    ensure_dir_secure(&identities, source)?;
    let path = identity_self_path(&dir, self_label);
    if path.exists() {
        enforce_safe_parents(&path, source)?;
        if identity_read_self_public(self_label)?.is_some() {
            return Ok(());
        }
    }
    let (kem_pk, kem_sk) = hs_kem_keypair();
    let (sig_pk, sig_sk) = hs_sig_keypair();
    vault::secret_set_with_passphrase(
        identity_secret_name(self_label).as_str(),
        hex_encode(&kem_sk).as_str(),
        passphrase,
    )
    .map_err(|_| ErrorCode::IoWriteFailed)?;
    vault::secret_set_with_passphrase(
        identity_sig_secret_name(self_label).as_str(),
        hex_encode(&sig_sk).as_str(),
        passphrase,
    )
    .map_err(|_| ErrorCode::IoWriteFailed)?;
    identity_write_public_record(self_label, &kem_pk, &sig_pk)?;
    Ok(())
}

fn initialize_account_after_init(alias: &str, passphrase: &str) -> Result<(), String> {
    if vault::secret_set_with_passphrase("profile_alias", alias, passphrase).is_err() {
        return Err("alias_store_failed".to_string());
    }
    init_account_defaults_with_passphrase(passphrase)
        .map_err(|_| "settings_init_failed".to_string())?;
    init_identity_with_passphrase(passphrase).map_err(|_| "identity_init_failed".to_string())?;
    Ok(())
}

pub(super) fn wipe_account_local_state_best_effort() {
    let Ok((dir, _)) = config_dir() else {
        return;
    };
    let identities = identities_dir(&dir);
    let sessions = dir.join(QSP_SESSIONS_DIR);
    let qsp_status = dir.join(QSP_STATUS_FILE_NAME);
    let send_state = dir.join(SEND_STATE_NAME);
    let outbox = dir.join(OUTBOX_FILE_NAME);
    let poll_legacy = dir.join("tui_polling.txt");
    let autolock_legacy = dir.join("tui_autolock.txt");
    let _ = fs::remove_dir_all(identities);
    let _ = fs::remove_dir_all(sessions);
    let _ = fs::remove_file(qsp_status);
    let _ = fs::remove_file(send_state);
    let _ = fs::remove_file(outbox);
    let _ = fs::remove_file(poll_legacy);
    let _ = fs::remove_file(autolock_legacy);
    fsync_dir_best_effort(&dir);
}

fn handle_locked_reject(state: &mut TuiState, cmd: &str, reason: &'static str) {
    state.set_command_error("locked: unlock required");
    emit_marker(
        "tui_locked_cmd_reject",
        Some("locked_unlock_required"),
        &[
            ("cmd", cmd),
            ("reason", reason),
            ("error", "locked_unlock_required"),
        ],
    );
    state
        .events
        .push_back("error: locked: unlock required".to_string());
}

fn handle_tui_locked_command(cmd: &TuiParsedCmd, state: &mut TuiState) -> Option<bool> {
    match cmd.cmd.as_str() {
        "exit" | "quit" => {
            emit_marker("tui_cmd", None, &[("cmd", "exit")]);
            Some(true)
        }
        "init" if !state.has_vault() => {
            emit_marker("tui_cmd", None, &[("cmd", "init")]);
            if cmd.args.is_empty() {
                state.start_init_prompt();
                return Some(false);
            }
            emit_marker(
                "tui_init_warning",
                None,
                &[("no_recovery", "true"), ("confirm_prompt", "Y_or_N")],
            );
            if cmd.args.len() < 4 {
                emit_marker(
                    "tui_init_reject",
                    Some("init_args_missing"),
                    &[("ok", "false"), ("required", "alias_pass_confirm_decision")],
                );
                state.start_init_prompt();
                return Some(false);
            }
            let alias = cmd.args[0].as_str();
            let passphrase = cmd.args[1].as_str();
            let confirm = cmd.args[2].as_str();
            let decision = cmd.args[3..].join(" ").to_ascii_uppercase();
            if !tui_alias_is_valid(alias) {
                emit_marker(
                    "tui_init_reject",
                    Some("alias_invalid"),
                    &[("ok", "false"), ("reason", "alias_invalid")],
                );
                state.start_init_prompt();
                return Some(false);
            }
            if !tui_passphrase_is_strong(passphrase) {
                emit_marker(
                    "tui_init_reject",
                    Some("passphrase_weak"),
                    &[("ok", "false"), ("reason", "passphrase_weak")],
                );
                state.start_init_prompt();
                return Some(false);
            }
            if passphrase != confirm {
                emit_marker(
                    "tui_init_reject",
                    Some("passphrase_mismatch"),
                    &[("ok", "false"), ("reason", "passphrase_mismatch")],
                );
                state.start_init_prompt();
                return Some(false);
            }
            if decision != "Y"
                && decision != "YES"
                && decision != "I AGREE"
                && decision != "I UNDERSTAND"
            {
                emit_marker(
                    "tui_init_reject",
                    Some("confirm_required"),
                    &[("ok", "false"), ("reason", "confirm_required")],
                );
                state.start_init_prompt();
                return Some(false);
            }
            match tui_try_vault_init(passphrase) {
                Ok(()) => {}
                Err(code) => {
                    emit_marker(
                        "tui_init_reject",
                        Some(code.as_str()),
                        &[("ok", "false"), ("reason", "vault_init_failed")],
                    );
                    state.start_init_prompt();
                    return Some(false);
                }
            }
            if let Err(code) = initialize_account_after_init(alias, passphrase) {
                emit_marker(
                    "tui_init_reject",
                    Some(code.as_str()),
                    &[("ok", "false"), ("reason", "account_init_failed")],
                );
                state.start_init_prompt();
                return Some(false);
            }
            let _ = vault_security_state_clear_files();
            state.unlock_attempt_limit = None;
            state.failed_unlock_attempts = 0;
            state.mark_vault_present();
            state.set_locked_state(true, "post_init_locked");
            state.locked_flow = LockedFlow::None;
            emit_marker(
                "tui_init",
                None,
                &[("ok", "true"), ("alias", "stored_local_only")],
            );
            Some(false)
        }
        "unlock" if state.has_vault() => {
            emit_marker("tui_cmd", None, &[("cmd", "unlock")]);
            if !state.is_locked() {
                emit_marker(
                    "tui_unlock",
                    None,
                    &[("ok", "true"), ("reason", "already_unlocked")],
                );
                return Some(false);
            }
            if cmd.args.is_empty() {
                state.start_unlock_prompt();
                return Some(false);
            }
            let mut passphrase = cmd.args.first().cloned().unwrap_or_default();
            match state.unlock_with_policy(passphrase.as_str()) {
                UnlockAttemptOutcome::Unlocked => {
                    state.set_locked_state(false, "explicit_command");
                    state.locked_clear_error();
                    emit_marker("tui_unlock", None, &[("ok", "true")]);
                }
                UnlockAttemptOutcome::Wiped => {
                    state.locked_set_error(
                        "vault wiped after failed unlock attempts; run /init to rebuild local state",
                    );
                    state.command_error = Some(format!(
                        "vault: {} (run /init to rebuild local state)",
                        QSC_ERR_VAULT_WIPED_AFTER_FAILED_UNLOCKS
                    ));
                }
                UnlockAttemptOutcome::Rejected => {
                    state
                        .locked_set_error("unlock failed: passphrase did not open the local vault");
                    state.command_error =
                        Some("unlock: passphrase did not open the local vault".to_string());
                    emit_marker(
                        "tui_unlock",
                        Some("vault_locked"),
                        &[("ok", "false"), ("reason", "passphrase_invalid")],
                    );
                }
            }
            passphrase.zeroize();
            Some(false)
        }
        _ => {
            handle_locked_reject(state, cmd.cmd.as_str(), "locked_unlock_required");
            Some(false)
        }
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
            match target {
                "events" => state.enter_focus_mode(TuiMode::FocusEvents),
                "files" => state.enter_focus_mode(TuiMode::FocusFiles),
                "activity" => state.enter_focus_mode(TuiMode::FocusActivity),
                "status" => state.enter_focus_mode(TuiMode::FocusStatus),
                "session" | "keys" => state.enter_focus_mode(TuiMode::FocusSession),
                "contacts" => state.enter_focus_mode(TuiMode::FocusContacts),
                "settings" => state.enter_focus_mode(TuiMode::FocusSettings),
                "lock" => state.enter_focus_mode(TuiMode::FocusLock),
                _ => {
                    state.set_command_error("focus: unknown pane");
                    emit_marker("tui_focus_invalid", None, &[("reason", "unknown_pane")]);
                }
            }
            false
        }
        "inspector" | "ins" => {
            emit_marker("tui_cmd", None, &[("cmd", "inspector")]);
            let target = cmd.args.first().map(|s| s.as_str()).unwrap_or("");
            match target {
                "events" => state.set_inspector(TuiInspectorPane::Events),
                "files" => state.set_inspector(TuiInspectorPane::Files),
                "activity" => state.set_inspector(TuiInspectorPane::Activity),
                "status" => state.set_inspector(TuiInspectorPane::Status),
                "overview" => state.set_inspector(TuiInspectorPane::Status),
                "account" => state.set_inspector(TuiInspectorPane::Account),
                "relay" | "server" => state.set_inspector(TuiInspectorPane::Relay),
                "cmdresults" | "results" => state.set_inspector(TuiInspectorPane::CmdResults),
                "session" | "keys" => state.set_inspector(TuiInspectorPane::Session),
                "contacts" => state.set_inspector(TuiInspectorPane::Contacts),
                "settings" => state.set_inspector(TuiInspectorPane::Settings),
                "lock" => state.set_inspector(TuiInspectorPane::Lock),
                "help" => state.set_inspector(TuiInspectorPane::Help),
                "about" => state.set_inspector(TuiInspectorPane::About),
                "legal" => state.set_inspector(TuiInspectorPane::Legal),
                _ => {
                    state.set_command_error("inspector: unknown pane");
                    emit_marker("tui_inspector_invalid", None, &[("reason", "unknown_pane")]);
                }
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

fn tui_msg_ensure_handshake(state: &mut TuiState, peer: &str) -> Result<(), &'static str> {
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

fn tui_msg_autotrust_first_use(state: &mut TuiState, peer: &str) -> Result<(), &'static str> {
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

fn tui_msg_recv_poll_bounded(state: &mut TuiState, peer: &str) {
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

fn tui_send_via_relay(state: &mut TuiState, to: &str) {
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

pub(super) fn tui_receive_via_relay(state: &mut TuiState, from: &str) {
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
