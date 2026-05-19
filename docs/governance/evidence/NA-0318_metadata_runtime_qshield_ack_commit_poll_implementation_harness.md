Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-19

# NA-0318 Metadata Runtime qshield Ack/Commit Poll Implementation Harness

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0318 implements the bounded qshield embedded-relay ack/commit-after-local-
verify semantic selected by NA-0317. The implementation keeps legacy `/poll`
visible as destructive demo behavior, adds `/poll-candidate` for queue-
preserving candidate fetch, adds `/ack` for explicit delete/commit of exactly
one candidate, and updates qshield receive paths to use the candidate/ack path.

The executable harness proves that invalid local receive-side decode reject
does not delete the remote queued message, create accepted local state, create
accepted output, or leak the configured token, candidate id, plaintext
sentinel, padding sentinel, panic, or backtrace. Valid ack/commit deletes
exactly one queued message in the embedded qshield relay harness.

Boundary: this is qshield embedded relay proof only. It does not prove
qsl-server production relay support, anonymity, metadata-free behavior,
untraceability, production readiness, public-internet readiness, or external
review completion.

## Live NA-0318 Scope

The live queue entry is `NA-0318 -- Metadata Runtime qshield Ack/Commit Poll
Implementation Harness`, status `READY`, with goals G1 through G5.

Allowed implementation/test files used:

- `apps/qshield-cli/src/commands/relay.rs`
- `apps/qshield-cli/src/commands/recv.rs`
- `apps/qshield-cli/src/commands/attachment.rs`
- `apps/qshield-cli/src/relay_client.rs`
- `apps/qshield-cli/tests/na_0318_qshield_ack_commit.rs`

Allowed governance/evidence files used:

