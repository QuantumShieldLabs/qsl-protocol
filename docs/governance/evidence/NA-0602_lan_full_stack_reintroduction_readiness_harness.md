Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-04

# NA-0602 LAN Full-Stack Reintroduction Readiness Harness

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0602 consumes D533, D-1193, and D-1194 and selects the first private-LAN
runtime lane. The selected first step is intentionally tiny: one E2EE qsc
message through qsl-server over a private home LAN, with the operator laptop
running two logical qsc client states and the build server running qsl-server as
relay/control-plane only.

Result classification:
`LAN_FULL_STACK_REINTRODUCTION_READINESS_SELECTED`.

Selected successor:
`NA-0603 -- QSL LAN Minimal qsc E2EE Relay Verification Harness`.

No LAN runtime test, qsc LAN command, qsl-server startup, qsl-attachments
runtime, Codex SSH to laptop, laptop SSH server setup, second Codex on laptop,
remote/Tailnet action, workflow dispatch/rerun, GitHub secret/variable
mutation, DNS/Cloudflare mutation, public endpoint, source mutation,
dependency mutation, or lockfile mutation occurred.

## qwork Proof Verification

Fresh qwork proof files from `/srv/qbuild/work/NA-0602/.qwork/` were copied into
the proof root and parsed with file-backed parsers before fetch, repository
mutation, GitHub metadata review, source-analysis result publication, PR
creation, or proof publication.

Verified values included:

- startup_result `OK`
- lane `NA-0602`
- repo `qsl-protocol`
- path `/srv/qbuild/work/NA-0602/qsl-protocol`
- branch `main`
- upstream `origin/main`
- `HEAD`, `origin/main`, and `main` at `f766a53a4cde`
- clean worktree, index, and untracked state
- READY_COUNT 1
- queue top READY `NA-0602`
- proof timestamp `2026-07-04T16:52:17Z`
- shared cargo target mode and shared target ready

Pre-fetch live `HEAD` and `origin/main` matched the qwork proof. Root disk usage
was below the stop threshold, and `/backup/qsl` was mounted. Codex did not run
qwork, qstart, or qresume.

Recovered proof-root tooling issues were recorded before fetch or mutation: an
initial proof parser set/list bug, a cargo env key mapping overconstraint, a
D498-style main-check visibility mapping issue for aggregate CodeQL and
PR-head-only goal-lint, and an optional absent qsc examples path discovery.

## D-1193 / D-1194 Inheritance

D533, D-1193, D-1194, NA-0601 evidence/testplan, and the current NA-0602 queue
block were consumed.

- D-1193 exists once and is Accepted.
- D-1194 exists once and is Accepted.
- NA-0601 is DONE.
- NA-0602 is READY.
- D-1195 was absent before this patch.
- D-1196 was absent before this patch.
- D-1193 result classification was
  `TAILNET_OPERATOR_SETUP_DEFERRED_LAN_PIVOT_SELECTED`.
- D-1193 selected NA-0602.
- D-1194 restored NA-0602.
- No LAN runtime test occurred.
- No Tailnet, GitHub-runner, workflow, DNS, Cloudflare, public-site, qsl-server
  deployment, or qsl-attachments deployment mutation occurred.

## Authority Model Application

NA-0602 used Tier 0 read-only review, Tier 1 proof-root planning tooling, and
Tier 2 governance/readiness documentation. It did not use Tier 3 authority.

No laptop command, qsc LAN command, qsl-server start, qsl-attachments start,
remote diagnostic, SSH, scp, Tailscale command, workflow dispatch/rerun, GitHub
secret or variable mutation, DNS/Cloudflare/public-site mutation, qsl-server
deployment, qsl-attachments deployment, qsc source/test mutation, qsl-server
source/test mutation, qsl-attachments source/test mutation, dependency
mutation, or lockfile mutation occurred.

## LAN Role and Topology Readiness

