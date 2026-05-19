Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-18

# NA-0317 Metadata Runtime qshield Ack/Commit Poll Semantics Authorization

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0317 authorizes a future qshield ack/commit-after-local-verify semantic as
the exact next queue-preserving poll shape needed before metadata runtime
identifier/default-padding proof can truthfully claim remote queue no-mutation
on malformed padding, decode, or stale-handle rejects.

The selected successor is:

`NA-0318 -- Metadata Runtime qshield Ack/Commit Poll Implementation Harness`

NA-0317 itself does not implement qshield runtime behavior. It records the
future lifecycle, files, tests, markers, compatibility rules, and stop
conditions. The inherited NA-0316 boundary remains visible: current qshield
`/poll` is destructive before local receive-side verification, so the current
remote queue boundary is `PROVEN_REMOTE_MUTATION` and `NEEDS_RUNTIME_CHANGE`.

## Live NA-0317 Scope

The live queue entry is `NA-0317 -- Metadata Runtime qshield Ack/Commit Poll
Semantics Authorization`, status `READY`, with goals G1 through G5.

Allowed by live scope:

- authorize exact qshield ack/commit or equivalent queue-preserving poll
  semantics;
- name exact future implementation files, tests, markers, compatibility rules,
  and stop conditions before implementation;
- choose the exact NA-0318 successor;
- preserve the inherited qshield poll mutation boundary and all metadata/public
  claim limits.

Forbidden by live scope:

- qshield ack/commit or peek runtime implementation;
- broader metadata runtime behavior;
- identifier/handle rotation runtime implementation;
- default padding runtime implementation;
- protocol, crypto, qsc, qsp, key-schedule, qsl-server, qsl-attachments,
  qsc-desktop, website/external repo, README, START_HERE, workflow,
  Cargo/dependency, branch-protection, or public-safety configuration changes;
- claims of anonymity, metadata-free behavior, untraceability,
  public-internet readiness, production readiness, or external review
  completion.

This live scope permits authorization/planning only. Runtime implementation must
wait for NA-0318 or another exact future directive.

## Inherited NA-0316 Mutation Boundary

NA-0316 classified the current qshield poll behavior as:

- `PROVEN_REMOTE_MUTATION`
- `NEEDS_RUNTIME_CHANGE`

Evidence inherited from NA-0316 and rechecked in NA-0317:

- `apps/qshield-cli/src/commands/relay.rs` stores messages in
  `state.queues` as `VecDeque<QueuedMsg>`.
- `/send` enqueues accepted messages with `push_back(...)`, increments
  `token_queued`, and increments `total_msgs`.
- `/poll` obtains the recipient queue, removes entries with `pop_front()`,
  builds the response from those removed entries, then decrements per-token and
  total queue accounting.
- `apps/qshield-cli/src/commands/recv.rs` calls `/poll` before checking
  `pad_len`, `bucket`, hex decoding, or `suite2.e2e.recv`.
- `apps/qshield-cli/src/commands/attachment.rs` also calls `/poll` before
  descriptor, integrity, decrypt, and output checks.

Therefore a malformed padded message, malformed wire value, stale peer/session
state, or actor decode failure can occur after the remote queue entry has
already been deleted. Current qshield behavior cannot support remote queue
no-mutation proof for invalid local receive.

## Sources Inspected

- `NEXT_ACTIONS.md` live NA-0317 entry.
- `tests/NA-0316_closeout_restore_na0317_testplan.md`.
- `docs/governance/evidence/NA-0316_metadata_runtime_qshield_poll_no_mutation_blocker_resolution.md`.
- `tests/NA-0316_metadata_runtime_qshield_poll_no_mutation_testplan.md`.
- `docs/governance/evidence/NA-0315_metadata_runtime_identifier_padding_executable_harness_plan.md`.
- `tests/NA-0315_metadata_runtime_identifier_padding_executable_harness_plan_testplan.md`.
- `scripts/ci/metadata_runtime_identifier_padding_harness_plan.sh`.
- `inputs/metadata_runtime/identifier_padding_runtime_fixture_v1.json`.
- `apps/qshield-cli/src/commands/relay.rs`.
- `apps/qshield-cli/src/commands/recv.rs`.
- `apps/qshield-cli/src/commands/send.rs`.
- `apps/qshield-cli/src/commands/attachment.rs`.
- `apps/qshield-cli/src/relay_client.rs`.
- `apps/qshield-cli/src/main.rs`.
- qshield-cli unit test inventory.
- qsc relay/pull comparison surfaces in `qsl/qsl-client/qsc/src/transport/mod.rs`
  and qsc no-mutation tests.
