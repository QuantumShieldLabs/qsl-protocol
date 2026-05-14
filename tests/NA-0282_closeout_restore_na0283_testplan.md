Goals: G1, G3, G4, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-14
Replaces: n/a
Superseded-By: n/a

# NA-0282 Closeout and NA-0283 Restoration Test Plan

## Objective

Validate that the closeout marks NA-0282 DONE after the qsl-attachments
retention / cleanup / recovery harness and qsl-protocol evidence PRs merged,
then restores exactly one READY successor, NA-0283, without implementing
NA-0283.

## Protected Invariants

- Exactly one READY item exists after closeout.
- READY item is NA-0283.
- NA-0282 is DONE.
- D-0534 exists once.
- D-0535 exists once.
- No qsl-attachments implementation change occurs in qsl-protocol closeout.
- No qsl-server implementation change occurs.
- No production-readiness claim is introduced.
- No qsl-protocol runtime, protocol, crypto, state-machine, qsc-desktop,
  website/external repo, workflow, script, Cargo, dependency,
  branch-protection, or public-safety configuration path changes.
- No branch deletion is performed.

## Evidence Carried Forward

- qsl-attachments PR #33 merged the executable retention / cleanup /
  recovery harness.
- qsl-attachments head `b68a61e7546c` merged as `248665c8b85a`.
- qsl-protocol PR #817 merged the NA-0282 evidence/governance record.
- qsl-protocol evidence head `10f2f8f34e44` merged as `be9b6a88e3f4`.
- D-0534 records the NA-0282 qsl-attachments retention / cleanup /
  recovery harness decision.
- Chosen semantics: request-path expiry cleanup marks expired sessions and
  objects, removes staged parts or committed ciphertext, clears capability
  hashes, preserves unexpired committed objects, and same-root startup
  recovery re-exposes only coherent open sessions plus paired committed
  objects while discarding incoherent artifacts fail-closed.

## Successor Handoff

NA-0283 is future executable qsl-attachments disk pressure / quota / abuse
harness work. It must cover quota rejects, no persistence on rejected writes,
cleanup under pressure, and no secret/plaintext logging. NA-0283 must not be
implemented in this closeout PR.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0282_closeout_restore_na0283_testplan.md`

## Forbidden Scope

Forbidden paths include `.github/**`, `scripts/**`, `Cargo.toml`,
`Cargo.lock`, `qsp/**`, `qsc/**`, `qsl/**`, `qsl-client/**`, `apps/**`,
`tools/**`, `inputs/**`, `formal/**`, `qsc-desktop/**`, `qsl-server/**`,
`qsl-attachments/**`, `website/**`, external website repository paths, and
runtime/protocol/crypto/demo/service code.

## Queue Checks

Expected:

- `python3 scripts/ci/qsl_evidence_helper.py queue` reports READY_COUNT `1`.
- READY is `NA-0283 — qsl-attachments Disk Pressure / Quota / Abuse Harness`.
- NA-0282 is DONE.

## Decision Checks

Expected:

- D-0534 exists once.
- D-0535 exists once.
- D-0536 is absent.
- duplicate decision count is zero.

## Scope Checks

Expected changed paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0282_closeout_restore_na0283_testplan.md`

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
- Post-merge main has READY_COUNT `1`, READY NA-0283, NA-0282 DONE, D-0535
  present once, and public-safety required/green.
- NA-0283 remains future executable qsl-attachments disk pressure / quota /
  abuse harness work and is not implemented inside the closeout.
