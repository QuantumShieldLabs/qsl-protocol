Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-21

# NA-0328 Metadata Runtime qshield Demo Bounded Jitter Authorization

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0328 authorizes a future bounded qshield embedded relay/demo jitter
implementation harness. This lane is authorization and design only. It does
not implement jitter, batching, cover traffic, queue scheduling, send
scheduling, receive scheduling, transport padding, qshield runtime behavior,
qsl-server behavior, qsl-attachments behavior, protocol behavior, crypto
behavior, workflow behavior, dependency changes, or public-safety changes.

The authorized future successor is:

`NA-0329 -- Metadata Runtime qshield Demo Bounded Jitter Implementation Harness`

The future lane is safe to authorize because NA-0325 ranked bounded jitter as
a plausible next low-to-medium latency qshield demo mitigation after retry
cadence normalization, and NA-0327 has now bounded the retry path that would
otherwise interact with jitter. The future implementation must remain opt-in,
qshield embedded relay/demo-only, deterministic under test mode, and bounded so
that it cannot weaken ack-after-verify, no-delete-before-verify, stale-ack
fail-closed, invalid-retry, or no-output-on-invalid invariants.

## Live NA-0328 Scope

The live queue item is `NA-0328 -- Metadata Runtime qshield Demo Bounded
Jitter Authorization Plan`, status `READY`, with goals G1 through G5.

Allowed work:

- produce a bounded qshield embedded relay/demo jitter authorization/design
  plan or exact prerequisite stop;
- review inherited NA-0327 retry-cadence proof and NA-0326/NA-0325 jitter
  context;
- define future jitter semantics, bounds, deterministic test mode, and marker
  plan;
- review abuse, DoS, latency, compatibility, reversibility, retry-cadence
  interaction, and ack/commit boundaries;
- preserve qshield embedded relay/demo versus qsl-server/qsl-attachments
  production boundaries;
- select one exact NA-0329 successor.

Forbidden work:

- bounded jitter implementation;
- runtime timing mitigation implementation;
- batching, cover traffic, broad queue scheduling, transport padding
  expansion, or service deployment behavior;
- qshield runtime implementation changes;
- qsl-server or qsl-attachments changes;
- qsc/qsp/protocol/crypto/key-schedule changes;
- Cargo or dependency changes;
- workflow, website, README, START_HERE, branch-protection, or public-safety
  configuration changes;
- claims that timing metadata or traffic shape is hidden;
- prohibited claims: anonymity, metadata-free, untraceable,
  production-readiness, public-internet-readiness, or
  external-review-complete.

## Inherited NA-0327 Retry-Cadence Proof

NA-0327 added an opt-in qshield embedded relay/demo retry-cadence policy named
`qshield_demo_retry_cadence_v1`. It uses a local demo ledger, deterministic
test mode, a 60 second local window, four invalid-candidate attempts per
window, and bounded delay classes of 0 ms, 500 ms, 1000 ms, and 2000 ms.

The executable harness proved:

- valid ack deletes exactly one queued candidate;
- duplicate valid ack and stale ack fail closed without deleting the wrong
  candidate;
- repeated candidate fetch does not delete before local verification;
- invalid receive attempts create no accepted local state and no plaintext
  output;
- empty poll cadence is bounded;
- invalid retry artifacts and output are secret-safe;
- qshield embedded relay/demo proof is distinct from qsl-server and
  qsl-attachments production timing.

This matters for bounded jitter because jitter must compose with the retry
ledger rather than bypass it. A future jitter implementation must not increase
the invalid-attempt cap, must not create a second retry loop, must not delay
the ack/delete decision in a way that changes ack-after-verify semantics, and
must not convert retry-cadence evidence into a production timing claim.

## Inherited NA-0326/0325 Jitter Context

NA-0325 ranked bounded jitter third behind retry cadence normalization and
padding bucket expansion for safe first authorization. Bounded jitter was
classified as low-to-medium latency and no direct bandwidth/storage cost, but
with correctness risk around randomness, deterministic CI assertions, retry
bounds, ack/commit timing, and claim safety.

