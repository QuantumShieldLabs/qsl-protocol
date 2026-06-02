Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-02

# NA-0405 Closeout and NA-0406 Restoration Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Verify that NA-0405 closes only after the backup coverage / authority blocker
PR merges and post-merge public-safety is green, then restores the exact
selected NA-0406 source-list authorization successor without implementing
NA-0406.

## Protected Invariants

- READY_COUNT remains exactly 1.
- NA-0405 becomes DONE.
- READY becomes exactly:
  `NA-0406 -- QSL Codex Ops Backup Coverage / Source-List Authorization Plan`.
- D-0793 exists once.
- D-0794 is added once by closeout.
- D-0795 remains absent.
- Public-safety remains required and green.
- No durable Director State Index output is created.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0405_closeout_restore_na0406_testplan.md`

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

- Packet M PR #1074 merged at `6ccf51542dab`.
- Packet M head SHA was `c904470fe1ba`.
- Post-merge public-safety on `6ccf51542dab` completed success.
- READY_COUNT is 1 and READY is NA-0405 before closeout.
- D-0793 exists once.
- D-0794 is absent before closeout.
- Selected successor is exact.

## Queue Requirements

After patch:

- `python3 scripts/ci/qsl_evidence_helper.py queue` reports READY_COUNT 1.
- READY line is NA-0406 with the selected source-list authorization title.
- NA-0405 is DONE.

## Decision Requirements

After patch:

- `python3 scripts/ci/qsl_evidence_helper.py decisions` reports latest D-0794.
- D-0793 exists once.
- D-0794 exists once.
- D-0795 is absent.
- duplicate decision count is zero.

## Boundary Requirements

Closeout must state:

- NA-0405 delivered Director State Index durable-storage backup coverage /
  authority blocker resolution.
- NA-0406 is selected based on NA-0405 evidence.
- no NA-0406 implementation is authorized by closeout.
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

NA-0406 must resolve Codex ops backup coverage / source-list authority before
any future durable local Director State Index write is authorized. It must not
implement durable index output unless future live scope explicitly authorizes
that implementation.
