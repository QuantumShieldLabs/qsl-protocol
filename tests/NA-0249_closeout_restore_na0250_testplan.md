Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-04

# NA-0249 Closeout and NA-0250 Restoration Test Plan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0249 closes only after merged formal downgrade/no-mutation evidence and that NA-0250 is restored as the sole READY successor for external-review and release-readiness evidence packaging.

## Preconditions

- PR #746 is merged.
- Packet A merge commit is `52131ee655e9`.
- Packet A validated head is `a9a4d8f28f54`.
- Post-merge main `public-safety` is successful.
- `NEXT_ACTIONS.md` has `READY_COUNT 1` and `READY NA-0249` before closeout edits.
- D-0464 exists once.
- D-0465 is absent before closeout edits.

## Scope Guard

Allowed changed paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0249_closeout_restore_na0250_testplan.md`

Forbidden changed paths include `.github/**`, `scripts/**`, Cargo metadata, qsp, qsc/qsl/qsl-client implementation paths, apps, tools, inputs, formal, qsc-desktop, qsl-server, qsl-attachments, website, runtime/protocol/crypto/demo/service code, branch-protection settings, and public-safety/check configuration.

## Queue Parser Expectation

After closeout, the canonical queue parser must report:

```text
READY_COUNT 1
READY NA-0250 External Review and Release-Readiness Evidence Package
NA-0249 DONE Formal Verification Expansion for Suite-2 Downgrade and No-Mutation Invariants
```

## Decision Parser Expectation

After closeout, the canonical decision parser must report:

- D-0110 exists once.
- D-0439 through D-0465 exist once each.
- No duplicate decision IDs exist.

## NA-0250 Successor Validation

`NEXT_ACTIONS.md` must state that NA-0250:

- is external review and release-readiness evidence packaging;
- has Goals G1, G2, G3, G4, G5;
- allows no wire/behavior change;
- allows no crypto/state-machine change;
- is docs-only;
- includes external review package, release-readiness evidence map, reproducible command list, known gaps / residual risk list, and reviewer-oriented artifact index deliverables;
- protects release-readiness truthfulness, no production-readiness overclaim, no "proven true Triple Ratchet" overclaim, reproducible evidence over marketing language, and visible known gaps;
- excludes website implementation changes and protocol/runtime/crypto/demo/service changes.

## Required Local Validation

Run:

```bash
git status --porcelain=v1 --branch
git diff --name-only origin/main...HEAD
git diff --stat origin/main...HEAD
git diff --check
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
```

Also run:

- canonical queue parser;
- canonical decision parser;
- goal-lint using a synthetic PR event payload;
- markdown inventory and link validation runbook;
- leak-safe added-line scan.

## PR And CI Acceptance

Acceptance:

- changed paths stay inside the closeout allowlist;
- no forbidden paths are touched;
- all required CI contexts attach and pass normally;
- `public-safety` remains required and green;
- no admin bypass, direct push, check spoofing, branch-protection exception, squash merge, or rebase merge is used.
