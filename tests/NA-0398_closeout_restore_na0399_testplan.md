Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-01

# NA-0398 Closeout Restore NA-0399 Test Plan

## Objective

Verify that NA-0398 is closed after the metadata privacy / secure messaging
claim-boundary plan merges, and that the exact selected NA-0399 successor is
restored without implementing NA-0399.

## Protected Invariants

- Exactly one READY item exists after closeout.
- The sole READY item is `NA-0399 -- QSL Backup / Restore / Key Custody External Guidance Mapping Plan`.
- NA-0398 is DONE.
- D-0778 and D-0779 each exist once.
- D-0780 is absent.
- qsl-protocol remains the only mutable repository.
- No runtime, protocol, crypto, dependency, workflow, public docs, website,
  qshield runtime, qsl-server, qsl-attachments, qsc-desktop, backup script,
  timer, fstab, off-host setup, real backup, real restore, real key handling,
  response archive, local tool, or secret-bearing path is changed.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0398_closeout_restore_na0399_testplan.md`

## Forbidden Scope

- `.github/**`
- `Cargo.toml`
- `Cargo.lock`
- `qsc/**`
- `qsp/**`
- `qsl/**`
- `apps/**`
- `tools/**`
- `inputs/**`
- `formal/**`
- `scripts/**`
- `qshield` runtime paths
- `qsl-server/**`
- `qsl-attachments/**`
- `qsc-desktop/**`
- `website/**`
- `README.md`
- `START_HERE.md`
- `docs/public/**`
- `/home/victor/work/qsl/codex/**`
- `/srv/qbuild/tools/**`
- backup scripts, timers, fstab, services, remote targets, credentials, keys,
  repositories, restore targets, deploy, rollback, backup, or restore state.

## Closeout Requirements

- Record qsl-protocol PR #1059 head `27f6d562a98` and merge `4859cdc524aa`.
- Record post-merge public-safety success on `4859cdc524aa`.
- Add D-0779 with status Accepted.
- Mark NA-0398 DONE.
- Promote only the selected NA-0399 successor.
- State that no NA-0399 implementation is authorized by closeout.

## Successor Requirements

- The NA-0399 objective maps external backup, restore, key custody, key
  recovery, off-host backup, and disaster-recovery guidance into current QSL
  evidence boundaries.
- The NA-0399 objective forbids real backup, restore, key, off-host, remote,
  repository-init, host-key-scan, credential, deploy, and rollback operations
  unless a future exact scope authorizes them.
- The NA-0399 objective forbids disaster-recovery-complete,
  off-host-backup-complete, restore-proven, key-custody-complete, and
  key-recovery-complete claims.

## Public Claim Boundary

- Closeout must not claim production readiness, public-internet readiness,
  external-review completion, complete disaster recovery, off-host backup
  completion, restore proof, key custody completion, metadata-free behavior,
  anonymity, untraceability, hidden timing, hidden traffic shape, hidden
  attachment size, vulnerability-free status, bug-free status, or perfect
  crypto.

## Backup Impact

- No backup-plan update is required if the closeout changes only the allowed
  qsl-protocol governance/testplan paths.
- Future durable backup, restore, key-custody, response, report, target,
  monitoring, or local-ops artifacts require separate backup-impact review.
- Same-host continuity must not be described as complete disaster recovery.

## Required Local Checks

- `git status --porcelain=v1 --branch`
- `git diff --name-only origin/main...HEAD`
- `git diff --check`
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- scope guard for the exact allowed path set.
- link-check.
- leak-scan.
- goal-lint.
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`

## CI Expectations

- Open a qsl-protocol closeout PR.
- Required checks must attach and pass without admin bypass.
- Merge with `--merge --match-head-commit`.
- Do not squash, rebase, force-push, amend after PR creation, direct-push to
  main, or use branch deletion flags.
- After merge, post-merge public-safety must complete successfully.

## Successor Handoff

- Final queue state: READY_COUNT 1, READY NA-0399, NA-0398 DONE.
- NA-0399 remains unimplemented.
