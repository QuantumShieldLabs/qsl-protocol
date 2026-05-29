Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-28

# NA-0378 QSL Local Ops qstart/qresume Fast-Forward Guard Harness

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0378 adds a narrowly bounded local qstart/qresume fast-forward guard in
`/srv/qbuild/tools/qshell.sh` and records qsl-protocol companion evidence. The
guard applies only to qsl-protocol worktrees. It fetches `origin/main`, rejects
unsafe local states, and advances clean stale `main` branches only with
`git merge --ff-only origin/main`.

This lane changes no qsl-protocol runtime, protocol, crypto, dependency,
workflow, qshield runtime, qsl-server, qsl-attachments, website, public docs,
backup script, timer, fstab, remote target, restore, deploy, rollback, key,
credential, or secret-handling surface.

## Live NA-0378 Scope

Startup proof from refreshed `origin/main`:

- qsl-protocol `origin/main`: `c48f658b7a515a46ef2dbdf5c146fa4eb47b31b1`
- READY_COUNT: `1`
- READY: `NA-0378 -- QSL Local Ops qstart/qresume Fast-Forward Guard Implementation Harness`
- NA-0377: `DONE`
- NA-0376: `DONE`
- D-0736: present once
- D-0737: present once
- D-0738: absent at start
- Required `public-safety`: present and success on `c48f658b7a515`

The live NA-0378 entry permits only the bounded local-ops implementation or
blocker-resolution lane selected by NA-0377 and protects against runtime,
service, protocol, crypto, dependency, workflow, secret, target-setup, backup
configuration, and public-claim drift.

## Inherited NA-0377 Authorization

NA-0377 established that qstart and qresume are shell functions sourced from
`/srv/qbuild/tools/qshell.sh`, that stale clean qsl-protocol startup was
reproducible, and that `/srv/qbuild/tools` appears in the same-host local
continuity backup source list. NA-0377 did not mutate qstart/qresume.

NA-0378 inherits these boundaries:

