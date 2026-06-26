Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-26

# NA-0542 QSL Local Ops SSD Hygiene Shared Cargo Target Authorization Plan

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0542 is authorization-only. It reviews the operator-installed qbuild SSD
maintenance arrangement read-only, formalizes the maintenance and pressure
policy, selects a shared Cargo target design for ordinary qsl-protocol lanes,
preserves proof-root-isolated targets where evidence sensitivity requires them,
and selects the exact implementation successor.

Classification:
`LOCAL_OPS_SSD_HYGIENE_SHARED_TARGET_IMPLEMENTATION_READY`.

Selected successor:
`NA-0543 -- QSL Local Ops SSD Hygiene / Shared Cargo Target Implementation Harness`.

No local-ops implementation is performed by NA-0542. This lane does not mutate
the installed maintenance script, systemd units, qwork/qbuild source, Cargo
configuration, shared targets, backup state, qsc source, dependencies,
workflows, public content, qsl-server, or qsl-attachments.

## qwork Proof Verification

- Codex did not run qwork, qstart, or qresume.
- Startup proof files were read from
  `/srv/qbuild/work/NA-0542/.qwork/startup.qsl-protocol.kv` and
  `/srv/qbuild/work/NA-0542/.qwork/startup.qsl-protocol.json`.
- The `.kv` and `.json` files mirrored required values:
  `startup_result=OK`, lane `NA-0542`, repo `qsl-protocol`, path
  `/srv/qbuild/work/NA-0542/qsl-protocol`, `head_equals_origin_main=yes`,
  clean worktree/index/untracked state, `ready_count=1`, top READY
  `NA-0542`, and requested lane status READY.
- Proof HEAD and proof origin/main matched live pre-fetch HEAD and origin/main
  at `dedd73199619`.
- Proof timestamp `2026-06-26T00:39:46Z` postdated the D454 response timestamp
  `2026-06-26T00:17:21Z`.
- Fetch occurred only after proof/live ref matching and disk gates passed.

## D454/D453 Inheritance

- D454 closed NA-0541, merged implementation PR #1355 at `7060fc40b57b`,
  merged closeout PR #1356 at `dedd73199619`, and restored NA-0542 as the sole
  READY item.
- D454 recorded classification
  `DAILY_PUBLIC_PROGRESS_SITE_ACCURACY_IMPLEMENTATION_PASS`.
- D454 verified post-merge public-safety and advisories success.
- D454 recorded the first dated public Progress entry for June 25, 2026 and the
  authorized public accuracy sweep.
- D453 and D452 inheritance confirm the daily Progress cadence and public-sync
  work were complete before this lane.
- D-1072 and D-1073 exist once. D-1074 and D-1075 were absent before this
  authorization.
- NA-0541 and NA-0540 were DONE, and READY_COUNT was 1 with NA-0542 READY.

## Operator Maintenance Context

The operator already performed emergency and preventive local maintenance:

- root filesystem usage was reduced from 95% to approximately 37%;
- old per-lane qsl-protocol target directories were deleted;
- 1,133 old proof roots, approximately 33 GB, were archived from
  `/srv/qbuild/tmp` to `/backup/qsl`;
- archived proof-root paths were retained through symlinks;
- `/usr/local/sbin/qbuild-ssd-maintenance` was installed as root:root mode 755;
- `qbuild-ssd-maintenance.service` and `qbuild-ssd-maintenance.timer` were
  installed, enabled, and activated;
- the timer is scheduled nightly around 03:30 America/Chicago with randomized
  delay;
- the script defaults to dry-run and the service invokes explicit apply mode;
- `/srv/qbuild/cache` remains on SSD.

This context is operator-owned local state. It is not tracked implementation
evidence. No protocol claim is made from this context. No security-completion
claim is made from this context. No public-readiness claim is made from this
context. No backup/restore-complete claim is made from this context.

## Installed Maintenance Inventory

Proof artifacts:

- `maintenance_installation/installed_maintenance_inventory.md`
- `maintenance_installation/installed_maintenance_inventory.json`

Read-only inventory found:

- Script path: `/usr/local/sbin/qbuild-ssd-maintenance`.
- Script owner/mode: root:root mode 755.
- Script SHA-256: `53e9820f0ece`.
- Default mode: dry-run.
- Service ExecStart:
  `/usr/local/sbin/qbuild-ssd-maintenance --apply --target-days 7 --tmp-days 7 --warn-percent 90`.
- Service context: root by default; no `User=` or `Group=` override.
- Service success policy: default only; `SuccessExitStatus` is empty.
- Service timeout: infinity.
- Service mount condition: `ConditionPathIsMountPoint=/backup/qsl`.
- Timer calendar: `*-*-* 03:30:00`.
- Timer randomized delay: 20 minutes.
- Timer persistence: false.
- Timer state: enabled and active.
- Next run: `2026-06-26 03:42:32 CDT`.

Script behavior:

- Target cleanup candidates are
  `/srv/qbuild/work/*/qsl-protocol/target` directories older than 7 days.
- Proof-root archive candidates are top-level `/srv/qbuild/tmp/NA*`
  directories older than 7 days.
- `/backup/qsl` mountpoint is required.
- A nonblocking flock protects the maintenance run.
- Active cargo/rustc/sccache/qwork/qstart/qresume processes cause exit 3.
- Proof roots are copied to `/backup/qsl/qbuild-tmp-archive/YYYY/MM`, original
  paths are replaced with symlinks, and `/srv/qbuild/cache` is excluded.

## Scheduled Run Review

Proof artifacts:

- `maintenance_run_review/nightly_run_review.md`
- `maintenance_run_review/nightly_run_review.json`

Classification:
`MAINTENANCE_NO_SCHEDULED_RUN_YET`.

The timer is enabled and active, but `LastTriggerUSec` is empty and the service
journal has no entries. The first scheduled run is still pending. The latest
housekeeping log is an operator/manual apply-mode run, not a timer run. That
manual run ended with `QBUILD_SSD_MAINTENANCE_OK`, archived 1,133 proof roots,
deleted zero target directories, and reported root usage falling from 45% to
37%. No stop, warning, or skip marker was present in that manual log.

## Storage and Cache Inventory

Proof artifacts:

- `storage_inventory/qbuild_storage_inventory.md`
- `storage_inventory/qbuild_storage_inventory.json`

Inventory:

- `/srv/qbuild/work`: 52G.
- Lane work directories: 292.
- Remaining per-lane `qsl-protocol/target` directories: 28.
- Remaining per-lane target total: about 43GB.
- Per-lane target age buckets: 10 are 1-3 days old, 16 are 3-7 days old, and
  2 are older than 7 days.
- `/srv/qbuild/tmp`: 65G.
- `/srv/qbuild/tmp` direct entries: 1810.
- Live top-level tmp directories: 150.
- Archived symlinks: 1133.
- Broken archived symlinks: 0.
- `/srv/qbuild/cache`: 21G.
- `/srv/qbuild/cache/targets`: 17G.
- `/srv/qbuild/cache/targets/qsl-protocol`: 17G.
- `/backup/qsl/qbuild-tmp-archive`: 37G.
- Housekeeping logs: 2.
- sccache is not installed.
- Current disk: `/` 37% used; `/backup/qsl` 33% used.

Largest rebuildable SSD consumers are remaining lane-local targets under
`/srv/qbuild/work` and the existing shared target under
`/srv/qbuild/cache/targets/qsl-protocol`.

## Current Maintenance Safety Review

Proof artifacts:

- `policy/current_maintenance_safety_review.md`
- `policy/current_maintenance_safety_review.json`

Safety findings:

- Candidate age uses only top-level directory `mtime`; a future implementation
  must inspect newest descendant `mtime` so recent nested writes are not missed.
