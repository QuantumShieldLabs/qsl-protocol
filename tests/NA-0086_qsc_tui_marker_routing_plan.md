# NA-0086 — qsc TUI marker routing plan

## Scope & assumptions
- Interactive TUI should not print QSC_MARK to stdout/stderr.
- Headless TUI continues to emit QSC_MARK to stdout for deterministic tests.

## Marker routing rules (interactive vs headless)
- Interactive: markers routed to in-app Events pane only.
- Headless: markers emitted to stdout as QSC_MARK lines.

## Test strategy (stdout capture vs internal hook)
- Headless run: capture stdout and assert QSC_MARK present.
- Interactive mode: capture stdout and assert no QSC_MARK lines.

## Manual verification steps (TUI screenshot criteria)
- Run TUI interactively, invoke /help.
- Verify layout is stable and events pane shows help items without marker text in the UI.

## Verification checklist
- cargo test -p qsc --locked
- cargo clippy -p qsc --all-targets -- -D warnings
- Headless marker test passes
- Interactive stdout marker suppression test passes

## Rollback
- Revert marker routing gate; restore prior stdout marker behavior.
