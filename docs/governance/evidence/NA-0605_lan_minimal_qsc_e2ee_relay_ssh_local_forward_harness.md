# NA-0605 LAN Minimal qsc E2EE Relay via SSH Local-Forward Harness

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-05

Goals: G1, G2, G3, G4, G5

## Summary

NA-0605 executed the first tiny private-LAN qsc E2EE relay verification after
the qsc TLS-required policy finding. The operator laptop ran both qsc logical
client states. The build server ran qsl-server on loopback only. The operator
created an SSH local-forward so laptop qsc used an allowed HTTP loopback relay
endpoint while LAN carriage stayed inside SSH.

Result classification:
`LAN_SSH_FORWARD_TINY_QSC_E2EE_RELAY_VERIFICATION_PASS`.

This is a tiny SSH-forward qsc/qsl-server relay pass only. It is not a public,
production, direct-LAN HTTPS, Tailnet, qsl-attachments, crypto-complete,
side-channel-free, vulnerability-free, or bug-free claim.

## Qwork, Queue, And Main Gates

- NA0605_FRESH_QWORK_PROOF_OK
- NA0605_CURRENT_MAIN_CHECKS_CLASSIFIED_OK
- NA0605_ONE_READY_INVARIANT_OK

Fresh qwork proof from `2026-07-05T17:41:41Z` was verified before fetch,
qsl-server clone/build/run, operator packet generation, repository mutation,
GitHub metadata review, PR creation, or proof publication. Live pre-fetch
`HEAD` and `origin/main` matched the qwork proof at `5bb8e78f9c7`.

Startup proof classified the worktree, index, and untracked state as clean.
Root disk usage was below the stop threshold, `/backup/qsl` was mounted, and
origin/main descended from `5bb8e78f9c7`.

Queue proof before mutation classified READY_COUNT 1 with READY NA-0605,
NA-0604 DONE, D-1199 once, D-1200 once, D-1201 absent, D-1202 absent, and
duplicate decision count zero.

Current-main checks were classified green before implementation: public-safety
success, advisories success, suite2-vectors success or conclusively satisfied,
no failed required checks, no pending required checks, root cargo audit success,
nested qsc fuzz cargo audit success, locked metadata success, cargo format
success, and no Cargo.toml or lockfile drift.

## Inheritance

- NA0605_D1199_TLS_POLICY_CONSUMED_OK
- NA0605_D1200_CLOSEOUT_CONSUMED_OK
- NA0605_QSL_PROTOCOL_TRANSPORT_DISTINCTION_OK

D-1199 was consumed once and accepted result
`NA0604_QSC_TLS_REQUIRED_POLICY_CONFIRMED`. D-1200 was consumed once and
restored NA-0605 as the SSH local-forward successor.

NA-0605 preserves the D-1200 protocol/transport distinction: QSL/qsc remains
the cryptographic/content protocol. HTTP, HTTPS, SSH local-forward, Tailnet,
and public internet paths are transport/carriage layers for opaque qsc/QSL
envelopes and relay APIs. qsc TLS-required policy remains intact: HTTP
loopback is allowed, HTTP non-loopback remains blocked, and HTTPS non-loopback
remains the later direct-network transport path.

## Authority Model

Codex controlled only the build-server side: qwork proof review, qsl-server
clone/build/test/audit, loopback qsl-server startup, proof-root logs, local-only
operator packet generation, class-safe result parsing, plaintext/metadata/seed
review, cleanup, and governance evidence.

The operator controlled all laptop-side commands. Codex did not SSH to the
laptop, did not install or enable a laptop SSH server, did not run a second
Codex on the laptop, and did not run qsl-attachments.

## qsl-server Readiness And Startup

- NA0605_QSL_SERVER_VALIDATION_OK
- NA0605_QSL_SERVER_LOOPBACK_BIND_OK
- NA0605_QSL_SERVER_STARTUP_CLASSIFIED_OK

qsl-server was acquired under the NA-0605 build-server workspace, checked out
on clean main, and verified to include the NA-0598 fix merge `544edfd213ea`.
Validation completed with locked metadata, audit, format, tests, and build.

