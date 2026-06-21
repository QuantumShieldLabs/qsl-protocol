Status: Supporting
Owner: QSL Governance / Core Assurance
Last-Updated: 2026-06-21

# NA-0511 Remote Host Read/Write Marker and Toolchain/Disk Capability Probe Scope Authorization Plan

## Executive summary

NA-0511 consumes NA-0510 / D401 inheritance and authorizes the next
time-sensitive remote assurance lane, without running any remote command in this
authorization lane.

Primary classification:

- `REMOTE_MARKER_TOOLCHAIN_DISK_PROBE_IMPLEMENTATION_READY`

Selected successor:

- `NA-0512 -- QSL Remote Host Read/Write Marker and Toolchain/Disk Capability Probe Implementation Harness`

The selected NA-0512 successor may execute one bounded SSH capability probe only
after fresh qwork proof and an explicit NA-0512 directive authorize it. That
future probe must recheck identity and safety boundaries, create/read/delete
exactly one synthetic marker file under the existing `$HOME/qsl-remote-test`
workdir, capture toolchain/disk status, leave no marker artifact behind, and
preserve the no remote E2E boundary.

NA-0511 makes no public-readiness claim, no production-readiness claim, no
public-internet-readiness claim, no external-review-complete claim, no
crypto-complete claim, no replay-proof claim, no downgrade-proof claim, no
secret-material-complete claim, no side-channel-free claim, no vulnerability-free
claim, no bug-free claim, and no perfect-crypto claim.

## Live NA-0511 scope

Allowed mutation paths for this authorization PR:

- `docs/governance/evidence/NA-0511_qsl_remote_host_read_write_marker_toolchain_disk_capability_probe_scope_authorization_plan.md`
- `tests/NA-0511_qsl_remote_host_read_write_marker_toolchain_disk_capability_probe_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Protected boundaries:

- no remote action in NA-0511.
- no SSH execution in NA-0511.
- no scp, sftp, or rsync to remote.
- no ssh-keygen or ssh-keyscan.
- no remote account creation.
- no SSH key generation or installation.
- no local/system SSH config mutation.
- no known_hosts or authorized_keys mutation.
- no remote host mutation.
- no sudo/admin action.
- no package installation.
- no marker write/read/delete in NA-0511.
- no remote toolchain command in NA-0511.
- no remote E2E.
- no qsc send/receive or remote qsc protocol command.
- no remote source checkout/build.
- no qwork/qstart/qresume mutation or remote execution.
- no qsl-backup execution or mutation.
- no qsc source/test/fuzz/Cargo mutation.
- no workflow/script/helper/dependency mutation.
- no corpus/vector/input mutation.
- no formal/refimpl/service/public/backup mutation.
- exactly one READY item remains mandatory.

## qwork proof-file verification

Codex did not run qwork, qstart, or qresume.

Read and copied proof files:

- `/srv/qbuild/work/NA-0511/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0511/.qwork/startup.qsl-protocol.json`

Verified required fields:

- `startup_result=OK`
- `lane=NA-0511`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0511/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0511`
- `requested_lane_status=READY`

Freshness proof:

- proof `HEAD` matched live pre-fetch `HEAD` at `451d596e8f94`.
- proof `origin_main` matched live pre-fetch `origin/main` at `451d596e8f94`.
- fetch occurred only after proof/live ref match and disk proof below the 95%
  stop threshold.

Disk proof:

- `/` usage was 81%.
- `/backup/qsl` usage was 25%.
- STOP threshold 95% was not hit.

Queue proof:

- READY_COUNT 1.
- READY item: `NA-0511 -- QSL Remote Host Read/Write Marker and Toolchain/Disk Capability Probe Scope Authorization Plan`.
- NA-0510 DONE.
- NA-0509 DONE.
- NA-0508 DONE.
- D-1009 exists once.
- D-1010 exists once.
- D-1011 was absent before this patch.
- duplicate decision count 0.

Current main health before patch:

- `public-safety` completed success on `451d596e8f94`.
- `qsc-adversarial-smoke` completed success on `451d596e8f94`.
- `qsc-linux-full-suite` completed skipped under accepted docs/governance policy.
- `macos-qsc-full-serial` completed skipped under accepted docs/governance policy.
- no completed red checks were observed.