NA-0325 deferred bounded jitter until after retry cadence normalization because
invalid retry behavior was a measured surface and because jitter without retry
bounds could amplify repeated invalid work. NA-0326 then authorized the retry
cadence lane, and NA-0327 implemented the bounded demo retry proof. That
sequence makes bounded jitter the next reasonable qshield demo-only
authorization candidate.

Neither NA-0325 nor NA-0326 authorizes production service timing mitigation,
qsl-server behavior, qsl-attachments behavior, cover traffic, batching, or a
claim that timing metadata or traffic shape is hidden.

## Sources Inspected

- `NEXT_ACTIONS.md` live NA-0328 entry.
- `docs/governance/evidence/NA-0327_metadata_runtime_qshield_demo_retry_cadence_normalization_harness.md`.
- `tests/NA-0327_metadata_runtime_qshield_demo_retry_cadence_normalization_harness_testplan.md`.
- `apps/qshield-cli/src/commands/recv.rs`.
- `apps/qshield-cli/src/commands/send.rs`.
- `apps/qshield-cli/src/commands/relay.rs`.
- `apps/qshield-cli/src/relay_client.rs`.
- `apps/qshield-cli/tests/na_0327_metadata_runtime_retry_cadence_normalization.rs`.
- `docs/governance/evidence/NA-0326_metadata_runtime_qshield_demo_retry_cadence_normalization_authorization.md`.
- `docs/governance/evidence/NA-0325_metadata_runtime_timing_traffic_mitigation_option_matrix.md`.
- `docs/governance/evidence/NA-0324_metadata_runtime_timing_traffic_instrumentation_harness.md`.
- `docs/governance/evidence/NA-0322_metadata_runtime_timing_traffic_measurement_harness.md`.
- `DECISIONS.md`.
- `TRACEABILITY.md`.

Search coverage included bounded jitter, jitter, retry cadence, invalid retry,
empty poll, valid ack, stale ack, duplicate ack, remote delete, local verify,
accepted state, output, qshield demo, qsl-server, qsl-attachments, production,
public-internet, metadata-free, anonymity, untraceable, external review,
timing-hidden, traffic-hidden, `FUTURE_GATE`, and `NOT_READY`.

## Bounded Jitter Semantic Design

The future implementation is authorized only for qshield embedded relay/demo
send and receive scheduling under an explicit opt-in policy. It must not
change protocol, crypto, qsc/qsp, qsl-server, qsl-attachments, dependencies,
workflow, branch protection, public-safety, README, START_HERE, website, or
production service behavior.

Future policy constants:

- policy name: `qshield_demo_bounded_jitter_v1`;
- opt-in switch: `QSHIELD_DEMO_BOUNDED_JITTER=1`;
- deterministic test mode: `QSHIELD_DEMO_BOUNDED_JITTER_TEST_MODE=1`;
- deterministic test seed: non-secret local test seed only, never protocol key
  material, route tokens, ack IDs, plaintext, or candidate bodies;
- jitter event classes: `send_submit`, `candidate_poll`, `empty_poll`, and
  `invalid_candidate_retry`;
- first implementation maximum added jitter: 250 ms per eligible event;
- minimum added jitter: 0 ms;
- total invalid/empty-poll delay when composed with NA-0327 retry cadence:
  retry delay plus jitter must remain bounded and test-visible, with a hard
  first-implementation cap of 2250 ms;
- ack-after-verify boundary: no added jitter between successful local
  verification and the remote ack/delete request in the first implementation
  harness;
- valid receive output boundary: no plaintext output before local verification
  and successful ack;
- invalid retry boundary: no jitter path may increase the NA-0327
  four-attempt invalid-candidate cap or skip the retry ledger;
- failure posture: if jitter state, seed parsing, delay selection, or
  deterministic-test evidence cannot be produced while the policy is enabled,
  fail closed with a coarse local demo error.

What a future implementation may change:

- add opt-in qshield demo-local jitter before `/send` submission;
- add opt-in qshield demo-local jitter before receive candidate polling;
- add deterministic test-mode delay selection and marker output;
- compose jitter with NA-0327 retry cadence for repeated invalid candidate and
  repeated empty poll classes without increasing retry attempts;
- record secret-safe test evidence for selected jitter classes.

What a future implementation must not change:

