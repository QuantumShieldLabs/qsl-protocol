Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-18

# NA-0314 Metadata Runtime Identifier and Default Padding Transition Plan

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0314 is a planning and governance transition lane. It moves metadata runtime identifier/handle rotation and default padding from the NA-0288 through NA-0293 design and fixture baseline toward a future executable runtime proof lane. It does not implement metadata runtime behavior.

The selected successor is:

`NA-0315 -- Metadata Runtime Identifier and Default Padding Executable Harness Plan`

The successor should build a bounded executable harness that proves runtime identifier/handle rotation and default padding together where the same demo/qsc surfaces already carry peer IDs, session IDs, route tokens, relay queue keys, message metadata, padding metadata, and receive-side strip/verify behavior. If that bounded runtime harness cannot keep the scope exact, it must stop before widening into protocol, crypto, service, dependency, workflow, website, README, or START_HERE changes.

## Live NA-0314 Scope

The live queue item is `NA-0314 -- Metadata Runtime Identifier and Default Padding Transition Plan`, status `READY`, with goals G1 through G5.

Allowed by live scope:

- review NA-0288 through NA-0293 metadata evidence;
- inventory runtime surfaces for identifier/handle rotation and default padding;
- map fixture/design evidence to future executable proof;
- select the exact NA-0315 successor;
- preserve all claim boundaries and visible runtime gaps.

Forbidden by live scope:

- runtime metadata behavior implementation;
- identifier/handle rotation implementation;
- default padding implementation;
- protocol, crypto, qsc/qsp, qsl-server, qsl-attachments, qsc-desktop, website, workflow, Cargo, dependency, README, START_HERE, branch-protection, or public-safety mutation;
- any claim that fixture proof is runtime metadata reduction;
- any claim of anonymity, metadata-free behavior, untraceability, public-internet readiness, production readiness, or external-review completion.

## Sources Inspected

