use super::super::render::*;
use super::commands::{
    format_message_transcript_line, tui_receive_via_relay, wipe_account_local_state_best_effort,
};
use super::render::{tui_help_items, TuiHelpItem};
use super::*;

pub(super) fn tui_vault_present() -> bool {
    config_dir()
        .ok()
        .map(|(dir, _)| dir.join("vault.qsv").exists())
        .unwrap_or(false)
}

pub(super) fn tui_relay_config(cfg: &TuiConfig) -> Option<TuiRelayConfig> {
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

    pub(super) fn is_locked(&self) -> bool {
        self.vault_locked
    }

    pub(super) fn has_vault(&self) -> bool {
        self.vault_present
    }

    pub(super) fn mark_vault_present(&mut self) {
        self.vault_present = true;
    }

    pub(super) fn mark_vault_absent(&mut self) {
        self.vault_present = false;
        vault::set_process_passphrase(None);
    }

    pub(super) fn reload_unlock_security_state(&mut self) {
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

    pub(super) fn persist_unlock_security_state(&self) -> Result<(), &'static str> {
        let state = VaultSecurityState {
            attempt_limit: self.unlock_attempt_limit,
            failed_unlocks: self.failed_unlock_attempts,
        };
        vault_security_state_store(&state).map_err(|_| "vault_attempt_limit_io")
    }

    pub(super) fn set_unlock_attempt_limit(
        &mut self,
        limit: Option<u32>,
    ) -> Result<(), &'static str> {
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

    pub(super) fn reset_unlock_failure_counter(&mut self) {
        if self.failed_unlock_attempts == 0 {
            return;
        }
        self.failed_unlock_attempts = 0;
        let _ = self.persist_unlock_security_state();
    }

    pub(super) fn wipe_after_failed_unlock_limit(&mut self) -> Result<(), &'static str> {
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

    pub(super) fn record_unlock_failure_and_maybe_wipe(&mut self) -> UnlockAttemptOutcome {
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

    pub(super) fn unlock_with_policy(&mut self, passphrase: &str) -> UnlockAttemptOutcome {
        let unlocked = vault::unlock_with_passphrase(passphrase).is_ok()
            && self.open_vault_session(Some(passphrase)).is_ok();
        if unlocked {
            vault::set_process_passphrase(Some(passphrase));
            self.reset_unlock_failure_counter();
            return UnlockAttemptOutcome::Unlocked;
        }
        self.record_unlock_failure_and_maybe_wipe()
    }

    pub(super) fn cmd_input_clear(&mut self) {
        self.cmd_input.clear();
    }

    pub(super) fn cmd_input_push(&mut self, ch: char) {
        self.cmd_input.push(ch);
    }

    pub(super) fn cmd_input_pop(&mut self) {
        self.cmd_input.pop();
    }

    pub(super) fn locked_flow_name(&self) -> &'static str {
        match self.locked_flow {
            LockedFlow::None => "none",
            LockedFlow::UnlockPassphrase => "unlock_passphrase",
            LockedFlow::InitAlias => "init_alias",
            LockedFlow::InitPassphrase { .. } => "init_passphrase",
            LockedFlow::InitConfirm { .. } => "init_confirm",
            LockedFlow::InitDecision { .. } => "init_decision",
        }
    }

    pub(super) fn locked_wizard_step_label(&self) -> Option<&'static str> {
        match self.locked_flow {
            LockedFlow::None => None,
            LockedFlow::UnlockPassphrase => Some("Passphrase"),
            LockedFlow::InitAlias => Some("Alias"),
            LockedFlow::InitPassphrase { .. } => Some("Passphrase"),
            LockedFlow::InitConfirm { .. } => Some("Confirm"),
            LockedFlow::InitDecision { .. } => Some("Confirm (I AGREE/N)"),
        }
    }

    pub(super) fn account_destroy_step_label(&self) -> Option<&'static str> {
        match self.account_destroy_flow {
            AccountDestroyFlow::None => None,
            AccountDestroyFlow::Passphrase => Some("Passphrase"),
            AccountDestroyFlow::ConfirmDecision { .. } => Some("Confirm (Y/N)"),
        }
    }

    pub(super) fn account_destroy_set_error(&mut self, message: impl Into<String>) {
        self.account_destroy_error = Some(message.into());
    }

    pub(super) fn account_destroy_clear_error(&mut self) {
        self.account_destroy_error = None;
    }

    pub(super) fn account_destroy_active(&self) -> bool {
        !matches!(self.account_destroy_flow, AccountDestroyFlow::None)
    }

    pub(super) fn locked_set_error(&mut self, message: impl Into<String>) {
        self.locked_error = Some(message.into());
    }

    pub(super) fn locked_clear_error(&mut self) {
        self.locked_error = None;
    }

    pub(super) fn set_command_error(&mut self, message: impl Into<String>) {
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

    pub(super) fn clear_command_error(&mut self) {
        self.command_error = None;
        self.request_redraw();
    }

    pub(super) fn set_command_feedback(&mut self, message: impl Into<String>) {
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

    pub(super) fn clear_command_feedback(&mut self) {
        self.command_feedback = None;
        self.request_redraw();
    }

    pub(super) fn request_redraw(&mut self) {
        self.needs_redraw = true;
    }

    pub(super) fn refresh_account_cache(&mut self, now_ms: u64, force: bool) -> bool {
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

    pub(super) fn set_status_last_command_result(&mut self, message: impl Into<String>) {
        let message = message.into();
        self.status_last_command_result = Some(message.clone());
        let _ = self.persist_account_secret(TUI_LAST_COMMAND_RESULT_SECRET_KEY, message.as_str());
        self.request_redraw();
    }

    pub(super) fn status_last_command_result_text(&self) -> &str {
        self.status_last_command_result.as_deref().unwrap_or("none")
    }

    pub(super) fn push_cmd_result(&mut self, command: &str, ok: bool, message: impl Into<String>) {
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

    pub(super) fn begin_command_tracking(&mut self, command: impl Into<String>) {
        self.active_command_label = Some(command.into());
        self.active_command_result_recorded = false;
    }

    pub(super) fn end_command_tracking(&mut self) {
        self.active_command_label = None;
        self.active_command_result_recorded = false;
    }

    pub(super) fn locked_cmd_masked(&self) -> bool {
        matches!(
            self.locked_flow,
            LockedFlow::UnlockPassphrase
                | LockedFlow::InitPassphrase { .. }
                | LockedFlow::InitConfirm { .. }
        )
    }

    pub(super) fn account_destroy_cmd_masked(&self) -> bool {
        matches!(self.account_destroy_flow, AccountDestroyFlow::Passphrase)
    }

    pub(super) fn cmd_display_value(&self) -> String {
        if self.locked_cmd_masked() || self.account_destroy_cmd_masked() {
            "•".repeat(self.cmd_input.chars().count())
        } else {
            self.cmd_input.clone()
        }
    }

    pub(super) fn cmd_bar_text(&self) -> String {
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

    pub(super) fn accent_color_enabled(&self) -> bool {
        tui_color_enabled()
    }

    pub(super) fn cmd_bar_style(&self, text: &str) -> Style {
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

    pub(super) fn locked_main_lines(&self) -> Vec<String> {
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

    pub(super) fn locked_main_body(&self) -> String {
        self.locked_main_lines().join("\n")
    }

    pub(super) fn start_unlock_prompt(&mut self) {
        self.home_focus = TuiHomeFocus::Command;
        self.locked_flow = LockedFlow::UnlockPassphrase;
        self.cmd_input_clear();
        self.locked_clear_error();
        emit_marker("tui_unlock_prompt", None, &[("step", "passphrase")]);
        emit_marker("tui_focus_home", None, &[("pane", self.home_focus_name())]);
    }

    pub(super) fn start_init_prompt(&mut self) {
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

    pub(super) fn start_account_destroy_prompt(&mut self) {
        self.inspector = TuiInspectorPane::Account;
        self.sync_nav_to_inspector_header();
        self.home_focus = TuiHomeFocus::Command;
        self.account_destroy_flow = AccountDestroyFlow::Passphrase;
        self.cmd_input_clear();
        self.account_destroy_clear_error();
        emit_marker("tui_account_destroy", None, &[("step", "passphrase")]);
        emit_marker("tui_focus_home", None, &[("pane", self.home_focus_name())]);
    }

    pub(super) fn cancel_account_destroy_prompt(&mut self) {
        self.account_destroy_flow = AccountDestroyFlow::None;
        self.account_destroy_clear_error();
        self.cmd_input_clear();
        self.home_focus = TuiHomeFocus::Nav;
        emit_marker("tui_account_destroy", None, &[("step", "cancel")]);
        emit_marker("tui_focus_home", None, &[("pane", self.home_focus_name())]);
    }

    pub(super) fn locked_focus_toggle(&mut self) {
        self.home_focus = if self.home_focus == TuiHomeFocus::Command {
            TuiHomeFocus::Nav
        } else {
            TuiHomeFocus::Command
        };
        emit_marker("tui_focus_home", None, &[("pane", self.home_focus_name())]);
    }

    pub(super) fn autolock_minutes(&self) -> u64 {
        let minutes = self.autolock_timeout_ms / 60_000;
        minutes.clamp(TUI_AUTOLOCK_MIN_MINUTES, TUI_AUTOLOCK_MAX_MINUTES)
    }

    pub(super) fn open_vault_session(
        &mut self,
        passphrase: Option<&str>,
    ) -> Result<(), &'static str> {
        let session = match passphrase {
            Some(value) => vault::open_session_with_passphrase(value),
            None => vault::open_session(None),
        }?;
        self.vault_session = Some(session);
        Ok(())
    }

    pub(super) fn close_vault_session(&mut self) {
        self.vault_session = None;
    }

    pub(super) fn persist_account_secret(
        &mut self,
        key: &str,
        value: &str,
    ) -> Result<(), &'static str> {
        if let Some(session) = self.vault_session.as_mut() {
            return vault::session_set(session, key, value);
        }
        Err("vault_locked")
    }

    pub(super) fn read_account_secret(&self, key: &str) -> Option<String> {
        self.vault_session
            .as_ref()
            .and_then(|session| vault::session_get(session, key).ok().flatten())
    }

    pub(super) fn relay_endpoint_redacted(&self) -> String {
        match self.relay_endpoint_hash_cache.as_ref() {
            Some(hash) => format!("set (hash: {})", hash),
            None => "unset".to_string(),
        }
    }

    pub(super) fn relay_auth_label(&self) -> &'static str {
        if self.relay_token_set_cache {
            "bearer token (set)"
        } else if self.relay_token_file_cache.is_some() {
            "bearer token file (set)"
        } else {
            "none (optional bearer token)"
        }
    }

    pub(super) fn relay_token_file_redacted(&self) -> String {
        match self.relay_token_file_hash_cache.as_ref() {
            Some(hash) => format!("set (hash: {})", hash),
            None => "unset".to_string(),
        }
    }

    pub(super) fn relay_token_file_status(&self) -> (&'static str, &'static str) {
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

    pub(super) fn relay_inbox_token_redacted(&self) -> String {
        match self.relay_inbox_token_hash_cache.as_ref() {
            Some(hash) => format!("set (hash: {})", hash),
            None => "unset".to_string(),
        }
    }

    pub(super) fn relay_setup_status(&self) -> (&'static str, &'static str) {
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

    pub(super) fn emit_setup_required_marker_if_needed(&self) {
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

    pub(super) fn set_relay_endpoint(&mut self, value: &str) -> Result<(), &'static str> {
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

    pub(super) fn set_relay_token(&mut self, value: &str) -> Result<(), &'static str> {
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

    pub(super) fn set_relay_token_file(&mut self, value: &str) -> Result<(), &'static str> {
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

    pub(super) fn set_relay_inbox_token(&mut self, value: &str) -> Result<(), &'static str> {
        let token = normalize_route_token(value)?;
        self.persist_account_secret(TUI_RELAY_INBOX_TOKEN_SECRET_KEY, token.as_str())
            .map_err(|_| "relay_config_unavailable")?;
        self.relay_inbox_token_set_cache = true;
        self.relay_inbox_token_hash_cache = Some(route_token_hash8(token.as_str()));
        self.request_redraw();
        Ok(())
    }

    pub(super) fn clear_relay_inbox_token(&mut self) -> Result<(), &'static str> {
        self.persist_account_secret(TUI_RELAY_INBOX_TOKEN_SECRET_KEY, "")
            .map_err(|_| "relay_config_unavailable")?;
        self.relay_inbox_token_set_cache = false;
        self.relay_inbox_token_hash_cache = None;
        self.request_redraw();
        Ok(())
    }

    pub(super) fn clear_relay_config(&mut self) -> Result<(), &'static str> {
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

    pub(super) fn effective_relay_config(&self) -> Option<TuiRelayConfig> {
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

    pub(super) fn finish_relay_test_task(&mut self, outcome: RelayTestOutcome) {
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

    pub(super) fn poll_relay_test_task(&mut self) {
        let outcome = self
            .relay_test_task
            .as_ref()
            .and_then(|rx| rx.try_recv().ok());
        let Some(outcome) = outcome else {
            return;
        };
        self.finish_relay_test_task(outcome);
    }

    pub(super) fn wait_for_relay_test_task_headless(&mut self) {
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

    pub(super) fn set_autolock_minutes(&mut self, minutes: u64) -> Result<(), &'static str> {
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

    pub(super) fn poll_mode(&self) -> TuiPollMode {
        self.poll_mode
    }

    pub(super) fn poll_interval_seconds(&self) -> u64 {
        self.poll_interval_seconds
            .clamp(TUI_POLL_MIN_INTERVAL_SECONDS, TUI_POLL_MAX_INTERVAL_SECONDS)
    }

    pub(super) fn poll_interval_ms(&self) -> u64 {
        self.poll_interval_seconds().saturating_mul(1_000)
    }

    pub(super) fn set_poll_mode_adaptive(&mut self) -> Result<(), &'static str> {
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

    pub(super) fn set_poll_mode_fixed(
        &mut self,
        seconds: u64,
        now_ms: u64,
    ) -> Result<(), &'static str> {
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

    pub(super) fn emit_poll_show_marker(&self) {
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

    pub(super) fn mark_input_activity(&mut self, now_ms: u64) {
        self.autolock_last_activity_ms = now_ms;
    }

    pub(super) fn headless_now_ms(&self) -> u64 {
        self.headless_clock_ms
    }

    pub(super) fn current_now_ms(&self) -> u64 {
        self.headless_clock_ms.max(self.autolock_last_activity_ms)
    }

    pub(crate) fn headless_advance_clock(&mut self, delta_ms: u64) {
        self.headless_clock_ms = self.headless_clock_ms.saturating_add(delta_ms);
        self.maybe_autolock(self.headless_clock_ms);
        let _ = self.maybe_run_fixed_poll(self.headless_clock_ms);
    }

    pub(super) fn maybe_autolock(&mut self, now_ms: u64) -> bool {
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

    pub(super) fn take_clear_screen_pending(&mut self) -> bool {
        let pending = self.clear_screen_pending;
        self.clear_screen_pending = false;
        pending
    }

    pub(super) fn take_force_full_redraw(&mut self) -> bool {
        let pending = self.force_full_redraw;
        self.force_full_redraw = false;
        pending
    }

    pub(super) fn clear_ui_buffers_on_lock(&mut self, reason: &'static str) {
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

    pub(super) fn set_locked_state(&mut self, locked: bool, reason: &'static str) {
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

    pub(super) fn last_payload_len(&self) -> usize {
        self.last_payload_len
    }

    pub(super) fn ensure_conversation(&mut self, peer: &str) {
        self.conversations.entry(peer.to_string()).or_default();
        self.visible_counts.entry(peer.to_string()).or_insert(0);
        self.unread_counts.entry(peer.to_string()).or_insert(0);
    }

    pub(super) fn conversation_labels(&self) -> Vec<String> {
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

    pub(super) fn selected_conversation_label(&self) -> String {
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

    pub(super) fn apply_default_account_settings(&mut self) {
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

    pub(super) fn reload_account_settings_from_vault(&mut self) {
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

    pub(super) fn refresh_identity_status(&mut self) {
        let fingerprint = compute_local_fingerprint();
        self.status.fingerprint = Box::leak(fingerprint.into_boxed_str());
        let peer_fp = compute_peer_fingerprint(self.session.peer_label);
        self.status.peer_fp = Box::leak(peer_fp.into_boxed_str());
        let _ = self.refresh_account_cache(self.current_now_ms(), true);
        self.request_redraw();
    }

    pub(super) fn selected_contact_label(&self) -> String {
        if self.contacts.is_empty() {
            "peer-0".to_string()
        } else {
            self.contacts[self
                .contacts_selected
                .min(self.contacts.len().saturating_sub(1))]
            .clone()
        }
    }

    pub(super) fn selected_peer_trust_state(&self) -> &'static str {
        contact_state(self.contact_record_cached(self.session.peer_label))
    }

    pub(super) fn trust_allows_peer_send_strict(&mut self, peer: &str) -> Result<(), &'static str> {
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

    pub(super) fn focus_messages_thread(&mut self, peer: &str) {
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

    pub(super) fn selected_peer_identity_short(&self) -> String {
        self.contact_record_cached(self.session.peer_label)
            .map(|rec| short_identity_display(rec.fp.as_str()))
            .unwrap_or_else(|| "untrusted".to_string())
    }

    pub(crate) fn contact_record_cached(&self, label: &str) -> Option<&ContactRecord> {
        self.contacts_records.get(label)
    }

    pub(super) fn contact_display_line_cached(&self, label: &str) -> String {
        label.to_string()
    }

    pub(super) fn persist_contacts_cache(&mut self) -> Result<(), ErrorCode> {
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

    pub(super) fn persist_contacts_cache_with(
        &mut self,
        label: &str,
        mut rec: ContactRecord,
    ) -> Result<(), ErrorCode> {
        normalize_contact_record(label, &mut rec);
        self.contacts_records.insert(label.to_string(), rec);
        self.persist_contacts_cache()
    }

    pub(super) fn tui_relay_inbox_route_token(&self) -> Result<String, &'static str> {
        // Reuse the shared vault helper so TUI and CLI resolve the persisted inbox
        // token through the same path.
        relay_self_inbox_route_token()
    }

    pub(super) fn tui_timeline_store_load(&self) -> Result<TimelineStore, &'static str> {
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

    pub(super) fn tui_timeline_store_save(
        &mut self,
        store: &TimelineStore,
    ) -> Result<(), &'static str> {
        let json = serde_json::to_string(store).map_err(|_| "timeline_unavailable")?;
        self.persist_account_secret(TIMELINE_SECRET_KEY, json.as_str())
            .map_err(|_| "timeline_unavailable")
    }

    pub(super) fn append_tui_timeline_entry(
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

    pub(super) fn selected_file_id(&self) -> Option<&str> {
        self.files
            .get(self.file_selected.min(self.files.len().saturating_sub(1)))
            .map(|v| v.id.as_str())
    }

    pub(super) fn refresh_file_selection_bounds(&mut self) {
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

    pub(super) fn upsert_file_item(&mut self, item: TuiFileItem, from_update: bool) {
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

    pub(super) fn refresh_files_from_timeline(&mut self) {
        for item in load_tui_files_snapshot() {
            self.upsert_file_item(item, true);
        }
    }

    pub(super) fn files_select_by_id(&mut self, id: &str) -> bool {
        if let Some(idx) = self.files.iter().position(|v| v.id == id) {
            self.file_selected = idx;
            true
        } else {
            false
        }
    }

    pub(super) fn files_toggle_selected(&mut self) -> bool {
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

    pub(super) fn files_move(&mut self, delta: i32) {
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

    pub(super) fn set_active_peer(&mut self, peer: &str) {
        self.session.peer_label = Box::leak(peer.to_string().into_boxed_str());
        self.refresh_qsp_status();
    }

    pub(super) fn sync_messages_if_main_focused(&mut self) {
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

    pub(super) fn sync_files_if_main_focused(&mut self) {
        if self.mode != TuiMode::Normal
            || self.inspector != TuiInspectorPane::Files
            || self.home_focus != TuiHomeFocus::Main
        {
            return;
        }
        self.file_unseen_updates = 0;
    }

    pub(super) fn sync_activity_if_main_focused(&mut self) {
        if self.mode != TuiMode::Normal
            || self.inspector != TuiInspectorPane::Activity
            || self.home_focus != TuiHomeFocus::Main
        {
            return;
        }
        self.activity_visible_count = self.events.len();
        self.activity_unseen_updates = 0;
    }

    pub(super) fn record_message_line(
        &mut self,
        peer: &str,
        state: &str,
        direction: &str,
        detail: &str,
    ) {
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

    pub(super) fn selected_messages_thread(&self) -> Option<String> {
        if self.inspector != TuiInspectorPane::Events {
            return None;
        }
        Some(self.selected_conversation_label())
    }

    pub(super) fn map_thread_to_timeline_peer(thread: &str) -> &str {
        if thread == TUI_NOTE_TO_SELF_LABEL {
            TUI_NOTE_TO_SELF_TIMELINE_PEER
        } else {
            thread
        }
    }

    pub(super) fn update_send_lifecycle(&mut self, value: &str) {
        self.send_lifecycle = value.to_string();
        self.status.send_lifecycle = Box::leak(self.send_lifecycle.clone().into_boxed_str());
        emit_marker(
            "tui_status_update",
            None,
            &[("field", "send_lifecycle"), ("value", value)],
        );
    }

    pub(super) fn refresh_envelope(&mut self, payload_len: usize) {
        self.last_payload_len = payload_len;
        self.envelope = compute_envelope_status(payload_len);
        self.status.envelope = Box::leak(self.envelope.clone().into_boxed_str());
        emit_marker(
            "tui_status_update",
            None,
            &[("field", "envelope"), ("value", &self.envelope)],
        );
    }

    pub(super) fn refresh_qsp_status(&mut self) {
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

    pub(super) fn refresh_contacts(&mut self) {
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

    pub(super) fn push_event(&mut self, kind: &str, action: &str) {
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

    pub(super) fn push_event_line(&mut self, line: String) {
        self.events.push_back(line);
        if self.events.len() > 64 {
            self.events.pop_front();
        }
        self.record_activity_update();
    }

    pub(super) fn record_activity_update(&mut self) {
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

    pub(super) fn enter_help_mode(&mut self) {
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

    pub(super) fn exit_help_mode(&mut self) {
        if self.mode == TuiMode::Help {
            self.mode = TuiMode::Normal;
            emit_marker("tui_help_mode", None, &[("on", "false")]);
        }
    }

    pub(super) fn toggle_help_mode(&mut self) {
        if self.mode == TuiMode::Help {
            self.exit_help_mode();
        } else {
            self.enter_help_mode();
        }
    }

    pub(super) fn focus_pane_name(mode: TuiMode) -> &'static str {
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

    pub(super) fn inspector_name(&self) -> &'static str {
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

    pub(super) fn set_inspector(&mut self, pane: TuiInspectorPane) {
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

    pub(super) fn route_show_to_system_nav(&mut self, pane: TuiInspectorPane) {
        self.set_inspector(pane);
        self.home_focus = TuiHomeFocus::Nav;
        self.cmd_input_clear();
        self.request_redraw();
        emit_marker("tui_focus_home", None, &[("pane", self.home_focus_name())]);
    }

    pub(super) fn focus_mode_for_inspector(&self) -> TuiMode {
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

    pub(super) fn home_layout_snapshot(&self, cols: u16, rows: u16) -> HomeLayoutSnapshot {
        HomeLayoutSnapshot {
            contacts_shown: cols >= TUI_H3_WIDE_MIN,
            header_compact: rows < TUI_H3_TALL_MIN,
        }
    }

    pub(super) fn home_focus_name(&self) -> &'static str {
        match self.home_focus {
            TuiHomeFocus::Nav => "nav",
            TuiHomeFocus::Main => "main",
            TuiHomeFocus::Command => "command",
        }
    }

    pub(super) fn home_focus_label_token(&self) -> &'static str {
        match self.home_focus {
            TuiHomeFocus::Nav => "NAV",
            TuiHomeFocus::Main => "MAIN",
            TuiHomeFocus::Command => "CMD",
        }
    }

    pub(super) fn main_marker_title(&self) -> &'static str {
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

    pub(super) fn home_focus_cycle(&mut self, delta: i32) {
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

    pub(super) fn main_scroll_key(&self) -> &'static str {
        self.inspector.as_name()
    }

    pub(super) fn main_scroll_offset(&self) -> usize {
        self.main_scroll_offsets
            .get(self.main_scroll_key())
            .copied()
            .unwrap_or(0)
    }

    pub(super) fn set_main_scroll_offset(&mut self, value: usize) {
        let key = self.main_scroll_key();
        if value == 0 {
            self.main_scroll_offsets.remove(key);
        } else {
            self.main_scroll_offsets.insert(key, value);
        }
    }

    pub(super) fn update_main_scroll_metrics(&mut self, content_lines: usize, view_rows: usize) {
        self.main_view_rows_current = view_rows.max(1);
        self.main_scroll_max_current = content_lines.saturating_sub(self.main_view_rows_current);
        let clamped = self.main_scroll_offset().min(self.main_scroll_max_current);
        self.set_main_scroll_offset(clamped);
    }

    pub(super) fn main_view_rows(&self) -> usize {
        usize::from(terminal_rows_for_headless().saturating_sub(5)).max(1)
    }

    pub(super) fn estimated_main_line_count(&self) -> usize {
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

    pub(super) fn ensure_main_scroll_metrics(&mut self) {
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

    pub(super) fn emit_main_scroll_marker(&self) {
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

    pub(super) fn main_scroll_move(&mut self, delta: i32) {
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

    pub(super) fn main_scroll_page(&mut self, direction: i32) {
        self.ensure_main_scroll_metrics();
        let page = self.main_view_rows_current.max(1) as i32;
        self.main_scroll_move(direction.saturating_mul(page));
    }

    pub(super) fn main_scroll_home(&mut self) {
        self.ensure_main_scroll_metrics();
        self.set_main_scroll_offset(0);
        self.request_redraw();
        self.emit_main_scroll_marker();
    }

    pub(super) fn main_scroll_end(&mut self) {
        self.ensure_main_scroll_metrics();
        self.set_main_scroll_offset(self.main_scroll_max_current);
        self.request_redraw();
        self.emit_main_scroll_marker();
    }

    pub(super) fn emit_home_render_marker(&self, cols: u16, rows: u16) {
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

    pub(super) fn focus_render_count(&self, mode: TuiMode) -> usize {
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

    pub(super) fn enter_focus_mode(&mut self, mode: TuiMode) {
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

    pub(super) fn exit_focus_mode(&mut self) {
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

    pub(super) fn is_help_mode(&self) -> bool {
        self.mode == TuiMode::Help
    }

    pub(super) fn is_focus_mode(&self) -> bool {
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

    pub(super) fn focus_max_len(&self) -> usize {
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

    pub(super) fn focus_view_rows(&self) -> usize {
        usize::from(terminal_rows_for_headless().saturating_sub(2)).max(1)
    }

    pub(super) fn focus_scroll_index(&self) -> usize {
        match self.mode {
            TuiMode::FocusContacts => self.contacts_selected,
            TuiMode::FocusFiles => self.file_selected,
            _ => self.focus_scroll,
        }
    }

    pub(super) fn focus_events_lines(&self) -> Vec<String> {
        self.events
            .iter()
            .enumerate()
            .map(|(i, line)| format!("{} {}", tui_timestamp_token(i), line))
            .collect()
    }

    pub(super) fn focus_activity_lines(&self) -> Vec<String> {
        self.focus_events_lines()
    }

    pub(super) fn focus_status_lines(&self) -> Vec<String> {
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

    pub(super) fn focus_session_lines(&self) -> Vec<String> {
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

    pub(super) fn focus_contacts_lines(&self) -> Vec<String> {
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

    pub(super) fn focus_files_lines(&self) -> Vec<String> {
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

    pub(super) fn focus_settings_lines(&self) -> Vec<String> {
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

    pub(super) fn focus_lock_lines(&self) -> Vec<String> {
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

    pub(super) fn emit_focus_render_marker(&self) {
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

    pub(super) fn help_selected_item(&self) -> Option<&'static TuiHelpItem> {
        let items = tui_help_items();
        if items.is_empty() {
            None
        } else {
            Some(&items[self.help_selected.min(items.len() - 1)])
        }
    }

    pub(super) fn help_move(&mut self, delta: i32) {
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

    pub(super) fn focus_scroll_move(&mut self, delta: i32, max_len: usize) {
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

    pub(super) fn contacts_move(&mut self, delta: i32) {
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

    pub(super) fn nav_move(&mut self, delta: i32) {
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

    pub(super) fn nav_activate(&mut self) {
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

    pub(super) fn locked_nav_activate(&mut self) -> bool {
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

    pub(super) fn nav_preview_select(&mut self, kind: NavRowKind) {
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

    pub(super) fn pane_domain_name(pane: TuiInspectorPane) -> &'static str {
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

    pub(super) fn nav_row_label(&self, row: &NavRow) -> String {
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

    pub(super) fn expanded_nav_domain(&self) -> Option<TuiNavDomain> {
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

    pub(super) fn nav_rows(&self) -> Vec<NavRow> {
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

    pub(super) fn sync_nav_to_inspector_header(&mut self) {
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

    pub(super) fn drain_marker_queue(&mut self) {
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

pub(super) fn tui_file_display_state(raw: &str) -> String {
    let upper = raw.trim().to_ascii_uppercase();
    match upper.as_str() {
        "VERIFIED" | "COMPLETE" => "VERIFIED".to_string(),
        "FAILED" | "REJECTED" => "FAILED".to_string(),
        "RECEIVING" | "CREATED" | "SENT" | "ANNOUNCED" | "PENDING" => "RECEIVING".to_string(),
        _ => upper,
    }
}

pub(super) fn load_tui_files_snapshot() -> Vec<TuiFileItem> {
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
