Goals: G4

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-04

# NA-0412 Closeout / NA-0413 Restoration Testplan

## Objective

Validate that NA-0412 closes only after the qwork startup proof-file handoff
implementation evidence merged, and that the preserved backup status / plan
authorization lane is restored as NA-0413 without implementing NA-0413.

## Protected Invariants

- READY_COUNT remains exactly `1`.
- NA-0412 is DONE.
- NA-0413 is the sole READY item.
- D-0810 exists exactly once.
- D-0811 exists exactly once.
- D-0812 exists exactly once.
- qwork proof files remain available outside the qsl-protocol worktree.
- Future-directive verification can read the qwork proof file and direct repo
  checks without rerunning qwork.
- No backup or restore operation is run.
- qsl-backup, backup source lists, backup status files, and backup plan files
  are not mutated.

## Allowed Scope

Only these qsl-protocol paths may change:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0412_closeout_restore_na0413_backup_status_plan_testplan.md`

## Forbidden Scope

The closeout must not change runtime, protocol, crypto, dependency, workflow,
qwork, qshell, qstart, qresume, qsl-server, qsl-attachments, qshield runtime,
website, public docs, README, START_HERE, qsl-backup, backup source lists,
backup status, backup plan, systemd, timers, fstab, durable Director State
Index output, response archives except the final D248 response file, local
history, or secret-handling paths.

## Evidence Dependency

- Packet A PR #1091 must be verified as merged.
- Packet C PR #1092 must be verified as merged.
- The Packet C merge commit must equal or be an ancestor of `origin/main`.
- Packet C post-merge public-safety must be green.
- D-0811 must remain accepted and must cite the NA-0412 evidence/testplan.

## NA-0413 Block Requirements

The restored NA-0413 block must include:

- title `QSL Codex Ops Backup Status / Plan Update Authorization Plan`;
- status `READY`;
- goals `G1, G2, G3, G4, G5`;
- objective preserving same-host manifest evidence boundaries;
- no future status/plan mutation unless live scope authorizes exact files and
  wording;
- no backup/restore/qsl-backup mutation;
- no public-readiness, backup-complete, restore-proof, off-host, or
  disaster-recovery claim.

## qwork Proof Availability

Validate:

```bash
test -f /srv/qbuild/work/NA-0412/.qwork/startup.qsl-protocol.kv
test -f /srv/qbuild/work/NA-0412/.qwork/startup.qsl-protocol.json
```

Then read the KV proof and compare direct repo/queue checks without invoking
qwork for proof parseability and the recorded post-Packet-C merge SHA.

The NA-0412 proof file is not expected to match the post-closeout READY lane
after NA-0413 is restored. Future NA-0413 directives should start from a fresh
operator `qwork NA-0413 qsl-protocol` run and then have Codex read
`/srv/qbuild/work/NA-0413/.qwork/startup.qsl-protocol.kv` plus direct repo
checks, without requiring pasted qwork output.

## Queue / Decision Validation

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue --select NA-0412 --select NA-0413
python3 scripts/ci/qsl_evidence_helper.py decisions --select D-0810 --select D-0811 --select D-0812
```

Required results:

- READY_COUNT `1`.
- READY NA-0413.
- NA-0412 DONE.
- latest decision D-0812.
- D-0810 once.
- D-0811 once.
- D-0812 once.
- duplicate decision count zero.

## CI / Public-Safety Expectations

Run:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allowed NEXT_ACTIONS.md --allowed DECISIONS.md --allowed TRACEABILITY.md --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md --allowed tests/NA-0412_closeout_restore_na0413_backup_status_plan_testplan.md
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file /tmp/na0412_closeout_pr_body.md --scan-overclaims
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo fmt --check
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

Also run goal-lint with a standalone near-top line:

```md
Goals: G4
```

Pass criteria:

- scope guard reports only the five allowed closeout paths;
- link check reports no missing links;
- leak scan reports no findings;
- PR body preflight reports required metadata and no prohibited overclaim
  phrases;
- dependency, formatting, qsc send_commit, formal, and goal-lint checks pass;
- required PR checks attach and pass before merge;
- post-merge public-safety completes success.
