Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-27

# NA-0550 Relay UI qsl-server Boundary Authorization Plan

## Executive Summary

NA-0550 is accepted as authorization-only evidence. It consumes D-1088 and
D-1089, verifies fresh qwork proof from `2026-06-27T21:49:17Z`, classifies
current main required checks on `93971209e26c`, and defines the relay UI
qsl-server boundary without running qsl-server, qsl-attachments, relay UI
tests, reruns, dispatches, or local reproduction.

Result classification:
`RELAY_UI_QSL_SERVER_BOUNDARY_REMOTE_SMOKE_REMEDIATION_FIRST_READY`.

Selected qsl-server boundary model: GitHub Actions black-box relay UI evidence
only. qsl-server may remain a black-box workflow dependency for the inherited
relay UI failure, and qsl-attachments remains out of scope for the relay UI
failure.

Selected successor: `NA-0551 -- QSL Remote Handshake and Remote Relay Demo
Script Exact Remediation Implementation Harness`. That successor implements
only the exact remote-handshake and remote-relay demo-script paths identified by
NA-0549 while preserving relay-ui-integration qsl-server work as a separate
unresolved topic.

## qwork Proof Verification

The qwork proof files were copied from `/srv/qbuild/work/NA-0550/.qwork/` into
the proof root and parsed from both `.kv` and JSON forms.

- startup_result: `OK`
- lane: `NA-0550`
- primary_repo: `qsl-protocol`
- repo_result: `OK`
- repo path: `/srv/qbuild/work/NA-0550/qsl-protocol`
- branch/upstream: `main` / `origin/main`
- proof HEAD/origin-main/main: `93971209e26c`
- live pre-fetch HEAD/origin-main: `93971209e26c`
- worktree/index/untracked proof: clean
- READY_COUNT proof: 1
- READY proof: NA-0550
- qwork proof timestamp: `2026-06-27T21:49:17Z`
- cargo target mode/source/class: shared / qwork-default / default
- cargo target dir: `/srv/qbuild/cache/targets/qsl-protocol/rustc-1.95.0-x86_64-unknown-linux-gnu/default`
- shared target ready: yes

Codex did not run qwork, qstart, or qresume.

## D-1088 / D-1089 Inheritance

D-1088 exists once and is Accepted. D-1088 accepted the NA-0549 targeted
follow-up evidence and selected
`REMOTE_RELAY_TARGETED_EVIDENCE_QSL_SERVER_BOUNDARY_READY`.

D-1089 exists once and is Accepted. D-1089 accepted NA-0549 closeout, marked
NA-0549 DONE, and restored NA-0550 as the exactly one READY successor without
implementing NA-0550.

No NA-0550 evidence/testplan files existed before this lane. Raw logs and raw
artifacts remain proof-root-only. No private material publication was inherited
or introduced.

## Current Main Required-Check Classification

Current main was verified at `93971209e26c`, equal to origin/main and descending
from `93971209e26c`.

Branch-protection required contexts were classified:

- `ci-4a`: `REQUIRED_GREEN`
- `ci-4b`: `REQUIRED_GREEN`
- `ci-4c`: `REQUIRED_GREEN`
- `ci-4d`: `REQUIRED_GREEN`
- `ci-4d-dur`: `REQUIRED_GREEN`
- `demo-cli-build`: `REQUIRED_GREEN`
- `demo-cli-smoke`: `REQUIRED_GREEN`
- `formal-scka-model`: `REQUIRED_GREEN`
- `goal-lint`: `REQUIRED_GREEN` using associated PR #1372 head proof
- `metadata-conformance-smoke`: `REQUIRED_GREEN`
- `suite2-vectors`: `REQUIRED_GREEN`
- `CodeQL`: `REQUIRED_GREEN` using associated PR #1372 aggregate proof
- `macos-qsc-qshield-build`: `REQUIRED_GREEN`
- `public-safety`: `REQUIRED_GREEN`

public-safety completed success. advisories completed success. No failed
required check was observed.

Non-required target status on current main:

- remote-handshake: `NON_REQUIRED_ABSENT_ON_CURRENT_MAIN`
- remote-relay: `NON_REQUIRED_ABSENT_ON_CURRENT_MAIN`
- relay-ui-integration: `NON_REQUIRED_ABSENT_ON_CURRENT_MAIN`
- qsc-adversarial-smoke: `NON_REQUIRED_SUCCESS`

## Relay UI Failure Inheritance

NA-0549 inherited and correlated the relay-ui-integration failure as:

- evidence gap: `RELAY_UI_GAP_QSL_SERVER_BOUNDARY_REQUIRED`
- implementation readiness:
  `IMPLEMENTATION_NOT_READY_QSL_SERVER_BOUNDARY_REQUIRED`
- qsl-server boundary:
  `QSL_SERVER_BOUNDARY_SEPARATE_AUTHORIZATION_REQUIRED`
- qsl-attachments: did not appear in target failure evidence
- bounded failure: loopback health HTTP 404 after TCP bind proof and before
  ignored tests ran