qsl-backup boundary:

- installed helper digest matched `e9ecff3d22eda21ceb0e889e4dd5d6f4e270e09349c77a1f4872bfc0052f6232`.
- the configured Codex ops source inclusion count was exactly 1.
- Codex did not run backup, restore, or qsl-backup.

## NA-0510 / D401 inheritance

NA-0510 completed and NA-0511 was restored READY by D401 closeout.

Inherited facts:

- D400 classification: `REMOTE_READ_ONLY_PROBE_PASS`.
- PR #1292 merged at `bff96ccc1fe7`.
- D401 closeout PR #1293 merged at `451d596e8f94`.
- Exactly one bounded SSH invocation occurred in NA-0510.
- No SSH invocation occurred in D401 closeout.
- The remote account proof for `qslcodex` passed.
- Non-root proof passed.
- No-sudo proof passed.
- Workdir existence/writability proof passed using read-only `test -w`.
- Backup exposure proof passed.
- qwork absence proof passed.
- qsl-backup absence proof passed.
- No remote E2E ran.
- No remote file write occurred.
- No remote marker write/read/delete occurred.
- No remote qsc/git/cargo/rustc/qsc/toolchain probe occurred.
- No private key, passphrase, token, password, production endpoint, or backup
  material was included in checked-in evidence.

Selected NA-0511 purpose:

- authorize exact future marker write/read/delete and toolchain/disk capability
  capture scope, or reject that scope with a documented remediation/no-action
  rationale.

## Time-sensitive remote assurance review

The `inspiron` host may be available only for a limited time. NA-0510 already
proved the read-only remote account boundary, but it did not prove short-lived
write cleanup or staging feasibility. The next useful lane should therefore
gather the largest safe capability signal without jumping to protocol traffic.

Conclusion:

- Process/tooling lanes are deferred unless they block remote testing.
- Remote E2E remains deferred until capability/staging boundaries are stronger.
- The next useful remote lane should gather short-lived write/read/delete plus
  toolchain/disk data.
- No public or production readiness claim may be made from this evidence.

Remote assurance ladder:

1. NA-0512: execute bounded marker write/read/delete and toolchain/disk
   capability probe.
2. NA-0513: authorize remote qsc staging/build/smoke or prebuilt-binary smoke,
   based on NA-0512 results.
3. NA-0514: implement chosen staging/build/smoke.
4. NA-0515: authorize remote client-to-client E2E.
5. NA-0516: implement remote client-to-client E2E if prior evidence supports it.

This ladder is guidance only. NA-0511 implements none of those lanes.

## Marker write/read/delete scope design

Future NA-0512 may create exactly one marker file.

Authorized future marker constraints:

- marker directory: existing `$HOME/qsl-remote-test` only.
- marker string: `QSL_REMOTE_MARKER_SYNTHETIC_NA0512`.
- marker path pattern: `$HOME/qsl-remote-test/na0512_marker_${PROOF_ID}.txt`.
- `PROOF_ID` must be a safe alphanumeric timestamp-like value supplied by the
  local NA-0512 proof wrapper.
- future probe must stop if the marker path already exists.
- future probe must verify the marker file exists.
- future probe must read back the exact marker content.
- future probe must delete the marker file.
- future probe must verify the marker no longer exists.
- future probe must stop if any other file would be touched.
- future probe must stop if marker deletion fails.
- future probe must stop if the marker remains after deletion.

Forbidden future marker behavior:

- no write outside `$HOME/qsl-remote-test`.
- no private data write.
- no production data write.
- no artifact left behind.
- no overwrite of an existing file.
- no directory creation outside the existing workdir.
- no remote E2E.
- no qsc send/receive.
- no qwork or qsl-backup.
- no sudo/admin action.
- no package installation.
- no source checkout/build.

Recommended future marker sequence for NA-0512:

```bash
MARKER="$HOME/qsl-remote-test/na0512_marker_${PROOF_ID}.txt"
test ! -e "$MARKER"
printf '%s\n' 'QSL_REMOTE_MARKER_SYNTHETIC_NA0512' > "$MARKER"
test -f "$MARKER"
readback="$(cat "$MARKER")"
test "$readback" = 'QSL_REMOTE_MARKER_SYNTHETIC_NA0512'
rm -f -- "$MARKER"
test ! -e "$MARKER"
```

## Toolchain/disk scope design

Future NA-0512 may capture toolchain presence and disk status only. Toolchain
absence is not failure; it informs successor selection.

Authorized future toolchain/disk commands:

- `command -v git`
- `command -v cargo`
- `command -v rustc`
- `command -v qsc`
- `git --version` only if git exists.
- `cargo --version` only if cargo exists.
- `rustc --version` only if rustc exists.
- `qsc --version` or `qsc --help` only if qsc exists and a future NA-0512
  pre-check proves the chosen qsc command is non-mutating; otherwise record qsc
  presence only.
- `df -h "$HOME"`
- `df -h "$HOME/qsl-remote-test"`
- `uname -srm` by default; `uname -a` only if NA-0512 records redaction need and
  redacts sensitive host details in final evidence.

Forbidden future toolchain/disk commands:

- no `git clone`.
- no `git fetch`.
- no `cargo build`.
- no `cargo test`.
- no `cargo update`.
- no rustup install/update.
- no package managers.
- no qsc protocol commands.
- no command that writes files or state.
- no qwork or qsl-backup.
- no sudo/admin action.

Successor-selection implications:

- if git/cargo/rustc exist, future staging/build/smoke may be feasible.
- if they are absent, future work may need prebuilt binary copy authorization or
  operator-managed toolchain setup authorization.
- if qsc exists, future qsc smoke may be possible after separate authorization.
- if qsc is absent, future staging must decide how qsc arrives on remote.

## Exact future command list

NA-0512 must perform local pre-SSH proof before any remote invocation.

Local pre-SSH proof may parse `ssh -G inspiron` for these safe fields only:

- hostname.
- user.
- identityfile basename only.
- identitiesonly.
- passwordauthentication.
- batchmode.
- stricthostkeychecking.
- forwardagent.
- forwardx11.
- clearallforwardings.

Local pre-SSH proof must also prove optional alias `remote` remains absent unless
later approved. It must not print private key contents, print a passphrase,
mutate known_hosts, or print full unrelated SSH configuration.

Future NA-0512 may run exactly one bounded SSH invocation:

```bash
ssh -o BatchMode=yes -o PasswordAuthentication=no -o ConnectTimeout=10 inspiron 'bash -s'
```

Authorized remote script command family:

- `hostname`
- `id -un`
- `id -u`
- `id -Gn`
- `pwd`
- `printf "%s\n" "$HOME"`
- `test -d "$HOME/qsl-remote-test"`
- `test -w "$HOME/qsl-remote-test"`
- `sudo -n true` as a negative capability check only.
- `test -e /backup/qsl`
- `test -r /backup/qsl`
- `command -v qwork`
- `command -v qsl-backup`
- `command -v git`
- `command -v cargo`
- `command -v rustc`
- `command -v qsc`
- `git --version` if git exists.
- `cargo --version` if cargo exists.
- `rustc --version` if rustc exists.
- safe qsc presence/help/version check only if qsc exists and the chosen command
  is proven non-mutating; otherwise qsc presence only.
- `uname -srm` by default.
- `df -h "$HOME"`
- `df -h "$HOME/qsl-remote-test"`
- create/read/delete one synthetic marker under `$HOME/qsl-remote-test`.
- verify marker absent after deletion.
- `printf` fixed proof markers.

Forbidden future NA-0512 remote commands:

- qsc send/receive.
- qsc key generation unless later authorized.
- git clone/fetch/pull.
- cargo build/test/update.
- package installs.
- sudo except negative `sudo -n true`.
- qwork/qstart/qresume.
- qsl-backup.
- service/daemon commands.
- file writes outside the marker file.
- remote E2E.
- scp/sftp/rsync.

## Expected future outputs

Expected success markers:

