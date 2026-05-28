Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-28

# NA-0373 Closeout Restore NA-0374 Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0373 is closed only after the response availability blocker
and collection follow-up evidence merged, and that the exact selected successor
`NA-0374 -- Metadata Runtime Off-Host Backup Operator Response Intake After
Collection Follow-Up` is restored as the sole READY item without implementing
NA-0374.

## Protected Invariants

- Exactly one READY item exists after closeout.
- NA-0373 is DONE.
- NA-0374 is READY.
- D-0729 exists once.
- D-0730 is absent.
- Closeout implements no NA-0374 work.
- No qsl-server, qsl-attachments, qshield runtime, qsc/qsp/protocol/crypto,
  Cargo/dependency, workflow, website, README, START_HERE, docs/public, input,
  script/runtime, service, backup script, timer, fstab, local backup config,
  remote/off-host setup, host-key scan, known_hosts mutation, repository init,
  tool installation, backup, restore, deploy, rollback, real restore target
  creation/mount/copy, real key, passphrase, credential, secret, private-key,
  or recovery-envelope material is changed or handled.
- No claim implies production readiness, public-internet readiness, external
  review completion, anonymity, metadata-free behavior, untraceability, hidden
  attachment size, hidden timing metadata, hidden traffic shape, complete
  off-host backup, complete disaster recovery, real restore completion,
  verified host identity, configured target, real key custody implementation,
  or real key recovery implementation.

## Allowed Scope

Allowed files:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0373_closeout_restore_na0374_testplan.md`

## Required Evidence

- qsl-protocol PR #1008 merged normally as `299d75878f2e`.
- PR #1008 validated head was `53467d90d01d`.
- Post-merge `public-safety` completed success on `299d75878f2e`.
- D-0728 exists once and records the NA-0373 blocker/follow-up decision.
- D-0729 exists once and records NA-0373 closeout / NA-0374 restoration.
- D-0730 is absent.
- `NEXT_ACTIONS.md` marks NA-0373 DONE.
- `NEXT_ACTIONS.md` promotes NA-0374 as the sole READY item.

## Validation Commands

- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions --select D-0728 --select D-0729 --select D-0730`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allowed NEXT_ACTIONS.md --allowed DECISIONS.md --allowed TRACEABILITY.md --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md --allowed tests/NA-0373_closeout_restore_na0374_testplan.md`
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- `bash scripts/ci/classify_ci_scope.sh NEXT_ACTIONS.md DECISIONS.md TRACEABILITY.md docs/ops/ROLLING_OPERATIONS_JOURNAL.md tests/NA-0373_closeout_restore_na0374_testplan.md`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`

## CI Expectations

The closeout PR must pass required qsl-protocol checks normally, including
`public-safety`, before merge. After merge, post-merge `public-safety` must
complete successfully.

## Successor Handoff

The next directive may implement only the exact READY item:

`NA-0374 -- Metadata Runtime Off-Host Backup Operator Response Intake After Collection Follow-Up`

This testplan does not authorize NA-0374 implementation.
