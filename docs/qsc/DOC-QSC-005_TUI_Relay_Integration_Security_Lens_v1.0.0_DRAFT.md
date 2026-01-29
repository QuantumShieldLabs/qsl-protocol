# DOC-QSC-005 TUI Relay Integration (Security Lens) v1.0.0 DRAFT

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