remote-handshake and remote-relay remain exact implementation-ready:

- remote-handshake future exact path:
  `scripts/demo/qsc_remote_handshake_smoke.sh`
- remote-relay future exact path:
  `scripts/demo/qsc_remote_relay_smoke.sh`

## Relay UI Workflow Boundary Review

The relay-ui-integration workflow was inspected read-only.

- Workflow name: relay-ui-integration
- Triggers: workflow_dispatch and daily schedule
- Job: relay-ui-integration
- OS image: ubuntu-latest
- Checkout: qsl-protocol only through actions/checkout
- qsl-server handling: cloned with depth 1 into runner temporary storage,
  outside the qsl-protocol workspace
- qsl-server ref: no explicit branch/ref is pinned in the workflow
- qsl-server build: release build with locked dependencies
- qsl-server run: runner temporary release binary, loopback bind, configured
  relay UI port
- wait/readiness logic: bounded TCP bind probe followed by v1-path HTTP health
  probes
- ignored test invocation: qsc relay UI integration test with ignored tests and
  nocapture
- artifacts: relay server log and relay UI integration log
- secrets/env: a random runtime bearer token is generated and masked; no
  repository secret use is visible
- network scope: loopback runtime after the GitHub clone/build setup

The inherited HTTP 404 occurred after TCP bind proof and before ignored qsc tests
ran. That makes pure TCP readiness less likely than workflow health route
expectation mismatch or qsl-server black-box route behavior mismatch. qsl-server
source behavior was not inspected and no qsl-server command was executed.

## Relay UI Test/Common Boundary Review

The qsc relay UI integration test file and shared common test module were
inspected read-only.

- `relay_ui_integration.rs` requires `QSC_RELAY_UI_URL` and
  `QSC_RELAY_UI_TOKEN`.
- The test passes the token into qsc as `QSC_RELAY_TOKEN`.
- The test invokes qsc with relay transport and the workflow-provided relay URL.
- The test file does not define its own qsl-server health path.
- The test assumes the workflow has already provided a live relay.
- Local reproduction of the workflow would require qsl-server clone/build/run,
  which NA-0550 did not authorize or execute.

`qsl/qsl-client/qsc/tests/common/mod.rs` contains unrelated qsl-attachments
test helpers, but `relay_ui_integration.rs` does not invoke those helpers and
the inherited failure occurs before relay UI tests run.

## qsl-server Read-Only Metadata Review

qsl-server repository metadata was visible through read-only GitHub API calls.

- repository: `QuantumShieldLabs/qsl-server`
- default branch: main
- visibility: public metadata visible
- workflow metadata: active `ci` and `release-linux` workflows
- recent metadata: latest visible main `ci` run completed success

No qsl-server source tree was fetched beyond shallow metadata. No qsl-server
clone, build, run, command, or mutation occurred.

The metadata supports treating qsl-server as a black-box GitHub Actions
dependency for NA-0550. It does not support a qsl-server implementation
successor or local reproduction successor now.

## qsl-attachments Read-Only Metadata Review

qsl-attachments repository metadata was visible through read-only GitHub API
calls.

- repository: `QuantumShieldLabs/qsl-attachments`
- default branch: main
- visibility: public metadata visible
- workflow metadata: active `rust` workflow
- recent metadata: latest visible main `rust` run completed success

qsl-attachments is absent from the relay-ui-integration workflow and from the
inherited target failure. The shared qsc test common module has unrelated
attachment helpers, but those helpers are not the relay UI workflow boundary and
the failure occurred before tests ran.

qsl-attachments remains out of scope.

## qsl-server Boundary Options Review

Option 1 -- GitHub Actions black-box relay UI evidence only:
selected for the relay UI boundary. Existing relay-ui-integration behavior is
enough to keep qsl-server as black-box workflow context.

Option 2 -- qsl-protocol workflow/test remediation without qsl-server source use:
deferred. The visible qsl-protocol workflow health probe is a candidate future
surface, but relay UI remediation is not selected as the immediate successor.

Option 3 -- qsl-server read-only source boundary authorization:
not required before the selected successor. It may be reconsidered only if relay
UI becomes the immediate priority and black-box evidence is insufficient.

Option 4 -- qsl-server local reproduction authorization:
rejected for now. Local reproduction would require qsl-server clone/build/run
and is not needed before exact remote smoke remediation.

Option 5 -- qsl-server implementation successor:
rejected. No exact qsl-server mutation path is supported.

Option 6 -- qsl-attachments boundary authorization:
rejected. qsl-attachments did not appear in the workflow or target failure
evidence.

## Remote Handshake / Remote Relay Sequencing Review

Sequencing classification:
`REMOTE_SMOKE_REMEDIATION_CAN_PROCEED_INDEPENDENTLY`.

The two exact demo-script paths do not clone, build, run, or mutate qsl-server
or qsl-attachments. They rely on a remote relay URL/token and qsc commands.
Fixing them first can reduce non-required CI noise without touching relay UI
workflow/qsl-server boundary work.

