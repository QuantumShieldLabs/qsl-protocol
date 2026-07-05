# NA-0607 LAN Minimal qsl-attachments Send Receive SSH Loopback-Forward Harness

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-05

Goals: G1, G2, G3, G4, G5

## Summary

NA-0607 ran the first bounded private-LAN qsl-attachments send/receive
verification after NA-0605 proved the tiny qsc relay path and NA-0606
stress-tested the tiny-message hostile analyst and fail-closed path. The
build server ran qsl-server and qsl-attachments on loopback-only/proof-root
runtime state. The authorized qscwork laptop account ran both qsc logical
states through SSH loopback-forwarded endpoints.

Result classification:
`LAN_QSL_ATTACHMENTS_SSH_FORWARD_SEND_RECEIVE_PASS`.

This is a bounded first qsl-attachments SSH loopback-forward send/receive
verification only.

It is not a public-readiness claim.
It is not a production-readiness claim.
It is not a remote-ready claim.
It is not a Tailnet-ready claim.
It is not a LAN-ready claim.
It is not a direct HTTPS LAN readiness claim.
It is not a crypto-complete claim.
It is not an attachment-complete claim.
It is not a vulnerability-free claim.
It is not a bug-free claim.
It is not a side-channel-free claim.
It is not a metadata-free claim.
It is not an anonymity claim.
It is not an untraceability claim.
It is not a formal-proof-complete claim.
It is not an external-review-complete claim.

## Required Markers

- NA0607_D1203_LAN_TINY_HOSTILE_PASS_CONSUMED_OK
- NA0607_D1204_CLOSEOUT_CONSUMED_OK
- NA0607_FRESH_QWORK_PROOF_OK
- NA0607_CURRENT_MAIN_CHECKS_CLASSIFIED_OK
- NA0607_QSCWORK_ACCESS_SAFETY_OK
- NA0607_QSCWORK_QSC_AVAILABLE_OK
- NA0607_QSL_SERVER_VALIDATION_OK
- NA0607_QSL_ATTACHMENTS_VALIDATION_OK
- NA0607_QSL_SERVER_STARTUP_OK
- NA0607_QSL_ATTACHMENTS_STARTUP_OK
- NA0607_SSH_LOOPBACK_FORWARDS_CLASSIFIED_OK
- NA0607_QSC_ATTACHMENT_SEND_CLASSIFIED_OK
- NA0607_QSC_ATTACHMENT_FETCH_DECRYPT_VALIDATE_CLASSIFIED_OK
- NA0607_QSC_ENCRYPTION_OWNER_CONFIRMED_OK
- NA0607_QSL_SERVER_PLAINTEXT_EXPOSURE_CLASSIFIED_OK
- NA0607_QSL_ATTACHMENTS_PLAINTEXT_KEY_EXPOSURE_CLASSIFIED_OK
- NA0607_SEED_FALLBACK_CLASSIFIED_OK
- NA0607_CAPABILITY_EXPOSURE_CLASSIFIED_OK
- NA0607_METADATA_MATRIX_OK
- NA0607_CLEANUP_DONE_OK
- NA0607_PRIVATE_MATERIAL_SCAN_OK
- NA0607_NO_ENDPOINT_VALUE_PUBLISHED_OK
- NA0607_NO_PRIVATE_PORT_VALUE_PUBLISHED_OK
- NA0607_NO_TOKEN_OR_AUTHORIZATION_PUBLISHED_OK
- NA0607_NO_CAPABILITY_VALUE_PUBLISHED_OK
- NA0607_NO_PAYLOAD_BODY_PLAINTEXT_PUBLISHED_OK
- NA0607_NO_SEED_OR_KEY_MATERIAL_PUBLISHED_OK
- NA0607_NO_QSC_SOURCE_MUTATION_OK
- NA0607_NO_QSL_SERVER_SOURCE_MUTATION_OK
- NA0607_NO_QSL_ATTACHMENTS_SOURCE_MUTATION_OK
- NA0607_NO_REMOTE_TAILSCALE_WORKFLOW_ACTION_OK
- NA0607_NO_PUBLIC_SITE_CLOUDFLARE_MUTATION_OK
- NA0607_NO_PUBLIC_READINESS_CLAIM_OK
- NA0607_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0607_NO_REMOTE_READY_CLAIM_OK
- NA0607_NO_TAILNET_READY_CLAIM_OK
- NA0607_NO_LAN_READY_OVERCLAIM_OK
- NA0607_NO_CRYPTO_COMPLETE_CLAIM_OK
- NA0607_NO_ATTACHMENT_COMPLETE_CLAIM_OK
- NA0607_RESULT_CLASSIFICATION_SELECTED_OK
- NA0607_SUCCESSOR_SELECTED_OK
- NA0607_ONE_READY_INVARIANT_OK

## Qwork, Queue, And Main Gates

Fresh qwork proof from `2026-07-05T22:31:07Z` was copied into the NA-0607
proof root and verified before fetch, qsl-server clone/build/run,
qsl-attachments clone/build/run, authorized qscwork SSH/scp use, qsc command
execution, repository mutation, GitHub metadata review, PR creation,
source-analysis publication, or proof publication.

