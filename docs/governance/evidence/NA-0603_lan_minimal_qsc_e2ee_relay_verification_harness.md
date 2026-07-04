# NA-0603 LAN Minimal qsc E2EE Relay Verification Harness

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-04

Goals: G1, G2, G3, G4, G5

## Summary

NA-0603 executed the first private-LAN minimal qsc/qsl-server runtime lane with
the operator laptop as the qsc sender and receiver logical-client host and the
build server as the qsl-server relay/control-plane host. The lane verified
fresh qwork proof, inherited D-1195/D-1196, validated qsl-server on the build
server, selected a private-LAN-safe bind class, started qsl-server with
proof-root-only logs, generated an operator laptop packet and later a laptop
bootstrap bundle, consumed class-safe laptop proof, reviewed plaintext,
metadata, seed-fallback, and cleanup evidence, and recorded a hostile analyst
metadata hardening roadmap.

The runtime result is `LAN_TINY_QSC_LAPTOP_READINESS_GAP`. The build-server
qsl-server side reached ready class and the laptop proof classified TCP connect
and qsl-server route shape as ready. The laptop qsc sender and receiver logical
states initialized, but qsc rejected the private-LAN HTTP relay endpoint with
the TLS-required policy before the tiny-message send path could complete. The
tiny send was classified `tls_required_gap`, and receive/decrypt/validate was
classified `not_reached`.

## Qwork And Main Gates

- NA0603_D1195_LAN_READINESS_CONSUMED_OK
- NA0603_D1196_CLOSEOUT_CONSUMED_OK
- NA0603_FRESH_QWORK_PROOF_OK
- NA0603_CURRENT_MAIN_CHECKS_CLASSIFIED_OK

Fresh qwork proof from `2026-07-04T18:42:17Z` was verified before fetch,
runtime setup, operator packet generation, repository mutation, or proof
publication. Live pre-fetch `HEAD` and `origin/main` matched the qwork proof at
`8908447cfe2b`. The startup worktree, index, and untracked state were clean.
Disk and mount gates were below stop thresholds, and `/backup/qsl` was mounted.

Queue proof before mutation classified READY_COUNT 1 with READY NA-0603,
NA-0602 DONE, NA-0601 DONE, D-1193 once, D-1194 once, D-1195 once, D-1196
once, D-1197 absent, D-1198 absent, and duplicate decision count zero.

## D534 / D-1195 / D-1196 Inheritance

D-1195 accepted NA-0602 and selected the first private-LAN runtime lane as a
tiny qsc E2EE message through qsl-server only. D-1196 restored NA-0603 as the
sole READY item. NA-0602 performed no LAN runtime test, no Codex SSH to the
laptop, no laptop SSH server setup, no second Codex on the laptop, no qsl-server
startup, no qsl-attachments runtime, and no source/workflow/dependency mutation.

## Authority Model Application

NA-0603 applied the operator-assisted private-LAN authority model. Codex
controlled only build-server qsl-server setup, proof-root logging, operator
packet generation, class-safe proof parsing, and cleanup. The operator controlled
all laptop commands and transferred only class-safe proof. No Codex SSH to the
laptop occurred. No laptop SSH server setup occurred. No second Codex on the
laptop occurred.

## qsl-server Build-Server Readiness

- NA0603_QSL_SERVER_VALIDATION_OK

qsl-server was acquired under the NA-0603 build-server workspace, checked out on
clean main, and verified to include the NA-0598 fix merge `544edfd213ea`.
Validation completed with locked metadata, audit, format, tests, and build. No
qsl-server source, dependency, lockfile, workflow, deployment, or service
configuration path was mutated.

## Private-LAN Bind And Startup

- NA0603_PRIVATE_LAN_BIND_CLASSIFIED_OK
- NA0603_QSL_SERVER_STARTUP_CLASSIFIED_OK

Build-server bind selection chose `private_lan_bind_class`. Public bind was not
required. qsl-server startup reached ready class with proof-root-only logs and
non-secret route/auth fixture values. A selected auth negative classified
fail-closed. Private endpoint, port, host, route-token, and bearer values remain
local/private and are not published.

## Operator Packet And Laptop Proof

- NA0603_OPERATOR_COMMAND_PACKET_CREATED_OK
- NA0603_LAPTOP_PROOF_CLASSIFIED_OK

Codex created the required operator packet under the stable operator handoff
directory, then added a laptop bundle to reduce manual command-entry risk. The
bundle used a bundled qsc binary on compatible Linux laptops and source-build
fallback when Cargo was already available locally. The bundle self-check passed
on the operator laptop and later produced a class-safe `laptop_result.json`.

Laptop proof validation classified the JSON schema as valid, exact private-value
hit count zero, private IPv4 literal publication false, and
`private_values_published_class` as `no`.

## LAN Tiny-Message Classification

- NA0603_TINY_SEND_CLASSIFIED_OK
- NA0603_TINY_RECEIVE_DECRYPT_VALIDATE_CLASSIFIED_OK
- NA0603_RESULT_CLASSIFICATION_SELECTED_OK

Class-safe laptop proof reported:

