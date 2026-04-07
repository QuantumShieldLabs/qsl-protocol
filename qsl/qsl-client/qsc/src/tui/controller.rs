use super::script::*;
use super::*;
use crate::*;

mod commands;
mod render;
mod state;

use self::commands::{handle_tui_command, handle_tui_key};
use self::render::draw_tui;

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

fn tui_interactive_test(cfg: TuiConfig) {
    set_marker_routing(MarkerRouting::InApp);
    let mut state = TuiState::new(cfg);
    emit_marker("tui_open", None, &[]);
    state.drain_marker_queue();
    println!("tui_test_done");
}
