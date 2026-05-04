Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-03

# NA-0247 Closeout and NA-0248 Restoration Test Plan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0247 is closed only from merged desktop GUI public demo readiness evidence and that NA-0248 is restored as the sole READY successor for Suite-2 Triple-Ratchet evidence and claim-boundary mapping.

## Scope

Allowed changed paths for this closeout:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `tests/NA-0247_closeout_restore_na0248_testplan.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden changed paths include `.github/**`, scripts, Cargo metadata, qsp, qsc/qsl implementation paths, apps, tools, inputs, qsc-desktop, qsl-server, qsl-attachments, website, runtime/protocol/crypto/demo/service code, branch-protection settings, and public-safety/check configuration.

## Required Queue State

After the patch:

```text
READY_COUNT 1
READY NA-0248 Suite-2 Triple-Ratchet Evidence and Claim Boundary
NA-0247 DONE Desktop GUI Prototype Validation and Public Demo Readiness
```

## Required Decision State

After the patch:

- D-0460 exists once.
- D-0461 exists once.
- D-0462 is absent.
- No duplicate decision IDs exist.

## NA-0248 Successor Validation

`NEXT_ACTIONS.md` must state that NA-0248:

- is docs-only claim-boundary/evidence mapping;
- allows no wire/behavior change;
- allows no crypto/state-machine change;
- allows no protocol/runtime/crypto/demo/service changes;
- allows no website implementation changes;
- protects safe public wording, no production-readiness overclaim, no unsupported true-Triple-Ratchet overclaim, no anonymity or metadata-elimination overclaim, and explicit release gaps;
- requires claim-boundary document, evidence map, unsafe wording examples, safe wording examples, and release-readiness gap list.

## Validation Commands

```bash
git diff --name-only origin/main...HEAD
git diff --check origin/main...HEAD
cargo audit --deny warnings
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

- PR #742 is merged and recorded.
- Post-merge `public-safety` on PR #742's merge commit completed successfully.
- `NA-0247` is DONE.
- `NA-0248` is the only READY item.
- D-0461 records closeout and successor restoration.
- No forbidden implementation, service, website, Cargo, workflow, script, branch-protection, or public-safety paths are touched.