- Active-process detection exists and is fail-closed for obvious cargo/rustc,
  sccache, qwork, qstart, and qresume activity. Future code should avoid
  self-match and path-regex ambiguity and add lane/worktree recency plus
  dirty-worktree checks before deleting lane-local targets.
- Archive flow copies before cutover, then moves the source to a
  `.moved.<timestamp>` path, creates a symlink, and removes the moved path.
  Future code must verify copy completeness before source removal and document
  `.moved.<timestamp>` recovery.
- Symlink preservation currently looks healthy: 1,133 archived symlinks and
  zero broken symlinks.
- systemd should distinguish intentional active-build safe skips from failures,
  while keeping disk warning exits non-success.
- Logs are on `/backup/qsl`, but future logs need a machine-readable summary,
  bounded retention, symlink verification, skipped paths, failed paths, and
  latest-result markers.

## qwork/qbuild Architecture Discovery

Proof artifacts:

- `qwork_qbuild_discovery/qwork_qbuild_architecture.md`
- `qwork_qbuild_discovery/qwork_qbuild_architecture.json`

Findings:

- `qwork` resolves to `/home/victor/.local/bin/qwork`, a wrapper that execs
  `/srv/qbuild/tools/qwork.sh`.
- `qstart`, `qresume`, and `qbuild` were not resolvable in this shell.
- `/srv/qbuild/tools/qwork.sh` is local operator state, not tracked by this
  repository.
- `/srv/qbuild/tools/env_qbuild.sh` defines `CARGO_HOME`,
  `RUSTUP_HOME`, `SCCACHE_DIR`, `TMPDIR`, and `qbuild_target_dir`, currently
  `/srv/qbuild/cache/targets/<repo>`.
- Current qwork creates `.qwork` proof files, performs lane locking, verifies
  clean main, and may create checkouts through `new_checkout.sh`.
- Current qwork does not create target symlinks and does not create lane-local
  Cargo config files.
- Current qwork cannot export `CARGO_TARGET_DIR` into the parent shell that
  later launches Codex because it runs as a child process.
- Future implementation ownership is clear: this repository may add tracked
  runbooks, operator action bundles, and tracked script templates; the installed
  `/srv/qbuild/tools`, `/home/victor/.local/bin`, `/usr/local/sbin`,
  `/etc/systemd/system`, and `/srv/qbuild/cache` changes remain operator-owned
  actions verified later read-only.

## Shared Cargo Target Options

Options reviewed:

- Option 1 - qwork/qbuild-managed shared SSD target: selected.
- Option 2 - lane target symlink to shared SSD target: rejected because it
  risks provenance and cleanup ambiguity.
- Option 3 - per-lane targets plus nightly cleanup only: rejected because it
  retains major duplicate rebuild cost and SSD churn.
- Option 4 - shell-profile or global `CARGO_TARGET_DIR`: rejected because it
  can affect unrelated repositories and provenance-sensitive work.
- Option 5 - shared target on platter: rejected because ordinary builds should
  stay on SSD.
- Option 6 - sccache only: rejected because sccache is not installed and would
  not remove duplicated target/link artifacts by itself.

## Selected Shared Target Design

Proof artifacts:

- `shared_target_design/shared_cargo_target_design.md`
- `shared_target_design/shared_cargo_target_design.json`

Selected ordinary shared target base:
`/srv/qbuild/cache/targets/qsl-protocol`.

Selected ordinary current partition:
`/srv/qbuild/cache/targets/qsl-protocol/rustc-1.95.0-x86_64-unknown-linux-gnu/default`.

The current partition is tied to the observed toolchain:

- `cargo 1.95.0`.
- `rustc 1.95.0`.
- host class: `x86_64-unknown-linux-gnu`.
- ordinary default build class only.

If NA-0543 observes a different rustc/toolchain class before implementation, it
must STOP and request refreshed path authorization rather than silently moving
the shared target.

Ownership/mode policy:

