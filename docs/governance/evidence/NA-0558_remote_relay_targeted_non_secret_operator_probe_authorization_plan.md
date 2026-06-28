Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-28

# NA-0558 Remote Relay Targeted Non-Secret Operator Probe Authorization Plan

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0558 is authorization-only. It consumes D-1104 and D-1105, verifies fresh
qwork proof from `2026-06-28T17:33:39Z`, classifies current main required
checks, and selects a future one-step-at-a-time operator-run non-secret probe
package for the remote relay timeout boundary.

Result classification:
`REMOTE_RELAY_TARGETED_NON_SECRET_OPERATOR_PROBE_CAPTURE_READY`.

Selected successor:
`NA-0559 -- QSL Remote Relay Targeted Non-Secret Operator Probe Capture
Harness`.

No probe was executed. No SSH, Tailscale, remote command, workflow dispatch,
rerun, local reproduction, qsc send/receive, qsc E2EE, source mutation, script
mutation, workflow mutation, dependency or lockfile mutation, qsl-server or
qsl-attachments action, public-site mutation, Cloudflare mutation, secret value
request, or private-material publication occurred.

## qwork Proof Verification

Codex copied the qwork proof files from the NA-0558 lane workspace into the
proof root and parsed both the `.kv` and JSON files using a file-backed parser.
The parser verified:

- lane `NA-0558`;
- repo `qsl-protocol`;
- branch `main`;
- path `/srv/qbuild/work/NA-0558/qsl-protocol`;
- startup HEAD, origin/main, and main all at
  `d7363ac6d9c5119c11ef9df86bcc395d87078025`;
- clean worktree, index, and untracked proof;
- READY_COUNT 1 with queue top READY `NA-0558`;
- qwork version
  `175b0ea1d5b9abc07bdab66e9b92446e2a3d533018468e94a95c26f8698f86cf`;
- proof timestamp `2026-06-28T17:33:39Z`;
- shared Cargo target mode and expected shared target directory.

Codex did not run qwork, qstart, or qresume.

## D-1104 / D-1105 Inheritance

D-1104 exists once, is Accepted, and records the NA-0557 operator proof result
`REMOTE_RELAY_OPERATOR_PROOF_TARGETED_NON_SECRET_PROBE_AUTH_READY`.

D-1105 exists once, is Accepted, marks NA-0557 DONE, and restores NA-0558 as the
exactly one READY successor. D-1105 also records that no NA-0558 implementation
occurred during closeout.

Inherited boundaries remain binding: no secret values, route-token/capability
values, bearer values, Authorization headers, private endpoints, private
topology, payloads, response bodies, or secret environment values were requested
or published.

## Current Main Required-Check Classification

Current main was verified at
`d7363ac6d9c5119c11ef9df86bcc395d87078025`, equal to origin/main.

GitHub REST metadata classified current main as healthy:

- public-safety completed success;
- advisories completed success;
- suite2-vectors completed success;
- no failed required checks;
- branch-protection required contexts classified;
- PR-head proof satisfied PR-only `goal-lint` and aggregate `CodeQL`
  contexts;
- no `Cargo.toml`, root `Cargo.lock`, or qsc fuzz `Cargo.lock` drift;
- root disk usage below the stop threshold;
- `/backup/qsl` mounted.

## Prior Evidence and Gap Review

NA-0555 diagnostic evidence still governs this boundary:

- remote-handshake result:
  `REMOTE_HANDSHAKE_DIAGNOSTIC_NETWORK_TLS_TIMEOUT`;
- remote-relay result: `REMOTE_RELAY_DIAGNOSTIC_NETWORK_TLS_TIMEOUT`;
- both reached endpoint label `relay_push_v1`;
- both emitted qsc error `relay_inbox_push_failed`;
- route-token header present true for both;
- bearer auth present true for both;
- HTTP status/body unknown for both.

NA-0557 operator proof was safe but insufficient:

- endpoint configuration `ENDPOINT_CONFIGURED_UNKNOWN`;
- endpoint host publication `ENDPOINT_PRIVATE_NOT_REPORTED`;
- DNS `DNS_RESOLUTION_NOT_CHECKED`;
- TCP `TCP_CONNECT_NOT_CHECKED`;
- TLS `TLS_HANDSHAKE_NOT_CHECKED`;
- service health `RELAY_SERVICE_HEALTH_NOT_CHECKED`;
- auth/route configuration `AUTH_ROUTE_CONFIGURED_UNKNOWN`;
- runner-specific proof `GITHUB_RUNNER_PROOF_NOT_PERFORMED`.

