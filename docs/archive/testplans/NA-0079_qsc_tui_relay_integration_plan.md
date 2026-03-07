# NA-0079 TUI Relay Integration Plan

## Scope & assumptions
- TUI relay mode uses explicit CLI args; no implicit network behavior.
- Headless test harness drives scripted commands deterministically.

## UI behaviors to prove
- Events pane reflects relay_event markers deterministically.
- Status pane shows lifecycle updates without secrets.

## Scenario matrix (seeded)
- drop+reorder (seed=7)
- reorder-only (seed=9)

## Deterministic marker subset definition
- Normalize marker stream to `event=tui_event`, `event=send_`, and `event=tui_cmd`.
- Same seed + script => identical normalized marker list.

## Tests/vectors
- tui_relay_drop_reorder_event_stream
- tui_relay_seeded_replay_deterministic

## Verification checklist
- cargo test -p qsc --locked
- cargo clippy -p qsc --all-targets -- -D warnings

## Executed evidence
- QSC_TUI_HEADLESS=1 QSC_TUI_SCRIPT=\"/send;/send;/send;/exit\" qsc tui --transport relay --relay 127.0.0.1:<port> --seed 7 --scenario drop-reorder
- QSC_TUI_HEADLESS=1 QSC_TUI_SCRIPT=\"/send;/send;/exit\" qsc tui --transport relay --relay 127.0.0.1:<port> --seed 9 --scenario reorder

## Rollback
