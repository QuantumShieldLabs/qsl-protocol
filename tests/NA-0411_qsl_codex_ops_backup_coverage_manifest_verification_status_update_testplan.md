Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-03

# NA-0411 Codex Ops Backup Manifest Verification Status Update Testplan

Goals: G4

## Purpose

Verify that NA-0411 records read-only qsl-backup source-list, scheduled
manifest/log, backup status/plan, and Codex ops safety evidence without running
backup or restore and without mutating forbidden local paths.

## Preconditions

- `qwork NA-0411 qsl-protocol` returns `startup_result=OK`.
- READY_COUNT is `1`.
- READY item is NA-0411.
- NA-0410 is DONE.
- D-0805, D-0806, and D-0807 each exist once.
- D-0808 is absent before the NA-0411 evidence patch.
- PR #1088 is MERGED and its merge commit equals or is an ancestor of
  `origin/main`.

## Read-Only Evidence Checks

1. Verify qsl-backup source-list state:
   - `sha256sum /usr/local/sbin/qsl-backup`
   - `bash -n /usr/local/sbin/qsl-backup`
   - exact Codex ops source count is `1`
2. Verify manifest/log state:
   - latest scheduled manifest is after the qsl-backup source-list mtime
   - latest scheduled manifest includes Codex ops source once
   - latest scheduled log is reviewed without claiming backup completion
3. Verify local backup status/plan docs read-only:
   - status file exists and is stale relative to live source-list state
   - plan file exists and is stale relative to live source-list state
   - neither local file is mutated by NA-0411
4. Verify Codex ops safety:
   - no symlink escape
   - no binary candidates
   - no high-confidence secret path/content findings
   - no durable Director State Index path exists

## Classification Expectation

Expected classification:

`CODEX_OPS_MANIFEST_PRESENT_SAME_HOST`

Expected successor:

`NA-0412 -- QSL Codex Ops Backup Status / Plan Update Authorization Plan`

## Forbidden-Path Guard

The changed qsl-protocol path set must be exactly:

- `docs/governance/evidence/NA-0411_qsl_codex_ops_backup_coverage_manifest_verification_status_update_plan.md`
- `tests/NA-0411_qsl_codex_ops_backup_coverage_manifest_verification_status_update_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No qsl-backup, backup status, backup plan, systemd/timer/fstab, runtime,
crypto, workflow, public docs, website, README, START_HERE, qsl-server, or
qsl-attachments path may change.

## Validation Commands

- `git diff --check`
- scope guard over changed paths
- markdown local-link check
- added-line leak scan
- overclaim scan
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- goal-lint / PR body preflight
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- qshield-cli build/test if feasible
- `qwork NA-0411 qsl-protocol`

## Required Markers

- `NA0411_CODEX_OPS_SOURCE_LIST_PRESENT_OK`
- `NA0411_CODEX_OPS_MANIFEST_PRESENT_SAME_HOST_OK`
- `NA0411_STATUS_PLAN_READ_ONLY_OK`
- `NA0411_NO_BACKUP_EXECUTION_OK`
- `NA0411_NO_RESTORE_EXECUTION_OK`
- `NA0411_NO_QSL_BACKUP_MUTATION_OK`
- `NA0411_NO_BACKUP_STATUS_MUTATION_OK`
- `NA0411_NO_BACKUP_PLAN_MUTATION_OK`
- `NA0411_NO_DURABLE_INDEX_WRITE_OK`
- `NA0411_NO_SECRET_MATERIAL_OK`
- `NA0411_SAME_HOST_CONTINUITY_CAVEAT_OK`
- `NA0411_NO_PUBLIC_READINESS_CLAIM_OK`

## Pass Criteria

- D-0808 exists once after the patch.
- D-0809 remains absent.
- Duplicate decision count remains zero.
- READY_COUNT remains `1`.
- READY remains NA-0411 until optional closeout.
- Scope guard finds only allowed NA-0411 paths.
- No backup or restore was run.
- No durable Director State Index was created.
- Public-safety remains required and green before merge and after merge.
