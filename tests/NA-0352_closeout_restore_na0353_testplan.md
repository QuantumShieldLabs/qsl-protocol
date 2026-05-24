Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-24

# NA-0352 Closeout and NA-0353 Restoration Testplan

## Objective

Close NA-0352 after the bounded production backup/deploy/rollback boundary
harness merged, and restore exactly one READY successor:
`NA-0353 -- Metadata Runtime Off-Host Encrypted Backup Prerequisite Plan`.

## Protected Invariants

- NA-0352 is DONE after closeout.
- READY_COUNT is exactly 1.
- The sole READY item is NA-0353.
- D-0686 and D-0687 each exist exactly once.
- D-0688 is absent.
- No NA-0353 implementation is included in closeout.
- No live backup, deploy, rollback, restore, purge, public-ingress cutover,
  service mutation, or secret-dependent operation is performed.
- No backup script, timer, fstab, service unit, dependency, workflow, website,
  README, START_HERE, docs/public, qsl-server, qsl-attachments, qshield
  runtime, qsc/qsp, protocol, crypto, or key-schedule path is changed.

## Required Checks

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
cargo audit --deny warnings
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/model_qsc_handshake_suite_id_bounded.py
```

Goal-lint must pass against a synthesized PR event or live PR event that
contains a standalone `Goals: G1, G2, G3, G4, G5` line.

## Claim Boundary Requirements

Closeout evidence must remain explicit that:

- the NA-0352 harness is not production readiness;
- local continuity backup is not complete disaster recovery;
- qsl-server PR #56 is bounded harness evidence, not production/public-internet
  proof;
- qsl-attachments PR #37 is service-local prerequisite evidence, not
  production/public-internet proof;
- external review remains incomplete;
- no claim says or implies anonymity, metadata-free behavior, untraceability,
  hidden attachment size, hidden timing metadata, hidden traffic shape, or
  padding that hides all metadata.

## Success Criteria

- `NEXT_ACTIONS.md` marks NA-0352 DONE and NA-0353 READY.
- `DECISIONS.md` includes D-0687 and no duplicate decisions.
- `TRACEABILITY.md` links the closeout decision and this testplan.
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md` records PR #966, post-merge
  public-safety, and the closeout validation result.
- Required local validation and required GitHub checks complete green before
  merge.
