Goals: G4

# NA-0409 qwork Director-Facing Startup Reprioritization Testplan

## Objective

Validate the governance-only reroute that promotes NA-0409 as the qwork
Director-facing startup availability / tmux safety hardening lane while
preserving the backup manifest/status lane as NA-0410.

## Scope

Allowed changed paths for this Packet A reroute:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0409_qwork_director_facing_startup_reprioritization_testplan.md`

Forbidden changes include qwork implementation files, backup source lists,
backup status or plan files, qsl-backup, runtime, protocol, crypto,
dependencies, workflows, public docs, website, README, START_HERE, qsl-server,
qsl-attachments, response archives, and durable Director State Index output.

## Required Queue Properties

- READY_COUNT is exactly `1`.
- The sole READY item is
  `NA-0409 -- QSL Local Ops qwork Director-Facing Startup Availability / tmux Safety Hardening`.
- `NA-0410 -- QSL Codex Ops Backup Coverage Manifest Verification / Status Update Plan`
  exists and is not READY.
- NA-0408 remains DONE.
- D-0799, D-0800, and D-0801 each exist exactly once.
- D-0802 exists exactly once.
- D-0803 and D-0804 are absent.

## Diagnostic Evidence

Record the operator-reported failure surface before implementation:

- bare `qwork` missing in a fresh shell;
- qwork core works after sourcing `/srv/qbuild/tools/qshell.sh`;
- interactive `set -e` fail-closed qshell wrapper exits before
  `shell-survived`;
- `/srv/qbuild/tools/qwork.sh` still returns nonzero for unsafe automation
  input;
- no backup or restore operation is run.

## Validation Commands

Run the following, or repo-local helper equivalents where available:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
git diff --check
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

Also run scope, link, leak, overclaim, and PR-body goal-lint checks. The PR
body must include a standalone line near the top:

```md
Goals: G4
```

## Pass Conditions

- The changed path set is exactly the Packet A allowed path set.
- Queue and decision helpers report the required properties above.
- Local validation passes.
- Required PR checks attach and pass normally.
- `public-safety` remains required and green before merge and after merge.
- The PR does not implement qwork hardening and does not implement NA-0410.
