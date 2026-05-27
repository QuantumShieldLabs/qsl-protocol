# NA-0367 Closeout and NA-0368 Restoration Testplan

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-27

## Objective

Validate that NA-0367 is closed only after the target-access and host-identity prerequisite evidence merged, and that the exact NA-0368 successor selected by D-0716 is restored as the sole READY item without implementing NA-0368.

## Protected Invariants

- Exactly one READY item exists after closeout.
- NA-0367 is DONE.
- NA-0368 is READY.
- D-0716 and D-0717 each exist exactly once.
- D-0718 is absent before NA-0368 work begins.
- qsl-protocol is the only mutable repository.
- No qsl-server, qsl-attachments, qshield runtime, qsc/qsp/protocol/crypto/key-schedule, dependency, workflow, README, START_HERE, docs/public, website, service, backup script, timer, fstab, or local backup configuration change is introduced.
- No remote/off-host connection, host-key scan, repository init, tool installation, backup, restore, deploy, rollback, real restore target creation/mount/copy, real key generation, key upload, passphrase collection, credential handling, private-key inspection, recovery-envelope content creation, or secret material handling occurs.
- No claim states or implies production readiness, public-internet readiness, external-review completion, anonymity, metadata-free behavior, untraceability, hidden attachment size, hidden timing metadata, hidden traffic shape, hidden all metadata, complete off-host backup, complete disaster recovery, real restore completion, real key custody implementation, or real key recovery implementation.

## Allowed Scope

Allowed closeout files:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0367_closeout_restore_na0368_testplan.md`

## Forbidden Scope

Forbidden changes include README, START_HERE, docs/public, `.github`, Cargo files, runtime/protocol/crypto/service paths, qsl-server, qsl-attachments, website paths, backup scripts, timers, fstab, local system paths, remote/off-host destinations, restore paths, key material, passphrase paths, credential paths, and recovery-envelope content.

## Queue Requirements

Validation must confirm:

- `READY_COUNT 1`
- `READY NA-0368 Metadata Runtime Off-Host Backup Target Candidate / Host Identity Operator Prerequisite Plan`
- `NA-0367` is DONE
- no remaining READY `NA-0367`

## Decision Requirements

Validation must confirm:

- D-0716 exists exactly once
- D-0717 exists exactly once
- D-0718 is absent
- no duplicate decisions
- D-0717 states that NA-0368 implementation is not authorized by closeout

## Traceability Requirements

TRACEABILITY must link:

- D-0717
- this closeout testplan
- qsl-protocol PR #996
- the selected NA-0368 successor
- qsl-server and qsl-attachments production boundaries
- backup-plan impact classification

## Closeout Evidence Requirements

NEXT_ACTIONS must record:

- qsl-protocol PR #996
- validated head `071d7b7b8a31`
- merge `3b174cd14272`
- post-merge public-safety success on `3b174cd14272`
- NA-0367 prerequisite result categories
- exact selected NA-0368 successor
- no NA-0368 implementation by closeout

## Required Local Checks

Minimum local validation:

- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main`
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- classifier proof for the changed closeout path set
- goal-lint on the closeout PR

## CI Expectations

The closeout PR body must include a standalone `Goals: G1, G2, G3, G4, G5` line near the top. Required checks, including `public-safety`, must pass before merge. Merge must use a normal merge with `--match-head-commit`, no admin bypass, no squash, no rebase, no direct push, and no delete-branch flag.

## Successor Handoff

After merge and post-merge public-safety success, NA-0368 remains the only READY item. NA-0368 must not be implemented by this closeout.
