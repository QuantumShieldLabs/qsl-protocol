Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-02

# NA-0406 Closeout and NA-0407 Restoration Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Verify that NA-0406 closes only after the source-list authorization PR merges
and post-merge public-safety is green, then restores the exact selected NA-0407
source-list implementation harness successor without implementing NA-0407.

## Protected Invariants

- READY_COUNT remains exactly 1.
- NA-0406 becomes DONE.
- READY becomes exactly:
  `NA-0407 -- QSL Codex Ops Backup Coverage / Source-List Implementation Harness`.
- D-0795 exists once.
- D-0796 is added once by closeout.
- D-0797 remains absent.
- Public-safety remains required and green.
- No durable Director State Index output is created.
- No backup/source-list mutation occurs during closeout.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0406_closeout_restore_na0407_testplan.md`

## Forbidden Scope

- `README.md`
- `START_HERE.md`
- `docs/public/**`
- `.github/**`
- `Cargo.toml`
- `Cargo.lock`
- `qsp/**`
- `qsc/**`
- `qsl/**`
- `qsl-client/**`
- `apps/**`
- `tools/**`
- `inputs/**`
- `formal/**`
- `scripts/**`
- `qsc-desktop/**`
- `qsl-server/**`
- `qsl-attachments/**`
- `website/**`
- runtime, service, protocol, crypto, qshield runtime, dependency, workflow,
  branch-protection, public-safety configuration, backup scripts, backup
  timers, fstab, source lists, backup status files, backup plans, local
  history, response archives, durable index output, branch deletion,
  secret-handling, and public technical paper files.

## Preconditions

- Packet N PR #1076 merged at `213afc580989`.
- Packet N head SHA was `0d5a5026b9e`.
- Post-merge public-safety on `213afc580989` completed success.
- READY_COUNT is 1 and READY is NA-0406 before closeout.
- D-0795 exists once.
- D-0796 is absent before closeout.
- Selected successor is exact:
  `NA-0407 -- QSL Codex Ops Backup Coverage / Source-List Implementation Harness`.

## Queue Requirements

After patch:

- `python3 scripts/ci/qsl_evidence_helper.py queue` reports READY_COUNT 1.
- READY line is NA-0407 with the selected source-list implementation harness
  title.
- NA-0406 is DONE.

## Decision Requirements

After patch:

- `python3 scripts/ci/qsl_evidence_helper.py decisions` reports latest D-0796.
- D-0795 exists once.
- D-0796 exists once.
- D-0797 is absent.
- duplicate decision count is zero.

## Boundary Requirements

Closeout must state:

- NA-0406 delivered Codex ops backup coverage / source-list authorization
  planning.
- NA-0407 is selected based on NA-0406 evidence.
- no NA-0407 implementation is authorized by closeout.
- runtime/security/public-claim boundaries remain protected.
- no durable local index is authorized by closeout.
- backup/source-list mutation remains future-scope only.

## Validation

Run:

- `git diff --check`
- queue/decision proof
- scope guard over the exact allowed closeout paths
- link-check
- leak-scan
- changed-line overclaim scan
- classifier proof
- PR body preflight / goal-lint
- `cargo audit --deny warnings`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`

Required PR checks must pass before merge, and post-merge public-safety must
complete successfully.

## Successor Handoff

NA-0407 must use the exact source-list authority identified by NA-0406. It must
not mutate backup scripts, source lists, status files, plan files, helper code,
fixtures, runtime, dependencies, workflows, public docs, website, response
archives, local history, or durable Director State Index paths unless future
live scope explicitly authorizes exact files and proof.
