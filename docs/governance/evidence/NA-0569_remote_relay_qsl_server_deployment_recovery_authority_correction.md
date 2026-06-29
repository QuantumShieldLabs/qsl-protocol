Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-29

# NA-0569 Remote Relay qsl-server Deployment Recovery Authority Correction

## Executive Summary

NA-0569 is accepted as a narrow Cargo audit recovery plus an
authorization-only correction to the D-1126/D-1127 successor path. The selected
authority classification is:

`REMOTE_RELAY_QSL_SERVER_DEPLOYMENT_RECOVERY_AUTH_READY`.

D-1126 selected qsc relay command discovery after NA-0568 repaired the
qslcodex test workspace but still found the expected listener not ready. That
selection is corrected here because qsc is the client/demo CLI. The relay/server
deployment target for basic remote-handshake and remote-relay recovery is
qsl-server. The exact selected successor is:

`NA-0570 -- QSL Remote Relay qsl-server Inspiron Deployment Recovery Harness`.

The D490 local cargo-audit stop is recovered with classification:

`CARGO_AUDIT_ANYHOW_RUSTSEC_RECOVERY_LOCKFILE_ONLY_PASS`.

NA-0569 did not run SSH, Tailscale, remote commands, qsc commands,
qsl-server commands, qsl-attachments commands, workflow dispatches, reruns, or
any source/runtime mutation. The only dependency mutation is the root
`Cargo.lock` update of `anyhow` from `1.0.100` to `1.0.103`.

## qwork Proof Verification

Fresh qwork proof files were copied from the NA-0569 lane workspace into the
proof root and parsed with a file-backed parser before fetch or repository
mutation.

Required qwork values passed:

- `startup_result=OK`
- `lane=NA-0569`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0569/qsl-protocol`
- `branch=main`
- `upstream=origin/main`
- `head=f4bfada25717`
- `origin_main=f4bfada25717`
- `main=f4bfada25717`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0569`
- `requested_lane_status=READY`
- `cargo_target_mode=shared`
- `shared_target_ready=yes`

Codex did not run `qwork`, `qstart`, or `qresume`.

The copied qwork proof timestamp was `2026-06-29T15:04:45Z`.

Pre-fetch live state passed:

- `HEAD`: `f4bfada25717`
- `origin/main`: `f4bfada25717`
- worktree/index/untracked: clean
- root disk usage: below the 95 percent stop threshold
- backup mount: mounted

After proof/live/disk gates passed, `origin/main` was fetched and local main was
verified at `f4bfada25717`.

## D-1126 / D-1127 Inheritance

D-1126 exists once and is Accepted. It accepted NA-0568, recorded the
classification `REMOTE_RECOVERY_QSC_RELAY_COMMAND_AUTH_REQUIRED`, and selected
NA-0569 as a qsc relay command discovery authorization lane.

D-1127 exists once and is Accepted. It accepted the NA-0568 closeout after PR
#1409 merged at `9be00b932806`, restored NA-0569 as the sole READY item, and
recorded the same qsc relay command discovery successor.

NA-0568 is DONE. NA-0567 is DONE. D-1128 and D-1129 were absent before this
patch. Duplicate decision count was zero. No NA-0569 implementation occurred
before this directive.

## D490 Audit Stop Recovery

D490 selected
`REMOTE_RELAY_QSL_SERVER_DEPLOYMENT_RECOVERY_AUTH_READY` before stopping. D490
did not commit, did not open a PR, did not merge, and did not add live D-1128
on main.

D490 stopped before PR creation because local root `cargo audit --deny
warnings` failed on `RUSTSEC-2026-0190` for `anyhow 1.0.100`. That corrective
action required dependency/lockfile scope that D490 did not authorize.

The saved D490 staged patch was reviewed before reuse. It touched only:

- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/governance/evidence/NA-0569_remote_relay_qsl_server_deployment_recovery_authority_correction.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0569_remote_relay_qsl_server_deployment_recovery_authority_correction_testplan.md`

The saved patch had no unstaged diff, no dependency or forbidden path changes,
and no sensitive-value scan hits. D491 reused it only as a governance base and
updated it with the Cargo audit recovery evidence in this document.

## Cargo Audit / RustSec Remediation

Root `cargo audit --deny warnings` reproduced the D490 failure with return code
1 before remediation. The advisory DB reported `RUSTSEC-2026-0190` for
`anyhow 1.0.100`, with patched versions `>=1.0.103`.

`cargo tree -i anyhow` showed the affected root lockfile package as:

- `anyhow v1.0.100`
- `qsl-tui v0.1.0`

The root `Cargo.toml` did not directly pin `anyhow` to the vulnerable exact
version. The workspace crate requirement allowed a compatible 1.x update, so
lockfile-only remediation was possible.

The remediation command was:

`cargo update -p anyhow --precise 1.0.103`

Post-remediation validation:

- root `cargo audit --deny warnings`: passed
- nested qsc fuzz `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`: passed
- `cargo metadata --locked --format-version=1`: passed
- `cargo tree -i anyhow`: resolved `anyhow v1.0.103`
- `cargo check --workspace --locked`: passed

Dependency recovery classification:

`CARGO_AUDIT_ANYHOW_RUSTSEC_RECOVERY_LOCKFILE_ONLY_PASS`

## Current Main Required-Check Classification

Current main was classified at `f4bfada25717`.

- public-safety: completed success
- advisories: completed success
- suite2-vectors: completed success
- branch-protection required contexts: green or conclusively satisfied
- failed required checks: none
- pending required checks: none
- Cargo.toml / Cargo.lock drift before remediation: none
- nested qsc fuzz Cargo.lock drift: none

Recovered proof note: the first current-main classifier treated `goal-lint` and
`CodeQL` as missing literal main check-run names. This was a recoverable
classifier-shape issue. The corrected proof classified CodeQL through successful
analysis jobs on the main commit and goal-lint through the PR #1410 head
check-run because that workflow is pull-request-only.

The known remote-handshake and remote-relay check-run failures on current main
are not branch-protection required checks for this authorization-only lane and
are the subject of the remote relay recovery queue.

## Repository Role / Test Topology Review

qsl-protocol contains protocol specs, evidence, reference/demo tooling, and the
qsc client/demo CLI. qsc creates relay client requests and drives demo/test
flows, but qsc is not the remote relay/server deployment.

qsc source defines a client command surface, including relay base URL arguments,
handshake over relay inbox, explicit relay send/receive, relay inbox token
configuration, and a local deterministic relay test surface. That local test
surface is not the canonical remote relay server recovery target.

qsl-server is the transport-only relay/server build. Read-only GitHub source
review at `d40e6003fdf0` shows qsl-server exposes canonical `/v1/push` and
`/v1/pull` routes, stores or forwards opaque payloads, validates route-token
headers, and supports optional bearer auth. qsl-server is therefore the remote
relay component that qsc clients need to reach for basic remote-handshake and
remote-relay tests.

qsl-attachments is the separate opaque encrypted attachment service/runtime.
Read-only GitHub source review at `96b9352bd63` shows attachment session/object
routes and ciphertext handling. It is relevant to attachment lanes, not the
basic remote-handshake / remote-relay smoke recovery selected here.

The remote-handshake and remote-relay workflows build qsc and use remote relay
secrets. They are orchestration around qsc clients talking to a remote relay
server. The relay-ui-integration workflow separately clones, builds, and starts
qsl-server locally for ignored UI integration tests, so it remains a deferred
separate issue.

## D-1126 qsc Relay Command Successor Correction

D-1126's qsc relay command-discovery successor was selected from an incomplete
topology assumption. qsc remains the client/demo CLI. Treating qsc as the
canonical relay server would chase a client-side or local test-relay surface
instead of recovering the transport-only qsl-server deployment needed by remote
tests.

The corrected path is to authorize a qsl-server-centered recovery lane.

## qsl-server Deployment Recovery Rationale

NA-0568 repaired non-secret qslcodex workspace directories and proved the qsc
binary was present and executable, but the expected listener remained not ready.
That makes the remaining recovery target the relay/server deployment or staging
state, not another qsc client command discovery lane.

NA-0570 must inspect whether qsl-server source, binary, or a staged artifact is
present under the qslcodex test workspace, then recover it only inside the
selected no-secret, non-privileged boundaries.

## qsl-attachments Boundary

qsl-attachments remains outside this basic remote-handshake / remote-relay
recovery lane. It is a separate opaque encrypted attachment service/runtime.
NA-0570 must not clone, build, run, mutate, or inspect qsl-attachments beyond
read-only classification unless a later attachment-specific lane selects it.

## relay-ui-integration Deferred Boundary

relay-ui-integration already has a separate local qsl-server startup pattern for
ignored UI integration tests. That workflow is not the remote inspiron recovery
lane and remains deferred. NA-0570 should recover only the qsl-server
deployment/staging needed by remote-handshake and remote-relay tests.

## NA-0570 Recovery Model

NA-0570 may authorize Codex to:

- verify qwork and current-main gates;
- run bounded SSH readiness to the operator-owned inspiron test host;
- run proof-root-generated remote inventory through SSH stdin;
- inspect only the qslcodex test workspace and coarse listener/deployment state;
- inspect whether qsl-server source, binary, or staged artifact exists under
  that workspace;
- obtain qsl-server source or a binary only through an exact approved method;
- build qsl-server locally in the proof root only when exact source and lockfile
  state are available;
- stage a built qsl-server binary only under qslcodex test workspace tmp/bin
  boundaries after rollback manifest capture;
- start only a user-owned, non-privileged qsl-server loopback test process if a
  safe no-secret command is discovered;
- verify canonical qsl-server route shape using no-auth/no-body coarse request
  classes only.

NA-0570 must stop before sudo, systemd, Tailscale, firewall, account, shell,
authorized_keys, root-owned service, secret, endpoint, private-topology, or
public-exposure action.

## NA-0570 Exact Command Allowlist

The selected future command families are:

- read-only qsl-server source acquisition into proof root, use of a clean
  discovered checkout, or stop for operator staging;
- `cargo build --release --locked` for qsl-server only in proof-root/shared
  target boundaries selected by qwork/cargo policy;
- bounded SSH readiness using BatchMode and ConnectTimeout;
- remote qsl-server inventory through SSH stdin;
- staged binary transfer to the qslcodex test workspace staging area by the
  exact method selected in NA-0570;
- remote install/stage through SSH stdin with rollback before replacement;
- remote start through SSH stdin only if a safe qsl-server command is
  discovered;
- remote postcheck through SSH stdin.

Future generated scripts must write only under the qslcodex test workspace,
capture rollback first, avoid sudo/systemctl/service, avoid
authorized_keys/shell/Tailscale/firewall mutation, avoid secret-file reads, avoid
printing endpoints/private ports/topology/tokens/process identities, avoid qsc
send/receive, avoid qsl-server response bodies, and publish only coarse classes.

## NA-0570 Private-Material Policy

NA-0570 repository evidence must not publish endpoint values, private ports,
private topology, route tokens, capabilities, bearer values, Authorization
headers, payloads, response bodies, process identities, authorized_keys content,
public/private key material, secret environment values, raw logs, raw artifacts,
or other private material.

NA-0570 may publish coarse classes, short SHAs, public PR/check identifiers, and
proof-root filenames that do not reveal private material.

## NA-0570 Decision Tree

Potential NA-0570 classifications:

- `QSL_SERVER_RECOVERY_DEPLOYED_AND_LISTENER_READY`
- `QSL_SERVER_RECOVERY_ALREADY_DEPLOYED_AND_LISTENER_READY`
- `QSL_SERVER_RECOVERY_BINARY_STAGED_BUT_START_COMMAND_UNKNOWN`
- `QSL_SERVER_RECOVERY_SOURCE_OR_BUILD_UNAVAILABLE`
- `QSL_SERVER_RECOVERY_NEEDS_OPERATOR_SUDO_SERVICE_ACTION`
- `QSL_SERVER_RECOVERY_NEEDS_SECRET_OR_ENDPOINT_ACTION`
- `QSL_SERVER_RECOVERY_PRIVATE_MATERIAL_STOP`
- `QSL_SERVER_RECOVERY_AMBIGUOUS_STOP`

Decision rules:

- already deployed and listener-ready selects the already-ready classification;
- safe source/build/stage/start/postcheck success selects deployed-and-ready;
- staged binary with unknown safe start command stops with staged-but-unknown;
- unavailable source or build stops for operator staging;
- sudo/systemd/root-owned service need stops for operator action;
- secret, endpoint, or public exposure need stops;
- private-material finding stops before publication;
- ambiguous root cause stops fail-closed.

## Selected Successor

Selected successor:

`NA-0570 -- QSL Remote Relay qsl-server Inspiron Deployment Recovery Harness`

Status: READY after NA-0569 closeout only.

Goals: G1, G2, G3, G4, G5.

NA-0570 is the first lane authorized to inspect and recover qsl-server
deployment or staging on inspiron. NA-0569 does not implement NA-0570.

## Required-Check Boundary

NA-0569 changes no branch-protection settings, check definitions, workflow
files, or required-check policy. Current main required checks were classified
green or conclusively satisfied before mutation, with public-safety and
advisories completed success.

## Dependency / Lockfile Boundary

The dependency recovery scope is limited to `Cargo.lock`.

The root lockfile change is only the `anyhow` package entry:

- version changed from `1.0.100` to `1.0.103`
- checksum changed to the crates.io checksum for `1.0.103`

`Cargo.toml` is unchanged because no root exact vulnerable pin required a
manifest update. `qsl/qsl-client/qsc/fuzz/Cargo.lock` is unchanged because the
nested qsc fuzz audit already passed and continued to pass after remediation.
No unrelated dependency drift was introduced.

## Source / Script Mutation Boundary

NA-0569 changes no qsl-protocol source, repository scripts, workflows, qsc
runtime, qsl-server source, or qsl-attachments source. The only non-governance
change is the bounded root `Cargo.lock` `anyhow` advisory remediation described
above.

## Workflow Mutation Boundary

NA-0569 does not dispatch, rerun, cancel, delete, or modify any workflow.

## Runtime / qsc Boundary

NA-0569 does not execute qsc. It records that qsc is the client/demo CLI and
that future remote-handshake and remote-relay recovery must center on qsl-server
deployment state.

## qsl-server / qsl-attachments Boundary

NA-0569 performs read-only GitHub source review for qsl-server and
qsl-attachments classification. It does not clone, build, run, or mutate either
repository. qsl-server is selected for future deployment recovery; qsl-attachments
is classified out of basic remote-handshake / remote-relay recovery scope.

## Remote-Action Boundary

NA-0569 performs no remote action. It does not run SSH, Tailscale, remote
commands, remote probes, scp, sftp, rsync, sudo, systemctl, service commands,
firewall commands, account changes, shell changes, or authorized_keys changes.

## Public-Site / Cloudflare Boundary

NA-0569 does not change README public-progress content, docs/public content,
website paths, public paths, Cloudflare configuration, or deployment settings.

## Claim Boundary

NA-0569 makes no public-readiness claim. NA-0569 makes no
production-readiness claim. NA-0569 makes no public-internet-readiness claim.
NA-0569 makes no vulnerability-free claim. NA-0569 makes no bug-free claim.
NA-0569 makes no perfect-build claim.

## Validation

Local validation performed:

- `git diff --check`
- exact implementation scope guard
- dependency diff guard
- queue/decision proof
- marker proof
- link-check
- added-line/new-file private-material scan
- overclaim scan
- docs/governance/dependency-recovery classifier
- PR body preflight
- goal-lint when available
- root cargo audit
- nested qsc fuzz lock cargo audit
- `cargo metadata --locked --format-version=1`
- `cargo tree -i anyhow`
- `cargo check --workspace --locked`
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`

Focused qsc runtime tests may be skipped because this lane mutates no qsc
source, runtime, workflow, executable test, fuzz target, or vector. The
dependency recovery is limited to `anyhow`, and cargo audit, cargo metadata,
workspace check, and required CI cover the dependency gate.

## Recommendation

Merge NA-0569 after required checks pass, then run the optional closeout to mark
NA-0569 DONE and restore exactly one READY successor: NA-0570.
