use super::super::commands::tui_receive_via_relay;
use super::super::*;

impl TuiState {
    pub(in super::super) fn effective_relay_config(&self) -> Option<TuiRelayConfig> {
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

    pub(in super::super) fn finish_relay_test_task(&mut self, outcome: RelayTestOutcome) {
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

    pub(in super::super) fn poll_relay_test_task(&mut self) {
        let outcome = self
            .relay_test_task
            .as_ref()
            .and_then(|rx| rx.try_recv().ok());
        let Some(outcome) = outcome else {
            return;
        };
        self.finish_relay_test_task(outcome);
    }

    pub(in super::super) fn wait_for_relay_test_task_headless(&mut self) {
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

    pub(in super::super) fn set_autolock_minutes(
        &mut self,
        minutes: u64,
    ) -> Result<(), &'static str> {
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

    pub(in super::super) fn poll_mode(&self) -> TuiPollMode {
        self.poll_mode
    }

    pub(in super::super) fn poll_interval_seconds(&self) -> u64 {
        self.poll_interval_seconds
            .clamp(TUI_POLL_MIN_INTERVAL_SECONDS, TUI_POLL_MAX_INTERVAL_SECONDS)
    }

    pub(in super::super) fn poll_interval_ms(&self) -> u64 {
        self.poll_interval_seconds().saturating_mul(1_000)
    }

    pub(in super::super) fn set_poll_mode_adaptive(&mut self) -> Result<(), &'static str> {
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

    pub(in super::super) fn set_poll_mode_fixed(
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

    pub(in super::super) fn emit_poll_show_marker(&self) {
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

    pub(in super::super) fn mark_input_activity(&mut self, now_ms: u64) {
        self.autolock_last_activity_ms = now_ms;
    }

    pub(in super::super) fn headless_now_ms(&self) -> u64 {
        self.headless_clock_ms
    }

    pub(in super::super) fn current_now_ms(&self) -> u64 {
        self.headless_clock_ms.max(self.autolock_last_activity_ms)
    }

    pub(crate) fn headless_advance_clock(&mut self, delta_ms: u64) {
        self.headless_clock_ms = self.headless_clock_ms.saturating_add(delta_ms);
        self.maybe_autolock(self.headless_clock_ms);
        let _ = self.maybe_run_fixed_poll(self.headless_clock_ms);
    }

    pub(in super::super) fn maybe_autolock(&mut self, now_ms: u64) -> bool {
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
}
