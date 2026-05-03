Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-03

# NA-0243 Closeout and NA-0244 Restoration Test Plan

## Objective

Close NA-0243 from merged implementation evidence and restore NA-0244 as the sole READY successor without implementing NA-0244.

## Scope Guard

Allowed changed paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0243_closeout_restore_na0244_testplan.md`

Forbidden changed paths include `.github/**`, `scripts/**`, Cargo manifests/locks, `qsp/**`, `qsc/**`, `qsl/**`, `qsl-client/**`, `apps/**`, `tools/refimpl/**`, `tools/actors/**`, `inputs/**`, `qsc-desktop/**`, `qsl-server/**`, `qsl-attachments/**`, `website/**`, runtime/protocol/crypto/demo/service code, branch protection, and public-safety/check configuration.

## Closeout Evidence Inputs

- PR #734 merged normally.
- PR #734 validated head: `cc44db30056f`.
- PR #734 merge commit: `dbd4bd7bd756`.
- D-0452 exists once.
- D-0453 is absent before closeout edits.
- Post-merge public-safety passed on `dbd4bd7bd756`.

## Expected Queue Result

The queue parser must report:

```text
READY_COUNT 1
READY NA-0244 Metadata Conformance Negative Expansion
NA-0243 DONE Skipped-Key and Receive-Decryption Reject No-Mutation Hardening
```

## Expected Decision Result

The decision parser must report:

- D-0110 exists once.
- D-0451 exists once.
- D-0452 exists once.
- D-0453 exists once.
- No duplicate decision IDs exist.

## Validation

- `git diff --name-only origin/main...HEAD` shows only allowed closeout paths.
- `git diff --check` passes.
- Synthetic-event goal-lint passes with `Goals: G1, G3, G5`.
- Manual markdown link check reports zero missing links.
- Leak-safe added-line scan reports no sensitive markers.
- `cargo audit --deny warnings` passes.
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1` passes.
- Required GitHub contexts, including `public-safety`, pass normally before merge.
