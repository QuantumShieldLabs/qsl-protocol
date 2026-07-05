# NA-0606 LAN Tiny-Message Hostile Analyst Metadata and Fail-Closed Harness

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-05

Goals: G1, G2, G3, G4, G5

## Summary

NA-0606 stress-tested the first working operator-assisted LAN SSH-forward tiny
qsc E2EE relay path under the hostile analyst model selected by D-1201/D-1202.
The laptop ran both qsc logical client states. The build server ran qsl-server
on loopback only for SSH local-forward carriage. qsl-attachments was not used.

Result classification:
`LAN_TINY_HOSTILE_ANALYST_FAIL_CLOSED_STRESS_PASS`.

This is a tiny SSH-forward qsc/qsl-server relay hostile-analysis pass only.
It is not a public, production, remote, Tailnet, direct HTTPS LAN, or
qsl-attachments claim. It is not a crypto-complete or side-channel-free claim.
It is not a vulnerability-free or bug-free claim.

## Qwork, Queue, And Main Gates

- NA0606_D1201_LAN_TINY_PASS_CONSUMED_OK
- NA0606_D1202_CLOSEOUT_CONSUMED_OK
- NA0606_FRESH_QWORK_PROOF_OK
- NA0606_CURRENT_MAIN_CHECKS_CLASSIFIED_OK
- NA0606_ONE_READY_INVARIANT_OK

Fresh qwork proof from `2026-07-05T20:01:04Z` was verified before fetch,
qsl-server clone/build/run, operator packet generation, repository mutation,
GitHub metadata review, PR creation, source-analysis result publication, or
proof publication. A post-laptop continuation qwork proof from
`2026-07-05T21:29:16Z` was also verified before consuming the v1.1 laptop
result.

Live pre-fetch `HEAD` and `origin/main` matched the qwork proof at
`5b1e83ad1ea`. Startup proof classified the worktree, index, and untracked
state as clean. Root disk usage was below the stop threshold, `/backup/qsl` was
mounted, and origin/main descended from `5b1e83ad1ea`.

Queue proof before mutation classified READY_COUNT 1 with READY NA-0606,
NA-0604 DONE, NA-0605 DONE, D-1199 once, D-1200 once, D-1201 once, D-1202
once, D-1203 absent, D-1204 absent, and duplicate decision count zero.

Current-main checks were classified green before implementation: public-safety
success, advisories success, suite2-vectors success or conclusively satisfied,
no failed required checks, no pending required checks, root cargo audit success,
nested qsc fuzz cargo audit success, locked metadata success, cargo format
success, and no Cargo.toml or lockfile drift.

## Inheritance

D-1201 was consumed once and accepted result
`LAN_SSH_FORWARD_TINY_QSC_E2EE_RELAY_VERIFICATION_PASS`. D-1202 was consumed
once and restored NA-0606 as the hostile analyst / metadata and fail-closed
successor.

NA-0606 preserves the D-1200 protocol/transport distinction: QSL/qsc remains the
cryptographic/content protocol. HTTP, HTTPS, SSH local-forward, Tailnet, and
public internet paths are transport/carriage layers for opaque qsc/QSL envelopes
and relay APIs. qsc TLS-required policy remains intact: HTTP loopback is allowed,
HTTP non-loopback remains blocked, and HTTPS non-loopback remains the later
direct-network transport path.

## Authority Model

Codex controlled only the build-server side: qwork proof review, qsl-server
clone/build/test/audit, loopback qsl-server startup, proof-root logs,
build-server route/auth negative probes, local-only operator packet generation,
class-safe result parsing, metadata/plaintext/seed/qsc-state review, cleanup,
and governance evidence.

The operator controlled all laptop-side commands. Codex did not SSH to the
laptop, did not install or enable a laptop SSH server, did not run a second
Codex on the laptop, and did not run qsl-attachments.

## qsl-server Readiness And Startup

- NA0606_QSL_SERVER_VALIDATION_OK
- NA0606_QSL_SERVER_LOOPBACK_STARTUP_OK

qsl-server was acquired under the NA-0606 build-server workspace, checked out on
clean main, and verified to include the NA-0598 fix merge `544edfd213ea`.
Validation completed with locked metadata, audit, format, tests, and build.

qsl-server was started with bind class
`build_server_loopback_bind_for_ssh_forward`. Public bind class was `no`. Route
shape class was `ready`. qsl-attachments runtime class was `no`.