- `TRACEABILITY.md`, `DECISIONS.md`, and
  `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

## Current qshield Poll / Recv / Relay Behavior

Current qshield relay behavior:

- `/send` validates basic padding metadata and appends a message to the
  recipient queue.
- `/poll` is a destructive read: entries are removed from the queue before the
  caller receives them.
- The poll response has no message handle or receipt handle that can be used for
  later commit.
- There is no qshield endpoint for `ack`, `commit`, read-only `peek`, or
  queue-preserving candidate fetch.
- Queue accounting is decremented as part of `/poll`, not after local receive
  verification.

Current qshield receive behavior:

- `recv` retrieves messages through `/poll`.
- Padding checks and bucket checks run only after the poll response.
- Actor receive/decode runs only after padding checks.
- Plaintext output is printed only after actor decode succeeds, so local output
  is gated; however the remote queue entry has already been removed.
- Attachment receive has the same remote mutation issue for descriptor and
  ciphertext pairs.

The only truthful current claim is local-output gating for the inspected paths.
Remote queue no-mutation is not proven.

## Ack/Commit Semantic Design

### Message Lifecycle

Future qshield queue-preserving lifecycle:

1. `queued`: relay accepts a message, assigns an opaque server-side message
   handle, stores the entry, and keeps queue accounting unchanged until commit.
2. `candidate`: receiver fetches one or more candidates through a
   queue-preserving poll/peek endpoint. The entry remains queued or leased but
   not deleted.
3. `locally_verified`: receiver performs padding, bucket, hex, peer/session,
   and actor decode checks locally.
4. `accepted`: receiver creates the scoped accepted local state/output for a
   valid message.
5. `committed`: receiver posts an ack/commit for the opaque handle; only then
   may the relay delete the queued entry and decrement queue accounting.
6. `rejected`: receiver rejects invalid input without ack/commit; no accepted
   local state or output is created, and the remote queue remains unchanged
   unless a later directive explicitly authorizes a quarantine boundary.

### Valid Receive Path

Required future valid path:

- queue-preserving poll/peek fetches a candidate with an opaque handle;
- local padding/decode/handle checks pass;
- accepted local output/state is created successfully;
- ack/commit deletes exactly that remote queued message;
- repeated commit for the same handle is deterministic and fail-closed or a
  documented idempotent success only when the same commit already succeeded;
- success markers prove the ordering.

### Invalid Receive Path

Required future invalid path:

- queue-preserving poll/peek fetches a candidate;
- local padding, decode, peer/session, or stale-handle checks fail;
- no accepted local state is created;
- no plaintext or accepted output is emitted;
- no route token, bearer token, plaintext sentinel, padding sentinel,
  internal path, panic, or backtrace is logged;
- no ack/commit is sent;
- the remote queued entry remains present and can be observed by a later
  candidate fetch, or the entry moves only to a future explicitly authorized
  quarantine boundary.

### Idempotency

Required future idempotency rules:

- repeated poll/peek of the same invalid candidate is deterministic;
- stale or unknown commit handles reject fail-closed with coarse errors;
- repeated commit after a successful commit is deterministic and must not delete
  any other message;
- handles must be scoped to the recipient queue and must not be guessable public
  identifiers;
- commit must match the exact candidate identity returned by queue-preserving
  poll/peek.

### Abuse Boundary

Ack/commit preserves remote no-mutation on invalid local receive, but it can
leave a poison message in the queue. NA-0318 must therefore prove bounded
behavior:

- receiver commands keep `max` and per-command processing bounded;
- repeated invalid receive does not loop without bound;
- tests show deterministic repeated invalid behavior;
- if future evidence shows a persistent poison message blocks useful receive
  progress, the lane must stop or add an explicitly authorized quarantine /
  dead-letter policy with its own no-overclaim boundary.

### Visibility

Errors and markers must remain coarse:

- no raw route token or bearer token;
- no raw plaintext or padding sentinel on invalid paths;
- no long secret-like dumps in evidence or artifacts;
- no panic/backtrace output accepted as normal evidence;
- queue handles are opaque and may appear only as redacted or short non-secret
  proof tokens if tests need observability.

### Compatibility

NA-0318 should avoid silently changing legacy destructive `/poll` behavior. The
preferred compatibility shape is:

- retain existing `/poll` and `qshield relay poll` as legacy destructive demo
  behavior, documented as incompatible with remote no-mutation claims; and
- add a new queue-preserving candidate endpoint plus ack/commit endpoint for
  `qshield recv` and metadata runtime proof.

If NA-0318 instead changes `/poll` itself, it must explicitly authorize that
compatibility change, update tests for old behavior, and preserve a clear claim
boundary. No old destructive path may be used as evidence for remote
no-mutation.

## Option Analysis

### Option A - Ack/commit-after-local-verify

Feasibility: recommended. qshield's relay is implemented inside
`apps/qshield-cli`, so a future qshield-only implementation can add candidate
handles and commit semantics without qsl-server changes.

Future files:

- `apps/qshield-cli/src/commands/relay.rs`
- `apps/qshield-cli/src/commands/recv.rs`
- `apps/qshield-cli/src/commands/attachment.rs`
- `apps/qshield-cli/src/relay_client.rs`
- `apps/qshield-cli/tests/na_0318_qshield_ack_commit.rs`
- NA-0318 governance/evidence/testplan files

Tests: valid commit delete exactly once; invalid padding/decode/stale-handle
no-delete; repeated invalid bounded; stale commit fail-closed; no output, no
accepted state, and no secret leak on invalid paths.

Compatibility risk: manageable if future implementation adds new
queue-preserving endpoints and leaves legacy `/poll` explicitly documented.

Abuse/retry risk: poison-message retry must be bounded by max/attempt policy or
must stop for a quarantine decision.

Claim boundary: supports future remote queue no-mutation only after executable
NA-0318 proof lands.

Future combined identifier/default-padding harness: permitted after NA-0318
passes because remote queue mutation-before-verify will be resolved.

Recommendation: selected.

### Option B - Peek-before-delete

Feasibility: acceptable equivalent if implemented as read-only candidate fetch
plus explicit delete. It is nearly the same implementation shape as Option A.

Future files: same as Option A.

Tests: same as Option A, with delete replacing commit naming.

Compatibility risk: manageable if added as a new endpoint. Risk increases if
existing `/poll` changes silently.

Abuse/retry risk: same poison-message concern as Option A.

Claim boundary: strong after executable proof; not proven by NA-0317.

Future combined identifier/default-padding harness: permitted after proof.

Recommendation: acceptable fallback, but Option A names the receive lifecycle
more clearly.

### Option C - Quarantine/dead-letter

Feasibility: more complex and not selected as the first implementation step.

Future files: would include the Option A files plus explicit quarantine state,
policy, tests, and evidence.

Tests: must prove invalid-message movement to quarantine is the selected claimed
mutation boundary and does not masquerade as remote no-mutation.

Compatibility risk: higher because queue visibility and retry behavior change.

Abuse/retry risk: addresses poison-message blocking, but introduces new state
and policy decisions.

Claim boundary: useful only if future evidence states quarantine is mutation of
an explicitly authorized safe boundary, not no remote mutation.

Future combined identifier/default-padding harness: possible later, but only if
the claim is phrased around the chosen quarantine boundary.

Recommendation: reject for NA-0318 unless Option A proves unsafe due unbounded
retry risk.

### Option D - Local-only no-mutation

Feasibility: possible without qshield remote runtime changes.

Future files: evidence/testplan or local qshield tests only, depending on scope.

Tests: malformed padding/decode reject creates no local accepted state, no
plaintext output, and no secret leak while explicitly excluding remote queue
mutation from the claim.

Compatibility risk: low.

Abuse/retry risk: no new remote retry risk because it does not change remote
behavior.

Claim boundary: weaker. It must not be presented as remote no-mutation.

Future combined identifier/default-padding harness: insufficient for the full
remote queue no-mutation proof required by the metadata runtime agenda.

Recommendation: reject as immediate successor because it would leave the
NA-0316 blocker unresolved.

### Option E - Blocker continuation

Feasibility: available if exact files or semantics cannot be safely named.

Future files: governance/evidence/testplan only.

Tests: none beyond proof that implementation remains blocked.

Compatibility risk: low.

Abuse/retry risk: unchanged.

Claim boundary: preserves current `PROVEN_REMOTE_MUTATION`.

Future combined identifier/default-padding harness: blocked.

Recommendation: not selected because qshield-only files and an exact
ack/commit semantic can be named.

## Selected Authorization

NA-0317 selects Option A:

`ack/commit-after-local-verify`

Exact authorization:

- Future NA-0318 may implement qshield queue-preserving candidate fetch plus
  ack/commit after local verify.
- Future NA-0318 must prove invalid local receive does not delete the remote
  queued message, does not create accepted local state/output, and does not leak
  secrets or sentinels.
- Future NA-0318 may use peek-before-delete naming only if it preserves the same
  candidate-before-commit lifecycle and test obligations.
- Future NA-0318 must not implement broader metadata runtime identifier/default
  padding behavior.

NA-0317 does not authorize runtime code edits in this PR.

## Future Implementation Files

Authorized future source/test files for NA-0318:

- `apps/qshield-cli/src/commands/relay.rs`
  - add queue-preserving candidate fetch and ack/commit handling;
  - assign and validate opaque message handles;
  - preserve/decrement queue accounting only on commit.
- `apps/qshield-cli/src/commands/recv.rs`
  - consume candidates through queue-preserving API;
  - verify padding/decode/peer/session before commit;
  - avoid output or accepted state on invalid paths.
- `apps/qshield-cli/src/commands/attachment.rs`
  - keep attachment receive compatible with the new poll response shape;
  - commit descriptor/ciphertext only after descriptor, integrity, decrypt, and
    output checks succeed, if attachment receive remains in scope.
- `apps/qshield-cli/src/relay_client.rs`
  - add request/response types for candidate fetch and ack/commit.
- `apps/qshield-cli/tests/na_0318_qshield_ack_commit.rs`
  - executable qshield ack/commit integration harness.

Authorized future governance/evidence files:

- `docs/governance/evidence/NA-0318_metadata_runtime_qshield_ack_commit_implementation_harness.md`
- `tests/NA-0318_metadata_runtime_qshield_ack_commit_implementation_harness_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Optional future harness files only if NA-0318 explicitly needs scriptable marker
aggregation:

- `scripts/ci/metadata_runtime_qshield_ack_commit_harness.sh`
- `inputs/metadata_runtime/qshield_ack_commit_fixture_v1.json`

If NA-0318 needs any other file, it must stop unless the future directive
explicitly expands scope.

## Future Forbidden Files

NA-0318 must not touch:

- `.github/**`
- `Cargo.toml`
- `Cargo.lock`
- `README.md`
- `START_HERE.md`
- `docs/public/**`
- qsc/qsp/protocol/crypto implementation outside the listed qshield files
- `qsl-server/**`
- `qsl-attachments/**`
- `qsl/qsl-client/qsc-desktop/**`
- `website/**`
- external website repositories
- branch-protection or public-safety configuration
- dependency manifests or lockfiles

## Future Markers

NA-0318 must emit or otherwise prove these markers:

- `NA0318_QSHIELD_PEEK_OR_POLL_CANDIDATE_OK`
- `NA0318_QSHIELD_LOCAL_VERIFY_BEFORE_COMMIT_OK`
- `NA0318_QSHIELD_ACK_COMMIT_DELETE_OK`
- `NA0318_QSHIELD_INVALID_RECV_NO_REMOTE_DELETE_OK`
- `NA0318_QSHIELD_INVALID_RECV_NO_ACCEPTED_STATE_OK`
- `NA0318_QSHIELD_INVALID_RECV_NO_OUTPUT_OK`
- `NA0318_QSHIELD_INVALID_RECV_NO_SECRET_LEAK_OK`
- `NA0318_QSHIELD_STALE_ACK_FAIL_CLOSED_OK`
- `NA0318_QSHIELD_REPEATED_INVALID_RECV_BOUNDED_OK`
- `NA0318_METADATA_RUNTIME_ACK_COMMIT_OK`

