Goals: G1, G3, G4, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-14
Replaces: n/a
Superseded-By: n/a

# NA-0286 Closeout and NA-0287 Restoration Test Plan

## Objective

Validate that the closeout marks NA-0286 DONE after the qsl-attachments
backup / partial restore / transactional recovery harness and qsl-protocol
evidence PRs merged, then restores exactly one READY successor, NA-0287,
without implementing NA-0287.

## Protected Invariants

- Exactly one READY item exists after closeout.
- READY item is NA-0287.
- NA-0286 is DONE.
- D-0542 exists once.
- D-0543 exists once.
- No qsl-attachments implementation change occurs in qsl-protocol closeout.
- No qsl-server implementation change occurs.
- No production-readiness claim is introduced.
- No production backup/restore readiness claim is introduced.
- NA-0287 remains service production-gate evidence mapping and deployment
  boundary planning only.
- No qsl-protocol runtime, protocol, crypto, state-machine, qsc-desktop,
  website/external repo, workflow, script, Cargo, dependency,
  branch-protection, or public-safety configuration path changes.
- No branch deletion is performed.

## Evidence Carried Forward

- qsl-attachments PR #36 merged the executable backup / partial restore /
  transactional recovery harness.
- qsl-attachments head `fafd4cecb614` merged as `320be68fe632`.
- qsl-protocol PR #825 recorded NA-0286 evidence.
- qsl-protocol head `a3f14175c6a2` merged as `d327430347a1`.
- Post-merge main public-safety completed success on `d327430347a1`.
- D-0542 records the NA-0286 qsl-attachments backup / partial restore /
  transactional recovery harness decision.
- Chosen semantics: stopped/quiesced full-root copy plus matching service
  configuration is the executable recovery unit; coherent committed objects
  require paired `object.json` plus `ciphertext.bin`; coherent open sessions
  are best-effort resumable only when `session.json` and journaled parts
  match; partial restore remains unsupported and fails closed; rejected,
  expired, deleted, and aborted state does not resurrect; logs and error
  bodies do not leak capabilities, descriptors, ciphertext, or plaintext.

## Successor Handoff

NA-0287 is future service production-gate evidence mapping and deployment
boundary planning. It must map qsl-server and qsl-attachments hardening
evidence without making a production-readiness claim. NA-0287 must not be
implemented in this closeout PR.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0286_closeout_restore_na0287_testplan.md`

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
  `NA-0287 — Service Production-Gate Evidence Map and Deployment Boundary Plan`.
- NA-0286 is DONE.

## Decision Checks

Expected:

- D-0542 exists once.
- D-0543 exists once.
- duplicate decision count is zero.

## Scope Checks

Expected changed paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0286_closeout_restore_na0287_testplan.md`

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
- Post-merge main has READY_COUNT `1`, READY NA-0287, NA-0286 DONE, D-0543
  present once, and public-safety required/green.
- NA-0287 remains future service production-gate evidence mapping and
  deployment boundary planning work and is not implemented inside the
  closeout.