Runtime startup required proof-root-only recovery from child-process lifetime and
listener-status parser issues. Both recoveries were bounded to runtime/status
tooling; no qsl-server source, dependency, lockfile, workflow, service, firewall,
deployment, or public endpoint mutation occurred.

## qsl-server Negative Probes

- NA0606_QSL_SERVER_NEGATIVES_CLASSIFIED_OK

Build-server-local qsl-server probes that did not require laptop qsc state were
classified before operator laptop proof consumption. Missing route token, missing
bearer, wrong bearer, unsupported legacy path, unsupported method, empty body,
route A/B isolation, and unknown-route pull all classified fail-closed. The
nonempty malformed body case was classified as a qsl-server transport-opaque
boundary: qsl-server accepts opaque nonempty relay bytes and qsc performs
envelope validation.

qsl-server negative probe result:
`QSL_SERVER_ROUTE_AUTH_FAIL_CLOSED_PROBES_PASS`.

## Operator Packet And Laptop Proof

- NA0606_OPERATOR_COMMAND_PACKET_CREATED_OK
- NA0606_LAPTOP_PROOF_CLASSIFIED_OK

The local/private operator packet was created under
`/srv/qbuild/operator/NA-0606/`. The packet included the required schema,
redacted example, private-values warning, cleanup instructions, build-server
service status, and a laptop helper script.

The first class-safe laptop result validated, but its tamper probe had a
proof-tooling bug: the helper flipped an outer QSE metadata byte rather than a
Suite2 payload byte. Read-only qsc/refimpl review showed qsc writes received
output only after Suite2 unpack succeeds and that the v1 helper had not exercised
authenticated payload tamper. The stale class-safe result was quarantined
proof-root-only. A v1.1 local/private operator packet was created that mutates a
Suite2 payload byte. The operator reran the packet and returned class-safe
`laptop_result.json`.

The final v1.1 laptop proof schema validated, private-material value scan
passed, and `private_values_published_class` was `no`.

## Baseline Tiny E2EE Classification

- NA0606_BASELINE_TINY_E2EE_CLASSIFIED_OK

Final class-safe laptop proof reported:

- SSH local-forward class: `established`
- baseline tiny E2EE class: `pass`
- qsl-server plaintext exposure class: `no`
- seed fallback use class: `no`
- laptop cleanup class: `complete`
- private values published class: `no`

Baseline result:
`LAN_TINY_HOSTILE_BASELINE_PASS`.

## Fail-Closed Negative Matrix

- NA0606_FAIL_CLOSED_NEGATIVE_MATRIX_OK

Final class-safe laptop proof classified supported negatives as:

- wrong route: `pass_fail_closed`
- wrong bearer or token: `pass_fail_closed`
- wrong peer: `not_supported`
- tampered envelope: `pass_fail_closed`
- malformed relay payload: `pass_fail_closed`
- replay-like duplicate: `no_duplicate_delivery`
- missing state: `pass_fail_closed`
- empty pull after drain: `no_item_class`

Negative result:
`LAN_TINY_HOSTILE_NEGATIVES_PASS_WITH_UNSUPPORTED_LIMITS`.

No supported critical negative failed open.

## Metadata Minimization Matrix

- NA0606_METADATA_MINIMIZATION_MATRIX_OK

Metadata result:
`LAN_TINY_METADATA_CLASSIFIED_ACCEPTABLE_FOR_NEXT_STEP`.

Message size, timing, route activity, sender/receiver correlation, and
online/offline correlation remain necessary residual metadata for this
SSH-forward relay shape. They were retained proof-root/operator-local and
published only as classes. Retry pattern was not observed. Endpoint, port,
hostname, route token, bearer, command-line, and ciphertext-body details remain
proof-root/operator-local. Payload/body/plaintext and seed/key material exposure
were classified eliminated from published evidence.

## qsl-server Plaintext / Logging Review

- NA0606_QSL_SERVER_PLAINTEXT_LOGGING_REVIEW_OK

qsl-server plaintext exposure was classified `no`. Payload body logging was
classified `no`. Route-token publication and bearer publication were classified
`no`. Command-line private values were classified `no`. Timing and route
activity metadata remain class-only or proof-root-only.

## Seed Fallback and qsc State Review

- NA0606_SEED_FALLBACK_QSC_STATE_REVIEW_OK

Seed fallback use was classified `no`; unsafe test seed fallback environment
class was `absent`; NA-0593 hardening regression was `no`. qsc local state
plaintext or key exposure was classified `proof_root_only`, with no publication
of qsc key material or plaintext.

