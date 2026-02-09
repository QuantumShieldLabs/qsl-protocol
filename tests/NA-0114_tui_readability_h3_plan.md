# NA-0114 TUI Readability + Information Density (H3 Inspector + Focus Panes) Plan

## Scope and assumptions
- Scope is limited to `qsl/qsl-client/qsc/**`.
- No server, refimpl, or workflow changes are in scope.
- Interactive mode remains marker-silent on stdout.
- Deterministic/headless tests must not rely on wall-clock time.

## Current pain points
- Home inspector can feel crowded at common terminal sizes.
- Information density can reduce clarity when panes compete for small vertical space.
- Focus-pane readability and keybinding discoverability need consistent rendering.
- Timestamp visibility exists in fragmented form and needs deterministic strategy.

## Target layouts
- Home:
  - H3 inspector remains single-pane summary-only.
  - Timeline is the only scrollable home area.
  - Small terminal widths use truncation and ellipsis, not nested tiny scroll boxes.
- Focus Events:
  - Full-screen scrollable event list.
  - Deterministic timestamp column and optional filter/search.
- Focus Status:
  - Full-screen status history with key epochs/ratchet counters/protocol mode.
- Focus Session:
  - Full-screen per-peer details plus recent handshake/ratchet markers.
- Focus Contacts:
  - Full-screen pinned peers with fingerprints and mismatch status.

## Timestamp strategy (deterministic)
- Interactive mode can display local timestamps for operator readability.
- Headless deterministic tests use tick-index or deterministic time source.
- Assertions key off deterministic timestamp tokens/format, never real wall-clock values.

## Search and filter strategy
- Minimum scope: Focus Events filter/search only.
- Keep behavior deterministic and bounded:
  - stable ordering
  - explicit input-state rendering
  - no hidden background refresh behavior

## Marker expectations for tests
- Interactive mode emits no `QSC_MARK` on stdout.
- Deterministic/headless render tests assert stable pane labels, keybinding hints, and timestamp fields.
- Security-sensitive output checks assert no token/secret credential patterns in rendered output.

## Verification checklist
- `cargo fmt -p qsc -- --check`
- `cargo test -p qsc --locked`
- `cargo clippy -p qsc --all-targets -- -D warnings`
- Headless tests prove:
  - timestamps are present in deterministic form
  - full-height focus scroll region behavior
  - stable keybindings (`F2`-`F5`, `Ctrl+F2`-`Ctrl+F5`, `Enter`, `Esc`, `/help`)
  - no overflow/panic on small breakpoints
  - no `QSC_MARK` in interactive stdout

## Rollback
- Revert NA-0114 implementation commits if readability or determinism regressions occur:
  - cluttered home inspector behavior
  - non-deterministic timestamp rendering in tests
  - keybinding inconsistency
  - marker leakage to interactive stdout
