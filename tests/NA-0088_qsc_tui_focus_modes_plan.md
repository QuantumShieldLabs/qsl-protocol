# NA-0088 — qsc TUI Focus Modes Plan

## Scope & assumptions
- Focus modes are full-screen views (Events/Status/Session/Contacts).
- No protocol-core changes.

## Focus mode UI spec (per pane)
- Events: full history list, scrollable.
- Status: expanded status fields, scrollable.
- Session: peer verification + counters, scrollable.
- Contacts: list view, scrollable.

## Keymap (incl. F1 conflict)
- /help enters help mode; Esc exits help.
- '?' toggles help mode (F1 unreliable in GNOME Terminal).
- F2 focus Events, F3 focus Status, F4 focus Session, F5 focus Contacts.
- Esc returns to Dashboard.

## Deterministic marker expectations
- Enter/exit focus emits deterministic markers, e.g.:
  - QSC_MARK/1 event=tui_focus pane=events on=true
  - QSC_MARK/1 event=tui_focus pane=events on=false

## Headless test vectors
- Scripted sequence: /focus events; /focus status; /exit
- Assert deterministic focus markers and stable rendering hints.

## Manual verification checklist (screenshots criteria)
- Each focus mode is full-screen and scrollable.
- Esc returns to dashboard.
- No stdout markers in interactive mode.

## Rollback
- Revert TuiMode additions and keymap bindings.
