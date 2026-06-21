Status: Supporting
Owner: QSL Governance / Core Assurance
Last-Updated: 2026-06-21

# NA-0512 Remote Host Read/Write Marker and Toolchain/Disk Capability Probe Implementation Harness

## Executive summary

NA-0512 consumed NA-0511 / D403 inheritance, verified fresh qwork proof files
without rerunning qwork, reviewed local SSH effective configuration, and executed
exactly one bounded SSH capability probe to the authorized remote alias.

Probe result classification:

- `REMOTE_MARKER_PROBE_PASS_TOOLCHAIN_ABSENT`

Selected successor:

- `NA-0513 -- QSL Remote qsc Staging Strategy Authorization Plan`

The remote marker lifecycle passed: one synthetic marker was written under the
existing `$HOME/qsl-remote-test` workdir, read back exactly, deleted, and
verified absent after deletion. Remote toolchain capture found `git`, `cargo`,
`rustc`, and `qsc` absent. Remote disk capture succeeded. The result proves
bounded filesystem capability and capacity visibility only; it does not prove
remote qsc build correctness, remote protocol correctness, crypto completeness,
or public/production readiness.

Claim boundary:

- no public-readiness claim.
- no production-readiness claim.
- no public-internet-readiness claim.
- no external-review-complete claim.
- no crypto-complete claim.
- no replay-proof claim.
- no downgrade-proof claim.
- no secret-material-complete claim.
- no side-channel-free claim.
- no vulnerability-free claim.
- no bug-free claim.
- no perfect-crypto claim.

## Live NA-0512 scope

Allowed mutation paths for this implementation PR:

- `docs/governance/evidence/NA-0512_qsl_remote_host_read_write_marker_toolchain_disk_capability_probe_implementation_harness.md`
- `tests/NA-0512_qsl_remote_host_read_write_marker_toolchain_disk_capability_probe_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Protected boundaries preserved:

- exactly one bounded SSH invocation was executed.
- no second SSH invocation was executed.
- no remote E2E was run.
- no qsc send/receive or qsc protocol command was run remotely.
- no remote source checkout/build was run.
- no package installation was run.
- no sudo/admin action was run except the authorized negative `sudo -n true`
  check.
- no remote file write occurred outside the single synthetic marker lifecycle.
- no marker artifact remained after deletion.
- no scp, sftp, or rsync was run.
- no ssh-keygen or ssh-keyscan was run.
- no local/system SSH config, known_hosts, or authorized_keys mutation occurred.
- no remote account, key, service, or host setup mutation occurred.
- no qwork, qstart, qresume, qsl-backup, backup, or restore was run.
- no qsc source/test/fuzz/Cargo, workflow/script/helper, dependency/lockfile,
  corpus/vector/input, formal/refimpl/service/public/backup path was mutated.
- exactly one READY item remains mandatory until a separate closeout.

## qwork proof-file verification

Codex did not run qwork, qstart, or qresume.

Read and copied proof files:

- `/srv/qbuild/work/NA-0512/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0512/.qwork/startup.qsl-protocol.json`

Verified required fields:

- `startup_result=OK`
- `lane=NA-0512`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0512/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0512`
- `requested_lane_status=READY`

Freshness proof:

- proof `HEAD` matched live pre-fetch `HEAD` at `6b18574cbf16`.
- proof `origin_main` matched live pre-fetch `origin/main` at
  `6b18574cbf16`.
- fetch occurred only after proof/live ref match and disk proof below the 95%
  stop threshold.

Disk proof:

- `/` usage was 81%.
- `/backup/qsl` usage was 25%.
- STOP threshold 95% was not hit.

Queue proof:

- READY_COUNT 1.
- READY item: `NA-0512 -- QSL Remote Host Read/Write Marker and Toolchain/Disk Capability Probe Implementation Harness`.
- NA-0511 DONE.
- NA-0510 DONE.
- NA-0509 DONE.
- D-1011 exists once.
- D-1012 exists once.
- D-1013 was absent before this patch.
- duplicate decision count 0.

