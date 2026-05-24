Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-24

# NA-0353 Closeout and NA-0354 Restoration Testplan

## Objective

Close NA-0353 after the off-host encrypted backup prerequisite plan merged, and
restore exactly one READY successor:
`NA-0354 -- Metadata Runtime Off-Host Encrypted Backup Implementation Authorization Plan`.

## Protected Invariants

- NA-0353 is DONE after closeout.
- READY_COUNT is exactly 1.
- The sole READY item is NA-0354.
- D-0688 and D-0689 each exist exactly once.
- D-0690 is absent.
- No NA-0354 implementation is included in closeout.
- No live backup, restore, deploy, rollback, purge, off-host setup, public
  ingress cutover, service mutation, key generation, passphrase collection, or
  secret-dependent operation is performed.
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

- the NA-0353 prerequisite plan is not off-host backup implementation;
- local continuity backup is not complete disaster recovery;
- qsl-server PR #56 is bounded harness evidence, not production/public-internet
  proof;
- qsl-attachments PR #37 is service-local prerequisite evidence, not
  production/public-internet proof;
- external review remains incomplete;
- no claim says or implies anonymity, metadata-free behavior, untraceability,
  hidden attachment size, hidden timing metadata, hidden traffic shape, or
  padding that hides all metadata.

## Successor Handoff

The restored NA-0354 item must require an implementation authorization plan
before any future off-host encrypted backup target, key custody, key recovery,
secret handling, restore drill, retention/purge, monitoring/alerting, backup
source-list, script, timer, fstab, system-service, backup, restore, deploy,
rollback, or public-claim mutation.

## Success Criteria

- `NEXT_ACTIONS.md` marks NA-0353 DONE and NA-0354 READY.
- `DECISIONS.md` includes D-0689 and no duplicate decisions.
- `TRACEABILITY.md` links the closeout decision and this testplan.
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md` records PR #968, post-merge
  public-safety, and the closeout validation result.
- Required local validation and required GitHub checks complete green before
  merge.