qsl-server was started with bind class
`build_server_loopback_bind_for_ssh_forward`. Public bind class was `no`.
Route shape class was `ready`. qsl-attachments runtime class was `no`.

An initial detached process/readiness attempt and a later clean-rerun restart
required proof-root-only recovery. Both recoveries were bounded to runtime
tooling and status classification; no qsl-server source, dependency, lockfile,
workflow, service, firewall, deployment, or public endpoint mutation occurred.

## Operator Packet And Laptop Proof

- NA0605_OPERATOR_COMMAND_PACKET_CREATED_OK
- NA0605_LAPTOP_PROOF_CLASSIFIED_OK
- NA0605_SSH_LOCAL_FORWARD_CLASSIFIED_OK

The local/private operator packet was created under
`/srv/qbuild/operator/NA-0605/`. The packet included the required schema,
redacted example, private-values warning, cleanup instructions, build-server
service status, and a laptop helper script.

The first helper run classified SSH local-forward established, tunnel success,
route ready, setup pass, and send pass, but receive/decrypt/validate
`fail_class`. Read-only qsc command-contract review showed the helper used the
wrong peer label for the Alice-to-Bob receive direction. The helper was revised
to use the current contract shape, the failed class result was archived
proof-root-only, the in-memory relay was restarted on build-server loopback for
a clean rerun, and the operator reran the helper.

The final class-safe laptop proof validated with no concrete private-value
shape findings and `private_values_published_class` `no`.

## Tiny qsc E2EE Result

- NA0605_TINY_SEND_CLASSIFIED_OK
- NA0605_TINY_RECEIVE_DECRYPT_VALIDATE_CLASSIFIED_OK
- NA0605_RESULT_CLASSIFICATION_SELECTED_OK

Final class-safe laptop proof reported:

- SSH local-forward class: `established`
- laptop qsc sender ready class: `ready`
- laptop qsc receiver ready class: `ready`
- qsc endpoint policy class: `loopback_http_allowed`
- laptop-to-build-server tunnel class: `success`
- qsl-server route shape class: `ready`
- qsc LAN state setup class: `pass`
- qsc LAN tiny send class: `pass`
- qsc LAN tiny receive/decrypt/validate class: `pass`
- laptop cleanup class: `complete`
- private values published class: `no`

The selected runtime result is
`LAN_SSH_FORWARD_TINY_QSC_E2EE_RELAY_PASS`.

## Plaintext, Seed, Metadata, And Cleanup

- NA0605_QSL_SERVER_PLAINTEXT_EXPOSURE_CLASSIFIED_OK
- NA0605_SEED_FALLBACK_CLASSIFIED_OK
- NA0605_METADATA_REVIEW_OK
- NA0605_CLEANUP_DONE_OK

qsl-server plaintext exposure was classified `no`. Payload body logging was
classified `no`. Route-token publication and bearer publication were classified
`no`. Command-line private values were classified `no`.

Seed fallback use was classified `no`; unsafe test seed fallback environment
class was `absent`; NA-0593 hardening regression was `no`.

Metadata exposure was classified `classified`. Timing, route activity, and
endpoint metadata remain proof-root/operator-local and are published only as
classes.

Cleanup classified `LAN_SSH_FORWARD_TINY_CLEANUP_DONE`. The owned build-server
qsl-server process was stopped, no owned listener remained, and laptop cleanup
class was `complete`.

## Hostile Analyst Roadmap

- NA0605_HOSTILE_ANALYST_METADATA_ROADMAP_RECORDED_OK

NA-0605 records a future QSL Hostile Analyst / Metadata Minimization and
Implementation Attack Hardening Plan. It must cover traffic-analysis metadata:
message size, attachment size, chunk/object count, timing, retry pattern, route
activity, sender/recipient correlation, online/offline correlation,
storage/fetch timing, and delete/retention timing.

It must also cover implementation-level attack analysis: hostile source/code
review, relay-compromise model, qsl-attachments compromise model, malformed
envelope tests, descriptor tamper tests, object corruption tests, wrong peer,
wrong route, wrong token, replay-like duplicate, stale state, seed fallback
regression, and logging/diagnostic leak tests.

