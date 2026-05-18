Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-18

# NA-0315 Metadata Runtime Identifier and Default Padding Executable Harness Plan

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0315 defines the next executable metadata runtime harness plan for
identifier/handle rotation and default padding. It also adds a bounded
non-runtime plan harness that validates the fixture describing allowed files,
runtime surfaces, expected future markers, claim boundaries, and stop
conditions.

NA-0315 does not implement metadata runtime behavior. Runtime
identifier/handle rotation and runtime default padding remain unimplemented.
The selected successor is:

`NA-0316 -- Metadata Runtime qshield Poll No-Mutation Blocker Resolution`

That successor is required because qshield relay polling removes queued
messages before local receive-side padding strip or decode failure can reject
the message. A future runtime no-mutation proof must resolve or explicitly
scope that behavior before the combined identifier/default-padding harness can
truthfully run.

## Live NA-0315 Scope

The live queue entry is `NA-0315 -- Metadata Runtime Identifier and Default
Padding Executable Harness Plan`, status `READY`, with goals G1 through G5.

Allowed by live scope:

- create a bounded executable harness plan;
- add exact files and behavior only when the live directive authorizes them;
- identify runtime identifier/handle and default-padding proof surfaces;
- produce executable proof or stop with exact prerequisite evidence;
- keep all metadata runtime gaps and public claim boundaries visible.

Forbidden by live scope:

- metadata runtime behavior implementation;
- identifier/handle rotation runtime implementation;
- default padding runtime implementation;
- protocol, crypto, qsp, key schedule, qsl-server, qsl-attachments,
  qsc-desktop, website, README, START_HERE, workflow, Cargo/dependency,
  branch-protection, or public-safety mutation;
- any claim of anonymity, metadata-free behavior, untraceability,
  public-internet readiness, production readiness, or external review
  completion.

## Sources Inspected

- `NEXT_ACTIONS.md` live NA-0315 entry.
- `tests/NA-0314_closeout_restore_na0315_testplan.md`.
- `docs/governance/evidence/NA-0314_metadata_runtime_identifier_padding_transition_plan.md`.
- `tests/NA-0314_metadata_runtime_identifier_padding_transition_testplan.md`.
- `docs/governance/evidence/NA-0291_metadata_phase2_identifier_padding_harness.md`.
- `docs/governance/evidence/NA-0293_metadata_phase2_sanitized_errors_retention_harness.md`.
- `scripts/ci/metadata_phase2_identifier_padding_harness.sh`.
- `scripts/ci/metadata_phase2_sanitized_errors_retention_harness.sh`.
- `scripts/ci/metadata_conformance_smoke.sh`.
- `apps/qshield-cli/src/commands/{init,register,establish,send,recv,relay,attachment}.rs`.
- `apps/qshield-cli/src/{config,relay_client,store}.rs`.
- `apps/qsl-tui/src/demo.rs` and qsl-tui metadata tests.
- `qsl/qsl-client/qsc/src/{contacts,transport}/mod.rs`.
- `qsl/qsl-client/qsc/src/tui/controller/state/ownership.rs`.
- `TRACEABILITY.md`, `DECISIONS.md`, and
  `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

## Inherited NA-0314 Transition

NA-0314 established that prior NA-0291 and NA-0293 executable fixtures are
policy-fixture proof, not runtime metadata reduction. It selected NA-0315 to
produce an exact executable harness plan for runtime identifier/handle rotation
and default padding while preserving the known gaps:

- identifier/handle rotation runtime;
- default padding runtime;
- broader sanitized-error runtime expansion;
- retention/purge runtime behavior;
- timing/traffic-shape threat modeling;
- deployment metadata posture;
- public-internet metadata behavior.

NA-0314 also called out the qshield poll/remove behavior as a possible
no-mutation blocker. NA-0315 confirms that risk from live source inspection.

## Candidate Runtime Surfaces

| Surface | Files | Current status | Future proof need |
| --- | --- | --- | --- |
| qshield peer and bundle identifiers | `apps/qshield-cli/src/commands/register.rs`, `establish.rs`, `store.rs` | Runtime exists; rotation not proven | stale/malformed/wrong-scope handle reject, opaque boundary, no accepted-state mutation, no raw handle output |
| qshield session identifiers | `apps/qshield-cli/src/commands/establish.rs`, `store.rs` | Runtime session values exist; rotation not proven | session-bound handle fixture, replay/stale reject, no raw session handle in logs |
| qshield relay queue and poll handles | `apps/qshield-cli/src/commands/relay.rs`, `recv.rs`, `relay_client.rs` | Runtime queue exists; poll removes before local decode | blocker resolution before receive-side no-mutation proof |
| qshield default padding | `apps/qshield-cli/src/commands/init.rs`, `send.rs`, `recv.rs`, `relay.rs`, `config.rs` | Optional padding exists; default runtime not implemented | default policy, buckets, invalid config reject, strip/verify, malformed padded input reject |
| qshield attachment handles | `apps/qshield-cli/src/commands/attachment.rs`, `send.rs`, `recv.rs` | Attachment demo proof exists; rotation not proven | attachment handle stale/malformed reject and no secret output if in later scope |
| qsl-tui padded demo metadata | `apps/qsl-tui/src/demo.rs`, qsl-tui metadata tests | Padded mode exists; default runtime not proven | bucket reporting and explicit non-metadata-free boundary |
| qsc route/contact handles | `qsl/qsl-client/qsc/src/contacts/mod.rs`, `transport/mod.rs`, `tui/controller/state/ownership.rs` | Route-token and contact/device handles exist | no raw token output, stale/malformed reject, no accepted-state mutation |

## Candidate Harness Surfaces

NA-0315 adds these non-runtime harness-planning artifacts:

- `scripts/ci/metadata_runtime_identifier_padding_harness_plan.sh`
- `inputs/metadata_runtime/identifier_padding_runtime_fixture_v1.json`

The script validates that the fixture records:

- selected scope and successor;
- allowed and forbidden files;
- candidate runtime surfaces;
- identifier/handle and padding plan requirements;
- qshield poll no-mutation blocker evidence;
- future runtime markers;
- claim boundaries and stop conditions.

The script emits only plan-level markers. It does not emit runtime proof markers.

## Selected Harness Scope

Selected scope:

`non_runtime_executable_harness_plan`

Allowed files in NA-0315:

- `scripts/ci/metadata_runtime_identifier_padding_harness_plan.sh`
- `inputs/metadata_runtime/identifier_padding_runtime_fixture_v1.json`
- `tests/NA-0315_metadata_runtime_identifier_padding_executable_harness_plan_testplan.md`
- `docs/governance/evidence/NA-0315_metadata_runtime_identifier_padding_executable_harness_plan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden files include runtime source, protocol/crypto/qsp/qsc implementation,
service repositories, public docs, README, START_HERE, workflows, Cargo files,
branch-protection, and public-safety configuration.

