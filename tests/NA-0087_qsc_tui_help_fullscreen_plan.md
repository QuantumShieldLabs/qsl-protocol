# NA-0087 — qsc TUI /help full-screen mode plan

## Scope & assumptions
- Full-screen help mode replaces normal layout when active.
- Content derived from the command registry/parser.

## Help mode UI spec (layout + navigation)
- Left: command list (scrollable).
- Right: details pane for selected command.
- Keys: Up/Down, Enter for details, Esc to exit help.

## Deterministic content and ordering rules
- Stable ordering and stable strings.
- Registry-derived list only (no hardcoded drift).

## Headless test vectors
- Script: /help;/exithelp;/exit (or equivalent).
- Assert: help mode rendered and deterministic list present.

## No-secrets checks
- Output contains no secret-like tokens.

## Manual verification checklist (screenshot criteria)
- /help replaces layout; Esc returns.
- Help list + details visible.

## Rollback
- Revert help mode rendering changes; restore previous help behavior.
