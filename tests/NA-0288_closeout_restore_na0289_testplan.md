Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-14
Replaces: n/a
Superseded-By: n/a

# NA-0288 Closeout and NA-0289 Restoration Test Plan

## Objective

Validate that the closeout marks NA-0288 DONE after the metadata phase-2 and
external review readiness gap plan merged, then restores exactly one READY
successor, NA-0289, without implementing NA-0289.

## Protected Invariants

- Exactly one READY item exists after closeout.
- READY item is NA-0289.
- NA-0288 is DONE.
- D-0546 exists once.
- D-0547 exists once.
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
- Metadata phase-2 gaps remain explicit.
- External review gaps remain explicit.

## Evidence Carried Forward From PR #829

- qsl-protocol PR #829 merged the NA-0288 metadata phase-2 and external
  review readiness gap plan.
- qsl-protocol head `121e8d39b64c` merged as `be8eb172f415`.
- Post-merge main public-safety completed success on `be8eb172f415`.
- D-0546 records the NA-0288 gap plan.
- NA-0288 classified metadata phase-2 and external review readiness as
  executable proof, docs-only planning, not-ready gaps, future gates, or
  out-of-scope claims.
- NA-0288 did not implement protocol, crypto, runtime, service, desktop,
  website, workflow, script, Cargo, dependency, branch-protection, or
  public-safety changes.

## Successor Handoff

NA-0289 is future external review package refresh and claim-boundary
alignment. It must refresh public review-package references and release
evidence map alignment while preserving no external-review-complete, no
anonymity, no metadata-free, no untraceable, and no production-readiness
claims. NA-0289 must not be implemented in this closeout PR.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0288_closeout_restore_na0289_testplan.md`

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
  `NA-0289 — External Review Package Refresh and Claim-Boundary Alignment`.
- NA-0288 is DONE.

## Decision Checks

Expected:

- D-0546 exists once.
- D-0547 exists once.
- D-0548 is absent.
- duplicate decision count is zero.

## Scope Checks

Expected changed paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0288_closeout_restore_na0289_testplan.md`

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
- A missing merged PR #829 head branch is recorded only as repository
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
- Post-merge main has READY_COUNT `1`, READY NA-0289, NA-0288 DONE, D-0547
  present once, and public-safety required/green.
- NA-0289 remains future external review package refresh and
  claim-boundary alignment work and is not implemented inside the closeout.
