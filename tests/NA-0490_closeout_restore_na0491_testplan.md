Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-17

# NA-0490 Closeout and NA-0491 Restoration Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate closeout of NA-0490 after the validator authorization evidence PR
merged and restore exactly one READY successor:

`NA-0491 -- QSL Binding Fuzz Corpus Secret-Material Validator Implementation Harness`

This closeout must not implement NA-0491.

## Protected invariants

- NA-0490 evidence PR #1251 is merged.
- PR #1251 merge commit has public-safety success.
- D-0969 exists once on main before closeout.
- D-0970 is absent before closeout and exists once after closeout.
- NA-0490 is marked DONE.
- NA-0491 is restored as the sole READY item.
- duplicate decision count is zero.
- no validator implementation is performed.
- no corpus/vector/input mutation is performed.
- no qsc source, qsc fuzz target, qsc fuzz Cargo, qsc fuzz lockfile,
  qsc-adversarial script, workflow, dependency, lockfile, formal, refimpl,
  service, public-doc, backup, qsl-backup, qwork, qstart, qresume, qshell,
  archive, move, or delete mutation is performed.
- no public-readiness claim, no production-readiness claim, no public-internet-readiness claim, no external-review-complete claim, no crypto-complete claim, no fuzz-complete claim, no corpus-complete claim, no vector-complete claim, no replay-proof claim, no downgrade-proof claim, no side-channel-free claim, no vulnerability-free claim, no bug-free claim, and no perfect-crypto claim is introduced.

## Allowed scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- this testplan

## Forbidden scope

- NA-0491 implementation
- validator script implementation
- corpus/vector/input mutation
- qsc source/fuzz/Cargo/script/workflow mutation
- dependency/lockfile mutation
- formal/refimpl/service/public/backup mutation
- qwork/qstart/qresume/qshell mutation
- backup/restore/qsl-backup mutation
- file moving, archiving, or deletion

## Validation commands

Run:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 -m json.tool inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json >/dev/null
cargo fmt --check
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

Required:

- exact five-path closeout scope.
- queue proof shows READY_COUNT 1 and READY NA-0491.
- decision proof shows D-0969 once, D-0970 once, duplicate decision count 0.
- PR body preflight PASS.
- goal-lint PASS.
- public-safety green before merge and after merge.

## Post-fix hardening review

Report:

1. Correctness under stress.
2. Minimality.
3. Maintainability.
4. Coverage quality.
5. Cross-lane stability.
