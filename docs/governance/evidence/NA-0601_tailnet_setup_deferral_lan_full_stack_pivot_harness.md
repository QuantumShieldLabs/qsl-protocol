Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-04

# NA-0601 Tailnet Setup Deferral / LAN Full-Stack Pivot Harness

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0601 consumes D532, D-1191, and D-1192, then records the operator decision
to intentionally defer Tailnet/GitHub-runner setup for now and pivot to a
two-machine private-LAN full-stack readiness path.

Result classification:
`TAILNET_OPERATOR_SETUP_DEFERRED_LAN_PIVOT_SELECTED`.

Selected successor:
`NA-0602 -- QSL LAN Full-Stack Reintroduction Readiness Harness`.

No LAN runtime test, remote test, Tailnet action, workflow dispatch, workflow
mutation, GitHub secret/variable mutation, DNS/Cloudflare mutation, deployment
mutation, source mutation, dependency mutation, or lockfile mutation occurred.
Tailnet evidence is preserved as deferred rather than abandoned.

## qwork Proof Verification

Fresh qwork proof files from `/srv/qbuild/work/NA-0601/.qwork/` were copied into
the proof root and parsed with file-backed parsers before fetch, repository
mutation, PR creation, source-analysis result publication, or proof publication.

Verified values included:

- startup_result `OK`
- lane `NA-0601`
- repo `qsl-protocol`
- path `/srv/qbuild/work/NA-0601/qsl-protocol`
- branch `main`
- upstream `origin/main`
- `HEAD`, `origin/main`, and `main` at `7217dd9a6265`
- clean worktree, index, and untracked state
- READY_COUNT 1
- queue top READY `NA-0601`
- proof timestamp at or after `2026-07-04T14:51:04Z`
- shared cargo target mode and shared target ready

Codex did not run qwork, qstart, or qresume.

## D-1191 / D-1192 Inheritance

D532, D-1191, D-1192, NA-0600 evidence, and NA-0600 closeout state were
consumed.

- D-1191 exists once and is Accepted.
- D-1192 exists once and is Accepted.
- NA-0600 is DONE.
- NA-0601 is READY.
- D-1193 was absent before this patch.
- D-1194 was absent before this patch.
- D-1191 result classification was
  `REMOTE_TAILNET_OPERATOR_SETUP_PROOF_STILL_REQUIRED`.
- D-1191 selected NA-0601.
- D-1192 restored NA-0601.
- No remote verification occurred.
- No Tailnet, GitHub secret, workflow, DNS, Cloudflare, public-site, qsl-server
  deployment, or qsl-attachments deployment mutation occurred.

## Authority Model Application

NA-0601 used Tier 0 read-only inheritance review, Tier 1 proof-root classifiers
and redaction matrices, and Tier 2 governance/readiness documentation. It did
not use Tier 3 authority.

No LAN runtime execution, remote/Tailnet diagnostics, SSH, scp, Tailscale
command, workflow dispatch/rerun, GitHub secret or variable mutation,
DNS/Cloudflare/public-site mutation, qsl-server deployment, qsl-attachments
deployment, qsc source/test mutation, qsl-server source/test mutation,
qsl-attachments source/test mutation, dependency mutation, or lockfile mutation
occurred.

## Operator Decision Record

The operator intentionally deferred Tailnet/GitHub-runner setup for now and
selected a two-machine private-LAN full-stack step first.

Reason: the LAN path introduces real client/server and service separation while
preserving physical access, simpler debugging, and no Tailnet, public endpoint,
DNS/Cloudflare, or GitHub-runner complexity.

Classifications:

- `TAILNET_OPERATOR_SETUP_DEFERRED_BY_OPERATOR`
- `LAN_PIVOT_SELECTED`
- `REMOTE_TAILNET_EVIDENCE_PRESERVED_DEFERRED`
- `NO_TAILNET_MUTATION_PERFORMED`
- `NO_LAN_RUNTIME_TEST_PERFORMED`

Remote/Tailnet work should resume after LAN verification/stress unless a later
accepted decision changes the roadmap.

## LAN Topology Classification

