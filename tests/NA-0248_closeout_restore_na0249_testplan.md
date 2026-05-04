Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-04

# NA-0248 Closeout and NA-0249 Restoration Test Plan

Goals: G3, G4

## Objective

Validate that NA-0248 is closed only from merged claim-boundary evidence and that NA-0249 is restored as the sole READY successor for formal/model-check expansion.

## Scope

Allowed changed paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0248_closeout_restore_na0249_testplan.md`

Forbidden changed paths include `.github/**`, scripts, Cargo metadata, qsp, qsc/qsl/qsl-client implementation paths, apps, tools/refimpl, tools/actors, inputs, qsc-desktop, qsl-server, qsl-attachments, website, runtime/protocol/crypto/demo/service code, branch-protection settings, and public-safety/check configuration.

## Required Precondition

- PR #744 is merged.
- PR #744 post-merge `public-safety` on `origin/main` completed successfully.
- `NEXT_ACTIONS.md` has `READY_COUNT 1` and `READY NA-0248`.
- D-0462 exists once.
- D-0463 is absent before closeout edits.

## Required Queue State

After the patch:

```text
READY_COUNT 1
READY NA-0249 Formal Verification Expansion for Suite-2 Downgrade and No-Mutation Invariants
NA-0248 DONE Suite-2 Triple-Ratchet Evidence and Claim Boundary
```

## Required Decision State

After the patch:

- D-0462 exists once.
- D-0463 exists once.
- No duplicate decision IDs exist.

## NA-0249 Successor Validation

`NEXT_ACTIONS.md` must state that NA-0249:

- is formal/model-check expansion, not protocol implementation;
- allows no wire/behavior change;
- allows crypto/state-machine changes only for formal/model changes and bounded harness evidence needed to model already-canonical downgrade/no-mutation invariants;
- is not docs-only and must include executable model/check expansion or formal-harness evidence;
- protects fail-closed downgrade rejects, no mutation of modeled durable state on reject, model-claim truthfulness, and required green public-safety;
- excludes `.github`, scripts, Cargo files, qsc/qsl apps, qsc-desktop, qsl-server, qsl-attachments, website, KT/SCKA protocol implementation, public-safety helper/configuration, and branch-protection changes.

## Validation Commands

```bash
git diff --name-only origin/main...HEAD
git diff --stat origin/main...HEAD
git diff --check origin/main...HEAD
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 tools/goal_lint.py
```

Goal-lint may be run with the repo's established synthetic pull-request event payload when local execution requires PR-body context.

Also run:

- canonical queue parser;
- canonical decision parser;
- manual markdown inventory/link validation runbook;
- added-line leak-safe scan.

## Acceptance

- PR #744 merge evidence is recorded.
- `NA-0248` is DONE.
- `NA-0249` is the only READY item.
- D-0463 records closeout and successor restoration.
- No forbidden implementation, service, website, Cargo, workflow, script, branch-protection, or public-safety paths are touched.
