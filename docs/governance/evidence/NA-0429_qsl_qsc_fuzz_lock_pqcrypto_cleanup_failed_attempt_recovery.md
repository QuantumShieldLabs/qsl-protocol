Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-05

# NA-0429 qsc Fuzz Lock pqcrypto Cleanup Failed Attempt Recovery

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0429 is recovered as BLOCKED, not DONE, after the lockfile-only cleanup
attempt in PR #1127 failed the active qsc adversarial fuzz validation path.
The PR is closed unmerged, its branch is intentionally retained for evidence,
and the queue is rerouted to the exact successor:

`NA-0430 -- QSL qsc Adversarial Fuzz Validation Blocker Triage Plan`

This recovery does not accept the failed lockfile cleanup as remediation and
does not mutate lockfiles, source code, workflows, scripts, tests, vectors,
runtime, crypto, qsl-server, qsl-attachments, qshield runtime, website, public
docs, README, START_HERE, backup, restore, or qsl-backup paths.

## D270 inheritance

D270 attempted the NA-0428-authorized nested qsc fuzz lockfile-only cleanup and
opened PR #1127 from branch `na-0429-qsc-fuzz-lock-pqcrypto-cleanup` at head
`967c95c37fea`. D270 stopped because `qsc-adversarial-smoke` failed in CI in a
dependency/fuzz-tooling build path. The D270 response file remains available at
the operator response archive path for NA-0429 D270 and records:

- PR #1127 failed `qsc-adversarial-smoke`.
- The failure involved `ml-dsa 0.1.0-rc.7` with `pkcs8 0.11.0` errors.
- The attempted lockfile-only cleanup was not merged.
- The local fuzz lock rollback restored the preimage SHA.
- The recommended successor was qsc adversarial fuzz validation blocker triage.

## Stopped-state preservation

Before resetting the local checkout, D271 preserved the stopped state under
proof root:

`/srv/qbuild/tmp/NA0429_failed_cleanup_recovery_20260605T201535-0500`

Captured stopped-state evidence includes:

- local and UTC timestamps;
- `pwd`;
- `git status --porcelain=v1 --branch`;
- `git branch --show-current`;
- `git rev-parse HEAD`;
- `git rev-parse origin/main`;
- `git diff --name-only`;
- full local diff;
- fuzz lock diff;
- untracked-file inventory;
- PR #1127 metadata, file list, and diff;
- failed check metadata and a non-secret job-log excerpt.

Recovered command-shape failure: the first PR metadata command asked `gh pr
view` for an unsupported JSON field named `checks`. This was corrected by using
`statusCheckRollup` and a separate `gh pr checks` capture. Final result: PR
#1127 metadata, checks, failed-job URL, and log excerpt were preserved.

Recovered reset-shape failure: the first `git checkout -B main origin/main`
refused to switch because the stopped branch still carried the preserved local
fuzz lock diff. Because the directive explicitly required preserving that state
and then resetting the local checkout to clean `origin/main`, Codex used the
forced checkout form to complete the requested reset. Final result: branch
`main` matched `origin/main` and the worktree was clean before recovery edits.

## PR #1127 failure summary

PR #1127 file list:

- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/governance/evidence/NA-0429_qsl_qsc_fuzz_lock_pqcrypto_residual_lockfile_cleanup_implementation_harness.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `tests/NA-0429_qsl_qsc_fuzz_lock_pqcrypto_residual_lockfile_cleanup_implementation_testplan.md`

PR #1127 failed exactly the active qsc adversarial smoke validation. The failed
job URL captured in the proof root is:

`https://github.com/QuantumShieldLabs/qsl-protocol/actions/runs/27046664839/job/79833893300`

The job excerpt shows the fuzz build downloaded and compiled `pkcs8 v0.11.0`
and `ml-dsa v0.1.0-rc.7`, then failed with Rust `E0277` conversion errors in
`ml-dsa` `pkcs8.rs` before the `qsc_route_http` fuzz binary could build.

## PR #1127 closure proof

PR #1127 was open, unmerged, and unstable before recovery closure:

- branch: `na-0429-qsc-fuzz-lock-pqcrypto-cleanup`;
- head: `967c95c37fea`;
- merge commit: none;
- mergedAt: null;
- failed check: `qsc-adversarial-smoke`.

