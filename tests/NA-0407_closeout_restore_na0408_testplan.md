Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-03

# NA-0407 Closeout and NA-0408 Restoration Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Verify that NA-0407 closes only after the source-list implementation evidence PR
merges and post-merge public-safety is green, then restores the exact selected
NA-0408 manifest verification / status update successor without implementing
NA-0408.

## Protected Invariants

- READY_COUNT remains exactly 1.
- NA-0407 becomes DONE.
- READY becomes exactly:
  `NA-0408 -- QSL Codex Ops Backup Coverage Manifest Verification / Status Update Plan`.
- D-0797 exists once.
- D-0798 is added once by closeout.
- Public-safety remains required and green.
- No durable Director State Index output is created.
- No qsl-backup, backup status, backup plan, backup operation, or restore
  operation is changed or run during closeout.
- Same-host continuity remains caveated.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0407_closeout_restore_na0408_testplan.md`

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

- Packet E PR #1078 merged at `cf0341324ab3`.
- Packet E head SHA was `81c746a80756`.
- Post-merge public-safety on `cf0341324ab3` completed success.
- READY_COUNT is 1 and READY is NA-0407 before closeout.
- D-0797 exists once.
- D-0798 is absent before closeout.
- Selected successor is exact:
  `NA-0408 -- QSL Codex Ops Backup Coverage Manifest Verification / Status Update Plan`.

## Queue Requirements

After patch:

- `python3 scripts/ci/qsl_evidence_helper.py queue` reports READY_COUNT 1.
- READY line is NA-0408 with the selected manifest verification / status update
  plan title.
- NA-0407 is DONE.

## Decision Requirements

After patch:

- `python3 scripts/ci/qsl_evidence_helper.py decisions` reports latest D-0798.
- D-0797 exists once.
- D-0798 exists once.
- duplicate decision count is zero.

## Boundary Requirements

Closeout must state:

- NA-0407 delivered Codex ops source-list implementation validation.
- NA-0408 is selected based on NA-0407 evidence.
- no NA-0408 implementation is authorized by closeout.
- runtime/security/public-claim boundaries remain protected.
- no durable local index is authorized by closeout.
- backup status and backup plan updates require future exact scope and
  manifest/status evidence.
- backup/source-list mutation and backup/restore execution remain out of scope.

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
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- post-merge public-safety proof after the closeout PR merges
