Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-21

# NA-0514 qsl remote qsc prebuilt binary staging and smoke implementation harness

## Executive summary

NA-0514 consumed NA-0513 / D406 inheritance, built the local `qsc`
binary from the clean current qsl-protocol checkout, staged that binary to the
approved Inspiron `qslcodex` test account under the qsl-remote-test binary
directory, verified the remote hash against the local hash, ran one bounded
non-protocol `--help` smoke, and retained the final binary for the next remote
E2EE authorization lane.

Result classification:

- `REMOTE_PREBUILT_QSC_STAGING_SMOKE_PASS_RETAINED`

Selected successor:

- `NA-0515 -- QSL Build-to-Inspiron Remote qsc Client-to-Client E2EE Scope Authorization Plan`

## Live NA-0514 scope

Allowed implementation scope was limited to local qsc build/provenance proof,
local non-protocol smoke proof, one bounded remote prep invocation, one bounded
transfer, one bounded remote hash/smoke/retention invocation, and checked-in
governance/testplan/decision/traceability/journal updates.

NA-0514 did not authorize remote E2EE, remote qsc send/receive, remote source
checkout/build, package installation, key generation or installation, SSH config
mutation, known_hosts mutation, qwork/qstart/qresume execution, qsl-backup
execution, qsc source/test/fuzz/Cargo mutation, workflow/script/helper mutation,
corpus/vector/input mutation, formal/refimpl/service/public/backup mutation, or
public/production/security-completion claims.

## qwork proof-file verification

Codex did not run qwork, qstart, or qresume.

Read-only qwork proof files existed and were copied into the directive proof
root:

- `/srv/qbuild/work/NA-0514/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0514/.qwork/startup.qsl-protocol.json`

Required proof fields passed:

- `startup_result=OK`
- `lane=NA-0514`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0514/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0514`
- `requested_lane_status=READY`

Freshness proof before fetch:

- proof `HEAD` matched live `HEAD`.
- proof `origin/main` matched live `origin/main`.

Post-fetch proof:

- `origin/main` equaled `6e0796de79c9`.
- `origin/main` descended from expected base `6e0796de79c9`.
- READY_COUNT remained 1.
- READY item remained NA-0514.
- NA-0513 and NA-0512 were DONE.
- D-1015 existed once.
- D-1016 existed once.
- D-1017 was absent before this implementation patch.
- duplicate decision count was zero.

Disk proof before fetch:

- `/` usage: 82%.
- `/backup/qsl` usage: 26%.
- STOP threshold 95% was not hit.

## NA-0513 / D406 inheritance

NA-0513 completed and NA-0514 was restored READY by D406 / PR #1299.

Inherited facts consumed:

- D406 triaged `remote-handshake`, `remote-relay`, and
  `relay-ui-integration` as non-required scheduled remote residuals.
- Those red scheduled checks were not required branch-protection checks, were
  not public-safety inputs, and were not NA-0512 closeout acceptance
  requirements.
- NA-0512 classification was `REMOTE_MARKER_PROBE_PASS_TOOLCHAIN_ABSENT`.
- NA-0512 proved marker write/read/delete/absent-after-delete under the
  approved remote test directory.
- NA-0512 proved remote `git`, `cargo`, `rustc`, and `qsc` were absent.
- NA-0512 proved no remote E2E, no source checkout/build, no package install,
  no backup exposure, remote qwork absence, and remote qsl-backup absence.
- NA-0513 selected prebuilt qsc binary staging/smoke.
- NA-0513 preferred retention if hash/provenance/smoke passed and cleanup
  instructions were recorded.

No public-readiness claim is made.
No production-readiness claim is made.
No public-internet-readiness claim is made.
No crypto-complete claim is made.
No replay-proof claim is made.
No downgrade-proof claim is made.
No secret-material-complete claim is made.

## Local qsc build / selection proof

Local source commit:

- `6e0796de79c9`

Build command:

- `cargo build -p qsc --locked --bin qsc`

Build target directory:

- directive proof-root-local Cargo target directory.

Binary selection:

- exactly one owner-executable file named `qsc` was found under the isolated
  debug target directory.
- binary size: `102103920` bytes.
- sha256: `6f12ab5eec2468a4146012dfc247cef15be2337cb8e8e99cefa96a8c258d91ea`
- file type: ELF 64-bit x86-64 dynamically linked executable, with debug info,
  not stripped.

Recovered local lookup issue:

- failing command: initial `find` predicate required owner, group, and other
  executable bits.
- classification: recoverable command-shape/permission predicate mistake; the
  binary existed with owner-executable permissions in the isolated target
  directory.
- corrective action: reran the lookup with current-user executable detection.
- final result: corrected lookup found exactly one `qsc` binary.

No Cargo manifest, lockfile, qsc source, qsc test, qsc fuzz, or corpus mutation
occurred.

## Local qsc smoke proof

Local non-protocol smoke command:

- `<local-qsc-bin> --help`

Local smoke result:

- exit code: `0`
- no private key block found.
- no passphrase/token/password assignment pattern found.
- no production endpoint marker found.
- no `qsc send` or `qsc receive` command invocation marker found.

No local qsc protocol send/receive command was run.

## Local pre-transfer SSH config proof

Local effective SSH config was collected with `ssh -G inspiron` before transfer.
Checked-in evidence records only safe fields:

- hostname: `inspiron`
- user: `qslcodex`
- identityfile basename: `qslcodex_ed25519`
- identitiesonly: `yes`
- passwordauthentication: `no`
- batchmode: `yes`
- stricthostkeychecking: `true`
- forwardagent: `no`
- forwardx11: `no`
- clearallforwardings: `yes`

The raw `ssh -G` output stayed proof-root-local. It contained no private key
material and no passphrase/token/password assignment pattern.

No SSH config, known_hosts file, authorized_keys file, key material, or system
SSH configuration was mutated.

## Remote prep proof

One bounded prep invocation was executed:

- SSH alias: `inspiron`
- remote account: `qslcodex`
- credential mode: BatchMode with password authentication disabled.

Remote prep verified:

- account was `qslcodex`.
- UID was not `0`.
- no privileged group was present.
- negative `sudo -n true` check failed as expected.
- backup exposure was absent.
- qwork was absent.
- qsl-backup was absent.
- qsl-remote-test workdir existed and was writable.
- qsl-remote-test bin directory was ready.
- final qsc path was absent before transfer.
- stage path was absent before transfer.

Remote prep created only the qsl-remote-test bin directory if absent. It wrote
no file.

## Transfer proof

Exactly one transfer command was run:

- `scp -o BatchMode=yes -o PasswordAuthentication=no -o ConnectTimeout=10 <local-qsc-bin> inspiron:"~/qsl-remote-test/bin/qsc_<proof_id>.stage"`

Transfer result:

- exit code: `0`
- no retry was attempted.
- no rsync or sftp was used.
- no transfer outside qsl-remote-test occurred.

## Remote hash verification proof

The final remote verification/smoke invocation rechecked account, privilege,
backup, qwork, and qsl-backup boundaries.

Hash proof:

- local sha256:
  `6f12ab5eec2468a4146012dfc247cef15be2337cb8e8e99cefa96a8c258d91ea`
- remote stage sha256:
  `6f12ab5eec2468a4146012dfc247cef15be2337cb8e8e99cefa96a8c258d91ea`
- remote final sha256:
  `6f12ab5eec2468a4146012dfc247cef15be2337cb8e8e99cefa96a8c258d91ea`
- remote hash match: yes.

The stage file was chmodded to qslcodex-only executable permissions before
smoke.

## Remote qsc smoke proof

Remote non-protocol smoke command:

- `qsl-remote-test/bin/qsc_<proof_id>.stage --help`

Remote smoke result:

- exit code: `0`
- no qsc send/receive was run.
- no remote E2E was run.
- no source checkout/build was run.
- no package installation was run.
- no key generation was run.

The help output listed qsc commands but was treated only as non-protocol
process-execution smoke evidence.

## Retention / cleanup decision

Retention decision:

- retained final binary for near-term E2EE authorization lane.

Retained final path, recorded as qsl-remote-test-relative:

- `qsl-remote-test/bin/qsc`

Final owner:

- `qslcodex`

Cleanup command for later use:

```bash
ssh -o BatchMode=yes -o PasswordAuthentication=no -o ConnectTimeout=10 inspiron 'rm -f -- "$HOME/qsl-remote-test/bin/qsc"'
```

The stage file was absent after final move. No stage residue was accepted.

## Remote output redaction review

Raw transfer and remote smoke output stayed under the directive proof root.

Checked-in evidence includes only summary values: local source commit, local
binary size/hash/type, qsl-remote-test-relative remote path, remote hash match,
smoke command family, smoke exit code, retention decision, and cleanup command.

Scan result over raw local smoke, transfer, SSH config, prep, and remote smoke
output:

- private key block count: 0.
- credential assignment pattern count: 0.
- production endpoint marker count: 0.
- backup material marker count: 0.
- qwork execution marker count: 0.
- qsl-backup execution marker count: 0.
- `qsc send` / `qsc receive` command invocation marker count: 0.

No private key, passphrase, token, password, production endpoint, backup
material, known_hosts content, authorized_keys content, unrelated environment,
or unrelated remote listing is included in this checked-in evidence.

## Result classification

Selected classification:

- `REMOTE_PREBUILT_QSC_STAGING_SMOKE_PASS_RETAINED`

Reason:

- local qsc build succeeded.
- exactly one qsc binary was selected.
- local `--help` smoke succeeded.
- local SSH config proof passed.
- remote prep boundaries passed.
- exactly one transfer succeeded.
- remote stage and final hashes matched the local binary hash.
- remote `--help` smoke succeeded.
- final binary was retained with owner/hash/path/cleanup proof.

## Hostile Cryptographer Review

Staging a locally built qsc binary proves only binary provenance, transfer
integrity, remote execution of a non-protocol help command, and retention
state. It does not prove protocol correctness, cryptographic correctness,
remote client-to-client E2EE, replay resistance, downgrade resistance, secret
material completeness, or side-channel freedom.

No public readiness is claimed from this staging smoke.
No production readiness is claimed from this staging smoke.

Remaining provenance gaps before remote E2EE:

- no release build reproducibility proof.
- no signed binary provenance proof.
- no remote attestation.
- no proof the retained binary cannot be replaced after this lane.
- no proof the remote environment is stable for protocol state.

Retaining the binary increases future risk if it becomes stale, is replaced, or
is used by the wrong account. Hash verification reduces transfer/replacement
risk at staging time, but it does not prevent later replacement.

No public-readiness claim is made.
No production-readiness claim is made.
No crypto-complete claim is made.

## Red-Team Review

If the retained binary is replaced after this lane, NA-0515 or any later E2EE
lane must recheck the hash before using it.

If qsc help unexpectedly wrote hidden state, this lane would not fully detect
it. The command was selected because `--help` is non-protocol and normally
read-only, but the next lane must still recheck remote path, owner, hash, and
unexpected residue before any E2EE use.

If the remote path is a symlink or unexpected file, this lane stops. Prep and
verify rejected unsafe stage/final path shapes.

If transfer partially writes then fails, this directive stops and does not retry
unless failure definitively occurred before remote write. In this run transfer
completed once with exit code 0.

If the retained binary is used by the wrong account later, future lanes must
stop. The retained path is only accepted for `qslcodex` in qsl-remote-test.

Cleanup command:

```bash
ssh -o BatchMode=yes -o PasswordAuthentication=no -o ConnectTimeout=10 inspiron 'rm -f -- "$HOME/qsl-remote-test/bin/qsc"'
```

Before using the retained binary, NA-0515 or a later E2EE execution lane must
recheck account, non-root, no-sudo, no privileged group, no backup exposure,
qwork absence, qsl-backup absence, final path type, final owner, final hash, and
absence of unexpected stage residue.

## Production SRE Review

Retaining the qsl-remote-test qsc binary is acceptable for a near-term E2EE
sprint because the remote artifact is restricted to a test account path, its
hash and owner were recorded, and a cleanup command is documented.

Needed retention proof:

- source commit.
- local binary sha256 and size.
- remote final relative path.
- remote final owner.
- final hash match.
- smoke exit code.
- cleanup command.

Failures are isolated from qbuild/qwork because qwork was not run, qsl-backup
was not run, raw proof remains under the directive proof root, and remote
mutation was limited to qsl-remote-test binary staging.

Package/toolchain install remains deferred because NA-0514 selected prebuilt
binary staging and expressly forbids remote package installation, rustup, cargo
setup, and remote source checkout/build.

The next E2EE authorization lane should verify retained binary freshness,
hash/path/owner, exact future qsc command family, proof capture, cleanup/retention
rules, negative boundary checks, and stop conditions before any protocol command
is run.

## Release-Claim Boundary Review

No public-ready claim is made. No production-ready claim is made. No
public-internet-ready claim is made. No external-review-complete claim is made.
No crypto-complete claim is made. No replay-proof claim is made. No
downgrade-proof claim is made. No secret-material-complete claim is made. No
side-channel-free claim is made. No vulnerability-free claim is made. No
bug-free claim is made. No perfect-crypto claim is made.

## Successor selection

Selected successor:

### NA-0515 -- QSL Build-to-Inspiron Remote qsc Client-to-Client E2EE Scope Authorization Plan

Status: READY

Goals: G1, G2, G3, G4, G5

Objective:

Authorize the exact Build-to-Inspiron remote qsc client-to-client E2EE test
scope after NA-0514 stages and smokes a qsc binary on Inspiron, selecting roles,
command family, artifact paths, redaction rules, negative/no-mutation boundary,
cleanup/retention rules, and stop conditions, without running remote E2EE in the
authorization lane and without public/production readiness claims.

## Future scope bundle

Future NA-0515 allowed scope:

- governance evidence/testplan paths for NA-0515.
- `DECISIONS.md`.
- `TRACEABILITY.md`.
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.
- read-only review of NA-0514 staging/smoke output and prior remote setup/probe
  evidence.

Future NA-0515 forbidden scope:

- running SSH in the authorization lane.
- qsc send/receive in the authorization lane.
- remote E2EE execution in the authorization lane.
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
- no public-readiness claim and no production-readiness claim.

## Future validation / marker plan

NA-0514 evidence markers:

- `NA0514_REMOTE_STAGING_SCOPE_CONSUMED_OK`
- `NA0514_LOCAL_QSC_BINARY_BUILT_OR_SELECTED_OK`
- `NA0514_LOCAL_QSC_BINARY_HASH_RECORDED_OK`
- `NA0514_LOCAL_QSC_SMOKE_OK`
- `NA0514_REMOTE_PREP_BOUNDARY_OK`
- `NA0514_REMOTE_QSC_BINARY_STAGED_OK`
- `NA0514_REMOTE_QSC_BINARY_HASH_MATCH_OK`
- `NA0514_REMOTE_QSC_SMOKE_OK`
- `NA0514_REMOTE_QSC_RETENTION_DECISION_OK`
- `NA0514_NO_REMOTE_E2E_OK`
- `NA0514_NO_REMOTE_SOURCE_BUILD_OK`
- `NA0514_NO_PACKAGE_INSTALL_OK`
- `NA0514_NO_SUDO_ADMIN_OK`
- `NA0514_NO_BACKUP_EXPOSURE_OK`
- `NA0514_NO_QWORK_QSLBACKUP_OK`
- `NA0514_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0514_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0514_ONE_READY_INVARIANT_OK`

Future NA-0515 markers should be created by the selected E2EE authorization
lane and should include retained-binary hash/path/provenance acceptance, exact
future command list, proof capture plan, redaction plan, cleanup plan, and
one-READY invariant proof.

## No remote E2E proof

No remote E2E was run. No remote qsc send/receive command was run. No qsc key
generation was run. Remote qsc execution was limited to a single `--help`
smoke.

## No remote source checkout/build proof

No remote git clone/fetch/pull was run. No remote cargo build/test/update was
run. No remote rustup install/update was run. No source checkout/build occurred
on the remote host.

## No package install proof

No remote package installation was run. No sudo/admin action occurred beyond the
negative `sudo -n true` boundary check.

## Public claim / website / external review boundary

No website, public docs, README, START_HERE, public technical paper, public
release metadata, or external-review surface was changed.

No public-readiness claim is made. No production-readiness claim is made. No
public-internet-readiness claim is made. No external-review-complete claim is
made.

## Backup-impact statement

Backup impact: none.

Codex did not run backup, restore, or qsl-backup. The local qsl-backup boundary
check was read-only and matched the expected helper digest
`e9ecff3d22ed`; the Codex ops source appeared exactly once in the installed
helper source list. Remote backup exposure checks passed. No `/backup/qsl`
mutation occurred.

## Rejected alternatives

- Remote E2EE now: rejected because NA-0514 is staging/smoke only.
- Remote qsc send/receive smoke: rejected because protocol commands are out of
  scope until a separate authorization lane.
- Remote source checkout/build: rejected because remote source build is out of
  scope and prior evidence found git/cargo/rustc absent.
- Remote package/toolchain installation: rejected because package install and
  setup mutation are forbidden in this lane.
- Cleanup-after-smoke: rejected for this successful run because hash, owner,
  path, smoke, and cleanup-command proof passed and near-term E2EE authorization
  benefits from the retained artifact.

## Next recommendation

Proceed to NA-0515 as an authorization-only lane. It should consume this
staging/smoke evidence, recheck the retained binary hash/path/owner before
authorizing future use, select the exact Build-to-Inspiron remote E2EE command
family, and preserve all public/production/security-completion claim boundaries.
