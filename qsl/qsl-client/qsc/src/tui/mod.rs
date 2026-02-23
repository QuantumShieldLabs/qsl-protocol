#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum TuiMode {
    Normal,
    Help,
    FocusEvents,
    FocusFiles,
    FocusActivity,
    FocusStatus,
    FocusSession,
    FocusContacts,
    FocusSettings,
    FocusLock,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum TuiInspectorPane {
    Events,
    Files,
    Activity,
    Status,
    Account,
    Relay,
    CmdResults,
    Session,
    Contacts,
    Settings,
    Lock,
    Help,
    About,
    Legal,
}

impl TuiInspectorPane {
    pub(crate) fn as_name(self) -> &'static str {
        match self {
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
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum TuiHomeFocus {
    Nav,
    Main,
    Command,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum TuiPollMode {
    Adaptive,
    Fixed,
}

impl TuiPollMode {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            TuiPollMode::Adaptive => "adaptive",
            TuiPollMode::Fixed => "fixed",
        }
    }
}

#[derive(Clone)]
pub(crate) enum LockedFlow {
    None,
    UnlockPassphrase,
    InitAlias,
    InitPassphrase { alias: String },
    InitConfirm { alias: String, passphrase: String },
    InitDecision { alias: String, passphrase: String },
}

#[derive(Clone)]
pub(crate) enum AccountDestroyFlow {
    None,
    Passphrase,
    ConfirmDecision { passphrase: String },
}

#[derive(Clone, Copy)]
pub(crate) enum NavRowKind {
    Domain(TuiNavDomain),
    SystemAccount,
    SystemRelay,
    SystemSettings,
    SystemCmdResults,
    Header(TuiInspectorPane),
    Conversation(usize),
    Contact(usize),
    Unlock,
    Exit,
}

#[derive(Clone, Copy)]
pub(crate) struct NavRow {
    pub(crate) kind: NavRowKind,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum TuiNavDomain {
    System,
    Contacts,
    Messages,
}

pub(crate) const TUI_H3_WIDE_MIN: u16 = 120;
pub(crate) const TUI_H3_TALL_MIN: u16 = 28;
pub(crate) const TUI_INSPECTOR_CONTACTS_MAX: usize = 8;
pub(crate) const PANEL_INNER_PAD: usize = 2;
pub(crate) const NAV_CHILD_INDENT: usize = 2;
pub(crate) const TUI_NOTE_TO_SELF_LABEL: &str = "Note to Self";
pub(crate) const TUI_NOTE_TO_SELF_TIMELINE_PEER: &str = "self";
pub(crate) const TUI_MESSAGE_MAX_CHARS: usize = 1024;
pub(crate) const CONTACTS_COL_ALIAS_WIDTH: usize = 12;
pub(crate) const CONTACTS_COL_TRUST_WIDTH: usize = 11;
pub(crate) const CONTACTS_COL_BLOCKED_WIDTH: usize = 7;

pub(crate) struct TuiStatus<'a> {
    pub(crate) fingerprint: &'a str,
    pub(crate) peer_fp: &'a str,
    pub(crate) qsp: &'a str,
    pub(crate) envelope: &'a str,
    pub(crate) send_lifecycle: &'a str,
    pub(crate) locked: &'a str,
}

pub(crate) struct TuiSession<'a> {
    pub(crate) peer_label: &'a str,
    pub(crate) verified: bool,
    pub(crate) sent_count: u64,
    pub(crate) recv_count: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct TuiFileItem {
    pub(crate) id: String,
    pub(crate) peer: String,
    pub(crate) filename: String,
    pub(crate) byte_len: usize,
    pub(crate) state: String,
    pub(crate) display_state: String,
}

pub(crate) struct HomeLayoutSnapshot {
    pub(crate) contacts_shown: bool,
    pub(crate) header_compact: bool,
}
