Goals: G4

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-03

# NA-0409 qwork Director-Facing Startup Hardening Evidence

## Summary

NA-0409 implements local qwork availability and qshell tmux/set-e safety
hardening after Packet A merged as qsl-protocol PR #1083 and post-merge
`public-safety` completed success on merge `9b00233895de`.

The hardening makes bare `qwork` available in fresh shells and keeps an
interactive qshell `qwork` failure from dropping a tmux-like shell when
`set -e` is active. The underlying `/srv/qbuild/tools/qwork.sh` remains
fail-closed and nonzero for automation.

## Diagnostic Intake

The operator reported that this fresh tmux command failed:

```bash
qwork NA-0409 qsl-protocol
```

Pre-mutation diagnostics confirmed:

- `/srv/qbuild/tools/qwork.sh` existed and worked after sourcing qshell.
- Bare `qwork` was missing in a fresh shell.
- The qshell wrapper could return nonzero under interactive `set -e`, so a
  fail-closed qwork result could exit the shell before `shell-survived` printed.

Diagnostic proof:

- fresh shell missing proof: `bash -lc 'command -v qwork || echo qwork_missing'`
  printed `qwork_missing`.
- sourced qshell proof: `bash -lc 'source /srv/qbuild/tools/qshell.sh; type qwork; qwork NA-0409 qsl-protocol'`
  passed and reported `requested_lane_status=READY`.
- tmux/set-e reproduction: `bash -ic 'set -e; source /srv/qbuild/tools/qshell.sh; qwork bad/lane qsl-protocol; echo shell-survived'`
  returned nonzero before the implementation and did not print
  `shell-survived`.

## Implemented Local Paths

Proof root:

`/srv/qbuild/tmp/NA0409_qwork_director_facing_hardening_20260603T130919-0500`

Implemented files:

- `/srv/qbuild/tools/qwork`: new executable wrapper that execs
  `/srv/qbuild/tools/qwork.sh`.
- `/home/victor/.local/bin/qwork`: symlink to `/srv/qbuild/tools/qwork`.
- `/home/victor/.bashrc`: existing qbuild shell integration block now
  idempotently prepends `$HOME/.local/bin` for interactive shells before
  sourcing `/srv/qbuild/tools/qshell.sh`.
- `/srv/qbuild/tools/qshell.sh`: qwork function now preserves the caller's
  original errexit state, forces errexit off around env loading and qwork
  execution, prints qwork output and wrapper proof on fail-closed paths, cd's
  to the `cd=` path on success, and returns zero only to preserve an
  interactive set-e shell after a fail-closed qwork result.

Unchanged qwork core:

- `/srv/qbuild/tools/qwork.sh` remained at checksum prefix `1f648cafba35`.

Post-install state:

- `/srv/qbuild/tools/qwork`: mode `755`, checksum prefix `9b5d1eb57fac`.
- `/srv/qbuild/tools/qshell.sh`: checksum prefix `6ad0dfff5fa4`.
- `/home/victor/.local/bin/qwork`: symlink to `/srv/qbuild/tools/qwork`.
- `/home/victor/.bashrc`: checksum prefix `f364cf6a3dcf`.

Rollback copies exist under:

`/srv/qbuild/tmp/NA0409_qwork_director_facing_hardening_20260603T130919-0500/rollback`

Rollback files:

- `.bashrc.rollback`
- `qshell.sh.rollback`
- `qwork.sh.rollback`

## User-Local Install Decision

The directive forbade `/usr/local/bin/qwork` mutation unless explicitly
switched to user action. No sudo or `/usr/local/bin` change was used.

The selected install path is user-local:

```text
/home/victor/.local/bin/qwork -> /srv/qbuild/tools/qwork
```

The fresh-shell proof shows `command -v qwork` resolves to
`/home/victor/.local/bin/qwork`.

## qshell tmux/set-e Safety

Passing proof:

`/srv/qbuild/tmp/NA0409_qwork_director_facing_hardening_20260603T130919-0500/tests/interactive_set_e_fail_closed_survives.out`

Key output:

```text
startup_result=FAIL
reason=unsafe-lane
lane=bad/lane
qshell_qwork_wrapper=fail-closed-shell-preserved
qshell_qwork_status=2
shell-survived
```

The wrapper now prints the qwork fail-closed proof and keeps the interactive
set-e shell alive. This wrapper behavior is for interactive safety only; it
does not weaken automation behavior because automation should call
`/srv/qbuild/tools/qwork.sh` directly.

## qwork Automation Fail-Closed Proof

Passing proof:

`/srv/qbuild/tmp/NA0409_qwork_director_facing_hardening_20260603T130919-0500/tests/automation_fail_closed_nonzero.out`

Key output:

```text
startup_result=FAIL
reason=unsafe-lane
lane=bad/lane
status=2
```

`/srv/qbuild/tools/qwork.sh bad/lane qsl-protocol` still returns nonzero.

## Fresh Shell and Success Proofs

Bare fresh-shell proof:

`/srv/qbuild/tmp/NA0409_qwork_director_facing_hardening_20260603T130919-0500/tests/bare_fresh_shell_qwork.out`

Key output:

```text
/home/victor/.local/bin/qwork
startup_result=OK
lane=NA-0409
repo=qsl-protocol
branch=main
upstream=origin/main
head_equals_origin_main=yes
worktree_clean=yes
index_clean=yes
untracked_clean=yes
ready_count=1
queue_top_ready=NA-0409
requested_lane_status=READY
cd=/srv/qbuild/work/NA-0409/qsl-protocol
```

Interactive qshell success proof:

`/srv/qbuild/tmp/NA0409_qwork_director_facing_hardening_20260603T130919-0500/tests/interactive_qshell_success.out`

The proof ends in:

```text
/srv/qbuild/work/NA-0409/qsl-protocol
```

qwork current-lane smoke proof:

`/srv/qbuild/tmp/NA0409_qwork_director_facing_hardening_20260603T130919-0500/tests/qwork_current_lane_smoke.out`

Live JSON proof:

`/srv/qbuild/logs/NA-0409/startup.qsl-protocol.json`

## .bashrc Idempotence Proof

The existing qbuild block remains single-copy and now handles user-local PATH:

```bash
# >>> qbuild shell integration >>>
case ":$PATH:" in
    *":$HOME/.local/bin:"*) ;;
    *) export PATH="$HOME/.local/bin:$PATH" ;;
esac
[ -f /srv/qbuild/tools/qshell.sh ] && . /srv/qbuild/tools/qshell.sh
# <<< qbuild shell integration <<<
```

Proof files:

- `tests/bashrc_idempotence_and_rollback.out`
- `tests/bashrc_repeated_source_idempotence.out`

Repeated sourcing kept `local_bin_path_count 1` and resolved qwork to
`/home/victor/.local/bin/qwork`.

## qstart/qresume Compatibility

Compatibility proof:

`/srv/qbuild/tmp/NA0409_qwork_director_facing_hardening_20260603T130919-0500/tests/qstart_qresume_compatibility.out`

Both compatibility helpers reported already-current qsl-protocol state at
`9b00233895de` and cd'd to:

```text
/srv/qbuild/work/NA-0409/qsl-protocol
```

## Forbidden Command Scan

Proof:

`/srv/qbuild/tmp/NA0409_qwork_director_facing_hardening_20260603T130919-0500/tests/forbidden_command_scan.out`

Result:

```text
FORBIDDEN_SCAN_MATCHES=0
```

The scan covered `/srv/qbuild/tools/qwork`, `/srv/qbuild/tools/qwork.sh`, and
`/srv/qbuild/tools/qshell.sh`.

## Recovered Failures

- Failing command:
  `grep -c '/home/victor/work/qsl/codex' /usr/local/sbin/qsl-backup`.
  Classification: recoverable command-shape proof mistake because the required
  source inclusion check is for the exact Codex ops source path, not the broader
  Codex root. Corrective action: reran an exact `daily_sources` count for
  `/home/victor/work/qsl/codex/ops`. Final result: count `1`.
- Failing command:
  `bash -ic 'set -e; source /srv/qbuild/tools/qshell.sh; qwork bad/lane qsl-protocol; echo shell-survived'`.
  Classification: recoverable in-scope local validation failure with understood
  cause. Root cause: `qbuild__load_env` sources `env_qbuild.sh`, which
  re-enabled `errexit` before the qwork helper command substitution. Corrective
  action: changed the qshell qwork function to force `errexit` off after env
  loading and restore only the caller's original errexit state. Final result:
  rerun printed fail-closed proof and `shell-survived`.
- Failing command:
  first parallel rerun of
  `bash -ic 'source /srv/qbuild/tools/qshell.sh; qwork NA-0409 qsl-protocol; pwd'`.
  Classification: recoverable test sequencing mistake because another qwork
  smoke held the lane lock at the same time. Corrective action: reran the
  interactive success smoke sequentially. Final result: qwork succeeded and
  `pwd` reported the qsl-protocol path.

## Boundary Proof

- qsl-protocol worktree remained clean after local Packet B implementation.
- No qsl-protocol runtime, protocol, crypto, dependency, or workflow file was
  changed by the local implementation.
- No backup or restore operation was run.
- `/usr/local/sbin/qsl-backup` remained unchanged at checksum prefix
  `e9ecff3d22ed`; exact Codex ops source inclusion count remained `1`.
- Backup source lists, backup status files, and backup plan files were not
  mutated.
- No durable Director State Index output was created.
- No qsl-server or qsl-attachments checkout was mutated.
- No public-readiness, public technical paper, backup-complete, restore-proof,
  off-host-backup, privacy, or assurance claim was introduced.

## Selected Successor

NA-0410 remains the selected successor after NA-0409 closeout:

`NA-0410 -- QSL Codex Ops Backup Coverage Manifest Verification / Status Update Plan`

NA-0410 is preserved only. It is not implemented by this evidence PR.