Combining remote-handshake/remote-relay script remediation with relay UI
qsl-server boundary work would widen scope unnecessarily.

## Selected qsl-server Boundary Model

Selected model: GitHub Actions black-box relay UI evidence only.

NA-0550 authorizes no qsl-server source inspection beyond read-only metadata,
no qsl-server local reproduction, and no qsl-server source mutation. A later
relay UI lane may be selected only with exact path and command boundaries.

## Result Classification

`RELAY_UI_QSL_SERVER_BOUNDARY_REMOTE_SMOKE_REMEDIATION_FIRST_READY`

Success requirements were satisfied:

- fresh qwork proof verified
- current main required checks classified
- public-safety success
- advisories success
- no failed required checks
- D-1088 and D-1089 consumed
- relay UI workflow boundary reviewed
- qsl-protocol relay UI test/common references reviewed
- qsl-server metadata reviewed
- qsl-attachments metadata reviewed and non-involvement recorded
- qsl-server boundary model selected
- remote-handshake/remote-relay sequencing reviewed
- exact successor selected

## Selected Successor

### NA-0551 -- QSL Remote Handshake and Remote Relay Demo Script Exact Remediation Implementation Harness
Status: READY
Goals: G1, G2, G3, G4, G5

Objective:
Implement exact remediation for the remote-handshake and remote-relay
non-required CI failures using the NA-0549 evidence that identified exact
implementation-ready demo script paths. Mutate only the authorized demo scripts,
tests/evidence/governance files, and traceability/journal files. Preserve the
relay-ui-integration qsl-server boundary as a separate unresolved successor
topic. Do not mutate workflows, qsc source, dependencies, lockfiles,
qsl-server, qsl-attachments, public-site, or Cloudflare paths.

Allowed scope:
- scripts/demo/qsc_remote_handshake_smoke.sh
- scripts/demo/qsc_remote_relay_smoke.sh
- docs/governance/evidence/NA-0551_remote_handshake_remote_relay_demo_script_exact_remediation_implementation_harness.md
- tests/NA-0551_remote_handshake_remote_relay_demo_script_exact_remediation_testplan.md
- DECISIONS.md
- TRACEABILITY.md
- docs/ops/ROLLING_OPERATIONS_JOURNAL.md
- proof-root-only logs and validation artifacts
- successor selection

Forbidden scope:
- relay-ui-integration remediation;
- qsl-server/qsl-attachments command, clone, build, run, source mutation, or
  local use;
- workflow mutation;
- qsc source/test/fuzz/Cargo mutation;
- dependency/lockfile mutation;
- public-site mutation;
- Cloudflare mutation;
- qwork/qstart/qresume execution by Codex;
- qsl-backup execution;
- backup mutation;
- remote command outside explicitly authorized GitHub check/log reads;
- public-readiness, production-readiness, public-internet-readiness,
  external-review-complete, reproducibility-complete, backup/restore-complete,
  vulnerability-free, bug-free, or perfect-build claim.

## Required-Check Boundary

NA-0550 performed read-only current-main check classification only. No rerun or
workflow dispatch was executed.

## Workflow Mutation Boundary

No workflow file was mutated. The relay UI workflow was inspected read-only.

## Runtime / qsc / Dependency Boundary

No runtime/source path was mutated. No qsc source, test, fuzz, or Cargo path was
mutated. No dependency or lockfile was mutated.

## qsl-server / qsl-attachments Boundary

No qsl-server command was executed. No qsl-server clone/build/run occurred. No
qsl-server mutation occurred.

No qsl-attachments command was executed. No qsl-attachments clone/build/run
occurred. No qsl-attachments mutation occurred.

## Remote-Action Boundary

Read-only GitHub API metadata was used. No remote command outside read-only
GitHub API/file access occurred.

## Public-Site / Cloudflare Boundary

No public-site content, README public-progress content, docs/public content,
website path, public path, Cloudflare configuration, or deployment setting was
mutated.

## Private-Material Boundary

No raw logs or raw artifacts were copied into repository docs. No private
material was published.

## Claim Boundary

No public-readiness claim was made. No production-readiness claim was made. No
public-internet-readiness claim was made. No external-review-complete claim was
made. No reproducibility-complete claim was made. No backup/restore-complete
claim was made. No vulnerability-free claim was made. No bug-free claim was
made. No perfect-build or perfect-crypto claim was made.

## Validation

Validation is recorded in the NA-0550 testplan and proof root. Focused qsc
runtime tests are skipped because NA-0550 is authorization-only, no local qsc
runtime reproduction is authorized, no qsc source/runtime/dependency/workflow
mutation occurred, and no qsl-server/qsl-attachments execution occurred.

## Recommendation

Merge NA-0550 as the boundary authorization record, then close it out to
NA-0551 only after required checks, public-safety, and advisories are green.
NA-0551 should remediate only the two exact remote demo scripts and preserve the
relay UI qsl-server topic for a later exact successor.
