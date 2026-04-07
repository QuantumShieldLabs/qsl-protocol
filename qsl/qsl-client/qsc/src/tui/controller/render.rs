use super::super::render::*;
use super::*;

pub(super) fn draw_tui(f: &mut ratatui::Frame, state: &mut TuiState) {
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

pub(super) struct TuiHelpItem {
    pub(super) cmd: &'static str,
    pub(super) desc: &'static str,
}

pub(super) fn tui_help_items() -> &'static [TuiHelpItem] {
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