Rationale:

- live NA-0315 authorizes exact executable proof or exact prerequisite stop;
- the directive authorizes a non-runtime plan fixture and script;
- runtime changes are not needed to prove the plan;
- live source inspection found a qshield poll no-mutation blocker that should
  be resolved before the combined runtime harness executes.

## Identifier/Handle Harness Plan

Future identifier proof must cover:

- deterministic fixture identifiers for peer, bundle, session, route, queue,
  message, attachment, and contact/device classes;
- opaque handle boundary, with no raw route token, bearer token, stable
  identity, or internal path used as a public handle;
- stale, malformed, replayed, and wrong-scope handle reject;
- no accepted-state mutation on reject;
- no raw handle, route token, bearer token, plaintext sentinel, or internal
  path in stdout, stderr, logs, or artifacts.

Future identifier markers:

- `NA0315_IDENTIFIER_ROTATION_POLICY_OK`
- `NA0315_OPAQUE_HANDLE_BOUNDARY_OK`
- `NA0315_STALE_HANDLE_REJECT_OK`
- `NA0315_IDENTIFIER_NO_MUTATION_ON_REJECT_OK`
- `NA0315_IDENTIFIER_NO_SECRET_LOG_OK`

These markers are future-gated by the qshield poll no-mutation blocker and are
not emitted by the NA-0315 plan harness.

## Default Padding Harness Plan

Future padding proof must cover:

- default padding profile `metadata-runtime-default-padding-v1`;
- bucket table `[512, 1024, 2048, 4096, 8192]` unless a future directive
  explicitly changes the table;
- invalid bucket configuration reject;
- strip/verify fixture for valid padded input;
- malformed padded input, bucket mismatch, too-large pad length, and over-limit
  payload reject;
- no accepted-state mutation on reject;
- no raw plaintext, plaintext sentinel, padding sentinel, or unscoped exact
  sensitive length in stdout, stderr, logs, or artifacts.

Future padding markers:

- `NA0315_DEFAULT_PADDING_POLICY_OK`
- `NA0315_PADDING_BUCKETS_OK`
- `NA0315_PADDING_INVALID_CONFIG_REJECT_OK`
- `NA0315_PADDING_STRIP_VERIFY_OK`
- `NA0315_PADDING_NO_MUTATION_ON_REJECT_OK`
- `NA0315_PADDING_NO_SECRET_LOG_OK`

