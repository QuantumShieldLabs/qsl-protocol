use crate::*;

pub(super) fn load_tui_script() -> Vec<String> {
    if let Ok(path) = env::var("QSC_TUI_SCRIPT_FILE") {
        if let Ok(text) = fs::read_to_string(path) {
            return parse_script_lines(&text);
        }
    }
    if let Ok(text) = env::var("QSC_TUI_SCRIPT") {
        return parse_script_lines(&text);
    }
    vec!["/exit".to_string()]
}

pub(super) fn parse_script_lines(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in text.lines() {
        for part in line.split(';') {
            let trimmed = part.trim();
            if !trimmed.is_empty() {
                out.push(trimmed.to_string());
            }
        }
    }
    out
}

pub(super) struct TuiParsedCmd {
    pub(super) cmd: String,
    pub(super) args: Vec<String>,
}

pub(super) fn parse_tui_command(line: &str) -> Option<TuiParsedCmd> {
    let trimmed = line.trim();
    if !trimmed.starts_with('/') {
        return None;
    }
    let parts = parse_tui_command_tokens(trimmed.trim_start_matches('/'));
    let cmd = parts.first()?;
    if cmd.is_empty() {
        return None;
    }
    let args = parts.iter().skip(1).cloned().collect::<Vec<_>>();
    Some(TuiParsedCmd {
        cmd: cmd.clone(),
        args,
    })
}

pub(super) fn parse_tui_command_tokens(input: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut buf = String::new();
    let mut chars = input.chars().peekable();
    let mut in_quotes = false;
    while let Some(ch) = chars.next() {
        match ch {
            '"' => {
                in_quotes = !in_quotes;
            }
            '\\' if in_quotes => {
                if let Some(next) = chars.next() {
                    buf.push(next);
                }
            }
            c if c.is_whitespace() && !in_quotes => {
                if !buf.is_empty() {
                    out.push(std::mem::take(&mut buf));
                }
            }
            _ => buf.push(ch),
        }
    }
    if !buf.is_empty() {
        out.push(buf);
    }
    out
}

pub(super) fn parse_tui_wait_ms(line: &str) -> Option<u64> {
    let mut parts = line.split_whitespace();
    let head = parts.next()?;
    if !head.eq_ignore_ascii_case("wait") {
        return None;
    }
    let ms = parts.next()?.parse::<u64>().ok()?;
    if parts.next().is_some() {
        return None;
    }
    Some(ms)
}

pub(super) fn parse_tui_perf_snapshot(line: &str) -> Option<String> {
    let mut parts = line.split_whitespace();
    let head = parts.next()?;
    if !head.eq_ignore_ascii_case("perf") {
        return None;
    }
    let action = parts.next()?;
    if !action.eq_ignore_ascii_case("snapshot") {
        return None;
    }
    let tag = parts.next().unwrap_or("default");
    if parts.next().is_some() {
        return None;
    }
    Some(tag.to_string())
}

pub(super) fn parse_tui_script_key(spec: &str) -> Option<KeyEvent> {
    let raw = spec.trim();
    let normalized = raw.to_ascii_lowercase();
    match normalized.as_str() {
        "esc" => Some(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)),
        "enter" => Some(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)),
        "tab" => Some(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE)),
        "shift-tab" | "s-tab" | "backtab" => {
            Some(KeyEvent::new(KeyCode::BackTab, KeyModifiers::SHIFT))
        }
        "up" => Some(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE)),
        "down" => Some(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE)),
        "pgup" | "pageup" => Some(KeyEvent::new(KeyCode::PageUp, KeyModifiers::NONE)),
        "pgdn" | "pagedown" => Some(KeyEvent::new(KeyCode::PageDown, KeyModifiers::NONE)),
        "home" => Some(KeyEvent::new(KeyCode::Home, KeyModifiers::NONE)),
        "end" => Some(KeyEvent::new(KeyCode::End, KeyModifiers::NONE)),
        "f2" => Some(KeyEvent::new(KeyCode::F(2), KeyModifiers::NONE)),
        "f3" => Some(KeyEvent::new(KeyCode::F(3), KeyModifiers::NONE)),
        "f4" => Some(KeyEvent::new(KeyCode::F(4), KeyModifiers::NONE)),
        "f5" => Some(KeyEvent::new(KeyCode::F(5), KeyModifiers::NONE)),
        "ctrl-f2" | "c-f2" => Some(KeyEvent::new(KeyCode::F(2), KeyModifiers::CONTROL)),
        "ctrl-f3" | "c-f3" => Some(KeyEvent::new(KeyCode::F(3), KeyModifiers::CONTROL)),
        "ctrl-f4" | "c-f4" => Some(KeyEvent::new(KeyCode::F(4), KeyModifiers::CONTROL)),
        "ctrl-f5" | "c-f5" => Some(KeyEvent::new(KeyCode::F(5), KeyModifiers::CONTROL)),
        "ctrl-l" | "c-l" => Some(KeyEvent::new(KeyCode::Char('l'), KeyModifiers::CONTROL)),
        "slash" => Some(KeyEvent::new(KeyCode::Char('/'), KeyModifiers::NONE)),
        "space" => Some(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE)),
        "backspace" => Some(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE)),
        "delete" | "del" => Some(KeyEvent::new(KeyCode::Delete, KeyModifiers::NONE)),
        _ => {
            let mut chars = raw.chars();
            let ch = chars.next()?;
            if chars.next().is_none() && !ch.is_control() {
                Some(KeyEvent::new(KeyCode::Char(ch), KeyModifiers::NONE))
            } else {
                None
            }
        }
    }
}