Codex posted a recovery comment on PR #1127 explaining the failed
qsc-adversarial-smoke result, dependency/lockfile/fuzz-tooling classification,
unmerged status, NA-0430 reroute, and intentional branch retention. Codex then
closed PR #1127 without deleting the branch. Post-close proof reports state
`CLOSED`, `mergedAt: null`, and `mergeCommit: null`.

## Rollback proof

D270 recorded local rollback of the nested fuzz lock to its preimage after the
failed cleanup attempt. D271 preserved that stopped branch state before cleanup
and then reset the local checkout to clean `origin/main`. The recovery PR does
not alter `qsl/qsl-client/qsc/fuzz/Cargo.lock` or any other lockfile.

## Origin/main queue truth

After resetting to clean `origin/main` at `c621ff09df61`, live helpers reported:

- `READY_COUNT 1`;
- `READY NA-0429`;
- NA-0428 DONE;
- latest decision D-0845;
- D-0844 exists once;
- D-0845 exists once;
- D-0846 absent before the recovery patch;
- duplicate decision count zero.

Public-safety on `c621ff09df61` completed success.

Root dependency health before the recovery patch:

- `cargo audit --deny warnings` passed;
- `cargo tree -i rustls-webpki --locked` reported `rustls-webpki v0.103.13`;
- root `ml-kem v0.2.1` remained active;
- root `pqcrypto-mlkem`, `pqcrypto-traits`, and `pqcrypto-internals` package IDs
  were absent.

## Why NA-0429 is BLOCKED, not DONE

NA-0429 acceptance required the lockfile-only cleanup to preserve both nested
fuzz lock audit health and qsc adversarial/fuzz validation. PR #1127 made the
nested lock audit green but failed the active fuzz build. Because the failure
appears dependency/lockfile/fuzz-tooling related, the implementation acceptance
criteria were not satisfied.

Therefore:

- NA-0429 is BLOCKED.
- NA-0429 is not DONE.
- PR #1127 remains closed and unmerged.
- The failed lockfile cleanup is not accepted as remediation.
- The next lane must triage the fuzz validation blocker before any retry or
  remediation scope is authorized.

## Recovery classification

`NA0429_LOCKFILE_ONLY_CLEANUP_FAILED_CI_FUZZ_TOOLING_BLOCKER`

## NA-0430 successor rationale

NA-0430 is restored as the sole READY successor because the next useful action
is not another lockfile refresh. The queue needs a read-only/focused triage lane
to determine why an audit-green nested fuzz lock selected a dependency set that
failed cargo-fuzz build validation and to authorize the next exact remediation
or retry lane.

## Public claim/external review/website boundary

This recovery is internal governance and dependency/fuzz-tooling triage
evidence only. It is not public readiness, production readiness,
public-internet readiness, external-review completion, crypto completeness,
side-channel assurance, bug absence, vulnerability absence, or perfect crypto
proof. Cargo audit green remains dependency-health evidence only.

No README, START_HERE, public docs, website, public technical paper, or public
claim surface is updated.

## No runtime/no dependency/no workflow/no test/no vector mutation proof

Allowed changed paths for this recovery PR are limited to:

- `NEXT_ACTIONS.md`;
- `DECISIONS.md`;
- `TRACEABILITY.md`;
- `docs/governance/evidence/NA-0429_qsl_qsc_fuzz_lock_pqcrypto_cleanup_failed_attempt_recovery.md`;
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`;
- `tests/NA-0429_failed_cleanup_recovery_restore_na0430_testplan.md`.

The recovery PR changes no runtime code, crypto code, dependency manifests,
lockfiles, workflows, scripts, fuzz target source, executable tests, vectors,
service paths, public surfaces, backup paths, qwork/qstart/qresume/qshell
paths, or qsl-backup paths.

## Backup impact statement

No backup or restore was run. qsl-backup remained at the expected checksum
prefix `e9ecff3d22ed`, and the Codex ops source-list inclusion count remained
1. This recovery changes only tracked qsl-protocol governance files and does
not require backup-plan or backup-status mutation.

## Next recommendation

Proceed with NA-0430 as a qsc adversarial fuzz validation blocker triage lane.
NA-0430 should preserve read-only/source-tree boundaries until it can classify
the PR #1127 dependency/fuzz-tooling failure and recommend an exact future
remediation or retry scope.
