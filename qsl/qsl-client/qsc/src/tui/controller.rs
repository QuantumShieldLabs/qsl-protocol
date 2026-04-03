use super::render::*;
use super::script::*;
use super::*;
use crate::*;

fn emit_tui_relay_test_event(result: &'static str, code: &'static str) {
    emit_tui_named_marker("QSC_TUI_RELAY_TEST", &[("result", result), ("code", code)]);
}

fn run_relay_test_probe(
    endpoint: &str,
    token: Option<String>,
    token_file: Option<&str>,
) -> RelayTestOutcome {
    let url = match relay_probe_url(endpoint) {
        Ok(v) => v,
        Err(code) => {
            return RelayTestOutcome {
                ok: false,
                code,
                message: code.to_string(),
            };
        }
    };
    let client = match HttpClient::builder()
        .timeout(Duration::from_secs(2))
        .build()
    {
        Ok(v) => v,
        Err(_) => {
            return RelayTestOutcome {
                ok: false,
                code: "relay_client_init_failed",
                message: "client init failed".to_string(),
            };
        }
    };
    let mut auth_token = token
        .as_ref()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty());
    if auth_token.is_none() {
        if let Some(path) = token_file {
            match read_relay_token_file(path) {
                Ok(v) => auth_token = Some(v),
                Err(code) => {
                    return RelayTestOutcome {
                        ok: false,
                        code,
                        message: relay_user_reason_from_code(code).to_string(),
                    };
                }
            }
        }
    }
    let mut req = client
        .get(url.as_str())
        .header("X-QSL-Route-Token", "qsc-relay-probe");
    if let Some(token) = auth_token.as_ref() {
        req = req.bearer_auth(token);
    }
    match req.send() {
        Ok(resp) => {
            let status = resp.status().as_u16();
            if status == 200 || status == 204 {
                RelayTestOutcome {
                    ok: true,
                    code: "relay_authenticated",
                    message: "authenticated".to_string(),
                }
            } else if status == 401 {
                RelayTestOutcome {
                    ok: false,
                    code: "relay_unauthorized",
                    message: "unauthorized (401)".to_string(),
                }
            } else if status == 429 {
                RelayTestOutcome {
                    ok: false,
                    code: "relay_overloaded",
                    message: "overloaded (429)".to_string(),
                }
            } else {
                RelayTestOutcome {
                    ok: false,
                    code: "relay_http_failure",
                    message: format!("http {}", status),
                }
            }
        }
        Err(err) => {
            let txt = err.to_string().to_ascii_lowercase();
            if txt.contains("dns")
                || txt.contains("name or service")
                || txt.contains("failed to lookup")
            {
                RelayTestOutcome {
                    ok: false,
                    code: "relay_dns_failure",
                    message: "dns failure".to_string(),
                }
            } else if txt.contains("timed out") {
                RelayTestOutcome {
                    ok: false,
                    code: "relay_network_timeout",
                    message: "network timeout".to_string(),
                }
            } else {
                RelayTestOutcome {
                    ok: false,
                    code: "relay_network_unreachable",
                    message: "network unreachable".to_string(),
                }
            }
        }
    }
}

pub(crate) fn tui_entry(headless: bool, cfg: TuiConfig) {
    let env_headless = env_bool("QSC_TUI_HEADLESS");
    let env_test_mode = env_bool("QSC_TUI_TEST_MODE");
    let headless = headless || env_headless;
    if headless {
        eprintln!("QSC_TUI_STARTUP OK mode=headless");
        tui_headless(cfg);
        return;
    }
    if env_test_mode {
        eprintln!("QSC_TUI_STARTUP OK mode=headless");
        tui_interactive_test(cfg);
        return;
    }
    if let Err(code) = tui_startup_preflight() {
        emit_tui_startup_fail(code);
        process::exit(2);
    }
    if let Err(code) = tui_interactive(cfg) {
        emit_tui_startup_fail(code);
        process::exit(2);
    }
}

fn tui_headless(cfg: TuiConfig) {
    set_marker_routing(MarkerRouting::Stdout);
    let mut state = TuiState::new(cfg);
    emit_marker("tui_open", None, &[]);
    state.emit_home_render_marker(terminal_cols_for_headless(), terminal_rows_for_headless());
    for line in load_tui_script() {
        state.poll_relay_test_task();
        if let Some(wait_ms) = parse_tui_wait_ms(&line) {
            state.headless_advance_clock(wait_ms);
            state.poll_relay_test_task();
            state.emit_home_render_marker(
                terminal_cols_for_headless(),
                terminal_rows_for_headless(),
            );
            continue;
        }
        if let Some(tag) = parse_tui_perf_snapshot(&line) {
            let (kdf, reads, decrypts, writes) = vault::perf_snapshot();
            let kdf_s = kdf.to_string();
            let reads_s = reads.to_string();
            let decrypts_s = decrypts.to_string();
            let writes_s = writes.to_string();
            emit_marker(
                "tui_perf",
                None,
                &[
                    ("tag", tag.as_str()),
                    ("kdf", kdf_s.as_str()),
                    ("reads", reads_s.as_str()),
                    ("decrypts", decrypts_s.as_str()),
                    ("writes", writes_s.as_str()),
                ],
            );
            continue;
        }
        if let Some(cmd) = parse_tui_command(&line) {
            if handle_tui_command(&cmd, &mut state) {
                state.wait_for_relay_test_task_headless();
                emit_marker("tui_exit", None, &[]);
                return;
            }
            state.wait_for_relay_test_task_headless();
            state.poll_relay_test_task();
            state.emit_home_render_marker(
                terminal_cols_for_headless(),
                terminal_rows_for_headless(),
            );
        } else {
            state.mark_input_activity(state.headless_now_ms());
            emit_marker("tui_input_text", None, &[("kind", "plain")]);
        }
    }
    emit_marker("tui_exit", None, &[]);
}

fn terminal_cols_for_headless() -> u16 {
    env::var("QSC_TUI_COLS")
        .ok()
        .and_then(|v| v.parse::<u16>().ok())
        .filter(|v| *v > 0)
        .unwrap_or(140)
}

fn terminal_rows_for_headless() -> u16 {
    env::var("QSC_TUI_ROWS")
        .ok()
        .and_then(|v| v.parse::<u16>().ok())
        .filter(|v| *v > 0)
        .unwrap_or(40)
}

pub(crate) fn tui_next_poll_timeout_ms() -> u64 {
    const TUI_POLL_MS_DEFAULT: u64 = 200;
    const TUI_POLL_MS_MIN: u64 = 50;
    TUI_POLL_MS_DEFAULT.max(TUI_POLL_MS_MIN)
}

fn tui_deterministic_timestamps() -> bool {
    env_bool("QSC_TUI_DETERMINISTIC") || env_bool("QSC_TUI_HEADLESS")
}

fn tui_timestamp_token(idx: usize) -> String {
    format!("t={:04}", idx.saturating_add(1))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TuiStartupCode {
    StdinNotTty,
    StdoutNotTty,
    TermInvalid,
    StdinClosed,
    RawModeFailed,
    AltScreenFailed,
    EventStreamFailed,
    Unknown,
}

impl TuiStartupCode {
    fn as_str(self) -> &'static str {
        match self {
            Self::StdinNotTty => "stdin_not_tty",
            Self::StdoutNotTty => "stdout_not_tty",
            Self::TermInvalid => "term_invalid",
            Self::StdinClosed => "stdin_closed",
            Self::RawModeFailed => "raw_mode_failed",
            Self::AltScreenFailed => "alt_screen_failed",
            Self::EventStreamFailed => "event_stream_failed",
            Self::Unknown => "unknown",
        }
    }
}

fn emit_tui_startup_fail(code: TuiStartupCode) {
    eprintln!("QSC_TUI_STARTUP FAIL code={}", code.as_str());
    eprintln!(
        "HINT: run in an interactive terminal (stdin+stdout must be a TTY). If running non-interactively, set QSC_TUI_HEADLESS=1."
    );
}

fn tui_startup_preflight() -> Result<(), TuiStartupCode> {
    if !std::io::stdin().is_terminal() {
        return Err(TuiStartupCode::StdinNotTty);
    }
    if !std::io::stdout().is_terminal() {
        return Err(TuiStartupCode::StdoutNotTty);
    }
    let term = env::var("TERM").unwrap_or_default();
    if term.trim().is_empty() || term.eq_ignore_ascii_case("dumb") {
        return Err(TuiStartupCode::TermInvalid);
    }
    match event::poll(Duration::from_millis(0)) {
        Ok(_) => Ok(()),
        Err(_) => Err(TuiStartupCode::StdinClosed),
    }
}

fn event_error_code(err: &std::io::Error) -> TuiStartupCode {
    match err.kind() {
        std::io::ErrorKind::UnexpectedEof
        | std::io::ErrorKind::BrokenPipe
        | std::io::ErrorKind::ConnectionAborted
        | std::io::ErrorKind::NotConnected => TuiStartupCode::StdinClosed,
        _ => TuiStartupCode::EventStreamFailed,
    }
}

fn tui_interactive(cfg: TuiConfig) -> Result<(), TuiStartupCode> {
    set_marker_routing(MarkerRouting::InApp);
    let mut state = TuiState::new(cfg);
    emit_marker("tui_open", None, &[]);
    enable_raw_mode().map_err(|_| TuiStartupCode::RawModeFailed)?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen).map_err(|_| TuiStartupCode::AltScreenFailed)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).map_err(|_| TuiStartupCode::Unknown)?;
    let started = Instant::now();

    let mut exit = false;
    let mut last_draw_ms = 0u64;
    let result = loop {
        let now_ms = started.elapsed().as_millis() as u64;
        state.drain_marker_queue();
        state.poll_relay_test_task();
        let force_full_redraw = state.take_force_full_redraw() || state.take_clear_screen_pending();
        if force_full_redraw
            || state.needs_redraw
            || now_ms == 0
            || now_ms.saturating_sub(last_draw_ms) >= 1_000
        {
            if terminal
                .draw(|f| {
                    if force_full_redraw {
                        let area = f.area();
                        f.render_widget(TuiClear, area);
                    }
                    draw_tui(f, &mut state);
                })
                .is_err()
            {
                break Err(TuiStartupCode::EventStreamFailed);
            }
            state.needs_redraw = false;
            last_draw_ms = now_ms;
        }

        let polled = match event::poll(Duration::from_millis(tui_next_poll_timeout_ms())) {
            Ok(polled) => polled,
            Err(err) => break Err(event_error_code(&err)),
        };
        if polled {
            let event = match event::read() {
                Ok(event) => event,
                Err(err) => break Err(event_error_code(&err)),
            };
            if let Event::Key(key) = event {
                state.mark_input_activity(now_ms);
                exit = handle_tui_key(&mut state, key);
                state.request_redraw();
            }
        } else if state.maybe_autolock(now_ms) {
            state.request_redraw();
        }
        if state.maybe_run_fixed_poll(now_ms) {
            state.request_redraw();
        }
        if exit {
            break Ok(());
        }
    };

    let _ = disable_raw_mode();
    let _ = execute!(terminal.backend_mut(), LeaveAlternateScreen);
    let _ = terminal.show_cursor();
    if result.is_ok() {
        emit_marker("tui_exit", None, &[]);
    } else {
        emit_marker("tui_exit", Some("io"), &[]);
    }
    result
}

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

fn handle_tui_key(state: &mut TuiState, key: KeyEvent) -> bool {
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

fn tui_interactive_test(cfg: TuiConfig) {
    set_marker_routing(MarkerRouting::InApp);
    let mut state = TuiState::new(cfg);
    emit_marker("tui_open", None, &[]);
    state.drain_marker_queue();
    println!("tui_test_done");
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

fn format_message_transcript_line(
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

fn wipe_account_local_state_best_effort() {
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

fn handle_tui_command(cmd: &TuiParsedCmd, state: &mut TuiState) -> bool {
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

fn tui_receive_via_relay(state: &mut TuiState, from: &str) {
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

fn draw_tui(f: &mut ratatui::Frame, state: &mut TuiState) {
    let area = f.area();
    match state.mode {
        TuiMode::Help => {
            draw_help_mode(f, area, state);
            return;
        }
        TuiMode::FocusEvents => {
            draw_focus_events(f, area, state);
            return;
        }
        TuiMode::FocusFiles => {
            draw_focus_files(f, area, state);
            return;
        }
        TuiMode::FocusActivity => {
            draw_focus_activity(f, area, state);
            return;
        }
        TuiMode::FocusStatus => {
            draw_focus_status(f, area, state);
            return;
        }
        TuiMode::FocusSession => {
            draw_focus_session(f, area, state);
            return;
        }
        TuiMode::FocusContacts => {
            draw_focus_contacts(f, area, state);
            return;
        }
        TuiMode::FocusSettings => {
            draw_focus_settings(f, area, state);
            return;
        }
        TuiMode::FocusLock => {
            draw_focus_lock(f, area, state);
            return;
        }
        TuiMode::Normal => {}
    }
    let outer = Block::default().borders(Borders::ALL);
    f.render_widget(outer, area);
    let inner = area.inner(ratatui::layout::Margin {
        vertical: 1,
        horizontal: 1,
    });
    if inner.width == 0 || inner.height == 0 {
        return;
    }

    // Fallback for tiny terminals: render command line only.
    if inner.width < 3 || inner.height < 3 {
        let cmd_text = pad_panel_text(state.cmd_bar_text().as_str());
        let cmd = Paragraph::new(Line::from(vec![Span::styled(
            cmd_text.as_str(),
            state.cmd_bar_style(cmd_text.as_str()),
        )]));
        f.render_widget(cmd, inner);
        return;
    }

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(inner);
    let content_area = rows[0];
    let h_divider_area = rows[1];
    let cmd_area = rows[2];

    let nav_width = ((u32::from(content_area.width) * 26) / 100) as u16;
    let nav_width = nav_width.clamp(1, content_area.width.saturating_sub(2));
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length(nav_width),
                Constraint::Length(1),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(content_area);
    render_unified_nav(f, cols[0], state);
    if content_area.height >= 2 {
        let header_divider_area = Rect {
            x: content_area.x,
            y: content_area.y + 1,
            width: content_area.width,
            height: 1,
        };
        render_header_divider(f, header_divider_area);
    }
    let body_main_area = if cols[2].height > 2 {
        Rect {
            x: cols[2].x,
            y: cols[2].y + 2,
            width: cols[2].width,
            height: cols[2].height - 2,
        }
    } else {
        Rect {
            x: cols[2].x,
            y: cols[2].y + cols[2].height,
            width: cols[2].width,
            height: 0,
        }
    };
    let body_v_divider_area = if cols[1].height > 2 {
        Rect {
            x: cols[1].x,
            y: cols[1].y + 2,
            width: cols[1].width,
            height: cols[1].height - 2,
        }
    } else {
        Rect {
            x: cols[1].x,
            y: cols[1].y + cols[1].height,
            width: cols[1].width,
            height: 0,
        }
    };
    render_vertical_divider(f, body_v_divider_area);
    render_main_panel(f, body_main_area, state);
    render_horizontal_divider(f, h_divider_area);

    let cmd_text = pad_panel_text(state.cmd_bar_text().as_str());
    let cmd_text_marker = cmd_text.replace(' ', "_");
    let cmd = Paragraph::new(Line::from(vec![Span::styled(
        cmd_text.as_str(),
        state.cmd_bar_style(cmd_text.as_str()),
    )]));
    f.render_widget(cmd, cmd_area);
    emit_marker(
        "tui_cmd_render",
        None,
        &[
            ("pad", "2"),
            ("text", cmd_text_marker.as_str()),
            ("focus", state.home_focus_name()),
        ],
    );
}

fn draw_help_mode(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(area);

    let items = tui_help_items();
    let list_items: Vec<ListItem> = items
        .iter()
        .map(|item| ListItem::new(format!("/{} — {}", item.cmd, item.desc)))
        .collect();
    let mut list_state = ratatui::widgets::ListState::default();
    list_state.select(Some(state.help_selected.min(items.len().saturating_sub(1))));

    let list = List::new(list_items)
        .block(Block::default().borders(Borders::ALL).title("Help"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
    f.render_stateful_widget(list, cols[0], &mut list_state);

    let detail = state.help_selected_item();
    let detail_body = match detail {
        Some(item) => format!("command: /{}\n\n{}", item.cmd, item.desc),
        None => "no help items".to_string(),
    };
    let details =
        Paragraph::new(detail_body).block(Block::default().borders(Borders::ALL).title("Details"));
    f.render_widget(details, cols[1]);
}

fn draw_focus_events(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    let body = state.focus_events_lines().join("\n");
    let panel = Paragraph::new(body)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("FOCUS: EVENTS (Up/Down PgUp/PgDn Esc back)"),
        )
        .scroll((state.focus_scroll_index() as u16, 0));
    f.render_widget(panel, area);
}

fn draw_focus_files(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    let body = state.focus_files_lines().join("\n");
    let panel = Paragraph::new(body)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("FOCUS: FILES (Up/Down PgUp/PgDn Esc back)"),
        )
        .scroll((state.focus_scroll_index() as u16, 0));
    f.render_widget(panel, area);
}

fn draw_focus_activity(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    let body = state.focus_activity_lines().join("\n");
    let panel = Paragraph::new(body)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("FOCUS: ACTIVITY (Up/Down PgUp/PgDn Esc back)"),
        )
        .scroll((state.focus_scroll_index() as u16, 0));
    f.render_widget(panel, area);
}

fn draw_focus_status(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    let body = state.focus_status_lines().join("\n");
    let panel = Paragraph::new(body)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("FOCUS: STATUS (Up/Down PgUp/PgDn Esc back)"),
        )
        .scroll((state.focus_scroll_index() as u16, 0));
    f.render_widget(panel, area);
}

fn draw_focus_session(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    let body = state.focus_session_lines().join("\n");
    let panel = Paragraph::new(body)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("FOCUS: SESSION (Up/Down PgUp/PgDn Esc back)"),
        )
        .scroll((state.focus_scroll_index() as u16, 0));
    f.render_widget(panel, area);
}

fn draw_focus_contacts(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    let body = state.focus_contacts_lines().join("\n");
    let panel = Paragraph::new(body)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("FOCUS: CONTACTS (Up/Down PgUp/PgDn Esc back)"),
        )
        .scroll((state.focus_scroll_index() as u16, 0));
    f.render_widget(panel, area);
}

fn draw_focus_settings(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    let body = state.focus_settings_lines().join("\n");
    let panel = Paragraph::new(body)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("FOCUS: SETTINGS (Up/Down PgUp/PgDn Esc back)"),
        )
        .scroll((state.focus_scroll_index() as u16, 0));
    f.render_widget(panel, area);
}

fn draw_focus_lock(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    let body = state.focus_lock_lines().join("\n");
    let panel = Paragraph::new(body)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("FOCUS: LOCK (Up/Down PgUp/PgDn Esc back)"),
        )
        .scroll((state.focus_scroll_index() as u16, 0));
    f.render_widget(panel, area);
}

