Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL local operations
Last-Updated: 2026-06-26

# DOC-OPS-005 — qbuild SSD Hygiene and Shared Cargo Target Runbook v0.1.0 DRAFT

## Purpose

This runbook defines the reviewed local-ops model for keeping qbuild SSD usage
bounded while giving ordinary qsl-protocol Rust builds a repo/toolchain-scoped
shared Cargo target. It is operational support for verification throughput. It
does not change QSL protocol, wire, crypto, auth, state-machine, qsc runtime,
qsl-server, or qsl-attachments behavior.

## Scope

In scope:

- the tracked maintenance template `scripts/local_ops/qbuild-ssd-maintenance.sh`;
- the tracked shared-target helper `scripts/local_ops/qbuild-shared-target-env.sh`;
- reviewed operator installation of local qbuild maintenance, qwork/env
  integration, systemd units, rollback capture, and shared-target preparation;
- later read-only proof review of operator-provided outputs.

Out of scope:

- Codex execution of privileged/local installation;
- Codex execution of qwork, qstart, qresume, qsl-backup, maintenance apply, or
  maintenance live dry-run;
- backup/restore completion claims;
- no production-readiness claim, no public-readiness claim, no
  reproducibility-complete claim, no vulnerability-free claim, no bug-free
  claim, and no perfect-build claim.

## Current Architecture

qbuild lanes live under `/srv/qbuild/work/<lane>/<repo>`. Local proof roots live
under `/srv/qbuild/tmp`. Backup/archive storage is mounted at `/backup/qsl`.
The current qwork entry point is `/home/victor/.local/bin/qwork`, which execs
`/srv/qbuild/tools/qwork.sh`; qwork creates `.qwork` proof files but cannot
magically export environment into a parent shell after it exits.

The reviewed propagation model is therefore file-backed: qwork writes target
selection proof fields and a shell-safe target environment file under the lane
`.qwork` directory. A later Codex/build shell must source or verify that file
before relying on the ordinary shared target.

## Disk Thresholds

- Healthy: root filesystem below 80%.
- Warning: root filesystem at or above 80%; report and stop with warning status.
- Pressure: root filesystem at or above 85%; separately authorized bounded
  cleanup only.
- Urgent: root filesystem at or above 90%; separately authorized bounded cleanup
  only.
- Hard stop: root filesystem at or above 95%; stop the directive/build.

Routine maintenance does not delete the shared target. Pressure deletion of the
shared target is not authorized by this runbook.

## Normal Nightly Policy

The normal scheduled policy is:

- dry-run by default for the script;
- service uses explicit `--apply`;
- 7-day age threshold for ordinary per-lane `qsl-protocol/target` directories;
- 7-day age threshold for top-level `NA*` proof-root archive candidates;
- candidate age uses newest descendant mtime, not top-directory mtime alone;
- archived proof-root symlinks are reported and not reprocessed as live roots;
- broken archive symlinks are reported, not silently removed;
- `/srv/qbuild/cache` is excluded from routine cleanup.

## Safe-Skip Semantics

Maintenance must skip safely when cargo, rustc, sccache, qwork, qstart, or
qresume activity is detected. The skip is neither cleanup success nor hard
failure. The canonical safe-skip exit is `3`, and proposed systemd units treat
it as a successful, visible safe skip.

## Maintenance Transaction Model

Apply mode uses a nonblocking lock, validates every destructive root and
candidate path, refuses dangerous roots, and requires `/backup/qsl` to be
mounted. For proof-root archival it copies to a temporary destination on
`/backup/qsl`, verifies the copy by entry count and byte total, atomically
finalizes the archive destination, moves the source to a recoverable
`.moved.<timestamp>` path, creates the original-path symlink, verifies the
symlink, and removes the moved source only after verification succeeds.

Destination collisions fail closed. Interrupted transactions leave either the
original source, the temporary copy, or the `.moved.<timestamp>` recovery path
rather than silently deleting evidence.

## Archive and Symlink Model

Archive destinations live under `/backup/qsl/qbuild-tmp-archive/YYYY/MM`.
Original `/srv/qbuild/tmp/NA*` paths are preserved as symlinks to the archive
destination after verified cutover. Symlinks are not followed as deletion
targets. Broken symlinks are counted and reported for operator review.

## Shared-Target Model

The ordinary shared target is repo/toolchain/build-class scoped:

`/srv/qbuild/cache/targets/qsl-protocol/rustc-1.95.0-x86_64-unknown-linux-gnu/default`

