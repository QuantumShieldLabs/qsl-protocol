# NA-0449 Closeout / Restore NA-0450 Testplan

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-09

## Objective

Verify NA-0449 closeout after fresh qwork proof recovery, PR #1167
post-merge public-safety success, and restoration of NA-0450 as the sole
READY successor.

## Protected invariants

- NA-0449 is DONE only after PR #1167 is merged and post-merge public-safety
  is success.
- NA-0450 is the sole READY item.
- NA-0434 and NA-0429 remain BLOCKED.
- D-0886 exists once and D-0887 remains absent until future NA-0450 work.
- Duplicate decision count remains zero.
- No public claim expands.

## Allowed scope

- `NEXT_ACTIONS.md`.
- `DECISIONS.md`.
- `TRACEABILITY.md`.
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.
- `tests/NA-0449_closeout_restore_na0450_testplan.md`.

## Forbidden scope

- No source, runtime, crypto, Cargo, lockfile, workflow, executable test,
  fuzz target, vector, formal model, qsl-server, qsl-attachments, qshield
  runtime, qshield-cli, website, public docs, README, START_HERE,
  qwork/qstart/qresume/qshell, qsl-backup, backup status file, backup plan
  file, rollback subtree, or backup tree mutation.
- No backup or restore execution.
- No public-readiness claim.
- No production-readiness claim.
- No public-internet-readiness claim.
- No external-review-complete claim.
- No crypto-complete claim.
- No RNG-failure-complete claim.
- No secret-material-complete claim.
- No side-channel-free claim.
- No vulnerability-free claim.
- No bug-free claim.
- No perfect-crypto claim.
- No metadata-free claim.
- No anonymity claim.
- No untraceable claim.
- No off-host-backup claim.
- No disaster-recovery claim.
- No restore-proof claim.
- No backup-complete claim.

## PR #1167 merge/public-safety checks

- Verify `gh pr view 1167 --repo QuantumShieldLabs/qsl-protocol --json number,state,mergedAt,mergeCommit,title,url,statusCheckRollup`.
- Required result: PR #1167 is MERGED with merge commit beginning
  `0a0c834b1514`.
- Verify merge-commit check-runs through REST polling only.
- Required result: public-safety is completed success and watched qsc/macOS
  checks are success or accepted by repo policy.

## Fresh qwork proof check

- Read `.qwork/startup.qsl-protocol.kv`.
- Read `.qwork/startup.qsl-protocol.json`.
- Required result: startup OK, lane NA-0449, repo qsl-protocol, path matches
  this worktree, clean-state fields are yes, READY_COUNT is 1, queue top is
  NA-0449, requested lane status is READY, and JSON mirrors `.kv`.
- Required result: proof HEAD and proof origin/main match live local HEAD and
  origin/main before fetch.
- Codex must not run qwork, qstart, or qresume.

## Stale qwork recovery check

- Verify D301 response exists and records the stale qwork stop.
- Verify D302 consumed fresh qwork proof and did not rerun qwork.
- Required result: D301 stale-proof recovery is recorded in D-0886,
  TRACEABILITY, and the rolling journal.

## cfg seam proof

- Run `RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test rng_failure_behavior -- --test-threads=1 --nocapture`.
- Required result: test passes and emits NA-0449 forced-failure markers for
  handshake no-pending-state, vault no-vault-write, and session-store
  no-session-write.

## Production semantics unchanged proof

- Run `cargo test -p qsc --locked --test rng_failure_behavior -- --test-threads=1 --nocapture`.
- Run qsc `send_commit`, `key_lifecycle_zeroization`, and
  `handshake_provider_error_no_mutation` regressions.
- Required result: normal builds do not activate the forced-failure seam and
  existing qsc behavior remains green.

## RNG-failure-complete caveat check

- Scan added closeout lines for forbidden overclaim language.
- Required result: selected NA-0449 evidence is represented as bounded internal
  proof only; no RNG-failure-complete claim is introduced.

## qshield-cli demo boundary check

- Verify D-0886 and TRACEABILITY keep qshield-cli demo RNG boundary as
  residual/backlog evidence only.
- Required result: qshield-cli demo evidence is not represented as qsc runtime
  RNG failure proof.

## refimpl/provider RNG backlog check

- Verify D-0886 and TRACEABILITY preserve refimpl/provider RNG failure as
  backlog/residual.
- Required result: refimpl/provider evidence is not represented as qsc runtime
  RNG failure proof.

## Root cargo audit green check

- Run `cargo audit --deny warnings`.
- Required result: PASS.
- Caveat: audit green is dependency-health evidence only, not public readiness.
- Caveat: audit green is not vulnerability-free proof.

## Nested fuzz lock audit green check

- Run `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`.
- Required result: PASS.
- Caveat: nested audit green is dependency-health evidence only.

## NA-0449 DONE / NA-0450 READY check

- Run `python3 scripts/ci/qsl_evidence_helper.py queue`.
- Required result: READY_COUNT 1, READY NA-0450, NA-0449 DONE, NA-0434
  BLOCKED, and NA-0429 BLOCKED.

## qsl-protocol closeout scope guard

- Run `git diff --name-only` before commit and compare against the allowed
  closeout path list.
- Required result: only the five allowed closeout paths are changed.

## No source/workflow/test/lockfile/vector/formal mutation

- Run a forbidden path guard over `git diff --name-only`.
- Required result: no source, workflow, executable test outside this
  governance testplan, lockfile, vector, fuzz target, or formal model path is
  changed by closeout.

## No public overclaim

- Run added-line overclaim scan.
- Required result: no affirmative public-readiness claim.
- Required result: no affirmative production-readiness claim.
- Required result: no affirmative public-internet-readiness claim.
- Required result: no affirmative external-review-complete claim.
- Required result: no affirmative crypto-complete claim.
- Required result: no affirmative RNG-failure-complete claim.
- Required result: no affirmative secret-material-complete claim.
- Required result: no affirmative side-channel-free claim.
- Required result: no affirmative vulnerability-free claim.
- Required result: no affirmative bug-free claim.
- Required result: no affirmative perfect-crypto claim.
- Required result: no affirmative metadata-free claim.
- Required result: no affirmative anonymity claim.
- Required result: no affirmative untraceable claim.
- Required result: no affirmative off-host-backup claim.
- Required result: no affirmative disaster-recovery claim.
- Required result: no affirmative restore-proof claim.
- Required result: no affirmative backup-complete claim.
