# QSC TUI Spec

## Purpose

QSC TUI is a security lens and inspectable tool for conversational messaging. The UI must favor explicit intent, deterministic behavior, and truthful security state over convenience shortcuts.

This specification defines the canonical unified layout:
- left navigation pane
- main panel
- command bar

The UI must reflect protocol and file-transfer truths from:
- NA-0118 message state model
- NA-0119 file transfer state model

## Design Goals

- Keep security state observable and auditable.
- Keep operator actions explicit; no hidden behavior.
- Keep update behavior bounded and non-disruptive.
- Keep layout predictable across terminal sizes.

## Canonical Layout

- Left navigation pane: domain navigation only.
- Main panel: domain-scoped content and inspection.
- Command bar: explicit intent entry for complex/sensitive operations.

No implicit actions are permitted from simple navigation alone.

## Navigation Pane Rules

- Domains render as headers (header selection is non-action).
- Exactly one domain is expanded at a time.
- Selection changes view only; selection does not execute actions.
- Minimal unread counters and minimal glyphs are allowed.
- No content previews in nav.

## Main Panel Semantics by Domain

### Messages

- Shows conversation stream.
- Auto-append is bounded when Main has focus.
- No implicit send/retry from view updates.

### Files

- Shows transfer inspection and file-state views.
- Multi-select is allowed for Files domain only.

### Keys

- Inspection-first domain.
- No multi-select.
- Dangerous operations must be command-bar driven.

### Activity

- Append-only ledger view.
- No duplicate activity stream rendered elsewhere.

### Status

- Snapshot of current state.
- Redacted output when locked or when sensitive fields are unavailable.

### Settings/Lock

- Explicit state transitions only.
- No inline dangerous toggles.

## Focus Model

Exactly one focus target is active at any time:
- Nav
- Main
- Command

Rules:
- No automatic focus changes.
- No focus stealing from updates.
- Focus transitions are user-initiated only.

## Auto-Update Rules

- Messages and Activity may update over time.
- Auto-scroll only when Main is focused.
- When unfocused, updates are represented as counters.
- No popups.
- No inline preview leaks.

## Command Bar Grammar

The command bar is explicit-intent only.

Example verb families:
- send
- export
- verify
- rotate-key
- lock
- unlock

Rules:
- Deterministic error responses.
- No implicit retries.
- No hidden recover flows.

## Canonical Keybindings

- `Tab` / `Shift+Tab`: focus cycling.
- Arrow keys: navigate or scroll within focused pane.
- `Esc`: cancel current intent or clear staged command input.
- Command bar activation: explicit key or command-mode gesture.
- Command execution: explicit commit key (for example Enter when command mode is active).

## Status Truth Sources

UI state must be derived from protocol/application truth, not guessed UI heuristics.

- Message state semantics must follow NA-0118.
- File transfer state semantics must follow NA-0119.
- UI must never invent stronger states than underlying truth.

## Non-Goals

- No speculative auto-actions.
- No hidden retries/recovery.
- No alternate state machines in UI.
