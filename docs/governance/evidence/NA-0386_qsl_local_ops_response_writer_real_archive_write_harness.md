Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-30

# NA-0386 QSL Local Ops Response Writer Real-Archive Write Harness

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0386 extends `scripts/ci/qsl_codex_response_writer.py` with a deliberately
gated real response archive smoke mode while preserving NA-0384 temp-output
behavior. The helper now requires both `--allow-real-archive-output` and
metadata `allow_real_archive_output: true`, accepts only
`output_mode: real_archive_smoke` for the real archive path, rejects every other
archive destination, scans for high-confidence secret patterns before opening
the output file, and writes with exclusive-create semantics.

Exactly one no-secret synthetic smoke file was written under the real response
archive:

- Path: `/home/victor/work/qsl/codex/responses/NA0386_20260530T080430-0500_D205.md`
- SHA-256: `2d06eb23330873576f813d875dadb08b5b26c019138f9cef77af27b8d20b5e40`

The smoke file is explicitly marked:

`NA-0386 SYNTHETIC REAL-ARCHIVE SMOKE FILE — NOT A FINAL CODEX RESPONSE`

This proves archive-write mechanics only. It does not prove disaster recovery,
off-host backup completion, production readiness, public-internet readiness,
external review completion, anonymity, metadata-free behavior, or untraceability.

## Live NA-0386 scope

Live `NEXT_ACTIONS.md` records:

- READY_COUNT `1`.
- READY `NA-0386 -- QSL Local Ops Response Writer Real-Archive Write Implementation Harness`.
- NA-0385 DONE.
- D-0752 exists once.
- D-0753 exists once.
- D-0754 absent at startup.
- public-safety remains required and green.

Live objective:

- implement a bounded response writer real-archive write harness with explicit
  archive authorization, backup-impact proof, no-secret scan, no-overwrite
  behavior, path/checksum proof, and no runtime/workflow/dependency drift.

## Inherited NA-0385 authorization

NA-0385 selected NA-0386 because read-only evidence showed the real response
archive is covered by same-host local continuity backup and because a one-file
no-secret smoke could be constrained by:

- explicit CLI gate;
- explicit metadata gate;
- exact output directory check;
- no-secret pre-write scan;
- no-overwrite and collision-safe filename behavior;
- path and checksum proof;
- no index creation;
- no delete or cleanup;
- same-host continuity caveat.

NA-0385 did not implement real archive writing and did not mutate the helper.

## Implemented helper changes

The helper now supports two output modes:

- `temp-output`: the NA-0384 behavior, still restricted to `/srv/qbuild/tmp`.
- `real_archive_smoke`: the NA-0386 real archive smoke mode, restricted exactly
  to `/home/victor/work/qsl/codex/responses`.

Real archive mode fails closed unless:

- `--allow-real-archive-output` is present;
- metadata `allow_real_archive_output` is `true`;
- `output_mode` is `real_archive_smoke`;
- `target_na` is `NA-0386`;
- `directive_suffix` is `D205`;
- `no_secret_required` is `true`;
- the output directory resolves exactly to the real response archive;
- the body and metadata pass the high-confidence no-secret scan.

The helper remains Python standard library only and does not call GitHub,
network, shell, git, or subprocess APIs.

## Fixture matrix and markers

Proof log:

`/srv/qbuild/tmp/NA0386_final_real_archive_20260530T101012-0500/fixture_matrix.log`

The NA-0386 fixture matrix passed 17/17 cases and emitted the required markers:

- `NA0386_REAL_ARCHIVE_WRITE_AUTHORIZATION_OK`
- `NA0386_RESPONSE_ARCHIVE_BACKUP_COVERAGE_OK`
- `NA0386_LOCAL_CONTINUITY_CAVEAT_OK`
- `NA0386_REAL_ARCHIVE_WRITE_HELPER_OK`
- `NA0386_ALLOW_REAL_ARCHIVE_EXPLICIT_OK`
- `NA0386_REAL_ARCHIVE_NO_SECRET_SCAN_OK`
- `NA0386_REAL_ARCHIVE_NO_OVERWRITE_OK`
- `NA0386_REAL_ARCHIVE_COLLISION_OK`
- `NA0386_REAL_ARCHIVE_PATH_CHECKSUM_OK`
- `NA0386_NO_INDEX_MUTATION_OK`
- `NA0386_NO_DELETE_OK`
- `NA0386_BACKUP_IMPACT_OK`
- `NA0386_NO_WORKFLOW_CHANGE_OK`
- `NA0386_NO_DEPENDENCY_CHANGE_OK`
- `NA0386_NO_RUNTIME_CHANGE_OK`
- `NA0386_NO_SECRET_MATERIAL_OK`
- `NA0386_METADATA_RUNTIME_REAL_ARCHIVE_WRITE_OK`

