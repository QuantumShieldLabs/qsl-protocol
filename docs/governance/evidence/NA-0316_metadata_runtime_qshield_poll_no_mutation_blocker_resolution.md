Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-18

# NA-0316 Metadata Runtime qshield Poll No-Mutation Blocker Resolution

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0316 classifies the qshield poll/no-mutation blocker inherited from
NA-0315. Current qshield relay polling removes queued messages before local
receive-side padding checks or Suite-2 decode can reject the message. The
remote queue mutation boundary is therefore:

`PROVEN_REMOTE_MUTATION`

The selected blocker resolution is not to claim remote no-mutation from current
runtime behavior. The exact successor is:

`NA-0317 -- Metadata Runtime qshield Ack/Commit Poll Semantics Authorization`

That successor should authorize and bound the smallest qshield runtime change
needed for a queue-preserving poll proof, preferably ack/commit-after-local-
verify. NA-0316 itself makes no runtime, protocol, crypto, qsc, qsp, service,
dependency, workflow, public-doc, README, START_HERE, branch-protection, or
public-safety configuration change.

## Live NA-0316 Scope

The live queue entry is `NA-0316 -- Metadata Runtime qshield Poll
No-Mutation Blocker Resolution`, status `READY`, with goals G1 through G5.

Allowed by live scope:

- inspect live qshield poll, receive, send, and relay behavior;
- decide the qshield poll no-mutation boundary;
- provide executable proof if the current boundary can be truthfully proven;
- stop with exact prerequisite evidence if a runtime change is required;
- keep public claim boundaries and metadata runtime gaps visible.

Forbidden by live scope unless exact future scope authorizes it:

- protocol, crypto, qsp, or key schedule implementation changes;
- qsl-server, qsl-attachments, qsc-desktop, website, external repo, README,
  START_HERE, workflow, Cargo/dependency, branch-protection, or public-safety
  configuration changes;
- claims of anonymity, metadata-free behavior, untraceability, public-internet
  readiness, production readiness, or external review completion.

The live entry does not name exact qshield runtime files authorized for
implementation. Runtime edits are therefore out of scope for NA-0316.

## Inherited NA-0315 Blocker

NA-0315 selected NA-0316 because the future metadata runtime
identifier/default-padding harness could not truthfully claim remote
no-mutation while qshield `/poll` removed messages before local receive-side
padding or decode rejection. NA-0315 explicitly recorded:

- qshield `/poll` calls `pop_front()` before returning queued messages;
- qshield `recv` performs padding strip and actor decode after `/poll`;
- malformed padded or undecodable messages can be removed from the remote queue
  before local reject;
- runtime identifier/default-padding proof remains future-gated until this
  blocker is classified or resolved.

## Sources Inspected