| Field | Class |
|---|---|
| client_host_class | `operator_personal_laptop` |
| server_host_class | `build_server_small_desktop` |
| network_class | `private_home_lan_via_cable_modem_wifi_router` |
| qsc_client_host | `operator_laptop_planned` |
| qsl_server_host | `build_server_planned` |
| qsl_attachments_host | `build_server_planned` |
| physical_access_class | `operator_has_physical_access_to_both_or_confirmed_control` |
| remote_access_class | `local_lan_only_planned` |
| public_exposure_class | `none_planned` |
| tailnet_use_class | `deferred_not_used` |
| github_runner_use_class | `deferred_not_used` |
| cloudflare_dns_use_class | `deferred_not_used` |
| raw_ip_publication | `forbidden` |
| private_port_publication | `forbidden` |
| hostname_publication | `forbidden_unless_explicitly_classified_safe` |

## LAN Access Model Matrix

| ID | Approach | Setup owner | Execution owner | Proof quality | Safety risk | Private-material risk | Codex may execute now |
|---|---|---|---|---|---|---|---|
| A | Operator runs commands on laptop and build server, then provides safe summaries | operator | operator | medium-high if structured | low | medium without checklist | no |
| B | Codex controls build-server side only; operator controls laptop commands | operator/Codex split | Codex on build server, operator on laptop | high server, medium laptop | low | low with class-only proof | no |
| C | Codex uses narrow SSH from build server to laptop only if later authorized | operator | Codex only if later authorized | high | medium-high | high without quarantine | no |
| D | Self-hosted LAN runner later | operator | CI runner if later configured | high repeatability | medium | medium-high | no |
| E | Tailnet/GitHub-runner path later | operator | GitHub-hosted runner if later configured | high once configured | medium | medium-high | no |

Each model can be authorized by a later exact directive, but none is executable
by Codex in NA-0601.

## Selected LAN Access Model

Initial model: Codex controls the build-server side only. The operator laptop
remains operator-owned. The operator can run laptop commands manually or provide
class-safe proof in a later lane. Codex must not SSH to the laptop unless a
later exact directive authorizes it.

## Operator / Codex Boundary

Operator-owned responsibilities:

- running commands on the operator laptop
- installing or building qsc on the laptop if needed
- providing class-safe laptop proof
- approving any laptop-side service/process action
- providing private LAN endpoint values only through proof-root-only channels if
  a later lane authorizes it
- ensuring the laptop remains on the private LAN
- confirming no public exposure

Codex-owned responsibilities:

- build-server qwork/repo proof
- build-server qsl-server/qsl-attachments local setup in later lanes if
  authorized
- redacted LAN diagnostic design
- proof-root analysis and governance documentation
- private-material scans and claim-boundary scans

## Redacted LAN Diagnostic Plan

NA-0601 defines diagnostic classes for a future lane only. It does not run LAN
verification.

Required future phase classes:

- client_host_ready: ready / not_ready / operator_required / unknown
- server_host_ready: ready / not_ready / unknown
- qsl_server_bind_class: loopback_only / lan_private_bind /
  public_bind_forbidden / unknown
- qsl_attachments_bind_class: loopback_only / lan_private_bind /
  filesystem_only / public_bind_forbidden / unknown
- lan_dns_or_address_source: operator_provided_private_class / hostname_class /
  absent / unknown
- tcp_connect_qsl_server: success / refused / timeout / not_attempted / unknown
- tcp_connect_qsl_attachments: success / refused / timeout / not_attempted /
  unknown
- qsl_server_route_shape: ready / auth_fail_closed / timeout / not_attempted /
  unknown
- qsl_attachments_shape: ready / auth_fail_closed / timeout / not_attempted /
  unknown
- qsc_lan_handshake: pass / fail_class / not_attempted / unknown
- qsc_lan_relay_e2ee: pass / fail_class / not_attempted / unknown
- qsc_lan_attachment_send_receive: pass / fail_class / not_attempted / unknown
- exact_4mib_lan_boundary: pass / fail_class / not_attempted / unknown
- cleanup: complete / partial / not_attempted / unknown

Published evidence must not include raw LAN IPs, private ports, hostnames,
topology beyond classes, tokens, Authorization values, route tokens,
capabilities, payload/body/plaintext, ciphertext bodies, seed/key material, raw
logs, or raw command lines with private values.

## LAN Readiness Matrix

| Class | Result |
|---|---|
| local single-machine correctness | `pass` |
| local full-stack attachments | `pass` |
| exact 4 MiB boundary | `pass` |
| qsl-server exact 4 MiB fix | `pass` |
| LAN client host | `operator_required` |
| LAN server host | `build_server_ready` |
| LAN private network | `assumed_present` |
| qsl-server LAN bind plan | `ready_to_plan` |
| qsl-attachments LAN bind/storage plan | `ready_to_plan` |
| redaction policy | `ready` |
| first LAN readiness lane | `ready` |

