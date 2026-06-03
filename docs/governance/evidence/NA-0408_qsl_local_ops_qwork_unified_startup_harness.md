Goals: G4

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-03

# NA-0408 qwork Unified Startup Harness Evidence

## Summary

NA-0408 implemented a local qbuild startup hardening harness after the Packet A
governance reroute merged and post-merge `public-safety` completed green. The
implementation adds one Director-facing command:

```bash
qwork <lane> <repo1> [repo2 ...]
```

The accepted recommendation was to replace Director-facing qstart/qresume
conditional startup guidance with one deterministic command. qstart and qresume
remain available as compatibility helpers, but qwork is the hardened path for
future directive startup.

## Reason

Recent directives repeatedly exposed startup brittleness:

- qstart could be unavailable until qshell was sourced.
- qresume-first startup guidance required operator judgment.
- local `main` could track stale `mirror/main`, producing misleading status.
- startup recovery repeatedly needed manual path, HEAD, origin/main, queue, and
  worktree checks.

qwork removes that branch in operator procedure by validating the lane and repo,
acquiring a lane lock, creating or reusing the checkout, fetching authoritative
`origin/main`, failing closed on unsafe states, fast-forwarding clean stale
`main`, setting upstream to `origin/main`, verifying queue state for
qsl-protocol, printing stable key-value proof, and writing JSON proof.

## Implemented Local Tool Paths

- qwork path: `/srv/qbuild/tools/qwork.sh`
- qshell wrapper path: `/srv/qbuild/tools/qshell.sh`
- qshell rollback copy: `/srv/qbuild/tmp/NA0408_qwork_startup_harness_20260603T112854-0500/qshell.sh.rollback`
- pre-mutation tool proof: `/srv/qbuild/tmp/NA0408_qwork_startup_harness_20260603T112854-0500/pre_mutation_tool_state.txt`

Post-implementation tool state:

- `/srv/qbuild/tools/qwork.sh`: mode `775`, owner `victor`, group `victor`, sha256 `1f648cafba35a6e1f7b62caf448fc3042eae94cd0bc4197e33f5675cdbf67a0c`
- `/srv/qbuild/tools/qshell.sh`: mode `664`, owner `victor`, group `victor`, sha256 `40a4180dc37fe4c5fa27e09c47a3232d21a32c666b4bf1ae719af12a7a272607`

## qwork Fail-Closed Surface

qwork rejects with machine-readable failure proof:

- `unsafe-lane`
- `unknown-repo`
- `lane-lock-held`
- `invalid-checkout`
- `missing-origin`
- `wrong-origin-url`
- `missing-origin-main`
- `dirty-worktree`
- `dirty-index`
- `untracked-files`
- `non-main-branch`
- `local-ahead`
- `local-diverged`
- `queue-lane-mismatch`
- `multiple-ready`
- `helper-missing`

The implementation contains no `git reset`, `git stash`, `git clean`, forced
checkout, force push, normal push, or non-fast-forward merge path.

## Test Matrix

Harness path:

`/srv/qbuild/tmp/NA0408_qwork_startup_harness_20260603T112854-0500/qwork_test_harness.sh`

Passing result file:

`/srv/qbuild/tmp/NA0408_qwork_startup_harness_20260603T112854-0500/qwork_test_harness.20260603T163625Z.results`

Covered cases:

| Case | Expected result |
|---|---|
| existing clean current checkout | `startup_result=OK` |
| existing clean stale main | fast-forwards to `origin/main` |
| dirty tracked file | `startup_result=FAIL`, `reason=dirty-worktree` |
| dirty index | `startup_result=FAIL`, `reason=dirty-index` |
| untracked file | `startup_result=FAIL`, `reason=untracked-files` |
| local ahead | `startup_result=FAIL`, `reason=local-ahead` |
| non-main branch | `startup_result=FAIL`, `reason=non-main-branch` |
| qsl-protocol current READY lane | `ready_count=1`, `queue_top_ready=NA-0408`, `requested_lane_status=READY` |
| qsl-protocol wrong requested lane | `startup_result=FAIL`, `reason=queue-lane-mismatch` |
| JSON proof written | `startup.<repo>.json` exists under the lane log dir |

Harness final marker:

```text
QWORK_TEST_HARNESS_OK
```

## Live Smoke Proof

Live smoke output path:

`/srv/qbuild/tmp/NA0408_qwork_startup_harness_20260603T112854-0500/live_qwork_smoke.out`

Key proof:

```text
startup_result=OK
lane=NA-0408
primary_repo=qsl-protocol
repo_result=OK
repo=qsl-protocol
created_or_existing=existing
path=/srv/qbuild/work/NA-0408/qsl-protocol
branch=main
upstream=origin/main
head=d49d7909980e72d7a734c81c2d0f81faac4be624
origin_main=d49d7909980e72d7a734c81c2d0f81faac4be624
main=d49d7909980e72d7a734c81c2d0f81faac4be624
head_equals_origin_main=yes
worktree_clean=yes
index_clean=yes
untracked_clean=yes
ready_count=1
queue_top_ready=NA-0408
requested_lane_status=READY
json_proof=/srv/qbuild/logs/NA-0408/startup.qsl-protocol.json
cd=/srv/qbuild/work/NA-0408/qsl-protocol
```

JSON proof path:

`/srv/qbuild/logs/NA-0408/startup.qsl-protocol.json`

qstart/qresume compatibility smoke path:

`/srv/qbuild/tmp/NA0408_qwork_startup_harness_20260603T112854-0500/qstart_qresume_compat_smoke.out`

Both compatibility helpers reported already-current clean qsl-protocol state at
`d49d7909980e72d7a734c81c2d0f81faac4be624`.

## Recovered Failures

- Failing command: first run of `qwork_test_harness.sh`. Classification:
  recoverable in-scope local validation failure with understood cause. Root
  cause: qwork used `git remote get-url origin`, which reports the effective
  URL after repo-local `insteadOf` mapping in the temp harness. Corrective
  action: validate raw `remote.origin.url` via git config. Final result:
  harness rerun passed with `QWORK_TEST_HARNESS_OK`.
- Failing command: live smoke capture while the worktree was on the Packet C
  evidence branch. Classification: recoverable sequencing mistake; qwork
  correctly failed closed with `reason=non-main-branch`. Corrective action:
  switched to clean `main`, reran qwork/qstart/qresume smoke, then returned to
  the evidence branch. Final result: qwork live smoke and compatibility smoke
  passed.

## Boundary Proof

- No qsl-protocol runtime, protocol, crypto, dependency, or workflow file was
  changed by the implementation.
- No backup or restore operation was run.
- `/usr/local/sbin/qsl-backup`, backup source lists, backup status, and backup
  plans were not mutated.
- No durable Director State Index output was created.
- No qsl-server or qsl-attachments checkout was mutated.
- No public readiness, public technical paper, backup-complete, restore-proof,
  off-host-backup, privacy, or assurance claim was introduced.
- The selected successor remains `NA-0409 -- QSL Codex Ops Backup Coverage Manifest Verification / Status Update Plan` after NA-0408 closeout.
