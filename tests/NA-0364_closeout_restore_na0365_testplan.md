Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-26

# NA-0364 Closeout Restore NA-0365 Testplan

## Objective

Verify that NA-0364 closes only after PR #990 post-merge public-safety is green,
and that the queue restores exactly one successor:
NA-0365 -- Metadata Runtime Restore Drill Isolated Restore No-Secret
Implementation Harness.

## Protected invariants

- NA-0364 is DONE.
- NA-0365 is the only READY item.
- D-0710 exists once.
- D-0711 exists once.
- D-0712 is absent.
- no NA-0365 implementation.
- no real restore execution.
- no real restore target creation/mount/copy.
- no real off-host target/repository/tool/remote setup.
- no real key generation/upload/passphrase/private-key/recovery-envelope/secret handling.
- no backup/restore/deploy/rollback.
- no runtime/service/dependency/workflow/website/public-doc changes.
- no production-readiness/public-internet-readiness/external-review-complete/anonymity/metadata-free/untraceable/hidden-size/hidden-timing/hidden-traffic-shape/off-host-backup-complete/restore-drill-complete/disaster-recovery-complete claim.

## Allowed scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0364_closeout_restore_na0365_testplan.md`

## Forbidden scope

- NA-0365 implementation.
- qsl-server mutation.
- qsl-attachments mutation.
- qshield runtime changes.
- qsc/qsp/protocol/crypto changes.
- dependency or Cargo changes.
- workflow, branch-protection, public-safety, website, public-doc, README, or START_HERE changes.
- backup script, timer, fstab, service, source-list, deploy, rollback, off-host target, restore target, local backup configuration, remote connection, repository init, tool installation, key, passphrase, private-key, recovery-envelope, secret, backup, restore, or copy operation.

## Queue requirements

- `python3 scripts/ci/qsl_evidence_helper.py queue` reports `READY_COUNT 1`.
- `python3 scripts/ci/qsl_evidence_helper.py queue` reports `READY NA-0365`.
- `NEXT_ACTIONS.md` marks NA-0364 DONE.
- `NEXT_ACTIONS.md` does not mark NA-0365 DONE or IN PROGRESS.

## Decision requirements

- `python3 scripts/ci/qsl_evidence_helper.py decisions` reports duplicate decision count zero.
- D-0710 appears exactly once.
- D-0711 appears exactly once.
- D-0712 is absent.
- D-0711 states that NA-0364 delivered restore drill isolated restore authorization result and that NA-0365 is only restored, not implemented.

## Scope requirements

- Scope guard reports zero forbidden paths against `origin/main`.
- `git diff --name-only origin/main...HEAD` lists only the allowed five paths.
- The patch contains no runtime, service, dependency, workflow, website, public-doc, backup, restore, key, off-host, deploy, rollback, branch-protection, or public-safety configuration mutation.

## Link/leak requirements

- Link check reports `TOTAL_MISSING 0`.
- Added-line leak scan reports `SECRET_FINDING_COUNT 0`.
- Evidence prose uses short SHAs and avoids secret-shaped path or endpoint dumps.

## Claim-boundary requirements

- Added text may describe NA-0364 as no-secret isolated restore authorization evidence only.
- Added text must not describe NA-0364 as real restore execution.
- Added text must not describe NA-0365 as implemented.
- Added text must not claim real restore execution, real key custody/recovery implementation, off-host backup completion, complete disaster recovery, production readiness, public-internet readiness, external review completion, anonymity, metadata-free behavior, untraceable behavior, hidden size, hidden timing, or hidden traffic shape.

## Required local checks

- `git status --porcelain=v1 --branch`
- `git diff --name-only origin/main...HEAD`
- `git diff --check`
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allowed NEXT_ACTIONS.md --allowed DECISIONS.md --allowed TRACEABILITY.md --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md --allowed tests/NA-0364_closeout_restore_na0365_testplan.md`
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- `bash scripts/ci/classify_ci_scope.sh NEXT_ACTIONS.md DECISIONS.md TRACEABILITY.md docs/ops/ROLLING_OPERATIONS_JOURNAL.md tests/NA-0364_closeout_restore_na0365_testplan.md`
- changed-line overclaim scan for high-risk claim phrases.
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- PR body preflight / goal-lint.

## CI expectations

- Closeout PR has `Goals: G1, G2, G3, G4, G5` near the top of the body.
- Required checks attach and complete successfully.
- `public-safety` remains required by branch protection and completes success.
- No admin bypass, squash, rebase, direct push, branch deletion, or branch-protection change is used.

## Successor handoff

- NA-0365 is the exact sole READY successor.
- NA-0365 objective is a no-secret isolated restore implementation harness
  selected by D-0710.
- NA-0365 must continue to protect against real restore execution, real restore
  target creation/mount/copy, real key handling, off-host backup setup,
  backup/restore/deploy/rollback operation, qsl-server/qsl-attachments
  production-boundary drift, and unsupported public/privacy/readiness claims
  unless a future exact directive authorizes otherwise.
