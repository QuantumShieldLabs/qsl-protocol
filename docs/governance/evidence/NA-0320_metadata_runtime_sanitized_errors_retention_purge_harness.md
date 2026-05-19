Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-19

# NA-0320 Metadata Runtime Sanitized Errors and Retention/Purge Harness

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0320 adds executable qshield embedded-relay/demo proof for sanitized-error
and retention/purge behavior after NA-0318 delivered candidate fetch plus
explicit ack/commit and NA-0319 delivered identifier/default-padding proof.

The implementation is a bounded runtime harness:

- qshield `recv`, relay `/ack`, relay `/send`, and `attachment recv` reject
  paths emit coarse diagnostics without route tokens, raw handles, candidate
  ack ids, plaintext sentinels, padding sentinels, passphrase sentinels, raw
  key sentinels, panic text, or backtrace text;
- invalid receive and attachment receive rejects preserve the remote queued
  candidate through `/poll-candidate`;
- invalid rejects do not create accepted local state or output artifacts;
- valid `/ack` removes exactly one verified queued candidate;
- stale `/ack` fails closed without deleting the remaining candidate.

This is qshield embedded relay/demo proof only. It is not qsl-server production
relay proof, qsl-attachments production proof, anonymity proof, metadata-free
proof, untraceability proof, public-internet-readiness proof, production
readiness proof, or external-review-complete proof.

## Live NA-0320 Scope

The live queue item is `NA-0320 -- Metadata Runtime Sanitized Errors and
Retention/Purge Executable Harness`, status `READY`, with goals G1 through G5.

Live deliverables require executable sanitized-error runtime proof or exact
prerequisite stop, executable retention/purge runtime proof or exact
prerequisite stop, no accepted state/output on invalid sanitized-error or
retention/purge reject, no secret/sentinel leak on invalid runtime reject, and
an explicit qshield embedded relay versus qsl-server production boundary.

## Inherited NA-0318/NA-0319 Proof

NA-0318 added qshield embedded relay `/poll-candidate` and `/ack` semantics.
`/poll-candidate` fetches queued candidates without deleting them. `/ack`
deletes exactly one matching candidate only after local verification.

NA-0319 added bounded qshield identifier/default-padding proof. It established
opaque per-candidate ack handles, stale/malformed handle reject, stale
peer/session receive reject, bounded default-padding strip verification,
malformed padding reject, no accepted local state/output on rejects, no
secret/sentinel leak on rejects, and valid ack/delete only after
identifier/padding verification.

NA-0320 builds on that boundary and does not use legacy destructive `/poll` as
remote no-delete proof.

## Sources Inspected

- `NEXT_ACTIONS.md` live NA-0320 entry.
- `tests/NA-0319_closeout_restore_na0320_testplan.md`.
- `docs/governance/evidence/NA-0319_metadata_runtime_identifier_default_padding_harness.md`.
- `tests/NA-0319_metadata_runtime_identifier_default_padding_harness_testplan.md`.
- `docs/governance/evidence/NA-0318_metadata_runtime_qshield_ack_commit_poll_implementation_harness.md`.
- `docs/governance/evidence/NA-0315_metadata_runtime_identifier_padding_executable_harness_plan.md`.
- `scripts/ci/metadata_phase2_sanitized_errors_retention_harness.sh`.
- `inputs/metadata_phase2/sanitized_errors_retention_policy_vectors_v1.json`.
- `apps/qshield-cli/src/commands/relay.rs`.
- `apps/qshield-cli/src/commands/recv.rs`.
- `apps/qshield-cli/src/commands/send.rs`.
- `apps/qshield-cli/src/commands/attachment.rs`.
- `apps/qshield-cli/src/relay_client.rs`.
- `apps/qshield-cli/tests/na_0319_metadata_runtime_identifier_padding.rs`.
- `apps/qshield-cli/tests/na_0318_qshield_ack_commit.rs`.
- `TRACEABILITY.md`.
- `DECISIONS.md`.

## Implementation Summary or Blocker

No qshield runtime source change was required. Live review found the bounded
qshield demo surfaces already return coarse errors for the NA-0320 reject paths
when exercised through the candidate/ack boundary.

The added executable harness is:

- `apps/qshield-cli/tests/na_0320_metadata_runtime_sanitized_retention.rs`

It starts the actual qshield embedded relay binary, drives real relay JSON
requests and real `qshield recv` / `qshield attachment recv` commands, and
emits all NA-0320 markers.

## Changed qshield Files

- `apps/qshield-cli/tests/na_0320_metadata_runtime_sanitized_retention.rs`

No qshield runtime source file changed. No qsl-server, qsl-attachments, qsc,
qsp, protocol, crypto, key-schedule, Cargo, workflow, README, START_HERE,
website, or public-safety configuration path changed.

## Sanitized-Error Behavior

The harness proves these reject paths are coarse and secret-safe:

- receive-side missing session / invalid handle rejects;
- receive-side malformed padded candidate rejects;
- receive-side malformed message decode rejects;
- relay malformed ack rejects;
- relay invalid padding metadata rejects;
- attachment receive missing descriptor/ciphertext pair rejects.

Reject output is scanned for route-token, raw-handle, candidate-ack,
plaintext, padding, attachment, passphrase, raw-key, panic, and backtrace
sentinels.

## Retention/Purge Behavior

The proven retention/purge boundary is qshield embedded relay candidate
retention and ack purge:

- queued candidate: `/send` queues a candidate visible through
  `/poll-candidate`;
- candidate fetched but unacked: repeated `/poll-candidate` returns the same
  candidate and does not delete it;
- valid acked/committed message: valid `/ack` removes exactly one matching
  candidate;
