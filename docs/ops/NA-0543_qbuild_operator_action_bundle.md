DO NOT RUN UNTIL DIRECTOR REVIEW

Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL local operations
Last-Updated: 2026-06-26

# NA-0543 qbuild Operator Action Bundle

This bundle is for the human operator, not Codex. Codex generated these
reviewable instructions and did not execute privileged/local installation,
maintenance dry-run/apply, qwork, qstart, qresume, qsl-backup, daemon-reload,
systemctl mutation, shared-target creation, or backup mutation.

Proof-root proposed files for this directive are under:

`/srv/qbuild/tmp/NA0543_local_ops_ssd_shared_target_impl_20260626T032331Z/proposed_operator_files`

## 1. Preconditions

Review the merged NA-0543 PR, D-1076, and this file. Do not continue if the
tracked scripts differ from the proposed file hashes, if `/backup/qsl` is not a
mount point, if root filesystem usage is at or above 95%, if any active build is
running, or if rollback capture cannot be completed first.

## 2. Active-Process Check

```bash
pgrep -af 'cargo|rustc|sccache|qwork|qstart|qresume'
```

Stop unless the output is empty or Director explicitly classifies every line as
safe. Do not run maintenance apply during active or ambiguous build state.

## 3. Disk and Mount Check

```bash
df -h / /backup/qsl
mountpoint /backup/qsl
```

Stop if `/backup/qsl` is not mounted or `/` is at or above 95%.

## 4. Current Installed Hashes/Metadata

```bash
stat -c '%a %U %G %s %y %n' /usr/local/sbin/qbuild-ssd-maintenance /etc/systemd/system/qbuild-ssd-maintenance.service /etc/systemd/system/qbuild-ssd-maintenance.timer /srv/qbuild/tools/env_qbuild.sh /srv/qbuild/tools/qwork.sh /home/victor/.local/bin/qwork
sha256sum /usr/local/sbin/qbuild-ssd-maintenance /etc/systemd/system/qbuild-ssd-maintenance.service /etc/systemd/system/qbuild-ssd-maintenance.timer /srv/qbuild/tools/env_qbuild.sh /srv/qbuild/tools/qwork.sh /home/victor/.local/bin/qwork
```

Save the output for NA-0544 proof review.

## 5. Dry-Run Comparison

Run dry-run only before any install:

```bash
sudo /usr/local/sbin/qbuild-ssd-maintenance --dry-run --target-days 7 --tmp-days 7 --log-retention-days 90
```

Stop if candidates include `/srv/qbuild/cache`, source checkout roots, dangerous
roots, traversal-shaped paths, or unexpected symlink behavior.

## 6. Rollback Capture

```bash
sudo mkdir -p /backup/qsl/qbuild-local-ops-rollback/NA-0543/pre-change
sudo cp -a /usr/local/sbin/qbuild-ssd-maintenance /backup/qsl/qbuild-local-ops-rollback/NA-0543/pre-change/usr_local_sbin_qbuild-ssd-maintenance
sudo cp -a /etc/systemd/system/qbuild-ssd-maintenance.service /backup/qsl/qbuild-local-ops-rollback/NA-0543/pre-change/qbuild-ssd-maintenance.service
sudo cp -a /etc/systemd/system/qbuild-ssd-maintenance.timer /backup/qsl/qbuild-local-ops-rollback/NA-0543/pre-change/qbuild-ssd-maintenance.timer
cp -a /srv/qbuild/tools/env_qbuild.sh /backup/qsl/qbuild-local-ops-rollback/NA-0543/pre-change/srv_qbuild_tools_env_qbuild.sh
cp -a /srv/qbuild/tools/qwork.sh /backup/qsl/qbuild-local-ops-rollback/NA-0543/pre-change/srv_qbuild_tools_qwork.sh
cp -a /home/victor/.local/bin/qwork /backup/qsl/qbuild-local-ops-rollback/NA-0543/pre-change/home_victor_local_bin_qwork
stat -c '%a %U %G %s %y %n' /backup/qsl/qbuild-local-ops-rollback/NA-0543/pre-change/usr_local_sbin_qbuild-ssd-maintenance /backup/qsl/qbuild-local-ops-rollback/NA-0543/pre-change/qbuild-ssd-maintenance.service /backup/qsl/qbuild-local-ops-rollback/NA-0543/pre-change/qbuild-ssd-maintenance.timer /backup/qsl/qbuild-local-ops-rollback/NA-0543/pre-change/srv_qbuild_tools_env_qbuild.sh /backup/qsl/qbuild-local-ops-rollback/NA-0543/pre-change/srv_qbuild_tools_qwork.sh /backup/qsl/qbuild-local-ops-rollback/NA-0543/pre-change/home_victor_local_bin_qwork
sha256sum /backup/qsl/qbuild-local-ops-rollback/NA-0543/pre-change/usr_local_sbin_qbuild-ssd-maintenance /backup/qsl/qbuild-local-ops-rollback/NA-0543/pre-change/qbuild-ssd-maintenance.service /backup/qsl/qbuild-local-ops-rollback/NA-0543/pre-change/qbuild-ssd-maintenance.timer /backup/qsl/qbuild-local-ops-rollback/NA-0543/pre-change/srv_qbuild_tools_env_qbuild.sh /backup/qsl/qbuild-local-ops-rollback/NA-0543/pre-change/srv_qbuild_tools_qwork.sh /backup/qsl/qbuild-local-ops-rollback/NA-0543/pre-change/home_victor_local_bin_qwork
```

