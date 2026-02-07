# NA-0104 TUI Layout H3 Plan

## Breakpoints (rows/cols)
- `WIDE_MIN=120` columns: show `Contacts | Timeline | Inspector`
- `<120` columns: auto-hide contacts pane, show `Timeline | Inspector`
- `TALL_MIN=28` rows: below this, compress command/header hints

## Home Layout Regions and Minimum Sizes
- Home mode uses H3 layout with one inspector drawer:
- Left: contacts list (auto-hidden when narrow)
- Center: timeline/chat (the only home scroll region)
- Right: inspector summary (`status|events|session|contacts`)
- Enter from home opens full-screen focus mode for current inspector pane.

## Inspector Switching Rules
- `F2` => inspector `events`
- `F3` => inspector `status`
- `F4` => inspector `session`
- `F5` => inspector `contacts`
- Slash command: `/inspector status|events|session|contacts`
- `Esc` exits full-screen focus/help back to home.

## Headless Render Test Vectors and Expected Markers
- Added `qsl/qsl-client/qsc/tests/tui_layout_h3.rs`.
- Required deterministic markers:
- `event=tui_render mode=home layout=h3 inspector=<pane> contacts=<shown|hidden> header=<full|compact>`
- `event=tui_inspector pane=<pane>`
- Test vectors:
- status/events/session inspector headless render
- narrow-width responsive hide contacts
- deterministic marker subset across repeated runs

## Verification Checklist
- `cargo fmt -p qsc -- --check`
- `cargo test -p qsc --locked`
- `cargo clippy -p qsc --all-targets -- -D warnings`
- Evidence bundle root:
- `/home/victor/work/qsl/_forensics/na0104_h3_20260207T042730Z`

## Rollback
- Revert `qsl/qsl-client/qsc/src/main.rs` H3 changes and remove `qsl/qsl-client/qsc/tests/tui_layout_h3.rs`.