- invalid rejected message: invalid `recv` and `attachment recv` paths do not
  post `/ack` and the candidate remains available;
- stale candidate/ack: a repeated stale `/ack` returns failure and leaves the
  remaining candidate intact;
- local output artifact: invalid attachment receive does not create the output
  directory, and the harness explicitly removes its temporary local artifact.

This is not a qsl-server production retention or purge claim.

## Valid Path Proof

The valid path fetches two candidates, locally verifies the first candidate
identity and message shape in the harness, then posts `/ack`. The relay removes
exactly that one candidate and leaves the second candidate queued.

Marker: `NA0320_VALID_ACK_PURGES_ONE_CANDIDATE_OK`.

## Invalid Sanitized-Error Proof

Invalid handle, padding, decode, malformed ack, invalid padding metadata, and
attachment reject outputs are scanned for all configured sentinels and
panic/backtrace indicators. No forbidden text is present.

## Invalid Retention/Purge Proof

Invalid receive and attachment receive rejects do not ack/delete candidates.
Malformed ack and invalid padding metadata rejects leave the queued candidates
unchanged.

## Remote Queue Retention Proof

All remote no-delete claims use `/poll-candidate`. After invalid receive or
attachment reject, a fresh `/poll-candidate` returns the same ack id. After
valid `/ack`, only one matching candidate is removed.

## Local Mutation Boundary

The harness byte-compares qshield `state.json` before and after invalid receive
and attachment receive rejects. The file remains unchanged.

## Output Boundary

Invalid receive output contains no accepted `from <peer>:` plaintext line.
Invalid attachment receive does not create the configured output directory.

## Secret/Log Boundary

Reject output is scanned for:

- route token sentinel;
- raw handle sentinel;
- candidate ack id and candidate ack sentinel;
- plaintext sentinel;
- padding sentinel;
- attachment sentinel;
- passphrase sentinel;
- raw key sentinel;
- panic/backtrace markers.

No forbidden text is present in the harnessed reject outputs.

## Harness Markers

Sanitized-error markers:

- `NA0320_SANITIZED_ERROR_POLICY_OK`
- `NA0320_RECV_INVALID_HANDLE_ERROR_REDACTED_OK`
- `NA0320_RECV_INVALID_PADDING_ERROR_REDACTED_OK`
- `NA0320_RECV_DECODE_ERROR_REDACTED_OK`
- `NA0320_ATTACHMENT_ERROR_REDACTED_OK`
- `NA0320_NO_ROUTE_TOKEN_LEAK_OK`
- `NA0320_NO_CANDIDATE_ACK_ID_LEAK_OK`
- `NA0320_NO_PLAINTEXT_SENTINEL_LEAK_OK`
- `NA0320_NO_PADDING_SENTINEL_LEAK_OK`
- `NA0320_NO_PANIC_BACKTRACE_LEAK_OK`

Retention/purge markers:

- `NA0320_RETENTION_PURGE_POLICY_OK`
- `NA0320_VALID_ACK_PURGES_ONE_CANDIDATE_OK`
- `NA0320_INVALID_RECV_RETAINS_REMOTE_CANDIDATE_OK`
- `NA0320_STALE_ACK_FAIL_CLOSED_OK`
- `NA0320_REPEATED_INVALID_RECV_BOUNDED_OK`
- `NA0320_INVALID_RECV_NO_LOCAL_OUTPUT_OK`
- `NA0320_INVALID_RECV_NO_ACCEPTED_STATE_OK`
- `NA0320_LOCAL_ARTIFACT_CLEANUP_OK`
- `NA0320_QSHIELD_EMBEDDED_RELAY_RETENTION_BOUNDARY_OK`
- `NA0320_METADATA_RUNTIME_SANITIZED_RETENTION_OK`

## Metadata Claim Boundary

The proof is bounded to qshield embedded relay/demo behavior. It does not claim
complete runtime metadata reduction, anonymity, metadata-free behavior,
untraceability, public-internet readiness, production readiness, external
review completion, timing/traffic-shape resistance, contact-graph hiding,
IP-level metadata hiding, or deployment metadata readiness.

## qsl-server/qsl-attachments Production Boundary

No qsl-server implementation changed. qsl-server production relay sanitized
errors, retention, purge, ack/commit, identifier, padding, deployment, and
public-internet behavior remain unproven.

No qsl-attachments implementation changed. qsl-attachments production
retention/purge, descriptor, ciphertext, and service error behavior remain
outside this NA-0320 proof.

## Limitations

- qshield embedded relay memory-resident queue behavior is not qsl-server
  production storage behavior.
- Attachment proof is limited to qshield demo `attachment recv` reject behavior,
  not qsl-attachments production service behavior.
- Broad peer/session/route rotation remains open.
- Global production default padding remains open.
- Timing/traffic-shape threat modeling remains open.
- Deployment metadata and public-internet metadata behavior remain open.

## Selected Successor

Selected successor:

`NA-0321 -- Metadata Runtime Timing and Traffic-Shape Threat Model / Executable Evidence Plan`

Rationale: bounded qshield sanitized-error and retention/purge proof succeeded
without requiring qsl-server or qsl-attachments implementation changes. The
next truthful metadata-runtime blocker is timing/traffic-shape threat modeling
and executable evidence planning.

## Backup-Plan Impact Statement

No backup-plan update is required. All durable changes stay inside the
qsl-protocol worktree under `/srv/qbuild/work`, which is already covered by
the qbuild backup scope. The Codex response file is written under the existing
Codex responses path.

## Next Recommendation

Merge the NA-0320 harness PR after local validation and required checks are
green. Then run a separate closeout PR to mark NA-0320 DONE and restore
NA-0321 as the sole READY successor without implementing NA-0321.