Live pre-fetch `HEAD` and `origin/main` matched the qwork proof at
`f88da9f5b3fc`. Startup proof classified the worktree, index, and untracked
state as clean. Root disk usage was below the stop threshold, `/backup/qsl` was
mounted, and origin/main equaled or descended from `f88da9f5b3fc`.

Queue proof before mutation classified READY_COUNT 1 with READY NA-0607,
NA-0605 DONE, NA-0606 DONE, D-1201 once, D-1202 once, D-1203 once, D-1204
once, D-1205 absent, D-1206 absent, and duplicate decision count zero.

Current-main checks were classified green before implementation: public-safety
success, advisories success, suite2-vectors success or conclusively satisfied,
no failed required checks, no pending required checks, root cargo audit success,
nested qsc fuzz cargo audit success, locked metadata success, cargo format
success, and no Cargo.toml or lockfile drift.

## Inheritance

D-1203 was consumed once and accepted result
`LAN_TINY_HOSTILE_ANALYST_FAIL_CLOSED_STRESS_PASS`. D-1204 was consumed once
and restored NA-0607 as the qsl-attachments minimal send/receive successor.
qsl-attachments was deferred until NA-0607, no qsc/qsl-server/qsl-attachments
source mutation had occurred in NA-0606, and no Codex SSH to the laptop,
laptop SSH server setup, or second Codex on the laptop occurred before NA-0607.

NA-0607 preserves the D-1200 protocol/transport distinction: QSL/qsc remains
the cryptographic/content protocol. HTTP, HTTPS, SSH loopback-forward, Tailnet,
and public internet paths are transport/carriage layers for opaque qsc/QSL
envelopes, relay APIs, and attachment-object APIs. qsc TLS-required policy is
not weakened: qsc used loopback HTTP endpoint classes exposed through the SSH
loopback-forward harness.

## Authority Model And qscwork Access

The only remote account operated by Codex was the authorized qscwork laptop test
account. Narrow readiness checks classified SSH connectivity as success, remote
user class as qscwork, remote workspace class as qscwork-owned test workspace
ready, sudo required class as no, broad scan class as no, and cleanup class as
planned.

No sudo, root, systemd, service-manager, firewall, package-manager, system
install, broad home scan, personal account path access, browser/email/desktop
file access, shell-history access, SSH-key access, Tailnet/Tailscale mutation,
public service exposure, or second Codex installation occurred on qscwork.

## qsc Availability

qsc availability on qscwork classified `ready_staged`. No existing qsc binary
was used from arbitrary locations. A build-server-produced qsc binary was
rebuilt from the current qsl-protocol workspace without source or dependency
mutation, staged only into the qscwork-owned test workspace, and executed there.
Architecture class was compatible. No system install or sudo was required.
qsc endpoint policy classified `loopback_http_allowed`, and qscwork seed
fallback environment classified `absent`.

## qsl-server Readiness And Startup

qsl-server was acquired under the NA-0607 build-server workspace, checked out on
clean main, and verified to include fix merge `544edfd213ea`. Validation
completed with locked metadata, audit, format, tests, and build.

qsl-server startup classified `ready`. Route shape classified `ready`.
Public bind class was `no`. Plaintext logging baseline classified `no`.
Runtime logs and process tracking remained proof-root-only.

## qsl-attachments Readiness And Startup

qsl-attachments was acquired under the NA-0607 build-server workspace, checked
out on clean main, and verified to include recovery commit `767eca189ee`.
Validation completed with locked metadata, audit, format, tests, and build.

qsl-attachments startup classified `ready`. Service shape classified `ready`.
The storage root was proof-root-local. Public bind class was `no`. Logs,
capability values, storage paths, and object details remained proof-root-only.

## SSH Loopback Forward Setup

SSH loopback-forward setup classified relay forward `established` and
qsl-attachments forward `established`. qsc endpoint policy for relay and
qsl-attachments classified `loopback_http_allowed`. Public exposure class was
`no`. Endpoint values, private port values, hostnames, topology values, and raw
command lines remain proof-root-only and are not published here.

## Remote qsc Attachment Send Receive

The qscwork workspace generated a synthetic non-secret attachment fixture above
the qsl-attachments threshold. qsc sender and receiver logical state both lived
under the qscwork-owned test workspace. qsc used the SSH loopback-forwarded
loopback endpoint classes for qsl-server and qsl-attachments.

qsc attachment send classified `pass`. Descriptor/envelope relay through
qsl-server occurred as opaque qsc/QSL data. Opaque attachment-object upload to
qsl-attachments occurred. qsc receive/pull, attachment fetch, decrypt, and
fixture validation classified `pass`.

qsc encryption owner classified `confirmed`. qsl-server plaintext exposure
classified `no`. qsl-attachments plaintext exposure classified `no`.
qsl-attachments key-material exposure classified `no`. Seed fallback use
classified `no`. Capability exposure classified `proof_root_only`.
Private values published classified `no`.

## Boundary Review

