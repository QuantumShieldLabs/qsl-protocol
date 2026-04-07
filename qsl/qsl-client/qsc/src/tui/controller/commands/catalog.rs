use super::*;

pub(super) fn focus_mode_for_fkey(code: KeyCode) -> Option<TuiMode> {
    match code {
        KeyCode::F(2) => Some(TuiMode::FocusEvents),
        KeyCode::F(3) => Some(TuiMode::FocusStatus),
        KeyCode::F(4) => Some(TuiMode::FocusSession),
        KeyCode::F(5) => Some(TuiMode::FocusContacts),
        _ => None,
    }
}

pub(super) fn inspector_for_fkey(code: KeyCode) -> Option<TuiInspectorPane> {
    match code {
        KeyCode::F(2) => Some(TuiInspectorPane::Events),
        KeyCode::F(3) => Some(TuiInspectorPane::Status),
        KeyCode::F(4) => Some(TuiInspectorPane::Session),
        KeyCode::F(5) => Some(TuiInspectorPane::Contacts),
        _ => None,
    }
}

pub(super) fn focus_mode_for_target(target: &str) -> Option<TuiMode> {
    match target {
        "events" => Some(TuiMode::FocusEvents),
        "files" => Some(TuiMode::FocusFiles),
        "activity" => Some(TuiMode::FocusActivity),
        "status" => Some(TuiMode::FocusStatus),
        "session" | "keys" => Some(TuiMode::FocusSession),
        "contacts" => Some(TuiMode::FocusContacts),
        "settings" => Some(TuiMode::FocusSettings),
        "lock" => Some(TuiMode::FocusLock),
        _ => None,
    }
}

pub(super) fn inspector_pane_for_target(target: &str) -> Option<TuiInspectorPane> {
    match target {
        "events" => Some(TuiInspectorPane::Events),
        "files" => Some(TuiInspectorPane::Files),
        "activity" => Some(TuiInspectorPane::Activity),
        "status" | "overview" => Some(TuiInspectorPane::Status),
        "account" => Some(TuiInspectorPane::Account),
        "relay" | "server" => Some(TuiInspectorPane::Relay),
        "cmdresults" | "results" => Some(TuiInspectorPane::CmdResults),
        "session" | "keys" => Some(TuiInspectorPane::Session),
        "contacts" => Some(TuiInspectorPane::Contacts),
        "settings" => Some(TuiInspectorPane::Settings),
        "lock" => Some(TuiInspectorPane::Lock),
        "help" => Some(TuiInspectorPane::Help),
        "about" => Some(TuiInspectorPane::About),
        "legal" => Some(TuiInspectorPane::Legal),
        _ => None,
    }
}

pub(in super::super) struct TuiHelpItem {
    pub(in super::super) cmd: &'static str,
    pub(in super::super) desc: &'static str,
}

pub(in super::super) fn tui_help_items() -> &'static [TuiHelpItem] {
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
