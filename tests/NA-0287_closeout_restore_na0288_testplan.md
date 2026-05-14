Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-14
Replaces: n/a
Superseded-By: n/a

# NA-0287 Closeout and NA-0288 Restoration Test Plan

## Objective

Validate that the closeout marks NA-0287 DONE after the service
production-gate evidence map and deployment boundary plan merged, then
restores exactly one READY successor, NA-0288, without implementing NA-0288.

## Protected Invariants

- Exactly one READY item exists after closeout.
- READY item is NA-0288.
- NA-0287 is DONE.
- D-0544 exists once.
- D-0545 exists once.
- No qsl-server implementation change occurs.
- No qsl-attachments implementation change occurs.
- No qsl-protocol runtime, protocol, crypto, or state-machine change occurs.
- No qsc-desktop, website/external repo, workflow, script, Cargo,
  dependency, branch-protection, or public-safety configuration path changes.
- No branch deletion is performed.
- No production-readiness claim is introduced.
- No public-internet readiness claim is introduced.
- No external-review-complete claim is introduced.
- No anonymity, metadata-free, or untraceable claim is introduced.
- Metadata phase-2 gaps remain explicit.
- Service deployment and production-gate gaps remain explicit.

## Evidence Carried Forward From PR #827

- qsl-protocol PR #827 merged the NA-0287 service production-gate evidence
  map and deployment boundary plan.
- qsl-protocol head `ad2bdf3876c1` merged as `dd7cc3fd2cfb`.
- Post-merge main public-safety completed success on `dd7cc3fd2cfb`.
- D-0544 records the NA-0287 service production-gate evidence map and
  deployment boundary plan.
- NA-0287 maps qsl-server and qsl-attachments executable hardening evidence
  into truthful boundaries: proven local harness scope, docs-only planning,
  not-ready service operation gaps, and future public/deployment/external
  review gates.
- NA-0287 does not make production, public exposure, or external review
  completion claims.

## Repository Auto-Cleanup Classification

If the merged PR #827 head branch is absent from origin, classify it as
repository auto-cleanup / platform side effect when PR #827 is merged, the
head and merge SHAs match the directive, Codex did not run or request a
branch deletion command, branch-protection settings were not changed, and no
required evidence depends on the remote head ref existing.

## Successor Handoff

NA-0288 is future metadata phase-2 and external review readiness gap
planning. It must map remaining metadata and review gaps while preserving no
anonymity, no metadata-free, no external-review-complete, and no production
readiness claims. NA-0288 must not be implemented in this closeout PR.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0287_closeout_restore_na0288_testplan.md`

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
  `NA-0288 — Metadata Phase-2 and External Review Readiness Gap Plan`.
- NA-0287 is DONE.

## Decision Checks

Expected:

- D-0544 exists once.
- D-0545 exists once.
- D-0546 is absent.
- duplicate decision count is zero.

## Scope Checks

Expected changed paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0287_closeout_restore_na0288_testplan.md`

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
- A missing merged PR #827 head branch is recorded only as repository
  auto-cleanup / platform side effect, not as Codex-initiated branch
  deletion.

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
- Post-merge main has READY_COUNT `1`, READY NA-0288, NA-0287 DONE, D-0545
  present once, and public-safety required/green.
- NA-0288 remains future metadata phase-2 and external review readiness gap
  planning work and is not implemented inside the closeout.