- `NA0512_REMOTE_MARKER_SCOPE_CONSUMED_OK`
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
- `NA0512_REMOTE_PROBE_DONE_OK`

Expected non-fatal status fields:

- `REMOTE_TOOLCHAIN_GIT present|absent`
- `REMOTE_TOOLCHAIN_CARGO present|absent`
- `REMOTE_TOOLCHAIN_RUSTC present|absent`
- `REMOTE_QSC_PRESENT present|absent`
- `REMOTE_DF_HOME captured`
- `REMOTE_DF_WORKDIR captured`

Toolchain absence must not fail NA-0512. It should guide NA-0513 successor
selection.

## Redaction rules

Future NA-0512 proof must:

- include no private key.
- include no passphrase.
- include no token.
- include no password.
- include no production endpoint.
- include no backup material.
- summarize or redact hostnames/IPs if sensitive.
- avoid full `ssh -G` output; only selected safe fields are allowed.
- avoid full known_hosts.
- avoid full authorized_keys.
- avoid unrelated environment variables.
- avoid directory listings outside `$HOME/qsl-remote-test`.
- avoid printing the full private key path if basename is enough.
- redact filesystem/device names from `df` output if needed in final response.
- save raw proof under the directive proof root only.
- keep final response proof summaries sanitized.

## Stop conditions

Future NA-0512 must stop if:

- SSH alias is not configured as expected.
- SSH host key changes or strict checking fails.
- SSH cannot connect with BatchMode/key-only.
- remote user is not `qslcodex`.
- remote UID is 0.
- privileged group is present: sudo, adm, docker, lxd, libvirt, wheel, or admin.
- `sudo -n true` returns 0.
- `/backup/qsl` is readable.
- qwork is present.
- qsl-backup is present.
- workdir is missing or not writable.
- marker path already exists.
- marker write fails.
- marker readback mismatches.
- marker deletion fails.
- marker remains after deletion.
- output contains private key, passphrase, token, or password material.
- output contains a production endpoint or backup material.
- command needs sudo/admin.
- command needs package installation.
- command needs remote E2E.
- command needs source checkout/build.
- command needs remote mutation outside the marker.
- remote host appears production or user-data sensitive.
- public/production readiness claim pressure appears.

## No remote E2E boundary

NA-0512 is a marker/toolchain/disk capability probe only.

NA-0512 must not:

- run remote E2E.
- run qsc send/receive.
- clone or build qsl-protocol.
- run cargo build/test on remote.
- install packages.
- create long-lived remote artifacts.
- claim public or production readiness.

Remote E2E remains deferred until:

- marker write/read/delete passes.
- toolchain/disk evidence is available.
- staging/build/smoke strategy is authorized or toolchain absence is addressed.
- a separate remote E2E authorization lane selects exact protocol flow.

## Option review

