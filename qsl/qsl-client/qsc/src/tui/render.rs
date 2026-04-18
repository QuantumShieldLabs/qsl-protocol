use crate::*;

pub(super) fn internal_divider_style() -> Style {
    Style::default()
        .fg(Color::DarkGray)
        .add_modifier(Modifier::DIM)
}

pub(super) fn render_header_divider(f: &mut Frame, area: Rect) {
    if area.width == 0 || area.height == 0 {
        return;
    }
    let body = "─".repeat(area.width as usize);
    let line = Line::from(vec![Span::styled(body, internal_divider_style())]);
    f.render_widget(Paragraph::new(line), area);
}

pub(super) fn render_vertical_divider(f: &mut Frame, area: Rect) {
    if area.width == 0 || area.height == 0 {
        return;
    }
    let body = std::iter::repeat_n("│", area.height as usize)
        .collect::<Vec<_>>()
        .join("\n");
    f.render_widget(Paragraph::new(body).style(internal_divider_style()), area);
}

pub(super) fn render_horizontal_divider(f: &mut Frame, area: Rect) {
    if area.width == 0 || area.height == 0 {
        return;
    }
    let body = "─".repeat(area.width as usize);
    f.render_widget(Paragraph::new(body).style(internal_divider_style()), area);
}

pub(super) fn pad_panel_text(text: &str) -> String {
    let pad = " ".repeat(PANEL_INNER_PAD);
    text.lines()
        .map(|line| {
            if line.is_empty() {
                String::new()
            } else {
                format!("{}{}", pad, line)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub(super) fn truncate_with_ellipsis(value: &str, width: usize) -> String {
    if width == 0 {
        return String::new();
    }
    let char_count = value.chars().count();
    if char_count <= width {
        return value.to_string();
    }
    if width == 1 {
        return "…".to_string();
    }
    let prefix = value.chars().take(width - 1).collect::<String>();
    format!("{prefix}…")
}

pub(super) fn format_contacts_table_row(
    alias: &str,
    trust: &str,
    blocked: &str,
    last_seen: &str,
) -> String {
    format!(
        "{:<alias_w$} {:<trust_w$} {:<blocked_w$} {}",
        truncate_with_ellipsis(alias, CONTACTS_COL_ALIAS_WIDTH),
        truncate_with_ellipsis(trust, CONTACTS_COL_TRUST_WIDTH),
        truncate_with_ellipsis(blocked, CONTACTS_COL_BLOCKED_WIDTH),
        last_seen,
        alias_w = CONTACTS_COL_ALIAS_WIDTH,
        trust_w = CONTACTS_COL_TRUST_WIDTH,
        blocked_w = CONTACTS_COL_BLOCKED_WIDTH
    )
}
