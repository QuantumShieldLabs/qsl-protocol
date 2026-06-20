Status: Supporting
Owner: QSL Governance / Core Assurance
Last-Updated: 2026-06-20

# NA-0510 Remote Host Read-Only Capability Probe Implementation Harness

## Executive summary

NA-0510 consumes NA-0509 / D398 inheritance and executes the first
Codex-authorized remote command lane: exactly one bounded read-only SSH
capability probe against the approved `inspiron` / `qslcodex` remote test
account.

Probe result classification:

- `REMOTE_READ_ONLY_PROBE_PASS`

The probe verified local effective SSH config, remote account identity,
non-root/no-sudo posture, absence of privileged groups, existence and
writability of the approved workdir by read-only `test -w`, no remote backup
exposure, remote qwork absence, remote qsl-backup absence, no remote E2E, and no
remote file write. Raw proof remains under the D400 proof root; this document
records only redacted summaries and fixed markers.

Selected successor:

- `NA-0511 -- QSL Remote Host Read/Write Marker and Toolchain/Disk Capability Probe Scope Authorization Plan`

NA-0510 makes no public-readiness claim, no production-readiness claim, no
public-internet-readiness claim, no external-review-complete claim, no
crypto-complete claim, no replay-proof claim, no downgrade-proof claim, no
secret-material-complete claim, no side-channel-free claim, no vulnerability-free
claim, no bug-free claim, and no perfect-crypto claim.

## Live NA-0510 scope

Allowed mutation paths for this implementation PR:

- `docs/governance/evidence/NA-0510_qsl_remote_host_read_only_capability_probe_implementation_harness.md`
- `tests/NA-0510_qsl_remote_host_read_only_capability_probe_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Protected boundaries:

- one bounded read-only SSH invocation to `inspiron` as `qslcodex`.
- no second remote SSH invocation.
- no scp, sftp, or rsync to remote.
- no ssh-keygen or ssh-keyscan.
- no remote account creation.
- no SSH key generation or installation.
- no local/system SSH config mutation.
- no known_hosts or authorized_keys mutation.
- no remote host mutation.
- no sudo/admin action except negative `sudo -n true` check.
- no remote file creation, write, marker write, read/delete marker cycle, or
  deletion.
- no remote qsc send/receive, key generation, source checkout/build, package
  install, service action, qwork/qstart/qresume, or qsl-backup execution.
- no qsc source/test/fuzz/Cargo mutation.
- no workflow/script/helper/dependency mutation.
- no corpus/vector/input mutation.
- no formal/refimpl/service/public/backup mutation.
- exactly one READY item remains mandatory until closeout.

## qwork proof-file verification

Codex did not run qwork, qstart, or qresume.

Read and copied proof files:

- `/srv/qbuild/work/NA-0510/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0510/.qwork/startup.qsl-protocol.json`

Verified required fields:

- `startup_result=OK`
- `lane=NA-0510`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0510/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0510`
- `requested_lane_status=READY`

Freshness and startup proof:

- proof HEAD matched live pre-fetch HEAD at `24e49881dc8b`.
- proof origin/main matched live pre-fetch origin/main at `24e49881dc8b`.
- fetch occurred only after proof/live match and disk proof below the 95% stop
  threshold.
- `/` usage before fetch was 80%.
- `origin/main` equaled or descended from `24e49881dc8b`.
- READY_COUNT was 1.
- READY item was NA-0510.
- NA-0509, NA-0508, and NA-0507 were DONE.
- D-1007 existed once.
- D-1008 existed once.
- D-1009 was absent before this patch.
- duplicate decision record count was zero.

Startup main health:

- `public-safety` completed success.
- `qsc-adversarial-smoke` completed success.
- `qsc-linux-full-suite` and `macos-qsc-full-serial` completed skipped under
  the accepted docs/governance policy.
- no completed red checks were present in the retrieved check-run set.

Read-only backup boundary proof:

- installed qsl-backup helper matched the required SHA-256 digest; the full
  digest is captured in the D400 proof root.
- the configured Codex ops source appeared exactly once in the installed helper
  source list.
- Codex did not run backup, restore, or qsl-backup.

Marker:

- `NA0510_REMOTE_PROBE_SCOPE_CONSUMED_OK`

## NA-0509 / D398 inheritance

Consumed:

- D398 response:
  `/home/victor/work/qsl/codex/responses/NA0509_20260620T223434Z_D398.md`
- NA-0509 evidence:
  `docs/governance/evidence/NA-0509_qsl_remote_host_capability_probe_scope_authorization_plan.md`
