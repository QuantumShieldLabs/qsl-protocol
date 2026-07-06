# NA-0608 LAN qsl-attachments Hostile Analyst / Metadata and Fail-Closed Adversarial Harness

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-06

Goals: G1, G2, G3, G4, G5

## Summary

NA-0608 stressed and analyzed the working LAN qsl-attachments path under a
hostile-analyst model, building on the NA-0607 send/receive verification
(`LAN_QSL_ATTACHMENTS_SSH_FORWARD_SEND_RECEIVE_PASS`). Under directive
QSL-DIR-2026-07-06-541 (D541) and its Addendum A (bounded LAN runtime
provisioning), the build server provisioned a disposable runtime (qsl-server
`19b9b02dbe1f`, qsl-attachments `a3ebad2fd19a`, both cloned/built/validated
outside the tracked repo, plus qsc built from the tracked workspace), ran
qsl-server and qsl-attachments bound to loopback only, and exercised a real
two-party qsc attachment path with real ML-KEM/ML-DSA identities and no seed
fallback.

Per operator direction, the adversarial negatives and the leakage, seed-fallback,
and metadata analyses were run on the build-host loopback harness. That harness
exercises the same qsc-to-service fail-closed behavior as the SSH
loopback-forward transport, which NA-0607 already established; this lane does
not re-run over the laptop forward and makes no LAN-transport claim beyond
NA-0607.

Result classification:
`LAN_QSL_ATTACHMENTS_HOSTILE_ANALYST_FAIL_CLOSED_PASS` (loopback-harness variant).

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

- NA0608_D1207_TRANSITION_CONSUMED_OK
- NA0608_D1208_CLOSEOUT_CONSUMED_OK
- NA0608_FRESH_QWORK_PROOF_OK
- NA0608_CURRENT_MAIN_HEALTH_OK
- NA0608_D1209_ABSENT_BEFORE_IMPLEMENTATION_OK
- NA0608_LAN_REACHABILITY_IDENTITY_OK
- NA0608_ADDENDUM_A_PROVISIONING_AUTHORIZED_OK
- NA0608_LOOPBACK_ONLY_BIND_CLASSIFIED_OK
- NA0608_SATELLITE_VALIDATION_OK
- NA0608_QSC_BUILD_NO_TRACKED_SOURCE_MUTATION_OK
- NA0608_BASELINE_REAL_PATH_NO_SEED_FALLBACK_PASS_OK
- NA0608_NEG_WRONG_CAPABILITY_FAIL_CLOSED_OK
- NA0608_NEG_WRONG_RESUME_TOKEN_FAIL_CLOSED_OK
- NA0608_NEG_CORRUPTED_OBJECT_FAIL_CLOSED_OK
- NA0608_NEG_MISSING_OBJECT_FAIL_CLOSED_OK
- NA0608_NEG_REPLAY_DUPLICATE_FAIL_CLOSED_OK
- NA0608_NEG_WRONG_ROUTE_FAIL_CLOSED_OK
- NA0608_NEG_WRONG_PEER_FAIL_CLOSED_OK
- NA0608_NEG_CORRUPTED_DESCRIPTOR_CLASSIFIED_OK
- NA0608_NO_MUTATION_ON_REJECT_CLASSIFIED_OK
- NA0608_PLAINTEXT_KEY_CAPABILITY_EXPOSURE_CLASSIFIED_OK
- NA0608_SEED_FALLBACK_NOT_USED_OK
- NA0608_METADATA_MATRIX_OK
- NA0608_CLEANUP_DONE_OK
- NA0608_PRIVATE_MATERIAL_SCAN_OK
- NA0608_NO_ENDPOINT_VALUE_PUBLISHED_OK
- NA0608_NO_PRIVATE_PORT_VALUE_PUBLISHED_OK
- NA0608_NO_TOKEN_OR_CAPABILITY_VALUE_PUBLISHED_OK
- NA0608_NO_PAYLOAD_BODY_PLAINTEXT_PUBLISHED_OK
- NA0608_NO_SEED_OR_KEY_MATERIAL_PUBLISHED_OK
- NA0608_NO_QSC_SOURCE_MUTATION_OK
- NA0608_NO_QSL_SERVER_SOURCE_MUTATION_OK
- NA0608_NO_QSL_ATTACHMENTS_SOURCE_MUTATION_OK
- NA0608_NO_DEPENDENCY_LOCKFILE_WORKFLOW_MUTATION_OK
- NA0608_NO_REMOTE_TAILSCALE_WORKFLOW_ACTION_OK
- NA0608_RESULT_CLASSIFICATION_SELECTED_OK
- NA0608_SUCCESSOR_SELECTED_OK
- NA0608_ONE_READY_INVARIANT_OK

