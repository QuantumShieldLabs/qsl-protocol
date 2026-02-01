# NA-0085 — qsc TUI help rendering plan

## Scope & assumptions
- qsl/qsl-client/qsc/** only.
- Headless scripted TUI tests are the enforcement mechanism.

## Help rendering spec (pane + deterministic format)
- `/help` renders a deterministic command list to a visible pane (Help or Events).
- Command list derived from the same registry as parser; no hard-coded drift.

## Command list source of truth
- Use the existing command registry / clap definitions.

## Headless test vectors
- QSC_TUI_HEADLESS=1 + QSC_TUI_SCRIPT="/help;/exit"
- Assert help list content appears in rendered output (deterministic text markers ok).

## No-secrets checks
- Output must not include TOKEN/SECRET/KEY/PASS/PRIVATE/BEARER/CREDENTIAL.

## Verification checklist
- cargo test -p qsc --locked
- cargo clippy -p qsc --all-targets -- -D warnings

## Rollback
- Revert help render path and tests if regressions occur.