Boundary result:
`BOUNDARY_QSC_ENCRYPTION_OWNER_CONFIRMED`,
`BOUNDARY_QSL_SERVER_CONTROL_PLANE_ONLY_CONFIRMED`,
`BOUNDARY_QSL_ATTACHMENTS_OPAQUE_STORAGE_CONFIRMED`, and
`BOUNDARY_SEED_FALLBACK_ABSENT_CONFIRMED`.

qsc owns encryption/decryption, descriptor processing, fetch, decrypt, and
validation. qsl-server is relay/control-plane only for this lane and did not
store attachment object bodies. qsl-attachments is opaque ciphertext/object
storage only for this lane and did not see qsc plaintext or qsc key material.

## Metadata Matrix

Metadata result:
`LAN_ATTACHMENT_METADATA_CLASSIFIED_ACCEPTABLE_FOR_NEXT_STEP`.

Attachment size, chunk/object count, timing, retry pattern if present, route
activity, sender/receiver correlation, storage/fetch timing, and retention
timing remain necessary residual or proof-root-only metadata for this bounded
harness. Capability values, storage paths, endpoint values, private port values,
hostnames, topology values, command-line details, qsc local state details,
qsl-server log details, qsl-attachments log details, and ciphertext bodies
remain proof-root-only or class-only. Plaintext exposure and key-material
exposure classified eliminated from published evidence.

## Cleanup

Cleanup classified `LAN_ATTACHMENT_CLEANUP_DONE`. NA-0607-owned qscwork test
processes and remote workspace state were cleaned within the qscwork-owned
workspace boundary. SSH forwards were stopped. NA-0607-owned build-server
qsl-server and qsl-attachments processes were stopped. No NA-0607-owned
build-server listener remained. Proof-root runtime state was retained as
evidence only.

## Private-Material Review

Private-material review classified pass for repository publication. Raw logs,
raw command lines, endpoint values, private port values, hostnames, topology
values, token values, Authorization values, route-token values, capability
values, payload/body/plaintext, ciphertext bodies, seed values, key material,
personal laptop paths, and raw qscwork proof details remain proof-root-only or
were removed from the qscwork workspace during cleanup.

No endpoint value, private port value, hostname, topology value, token value,
Authorization value, route-token value, bearer value, capability value,
payload/body/plaintext, ciphertext body, seed, key material, raw command line,
raw log, raw artifact, or private material is published.

## Recoveries

Recovered proof/runtime issues were bounded and recorded in the proof root:

- qwork env parser accepted exported shell assignment syntax after the first
  proof-tool parser shape rejected it.
- inheritance absence proof used exact decision-ID counting after a broad text
  check matched the phrase that D-1205 was absent.
- qsl-server validation summary and source-clean classifiers were corrected to
  handle proof-tool path typing and build artifacts under the allowed workspace.
- qsl-attachments help/readiness probing was contained and corrected after the
  service binary started rather than printing help, and after the first
  readiness payload used an obsolete JSON shape.
- seed-fallback scanning was corrected to scan raw runtime output rather than
  proof-summary field names.

All recoveries were proof-root/runtime-tooling recoveries. They did not mutate
qsc source, qsl-server source, qsl-attachments source, workflows, dependencies,
lockfiles, deployment paths, public-site paths, Tailnet/Tailscale state, or
system services.

## Result And Successor

Selected result:
`LAN_QSL_ATTACHMENTS_SSH_FORWARD_SEND_RECEIVE_PASS`.

Selected successor:
`NA-0608 -- QSL LAN qsl-attachments Hostile Analyst / Metadata and Fail-Closed Adversarial Harness`.

The successor should stress and analyze the working LAN qsl-attachments SSH
loopback-forward path under a hostile analyst model, including capability,
descriptor, object, route, replay-like, log/state, seed-fallback, metadata, and
cleanup negatives where supported. It must not start NA-0608 in this lane.

## Source, Workflow, Remote, And Public Boundaries

No qsc source/test/fuzz mutation occurred. No qsl-server source/test mutation
occurred. No qsl-attachments source/test mutation occurred. No dependency or
lockfile mutation occurred. No workflow mutation, workflow dispatch, or workflow
rerun occurred. No Tailnet/Tailscale action occurred. No public endpoint, DNS,
Cloudflare, public-site, firewall, sudo, systemd, service-manager, deployment,
or package-manager action occurred.

The qscwork remote action was limited to the authorized account, authorized
SSH/scp transport, and qscwork-owned test workspace. No personal file access
occurred.

No public-readiness claim is introduced.
No production-readiness claim is introduced.
No remote-ready claim is introduced.
No Tailnet-ready claim is introduced.
No LAN-ready claim is introduced.
No direct HTTPS LAN readiness claim is introduced.
No crypto-complete claim is introduced.
No attachment-complete claim is introduced.
No vulnerability-free claim is introduced.
No bug-free claim is introduced.
No side-channel-free claim is introduced.
No metadata-free claim is introduced.
No anonymity claim is introduced.
No untraceability claim is introduced.
No formal-proof-complete claim is introduced.
No external-review-complete claim is introduced.
