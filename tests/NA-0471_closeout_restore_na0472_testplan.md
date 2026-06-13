Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-06-13

# NA-0471 Closeout and NA-0472 Restoration Testplan

## Scope

This testplan verifies closeout of `NA-0471 -- QSL qsc TUI Account Bootstrap Transactionality Design Authorization Plan` and restoration of `NA-0472 -- QSL qsc TUI Account Bootstrap Pre-Generation Transactionality Implementation Harness` as the sole READY item.

The closeout is governance-only. It does not implement NA-0472 and does not mutate qsc source, executable qsc tests other than this governance testplan, runtime behavior, crypto behavior, dependencies, Cargo manifests, lockfiles, workflows, fuzz targets, vectors, formal models, refimpl, qsl-server, qsl-attachments, qshield runtime, qshield-cli runtime, website, public docs, README, START_HERE, qwork/qstart/qresume/qshell, backup/restore paths, qsl-backup, backup status files, backup plan files, rollback subtree paths, or `/backup/qsl`.

## Expected Governance State

- `NA-0471` is DONE.
- `NA-0472` is READY.
- READY count is exactly one.
- `D-0931` exists once.
- `D-0932` is absent before NA-0472 execution.
- Duplicate decision count is zero.
- `TRACEABILITY.md` records the closeout and selected NA-0472 successor.
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md` records the closeout evidence.

## Required Validation

Run these checks from the qsl-protocol repository root:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
```

Required queue markers:

- `READY_COUNT 1`
- `READY NA-0472`
- `NA-0471 DONE`
- `NA-0470 DONE`

Required decision markers:

- latest decision is `D-0931`.
- `D-0930` exists once.
- `D-0931` exists once.
- `D-0932` is absent.
- duplicate decision count is zero.

## Scope Guard

The closeout PR may mutate only:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0471_closeout_restore_na0472_testplan.md`

Reject the PR if any runtime, crypto, dependency, Cargo, lockfile, workflow, executable test, fuzz target, vector, formal model, refimpl, qsl-server, qsl-attachments, qshield runtime, qshield-cli runtime, website, public doc, README, START_HERE, qwork/qstart/qresume/qshell, backup, restore, qsl-backup, backup status, backup plan, rollback subtree, or `/backup/qsl` path is changed.

## Closeout Markers

- `NA0471_CLOSEOUT_PR1212_MERGED_OK`
- `NA0471_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK`
- `NA0471_CLOSEOUT_D0930_CONSUMED_OK`
- `NA0471_CLOSEOUT_NA0471_DONE_OK`
- `NA0471_CLOSEOUT_NA0472_READY_OK`
- `NA0471_CLOSEOUT_PREGENERATION_SUCCESSOR_OK`
- `NA0471_CLOSEOUT_NO_IMPLEMENTATION_MUTATION_OK`
- `NA0471_CLOSEOUT_NO_RUNTIME_MUTATION_OK`
- `NA0471_CLOSEOUT_NO_CRYPTO_MUTATION_OK`
- `NA0471_CLOSEOUT_NO_DEPENDENCY_CHANGE_OK`
- `NA0471_CLOSEOUT_NO_CARGO_OR_LOCKFILE_CHANGE_OK`
- `NA0471_CLOSEOUT_NO_WORKFLOW_CHANGE_OK`
- `NA0471_CLOSEOUT_NO_PUBLIC_OVERCLAIM_OK`
- `NA0471_CLOSEOUT_ONE_READY_INVARIANT_OK`

## Public Claim Boundary

This closeout makes no public-readiness claim, no production-readiness claim, no public-internet-readiness claim, no external-review-complete claim, no crypto-complete claim, no KEM-complete claim, no signature-complete claim, no identity-complete claim, no RNG-failure-complete claim, no provider-RNG-complete claim, no secret-material-complete claim, no side-channel-free claim, no vulnerability-free claim, no bug-free claim, and no perfect-crypto claim.

Cargo audit green remains dependency-health evidence only.

## Acceptance Criteria

- PR #1212 is merged.
- Post-merge public-safety on PR #1212 merge commit is success.
- `NA-0471` is DONE.
- `NA-0472` is the only READY item.
- `D-0931` records closeout and restoration.
- Changed paths are limited to the closeout scope guard.
- No implementation mutation occurs.
- Root cargo audit remains green.
- Nested qsc fuzz lock audit remains green.
- Public-safety is green before merge and after merge.
