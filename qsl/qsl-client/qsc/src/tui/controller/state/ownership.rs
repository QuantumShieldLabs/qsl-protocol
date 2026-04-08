use super::super::commands::wipe_account_local_state_best_effort;
use super::super::*;

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
            files: super::load_tui_files_snapshot(),
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

    pub(in super::super) fn is_locked(&self) -> bool {
        self.vault_locked
    }

    pub(in super::super) fn has_vault(&self) -> bool {
        self.vault_present
    }

    pub(in super::super) fn mark_vault_present(&mut self) {
        self.vault_present = true;
    }

    pub(in super::super) fn mark_vault_absent(&mut self) {
        self.vault_present = false;
        vault::set_process_passphrase(None);
    }

    pub(in super::super) fn reload_unlock_security_state(&mut self) {
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

    pub(in super::super) fn persist_unlock_security_state(&self) -> Result<(), &'static str> {
        let state = VaultSecurityState {
            attempt_limit: self.unlock_attempt_limit,
            failed_unlocks: self.failed_unlock_attempts,
        };
        vault_security_state_store(&state).map_err(|_| "vault_attempt_limit_io")
    }

    pub(in super::super) fn set_unlock_attempt_limit(
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

    pub(in super::super) fn reset_unlock_failure_counter(&mut self) {
        if self.failed_unlock_attempts == 0 {
            return;
        }
        self.failed_unlock_attempts = 0;
        let _ = self.persist_unlock_security_state();
    }

    pub(in super::super) fn wipe_after_failed_unlock_limit(&mut self) -> Result<(), &'static str> {
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

    pub(in super::super) fn record_unlock_failure_and_maybe_wipe(
        &mut self,
    ) -> UnlockAttemptOutcome {
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

    pub(in super::super) fn unlock_with_policy(
        &mut self,
        passphrase: &str,
    ) -> UnlockAttemptOutcome {
        let unlocked = vault::unlock_with_passphrase(passphrase).is_ok()
            && self.open_vault_session(Some(passphrase)).is_ok();
        if unlocked {
            vault::set_process_passphrase(Some(passphrase));
            self.reset_unlock_failure_counter();
            return UnlockAttemptOutcome::Unlocked;
        }
        self.record_unlock_failure_and_maybe_wipe()
    }

    pub(in super::super) fn cmd_input_clear(&mut self) {
        self.cmd_input.clear();
    }

    pub(in super::super) fn cmd_input_push(&mut self, ch: char) {
        self.cmd_input.push(ch);
    }

    pub(in super::super) fn cmd_input_pop(&mut self) {
        self.cmd_input.pop();
    }

    pub(in super::super) fn locked_flow_name(&self) -> &'static str {
        match self.locked_flow {
            LockedFlow::None => "none",
            LockedFlow::UnlockPassphrase => "unlock_passphrase",
            LockedFlow::InitAlias => "init_alias",
            LockedFlow::InitPassphrase { .. } => "init_passphrase",
            LockedFlow::InitConfirm { .. } => "init_confirm",
            LockedFlow::InitDecision { .. } => "init_decision",
        }
    }

    pub(in super::super) fn locked_wizard_step_label(&self) -> Option<&'static str> {
        match self.locked_flow {
            LockedFlow::None => None,
            LockedFlow::UnlockPassphrase => Some("Passphrase"),
            LockedFlow::InitAlias => Some("Alias"),
            LockedFlow::InitPassphrase { .. } => Some("Passphrase"),
            LockedFlow::InitConfirm { .. } => Some("Confirm"),
            LockedFlow::InitDecision { .. } => Some("Confirm (I AGREE/N)"),
        }
    }

    pub(in super::super) fn account_destroy_step_label(&self) -> Option<&'static str> {
        match self.account_destroy_flow {
            AccountDestroyFlow::None => None,
            AccountDestroyFlow::Passphrase => Some("Passphrase"),
            AccountDestroyFlow::ConfirmDecision { .. } => Some("Confirm (Y/N)"),
        }
    }

    pub(in super::super) fn account_destroy_set_error(&mut self, message: impl Into<String>) {
        self.account_destroy_error = Some(message.into());
    }

    pub(in super::super) fn account_destroy_clear_error(&mut self) {
        self.account_destroy_error = None;
    }

    pub(in super::super) fn account_destroy_active(&self) -> bool {
        !matches!(self.account_destroy_flow, AccountDestroyFlow::None)
    }

    pub(in super::super) fn locked_set_error(&mut self, message: impl Into<String>) {
        self.locked_error = Some(message.into());
    }

    pub(in super::super) fn locked_clear_error(&mut self) {
        self.locked_error = None;
    }

    pub(in super::super) fn set_command_error(&mut self, message: impl Into<String>) {
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

    pub(in super::super) fn clear_command_error(&mut self) {
        self.command_error = None;
        self.request_redraw();
    }

    pub(in super::super) fn set_command_feedback(&mut self, message: impl Into<String>) {
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

    pub(in super::super) fn clear_command_feedback(&mut self) {
        self.command_feedback = None;
        self.request_redraw();
    }

    pub(in super::super) fn request_redraw(&mut self) {
        self.needs_redraw = true;
    }

    pub(in super::super) fn refresh_account_cache(&mut self, now_ms: u64, force: bool) -> bool {
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

    pub(in super::super) fn set_status_last_command_result(&mut self, message: impl Into<String>) {
        let message = message.into();
        self.status_last_command_result = Some(message.clone());
        let _ = self.persist_account_secret(TUI_LAST_COMMAND_RESULT_SECRET_KEY, message.as_str());
        self.request_redraw();
    }

    pub(in super::super) fn status_last_command_result_text(&self) -> &str {
        self.status_last_command_result.as_deref().unwrap_or("none")
    }

    pub(in super::super) fn push_cmd_result(
        &mut self,
        command: &str,
        ok: bool,
        message: impl Into<String>,
    ) {
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

    pub(in super::super) fn begin_command_tracking(&mut self, command: impl Into<String>) {
        self.active_command_label = Some(command.into());
        self.active_command_result_recorded = false;
    }

    pub(in super::super) fn end_command_tracking(&mut self) {
        self.active_command_label = None;
        self.active_command_result_recorded = false;
    }

    pub(in super::super) fn locked_cmd_masked(&self) -> bool {
        matches!(
            self.locked_flow,
            LockedFlow::UnlockPassphrase
                | LockedFlow::InitPassphrase { .. }
                | LockedFlow::InitConfirm { .. }
        )
    }

    pub(in super::super) fn account_destroy_cmd_masked(&self) -> bool {
        matches!(self.account_destroy_flow, AccountDestroyFlow::Passphrase)
    }

    pub(in super::super) fn cmd_display_value(&self) -> String {
        if self.locked_cmd_masked() || self.account_destroy_cmd_masked() {
            "•".repeat(self.cmd_input.chars().count())
        } else {
            self.cmd_input.clone()
        }
    }

    pub(in super::super) fn cmd_bar_text(&self) -> String {
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

    pub(in super::super) fn accent_color_enabled(&self) -> bool {
        tui_color_enabled()
    }

    pub(in super::super) fn cmd_bar_style(&self, text: &str) -> Style {
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

    pub(in super::super) fn locked_main_lines(&self) -> Vec<String> {
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

    pub(in super::super) fn locked_main_body(&self) -> String {
        self.locked_main_lines().join("\n")
    }

    pub(in super::super) fn start_unlock_prompt(&mut self) {
        self.home_focus = TuiHomeFocus::Command;
        self.locked_flow = LockedFlow::UnlockPassphrase;
        self.cmd_input_clear();
        self.locked_clear_error();
        emit_marker("tui_unlock_prompt", None, &[("step", "passphrase")]);
        emit_marker("tui_focus_home", None, &[("pane", self.home_focus_name())]);
    }

    pub(in super::super) fn start_init_prompt(&mut self) {
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

    pub(in super::super) fn start_account_destroy_prompt(&mut self) {
        self.inspector = TuiInspectorPane::Account;
        self.sync_nav_to_inspector_header();
        self.home_focus = TuiHomeFocus::Command;
        self.account_destroy_flow = AccountDestroyFlow::Passphrase;
        self.cmd_input_clear();
        self.account_destroy_clear_error();
        emit_marker("tui_account_destroy", None, &[("step", "passphrase")]);
        emit_marker("tui_focus_home", None, &[("pane", self.home_focus_name())]);
    }

    pub(in super::super) fn cancel_account_destroy_prompt(&mut self) {
        self.account_destroy_flow = AccountDestroyFlow::None;
        self.account_destroy_clear_error();
        self.cmd_input_clear();
        self.home_focus = TuiHomeFocus::Nav;
        emit_marker("tui_account_destroy", None, &[("step", "cancel")]);
        emit_marker("tui_focus_home", None, &[("pane", self.home_focus_name())]);
    }

    pub(in super::super) fn locked_focus_toggle(&mut self) {
        self.home_focus = if self.home_focus == TuiHomeFocus::Command {
            TuiHomeFocus::Nav
        } else {
            TuiHomeFocus::Command
        };
        emit_marker("tui_focus_home", None, &[("pane", self.home_focus_name())]);
    }

    pub(in super::super) fn autolock_minutes(&self) -> u64 {
        let minutes = self.autolock_timeout_ms / 60_000;
        minutes.clamp(TUI_AUTOLOCK_MIN_MINUTES, TUI_AUTOLOCK_MAX_MINUTES)
    }

    pub(in super::super) fn open_vault_session(
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

    pub(in super::super) fn close_vault_session(&mut self) {
        self.vault_session = None;
    }

    pub(in super::super) fn persist_account_secret(
        &mut self,
        key: &str,
        value: &str,
    ) -> Result<(), &'static str> {
        if let Some(session) = self.vault_session.as_mut() {
            return vault::session_set(session, key, value);
        }
        Err("vault_locked")
    }

    pub(in super::super) fn read_account_secret(&self, key: &str) -> Option<String> {
        self.vault_session
            .as_ref()
            .and_then(|session| vault::session_get(session, key).ok().flatten())
    }

    pub(in super::super) fn relay_endpoint_redacted(&self) -> String {
        match self.relay_endpoint_hash_cache.as_ref() {
            Some(hash) => format!("set (hash: {})", hash),
            None => "unset".to_string(),
        }
    }

    pub(in super::super) fn relay_auth_label(&self) -> &'static str {
        if self.relay_token_set_cache {
            "bearer token (set)"
        } else if self.relay_token_file_cache.is_some() {
            "bearer token file (set)"
        } else {
            "none (optional bearer token)"
        }
    }

    pub(in super::super) fn relay_token_file_redacted(&self) -> String {
        match self.relay_token_file_hash_cache.as_ref() {
            Some(hash) => format!("set (hash: {})", hash),
            None => "unset".to_string(),
        }
    }

    pub(in super::super) fn relay_token_file_status(&self) -> (&'static str, &'static str) {
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

    pub(in super::super) fn relay_inbox_token_redacted(&self) -> String {
        match self.relay_inbox_token_hash_cache.as_ref() {
            Some(hash) => format!("set (hash: {})", hash),
            None => "unset".to_string(),
        }
    }

    pub(in super::super) fn relay_setup_status(&self) -> (&'static str, &'static str) {
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

    pub(in super::super) fn emit_setup_required_marker_if_needed(&self) {
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

    pub(in super::super) fn set_relay_endpoint(&mut self, value: &str) -> Result<(), &'static str> {
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

    pub(in super::super) fn set_relay_token(&mut self, value: &str) -> Result<(), &'static str> {
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

    pub(in super::super) fn set_relay_token_file(
        &mut self,
        value: &str,
    ) -> Result<(), &'static str> {
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

    pub(in super::super) fn set_relay_inbox_token(
        &mut self,
        value: &str,
    ) -> Result<(), &'static str> {
        let token = normalize_route_token(value)?;
        self.persist_account_secret(TUI_RELAY_INBOX_TOKEN_SECRET_KEY, token.as_str())
            .map_err(|_| "relay_config_unavailable")?;
        self.relay_inbox_token_set_cache = true;
        self.relay_inbox_token_hash_cache = Some(route_token_hash8(token.as_str()));
        self.request_redraw();
        Ok(())
    }

    pub(in super::super) fn clear_relay_inbox_token(&mut self) -> Result<(), &'static str> {
        self.persist_account_secret(TUI_RELAY_INBOX_TOKEN_SECRET_KEY, "")
            .map_err(|_| "relay_config_unavailable")?;
        self.relay_inbox_token_set_cache = false;
        self.relay_inbox_token_hash_cache = None;
        self.request_redraw();
        Ok(())
    }

    pub(in super::super) fn clear_relay_config(&mut self) -> Result<(), &'static str> {
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

    pub(in super::super) fn take_clear_screen_pending(&mut self) -> bool {
        let pending = self.clear_screen_pending;
        self.clear_screen_pending = false;
        pending
    }

    pub(in super::super) fn take_force_full_redraw(&mut self) -> bool {
        let pending = self.force_full_redraw;
        self.force_full_redraw = false;
        pending
    }

    pub(in super::super) fn clear_ui_buffers_on_lock(&mut self, reason: &'static str) {
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

    pub(in super::super) fn set_locked_state(&mut self, locked: bool, reason: &'static str) {
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

    pub(in super::super) fn last_payload_len(&self) -> usize {
        self.last_payload_len
    }

    pub(in super::super) fn ensure_conversation(&mut self, peer: &str) {
        self.conversations.entry(peer.to_string()).or_default();
        self.visible_counts.entry(peer.to_string()).or_insert(0);
        self.unread_counts.entry(peer.to_string()).or_insert(0);
    }

    pub(in super::super) fn conversation_labels(&self) -> Vec<String> {
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

    pub(in super::super) fn selected_conversation_label(&self) -> String {
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

    pub(in super::super) fn apply_default_account_settings(&mut self) {
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

    pub(in super::super) fn reload_account_settings_from_vault(&mut self) {
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

    pub(in super::super) fn refresh_identity_status(&mut self) {
        let fingerprint = compute_local_fingerprint();
        self.status.fingerprint = Box::leak(fingerprint.into_boxed_str());
        let peer_fp = compute_peer_fingerprint(self.session.peer_label);
        self.status.peer_fp = Box::leak(peer_fp.into_boxed_str());
        let _ = self.refresh_account_cache(self.current_now_ms(), true);
        self.request_redraw();
    }
}
