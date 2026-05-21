Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-21

# NA-0330 Metadata Runtime qshield Demo Batching Authorization

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0330 authorizes a future bounded qshield embedded relay/demo batching
implementation harness and selects the exact successor:

`NA-0331 -- Metadata Runtime qshield Demo Batching Implementation Harness`

This lane is authorization and design only. It does not implement batching,
cover traffic, runtime timing mitigation, broad queue scheduling, send
scheduling, receive scheduling, transport padding, qshield runtime behavior,
qsl-server behavior, qsl-attachments behavior, qsc/qsp/protocol/crypto
behavior, workflow behavior, dependency changes, branch-protection changes, or
public-safety changes.

The future implementation is safe to authorize only as bounded qshield
embedded relay/demo evidence because NA-0327 already proved retry-cadence caps
and NA-0329 already proved bounded-jitter compatibility with no remote delete
before local verification. The future batching harness must preserve ordering,
ack-after-verify, retry-cadence, bounded-jitter, retention/purge, artifact
safety, and claim-safety boundaries.

This evidence does not claim that timing metadata or traffic shape is hidden.
This evidence does not claim anonymity, metadata-free behavior, untraceable
behavior, production readiness, public-internet readiness, or external-review
completion. Batching is not implemented by NA-0330.

## Live NA-0330 Scope

Live `NEXT_ACTIONS.md` entry:

- `NA-0330 -- Metadata Runtime qshield Demo Batching Authorization Plan`
- Status: READY
- Goals: G1, G2, G3, G4, G5
- Objective: execute the next timing/traffic-shape mitigation/design lane
  selected by NA-0329 evidence: an authorization plan for qshield embedded
  relay/demo batching, or stop on an exact prerequisite.

Allowed work for NA-0330:

- produce a qshield embedded relay/demo batching authorization/design plan or
  exact prerequisite stop;
- review inherited NA-0329 bounded-jitter proof and NA-0327 retry-cadence
  proof;
- define future batching semantics, allowed implementation files, validation
  markers, and claim boundaries;
- review abuse, DoS, latency, compatibility, reversibility, retry-cadence,
  bounded-jitter interaction, and ack/commit boundaries;
- preserve qshield embedded relay/demo versus qsl-server/qsl-attachments
  production boundaries;
- select one exact NA-0331 successor.

Forbidden work preserved by NA-0330:

- batching implementation;
- cover traffic implementation;
- runtime timing mitigation implementation;
- broad queue scheduling, send scheduling, receive scheduling, or transport
  padding implementation;
- qshield runtime source changes;
- qsl-server or qsl-attachments changes;
- qsc/qsp/protocol/crypto/key-schedule changes;
- Cargo or dependency changes;
- workflow, website, README, START_HERE, docs/public, branch-protection, or
  public-safety configuration changes;
- any claim that timing metadata or traffic shape is hidden;
- prohibited claims: anonymity, metadata-free, untraceable,
  production-readiness, public-internet-readiness, or
  external-review-complete.

## Inherited NA-0329 Bounded-Jitter Proof

NA-0329 D-0640 implemented only opt-in qshield embedded relay/demo bounded
jitter in the qshield receive retry ledger:

- policy name: `qshield_demo_bounded_jitter_v1`;
- opt-in switch: `QSHIELD_DEMO_BOUNDED_JITTER=1`;
- deterministic test mode: `QSHIELD_DEMO_BOUNDED_JITTER_TEST_MODE=1`;
- maximum added jitter: 250 ms;
- invalid/empty-poll retry plus jitter cap: 2250 ms;
- no added jitter between successful local verification and remote ack/delete;
- no increase to the NA-0327 invalid-candidate attempt cap;
- no accepted local state and no plaintext output on invalid jitter;
- no secret jitter artifacts;
- qshield embedded relay/demo boundary only.

NA-0329 did not implement batching, cover traffic, broad scheduling,
qsl-server behavior, qsl-attachments behavior, production-service timing, or
any claim that timing metadata or traffic shape is hidden.

## Inherited NA-0327 Retry-Cadence Proof

NA-0327 D-0636 added the opt-in `qshield_demo_retry_cadence_v1` receive retry
ledger:

- invalid candidate attempts are capped at four per 60 second local window;
- retry classes remain 0 ms, 500 ms, 1000 ms, and 2000 ms;
- empty poll remains bounded at the 2000 ms retry class;
- valid ack deletes exactly one candidate;
- duplicate and stale ack attempts fail closed or no-op deterministically;
- invalid retries do not delete remote candidates before local verification;
- invalid retries produce no accepted local state and no plaintext output;
- retry artifacts avoid route tokens, raw handles, raw ack IDs, plaintext,
  padding sentinels, passphrases, raw key material, panic text, and backtraces.

Future batching must compose with this ledger rather than replacing it.

## Sources Inspected

- `NEXT_ACTIONS.md`
- `GOALS.md`
- `PROJECT_CHARTER.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `CHECKLIST_PROTOCOL_CHANGE.md`
- `tests/NA-0329_closeout_restore_na0330_testplan.md`
- `docs/governance/evidence/NA-0329_metadata_runtime_qshield_demo_bounded_jitter_harness.md`
- `tests/NA-0329_metadata_runtime_qshield_demo_bounded_jitter_harness_testplan.md`
- `docs/governance/evidence/NA-0328_metadata_runtime_qshield_demo_bounded_jitter_authorization.md`
- `docs/governance/evidence/NA-0327_metadata_runtime_qshield_demo_retry_cadence_normalization_harness.md`
- `tests/NA-0327_metadata_runtime_qshield_demo_retry_cadence_normalization_harness_testplan.md`
- `docs/governance/evidence/NA-0325_metadata_runtime_timing_traffic_mitigation_option_matrix.md`
- `docs/governance/evidence/NA-0324_metadata_runtime_timing_traffic_instrumentation_harness.md`
- `apps/qshield-cli/src/commands/send.rs`
- `apps/qshield-cli/src/commands/recv.rs`
- `apps/qshield-cli/src/commands/relay.rs`
- `apps/qshield-cli/src/relay_client.rs`
- `apps/qshield-cli/tests/na_0329_metadata_runtime_bounded_jitter.rs`
- `apps/qshield-cli/tests/na_0327_metadata_runtime_retry_cadence_normalization.rs`
- qshield NA-0318, NA-0319, NA-0320, NA-0322, and NA-0324 harnesses
- metadata runtime and metadata phase-2 harnesses
- qsc send_commit, formal model, NA-0310 oracle, and NA-0313 harness evidence

Search coverage included batching, batch, batch window, batch size, queue
scheduling, bounded jitter, retry cadence, ack, candidate, ordering, latency,
compatibility, qshield demo, qsl-server, qsl-attachments, production,
and prohibited phrase families such as timing-hidden, traffic-hidden,
metadata-free, anonymity, untraceable, `FUTURE_GATE`, and `NOT_READY`.

## Batching Problem Statement

NA-0324 instrumentation and NA-0325 option analysis showed that qshield demo
send, candidate fetch, ack/commit, invalid retry, queue depth, padding-size
class, and ordering/correlation events remain observable. NA-0327 and NA-0329
bounded selected retry and jitter behavior, but no batching implementation
exists.

The next safe batching step is not a production service change. It is a
bounded qshield embedded relay/demo harness that proves exact semantics for
small batches without weakening fail-closed validation or overclaiming privacy.

Batching may reduce exact local/demo event regularity inside a bounded profile,
but it does not hide timing metadata, traffic shape, endpoints, volume, queue
existence, batch size, batch windows, size buckets, service logs, deployment
metadata, or attachment/object behavior.

## Batching Semantic Design

Future policy constants:

- policy name: `qshield_demo_batching_v1`;
- opt-in switch: `QSHIELD_DEMO_BATCHING=1`;
- deterministic test mode: `QSHIELD_DEMO_BATCHING_TEST_MODE=1`;
- deterministic logical clock: `QSHIELD_DEMO_BATCHING_NOW_MS`;
- maximum batch size: 4 members;
- maximum send-side batch window: 750 ms;
- receive candidate batch size: at most 4 candidates from one
  `/poll-candidate` response;
- ack batch size: at most 4 locally verified ack IDs;
- no long sleeps in deterministic test mode;
- no batch wait may be added after successful local verification and before
  ack/delete in the first implementation harness;
- no receive-side wait may extend NA-0327 retry cadence or NA-0329
  retry-plus-jitter caps in the first implementation harness.

Future batch target:

- send-side qshield demo messages through a demo-only batch submission path;
- receive candidate processing for up to four candidates already returned by
  the embedded relay;
- invalid retry handling through the existing NA-0327 retry ledger;
- ack/commit grouping only for ack IDs that were locally verified before the
  batch ack request is built;
- attachments remain future-gated unless a later directive explicitly opens a
  qsl-attachments cross-repo lane.

Future send batch semantics:

- A future `send_batch` request contains 1 to 4 already encrypted qshield demo
  message members.
- Each member keeps independent `to`, `from`, `msg`, `pad_len`, and `bucket`
  fields; no protocol wire format or crypto changes are allowed.
- The relay must validate every member and all queue/rate-limit capacity before
  mutating state.
- If any member is malformed, exceeds limits, fails padding metadata checks, or
  would exceed queue/rate limits, the whole send batch rejects and no member is
  enqueued.
- If all members validate, the relay enqueues all members in request order,
  preserving per-recipient/per-session order.
- Cross-recipient ordering must be explicitly non-promised; if a future
  implementation cannot prove an ordering class, it must fail closed or narrow
  the supported batch shape.

Future receive/candidate batch semantics:

- Candidate batching uses the queue-preserving candidate path, not destructive
  legacy polling.
- A receive batch processes at most four candidates returned by one
  `/poll-candidate` call.
- Candidates are processed in relay order.
- The receiver must stop at the first candidate whose local verification,
  padding strip, session lookup, actor decode, or output boundary fails.
- Valid candidates before the first invalid member may be acked and output in
  order only after local verification succeeds.
- The invalid member and all later members must remain remotely queued and
  must not create accepted local state or plaintext output.
- If the invalid member is first, no batch member is acked or output.

Future ack/commit semantics:

- Ack/commit remains after local verification only.
- A future ack batch may contain only verified ack IDs for one recipient.
- The relay must validate all ack IDs exist before mutating queue state.
- If any ack ID is malformed, stale, duplicate, missing, or belongs to another
  recipient, the ack batch rejects and deletes nothing.
- If all ack IDs validate, the relay deletes exactly those candidates once and
  leaves all other candidates unchanged.
- A future implementation may choose individual ack requests instead of
  ack-batch requests for the first harness, but it must still emit marker proof
  that batching did not weaken ack-after-verify or no-delete-before-verify.

Future retry-cadence and bounded-jitter interaction:

- The NA-0327 four-attempt invalid-candidate cap per 60 second local window
  remains authoritative.
- Invalid batch members consume the same local candidate-tag retry ledger
  attempt that a non-batched invalid candidate would consume.
- Empty poll retry remains successful and bounded at the existing retry class.
- NA-0329 retry-plus-jitter cap of 2250 ms remains unchanged for invalid and
  empty-poll classes.
- Send-side batch window delay is separate from invalid/empty retry cadence and
  must not be added to the invalid/empty retry cap in the first harness.
- Deterministic test mode must record logical delay/window classes without real
  sleeps.

Future retention and purge semantics:

- Any local demo batch staging file must live under the qshield demo store with
  the policy name `.qshield_demo_batching_v1.json`.
- The staging file may contain only encrypted relay member bodies, coarse
  class labels, logical timestamps, and non-secret local tags needed for
  deterministic proof.
- It must not contain route tokens, auth headers, passphrases, raw key
  material, plaintext, raw output, raw ack IDs in evidence artifacts, or panic
  backtraces.
- Pending staged entries must be purged after successful flush.
- Entries older than the 750 ms logical window must flush or be discarded
  fail-closed on the next batching operation; stale local staging must not be
  silently sent after an ambiguous resume.
- If staging state cannot be read, parsed, serialized, or written while the
  policy is enabled, qshield must fail closed with a coarse local demo error.

Future no-output/no-state boundaries:

- No invalid send batch member may enqueue any remote message.
- No invalid receive batch member may create accepted local state, plaintext
  output, or remote delete.
- No later member after an invalid receive member may be processed in the same
  batch.
- No invalid ack batch member may delete any candidate.
- No secret batch artifact may be written to governance evidence, test output,
  logs, or durable non-store artifacts.

## Future Implementation Boundary

Future allowed qshield files for NA-0331, only if separately directed:

- `apps/qshield-cli/src/commands/send.rs`
- `apps/qshield-cli/src/commands/recv.rs`
- `apps/qshield-cli/src/commands/relay.rs`
- `apps/qshield-cli/src/relay_client.rs`
- `apps/qshield-cli/tests/na_0331_metadata_runtime_batching.rs`
- existing qshield test helper paths if already established under
  `apps/qshield-cli/tests`

Future forbidden files:

- `qsl-server/**`
- `qsl-attachments/**`
- `qsc/**`
- `qsp/**`
- qsl-client qsc runtime implementation paths
- protocol, crypto, or key-schedule implementation paths
- `Cargo.toml`
- `Cargo.lock`
- `.github/**`
- `website/**`
- `README.md`
- `START_HERE.md`
- `qsc-desktop/**`
- branch-protection or public-safety configuration

What a future implementation would change:

- add an opt-in qshield demo batching policy and deterministic test mode;
- add bounded relay/client support for 1 to 4 member send batches;
- optionally add bounded relay/client support for verified ack batches;
- process receive candidate batches from the existing queue-preserving
  candidate path;
- record secret-safe batch markers and local demo evidence.

What a future implementation must not change:

- protocol wire format, crypto, key schedule, negotiation, or qsc/qsp state
  machine semantics;
- qsl-server or qsl-attachments production behavior;
- default qshield behavior when `QSHIELD_DEMO_BATCHING` is not enabled;
- ack/delete before local verification;
- valid ack-once behavior;
- stale/duplicate ack fail-closed behavior;
- no remote delete before local verification;
- retry-cadence caps and fail-closed behavior;
- bounded-jitter caps;
- sanitized-error, retention, and purge boundaries;
- dependency, workflow, branch-protection, public-safety, website, README,
  START_HERE, or docs/public content.

## Abuse / DoS / Latency / Compatibility Matrix

| Scenario | Risk | Proposed bound | Future test | Failure mode | Stop condition | Compatibility impact | Claim boundary |
| --- | --- | --- | --- | --- | --- | --- | --- |
| One valid message | Batching path could break the normal single-message flow. | Default non-batched path unchanged; batching accepts a one-member batch and delivers it. | Send one message with batching disabled and one one-member batch with batching enabled. | Send fails, output changes, or ack/delete drifts. | Any default single-message regression. | Default user flow remains valid. | One-member batch is not privacy proof. |
| Multiple valid messages | Batch enqueue could reorder or drop messages. | Max 4 members; validate all before mutation; enqueue in request order. | Send four valid members and receive them in order. | Reordering, partial enqueue, or missing member. | Any unproven ordering drift. | Opt-in demo latency only. | Batch size and timing remain observable. |
| One invalid candidate in batch | Invalid member could delete remotely or mutate local state. | Stop at first invalid; do not ack invalid or later members. | Candidate batch with invalid first member. | Remote delete, accepted state, or output. | Any delete before verify or output on invalid. | Valid later members wait for later run. | Reject remains visible. |
| Mixed valid/invalid candidates | Partial processing can reorder or starve valid messages. | Process ordered prefix only; stop at first invalid; ack only verified prefix. | Valid, invalid, valid sequence. | Later valid member processed after invalid. | Any non-prefix processing after invalid. | Deterministic prefix behavior. | Queue shape remains visible. |
| Repeated invalid batch | Attacker could amplify retry work. | Existing four-attempt cap per candidate tag per 60 second window. | Repeat same invalid member through batched receive. | More than four attempts or cap bypass. | Any retry cap weakening. | Same as NA-0327 invalid retry. | Abuse volume remains observable. |
| Empty batch/poll | Empty polling could create busy loops. | Empty receive uses existing empty-poll retry; empty send batch rejects. | Empty poll under batching and zero-member send batch. | Busy loop or accepted empty send batch. | Any unbounded retry or empty send acceptance. | Empty poll remains successful and bounded. | Empty state remains observable. |
| Stale ack in batch | One stale ack could delete wrong candidates. | Ack batch validates all IDs before mutation; stale rejects all. | Ack batch with one stale ID and one valid queued ID. | Valid member deleted despite stale peer. | Any partial delete on invalid ack batch. | Individual ack remains available if implementation chooses it. | Ack failure remains visible. |
| Duplicate ack in batch | Duplicate ack could double-delete or skew counters. | Duplicate in ack batch rejects all and deletes none. | Ack same ID twice in one ack batch. | Counter drift or successful duplicate. | Any duplicate success. | Preserves ack-once. | No hidden receipt claim. |
| Slow valid receiver | Batch window can delay valid messages. | Send-side window max 750 ms; no receive wait added before ack/delete. | Measure deterministic logical window classes. | Window exceeds 750 ms or post-verify ack waits. | Any unbounded or post-verify delay. | Opt-in demo latency only. | Timing remains observable. |
| Local demo stress | Queued batches can pressure memory or queue caps. | Max 4 members; existing queue/rate limits prechecked. | Stress send-batch near queue caps. | Partial state or rate-limit bypass. | Any cap bypass or unbounded memory. | Existing relay caps preserved. | Not production capacity proof. |
| Production qsl-server equivalent | Service queues, logs, proxies, auth, and deployment timing differ. | Future-gated cross-repo lane only. | qsl-server-specific future harness. | qshield proof presented as production proof. | Any qsl-server path change or production claim. | None in NA-0331 qshield lane. | Production timing unproven. |
| qsl-attachments equivalent | Object upload/fetch timing, size classes, quota, and retention differ. | Future-gated cross-repo lane only. | qsl-attachments-specific future harness. | qshield proof presented as attachment proof. | Any qsl-attachments path change or attachment timing claim. | None in NA-0331 qshield lane. | Attachment timing unproven. |

## Future Validation / Marker Plan

Required NA-0331 marker candidates:

- `NA0331_BATCHING_AUTHORIZATION_OK`
- `NA0331_BATCH_POLICY_OK`
- `NA0331_DETERMINISTIC_TEST_BATCHING_OK`
- `NA0331_VALID_SINGLE_MESSAGE_UNCHANGED_OK`
- `NA0331_VALID_BATCH_DELIVERY_OK`
- `NA0331_BATCH_ORDERING_PRESERVED_OK`
- `NA0331_INVALID_BATCH_MEMBER_NO_REMOTE_DELETE_OK`
- `NA0331_PARTIAL_INVALID_BATCH_FAIL_CLOSED_OK`
- `NA0331_RETRY_CADENCE_STILL_BOUNDED_OK`
- `NA0331_BOUNDED_JITTER_STILL_BOUNDED_OK`
- `NA0331_NO_ACCEPTED_STATE_ON_INVALID_BATCH_OK`
- `NA0331_NO_OUTPUT_ON_INVALID_BATCH_OK`
- `NA0331_NO_SECRET_BATCH_ARTIFACT_OK`
- `NA0331_QSHIELD_DEMO_BOUNDARY_OK`
- `NA0331_SERVICE_PRODUCTION_BOUNDARY_OK`
- `NA0331_NO_TIMING_HIDDEN_CLAIM_OK`
- `NA0331_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK`
- `NA0331_NO_METADATA_FREE_CLAIM_OK`

Additional recommended markers:

- `NA0331_SEND_BATCH_ALL_OR_NOTHING_OK`
- `NA0331_ACK_BATCH_ALL_OR_NOTHING_OK`
- `NA0331_BATCH_WINDOW_BOUNDED_OK`
- `NA0331_EMPTY_BATCH_REJECT_OK`
- `NA0331_STALE_DUPLICATE_BATCH_ACK_FAIL_CLOSED_OK`
- `NA0331_QSL_ATTACHMENTS_FUTURE_GATED_OK`
- `NA0331_METADATA_RUNTIME_BATCHING_OK`

Required future proof:

- valid single message still succeeds;
- valid batch succeeds;
- send batch rejects all-or-nothing on invalid member or queue cap pressure;
- invalid candidate in receive batch does not delete before verify;
- partial invalid receive batch behavior is deterministic and fail-closed;
- retry cadence remains bounded;
- bounded jitter remains bounded;
- per-recipient/per-session ordering is preserved or the batch rejects;
- no accepted state or plaintext output on invalid batch member;
- stale/duplicate ack batch fails closed and deletes nothing;
- no secret batch artifacts;
- qsl-server and qsl-attachments production boundaries explicit;
- no claim of hidden timing metadata or hidden traffic shape.

## Production Boundary

### qshield Embedded Relay/Demo

Future NA-0331 batching is authorized only for the qshield embedded relay/demo
surface. It may prove bounded local/demo batch behavior and deterministic
markers. It must not be presented as public service, hosted relay, mobile,
desktop, qsl-server, or qsl-attachments production behavior.

### qsl-server

qsl-server production batching requires a separate cross-repo authorization
lane. Production relay timing includes deployment topology, auth, service
queues, logs, rate limits, reverse proxies, observability, retention, support
bundles, public-internet exposure, and operator runbooks. None of those are
proven by qshield local/demo evidence.

### qsl-attachments

qsl-attachments batching or object timing requires a separate cross-repo lane.
Attachment behavior includes upload/fetch timing, object sizes, quotas,
capabilities, retention, backup/recovery, and storage pressure. qshield
message batching evidence does not prove attachment service timing or object
shape behavior.

## External-Review Sensitivity

Batching changes observable timing and traffic shape and can introduce
ordering, queue pressure, and claim-safety risk. External review is recommended
before any stronger public or production claim. NA-0330 does not claim
external-review completion.

## Public Claim Boundary

Allowed wording:

- future qshield embedded relay/demo batching authorization;
- bounded local/demo batching proof after future NA-0331 implementation;
- may reduce selected local/demo event regularity inside stated bounds;
- timing metadata and traffic shape remain observable;
- qsl-server and qsl-attachments production timing remain unproven.

Forbidden wording:

- batching is implemented by NA-0330;
- prohibited wording: batching hides timing;
- prohibited wording: batching hides traffic shape;
- prohibited wording: metadata-free behavior;
- prohibited wording: anonymity;
- prohibited wording: untraceable behavior;
- production readiness;
- public-internet readiness;
- external-review completion;
- qshield local/demo evidence proves qsl-server or qsl-attachments production
  behavior.

## Selected Successor

Selected successor:

`NA-0331 -- Metadata Runtime qshield Demo Batching Implementation Harness`

Rationale:

- live NA-0330 scope explicitly asks whether a future batching implementation
  harness is safe;
- NA-0327 and NA-0329 already prove the retry-cadence and bounded-jitter
  invariants that future batching must preserve;
- the future implementation boundary can stay inside qshield embedded
  relay/demo files and tests;
- exact semantics, caps, ordering, ack/commit, artifact, production, and claim
  boundaries are now defined;
- service production timing remains explicitly future-gated.

## Rejected Alternatives

- Direct implementation inside NA-0330: rejected because NA-0330 is
  authorization/design only.
- qsl-server production batching first: rejected because service timing is
  cross-repo and not proven by qshield local/demo evidence.
- qsl-attachments batching first: rejected because attachment object timing and
  quota/retention behavior are separate cross-repo service concerns.
- Cover traffic as NA-0331: rejected because cover traffic has higher cost,
  abuse, storage, bandwidth, and claim-safety risk and needs a separate cost
  model.
- Metadata-free or anonymity claim: rejected because batching does not remove
  endpoint, timing, volume, queue, size, service log, or deployment metadata.

## Backup-Plan Impact Statement

No backup-plan update is required. NA-0330 changes only tracked qsl-protocol
governance/evidence/testplan files under `/srv/qbuild/work`, which is already
inside the qbuild backup scope. The Codex response file remains under the
existing Codex responses path. No new durable evidence location outside the
current backup scope is introduced. The D132 preservation bundle is not
deleted.

## Next Recommendation

Proceed to:

`NA-0331 -- Metadata Runtime qshield Demo Batching Implementation Harness`

NA-0331 should implement only the bounded qshield embedded relay/demo batching
harness described here, or stop on an exact prerequisite if any required proof
would require out-of-scope service, protocol, crypto, dependency, workflow,
website, branch-protection, public-safety, README, START_HERE, or production
behavior changes.