- `NEXT_ACTIONS.md` live NA-0316 entry.
- `tests/NA-0315_closeout_restore_na0316_testplan.md`.
- `docs/governance/evidence/NA-0315_metadata_runtime_identifier_padding_executable_harness_plan.md`.
- `tests/NA-0315_metadata_runtime_identifier_padding_executable_harness_plan_testplan.md`.
- `scripts/ci/metadata_runtime_identifier_padding_harness_plan.sh`.
- `inputs/metadata_runtime/identifier_padding_runtime_fixture_v1.json`.
- `apps/qshield-cli/src/commands/relay.rs`.
- `apps/qshield-cli/src/commands/recv.rs`.
- `apps/qshield-cli/src/commands/send.rs`.
- `apps/qshield-cli/src/commands/attachment.rs`.
- `apps/qshield-cli/src/relay_client.rs`.
- `apps/qshield-cli/src/actor.rs`.
- `scripts/ci/metadata_conformance_smoke.sh`.
- qshield and qsc test inventory from `rg` searches.
- `TRACEABILITY.md`, `DECISIONS.md`, and
  `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

## qshield Poll / Recv / Relay Behavior Baseline

Relay send path:

- `apps/qshield-cli/src/commands/relay.rs` stores messages in
  `state.queues` as `VecDeque<QueuedMsg>`.
- `/send` validates basic padding metadata before enqueue.
- `/send` pushes accepted entries with `entry.push_back(...)`, increments
  `token_queued`, and increments `total_msgs`.

Relay poll path:

- `/poll` reads `id` and `max`, then locks relay state.
- It obtains `state.queues.entry(id).or_default()`.
- It loops up to `max` and removes each entry with `queue.pop_front()`.
- It builds the response from removed entries.
- It decrements per-token queue accounting and `state.total_msgs`.
- It returns the removed entries to the caller.

Receive path:

- `apps/qshield-cli/src/commands/recv.rs` posts to `/poll` first.
- Only after the response returns does it inspect each message.
- It then checks `pad_len`, `bucket`, truncates padding bytes, and calls
  `suite2.e2e.recv` through the refimpl actor.
- On `pad_len exceeds message length`, `bucket size mismatch`, malformed hex,
  actor decode failure, or missing actor plaintext, the command returns an
  error after the relay has already removed the message.

Attachment receive path:

- `apps/qshield-cli/src/commands/attachment.rs` also posts to `/poll` first.
- Descriptor/decrypt/integrity checks occur only after the descriptor and
  ciphertext messages are removed from the relay queue.

## Queue Mutation Boundary

Classification:

`PROVEN_REMOTE_MUTATION`

Current `/poll` is a destructive read. A message can be deleted from the
remote queue before local padding, decode, descriptor, integrity, or plaintext
verification succeeds.

No current qshield endpoint or client API provides:

- `ack`;
- `commit`;
- read-only `peek`;
- message identifier suitable for later delete;
- retry-preserving invalid-message quarantine;
- observable queue length proof after a local receive reject.

Therefore current qshield behavior cannot support a truthful remote
no-mutation claim for invalid local receive input.

## Local Mutation Boundary

Current `recv` loads local state and reads the session entry, but it does not
write accepted plaintext state before printing the decoded message. For the
specific qshield CLI path inspected, local accepted-output mutation is gated
behind local padding checks and actor decode.

However, NA-0316 does not add an executable local-only no-mutation harness.
Local-only proof remains possible as a narrower future lane, but it would not
resolve the remote queue mutation blocker needed by the combined metadata
runtime harness.

## Output Boundary

For message receive, stdout prints `from <peer>: <plaintext>` only after
padding checks and `suite2.e2e.recv` produce `plaintext_hex`. Known reject
paths return errors before printing plaintext. Attachment receive writes output
only after descriptor validation, ciphertext integrity check, and actor decrypt
success.

Because `/poll` is destructive before these checks, the output boundary is
local-only. It does not restore or preserve removed remote queue entries.

## Secret / Log Boundary

The inspected qshield relay error paths use short generic errors such as
`invalid padding metadata`, `missing or invalid relay token`, and
`unsupported content type`. `scripts/ci/metadata_conformance_smoke.sh` already
checks selected malformed/auth/padding errors for relay-token and sentinel
absence.

NA-0316 does not add new runtime logging, stdout markers, or secret-bearing
artifacts. The current blocker evidence avoids raw route tokens, bearer tokens,
plaintext sentinels, exact sensitive endpoint fragments, and long-hex dumps.

## Semantic Option Analysis

### Option 1 - Ack/commit-after-local-verify

Feasibility: strong, but requires explicit runtime authorization.

Likely files:

- `apps/qshield-cli/src/commands/relay.rs`
- `apps/qshield-cli/src/commands/recv.rs`
- `apps/qshield-cli/src/relay_client.rs`
- qshield CLI tests or scripts for valid commit and invalid non-delete

Scope risk: runtime API and relay semantics change. This is outside live
NA-0316 because no exact runtime file allowlist is present.

Tests needed:

- valid local verify commits/deletes exactly one remote message;
- invalid local padding/decode reject leaves the queued message available or
  moves it to an explicitly scoped safe state;
- no accepted output/state remains after reject;
- no route token, bearer token, plaintext sentinel, padding sentinel, panic, or
  backtrace leaks.

Future markers:

- `NA0316_QSHIELD_POLL_BEHAVIOR_CLASSIFIED_OK`
- `NA0316_QSHIELD_REMOTE_QUEUE_MUTATION_BOUNDARY_OK`
- `NA0316_QSHIELD_LOCAL_NO_MUTATION_OK`
- `NA0316_QSHIELD_NO_OUTPUT_ON_REJECT_OK`
- `NA0316_QSHIELD_NO_SECRET_LEAK_OK`
- `NA0316_QSHIELD_ACK_COMMIT_REQUIRED_OK`
- `NA0316_METADATA_RUNTIME_POLL_BLOCKER_RESOLUTION_OK`

Claim boundary: supports remote no-mutation only after executable proof lands.

Recommendation: select an authorization lane for this option.

### Option 2 - Peek-before-delete

Feasibility: possible only if qshield adds a read-only poll surface or changes
`/poll` semantics.

Likely files and tests are similar to Option 1, but the API shape would be
peek/delete instead of poll/ack.

Scope risk: runtime API and relay semantics change. This is outside live
NA-0316.

Claim boundary: strong if executable tests prove invalid local reject does not
delete or otherwise mutate the claimed remote queue boundary.

Recommendation: acceptable alternative inside the same future authorization,
but ack/commit has the clearer send/receive lifecycle.

### Option 3 - Local-only no-mutation boundary

Feasibility: possible without changing remote behavior if a future lane limits
claims to local accepted state and local output.

Likely files:

- qshield CLI test/script surfaces;
- evidence/testplan only if the lane remains non-runtime.

Scope risk: lower than runtime semantics, but it does not satisfy the stronger
remote no-mutation proof requested by the future combined harness.

Tests needed:

- malformed padding/decode reject creates no local output;
- no accepted local state mutation;
- no secret/sentinel leak;
- remote queue deletion explicitly classified as out of claim scope.

Future markers:

- `NA0316_QSHIELD_POLL_BEHAVIOR_CLASSIFIED_OK`
- `NA0316_QSHIELD_LOCAL_NO_MUTATION_OK`
- `NA0316_QSHIELD_NO_OUTPUT_ON_REJECT_OK`
- `NA0316_QSHIELD_NO_SECRET_LEAK_OK`
- `NA0316_QSHIELD_ACK_COMMIT_NOT_AVAILABLE`

Claim boundary: local only. It must never be presented as remote no-mutation.

Recommendation: reject as the immediate NA-0317 successor because the current
combined identifier/default-padding harness needs the remote queue blocker
resolved, not hidden.

### Option 4 - Dead-letter/quarantine semantics

Feasibility: possible but more complex than needed for the first blocker
resolution.

Scope risk: higher. It introduces new state and retention semantics.

Tests needed:

- invalid local reject moves a message to deterministic quarantine;
- accepted queue state is not mutated;
- quarantine does not leak secrets;
- retry, purge, and retention behavior are explicit.

Claim boundary: could be truthful if scoped, but it is more complicated than
ack/commit.

Recommendation: defer unless ack/commit or peek is rejected by a future
authorization lane.

### Option 5 - Blocker continuation

Feasibility: true only if source inspection could not classify behavior.

NA-0316 can classify the behavior: current qshield destructive poll is proven
from source. A generic continuation is less precise than selecting the
ack/commit authorization successor.

Recommendation: reject.

## Selected Blocker Resolution

Selected resolution:

`NEEDS_RUNTIME_CHANGE`

Current source proves destructive poll before local verify, and live NA-0316
does not authorize exact qshield runtime files. NA-0316 therefore records the
blocker and selects the exact future authorization lane instead of patching
runtime behavior.

## Executable Proof Or Blocker Evidence

NA-0316 provides source-level blocker evidence and governance/testplan proof.
It does not emit a remote queue no-mutation marker. It does not add executable
qshield runtime tests because runtime file authorization is absent and a test
that merely confirms destructive poll would not resolve the future harness
blocker.

Current evidence is sufficient to select a successor:

- qshield `/poll` removes with `pop_front()`;
- qshield `recv` validates padding and decodes only after `/poll`;
- no ack/commit or peek endpoint exists;
- therefore remote no-mutation is not provable under current behavior.

## Future Markers

NA-0316 records these marker meanings for the successor lane:

| Marker | NA-0316 status |
| --- | --- |
| `NA0316_QSHIELD_POLL_BEHAVIOR_CLASSIFIED_OK` | Evidence-classified: current poll is destructive |
| `NA0316_QSHIELD_REMOTE_QUEUE_MUTATION_BOUNDARY_OK` | Future executable proof required; not emitted as remote no-mutation |
| `NA0316_QSHIELD_LOCAL_NO_MUTATION_OK` | Future executable proof required |
| `NA0316_QSHIELD_NO_OUTPUT_ON_REJECT_OK` | Future executable proof required |
| `NA0316_QSHIELD_NO_SECRET_LEAK_OK` | Future executable proof required |
| `NA0316_QSHIELD_ACK_COMMIT_REQUIRED_OK` | Selected successor requires authorization |
| `NA0316_QSHIELD_ACK_COMMIT_NOT_AVAILABLE` | Current API classification only |
| `NA0316_METADATA_RUNTIME_POLL_BLOCKER_RESOLUTION_OK` | Blocker classified; runtime resolution remains future |

## Selected Successor

`NA-0317 -- Metadata Runtime qshield Ack/Commit Poll Semantics Authorization`

Rationale:

- remote queue mutation is proven in current qshield poll behavior;
- local-only scope would be truthful but would leave the combined metadata
  runtime harness blocked for remote no-mutation;
- source seams are sufficient to identify exact likely files for a future
  runtime authorization;
- ack/commit-after-local-verify is the smallest clear semantic shape that can
  preserve remote queued messages until local verification succeeds.

Rejected successors:

- `Metadata Runtime Identifier and Default Padding Executable Harness`:
  rejected because remote queue no-mutation is not proven.
- `Metadata Runtime Identifier and Default Padding Local No-Mutation Harness`:
  rejected as immediate successor because it narrows the claim instead of
  resolving the remote queue blocker.
- `Metadata Runtime qshield Poll No-Mutation Blocker Continuation`:
  rejected because current source is sufficient to classify the blocker.
- `Metadata Runtime Default Padding Executable Harness`:
  rejected because receive-side padding reject still depends on poll semantics.

## Claim Boundaries

NA-0316 does not prove:

- runtime metadata reduction;
- identifier or handle rotation;
- default padding as a runtime default;
- remote queue no-mutation on invalid local receive input;
- anonymity;
- metadata-free behavior;
- untraceability;
- public-internet readiness;
- production readiness;
- external review completion.

All metadata runtime gaps remain visible:

- qshield ack/commit or peek semantics;
- identifier/handle rotation runtime;
- default padding runtime;
- sanitized-error runtime expansion;
- retention/purge runtime behavior;
- timing/traffic-shape threat modeling;
- deployment metadata posture;
- public-internet metadata behavior.

## No Unsupported Runtime Metadata Implementation Proof

Changed paths for NA-0316 are limited to governance/evidence/testplan/journal
files:

- `docs/governance/evidence/NA-0316_metadata_runtime_qshield_poll_no_mutation_blocker_resolution.md`
- `tests/NA-0316_metadata_runtime_qshield_poll_no_mutation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No qshield runtime source, qsc/qsp implementation, protocol-core code, crypto
state-machine, key schedule, service implementation, Cargo/dependency,
workflow, public-safety, branch-protection, website, README, or START_HERE
path is changed.

## Backup-Plan Impact Statement

No backup-plan update is required. NA-0316 changes only tracked
qsl-protocol governance, evidence, testplan, and rolling-journal files under
`/srv/qbuild/work`, which is already within the expected local backup scope.
No durable evidence is created outside the existing response-file path and the
repository worktree.

## Next Recommendation

Run the selected successor:

`NA-0317 -- Metadata Runtime qshield Ack/Commit Poll Semantics Authorization`

The first artifact should freeze exact allowed qshield runtime files, endpoint
shape, message identifiers, commit/delete semantics, failure handling,
secret-safe output markers, and tests before implementation begins.