- laptop qsc sender ready class: `ready`
- laptop qsc receiver ready class: `ready`
- laptop-to-build-server TCP connect class: `success`
- qsl-server route shape class: `ready`
- qsc LAN state setup class: `tls_required_gap`
- qsc LAN tiny send class: `tls_required_gap`
- qsc LAN receive/decrypt/validate class: `not_reached`

The selected NA-0603 result classification is
`LAN_TINY_QSC_LAPTOP_READINESS_GAP`. This is not a tiny-message E2EE pass.

## Plaintext, Seed, Metadata, And Cleanup

- NA0603_QSL_SERVER_PLAINTEXT_EXPOSURE_CLASSIFIED_OK
- NA0603_SEED_FALLBACK_CLASSIFIED_OK
- NA0603_METADATA_REVIEW_OK
- NA0603_CLEANUP_DONE_OK

qsl-server plaintext exposure was classified `no`; tiny-message plaintext was
not observed in qsl-server proof-root logs. Payload body logging was not reached
because qsc did not send the tiny message. Route-token and bearer publication
were classified `no` for repository/public evidence, and raw scan details remain
proof-root-only.

Seed fallback use was classified `no`; unsafe seed-fallback environment class
was `absent`; NA-0593 hardening regression was `no`.

Metadata exposure was classified `classified`; timing, route activity, and
endpoint details remain proof-root/operator-local and are published only as
classes. Build-server qsl-server cleanup classified `LAN_TINY_CLEANUP_DONE`, and
the operator laptop cleanup class was `complete`.

## Hostile Analyst Roadmap

- NA0603_HOSTILE_ANALYST_METADATA_ROADMAP_RECORDED_OK

NA-0603 records a future hostile analyst / metadata minimization and
implementation attack hardening lane family. It must cover traffic-analysis
metadata such as message size, attachment size, chunk/object count, timing,
retry pattern, route activity, sender/recipient correlation, online/offline
correlation, storage/fetch timing, and delete/retention timing. It must also
cover hostile source/code review, relay-compromise and qsl-attachments
compromise models, malformed envelope tests, descriptor tamper tests, object
corruption tests, wrong peer, wrong route, wrong token, replay-like duplicate,
stale state, seed fallback regression, logging/diagnostic leak tests, filename
and MIME omission/encryption, size bucketing, optional padding, timing buckets,
retry normalization, error-message normalization, route/activity minimization,
short object lifetime, cleanup/delete proof, capability secrecy proof, and no
raw command-line secret leakage.

## Private-Material And Boundary Markers

- NA0603_PRIVATE_MATERIAL_SCAN_OK
- NA0603_NO_ENDPOINT_VALUE_PUBLISHED_OK
- NA0603_NO_PRIVATE_PORT_VALUE_PUBLISHED_OK
- NA0603_NO_TOKEN_OR_AUTHORIZATION_PUBLISHED_OK
- NA0603_NO_CAPABILITY_VALUE_PUBLISHED_OK
- NA0603_NO_PAYLOAD_BODY_PLAINTEXT_PUBLISHED_OK
- NA0603_NO_SEED_OR_KEY_MATERIAL_PUBLISHED_OK
- NA0603_NO_QSL_ATTACHMENTS_RUNTIME_OK
- NA0603_NO_CODEX_SSH_TO_LAPTOP_OK
- NA0603_NO_LAPTOP_SSH_SERVER_SETUP_OK
- NA0603_NO_SECOND_CODEX_ON_LAPTOP_OK
- NA0603_NO_REMOTE_TAILSCALE_WORKFLOW_ACTION_OK
- NA0603_NO_PUBLIC_SITE_CLOUDFLARE_MUTATION_OK
- NA0603_NO_PUBLIC_READINESS_CLAIM_OK
- NA0603_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0603_NO_REMOTE_READY_CLAIM_OK
- NA0603_NO_TAILNET_READY_CLAIM_OK
- NA0603_NO_LAN_READY_OVERCLAIM_OK
- NA0603_NO_CRYPTO_COMPLETE_CLAIM_OK
- NA0603_NO_ATTACHMENT_COMPLETE_CLAIM_OK

No raw LAN IP, private port, hostname, topology, token value, Authorization
value, route token, capability, payload/body/plaintext, ciphertext body, seed,
key material, raw command line, raw log, or private material is published. No
qsl-attachments runtime occurred. No remote action, Tailnet/Tailscale action,
workflow dispatch/rerun, public endpoint, DNS, Cloudflare, public-site,
deployment, qsc source/test, qsl-server source/test, qsl-attachments source/test,
dependency, or lockfile mutation occurred.

## Selected Successor

- NA0603_SUCCESSOR_SELECTED_OK
- NA0603_ONE_READY_INVARIANT_OK

Selected successor for closeout:

### NA-0604 -- QSL LAN Operator Laptop qsc Readiness Follow-Up Harness
Status: READY
Goals: G1, G2, G3, G4, G5

Objective:
Guide and review operator-owned laptop qsc readiness proof for the minimal LAN
tiny-message test. Codex must not run laptop commands or publish private values.

NA-0604 is not implemented by this evidence. Until closeout, NA-0603 remains the
sole READY item.
