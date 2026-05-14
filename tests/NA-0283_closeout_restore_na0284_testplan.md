Goals: G1, G3, G4, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-14
Replaces: n/a
Superseded-By: n/a

# NA-0283 Closeout and NA-0284 Restoration Test Plan

## Objective

Validate that the closeout marks NA-0283 DONE after the qsl-attachments disk
pressure / quota / abuse harness and qsl-protocol evidence PRs merged, then
restores exactly one READY successor, NA-0284, without implementing NA-0284.

## Protected Invariants

- Exactly one READY item exists after closeout.
- READY item is NA-0284.
- NA-0283 is DONE.
- D-0536 exists once.
- D-0537 exists once.
- No qsl-attachments implementation change occurs in qsl-protocol closeout.
- No qsl-server implementation change occurs.
- No production-readiness claim is introduced.
- No qsl-protocol runtime, protocol, crypto, state-machine, qsc-desktop,
  website/external repo, workflow, script, Cargo, dependency,
  branch-protection, or public-safety configuration path changes.
- No branch deletion is performed.

## Evidence Carried Forward

- qsl-attachments PR #34 merged the executable disk pressure / quota / abuse
  harness.
- qsl-attachments head `baf5a9c9d3b7` merged as `4ae5ceab6f1a`.
- qsl-protocol PR #819 merged the NA-0283 evidence/governance record.
- qsl-protocol evidence head `2a6b7944edf5` merged as `967fa59ce6b1`.
- D-0536 records the NA-0283 qsl-attachments disk pressure / quota / abuse
  harness decision.
- Chosen semantics: quota and simulated disk-headroom rejects return
  deterministic `REJECT_QATTSVC_QUOTA`; rejected create/upload paths do not
  persist new state; low-headroom commit does not create objects and preserves
  only the pre-existing committable session/part contract; same-root restart
  does not resurrect rejected writes; startup recovery discards partial staged
  or object artifacts fail-closed; resource-scoped capabilities cannot bypass
  pressure or fetch another object; abuse loops remain bounded and secret-safe.

## Successor Handoff

NA-0284 is future executable qsl-attachments capability scope / abuse /
logging harness work. It must prove wrong-resource capability rejects,
replay/duplicate capability behavior where applicable, abuse loops, no
unauthorized fetch/delete/update, and no secret/plaintext logging. NA-0284 must
not be implemented in this closeout PR.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0283_closeout_restore_na0284_testplan.md`

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
  `NA-0284 — qsl-attachments Capability Scope / Abuse / Logging Harness`.
- NA-0283 is DONE.

## Decision Checks

Expected:

- D-0536 exists once.
- D-0537 exists once.
- D-0538 is absent.
- duplicate decision count is zero.

## Scope Checks

Expected changed paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0283_closeout_restore_na0284_testplan.md`

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
- Post-merge main has READY_COUNT `1`, READY NA-0284, NA-0283 DONE, D-0537
  present once, and public-safety required/green.
- NA-0284 remains future executable qsl-attachments capability scope / abuse /
  logging harness work and is not implemented inside the closeout.
