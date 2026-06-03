Goals: G4

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-03

# NA-0410 qwork CWD-Independent Queue Verification Bugfix Evidence

## Summary

NA-0410 fixes a local qwork implementation bug where queue verification could
depend on the caller's current directory. The fix is local to
`/srv/qbuild/tools/qwork.sh`; qsl-protocol records evidence only.

## Diagnostic Report Summary

Before the fix, running `qwork NA-0410 qsl-protocol` from `/home/victor`
failed with `reason=queue-lane-mismatch` even though the target checkout was
READY NA-0410. The failure reproduced the operator diagnostic.

## Root Cause

qwork invoked the qsl-protocol queue helper as `python3 "$helper" queue`.
Because the helper read `NEXT_ACTIONS.md` relative to the caller's current
directory, a non-checkout cwd could make helper read failure look like a real
lane mismatch.

## Fix

qwork now invokes the helper against the target checkout queue file:

```bash
python3 "$helper" queue --file "$path/NEXT_ACTIONS.md"
```

The implementation also checks that the target helper exists and the target
queue file is readable before parsing queue state.

## Failure Classification

- Helper execution failure: `queue-helper-failed`.
- Queue read or parse failure: `queue-read-failed`.
- Successfully read queue with READY_COUNT greater than 1: `multiple-ready`.
- Successfully read queue whose READY lane differs from the requested lane:
  `queue-lane-mismatch`.

## Proof Root

Packet B proof root:

```text
/srv/qbuild/tmp/NA0410_qwork_cwd_queue_bugfix_20260603T151931-0500
```

Rollback copies and test outputs are stored under that root. qwork checksum
prefix changed from `1f648cafba35` to `e8f6dc0a5ed4`; qshell checksum prefix
remained `6ad0dfff5fa4`.

## Validation

- READY-lane qwork succeeded from `/home/victor`.
- READY-lane qwork succeeded from `/tmp`.
- READY-lane qwork succeeded from an unrelated temp cwd.
- READY-lane qwork succeeded from the qsl-protocol checkout.
- Bare qwork success also passed from `/home/victor`, `/tmp`, and the unrelated
  temp cwd.
- Wrong-lane `qwork NA-0411 qsl-protocol` failed with
  `queue-lane-mismatch`.
- Controlled helper execution failure fixture returned `queue-helper-failed`.
- Controlled queue read/parse failure fixture returned `queue-read-failed`.
- Controlled multiple READY fixture returned `multiple-ready`.
- Interactive `set -e` qshell failure printed `shell-survived`.
- Direct automation failure remained nonzero.
- qstart/qresume compatibility smoke passed.

## Boundaries

- qshell was not changed.
- qsl-backup checksum prefix remained `e9ecff3d22ed`.
- Codex ops source inclusion count remained `1`.
- No backup or restore operation ran.
- No qsl-protocol runtime, protocol, crypto, dependency, workflow, public docs,
  website, README, START_HERE, qsl-server, qsl-attachments, backup status,
  backup plan, backup source-list, durable Director State Index, or public
  claim path changed.

## Successor

The selected successor after NA-0410 closeout remains:

```text
NA-0411 -- QSL Codex Ops Backup Coverage Manifest Verification / Status Update Plan
```

This evidence does not implement NA-0411.