The evidence does not prove script ownership, qsc runtime ownership, remote API
shape mismatch, or environment/secret correctness.

## Targeted Non-Secret Probe Design

The future package must be one-step-at-a-time and operator-run only. The
operator must review every raw output before sharing. Codex must review only the
provided non-secret proof.

Required proof classes:

- endpoint configuration presence;
- endpoint label only;
- endpoint host public-disclosure flag;
- DNS resolution class;
- TCP connect class;
- TLS handshake class;
- service health class;
- secret presence class;
- secret age/coarse rotation class;
- route-token/capability presence class;
- bearer/auth presence class;
- GitHub Actions runner specificity;
- operator-local versus remote/runner environment distinction;
- raw-output private-material review;
- operator no-secret assertion.

Allowed probe-origin classifications are `OPERATOR_LOCAL_HOST`,
`OPERATOR_SELECTED_REMOTE_HOST`, `GITHUB_ACTIONS_RUNNER`,
`REMOTE_SERVICE_OWNER`, and `UNKNOWN_OR_NOT_PERFORMED`.

Local/operator-host success does not prove GitHub-hosted runner success.
Operator-selected remote-host success does not prove GitHub-hosted runner
success unless that host is the actual runner context. GitHub Actions evidence
requires a later exact workflow/dispatch lane. Service-owner proof may be needed
if operator probes cannot access the relevant boundary.

## Exact Future Operator Command Design

The future Director-provided commands must create these proof files:

- `00_probe_manifest.kv`;
- `01_endpoint_config_presence.json`;
- `02_dns_tcp_tls_probe.json`;
- `03_service_health_probe.json`;
- `04_secret_presence_metadata.json`;
- `05_probe_private_material_scan.json`;
- `06_safe_to_paste_summary.json`.

Command outputs must be structured JSON or key-value summaries, redacted by
construction, and must not echo endpoint host, token, bearer, Authorization
header, route capability, payload, response body, or private topology.

Future command shapes:

1. `00_probe_manifest.kv`: operator writes only proof timestamp, non-secret
   operator label, non-secret host label, probe origin enum, target label, and
   no-secret assertion.
2. `01_endpoint_config_presence.json`: Python stdlib reads operator-local
   endpoint input and emits only yes/no/unknown endpoint configuration, endpoint
   label, endpoint host public-disclosure flag, and
   `endpoint_value_disclosed: no`.
3. `02_dns_tcp_tls_probe.json`: Python stdlib internally performs DNS,
   socket, and TLS checks and emits only DNS/TCP/TLS classes. It must not print
   hostnames, IP addresses, ports, certificate subjects, SANs, peer details,
   exception text, or endpoint values.
4. `03_service_health_probe.json`: Python stdlib performs only a non-mutating
   HEAD/health check when the operator marks the health endpoint safe. It emits
   only a health class and `health_body_disclosed: no`.
5. `04_secret_presence_metadata.json`: operator or GitHub metadata review emits
   only yes/no/unknown and coarse age classes for bearer and route material
   without accessing or printing secret values.
6. `05_probe_private_material_scan.json`: local scanner emits only
   `raw_output_contains_private_material` and `redaction_review`.
7. `06_safe_to_paste_summary.json`: validator combines prior files and refuses
   to emit a shareable summary unless every disclosure flag is safe and the
   operator asserts no secret values are included.

Forbidden unless a separate later lane explicitly authorizes them: qsc
send/receive, qsc E2EE, POST payload push, workflow dispatch, remote service
mutation, secret value printing, endpoint host printing, and private topology
printing.

## Future Proof Schema

The future proof-capture lane must require all six proof files and all safe
output fields selected by NA-0558. It must reject or stop if:

- `endpoint_value_disclosed` is not `no`;
- `bearer_secret_value_disclosed` is not `no`;
- `route_secret_value_disclosed` is not `no`;
- `health_body_disclosed` is not `no`;
- `dns_details_disclosed` is not `no`;
- `tcp_endpoint_disclosed` is not `no`;
- `tls_peer_details_disclosed` is not `no`;
- `raw_output_contains_private_material` is `yes`;
- `redaction_review` is not `pass`;
- `operator_asserts_no_secret_values_included` is not `yes`.