| Option | Disposition | Risk reduced | Evidence gap addressed | Feasibility | Scope risk | Remote mutation risk | Secret/key risk | Public-claim risk | Likely future allowed paths | Likely future forbidden paths | P0/P1/P2 risks |
|---|---|---|---|---|---|---|---|---|---|---|---|
| Remote Read/Write Marker and Toolchain/Disk Capability Probe Implementation Harness | Select | Proves short-lived write cleanup and staging feasibility signal | marker lifecycle plus git/cargo/rustc/qsc/disk status | High after NA-0510 pass | Medium, bounded by exact command list | Medium, exactly one marker only | Low if redacted | Medium if overstated, mitigated by no-claim boundary | NA-0512 evidence/testplan/decision/traceability/journal plus proof-root output | remote E2E, source checkout/build, package install, qsc protocol commands | P0: marker cleanup failure; P1: redaction miss; P2: tool absence requiring new lane |
| Remote Read/Write Marker Probe Only | Reject/defer | Proves write cleanup only | marker lifecycle only | High | Low | Medium | Low | Low | later marker-only evidence/testplan if combined scope is rejected | toolchain/disk capture and E2E | P0: cleanup failure; P1: repeated remote access; P2: staging gap remains |
| Remote Toolchain/Disk Probe Only | Reject/defer | Proves staging feasibility only | toolchain/disk only | High | Low | Low | Medium from topology output | Medium if build readiness overstated | later toolchain-only evidence/testplan if marker scope becomes unsafe | marker write/delete and E2E | P0: topology leakage; P1: write gap remains; P2: extra lane cost |
| Remote qsc Staging/Build/Smoke Authorization | Defer | Would reduce future staging ambiguity | staging strategy | Medium after NA-0512 | High now | Medium/high | Medium | High if treated as product proof | NA-0513 authorization docs after NA-0512 | E2E and unproven package/source actions | P0: premature remote build; P1: toolchain absent; P2: stale staging plan |
| Remote Client-to-Client E2E Authorization | Defer | Would scope protocol test | remote E2E flow | Low before NA-0512/NA-0513 | High | High | Medium | High | later authorization docs only | implementation before capability/staging proof | P0: premature protocol claim; P1: unclear staging; P2: operator burden |
| Remote Client-to-Client E2E Implementation | Reject/defer | Would prove remote protocol flow if ready | full remote E2E | Low now | Very high | High | Medium/high | Very high | none now | remote E2E implementation now | P0: broad unbounded mutation; P1: cleanup failure; P2: noisy failures |
| Same-host E2E negative expansion | Defer | Improves local qsc regression safety | local negative coverage | High | Medium | None | Low | Low | qsc test only in later lane | remote work | P0: distracts from time-limited host; P1: scope churn; P2: duplicate coverage |
| Remote setup remediation | Defer unless NA-0512 fails safety | Repairs boundary weakness | account/setup issue | Unknown | Medium/high | High if setup mutates host | Medium | Medium | future runbook/proof-review docs only unless approved | Codex setup now | P0: unsafe remote setup; P1: key/config mutation; P2: delay |
| CI/tooling lane | Reject unless blocker appears | Reduces process friction | process only | High | Low/medium | None | Low | Low | focused tooling docs/scripts only if authorized | remote capability work | P0: loses remote window; P1: not evidence-direct; P2: queue delay |

## Best-Known-Method Review

Best known method is a single bounded implementation lane that rechecks the
read-only safety boundary immediately before the only write, performs one
synthetic marker lifecycle, captures presence/disk signals using read-only
commands, and stops on any privilege, backup, qwork/qsl-backup, cleanup, or
redaction anomaly.

This is narrower than staging/build/smoke and more informative than marker-only
or toolchain-only probes while the remote host window is open.

## Hostile Cryptographer Review

Does marker write/read/delete add useful capability evidence without becoming
protocol evidence?

- Yes. It proves the dedicated workdir can tolerate one synthetic write lifecycle
  with cleanup. It does not prove confidentiality, authentication, transcript
  binding, downgrade resistance, replay behavior, or remote E2E.

What could the marker probe fail to prove?

- It does not prove qsc protocol behavior, durable staging, concurrent cleanup,
  remote build safety, future account state, or data-retention policy beyond the
  one marker path.

Could toolchain/disk output leak sensitive local topology?

- Yes. Device names, hostnames, kernel strings, and paths can be sensitive. NA-0512
  must redact final summaries and keep raw proof under the proof root.

Does checking toolchain presence create unsafe expectations of remote build
readiness?

- It can if overstated. NA-0512 must treat toolchain presence as a staging signal
  only, not a build authorization or product-readiness claim.

Does this lane avoid public/production readiness claims?

- Yes. It records only operational capability authorization and preserves no
  public-readiness claim and no production-readiness claim.

## Red-Team Review

What if marker cleanup fails?

- NA-0512 must stop, report cleanup failure, and not proceed to staging or E2E.
  Manual cleanup/revocation proof should be required before any later remote lane.

What if a malicious or misconfigured remote shell aliases commands?

- NA-0512 should rely on POSIX/core commands in a non-interactive script and
  fail closed on inconsistent markers. A future hardening improvement may use
  absolute paths if the remote baseline justifies it.

What if qslcodex gains sudo after NA-0510?

- NA-0512 must stop if privileged groups appear or `sudo -n true` succeeds.

What if `/backup/qsl` appears or becomes readable?

- NA-0512 must stop before marker write if backup exposure is readable.