Current main health before patch:

- `public-safety` completed success on `6b18574cbf16`.
- `qsc-adversarial-smoke` completed success on `6b18574cbf16`.
- `qsc-linux-full-suite` completed skipped under accepted docs/governance
  policy.
- `macos-qsc-full-serial` completed skipped under accepted docs/governance
  policy.
- no completed red checks were observed.

qsl-backup boundary:

- installed helper digest matched the required helper digest, recorded in proof
  as short `e9ecff3d22ed`.
- the configured Codex ops source inclusion count was exactly 1.
- Codex did not run backup, restore, or qsl-backup.

Recovered proof-tooling notes:

- The first queue/decision parser matched a narrower decision-ID shape than the
  repository uses. This was classified as a recoverable command-shape parser
  mistake, corrected to match `- **ID:** D-...`, and rerun to PASS.
- The first redaction scan matched the allowed qsl-backup absence marker as if
  it were an execution marker. This was classified as a recoverable scan-pattern
  false positive, narrowed to execution evidence only, and rerun to PASS.

## NA-0511 / D403 inheritance

NA-0511 completed and NA-0512 was restored READY by D403 closeout.

Inherited facts:

- D402 selected `REMOTE_MARKER_TOOLCHAIN_DISK_PROBE_IMPLEMENTATION_READY`.
- D403 recovered PR #1294 missing required checks via one close/reopen cycle and
  no empty commit.
- PR #1294 merged at `5f27d289e088`.
- D403 closeout PR #1295 merged at `6b18574cbf16`.
- NA-0512 is authorized to run exactly one bounded SSH command.
- NA-0512 is authorized for one short-lived marker write/read/delete under
  `$HOME/qsl-remote-test`.
- NA-0512 is authorized for toolchain/disk capture.
- NA-0512 is not authorized for remote E2E.
- NA-0512 is not authorized for source checkout/build.
- NA-0512 is not authorized for package install.
- no remote action occurred in NA-0511 or D403.
- no public-readiness claim was made.
- no production-readiness claim was made.
- no crypto-complete claim was made.
- no replay-proof claim was made.
- no downgrade-proof claim was made.
- no secret-material-complete claim was made.

## Local pre-SSH proof

Local `ssh -G inspiron` was run before the remote probe. Codex did not mutate SSH
configuration and did not run `ssh -G remote`.

Parsed safe fields:

- `hostname=inspiron`
- `user=qslcodex`
- identity basename: `qslcodex_ed25519`
- `identitiesonly=yes`
- `passwordauthentication=no`
- `batchmode=yes`
- strict host-key checking enabled.
- agent forwarding disabled.
- X11 forwarding disabled.
- clear all forwardings enabled.

The raw local config proof remained under the proof root. Checked-in evidence
does not include full private identity paths, private key material,
passphrases, tokens, credentials, known_hosts contents, or authorized_keys
contents.

## Exact remote command

Exactly one remote SSH invocation was executed:

```bash
ssh -o BatchMode=yes -o PasswordAuthentication=no -o ConnectTimeout=10 inspiron "PROOF_ID=<SAFE_PROOF_ID> bash -s" < "$PROOF_DIR/remote_probe/na0512_remote_probe.sh"
```

The remote script was limited to:

- fixed marker prints.
- identity and boundary rechecks.
- negative sudo check.
- `/backup/qsl` existence/readability checks.
- qwork and qsl-backup absence checks.
- toolchain presence/version checks for `git`, `cargo`, `rustc`, and qsc
  presence only.
- `uname` and `df` capacity checks.
- exactly one marker write/read/delete/absent-after-delete lifecycle under
  `$HOME/qsl-remote-test`.

No qsc help command was run because qsc was absent.

## Remote probe output summary

Remote probe outcome:

- SSH exit code: 0.
- stdout lines: 43.
- stderr lines: 0.
- no `STOP_` marker appeared.
- final marker `NA0512_REMOTE_PROBE_DONE_OK` appeared.

Sanitized remote identity summary:

- remote account was `qslcodex`.
- remote UID was non-zero.
- remote groups did not include privileged groups.
- remote host label was consistent with the authorized host; full host details
  are not expanded in this evidence.
- remote `$HOME` and `pwd` are summarized as the account home.

Sanitized disk summary:

- `$HOME` and `$HOME/qsl-remote-test` disk checks succeeded.
- remote device names are redacted from checked-in evidence.
- reported remote usage was low, about 2% on the relevant filesystem.

## Marker write/read/delete proof

The remote script used a safe alphanumeric proof identifier and marker path
pattern:

- `$HOME/qsl-remote-test/na0512_marker_<SAFE_PROOF_ID>.txt`

Marker lifecycle:

- marker did not already exist before the write.
- marker content written exactly: `QSL_REMOTE_MARKER_SYNTHETIC_NA0512`.
- marker file existence was verified.
- marker content was read back exactly.
- marker was deleted with `rm -f -- "$MARKER"`.
- marker absence after deletion was verified.

Required markers observed:

- `NA0512_REMOTE_MARKER_WRITE_OK`
- `NA0512_REMOTE_MARKER_READ_OK`
- `NA0512_REMOTE_MARKER_DELETE_OK`
- `NA0512_REMOTE_MARKER_ABSENT_AFTER_DELETE_OK`

## Toolchain/disk capability proof

Toolchain capture result:

- `git`: absent.
- `cargo`: absent.
- `rustc`: absent.
- `qsc`: absent.

Disk capture result:

- `df -h "$HOME"` completed.
- `df -h "$HOME/qsl-remote-test"` completed.
- `NA0512_REMOTE_DISK_STATUS_CAPTURED_OK` appeared.

This proves only the remote capability snapshot at probe time. It does not prove
build correctness, qsc installation correctness, or future staging feasibility.

## Remote output redaction review

Raw remote stdout/stderr were kept under the proof root and reviewed before this
evidence was written.

Redaction scan result:

- private key block count: 0.
- credential assignment pattern count: 0.
- production endpoint marker count: 0.
- backup material count: 0.
- qsl-backup execution marker count: 0.
- qwork run marker count: 0.
- source checkout/build marker count: 0.
- package install marker count: 0.

Checked-in evidence summarizes host, home, and disk topology. It does not include
private key material, passphrases, tokens, credential values, production
endpoints, backup material, full known_hosts content, authorized_keys content,
unrelated environment variables, or unrelated directory listings.

## Probe result classification

Selected classification:

- `REMOTE_MARKER_PROBE_PASS_TOOLCHAIN_ABSENT`

Rationale:

- boundary checks passed.
- marker write/read/delete/absent-after-delete passed.
- toolchain and disk status capture passed.
- qsc was absent.
- git/cargo/rustc were absent.
- no STOP marker appeared.
- no remote E2E, remote source checkout/build, or package installation occurred.

## Account identity proof

The remote probe emitted:

- `NA0512_REMOTE_ACCOUNT_QSLCODEX_OK`
- `NA0512_REMOTE_NOT_ROOT_OK`
- `NA0512_REMOTE_NO_PRIVILEGED_GROUP_OK`

The account was the authorized `qslcodex` account. The UID was non-zero and the
group set did not include privileged groups.

## Non-root / no-sudo proof

The only sudo action was the authorized negative probe:

- `sudo -n true`

The command did not succeed and the probe emitted:

- `NA0512_REMOTE_NO_SUDO_OK`

No admin action, sudo shell, privilege escalation, package installation, service
action, or remote setup mutation was attempted.

## Workdir proof

The existing remote workdir was checked without creating directories:

- `test -d "$HOME/qsl-remote-test"` passed.
- `test -w "$HOME/qsl-remote-test"` passed.

The probe emitted:

- `NA0512_REMOTE_WORKDIR_EXISTS_OK`
- `NA0512_REMOTE_WORKDIR_WRITABLE_OK`

## Backup exposure proof

The remote probe checked `/backup/qsl` existence and readability. It reported no
readable backup exposure and emitted:

- `NA0512_REMOTE_NO_BACKUP_EXPOSURE_OK`

Codex did not read backup contents, did not list backup directories, did not run
qsl-backup, and did not mutate `/backup/qsl`.

## qwork / qsl-backup absence proof

Remote command presence checks emitted:

- `NA0512_REMOTE_QWORK_ABSENT_OK`
- `NA0512_REMOTE_QSL_BACKUP_ABSENT_OK`

Codex did not run qwork, qstart, qresume, qsl-backup, backup, or restore locally
or remotely.

## No remote E2E proof

The probe emitted:

- `NA0512_NO_REMOTE_E2E_OK`

No qsc protocol send/receive, remote client-to-client flow, service action,
daemon action, key generation, or protocol traffic was run remotely.

## No source checkout/build proof

The probe emitted:

- `NA0512_NO_REMOTE_SOURCE_BUILD_OK`

No remote git clone/fetch/pull, cargo build/test/update, rustup install/update,
source checkout, source build, or dependency update was run remotely.

## No package install proof

The probe emitted:

- `NA0512_NO_PACKAGE_INSTALL_OK`

No package manager command and no sudo/admin install action was run.

## Best-Known-Method Review

The best-known safe next step after a read-only account probe was a bounded
marker lifecycle plus toolchain/disk capability snapshot. That collected the
largest safe signal available under the time-sensitive remote window without
jumping to qsc staging, qsc protocol execution, or remote E2E.

The method remains intentionally conservative:

- one SSH invocation only.
- one marker file only.
- marker cleanup verified before success.
- raw output retained under proof root.
- checked-in evidence sanitized.
- remote E2E deferred.
- qsc staging strategy moved to an authorization lane because toolchain and qsc
  are absent.

## Hostile Cryptographer Review

Marker write/read/delete proves only filesystem capability for one synthetic
file under the approved workdir. It proves no protocol correctness, no wire
compatibility, no key schedule property, no replay resistance, and no downgrade
resistance.

Toolchain/disk capture proves only the observed capability snapshot at probe
time. It does not prove that a future toolchain is trustworthy, that builds will
reproduce, that qsc is installed, or that any remote binary is correct.

A malicious remote host could fake:

- account identity and group output.
- toolchain presence/absence.
- disk capacity.
- marker readback.
- command output and hostname.
- future qsc binary behavior if installed later.

Remaining before remote E2E:

- approved staging strategy.
- approved source/binary provenance plan.
- remote qsc smoke evidence.
- command-output redaction rules for future qsc output.
- explicit remote E2E authorization.
- failure isolation from local qbuild/qwork state.

This lane preserves:

- no crypto-complete claim.
- no replay-proof claim.
- no downgrade-proof claim.
- no public-readiness claim.
- no production-readiness claim.

## Red-Team Review

If marker cleanup fails, the lane must classify as marker deletion/residue
failure and stop with remediation rather than proceed to staging or E2E.

If `qslcodex` gains sudo later, future probes must stop before running staging,
because the account boundary would no longer match the non-admin test role.

If `/backup/qsl` appears later and is readable, future probes must stop before
remote testing because backup exposure would violate the isolation boundary.

If qwork or qsl-backup appears later, future probes must stop because remote
operational tooling exposure changes the host boundary.

If qsc appears later, it must be treated as untrusted/stale until a separate
authorization lane checks provenance, allowed commands, and smoke-test scope.

If cargo/rustc appear later, the toolchain state remains untrusted until a
future lane authorizes exact build/staging commands and output handling.

