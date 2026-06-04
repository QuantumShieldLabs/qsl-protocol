Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-04

# NA-0418 Backup Log Code 23 Root Operator Packet Generation Testplan

Goals: G4

## Purpose

Validate that NA-0418 generates only a static, no-secret root-operator packet
for preserving rollback evidence and remediating the NA-0407 rollback directory
permission warning, while avoiding Codex sudo, packet execution, backup/restore
execution, qsl-backup mutation, rollback subtree mutation by Codex, status/plan
mutation, runtime/workflow mutation, sibling-repo mutation, and public claim
expansion.

## Scope

Allowed qsl-protocol changes:

- `docs/governance/evidence/NA-0418_qsl_backup_log_code_23_root_operator_evidence_preservation_permission_remediation_packet_generation_harness.md`
- `tests/NA-0418_qsl_backup_log_code_23_root_operator_evidence_preservation_permission_remediation_packet_generation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Allowed local temp output:

- proof root under `/srv/qbuild/tmp/NA0418_packet_generation_proof_<timestamp>/`
- packet root under `/srv/qbuild/tmp/NA0418_code23_root_operator_packet_<timestamp>/`

Forbidden scope includes qwork/qstart/qresume execution by Codex, sudo,
generated script execution, backup execution, restore execution, qsl-backup
mutation, `/backup/qsl` mutation, rollback subtree deletion/move/copy/chmod/
chown or other mutation by Codex, backup status/plan mutation, backup
script/service/timer/fstab mutation, durable Director State Index output,
runtime, protocol, crypto, dependency, workflow, qsl-server, qsl-attachments,
qshield runtime, website, public docs, README, START_HERE, public technical
paper, secret handling, and public-claim expansion.

## Required Assertions

- qwork proof files exist, parse, and match live repo state.
- Codex does not run qwork, qstart, or qresume.
- PR #1103 is merged at `72ccd9a7cd68`.
- READY_COUNT remains `1` and READY remains NA-0418 before and after merge.
- NA-0417 is DONE.
- D-0821 exists once.
- D-0822 exists once.
- D-0823 exists once after the patch.
- D-0824 is absent.
- Duplicate decision count remains `0`.
- qsl-backup checksum prefix remains `e9ecff3d22ed`.
- Codex ops source inclusion count in qsl-backup is exactly `1`.
- Latest scheduled manifest includes `/home/victor/work/qsl/codex/ops`
  exactly once.
- Latest scheduled log status is classified as
  `CODE23_STILL_ACTIVE_NA0407_ROLLBACK_SUBTREE`.
- No other latest-log backup failure source is found.
- Temp rollback subtree is inspected read-only by Codex.
- Packet files are generated only under the allowed packet root.
- Packet scripts are executable.
- Packet scripts are statically validated but not executed.
- Apply and rollback scripts require root.
- Verify script does not require root.
- Scripts contain exact qsl-backup SHA and rollback path.
- Scripts do not invoke sudo.
- Scripts do not run backup or restore modes.
- Scripts do not mutate qsl-backup.
- Scripts do not print rollback file content.
- Packet validation has no secret findings.
- NA-0418 remains READY with USER ACTION REQUIRED.

## Packet Validation Commands

```bash
bash -n \
  /srv/qbuild/tmp/NA0418_code23_root_operator_packet_20260604T092447-05-00/apply_code23_permission_remediation.sh \
  /srv/qbuild/tmp/NA0418_code23_root_operator_packet_20260604T092447-05-00/verify_after_operator_action.sh \
  /srv/qbuild/tmp/NA0418_code23_root_operator_packet_20260604T092447-05-00/rollback_after_operator_action.sh
python3 -m json.tool \
  /srv/qbuild/tmp/NA0418_code23_root_operator_packet_20260604T092447-05-00/operator_packet_manifest.json
sha256sum /srv/qbuild/tmp/NA0418_code23_root_operator_packet_20260604T092447-05-00/*
```

Also run deterministic static scans for:

- forbidden backup/restore/cleanup/push/merge/dependency-update commands
- direct sudo invocation inside scripts
- exact root checks in apply and rollback
- no root check in verify
- exact qsl-backup SHA and rollback path in all scripts
- no qsl-backup mutation terms
- no rollback file content-dump tools
- no secret-looking values beyond expected static checksums

Generated packet scripts must not be run during validation.

## qsl-protocol Validation Commands

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allowed docs/governance/evidence/NA-0418_qsl_backup_log_code_23_root_operator_evidence_preservation_permission_remediation_packet_generation_harness.md \
  --allowed tests/NA-0418_qsl_backup_log_code_23_root_operator_evidence_preservation_permission_remediation_packet_generation_testplan.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file /tmp/na0418_pr_body.md --scan-overclaims
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo fmt --check
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

Also run goal-lint with a synthetic pull-request event containing a standalone
near-top `Goals: G4` line.

## Scope Guard

The qsl-protocol changed path set must be exactly the five allowed NA-0418
evidence paths. Packet files must exist only under the allowed packet root.

## Public-Safety and CI

Before merge, required PR checks must pass, including `public-safety`. After
merge, public-safety must complete success on the merge commit. Use bounded
REST polling only; do not use watch modes.

## Acceptance Criteria

- Code 23 active status is classified read-only.
- Same-host continuity caveat is explicit.
- Latest manifest presence is not described as backup completion.
- Rollback evidence preservation requirements are explicit.
- Packet generation only; no packet execution by Codex.
- No sudo is run by Codex.
- No backup or restore mode is run.
- qsl-backup remains unchanged.
- Temp rollback subtree remains unchanged by Codex.
- Local backup status and plan docs remain unchanged.
- No public-readiness or backup-complete overclaim is introduced.
- USER ACTION REQUIRED is explicit.
- NA-0418 remains READY until operator output is reviewed.