| Field | Class |
|---|---|
| client_host_class | `operator_personal_laptop` |
| server_host_class | `build_server_small_desktop` |
| network_class | `private_home_lan` |
| first_runtime_client_model | `laptop_two_qsc_logical_clients_preferred` |
| first_runtime_server_model | `build_server_qsl_server_only` |
| qsl_attachments_first_runtime | `deferred` |
| laptop_codex_control | `not_authorized` |
| laptop_ssh_server | `not_required_initially` |
| second_laptop | `later_optional` |
| public_exposure | `none_planned` |
| tailnet | `deferred` |
| github_runner | `deferred` |

This topology keeps the first runtime lane as small as possible: one private-LAN
relay/control-plane service on the build server and two logical qsc client
states on the operator laptop.

## Operator Laptop Readiness Plan

| Field | Class |
|---|---|
| qsc binary/source availability | `unknown` |
| operator can run qsc command | `unknown` |
| laptop on private LAN | `unknown` |
| laptop can reach build-server service class | `future_test_required` |
| laptop proof-root or local temp directory class | `planned` |
| laptop raw command output handling | `proof_root_or_operator_paste_class_only` |
| laptop cleanup responsibility | `operator_owned` |
| SSH server | `not_required_initially` |
| second Codex | `not_required_initially` |

NA-0603 should have the operator confirm qsc availability, command ability,
private-LAN presence, two separate qsc state directories, and laptop cleanup.
The operator must keep raw private endpoint, port, hostname, route/auth,
capability, payload/body/plaintext, ciphertext body, seed, and key values out of
chat and publish only class-safe summaries unless a later proof-root transfer
method is explicitly authorized.

## Build-Server Readiness Plan

| Field | Class |
|---|---|
| qsl-server role | `relay_control_plane` |
| qsl-server initial bind class | `private_lan_bind_or_loopback_plus_forwarding_to_be_selected_later` |
| qsl-server public bind | `forbidden` |
| qsl-server route/auth token policy | `non_secret_fixture_for_lan_tests` |
| qsl-server logs | `proof_root_only` |
| qsl-server cleanup | `required` |
| qsl-attachments initial role | `deferred_until_after_tiny_message` |
| qsl-attachments LAN bind/storage | `planned_later` |
| qsl-attachments public bind | `forbidden` |
| qsl-attachments capability policy | `non_secret_fixture_for_later_lan_attachment_tests` |
| proof-root storage | `required` |
| service ownership | `Codex_owned_on_build_server_only_when_authorized` |

No service was started in NA-0602. NA-0603 may authorize exact build-server
qsl-server setup, proof-root-only logs, and cleanup. qsl-attachments remains
out of the first tiny-message lane.

## LAN Access Model Matrix

| ID | Model | Proof quality | Safety risk | Private-material risk | Complexity | Required authorization | Result |
|---|---|---|---|---|---|---|---|
| A | Operator runs laptop commands; Codex controls build-server services | high server, medium laptop | low | low with class-only proof | low-medium | NA-0603 exact build-server service authority plus operator laptop action | selected |
| B | Operator runs commands on both hosts | medium | low | medium | medium | operator-only runtime directive | deferred |
| C | Codex SSH to laptop | high | medium-high | high | high | later exact laptop SSH authority | rejected for initial lane |
| D | Second Codex on laptop | high | medium-high | high | high | later exact multi-agent/laptop authority | rejected for initial lane |
| E | Self-hosted LAN runner | high repeatability | medium | medium-high | high | later LAN runner setup authority | deferred |
| F | Tailnet/GitHub-runner | high after setup | medium | medium-high | high | later Tailnet/GitHub runner authority | deferred |

## Selected LAN Access Model

Model A is selected for NA-0603:

- operator runs qsc commands on the laptop
- Codex controls only the build-server qsl-server service, if NA-0603
  authorizes it
- qsl-attachments is not used
- Codex does not SSH to the laptop
- no laptop SSH server setup is required
- no second Codex is required

## NA-0603 Operator / Codex Boundary

Codex-owned in NA-0603, if authorized:

- verify fresh qwork proof
- start qsl-server on the build server with a private-LAN-safe bind class
- use a non-secret route/auth fixture
- capture proof-root-only qsl-server logs
- provide exact laptop command templates with placeholders
- parse operator-provided class-safe proof
- clean up the build-server qsl-server process
- record governance evidence