If output leaks hostnames, IPs, or disk topology, checked-in evidence must remain
redacted and the raw proof must stay under the proof root.

Before remote E2E, the project should check staging strategy, qsc provenance or
build plan, smoke-test scope, marker cleanup, redaction, and one-READY queue
state.

## Production SRE Review

Marker/toolchain/disk evidence is enough to proceed only to a remote qsc staging
strategy authorization lane. It is not enough to proceed directly to remote E2E
because no qsc binary or toolchain exists on the remote host.

Marker artifacts should be cleaned by deleting the exact marker path and then
checking that path is absent. Future lanes should preserve the same
absent-after-delete proof before any additional remote work.

Future remote command output should keep raw logs in a proof root, redact
hostnames/IPs/home paths/disk devices in checked-in evidence, and scan for
private key blocks, credential assignments, production endpoint markers, backup
material, qwork/qsl-backup execution, source checkout/build, and package install
markers.

Remote failures remain isolated because qwork is not run remotely, qsl-backup is
not run remotely, remote state is not used to mutate queue files directly, and
future lanes must keep bounded proof roots and explicit stop classifications.

## Release-Claim Boundary Review

This evidence preserves:

- no public-readiness claim.
- no production-readiness claim.
- no public-internet-readiness claim.
- no external-review-complete claim.
- no crypto-complete claim.
- no replay-proof claim.
- no downgrade-proof claim.
- no secret-material-complete claim.
- no side-channel-free claim.
- no vulnerability-free claim.
- no bug-free claim.
- no perfect-crypto claim.

## Side-Channel Caveat

This remote marker/toolchain/disk probe does not evaluate timing behavior,
memory-access patterns, cache behavior, process isolation, command-output
side channels, or remote host observability. It makes no side-channel-free claim.

## Formal-Model Mapping Residual

No formal model was changed and no new protocol transition was modeled. The
probe is operational capability evidence only. Mapping between remote qsc
behavior and the formal model remains unproven until a later authorized staging
or E2E lane provides protocol evidence.

## External-Review Readiness

The evidence is useful for internal planning of remote staging strategy. It is
not an external-review package and makes no external-review-complete claim.

## Assurance Gap Review Trigger

Because qsc and the Rust toolchain are absent remotely, NA-0513 must compare
safe staging strategies before any future smoke or E2E lane. If a future remote
host state changes materially, identity, privilege, backup exposure, qwork,
qsl-backup, workdir, disk, and marker cleanup must be rechecked.

## Successor selection

Observed state:

- marker lifecycle passed.
- qsc absent.
- git absent.
- cargo absent.
- rustc absent.
- disk capture passed.

Selected successor:

- `NA-0513 -- QSL Remote qsc Staging Strategy Authorization Plan`

Reason:

- the marker passed, but there is no installed qsc and no remote toolchain path
  to justify an installed-binary smoke or remote build/smoke successor.
- the fastest safe next lane is authorization-only staging strategy selection.

## Future scope bundle

### NA-0513 -- QSL Remote qsc Staging Strategy Authorization Plan

Status: READY
Goals: G1, G2, G3, G4, G5

Objective:

Authorize the safest remote qsc staging strategy after NA-0512 proves marker
cleanup but finds no sufficient remote qsc/toolchain path, comparing prebuilt
binary transfer, operator-managed toolchain setup, and deferral/remediation,
without running remote commands in the authorization lane.

Allowed scope:

- governance evidence/testplan paths for NA-0513.
- `DECISIONS.md`.
- `TRACEABILITY.md`.
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.
- read-only inspection of NA-0512 probe output and prior remote setup/probe
  evidence.

Forbidden scope:

- running SSH in the authorization lane.
- remote E2E.
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
- no public-readiness claim.
- no production-readiness claim.

Deliverables:

- remote staging strategy authorization evidence.
- testplan.
- decision.
- TRACEABILITY update.
- rolling journal update.
- exact future staging/smoke command list, prebuilt-binary strategy, or
  remediation/no-action rationale.

