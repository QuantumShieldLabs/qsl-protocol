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

## Rollback
- Revert NA-0074 implementation PR
- Remove new tests and revert charter references
