Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-09

# NA-0451 Closeout Restore NA-0452 Testplan

Goals: G1, G2, G3, G4, G5

## Scope

This closeout testplan covers the governance-only transition from NA-0451 to
the selected NA-0452 successor. It does not implement NA-0452, executable tests,
test seams, runtime behavior, crypto behavior, dependencies, workflows, fuzz
targets, vectors, formal models, qsl-server changes, qsl-attachments changes,
qshield-cli changes, public documentation, website changes, backup/restore
changes, or qwork tooling changes.

## Required Queue Checks

- `python3 scripts/ci/qsl_evidence_helper.py queue` reports READY_COUNT 1.
- NA-0451 is DONE.
- NA-0452 is READY.
- No other READY item exists.

## Required Decision Checks

- D-0889 exists once.
- D-0890 exists once.
- D-0891 is absent until future NA-0452 work.
- Duplicate decision ID count is zero.
- D-0890 records PR #1171 closeout evidence and selected successor scope.

## Required Scope Checks

- Changed paths are limited to:
  - `NEXT_ACTIONS.md`
  - `DECISIONS.md`
  - `TRACEABILITY.md`
  - `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
  - `tests/NA-0451_closeout_restore_na0452_testplan.md`
- No runtime, crypto, dependency, Cargo, lockfile, workflow, executable test,
  fuzz target, vector, formal model, qsl-server, qsl-attachments, qshield
  runtime, qshield-cli, website, public-doc, README, START_HERE, qwork/qstart/
  qresume/qshell, backup, restore, qsl-backup, status/plan, rollback, backup
  tree, branch-protection, or public-surface path is mutated by closeout.

## Required Claim Checks

- No public-readiness claim is introduced.
- No production-readiness claim is introduced.
- No public-internet-readiness claim is introduced.
- No external-review-complete claim is introduced.
- No crypto-complete claim is introduced.
- No RNG-failure-complete claim is introduced.
- No side-channel-free claim is introduced.
- No vulnerability-free claim is introduced.
- No bug-free claim is introduced.
- No perfect-crypto claim is introduced.
- Cargo audit green remains dependency-health evidence only.

## Required Validation

- `git diff --check`.
- Exact changed-path scope guard against `origin/main...HEAD`.
- Deterministic local-link check reports `TOTAL_MISSING 0`.
- Added-line leak scan reports no secret findings.
- Added-line overclaim scan reports no affirmative overclaim findings.
- PR body preflight passes.
- Goal-lint passes.
- Root `cargo audit --deny warnings` passes.
- Nested qsc fuzz lock audit passes:
  `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`.
- Public-safety is green before merge and after merge.

## Expected Result

NA-0451 is DONE, NA-0452 is the sole READY item, D-0890 is recorded, D-0891 is
absent, and the exact D-0889 future implementation path boundaries are preserved
without implementing NA-0452 or expanding public claims.
