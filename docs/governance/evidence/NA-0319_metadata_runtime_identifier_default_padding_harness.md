Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-19

# NA-0319 Metadata Runtime Identifier and Default Padding Executable Harness

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0319 adds a bounded qshield embedded-relay executable harness for metadata
runtime identifier/handle and default-padding behavior after NA-0318 delivered
queue-preserving candidate fetch plus explicit ack/commit semantics.

The implementation remains limited to the qshield demo boundary:

- receive-side stale peer handles now reject without echoing the raw peer
  handle;
- receive-side padded candidates now verify that stripped padding bytes are
  zero before actor decode;
- the executable harness proves opaque per-candidate ack handles, malformed and
  stale handle rejects, bounded default-padding bucket/strip behavior, invalid
  padding config reject, malformed padding reject, no accepted local state or
  output on rejects, no secret/sentinel leak on rejects, and no remote delete
  through `/poll-candidate` unless `/ack` is posted after verification.

This is not qsl-server production relay proof. It is not an anonymity,
metadata-free, untraceable, public-internet-readiness, production-readiness, or
external-review-complete claim.

## Live NA-0319 Scope

The live queue item is `NA-0319 -- Metadata Runtime Identifier and Default
Padding Executable Harness`, status `READY`, with goals G1 through G5.

Live deliverables require executable identifier/handle proof, executable
default-padding proof, no accepted state/output on invalid metadata runtime
reject, no secret/sentinel leak on invalid metadata runtime reject, and an
explicit qshield embedded relay versus qsl-server production boundary.

## Inherited NA-0318 Ack/Commit Proof

NA-0318 added qshield embedded relay `/poll-candidate` and `/ack` semantics.
Candidate fetch returns queued messages without deleting them. Valid `/ack`
after local verification deletes exactly one queued candidate. Invalid local
decode reject preserves the remote queued candidate, creates no accepted local
state/output, and leaks no token, candidate id, plaintext sentinel, padding
sentinel, panic, or backtrace text.

NA-0319 uses only that qshield embedded relay candidate/ack path for remote
no-delete claims. Legacy `/poll` remains visibly destructive and is not used as
metadata no-mutation evidence.

## Sources Inspected

- `NEXT_ACTIONS.md` live NA-0319 entry.
- `tests/NA-0318_closeout_restore_na0319_testplan.md`.
- `docs/governance/evidence/NA-0318_metadata_runtime_qshield_ack_commit_poll_implementation_harness.md`.
- `tests/NA-0318_metadata_runtime_qshield_ack_commit_poll_implementation_harness_testplan.md`.
- `apps/qshield-cli/src/commands/relay.rs`.
- `apps/qshield-cli/src/commands/recv.rs`.
- `apps/qshield-cli/src/commands/send.rs`.
- `apps/qshield-cli/src/commands/attachment.rs`.
- `apps/qshield-cli/src/relay_client.rs`.
- `apps/qshield-cli/tests/na_0318_qshield_ack_commit.rs`.
- `docs/governance/evidence/NA-0314_metadata_runtime_identifier_padding_transition_plan.md`.
- `docs/governance/evidence/NA-0315_metadata_runtime_identifier_padding_executable_harness_plan.md`.
- `inputs/metadata_runtime/identifier_padding_runtime_fixture_v1.json`.
- `scripts/ci/metadata_runtime_identifier_padding_harness_plan.sh`.
- `TRACEABILITY.md`.
- `DECISIONS.md`.

## Implementation Summary

Changed qshield files:

- `apps/qshield-cli/src/commands/recv.rs`
- `apps/qshield-cli/src/commands/attachment.rs`
- `apps/qshield-cli/tests/na_0319_metadata_runtime_identifier_padding.rs`

`recv.rs` now rejects receive candidates whose stripped padding bytes are
non-zero. This is a fail-closed validation strengthening before actor decode.

`recv.rs` and `attachment.rs` now report missing receive-side sessions with a
coarse `no session for peer` error instead of echoing the raw peer handle from
the candidate.

The new harness starts the actual qshield embedded relay binary, exercises
runtime `/send`, `/poll-candidate`, `/ack`, and `qshield recv` paths, and emits
the NA-0319 markers.

## Identifier / Handle Behavior

The proven identifier boundary is the qshield embedded relay candidate ack
handle plus the qshield receive-side peer/session lookup boundary.

Proven behavior:

- each queued relay candidate receives a distinct opaque 64-hex ack handle;
- the ack handle does not contain the relay token, sender id, recipient id, or
  harness sentinels;
- malformed ack handles reject without deleting queued candidates;
- stale ack handles reject after the original candidate has already been
  committed, without deleting the remaining candidate;
- receive-side stale peer/session handles reject before ack and do not echo the
  raw peer handle.

Limitation: this is not broad peer id, session id, route-token, contact-device,
or attachment-handle rotation. It proves the bounded qshield candidate handle
and stale peer receive boundary selected for this lane.

## Default Padding Behavior

The bounded default-padding profile proven by the harness is:

`metadata-runtime-default-padding-v1` with buckets `[512, 1024, 2048, 4096,
8192]`.

Proven behavior:

- the harness pads to the first bucket that can contain the candidate payload;
- valid padded candidates strip back to the original payload only after bucket,
  pad length, and zero-padding verification;
- invalid padding config rejects before writing a config file;
- malformed padded candidates with non-zero padding reject in `qshield recv`
  before actor decode;
- invalid padding rejects do not ack/delete the remote queued candidate.

Limitation: this is bounded qshield demo proof. It does not claim that all
runtime messages are padded by default in every product surface, and it does
not claim that all metadata is hidden.

## Valid Path Proof

The valid path harness:

1. pads a candidate payload to the default bucket profile;
2. queues it through qshield embedded relay `/send`;
3. fetches it through `/poll-candidate`;
4. verifies the opaque ack handle and padding strip locally in the harness;
5. posts `/ack`;
6. proves the candidate is deleted exactly after verification.

Marker: `NA0319_ACK_COMMIT_AFTER_IDENTIFIER_PADDING_VERIFY_OK`.

## Invalid Identifier Proof

Malformed ack handles reject with a coarse relay error and leave the queued
candidate unchanged. Stale ack handles reject after the original candidate is
committed and do not delete the remaining queued candidate. Stale peer/session
receive rejects preserve the candidate and do not mutate the local store.

## Invalid Padding Proof

Invalid padding configuration rejects before config creation. Malformed
receive-side padding with valid relay metadata but non-zero padding bytes
rejects in `qshield recv`, preserves the queued candidate, keeps local state
unchanged, and produces no accepted plaintext output.

## Remote Queue No-Delete Proof

No remote queue no-delete claim uses legacy `/poll`. All no-delete claims use
the qshield embedded relay candidate path:

- `/poll-candidate` returns the candidate without deleting it;
- malformed/stale identifier rejects do not post valid `/ack`;
- malformed padding rejects do not post `/ack`;
- the same candidate ack id remains available after reject;
- valid `/ack` after verification deletes exactly one candidate.

## Local Mutation Boundary

The harness byte-compares qshield local state before and after stale peer and
malformed padding receive rejects. The state remains unchanged on both reject
paths.

## Output Boundary

Reject paths produce no accepted `from <peer>:` plaintext output and do not
write attachment or message output artifacts.

## Secret / Log Boundary

Reject output is scanned for:

- relay token;
- candidate ack id;
- raw peer-handle sentinel;
- plaintext sentinel;
- padding sentinel;
- panic/backtrace markers.

No forbidden text is present in the harnessed reject output.

## Harness Markers

Identifier markers:

- `NA0319_IDENTIFIER_ROTATION_POLICY_OK`
- `NA0319_OPAQUE_HANDLE_BOUNDARY_OK`
- `NA0319_STALE_HANDLE_REJECT_OK`
- `NA0319_MALFORMED_HANDLE_REJECT_OK`
- `NA0319_IDENTIFIER_NO_REMOTE_DELETE_ON_REJECT_OK`
- `NA0319_IDENTIFIER_NO_ACCEPTED_STATE_ON_REJECT_OK`
- `NA0319_IDENTIFIER_NO_OUTPUT_ON_REJECT_OK`
- `NA0319_IDENTIFIER_NO_SECRET_LOG_OK`

Padding markers:

- `NA0319_DEFAULT_PADDING_POLICY_OK`
- `NA0319_PADDING_BUCKETS_OK`
- `NA0319_PADDING_INVALID_CONFIG_REJECT_OK`
- `NA0319_PADDING_STRIP_VERIFY_OK`
- `NA0319_PADDING_MALFORMED_REJECT_OK`
- `NA0319_PADDING_NO_REMOTE_DELETE_ON_REJECT_OK`
- `NA0319_PADDING_NO_ACCEPTED_STATE_ON_REJECT_OK`
- `NA0319_PADDING_NO_OUTPUT_ON_REJECT_OK`
- `NA0319_PADDING_NO_SECRET_LOG_OK`

Integration markers:

- `NA0319_ACK_COMMIT_AFTER_IDENTIFIER_PADDING_VERIFY_OK`
- `NA0319_QSHIELD_EMBEDDED_RELAY_BOUNDARY_OK`
- `NA0319_METADATA_RUNTIME_IDENTIFIER_PADDING_OK`

## Metadata Claim Boundary

This evidence supports only a bounded qshield embedded relay/demo metadata
runtime proof. It does not claim complete metadata reduction, anonymity,
metadata-free behavior, untraceability, public-internet readiness, production
readiness, external review completion, timing/traffic-shape resistance,
contact-graph hiding, IP-level metadata hiding, or deployment metadata
readiness.

## qsl-server Production Boundary

No qsl-server implementation changed. qsl-server production relay ack/commit,
identifier, padding, retention, purge, or deployment metadata behavior remains
unproven and requires a separate authorization lane.

## Limitations

- Identifier rotation proof is limited to per-candidate qshield ack handles and
  receive-side stale peer/session reject; broad stable peer/session/route
  rotation remains open.
- Default padding proof is a bounded qshield demo profile, not a global product
  default and not a claim that all metadata is hidden.
- qshield embedded relay proof is not qsl-server production relay proof.
- Sanitized-error coverage is limited to the identifier/padding reject paths in
  this harness; broader sanitized-error runtime expansion remains open.
- Retention/purge runtime behavior remains open.
- Timing/traffic-shape threat modeling remains open.

## Selected Successor

Selected successor after successful NA-0319 implementation/closeout:

`NA-0320 -- Metadata Runtime Sanitized Errors and Retention/Purge Executable Harness`

Rationale: the qshield embedded relay identifier/default-padding proof is now
bounded and executable, so the next metadata-runtime lane can return to
sanitized-error expansion and retention/purge behavior without hiding the
remaining identifier/default-padding limitations.

## Backup-Plan Impact Statement

No backup-plan update is required. All durable changes stay inside the
qsl-protocol worktree under `/srv/qbuild/work`, which is already covered by the
qbuild backup scope. The Codex response file remains under the existing Codex
responses path.

## Next Recommendation

Merge the NA-0319 implementation PR only after required local validation and
protected checks are green. Then close out NA-0319 and restore:

`NA-0320 -- Metadata Runtime Sanitized Errors and Retention/Purge Executable Harness`

Do not implement NA-0320 in the closeout.
