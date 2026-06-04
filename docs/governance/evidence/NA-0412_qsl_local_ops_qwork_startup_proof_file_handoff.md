Goals: G4

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-04

# NA-0412 qwork Startup Proof File Handoff Evidence

## Summary

NA-0412 implements a file-based qwork startup handoff. Successful qwork startup
now writes stable proof files outside the tracked qsl-protocol checkout:

- `/srv/qbuild/work/NA-0412/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0412/.qwork/startup.qsl-protocol.json`

The existing log JSON proof is preserved at:

- `/srv/qbuild/logs/NA-0412/startup.qsl-protocol.json`

D247 stopped because the directive required the operator to paste qwork output
into the directive body. The user requested a file-based handoff instead. The
new rule for future non-qwork directives is:

- the operator may run qwork once;
- Codex reads `/srv/qbuild/work/<NA>/.qwork/startup.qsl-protocol.kv`;
- Codex performs direct repo and queue checks from the proof path;
- Codex does not rerun qwork unless the directive is specifically a qwork
  test/fix lane; and
- Codex stops if the proof file is stale or disagrees with live checks.

## Proof Directory Design

The lane workspace proof directory is:

- `/srv/qbuild/work/<NA>/.qwork/`

This directory is outside the qsl-protocol worktree. For NA-0412, qwork wrote
only two files there after repeated startup runs:

- `startup.qsl-protocol.kv`
- `startup.qsl-protocol.json`

The directory mode was `2775`, and both proof files were mode `0644`. None of
the proof paths are world-writable.

## Required Proof Fields

The `.kv` and workspace `.json` proofs include:

- `startup_result=OK`
- `lane=NA-0412`
- `primary_repo=qsl-protocol`
- `repo_result=OK`
- `repo=qsl-protocol`
- `created_or_existing=existing`
- `path=/srv/qbuild/work/NA-0412/qsl-protocol`
- `branch=main`
- `upstream=origin/main`
- `head`, `origin_main`, and `main` all at `23243ea53fba`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0412`
- `requested_lane_status=READY`
- `json_proof=/srv/qbuild/logs/NA-0412/startup.qsl-protocol.json`
- `lane_workspace_proof_dir=/srv/qbuild/work/NA-0412/.qwork`
- `lane_workspace_kv_proof=/srv/qbuild/work/NA-0412/.qwork/startup.qsl-protocol.kv`
- `lane_workspace_json_proof=/srv/qbuild/work/NA-0412/.qwork/startup.qsl-protocol.json`
- `cd=/srv/qbuild/work/NA-0412/qsl-protocol`
- `qwork_version_or_sha` with checksum prefix `438b81623d3a`
- `proof_written_at_utc`

The workspace JSON and the existing log JSON parsed successfully and contained
the same required fields. The KV proof and workspace JSON matched for all
required key values.

## Idempotence

Repeated qwork startup replaced the stable proof files in place. The proof
directory contained exactly two files after repeated runs:

- `startup.qsl-protocol.kv`
- `startup.qsl-protocol.json`

No unbounded stale proof files accumulated in `.qwork`.

## Future-Directive Simulation Without qwork Rerun

A simulation from `/tmp` read
`/srv/qbuild/work/NA-0412/.qwork/startup.qsl-protocol.kv`, verified
`startup_result=OK`, changed directory to the proof `cd` path, and ran direct
repo helpers without invoking qwork.

The direct queue/decision checks reported:

- READY_COUNT `1`
- READY `NA-0412 -- QSL Local Ops qwork Startup Proof File Handoff Implementation Harness`
- NA-0413 BACKLOG backup status / plan authorization lane
- D-0810 once
- D-0811 absent before this Packet C evidence
- D-0812 absent
- duplicate decision count zero

The simulation ended with `SIM_WITHOUT_QWORK=PASS`.

## qsl-protocol Clean-State Proof

After qwork proof writes, repeated qwork runs, the future-directive simulation,
and qstart/qresume smokes, qsl-protocol remained clean:

- `git status --porcelain=v1 --branch` showed `## main...origin/main`
- no tracked diff
- no untracked files

This proves the qwork proof files were not written inside the qsl-protocol
worktree.

## qshell set-e Preservation

The qshell interactive fail-closed smoke used an invalid lane under `set -e`.
It printed:

- `startup_result=FAIL`
- `reason=unsafe-lane`
- `qshell_qwork_wrapper=fail-closed-shell-preserved`
- `qshell_qwork_status=2`
- `shell-survived`

This preserves the intended interactive shell survival behavior while qwork
automation remains nonzero for direct failures.

## qstart/qresume Compatibility

Both compatibility helpers passed from `/tmp`:

- `qstart NA-0412 qsl-protocol`
- `qresume NA-0412 qsl-protocol`

Each reported the NA-0378 fast-forward guard result as already current and
landed in `/srv/qbuild/work/NA-0412/qsl-protocol`.

## Fail-Closed Wrong-Lane Proof

`qwork NA-0411 qsl-protocol` returned status `2` with:

- `startup_result=FAIL`
- `reason=queue-lane-mismatch`

This confirms wrong-lane qwork remains fail-closed.

## qsl-backup Boundary

No backup or restore operation was run. `/usr/local/sbin/qsl-backup` remained
unchanged with checksum prefix `e9ecff3d22ed`, and the Codex ops source
inclusion count remained exactly `1`.

No backup source-list, backup status, backup plan, systemd, timer, fstab, or
backup target setting was mutated.

## Successor

The selected successor after NA-0412 closeout remains:

- `NA-0413 -- QSL Codex Ops Backup Status / Plan Update Authorization Plan`

NA-0413 is preserved but not implemented by NA-0412.