## Hostile Analyst Roadmap

- NA0606_HOSTILE_ANALYST_ROADMAP_EXPANDED_OK

NA-0606 expands the future QSL Hostile Analyst / Metadata Minimization and
Implementation Attack Hardening Plan. Future lane blocks should cover:

1. LAN Tiny-Message Metadata Hardening, if needed.
2. LAN qsl-attachments first send/receive, after NA-0606 pass.
3. LAN qsl-attachments hostile analyst / metadata stress.
4. Project-wide traffic-analysis metadata matrix.
5. Project-wide implementation-level hostile source/code review.
6. Padding/bucketing feasibility review.
7. Error-message and retry-normalization review.
8. Relay-compromise model review.
9. qsl-attachments compromise model review.
10. Formal/external-review readiness roadmap.

This roadmap is recorded only; it is not implemented in NA-0606.

## Cleanup

- NA0606_CLEANUP_DONE_OK

Cleanup classified `LAN_TINY_HOSTILE_CLEANUP_DONE`. The owned build-server
qsl-server process was stopped, no owned listener remained, proof-root logs were
retained only as artifacts, and laptop cleanup class was `complete`.

## Private-Material Boundary

- NA0606_PRIVATE_MATERIAL_SCAN_OK
- NA0606_NO_ENDPOINT_VALUE_PUBLISHED_OK
- NA0606_NO_PRIVATE_PORT_VALUE_PUBLISHED_OK
- NA0606_NO_TOKEN_OR_AUTHORIZATION_PUBLISHED_OK
- NA0606_NO_CAPABILITY_VALUE_PUBLISHED_OK
- NA0606_NO_PAYLOAD_BODY_PLAINTEXT_PUBLISHED_OK
- NA0606_NO_SEED_OR_KEY_MATERIAL_PUBLISHED_OK

No endpoint value, private port value, hostname, private topology value, token
value, Authorization value, route-token value, bearer value, capability value,
payload/body/plaintext, ciphertext body, seed, key material, raw command line,
raw log, process identity, or private material is published.

## Runtime And Mutation Boundaries

- NA0606_NO_QSL_ATTACHMENTS_RUNTIME_OK
- NA0606_NO_CODEX_SSH_TO_LAPTOP_OK
- NA0606_NO_LAPTOP_SSH_SERVER_SETUP_OK
- NA0606_NO_SECOND_CODEX_ON_LAPTOP_OK
- NA0606_NO_REMOTE_TAILSCALE_WORKFLOW_ACTION_OK
- NA0606_NO_PUBLIC_SITE_CLOUDFLARE_MUTATION_OK

No qsl-attachments runtime occurred. No Codex SSH to the laptop occurred. No
laptop SSH server setup occurred. No second Codex on the laptop occurred. No
Tailnet/Tailscale action occurred. No workflow dispatch or rerun occurred. No
public endpoint, DNS, Cloudflare, public-site, firewall, sudo, systemd,
deployment, qsc source/test/fuzz, qsl-server source/test, qsl-attachments
source/test, workflow, dependency, or lockfile mutation occurred.

## Claim Boundary

- NA0606_NO_PUBLIC_READINESS_CLAIM_OK
- NA0606_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0606_NO_REMOTE_READY_CLAIM_OK
- NA0606_NO_TAILNET_READY_CLAIM_OK
- NA0606_NO_LAN_READY_OVERCLAIM_OK
- NA0606_NO_CRYPTO_COMPLETE_CLAIM_OK
- NA0606_NO_ATTACHMENT_COMPLETE_CLAIM_OK

NA-0606 proves only a tiny operator-assisted SSH local-forward qsc E2EE relay
path under selected hostile-analysis negatives and metadata classification.
It does not claim public readiness, production readiness, remote readiness, or
Tailnet readiness. It does not claim broad LAN readiness, direct HTTPS LAN
readiness, or crypto completion. It does not claim attachment completion. It
does not claim metadata freedom, anonymity, untraceability, or side-channel
freedom. It does not claim vulnerability freedom or bug freedom.
It does not claim formal proof completion or external-review completion.

## Result And Successor

- NA0606_RESULT_CLASSIFICATION_SELECTED_OK
- NA0606_SUCCESSOR_SELECTED_OK

Selected result:
`LAN_TINY_HOSTILE_ANALYST_FAIL_CLOSED_STRESS_PASS`.

Selected successor:
`NA-0607 -- QSL LAN Minimal qsl-attachments Send / Receive via SSH Local-Forward Harness`.