fn render_unified_nav(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    let rows = state.nav_rows();
    let selected_idx = state.nav_selected.min(rows.len().saturating_sub(1));
    let show_nav_marker = state.home_focus == TuiHomeFocus::Nav;
    let base_pad = " ".repeat(PANEL_INNER_PAD);
    let child_pad = " ".repeat(PANEL_INNER_PAD + NAV_CHILD_INDENT);
    let mut lines = Vec::new();
    for (idx, row) in rows.iter().enumerate() {
        let prefix = if show_nav_marker && idx == selected_idx {
            ">"
        } else {
            " "
        };
        match row.kind {
            NavRowKind::Domain(domain) => {
                let title = match domain {
                    TuiNavDomain::System => "System",
                    TuiNavDomain::Contacts => "Contacts",
                    TuiNavDomain::Messages => "Messages",
                };
                lines.push(format!("{}{}{}", prefix, base_pad, title));
            }
            NavRowKind::SystemAccount => lines.push(format!("{}{}Account", prefix, child_pad)),
            NavRowKind::SystemRelay => lines.push(format!("{}{}Relay", prefix, child_pad)),
            NavRowKind::SystemSettings => lines.push(format!("{}{}Settings", prefix, child_pad)),
            NavRowKind::SystemCmdResults => lines.push(format!("{}{}Results", prefix, child_pad)),
            NavRowKind::Header(pane) => {
                let header = match pane {
                    TuiInspectorPane::Events => format!("{}{}Messages", prefix, base_pad),
                    TuiInspectorPane::Files => format!("{}{}Files", prefix, base_pad),
                    TuiInspectorPane::Activity => format!("{}{}Activity", prefix, base_pad),
                    TuiInspectorPane::Status => format!("{}{}Status", prefix, base_pad),
                    TuiInspectorPane::Account => format!("{}{}Account", prefix, base_pad),
                    TuiInspectorPane::Relay => format!("{}{}Relay", prefix, base_pad),
                    TuiInspectorPane::CmdResults => format!("{}{}Results", prefix, base_pad),
                    TuiInspectorPane::Session => format!("{}{}Keys", prefix, base_pad),
                    TuiInspectorPane::Contacts => format!("{}{}Contacts", prefix, base_pad),
                    TuiInspectorPane::Settings => format!("{}{}Settings", prefix, base_pad),
                    TuiInspectorPane::Lock => format!("{}{}Lock", prefix, base_pad),
                    TuiInspectorPane::Help => format!("{}{}Help", prefix, base_pad),
                    TuiInspectorPane::About => format!("{}{}About", prefix, base_pad),
                    TuiInspectorPane::Legal => format!("{}{}Legal", prefix, base_pad),
                };
                lines.push(header);
            }
            NavRowKind::Conversation(item_idx) => {
                let labels = state.conversation_labels();
                if let Some(peer) = labels.get(item_idx) {
                    lines.push(format!("{}{}{}", prefix, child_pad, peer));
                }
            }
            NavRowKind::Contact(item_idx) => {
                if let Some(peer) = state.contacts.get(item_idx) {
                    lines.push(format!("{}{}{}", prefix, child_pad, peer));
                }
            }
            NavRowKind::Unlock => lines.push(format!("{}{}Unlock", prefix, base_pad)),
            NavRowKind::Exit => lines.push(format!("{}{}Exit", prefix, base_pad)),
        }
    }
    let selected_markers = if rows.is_empty() || !show_nav_marker {
        0
    } else {
        1
    };
    let selected_idx_s = selected_idx.to_string();
    let selected_label = rows
        .get(selected_idx)
        .map(|row| state.nav_row_label(row))
        .unwrap_or_else(|| "none".to_string());
    let header_text = "[ QSC ]";
    let header_left_padding = 1usize;
    let header_left_padding_s = header_left_padding.to_string();
    emit_marker(
        "tui_nav_render",
        None,
        &[
            (
                "selected_markers",
                if selected_markers == 1 { "1" } else { "0" },
            ),
            ("selected_index", selected_idx_s.as_str()),
            ("selected_label", selected_label.as_str()),
            ("header", header_text),
            ("header_left_padding", header_left_padding_s.as_str()),
            ("counters", "none"),
        ],
    );
    lines.insert(
        0,
        format!("{}{}", " ".repeat(header_left_padding), header_text),
    );
    lines.insert(1, String::new());
    let panel = Paragraph::new(lines.join("\n"));
    f.render_widget(panel, area);
}

fn tui_vault_present() -> bool {
    config_dir()
        .ok()
        .map(|(dir, _)| dir.join("vault.qsv").exists())
        .unwrap_or(false)
}

fn tui_relay_config(cfg: &TuiConfig) -> Option<TuiRelayConfig> {
    let relay = cfg.relay.clone()?;
    if normalize_relay_endpoint(relay.as_str()).is_err() {
        return None;
    }
    Some(TuiRelayConfig {
        relay,
        seed: cfg.seed,
        scenario: cfg.scenario.clone(),
    })
}

pub(crate) struct TuiState {
    contacts: Vec<String>,
    conversations: BTreeMap<String, VecDeque<String>>,
    unread_counts: BTreeMap<String, usize>,
    visible_counts: BTreeMap<String, usize>,
    files: Vec<TuiFileItem>,
    file_selected: usize,
    file_multi_selected: BTreeSet<String>,
    file_unseen_updates: usize,
    activity_visible_count: usize,
    activity_unseen_updates: usize,
    events: VecDeque<String>,
    pub(crate) status: TuiStatus<'static>,
    session: TuiSession<'static>,
    send_lifecycle: String,
    qsp_status: String,
    envelope: String,
    last_payload_len: usize,
    event_seq: u64,
    relay: Option<TuiRelayConfig>,
    send_seq: u64,
    mode: TuiMode,
    help_selected: usize,
    focus_scroll: usize,
    contacts_selected: usize,
    conversation_selected: usize,
    inspector: TuiInspectorPane,
    home_focus: TuiHomeFocus,
    nav_selected: usize,
    pub(crate) vault_locked: bool,
    vault_present: bool,
    vault_session: Option<vault::VaultSession>,
    autolock_timeout_ms: u64,
    autolock_last_activity_ms: u64,
    pub(crate) poll_mode: TuiPollMode,
    pub(crate) poll_interval_seconds: u64,
    receipt_policy: ReceiptPolicy,
    trust_onboarding_mode: TrustOnboardingMode,
    pub(crate) poll_next_due_ms: Option<u64>,
    headless_clock_ms: u64,
    clear_screen_pending: bool,
    force_full_redraw: bool,
    cmd_input: String,
    locked_flow: LockedFlow,
    locked_error: Option<String>,
    account_destroy_flow: AccountDestroyFlow,
    account_destroy_error: Option<String>,
    command_error: Option<String>,
    command_feedback: Option<String>,
    status_last_command_result: Option<String>,
    cmd_results: VecDeque<String>,
    active_command_label: Option<String>,
    active_command_result_recorded: bool,
    contacts_records: BTreeMap<String, ContactRecord>,
    account_alias_cache: String,
    account_verification_code_cache: String,
    account_storage_safety_cache: String,
    account_cache_last_refresh_ms: u64,
    relay_endpoint_cache: Option<String>,
    relay_endpoint_hash_cache: Option<String>,
    relay_token_set_cache: bool,
    relay_token_file_cache: Option<String>,
    relay_token_file_hash_cache: Option<String>,
    relay_inbox_token_hash_cache: Option<String>,
    relay_inbox_token_set_cache: bool,
    relay_last_test_result: String,
    relay_test_task: Option<mpsc::Receiver<RelayTestOutcome>>,
    unlock_attempt_limit: Option<u32>,
    failed_unlock_attempts: u32,
    main_scroll_offsets: BTreeMap<&'static str, usize>,
    main_scroll_max_current: usize,
    main_view_rows_current: usize,
    pub(crate) needs_redraw: bool,
}

impl TuiState {
    pub(crate) fn new(cfg: TuiConfig) -> Self {
        let vault_present = tui_vault_present();
        let contacts = vec!["peer-0".to_string()];
        let conversations = BTreeMap::new();
        let unread_counts = BTreeMap::new();
        let visible_counts = BTreeMap::new();
        let mut events = VecDeque::new();
        let fingerprint = compute_local_fingerprint();
        let peer_fp = compute_peer_fingerprint("peer-0");
        let qsp_status = qsp_status_string("peer-0");
        let envelope = compute_envelope_status(0);
        let send_lifecycle = "idle".to_string();
        let locked_s = if vault_unlocked() {
            "UNLOCKED"
        } else {
            "LOCKED"
        };
        let status = TuiStatus {
            fingerprint: Box::leak(fingerprint.clone().into_boxed_str()),
            peer_fp: Box::leak(peer_fp.clone().into_boxed_str()),
            qsp: Box::leak(qsp_status.clone().into_boxed_str()),
            envelope: Box::leak(envelope.clone().into_boxed_str()),
            send_lifecycle: Box::leak(send_lifecycle.clone().into_boxed_str()),
            locked: Box::leak(locked_s.to_string().into_boxed_str()),
        };
        let session = TuiSession {
            peer_label: "peer-0",
            verified: false,
            sent_count: 0,
            recv_count: 0,
        };
        let relay = tui_relay_config(&cfg);
        if let Some(r) = relay.as_ref() {
            let seed_s = r.seed.to_string();
            emit_marker(
                "tui_transport",
                None,
                &[
                    ("transport", "relay"),
                    ("relay", r.relay.as_str()),
                    ("seed", seed_s.as_str()),
                    ("scenario", r.scenario.as_str()),
                ],
            );
            events.push_back(format!(
                "transport relay {} seed={} scenario={}",
                r.relay, r.seed, r.scenario
            ));
        }
        let mut state = Self {
            contacts,
            conversations,
            unread_counts,
            visible_counts,
            files: load_tui_files_snapshot(),
            file_selected: 0,
            file_multi_selected: BTreeSet::new(),
            file_unseen_updates: 0,
            activity_visible_count: 0,
            activity_unseen_updates: 0,
            events,
            status,
            session,
            send_lifecycle,
            qsp_status,
            envelope,
            last_payload_len: 0,
            event_seq: 0,
            relay,
            send_seq: 0,
            mode: TuiMode::Normal,
            help_selected: 0,
            focus_scroll: 0,
            contacts_selected: 0,
            conversation_selected: 0,
            inspector: TuiInspectorPane::Status,
            home_focus: TuiHomeFocus::Nav,
            nav_selected: 0,
            vault_locked: !vault_unlocked(),
            vault_present,
            vault_session: None,
            autolock_timeout_ms: TUI_AUTOLOCK_DEFAULT_MINUTES.saturating_mul(60_000),
            autolock_last_activity_ms: 0,
            poll_mode: TuiPollMode::Adaptive,
            poll_interval_seconds: TUI_POLL_DEFAULT_INTERVAL_SECONDS,
            receipt_policy: ReceiptPolicy::default(),
            trust_onboarding_mode: TrustOnboardingMode::Balanced,
            poll_next_due_ms: None,
            headless_clock_ms: 0,
            clear_screen_pending: false,
            force_full_redraw: false,
            cmd_input: String::new(),
            locked_flow: LockedFlow::None,
            locked_error: None,
            account_destroy_flow: AccountDestroyFlow::None,
            account_destroy_error: None,
            command_error: None,
            command_feedback: None,
            status_last_command_result: None,
            cmd_results: VecDeque::new(),
            active_command_label: None,
            active_command_result_recorded: false,
            contacts_records: BTreeMap::new(),
            account_alias_cache: "unset".to_string(),
            account_verification_code_cache: "none".to_string(),
            account_storage_safety_cache: "unknown".to_string(),
            account_cache_last_refresh_ms: 0,
            relay_endpoint_cache: None,
            relay_endpoint_hash_cache: None,
            relay_token_set_cache: false,
            relay_token_file_cache: cfg.token_file.as_ref().map(|p| p.display().to_string()),
            relay_token_file_hash_cache: cfg
                .token_file
                .as_ref()
                .map(|p| relay_token_file_hash8(p.display().to_string().as_str())),
            relay_inbox_token_hash_cache: None,
            relay_inbox_token_set_cache: false,
            relay_last_test_result: "none".to_string(),
            relay_test_task: None,
            unlock_attempt_limit: None,
            failed_unlock_attempts: 0,
            main_scroll_offsets: BTreeMap::new(),
            main_scroll_max_current: 0,
            main_view_rows_current: 1,
            needs_redraw: true,
        };
        if env_bool("QSC_TUI_TEST_UNLOCK") {
            state.vault_locked = false;
            state.vault_present = true;
            state.status.locked = "UNLOCKED";
            set_vault_unlocked(true);
        }
        state.reload_unlock_security_state();
        if state.vault_locked {
            state.inspector = TuiInspectorPane::Lock;
            state.nav_selected = 0;
        } else {
            let _ = state.open_vault_session(None);
            state.reload_account_settings_from_vault();
            state.refresh_identity_status();
            state.sync_nav_to_inspector_header();
        }
        if let Some(path) = cfg.token_file.as_ref() {
            let canonical = path
                .canonicalize()
                .unwrap_or(path.to_path_buf())
                .to_string_lossy()
                .to_string();
            state.relay_token_file_hash_cache = Some(relay_token_file_hash8(canonical.as_str()));
            state.relay_token_file_cache = Some(canonical.clone());
            if !state.is_locked() {
                let _ = state
                    .persist_account_secret(TUI_RELAY_TOKEN_FILE_SECRET_KEY, canonical.as_str());
            }
        }
        state.emit_setup_required_marker_if_needed();
        state
    }

    fn is_locked(&self) -> bool {
        self.vault_locked
    }

    fn has_vault(&self) -> bool {
        self.vault_present
    }

    fn mark_vault_present(&mut self) {
        self.vault_present = true;
    }

    fn mark_vault_absent(&mut self) {
        self.vault_present = false;
        vault::set_process_passphrase(None);
    }

    fn reload_unlock_security_state(&mut self) {
        match vault_security_state_load() {
            Ok(state) => {
                self.unlock_attempt_limit = state.attempt_limit;
                self.failed_unlock_attempts = state.failed_unlocks;
            }
            Err(_) => {
                self.unlock_attempt_limit = None;
                self.failed_unlock_attempts = 0;
            }
        }
    }

