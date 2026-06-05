Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-05

# NA-0421 QSL Backup Log Code 23 Clean Follow-Up Status Refresh Authorization Testplan

Goals: G4

## Objective

Validate that NA-0421 authorizes, but does not implement, a bounded future
local status/plan refresh after NA-0420 verified a clean scheduled same-host
backup log and manifest.

## Protected invariants

- qwork proof files are read and verified without rerunning qwork.
- Queue starts with exactly one READY item: NA-0421.
- D-0829 exists once; D-0830 is absent at start.
- Clean scheduled same-host evidence remains current.
- Newer backup evidence is not ignored.
- Backup status and backup plan files remain read-only in NA-0421.
- qsl-backup remains unchanged.
- Codex does not mutate the rollback subtree.
- Codex does not run sudo, backup, restore, or generated packet scripts.
- Same-host continuity caveats remain explicit.
- No public overclaim is introduced.
- NA-0422 is selected but not implemented.

## Allowed scope

Allowed qsl-protocol mutation paths:

- `docs/governance/evidence/NA-0421_qsl_backup_log_code_23_clean_follow_up_status_refresh_authorization_plan.md`
- `tests/NA-0421_qsl_backup_log_code_23_clean_follow_up_status_refresh_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Allowed local proof output root:

- `/srv/qbuild/tmp/NA0421_clean_followup_status_refresh_authorization_<timestamp>/`

Allowed read-only local paths:

- qwork proof files under `/srv/qbuild/work/NA-0421/.qwork/`
- `/backup/qsl/logs`
- `/backup/qsl/manifests`
- packet-local operator result files
- the NA-0407 rollback directory
- `/usr/local/sbin/qsl-backup`
- `/home/victor/work/qsl/codex/ops/backup/QSL_BACKUP_STATUS.md`
- `/home/victor/work/qsl/codex/QSL_BACKUP_PLAN.md`
- `/home/victor/work/qsl/codex/ops`

## Forbidden scope

- Running qwork, qstart, or qresume.
- Running sudo.
- Running generated packet scripts.
- Running backup.
- Running restore.
- Mutating `/usr/local/sbin/qsl-backup`.
- Mutating `/backup/qsl`.
- Mutating the NA-0407 rollback subtree.
- Mutating backup status or backup plan files.
- Mutating systemd units, timers, fstab, source lists, retention, or backup
  scripts.
- Mutating qwork/qstart/qresume/qshell.
- Mutating runtime, crypto, dependency, workflow, qsl-server, qsl-attachments,
  qshield runtime, website, public docs, README, or START_HERE.
- Creating durable Director State Index output.
- Creating public technical paper content or public readiness claims.
- Handling secret material.

## qwork proof gate

Required checks:

- `.kv` proof exists and has `startup_result=OK`, lane `NA-0421`, repo
  `qsl-protocol`, clean-state fields, READY_COUNT `1`, queue top READY
  `NA-0421`, and requested lane status `READY`.
- JSON proof parses and mirrors the required `.kv` fields.
- live `HEAD` and `origin/main` match the proof after fetch.
- PR #1110 is MERGED with merge commit `4b520529256c`.

Stop if any proof is missing, stale, malformed, or inconsistent.

## Queue and decision gate

Required checks:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
```

Acceptance:

- READY_COUNT `1`;
- READY NA-0421;
- NA-0420 DONE;
- latest decision D-0829;
- D-0828 once;
- D-0829 once;
- D-0830 absent at start;
- duplicate decision count zero.

## Clean log/manifest reconfirmation gate

Required read-only checks:

- inspect `/backup/qsl/logs/daily-20260605T023308-0500.log`;
- inspect `/backup/qsl/manifests/daily-20260605T023308-0500.manifest.txt`;
- identify the latest scheduled log and manifest under `/backup/qsl`;
- if a newer matched scheduled pair exists, inspect it before choosing the
  classification.

Required log counts:

- `rsync error`
- `code 23`
- `Permission denied`
- `NA0407`
- `rollback`
- exact NA-0407 rollback directory reference
- `qsl-backup.preimage`
- nonzero exit-code marker

Required manifest counts:

- `/home/victor/work/qsl/codex/ops`
- `NA0407`
- `rollback`
- exact NA-0407 rollback directory reference
- `qsl-backup.preimage`

Acceptance for clean proof:

- log SHA256 matches inherited NA-0420 evidence;
- manifest SHA256 matches inherited NA-0420 evidence;
- latest pair is the same reviewed NA-0420 pair unless a newer clean pair is
  found;