What if qwork/qsl-backup appears remotely?

- NA-0512 must stop before marker write because the remote boundary changed.

What if toolchain commands write state unexpectedly?

- Only version/presence commands are authorized, and qsc help/version is gated by
  a non-mutation proof. If uncertainty remains, record qsc presence only.

What if output contains hostnames/IPs or sensitive paths?

- Raw proof stays under proof root; final response and checked-in evidence use
  redacted summaries.

What cleanup/revocation proof is needed before remote E2E?

- Marker absent-after-delete proof, no backup exposure, no qwork/qsl-backup,
  no-sudo account boundary, and a separate staging/build/smoke strategy proof.

## Production SRE Review

Is combined marker/toolchain/disk probing the smallest useful next step while
the host is available?

- Yes. It combines one short-lived marker with read-only capability capture in a
  single bounded SSH session, reducing repeated remote access.

Which future probe results are hard stop conditions?

- identity drift, root UID, privileged groups, sudo success, backup readability,
  qwork/qsl-backup presence, missing/unwritable workdir, marker lifecycle
  failure, output containing sensitive material, or need for package/source/E2E
  work.

How should future marker artifacts be retained or deleted?

- The marker must be deleted immediately by the same probe. Evidence retains only
  fixed markers and redacted summaries, not a remote artifact.

How should future remote command output be redacted?

- Keep raw under proof root, redact final summaries, avoid private key paths
  beyond basenames, and summarize device/host fields if sensitive.

Why should remote E2E remain deferred until after staging strategy?

- E2E needs qsc presence or a chosen staging path. NA-0510 did not prove that,
  and NA-0511 only authorizes the next capability step.

How should future remote probe failures be isolated from local qbuild/qwork
state?

- Use a directive proof root, do not run qwork remotely, do not mutate qwork
  state, and treat remote failure as evidence for a follow-up remediation lane.

## Side-Channel Caveat

Marker/toolchain/disk probing does not prove side-channel-free behavior. It does
not measure timing, traffic shape, filesystem side effects beyond one marker
path, command-history behavior, terminal logging, or remote host telemetry. Any
future side-channel claim remains forbidden without separate evidence.

## Formal-Model Mapping Residual

NA-0511 maps to G4 process verification and operational evidence only. It does
not add or validate a formal model for remote operations, qsc protocol flow,
state machines, key schedules, or negotiation. Formal-model coverage remains a
separate residual for protocol behavior.

## External-Review Readiness

This evidence is organized for later external review of the remote assurance
ladder, but external review is not complete. A reviewer would still need NA-0512
raw proof summaries, staging/build/smoke evidence, remote E2E authorization, and
claim-boundary review.

## Release-Claim Boundary Review

Preserved boundaries:

- no public-readiness claim is made.
- no production-readiness claim is made.
- no public-internet-readiness claim is made.
- no external-review-complete claim is made.
- no crypto-complete claim is made.
- no replay-proof claim is made.
- no downgrade-proof claim is made.
- no secret-material-complete claim is made.
- no side-channel-free claim is made.
- no vulnerability-free claim is made.
- no bug-free claim is made.
- no perfect-crypto claim is made.

## Assurance Gap Review Trigger

Open an assurance-gap review instead of proceeding to remote E2E if NA-0512
finds any of these:

- marker cleanup failure.
- privilege drift.
- backup exposure.
- qwork or qsl-backup on remote.
- toolchain absence that blocks staging.
- qsc presence without safe non-mutating command proof.
- disk pressure or filesystem uncertainty.
- sensitive output requiring broader redaction than expected.

## Prioritization matrix

