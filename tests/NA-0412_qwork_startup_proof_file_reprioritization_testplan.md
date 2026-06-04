Goals: G4

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-04

# NA-0412 qwork Startup Proof File Reprioritization Testplan

## Objective

Validate the governance-only reroute that promotes NA-0412 as the qwork
startup proof-file handoff implementation lane while preserving the previously
READY backup status / plan update authorization lane as NA-0413.

## Scope

Allowed changed paths for this Packet A reroute:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0412_qwork_startup_proof_file_reprioritization_testplan.md`

Forbidden changes include qwork implementation files, qshell implementation
files, backup source lists, backup status or plan files, qsl-backup, runtime,
protocol, crypto, dependencies, workflows, public docs, website, README,
START_HERE, qsl-server, qsl-attachments, response archives, and durable
Director State Index output.

## Required Queue Properties

- READY_COUNT is exactly `1`.
- The sole READY item is
  `NA-0412 -- QSL Local Ops qwork Startup Proof File Handoff Implementation Harness`.
- `NA-0413 -- QSL Codex Ops Backup Status / Plan Update Authorization Plan`
  exists and is not READY.
- NA-0411 remains DONE.
- D-0808 exists exactly once.
- D-0809 exists exactly once.
- D-0810 exists exactly once.
- D-0811 is absent.
- D-0812 is absent.

## Reprioritization Evidence

Record:

- D247 stopped because the directive required the operator to paste qwork
  output into the directive body.
- The user requested file-based qwork startup proof instead of directive-body
  qwork output editing.
- The backup status / plan authorization lane is deferred, not discarded, and
  is preserved as NA-0413.
- Packet A does not implement qwork and does not implement NA-0413.
- No backup or restore operation is run.
- qsl-backup, backup source lists, backup status files, and backup plan files
  are not mutated.

## Validation Commands

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue --select NA-0411 --select NA-0412 --select NA-0413
python3 scripts/ci/qsl_evidence_helper.py decisions --select D-0808 --select D-0809 --select D-0810 --select D-0811 --select D-0812
git diff --name-only origin/main...HEAD
git diff --check
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allowed NEXT_ACTIONS.md --allowed DECISIONS.md --allowed TRACEABILITY.md --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md --allowed tests/NA-0412_qwork_startup_proof_file_reprioritization_testplan.md
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo fmt --check
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

Also run PR-body goal-lint/preflight with a standalone near-top line:

```md
Goals: G4
```

## Pass Conditions

- The changed path set is exactly the Packet A allowed path set.
- Queue and decision helpers report the required properties above.
- Local validation passes.
- Required PR checks attach and pass normally.
- `public-safety` remains required and green before merge and after merge.
- The PR does not implement qwork startup proof files and does not implement
  NA-0413 backup status / plan authorization work.
