Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-26

# NA-0543 QSL Local Ops SSD Hygiene Shared Cargo Target Implementation Harness

## Executive Summary

NA-0543 implements the D-1074-authorized tracked artifacts for local qbuild SSD
hygiene and ordinary qsl-protocol shared Cargo target selection. The result
classification is:

`LOCAL_OPS_SSD_HYGIENE_SHARED_TARGET_OPERATOR_ACTION_BUNDLE_READY`

No operator action was executed by Codex. The installed maintenance script,
systemd units, qwork/env/wrapper files, shared target path, backup/archive
state, qsl-backup, qwork/qstart/qresume, qsc source/Cargo paths, workflows,
qsl-server, and qsl-attachments were not mutated by Codex.

## qwork Proof Verification

Startup proof files existed at
`/srv/qbuild/work/NA-0543/.qwork/startup.qsl-protocol.kv` and
`/srv/qbuild/work/NA-0543/.qwork/startup.qsl-protocol.json`.

Verified values:

- `startup_result=OK`
- `lane=NA-0543`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0543/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0543`
- `requested_lane_status=READY`

Proof HEAD and origin/main matched live pre-fetch refs at `8635151e3fdd`.
Proof timestamp `2026-06-26T03:22:27Z` postdated the D455 response timestamp
`2026-06-26T01:26:19Z`. Codex did not run qwork, qstart, or qresume.

## Inheritance Consumed

Consumed D455, D454, D453, NA-0542 evidence/testplan, D-1074, D-1075, and the
NA-0543 READY block. Inherited findings include:

- D-1074 exact tracked path bundle and operator-owned path bundle;
- installed maintenance inventory and scheduled-run classification;
- current maintenance hardening findings;
- qwork/qbuild child-process export boundary;
- selected shared target base and ordinary partition;
- explicit target precedence;
- proof-root-isolated target exceptions;
- pressure thresholds;
- cleanup/retention/logging policy;
- operator/Codex boundary;
- rollback requirements;
- no implementation occurred in NA-0542.

## Latest Scheduled Maintenance Recheck

Read-only recheck found:

- installed script SHA-256 `53e9820f0ece`;
- service SHA-256 `d4f37b7ed0ee`;
- timer SHA-256 `d50064ce5878`;
- timer enabled and active;
- next run listed as `2026-06-26 03:42:32 CDT`;
- `LastTriggerUSec` empty;
- service inactive/dead with no ExecMain timestamps;
- timer journal only recorded timer start;
- service journal had no entries;
- latest housekeeping log remained the manual apply run from 2026-06-25.

Classification:

`MAINTENANCE_NO_SCHEDULED_RUN_YET`

No destructive cleanup outside documented classes, source deletion, archive
corruption, broken current evidence path, cleanup during active build, mount
confusion, or missing proof data was observed.

## Canonical Maintenance Script

Implemented `scripts/local_ops/qbuild-ssd-maintenance.sh` with:

- dry-run default and explicit `--apply`;
- apply root requirement outside fixture mode;
- `/backup/qsl` mount requirement or fixture mount marker;
- nonblocking flock;
- validated live and fixture roots;
- candidate newest-descendant mtime policy;
- shared cache exclusion;
- active process safe skip with exit `3`;
- destination collision fail-closed;
- copy/verify/finalize/move/symlink archive transaction;
- human and JSON summaries;
- warning/hard-stop disk policy;
- fixture-root-only test mode.

Exit semantics:

- `0`: success/no candidates or dry-run report;
- `3`: safe active-build skip;
- `4`: warning disk state;
- `10`: apply success with reclaimed bytes;
- `2`: hard failure.

## Shared Cargo Target Helper

Implemented `scripts/local_ops/qbuild-shared-target-env.sh` with:

- default base `/srv/qbuild/cache/targets`;
- expected qsl-protocol ordinary path
  `/srv/qbuild/cache/targets/qsl-protocol/rustc-1.95.0-x86_64-unknown-linux-gnu/default`;
- strict component validation;
- explicit target preservation;
- directive isolated target support;
- JSON, shell, and qwork-proof output;
- no target creation unless explicit prepare mode is selected;
- fixture preparation restricted to proof-root fixtures.

## Proposed qwork/qbuild Integration

Proof-root proposed patches update `env_qbuild.sh`, `qwork.sh`, and the qwork
wrapper copies only. The mechanism is file-backed:

- qwork computes target selection only when no explicit target exists;
- qwork writes target proof fields to startup `.kv` and `.json`;
- qwork writes a shell-safe env file under the lane `.qwork` directory;
- the wrapper can source that env file when invoked as a sourced shell helper;
- ordinary execution still prints the file path for explicit sourcing/verification.

This survives the qwork child-process boundary because the selected target is
persisted in a file-backed handoff rather than relying on child-process export.

## Proposed Systemd Hardening

Proof-root proposed units add:

- `ConditionPathIsMountPoint=/backup/qsl`;
- exact installed script `ExecStart`;
- `SuccessExitStatus=3 10`;
- bounded timeout;
- background scheduling settings;
- `NoNewPrivileges`;
- `PrivateTmp`;
- filesystem protection with explicit writable paths;
- no network requirement.

## Operator Bundle and Rollback

Created `docs/ops/NA-0543_qbuild_operator_action_bundle.md`. It starts with
`DO NOT RUN UNTIL DIRECTOR REVIEW`, is dry-run-first, lists exact root-owned
steps, requires rollback capture at
`/backup/qsl/qbuild-local-ops-rollback/NA-0543/pre-change`, and includes exact
rollback commands. Proof-root manifests record proposed file hashes, proof
requirements, operator action metadata, and rollback inventory.

## Validation Summary

Fixture/static validation covers:

- maintenance dry-run and apply behavior in proof-root fixtures;
- active-process safe skip;
- newest-descendant mtime;
- archive transaction and collision failure;
- broken symlink reporting;
- JSON/human logging;
- log retention;
- shared-target helper path/proof output;
- explicit target preservation;
- isolated target override;
- unrelated repo rejection;
- proposed qwork/systemd copy syntax and patch application.

Focused qsc runtime tests are skipped because this lane mutates no qsc
source/runtime/dependency/workflow path and current main public-safety and
advisories are green.

## Boundary Proof

No operator-owned path was changed by Codex. No qwork/qstart/qresume was run by
Codex. No qsl-backup was run. No backup mutation occurred. No maintenance live
dry-run/apply occurred. No shared target was created or mutated. No qsc source,
test, fuzz, Cargo, dependency, lockfile, workflow, qsl-server, or
qsl-attachments path was mutated.

## Claim Boundary

This lane does not claim public readiness. This lane does not claim production
readiness. This lane does not claim public-internet readiness. This lane does
not claim external-review completion. This lane does not claim reproducibility
completion. This lane does not claim backup/restore completion. This lane does
not claim vulnerability freedom. This lane does not claim bug freedom. This
lane does not claim perfect builds.

## Selected Successor

Expected successor after implementation merge and green checks:

`NA-0544 -- QSL Local Ops SSD Hygiene / Shared Cargo Target Operator Action Proof Review Harness`