Operator-owned in NA-0603:

- run qsc commands on the laptop
- keep laptop state under operator control
- keep raw private values out of chat
- provide class-safe output or an explicitly authorized proof-root transfer
  method
- confirm laptop cleanup
- confirm no public exposure

Forbidden in NA-0603 unless a later directive explicitly authorizes it:

- Codex SSH to laptop
- laptop SSH server setup
- second Codex on laptop
- qsl-attachments runtime
- remote/Tailnet action
- GitHub workflow dispatch/rerun
- public endpoint
- qsc source mutation
- qsl-server source mutation
- dependency or lockfile mutation

## Redacted LAN Tiny-Message Diagnostic Plan

Required NA-0603 phase classes:

| Phase | Allowed classes |
|---|---|
| build_server_qsl_server_start | ready / fail_class / not_attempted / unknown |
| qsl_server_bind_class | private_lan_bind / loopback_only_not_sufficient / public_bind_forbidden / unknown |
| laptop_qsc_sender_ready | ready / operator_required / unknown |
| laptop_qsc_receiver_ready | ready / operator_required / unknown |
| laptop_to_build_server_tcp_connect | success / refused / timeout / unknown |
| qsl_server_route_shape | ready / auth_fail_closed / timeout / unknown |
| qsc_lan_handshake_or_state_setup | pass / fail_class / unknown |
| qsc_lan_tiny_send | pass / fail_class / unknown |
| qsc_lan_tiny_receive_decrypt_validate | pass / fail_class / unknown |
| qsl_server_plaintext_exposure | no / yes / unknown |
| seed_fallback_use | no / yes / unknown |
| cleanup | complete / partial / unknown |

Required proof fields:

- message delivered class
- receiver decrypt/validate class
- relay plaintext exposure class
- seed fallback class
- metadata exposure class
- cleanup class

NA-0603 command templates must use placeholders for private endpoint, bind,
state-directory, route/auth fixture, and tiny synthetic message values. The
first runtime path must not enable seed fallback.

## LAN Readiness Matrix

| Readiness item | Result |
|---|---|
| local single-machine qsc/qsl-server evidence | pass |
| local qsl-attachments evidence | pass |
| exact 4 MiB qsl-server fix | pass |
| qsc seed fallback hardening | pass |
| laptop qsc readiness | operator_required |
| build-server qsl-server readiness | ready_to_authorize |
| qsl-attachments first LAN lane | deferred |
| private LAN | assumed_present |
| redaction policy | ready |
| first LAN runtime lane | ready |

The first runtime lane is ready to select because the remaining laptop proof is
appropriately operator-owned in NA-0603 and does not block a class-only, tiny
message relay verification plan.

## Security / Metadata / Claim Review

Reviewed risks:

- private LAN endpoint exposure
- private port exposure
- laptop command-line/log leakage
- build-server qsl-server logs
- route/auth fixture leakage
- qsc local state leakage on laptop
- plaintext/key leakage
- qsl-server public-bind risk
- Wi-Fi/LAN trust boundary
- proof transfer risk
- cleanup responsibilities
- no public-readiness claim
- no production-readiness claim
- no remote-ready claim
- no Tailnet-ready claim
- no LAN-ready overclaim
- no metadata-free, anonymity, or untraceability claim

The plan publishes only classes. It does not require publishing raw LAN IPs,
private ports, hostnames, topology details beyond classes, tokens,
Authorization values, route tokens, capabilities, payload/body/plaintext,
ciphertext bodies, seeds, key material, raw logs, or raw artifacts.

## Result Classification

Selected result:
`LAN_FULL_STACK_REINTRODUCTION_READINESS_SELECTED`.

Rationale: LAN topology, operator/Codex boundary, first runtime access model,
redacted tiny-message diagnostic plan, security/metadata/claim review, and the
readiness matrix are sufficient to select NA-0603. No private-material or
ambiguous stop condition applies.

## Selected Successor