Acceptance criteria:

- NA-0512 probe consumed.
- staging options reviewed.
- exact future staging/smoke scope selected or deferred with rationale.
- no remote command run in this authorization lane.
- no public/production readiness claim.
- exactly one READY item remains.

## Future validation / marker plan

NA-0512 markers observed or preserved:

- `NA0512_REMOTE_SCOPE_CONSUMED_OK`
- `NA0512_REMOTE_MARKER_TOOLCHAIN_DISK_PROBE_EXECUTED_OK`
- `NA0512_REMOTE_ACCOUNT_QSLCODEX_OK`
- `NA0512_REMOTE_NOT_ROOT_OK`
- `NA0512_REMOTE_NO_SUDO_OK`
- `NA0512_REMOTE_WORKDIR_EXISTS_OK`
- `NA0512_REMOTE_WORKDIR_WRITABLE_OK`
- `NA0512_REMOTE_MARKER_WRITE_OK`
- `NA0512_REMOTE_MARKER_READ_OK`
- `NA0512_REMOTE_MARKER_DELETE_OK`
- `NA0512_REMOTE_MARKER_ABSENT_AFTER_DELETE_OK`
- `NA0512_REMOTE_NO_BACKUP_EXPOSURE_OK`
- `NA0512_REMOTE_QWORK_ABSENT_OK`
- `NA0512_REMOTE_QSL_BACKUP_ABSENT_OK`
- `NA0512_REMOTE_TOOLCHAIN_STATUS_CAPTURED_OK`
- `NA0512_REMOTE_DISK_STATUS_CAPTURED_OK`
- `NA0512_NO_REMOTE_E2E_OK`
- `NA0512_NO_REMOTE_SOURCE_BUILD_OK`
- `NA0512_NO_PACKAGE_INSTALL_OK`
- `NA0512_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0512_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0512_ONE_READY_INVARIANT_OK`

Recommended NA-0513 markers:

- `NA0513_NA0512_PROBE_CONSUMED_OK`
- `NA0513_TOOLCHAIN_ABSENCE_CONSUMED_OK`
- `NA0513_STAGING_STRATEGY_OPTIONS_REVIEWED_OK`
- `NA0513_EXACT_FUTURE_SCOPE_SELECTED_OK`
- `NA0513_NO_REMOTE_COMMAND_OK`
- `NA0513_NO_REMOTE_E2E_OK`
- `NA0513_NO_PACKAGE_INSTALL_OK`
- `NA0513_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0513_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0513_ONE_READY_INVARIANT_OK`

## Public claim / website / external review boundary

NA-0512 does not modify public docs, website content, README, START_HERE, public
technical paper material, service documentation, or external review packaging.

This lane makes:

- no public-readiness claim.
- no production-readiness claim.
- no public-internet-readiness claim.
- no external-review-complete claim.

## Backup-impact statement

Backup impact: none.

Codex performed a read-only installed-helper digest/source-count check locally
and a remote read-only `/backup/qsl` exposure check. Codex did not run backup,
restore, qsl-backup, or mutate backup paths.

## Rejected alternatives

- Direct remote E2E was rejected because this lane proves only marker
  lifecycle, boundary rechecks, and capability snapshot.
- Remote qsc installed-binary smoke was rejected because qsc is absent.
- Remote source build/smoke was rejected because git/cargo/rustc are absent and
  source checkout/build was forbidden.
- Package installation was rejected because it is forbidden and would alter the
  remote host boundary.
- A second SSH verification was rejected because the directive permits exactly
  one remote SSH invocation.

## Next recommendation

Open and merge the NA-0512 implementation PR after required validation and CI
pass. If post-merge public-safety attaches and completes green inside the short
attach/early-failure window, close out NA-0512 and restore the selected NA-0513
staging-strategy authorization lane.