- ack/delete only after local verification succeeds;
- valid ack-once and stale/duplicate ack fail-closed behavior;
- no remote delete before local verification;
- no accepted local state and no plaintext output on invalid retry;
- retry-cadence caps and fail-closed behavior;
- sanitized error output;
- qsl-server or qsl-attachments production behavior;
- qsc/qsp/protocol/crypto/key-schedule behavior;
- dependencies, workflows, public-safety, branch protection, README,
  START_HERE, or website content.

## Future Implementation Boundary

Future allowed qshield files for NA-0329, if separately directed:

- `apps/qshield-cli/src/commands/send.rs`
- `apps/qshield-cli/src/commands/recv.rs`
- `apps/qshield-cli/src/relay_client.rs`
- `apps/qshield-cli/tests/na_0329_metadata_runtime_bounded_jitter.rs`
- existing qshield test helper code if needed and already local to
  `apps/qshield-cli/tests`.

Future forbidden files:

- `qsl-server/**`
- `qsl-attachments/**`
- `qsl/qsl-client/qsc/**`
- qsp, protocol, crypto, or key-schedule implementation paths;
- `Cargo.toml`
- `Cargo.lock`
- `.github/**`
- `website/**`
- `README.md`
- `START_HERE.md`
- `qsc-desktop/**`
- branch-protection or public-safety configuration.

Future proof requirements:

- valid send and receive still succeed under the opt-in policy;
- selected jitter delay is always within the configured bound;
- deterministic test mode proves exact selected delay classes without long
  sleeps;
- retry cadence and jitter composition preserves the invalid-attempt cap;
- no candidate is remotely deleted before local verification succeeds;
- valid ack deletes exactly one candidate after verification;
- stale and duplicate ack fail closed without state drift;
- invalid retry creates no accepted state and no plaintext output;
- jitter artifacts and logs contain no secrets;
- qsl-server and qsl-attachments production boundaries remain explicit;
- no claim says timing metadata or traffic shape is hidden.

## Abuse/DoS/Latency/Compatibility Matrix