- `docs/governance/evidence/NA-0318_metadata_runtime_qshield_ack_commit_poll_implementation_harness.md`
- `tests/NA-0318_metadata_runtime_qshield_ack_commit_poll_implementation_harness_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No script or input artifact was added because live NA-0318 did not require one.

## Inherited NA-0317 Authorization

NA-0317 authorized future qshield ack/commit-after-local-verify, or equivalent
queue-preserving poll semantics, after NA-0316 classified current qshield
`/poll` as `PROVEN_REMOTE_MUTATION` and `NEEDS_RUNTIME_CHANGE`.

NA-0317 required:

- candidate fetch before remote delete;
- explicit commit/delete only after local verification;
- invalid local receive reject with no remote delete;
- no accepted local state or output on invalid receive;
- no secret/sentinel leak;
- bounded repeated invalid behavior;
- stale/duplicate ack fail-closed or deterministic no-op;
- qsl-server production boundary explicit.

## Sources Inspected

- `NEXT_ACTIONS.md` live NA-0318 entry.
- `docs/governance/evidence/NA-0317_metadata_runtime_qshield_ack_commit_authorization.md`.
- `tests/NA-0317_metadata_runtime_qshield_ack_commit_authorization_testplan.md`.
- `tests/NA-0317_closeout_restore_na0318_testplan.md`.
- `docs/governance/evidence/NA-0316_metadata_runtime_qshield_poll_no_mutation_blocker_resolution.md`.
- `docs/governance/evidence/NA-0315_metadata_runtime_identifier_padding_executable_harness_plan.md`.
- `apps/qshield-cli/src/commands/relay.rs`.
- `apps/qshield-cli/src/commands/recv.rs`.
- `apps/qshield-cli/src/commands/attachment.rs`.
- `apps/qshield-cli/src/relay_client.rs`.
- qshield test inventory.
- `scripts/ci/**`.
- `inputs/metadata_runtime/**`.
- `TRACEABILITY.md`.
- `DECISIONS.md`.

## Implementation Summary

Changed qshield files:

- `apps/qshield-cli/src/commands/relay.rs`
- `apps/qshield-cli/src/commands/recv.rs`
- `apps/qshield-cli/src/commands/attachment.rs`
- `apps/qshield-cli/src/relay_client.rs`
- `apps/qshield-cli/tests/na_0318_qshield_ack_commit.rs`

The relay now assigns an opaque per-message `ack_id` when `/send` accepts a
queued message. The `ack_id` is derived inside the embedded relay from the
configured bearer token, recipient id, and a relay-local sequence counter. The
raw bearer token is not printed or returned.

## Candidate Fetch Behavior

`/poll-candidate` accepts the existing `{id, max}` request shape, enforces the
same bearer-token check and poll rate limit as `/poll`, and returns up to `max`
front-of-queue candidates with `ack_id`, `from`, `msg`, `pad_len`, and
`bucket`.

It does not call `pop_front`, does not decrement per-token queue accounting,
and does not decrement total queue accounting. Repeated candidate fetch returns
the same front candidate while the message is uncommitted.

## Ack/Commit Behavior

`/ack` accepts `{id, ack_id}` after the bearer-token check. A valid ack removes
exactly one matching queued candidate from that recipient queue, decrements
that candidate's token queue count, and decrements total queue accounting by
one.

Unknown, malformed, stale, or duplicate `ack_id` values reject with a coarse
error and do not delete any queued message. The harness proves stale ack returns
failure and leaves the next queued message intact.

## Invalid Receive Behavior

`qshield recv` now fetches from `/poll-candidate`, validates local candidate
metadata and wire decode before spawning the actor for invalid decode paths, and
posts `/ack` only after local Suite-2 receive verification produces plaintext.

Malformed local decode reject returns before `/ack`. The harness proves the
remote candidate remains available after the reject and remains available after
a repeated reject.

`qshield attachment recv` now fetches descriptor/ciphertext candidates through
`/poll-candidate` and acks them only after descriptor validation, ciphertext
integrity, and decrypt verification succeed. Invalid attachment receive paths
therefore do not use destructive `/poll`.

## Local Mutation Boundary

The NA-0318 harness records the qshield local state file before invalid
receive, runs invalid receive, and byte-compares the state file afterward. The
state remains unchanged on the invalid decode reject path.

## Remote Queue Mutation Boundary

For the qshield embedded relay candidate path, invalid local receive-side
decode reject does not delete the remote queued message. Valid `/ack` after
local verification deletes exactly one queued message. Legacy `/poll` remains
destructive and is not used by `recv` or `attachment recv`.

## Output Boundary

Invalid receive produces no accepted `from <peer>:` plaintext output. The
accepted output path remains gated behind local verification and successful
ack/commit.

## Secret / Log Boundary

The invalid receive harness scans stdout and stderr for:

- configured relay token;
- candidate `ack_id`;
- plaintext sentinel;
- padding sentinel;
- panic/backtrace markers.

No forbidden text is present on the invalid path. The relay still prints that a
token is configured, but it does not print the token value.

## Harness Markers

The executable harness declares and emits the following markers when run with
`--nocapture`:

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

## Metadata Claim Boundary

This implementation enables the next bounded metadata runtime harness to rely
on qshield embedded relay queue-preserving ack/commit proof. It does not
implement identifier/handle rotation, default padding as a runtime default,
sanitized-error expansion, retention/purge behavior, timing/traffic-shape
mitigation, or deployment metadata posture.

## qsl-server Production Boundary

No qsl-server implementation changed. This PR does not prove qsl-server
production relay ack/commit support, production deployment support, or public
internet relay behavior. Any qsc/qsl-server production relay ack/commit
semantics require a separate authorization lane.

## Limitations

- Legacy `/poll` remains destructive for raw demo relay polling and is not
  evidence for remote no-mutation.
- The qshield candidate handle is embedded-relay scoped and memory-resident.
- Invalid padding combinations that the relay rejects at `/send` are not queued
  for local receive; the local invalid receive harness uses malformed decode
  as the representable queued invalid candidate.
- The attachment receive path acks its descriptor/ciphertext pair after local
  validation; this remains demo attachment behavior, not a production service
  claim.

## Selected Successor

Selected successor after successful NA-0318 implementation/closeout:

`NA-0319 -- Metadata Runtime Identifier and Default Padding Executable Harness`

Rationale: qshield embedded relay queue-preserving ack/commit behavior is now
implemented and harnessed, so the next lane can return to the metadata runtime
identifier/default-padding executable proof without claiming qsl-server
production support.

## Backup-Plan Impact Statement

No backup-plan update is required. All durable changes stay inside the
qsl-protocol worktree under `/srv/qbuild/work`, which is already covered by the
qbuild backup scope. The response file is written under the existing Codex
responses path.

## Next Recommendation

Close out NA-0318 after the implementation PR merges and post-merge
`public-safety` is green, then restore `NA-0319 -- Metadata Runtime Identifier
and Default Padding Executable Harness` as the sole READY successor. Do not
implement NA-0319 in the closeout PR.
