Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-28

# NA-0378 QSL Local Ops qstart/qresume Fast-Forward Guard Harness Test Plan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0378 adds a bounded local qstart/qresume fast-forward guard for
qsl-protocol worktrees, proves it with local temp repositories, and records
qsl-protocol companion evidence without runtime, service, dependency, workflow,
backup-configuration, secret, target, or public-claim drift.

## Protected Invariants

- READY remains NA-0378 during the implementation evidence PR.
- READY_COUNT remains 1.
- D-0738 exists once after the implementation evidence PR.
- D-0739 remains absent until optional closeout.
- qsl-protocol changes are governance/testplan/journal only.
- The only authorized non-repo mutable local file is
  `/srv/qbuild/tools/qshell.sh`.
- qstart/qresume rejects unsafe qsl-protocol worktrees fail-closed.
- qstart/qresume never uses reset-hard, force push, force-with-lease, branch
  deletion, checkout force, clean-force, rebase, amend, or user-worktree
  removal.

## Allowed Scope

- `/srv/qbuild/tools/qshell.sh`
- `/srv/qbuild/tools/backups/NA0378/**`
- `/srv/qbuild/tmp/NA0378_*`
- `/srv/qbuild/tmp/NA0378_qshell.patch`
- `docs/governance/evidence/NA-0378_qsl_local_ops_qstart_qresume_fast_forward_guard_harness.md`
- `tests/NA-0378_qsl_local_ops_qstart_qresume_fast_forward_guard_harness_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

No changes are allowed in:

- qsl-server
- qsl-attachments
- qshield runtime paths
- qsc/qsp/qsl runtime paths
- protocol, crypto, key schedule, auth, or wire semantics
- `.github/**`
- `Cargo.toml`
- `Cargo.lock`
- dependency files
- website or external website repositories
- `README.md`
- `START_HERE.md`
- `docs/public/**`
- `/usr/local/sbin/qsl-backup`
- backup scripts, timers, fstab, services, source lists, keys, credentials, or
  secret material
- `/home/victor/work/qsl/codex/**` except the required final D197 response file

## qshell Backup and Rollback Requirements

Before qshell mutation, record:

- SHA-256 checksum;
- file mode, owner, and timestamps;
- qstart/qresume source authority;
- syntax check result.

Create a rollback copy under `/srv/qbuild/tools/backups/NA0378/` preserving
mode and timestamp. The rollback filename must include timestamp and original
SHA prefix. Record backup path and checksum, and prove it matches the original.

Generate a before/after patch at `/srv/qbuild/tmp/NA0378_qshell.patch`.

## Source and Authority Requirements

The evidence must prove:

- `/srv/qbuild/tools/qshell.sh` exists and is writable;
- qshell syntax is valid before and after mutation;
- sourcing qshell exposes `qstart` and `qresume`;
- both qstart and qresume call the fast-forward guard or equivalent exact
  helper before handoff.

## Guard Semantic Requirements

The guard must:

- apply only to qsl-protocol worktrees;
- fetch `origin/main` metadata;
- check `QSL_EXPECTED_MAIN_SHA` when set and reject mismatches before local
  branch mutation;
- reject tracked modifications;
- reject staged/index modifications;
- reject untracked non-ignored files;
- reject detached HEAD;
- reject non-main branch states;
- no-op when local HEAD already equals `origin/main`;
- fast-forward only when local HEAD is an ancestor of `origin/main`;
- reject ahead/diverged histories;
- print local HEAD, origin/main, clean status, and guard result;
- return nonzero on reject.

## Harness Scenario Requirements

Use local temporary git repositories only under `/srv/qbuild/tmp/NA0378_*`.

Required scenarios:

- source qshell and prove qstart, qresume, and helper exist;
- clean stale local qsl-protocol worktree fast-forwards to local test
  `origin/main`;
- already-current local qsl-protocol worktree no-ops;
- dirty tracked file rejects;
- untracked file rejects;
- expected-main SHA mismatch rejects;
- diverged local branch rejects;
- dirty tracked file and untracked file are preserved;
- qstart/qresume integration is statically proven;
- rollback backup exists and checksum matches original.

## Negative and Fail-Closed Requirements

For every reject scenario, the harness must prove:

- nonzero guard result;
- HEAD unchanged;
- local file state preserved;
- no reset, force, clean-force, checkout-force, rebase, amend, or branch
  deletion behavior.

## No-Forbidden-Command Requirements

Scan the qshell patch and resulting qshell file for:

- `git reset --hard`
- `push --force`
- `force-with-lease`
- `branch -D`
- `clean -fd`
- `checkout -f`
- `git rebase`
- `commit --amend`
- `rm -rf`

Result must be zero matches.

## Backup-Impact Requirements

The evidence must classify backup-plan impact and must not present the local
same-host continuity backup as complete disaster recovery. Expected result: no
backup-plan update is required if `/srv/qbuild/tools` remains in same-host
continuity backup coverage and qshell rollback/patch proof is recorded.

## Public-Claim Boundary Requirements

Evidence must not introduce production-readiness, public-internet-readiness,
external-review-complete, metadata-free, anonymity, untraceable,
disaster-recovery-complete, off-host-backup-complete, restore-complete,
target-configured, host-identity-verified, key-custody, or key-recovery claims.

## Successor Selection Requirements

Selected successor after closeout:

`NA-0379 -- QSL Local Ops Bounded CI Polling Helper Implementation Authorization Plan`

The implementation evidence PR must not implement NA-0379.

## Required Local Checks

Run and record:

- qshell syntax checks before and after mutation;
- qshell checksum/stat before and after mutation;
- qshell rollback checksum equality;
- qshell patch creation;
- local temp-repo harness and marker scan;
- qsl-protocol queue/decisions helper;
- scope guard for changed qsl-protocol files;
- markdown link check;
- leak scan;
- overclaim scan;
- cargo audit;
- rustls-webpki dependency proof;
- qsc send_commit test when feasible;
- formal model checks when feasible.

## CI Expectations

The qsl-protocol PR must include Goals, Impact, No-regression, and
Tests/Vectors metadata. Required checks, including `public-safety`, must pass
before merge. Post-merge `public-safety` must remain required and green before
optional closeout.

## Successor Handoff

After the evidence PR merges and post-merge public-safety is green, a separate
closeout PR may mark NA-0378 DONE and restore NA-0379 as READY. That closeout
must not implement NA-0379 or mutate qshell.