- NA-0509 authorization testplan:
  `tests/NA-0509_qsl_remote_host_capability_probe_scope_authorization_testplan.md`
- D-1007 and D-1008 in `DECISIONS.md`
- NA-0510 READY block in `NEXT_ACTIONS.md`
- same-host client-to-client E2E test path as inherited local-only evidence:
  `qsl/qsl-client/qsc/tests/same_host_client_to_client_e2e.rs`
- read-only qsl-backup boundary evidence.

Inherited facts:

- NA-0509 completed.
- NA-0510 was restored READY.
- D398 selected `REMOTE_READ_ONLY_CAPABILITY_PROBE_IMPLEMENTATION_READY`.
- operator setup proof had been accepted by NA-0508.
- remote alias is `inspiron`.
- remote account is `qslcodex`.
- optional alias `remote` was not approved/configured.
- D398 authorized exactly one future bounded SSH read-only probe.
- D398 did not authorize marker write/read/delete.
- D398 did not authorize toolchain or disk probes beyond the listed command
  family.
- D398 did not authorize remote E2E.
- no remote action occurred in NA-0509.
- no SSH execution occurred in NA-0509.
- no account, key, SSH config, known_hosts, authorized_keys, or remote host
  mutation occurred in NA-0509.
- no public-readiness, production-readiness, crypto-complete, replay-proof,
  downgrade-proof, or secret-material-complete claim was made.

## Local pre-SSH proof

Local command:

- `ssh -G inspiron`

Raw output is saved under the D400 proof root. Checked-in evidence records only
safe parsed fields:

- `hostname=inspiron`
- `user=qslcodex`
- `identityfile` basename: `qslcodex_ed25519`
- `identitiesonly=yes`
- `passwordauthentication=no`
- `batchmode=yes`
- `stricthostkeychecking=true`
- `forwardagent=no`
- `forwardx11=no`
- `clearallforwardings=yes`

The local effective config proof contained no private key block markers and no
passphrase/token/password assignment pattern. Codex did not run `ssh -G remote`;
the optional `remote` alias remains unapproved by inherited evidence.

Markers:

- `NA0510_NO_SSH_KEY_GENERATION_OK`
- `NA0510_NO_SSH_CONFIG_MUTATION_OK`

## Exact remote command

NA-0510 ran exactly one remote SSH invocation:

```bash
ssh -o BatchMode=yes -o PasswordAuthentication=no -o ConnectTimeout=10 inspiron 'bash -s'
```

The remote script was passed on stdin and was limited to:

- `set -u`
- `hostname`
- `id -un`
- `id -u`
- `id -Gn`
- `pwd`
- `printf` of fixed markers and `$HOME`
- `test -d "$HOME/qsl-remote-test"`
- `test -w "$HOME/qsl-remote-test"`
- `sudo -n true` as a negative capability check only
- `test -e /backup/qsl`
- `test -r /backup/qsl`
- `command -v qwork`
- `command -v qsl-backup`

The remote script did not write files, create a marker file, delete files, run
qsc, run git/cargo/rustc/qsc, run uname, run df, run qwork/qstart/qresume, run
qsl-backup, run service/systemctl commands, run package managers, or run remote
E2E.

## Remote probe output summary

Remote probe exit:

- `0`

Redacted output summary:

```text
remote_host=<redacted-present>
remote_user=qslcodex
remote_uid=1003
remote_groups=<redacted; privileged-groups-absent>
remote_pwd=<redacted-present>
remote_home=<redacted-present>
NA0510_REMOTE_ACCOUNT_QSLCODEX_OK
NA0510_REMOTE_NOT_ROOT_OK
REMOTE_PRIVILEGED_GROUPS_ABSENT yes
NA0510_REMOTE_WORKDIR_EXISTS_OK
NA0510_REMOTE_WORKDIR_WRITABLE_OK
NA0510_REMOTE_NO_SUDO_OK
NA0510_REMOTE_NO_BACKUP_EXPOSURE_OK absent
NA0510_REMOTE_QWORK_ABSENT_OK
NA0510_REMOTE_QSL_BACKUP_ABSENT_OK
NA0510_NO_REMOTE_E2E_OK
NA0510_NO_REMOTE_FILE_WRITE_OK
REMOTE_PROBE_DONE yes
```

Raw stdout/stderr, the remote exit code, and the redacted output are saved under
the D400 proof root. The remote stderr line count was zero.

Marker:

- `NA0510_REMOTE_READ_ONLY_PROBE_EXECUTED_OK`