These are future runtime proof markers. NA-0317 records them but does not emit
them as proof.

## Future Tests

NA-0318 must add executable tests for:

- valid message ack/commit deletes exactly one queued message;
- repeat ack/commit of the same handle is deterministic and does not delete any
  other message;
- invalid padded message does not delete the remote queue entry;
- malformed decode does not delete the remote queue entry;
- stale or wrong-scope handle does not delete the remote queue entry;
- stale ack/commit rejects fail-closed;
- repeated invalid receive is deterministic and bounded;
- invalid receive creates no local accepted state;
- invalid receive creates no plaintext or accepted output;
- invalid receive emits no route token, bearer token, plaintext sentinel,
  padding sentinel, panic, or backtrace;
- legacy destructive `/poll`, if retained, is documented and excluded from
  remote no-mutation claims.

## Future Stop Conditions

NA-0318 must stop if any of these occur:

- relay API cannot represent candidate vs commit state;
- opaque handles cannot be scoped to the intended recipient queue;
- existing qshield relay state cannot preserve queue accounting until commit;
- stale ack/commit cannot fail closed or idempotently target only the original
  message;
- qshield attachment receive cannot remain compatible without widening scope;
- qsl-server changes are required;
- qsl-attachments changes are required;
- qsc/qsp/protocol/crypto implementation changes are required;
- dependency changes are required;
- unbounded retry/poison-message risk cannot be bounded or explicitly
  quarantined;
