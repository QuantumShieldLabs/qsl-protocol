Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-14
Replaces: n/a
Superseded-By: n/a

# NA-0289 Closeout and NA-0290 Restoration Test Plan

## Objective

Validate that the closeout marks NA-0289 DONE after the external review
package refresh and claim-boundary alignment merged, then restores exactly one
READY successor, NA-0290, without implementing NA-0290.

## Protected Invariants

- Exactly one READY item exists after closeout.
- READY item is NA-0290.
- NA-0289 is DONE.
- D-0548 exists once.
- D-0549 exists once.
- No qsl-protocol runtime, protocol, crypto, or state-machine change occurs.
- No qsl-server implementation change occurs.
- No qsl-attachments implementation change occurs.
- No qsc-desktop, website/external repo, workflow, script, Cargo,
  dependency, branch-protection, or public-safety configuration path changes.
- No branch deletion is performed.
- No production-readiness claim is introduced.
- No public-internet readiness claim is introduced.
- No external-review-complete claim is introduced.
- No anonymity, metadata-free, or untraceable claim is introduced.
- Metadata phase-2 remains incomplete.
- Identifier rotation / opaque handle policy and padding-default policy remain
  design work until separate executable evidence lands.

## Evidence Carried Forward From PR #831

- qsl-protocol PR #831 merged the NA-0289 external review package refresh and
  release readiness evidence map alignment.
- qsl-protocol head `b5bbc92f0730` merged as `d8da8104391d`.
- Post-merge main public-safety completed success on `d8da8104391d`.
- D-0548 records the NA-0289 external review package refresh and
  claim-boundary alignment.
- NA-0289 updated stale main/public-safety references, added NA-0287 and
  NA-0288 alignment, added reviewer checklist and expected reviewer-output
  boundaries, and preserved NOT_READY status for external review completion,
  production readiness, public internet service readiness, metadata phase-2
  completion, anonymity, metadata-free messaging, untraceability, and
  production backup/restore readiness.
- NA-0289 did not implement protocol, crypto, runtime, service, desktop,
  website, workflow, script, Cargo, dependency, branch-protection, or
  public-safety changes.

## Successor Handoff

NA-0290 is future metadata phase-2 identifier rotation and padding defaults
design. It must design identifier rotation / opaque handle policy,
padding-default policy, and the executable-harness plan while preserving no
anonymity, no metadata-free, no untraceable, no external-review-complete, and
no production-readiness claims. NA-0290 must not be implemented in this
closeout PR.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0289_closeout_restore_na0290_testplan.md`

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
  `NA-0290 — Metadata Phase-2 Identifier Rotation and Padding Defaults Design`.
- NA-0289 is DONE.

## Decision Checks

Expected:

- D-0548 exists once.
- D-0549 exists once.
- D-0550 is absent.
- duplicate decision count is zero.

## Scope Checks

Expected changed paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0289_closeout_restore_na0290_testplan.md`

No implementation, workflow, script, Cargo, website, branch-protection, or
public-safety configuration paths are changed.

## Public-Safety and Cost-Control Checks

Expected:

- Starting post-Packet-E main public-safety is success.
- Closeout PR required checks attach and pass normally.
- Docs/governance-only cost-control may skip heavy full-suite jobs.
- Public-safety remains required before and after merge.

## No Branch Deletion Check

Expected:

- No `git push origin --delete` command is run.
- No GitHub refs/branches DELETE API call is run.
- No delete-branch merge flag is passed.
- Repository branch-cleanup or branch-protection settings are not mutated.
- A missing merged PR #831 head branch is recorded only as repository
  auto-cleanup / platform side effect, not as Codex-initiated branch deletion.

## Local Validation

Run:

- `git diff --check`
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main`
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- overclaim scan for production, public, external-review, metadata, and
  anonymity phrases
- branch inventory to confirm no branch deletion
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/run_model_checks.py`
- local goal-lint via synthetic PR event or helper-supported equivalent.

## Success Criteria

- Closeout PR merges normally.
- Post-merge main has READY_COUNT `1`, READY NA-0290, NA-0289 DONE, D-0549
  present once, and public-safety required/green.
- NA-0290 remains future metadata phase-2 identifier rotation and padding
  defaults design work and is not implemented inside the closeout.
