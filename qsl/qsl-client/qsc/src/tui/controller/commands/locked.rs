use super::*;

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
                let exit = super::handle_tui_command(&cmd, state);
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

pub(super) fn handle_tui_locked_key(state: &mut TuiState, key: KeyEvent) -> bool {
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

pub(super) fn handle_tui_account_destroy_key(state: &mut TuiState, key: KeyEvent) -> bool {
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

pub(super) fn tui_alias_is_valid(alias: &str) -> bool {
    let len = alias.chars().count();
    if !(2..=32).contains(&len) {
        return false;
    }
    alias
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' || ch == '.')
}

pub(super) fn tui_verification_code_is_valid(code: &str) -> bool {
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

pub(in super::super) fn wipe_account_local_state_best_effort() {
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

pub(super) fn handle_locked_reject(state: &mut TuiState, cmd: &str, reason: &'static str) {
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

pub(super) fn handle_tui_locked_command(cmd: &TuiParsedCmd, state: &mut TuiState) -> Option<bool> {
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
