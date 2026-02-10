# QSC TUI Implementation Checklist

## NA-0121 Engineering Checklist

## 1) Layout Scaffolding

- Add canonical three-region layout: Nav, Main, Command.
- Enforce one expanded nav domain at a time.
- Keep nav headers non-action.

## 2) Focus Handling

- Implement exclusive focus set: `{Nav, Main, Command}`.
- Bind `Tab`/`Shift+Tab` for focus cycling.
- Ensure no background update can steal focus.

## 3) Event Handling and Redraw Constraints

- Route navigation events as view changes only.
- Keep command execution explicit from command bar.
- Keep redraw bounded under update bursts.

## 4) Update and Counter Mechanics

- Auto-append only when Main is focused.
- Maintain unread/update counters when unfocused.
- Prevent popup and preview leakage in nav.

## 5) Lock-State Redaction

- Render safe redactions in locked mode.
- Keep lock/unlock transitions explicit.
- Prevent sensitive values in visual/log outputs.

## 6) Domain Rules

- Messages: stream inspection, no implicit actions.
- Files: allow multi-select.
- Keys: no multi-select; sensitive ops via command bar only.
- Activity: append-only ledger; avoid duplicate streams.
- Status: snapshot and containment.

## 7) State Semantics Binding

- Bind message state rendering to NA-0118 semantics.
- Bind file transfer state rendering to NA-0119 semantics.
- Never synthesize stronger UI states.

## 8) Regression Requirements

- Deterministic headless marker expectations remain unchanged.
- Existing tests remain green.
- No behavioral drift in locked/unlocked safety constraints.

## 9) Manual QA Checklist

- Verify terminal sizes: small, medium, large.
- Verify key modifiers: Tab, Shift+Tab, Esc, arrows.
- Verify command bar activation/cancel/commit paths.
- Verify update bursts do not steal focus.
- Verify no secret leakage in visible UI/log output.

## 10) Exit Criteria

- Layout, focus, and update rules match `QSC_TUI_SPEC.md`.
- Invariants in `QSC_TUI_INVARIANTS.md` are upheld.
- NA-0118 and NA-0119 truth-sourcing constraints are preserved.