Hardening options include filename encryption or omission, MIME/content-type
encryption or omission, size bucketing, optional padding, final-chunk padding,
timing buckets, retry normalization, error-message normalization,
route/activity minimization, short object lifetime, cleanup/delete proof,
capability secrecy proof, and no raw command-line secret leakage.

This roadmap is recorded only; it is not implemented in NA-0605.

## Private-Material Boundary

- NA0605_PRIVATE_MATERIAL_SCAN_OK
- NA0605_NO_ENDPOINT_VALUE_PUBLISHED_OK
- NA0605_NO_PRIVATE_PORT_VALUE_PUBLISHED_OK
- NA0605_NO_TOKEN_OR_AUTHORIZATION_PUBLISHED_OK
- NA0605_NO_CAPABILITY_VALUE_PUBLISHED_OK
- NA0605_NO_PAYLOAD_BODY_PLAINTEXT_PUBLISHED_OK
- NA0605_NO_SEED_OR_KEY_MATERIAL_PUBLISHED_OK

No endpoint value, private port value, hostname, private topology value, token
value, Authorization value, route-token value, bearer value, capability value,
payload/body/plaintext, ciphertext body, seed, key material, raw command line,
raw log, process identity, or private material is published.

## Runtime And Mutation Boundaries

- NA0605_NO_QSL_ATTACHMENTS_RUNTIME_OK
- NA0605_NO_CODEX_SSH_TO_LAPTOP_OK
- NA0605_NO_LAPTOP_SSH_SERVER_SETUP_OK
- NA0605_NO_SECOND_CODEX_ON_LAPTOP_OK
- NA0605_NO_REMOTE_TAILSCALE_WORKFLOW_ACTION_OK
- NA0605_NO_PUBLIC_SITE_CLOUDFLARE_MUTATION_OK

No qsl-attachments runtime occurred. No Codex SSH to the laptop occurred. No
laptop SSH server setup occurred. No second Codex on the laptop occurred. No
Tailnet/Tailscale action occurred. No workflow dispatch or rerun occurred. No
public endpoint, DNS, Cloudflare, public-site, firewall, sudo, systemd,
deployment, qsc source/test/fuzz, qsl-server source/test, qsl-attachments
source/test, workflow, dependency, or lockfile mutation occurred.

## Claim Boundary

- NA0605_NO_PUBLIC_READINESS_CLAIM_OK
- NA0605_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0605_NO_REMOTE_READY_CLAIM_OK
- NA0605_NO_TAILNET_READY_CLAIM_OK
- NA0605_NO_LAN_READY_OVERCLAIM_OK
- NA0605_NO_CRYPTO_COMPLETE_CLAIM_OK
- NA0605_NO_ATTACHMENT_COMPLETE_CLAIM_OK

NA-0605 proves only a tiny operator-assisted SSH local-forward qsc E2EE relay
path using build-server loopback qsl-server. It does not claim public readiness,
production readiness, remote readiness, Tailnet readiness, broad LAN readiness,
direct HTTPS LAN readiness, crypto completion, attachment completion, metadata
freedom, anonymity, untraceability, side-channel freedom, vulnerability freedom,
bug freedom, formal proof completion, or external-review completion.

## Successor

- NA0605_SUCCESSOR_SELECTED_OK

Selected successor: `NA-0606 -- QSL LAN Tiny-Message Hostile Analyst / Metadata
and Fail-Closed Adversarial Harness`.

Objective: Stress and analyze the first working LAN SSH-forward tiny-message
qsc E2EE relay path under a hostile analyst model. Assume an attacker can read
the protocol and code, observe relay metadata, collect timing/size/route
metadata, and send malformed or adversarial traffic. Run selected LAN-safe
negatives: wrong route, wrong bearer or route token, wrong peer if supported,
tampered envelope, malformed relay payload, stale/replay-like duplicate,
missing state, empty pull after drain, qsl-server log review, qsc local
state/log review, seed fallback regression check, metadata minimization matrix,
and cleanup. No qsl-attachments yet. No Codex SSH to laptop, no laptop SSH
server, no second Codex, no Tailnet, no workflow dispatch, no public endpoint,
and no source/dependency/lockfile mutation unless a later directive explicitly
authorizes an exact safe fix. Publish only class summaries and no private
values.
