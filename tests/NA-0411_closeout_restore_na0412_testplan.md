Goals: G4

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-03

# NA-0411 Closeout / NA-0412 Restoration Testplan

## Objective

Validate that NA-0411 closes only after PR #1089 merged the Codex ops
same-host manifest verification evidence, and that the explicit NA-0412
backup status / plan update authorization plan is restored as the sole READY
successor without implementing NA-0412.

## Protected Invariants

- READY_COUNT remains exactly `1`.
- NA-0411 is DONE.
- NA-0412 is the sole READY item.
- D-0808 exists exactly once.
- D-0809 exists exactly once.
- D-0810 remains absent until NA-0412 live work.
- Classification remains `CODEX_OPS_MANIFEST_PRESENT_SAME_HOST`.
- Same-host continuity remains a caveat, not a disaster-recovery result.
- The latest scheduled backup log code 23 caveat remains explicit.

## Allowed Scope

Only these qsl-protocol paths may change:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0411_closeout_restore_na0412_testplan.md`

## Forbidden Scope

The closeout must not change runtime, protocol, crypto, dependency, workflow,
qsl-server, qsl-attachments, qshield runtime, website, public docs, README,
START_HERE, qsl-backup, backup source lists, backup status, backup plan,
systemd, timers, fstab, durable Director State Index output, qwork, qstart,
qresume, qshell, response archives except the final D246 response file, local
history, or secret-handling paths.

## NA-0411 Evidence Dependency

- PR #1089 must be verified as merged.
- The PR #1089 merge commit must equal or be an ancestor of `origin/main`.
- D-0808 must remain accepted and must cite the NA-0411 evidence and testplan.
- The NA-0411 evidence classification must remain
  `CODEX_OPS_MANIFEST_PRESENT_SAME_HOST`.
- The qsl-backup source-list count for Codex ops must remain exactly `1`.

## NA-0412 Block Requirements

The restored NA-0412 block must include:

- title `QSL Codex Ops Backup Status / Plan Update Authorization Plan`;
- status `READY`;
- goals `G1, G2, G3, G4, G5`;
- objective preserving same-host manifest evidence boundaries;
- allowed read-only inspection of status, plan, manifest, log, and qsl-backup
  inputs;
- no future status/plan mutation unless live scope authorizes exact files and
  wording;
- deliverables for NA-0412 evidence or blocker evidence, testplan, next
  decision, traceability, journal, and a status/plan update recommendation.

## Same-Host Caveat Requirements

- Same-host manifest presence may be described only as same-host local
  continuity evidence.
- Same-host continuity must not be described as off-host backup, complete
  disaster recovery, restore proof, backup completion, public readiness, or
  external-review evidence.

## Log Code 23 Caveat Requirements

- The latest scheduled backup log code 23 caveat must remain visible.
- The code 23 caveat must not be hidden, minimized, or converted into a
  backup-complete claim.

## No Backup / No Restore / No Status Mutation Requirements

- No backup operation is run.
- No restore operation is run.
- `/usr/local/sbin/qsl-backup` is not mutated.
- Backup source lists are not mutated.
- `/home/victor/work/qsl/codex/ops/backup/QSL_BACKUP_STATUS.md` is not mutated.
- `/home/victor/work/qsl/codex/QSL_BACKUP_PLAN.md` is not mutated.
- systemd units, timers, fstab, backup target mounts, retention, and backup
  scripts are not mutated.

## No Public Overclaim Requirements

The closeout must not create or imply:

- off-host backup completion;
- complete disaster recovery;
- restore proof;
- backup completion;
- production readiness;
- public-internet readiness;
- external-review completion;
- metadata-free behavior;
- anonymity;
- untraceability;
- bug-free status;
- vulnerability-free status;
- perfect-crypto status;
- public technical paper content.

## Queue / Decision Validation

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue --select NA-0411 --select NA-0412
python3 scripts/ci/qsl_evidence_helper.py decisions --select D-0808 --select D-0809 --select D-0810
```

Required results:

- READY_COUNT `1`.
- READY NA-0412.
- NA-0411 DONE.
- latest decision D-0809.
- D-0808 once.
- D-0809 once.
- D-0810 absent.
- duplicate decision count zero.

## CI / Public-Safety Expectations

Run:

```bash
git diff --check
git diff --name-only origin/main...HEAD
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allowed NEXT_ACTIONS.md --allowed DECISIONS.md --allowed TRACEABILITY.md --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md --allowed tests/NA-0411_closeout_restore_na0412_testplan.md
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file /tmp/na0411_closeout_pr_body.md --scan-overclaims
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo fmt --check
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

Pass criteria:

- scope guard reports only the five allowed paths;
- link check reports no missing links;
- leak scan reports no findings;
- PR body preflight reports required metadata and no prohibited overclaim
  phrases;
- dependency, formatting, qsc send_commit, and formal checks pass;
- required PR checks attach and pass before merge;
- post-merge public-safety completes success;
- post-merge qwork smoke succeeds for `qwork NA-0412 qsl-protocol`.