- Parent `/srv/qbuild/cache/targets` may remain root:victor setgid as currently
  observed.
- Repo/class target directories should be victor:victor mode 2775.
- No global shell profile or committed Cargo config is authorized solely for
  local qbuild.

## Isolated Target Exceptions

Proof-root-isolated targets remain mandatory for:

- binary provenance or binary hash evidence;
- remote binary staging or restaging;
- reproducibility-sensitive builds;
- sanitizer, Miri, fuzz, or toolchain-specialized runs when collision risk
  exists;
- directives explicitly requiring isolated build artifacts;
- any case where source-to-binary attribution would otherwise be ambiguous.

Shared targets speed ordinary validation. They do not prove reproducibility and
must not be used as the sole basis for binary provenance claims.

## Environment Precedence

Selected precedence:

1. A directive-provided `CARGO_TARGET_DIR` wins.
2. A preexisting operator-provided `CARGO_TARGET_DIR` wins.
3. The ordinary qwork/qbuild shared default applies only when
   `CARGO_TARGET_DIR` is unset.

qwork must not overwrite an explicit target override. The chosen default must be
visible in qwork proof or a lane-local `.qwork` environment proof so later
validation can distinguish ordinary shared-target builds from isolated-target
builds.

## Concurrency and Locking Policy

- One-lane-at-a-time governance remains the primary collision reducer.
- qwork's lane lock remains mandatory.
- Future shared-target setup should use a repo/class advisory lock before
  creating or changing the target directory.
- Maintenance must skip cleanup when cargo/rustc/sccache/qwork/qstart/qresume
  activity is detected.
- Shared-target pressure cleanup must be separately authorized and must require
  active-process absence, exact path checks, and size reporting.
- No maintenance process may move active build targets to platter.

## Pressure Threshold Policy

Selected thresholds:

- Healthy: root usage below 80%.
- Warning: root usage at or above 80%.
- Pressure: root usage at or above 85%.
- Urgent: root usage at or above 90%.
- Hard build/directive stop: root usage at or above 95%.

Warning state is report-only. Pressure and urgent states require a separately
authorized bounded pressure-cleanup policy. No automatic broad deletion is
authorized merely because warning threshold is crossed.

## Cleanup and Retention Policy

Normal nightly policy:

- old per-lane target cleanup age: 7 days;
- old proof-root archive age: 7 days;
- `/backup/qsl` must be mounted;
- active build/qwork detection is required;
- path-preserving archive links are required;
- shared targets are excluded from routine age-based deletion.

Shared target cleanup is delete/rebuild only and only under exact, separately
authorized pressure policy. Backup or proof archive deletion is not ordinary SSD
pressure relief.

## Archive Transaction Policy

Future implementation must preserve copy-before-cutover semantics but harden the
transaction:

- create destination under verified `/backup/qsl` mount;
- detect destination collisions before copying;
- copy source to archive destination;
- verify file count and byte count or run an rsync dry-diff before source
  removal;
- move source to `.moved.<timestamp>`;
- create symlink from original path to archive destination;
- verify symlink target;
- remove `.moved.<timestamp>` only after verification succeeds;
- document recovery for partial destination, missing symlink, and retained
  `.moved.<timestamp>` cases.

## Logging and Monitoring Policy

Future implementation must write:

- human-readable log on `/backup/qsl`;
- machine-readable summary on `/backup/qsl`;
- before/after `df`;
- candidate counts and bytes;
- reclaimed bytes;
- archived, skipped, and failed paths;
- symlink verification result;
- timer and service state;
- next run;
- latest result classification;
- warning if no successful or safe-skip run occurs within 48 hours.

Log retention: 90 days on
`/backup/qsl/qbuild-tmp-archive/housekeeping-logs`. Logs must not fill the SSD.

## Operator/Codex Boundary