- compatibility impact is broader than the future directive authorizes;
- public claim boundaries would become ambiguous.

## Cross-Repo qsl-server Dependency Assessment

No qsl-server implementation change is required for the selected NA-0318
qshield implementation harness. The qshield relay under review is the embedded
demo relay in `apps/qshield-cli/src/commands/relay.rs`, and its queue state,
poll endpoint, and receive client types live inside qsl-protocol.

Important boundary:

- qsc's local inbox relay comparison surface in
  `qsl/qsl-client/qsc/src/transport/mod.rs` also has a destructive pull shape
  for its own inbox queue.
- That qsc/qsl-server-facing behavior is not authorized by NA-0318.
- Any later production/server-backed queue-preserving semantics must use a
  separate qsl-server/qshield or qsc/qsl-server authorization lane.

If NA-0318 discovers that the qshield proof cannot be completed without
qsl-server changes, it must stop and select a cross-repo authorization lane
instead of widening the qsl-protocol implementation.

## Metadata Runtime Claim Boundaries

NA-0317 does not prove:

- runtime metadata reduction;
- identifier/handle rotation;
- default padding as a runtime default;
- sanitized-error runtime expansion;
- retention/purge runtime behavior;
- timing or traffic-shape protection;
- public-internet metadata behavior;
- remote queue no-mutation under current destructive `/poll`;
- anonymity;
- metadata-free behavior;
- untraceability;
- production readiness;
- public-internet readiness;
- external review completion.

NA-0317 proves only an authorization boundary for the future queue-preserving
qshield poll semantic.

## Selected Successor

Selected successor:

`NA-0318 -- Metadata Runtime qshield Ack/Commit Poll Implementation Harness`

Rationale:

- NA-0316 proved current qshield remote queue mutation before local verify.
- NA-0317 can name exact qshield-only files and tests for ack/commit.
- Ack/commit is the smallest semantic that preserves the remote queue until
  local verification and accepted output/state succeed.
- Local-only no-mutation would be truthful but insufficient for the future full
  metadata runtime identifier/default-padding proof.
- Blocker continuation is unnecessary because the implementation boundary is
  exact enough for a future directive.

## Backup-Plan Impact Statement

No backup-plan update is required for NA-0317. Durable changes stay under the
qsl-protocol worktree in governance/evidence, testplan, traceability,
decisions, and rolling-journal paths already covered by `/srv/qbuild/work`.
No new durable evidence location outside the current backup scope is created.

## Next Recommendation

Run NA-0318 as a qshield-only implementation harness for the selected
ack/commit-after-local-verify lifecycle. The first implementation should add
queue-preserving candidate fetch plus ack/commit in qshield, prove invalid
local receive does not delete the remote queue entry, and keep the broader
metadata runtime identifier/default-padding harness blocked until that proof is
green.