## Remote output redaction review

Reviewed raw remote output under the D400 proof root.

Accepted redaction findings:

- no OpenSSH private-key block marker.
- no RSA private-key block marker.
- no generic private-key block marker.
- no passphrase/token/password assignment pattern.
- no production endpoint marker.
- no qsl-backup run marker.
- no qwork run marker.
- no remote file write marker.
- no unrelated environment variables.
- no directory listing outside `$HOME/qsl-remote-test`.

The checked-in evidence redacts host, groups, `pwd`, and `$HOME` details while
preserving fixed success markers and account/UID proof.

## Probe result classification

Selected classification:

- `REMOTE_READ_ONLY_PROBE_PASS`

Rationale:

- SSH command exited 0.
- no STOP marker appeared.
- all required success markers appeared.
- remote stderr was empty.
- redaction review passed.

## Account identity proof

Remote identity proof:

- account marker: `NA0510_REMOTE_ACCOUNT_QSLCODEX_OK`
- remote user: `qslcodex`
- remote UID: `1003`

This is account-boundary evidence only. It is not protocol, wire, crypto, or
remote E2E evidence.

Marker:

- `NA0510_REMOTE_ACCOUNT_QSLCODEX_OK`

## Non-root / no-sudo proof

Remote privilege-boundary proof:

- `NA0510_REMOTE_NOT_ROOT_OK`
- `REMOTE_PRIVILEGED_GROUPS_ABSENT yes`
- `NA0510_REMOTE_NO_SUDO_OK`

The only sudo-related remote action was the authorized negative probe:

- `sudo -n true >/dev/null 2>&1`

It did not succeed.

Markers:

- `NA0510_REMOTE_NOT_ROOT_OK`
- `NA0510_REMOTE_NO_SUDO_OK`

## Workdir proof

Remote workdir proof:

- `NA0510_REMOTE_WORKDIR_EXISTS_OK`
- `NA0510_REMOTE_WORKDIR_WRITABLE_OK`

Writability was checked with read-only `test -w "$HOME/qsl-remote-test"`.
NA-0510 did not create, write, read back, or delete any marker file.

Markers:

- `NA0510_REMOTE_WORKDIR_EXISTS_OK`
- `NA0510_REMOTE_WORKDIR_WRITABLE_OK`

## Backup exposure proof

Remote backup exposure proof:

- `NA0510_REMOTE_NO_BACKUP_EXPOSURE_OK absent`

The probe did not read backup data and did not run qsl-backup. It checked only
the existence/readability boundary authorized by D398.

Marker:

- `NA0510_REMOTE_NO_BACKUP_EXPOSURE_OK`

## qwork / qsl-backup absence proof

Remote helper-boundary proof:

- `NA0510_REMOTE_QWORK_ABSENT_OK`
- `NA0510_REMOTE_QSL_BACKUP_ABSENT_OK`

The probe used `command -v` only. It did not run qwork, qstart, qresume, or
qsl-backup locally or remotely.

Markers:

- `NA0510_REMOTE_QWORK_ABSENT_OK`
- `NA0510_REMOTE_QSL_BACKUP_ABSENT_OK`

## No remote E2E proof

Remote E2E remained deferred:

- `NA0510_NO_REMOTE_E2E_OK`

NA-0510 did not run qsc remotely, did not run remote qsc send/receive, did not
check out or build source remotely, did not start services, and did not perform
remote protocol execution.

Marker:

- `NA0510_NO_REMOTE_E2E_OK`

## No remote file write proof

Remote file-write boundary proof:

- `NA0510_NO_REMOTE_FILE_WRITE_OK`

The remote script used read-only identity/configuration checks and shell fixed
markers only. It did not create, mutate, delete, upload, or synchronize remote
files.

Marker:

- `NA0510_NO_REMOTE_FILE_WRITE_OK`

## Hostile Cryptographer Review

The read-only probe adds bounded remote account assurance by proving that the
currently configured `inspiron` alias reaches `qslcodex`, that the account is
non-root, lacks sudo and privileged groups, has the expected workdir boundary,
has no visible qwork/qsl-backup helpers, and does not expose `/backup/qsl`.

It must not be overread as protocol evidence. The probe does not prove qsc
handshake behavior, cross-host send/receive, remote source correctness,
wire-level behavior, key schedule behavior, downgrade resistance, replay
resistance, or secret-material completeness.

Before remote E2E, the program still needs a time-sensitive lane to authorize
and then execute marker write/read/delete proof and toolchain/disk capability
capture. Lack of marker, toolchain, and disk proof means remote E2E remains
premature.

