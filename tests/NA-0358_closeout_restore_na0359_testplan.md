# NA-0358 Closeout and NA-0359 Restoration Testplan

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-25

## Objective

Validate that NA-0358 is closed only after its implementation authorization PR
merged with required checks green, and that the queue restores exactly one READY
successor: NA-0359 -- Metadata Runtime Restore Drill Dry-Run Implementation
Harness.

## Protected Invariants

- NA-0358 is DONE.
- NA-0359 is the sole READY item.
- D-0698 and D-0699 each exist exactly once.
- D-0700 is absent.
- The closeout does not implement NA-0359.
- The closeout does not execute backup, restore, deploy, rollback, off-host
  operation, restore target creation/mount/copy, key generation, key upload,
  passphrase collection, private-key inspection, or secret material handling.
- The closeout does not mutate runtime, protocol, crypto, qsc/qsp, qshield,
  qsl-server, qsl-attachments, dependency, workflow, website, README,
  START_HERE, docs/public, branch-protection, public-safety, backup script,
  timer, fstab, or service configuration paths.
- The closeout does not introduce production/public-internet readiness,
  external-review-complete, anonymity, metadata-free, untraceable, hidden-size,
  hidden-timing, hidden-traffic-shape, restore-drill-complete,
  disaster-recovery-complete, key-custody-implemented, or off-host-backup-
  complete claims.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0358_closeout_restore_na0359_testplan.md`

## Forbidden Scope

- Runtime, protocol, crypto, qsc/qsp, qshield, qsl-server, qsl-attachments,
  qsc-desktop, website, docs/public, README, START_HERE, `.github`, Cargo,
  formal, input, script, service, backup script, timer, fstab, key, passphrase,
  secret, restore target, deploy, rollback, and off-host operation paths.

## Queue Requirements

- `python3 scripts/ci/qsl_evidence_helper.py queue` reports:
  - `READY_COUNT 1`
  - `READY NA-0359 Metadata Runtime Restore Drill Dry-Run Implementation Harness`
- `NEXT_ACTIONS.md` records NA-0358 as DONE.
- `NEXT_ACTIONS.md` records the PR #978 merge evidence and selected NA-0359
  successor.

## Decision Requirements

- `python3 scripts/ci/qsl_evidence_helper.py decisions` reports no duplicate
  decision IDs.
- D-0698 exists once and records the NA-0358 authorization result.
- D-0699 exists once and records closeout plus NA-0359 restoration.
- D-0700 is absent.

## Traceability Requirements

- `TRACEABILITY.md` links D-0699, this testplan, PR #978, and post-merge
  public-safety evidence.
- The traceability row preserves qsl-server PR #56 and qsl-attachments PR #37
  as bounded evidence only through the inherited NA-0358 row.
- Backup-plan impact remains explicit: no current backup-plan update is required
  for governance-only qsl-protocol paths, while future durable restore artifacts,
  restore targets, key material, off-host targets, source-list changes, service
  changes, and monitoring artifacts remain gated by backup-plan and local-ops
  authorization.

## Scope Guard Requirements

- Changed paths are limited to the allowed scope.
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main`
  is run with the exact allowed path list.
- `bash scripts/ci/classify_ci_scope.sh <changed_paths>` classifies the closeout
  as docs/governance/testplan scope and does not mark runtime-critical or
  workflow-security scope.

## Link and Leak Requirements

- `python3 scripts/ci/qsl_evidence_helper.py link-check` passes.
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base
  origin/main` reports no secret findings.
- Added evidence uses short SHAs and no raw secret, key, passphrase, private-key,
  endpoint, auth-header, or long-hex dumps.

## Claim Boundary Requirements

- Added lines are scanned for high-risk readiness, privacy, metadata, restore,
  backup, key, and disaster-recovery phrases.
- Any match is negated, prohibited, partial/not-ready, future-gated, or exact
  bounded evidence wording.
- No public docs or website files are changed.

## Validation Requirements

Required local validation:

```bash
git status --porcelain=v1 --branch
git diff --name-only origin/main...HEAD
git diff --stat origin/main...HEAD
git diff --check
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo fmt --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allowed NEXT_ACTIONS.md DECISIONS.md TRACEABILITY.md docs/ops/ROLLING_OPERATIONS_JOURNAL.md tests/NA-0358_closeout_restore_na0359_testplan.md
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
bash scripts/ci/classify_ci_scope.sh NEXT_ACTIONS.md DECISIONS.md TRACEABILITY.md docs/ops/ROLLING_OPERATIONS_JOURNAL.md tests/NA-0358_closeout_restore_na0359_testplan.md
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

Required PR validation:

- PR body includes a standalone `Goals: G1, G2, G3, G4, G5` line.
- Goal-lint passes against the PR body.
- Required GitHub checks attach and complete green before merge.
- Merge uses a normal merge with `--match-head-commit`.
- No admin bypass, direct push, squash, rebase, delete-branch flag, or branch
  deletion command is used.
- Post-merge qsl-protocol main keeps public-safety required and green.

## Success Criteria

NA-0358 is DONE, NA-0359 is READY, D-0699 is recorded once, D-0700 is absent,
scope guard/link/leak/claim-boundary checks pass, and public-safety remains
required and green. NA-0359 implementation remains future work.
