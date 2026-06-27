Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-26

# NA-0544 QSL Local Ops SSD Hygiene Shared Cargo Target Operator Action Proof Review

## Executive Summary

NA-0544 reviewed the human operator action proof for the NA-0543 local-ops
SSD hygiene and shared Cargo target bundle. The result classification is:

`LOCAL_OPS_SSD_HYGIENE_SHARED_TARGET_OPERATOR_ACTION_PROOF_REVIEW_PASS`

The review was read-only for installed/operator-owned state. Codex did not run
qwork, qstart, qresume, sudo, qsl-backup, systemctl mutation commands,
maintenance dry-run/apply, remote commands, qsc send/receive or E2EE,
qsl-server, or qsl-attachments.

## qwork Proof Verification

Fresh qwork proof was verified from the operator handoff written at
`2026-06-27T00:53:48Z`.

Verified startup fields:

- `startup_result=OK`
- `lane=NA-0544`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0544/qsl-protocol`
- HEAD, `origin/main`, and `main` at `b4a64f78efe7`
- clean worktree, index, and untracked state
- `ready_count=1`
- `queue_top_ready=NA-0544`
- `requested_lane_status=READY`
- `qwork_version_or_sha=175b0ea1d5b9abc07bdab66e9b92446e2a3d533018468e94a95c26f8698f86cf`

The `.kv` and JSON files mirrored the required values. The cargo-target env
file exported the same shared-target selection.

## D-1076 / D-1077 Inheritance

D-1076 exists once, is Accepted, and records
`LOCAL_OPS_SSD_HYGIENE_SHARED_TARGET_OPERATOR_ACTION_BUNDLE_READY`. D-1077
exists once, is Accepted, marked NA-0543 DONE, and restored NA-0544 as the sole
READY successor.

The NA-0543 operator bundle begins with `DO NOT RUN UNTIL DIRECTOR REVIEW` and
requires rollback capture, shared-target preparation, canonical script install,
systemd unit install, qwork/env/wrapper update, timer transition, post-install
dry-run, qwork proof, explicit-target proof, unrelated-repository negative
proof, and rollback proof.

## Operator Action Transcript Summary

The operator transcript was treated as operator-supplied proof, not as
forensic proof. NA-0544 verified every still-observable fact against current
proof files, file metadata, hashes, systemd state, maintenance logs, disk
state, and qwork helper outputs.

## Rollback Inventory Review

Rollback inventory exists under
`/backup/qsl/qbuild-local-ops-rollback/NA-0543/pre-change` and matches the
expected hashes:

- `usr_local_sbin_qbuild-ssd-maintenance`: `53e9820f0ece7329864fb5716b9f08fbe64af1f38a775c505a62685422b845c5`
- `qbuild-ssd-maintenance.service`: `d4f37b7ed0eee160ee0593fdddc03b747785d965d5ed0931c6a53a5c84db3d8c`
- `qbuild-ssd-maintenance.timer`: `d50064ce5878a2910bd27c86dfb2631bf3bc94c5e7aee30c564704c94b323102`
- `srv_qbuild_tools_env_qbuild.sh`: `621c7d97c685627ef3f921c89161aed57c63a540daa5d408ef9b1956c4ece294`
- `srv_qbuild_tools_qwork.sh`: `438b81623d3a461213974af1ea3219105aa8c14c988cf2de835f1df240eff742`
- `home_victor_local_bin_qwork`: `9b5d1eb57fac554c0f5ad0d9dc168df86aa3e847565ad6bc3120155dd333e65b`

The captured old wrapper is a symlink to `/srv/qbuild/tools/qwork`, matching
the operator report.

## Installed Maintenance Script Review

The installed canonical maintenance script at
`/usr/local/sbin/qbuild-ssd-maintenance` is `root:root`, mode `755`, and
matches:

`84f20f9e3189b8994f378ac8f8a2cde5a99f899496a8b393b7b15800a1149b0c`

## Installed Systemd Unit Review

The installed systemd unit hashes match:

- service: `7841add332a4ee37bd53b29e20b5d698c9e3f6014bec65ff98b6fc723b01c089`
- timer: `d50064ce5878a2910bd27c86dfb2631bf3bc94c5e7aee30c564704c94b323102`

Both files are `root:root`, mode `644`.

## Installed qwork / env / wrapper Review

Installed qwork/env/wrapper hashes match:

- `/srv/qbuild/tools/env_qbuild.sh`: `adf2c6dc8d719ca5b3a27260e8868ad7fde4d32ae98d19fdf8c5f10c3e30020d`
- `/srv/qbuild/tools/qwork.sh`: `175b0ea1d5b9abc07bdab66e9b92446e2a3d533018468e94a95c26f8698f86cf`
- `/home/victor/.local/bin/qwork`: `97f27ced9daa2444742ba4a881f580c25b5cef1a670d83cb77668c7c4c791ee9`

The installed wrapper is a regular executable shell script, not a symlink.

## Shared Target Review

The shared target path exists as real directories, not symlinks:

- `/srv/qbuild/cache/targets/qsl-protocol`
- `/srv/qbuild/cache/targets/qsl-protocol/rustc-1.95.0-x86_64-unknown-linux-gnu`
- `/srv/qbuild/cache/targets/qsl-protocol/rustc-1.95.0-x86_64-unknown-linux-gnu/default`

All three are owned `victor:victor` with mode `2775`. The qwork proof target
dir matches the default directory.

## qwork Target-Selection Proof

Default qwork target proof shows:

- `cargo_target_mode=shared`
- `cargo_target_dir=/srv/qbuild/cache/targets/qsl-protocol/rustc-1.95.0-x86_64-unknown-linux-gnu/default`
- `cargo_target_source=qwork-default`
- `cargo_target_build_class=default`
- `cargo_target_toolchain_key=rustc-1.95.0-x86_64-unknown-linux-gnu`
- `explicit_target_preserved=no`
- `shared_target_ready=yes`

## Explicit Target Preservation Proof

The explicit-target helper proof shows:

- `cargo_target_mode=explicit`
- `cargo_target_source=preexisting-env`
- `explicit_target_preserved=yes`
- `shared_target_ready=not-applicable`

## Unrelated Repository Negative Proof

The unrelated-repository helper proof exits `2`, emits the expected rejection
phrase for an unauthorized ordinary shared target, and has empty stdout. This
proves the helper rejects unrelated repositories fail-closed.

## Post-Install Dry-Run Review

The post-install dry-run artifacts are:

- `/backup/qsl/qbuild-tmp-archive/housekeeping-logs/qbuild-ssd-maintenance_20260627T004638Z.log`
- `/backup/qsl/qbuild-tmp-archive/housekeeping-logs/qbuild-ssd-maintenance_20260627T004638Z.json`

The JSON summary reports:

- `version=NA-0543-qbuild-ssd-maintenance-v0.1.0`
- `mode=dry-run`
- `classification=DRY_RUN_CANDIDATES`
- `exit_code=0`
- `target_candidate_count=7`
- `proof_candidate_count=20`
- `broken_symlink_count=0`
- `deleted_count=0`
- `archived_count=0`
- `failed_count=0`
- `reclaimed_bytes=0`

No maintenance apply was run manually by Codex.

## Latest Maintenance State Review

Latest maintenance state classification:

`MAINTENANCE_CANONICAL_DRY_RUN_PASS`

The latest housekeeping JSON is the canonical post-install dry-run. The
service journal also contains an earlier successful scheduled apply before the
canonical install. No scheduled canonical apply after the operator install is
visible yet; the timer is enabled and active, so this is a monitoring
follow-up rather than a failure.

## Timer and Service State

Read-only systemd review found:

- timer enabled
- timer active
- next run listed
- service inactive/dead
- service `Result=success`
- service `ExecMainStatus=0`

The timer transition did not start the service during NA-0544 review.

## Disk and Mount State

Disk and mount review passed:

- `/` usage: 36%
- `/backup/qsl` usage: 34%
- `/backup/qsl` is a mount point

## No qsl-backup / Backup Boundary Review

Codex did not run qsl-backup and did not mutate backup state. Operator-side
no-qsl-backup proof is bounded: NA-0544 relies on the operator transcript plus
observable rollback/log/path state, not full forensic history.

## Operator / Codex Boundary

Codex did not execute the operator action bundle and did not perform
privileged/local actions. Codex did not mutate installed maintenance files,
systemd state, qwork/env/wrapper files, shared-target directories, backup
state, qsc source/test/fuzz/Cargo paths, dependencies, lockfiles, workflows,
remote state, qsl-server, or qsl-attachments.

## Result Classification

`LOCAL_OPS_SSD_HYGIENE_SHARED_TARGET_OPERATOR_ACTION_PROOF_REVIEW_PASS`

## Successor Selection

Selected successor:

`NA-0545 -- QSL Remote/Relay Non-Required CI Failure Forward-Audit Authorization Plan`

Exactly one READY remains mandatory until closeout restores the successor.

## Validation

The proof review produced file-backed artifacts under the NA-0544 proof root
and local validation is required to include scope guard, queue/decision proof,
marker proof, link check, private-material scan, overclaim scan,
docs/governance-only classifier, PR body preflight, goal-lint when available,
root and nested cargo audits, cargo fmt, and qsc-adversarial shell syntax
checks.

## qsl-server / qsl-attachments Boundary

No qsl-server or qsl-attachments command, source mutation, or service work was
performed.

## Claim Boundary

This review makes no public-readiness claim. It makes no production-readiness
claim. It makes no public-internet-readiness claim. It makes no
external-review-complete claim. It makes no reproducibility-complete claim. It
makes no backup/restore-complete claim. It makes no vulnerability-free claim.
It makes no bug-free claim. It makes no perfect-build claim.

## Recommendation

Accept the operator action proof review, merge the NA-0544 governance evidence,
then close out NA-0544 only after post-merge public-safety and advisories are
green. Restore NA-0545 as the next bounded forward-audit lane for the failed
non-required remote/relay checks recorded in D-1077.