The helper validates the repo identifier, rustc release, rustc host, build
class, and every generated path component. The ordinary build class is
`default`. The helper does not write global Cargo config, create lane target
symlinks, or affect unrelated repositories.

## Isolated-Target Exceptions

Use directive-provided isolated targets for:

- binary provenance or binary hash evidence;
- remote binary staging or restaging;
- reproducibility-sensitive builds;
- sanitizer runs;
- Miri;
- fuzz or toolchain-specialized runs where collision risk exists;
- directives explicitly requiring isolated artifacts;
- any source-to-binary attribution ambiguity.

## Environment Precedence

Precedence is:

1. directive-provided isolated or explicit `CARGO_TARGET_DIR`;
2. preexisting operator environment `CARGO_TARGET_DIR`;
3. qwork-managed ordinary shared target;
4. lane-local fallback only if separately authorized.

qwork must not overwrite an existing explicit target.

## qwork Proof Fields

The target-selection proof fields are:

- `cargo_target_mode`
- `cargo_target_dir`
- `cargo_target_source`
- `cargo_target_build_class`
- `cargo_target_toolchain_key`
- `explicit_target_preserved`
- `shared_target_ready`

## Operator/Codex Boundary

Codex may edit tracked files and generate proof-root proposed artifacts. Codex
must not install, mutate, create, delete, archive, relink, daemon-reload, start,
stop, or restart local qbuild paths. The operator performs any privileged/local
action after Director review and keeps proof outputs for later read-only review.

## Dry-Run Procedure

Operator dry-run is review-only and must run before apply:

```bash
sudo /usr/local/sbin/qbuild-ssd-maintenance --dry-run --target-days 7 --tmp-days 7 --log-retention-days 90
```

The dry-run must report candidates, disk state, broken symlink count, and JSON
summary path. It must not delete targets or archive proof roots.

## Apply Procedure

Apply is operator-only:

```bash
sudo /usr/local/sbin/qbuild-ssd-maintenance --apply --target-days 7 --tmp-days 7 --log-retention-days 90
```

Do not run apply if active build/qwork processes are present, `/backup/qsl` is
not mounted, root usage is at or above 95%, candidate paths are unexpected, or
the rollback bundle has not been captured.

## Verification

Verify:

- installed file hashes, owners, and modes;
- service/timer content;
- timer enabled/active/next-run state;
- service `Result` and `ExecMainStatus`;
- dry-run JSON and human summary;
- qwork target proof fields;
- explicit-target preservation;
- unrelated-repository rejection;
- rollback inventory;
- disk state.

## Monitoring

Monitor the latest human and JSON summaries under
`/backup/qsl/qbuild-tmp-archive/housekeeping-logs`. A safe active-build skip
must stay distinguishable from cleanup success. A warning state should be
treated as an operator follow-up, not an authorization to broaden deletion.

## Failure Recovery

If archive cutover fails, inspect temporary archive destinations and
`.moved.<timestamp>` paths before taking action. Do not delete recovery
artifacts until the source/archive relationship is verified. If qwork target
proof fields are missing or inconsistent, stop ordinary shared-target use and
fall back only through an explicitly authorized isolated target.

## Rollback

Rollback is operator-run only. Restore the pre-change script, service, timer,
env script, qwork script, and wrapper from
`/backup/qsl/qbuild-local-ops-rollback/NA-0543/pre-change`, then run operator
daemon-reload and timer restart. Rollback does not delete the shared target by
default.

## Security Caveats

Shared targets are ordinary-build acceleration, not security proof. They must
not be used to infer binary provenance, release readiness, or vulnerability
absence. Fail closed on path ambiguity, active-build ambiguity, mount ambiguity,
copy verification failure, destination collision, or missing qwork target proof.

## Reproducibility/Provenance Caveats

The shared target intentionally mixes ordinary incremental build artifacts for
one repo/toolchain/build-class partition. Use isolated targets whenever the
artifact itself is evidence.

## No Backup/Restore Completion Claim

This runbook does not claim backup completion, restore completion, off-host
continuity, or disaster-recovery proof.

## No Production-Readiness Claim

This runbook does not claim public readiness. This runbook does not claim
production readiness. This runbook does not claim public-internet readiness.
This runbook does not claim release readiness. This runbook does not claim
external-review completion. This runbook does not claim vulnerability freedom.
This runbook does not claim bug freedom. This runbook does not claim perfect
builds.