Codex may mutate only tracked paths explicitly authorized by a future directive.
Codex must not run qwork/qstart/qresume, sudo, qsl-backup, maintenance apply,
systemctl mutation, daemon-reload, target creation, symlink creation, deletion,
archive, relink, or backup mutation.

Operator-owned local actions include:

- installing or replacing `/usr/local/sbin/qbuild-ssd-maintenance`;
- installing or replacing systemd units under `/etc/systemd/system`;
- mutating `/srv/qbuild/tools/env_qbuild.sh`;
- mutating `/srv/qbuild/tools/qwork.sh`;
- correcting `/home/victor/.local/bin/qwork` ownership or mode if selected;
- creating or changing `/srv/qbuild/cache/targets/qsl-protocol/...`;
- daemon-reload, timer enable/restart, and service/timer management.

Future implementation must emit an exact reviewed operator action bundle.
Operator executes privileged/local actions. Codex verifies results later
read-only.

## Implementation Ownership

Codex-repo mutation class:

- tracked runbook;
- tracked operator action bundle;
- tracked script templates;
- governance evidence;
- testplan;
- DECISIONS, TRACEABILITY, and rolling journal updates.

Operator-owned local mutation class:

- `/srv/qbuild/tools` qwork/qbuild helpers;
- `/home/victor/.local/bin/qwork`;
- `/usr/local/sbin`;
- `/etc/systemd/system`;
- `/srv/qbuild/cache/targets`;
- daemon-reload and timer/service management.

Read-only proof review class:

- qwork proof files;
- systemd status and journals;
- housekeeping logs;
- disk and storage metadata;
- target/cache metadata;
- qsl-backup digest and source-list inclusion proof.

## Exact Future Path Bundle

Tracked paths authorized for the NA-0543 implementation harness:

- `docs/ops/DOC-OPS-005_qbuild_SSD_Hygiene_and_Shared_Cargo_Target_Runbook_v0.1.0_DRAFT.md`
- `docs/ops/NA-0543_qbuild_operator_action_bundle.md`
- `scripts/local_ops/qbuild-ssd-maintenance.sh`
- `scripts/local_ops/qbuild-shared-target-env.sh`
- `docs/governance/evidence/NA-0543_qsl_local_ops_ssd_hygiene_shared_cargo_target_implementation_harness.md`
- `tests/NA-0543_qsl_local_ops_ssd_hygiene_shared_cargo_target_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Operator-owned action paths for NA-0543:

- `/srv/qbuild/tools/env_qbuild.sh`
- `/srv/qbuild/tools/qwork.sh`
- `/home/victor/.local/bin/qwork`
- `/usr/local/sbin/qbuild-ssd-maintenance`
- `/etc/systemd/system/qbuild-ssd-maintenance.service`
- `/etc/systemd/system/qbuild-ssd-maintenance.timer`
- `/srv/qbuild/cache/targets/qsl-protocol/rustc-1.95.0-x86_64-unknown-linux-gnu/default`
- `/backup/qsl/qbuild-tmp-archive/housekeeping-logs`
- `/backup/qsl/qbuild-local-ops-rollback/NA-0543/pre-change`

Required NA-0543 proof-review outputs:

- `maintenance_installation/post_install_inventory.md`
- `maintenance_installation/post_install_inventory.json`
- `maintenance_run_review/post_install_run_review.md`
- `maintenance_run_review/post_install_run_review.json`
- `shared_target/post_operator_shared_target_verification.md`
- `shared_target/post_operator_shared_target_verification.json`
- `rollback/rollback_bundle_inventory.md`
- `rollback/rollback_bundle_inventory.json`
- `validation/na0543_scope_guard.txt`
- `validation/na0543_marker_proof.txt`

No wildcard mutation authority is granted.

## Operator Action Bundle Requirements

The NA-0543 operator action bundle must:

- default to dry-run where applicable;
- show exact install paths, owners, groups, modes, and SHA-256 values;
- create pre-change copies under
  `/backup/qsl/qbuild-local-ops-rollback/NA-0543/pre-change`;
- preserve root-owned maintenance script and unit ownership;
- create the selected shared target path with victor:victor mode 2775;
- avoid shell profiles and global Cargo config;
- not include credentials, tokens, auth headers, private keys, or private proof
  contents;
- require operator execution for privileged/local actions;
- require read-only Codex proof review after operator execution.

## Rollback Plan

Rollback is operator action only. Codex must not run these commands.

Required rollback source paths:

- `/backup/qsl/qbuild-local-ops-rollback/NA-0543/pre-change/usr_local_sbin_qbuild-ssd-maintenance`
- `/backup/qsl/qbuild-local-ops-rollback/NA-0543/pre-change/qbuild-ssd-maintenance.service`
- `/backup/qsl/qbuild-local-ops-rollback/NA-0543/pre-change/qbuild-ssd-maintenance.timer`
- `/backup/qsl/qbuild-local-ops-rollback/NA-0543/pre-change/srv_qbuild_tools_env_qbuild.sh`
- `/backup/qsl/qbuild-local-ops-rollback/NA-0543/pre-change/srv_qbuild_tools_qwork.sh`
- `/backup/qsl/qbuild-local-ops-rollback/NA-0543/pre-change/home_victor_local_bin_qwork`

Required operator rollback commands:

```bash
sudo install -o root -g root -m 755 /backup/qsl/qbuild-local-ops-rollback/NA-0543/pre-change/usr_local_sbin_qbuild-ssd-maintenance /usr/local/sbin/qbuild-ssd-maintenance
sudo install -o root -g root -m 644 /backup/qsl/qbuild-local-ops-rollback/NA-0543/pre-change/qbuild-ssd-maintenance.service /etc/systemd/system/qbuild-ssd-maintenance.service
sudo install -o root -g root -m 644 /backup/qsl/qbuild-local-ops-rollback/NA-0543/pre-change/qbuild-ssd-maintenance.timer /etc/systemd/system/qbuild-ssd-maintenance.timer
install -m 775 /backup/qsl/qbuild-local-ops-rollback/NA-0543/pre-change/srv_qbuild_tools_env_qbuild.sh /srv/qbuild/tools/env_qbuild.sh
install -m 775 /backup/qsl/qbuild-local-ops-rollback/NA-0543/pre-change/srv_qbuild_tools_qwork.sh /srv/qbuild/tools/qwork.sh
install -m 755 /backup/qsl/qbuild-local-ops-rollback/NA-0543/pre-change/home_victor_local_bin_qwork /home/victor/.local/bin/qwork
sudo systemctl daemon-reload
sudo systemctl restart qbuild-ssd-maintenance.timer
```

Rollback must not delete the shared target directory by default. If shared target
deletion is needed, it requires separate exact-path pressure-cleanup
authorization.

## Best-Known-Method Review

- Shared target remains on SSD.
- Proof-root-isolated targets remain available.
- One-lane-at-a-time governance reduces collisions.
- Explicit target override takes precedence.
- No global unrelated-repository effect is authorized.
- Rebuildable targets are deleted/rebuilt, not archived to slow storage.

## Hostile Build Engineer Review

- Shared target must not create false provenance.
- Binary hashes must never be attributed to source without isolated or otherwise
  proven build context.
- Toolchain, RUSTFLAGS, profile, fuzz, Miri, and sanitizer collisions are
  bounded by isolated-target exceptions.
- Concurrent builds are bounded by governance, qwork locking, and active-process
  maintenance skips.
- qwork must not overwrite explicit target overrides.
- Rollback restores ordinary lane-local behavior by removing the qwork default,
  not by deleting build output.

## Red-Team Review

- The installed maintenance script is root-owned and not user-writable.
- Future service hardening must not follow attacker-controlled paths.
- Archive destination must remain mounted and verified.
- Symlink handling must stay inside approved `/srv/qbuild/tmp` and
  `/backup/qsl/qbuild-tmp-archive` roots.
- Active-build detection must avoid self-match and obvious misses.
- Internal/private proof contents must not be exposed.
- Operator action bundle must contain no secrets.
- No broad `rm -rf` target may be constructed from unvalidated input.

## Production SRE Review

- Maintenance must not compete aggressively with active builds.
- Timer result must be observable.
- Safe skips must be distinguishable from failures.
- Logs must be bounded and on `/backup/qsl`.
- Pressure states must be visible before hard stop.
- Nightly maintenance must not become a hidden single point of failure.
- A missed run is safer than cleanup during active work.
- Restore and rollback instructions must be exact and operator-run.

## Claim Boundary

- No public-readiness claim.
- No production-readiness claim.
- No public-internet-readiness claim.
- No external-review-complete claim.
- No reproducibility-complete claim.
- No backup/restore-complete claim.
- No crypto-complete claim.
- No identity-complete claim.
- No trust-complete claim.
- No replay-proof claim.
- No downgrade-proof claim.
- No vulnerability-free claim.
- No bug-free claim.
- No perfect-build claim.

## Authorization Decision

NA-0542 authorizes a future implementation harness and selects classification
`LOCAL_OPS_SSD_HYGIENE_SHARED_TARGET_IMPLEMENTATION_READY`.

The current maintenance installation is safe enough to remain operator-owned
local state, but it needs hardening before being treated as a complete tracked
local-ops implementation baseline. The qwork/qbuild architecture is clear
enough to implement a tracked operator bundle and later read-only proof review.

## Successor Selection

Selected successor:
`NA-0543 -- QSL Local Ops SSD Hygiene / Shared Cargo Target Implementation Harness`.

NA-0543 must implement only the exact tracked path bundle above, emit the exact
operator action bundle, and verify operator-owned local changes read-only after
operator execution. NA-0543 must not run qwork/qstart/qresume, sudo,
qsl-backup, maintenance apply, daemon-reload, systemctl mutation, or
operator-owned privileged actions.

## Future Validation Markers

The NA-0543 testplan must include markers for:

- D-1074 authorization consumed.
- exact tracked path bundle used.
- exact operator-owned path bundle used.
- maintenance hardening template generated.
- shared target environment template generated.
- operator action bundle generated.
- no privileged action by Codex.
- post-operator proof review completed.
- rollback bundle inventory completed.
- isolated-target exceptions preserved.
- no qwork/qstart/qresume execution by Codex.
- no qsl-backup execution.
- no qsc source/test/fuzz/Cargo mutation unless separately authorized.
- no dependency/lockfile mutation unless separately authorized.
- no qsl-server/qsl-attachments use.
- exactly one READY invariant.

## No-Implementation Boundary

NA-0542 did not:

- mutate `/usr/local/sbin/qbuild-ssd-maintenance`;
- mutate `/etc/systemd/system` units;
- run maintenance apply or dry-run;
- start, stop, restart, enable, disable, or daemon-reload systemd;
- mutate qwork/qbuild source or configuration;
- run qwork/qstart/qresume;
- create or mutate shared targets;
- create target symlinks;
- delete, move, archive, or relink any path;
- run qsl-backup;
- mutate backup state;
- mutate qsc source/test/fuzz/Cargo paths;
- mutate dependencies or lockfiles;
- mutate workflows;
- use qsl-server or qsl-attachments;
- mutate public content.

## Backup Impact

No qsl-backup command was run. No backup content was inspected. No backup state
was mutated. `/backup/qsl` was inspected read-only for mount, archive layout,
housekeeping logs, and aggregate sizing. The installed qsl-backup helper digest
and source-list inclusion were checked read-only.

## Recommendation

Proceed to NA-0543 after this authorization is merged and closed out. NA-0543
should generate the tracked runbook, operator action bundle, and script
templates, then leave privileged/local installation to the operator and verify
the result read-only in a later proof phase.