These markers are future-gated by the qshield poll no-mutation blocker and are
not emitted by the NA-0315 plan harness.

## qshield Poll / No-Mutation Risk Decision

Classification:

`STOP_RISK_FOR_RUNTIME_NO_MUTATION_PROOF`

Evidence:

- `apps/qshield-cli/src/commands/relay.rs` handles `/poll` by calling
  `pop_front()` on queued entries before returning them to the caller.
- `apps/qshield-cli/src/commands/recv.rs` performs padding strip/verify and
  actor decode only after the relay poll response is returned.
- A malformed padded or undecodable message can therefore be removed from the
  remote queue before local reject.

Decision:

Do not run the combined runtime identifier/default-padding harness yet. Select
NA-0316 as a qshield poll no-mutation blocker resolution lane. That lane should
decide whether the runtime proof boundary is queue-preserving, ack/commit based,
or explicitly scoped to local accepted-state only. Until that decision is made,
the combined runtime harness would risk false no-mutation evidence.

## Harness Markers

Plan-harness markers emitted by NA-0315:

- `NA0315_SCOPE_DECISION_PLAN_HARNESS_OK`
- `NA0315_RUNTIME_SURFACES_INVENTORIED_OK`
- `NA0315_QSHIELD_POLL_NO_MUTATION_BLOCKER_RECORDED_OK`
- `NA0315_REQUIRED_FUTURE_MARKERS_RECORDED_OK`
- `NA0315_METADATA_RUNTIME_HARNESS_PLAN_OK`

Runtime markers recorded but not emitted by NA-0315:

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

## Fixture And Script Paths

- Fixture: `inputs/metadata_runtime/identifier_padding_runtime_fixture_v1.json`
- Script: `scripts/ci/metadata_runtime_identifier_padding_harness_plan.sh`

These are deterministic, local, non-network artifacts. The script uses the
standard Python interpreter already used by repository CI helpers and does not
add dependencies.

## Blockers

Primary blocker:

- qshield `/poll` removes queued messages before local `recv` padding/decode
  reject. This blocks truthful receive-side queue no-mutation proof.

Residual future gaps:

- runtime identifier/handle rotation remains unimplemented;
- runtime default padding remains unimplemented;
- broader runtime sanitized-error expansion remains open;
- retention/purge runtime behavior remains open;
- timing/traffic-shape threat modeling remains open;
- deployment and public-internet metadata behavior remain open.

## Selected Successor

Selected successor:

`NA-0316 -- Metadata Runtime qshield Poll No-Mutation Blocker Resolution`

Rationale:

- qshield is the first runtime surface where identifier and padding proof meet;
- qshield poll/remove behavior directly affects receive-side no-mutation proof;
- resolving that blocker first avoids a combined harness that can only pass by
  narrowing or hiding the queue mutation boundary.

Rejected successors:

- Combined runtime identifier/default-padding harness: blocked until qshield
  poll no-mutation semantics are resolved.
- Identifier-only harness: viable later, but still touches qshield poll/route
  semantics if receive-side stale or wrong-scope cases are included.
- Padding-only harness: viable later, but malformed padded receive input is the
  clearest poll/remove no-mutation blocker.

## Claim Boundaries

NA-0315 does not claim:

- anonymity;
- metadata-free behavior;
- untraceability;
- public-internet readiness;
- production readiness;
- external review completion;
- complete metadata reduction;
- runtime identifier/handle rotation;
- runtime default padding.

Existing NA-0291 and NA-0293 fixture evidence remains fixture evidence only.

## No Runtime Metadata Implementation Proof

NA-0315 changes only:

- a non-runtime plan script;
- a deterministic plan fixture;
- governance evidence and testplan files;
- decision, traceability, and rolling journal text.

It does not change qshield, qsl-tui, qsc, qsp, protocol-core, crypto
state-machine, key schedule, service, qsl-server, qsl-attachments,
qsc-desktop, website, workflow, Cargo/dependency, README, START_HERE,
branch-protection, or public-safety configuration paths.

## Backup-Plan Impact Statement

No backup-plan update is required. New durable artifacts are under the
qsl-protocol worktree within `/srv/qbuild/work`, using repository docs, tests,
scripts, and inputs paths already covered by the current qbuild backup scope.

## Next Recommendation

Run NA-0316 as a narrow qshield poll no-mutation blocker resolution. The first
task should be to define the future no-mutation boundary for polled messages:
queue-preserving until local decode succeeds, explicit ack/commit semantics, or
a documented local-only boundary that does not overstate remote queue behavior.
