Goals: G1, G3, G4, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-14
Replaces: n/a
Superseded-By: n/a

# NA-0284 Closeout and NA-0285 Restoration Test Plan

## Objective

Validate that the closeout marks NA-0284 DONE after the qsl-attachments
capability scope / abuse / logging harness and qsl-protocol evidence PRs
merged, then restores exactly one READY successor, NA-0285, without
implementing NA-0285.

## Protected Invariants

- Exactly one READY item exists after closeout.
- READY item is NA-0285.
- NA-0284 is DONE.
- D-0538 exists once.
- D-0539 exists once.
- No qsl-attachments implementation change occurs in qsl-protocol closeout.
- No qsl-server implementation change occurs.
- No production-readiness claim is introduced.
- Backup/restore gaps remain explicit and future-gated.
- No qsl-protocol runtime, protocol, crypto, state-machine, qsc-desktop,
  website/external repo, workflow, script, Cargo, dependency,
  branch-protection, or public-safety configuration path changes.
- No branch deletion is performed.

## Evidence Carried Forward

- qsl-attachments PR #35 merged the executable capability scope / abuse /
  logging harness.
- qsl-attachments head `d95e2ad6aef6` merged as `0b7b3fcf9afc`.
- qsl-protocol PR #821 merged the NA-0284 evidence/governance record.
- qsl-protocol evidence head `d20580380257` merged as `f4c6f5fef195`.
- D-0538 records the NA-0284 qsl-attachments capability scope / abuse /
  logging harness decision.
- Chosen semantics: resume tokens and fetch capabilities remain
  resource-scoped and reusable only inside that resource scope while valid;
  commit, abort, session expiry, and object expiry invalidate the relevant
  capability; wrong-resource and malformed capability attempts fail closed
  with canonical reason codes and configured abuse escalation; unauthorized
  operations do not mutate another session/object or expose ciphertext or
  plaintext; capability abuse logs and error bodies do not leak capabilities,
  descriptors, ciphertext, or plaintext; the service remains
  opaque-ciphertext only.

## Successor Handoff

NA-0285 is future qsl-attachments backup / partial restore / transactional
recovery boundary planning work. It must design the backup, partial restore,
transactional recovery, and harness boundaries after NA-0282, NA-0283, and
NA-0284 proved local single-root retention, cleanup, quota, and capability
hardening evidence. NA-0285 must not be implemented in this closeout PR.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0284_closeout_restore_na0285_testplan.md`

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
  `NA-0285 — qsl-attachments Backup / Partial Restore / Transactional Recovery Boundary Plan`.
- NA-0284 is DONE.

## Decision Checks

Expected:

- D-0538 exists once.
- D-0539 exists once.
- duplicate decision count is zero.

## Scope Checks

Expected changed paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0284_closeout_restore_na0285_testplan.md`

No implementation, workflow, script, Cargo, website, branch-protection, or
public-safety configuration paths are changed.

## Public-Safety and Cost-Control Checks

Expected:

- Starting post-Packet-F main public-safety is success.
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
- Post-merge main has READY_COUNT `1`, READY NA-0285, NA-0284 DONE, D-0539
  present once, and public-safety required/green.
- NA-0285 remains future qsl-attachments backup / partial restore /
  transactional recovery boundary planning work and is not implemented inside
  the closeout.
