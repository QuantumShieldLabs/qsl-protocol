Goals: G4

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-03

# NA-0410 qwork CWD Queue Bugfix Reprioritization Testplan

## Objective

Validate the governance-only reroute that promotes NA-0410 as the qwork
CWD-independent queue verification bugfix lane while preserving the backup
manifest/status lane as NA-0411.

## Scope

Allowed changed paths for this Packet A reroute:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0410_qwork_cwd_queue_bugfix_reprioritization_testplan.md`

Forbidden changes include qwork implementation files, backup source lists,
backup status or plan files, qsl-backup, runtime, protocol, crypto,
dependencies, workflows, public docs, website, README, START_HERE, qsl-server,
qsl-attachments, response archives, and durable Director State Index output.

## Required Queue Properties

- READY_COUNT is exactly `1`.
- The sole READY item is
  `NA-0410 -- QSL Local Ops qwork CWD-Independent Queue Verification Bugfix`.
- `NA-0411 -- QSL Codex Ops Backup Coverage Manifest Verification / Status Update Plan`
  exists and is not READY.
- NA-0409 remains DONE.
- D-0802, D-0803, and D-0804 each exist exactly once.
- D-0805 exists exactly once.
- D-0806 and D-0807 are absent.

## Diagnostic Evidence

Record the reproduced pre-fix failure surface before implementation:

- from a current directory outside the qsl-protocol checkout,
  `qwork NA-0410 qsl-protocol` can fail with `reason=queue-lane-mismatch`;
- the target checkout queue itself is READY NA-0410;
- root cause is cwd-dependent queue helper invocation rather than a real READY
  lane mismatch;
- no backup or restore operation is run.

## Validation Commands

Run the following, or repo-local helper equivalents where available:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
git diff --name-only origin/main...HEAD
git diff --check
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

Also run scope, overclaim, and PR-body goal-lint checks. The PR body must
include a standalone line near the top:

```md
Goals: G4
```

## Pass Conditions

- The changed path set is exactly the Packet A allowed path set.
- Queue and decision helpers report the required properties above.
- Local validation passes.
- Required PR checks attach and pass normally.
- `public-safety` remains required and green before merge and after merge.
- The PR does not implement qwork cwd bugfix and does not implement NA-0411.