Stop if any rollback file is missing.

## 7. Shared-Target Directory Preparation

```bash
sudo install -d -o victor -g victor -m 2775 /srv/qbuild/cache/targets/qsl-protocol
sudo install -d -o victor -g victor -m 2775 /srv/qbuild/cache/targets/qsl-protocol/rustc-1.95.0-x86_64-unknown-linux-gnu
sudo install -d -o victor -g victor -m 2775 /srv/qbuild/cache/targets/qsl-protocol/rustc-1.95.0-x86_64-unknown-linux-gnu/default
stat -c '%a %U %G %s %y %n' /srv/qbuild/cache/targets/qsl-protocol/rustc-1.95.0-x86_64-unknown-linux-gnu/default
```

Do not create lane-local target symlinks. Do not write global Cargo config.

## 8. Canonical Maintenance-Script Installation

```bash
sudo install -o root -g root -m 755 /srv/qbuild/work/NA-0543/qsl-protocol/scripts/local_ops/qbuild-ssd-maintenance.sh /usr/local/sbin/qbuild-ssd-maintenance
sha256sum /usr/local/sbin/qbuild-ssd-maintenance
stat -c '%a %U %G %s %y %n' /usr/local/sbin/qbuild-ssd-maintenance
```

## 9. Proposed Service/Timer Installation

```bash
sudo install -o root -g root -m 644 /srv/qbuild/tmp/NA0543_local_ops_ssd_shared_target_impl_20260626T032331Z/proposed_operator_files/qbuild-ssd-maintenance.service /etc/systemd/system/qbuild-ssd-maintenance.service
sudo install -o root -g root -m 644 /srv/qbuild/tmp/NA0543_local_ops_ssd_shared_target_impl_20260626T032331Z/proposed_operator_files/qbuild-ssd-maintenance.timer /etc/systemd/system/qbuild-ssd-maintenance.timer
sha256sum /etc/systemd/system/qbuild-ssd-maintenance.service /etc/systemd/system/qbuild-ssd-maintenance.timer
```

## 10. qwork/env/wrapper Update

```bash
install -m 775 /srv/qbuild/tmp/NA0543_local_ops_ssd_shared_target_impl_20260626T032331Z/proposed_operator_files/env_qbuild.sh /srv/qbuild/tools/env_qbuild.sh
install -m 775 /srv/qbuild/tmp/NA0543_local_ops_ssd_shared_target_impl_20260626T032331Z/proposed_operator_files/qwork.sh /srv/qbuild/tools/qwork.sh
install -m 755 /srv/qbuild/tmp/NA0543_local_ops_ssd_shared_target_impl_20260626T032331Z/proposed_operator_files/qwork /home/victor/.local/bin/qwork
sha256sum /srv/qbuild/tools/env_qbuild.sh /srv/qbuild/tools/qwork.sh /home/victor/.local/bin/qwork
```

The proposed qwork integration writes target-selection proof fields and a
file-backed shell environment under the lane `.qwork` directory. It does not
depend on child-process environment export.

## 11. daemon-reload and Timer Transition

```bash
sudo systemctl daemon-reload
sudo systemctl enable qbuild-ssd-maintenance.timer
sudo systemctl restart qbuild-ssd-maintenance.timer
systemctl is-enabled qbuild-ssd-maintenance.timer
systemctl is-active qbuild-ssd-maintenance.timer
systemctl list-timers qbuild-ssd-maintenance.timer --no-pager
```

## 12. Post-Install Dry-Run Only

```bash
sudo /usr/local/sbin/qbuild-ssd-maintenance --dry-run --target-days 7 --tmp-days 7 --log-retention-days 90
```

Do not run apply as part of proof capture unless Director separately authorizes
it after reviewing dry-run output.

## 13. qwork Operator Verification

Operator runs fresh qwork only after installation or intentional decline:

```bash
qwork NA-0544 qsl-protocol
```

Then collect the `.qwork` proof fields:

```bash
sed -n '1,220p' /srv/qbuild/work/NA-0544/.qwork/startup.qsl-protocol.kv
python3 -m json.tool /srv/qbuild/work/NA-0544/.qwork/startup.qsl-protocol.json
```

Verify the proof contains `cargo_target_mode`, `cargo_target_dir`,
`cargo_target_source`, `cargo_target_build_class`,
`cargo_target_toolchain_key`, `explicit_target_preserved`, and
`shared_target_ready`.

## 14. Proof Collection

Required proof outputs:

- pre/post hashes, owners, and modes;
- service/timer unit content;
- timer enabled/active/next-run state;
- service `Result` and `ExecMainStatus`;
- maintenance dry-run human and JSON summary;
- shared target owner/mode/path;
- qwork proof fields for target selection;
- explicit-target preservation proof;
- unrelated-repository negative proof;
- rollback inventory;
- disk state;
- no active build during install;
- no qsl-backup execution;
- no backup mutation outside rollback/archive/log paths.

## 15. Rollback

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

Rollback does not delete the shared target by default.

## 16. Stop Conditions

Stop for missing rollback capture, mount failure, root usage at or above 95%,
unexpected candidate paths, active or ambiguous build state, destination archive
collision, missing JSON summary, missing qwork target proof fields, explicit
target overwrite, unrelated repo target mutation, qsl-backup execution, backup
mutation outside rollback/archive/log paths, or any command not explicitly
reviewed here.
