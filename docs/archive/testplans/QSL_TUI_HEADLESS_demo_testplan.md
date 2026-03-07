# QSL TUI Headless Demo Test Plan (DRAFT)

Goals: G2, G3

## Scope
- Validate qsl-tui headless mode runs in non-interactive environments.
- Confirm relay mode remains opt-in and does not change protocol behavior.

## Invariants
- No protocol-core changes.
- Relay is transport-only; payloads remain opaque bytes.
- Deterministic error on missing opt-in: reason_code=REMOTE_OPT_IN_REQUIRED.

## Tests
- Headless local demo: `qsl-tui --headless --mode local` exits 0.
- Headless relay demo: requires `QSL_ALLOW_REMOTE=1` and completes without TTY.
- Opt-in guard: missing QSL_ALLOW_REMOTE yields deterministic error.

## Evidence
- PR #83 CI logs + local headless run logs under _forensics.