- `NEXT_ACTIONS.md` live NA-0314 entry.
- `tests/NA-0313_closeout_restore_na0314_testplan.md`.
- `docs/governance/evidence/NA-0288_metadata_phase2_external_review_gap_plan.md`.
- `docs/governance/evidence/NA-0290_metadata_phase2_identifier_padding_design.md`.
- `docs/governance/evidence/NA-0291_metadata_phase2_identifier_padding_harness.md`.
- `docs/governance/evidence/NA-0292_metadata_phase2_sanitized_errors_retention_design.md`.
- `docs/governance/evidence/NA-0293_metadata_phase2_sanitized_errors_retention_harness.md`.
- `docs/public/RELEASE_READINESS_EVIDENCE_MAP.md`.
- `docs/public/EXTERNAL_REVIEW_PACKAGE.md`.
- `docs/public/INDEX.md`.
- `README.md` and `START_HERE.md` for claim-boundary context only.
- `apps/qshield-cli/src/commands/{attachment,establish,init,recv,register,relay,send}.rs`.
- `apps/qshield-cli/src/{relay_client,store}.rs`.
- `apps/qsl-tui/src/{demo,main}.rs`.
- `qsl/qsl-client/qsc/src/{contacts,handshake,transport}/mod.rs`.
- `qsl/qsl-client/qsc/scripts/remote_soak.py`.
- `tools/refimpl/quantumshield_refimpl/tests/qse_bucket_confidentiality.rs`.
- `scripts/ci/metadata_conformance_smoke.sh`.
- `scripts/ci/metadata_phase2_identifier_padding_harness.sh`.
- `scripts/ci/metadata_phase2_sanitized_errors_retention_harness.sh`.
- `scripts/ci/demo_cli_smoke.sh`.
- `scripts/ci/demo_adversarial_stress.sh`.
- `scripts/ci/demo_soak_repeated_run.sh`.
- `TRACEABILITY.md`, `DECISIONS.md`, and `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

## Metadata Evidence Baseline

NA-0288 maps metadata phase-2 and external-review gaps. It proves selected local/demo/service evidence, but keeps identifier rotation, default padding, timing/traffic-shape, deployment metadata, public-internet behavior, and external review as open.

NA-0290 designs rotating opaque delivery handles and a named padding-default profile. It identifies stable demo peer/session/route/capability handles and optional padding as future runtime work.

NA-0291 adds deterministic executable policy-fixture proof for identifier/handle policy and padding buckets. Its markers are design/fixture markers, including `DESIGN_ONLY_ROTATION_POLICY_PROOF`, `DESIGN_ONLY_PADDING_POLICY_PROOF`, `NA0291_IDENTIFIER_POLICY_OK`, `NA0291_ROTATION_POLICY_OK`, `NA0291_PADDING_POLICY_OK`, and `NA0291_METADATA_PHASE2_HARNESS_OK`.

NA-0292 designs sanitized-error and retention/purge policy. It separates selected demo/service proof from broader runtime coverage.

NA-0293 adds deterministic executable policy-fixture proof for sanitized-error and retention/purge metadata policy. It does not prove complete runtime sanitized-error behavior or production retention/purge behavior.

## PROVEN_EXECUTABLE Inventory

- qshield loopback demo positive send/receive and selected negative auth/malformed/replay/id checks through `scripts/ci/demo_cli_smoke.sh`.
- qshield metadata conformance smoke for selected local relay auth, invalid padding metadata/config, padded queue inspection, and loopback behavior through `scripts/ci/metadata_conformance_smoke.sh`.
- qshield baseline adversarial stress and bounded repeated-run soak through `scripts/ci/demo_adversarial_stress.sh` and `scripts/ci/demo_soak_repeated_run.sh`.
- NA-0291 identifier/padding executable policy fixture, explicitly not runtime rotation/default padding.
- NA-0293 sanitized-error/retention executable policy fixture, explicitly not complete runtime coverage.
- QSE bucket confidentiality primitives in the refimpl test suite, not default transport policy.
- qsc suite-id implementation/harness evidence from NA-0313, unrelated to metadata runtime reduction.

## DOCS_ONLY Inventory

- NA-0288 metadata phase-2 gap plan.
- NA-0290 identifier rotation and padding defaults design.
- NA-0292 sanitized-error and retention/purge design.
- Public evidence package wording that classifies metadata phase-2 as partial or not ready.
- Current NA-0314 transition plan and successor selection.

## NOT_READY Inventory

- Runtime identifier/handle rotation for stable peer IDs, route identifiers, session IDs, message handles, attachment handles, and demo transcript/correlation handles.
- Runtime default padding mode for qshield/qsl-tui/qsc send/receive surfaces.
- Broader runtime sanitized-error normalization beyond selected harnessed cases.
- Runtime retention/purge behavior across demo state, route queues, attachment sessions/objects, and durable artifacts.
- Timing/traffic-shape resistance, batching, jitter, cover traffic, contact graph hiding, IP-level metadata, and public-internet metadata behavior.
- Production service metadata posture and deployment metadata posture.

## FUTURE_GATE Inventory

- Exact runtime handle-rotation harness with stale/malformed/replayed handle reject, no accepted-state mutation on reject, and no raw handle secret logs.
- Exact runtime default-padding harness with default-mode policy, invalid config reject, strip/verify, over-limit reject, no accepted-state mutation on reject, and no secret/plaintext leakage.
- Runtime sanitized-error expansion after identifier/default-padding behavior has executable coverage.
- Runtime retention/purge proof after object/session lifetimes are explicitly scoped.
- Timing/traffic-shape threat model and any later implementation authorization.
- qsl-server and qsl-attachments implementation proof in their own exact scopes, not by NA-0314.

## Runtime Surface Inventory

| Surface | Current visible files | Current proof level | Future artifact | Risk | NA-0315 fit |
| --- | --- | --- | --- | --- | --- |
| qshield peer/contact handles | `apps/qshield-cli/src/commands/register.rs`, `establish.rs`, `store.rs` | Runtime exists, rotation not proven | Runtime harness around peer handle lifecycle and stale peer reject | Stable `my_id`/peer IDs are queue and store keys | Yes, bounded |
| qshield session identifiers | `establish.rs`, `store.rs`, `relay_client.rs` | Runtime exists, rotation not proven | Session-bound handle fixture and stale session reject proof | Session ID currently printed and stored for demo | Yes, bounded |
| qshield route/message handles | `relay.rs`, `send.rs`, `recv.rs`, `relay_client.rs` | Selected conformance smoke only | Queue key/message metadata rotation and no-mutation reject proof | Poll removes queued message before local decode failure | Yes, but stop if no-mutation needs behavior change outside scope |
| qshield attachment handles | `attachment.rs`, `send.rs`, `recv.rs` | Attachment demo opaque-boundary proof, no rotation proof | Attachment descriptor/session/object handle proof | Attachment ID is deterministic from session and wire | Yes if bounded to demo attachment |
| qshield default padding | `init.rs`, `send.rs`, `recv.rs`, `relay.rs` | Optional padding validation proof | Default policy harness with bucket/strip/verify proof | Compatibility choice between default and opt-in mode | Yes |
| qsl-tui demo metadata | `apps/qsl-tui/src/demo.rs`, `main.rs` | Padded mode exists; default not proven | Padded/default profile test and metadata note check | Plaintext length prefix inside padded demo format is not a metadata-free claim | Yes if harness-only |
| qsc contact/route/session handles | `qsl/qsl-client/qsc/src/contacts/mod.rs`, `handshake/mod.rs`, `transport/mod.rs` | Runtime handles exist; rotation/default metadata proof absent | CLI/transport harness markers for route-token/session-bound behavior | qsc route tokens and contact labels are security-sensitive | Yes, bounded to tests/harness |
| sanitized errors | qshield relay commands, qsc errors, metadata harness scripts | Selected cases and fixtures | Later runtime expansion | Easy to overclaim complete coverage | Later |
| retention/purge | qshield relay state, qsc stores, service evidence docs | Selected local/service proof and fixtures | Later runtime lifetime proof | Durability semantics differ by component | Later |
| timing/traffic-shape | qshield/qsl-tui/qsc loops and CI scripts | Not ready | Threat model first | Could imply unavailable traffic shaping | Later |

## Identifier/Handle Rotation Transition Plan

Identifier classes for NA-0315:

- session identifiers;
- route identifiers and route tokens;
- peer/contact handles;
- message identifiers and relay queue handles;
- attachment session handles;
- attachment object/fetch handles;
- demo transcript and correlation handles.

Rotation triggers to model:

- session boundary;
- epoch boundary when available in the surface;
- route renewal;
- message batch;
- attachment commit/fetch;
- explicit deterministic operator/test fixture trigger.

Opaque-handle requirements:

- no raw secret, route token, bearer token, or stable identity should be emitted as a public handle;
- deterministic fixtures are allowed only where needed for reproducible tests;
- correlation boundaries must be explicit, including what remains linkable inside a demo run;
- rejected handles must use coarse reason classes.

Fail-closed behavior:

- malformed, invalid, replayed, stale, or wrong-scope handles reject;
- reject paths do not mutate accepted state;
- rejected handles do not reveal existence beyond the allowed coarse reason class;
- if a runtime surface cannot prove no mutation without changing semantics outside NA-0315 scope, NA-0315 must stop and select blocker resolution.

Logging boundaries:

- no raw handle secrets or bearer tokens in stdout/stderr, logs, harness artifacts, or CI output;
- only redacted, bounded, correlation-safe handles or deterministic fixture labels may appear.

Future executable markers:

- `NA0315_IDENTIFIER_ROTATION_POLICY_OK`
- `NA0315_OPAQUE_HANDLE_BOUNDARY_OK`
- `NA0315_STALE_HANDLE_REJECT_OK`
- `NA0315_IDENTIFIER_NO_MUTATION_ON_REJECT_OK`
- `NA0315_IDENTIFIER_NO_SECRET_LOG_OK`

## Default Padding Transition Plan

Padding surfaces for NA-0315:

- qshield CLI message send/receive and relay metadata;
- qsl-tui demo message mode;
- qsc transport send/receive metadata knobs;
- metadata fixture harness continuity from NA-0291;
- attachment descriptor/ciphertext sizes only if bounded to qshield demo attachment behavior.

Padding policy requirements:

- define a bucket table and deterministic test profile;
- define max overhead and over-limit behavior;
- invalid bucket config rejects at startup or test preflight;
- default runtime mode versus opt-in mode must be explicit;
- strip/verify behavior must reject malformed padded input and preserve valid plaintext recovery;
- no public error/log should expose a sensitive exact unpadded length where the surface treats that length as sensitive.

Fail-closed behavior:

- invalid padding config rejects before accepting runtime work;
- malformed padded input rejects;
- over-limit padding rejects;
- padding reject paths do not mutate accepted state;
- rejected padded input uses coarse reason labels.

Leakage boundaries:

- no plaintext sentinel in relay-visible metadata, public errors, logs, or artifacts;
- no raw unpadded length in public error/log output where sensitive;
- bucket metadata may appear only as the explicitly scoped padded profile behavior.

Future executable markers:

- `NA0315_DEFAULT_PADDING_POLICY_OK`
- `NA0315_PADDING_BUCKETS_OK`
- `NA0315_PADDING_INVALID_CONFIG_REJECT_OK`
- `NA0315_PADDING_STRIP_VERIFY_OK`
- `NA0315_PADDING_NO_MUTATION_ON_REJECT_OK`
- `NA0315_PADDING_NO_SECRET_LOG_OK`

## Sanitized Error Relationship

Identifier and padding rejects must use the NA-0292/NA-0293 coarse error posture, but NA-0315 should not attempt complete runtime sanitized-error expansion. NA-0315 should prove no secret-bearing handle, token, sentinel, plaintext, panic, backtrace, or oversized metadata dump appears in its own identifier/padding harness artifacts. Broader runtime error normalization remains a later lane.

## Retention/Purge Relationship

Identifier rotation and default padding interact with retention/purge through relay queue keys, session stores, attachment descriptors, and CI artifacts. NA-0315 should prove no accepted-state mutation on rejected identifier/padding inputs in the surfaces it touches. It should not claim production retention, deletion, purge, or backup lifecycle behavior.

## Timing/Traffic-Shape Relationship

The qshield, qsl-tui, qsc, and CI demo loops include visible polling, sleep, batch, and jitter knobs. NA-0315 should not implement or claim timing/traffic-shape resistance. If default padding changes message size behavior, the evidence must state that timing, contact graph, IP-level metadata, and traffic volume remain open.

## Combined Runtime Proof Strategy

| Area | Current proof level | Future artifact | Expected markers | Allowed future files | Forbidden future files |
| --- | --- | --- | --- | --- | --- |
| qshield identifiers | Runtime exists; rotation not proven | Runtime harness and fixtures | Identifier markers | `apps/qshield-cli/src/**`, `scripts/ci/metadata_runtime_identifier_padding_harness.sh`, `inputs/metadata_runtime/**`, tests/evidence/governance | README, START_HERE, `.github/**`, Cargo/dependency, website, qsl-server, qsl-attachments |
| qshield padding | Optional proof; default not proven | Default padding harness | Padding markers | same bounded qshield/harness files | same forbidden files |
| qsl-tui padding/metadata notes | Padded mode exists; default not proven | Harness assertions | Padding markers and no-secret log marker | `apps/qsl-tui/src/**` and tests if exact | Service, protocol-core, dependency, website paths |
| qsc route/session metadata | Runtime handles exist; metadata reduction not proven | CLI/transport harness assertions | Identifier markers, possibly padding markers | `qsl/qsl-client/qsc/src/{contacts,transport}/mod.rs`, qsc tests, scripts if exact | qsp protocol-core, key schedule, broad handshake changes |
| attachments demo handles | Opaque boundary proof; rotation not proven | Attachment handle fixture | Identifier markers | `apps/qshield-cli/src/commands/attachment.rs` and qshield tests if exact | qsl-attachments implementation unless separately authorized |
| sanitized errors | Fixture proof | Relationship checks only | no-secret log marker | same NA-0315 harness artifacts | broad error-normalization implementation |
| retention/purge | Fixture/local proof | no-mutation-on-reject checks only | no-mutation markers | same NA-0315 harness artifacts | production retention/purge implementation |

## Successor Selection

Selected successor:

`NA-0315 -- Metadata Runtime Identifier and Default Padding Executable Harness Plan`

Rationale:

- qshield runtime identifier and padding behavior meet at the same relay send/poll/register/establish/session store surfaces.
- qsl-tui and qsc already expose adjacent metadata knobs that can be harnessed without changing protocol or dependency surfaces if scope remains exact.
- Separating identifier and padding would duplicate setup across the same demo/session/relay flows and make the no-mutation/no-secret-log boundary harder to evaluate across one runtime path.
- A combined lane is acceptable only as a bounded executable harness plan; it must stop if exact allowed files, reject behavior, or public claim boundaries cannot be maintained.

Rejected alternatives:

- Standalone identifier-only successor: viable but would defer padding across the same send/receive surfaces and duplicate fixture setup.
- Standalone padding-only successor: viable but would defer the stable-handle risk that currently dominates qshield relay/session metadata.
- Blocker resolution: not selected because the visible runtime surfaces are clear enough to plan a bounded harness, with stop conditions for qshield poll mutation and any protocol/service spillover.

## Future NA-0315 Likely Files

Likely allowed files for the future directive, subject to explicit NA-0315 scope:

- `apps/qshield-cli/src/commands/relay.rs`
- `apps/qshield-cli/src/commands/send.rs`
- `apps/qshield-cli/src/commands/recv.rs`
- `apps/qshield-cli/src/commands/init.rs`
- `apps/qshield-cli/src/commands/establish.rs`
- `apps/qshield-cli/src/commands/register.rs`
- `apps/qshield-cli/src/commands/attachment.rs`
- `apps/qshield-cli/src/relay_client.rs`
- `apps/qshield-cli/src/store.rs`
- `apps/qsl-tui/src/demo.rs`
- `apps/qsl-tui/src/main.rs`
- `qsl/qsl-client/qsc/src/contacts/mod.rs`
- `qsl/qsl-client/qsc/src/transport/mod.rs`
- `qsl/qsl-client/qsc/tests/**`
- `scripts/ci/metadata_runtime_identifier_padding_harness.sh`
- `inputs/metadata_runtime/identifier_padding_runtime_vectors_v1.json`
- NA-0315 evidence, testplan, decisions, traceability, and rolling journal files.

Likely later, not NA-0315 unless separately authorized:

- qsl-server implementation;
- qsl-attachments implementation;
- qsc-desktop implementation;
- qsp protocol-core or key schedule files;
- website or public-copy paths;
- workflow, dependency, README, or START_HERE paths.

## Future NA-0315 Expected Markers

- `NA0315_IDENTIFIER_ROTATION_POLICY_OK`
- `NA0315_OPAQUE_HANDLE_BOUNDARY_OK`
- `NA0315_STALE_HANDLE_REJECT_OK`
- `NA0315_IDENTIFIER_NO_MUTATION_ON_REJECT_OK`
- `NA0315_IDENTIFIER_NO_SECRET_LOG_OK`
- `NA0315_DEFAULT_PADDING_POLICY_OK`
- `NA0315_PADDING_BUCKETS_OK`
- `NA0315_PADDING_INVALID_CONFIG_REJECT_OK`
- `NA0315_PADDING_STRIP_VERIFY_OK`
- `NA0315_PADDING_NO_MUTATION_ON_REJECT_OK`
- `NA0315_PADDING_NO_SECRET_LOG_OK`
- `NA0315_METADATA_RUNTIME_IDENTIFIER_PADDING_HARNESS_OK`

## Stop Conditions

NA-0315 must stop if:

- proof requires protocol, crypto, qsp, key schedule, dependency, workflow, branch-protection, public-safety, website, README, START_HERE, qsl-server, qsl-attachments, or qsc-desktop changes outside explicit scope;
- qshield poll/destructive queue semantics prevent no-mutation proof without an authorized runtime behavior decision;
- default padding cannot be switched from opt-in to default without a compatibility or operator-mode decision;
- handle rotation requires hiding stable identities beyond what the current demo/service model can truthfully prove;
- timing, contact graph, IP-level, public-internet, deployment, or traffic-shape claims would be needed to describe success;
- local or required CI fails conclusively.

## Claim Boundaries

NA-0314 and the selected NA-0315 successor do not claim:

- production readiness;
- public-internet readiness;
- external review completion;
- anonymity;
- metadata-free behavior;
- untraceability;
- complete metadata reduction;
- complete sanitized-error runtime coverage;
- production retention/purge behavior;
- timing or traffic-shape resistance.

## No Runtime Metadata Implementation Proof

NA-0314 changes only governance/evidence/testplan/traceability/journal content. It does not change runtime source, tests, scripts, fixtures, protocol files, qsc/qsp implementation, qsl-server, qsl-attachments, qsc-desktop, Cargo, workflow, README, START_HERE, public docs, website, branch-protection, or public-safety configuration.

## Backup-Plan Impact Statement

No backup-plan update is required for NA-0314. The durable evidence remains under qsl-protocol docs/tests/governance paths in `/srv/qbuild/work`, which are already within the expected qbuild backup scope. No new durable evidence location, source root, excluded path, or non-rebuildable artifact outside current scope is introduced.

## Next Recommendation

Close out NA-0314 after this transition plan merges and public-safety is green, then restore `NA-0315 -- Metadata Runtime Identifier and Default Padding Executable Harness Plan` as the sole READY successor. NA-0315 should first create a bounded runtime harness around qshield identifiers/default padding, then include qsl-tui/qsc checks only where exact files, no-mutation behavior, and no-claim boundaries remain enforceable.