    fn persist_unlock_security_state(&self) -> Result<(), &'static str> {
        let state = VaultSecurityState {
            attempt_limit: self.unlock_attempt_limit,
            failed_unlocks: self.failed_unlock_attempts,
        };
        vault_security_state_store(&state).map_err(|_| "vault_attempt_limit_io")
    }

    fn set_unlock_attempt_limit(&mut self, limit: Option<u32>) -> Result<(), &'static str> {
        if let Some(value) = limit {
            if !(VAULT_ATTEMPT_LIMIT_MIN..=VAULT_ATTEMPT_LIMIT_MAX).contains(&value) {
                return Err("vault_attempt_limit_invalid");
            }
        }
        self.unlock_attempt_limit = limit;
        self.failed_unlock_attempts = 0;
        self.persist_unlock_security_state()?;
        self.request_redraw();
        Ok(())
    }

    fn reset_unlock_failure_counter(&mut self) {
        if self.failed_unlock_attempts == 0 {
            return;
        }
        self.failed_unlock_attempts = 0;
        let _ = self.persist_unlock_security_state();
    }

    fn wipe_after_failed_unlock_limit(&mut self) -> Result<(), &'static str> {
        wipe_vault_file_best_effort().map_err(|_| "vault_wipe_failed")?;
        let _ = vault_security_state_clear_files();
        wipe_account_local_state_best_effort();
        self.close_vault_session();
        self.mark_vault_absent();
        self.unlock_attempt_limit = None;
        self.failed_unlock_attempts = 0;
        self.apply_default_account_settings();
        self.cmd_results.clear();
        self.status_last_command_result = None;
        self.command_feedback = None;
        self.locked_flow = LockedFlow::None;
        self.locked_clear_error();
        self.set_locked_state(true, "unlock_attempt_limit_wipe");
        self.push_cmd_result(
            "unlock",
            false,
            format!(
                "vault wiped after failed unlock attempts; run /init to rebuild local state ({})",
                QSC_ERR_VAULT_WIPED_AFTER_FAILED_UNLOCKS
            ),
        );
        emit_marker(
            "tui_unlock",
            Some(QSC_ERR_VAULT_WIPED_AFTER_FAILED_UNLOCKS),
            &[("ok", "false"), ("reason", "failed_unlock_limit_reached")],
        );
        Ok(())
    }

    fn record_unlock_failure_and_maybe_wipe(&mut self) -> UnlockAttemptOutcome {
        let Some(limit) = self.unlock_attempt_limit else {
            return UnlockAttemptOutcome::Rejected;
        };
        self.failed_unlock_attempts = self.failed_unlock_attempts.saturating_add(1);
        if self.persist_unlock_security_state().is_err() {
            return UnlockAttemptOutcome::Rejected;
        }
        if self.failed_unlock_attempts >= limit && self.wipe_after_failed_unlock_limit().is_ok() {
            return UnlockAttemptOutcome::Wiped;
        }
        UnlockAttemptOutcome::Rejected
    }

    fn unlock_with_policy(&mut self, passphrase: &str) -> UnlockAttemptOutcome {
        let unlocked = vault::unlock_with_passphrase(passphrase).is_ok()
            && self.open_vault_session(Some(passphrase)).is_ok();
        if unlocked {
            vault::set_process_passphrase(Some(passphrase));
            self.reset_unlock_failure_counter();
            return UnlockAttemptOutcome::Unlocked;
        }
        self.record_unlock_failure_and_maybe_wipe()
    }

    fn cmd_input_clear(&mut self) {
        self.cmd_input.clear();
    }

    fn cmd_input_push(&mut self, ch: char) {
        self.cmd_input.push(ch);
    }

    fn cmd_input_pop(&mut self) {
        self.cmd_input.pop();
    }

    fn locked_flow_name(&self) -> &'static str {
        match self.locked_flow {
            LockedFlow::None => "none",
            LockedFlow::UnlockPassphrase => "unlock_passphrase",
            LockedFlow::InitAlias => "init_alias",
            LockedFlow::InitPassphrase { .. } => "init_passphrase",
            LockedFlow::InitConfirm { .. } => "init_confirm",
            LockedFlow::InitDecision { .. } => "init_decision",
        }
    }

    fn locked_wizard_step_label(&self) -> Option<&'static str> {
        match self.locked_flow {
            LockedFlow::None => None,
            LockedFlow::UnlockPassphrase => Some("Passphrase"),
            LockedFlow::InitAlias => Some("Alias"),
            LockedFlow::InitPassphrase { .. } => Some("Passphrase"),
            LockedFlow::InitConfirm { .. } => Some("Confirm"),
            LockedFlow::InitDecision { .. } => Some("Confirm (I AGREE/N)"),
        }
    }

    fn account_destroy_step_label(&self) -> Option<&'static str> {
        match self.account_destroy_flow {
            AccountDestroyFlow::None => None,
            AccountDestroyFlow::Passphrase => Some("Passphrase"),
            AccountDestroyFlow::ConfirmDecision { .. } => Some("Confirm (Y/N)"),
        }
    }

    fn account_destroy_set_error(&mut self, message: impl Into<String>) {
        self.account_destroy_error = Some(message.into());
    }

    fn account_destroy_clear_error(&mut self) {
        self.account_destroy_error = None;
    }

    fn account_destroy_active(&self) -> bool {
        !matches!(self.account_destroy_flow, AccountDestroyFlow::None)
    }

    fn locked_set_error(&mut self, message: impl Into<String>) {
        self.locked_error = Some(message.into());
    }

    fn locked_clear_error(&mut self) {
        self.locked_error = None;
    }

    fn set_command_error(&mut self, message: impl Into<String>) {
        let message = message.into();
        self.command_error = Some(message.clone());
        self.command_feedback = None;
        if !self.active_command_result_recorded {
            if let Some(command) = self.active_command_label.clone() {
                self.push_cmd_result(command.as_str(), false, message);
            }
        }
        self.route_show_to_system_nav(TuiInspectorPane::CmdResults);
        self.request_redraw();
    }

    fn clear_command_error(&mut self) {
        self.command_error = None;
        self.request_redraw();
    }

    fn set_command_feedback(&mut self, message: impl Into<String>) {
        let message = message.into();
        let marker_msg = message.replace(' ', "_");
        emit_marker(
            "tui_cmd_feedback",
            None,
            &[("kind", "ok"), ("message", marker_msg.as_str())],
        );
        self.command_feedback = Some(message);
        self.request_redraw();
    }

    fn clear_command_feedback(&mut self) {
        self.command_feedback = None;
        self.request_redraw();
    }

    fn request_redraw(&mut self) {
        self.needs_redraw = true;
    }

    fn refresh_account_cache(&mut self, now_ms: u64, force: bool) -> bool {
        let alias = self
            .read_account_secret("profile_alias")
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| "unset".to_string());
        let verification_code = self
            .read_account_secret(ACCOUNT_VERIFICATION_SEED_SECRET_KEY)
            .map(|seed| format_verification_code_from_fingerprint(seed.as_str()))
            .unwrap_or_else(|| "none".to_string());
        let storage_safety = if force || self.account_storage_safety_cache == "unknown" {
            account_storage_safety_status()
        } else {
            self.account_storage_safety_cache.clone()
        };
        let changed = self.account_alias_cache != alias
            || self.account_verification_code_cache != verification_code
            || self.account_storage_safety_cache != storage_safety;
        self.account_alias_cache = alias;
        self.account_verification_code_cache = verification_code;
        self.account_storage_safety_cache = storage_safety;
        self.account_cache_last_refresh_ms = now_ms;
        emit_marker(
            "tui_account_cache_refresh",
            None,
            &[
                ("changed", if changed { "true" } else { "false" }),
                ("force", if force { "true" } else { "false" }),
            ],
        );
        changed
    }

    fn set_status_last_command_result(&mut self, message: impl Into<String>) {
        let message = message.into();
        self.status_last_command_result = Some(message.clone());
        let _ = self.persist_account_secret(TUI_LAST_COMMAND_RESULT_SECRET_KEY, message.as_str());
        self.request_redraw();
    }

    fn status_last_command_result_text(&self) -> &str {
        self.status_last_command_result.as_deref().unwrap_or("none")
    }

    fn push_cmd_result(&mut self, command: &str, ok: bool, message: impl Into<String>) {
        let status = if ok { "ok" } else { "err" };
        let message = message.into();
        let line = format!("[{}] /{} {}", status, command, message);
        let cmd_marker = command.replace(' ', "_");
        emit_marker(
            "tui_cmd_result",
            None,
            &[("kind", status), ("command", cmd_marker.as_str())],
        );
        self.cmd_results.push_back(line);
        self.status_last_command_result = self.cmd_results.back().cloned();
        if let Some(last) = self.status_last_command_result.clone() {
            let _ = self.persist_account_secret(TUI_LAST_COMMAND_RESULT_SECRET_KEY, last.as_str());
        }
        self.active_command_result_recorded = true;
        while self.cmd_results.len() > 50 {
            self.cmd_results.pop_front();
        }
        self.request_redraw();
    }

    fn begin_command_tracking(&mut self, command: impl Into<String>) {
        self.active_command_label = Some(command.into());
        self.active_command_result_recorded = false;
    }

    fn end_command_tracking(&mut self) {
        self.active_command_label = None;
        self.active_command_result_recorded = false;
    }

    fn locked_cmd_masked(&self) -> bool {
        matches!(
            self.locked_flow,
            LockedFlow::UnlockPassphrase
                | LockedFlow::InitPassphrase { .. }
                | LockedFlow::InitConfirm { .. }
        )
    }

    fn account_destroy_cmd_masked(&self) -> bool {
        matches!(self.account_destroy_flow, AccountDestroyFlow::Passphrase)
    }

    fn cmd_display_value(&self) -> String {
        if self.locked_cmd_masked() || self.account_destroy_cmd_masked() {
            "•".repeat(self.cmd_input.chars().count())
        } else {
            self.cmd_input.clone()
        }
    }

    fn cmd_bar_text(&self) -> String {
        let content = if self.is_locked() {
            if let Some(label) = self.locked_wizard_step_label() {
                if self.home_focus == TuiHomeFocus::Command {
                    format!("{}: {}{}", label, self.cmd_display_value(), '█')
                } else {
                    format!("{}:", label)
                }
            } else if self.home_focus == TuiHomeFocus::Command {
                format!("Cmd: {}{}", self.cmd_display_value(), '█')
            } else {
                "Cmd:".to_string()
            }
        } else if let Some(label) = self.account_destroy_step_label() {
            if self.home_focus == TuiHomeFocus::Command {
                format!("{}: {}{}", label, self.cmd_display_value(), '█')
            } else {
                format!("{}:", label)
            }
        } else if self.cmd_input.is_empty() {
            if let Some(msg) = self.command_feedback.as_ref() {
                msg.clone()
            } else if self.home_focus == TuiHomeFocus::Command {
                format!("Cmd: {}{}", self.cmd_display_value(), '█')
            } else {
                "Cmd: /help".to_string()
            }
        } else if self.home_focus == TuiHomeFocus::Command {
            format!("Cmd: {}{}", self.cmd_display_value(), '█')
        } else {
            "Cmd: /help".to_string()
        };
        format!("Focus: {} | {}", self.home_focus_label_token(), content)
    }

    fn accent_color_enabled(&self) -> bool {
        tui_color_enabled()
    }

    fn cmd_bar_style(&self, text: &str) -> Style {
        if !self.accent_color_enabled() {
            return Style::default();
        }
        if text.starts_with("ok:") {
            Style::default().fg(Color::Green)
        } else if text.starts_with("error:") {
            Style::default().fg(Color::Red)
        } else {
            Style::default()
        }
    }

    fn locked_main_lines(&self) -> Vec<String> {
        match &self.locked_flow {
            LockedFlow::None => {
                let mut lines = if self.has_vault() {
                    vec![
                        "Locked: unlock required".to_string(),
                        "Use /unlock to open the local vault.".to_string(),
                    ]
                } else {
                    vec![
                        "No vault found - run /init".to_string(),
                        "This creates the local encrypted state qsc needs before use.".to_string(),
                    ]
                };
                if let Some(err) = self.locked_error.as_ref() {
                    lines.push(format!("error: {}", err));
                }
                lines
            }
            LockedFlow::UnlockPassphrase => {
                let mut lines = vec![
                    "Unlock".to_string(),
                    String::new(),
                    format!("Passphrase: {}", self.cmd_display_value()),
                ];
                if let Some(err) = self.locked_error.as_ref() {
                    lines.push(format!("error: {}", err));
                }
                lines
            }
            LockedFlow::InitAlias
            | LockedFlow::InitPassphrase { .. }
            | LockedFlow::InitConfirm { .. }
            | LockedFlow::InitDecision { .. } => {
                let mut lines = vec![
                    "Initialize Vault".to_string(),
                    String::new(),
                    "This will create an encrypted vault to store your identity, contacts, messages, and files.".to_string(),
                    "Choose a strong passphrase — there is no recovery if it’s lost.".to_string(),
                    String::new(),
                ];
                let alias_summary = match &self.locked_flow {
                    LockedFlow::InitPassphrase { alias }
                    | LockedFlow::InitConfirm { alias, .. }
                    | LockedFlow::InitDecision { alias, .. } => alias.clone(),
                    LockedFlow::InitAlias => String::new(),
                    _ => String::new(),
                };
                if !alias_summary.is_empty() {
                    lines.push(format!("Alias: {}", alias_summary));
                }
                let passphrase_ready = matches!(
                    self.locked_flow,
                    LockedFlow::InitConfirm { .. } | LockedFlow::InitDecision { .. }
                );
                if passphrase_ready {
                    lines.push("Passphrase: set (hidden)".to_string());
                }
                if !alias_summary.is_empty() || passphrase_ready {
                    lines.push(String::new());
                }
                let input_label = match self.locked_flow {
                    LockedFlow::InitAlias => "Alias",
                    LockedFlow::InitPassphrase { .. } => "Passphrase",
                    LockedFlow::InitConfirm { .. } => "Confirm",
                    LockedFlow::InitDecision { .. } => "Confirm (I AGREE/N)",
                    _ => "Input",
                };
                if matches!(self.locked_flow, LockedFlow::InitDecision { .. }) {
                    lines.push("Legal acceptance required: type I AGREE to continue.".to_string());
                    lines.push(
                        "Type N to cancel. See full terms in System -> Legal after unlock."
                            .to_string(),
                    );
                    lines.push(String::new());
                }
                lines.push(format!("{}: {}", input_label, self.cmd_display_value()));
                if let Some(err) = self.locked_error.as_ref() {
                    lines.push(format!("error: {}", err));
                }
                lines
            }
        }
    }

    fn locked_main_body(&self) -> String {
        self.locked_main_lines().join("\n")
    }

    fn start_unlock_prompt(&mut self) {
        self.home_focus = TuiHomeFocus::Command;
        self.locked_flow = LockedFlow::UnlockPassphrase;
        self.cmd_input_clear();
        self.locked_clear_error();
        emit_marker("tui_unlock_prompt", None, &[("step", "passphrase")]);
        emit_marker("tui_focus_home", None, &[("pane", self.home_focus_name())]);
    }

    fn start_init_prompt(&mut self) {
        self.home_focus = TuiHomeFocus::Command;
        self.locked_flow = LockedFlow::InitAlias;
        self.cmd_input_clear();
        self.locked_clear_error();
        emit_marker(
            "tui_init_warning",
            None,
            &[("no_recovery", "true"), ("confirm_prompt", "Y_or_N")],
        );
        emit_marker("tui_init_wizard", None, &[("step", "alias")]);
        emit_marker("tui_focus_home", None, &[("pane", self.home_focus_name())]);
    }

    fn start_account_destroy_prompt(&mut self) {
        self.inspector = TuiInspectorPane::Account;
        self.sync_nav_to_inspector_header();
        self.home_focus = TuiHomeFocus::Command;
        self.account_destroy_flow = AccountDestroyFlow::Passphrase;
        self.cmd_input_clear();
        self.account_destroy_clear_error();
        emit_marker("tui_account_destroy", None, &[("step", "passphrase")]);
        emit_marker("tui_focus_home", None, &[("pane", self.home_focus_name())]);
    }

    fn cancel_account_destroy_prompt(&mut self) {
        self.account_destroy_flow = AccountDestroyFlow::None;
        self.account_destroy_clear_error();
        self.cmd_input_clear();
        self.home_focus = TuiHomeFocus::Nav;
        emit_marker("tui_account_destroy", None, &[("step", "cancel")]);
        emit_marker("tui_focus_home", None, &[("pane", self.home_focus_name())]);
    }

    fn locked_focus_toggle(&mut self) {
        self.home_focus = if self.home_focus == TuiHomeFocus::Command {
            TuiHomeFocus::Nav
        } else {
            TuiHomeFocus::Command
        };
        emit_marker("tui_focus_home", None, &[("pane", self.home_focus_name())]);
    }

    fn autolock_minutes(&self) -> u64 {
        let minutes = self.autolock_timeout_ms / 60_000;
        minutes.clamp(TUI_AUTOLOCK_MIN_MINUTES, TUI_AUTOLOCK_MAX_MINUTES)
    }

    fn open_vault_session(&mut self, passphrase: Option<&str>) -> Result<(), &'static str> {
        let session = match passphrase {
            Some(value) => vault::open_session_with_passphrase(value),
            None => vault::open_session(None),
        }?;
        self.vault_session = Some(session);
        Ok(())
    }

    fn close_vault_session(&mut self) {
        self.vault_session = None;
    }

    fn persist_account_secret(&mut self, key: &str, value: &str) -> Result<(), &'static str> {
        if let Some(session) = self.vault_session.as_mut() {
            return vault::session_set(session, key, value);
        }
        Err("vault_locked")
    }

    fn read_account_secret(&self, key: &str) -> Option<String> {
        self.vault_session
            .as_ref()
            .and_then(|session| vault::session_get(session, key).ok().flatten())
    }

    fn relay_endpoint_redacted(&self) -> String {
        match self.relay_endpoint_hash_cache.as_ref() {
            Some(hash) => format!("set (hash: {})", hash),
            None => "unset".to_string(),
        }
    }

    fn relay_auth_label(&self) -> &'static str {
        if self.relay_token_set_cache {
            "bearer token (set)"
        } else if self.relay_token_file_cache.is_some() {
            "bearer token file (set)"
        } else {
            "none (optional bearer token)"
        }
    }

    fn relay_token_file_redacted(&self) -> String {
        match self.relay_token_file_hash_cache.as_ref() {
            Some(hash) => format!("set (hash: {})", hash),
            None => "unset".to_string(),
        }
    }

    fn relay_token_file_status(&self) -> (&'static str, &'static str) {
        let Some(path) = self.relay_token_file_cache.as_ref() else {
            return ("unset", "n/a");
        };
        let p = Path::new(path.as_str());
        let md = match fs::metadata(p) {
            Ok(v) => v,
            Err(_) => return ("missing", "n/a"),
        };
        if !md.is_file() {
            return ("invalid", "n/a");
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = md.permissions().mode() & 0o777;
            if mode == 0o600 {
                ("exists", "0600_ok")
            } else {
                ("exists", "too_open")
            }
        }
        #[cfg(not(unix))]
        {
            ("exists", "platform_default")
        }
    }

    fn relay_inbox_token_redacted(&self) -> String {
        match self.relay_inbox_token_hash_cache.as_ref() {
            Some(hash) => format!("set (hash: {})", hash),
            None => "unset".to_string(),
        }
    }

    fn relay_setup_status(&self) -> (&'static str, &'static str) {
        let relay_state = if self.relay_endpoint_cache.is_some() {
            "set"
        } else {
            "missing"
        };
        let auth_state = if self.relay_token_set_cache {
            "set"
        } else {
            let (file_state, file_perms) = self.relay_token_file_status();
            if file_state == "unset" {
                "missing"
            } else if file_state == "missing" {
                "token_file_missing"
            } else if file_perms == "too_open" {
                "bad_perms"
            } else if file_state == "exists" {
                "set"
            } else {
                "missing"
            }
        };
        (relay_state, auth_state)
    }

    fn emit_setup_required_marker_if_needed(&self) {
        let (relay_state, auth_state) = self.relay_setup_status();
        if relay_state == "set" && auth_state == "set" {
            return;
        }
        emit_tui_named_marker(
            "QSC_TUI_SETUP_REQUIRED",
            &[("relay", relay_state), ("auth", auth_state)],
        );
        emit_marker(
            "tui_setup_required",
            None,
            &[("relay", relay_state), ("auth", auth_state)],
        );
    }

    fn set_relay_endpoint(&mut self, value: &str) -> Result<(), &'static str> {
        let endpoint = normalize_relay_endpoint(value)?;
        self.persist_account_secret(TUI_RELAY_ENDPOINT_SECRET_KEY, endpoint.as_str())
            .map_err(|_| "relay_config_unavailable")?;
        self.relay_endpoint_hash_cache = Some(relay_endpoint_hash8(endpoint.as_str()));
        self.relay_endpoint_cache = Some(endpoint.clone());
        self.relay = Some(TuiRelayConfig {
            relay: endpoint,
            seed: 0,
            scenario: "default".to_string(),
        });
        self.request_redraw();
        Ok(())
    }

    fn set_relay_token(&mut self, value: &str) -> Result<(), &'static str> {
        let token = value.trim();
        if token.is_empty() {
            return Err("relay_token_missing");
        }
        self.persist_account_secret(TUI_RELAY_TOKEN_SECRET_KEY, token)
            .map_err(|_| "relay_config_unavailable")?;
        self.relay_token_set_cache = true;
        self.request_redraw();
        Ok(())
    }

    fn set_relay_token_file(&mut self, value: &str) -> Result<(), &'static str> {
        let raw = value.trim();
        if raw.is_empty() {
            return Err("relay_token_file_missing");
        }
        let path = PathBuf::from(raw);
        let canonical = path
            .canonicalize()
            .unwrap_or(path)
            .to_string_lossy()
            .to_string();
        self.persist_account_secret(TUI_RELAY_TOKEN_FILE_SECRET_KEY, canonical.as_str())
            .map_err(|_| "relay_config_unavailable")?;
        self.relay_token_file_hash_cache = Some(relay_token_file_hash8(canonical.as_str()));
        self.relay_token_file_cache = Some(canonical);
        self.request_redraw();
        Ok(())
    }

    fn set_relay_inbox_token(&mut self, value: &str) -> Result<(), &'static str> {
        let token = normalize_route_token(value)?;
        self.persist_account_secret(TUI_RELAY_INBOX_TOKEN_SECRET_KEY, token.as_str())
            .map_err(|_| "relay_config_unavailable")?;
        self.relay_inbox_token_set_cache = true;
        self.relay_inbox_token_hash_cache = Some(route_token_hash8(token.as_str()));
        self.request_redraw();
        Ok(())
    }

    fn clear_relay_inbox_token(&mut self) -> Result<(), &'static str> {
        self.persist_account_secret(TUI_RELAY_INBOX_TOKEN_SECRET_KEY, "")
            .map_err(|_| "relay_config_unavailable")?;
        self.relay_inbox_token_set_cache = false;
        self.relay_inbox_token_hash_cache = None;
        self.request_redraw();
        Ok(())
    }

    fn clear_relay_config(&mut self) -> Result<(), &'static str> {
        self.persist_account_secret(TUI_RELAY_ENDPOINT_SECRET_KEY, "")
            .map_err(|_| "relay_config_unavailable")?;
        self.persist_account_secret(TUI_RELAY_TOKEN_SECRET_KEY, "")
            .map_err(|_| "relay_config_unavailable")?;
        self.persist_account_secret(TUI_RELAY_TOKEN_FILE_SECRET_KEY, "")
            .map_err(|_| "relay_config_unavailable")?;
        self.persist_account_secret(TUI_RELAY_INBOX_TOKEN_SECRET_KEY, "")
            .map_err(|_| "relay_config_unavailable")?;
        self.relay_endpoint_cache = None;
        self.relay_endpoint_hash_cache = None;
        self.relay_token_set_cache = false;
        self.relay_token_file_cache = None;
        self.relay_token_file_hash_cache = None;
        self.relay_inbox_token_hash_cache = None;
        self.relay_inbox_token_set_cache = false;
        self.relay_last_test_result = "none".to_string();
        self.relay_test_task = None;
        self.relay = None;
        self.request_redraw();
        Ok(())
    }

    fn effective_relay_config(&self) -> Option<TuiRelayConfig> {
        if let Some(relay) = self.relay.as_ref() {
            return Some(relay.clone());
        }
        self.relay_endpoint_cache
            .as_ref()
            .map(|endpoint| TuiRelayConfig {
                relay: endpoint.clone(),
                seed: 0,
                scenario: "default".to_string(),
            })
    }

    fn finish_relay_test_task(&mut self, outcome: RelayTestOutcome) {
        self.relay_test_task = None;
        if outcome.ok {
            self.relay_last_test_result = format!("ok: {}", outcome.message);
            self.set_status_last_command_result(format!("relay test ok ({})", outcome.message));
            self.push_cmd_result("relay test", true, outcome.message);
            self.set_command_feedback("ok: relay test succeeded");
        } else {
            let reason = relay_user_reason_from_code(outcome.code);
            self.relay_last_test_result = format!("err: {}", reason);
            self.set_status_last_command_result(format!("relay test err ({})", reason));
            self.push_cmd_result("relay test", false, reason);
            self.command_error = Some(format!("relay: {}", reason));
            self.command_feedback = None;
            self.route_show_to_system_nav(TuiInspectorPane::CmdResults);
        }
        emit_tui_relay_test_event(if outcome.ok { "ok" } else { "err" }, outcome.code);
        emit_marker(
            "tui_relay_test_done",
            None,
            &[
                ("ok", if outcome.ok { "true" } else { "false" }),
                ("reason", outcome.code),
            ],
        );
        self.request_redraw();
    }

    fn poll_relay_test_task(&mut self) {
        let outcome = self
            .relay_test_task
            .as_ref()
            .and_then(|rx| rx.try_recv().ok());
        let Some(outcome) = outcome else {
            return;
        };
        self.finish_relay_test_task(outcome);
    }

    fn wait_for_relay_test_task_headless(&mut self) {
        if !env_bool("QSC_TUI_HEADLESS") || self.relay_test_task.is_none() {
            return;
        }
        let outcome = self
            .relay_test_task
            .as_ref()
            .and_then(|rx| rx.recv_timeout(Duration::from_secs(3)).ok())
            .unwrap_or(RelayTestOutcome {
                ok: false,
                code: "relay_test_pending_timeout",
                message: "relay test timed out".to_string(),
            });
        self.finish_relay_test_task(outcome);
    }

    fn set_autolock_minutes(&mut self, minutes: u64) -> Result<(), &'static str> {
        if !(TUI_AUTOLOCK_MIN_MINUTES..=TUI_AUTOLOCK_MAX_MINUTES).contains(&minutes) {
            return Err("autolock_invalid_minutes");
        }
        let minutes_value = minutes.to_string();
        self.persist_account_secret(TUI_AUTOLOCK_SECRET_KEY, minutes_value.as_str())
            .map_err(|_| "autolock_config_unavailable")?;
        self.autolock_timeout_ms = minutes.saturating_mul(60_000);
        let minutes_s = minutes.to_string();
        emit_marker(
            "tui_autolock_set",
            None,
            &[("ok", "true"), ("minutes", minutes_s.as_str())],
        );
        self.request_redraw();
        Ok(())
    }

    fn poll_mode(&self) -> TuiPollMode {
        self.poll_mode
    }

    fn poll_interval_seconds(&self) -> u64 {
        self.poll_interval_seconds
            .clamp(TUI_POLL_MIN_INTERVAL_SECONDS, TUI_POLL_MAX_INTERVAL_SECONDS)
    }

    fn poll_interval_ms(&self) -> u64 {
        self.poll_interval_seconds().saturating_mul(1_000)
    }

    fn set_poll_mode_adaptive(&mut self) -> Result<(), &'static str> {
        self.persist_account_secret(TUI_POLL_MODE_SECRET_KEY, TuiPollMode::Adaptive.as_str())
            .map_err(|_| "poll_config_unavailable")?;
        let interval_value = self.poll_interval_seconds().to_string();
        self.persist_account_secret(TUI_POLL_INTERVAL_SECRET_KEY, interval_value.as_str())
            .map_err(|_| "poll_config_unavailable")?;
        self.poll_mode = TuiPollMode::Adaptive;
        self.poll_next_due_ms = None;
        let interval_s = self.poll_interval_seconds().to_string();
        emit_marker(
            "tui_poll_set",
            None,
            &[
                ("ok", "true"),
                ("mode", self.poll_mode.as_str()),
                ("interval_seconds", interval_s.as_str()),
            ],
        );
        self.request_redraw();
        Ok(())
    }

    fn set_poll_mode_fixed(&mut self, seconds: u64, now_ms: u64) -> Result<(), &'static str> {
        if !(TUI_POLL_MIN_INTERVAL_SECONDS..=TUI_POLL_MAX_INTERVAL_SECONDS).contains(&seconds) {
            return Err("poll_invalid_seconds");
        }
        self.persist_account_secret(TUI_POLL_MODE_SECRET_KEY, TuiPollMode::Fixed.as_str())
            .map_err(|_| "poll_config_unavailable")?;
        let interval_value = seconds.to_string();
        self.persist_account_secret(TUI_POLL_INTERVAL_SECRET_KEY, interval_value.as_str())
            .map_err(|_| "poll_config_unavailable")?;
        self.poll_mode = TuiPollMode::Fixed;
        self.poll_interval_seconds = seconds;
        self.poll_next_due_ms = Some(now_ms.saturating_add(self.poll_interval_ms()));
        let interval_s = self.poll_interval_seconds().to_string();
        emit_marker(
            "tui_poll_set",
            None,
            &[
                ("ok", "true"),
                ("mode", self.poll_mode.as_str()),
                ("interval_seconds", interval_s.as_str()),
            ],
        );
        self.request_redraw();
        Ok(())
    }

    fn emit_poll_show_marker(&self) {
        let interval_s = self.poll_interval_seconds().to_string();
        emit_marker(
            "tui_poll_show",
            None,
            &[
                ("ok", "true"),
                ("mode", self.poll_mode.as_str()),
                ("interval_seconds", interval_s.as_str()),
            ],
        );
    }

    pub(crate) fn maybe_run_fixed_poll(&mut self, now_ms: u64) -> bool {
        if self.is_locked() || self.poll_mode != TuiPollMode::Fixed {
            return false;
        }
        if self.relay.is_none() {
            return false;
        }
        let interval_ms = self.poll_interval_ms();
        if interval_ms == 0 {
            return false;
        }
        let mut due = self
            .poll_next_due_ms
            .unwrap_or_else(|| now_ms.saturating_add(interval_ms));
        if due > now_ms {
            self.poll_next_due_ms = Some(due);
            return false;
        }
        let mut ticked = false;
        while due <= now_ms {
            let due_s = due.to_string();
            let now_s = now_ms.to_string();
            let interval_s = self.poll_interval_seconds().to_string();
            let peer = self.selected_conversation_label();
            emit_marker(
                "tui_poll_tick",
                None,
                &[
                    ("mode", self.poll_mode.as_str()),
                    ("peer", peer.as_str()),
                    ("interval_seconds", interval_s.as_str()),
                    ("due_ms", due_s.as_str()),
                    ("now_ms", now_s.as_str()),
                ],
            );
            tui_receive_via_relay(self, peer.as_str());
            ticked = true;
            due = due.saturating_add(interval_ms);
        }
        self.poll_next_due_ms = Some(due);
        ticked
    }

    fn mark_input_activity(&mut self, now_ms: u64) {
        self.autolock_last_activity_ms = now_ms;
    }

    fn headless_now_ms(&self) -> u64 {
        self.headless_clock_ms
    }

    fn current_now_ms(&self) -> u64 {
        self.headless_clock_ms.max(self.autolock_last_activity_ms)
    }

    pub(crate) fn headless_advance_clock(&mut self, delta_ms: u64) {
        self.headless_clock_ms = self.headless_clock_ms.saturating_add(delta_ms);
        self.maybe_autolock(self.headless_clock_ms);
        let _ = self.maybe_run_fixed_poll(self.headless_clock_ms);
    }

    fn maybe_autolock(&mut self, now_ms: u64) -> bool {
        if self.is_locked() || self.autolock_timeout_ms == 0 {
            return false;
        }
        if now_ms.saturating_sub(self.autolock_last_activity_ms) < self.autolock_timeout_ms {
            return false;
        }
        self.set_locked_state(true, "inactivity_timeout");
        let minutes_s = self.autolock_minutes().to_string();
        emit_marker(
            "tui_autolock",
            None,
            &[("ok", "true"), ("minutes", minutes_s.as_str())],
        );
        true
    }

    fn take_clear_screen_pending(&mut self) -> bool {
        let pending = self.clear_screen_pending;
        self.clear_screen_pending = false;
        pending
    }

    fn take_force_full_redraw(&mut self) -> bool {
        let pending = self.force_full_redraw;
        self.force_full_redraw = false;
        pending
    }

    fn clear_ui_buffers_on_lock(&mut self, reason: &'static str) {
        self.mode = TuiMode::Normal;
        self.help_selected = 0;
        self.focus_scroll = 0;
        self.home_focus = TuiHomeFocus::Nav;
        self.locked_flow = LockedFlow::None;
        self.locked_clear_error();
        self.account_destroy_flow = AccountDestroyFlow::None;
        self.account_destroy_clear_error();
        self.clear_command_error();
        self.cmd_input_clear();
        self.inspector = TuiInspectorPane::Lock;
        self.nav_selected = 0;
        self.sync_messages_if_main_focused();
        self.sync_files_if_main_focused();
        self.sync_activity_if_main_focused();
        self.clear_screen_pending = true;
        self.force_full_redraw = true;
        self.request_redraw();
        emit_marker(
            "tui_buffer_clear",
            None,
            &[("ok", "true"), ("reason", reason)],
        );
    }

    fn set_locked_state(&mut self, locked: bool, reason: &'static str) {
        let was_locked = self.vault_locked;
        self.vault_locked = locked;
        self.status.locked = if locked { "LOCKED" } else { "UNLOCKED" };
        set_vault_unlocked(!locked);
        if locked && !was_locked {
            self.close_vault_session();
            self.reload_unlock_security_state();
            self.clear_ui_buffers_on_lock(reason);
        } else if locked {
            self.close_vault_session();
            self.reload_unlock_security_state();
            self.home_focus = TuiHomeFocus::Nav;
            self.inspector = TuiInspectorPane::Lock;
            self.nav_selected = 0;
        } else {
            self.locked_flow = LockedFlow::None;
            self.account_destroy_flow = AccountDestroyFlow::None;
            self.account_destroy_clear_error();
            self.clear_command_error();
            self.cmd_input_clear();
            if self.vault_session.is_none() {
                let _ = self.open_vault_session(None);
            }
            self.reload_unlock_security_state();
            self.reload_account_settings_from_vault();
            self.home_focus = TuiHomeFocus::Nav;
            self.inspector = TuiInspectorPane::Status;
            self.sync_nav_to_inspector_header();
            self.refresh_identity_status();
            let _ = self.refresh_account_cache(self.current_now_ms(), true);
        }
        self.request_redraw();
        emit_marker(
            "tui_lock_state",
            None,
            &[
                ("locked", self.status.locked),
                ("reason", reason),
                (
                    "vault",
                    if self.vault_present {
                        "present"
                    } else {
                        "missing"
                    },
                ),
                ("ok", "true"),
            ],
        );
    }

    fn last_payload_len(&self) -> usize {
        self.last_payload_len
    }

    fn ensure_conversation(&mut self, peer: &str) {
        self.conversations.entry(peer.to_string()).or_default();
        self.visible_counts.entry(peer.to_string()).or_insert(0);
        self.unread_counts.entry(peer.to_string()).or_insert(0);
    }

    fn conversation_labels(&self) -> Vec<String> {
        let mut labels = BTreeSet::new();
        labels.insert(TUI_NOTE_TO_SELF_LABEL.to_string());
        for (peer, stream) in &self.conversations {
            if !stream.is_empty() {
                labels.insert(peer.clone());
            }
        }
        for item in &self.files {
            if !item.peer.trim().is_empty() {
                labels.insert(item.peer.clone());
            }
        }
        let mut labels = labels.into_iter().collect::<Vec<_>>();
        if let Some(note_idx) = labels
            .iter()
            .position(|label| label == TUI_NOTE_TO_SELF_LABEL)
        {
            let note = labels.remove(note_idx);
            labels.insert(0, note);
        }
        labels
    }

    fn selected_conversation_label(&self) -> String {
        let active = self.session.peer_label.to_string();
        if active == TUI_NOTE_TO_SELF_TIMELINE_PEER {
            return TUI_NOTE_TO_SELF_LABEL.to_string();
        }
        if self.contact_record_cached(active.as_str()).is_some()
            || self.conversations.contains_key(active.as_str())
            || self.files.iter().any(|item| item.peer == active)
        {
            return active;
        }
        let labels = self.conversation_labels();
        labels
            .get(
                self.conversation_selected
                    .min(labels.len().saturating_sub(1)),
            )
            .cloned()
            .unwrap_or_else(|| TUI_NOTE_TO_SELF_LABEL.to_string())
    }

    fn apply_default_account_settings(&mut self) {
        self.autolock_timeout_ms = TUI_AUTOLOCK_DEFAULT_MINUTES.saturating_mul(60_000);
        self.poll_mode = TuiPollMode::Adaptive;
        self.poll_interval_seconds = TUI_POLL_DEFAULT_INTERVAL_SECONDS;
        self.receipt_policy = ReceiptPolicy::default();
        self.trust_onboarding_mode = TrustOnboardingMode::Balanced;
        self.poll_next_due_ms = None;
        self.status_last_command_result = None;
        self.relay_endpoint_cache = None;
        self.relay_endpoint_hash_cache = None;
        self.relay_token_set_cache = false;
        self.relay_inbox_token_hash_cache = None;
        self.relay_inbox_token_set_cache = false;
        self.relay_last_test_result = "none".to_string();
        self.relay_test_task = None;
        self.contacts_records.clear();
        self.refresh_contacts();
        self.request_redraw();
    }

    fn reload_account_settings_from_vault(&mut self) {
        let autolock_minutes = self
            .read_account_secret(TUI_AUTOLOCK_SECRET_KEY)
            .as_deref()
            .and_then(|v| normalize_tui_autolock_minutes(v).ok())
            .unwrap_or(TUI_AUTOLOCK_DEFAULT_MINUTES);
        self.autolock_timeout_ms = autolock_minutes.saturating_mul(60_000);
        let mode = self
            .read_account_secret(TUI_POLL_MODE_SECRET_KEY)
            .as_deref()
            .and_then(|v| normalize_tui_poll_mode(v).ok())
            .unwrap_or(TuiPollMode::Adaptive);
        let interval = self
            .read_account_secret(TUI_POLL_INTERVAL_SECRET_KEY)
            .as_deref()
            .and_then(|v| normalize_tui_poll_interval_seconds(v).ok())
            .unwrap_or(TUI_POLL_DEFAULT_INTERVAL_SECONDS);
        let receipt_mode = self
            .read_account_secret(TUI_RECEIPT_MODE_SECRET_KEY)
            .as_deref()
            .and_then(ReceiptEmitMode::from_raw)
            .unwrap_or(ReceiptEmitMode::Off);
        let receipt_batch_window_ms = self
            .read_account_secret(TUI_RECEIPT_BATCH_WINDOW_MS_SECRET_KEY)
            .as_deref()
            .and_then(parse_receipt_batch_window_ms)
            .unwrap_or(RECEIPT_BATCH_WINDOW_MS_DEFAULT);
        let receipt_jitter_ms = self
            .read_account_secret(TUI_RECEIPT_JITTER_MS_SECRET_KEY)
            .as_deref()
            .and_then(parse_receipt_jitter_ms)
            .unwrap_or(RECEIPT_JITTER_MS_DEFAULT);
        let file_confirm_mode = self
            .read_account_secret(TUI_FILE_CONFIRM_MODE_SECRET_KEY)
            .as_deref()
            .and_then(FileConfirmEmitMode::from_raw)
            .unwrap_or(FileConfirmEmitMode::CompleteOnly);
        let trust_onboarding_mode = self
            .read_account_secret(TUI_TRUST_MODE_SECRET_KEY)
            .as_deref()
            .and_then(TrustOnboardingMode::from_raw)
            .unwrap_or(TrustOnboardingMode::Balanced);
        self.poll_mode = mode;
        self.poll_interval_seconds = interval;
        self.receipt_policy = ReceiptPolicy {
            mode: receipt_mode,
            batch_window_ms: receipt_batch_window_ms,
            jitter_ms: receipt_jitter_ms,
            file_confirm_mode,
        };
        self.trust_onboarding_mode = trust_onboarding_mode;
        self.poll_next_due_ms = if self.poll_mode == TuiPollMode::Fixed {
            Some(
                self.current_now_ms()
                    .saturating_add(self.poll_interval_ms()),
            )
        } else {
            None
        };
        self.status_last_command_result =
            self.read_account_secret(TUI_LAST_COMMAND_RESULT_SECRET_KEY);
        self.relay_endpoint_cache = self
            .read_account_secret(TUI_RELAY_ENDPOINT_SECRET_KEY)
            .and_then(|v| {
                let trimmed = v.trim().to_string();
                if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed)
                }
            });
        self.relay_endpoint_hash_cache = self
            .relay_endpoint_cache
            .as_ref()
            .map(|endpoint| relay_endpoint_hash8(endpoint.as_str()));
        self.relay_token_set_cache = self
            .read_account_secret(TUI_RELAY_TOKEN_SECRET_KEY)
            .map(|v| !v.trim().is_empty())
            .unwrap_or(false);
        self.relay_token_file_cache = self
            .read_account_secret(TUI_RELAY_TOKEN_FILE_SECRET_KEY)
            .and_then(|v| {
                let trimmed = v.trim().to_string();
                if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed)
                }
            });
        self.relay_token_file_hash_cache = self
            .relay_token_file_cache
            .as_ref()
            .map(|path| relay_token_file_hash8(path.as_str()));
        let relay_inbox_token = self
            .read_account_secret(TUI_RELAY_INBOX_TOKEN_SECRET_KEY)
            .and_then(|v| {
                let trimmed = v.trim().to_string();
                if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed)
                }
            });
        self.relay_inbox_token_set_cache = relay_inbox_token.is_some();
        self.relay_inbox_token_hash_cache = relay_inbox_token
            .as_ref()
            .map(|token| route_token_hash8(token.as_str()));
        self.relay_last_test_result = "none".to_string();
        self.relay_test_task = None;
        let mut contacts = self
            .read_account_secret(CONTACTS_SECRET_KEY)
            .and_then(|raw| serde_json::from_str::<ContactsStore>(&raw).ok())
            .map(|store| store.peers)
            .unwrap_or_default();
        for (alias, rec) in contacts.iter_mut() {
            normalize_contact_record(alias.as_str(), rec);
        }
        self.contacts_records = contacts;
        self.refresh_contacts();
        let _ = self.refresh_account_cache(self.current_now_ms(), true);
        self.request_redraw();
    }

    fn refresh_identity_status(&mut self) {
        let fingerprint = compute_local_fingerprint();
        self.status.fingerprint = Box::leak(fingerprint.into_boxed_str());
        let peer_fp = compute_peer_fingerprint(self.session.peer_label);
        self.status.peer_fp = Box::leak(peer_fp.into_boxed_str());
        let _ = self.refresh_account_cache(self.current_now_ms(), true);
        self.request_redraw();
    }

    fn selected_contact_label(&self) -> String {
        if self.contacts.is_empty() {
            "peer-0".to_string()
        } else {
            self.contacts[self
                .contacts_selected
                .min(self.contacts.len().saturating_sub(1))]
            .clone()
        }
    }

    fn selected_peer_trust_state(&self) -> &'static str {
        contact_state(self.contact_record_cached(self.session.peer_label))
    }

    fn trust_allows_peer_send_strict(&mut self, peer: &str) -> Result<(), &'static str> {
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

    fn focus_messages_thread(&mut self, peer: &str) {
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

    fn selected_peer_identity_short(&self) -> String {
        self.contact_record_cached(self.session.peer_label)
            .map(|rec| short_identity_display(rec.fp.as_str()))
            .unwrap_or_else(|| "untrusted".to_string())
    }

    pub(crate) fn contact_record_cached(&self, label: &str) -> Option<&ContactRecord> {
        self.contacts_records.get(label)
    }

    fn contact_display_line_cached(&self, label: &str) -> String {
        label.to_string()
    }

    fn persist_contacts_cache(&mut self) -> Result<(), ErrorCode> {
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

    fn persist_contacts_cache_with(
        &mut self,
        label: &str,
        mut rec: ContactRecord,
    ) -> Result<(), ErrorCode> {
        normalize_contact_record(label, &mut rec);
        self.contacts_records.insert(label.to_string(), rec);
        self.persist_contacts_cache()
    }

    fn tui_relay_inbox_route_token(&self) -> Result<String, &'static str> {
        // Reuse the shared vault helper so TUI and CLI resolve the persisted inbox
        // token through the same path.
        relay_self_inbox_route_token()
    }

    fn tui_timeline_store_load(&self) -> Result<TimelineStore, &'static str> {
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

    fn tui_timeline_store_save(&mut self, store: &TimelineStore) -> Result<(), &'static str> {
        let json = serde_json::to_string(store).map_err(|_| "timeline_unavailable")?;
        self.persist_account_secret(TIMELINE_SECRET_KEY, json.as_str())
            .map_err(|_| "timeline_unavailable")
    }

    fn append_tui_timeline_entry(
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

    fn selected_file_id(&self) -> Option<&str> {
        self.files
            .get(self.file_selected.min(self.files.len().saturating_sub(1)))
            .map(|v| v.id.as_str())
    }

    fn refresh_file_selection_bounds(&mut self) {
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

    fn upsert_file_item(&mut self, item: TuiFileItem, from_update: bool) {
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

    fn refresh_files_from_timeline(&mut self) {
        for item in load_tui_files_snapshot() {
            self.upsert_file_item(item, true);
        }
    }

    fn files_select_by_id(&mut self, id: &str) -> bool {
        if let Some(idx) = self.files.iter().position(|v| v.id == id) {
            self.file_selected = idx;
            true
        } else {
            false
        }
    }

    fn files_toggle_selected(&mut self) -> bool {
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

    fn files_move(&mut self, delta: i32) {
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

    fn set_active_peer(&mut self, peer: &str) {
        self.session.peer_label = Box::leak(peer.to_string().into_boxed_str());
        self.refresh_qsp_status();
    }

    fn sync_messages_if_main_focused(&mut self) {
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

    fn sync_files_if_main_focused(&mut self) {
        if self.mode != TuiMode::Normal
            || self.inspector != TuiInspectorPane::Files
            || self.home_focus != TuiHomeFocus::Main
        {
            return;
        }
        self.file_unseen_updates = 0;
    }

    fn sync_activity_if_main_focused(&mut self) {
        if self.mode != TuiMode::Normal
            || self.inspector != TuiInspectorPane::Activity
            || self.home_focus != TuiHomeFocus::Main
        {
            return;
        }
        self.activity_visible_count = self.events.len();
        self.activity_unseen_updates = 0;
    }

    fn record_message_line(&mut self, peer: &str, state: &str, direction: &str, detail: &str) {
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

    fn selected_messages_thread(&self) -> Option<String> {
        if self.inspector != TuiInspectorPane::Events {
            return None;
        }
        Some(self.selected_conversation_label())
    }

    fn map_thread_to_timeline_peer(thread: &str) -> &str {
        if thread == TUI_NOTE_TO_SELF_LABEL {
            TUI_NOTE_TO_SELF_TIMELINE_PEER
        } else {
            thread
        }
    }

    fn update_send_lifecycle(&mut self, value: &str) {
        self.send_lifecycle = value.to_string();
        self.status.send_lifecycle = Box::leak(self.send_lifecycle.clone().into_boxed_str());
        emit_marker(
            "tui_status_update",
            None,
            &[("field", "send_lifecycle"), ("value", value)],
        );
    }

    fn refresh_envelope(&mut self, payload_len: usize) {
        self.last_payload_len = payload_len;
        self.envelope = compute_envelope_status(payload_len);
        self.status.envelope = Box::leak(self.envelope.clone().into_boxed_str());
        emit_marker(
            "tui_status_update",
            None,
            &[("field", "envelope"), ("value", &self.envelope)],
        );
    }

    fn refresh_qsp_status(&mut self) {
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

    fn refresh_contacts(&mut self) {
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

    fn push_event(&mut self, kind: &str, action: &str) {
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

    fn push_event_line(&mut self, line: String) {
        self.events.push_back(line);
        if self.events.len() > 64 {
            self.events.pop_front();
        }
        self.record_activity_update();
    }

    fn record_activity_update(&mut self) {
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

    fn enter_help_mode(&mut self) {
        if matches!(
            self.mode,
            TuiMode::FocusEvents
                | TuiMode::FocusFiles
                | TuiMode::FocusActivity
                | TuiMode::FocusStatus
                | TuiMode::FocusSession
                | TuiMode::FocusContacts
                | TuiMode::FocusSettings
                | TuiMode::FocusLock
        ) {
            let pane = TuiState::focus_pane_name(self.mode);
            emit_marker("tui_focus", None, &[("pane", pane), ("on", "false")]);
        }
        self.mode = TuiMode::Help;
        self.help_selected = 0;
        emit_marker("tui_help_mode", None, &[("on", "true")]);
        let items = tui_help_items();
        for item in items {
            emit_marker("tui_help_item", None, &[("cmd", item.cmd)]);
        }
        let count_s = items.len().to_string();
        emit_marker("tui_help_rendered", None, &[("count", count_s.as_str())]);
    }

    fn exit_help_mode(&mut self) {
        if self.mode == TuiMode::Help {
            self.mode = TuiMode::Normal;
            emit_marker("tui_help_mode", None, &[("on", "false")]);
        }
    }

    fn toggle_help_mode(&mut self) {
        if self.mode == TuiMode::Help {
            self.exit_help_mode();
        } else {
            self.enter_help_mode();
        }
    }

    fn focus_pane_name(mode: TuiMode) -> &'static str {
        match mode {
            TuiMode::FocusEvents => "events",
            TuiMode::FocusFiles => "files",
            TuiMode::FocusActivity => "activity",
            TuiMode::FocusStatus => "status",
            TuiMode::FocusSession => "session",
            TuiMode::FocusContacts => "contacts",
            TuiMode::FocusSettings => "settings",
            TuiMode::FocusLock => "lock",
            _ => "dashboard",
        }
    }

    fn inspector_name(&self) -> &'static str {
        match self.inspector {
            TuiInspectorPane::Events => "events",
            TuiInspectorPane::Files => "files",
            TuiInspectorPane::Activity => "activity",
            TuiInspectorPane::Status => "status",
            TuiInspectorPane::Account => "account",
            TuiInspectorPane::Relay => "relay",
            TuiInspectorPane::CmdResults => "cmd_results",
            TuiInspectorPane::Session => "session",
            TuiInspectorPane::Contacts => "contacts",
            TuiInspectorPane::Settings => "settings",
            TuiInspectorPane::Lock => "lock",
            TuiInspectorPane::Help => "help",
            TuiInspectorPane::About => "about",
            TuiInspectorPane::Legal => "legal",
        }
    }

    fn set_inspector(&mut self, pane: TuiInspectorPane) {
        self.inspector = pane;
        self.main_scroll_max_current = 0;
        self.main_view_rows_current = self.main_view_rows();
        self.sync_nav_to_inspector_header();
        self.sync_messages_if_main_focused();
        self.sync_files_if_main_focused();
        self.sync_activity_if_main_focused();
        self.request_redraw();
        emit_marker("tui_inspector", None, &[("pane", self.inspector_name())]);
    }

    fn route_show_to_system_nav(&mut self, pane: TuiInspectorPane) {
        self.set_inspector(pane);
        self.home_focus = TuiHomeFocus::Nav;
        self.cmd_input_clear();
        self.request_redraw();
        emit_marker("tui_focus_home", None, &[("pane", self.home_focus_name())]);
    }

    fn focus_mode_for_inspector(&self) -> TuiMode {
        match self.inspector {
            TuiInspectorPane::Events => TuiMode::FocusEvents,
            TuiInspectorPane::Files => TuiMode::FocusFiles,
            TuiInspectorPane::Activity => TuiMode::FocusActivity,
            TuiInspectorPane::Status => TuiMode::FocusStatus,
            TuiInspectorPane::Account => TuiMode::FocusSettings,
            TuiInspectorPane::Relay => TuiMode::FocusSettings,
            TuiInspectorPane::CmdResults => TuiMode::FocusStatus,
            TuiInspectorPane::Session => TuiMode::FocusSession,
            TuiInspectorPane::Contacts => TuiMode::FocusContacts,
            TuiInspectorPane::Settings => TuiMode::FocusSettings,
            TuiInspectorPane::Lock => TuiMode::FocusLock,
            TuiInspectorPane::Help => TuiMode::FocusSettings,
            TuiInspectorPane::About => TuiMode::FocusSettings,
            TuiInspectorPane::Legal => TuiMode::FocusSettings,
        }
    }

    fn home_layout_snapshot(&self, cols: u16, rows: u16) -> HomeLayoutSnapshot {
        HomeLayoutSnapshot {
            contacts_shown: cols >= TUI_H3_WIDE_MIN,
            header_compact: rows < TUI_H3_TALL_MIN,
        }
    }

    fn home_focus_name(&self) -> &'static str {
        match self.home_focus {
            TuiHomeFocus::Nav => "nav",
            TuiHomeFocus::Main => "main",
            TuiHomeFocus::Command => "command",
        }
    }

    fn home_focus_label_token(&self) -> &'static str {
        match self.home_focus {
            TuiHomeFocus::Nav => "NAV",
            TuiHomeFocus::Main => "MAIN",
            TuiHomeFocus::Command => "CMD",
        }
    }

    fn main_marker_title(&self) -> &'static str {
        match self.inspector {
            TuiInspectorPane::Events => "Messages Overview",
            TuiInspectorPane::Files => "Files",
            TuiInspectorPane::Activity => "Activity",
            TuiInspectorPane::Status => "System Overview",
            TuiInspectorPane::Account => "Account",
            TuiInspectorPane::Relay => "Relay",
            TuiInspectorPane::CmdResults => "Results",
            TuiInspectorPane::Session => "Keys",
            TuiInspectorPane::Contacts => "Contacts Overview",
            TuiInspectorPane::Settings => "System Settings",
            TuiInspectorPane::Lock => "Lock Status",
            TuiInspectorPane::Help => "Help",
            TuiInspectorPane::About => "About",
            TuiInspectorPane::Legal => "Legal",
        }
    }

    fn home_focus_cycle(&mut self, delta: i32) {
        let idx = match self.home_focus {
            TuiHomeFocus::Nav => 0i32,
            TuiHomeFocus::Main => 1i32,
            TuiHomeFocus::Command => 2i32,
        };
        let next = (idx + delta).rem_euclid(3);
        self.home_focus = match next {
            0 => TuiHomeFocus::Nav,
            1 => TuiHomeFocus::Main,
            _ => TuiHomeFocus::Command,
        };
        self.sync_messages_if_main_focused();
        self.sync_files_if_main_focused();
        self.sync_activity_if_main_focused();
        emit_marker("tui_focus_home", None, &[("pane", self.home_focus_name())]);
    }

    fn main_scroll_key(&self) -> &'static str {
        self.inspector.as_name()
    }

    fn main_scroll_offset(&self) -> usize {
        self.main_scroll_offsets
            .get(self.main_scroll_key())
            .copied()
            .unwrap_or(0)
    }

    fn set_main_scroll_offset(&mut self, value: usize) {
        let key = self.main_scroll_key();
        if value == 0 {
            self.main_scroll_offsets.remove(key);
        } else {
            self.main_scroll_offsets.insert(key, value);
        }
    }

    fn update_main_scroll_metrics(&mut self, content_lines: usize, view_rows: usize) {
        self.main_view_rows_current = view_rows.max(1);
        self.main_scroll_max_current = content_lines.saturating_sub(self.main_view_rows_current);
        let clamped = self.main_scroll_offset().min(self.main_scroll_max_current);
        self.set_main_scroll_offset(clamped);
    }

    fn main_view_rows(&self) -> usize {
        usize::from(terminal_rows_for_headless().saturating_sub(5)).max(1)
    }

    fn estimated_main_line_count(&self) -> usize {
        match self.inspector {
            TuiInspectorPane::Events => self
                .conversations
                .get(self.selected_conversation_label().as_str())
                .map(|v| v.len())
                .unwrap_or(0)
                .saturating_add(8),
            TuiInspectorPane::Files => self.files.len().saturating_add(14),
            TuiInspectorPane::Activity => self.events.len().saturating_add(8),
            TuiInspectorPane::Status => 22,
            TuiInspectorPane::Account => 20,
            TuiInspectorPane::Relay => 20,
            TuiInspectorPane::CmdResults => 8,
            TuiInspectorPane::Session => 16,
            TuiInspectorPane::Contacts => self.contacts.len().saturating_add(14),
            TuiInspectorPane::Settings => 20,
            TuiInspectorPane::Lock => 16,
            TuiInspectorPane::Help => 24,
            TuiInspectorPane::About => 8,
            TuiInspectorPane::Legal => 8,
        }
    }

    fn ensure_main_scroll_metrics(&mut self) {
        if self.main_view_rows_current <= 1 {
            self.main_view_rows_current = self.main_view_rows();
        }
        if self.main_scroll_max_current == 0 {
            let line_count = self.estimated_main_line_count();
            self.main_scroll_max_current = line_count.saturating_sub(self.main_view_rows_current);
            let clamped = self.main_scroll_offset().min(self.main_scroll_max_current);
            self.set_main_scroll_offset(clamped);
        }
    }

    fn emit_main_scroll_marker(&self) {
        let offset_s = self.main_scroll_offset().to_string();
        let max_s = self.main_scroll_max_current.to_string();
        let rows_s = self.main_view_rows_current.to_string();
        emit_marker(
            "tui_main_scroll",
            None,
            &[
                ("inspector", self.inspector_name()),
                ("offset", offset_s.as_str()),
                ("max", max_s.as_str()),
                ("view_rows", rows_s.as_str()),
            ],
        );
    }

    fn main_scroll_move(&mut self, delta: i32) {
        self.ensure_main_scroll_metrics();
        let mut idx = self.main_scroll_offset() as i32 + delta;
        if idx < 0 {
            idx = 0;
        }
        let max = self.main_scroll_max_current as i32;
        if idx > max {
            idx = max;
        }
        self.set_main_scroll_offset(idx as usize);
        self.request_redraw();
        self.emit_main_scroll_marker();
    }

    fn main_scroll_page(&mut self, direction: i32) {
        self.ensure_main_scroll_metrics();
        let page = self.main_view_rows_current.max(1) as i32;
        self.main_scroll_move(direction.saturating_mul(page));
    }

    fn main_scroll_home(&mut self) {
        self.ensure_main_scroll_metrics();
        self.set_main_scroll_offset(0);
        self.request_redraw();
        self.emit_main_scroll_marker();
    }

    fn main_scroll_end(&mut self) {
        self.ensure_main_scroll_metrics();
        self.set_main_scroll_offset(self.main_scroll_max_current);
        self.request_redraw();
        self.emit_main_scroll_marker();
    }

    fn emit_home_render_marker(&self, cols: u16, rows: u16) {
        if self.mode != TuiMode::Normal {
            return;
        }
        if self.is_locked() {
            let cmdbar_text = self.cmd_bar_text();
            let main_lines = self.locked_main_lines();
            let main_step = self.locked_flow_name();
            let main_intro_a = main_lines
                .iter()
                .find(|line| line.contains("This will create an encrypted vault"))
                .map(|v| v.as_str())
                .unwrap_or("none");
            let main_intro_b = main_lines
                .iter()
                .find(|line| line.contains("Choose a strong passphrase"))
                .map(|v| v.as_str())
                .unwrap_or("none");
            let main_input_line = main_lines
                .iter()
                .rev()
                .find(|line| {
                    line.starts_with("Alias:")
                        || line.starts_with("Passphrase:")
                        || line.starts_with("Confirm:")
                        || line.starts_with("Confirm (Y/N):")
                        || line.starts_with("Confirm (I AGREE/N):")
                })
                .map(|v| v.as_str())
                .unwrap_or("none");
            let main_error_line = main_lines
                .iter()
                .find(|line| line.starts_with("error:"))
                .map(|v| v.as_str())
                .unwrap_or("none");
            let main_summary_alias = main_lines
                .iter()
                .find(|line| line.starts_with("Alias:"))
                .map(|v| v.as_str())
                .unwrap_or("none");
            let main_summary_passphrase = if main_lines
                .iter()
                .any(|line| line.as_str() == "Passphrase: set (hidden)")
            {
                "set_hidden"
            } else {
                "none"
            };
            let main_hints_line = main_lines
                .iter()
                .find(|line| line.starts_with("Keys:") || line.starts_with("Submit:"))
                .map(|v| v.as_str())
                .unwrap_or("none");
            emit_marker(
                "tui_locked_shell",
                None,
                &[
                    (
                        "vault",
                        if self.has_vault() {
                            "present"
                        } else {
                            "missing"
                        },
                    ),
                    ("nav", "unlock,exit"),
                    (
                        "main",
                        if self.has_vault() {
                            "locked"
                        } else {
                            "init_required"
                        },
                    ),
                    ("cmd", if self.has_vault() { "/unlock" } else { "/init" }),
                    ("cmdbar_text", cmdbar_text.as_str()),
                    ("wizard", self.locked_flow_name()),
                    ("nav_title", "qsc"),
                    ("main_title", "none"),
                    ("cmd_panel_title", "none"),
                    ("focus", self.home_focus_name()),
                    ("main_step", main_step),
                    ("main_intro_a", main_intro_a),
                    ("main_intro_b", main_intro_b),
                    ("main_input", main_input_line),
                    ("main_error", main_error_line),
                    ("main_summary_alias", main_summary_alias),
                    ("main_summary_passphrase", main_summary_passphrase),
                    ("main_hints", main_hints_line),
                    (
                        "main_locked_line",
                        main_lines
                            .iter()
                            .find(|line| line.starts_with("Locked:"))
                            .map(|v| v.as_str())
                            .unwrap_or("none"),
                    ),
                    ("panel_pad", "2"),
                    ("nav_child_indent", "2"),
                    ("chrome", "single"),
                    ("outer_border", "1"),
                    ("header_divider", "1"),
                    ("header_row_vdiv", "0"),
                    ("v_divider", "1"),
                    ("h_divider", "1"),
                    ("divider_h_char", "─"),
                    ("divider_v_char", "│"),
                    ("divider_style", "dim"),
                ],
            );
            let nav_rows = self.nav_rows();
            let nav_selected = self.nav_selected.min(nav_rows.len().saturating_sub(1));
            let nav_selected_s = nav_selected.to_string();
            let selected_label = nav_rows
                .get(nav_selected)
                .map(|row| self.nav_row_label(row))
                .unwrap_or_else(|| "none".to_string());
            let selected_label_marker = selected_label.replace(' ', "_");
            let selected_markers = if self.home_focus == TuiHomeFocus::Nav && !nav_rows.is_empty() {
                "1"
            } else {
                "0"
            };
            emit_marker(
                "tui_nav_render",
                None,
                &[
                    ("selected_markers", selected_markers),
                    ("selected_index", nav_selected_s.as_str()),
                    ("selected_label", selected_label_marker.as_str()),
                    ("header", "[ QSC ]"),
                    ("header_left_padding", "1"),
                    ("counters", "none"),
                ],
            );
            return;
        }
        let layout = self.home_layout_snapshot(cols, rows);
        let cmdbar_text = self.cmd_bar_text();
        let cmdbar_marker = cmdbar_text.replace(' ', "_");
        let cmdbar_padded_marker = pad_panel_text(cmdbar_text.as_str()).replace(' ', "_");
        let main_scroll_s = self.main_scroll_offset().to_string();
        let main_scroll_max_s = self.main_scroll_max_current.to_string();
        let main_first_line_marker = format!(
            "{}{}",
            " ".repeat(PANEL_INNER_PAD),
            self.main_marker_title()
        )
        .replace(' ', "_");
        emit_marker(
            "tui_render",
            None,
            &[
                ("mode", "home"),
                ("layout", "h3"),
                ("inspector", self.inspector_name()),
                (
                    "contacts",
                    if layout.contacts_shown {
                        "shown"
                    } else {
                        "hidden"
                    },
                ),
                (
                    "header",
                    if layout.header_compact {
                        "compact"
                    } else {
                        "full"
                    },
                ),
                ("focus", self.home_focus_name()),
                ("nav", "shown"),
                ("expanded", self.inspector_name()),
                ("cmdbar", "full"),
                (
                    "cmd_hint",
                    if self.home_focus == TuiHomeFocus::Command {
                        "input"
                    } else {
                        "help"
                    },
                ),
                ("nav_title", "qsc"),
                ("main_title", "none"),
                ("cmd_panel_title", "none"),
                ("cmdbar_text", cmdbar_marker.as_str()),
                ("cmdbar_padded", cmdbar_padded_marker.as_str()),
                ("main_scroll", main_scroll_s.as_str()),
                ("main_scroll_max", main_scroll_max_s.as_str()),
                ("panel_pad", "2"),
                ("nav_child_indent", "2"),
                ("chrome", "single"),
                ("outer_border", "1"),
                ("header_divider", "1"),
                ("header_row_vdiv", "0"),
                ("v_divider", "1"),
                ("h_divider", "1"),
                ("divider_h_char", "─"),
                ("divider_v_char", "│"),
                ("divider_style", "dim"),
                ("main_first_line_padded", main_first_line_marker.as_str()),
            ],
        );
        match self.inspector {
            TuiInspectorPane::About => emit_marker(
                "tui_about_links",
                None,
                &[
                    ("governance", "1"),
                    ("traceability", "1"),
                    ("decisions", "1"),
                    ("docs", "1"),
                    ("tests", "1"),
                ],
            ),
            TuiInspectorPane::Legal => emit_marker(
                "tui_legal_fulltext",
                None,
                &[
                    ("sections", "summary,warranty,operator,privacy,init"),
                    ("overclaim", "none"),
                ],
            ),
            _ => {}
        }
        let nav_rows = self.nav_rows();
        let nav_selected = self.nav_selected.min(nav_rows.len().saturating_sub(1));
        let nav_selected_s = nav_selected.to_string();
        let selected_label = nav_rows
            .get(nav_selected)
            .map(|row| self.nav_row_label(row))
            .unwrap_or_else(|| "none".to_string());
        let selected_label_marker = selected_label.replace(' ', "_");
        emit_marker(
            "tui_nav_render",
            None,
            &[
                (
                    "selected_markers",
                    if nav_rows.is_empty() { "0" } else { "1" },
                ),
                ("selected_index", nav_selected_s.as_str()),
                ("selected_label", selected_label_marker.as_str()),
                ("header", "[ QSC ]"),
                ("header_left_padding", "1"),
                ("counters", "none"),
            ],
        );
        if self.inspector == TuiInspectorPane::Events {
            let peer = self.selected_conversation_label();
            let peer_marker = peer.replace(' ', "_");
            let total = self
                .conversations
                .get(peer.as_str())
                .map(|v| v.len())
                .unwrap_or(0usize);
            let visible = self
                .visible_counts
                .get(peer.as_str())
                .copied()
                .unwrap_or(total);
            let unread = self
                .unread_counts
                .get(peer.as_str())
                .copied()
                .unwrap_or(0usize);
            let total_s = total.to_string();
            let visible_s = visible.to_string();
            let unread_s = unread.to_string();
            emit_marker(
                "tui_messages_view",
                None,
                &[
                    ("peer", peer_marker.as_str()),
                    ("total", total_s.as_str()),
                    ("visible", visible_s.as_str()),
                    ("unread", unread_s.as_str()),
                    ("preview", "none"),
                    ("redacted", if self.is_locked() { "true" } else { "false" }),
                ],
            );
        }
        if self.inspector == TuiInspectorPane::Contacts {
            let selected = self.selected_contact_label();
            let selected_line = self.contact_display_line_cached(selected.as_str());
            let rec = self.contact_record_cached(selected.as_str());
            let nav_rows = self.nav_rows();
            let nav_kind = nav_rows
                .get(self.nav_selected.min(nav_rows.len().saturating_sub(1)))
                .map(|row| row.kind);
            let view = if matches!(nav_kind, Some(NavRowKind::Domain(TuiNavDomain::Contacts))) {
                "overview"
            } else {
                "detail"
            };
            emit_marker(
                "tui_contacts_view",
                None,
                &[
                    ("selected", selected.as_str()),
                    ("summary", selected_line.as_str()),
                    ("view", view),
                    ("trust", contact_state(rec)),
                    ("blocked", bool_str(rec.map(|v| v.blocked).unwrap_or(false))),
                    ("sections", "overview_table,contact_card,commands"),
                    ("you_copy", "hidden"),
                    ("commands_gap", "2"),
                    ("preview", "none"),
                    ("redacted", if self.is_locked() { "true" } else { "false" }),
                ],
            );
            if view == "overview" {
                let header = format_contacts_table_row("Alias", "Trust", "Blocked", "Last seen");
                let first_row = self
                    .contacts
                    .iter()
                    .take(TUI_INSPECTOR_CONTACTS_MAX)
                    .next()
                    .map(|alias| {
                        if let Some(rec) = self.contact_record_cached(alias) {
                            let trust = contact_state(Some(rec));
                            let blocked = if rec.blocked { "yes" } else { "no" };
                            let last_seen = rec
                                .seen_at
                                .map(|v| v.to_string())
                                .unwrap_or_else(|| "-".to_string());
                            format_contacts_table_row(alias, trust, blocked, last_seen.as_str())
                        } else {
                            format_contacts_table_row(alias, "UNVERIFIED", "no", "-")
                        }
                    })
                    .unwrap_or_else(|| format_contacts_table_row("-", "-", "-", "-"));
                let header_marker = header.replace(' ', "_");
                let first_row_marker = first_row.replace(' ', "_");
                emit_marker(
                    "tui_contacts_table",
                    None,
                    &[
                        ("header", header_marker.as_str()),
                        ("row0", first_row_marker.as_str()),
                    ],
                );
            }
        }
        if self.inspector == TuiInspectorPane::Activity {
            let total_s = self.events.len().to_string();
            let visible_s = self
                .activity_visible_count
                .min(self.events.len())
                .to_string();
            let unread_s = self.activity_unseen_updates.to_string();
            emit_marker(
                "tui_activity_view",
                None,
                &[
                    ("total", total_s.as_str()),
                    ("visible", visible_s.as_str()),
                    ("unread", unread_s.as_str()),
                    ("sections", "ledger,commands"),
                    ("redacted", if self.is_locked() { "true" } else { "false" }),
                ],
            );
        }
        if self.inspector == TuiInspectorPane::Files {
            let selected = self.selected_file_id().unwrap_or("none");
            let selected_state = self
                .files
                .get(self.file_selected.min(self.files.len().saturating_sub(1)))
                .map(|v| v.display_state.as_str())
                .unwrap_or("none");
            if let Some(item) = self
                .files
                .get(self.file_selected.min(self.files.len().saturating_sub(1)))
            {
                if let Some(delivery) = file_delivery_semantic_from_state(item.state.as_str()) {
                    emit_tui_file_delivery(item.peer.as_str(), delivery, item.id.as_str());
                }
            }
            emit_marker(
                "tui_files_view",
                None,
                &[
                    ("total", self.files.len().to_string().as_str()),
                    ("selected", selected),
                    (
                        "selected_count",
                        self.file_multi_selected.len().to_string().as_str(),
                    ),
                    ("updates", self.file_unseen_updates.to_string().as_str()),
                    ("state", selected_state),
                    ("preview", "none"),
                    ("redacted", if self.is_locked() { "true" } else { "false" }),
                ],
            );
        }
        if self.inspector == TuiInspectorPane::Session {
            let selected = self.session.peer_label;
            emit_marker(
                "tui_keys_view",
                None,
                &[
                    ("selected", selected),
                    ("sections", "metadata,verification,commands"),
                    ("multi_select", "false"),
                    ("preview", "none"),
                    ("redacted", if self.is_locked() { "true" } else { "false" }),
                ],
            );
        }
        if self.inspector == TuiInspectorPane::Settings {
            let minutes_s = self.autolock_minutes().to_string();
            let poll_interval_s = self.poll_interval_seconds().to_string();
            let receipt_batch_window_s = self.receipt_policy.batch_window_ms.to_string();
            let receipt_jitter_s = self.receipt_policy.jitter_ms.to_string();
            let attempt_limit = vault_attempt_limit_note(self.unlock_attempt_limit);
            let failed_unlocks_s = self.failed_unlock_attempts.to_string();
            emit_marker(
                "tui_settings_view",
                None,
                &[
                    ("read_only", "true"),
                    ("inline_actions", "false"),
                    ("lock_state", self.status.locked),
                    ("autolock_minutes", minutes_s.as_str()),
                    ("poll_mode", self.poll_mode().as_str()),
                    ("poll_interval_seconds", poll_interval_s.as_str()),
                    ("receipt_mode", self.receipt_policy.mode.as_str()),
                    ("receipt_batch_window_ms", receipt_batch_window_s.as_str()),
                    ("receipt_jitter_ms", receipt_jitter_s.as_str()),
                    (
                        "file_confirm_mode",
                        self.receipt_policy.file_confirm_mode.as_str(),
                    ),
                    ("vault_attempt_limit", attempt_limit.as_str()),
                    ("vault_failed_unlocks", failed_unlocks_s.as_str()),
                    ("commands_gap", "2"),
                    (
                        "sections",
                        "system_settings,lock,autolock,polling,vault_security,commands",
                    ),
                ],
            );
        }
        if self.inspector == TuiInspectorPane::Account {
            let alias_set =
                if self.account_alias_cache.is_empty() || self.account_alias_cache == "unset" {
                    "false"
                } else {
                    "true"
                };
            let alias_value = if self.is_locked() {
                "hidden"
            } else {
                self.account_alias_cache.as_str()
            };
            emit_marker(
                "tui_account_view",
                None,
                &[
                    ("sections", "identity,vault,device,commands"),
                    ("alias_set", alias_set),
                    ("alias_value", alias_value),
                    (
                        "verification_code",
                        self.account_verification_code_cache.as_str(),
                    ),
                    ("commands_gap", "2"),
                ],
            );
        }
        if self.inspector == TuiInspectorPane::Relay {
            let endpoint = self.relay_endpoint_redacted();
            let inbox_token = self.relay_inbox_token_redacted();
            let token_file = self.relay_token_file_redacted();
            let (token_file_state, token_file_perms) = self.relay_token_file_status();
            let pinning = relay_pinning_label(self.relay_endpoint_cache.as_deref());
            emit_marker(
                "tui_relay_view",
                None,
                &[
                    (
                        "configured",
                        if self.relay_endpoint_cache.is_some() {
                            "true"
                        } else {
                            "false"
                        },
                    ),
                    ("endpoint", endpoint.as_str()),
                    (
                        "transport",
                        relay_transport_label(self.relay_endpoint_cache.as_deref()),
                    ),
                    ("tls", relay_tls_label(self.relay_endpoint_cache.as_deref())),
                    ("pinning", pinning),
                    ("auth", self.relay_auth_label()),
                    ("token_file", token_file.as_str()),
                    ("token_file_state", token_file_state),
                    ("token_file_perms", token_file_perms),
                    ("inbox_token", inbox_token.as_str()),
                    ("last_test", self.relay_last_test_result.as_str()),
                    ("baseline", validated_front_door_marker()),
                    ("compatibility", compatibility_surface_marker()),
                    ("commands_gap", "2"),
                    (
                        "sections",
                        "relay_status,transport,auth,token_file,inbox_token,test,baseline,commands",
                    ),
                ],
            );
        }
        if self.inspector == TuiInspectorPane::Contacts {
            let self_alias = if self.is_locked() {
                "hidden"
            } else {
                self.account_alias_cache.as_str()
            };
            emit_marker(
                "tui_contacts_view",
                None,
                &[
                    ("selected", self.selected_contact_label().as_str()),
                    ("self_alias", self_alias),
                ],
            );
        }
        if self.inspector == TuiInspectorPane::CmdResults {
            let count_s = self.cmd_results.len().to_string();
            let (last_status, last_command, _last_detail) = self
                .cmd_results
                .back()
                .map(|entry| split_cmd_result_entry(entry.as_str()))
                .unwrap_or(("none", "none", "none"));
            let last_command_marker = last_command.replace(' ', "_");
            emit_marker(
                "tui_cmd_results_view",
                None,
                &[
                    ("count", count_s.as_str()),
                    ("sections", "last"),
                    ("last_command", last_command_marker.as_str()),
                    ("last_status", last_status),
                ],
            );
        }
        if self.inspector == TuiInspectorPane::Lock {
            let effect = if self.status.locked == "UNLOCKED" {
                "sensitive_content_displayed"
            } else {
                "sensitive_content_redacted"
            };
            let minutes_s = self.autolock_minutes().to_string();
            let attempt_limit = vault_attempt_limit_note(self.unlock_attempt_limit);
            let attempt_limit_mode = if self.unlock_attempt_limit.is_some() {
                "threshold_wipe"
            } else {
                "off"
            };
            let attempt_limit_threshold = self
                .unlock_attempt_limit
                .map(|value| value.to_string())
                .unwrap_or_else(|| "0".to_string());
            let failed_unlocks = self.failed_unlock_attempts.to_string();
            emit_marker(
                "tui_lock_view",
                None,
                &[
                    ("locked", self.status.locked),
                    ("redacted", if self.is_locked() { "true" } else { "false" }),
                    ("sections", "state,effect,autolock,vault_security,commands"),
                    ("title", "Lock Status"),
                    ("state", self.status.locked),
                    ("effect", effect),
                    ("autolock_minutes", minutes_s.as_str()),
                    ("vault_attempt_limit", attempt_limit.as_str()),
                    ("vault_attempt_limit_mode", attempt_limit_mode),
                    (
                        "vault_attempt_limit_threshold",
                        attempt_limit_threshold.as_str(),
                    ),
                    ("vault_attempt_limit_scope", "vault_and_state"),
                    ("failed_unlock_attempts", failed_unlocks.as_str()),
                    ("recovery", "rerun_init_if_wiped"),
                    ("commands", "lock,autolock_show,autolock_set"),
                ],
            );
        }
        if self.inspector == TuiInspectorPane::Status {
            let redacted = if self.status.locked == "LOCKED" {
                "true"
            } else {
                "false"
            };
            let attachment_service_active = validated_attachment_service_from_env().is_some();
            let minutes_s = self.autolock_minutes().to_string();
            let poll_interval_s = self.poll_interval_seconds().to_string();
            let receipt_batch_window_s = self.receipt_policy.batch_window_ms.to_string();
            let receipt_jitter_s = self.receipt_policy.jitter_ms.to_string();
            let peer_identity = self.selected_peer_identity_short();
            let setup_token_file = self.relay_token_file_redacted();
            emit_marker(
                "tui_status_view",
                None,
                &[
                    ("locked", self.status.locked),
                    ("redacted", redacted),
                    ("autolock_minutes", minutes_s.as_str()),
                    ("poll_mode", self.poll_mode().as_str()),
                    ("poll_interval_seconds", poll_interval_s.as_str()),
                    ("receipt_mode", self.receipt_policy.mode.as_str()),
                    (
                        "file_confirm_mode",
                        self.receipt_policy.file_confirm_mode.as_str(),
                    ),
                    ("receipt_batch_window_ms", receipt_batch_window_s.as_str()),
                    ("receipt_jitter_ms", receipt_jitter_s.as_str()),
                    ("last_result", self.status_last_command_result_text()),
                    (
                        "qsp_note",
                        qsp_status_user_note(qsp_status_parts(self.status.qsp).1),
                    ),
                    ("baseline", validated_front_door_marker()),
                    ("compatibility", compatibility_surface_marker()),
                    (
                        "migration",
                        migration_posture_marker(attachment_service_active),
                    ),
                    ("peer_trust", self.selected_peer_trust_state()),
                    ("peer_identity", peer_identity.as_str()),
                    ("setup_token_file", setup_token_file.as_str()),
                    ("sections", "system_overview,snapshot,transport,setup,queue"),
                ],
            );
        }
    }

    fn focus_render_count(&self, mode: TuiMode) -> usize {
        match mode {
            TuiMode::FocusEvents => self.focus_events_lines().len(),
            TuiMode::FocusFiles => self.focus_files_lines().len(),
            TuiMode::FocusActivity => self.focus_activity_lines().len(),
            TuiMode::FocusContacts => self.contacts.len(),
            TuiMode::FocusStatus => self.focus_status_lines().len(),
            TuiMode::FocusSession => self.focus_session_lines().len(),
            TuiMode::FocusSettings => self.focus_settings_lines().len(),
            TuiMode::FocusLock => self.focus_lock_lines().len(),
            _ => 0,
        }
    }

    fn enter_focus_mode(&mut self, mode: TuiMode) {
        if self.mode == mode {
            return;
        }
        if matches!(
            self.mode,
            TuiMode::FocusEvents
                | TuiMode::FocusFiles
                | TuiMode::FocusActivity
                | TuiMode::FocusStatus
                | TuiMode::FocusSession
                | TuiMode::FocusContacts
                | TuiMode::FocusSettings
                | TuiMode::FocusLock
        ) {
            let pane = TuiState::focus_pane_name(self.mode);
            emit_marker("tui_focus", None, &[("pane", pane), ("on", "false")]);
        }
        self.mode = mode;
        self.focus_scroll = 0;
        self.contacts_selected = 0;
        let pane = TuiState::focus_pane_name(self.mode);
        emit_marker("tui_focus", None, &[("pane", pane), ("on", "true")]);
        let count = self.focus_render_count(self.mode);
        emit_marker(
            "tui_focus_rendered",
            None,
            &[("pane", pane), ("count", count.to_string().as_str())],
        );
        self.emit_focus_render_marker();
    }

    fn exit_focus_mode(&mut self) {
        if matches!(
            self.mode,
            TuiMode::FocusEvents
                | TuiMode::FocusFiles
                | TuiMode::FocusActivity
                | TuiMode::FocusStatus
                | TuiMode::FocusSession
                | TuiMode::FocusContacts
                | TuiMode::FocusSettings
                | TuiMode::FocusLock
        ) {
            let pane = TuiState::focus_pane_name(self.mode);
            emit_marker("tui_focus", None, &[("pane", pane), ("on", "false")]);
            self.mode = TuiMode::Normal;
        }
    }

    fn is_help_mode(&self) -> bool {
        self.mode == TuiMode::Help
    }

    fn is_focus_mode(&self) -> bool {
        matches!(
            self.mode,
            TuiMode::FocusEvents
                | TuiMode::FocusFiles
                | TuiMode::FocusActivity
                | TuiMode::FocusStatus
                | TuiMode::FocusSession
                | TuiMode::FocusContacts
                | TuiMode::FocusSettings
                | TuiMode::FocusLock
        )
    }

    fn focus_max_len(&self) -> usize {
        match self.mode {
            TuiMode::FocusEvents => self.focus_events_lines().len(),
            TuiMode::FocusFiles => self.focus_files_lines().len(),
            TuiMode::FocusActivity => self.focus_activity_lines().len(),
            TuiMode::FocusContacts => self.contacts.len(),
            TuiMode::FocusStatus => self.focus_status_lines().len(),
            TuiMode::FocusSession => self.focus_session_lines().len(),
            TuiMode::FocusSettings => self.focus_settings_lines().len(),
            TuiMode::FocusLock => self.focus_lock_lines().len(),
            _ => 0,
        }
    }

    fn focus_view_rows(&self) -> usize {
        usize::from(terminal_rows_for_headless().saturating_sub(2)).max(1)
    }

    fn focus_scroll_index(&self) -> usize {
        match self.mode {
            TuiMode::FocusContacts => self.contacts_selected,
            TuiMode::FocusFiles => self.file_selected,
            _ => self.focus_scroll,
        }
    }

    fn focus_events_lines(&self) -> Vec<String> {
        self.events
            .iter()
            .enumerate()
            .map(|(i, line)| format!("{} {}", tui_timestamp_token(i), line))
            .collect()
    }

    fn focus_activity_lines(&self) -> Vec<String> {
        self.focus_events_lines()
    }

    fn focus_status_lines(&self) -> Vec<String> {
        let locked = self.status.locked == "LOCKED";
        let (qsp_state, qsp_reason) = qsp_status_parts(self.status.qsp);
        let attachment_service_active = validated_attachment_service_from_env().is_some();
        let poll_interval_s = self.poll_interval_seconds().to_string();
        let own_fp = if locked {
            "hidden (unlock required)".to_string()
        } else {
            short_identity_display(self.status.fingerprint)
        };
        let peer_fp = if locked {
            "hidden (unlock required)".to_string()
        } else {
            self.selected_peer_identity_short()
        };
        let peer_trust = self.selected_peer_trust_state();
        [
            format!("vault locked: {}", self.status.locked),
            format!("vault access: {}", vault_access_note(locked)),
            format!("fingerprint: {}", own_fp),
            format!("peer fp: {}", peer_fp),
            format!("peer trust: {}", peer_trust),
            format!("session state: {}", qsp_state),
            format!("session reason: {}", qsp_reason),
            format!("session note: {}", qsp_status_user_note(qsp_reason)),
            format!("envelope: {}", self.status.envelope),
            format!("send: {}", self.status.send_lifecycle),
            format!("poll mode: {}", self.poll_mode().as_str()),
            format!("poll interval seconds: {}", poll_interval_s),
            format!("trust mode: {}", self.trust_onboarding_mode.as_str()),
            format!("receipt mode: {}", self.receipt_policy.mode.as_str()),
            format!(
                "file confirm mode: {}",
                self.receipt_policy.file_confirm_mode.as_str()
            ),
            format!("baseline: {}", validated_front_door_note()),
            format!(
                "migration note: {}",
                migration_posture_note(attachment_service_active)
            ),
        ]
        .into_iter()
        .enumerate()
        .map(|(i, line)| format!("{} {}", tui_timestamp_token(i), line))
        .collect()
    }

    fn focus_session_lines(&self) -> Vec<String> {
        let locked = self.is_locked();
        [
            "domain: keys".to_string(),
            format!("selected_peer: {}", self.session.peer_label),
            format!(
                "verified: {}",
                if locked {
                    "hidden (unlock required)".to_string()
                } else {
                    self.session.verified.to_string()
                }
            ),
            if locked {
                "identity_metadata: hidden (unlock required)".to_string()
            } else {
                "identity_metadata: visible".to_string()
            },
            "dangerous_ops: command_bar_only".to_string(),
        ]
        .into_iter()
        .enumerate()
        .map(|(i, line)| format!("{} {}", tui_timestamp_token(i), line))
        .collect()
    }

    fn focus_contacts_lines(&self) -> Vec<String> {
        self.contacts
            .iter()
            .enumerate()
            .map(|(i, peer)| {
                format!(
                    "{} {}",
                    tui_timestamp_token(i),
                    self.contact_display_line_cached(peer)
                )
            })
            .collect()
    }

    fn focus_files_lines(&self) -> Vec<String> {
        let locked = self.is_locked();
        self.files
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let selected = if self.file_multi_selected.contains(item.id.as_str()) {
                    "[x]"
                } else {
                    "[ ]"
                };
                format!(
                    "{} {} id={} peer={} size={} state={} name={}",
                    tui_timestamp_token(i),
                    selected,
                    item.id,
                    item.peer,
                    item.byte_len,
                    item.display_state,
                    if locked {
                        "hidden (unlock required)"
                    } else {
                        item.filename.as_str()
                    }
                )
            })
            .collect()
    }

    fn focus_settings_lines(&self) -> Vec<String> {
        let poll_interval = if self.poll_mode() == TuiPollMode::Fixed {
            self.poll_interval_seconds().to_string()
        } else {
            "n/a".to_string()
        };
        let attempt_limit = vault_attempt_limit_note(self.unlock_attempt_limit);
        [
            "settings".to_string(),
            String::new(),
            format!("lock state: {}", self.status.locked),
            format!("autolock timeout minutes: {}", self.autolock_minutes()),
            format!("poll mode: {}", self.poll_mode().as_str()),
            format!("poll interval seconds: {}", poll_interval),
            format!("trust mode: {}", self.trust_onboarding_mode.as_str()),
            format!("receipt mode: {}", self.receipt_policy.mode.as_str()),
            format!(
                "receipt batch window ms: {}",
                self.receipt_policy.batch_window_ms
            ),
            format!("receipt jitter ms: {}", self.receipt_policy.jitter_ms),
            format!(
                "file confirm mode: {}",
                self.receipt_policy.file_confirm_mode.as_str()
            ),
            format!("vault attempt limit: {}", attempt_limit),
            format!(
                "failed unlock attempts since last success: {}",
                self.failed_unlock_attempts
            ),
            "commands: /status /autolock show /autolock set <minutes> /poll show /poll set adaptive /poll set fixed <seconds>".to_string(),
        ]
        .into_iter()
        .enumerate()
        .map(|(i, line)| format!("{} {}", tui_timestamp_token(i), line))
        .collect()
    }

    fn focus_lock_lines(&self) -> Vec<String> {
        let attempt_limit = vault_attempt_limit_note(self.unlock_attempt_limit);
        [
            "domain: lock".to_string(),
            format!("state: {}", self.status.locked),
            format!(
                "vault: {}",
                if self.has_vault() {
                    "present"
                } else {
                    "missing"
                }
            ),
            "redaction: sensitive_content_hidden_when_locked".to_string(),
            format!("attempt limit: {}", attempt_limit),
            format!("failed unlock attempts: {}", self.failed_unlock_attempts),
            "recovery: rerun /init if the wipe threshold is reached".to_string(),
            "commands: /lock /unlock".to_string(),
        ]
        .into_iter()
        .enumerate()
        .map(|(i, line)| format!("{} {}", tui_timestamp_token(i), line))
        .collect()
    }

    fn emit_focus_render_marker(&self) {
        if !self.is_focus_mode() {
            return;
        }
        let pane = TuiState::focus_pane_name(self.mode);
        let count = self.focus_max_len();
        let scroll = self.focus_scroll_index();
        let view_rows = self.focus_view_rows();
        let ts_start_idx = if count == 0 { 0 } else { scroll.min(count - 1) };
        let ts_end_idx = if count == 0 {
            0
        } else {
            count
                .min(scroll.saturating_add(view_rows))
                .saturating_sub(1)
        };
        let scroll_s = scroll.to_string();
        let count_s = count.to_string();
        let view_rows_s = view_rows.to_string();
        let ts_start = tui_timestamp_token(ts_start_idx);
        let ts_end = tui_timestamp_token(ts_end_idx);
        emit_marker(
            "tui_render",
            None,
            &[
                ("mode", "focus"),
                ("layout", "h3"),
                ("focus", pane),
                ("viewport", "full"),
                ("scroll", scroll_s.as_str()),
                ("count", count_s.as_str()),
                ("view_rows", view_rows_s.as_str()),
                ("ts_start", ts_start.as_str()),
                ("ts_end", ts_end.as_str()),
                (
                    "deterministic",
                    if tui_deterministic_timestamps() {
                        "true"
                    } else {
                        "false"
                    },
                ),
            ],
        );
    }

    fn help_selected_item(&self) -> Option<&'static TuiHelpItem> {
        let items = tui_help_items();
        if items.is_empty() {
            None
        } else {
            Some(&items[self.help_selected.min(items.len() - 1)])
        }
    }

    fn help_move(&mut self, delta: i32) {
        let items = tui_help_items();
        if items.is_empty() {
            self.help_selected = 0;
            return;
        }
        let len = items.len() as i32;
        let mut idx = self.help_selected as i32 + delta;
        if idx < 0 {
            idx = 0;
        }
        if idx >= len {
            idx = len - 1;
        }
        self.help_selected = idx as usize;
    }

    fn focus_scroll_move(&mut self, delta: i32, max_len: usize) {
        if max_len == 0 {
            self.focus_scroll = 0;
            self.emit_focus_render_marker();
            return;
        }
        let max = (max_len.saturating_sub(1)) as i32;
        let mut idx = self.focus_scroll as i32 + delta;
        if idx < 0 {
            idx = 0;
        }
        if idx > max {
            idx = max;
        }
        self.focus_scroll = idx as usize;
        self.emit_focus_render_marker();
    }

    fn contacts_move(&mut self, delta: i32) {
        if self.contacts.is_empty() {
            self.contacts_selected = 0;
            self.emit_focus_render_marker();
            return;
        }
        let max = (self.contacts.len() - 1) as i32;
        let mut idx = self.contacts_selected as i32 + delta;
        if idx < 0 {
            idx = 0;
        }
        if idx > max {
            idx = max;
        }
        self.contacts_selected = idx as usize;
        self.emit_focus_render_marker();
    }

    fn nav_move(&mut self, delta: i32) {
        if self.home_focus != TuiHomeFocus::Nav {
            return;
        }
        let rows = self.nav_rows();
        if rows.is_empty() {
            self.nav_selected = 0;
            return;
        }
        let len = rows.len() as i32;
        let base = self.nav_selected as i32;
        let idx = (base + delta).rem_euclid(len);
        self.nav_selected = idx as usize;
        self.nav_preview_select(rows[self.nav_selected].kind);
    }

    fn nav_activate(&mut self) {
        if self.home_focus != TuiHomeFocus::Nav {
            return;
        }
        let rows = self.nav_rows();
        if rows.is_empty() {
            return;
        }
        let row = rows[self.nav_selected.min(rows.len().saturating_sub(1))];
        match row.kind {
            NavRowKind::Domain(_) => self.nav_preview_select(row.kind),
            NavRowKind::SystemAccount => self.set_inspector(TuiInspectorPane::Account),
            NavRowKind::SystemRelay => self.set_inspector(TuiInspectorPane::Relay),
            NavRowKind::SystemSettings => self.set_inspector(TuiInspectorPane::Settings),
            NavRowKind::SystemCmdResults => self.set_inspector(TuiInspectorPane::CmdResults),
            NavRowKind::Header(pane) => self.set_inspector(pane),
            NavRowKind::Conversation(idx) => {
                self.set_inspector(TuiInspectorPane::Events);
                self.nav_preview_select(NavRowKind::Conversation(idx));
            }
            NavRowKind::Contact(idx) => {
                self.set_inspector(TuiInspectorPane::Contacts);
                self.nav_preview_select(NavRowKind::Contact(idx));
            }
            NavRowKind::Unlock => {
                emit_marker(
                    "tui_nav_select",
                    None,
                    &[("domain", "locked"), ("label", "unlock")],
                );
            }
            NavRowKind::Exit => {
                emit_marker(
                    "tui_nav_select",
                    None,
                    &[("domain", "locked"), ("label", "exit")],
                );
            }
        }
        emit_marker("tui_nav_activate", None, &[("pane", self.inspector_name())]);
    }

    fn locked_nav_activate(&mut self) -> bool {
        if !self.is_locked() || self.home_focus != TuiHomeFocus::Nav {
            return false;
        }
        self.nav_activate();
        let rows = self.nav_rows();
        if rows.is_empty() {
            return false;
        }
        let row = rows[self.nav_selected.min(rows.len().saturating_sub(1))];
        match row.kind {
            NavRowKind::Exit => {
                emit_marker("tui_cmd", None, &[("cmd", "exit")]);
                true
            }
            NavRowKind::Unlock => {
                if self.has_vault() {
                    self.start_unlock_prompt();
                } else {
                    self.start_init_prompt();
                }
                false
            }
            _ => false,
        }
    }

    fn nav_preview_select(&mut self, kind: NavRowKind) {
        match kind {
            NavRowKind::Domain(domain) => {
                match domain {
                    TuiNavDomain::System => self.set_inspector(TuiInspectorPane::Status),
                    TuiNavDomain::Contacts => self.set_inspector(TuiInspectorPane::Contacts),
                    TuiNavDomain::Messages => self.set_inspector(TuiInspectorPane::Events),
                }
                emit_marker(
                    "tui_nav_select",
                    None,
                    &[(
                        "domain",
                        match domain {
                            TuiNavDomain::System => "system",
                            TuiNavDomain::Contacts => "contacts",
                            TuiNavDomain::Messages => "messages",
                        },
                    )],
                );
            }
            NavRowKind::SystemAccount => {
                self.set_inspector(TuiInspectorPane::Account);
                emit_marker(
                    "tui_nav_select",
                    None,
                    &[("domain", "system"), ("label", "account")],
                );
            }
            NavRowKind::SystemRelay => {
                self.set_inspector(TuiInspectorPane::Relay);
                emit_marker(
                    "tui_nav_select",
                    None,
                    &[("domain", "system"), ("label", "relay")],
                );
            }
            NavRowKind::SystemSettings => {
                self.set_inspector(TuiInspectorPane::Settings);
                emit_marker(
                    "tui_nav_select",
                    None,
                    &[("domain", "system"), ("label", "settings")],
                );
            }
            NavRowKind::SystemCmdResults => {
                self.set_inspector(TuiInspectorPane::CmdResults);
                emit_marker(
                    "tui_nav_select",
                    None,
                    &[("domain", "system"), ("label", "cmd_results")],
                );
            }
            NavRowKind::Header(pane) => {
                self.inspector = pane;
                self.sync_nav_to_inspector_header();
                self.sync_messages_if_main_focused();
                self.sync_files_if_main_focused();
                self.sync_activity_if_main_focused();
                emit_marker("tui_inspector", None, &[("pane", self.inspector_name())]);
                emit_marker(
                    "tui_nav_select",
                    None,
                    &[
                        ("domain", Self::pane_domain_name(pane)),
                        ("label", pane.as_name()),
                    ],
                );
            }
            NavRowKind::Conversation(idx) => {
                let labels = self.conversation_labels();
                if labels.is_empty() {
                    self.conversation_selected = 0;
                    return;
                }
                self.conversation_selected = idx.min(labels.len().saturating_sub(1));
                let selected = self.selected_conversation_label();
                self.set_active_peer(selected.as_str());
                self.sync_messages_if_main_focused();
                let selected_marker = selected.replace(' ', "_");
                emit_marker(
                    "tui_nav_select",
                    None,
                    &[("domain", "messages"), ("label", selected_marker.as_str())],
                );
            }
            NavRowKind::Contact(idx) => {
                if self.contacts.is_empty() {
                    self.contacts_selected = 0;
                    return;
                }
                self.contacts_selected = idx.min(self.contacts.len().saturating_sub(1));
                let selected = self.selected_contact_label();
                emit_marker(
                    "tui_nav_select",
                    None,
                    &[("domain", "contacts"), ("label", selected.as_str())],
                );
            }
            NavRowKind::Unlock => {
                emit_marker(
                    "tui_nav_select",
                    None,
                    &[("domain", "locked"), ("label", "unlock")],
                );
            }
            NavRowKind::Exit => {
                emit_marker(
                    "tui_nav_select",
                    None,
                    &[("domain", "locked"), ("label", "exit")],
                );
            }
        }
    }

    fn pane_domain_name(pane: TuiInspectorPane) -> &'static str {
        match pane {
            TuiInspectorPane::Events => "messages",
            TuiInspectorPane::Files => "files",
            TuiInspectorPane::Activity => "activity",
            TuiInspectorPane::Status => "system",
            TuiInspectorPane::Account => "system",
            TuiInspectorPane::Relay => "system",
            TuiInspectorPane::CmdResults => "system",
            TuiInspectorPane::Session => "keys",
            TuiInspectorPane::Contacts => "contacts",
            TuiInspectorPane::Settings => "system",
            TuiInspectorPane::Lock => "lock",
            TuiInspectorPane::Help => "help",
            TuiInspectorPane::About => "about",
            TuiInspectorPane::Legal => "legal",
        }
    }

    fn nav_row_label(&self, row: &NavRow) -> String {
        match row.kind {
            NavRowKind::Domain(TuiNavDomain::System) => "system".to_string(),
            NavRowKind::Domain(TuiNavDomain::Contacts) => "contacts".to_string(),
            NavRowKind::Domain(TuiNavDomain::Messages) => "messages".to_string(),
            NavRowKind::SystemAccount => "account".to_string(),
            NavRowKind::SystemRelay => "relay".to_string(),
            NavRowKind::SystemSettings => "settings".to_string(),
            NavRowKind::SystemCmdResults => "results".to_string(),
            NavRowKind::Header(_) => "header".to_string(),
            NavRowKind::Conversation(item_idx) => self
                .conversation_labels()
                .get(item_idx)
                .cloned()
                .unwrap_or_else(|| "none".to_string()),
            NavRowKind::Contact(item_idx) => self
                .contacts
                .get(item_idx)
                .cloned()
                .unwrap_or_else(|| "none".to_string()),
            NavRowKind::Unlock => "unlock".to_string(),
            NavRowKind::Exit => "exit".to_string(),
        }
    }

    fn expanded_nav_domain(&self) -> Option<TuiNavDomain> {
        match self.inspector {
            TuiInspectorPane::Status
            | TuiInspectorPane::Account
            | TuiInspectorPane::Relay
            | TuiInspectorPane::Settings
            | TuiInspectorPane::CmdResults => Some(TuiNavDomain::System),
            TuiInspectorPane::Contacts => Some(TuiNavDomain::Contacts),
            TuiInspectorPane::Events => Some(TuiNavDomain::Messages),
            _ => None,
        }
    }

    fn nav_rows(&self) -> Vec<NavRow> {
        if self.is_locked() {
            return vec![
                NavRow {
                    kind: NavRowKind::Unlock,
                },
                NavRow {
                    kind: NavRowKind::Exit,
                },
            ];
        }
        let expanded = self.expanded_nav_domain();
        let mut rows = Vec::new();
        rows.push(NavRow {
            kind: NavRowKind::Domain(TuiNavDomain::System),
        });
        if expanded == Some(TuiNavDomain::System) {
            rows.push(NavRow {
                kind: NavRowKind::SystemAccount,
            });
            rows.push(NavRow {
                kind: NavRowKind::SystemRelay,
            });
            rows.push(NavRow {
                kind: NavRowKind::SystemSettings,
            });
            rows.push(NavRow {
                kind: NavRowKind::SystemCmdResults,
            });
        }
        rows.push(NavRow {
            kind: NavRowKind::Domain(TuiNavDomain::Contacts),
        });
        if expanded == Some(TuiNavDomain::Contacts) {
            for idx in 0..self.contacts.len().min(4) {
                rows.push(NavRow {
                    kind: NavRowKind::Contact(idx),
                });
            }
        }
        rows.push(NavRow {
            kind: NavRowKind::Domain(TuiNavDomain::Messages),
        });
        if expanded == Some(TuiNavDomain::Messages) {
            for idx in 0..self.conversation_labels().len().min(6) {
                rows.push(NavRow {
                    kind: NavRowKind::Conversation(idx),
                });
            }
        }
        for pane in [
            TuiInspectorPane::Activity,
            TuiInspectorPane::Session,
            TuiInspectorPane::Help,
            TuiInspectorPane::About,
            TuiInspectorPane::Legal,
        ] {
            rows.push(NavRow {
                kind: NavRowKind::Header(pane),
            });
        }
        rows
    }

    fn sync_nav_to_inspector_header(&mut self) {
        if self.is_locked() {
            self.nav_selected = 0;
            return;
        }
        let rows = self.nav_rows();
        self.nav_selected = match self.inspector {
            TuiInspectorPane::Status => rows
                .iter()
                .position(|row| matches!(row.kind, NavRowKind::Domain(TuiNavDomain::System)))
                .unwrap_or(0),
            TuiInspectorPane::Account => rows
                .iter()
                .position(|row| matches!(row.kind, NavRowKind::SystemAccount))
                .unwrap_or(0),
            TuiInspectorPane::Relay => rows
                .iter()
                .position(|row| matches!(row.kind, NavRowKind::SystemRelay))
                .unwrap_or(0),
            TuiInspectorPane::Settings => rows
                .iter()
                .position(|row| matches!(row.kind, NavRowKind::SystemSettings))
                .unwrap_or(0),
            TuiInspectorPane::CmdResults => rows
                .iter()
                .position(|row| matches!(row.kind, NavRowKind::SystemCmdResults))
                .unwrap_or(0),
            TuiInspectorPane::Contacts => rows
                .iter()
                .position(|row| matches!(row.kind, NavRowKind::Domain(TuiNavDomain::Contacts)))
                .unwrap_or(0),
            TuiInspectorPane::Events => rows
                .iter()
                .position(|row| matches!(row.kind, NavRowKind::Domain(TuiNavDomain::Messages)))
                .unwrap_or(0),
            pane => rows
                .iter()
                .position(|row| matches!(row.kind, NavRowKind::Header(p) if p == pane))
                .unwrap_or(0),
        };
    }

    fn drain_marker_queue(&mut self) {
        self.refresh_files_from_timeline();
        let mut queue = marker_queue().lock().expect("marker queue lock");
        while let Some(line) = queue.pop_front() {
            if line.contains("QSC_MARK/1") || line.contains("event=tui_nav_render") {
                continue;
            }
            self.push_event_line(line);
        }
    }
}