| Scenario | Risk | Proposed bound | Future test | Failure mode | Stop condition | Compatibility impact | Claim boundary |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Valid send with jitter enabled | Jitter can add surprising latency or reorder user expectations. | 0-250 ms before demo `/send` submission; deterministic in test mode. | Send two messages and assert each selected delay is inside bounds and delivery succeeds. | Delay exceeds bound or send fails. | Any unbounded delay or send semantic drift. | Adds only opt-in local demo latency. | Does not hide total sends. |
| Candidate poll with jitter enabled | Receive polling can become slow or hide failure classes behind long waits. | 0-250 ms before candidate poll; no jitter after local verify before ack. | Poll candidates and prove delay class plus ack-after-verify unchanged. | Ack delayed after verify or poll loop blocks. | Any post-verify ack jitter in first implementation. | Valid receive remains compatible. | Poll timing remains observable. |
| Empty poll plus retry cadence | Empty queue polling can amplify load and exact cadence. | NA-0327 retry delay plus at most 250 ms jitter, hard cap 2250 ms. | Repeat empty polls in deterministic mode and assert composed classes. | Busy loop, cap bypass, or hidden long sleep. | Retry cap or jitter cap exceeded. | Empty poll remains successful. | Empty state remains observable. |
| Invalid candidate retry plus jitter | Attacker can force repeated local work and exploit jitter as another loop. | NA-0327 four-attempt cap remains authoritative; jitter adds at most 250 ms after accepted retry attempt. | Queue invalid candidate and assert attempts, cap, state, output, and delay classes. | More than four invalid attempts or accepted state. | Any retry cap bypass. | Preserves invalid-retain behavior. | Reject remains visible. |
| Valid ack after verify | Jitter could delay delete and prolong retention after plaintext is proven. | No added jitter between successful local verification and ack in first implementation. | Valid receive harness asserts ack timing class is unchanged by jitter policy. | Ack/delete delayed by jitter. | Any post-verify ack jitter. | Preserves NA-0318/NA-0327 ack boundary. | No production proof. |
| Duplicate/stale ack | Jitter could mask a replay loop or make fail-closed behavior nondeterministic. | No retry loop; stale and duplicate ack fail closed as NA-0327 proved. | Replay duplicate and stale ack under jitter policy. | Duplicate success or wrong delete. | Any state mutation on stale/duplicate ack. | Preserves one-delete semantics. | Ack failure remains visible. |
| Multiple queued candidates | Jitter could alter ordering or starve valid candidates behind invalid ones. | No queue reordering; no delete before verify; invalid cap remains. | Queue invalid plus valid candidates and assert no reordering or wrong delete. | Candidate order drift or wrong candidate accepted. | Any reordering beyond explicit future directive. | Preserves current queue semantics. | Queue shape remains observable. |
| Deterministic CI | Random jitter can make tests flaky or long. | Test mode records deterministic selected delays and may skip actual sleeps. | Run harness repeatedly with fixed seed and assert stable markers. | Flaky delay selection or long sleep in CI. | Non-deterministic CI proof. | Keeps CI bounded. | Test mode is not production proof. |
| Secret-bearing artifacts | Jitter evidence could record route tokens, ack IDs, plaintext, or keys. | Record only class labels, bounds, and non-secret local tags. | Scan output and artifacts for sentinels and sensitive shapes. | Secret-like artifact text. | Any secret finding. | No user-visible behavior change. | Evidence remains safe. |
| qsl-server production equivalent | Production relay timing includes auth, queues, logs, rate limits, deployment, and public-internet exposure. | No qsl-server implementation in NA-0329. | Future qsl-server lane only. | qshield proof presented as service proof. | Any qsl-server path change or production timing claim. | None in qsl-protocol. | Production timing unproven. |
| qsl-attachments production equivalent | Object upload/fetch timing and size classes are separate service behavior. | No qsl-attachments implementation in NA-0329. | Future qsl-attachments lane only. | qshield proof presented as attachment proof. | Any qsl-attachments path change or production timing claim. | None in qsl-protocol. | Attachment timing unproven. |
| Cover traffic or batching escalation | Jitter can be overgeneralized into broader traffic-shaping. | Not authorized by NA-0329. | Scope guard excludes batching/cover traffic paths. | Dummy traffic, queue batching, or padding expansion appears. | Any batching or cover traffic implementation. | No direct impact. | No metadata-free claim. |

## Future Validation/Marker Plan

NA-0329 marker candidates:

- `NA0329_BOUNDED_JITTER_AUTHORIZATION_OK`
- `NA0329_JITTER_POLICY_BOUND_OK`
- `NA0329_DETERMINISTIC_JITTER_TEST_MODE_OK`
- `NA0329_SEND_JITTER_BOUNDS_OK`
- `NA0329_RECEIVE_POLL_JITTER_BOUNDS_OK`
- `NA0329_RETRY_CADENCE_INTERACTION_OK`
- `NA0329_VALID_SEND_RECEIVE_COMPATIBILITY_OK`
- `NA0329_ACK_AFTER_VERIFY_UNCHANGED_OK`
- `NA0329_VALID_ACK_ONCE_OK`
- `NA0329_STALE_DUPLICATE_ACK_FAIL_CLOSED_OK`
- `NA0329_NO_REMOTE_DELETE_BEFORE_VERIFY_OK`
- `NA0329_NO_ACCEPTED_STATE_ON_INVALID_RETRY_OK`
- `NA0329_NO_OUTPUT_ON_INVALID_RETRY_OK`
- `NA0329_NO_SECRET_JITTER_ARTIFACT_OK`
- `NA0329_QSHIELD_DEMO_BOUNDARY_OK`
- `NA0329_SERVICE_PRODUCTION_BOUNDARY_OK`
- `NA0329_NO_TIMING_HIDDEN_CLAIM_OK`
- `NA0329_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK`
- `NA0329_NO_METADATA_FREE_CLAIM_OK`
- `NA0329_METADATA_RUNTIME_BOUNDED_JITTER_OK`

The future harness should preserve inherited NA-0318 through NA-0327
assertions or equivalent checks for ack-after-verify, invalid candidate
retention, sanitized errors, padding/identifier rejects, measurement and
instrumentation boundaries, retry-cadence caps, and retry artifact safety.

## Production Boundary