## Qwork, Queue, And Main Gates

Operator-run qwork proof from `2026-07-06T18:53:09Z` was verified before any
mutation, fetch, provisioning, service work, remote access, GitHub metadata, or
proof publication. Live pre-fetch HEAD and origin/main matched the proof at
`0d404db94baa`; startup proof classified worktree, index, and untracked state
clean; root disk usage was below the stop threshold and `/backup/qsl` was
mounted; READY_COUNT was 1 with READY NA-0608. Queue and decision proof before
mutation classified D-1207 once, D-1208 once, D-1209 absent, and READY_COUNT 1.

## Inheritance

D-1207 (NA-0608A executor transition) and D-1208 (NA-0608A closeout) were each
consumed once and accepted; the guardrail/layout transition they record is live
(`CLAUDE.md`, `.claude/settings.json`, `docs/ops/DIRECTOR_OPERATIONS.md`
present). The NA-0607 chain (D-1205/D-1206) established the working LAN
qsl-attachments send/receive path this lane stresses. NA-0608 preserves the
protocol/transport distinction: QSL/qsc is the cryptographic/content protocol;
SSH loopback-forward is a LAN test transport only.

## Authority Model And Provisioning

The lane used D541 Tier 0 read-only, and D541 Addendum A Tier 2P/2S bounded
provisioning, confined to a disposable build-host runtime workspace outside the
tracked repo and to the qscwork-owned test workspace. LAN reachability was
verified read-only (SSH alias reachable, remote user class qscwork, remote test
workspace present). No sudo, systemd, firewall, package install, Tailnet,
workflow dispatch, public bind, personal file access, or broad laptop scan
occurred. qwork/qstart/qresume were not run by the executor.

qsl-server (`19b9b02dbe1f`) and qsl-attachments (`a3ebad2fd19a`) were cloned at
main into the disposable runtime workspace and validated (locked metadata,
audit, format, build). qsc was built from the tracked qsl-protocol workspace
with no tracked source, dependency, or lockfile mutation. All services bound
127.0.0.1 loopback only; no non-loopback/public bind occurred. Raw endpoints,
ports, storage paths, logs, and command lines remain proof-root-only.

## Baseline Positive Path

A real two-party qsc attachment send/receive/decrypt/validate was exercised with
real ML-KEM/ML-DSA identities, real vault passphrase key sources, and no seed
fallback. The handshake completed on both sides (QSP v5.0, real ratchet), the
attachment committed to the opaque object service, and the receiver's fetched
plaintext matched the sender fixture by digest. Baseline classified `pass`.

An early baseline failure was root-caused to a local self/peer identity label
inconsistency in the harness driver (not a protocol defect) and corrected;
recorded under Recoveries.

## Hostile-Analyst Negatives

Each negative fail-closed with a deterministic reject class and produced no
plaintext output; prep sends committed before fetch-side attacks:

- wrong capability: `REJECT_QATTSVC_FETCH_CAPABILITY`; no plaintext.
- wrong resume token: `REJECT_QATTSVC_RESUME_TOKEN` (and `REJECT_QATTSVC_SESSION_STATE` on reuse); no committed object.
- corrupted object: `REJECT_ATT_CIPHERTEXT_PRECHECK`; no plaintext.
- missing/deleted object: `REJECT_QATTSVC_LOCATOR_UNKNOWN`; no plaintext.
- replay-like duplicate: first pull succeeded, duplicate pull rejected at QSP header authentication (`qsp_hdr_auth_failed`); no duplicate plaintext.
- wrong route: no delivery from an unregistered mailbox route token (`recv_none`); no plaintext.
- wrong peer: no session for an unknown peer label (`protocol_inactive`); no plaintext.
- corrupted descriptor: the attachment descriptor travels inside the opaque QSP
  envelope; corrupting it manifests as the same envelope-authentication reject
  class as the replay case and was not separately exercised. Classified
  covered-by-envelope-authentication.

No-mutation-on-reject: rejected uploads left only staged/orphan artifacts that
the service's startup storage reconciliation discards; no committed or fetchable
object and no plaintext were produced by any rejected path.

## Log, Storage, And State Leakage Review

Leakage result:
`BOUNDARY_QSL_ATTACHMENTS_OPAQUE_STORAGE_CONFIRMED`.