## Positive cases

- Existing NA-0384 temp-output fixture matrix still passed 22/22 cases.
- NA-0386 temp-output baseline write passed under `/srv/qbuild/tmp`.
- Real archive metadata/body validate only with both gates present.
- Real archive dry-run computes the expected archive filename without writing.
- Collision suffix behavior remains shared with temp-output simulation.
- JSON summary serialization includes command, path/candidate, markers, errors,
  and checksum after writes.

## Negative / fail-closed cases

The fixture matrix proved rejection for:

- real archive output without CLI gate;
- real archive output without metadata gate;
- real archive output to a wrong directory;
- `no_secret_required: false`;
- secret sentinel in body;
- existing output path with collision disabled;
- index output request;
- delete/cleanup request;
- malformed metadata;
- invalid target;
- invalid directive suffix;
- invalid timestamp;
- missing required section.

## Real archive smoke write proof

Pre-write archive count: `166` files.

Post-write archive count: `167` files.

Exactly one new file appeared:

`/home/victor/work/qsl/codex/responses/NA0386_20260530T080430-0500_D205.md`

Existing changed count from before/after SHA-256 manifests: `0`.

Missing existing file count: `0`.

Smoke file mode/owner/size:

`mode=600 owner=victor:victor size=2504`

Smoke file SHA-256:

`2d06eb23330873576f813d875dadb08b5b26c019138f9cef77af27b8d20b5e40`

The file contains the synthetic marker, local-continuity caveat, and public claim
boundary required by the directive.

## No-overwrite / no-delete / no-index proof

- A post-write dry-run with collision enabled selected `_r2` without writing.
- A post-write dry-run with `--no-collision` rejected the existing base path.
- The before/after archive checksum manifest showed zero existing files changed.
- No response, directive, journal, or local-history index/catalog path appeared
  under `/home/victor/work/qsl/codex` during the smoke proof.
- The helper uses exclusive-create `open("x")` for writes and has no delete path.

## No-mutation / no-network / no-secret proof

Changed qsl-protocol paths are limited to the helper, NA-0386 fixture directory,
governance evidence, testplan, D-0754, TRACEABILITY, and rolling journal.

The helper imports only Python standard library modules. It does not import or
call network, GitHub, shell, git, or subprocess APIs.

The smoke file body and metadata passed the helper no-secret scan before write.

## Backup / local continuity caveat

Read-only post-write backup check:

- `/backup/qsl` remained mounted as ext4.
- The local continuity source list includes `/home/victor/work/qsl/codex/responses`.
- Latest daily snapshot: `daily-20260530T023019-0500`.
- The latest daily snapshot includes the response archive directory.
- The latest daily snapshot predates this NA-0386 smoke file and does not prove
  that the smoke file has been snapshotted or restored.

Classification: same-host local continuity covered for the archive directory;
disaster recovery is not established; off-host backup remains absent.

## Runtime / service / dependency / workflow boundary

NA-0386 changes no `.github` workflow, Cargo manifest, Cargo lockfile, qsl/qsc
runtime, qsp/protocol, crypto/key-schedule, qshield runtime, qsl-server,
qsl-attachments, qsc-desktop, website, docs/public, README, START_HERE, backup
script, timer, fstab, service, source list, branch protection, or public-safety
configuration.

## qsl-server / qsl-attachments boundary

Read-only boundary evidence remains:

- qsl-server PR #56 merged at `d40e6003fdf0`.
- qsl-attachments PR #37 merged at `96b9352bd63`.

These are service-local prerequisite evidence only and are not production,
public-internet, or external-review-complete proof.

## Public-claim boundary

No production, public-internet, anonymity, metadata-free, untraceable,
external-review-complete, off-host-backup-complete, or disaster-recovery-complete
claim is made by NA-0386.

## Selected successor

Selected successor:

`NA-0387 -- QSL Local Ops Response Archive Index and History Catalog Authorization Plan`

## Rejected alternatives

- Implement response/archive indexing now: rejected because index policy must be
  separately authorized and backup-aware.
- Write multiple archive files: rejected by the one-smoke-file authorization.
- Delete the smoke artifact: rejected because retention requires separate
  authorization.
- Change backup plans or run backup/restore tooling: rejected as out of scope.
- Change workflows or dependencies: rejected as unnecessary.

## Next recommendation

Close NA-0386 only after the implementation PR merges and post-merge
public-safety is green. Then restore NA-0387 as the sole READY item without
implementing indexing.
