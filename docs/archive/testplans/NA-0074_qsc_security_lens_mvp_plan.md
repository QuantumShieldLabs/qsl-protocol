# NA-0074 — QSC Security Lens MVP Test Plan (DRAFT)

## Scope & assumptions
- Scope: qsl/qsl-client/qsc/** (CLI + TUI “Security Lens”).
- Assumes existing deterministic marker schema can be extended without secrets.
- No protocol wire changes.

## MVP screens/panels
- Contacts list
- Per-peer session panel
- Message timeline
- Status pane: fingerprint, epoch/ratchet counters, envelope bucket/tick, ACK camouflage, send lifecycle
- Command bar: explicit commands only

## Event/marker schema expectations
- prepare/send/commit markers are deterministic and ordered
- rejects/failures emit deterministic error markers
- no secrets in any marker/log output

## Invariants checklist → mapped tests
1) No hidden state transitions: markers emitted for all persistent state changes
2) No mutation on reject/failure (persistent state unchanged)
3) Redaction: secrets never appear in UI/markers/logs
4) Fail-closed filesystem safety (unsafe parents/symlinks/perms refuse)
5) TUI is a “lens”: no implicit send/retry/recovery
6) Deterministic marker ordering across runs

## Verification checklist
- Add tests for invariants 1–6
- `cargo test -p qsc --locked`
- CI required contexts remain green
- Charter referenced by TRACEABILITY

## Phase 1 execution (this PR)
- Implemented TUI skeleton (read-mostly lens) + headless scripted mode.
- Wired real TUI status values (deterministic local fingerprint, peer status, envelope bucket/tick, send lifecycle).
- Added per-peer session panel with client counters (sent/recv).
- Added tests:
  - `tui_does_not_send_without_explicit_command`
  - `tui_markers_are_deterministic`
  - `tui_no_secrets_in_output`
  - `receive_reject_no_mutation`
- Commands run:
  - `cargo fmt --check` (fallback to file-scoped rustfmt if needed)
  - `cargo test -p qsc --locked`
  - `cargo clippy -p qsc --all-targets -- -D warnings`
  - Clippy fix validation logs: `/home/victor/work/qsl/_forensics/na0074_qsc_clippyfix_20260127T133200Z`
  - Latest gate run logs: `/home/victor/work/qsl/_forensics/na0074_qsc_clippyfix_20260128T000104Z`
  - Phase 1 completion gate logs: `/home/victor/work/qsl/_forensics/na0074_phase1_complete_20260128T003930Z`
  - Note: used isolated `CARGO_HOME`/`CARGO_TARGET_DIR` to avoid ~/.cargo permission errors (see OUT_DIR logs)

## Rollback
- Revert NA-0074 implementation PR
- Remove new tests and revert charter references
