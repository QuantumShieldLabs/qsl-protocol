Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-30

# NA-0384 Closeout and NA-0385 Restoration Testplan

## Objective

Close NA-0384 after the response writer temp-output harness has merged, and
restore exactly one READY successor:
`NA-0385 -- QSL Local Ops Response Archive Backup Coverage / Real-Archive Write Authorization Plan`.

## Protected Invariants

- Exactly one READY item exists after closeout.
- NA-0384 is DONE.
- NA-0385 is READY.
- D-0750 exists once.
- D-0751 exists once.
- D-0752 is absent.
- NA-0385 is not implemented by this closeout.
- No real response archive write is authorized by this closeout.
- No response index or history index is created.
- No runtime, service, protocol, crypto, workflow, dependency, backup script,
  timer, fstab, qsl-server, qsl-attachments, qshield runtime, website, README,
  START_HERE, docs/public, public-safety, branch-protection, secret, key,
  credential, restore, deploy, rollback, or target setup mutation is authorized.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0384_closeout_restore_na0385_testplan.md`

## Forbidden Scope

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

## Required Local Checks

- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- scope guard proving only allowed closeout paths changed
- `python3 scripts/ci/qsl_evidence_helper.py link-check --root .`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- `cargo audit --deny warnings`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- goal-lint / PR-body preflight before PR creation

## Success Criteria

- Queue helper reports READY_COUNT `1` and READY `NA-0385`.
- Decisions helper reports latest D-0751, D-0751 once, D-0752 absent, and duplicate count zero.
- Scope guard reports zero forbidden paths.
- Link-check, leak-scan, cargo audit, qsc send_commit, and formal checks pass.
- PR body includes Goals, Impact, No-regression, and Tests/Vectors.
- Required GitHub checks complete green before merge.
- Post-merge public-safety completes green.

## Successor Handoff

NA-0385 should decide whether and how the response writer may write to the real
Codex response archive, including backup coverage, no-secret, no-overwrite, and
local-history boundaries. It must not inherit authorization to implement real
archive writes from this closeout.
