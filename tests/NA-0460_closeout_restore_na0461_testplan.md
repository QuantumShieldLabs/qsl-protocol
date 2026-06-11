Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-11

# NA-0460 Closeout Restore NA-0461 Testplan

Goals: G1, G2, G3, G4, G5

## Purpose

Close out NA-0460 after its evidence PR merges, record D-0908, and restore `NA-0461 -- QSL qsc B1 Signature Provider RNG Failure Test Seam Implementation Harness` as the sole READY item without implementing NA-0461.

## Scope

Allowed closeout mutation paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0460_closeout_restore_na0461_testplan.md`

This closeout does not mutate qsc source, executable test source, runtime behavior, crypto behavior, dependencies, Cargo manifests, lockfiles, workflows, fuzz targets, vectors, formal models, refimpl, qsl-server, qsl-attachments, qshield runtime, qshield-cli, website, public docs, README, START_HERE, qwork/qstart/qresume/qshell, backup/restore/local-ops paths, qsl-backup, backup status files, backup plan files, rollback subtree paths, or backup tree paths.

## Required Proofs

- qsl-protocol PR #1189 merged at `ed9a28e3c216`.
- Post-merge public-safety completed success on `ed9a28e3c216`.
- NA-0460 is DONE.
- NA-0461 is READY.
- READY_COUNT is exactly 1.
- D-0908 exists exactly once.
- D-0909 is absent.
- Duplicate decision count is zero.
- Changed paths are exactly the five allowed closeout paths.
- No backup or restore was run.
- No qsl-backup, backup status, or backup plan path was mutated.

## Validation Commands

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
git diff --name-only origin/main...HEAD
python3 scripts/ci/qsl_evidence_helper.py link-check
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo fmt --check
```

PR body validation must include the required `Goals:` line and must state the closeout-only scope, selected NA-0461 successor, no NA-0461 implementation mutation, no runtime/crypto/dependency/Cargo/lockfile/workflow/test/fuzz/vector/formal mutation, and no public overclaim.

## Expected Result

The closeout PR restores NA-0461 as the sole READY item and preserves all NA-0460 public-claim caveats. Future NA-0461 implementation remains bounded to the exact B1 signature provider RNG failure scope selected by NA-0460 unless a later exact directive narrows or changes that scope.