### qshield Embedded Relay/Demo

The future NA-0329 implementation harness may change only opt-in qshield
embedded relay/demo jitter behavior under a separate exact directive. Local
qshield evidence can prove selected demo jitter bounds, deterministic test
mode, and invariant preservation. It cannot prove production service timing.

### qsl-server

qsl-server production jitter or service timing mitigation requires a separate
cross-repo lane. That lane must handle service queues, auth, rate/global caps,
logs, metrics, deployment topology, reverse proxies, backup artifacts,
support bundles, and public-internet exposure before any production timing
statement changes.

### qsl-attachments

qsl-attachments production upload/fetch jitter, retention timing, object-size
classes, and quota behavior require a separate cross-repo lane. That lane must
handle capabilities, opaque object storage, quotas, cleanup, recovery, support
bundles, backup artifacts, and service logs.

## External-Review Sensitivity

External review is recommended before stronger timing or traffic-shape claims,
production service mitigation, attachment-size class handling, batching, cover
traffic, broad queue scheduling, or public-internet statements. NA-0328 does
not claim external review completion.

The selected future qshield demo bounded-jitter harness has moderate
external-review sensitivity because it can remain local/demo-only and bounded,
but any claim beyond local demo jitter bounds requires later review and
authorization.

## Public Claim Boundary

NA-0328 permits only these claim-safe statements:

- the bounded-jitter authorization plan exists;
- the plan is grounded in NA-0325 option-matrix evidence and NA-0327
  retry-cadence proof;
- bounded jitter is authorized only for a future qshield embedded relay/demo
  implementation harness;
- the future implementation must prove jitter bounds, deterministic test mode,
  retry-cadence interaction, ack-after-verify preservation, no-delete before
  verify, no accepted state or output on invalid retry, and no secret
  artifacts;
- qsl-server and qsl-attachments production timing remain unproven and
  future-gated.

NA-0328 does not permit a claim of anonymity, metadata-free behavior,
untraceability, production readiness, public-internet readiness, external
review completion, hidden timing metadata, hidden traffic shape, or implemented
mitigation.

## Selected Successor

`NA-0329 -- Metadata Runtime qshield Demo Bounded Jitter Implementation Harness`

Rationale:

- NA-0327 has already bounded the retry cadence surface that interacts with
  jitter;
- the future semantics above are exact enough to test without changing
  service or protocol semantics;
- abuse and latency bounds are explicit;
- deterministic test mode keeps CI bounded and reproducible;
- compatibility invariants from NA-0318 through NA-0327 remain protected;
- the future implementation files can be bounded to qshield demo paths plus a
  qshield test harness.

## Rejected Alternatives

- Direct implementation in NA-0328: rejected because this lane is
  authorization/design only.
- More detailed design before implementation: rejected because the policy
  name, opt-in boundary, maximum jitter, deterministic test mode,
  retry-cadence interaction, file boundary, abuse matrix, and marker plan are
  precise enough for a future implementation harness.
- qsl-server production jitter as NA-0329: rejected because production service
  timing requires cross-repo authorization.
- qsl-attachments production timing or object-size jitter as NA-0329:
  rejected because attachment service timing requires cross-repo
  authorization.
- Cover traffic or batching: rejected as the immediate successor because both
  have broader cost, abuse, dummy-state, ordering, and external-review gates.
- Claiming metadata-free behavior: rejected because current evidence proves
  observable timing and traffic-shape surfaces remain.

## Backup-Plan Impact Statement

No backup-plan update is required for NA-0328. Tracked evidence remains under
the qsl-protocol worktree inside `/srv/qbuild/work`, which is already in the
operator backup scope. NA-0328 creates no durable evidence location outside
the current backup scope and no non-rebuildable artifact outside the repo. The
D132 preservation bundle is not deleted or modified.

## Next Recommendation

Merge this authorization plan, keep NA-0328 READY until the closeout PR, then
restore exactly one successor:

`NA-0329 -- Metadata Runtime qshield Demo Bounded Jitter Implementation Harness`

NA-0329 must not be implemented by NA-0328 closeout. The future NA-0329
directive should implement only the bounded qshield demo jitter harness or
stop on an exact prerequisite.
