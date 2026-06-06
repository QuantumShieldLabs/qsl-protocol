Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-05

# NA-0429 Failed Cleanup Recovery / Restore NA-0430 Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that the failed NA-0429 lockfile-only cleanup attempt is recovered
truthfully: PR #1127 is preserved, closed, and left unmerged; NA-0429 is marked
BLOCKED, not DONE; and `NA-0430 -- QSL qsc Adversarial Fuzz Validation Blocker
Triage Plan` is restored as the sole READY item without lockfile, source,
workflow, script, test, vector, dependency, runtime, crypto, service, public,
backup, or qsl-backup mutation.

## Protected invariants

- The failed PR #1127 attempt is not merged.
- The PR #1127 branch is not deleted.
- NA-0429 remains not DONE because acceptance criteria failed.
- NA-0430 is the only READY item.
- D-0846 is recovery, not implementation acceptance.
- Root dependency health remains green.
- Public-safety remains green before merge and after merge.
- Public claim boundaries do not expand.

## Allowed scope

Allowed changed paths:

- `NEXT_ACTIONS.md`;
- `DECISIONS.md`;
- `TRACEABILITY.md`;
- `docs/governance/evidence/NA-0429_qsl_qsc_fuzz_lock_pqcrypto_cleanup_failed_attempt_recovery.md`;
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`;
- `tests/NA-0429_failed_cleanup_recovery_restore_na0430_testplan.md`.

No other qsl-protocol paths may change.

## Forbidden scope

This recovery must not:

- mutate `qsl/qsl-client/qsc/fuzz/Cargo.lock`;
- mutate `qsl/qsl-client/qsc/fuzz/Cargo.toml`;
- mutate root `Cargo.toml` or `Cargo.lock`;
- mutate `qsl/qsl-client/qsc/Cargo.toml`;
- mutate runtime code;
- mutate crypto code;
- mutate dependency manifests or lockfiles;
- mutate workflows or scripts;
- mutate tests other than this governance testplan;
- mutate fuzz target source;
- mutate vectors;
- mutate qsl-server, qsl-attachments, qshield runtime, website, public docs,
  README, or START_HERE paths;
- run qwork, qstart, or qresume;
- run backup or restore;
- mutate qsl-backup;
- mutate backup status or backup plan files;
- mutate qwork/qstart/qresume/qshell;
- delete the PR #1127 branch;
- merge PR #1127;
- handle secret material.

Forbidden claim outcomes include public readiness, production readiness,
public-internet readiness, external-review completion, crypto completeness,
side-channel assurance, bug absence, vulnerability absence, perfect crypto,
off-host backup completion, disaster recovery completion, restore proof, backup
completion, anonymity, metadata-free behavior, or untraceability.

## PR #1127 preservation/closure checks

Required checks:

- Preserve stopped local state before reset.
- Capture PR #1127 metadata, file list, diff, failed check summary, and
  non-secret failed-job excerpt.
- Verify PR #1127 was open or otherwise unmerged before closure.
- Comment on PR #1127 explaining the failed qsc-adversarial-smoke result and
  recovery reroute.
- Close PR #1127 without deleting the branch.
- Verify PR #1127 state is closed and `mergedAt` remains null.

## NA-0429 blocked-not-DONE check

Required checks:

- `NEXT_ACTIONS.md` lists `NA-0429` with `Status: BLOCKED`.
- `NEXT_ACTIONS.md` does not mark NA-0429 DONE.
- D-0846 states the failed lockfile-only attempt is not accepted as
  remediation.
- D-0846 records classification
  `NA0429_LOCKFILE_ONLY_CLEANUP_FAILED_CI_FUZZ_TOOLING_BLOCKER`.

## NA-0430 READY check

Required checks:

- Queue helper reports `READY_COUNT 1`.
- Queue helper reports READY NA-0430.
- NA-0430 block includes no-runtime, no-crypto, no-dependency, no-Cargo,
  no-lockfile, no-workflow, no-test, no-vector, no-public-overclaim,
  no-backup/restore, and no-secret boundaries.
- D-0847 is absent before the future NA-0430 lane.

## qsl-protocol scope guard

Required path guard:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/governance/evidence/NA-0429_qsl_qsc_fuzz_lock_pqcrypto_cleanup_failed_attempt_recovery.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0429_failed_cleanup_recovery_restore_na0430_testplan.md`

Any changed path outside this set is a stop condition.

## No runtime/dependency/workflow/test/vector mutation

Required checks:

- `git diff --name-only` shows only the allowed recovery paths.
- No `Cargo.toml` or `Cargo.lock` path changes.
- No qsc fuzz source path changes.
- No workflow or script path changes.
- No runtime or crypto source path changes.
- No qsl-server, qsl-attachments, qshield runtime, website, public docs,
  README, or START_HERE path changes.

## Root cargo audit green

Required checks:

- `cargo audit --deny warnings` passes.
- `cargo tree -i rustls-webpki --locked` reports `rustls-webpki v0.103.13` or
  newer safe version.
- Root `pqcrypto-mlkem`, `pqcrypto-traits`, and `pqcrypto-internals` package IDs
  remain absent from the locked graph.

## Public-safety requirements

Required checks:

- public-safety is green on clean `origin/main` before the recovery patch.
- public-safety is green on the recovery PR before merge.
- public-safety is green on the merge commit after merge.

## No public overclaim requirements

Required checks:

- No public technical paper content is created.
- No README, START_HERE, public docs, or website path changes.
- Recovery evidence states cargo audit green is dependency-health evidence only.
- Recovery evidence does not imply production readiness, public-internet
  readiness, external-review completion, crypto completeness, side-channel
  assurance, bug absence, vulnerability absence, perfect crypto, backup
  completion, off-host backup completion, disaster recovery completion, restore
  proof, anonymity, metadata-free behavior, or untraceability.