## Security / Metadata / Claim Review

Reviewed risks:

- private LAN endpoint exposure
- private port exposure
- laptop command-line/log leakage
- build-server service logs
- qsl-server route/auth boundary
- qsl-attachments capability boundary
- qsl-attachments storage path exposure
- plaintext/key leakage
- Wi-Fi/LAN trust boundary
- firewall/public-bind risk
- service cleanup
- no Tailnet/public readiness claim
- no production readiness claim
- no metadata-free, anonymity, or untraceability claim

The LAN plan does not require publishing private endpoint, private port,
hostname, token, Authorization, capability, payload/body/plaintext, ciphertext
body, seed, key material, raw log, or private topology values.

## Result Classification

Selected result:
`TAILNET_OPERATOR_SETUP_DEFERRED_LAN_PIVOT_SELECTED`.

This is not a private-material stop and not an ambiguous stop.

## Selected Successor

Selected successor:
`NA-0602 -- QSL LAN Full-Stack Reintroduction Readiness Harness`.

The successor must plan the first two-machine private-LAN full-stack
verification after local single-machine qsc/qsl-server/qsl-attachments success
and the operator decision to defer Tailnet. It must not run LAN verification
unless a later directive explicitly authorizes it. It must not authorize
Tailnet, public endpoint, GitHub runner, DNS, Cloudflare, workflow dispatch,
remote-public exposure, dependency/lockfile mutation, qsc/qsl-server/
qsl-attachments source mutation, private endpoint/port/token publication, or
public/production/security-completion claims.

## Required-Check Boundary

Current main health was classified before mutation:

- public-safety: success
- advisories: success
- suite2-vectors: success
- required contexts: green after D498-style visibility recovery for aggregate
  CodeQL and PR-head-only goal-lint
- visible failed remote/demo check-runs: present but nonrequired for this gate
- root cargo audit: success
- nested qsc fuzz cargo audit: success
- `cargo metadata --locked --format-version=1`: success
- Cargo.toml, Cargo.lock, and qsc fuzz Cargo.lock drift: absent

## Source / Workflow Mutation Boundary

No qsl-protocol source, workflow, runtime, dependency, lockfile, script,
public-site, docs/public, qwork/qstart/qresume, qshield, qshield-cli, formal,
refimpl, backup, qsc source/test, qsl-server, or qsl-attachments path was
mutated.

## qsc Boundary

qsc source/tests/examples/fuzz paths were read-only if needed for inherited
context and were not mutated. No qsc runtime or LAN command execution occurred.

## qsl-server Boundary

qsl-server source, tests, deployment, runtime, service, and remote paths were
not mutated. No qsl-server LAN bind, start, stop, deploy, or diagnostic action
occurred.

## qsl-attachments Boundary

qsl-attachments source, tests, deployment, runtime, service, storage, and
remote paths were not mutated. No qsl-attachments LAN bind, start, stop, deploy,
or diagnostic action occurred.

## Remote / Tailnet Boundary

No remote command, SSH, scp, Tailscale command, Tailnet setup, remote
diagnostic, workflow dispatch, workflow rerun, or GitHub runner Tailnet join was
performed.

## Public-Site / Cloudflare Boundary

No public-site, docs/public, website, DNS, Cloudflare, firewall, public
endpoint, TLS, public service exposure, or deployment mutation occurred.

## Evidence / Decision / Traceability

NA-0601 adds this evidence document, the NA-0601 testplan, D-1193 in
`DECISIONS.md`, a TRACEABILITY row, and a rolling operations journal entry.

## Validation

Planned validation covers git diff check, scope guard, queue/decision proof,
marker proof, link-check, private-material scans, prohibited-material scan,
overclaim scan, LAN/private-topology publication scan, remote/Tailnet
publication scan, claim-boundary scan, docs/governance-only classifier, PR body
preflight, goal-lint if available, cargo audits, locked metadata, cargo fmt, and
qsc-adversarial shell syntax.

Focused runtime tests may be skipped because NA-0601 is pivot/readiness-only,
mutates no qsc/qsl-server/qsl-attachments source or runtime surface, mutates no
workflow/dependency/lockfile surface, and performs no LAN or remote/Tailnet
execution.

## Recommendation

After NA-0601 merges and post-merge checks pass, close out NA-0601 and restore
`NA-0602 -- QSL LAN Full-Stack Reintroduction Readiness Harness` as exactly one
READY successor.
