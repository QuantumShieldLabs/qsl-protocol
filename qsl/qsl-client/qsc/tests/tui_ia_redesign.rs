use assert_cmd::Command as AssertCommand;

fn run_headless(script: &str) -> String {
    let mut cmd = AssertCommand::new(assert_cmd::cargo::cargo_bin!("qsc"));
    let out = cmd
        .env("QSC_TUI_HEADLESS", "1")
        .env("QSC_TUI_TEST_UNLOCK", "1")
        .env("QSC_TUI_SCRIPT", script)
        .env("QSC_TUI_COLS", "140")
        .env("QSC_TUI_ROWS", "40")
        .env("QSC_MARK_FORMAT", "plain")
        .args(["tui"])
        .output()
        .expect("run tui headless");
    let mut combined = String::from_utf8_lossy(&out.stdout).to_string();
    combined.push_str(&String::from_utf8_lossy(&out.stderr));
    combined
}

#[test]
fn nav_has_no_parenthetical_counters() {
    let out = run_headless("/inspector status;/inspector events;/inspector contacts;/exit");
    assert!(
        out.contains("event=tui_nav_render") && out.contains("counters=none"),
        "nav marker must assert no counters mode: {}",
        out
    );
    assert!(
        !out.contains("Messages (") && !out.contains("Contacts (") && !out.contains("Activity ("),
        "nav should not render parenthetical counters: {}",
        out
    );
}

#[test]
fn system_subnav_exists_and_routing_works() {
    let out = run_headless("/inspector settings;/status;/poll show;/autolock show;/exit");
    assert!(
        out.contains("event=tui_inspector pane=status")
            && out.contains("event=tui_focus_home pane=nav")
            && out.contains("event=tui_nav_render selected_markers=1 selected_index=1"),
        "/status should route to system overview and focus nav on Overview: {}",
        out
    );
    assert!(
        out.contains("event=tui_inspector pane=cmd_results")
            && out.contains("event=tui_nav_render selected_markers=1 selected_index=3"),
        "show commands should route to system cmd_results and focus nav there: {}",
        out
    );
}

#[test]
fn command_results_history_appends() {
    let out = run_headless("/poll show;/poll set fixed 0;/inspector cmdresults;/exit");
    assert!(
        out.contains("event=tui_cmd_results_view count=2"),
        "cmd results history should append show and invalid-set entries: {}",
        out
    );
    assert!(
        out.contains("event=tui_poll_set code=poll_invalid_seconds ok=false"),
        "invalid poll interval should be recorded as deterministic error: {}",
        out
    );
}

#[test]
fn contacts_and_messages_subnav_present() {
    let out = run_headless("/inspector contacts;/key down;/inspector events;/key down;/exit");
    assert!(
        out.contains("event=tui_nav_select domain=contacts label=peer-0")
            && out.contains("event=tui_nav_select domain=messages label=peer-0"),
        "contacts/messages subnav children should be selectable: {}",
        out
    );
    assert!(
        out.contains("event=tui_contacts_view")
            && out.contains("event=tui_messages_view peer=peer-0"),
        "selecting contacts/messages child should deterministically update main content: {}",
        out
    );
}
