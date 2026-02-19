use assert_cmd::Command as AssertCommand;

fn run_headless(script: &str, unlocked: bool, cols: &str, rows: &str) -> String {
    let mut cmd = AssertCommand::new(assert_cmd::cargo::cargo_bin!("qsc"));
    let out = cmd
        .env("QSC_TUI_HEADLESS", "1")
        .env("QSC_TUI_TEST_UNLOCK", if unlocked { "1" } else { "0" })
        .env("QSC_TUI_DETERMINISTIC", "1")
        .env("QSC_TUI_SCRIPT", script)
        .env("QSC_TUI_COLS", cols)
        .env("QSC_TUI_ROWS", rows)
        .env("QSC_MARK_FORMAT", "plain")
        .args(["tui"])
        .output()
        .expect("run tui headless");
    let mut combined = String::from_utf8_lossy(&out.stdout).to_string();
    combined.push_str(&String::from_utf8_lossy(&out.stderr));
    combined
}

fn marker_line_positions(out: &str, needle: &str) -> Vec<usize> {
    out.lines()
        .enumerate()
        .filter_map(|(idx, line)| line.contains(needle).then_some(idx))
        .collect()
}

fn parse_selected_indices(out: &str) -> Vec<usize> {
    out.lines()
        .filter(|line| line.contains("event=tui_nav_render"))
        .filter_map(|line| {
            line.split_whitespace().find_map(|tok| {
                tok.strip_prefix("selected_index=")
                    .and_then(|v| v.parse::<usize>().ok())
            })
        })
        .collect()
}

fn parse_main_scroll_offsets(out: &str, inspector: &str) -> Vec<usize> {
    let marker = format!("event=tui_main_scroll inspector={}", inspector);
    out.lines()
        .filter(|line| line.contains(marker.as_str()))
        .filter_map(|line| {
            line.split_whitespace().find_map(|tok| {
                tok.strip_prefix("offset=")
                    .and_then(|v| v.parse::<usize>().ok())
            })
        })
        .collect()
}

#[test]
fn tab_cycles_focus_nav_main_cmd() {
    let out = run_headless("/key tab;/key tab;/key tab;/exit", true, "120", "30");
    let main = marker_line_positions(&out, "event=tui_focus_home pane=main");
    let cmd = marker_line_positions(&out, "event=tui_focus_home pane=command");
    let nav = marker_line_positions(&out, "event=tui_focus_home pane=nav");
    assert!(!main.is_empty(), "missing main focus marker: {}", out);
    assert!(!cmd.is_empty(), "missing command focus marker: {}", out);
    assert!(!nav.is_empty(), "missing nav focus marker: {}", out);
    let first_main = main[0];
    let first_cmd = cmd[0];
    let nav_after_cmd = nav.into_iter().find(|idx| *idx > first_cmd);
    assert!(
        first_main < first_cmd,
        "expected tab focus order nav->main->command: {}",
        out
    );
    assert!(
        nav_after_cmd.is_some(),
        "expected final tab to return focus to nav: {}",
        out
    );
}

#[test]
fn main_scrolls_only_when_main_focused() {
    let out_nav = run_headless("/inspector help;/key down;/exit", true, "120", "14");
    assert!(
        !out_nav.contains("event=tui_main_scroll inspector=help"),
        "nav-focused down must not scroll main: {}",
        out_nav
    );
    let nav_indices = parse_selected_indices(&out_nav);
    assert!(
        nav_indices.len() >= 2 && nav_indices.first() != nav_indices.last(),
        "expected nav selection to move while nav focused: {}",
        out_nav
    );

    let out_main = run_headless(
        "/inspector help;/key tab;/key down;/exit",
        true,
        "120",
        "14",
    );
    assert!(
        out_main.contains("event=tui_focus_home pane=main"),
        "missing main focus marker: {}",
        out_main
    );
    assert!(
        out_main.contains("event=tui_main_scroll inspector=help offset=1"),
        "main-focused down must scroll content: {}",
        out_main
    );
    let main_indices = parse_selected_indices(&out_main);
    assert!(
        main_indices.len() >= 2
            && main_indices[main_indices.len().saturating_sub(1)]
                == main_indices[main_indices.len().saturating_sub(2)],
        "nav selection must remain unchanged while main focused: {}",
        out_main
    );
}

#[test]
fn scroll_clamps_and_home_end_work() {
    let out = run_headless(
        "/inspector help;/key tab;/key end;/key pgdn;/key home;/exit",
        true,
        "120",
        "10",
    );
    let offsets = parse_main_scroll_offsets(&out, "help");
    assert!(
        offsets.len() >= 3,
        "expected scroll markers for end/pgdn/home: {}",
        out
    );
    let max_seen = offsets.iter().copied().max().unwrap_or(0);
    assert!(
        max_seen > 0,
        "expected a non-zero max scroll offset: {}",
        out
    );
    assert_eq!(
        offsets.last().copied().unwrap_or(usize::MAX),
        0,
        "home key must reset scroll to top: {}",
        out
    );
}

#[test]
fn locked_mode_tab_does_not_enter_main() {
    let out = run_headless("/key tab;/key tab;/exit", false, "120", "30");
    assert!(
        !out.contains("event=tui_focus_home pane=main"),
        "locked mode must not enter main focus: {}",
        out
    );
    assert!(
        out.contains("event=tui_focus_home pane=command")
            && out.contains("event=tui_focus_home pane=nav"),
        "locked mode tab should only toggle nav and command focus: {}",
        out
    );
}
