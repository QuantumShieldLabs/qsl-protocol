Goals: G1, G3, G4, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-14
Replaces: n/a
Superseded-By: n/a

# NA-0285 Closeout and NA-0286 Restoration Test Plan

## Objective

Validate that the closeout marks NA-0285 DONE after the qsl-attachments backup
/ partial restore / transactional recovery boundary plan merged, then restores
exactly one READY successor, NA-0286, without implementing NA-0286.

## Protected Invariants

- Exactly one READY item exists after closeout.
- READY item is NA-0286.
- NA-0285 is DONE.
- D-0540 exists once.
- D-0541 exists once.
- No qsl-attachments implementation change occurs in qsl-protocol closeout.
- No qsl-server implementation change occurs.
- No production-readiness claim is introduced.
- No production backup/restore claim is introduced.
- NA-0286 remains future executable qsl-attachments harness work.
- No qsl-protocol runtime, protocol, crypto, state-machine, qsc-desktop,
  website/external repo, workflow, script, Cargo, dependency,
  branch-protection, or public-safety configuration path changes.
- No branch deletion is performed.

## Evidence Carried Forward

- qsl-protocol PR #823 merged the NA-0285 boundary plan.
- qsl-protocol head `1f5b4db2b103` merged as `85d309ec0167`.
- Post-merge main public-safety completed success on `85d309ec0167`.
- D-0540 records the NA-0285 qsl-attachments backup / partial restore /
  transactional recovery boundary plan.
- Chosen boundary: single-node local storage root behavior with same-root
  startup reconciliation; cold full-root backup/restore plus matching service
  configuration as the only documented backup shape needing executable
  full-root copy proof; unsupported hot/live backup; unsupported partial
  restore; no cross-file transactional recovery claim.

## Successor Handoff

NA-0286 is future executable qsl-attachments backup / partial restore /
transactional recovery harness work. It must prove cold full-root
backup/restore behavior, partial restore fail-closed boundaries, transactional
recovery invariants, no resurrection of rejected/expired/deleted state, and no
secret/plaintext logging. NA-0286 must not be implemented in this closeout PR.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0285_closeout_restore_na0286_testplan.md`

## Forbidden Scope

Forbidden paths include `.github/**`, `scripts/**`, `Cargo.toml`,
`Cargo.lock`, `qsp/**`, `qsc/**`, `qsl/**`, `qsl-client/**`, `apps/**`,
`tools/**`, `inputs/**`, `formal/**`, `qsc-desktop/**`, `qsl-server/**`,
`qsl-attachments/**`, `website/**`, external website repository paths, and
runtime/protocol/crypto/demo/service code.

## Queue Checks

Expected:

- `python3 scripts/ci/qsl_evidence_helper.py queue` reports READY_COUNT `1`.
- READY is
  `NA-0286 — qsl-attachments Executable Backup / Partial Restore / Transactional Recovery Harness`.
- NA-0285 is DONE.

## Decision Checks

Expected:

- D-0540 exists once.
- D-0541 exists once.
- duplicate decision count is zero.

## Scope Checks

Expected changed paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0285_closeout_restore_na0286_testplan.md`

No implementation, workflow, script, Cargo, website, branch-protection, or
public-safety configuration paths are changed.

## Public-Safety and Cost-Control Checks

Expected:

- Starting post-Packet-E main public-safety is success.
- Closeout PR required checks attach and pass normally.
- Docs/governance-only cost-control may skip heavy full-suite jobs.
- Public-safety remains required before and after merge.

## Local Validation

Run:

- `git diff --check`
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main`
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- overclaim scan for production and public-claim phrases
- branch inventory to confirm no branch deletion
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/run_model_checks.py`
- local goal-lint via synthetic PR event or helper-supported equivalent.

## Success Criteria

- Closeout PR merges normally.
- Post-merge main has READY_COUNT `1`, READY NA-0286, NA-0285 DONE, D-0541
  present once, and public-safety required/green.
- NA-0286 remains future executable qsl-attachments backup / partial restore /
  transactional recovery harness work and is not implemented inside the
  closeout.