`NA-0603 -- QSL LAN Minimal qsc E2EE Relay Verification Harness`

Status: READY

Goals: G1, G2, G3, G4, G5

Objective:
Run the first private-LAN tiny-message qsc E2EE relay verification. Use the
operator laptop as the qsc sender and receiver logical-client host, with the
operator running exact laptop-side commands manually. Use the build server as
qsl-server relay/control-plane host, with Codex authorized only for exact
build-server service setup, proof-root logging, and cleanup. qsl-attachments is
deferred. Publish only class summaries and do not publish private values.

## Required-Check Boundary

Current main before mutation was classified green:

- public-safety success
- advisories success
- suite2-vectors success
- no failed attached check-runs
- no pending attached check-runs
- required contexts classified green with D498-style visibility recovery for
  aggregate CodeQL and PR-head-only goal-lint

Root cargo audit passed. Nested qsc fuzz cargo audit passed. Locked cargo
metadata passed. Cargo manifest and lockfile drift was absent.

## Source / Workflow Mutation Boundary

This implementation mutates only NA-0602 governance/readiness files:

- this evidence document
- `tests/NA-0602_lan_full_stack_reintroduction_readiness_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No source, workflow, runtime, dependency, lockfile, public-site, deployment,
formal, refimpl, backup, qwork/qstart/qresume, qshield, or qshield-cli path is
mutated.

## qsc Boundary

qsc source/tests/fuzz/examples were not mutated. No qsc LAN command was run.
Read-only qsc source/test/runbook review showed the relevant shape for the
future lane: separate client state roots, relay-backed handshake, relay send,
relay receive/decrypt/validate, route/auth fail-closed behavior, and hardened
seed-fallback boundaries.

## qsl-server Boundary

qsl-server source/tests/deployment were not mutated. qsl-server was not started.
NA-0603 may authorize build-server qsl-server startup only as
relay/control-plane with proof-root-only logs and cleanup.

## qsl-attachments Boundary

qsl-attachments source/tests/deployment were not mutated. qsl-attachments was
not started and is deferred until after tiny-message LAN relay verification.

## Remote / Tailnet Boundary

No remote command, SSH, scp, Tailscale command, Tailnet setup, GitHub runner
setup, workflow dispatch/rerun, GitHub secret/variable mutation, remote
diagnostic, DNS/Cloudflare action, public exposure, or deployment mutation
occurred. Tailnet/GitHub-runner work remains deferred.

## Public-Site / Cloudflare Boundary

No public-site, docs/public, website, DNS, Cloudflare, public endpoint, TLS,
firewall, public exposure, or deployment mutation occurred.

## Evidence / Decision / Traceability

- D-1195 records NA-0602 LAN full-stack reintroduction readiness.
- TRACEABILITY maps NA-0602 readiness selection and NA-0603 successor selection.
- The rolling operations journal records proof gates, recoveries, readiness
  plan summaries, result classification, selected successor, validation, and
  boundaries.

## Validation

Validation plan covers:

- `git diff --check`
- exact scope guard
- queue/decision proof
- marker proof
- link-check
- added-line/new-file private-material scan
- secret/prohibited-material scan
- overclaim scan
- LAN/private-topology publication scan
- remote/Tailnet/private-topology publication scan
- crypto/triple-ratchet/attachment/remote-readiness/LAN-readiness
  claim-boundary scan
- docs/governance-only classifier
- PR body preflight
- goal-lint if available
- root cargo audit
- nested qsc fuzz cargo audit
- `cargo metadata --locked --format-version=1`
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`

Focused runtime tests are skipped because NA-0602 is readiness-only, mutates no
qsc/qsl-server/qsl-attachments source or runtime behavior, mutates no workflows
or dependencies, and performs no LAN or remote/Tailnet execution.

## Recommendation

Proceed to NA-0603 as the tiny-message qsc E2EE relay verification lane. Keep
the operator laptop operator-owned, keep Codex on the build-server side only,
publish only classes, avoid seed fallback, and defer qsl-attachments until the
tiny-message relay path is proven.
