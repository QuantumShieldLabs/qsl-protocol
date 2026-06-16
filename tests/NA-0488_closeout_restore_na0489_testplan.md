Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-16

# NA-0488 Closeout and NA-0489 Restoration Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the closeout-only governance change that marks NA-0488 DONE and
restores the selected NA-0489 successor. This closeout must not implement
NA-0489.

## Protected invariants

- NA-0488 evidence PR #1247 is merged.
- Post-merge public-safety on the NA-0488 evidence merge commit is success.
- D-0965 exists once before closeout.
- D-0966 is absent before closeout and exists once after closeout.
- NA-0488 is marked DONE.
- exactly one READY item remains.
- READY item is NA-0489.
- no implementation mutation is introduced.
- no corpus/vector/input mutation is introduced.
- no qsc source, qsc fuzz target, qsc fuzz Cargo, qsc-adversarial script,
  workflow, dependency, lockfile, formal, refimpl, service, public-doc, backup,
  or qsl-backup mutation is introduced.
- no public-readiness claim, no production-readiness claim, no
  public-internet-readiness claim, no external-review-complete claim, no
  crypto-complete claim, no fuzz-complete claim, no corpus-complete claim, no
  vector-complete claim, no replay-proof claim, no downgrade-proof claim, no
  side-channel-free claim, no vulnerability-free claim, and no perfect-crypto
  claim is introduced.

## Allowed scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- this testplan

## Forbidden scope

- NA-0489 implementation
- qsc source/fuzz/Cargo/script mutation
- corpus/vector/input mutation
- workflow mutation
- dependency mutation
- lockfile mutation
- formal/refimpl/service/public-doc mutation
- qsl-server, qsl-attachments, qshield runtime, or qshield-cli mutation
- backup, restore, qsl-backup, backup status, backup plan, rollback, or backup
  tree mutation
- public claim expansion

## Required validation

Run:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 -m json.tool inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json >/dev/null
cargo fmt --check
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
```

Run exact scope guard against the closeout branch:

```bash
git diff --name-only origin/main...HEAD
```

Required changed paths:

- `DECISIONS.md`
- `NEXT_ACTIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0488_closeout_restore_na0489_testplan.md`

Required final queue proof:

- READY_COUNT 1.
- READY NA-0489.
- NA-0488 DONE.
- D-0966 count 1.
- duplicate decision count 0.

## PR requirements

The closeout PR body must include:

- `Goals: G1, G2, G3, G4, G5`
- Impact
- No-regression
- Tests/Vectors
- closeout-only statement
- selected NA-0489 successor
- no NA-0489 implementation statement
- no corpus/vector/input mutation statement
- no qsc source/fuzz/Cargo/script/workflow mutation statement
- no dependency/lockfile mutation statement
- no public overclaim statement

## Post-merge validation

After merge:

- fetch origin
- fast-forward local main
- verify READY_COUNT 1
- verify READY NA-0489
- verify NA-0488 DONE
- verify D-0966 exists once
- verify public-safety on the closeout merge commit is success
- do not run qwork post-merge