Allowed class enums are coarse only. DNS may be `resolved`, `unresolved`,
`timeout`, `not_checked`, `private_not_reported`, or `error`. TCP may be
`success`, `timeout`, `refused`, `not_checked`, `private_not_reported`, or
`error`. TLS may be `success`, `timeout`, `failure`, `not_checked`,
`private_not_reported`, or `error`. Health may be `success`, `not_found`,
`timeout`, `not_available`, `not_checked`, `private_not_reported`, or `error`.

## Future Private-Material Policy

The future lane must scan operator proof files before repository publication.
Pattern families must cover:

- URLs and schemes;
- probable IP addresses;
- Authorization header text;
- bearer-token-looking text;
- route-token/capability-looking assignments;
- private key blocks;
- SSH private material;
- Cloudflare token-looking text;
- generic API-key-looking assignments;
- long opaque token strings;
- payload or response-body hints.

Allowable exceptions are public commit SHAs, GitHub run/job IDs, non-secret
labels, known safe enum values, and explicit `no`, `unknown`, or `not_checked`
strings.

If private material is detected, the future lane must stop before publication.

## Option Review

Option A, targeted non-secret operator probe capture, is selected because exact
one-step proof commands, safe proof fields, schema validation, and
private-material policy can be defined.

Option B, remote service owner proof authorization, is not selected because
operator probe capture is the narrower next step. Option C, GitHub Actions
runner-specific proof authorization, is not selected because a separate exact
workflow lane is required if runner proof becomes necessary. Option D, qsc
runtime review, is not selected because environment proof is not clean enough to
shift suspicion to qsc. Option E, stop/ambiguous, is not selected because the
safe package is definable.

## Result Classification

`REMOTE_RELAY_TARGETED_NON_SECRET_OPERATOR_PROBE_CAPTURE_READY`.

## Selected Successor

`NA-0559 -- QSL Remote Relay Targeted Non-Secret Operator Probe Capture
Harness`.

The successor must capture and review one-step-at-a-time operator-run
non-secret probe proof, verify no private material is published, classify
endpoint/DNS/TCP/TLS/service-health/auth/route/runner boundaries, and select a
later exact environment fix, service-owner proof, GitHub-runner confirmation,
qsc runtime review, or stop successor.

## Required-Check Boundary

Current main required checks were classified before mutation. NA-0558 does not
execute workflow dispatches or reruns.

## Source / Script Mutation Boundary

No qsc source, qsc tests, qsc fuzz files, Cargo files, demo scripts, or source
files are changed by NA-0558.

## Workflow Mutation Boundary

No workflow file is changed. No workflow dispatch, rerun, cancel, or delete is
executed.

## Runtime / qsc / Dependency Boundary

No local qsc send/receive, qsc E2EE, qsc runtime reproduction, dependency
update, manifest update, or lockfile update is executed or made.

## qsl-server / qsl-attachments Boundary

No qsl-server or qsl-attachments command, clone, build, run, local use, or
mutation occurs.

## Remote-Action Boundary

No SSH, scp, sftp, rsync, Tailscale, ping, nc, endpoint curl, endpoint TLS
probe, remote command, sudo/admin action, systemctl action, backup command, or
remote-service mutation occurs.

## Public-Site / Cloudflare Boundary

No README public-progress content, docs/public content, website path, public
path, public-site content, deployment setting, or Cloudflare configuration is
changed.

## Claim Boundary

- No public-readiness claim is made.
- No production-readiness claim is made.
- No public-internet-readiness claim is made.
- No external-review-complete claim is made.
- No backup/restore-complete claim is made.
- No vulnerability-free claim is made.
- No bug-free claim is made.
- No perfect-build or perfect-crypto claim is made.

## Validation

Validation covers qwork proof, queue/decision proof, D-1104/D-1105 inheritance,
current-main required-check classification, prior evidence gap review, targeted
probe design, future operator command design, future proof schema,
private-material policy, option review, result classification, selected
successor, implementation scope guard, marker proof, link-check,
private-material scan, prohibited-material scan, overclaim scan,
docs/governance-only classifier, PR body preflight, goal-lint when available,
cargo audits, cargo fmt, and qsc-adversarial shell syntax.

Focused qsc runtime tests are skipped because NA-0558 is authorization-only,
changes governance/evidence/testplan paths only, changes no qsc
source/runtime/dependency/workflow path, and local qsc execution is not
authorized.

## Recommendation

Merge NA-0558 only after validation and required checks are green. After
implementation merge and healthy post-merge checks, a separate closeout may mark
NA-0558 DONE and restore NA-0559 using the exact D-1106-selected successor
block. Do not start NA-0559 work during NA-0558 closeout.