| Candidate | Risk reduced | Directness of evidence | Security risk | Operator burden | Implementation feasibility | Scope risk | Remote mutation risk | Secret/key risk | Public-claim risk | Recommended disposition | Next-lane yes/no |
|---|---|---|---|---|---|---|---|---|---|---|---|
| Combined marker + toolchain/disk capability probe implementation | High | High | Medium | Low/medium | High | Medium | Medium, one marker only | Low with redaction | Medium | Select | Yes |
| Marker-only capability probe implementation | Medium | High for write cleanup | Medium | Medium | High | Low | Medium | Low | Low | Defer | No |
| Toolchain/disk-only capability probe implementation | Medium | High for staging signal | Low/medium | Medium | High | Low | Low | Medium from topology | Medium | Defer | No |
| Remote qsc staging/build/smoke authorization | Medium/high later | Medium now | High before capability proof | Medium | Medium after NA-0512 | High now | Medium/high | Medium | High | Defer | No |
| Remote client-to-client E2E authorization | High later | Low now | High | High | Low before staging | High | High | Medium | High | Defer | No |
| Remote client-to-client E2E implementation | High only if ready | Premature now | Very high | High | Low | Very high | High | Medium/high | Very high | Reject/defer | No |
| Same-host E2E negative expansion | Medium local | Indirect for remote | Low | Low | High | Medium | None | Low | Low | Defer | No |
| Remote setup remediation | High only if boundary weak | Conditional | High if setup mutates | High | Conditional | High | High | Medium | Medium | Defer unless needed | No |
| CI/tooling lane | Low for remote assurance | Indirect | Low | Low | High | Low/medium | None | Low | Low | Reject unless blocker | No |

## Authorization decision

Primary classification:

- `REMOTE_MARKER_TOOLCHAIN_DISK_PROBE_IMPLEMENTATION_READY`

Reason:

NA-0510 passed the read-only remote boundary. The remote host may be time
limited. The next safest high-value step is a single bounded remote capability
probe that rechecks safety boundaries, performs a short-lived marker
write/read/delete under the dedicated workdir, captures toolchain/disk status,
and leaves remote E2E deferred.

Decision requirements satisfied:

- NA-0510 / D401 consumed.
- time-sensitive remote assurance review completed.
- marker write/read/delete scope selected.
- toolchain/disk scope selected.
- exact future command list selected.
- expected future outputs selected.
- redaction rules selected.
- stop conditions selected.
- no remote E2E boundary recorded.
- option review completed.
- hostile cryptographer, red-team, SRE, release-claim, side-channel, formal
  residual, external-review, and assurance-gap reviews completed.
- exact NA-0512 successor selected.
- no remote action in this directive.
- no key generation.
- no SSH config mutation.
- no implementation mutation.
- no public claim expansion.
- exactly one READY successor remains mandatory.

## Selected NA-0512 successor

### NA-0512 -- QSL Remote Host Read/Write Marker and Toolchain/Disk Capability Probe Implementation Harness
Status: READY
Goals: G1, G2, G3, G4, G5

Objective:
Execute one bounded SSH capability probe against the approved `inspiron` /
`qslcodex` remote test account to reverify account identity, no sudo/admin
access, workdir existence/writability, short-lived marker-file write/read/delete,
no backup exposure, no remote qwork/qsl-backup presence, toolchain/disk
capability, and no alias/host drift, without remote E2E, without remote source
checkout/build, without package installs, and without public/production
readiness claims.

## Future scope bundle

Allowed future NA-0512 scope:

- `docs/governance/evidence/NA-0512_qsl_remote_host_read_write_marker_toolchain_disk_capability_probe_implementation_harness.md`
- `tests/NA-0512_qsl_remote_host_read_write_marker_toolchain_disk_capability_probe_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- proof-root-local capture of the exact SSH command output.
- one bounded SSH command to `inspiron` as `qslcodex` using the preconfigured
  local alias, if and only if fresh qwork proof and lane scope authorize it.
- one short-lived synthetic marker-file write/read/delete under
  `$HOME/qsl-remote-test`.

Forbidden future NA-0512 scope:

- remote E2E.
- qsc protocol execution remotely.
- remote source checkout/build.
- remote package install.
- sudo/admin action other than negative `sudo -n true` probe.
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
- no public-readiness claim and no production-readiness claim.
- long-lived remote artifacts.
- writes outside `$HOME/qsl-remote-test`.

Future NA-0512 deliverables:

- remote capability probe implementation evidence.
- testplan.
- decision.
- TRACEABILITY update.
- rolling journal update.
- selected future remote staging/build/smoke authorization scope, remote E2E
  authorization scope, or remediation/no-action rationale.

Future NA-0512 acceptance criteria:

- qwork proof fresh.
- exact remote command captured.
- redaction rules applied.
- account identity `qslcodex` verified.
- non-root verified.
- no sudo/admin verified.
- workdir exists and writable verified.
- marker write/read/delete verified.
- marker deletion verified.
- no backup exposure verified.
- no qwork/qsl-backup presence verified.
- toolchain/disk capability captured.
- no remote E2E.
- no key material included.
- exactly one READY item remains after closeout.

## Future validation / marker plan

Common future NA-0512 markers:

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

NA-0511 evidence markers:

- `NA0511_QWORK_PROOF_FILES_VERIFIED_OK`
- `NA0511_D401_INHERITANCE_CONSUMED_OK`
- `NA0511_TIME_SENSITIVE_REMOTE_ASSURANCE_REVIEW_OK`
- `NA0511_MARKER_SCOPE_SELECTED_OK`
- `NA0511_TOOLCHAIN_DISK_SCOPE_SELECTED_OK`
- `NA0511_EXACT_FUTURE_COMMANDS_SELECTED_OK`
- `NA0511_EXPECTED_OUTPUTS_SELECTED_OK`
- `NA0511_REDACTION_RULES_SELECTED_OK`
- `NA0511_STOP_CONDITIONS_SELECTED_OK`
- `NA0511_NO_REMOTE_E2E_BOUNDARY_OK`
- `NA0511_STEWARDSHIP_REVIEWS_COMPLETED_OK`
- `NA0511_PRIORITY_MATRIX_COMPLETED_OK`
- `NA0511_REMOTE_MARKER_TOOLCHAIN_DISK_PROBE_IMPLEMENTATION_READY`
- `NA0511_SELECTED_NA0512_SUCCESSOR_OK`
- `NA0511_NO_REMOTE_ACTION_BY_CODEX_OK`
- `NA0511_NO_SSH_EXECUTION_BY_CODEX_OK`
- `NA0511_NO_MARKER_WRITE_READ_DELETE_BY_CODEX_OK`
- `NA0511_NO_QSC_IMPLEMENTATION_MUTATION_OK`
- `NA0511_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0511_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0511_ONE_READY_INVARIANT_OK`

## Public claim / website / external review boundary

NA-0511 changes no website, README, START_HERE, public technical paper,
public-service, public-demo, or external-review artifact. It makes no
public-readiness claim, no production-readiness claim, no public-internet-readiness
claim, no external-review-complete claim, no crypto-complete claim, no
replay-proof claim, no downgrade-proof claim, no secret-material-complete claim,
no side-channel-free claim, no vulnerability-free claim, no bug-free claim, and
no perfect-crypto claim.

## Backup-impact statement

Backup impact classification: no backup-plan update required.

NA-0511 performs no backup, restore, qsl-backup execution, qsl-backup mutation,
backup status mutation, backup plan mutation, `/backup/qsl` mutation, rollback
mutation, archive, move, or delete. Future NA-0512 must stop if `/backup/qsl` is
readable remotely or if qsl-backup is present on the remote account path.

## Rejected alternatives

- Marker-only probe was rejected as the primary successor because it would leave
  toolchain/disk feasibility unknown and cost another remote lane.
- Toolchain/disk-only probe was rejected as the primary successor because it
  would leave write/delete cleanup unknown.
- Remote qsc staging/build/smoke authorization was deferred until NA-0512
  capability evidence exists.
- Remote client-to-client E2E authorization and implementation were deferred as
  premature.
- Same-host E2E negative expansion was deferred because it does not address the
  time-limited remote host opportunity.
- Remote setup remediation was deferred because NA-0510 did not show boundary
  weakness.
- CI/tooling work was rejected because no process blocker currently outranks the
  remote capability lane.

## Next recommendation

After this authorization evidence merges and closeout restores NA-0512, execute
NA-0512 as one bounded marker/toolchain/disk capability probe only. Do not run
remote E2E, qsc protocol commands, source checkout/build, package installation,
or public/production claim work until separate successor evidence authorizes it.