struct TuiHelpItem {
    cmd: &'static str,
    desc: &'static str,
}

fn tui_help_items() -> &'static [TuiHelpItem] {
    &[
        TuiHelpItem {
            cmd: "help",
            desc: "show commands",
        },
        TuiHelpItem {
            cmd:
                "inspector status|account|relay|settings|cmdresults|events|session|contacts|lock|help|about|legal",
            desc: "set home inspector pane",
        },
        TuiHelpItem {
            cmd: "focus events",
            desc: "focus Events pane",
        },
        TuiHelpItem {
            cmd: "focus files",
            desc: "focus Files pane",
        },
        TuiHelpItem {
            cmd: "focus activity",
            desc: "focus Activity pane",
        },
        TuiHelpItem {
            cmd: "focus status",
            desc: "focus Status pane",
        },
        TuiHelpItem {
            cmd: "focus session",
            desc: "focus Session pane",
        },
        TuiHelpItem {
            cmd: "focus contacts",
            desc: "focus Contacts pane",
        },
        TuiHelpItem {
            cmd: "focus settings",
            desc: "focus Settings pane",
        },
        TuiHelpItem {
            cmd: "focus lock",
            desc: "focus Lock pane",
        },
        TuiHelpItem {
            cmd: "contacts list|block <alias>|unblock <alias>|add <alias> <verification code> [route token]|route set <alias> <route token>",
            desc: "manage contact states",
        },
        TuiHelpItem {
            cmd: "verify <alias> <verification code>",
            desc: "verify stored contact code (mismatch routes to Results)",
        },
        TuiHelpItem {
            cmd: "trust pin <alias> confirm",
            desc: "pin trusted peer after out-of-band verification",
        },
        TuiHelpItem {
            cmd: "messages list|select <peer>",
            desc: "manage conversation selection",
        },
        TuiHelpItem {
            cmd: "files list|select <id>|toggle <id?>|clear-selection|inject <id> <state>",
            desc: "manage files view and multi-select in Files domain only",
        },
        TuiHelpItem {
            cmd: "injectmsg <peer> [STATE]",
            desc: "headless-only deterministic message injection",
        },
        TuiHelpItem {
            cmd: "injectevent <kind> <action>",
            desc: "headless-only deterministic activity event injection",
        },
        TuiHelpItem {
            cmd: "back",
            desc: "exit focus mode",
        },
        TuiHelpItem {
            cmd: "exit",
            desc: "exit TUI",
        },
        TuiHelpItem {
            cmd: "exithelp",
            desc: "exit help mode",
        },
        TuiHelpItem {
            cmd: "send",
            desc: "send via explicit transport",
        },
        TuiHelpItem {
            cmd: "handshake status",
            desc: "show handshake status",
        },
        TuiHelpItem {
            cmd: "handshake init",
            desc: "initiate handshake to peer",
        },
        TuiHelpItem {
            cmd: "handshake poll",
            desc: "poll inbox for handshake",
        },
        TuiHelpItem {
            cmd: "status",
            desc: "refresh status",
        },
        TuiHelpItem {
            cmd: "autolock show|set <minutes>",
            desc: "view or set inactivity lock timeout (minutes)",
        },
        TuiHelpItem {
            cmd: "poll show|set adaptive|set fixed <seconds>",
            desc: "view or set optional fixed poll cadence",
        },
        TuiHelpItem {
            cmd: "msg \"<text>\"|msg <peer> \"<text>\"",
            desc: "send message to selected thread or explicit peer",
        },
        TuiHelpItem {
            cmd: "relay show|set endpoint <url>|set token <token>|set token-file <path>|inbox set <token>|clear|clear token|clear inbox|test",
            desc: "configure/test relay endpoint with redacted output",
        },
        TuiHelpItem {
            cmd: "vault where|attempt_limit show|attempt_limit set <N>|attempt_limit clear",
            desc: "show vault path or configure failed-unlock wipe option",
        },
        TuiHelpItem {
            cmd: "device show",
            desc: "show local device mode/id summary",
        },
        TuiHelpItem {
            cmd: "lock",
            desc: "explicitly lock and redact sensitive content",
        },
        TuiHelpItem {
            cmd: "unlock",
            desc: "explicitly unlock using configured vault auth",
        },
        TuiHelpItem {
            cmd: "envelope",
            desc: "refresh envelope",
        },
        TuiHelpItem {
            cmd: "export",
            desc: "export redacted diagnostics",
        },
    ]
}