The proof preserves claim boundaries: no public-readiness claim, no
production-readiness claim, no public-internet-readiness claim, no
external-review-complete claim, no crypto-complete claim, no replay-proof claim,
no downgrade-proof claim, no secret-material-complete claim, no side-channel-free
claim, no vulnerability-free claim, no bug-free claim, and no perfect-crypto
claim.

## Red-Team Review

If `inspiron` is retargeted after this probe, this evidence becomes historical
only. A later remote lane must recheck effective local SSH config and account
identity before any write, toolchain, staging, or E2E action.

If `qslcodex` later gains sudo or privileged groups, the privilege boundary is
invalidated. Future remote lanes must repeat the negative sudo and group checks.

If `/backup/qsl` appears later or becomes readable, future remote work must stop
before any protocol or staging action.

If qwork or qsl-backup appears later, future remote work must stop unless a
new directive explicitly authorizes and justifies that changed boundary.

If workdir writability changes, a marker lane must fail closed rather than
creating alternate paths.

Remote output can leak topology through host, group, `pwd`, `$HOME`, IP, or
identity path details. Checked-in evidence must keep those fields redacted or
summarized.

The next time-sensitive remote lane should probe only the missing capability
boundary: marker write/read/delete under `$HOME/qsl-remote-test`, selected
toolchain presence, and disk capacity, with no remote E2E.

## Production SRE Review

The probe confirms enough to proceed toward a remote read/write marker and
toolchain/disk authorization lane. It does not confirm enough to proceed
directly to remote E2E.

Next checks while the host is available:

- authorize exact marker create/read/delete commands under `$HOME/qsl-remote-test`.
- authorize exact toolchain probes if needed for staging, such as qsc/cargo/rustc
  presence, only in a separate scope lane.
- authorize exact disk/capacity probe commands if needed, only in a separate
  scope lane.
- preserve no package install, no remote checkout/build, no service action, and
  no sudo/admin action.

Remote output should remain redacted by default, with fixed markers preferred
over raw environment, path, topology, or listing output.

Future remote failures should stay isolated to directive proof roots and must
not mutate qbuild/qwork state, qsl-backup state, SSH config, known_hosts, or the
remote host outside the authorized command family.

Remote E2E remains deferred because NA-0510 did not prove marker write safety,
toolchain availability, disk suitability, remote staging model, source checkout
boundary, or qsc execution boundary.

## Release-Claim Boundary Review

NA-0510 preserves:

- no public-ready claim.
- no production-ready claim.
- no public-internet-ready claim.
- no external-review-complete claim.
- no crypto-complete claim.
- no replay-proof claim.
- no downgrade-proof claim.
- no secret-material-complete claim.
- no side-channel-free claim.
- no vulnerability-free, bug-free, or perfect-crypto claim.

## Successor selection

Selected successor:

`NA-0511 -- QSL Remote Host Read/Write Marker and Toolchain/Disk Capability Probe Scope Authorization Plan`

Reason:

- NA-0510 passed the read-only account and boundary probe.
- host availability is time-sensitive.
- marker write/read/delete remains unproven and was explicitly deferred by
  NA-0509/D398.
- toolchain and disk capability remain unproven because D398 did not authorize
  those commands in NA-0510.
- remote E2E remains too broad until marker and staging prerequisites are
  authorized and evidenced.

If future evidence contradicts this pass result, the successor should become an
exact remediation lane instead of a marker/toolchain/disk authorization lane.

## Future scope bundle

Proposed NA-0511 scope:

### NA-0511 -- QSL Remote Host Read/Write Marker and Toolchain/Disk Capability Probe Scope Authorization Plan
Status: READY
Goals: G1, G2, G3, G4, G5

Objective:
Authorize the next time-sensitive remote capability lane after the NA-0510
read-only probe, selecting exact bounded commands for a short-lived marker-file
write/read/delete under `$HOME/qsl-remote-test` plus remote toolchain and disk
capability capture, without remote E2E, source checkout/build, package
installation, sudo/admin action, qwork/qsl-backup execution, or public/production
readiness claims.

Allowed scope:

- governance evidence/testplan paths for NA-0511.
- `DECISIONS.md`.
- `TRACEABILITY.md`.
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.
- read-only review of NA-0510 probe output and prior remote setup evidence.

Forbidden scope:

- running SSH in the authorization lane.
- remote E2E.
- qsc protocol execution remotely.
- source checkout/build remotely.
- package installation.
- sudo/admin action.
- key generation or installation.
- SSH config mutation.
- known_hosts mutation.
- remote host mutation.
- qwork/qstart/qresume mutation.
- qsl-backup execution.
- qsc source/test/fuzz/Cargo mutation.
- workflow/dependency mutation.
- corpus/vector/input mutation.
- formal/refimpl/service/public/backup mutation.
- public-readiness or production-readiness claims.

Deliverables:

- read/write marker and toolchain/disk capability scope authorization evidence.
- testplan.
- decision.
- TRACEABILITY update.
- rolling journal update.
- exact future command list or remediation/no-action rationale.

Acceptance criteria:

- NA-0510 read-only probe consumed.
- marker write/read/delete scope selected or rejected.
- toolchain/disk scope selected or rejected.
- no remote command run in this authorization lane.
- no public/production readiness claim.
- exactly one READY item remains.

## Future validation / marker plan

NA-0510 markers:

- `NA0510_REMOTE_PROBE_SCOPE_CONSUMED_OK`
- `NA0510_REMOTE_READ_ONLY_PROBE_EXECUTED_OK`
- `NA0510_REMOTE_ACCOUNT_QSLCODEX_OK`
- `NA0510_REMOTE_NOT_ROOT_OK`
- `NA0510_REMOTE_NO_SUDO_OK`
- `NA0510_REMOTE_WORKDIR_EXISTS_OK`
- `NA0510_REMOTE_WORKDIR_WRITABLE_OK`
- `NA0510_REMOTE_NO_BACKUP_EXPOSURE_OK`
- `NA0510_REMOTE_QWORK_ABSENT_OK`
- `NA0510_REMOTE_QSL_BACKUP_ABSENT_OK`
- `NA0510_NO_REMOTE_E2E_OK`
- `NA0510_NO_REMOTE_FILE_WRITE_OK`
- `NA0510_NO_SSH_KEY_GENERATION_OK`
- `NA0510_NO_SSH_CONFIG_MUTATION_OK`
- `NA0510_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0510_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0510_ONE_READY_INVARIANT_OK`

Future NA-0511 markers if selected by closeout:

- `NA0511_REMOTE_READ_ONLY_PROBE_CONSUMED_OK`
- `NA0511_REMOTE_MARKER_PROBE_SCOPE_SELECTED_OK`
- `NA0511_REMOTE_TOOLCHAIN_DISK_SCOPE_SELECTED_OK`
- `NA0511_NO_REMOTE_ACTION_IN_AUTHORIZATION_OK`
- `NA0511_NO_REMOTE_E2E_SCOPE_OK`
- `NA0511_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0511_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0511_ONE_READY_INVARIANT_OK`

## Public claim / website / external review boundary

NA-0510 changes no public docs, website, README, START_HERE, release note, public
technical paper, or external review material.

The evidence is operational capability proof only. It is not public-readiness,
production-readiness, public-internet-readiness, external-review-complete,
crypto-complete, replay-proof, downgrade-proof, secret-material-complete,
side-channel-free, vulnerability-free, bug-free, or perfect-crypto proof.

Markers:

- `NA0510_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0510_NO_PRODUCTION_READINESS_CLAIM_OK`

## Backup-impact statement

Backup impact:

- no backup, restore, or qsl-backup command was run.
- no `/backup/qsl` local or remote mutation occurred.
- no qsl-backup helper, status, plan, timer, service, fstab, manifest, log, or
  backup source-list mutation occurred.
- remote `/backup/qsl` was checked only for exposure using read-only `test`
  commands and was reported absent.

## Rejected alternatives

Remote read/write marker implementation in NA-0510 was rejected because D398
explicitly did not authorize marker write/read/delete.

Remote toolchain/disk probing in NA-0510 was rejected because D398 did not
authorize `command -v git/cargo/rustc/qsc`, `uname`, `df`, or related probes.

Remote qsc staging/build/smoke was rejected because marker and capability
evidence remain missing.

Remote E2E implementation was rejected because it would mix account/setup
assurance with protocol behavior before the staging prerequisites are proven.

## Next recommendation

After NA-0510 implementation merges and closeout is authorized, restore:

`NA-0511 -- QSL Remote Host Read/Write Marker and Toolchain/Disk Capability Probe Scope Authorization Plan`

NA-0511 should be authorization-only and should not run SSH. It should select the
exact future marker write/read/delete, toolchain, and disk commands for a later
implementation lane or record a remediation/no-action rationale if the current
remote evidence becomes stale or unsafe.
