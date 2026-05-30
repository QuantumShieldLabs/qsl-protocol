Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-30

# NA-0385 Closeout and NA-0386 Restoration Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Close NA-0385 after the response archive backup coverage / real-archive write
authorization evidence has merged, and restore exactly one READY successor:
`NA-0386 -- QSL Local Ops Response Writer Real-Archive Write Implementation Harness`.

## Protected invariants

- Exactly one READY item exists after closeout.
- NA-0385 is DONE.
- NA-0386 is READY.
- D-0752 exists once.
- D-0753 exists once after closeout.
- D-0754 is absent.
- NA-0386 is not implemented by this closeout.
- No real response archive write is performed by this closeout.
- No response index or history index is created.
- No runtime, service, protocol, crypto, workflow, dependency, backup script,
  timer, fstab, qsl-server, qsl-attachments, qshield runtime, website, README,
  START_HERE, docs/public, public-safety, branch-protection, secret, key,
  credential, restore, deploy, rollback, or target setup mutation is authorized.

## Allowed scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0385_closeout_restore_na0386_testplan.md`

## Forbidden scope

- `.github/**`
- `Cargo.toml`
- `Cargo.lock`
- `scripts/**`
- `inputs/**`
- `qsc/**`
- `qsp/**`
- `qsl/**`
- `qsl-client/**`
- `apps/**`
- `tools/**`
- `formal/**`
- `qsc-desktop/**`
- `qsl-server/**`
- `qsl-attachments/**`
- `website/**`
- `docs/public/**`
- `README.md`
- `START_HERE.md`
- backup scripts, timers, fstab, service units, local backup configuration
- `/home/victor/work/qsl/codex/**`
- `/srv/qbuild/tools/**`

## Required local checks

- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- scope guard proving only allowed closeout paths changed
- `python3 scripts/ci/qsl_evidence_helper.py link-check --root .`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- `cargo audit --deny warnings`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- PR-body preflight and goal-lint before PR creation

## Success criteria

- Queue helper reports READY_COUNT `1` and READY `NA-0386`.
- Decisions helper reports latest D-0753, D-0753 once, D-0754 absent, and
  duplicate count zero.
- Scope guard reports zero forbidden paths.
- Link-check, leak-scan, cargo audit, qsc send_commit, and formal checks pass.
- PR body includes Goals, Impact, No-regression, and Tests/Vectors.
- Required GitHub checks complete green before merge.
- Post-merge public-safety completes green.

## Successor handoff

NA-0386 should implement the bounded response writer real-archive write harness
authorized by NA-0385, with explicit real-archive authorization, backup-impact
proof, no-secret scan, no-overwrite behavior, path/checksum proof, and no
runtime/workflow/dependency drift. This closeout does not implement NA-0386.
