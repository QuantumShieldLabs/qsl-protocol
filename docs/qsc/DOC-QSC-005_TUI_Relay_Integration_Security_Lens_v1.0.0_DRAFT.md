# DOC-QSC-005 TUI Relay Integration (Security Lens) v1.0.0 DRAFT

> **SUPERSEDED — the qsc TUI was RETIRED at NA-0645 (directive D581, 2026-07-14).**
> Operator product decision: the GUI is the only end-user UI; the CLI remains a thin
> test-harness/operator surface. This document is retained as history and as prior
> art for the GUI. Its security-lens principles (explicit intent, deterministic
> markers, truthful security state, no implicit mutation) carry forward to the GUI
> per DOC-PROG-003 §6. Core behaviors it describes that OUTLIVED the TUI (the
> QSC_TUI_* receipt/delivery/contact-request emitters, the persisted `tui.*` config
> keys, the relay URL policy) are asserted by CLI tests, not by this document.

## Purpose
Define how the qsc Security Lens TUI integrates with relay transport while preserving charter invariants and deterministic observability.

## Transport selection contract
- Explicit-only: `qsc tui --transport relay --relay <url>`
- No implicit network behavior; no background recovery.

## Events taxonomy
- relay_event=drop|dup|reorder|delay|deliver
- send lifecycle: send_prepare|send_attempt|send_commit|send_fail

## UI panels
- Events pane: last N events, filterable
- Status pane: send lifecycle, transport state

## Charter compliance
- No implicit retries/recovery
- No secrets in UI/markers/logs

## Determinism requirements
- Seeded scenarios
- Headless test mode must yield stable normalized marker stream