A definitive plaintext-marker test sent an attachment payload built from many
copies of a unique canary string. The canary appeared zero times in the
attachment object storage and zero times in the qsl-server and qsl-attachments
runtime logs, confirming opaque ciphertext-only storage. Service logs emitted no
plaintext, passphrase, private-key, or fetch-capability values. qsl-server is
transport/control-plane only; qsl-attachments is opaque object storage only; qsc
owns encryption, decryption, descriptor processing, fetch, decrypt, and
validation.

## Seed Fallback

Seed fallback classified `not_used`. `QSC_ALLOW_SEED_FALLBACK` and
`QSC_UNSAFE_TEST_SEED_FALLBACK` were never set. No seed-fallback activation
marker appeared in runtime output; identities were real ML-KEM/ML-DSA keypairs.

## Metadata Matrix

Metadata result:
`LAN_ATTACHMENT_METADATA_CLASSIFIED_ACCEPTABLE_FOR_NEXT_STEP`.

Class-only, for a hostile analyst at the transport and storage boundaries:
plaintext body, key material, and fetch capability are PROTECTED (ciphertext-only
storage; capability never placed in URL or logs, guarded by
`REJECT_QATTSVC_SECRET_URL_PLACEMENT`); attachment ciphertext-object size,
object/part count, and upload/fetch timing are EXPOSED residual metadata (no
padding/bucketing at this layer); route/mailbox tokens are pseudonymous to the
relay (markers expose only a mailbox hash); sender/receiver identities are
PROTECTED (the relay sees pseudonymous route tokens, not identities). The
size/count/timing residual metadata is the documented next hardening surface for
the NA-0609 plan.

## Cleanup

Cleanup classified `LAN_ATTACHMENT_CLEANUP_DONE`. Loopback service listeners were
stopped (no listener remained). The qscwork-owned test scratch directory was
removed, returning the workspace to its pre-test class state. The disposable
build-host runtime tree is retained until closeout and then removed; it is never
committed. Proof-root runtime state was retained as evidence only.

## Private-Material Review

Private-material review classified pass for repository publication. No endpoint
value, private port value, hostname, topology value, token value, Authorization
value, route-token value, bearer value, capability value, payload/body/plaintext,
ciphertext body, seed value, key material, personal laptop path, raw command
line, raw log, or raw artifact is published. Only class summaries, class reject
codes, satellite commit short SHAs, and operational path constants appear.

## Recoveries

Recovered issues were bounded and recorded in the proof root:

- baseline pairing failure was root-caused to a self/peer identity label
  inconsistency in the harness driver (two labels selected two different local
  identities) and corrected to a consistent local label; not a protocol defect.
- negative isolation was corrected after shared storage/session state
  contaminated a first pass; the authoritative negatives were re-run with a
  fresh session per send-consuming negative and per-object storage isolation.
- process management was corrected to stop services by explicit listener PID
  after a pattern-based stop matched unrelated command text.

All recoveries were proof-root/runtime-harness recoveries. They did not mutate
qsc source, qsl-server source, qsl-attachments source, workflows, dependencies,
lockfiles, or system services.

## Result And Successor

Selected result:
`LAN_QSL_ATTACHMENTS_HOSTILE_ANALYST_FAIL_CLOSED_PASS` (loopback-harness variant).

Selected successor:
`NA-0609 -- QSL Hostile Analyst / Metadata Minimization and Implementation-Attack Hardening Plan`.

The successor should turn these findings into a bounded hardening plan covering
traffic-analysis metadata (size/count/timing/route/peer), implementation-attack
surface, relay and qsl-attachments compromise models, malformed
envelope/descriptor/object test expansion, padding/bucketing feasibility,
error/retry normalization, and external/formal-review readiness. It must not
start NA-0609 in this lane.

## Source, Workflow, Remote, And Public Boundaries

No qsc source/test/fuzz mutation occurred. No qsl-server source/test mutation
occurred. No qsl-attachments source/test mutation occurred. No dependency or
lockfile mutation occurred. No workflow mutation, workflow dispatch, or workflow
rerun occurred. No Tailnet/Tailscale action occurred. No public endpoint, DNS,
Cloudflare, public-site, firewall, sudo, systemd, service-manager, deployment,
or package-manager action occurred. The qscwork remote action was limited to the
authorized account, authorized SSH transport, and qscwork-owned test workspace;
no personal file access occurred. The disposable build-host runtime clones and
builds occurred outside the tracked repository and were not committed.

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