- log has zero code 23, permission-denied, rsync-error, rollback, and
  nonzero-exit markers;
- manifest includes Codex ops exactly once.

## Status/plan read-only gate

Inspect read-only:

- `/home/victor/work/qsl/codex/ops/backup/QSL_BACKUP_STATUS.md`
- `/home/victor/work/qsl/codex/QSL_BACKUP_PLAN.md`

Record for each file:

- existence;
- owner/mode/mtime;
- SHA-256;
- whether it mentions the older NA-0414 code 23 caveat;
- whether it mentions NA-0420 clean scheduled proof;
- whether it mentions the clean log path;
- whether it mentions the clean manifest path;
- whether it preserves same-host continuity caveat;
- whether it avoids positive off-host claims, positive disaster-recovery
  claims, positive restore-proof claims, positive backup-complete claims,
  positive public-readiness claims, positive vulnerability-free claims,
  positive bug-free claims, and positive perfect-crypto claims;
- exact sections that would need future update;
- whether the file should be a future mutable candidate.

Acceptance:

- no local status/plan mutation in NA-0421;
- both files may be selected as future mutable candidates if they still carry
  stale NA-0414 code-23 caveat wording and omit NA-0420 clean proof.

## Classification gate

Choose exactly one classification:

- `STATUS_REFRESH_AUTHORIZED_CLEAN_SAME_HOST_CODE23_CLEARED`
- `STATUS_REFRESH_NOT_REQUIRED`
- `STATUS_REFRESH_BLOCKED_CODE23_REGRESSION`
- `STATUS_REFRESH_BLOCKED_NEW_BACKUP_FAILURE`
- `STATUS_REFRESH_BLOCKED_EVIDENCE_AMBIGUITY`

Expected result for clean current proof plus stale local docs:

`STATUS_REFRESH_AUTHORIZED_CLEAN_SAME_HOST_CODE23_CLEARED`

## Successor gate

Select exactly one NA-0422 successor:

- authorized refresh:
  `NA-0422 -- QSL Backup Log Code 23 Clean Status / Plan Refresh Implementation Harness`;
- no update required:
  `NA-0422 -- QSL Backup Log Code 23 Clean Status / Plan Refresh Not Required Closeout`;
- code 23 regression:
  `NA-0422 -- QSL Backup Log Code 23 Regression After Clean Proof Triage Plan`;
- new backup failure:
  `NA-0422 -- QSL Backup Log Post-Clean-Proof New Failure Triage Plan`;
- evidence ambiguity:
  `NA-0422 -- QSL Backup Log Clean Proof Evidence Ambiguity Resolution Plan`.

Do not implement NA-0422 in the NA-0421 evidence PR.

## Future marker gate

For the selected implementation successor, require future markers:

- `NA0422_CLEAN_SCHEDULED_LOG_REFERENCE_OK`
- `NA0422_CLEAN_MANIFEST_REFERENCE_OK`
- `NA0422_CODE23_CLEARED_CAVEATED_OK`
- `NA0422_CODEX_OPS_MANIFEST_PRESENT_OK`
- `NA0422_SAME_HOST_CONTINUITY_CAVEAT_OK`
- `NA0422_NO_OFF_HOST_BACKUP_CLAIM_OK`
- `NA0422_NO_DISASTER_RECOVERY_CLAIM_OK`
- `NA0422_NO_RESTORE_PROOF_CLAIM_OK`
- `NA0422_NO_BACKUP_COMPLETE_CLAIM_OK`
- `NA0422_NO_BACKUP_EXECUTION_OK`
- `NA0422_NO_RESTORE_EXECUTION_OK`
- `NA0422_NO_QSL_BACKUP_MUTATION_OK`
- `NA0422_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0422_NO_SECRET_MATERIAL_OK`
- `NA0422_STATUS_PLAN_REFRESH_IMPLEMENTATION_AUTHORIZED_OK`
- `NA0422_EXACT_LOCAL_PATHS_SELECTED_OK`

## Validation bundle

Required commands before PR:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
cargo fmt --check
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

Additional gates:

- scope guard confirms only the five allowed qsl-protocol paths changed;
- changed-line overclaim scan reports no positive public overclaim;
- classifier passes;
- PR body preflight passes;
- goal-lint passes;
- public-safety remains required and green before merge.

## Expected result

D-0830 records the clean scheduled proof classification, exact clean log and
manifest paths, status/plan refresh authorization decision, both future mutable
local file candidates, selected NA-0422 successor, no backup/restore by Codex,
no qsl-backup mutation, no rollback subtree mutation by Codex, same-host
caveat, and no public overclaim.
