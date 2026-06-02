Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-02

# NA-0404 Closeout and NA-0405 Restoration Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Verify that NA-0404 closes only after the storage authorization PR merges and
post-merge public-safety is green, then restores the exact selected NA-0405
backup coverage / authority blocker successor without implementing NA-0405.

## Protected Invariants

- READY_COUNT remains exactly 1.
- NA-0404 becomes DONE.
- READY becomes exactly:
  `NA-0405 -- QSL Director State Index Durable Storage Backup Coverage / Authority Blocker Resolution`.
- D-0791 exists once.
- D-0792 is added once by closeout.
- D-0793 remains absent.
- Public-safety remains required and green.
- No durable Director State Index output is created.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0404_closeout_restore_na0405_testplan.md`

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
  timers, fstab, source lists, local history, response archives, durable index
  output, branch deletion, secret-handling, and public technical paper files.

## Preconditions

- Packet O PR #1072 merged at `f4ebd89b69e0`.
- Packet O head SHA was `ee3cbaf92ad6`.
- Post-merge public-safety on `f4ebd89b69e0` completed success.
- READY_COUNT is 1 and READY is NA-0404 before closeout.
- D-0791 exists once.
- D-0792 is absent before closeout.
- Selected successor is exact.

## Queue Requirements

After patch:

- `python3 scripts/ci/qsl_evidence_helper.py queue` reports READY_COUNT 1.
- READY line is NA-0405 with the selected blocker title.
- NA-0404 is DONE.

## Decision Requirements

After patch:

- `python3 scripts/ci/qsl_evidence_helper.py decisions` reports latest D-0792.
- D-0791 exists once.
- D-0792 exists once.
- D-0793 is absent.
- duplicate decision count is zero.

## Boundary Requirements

Closeout must state:

- NA-0404 delivered durable Director State Index storage / backup-impact
  authorization planning.
- NA-0405 is selected based on NA-0404 evidence.
- no NA-0405 work is authorized by closeout.
- runtime/security/public-claim boundaries remain protected.
- no durable local index is authorized by closeout.

## Validation

Run:

- `git diff --check`
- queue/decision proof
- scope guard over the exact allowed closeout paths
- link-check
- leak-scan
- changed-line overclaim scan
- classifier proof
- PR body preflight or goal-lint
- `cargo audit --deny warnings`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`

Required PR checks must pass before merge, and post-merge public-safety must
complete successfully.

## Successor Handoff

NA-0405 must resolve Codex ops durable index backup coverage / authority before
any future durable local Director State Index write is authorized.
