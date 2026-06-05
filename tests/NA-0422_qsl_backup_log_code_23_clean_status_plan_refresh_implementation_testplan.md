Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-05

# NA-0422 QSL Backup Log Code 23 Clean Status Plan Refresh Implementation Testplan

Goals: G4

## Objective

Validate that NA-0422 updates exactly the two local backup status/plan files
authorized by NA-0421 to cite the clean scheduled same-host log and manifest,
while preserving same-host and public-claim boundaries.

## Protected invariants

- qwork proof files are read and verified without rerunning qwork.
- Queue starts with exactly one READY item: NA-0422.
- D-0831 exists once; D-0832 is absent at start.
- Clean scheduled same-host evidence remains current.
- Newer backup evidence is not ignored.
- Only the two authorized local docs are manually patched.
- Rollback copies exist before local mutation and remain valid after mutation.
- qsl-backup remains unchanged.
- Codex does not mutate `/backup/qsl` or rollback subtree paths.
- Codex does not run sudo, backup, restore, or generated packet scripts.
- Same-host continuity caveats remain explicit.
- No public overclaim is introduced.
- NA-0423 is selected but not implemented.

## Allowed scope

Allowed local mutable paths:

- `/home/victor/work/qsl/codex/ops/backup/QSL_BACKUP_STATUS.md`
- `/home/victor/work/qsl/codex/QSL_BACKUP_PLAN.md`

Allowed qsl-protocol mutation paths:

- `docs/governance/evidence/NA-0422_qsl_backup_log_code_23_clean_status_plan_refresh_implementation_harness.md`
- `tests/NA-0422_qsl_backup_log_code_23_clean_status_plan_refresh_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Allowed proof output root:

- `/srv/qbuild/tmp/NA0422_clean_status_plan_refresh_<timestamp>/`

Allowed read-only local paths:

- qwork proof files under `/srv/qbuild/work/NA-0422/.qwork/`
- `/backup/qsl/logs`
- `/backup/qsl/manifests`
- packet-local operator result files
- `/usr/local/sbin/qsl-backup`
- `/home/victor/work/qsl/codex/ops`

## Forbidden scope

- Running qwork, qstart, or qresume.
- Running sudo.
- Running generated packet scripts.
- Running backup.
- Running restore.
- Mutating `/usr/local/sbin/qsl-backup`.
- Mutating `/backup/qsl`.
- Mutating rollback subtree paths.
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

- `.kv` proof exists and has `startup_result=OK`, lane `NA-0422`, repo
  `qsl-protocol`, clean-state fields, READY_COUNT `1`, queue top READY
  `NA-0422`, and requested lane status `READY`.
- JSON proof parses and mirrors the required `.kv` fields.
- live `HEAD` and `origin/main` match the proof after fetch.
- PR #1112 is MERGED with merge commit `e7328d8fb3cf`.

Stop if any proof is missing, stale, malformed, or inconsistent.

## Queue and decision gate

Required commands:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
```

Acceptance:

- READY_COUNT `1`;
- READY NA-0422;
- NA-0421 DONE;
- latest decision D-0831 before patch;
- D-0830 once;
- D-0831 once;
- D-0832 absent at start;
- duplicate decision count zero.

## Clean log/manifest reconfirmation gate

Required read-only checks:

- inspect `/backup/qsl/logs/daily-20260605T023308-0500.log`;
- inspect `/backup/qsl/manifests/daily-20260605T023308-0500.manifest.txt`;
- identify the latest scheduled log and manifest under `/backup/qsl`;
- if a newer matched scheduled pair exists, inspect it before choosing the
  evidence pair.

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

- latest pair is the same reviewed NA-0420/NA-0421 pair unless a newer clean
  pair is found;
- log has zero code 23, permission-denied, rsync-error, rollback, and
  nonzero-exit markers;
- manifest includes Codex ops exactly once.

Stop before local mutation if newer evidence regresses.

## Local doc pre-mutation gate

Inspect:

- `/home/victor/work/qsl/codex/ops/backup/QSL_BACKUP_STATUS.md`
- `/home/victor/work/qsl/codex/QSL_BACKUP_PLAN.md`

Record for each file:

- existence;
- regular file, not symlink;
- owner/mode/mtime;
- SHA-256;
- stale NA-0414 code 23 caveat presence;
- clean log/manifest reference presence;
- same-host caveat presence;
- high-confidence secret marker count;
- positive overclaim marker count.

Stop before mutation if either file is missing, symlinked, unreadable, or has a
high-confidence secret finding.

## Rollback gate

Create rollback copies under:

- `/srv/qbuild/tmp/NA0422_clean_status_plan_refresh_<timestamp>/rollback/`

Acceptance:

- rollback copies preserve content;
- rollback file SHA-256 matches pre-mutation SHA-256;
- rollback copies still exist after local mutation.

## Patch gate

Required local doc updates:

- cite the clean scheduled same-host log path;
- cite the clean scheduled same-host manifest path;
- state the prior code 23 warning was cleared in the reviewed scheduled log;
- state Codex ops remained manifest-present exactly once;
- state Codex did not run backup or restore;
- state Codex did not mutate qsl-backup;
- preserve same-host continuity caveats;
- preserve no off-host backup claim;
- preserve no disaster recovery claim;
- preserve no restore proof claim;
- preserve no complete backup coverage claim;
- preserve no public readiness claim.

The backup plan must not change backup architecture or source-list behavior.

## Local validation gate

After applying the local patch:

- record after SHA-256 for both files;
- record diff;
- record owner/mode/mtime for both files;
- verify rollback copies still exist;
- run local secret scan on changed files/diff;
- run positive-overclaim scan on changed files/diff;
- verify clean scheduled log path is present;
- verify clean scheduled manifest path is present;
- verify same-host caveat is present;
- verify qsl-backup SHA is unchanged;
- verify qsl-backup Codex ops source inclusion count remains `1`.

If validation fails, restore both local files from rollback copies, verify the
rollback SHA-256 values, and stop.

## Governance evidence gate

Required qsl-protocol additions:

- NA-0422 evidence doc.
- NA-0422 testplan.
- D-0832.
- TRACEABILITY update.
- Rolling journal update.

Required successor:

- `NA-0423 -- QSL Domain Stewardship / Director Workflow Governance Authorization Plan`
  if local status/plan refresh succeeds.

Do not implement NA-0423 in this evidence PR.

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
- changed-line positive-overclaim scan reports no positive public overclaim;
- classifier passes;
- PR body preflight passes;
- goal-lint passes;
- public-safety is green before merge and after merge.

## Expected result

After merge, queue proof still reports READY_COUNT `1` and READY NA-0422.
D-0832 exists once. D-0833 remains absent until closeout. The two local docs
cite the clean scheduled same-host log/manifest evidence and preserve all
same-host/no-public-overclaim caveats.