- local-tool mutation path: `/srv/qbuild/tools/qshell.sh`
- qsl-protocol companion paths: this evidence file, the NA-0378 testplan,
  `DECISIONS.md`, `TRACEABILITY.md`, and `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- no qsl-server, qsl-attachments, qshield runtime, qsc/qsp runtime, workflow,
  dependency, website, public-doc, backup-script, timer, fstab, target, restore,
  deploy, rollback, key, credential, or secret mutation

## qshell Source, Authority, and Backup Proof

Pre-mutation qshell proof:

- source path: `/srv/qbuild/tools/qshell.sh`
- original SHA-256:
  `6b74ed7a7903ac1574ead2df80c285f06c8550f447c745407d12c741161d339a`
- original mode/owner: `0664`, `victor:victor`
- original mtime: `2026-04-20 23:58:52.369912457 -0500`
- syntax: `bash -n /srv/qbuild/tools/qshell.sh` passed before mutation
- source authority: sourcing qshell exposed `qstart` and `qresume` functions

Rollback proof:

- backup path:
  `/srv/qbuild/tools/backups/NA0378/qshell.sh.20260528T200427-0500.6b74ed7a7903.bak`
- backup SHA-256:
  `6b74ed7a7903ac1574ead2df80c285f06c8550f447c745407d12c741161d339a`
- checksum equality: backup checksum equals original qshell checksum
- before/after patch path: `/srv/qbuild/tmp/NA0378_qshell.patch`
- patch SHA-256:
  `45085ddf81740c5f8f328e8573c66d1482f269d48e8dbf95ebdc47d093df0ace`
- new qshell SHA-256:
  `7200e968f1b1d70a106aba1043b48739eb44eeb592058e74930002f5fc915f3b`

## Implementation Summary

The qshell patch adds
`qbuild__fast_forward_clean_qsl_protocol_to_origin_main` and calls it from both
`qstart` and `qresume` after the target worktree exists and before handoff.

The helper is qsl-protocol-only. For other known qbuild repos it returns
without changing behavior.

## Fast-Forward Guard Semantics

The helper:

- validates the target worktree before acting;
- rejects detached HEAD and non-`main` branch states;
- rejects tracked worktree modifications;
- rejects staged/index modifications;
- rejects untracked non-ignored files;
- fetches `origin main` with prune metadata update;
- compares `QSL_EXPECTED_MAIN_SHA` to `origin/main` when the variable is set;
- no-ops when local HEAD already equals `origin/main`;
- fast-forwards only when local HEAD is an ancestor of `origin/main`;
- rejects ahead or diverged histories;
- emits local HEAD, `origin/main`, clean-status, and guard-result evidence;
- returns nonzero on every fail-closed reject.

The helper does not use reset-hard, force push, force-with-lease, branch
deletion, checkout force, clean-force, rebase, amend, or user-worktree removal.

## qstart/qresume Integration Proof

Static inspection proved both functions call the guard:

- qstart call site:
  `qbuild__fast_forward_clean_qsl_protocol_to_origin_main "$(qbuild__worktree_path "$lane" "$repo")" "$repo"`
- qresume call site:
  `qbuild__fast_forward_clean_qsl_protocol_to_origin_main "$path" "$repo"`

A controlled local `qresume NA-0378 qsl-protocol` invocation on the active clean
worktree printed:

- `result=already-current`
- local HEAD `c48f658b7a515a46ef2dbdf5c146fa4eb47b31b1`
- origin/main `c48f658b7a515a46ef2dbdf5c146fa4eb47b31b1`
- clean state `tracked-clean,untracked-clean`

## Harness Scenarios and Markers

Corrected harness log:

`/srv/qbuild/tmp/NA0378_qstart_qresume_guard_20260528T200719-0500/harness.log`

The corrected harness used only local temporary bare remotes and local clones
under `/srv/qbuild/tmp`. It did not use real qsl-protocol history for
destructive scenarios and did not connect to GitHub for its test remotes.

Markers present in the corrected log:

- `NA0378_QSTART_QRESUME_SOURCE_AUTHORITY_OK`
- `NA0378_QSTART_QRESUME_FAST_FORWARD_GUARD_OK`
- `NA0378_CLEAN_STALE_WORKTREE_FAST_FORWARDED_OK`
- `NA0378_ALREADY_CURRENT_NOOP_OK`
- `NA0378_DIRTY_TRACKED_WORKTREE_REJECT_OK`
- `NA0378_UNTRACKED_WORKTREE_REJECT_OK`
- `NA0378_EXPECTED_MAIN_SHA_MISMATCH_REJECT_OK`
- `NA0378_DIVERGED_BRANCH_REJECT_OK`
- `NA0378_NO_DIRTY_OVERWRITE_OK`
- `NA0378_NO_FORCE_OK`
- `NA0378_NO_RESET_HARD_OK`
- `NA0378_BACKUP_IMPACT_OK`
- `NA0378_NO_RUNTIME_CHANGE_OK`
- `NA0378_NO_SECRET_MATERIAL_OK`
- `NA0378_METADATA_RUNTIME_QSTART_QRESUME_GUARD_OK`

## Negative and Fail-Closed Cases

The harness proved:

- dirty tracked files reject and keep HEAD unchanged;
- dirty tracked file contents remain preserved;
- untracked files reject and remain preserved;
- wrong `QSL_EXPECTED_MAIN_SHA` rejects without branch mutation;
- diverged local history rejects without reset or force behavior;
- clean stale `main` fast-forwards to the local test remote's `origin/main`;
- already-current `main` no-ops.

## No-Forbidden-Command Proof

The qshell patch and resulting qshell file were scanned for these forbidden
commands and patterns:

- `git reset --hard`
- `push --force`
- `force-with-lease`
- `branch -D`
- `clean -fd`
- `checkout -f`
- `git rebase`
- `commit --amend`
- `rm -rf`

Result: zero matches.

## Backup-Plan Impact

No backup-plan update is required for NA-0378 because:

- `/srv/qbuild/tools` is already in the same-host local continuity daily source
  list;
- a rollback copy was stored under `/srv/qbuild/tools/backups/NA0378/`;
- a before/after patch was stored under `/srv/qbuild/tmp/NA0378_qshell.patch`;
- qsl-protocol companion evidence is under `/srv/qbuild/work`;
- the current backup status remains same-host continuity only, not complete
  disaster recovery.

Future off-host backup coverage for local tooling, local history indexes, and
response/journal helper outputs remains future-gated.

## Runtime, Service, Dependency, and Workflow Boundary

qsl-protocol changed paths are governance/testplan/journal files only. The
runtime qshell change is local to `/srv/qbuild/tools/qshell.sh` and is not a
qsl-protocol runtime, protocol, crypto, dependency, or workflow change.

No `.github/**`, `Cargo.toml`, `Cargo.lock`, qsc/qsp/qsl runtime, qshield
runtime, qsl-server, qsl-attachments, qsc-desktop, website, public docs,
backup-script, timer, fstab, target, restore, deploy, rollback, key,
credential, or secret paths were changed.

## qsl-server and qsl-attachments Boundary

Read-only state only:

- qsl-server PR #56 is merged at
  `d40e6003fdf0b0c59033b8f4dac2ea37fbded5b1`
- qsl-attachments PR #37 is merged at
  `96b9352bd63e1459389d74b1abef4c5e99a857d3`

NA-0378 does not mutate either repository and does not convert their bounded
service-local evidence into production, public-internet, privacy, external
review, or disaster-recovery claims.

## Public-Claim Boundary

This lane claims only a bounded local qstart/qresume guard implementation and
harness proof. It does not claim production readiness, public-internet
readiness, external-review completion, metadata-free behavior, anonymity,
untraceability, disaster-recovery completion, off-host backup completion, real
restore completion, real key custody, real key recovery, configured target, or
verified host identity.

## Selected Successor

Selected successor after NA-0378 closeout:

`NA-0379 -- QSL Local Ops Bounded CI Polling Helper Implementation Authorization Plan`

Rationale: NA-0376 and NA-0377 ranked qstart/qresume first and bounded CI
polling second. After the qstart/qresume guard is harnessed, the next smallest
workflow-support lane is an authorization plan for bounded polling helpers,
without implementing polling in this PR.

## Rejected Alternatives

- direct reset-hard: rejected because it can discard local work;
- force-update: rejected because it can hide history changes;
- ignoring dirty worktrees: rejected because it can overwrite operator state;
- leaving stale qstart/qresume behavior unchanged: rejected because stale clean
  startup was reproduced;
- implementing response writer, directive manifest, validation profiles,
  source/authority helper, claim scanner, history index, or polling helper now:
  rejected as out of scope for NA-0378.

## Recovered Failures

Recovered startup/path failure:

- failing command: `python3 scripts/ci/qsl_evidence_helper.py queue`
- classification: recoverable local stale-worktree/path issue; the file existed
  on required `origin/main` but not in the stale local worktree;
- corrective action: fast-forwarded the clean local qsl-protocol worktree with
  `git merge --ff-only origin/main`;
- final result: helper queue/decision proof passed from
  `c48f658b7a515a46ef2dbdf5c146fa4eb47b31b1`.

Recovered harness failure:

- failing command: first temp-repo harness under
  `/srv/qbuild/tmp/NA0378_qstart_qresume_guard_20260528T200612-0500/`;
- classification: recoverable harness command-shape issue; local shell variable
  assignment and pipeline behavior masked fixture setup failures;
- corrective action: reran with explicit local assignments and
  `set -euo pipefail`;
- final result: corrected harness log at
  `/srv/qbuild/tmp/NA0378_qstart_qresume_guard_20260528T200719-0500/harness.log`
  contains all required markers.

## Next Recommendation

Merge this qsl-protocol evidence PR only after required checks pass. Then, if
post-merge `public-safety` remains green and READY is still NA-0378, close
NA-0378 and restore the exact NA-0379 successor without implementing NA-0379.