fn render_main_panel(f: &mut ratatui::Frame, area: Rect, state: &mut TuiState) {
    if state.is_locked() {
        let body = pad_panel_text(state.locked_main_body().as_str());
        let main_first_line = body
            .lines()
            .find(|line| !line.trim().is_empty())
            .unwrap_or("none")
            .replace(' ', "_");
        let panel = Paragraph::new(body);
        f.render_widget(panel, area);
        emit_marker(
            "tui_main_render",
            None,
            &[("pad", "2"), ("first_line", main_first_line.as_str())],
        );
        return;
    }
    let body = match state.inspector {
        TuiInspectorPane::Events => {
            let peer = state.selected_conversation_label();
            let stream = state.conversations.get(peer.as_str());
            let total = stream.map(|v| v.len()).unwrap_or(0usize);
            let visible = state
                .visible_counts
                .get(peer.as_str())
                .copied()
                .unwrap_or(total)
                .min(total);
            if total == 0 {
                if peer == TUI_NOTE_TO_SELF_LABEL {
                    "Messages Overview\n\nThread: Note to Self\n\nNo messages yet.\nUse command bar: /msg \"<text>\"."
                        .to_string()
                } else {
                    format!(
                        "Messages Overview\n\nThread: {peer}\n\nNo messages yet.\nUse command bar: /msg \"<text>\"."
                    )
                }
            } else {
                let mut lines = Vec::new();
                lines.push("Messages Overview".to_string());
                lines.push(String::new());
                lines.push(format!("Thread: {}", peer));
                lines.push(String::new());
                if let Some(entries) = stream {
                    for line in entries.iter().take(visible) {
                        lines.push(line.clone());
                    }
                }
                if visible < total {
                    lines.push(String::new());
                    lines.push(format!(
                        "(buffered: {} unread; focus Main on Messages to append)",
                        total - visible
                    ));
                }
                lines.join("\n")
            }
        }
        TuiInspectorPane::Files => {
            if state.files.is_empty() {
                "Files\n\nNo file transfers yet.\nUse command bar only for actions.".to_string()
            } else {
                let selected = state
                    .files
                    .get(state.file_selected.min(state.files.len().saturating_sub(1)));
                let mut lines = Vec::new();
                lines.push(format!(
                    "files: {} ({} selected)",
                    state.files.len(),
                    state.file_multi_selected.len()
                ));
                lines.push(String::new());
                if let Some(item) = selected {
                    lines.push(format!("id: {}", item.id));
                    lines.push(format!(
                        "peer: {}",
                        if state.is_locked() {
                            "hidden (unlock required)"
                        } else {
                            item.peer.as_str()
                        }
                    ));
                    lines.push(format!(
                        "name: {}",
                        if state.is_locked() {
                            "hidden (unlock required)"
                        } else {
                            item.filename.as_str()
                        }
                    ));
                    lines.push(format!("size: {} bytes", item.byte_len));
                    lines.push(format!("state: {}", item.display_state));
                    lines.push("at_rest: encrypted(vault timeline)".to_string());
                } else {
                    lines.push("selected: none".to_string());
                }
                if state.file_unseen_updates > 0 && state.home_focus != TuiHomeFocus::Main {
                    lines.push(String::new());
                    lines.push(format!(
                        "(buffered updates: {}; focus Main on Files to clear)",
                        state.file_unseen_updates
                    ));
                }
                lines.push(String::new());
                lines.push("Commands (command bar only)".to_string());
                lines.push("- /files list".to_string());
                lines.push("- /files select <id>".to_string());
                lines.push("- /files toggle <id?>".to_string());
                lines.push("- /files clear-selection".to_string());
                lines
                    .push("- /files inject <id> <state> [size] [name] (headless test)".to_string());
                lines.join("\n")
            }
        }
        TuiInspectorPane::Activity => {
            let total = state.events.len();
            let visible = state.activity_visible_count.min(total);
            let mut lines = Vec::new();
            lines.push("Activity".to_string());
            lines.push(String::new());
            lines.push(format!(
                "ledger: {} (visible={} unread={})",
                total, visible, state.activity_unseen_updates
            ));
            lines.push(String::new());
            for line in state.events.iter().take(visible) {
                lines.push(line.clone());
            }
            if visible < total {
                lines.push(String::new());
                lines.push(format!(
                    "(buffered: {} events; focus Main on Activity to append)",
                    total - visible
                ));
            }
            lines.push(String::new());
            lines.push("Commands (command bar only)".to_string());
            lines.push("- /focus activity".to_string());
            lines.join("\n")
        }
        TuiInspectorPane::Status => {
            let locked = state.status.locked == "LOCKED";
            let (qsp_state, qsp_reason) = qsp_status_parts(state.status.qsp);
            let attachment_service_active = validated_attachment_service_from_env().is_some();
            let own_fp = if locked {
                "hidden (unlock required)".to_string()
            } else {
                short_identity_display(state.status.fingerprint)
            };
            let peer_fp = if locked {
                "hidden (unlock required)".to_string()
            } else {
                state.selected_peer_identity_short()
            };
            let poll_interval_s = state.poll_interval_seconds().to_string();
            let receipt_batch_window_s = state.receipt_policy.batch_window_ms.to_string();
            let receipt_jitter_s = state.receipt_policy.jitter_ms.to_string();
            let last_result = state.status_last_command_result_text();
            let peer_trust = state.selected_peer_trust_state();
            let (token_file_state, token_file_perms) = state.relay_token_file_status();
            format!(
                "System Overview\n\nlocked: {}\nvault access: {}\nautolock minutes: {}\npoll mode: {}\npoll interval seconds: {}\nreceipt mode: {}\nreceipt batch window ms: {}\nreceipt jitter ms: {}\nfile confirm mode: {}\nlast command result: {}\n\nSession Snapshot\n\nsession state: {}\nsession reason: {}\nsession note: {}\nown fp12: {}\npeer fp12: {}\npeer trust: {}\nsend: {}\ncounts: sent={} recv={}\n\nConnection Setup\n\nrelay endpoint: {}\nauth source: {}\ntoken file: {} (state={} perms={})\nauth check: {}\n\nValidated Lane\n\nbaseline: {}\ncompatibility: {}\nmigration posture: {}",
                state.status.locked,
                vault_access_note(locked),
                state.autolock_minutes(),
                state.poll_mode().as_str(),
                poll_interval_s,
                state.receipt_policy.mode.as_str(),
                receipt_batch_window_s,
                receipt_jitter_s,
                state.receipt_policy.file_confirm_mode.as_str(),
                last_result,
                qsp_state,
                qsp_reason,
                qsp_status_user_note(qsp_reason),
                own_fp,
                peer_fp,
                peer_trust,
                state.status.send_lifecycle,
                state.session.sent_count,
                state.session.recv_count,
                state.relay_endpoint_redacted(),
                state.relay_auth_label(),
                state.relay_token_file_redacted(),
                token_file_state,
                token_file_perms,
                state.relay_last_test_result,
                validated_front_door_note(),
                compatibility_surface_note(),
                migration_posture_note(attachment_service_active)
            )
        }
        TuiInspectorPane::Account => {
            let alias = if state.is_locked() {
                "hidden (unlock required)".to_string()
            } else {
                state.account_alias_cache.clone()
            };
            let verification_code = state.account_verification_code_cache.clone();
            let storage_safety = if state.account_storage_safety_cache == "OK" {
                "OK (path perms)".to_string()
            } else {
                state.account_storage_safety_cache.clone()
            };
            let mut lines = vec![
                "Account".to_string(),
                String::new(),
                "Identity:".to_string(),
                format!("  alias: {}", alias),
                format!("  verification code: {}", verification_code),
                String::new(),
                "Vault:".to_string(),
                format!(
                    "  state: {}",
                    if state.is_locked() {
                        "LOCKED"
                    } else {
                        "UNLOCKED"
                    }
                ),
                "  location: hidden (use /vault where)".to_string(),
                format!("  storage safety: {}", storage_safety),
                "  vault: encrypted at rest".to_string(),
                String::new(),
                "Device:".to_string(),
                "  mode: single device".to_string(),
                "  device id: hidden (use /device show)".to_string(),
                String::new(),
                String::new(),
                "Commands:".to_string(),
                "  /account destroy".to_string(),
                "  /vault where".to_string(),
                "  /device show".to_string(),
            ];
            if state.account_destroy_active() {
                lines.push(String::new());
                lines.push("Destroy Vault".to_string());
                match state.account_destroy_flow {
                    AccountDestroyFlow::None => {}
                    AccountDestroyFlow::Passphrase => {
                        lines.push(format!("Passphrase: {}", state.cmd_display_value()));
                    }
                    AccountDestroyFlow::ConfirmDecision { .. } => {
                        lines.push(format!(
                            "Confirm destroy (Y/N): {}",
                            state.cmd_display_value()
                        ));
                    }
                }
                if let Some(err) = state.account_destroy_error.as_ref() {
                    lines.push(format!("error: {}", err));
                }
            }
            lines.join("\n")
        }
        TuiInspectorPane::Relay => {
            let endpoint_redacted = state.relay_endpoint_redacted();
            let endpoint = state.relay_endpoint_cache.as_deref();
            let transport = relay_transport_label(endpoint);
            let tls = relay_tls_label(endpoint);
            let pinning = relay_pinning_label(endpoint);
            let token_file_redacted = state.relay_token_file_redacted();
            let (token_file_state, token_file_perms) = state.relay_token_file_status();
            let inbox_token_redacted = state.relay_inbox_token_redacted();
            let mut lines = vec![
                "Relay".to_string(),
                String::new(),
                format!(
                    "relay status: {}",
                    if endpoint.is_some() {
                        "configured"
                    } else {
                        "not configured"
                    }
                ),
                format!("endpoint: {}", endpoint_redacted),
                format!("transport: {}", transport),
                format!("tls: {}", tls),
                format!("pinning: {}", pinning),
                format!("auth: {}", state.relay_auth_label()),
                format!("token file: {}", token_file_redacted),
                format!("token file state: {}", token_file_state),
                format!("token file perms: {}", token_file_perms),
                format!("inbox token: {}", inbox_token_redacted),
                format!("test status: {}", state.relay_last_test_result),
                format!("validated baseline: {}", validated_front_door_note()),
                format!("compatibility note: {}", compatibility_surface_note()),
                String::new(),
                String::new(),
                "Commands:".to_string(),
                "  /relay show".to_string(),
                "  /relay set endpoint <https://...>".to_string(),
                "  /relay set token <token>".to_string(),
                "  /relay set token-file <path>".to_string(),
                "  /relay inbox set <token>".to_string(),
                "  /relay clear".to_string(),
                "  /relay clear token".to_string(),
                "  /relay clear inbox".to_string(),
                "  /relay test".to_string(),
            ];
            if state.is_locked() {
                lines.push(String::new());
                lines.push("locked: unlock required".to_string());
            }
            lines.join("\n")
        }
        TuiInspectorPane::CmdResults => {
            let mut lines = Vec::new();
            lines.push("Results".to_string());
            lines.push(String::new());
            if let Some(entry) = state.cmd_results.back() {
                let (status, command, detail) = split_cmd_result_entry(entry.as_str());
                lines.push(format!("last command: /{}", command));
                lines.push(format!("status: {}", status));
                lines.push(format!("detail: {}", detail));
            } else if let Some(last) = state.status_last_command_result.as_ref() {
                let (status, command, detail) = split_cmd_result_entry(last.as_str());
                lines.push(format!("last command: /{}", command));
                lines.push(format!("status: {}", status));
                lines.push(format!("detail: {}", detail));
            } else {
                lines.push("No command results yet.".to_string());
            }
            lines.join("\n")
        }
        TuiInspectorPane::Session => {
            let replay_rejects = state
                .events
                .iter()
                .filter(|line| line.contains("ratchet_replay_reject"))
                .count();
            let mut lines = Vec::new();
            lines.push("Keys".to_string());
            lines.push(String::new());
            lines.push(format!("selected_peer: {}", state.session.peer_label));
            lines.push(format!("qsp: {}", state.status.qsp));
            lines.push(format!(
                "verification: {}",
                if state.is_locked() {
                    "hidden (unlock required)"
                } else if state.session.verified {
                    "verified"
                } else {
                    "not_verified"
                }
            ));
            lines.push(format!("replay_rejects: {}", replay_rejects));
            lines.push(String::new());
            lines.push("Metadata".to_string());
            if state.is_locked() {
                lines.push("- identity: hidden (unlock required)".to_string());
                lines.push("- peer key: hidden (unlock required)".to_string());
                lines.push("- transport key: hidden (unlock required)".to_string());
            } else {
                lines.push("- identity: inspection only".to_string());
                lines.push("- peer key: inspection only".to_string());
                lines.push("- transport key: inspection only".to_string());
            }
            lines.push(String::new());
            lines.push("Commands (command bar only)".to_string());
            lines.push("- /verify <alias> <verification code>".to_string());
            lines.push("- /trust pin <alias> confirm".to_string());
            lines.push("- /contacts add <alias> <verification code> [route token]".to_string());
            lines.push("- /contacts route set <alias> <route token>".to_string());
            lines.push("- /contacts block <peer>".to_string());
            lines.join("\n")
        }
        TuiInspectorPane::Contacts => {
            let mut lines = Vec::new();
            lines.push("Contacts".to_string());
            lines.push(String::new());
            let nav_rows = state.nav_rows();
            let nav_kind = nav_rows
                .get(state.nav_selected.min(nav_rows.len().saturating_sub(1)))
                .map(|row| row.kind);
            if matches!(nav_kind, Some(NavRowKind::Domain(TuiNavDomain::Contacts))) {
                lines.push(format_contacts_table_row(
                    "Alias",
                    "Trust",
                    "Blocked",
                    "Last seen",
                ));
                for alias in state.contacts.iter().take(TUI_INSPECTOR_CONTACTS_MAX) {
                    if let Some(rec) = state.contact_record_cached(alias) {
                        let trust = contact_state(Some(rec));
                        let blocked = if rec.blocked { "yes" } else { "no" };
                        let last_seen = rec
                            .seen_at
                            .map(|v| v.to_string())
                            .unwrap_or_else(|| "-".to_string());
                        lines.push(format_contacts_table_row(
                            alias,
                            trust,
                            blocked,
                            last_seen.as_str(),
                        ));
                    } else {
                        lines.push(format_contacts_table_row(alias, "UNVERIFIED", "no", "-"));
                    }
                }
                lines.push(String::new());
                lines.push(String::new());
                lines.push("Commands:".to_string());
                lines.push("  /contacts add <alias> <verification code> [route token]".to_string());
                lines.push("  /contacts route set <alias> <route token>".to_string());
                lines.push("  /verify <alias> <verification code>".to_string());
                lines.push("  /trust pin <alias> confirm".to_string());
                lines.push("  /contacts block <alias>".to_string());
                lines.push("  /contacts unblock <alias>".to_string());
            } else {
                let selected = state.selected_contact_label();
                let rec = state.contact_record_cached(selected.as_str()).cloned();
                let trust = contact_state(rec.as_ref());
                let blocked = rec.as_ref().map(|v| v.blocked).unwrap_or(false);
                let verification_code = if state.is_locked() {
                    "hidden (unlock required)".to_string()
                } else {
                    rec.as_ref()
                        .map(|v| v.fp.clone())
                        .unwrap_or_else(|| "unknown".to_string())
                };
                lines.push(format!("Contact: {}", selected));
                lines.push(String::new());
                lines.push("Trust".to_string());
                lines.push(format!("  state: {}", trust));
                lines.push(format!(
                    "  last verified: {}",
                    rec.as_ref()
                        .and_then(|v| v.seen_at)
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "-".to_string())
                ));
                lines.push(String::new());
                lines.push("Identity".to_string());
                lines.push(format!("  verification code: {}", verification_code));
                lines.push("  fingerprint: hidden".to_string());
                lines.push(String::new());
                lines.push("Policy".to_string());
                lines.push(format!("  blocked: {}", if blocked { "yes" } else { "no" }));
                lines.push(String::new());
                lines.push("Notes".to_string());
                lines.push("  local only: -".to_string());
                lines.push(String::new());
                lines.push(String::new());
                lines.push("Commands:".to_string());
                lines.push("  /verify <alias> <verification code>".to_string());
                lines.push("  /trust pin <alias> confirm".to_string());
                lines.push("  /contacts route set <alias> <route token>".to_string());
                lines.push("  /contacts block <alias>".to_string());
                lines.push("  /contacts unblock <alias>".to_string());
            }
            lines.join("\n")
        }
        TuiInspectorPane::Settings => {
            let poll_interval = if state.poll_mode() == TuiPollMode::Fixed {
                state.poll_interval_seconds().to_string()
            } else {
                "n/a".to_string()
            };
            let attempt_limit = vault_attempt_limit_note(state.unlock_attempt_limit);
            [
                "System Settings".to_string(),
                String::new(),
                "Lock:".to_string(),
                format!("  state: {}", state.status.locked),
                String::new(),
                "Auto-lock:".to_string(),
                "  enabled by default: true".to_string(),
                format!("  timeout minutes: {}", state.autolock_minutes()),
                String::new(),
                "Polling:".to_string(),
                format!("  mode: {}", state.poll_mode().as_str()),
                format!("  interval seconds: {}", poll_interval),
                String::new(),
                "Vault Security:".to_string(),
                format!("  attempt limit: {}", attempt_limit),
                format!(
                    "  failures since last success: {}",
                    state.failed_unlock_attempts
                ),
                "  recovery: rerun /init if the wipe threshold is reached".to_string(),
                String::new(),
                String::new(),
                "Commands:".to_string(),
                "  /status".to_string(),
                "  /autolock show".to_string(),
                "  /autolock set <minutes>".to_string(),
                "  /poll show".to_string(),
                "  /poll set adaptive".to_string(),
                "  /poll set fixed <seconds>".to_string(),
                "  /vault attempt_limit show".to_string(),
                "  /vault attempt_limit set <N>".to_string(),
                "  /vault attempt_limit clear".to_string(),
                "  /vault where".to_string(),
                "  /device show".to_string(),
            ]
            .join("\n")
        }
        TuiInspectorPane::Lock => {
            let attempt_limit = vault_attempt_limit_note(state.unlock_attempt_limit);
            let mut lines = Vec::new();
            lines.push("Lock Status".to_string());
            lines.push(String::new());
            lines.push(format!("State: {}", state.status.locked));
            lines.push(format!(
                "Vault: {}",
                if state.has_vault() {
                    "present"
                } else {
                    "missing"
                }
            ));
            if state.status.locked == "UNLOCKED" {
                lines.push("Effect: sensitive content is displayed while UNLOCKED.".to_string());
            } else {
                lines.push("Effect: sensitive content is redacted while LOCKED.".to_string());
            }
            lines.push(String::new());
            lines.push(format!(
                "Auto-lock: enabled, timeout={} min",
                state.autolock_minutes()
            ));
            lines.push(format!("Attempt limit: {}", attempt_limit));
            lines.push(format!(
                "Failed unlock attempts since last success: {}",
                state.failed_unlock_attempts
            ));
            lines.push("Recovery: rerun /init if the wipe threshold is reached.".to_string());
            lines.push(String::new());
            lines.push(String::new());
            lines.push("Commands:".to_string());
            lines.push("  /lock".to_string());
            lines.push("  /autolock show".to_string());
            lines.push("  /autolock set <min>".to_string());
            lines.join("\n")
        }
        TuiInspectorPane::Help => [
            "Help".to_string(),
            String::new(),
            "Global".to_string(),
            "- /help (opens fullscreen help)".to_string(),
            "- /inspector <domain>".to_string(),
            "- /exit".to_string(),
            String::new(),
            "Keybindings".to_string(),
            "- Tab / Shift+Tab: cycle Nav/Main/Cmd focus".to_string(),
            "- Up / Down: move nav selection".to_string(),
            "- Enter: activate selected nav item only".to_string(),
            "- Esc: return focus to Nav / clear-cancel prompts".to_string(),
            String::new(),
            "Safety".to_string(),
            "- command bar explicit intent only".to_string(),
            String::new(),
            "Validated Baseline".to_string(),
            "- qbuild/local: LOCAL_TWO_CLIENT_RUNBOOK.md is the current front door.".to_string(),
            "- remote/AWS: compatibility evidence only, not the validated baseline.".to_string(),
            String::new(),
            "Attachment Migration".to_string(),
            "- Set QSC_ATTACHMENT_SERVICE to activate the validated post-w0 lane.".to_string(),
            "- On that lane, <= 4 MiB sends use w2 and legacy receive defaults to retired."
                .to_string(),
        ]
        .join("\n"),
        TuiInspectorPane::About => {
            emit_marker(
                "tui_about_links",
                None,
                &[
                    ("governance", "1"),
                    ("traceability", "1"),
                    ("decisions", "1"),
                    ("docs", "1"),
                    ("tests", "1"),
                ],
            );
            [
                "About".to_string(),
                String::new(),
                format!("version: {}", env!("CARGO_PKG_VERSION")),
                format!(
                    "commit: {}",
                    option_env!("QSC_GIT_SHA")
                        .or(option_env!("VERGEN_GIT_SHA"))
                        .unwrap_or("unknown")
                ),
                "posture: truthful state reflection; explicit intent only".to_string(),
                String::new(),
                "Proof links".to_string(),
                "  governance: NEXT_ACTIONS.md".to_string(),
                "  traceability: TRACEABILITY.md".to_string(),
                "  decisions: DECISIONS.md".to_string(),
                "  docs: docs/canonical/".to_string(),
                "  tests: qsl/qsl-client/qsc/tests/".to_string(),
            ]
            .join("\n")
        }
        TuiInspectorPane::Legal => {
            emit_marker(
                "tui_legal_fulltext",
                None,
                &[
                    ("sections", "summary,warranty,operator,privacy,init"),
                    ("overclaim", "none"),
                ],
            );
            [
                "Legal".to_string(),
                String::new(),
                "Summary".to_string(),
                "  This software is for testing and research workflows.".to_string(),
                "  It may fail, lose data, or become unavailable without notice.".to_string(),
                String::new(),
                "Warranty and liability".to_string(),
                "  Provided \"as is\" and \"as available\" without warranties.".to_string(),
                "  Operators and contributors are not liable for indirect or consequential losses."
                    .to_string(),
                String::new(),
                "Operator responsibility".to_string(),
                "  You are responsible for lawful use, local policy compliance, and key handling."
                    .to_string(),
                "  Verify identities out-of-band before relying on trust state.".to_string(),
                String::new(),
                "Privacy and security notes".to_string(),
                "  This interface does not claim metadata elimination.".to_string(),
                "  Treat endpoint, traffic timing, and deployment logs as potentially observable."
                    .to_string(),
                String::new(),
                "Init acceptance".to_string(),
                "  /init requires explicit legal acceptance (I AGREE) before vault creation."
                    .to_string(),
            ]
            .join("\n")
        }
    };
    let commands_gap = if body.contains("\n\n\nCommands:") {
        "2_plus"
    } else if body.contains("\n\nCommands:") {
        "1"
    } else if body.contains("\nCommands:") {
        "0"
    } else {
        "na"
    };
    emit_marker(
        "tui_commands_spacing",
        None,
        &[("inspector", state.inspector_name()), ("gap", commands_gap)],
    );
    let body = pad_panel_text(body.as_str());
    let main_first_line = body
        .lines()
        .find(|line| !line.trim().is_empty())
        .unwrap_or("none")
        .replace(' ', "_");
    let view_rows = usize::from(area.height).max(1);
    let content_lines = body.lines().count().max(1);
    state.update_main_scroll_metrics(content_lines, view_rows);
    let scroll = state.main_scroll_offset();
    let panel = Paragraph::new(body).scroll((scroll as u16, 0));
    f.render_widget(panel, area);
    emit_marker(
        "tui_main_render",
        None,
        &[("pad", "2"), ("first_line", main_first_line.as_str())],
    );
}

fn tui_file_display_state(raw: &str) -> String {
    let upper = raw.trim().to_ascii_uppercase();
    match upper.as_str() {
        "VERIFIED" | "COMPLETE" => "VERIFIED".to_string(),
        "FAILED" | "REJECTED" => "FAILED".to_string(),
        "RECEIVING" | "CREATED" | "SENT" | "ANNOUNCED" | "PENDING" => "RECEIVING".to_string(),
        _ => upper,
    }
}

fn load_tui_files_snapshot() -> Vec<TuiFileItem> {
    let store = match timeline_store_load() {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };
    let mut out: Vec<TuiFileItem> = store
        .file_transfers
        .values()
        .map(|rec| TuiFileItem {
            id: rec.id.clone(),
            peer: rec.peer.clone(),
            filename: rec.filename.clone(),
            byte_len: rec.total_size,
            state: rec.state.clone(),
            display_state: tui_file_display_state(rec.state.as_str()),
        })
        .collect();
    out.sort_by(|a, b| a.id.cmp(&b.id));
    out
}
