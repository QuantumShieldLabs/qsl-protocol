Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-28

# NA-0237A Send Commit Fallout Repair Testplan

Goals: G4

## Implementation checkpoints

- Current `main` reproduces the exact bounded failure in `tests/send_commit.rs`: `outbox_commit_advances_once` and `send_failure_no_commit` both fail because `qsc vault init --key-source mock` returns `event=error code=vault_mock_provider_retired`.
- `qsl/qsl-client/qsc/tests/send_commit.rs` reuses the shared passphrase-backed helpers from `tests/common/mod.rs` instead of invoking retired MockProvider init directly.
- The repaired `send_commit` tests still prove the same behavioral contract:
  - no send-state advance on relay failure
  - one outbox commit per successful send
  - no production MockProvider behavior is restored
- The same test file directly asserts `qsc vault init --key-source mock` still fails with `vault_mock_provider_retired`.
- The preserved `tools/refimpl/quantumshield_refimpl/src/qsp/state.rs` WIP is dropped from the final diff because PR `#713` already merged the bounded clippy-only `sort_by_key` fix on current `main`.
- The required local validation bundle passes:
  - `cargo fmt --check`
  - `cargo build --locked`
  - `cargo clippy --locked -- -D warnings`
  - `cargo audit --deny warnings`
  - the two exact `send_commit` regressions
  - the full `send_commit` test file
  - the representative qsc smoke subset
  - queue parser with exactly one READY item

## Governance validation checkpoints

- `DECISIONS.md` records `D-0439` and explicitly states the send_commit repair stays test-harness scoped while MockProvider retirement remains fail-closed.
- `TRACEABILITY.md` contains the `NA-0237A implementation/closeout evidence` entry pointing to the repaired send_commit seam, the baseline failure proof, the repaired proof, the PR `#708` preservation proof, and the Mode A closeout result.
- `NEXT_ACTIONS.md` marks `NA-0237A` `DONE`, restores `NA-0237` as the sole `READY` item, keeps `NA-0237B`/`NA-0237C`/`NA-0237D` `DONE`, and leaves `NA-0238` `BACKLOG` only.
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md` contains the matching `QSL-DIR-2026-04-28-005` entry.
- Local goal-lint passes via the accepted synthetic-event path with the implementation PR body metadata.
- The markdown inventory commands and the manual link-integrity runbook from `AGENTS.md` pass.
- The added-line leak-safe scan reports zero secret-like markers.

## References

- `DECISIONS.md` (`D-0439`)
- `TRACEABILITY.md`
- `NEXT_ACTIONS.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
