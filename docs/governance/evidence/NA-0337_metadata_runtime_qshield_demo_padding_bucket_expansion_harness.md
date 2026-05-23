Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-22

# NA-0337 Metadata Runtime qshield Demo Padding Bucket Expansion Harness

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0337 implements the bounded qshield embedded relay/demo padding bucket
expansion authorized by NA-0336 and backs it with an executable harness.

The implemented policy is `qshield_demo_padding_bucket_expansion_v1`, enabled
for qshield demo initialization through `QSHIELD_DEMO_PADDING_BUCKETS=expanded`
or by explicitly configuring the exact expanded bucket table. The table is:

`[256, 512, 768, 1024, 1536, 2048, 3072, 4096, 5120, 6144, 7168, 8192]`

The harness proves valid small, medium, and large padded candidates; max
overhead of 1023 bytes; deterministic invalid-config rejection; strip/verify;
malformed padding rejection before actor decode; no remote delete before local
verification; no accepted state/output on reject; secret-free artifacts; and
preservation of batching, retry, bounded jitter, and cover prototype bounds.

This is local qshield embedded relay/demo evidence only. It is not qsl-server
production padding proof, not qsl-attachments object-size proof, not attachment
padding, not public-internet proof, and not evidence that timing metadata,
traffic shape, or all metadata is removed.

## Live NA-0337 Scope

Live `NEXT_ACTIONS.md` authorized:

- `NA-0337 -- Metadata Runtime qshield Demo Padding Bucket Expansion Implementation Harness`
- Status: READY
- Goals: G1, G2, G3, G4, G5
- Objective: execute a bounded qshield embedded relay/demo padding bucket
  expansion implementation harness or stop on an exact prerequisite.

Protected boundaries:

- no qsl-server, qsl-attachments, qsc/qsp/protocol/crypto/key-schedule,
  dependency, workflow, website, README, START_HERE, docs/public,
  branch-protection, or public-safety configuration change;
- no attachment-size padding, qsl-server production padding,
  qsl-attachments production object-size padding, public-internet traffic
  observation, or production service timing implementation;
- no unsupported production, public-internet, external-review, anonymity,
  metadata-free, or untraceable claim;
- no claim that timing metadata or traffic shape is hidden.

The implementation stays within the allowed qshield demo surface and
governance/evidence files.

## Inherited NA-0336 Authorization

NA-0336 D-0654 authorized one future qshield demo implementation harness for
the expanded bucket policy. It required:

- deterministic bucket table;
- max padded payload of 8192 bytes;
- max overhead of 1023 bytes for the selected table;
- deterministic rejection of empty, nonpositive, duplicate-after-normalization,
  unsorted, and too-large configs;
- zero padding verification before actor decode;
- strip only after verification;
- no remote delete before local verification;
- no accepted state/output on invalid padding;
- secret-free artifacts;
- batching, retry-cadence, bounded-jitter, and cover prototype compatibility;
- explicit qshield demo versus qsl-server/qsl-attachments production boundary.

## Inherited NA-0319 Default-Padding Proof

NA-0319 D-0617 proved the existing qshield demo padding path with the
`metadata-runtime-default-padding-v1` table `[512, 1024, 2048, 4096, 8192]`.
It established:

- send pads candidate bytes to the first configured bucket;
- receive verifies bucket length, pad length, and zero padding before actor
  decode;
- malformed padding rejects without acking the remote candidate;
- invalid padding creates no accepted state or plaintext output;
- reject output is scanned for route/token, ack id, plaintext, padding, panic,
  and backtrace sentinels.

NA-0337 reuses those runtime boundaries while adding strict expanded-policy
configuration, caps, and executable proof.

## Inherited NA-0335 Cover Prototype Proof

NA-0335 D-0652 implemented opt-in bounded qshield embedded relay/demo cover
traffic with `qshield_demo_cover_traffic_v1`, deterministic test mode,
synthetic local cover, active-session cover, batch-fill cover, real-message
priority, in-memory artifact summaries, purge, no recursive cover generation,
and `COVER_ARTIFACT_SECRET_FINDING_COUNT 0`.

NA-0337 verifies the expanded padding policy coexists with the cover prototype
and batching path without broadening cover traffic or production behavior.

## Sources Inspected

Sources reviewed before patching:

- `NEXT_ACTIONS.md` live NA-0337 entry.
- `tests/NA-0336_closeout_restore_na0337_testplan.md`.
- `docs/governance/evidence/NA-0336_metadata_runtime_padding_bucket_expansion_authorization.md`.
- `tests/NA-0336_metadata_runtime_padding_bucket_expansion_authorization_testplan.md`.
- `docs/governance/evidence/NA-0319_metadata_runtime_identifier_default_padding_harness.md`.
- `tests/NA-0319_metadata_runtime_identifier_default_padding_harness_testplan.md`.
- `docs/governance/evidence/NA-0314_metadata_runtime_identifier_padding_transition_plan.md`.
- `docs/governance/evidence/NA-0335_metadata_runtime_qshield_demo_cover_traffic_prototype_harness.md`.
- `apps/qshield-cli/src/commands/send.rs`.
- `apps/qshield-cli/src/commands/recv.rs`.
- `apps/qshield-cli/src/commands/init.rs`.
- `apps/qshield-cli/src/commands/relay.rs`.
- `apps/qshield-cli/src/config.rs`.
- `apps/qshield-cli/src/relay_client.rs`.
- `apps/qshield-cli/tests/na_0319_metadata_runtime_identifier_padding.rs`.
- `apps/qshield-cli/tests/na_0335_metadata_runtime_cover_traffic_prototype.rs`.
- `TRACEABILITY.md`.
- `DECISIONS.md`.

Search coverage included padding bucket, bucket, default padding,
strip/verify, malformed padding, invalid padding, padding config, max overhead,
size class, zero padding, padded payload, no remote delete, accepted state,
output, qshield demo, qsl-server, qsl-attachments, production, metadata-free,
anonymity, untraceable, timing hidden, traffic hidden, `FUTURE_GATE`, and
`NOT_READY`.

## Implementation Summary

Changed qshield files:

- `apps/qshield-cli/src/config.rs`
- `apps/qshield-cli/src/commands/init.rs`
- `apps/qshield-cli/src/commands/send.rs`
- `apps/qshield-cli/src/commands/recv.rs`
- `apps/qshield-cli/src/commands/relay.rs`
- `apps/qshield-cli/tests/na_0337_metadata_runtime_padding_bucket_expansion.rs`

Implementation details:

- adds named qshield demo expanded-padding constants;
- adds strict CSV parsing and validation for empty, zero, duplicate, unsorted,
  and too-large bucket configs;
- adds `QSHIELD_DEMO_PADDING_BUCKETS=expanded` as the opt-in init profile;
- validates padding config before qshield send starts state/session/actor work;
- caps relay padding metadata to the 8192-byte qshield demo maximum;
- rejects nonnumeric or overflowing relay `pad_len` / `bucket` metadata;
- rejects receive-side padding with `pad_len > 0` and missing bucket metadata;
- rejects receive-side bucket values above the qshield demo cap before actor
  decode.

## Padding Bucket Policy

Policy: `qshield_demo_padding_bucket_expansion_v1`.

Expanded table:

`[256, 512, 768, 1024, 1536, 2048, 3072, 4096, 5120, 6144, 7168, 8192]`

Maximum padded payload: 8192 bytes.

Maximum overhead: 1023 bytes.

The largest overhead occurs for a payload length of 7169 bytes, which pads to
8192 bytes. The harness iterates payload lengths 1 through 8192 and proves the
computed worst overhead is 1023 bytes.

## Deterministic Test Mode

The policy is deterministic because bucket selection is table-driven and the
harness uses loopback relays, fixed candidate payloads, fixed environment
gates, direct candidate verification, and retry/jitter test-mode environment
flags. No long sleeps or nondeterministic external services are required.

## Future / Actual Implementation Boundary

Actual implementation is limited to qshield embedded relay/demo transport
metadata and local CLI config behavior.

Future-gated and not implemented:

- qsl-server production padding;
- qsl-attachments production object-size padding;
- attachment-size padding;
- production service timing;
- public-internet traffic observation;
- qsc/qsp/protocol/crypto/key-schedule behavior;
- dependency, Cargo, workflow, branch-protection, or public-safety changes.

## Abuse / Cost / Latency / Compatibility Behavior

Abuse and cost are bounded by:

- max padded candidate payload of 8192 bytes;
- relay body cap and queue caps already present in qshield demo relay;
- relay-side rejection of oversized padding metadata;
- send-side rejection of too-large bucket configs;
- no silent fallback from invalid padding config to unpadded send.

Latency behavior remains explicit:

- padding does not claim timing metadata is hidden;
- retry cadence and bounded jitter remain opt-in qshield demo behaviors;
- the harness checks retry/jitter ledger caps on malformed padding reject.

Compatibility behavior:

- no-padding behavior remains unchanged;
- current valid explicit qshield padding configs remain accepted if sorted,
  unique, positive, and capped at 8192;
- batching and cover prototype behavior remain opt-in and bounded.

## Valid Small / Medium / Large Proof

The harness sends padded relay candidates with payload lengths selected to hit
small, medium, and large expanded buckets:

- small: 17-byte payload pads to 256;
- medium: 1537-byte payload pads to 2048;
- large: 7169-byte payload pads to 8192.

Each candidate is fetched through `/poll-candidate`, verified for bucket and
pad length, stripped only after confirming zero padding, and then acked.

Markers:

- `NA0337_VALID_SMALL_MESSAGE_PADDING_OK`
- `NA0337_VALID_MEDIUM_MESSAGE_PADDING_OK`
- `NA0337_VALID_LARGE_MESSAGE_PADDING_OK`
- `NA0337_PADDING_STRIP_VERIFY_OK`

## Max Overhead Proof

The harness computes worst-case overhead across payload lengths 1..8192 and
asserts the result is 1023 bytes.

Marker:

- `NA0337_PADDING_MAX_OVERHEAD_BOUNDARY_OK`

## Invalid Config Proof

The harness proves deterministic rejection for:

- empty bucket string;
- zero bucket;
- negative bucket input reaching app validation;
- duplicate-after-normalization bucket;
- unsorted bucket table;
- bucket above 8192;
- very large numeric bucket;
- invalid environment policy value;
- hand-written unsorted config on `qshield send` before actor/session work.

Marker:

- `NA0337_PADDING_INVALID_CONFIG_REJECT_OK`

## Strip / Verify Proof

Valid candidates are stripped only after:

- candidate hex decodes;
- bucket metadata is present and equals candidate byte length;
- pad length does not exceed candidate length;
- stripped bytes are all zero;
- bucket does not exceed 8192 bytes.

Marker:

- `NA0337_PADDING_STRIP_VERIFY_OK`

## Malformed Padding Proof

The harness queues a candidate whose declared padding bytes contain the
padding sentinel instead of zeros. `qshield recv` rejects with `padding reject`
before actor decode.

Marker:

- `NA0337_PADDING_MALFORMED_REJECT_OK`

## No Remote Delete Proof

The malformed candidate remains visible through `/poll-candidate` with the
same ack handle after the receive reject.

Marker:

- `NA0337_PADDING_NO_REMOTE_DELETE_ON_REJECT_OK`

## No Accepted State / Output Proof

The harness snapshots the local qshield state file before malformed receive,
runs receive, and confirms:

- state bytes are unchanged;
- no accepted `from alice:` output appears;
- plaintext and padding sentinels are absent from output.

Markers:

- `NA0337_PADDING_NO_ACCEPTED_STATE_ON_REJECT_OK`
- `NA0337_PADDING_NO_OUTPUT_ON_REJECT_OK`

## Batching / Retry / Jitter / Cover Preservation Proof

The harness proves:

- padded members are accepted through the existing qshield demo batch endpoint;
- batch cap remains 4;
- cover traffic remains opt-in, tagged, and bounded;
- real padded messages remain ahead of cover candidates;
- malformed padding reject records retry/jitter ledger values bounded by the
  inherited caps.

Marker:

- `NA0337_BATCHING_RETRY_JITTER_COVER_STILL_BOUNDED_OK`

## Artifact / Log Safety Proof

The harness scans command output, retry ledger text, and response bodies for:

- route-token sentinel;
- raw-handle sentinel;
- candidate/ack sentinel;
- plaintext sentinel;
- padding sentinel;
- passphrase sentinel;
- raw-key sentinel;
- panic/backtrace markers;
- local sensitive path markers.

Harness output:

- `PADDING_ARTIFACT_SECRET_FINDING_COUNT 0`
- `PADDING_ARTIFACT_SIZE_WITHIN_CAP_OK`
- `PADDING_NO_PLAINTEXT_SENTINEL_LEAK_OK`
- `NA0337_PADDING_NO_SECRET_ARTIFACT_OK`

No durable runtime artifacts are committed.

## Harness Markers

The executable harness emits:

- `NA0337_PADDING_BUCKET_AUTHORIZATION_OK`
- `NA0337_PADDING_BUCKET_POLICY_OK`
- `NA0337_DETERMINISTIC_TEST_PADDING_OK`
- `NA0337_VALID_SMALL_MESSAGE_PADDING_OK`
- `NA0337_VALID_MEDIUM_MESSAGE_PADDING_OK`
- `NA0337_VALID_LARGE_MESSAGE_PADDING_OK`
- `NA0337_PADDING_MAX_OVERHEAD_BOUNDARY_OK`
- `NA0337_PADDING_INVALID_CONFIG_REJECT_OK`
- `NA0337_PADDING_STRIP_VERIFY_OK`
- `NA0337_PADDING_MALFORMED_REJECT_OK`
- `NA0337_PADDING_NO_REMOTE_DELETE_ON_REJECT_OK`
- `NA0337_PADDING_NO_ACCEPTED_STATE_ON_REJECT_OK`
- `NA0337_PADDING_NO_OUTPUT_ON_REJECT_OK`
- `NA0337_PADDING_NO_SECRET_ARTIFACT_OK`
- `NA0337_BATCHING_RETRY_JITTER_COVER_STILL_BOUNDED_OK`
- `NA0337_QSHIELD_DEMO_BOUNDARY_OK`
- `NA0337_SERVICE_PRODUCTION_BOUNDARY_OK`
- `NA0337_NO_METADATA_FREE_CLAIM_OK`
- `NA0337_NO_TIMING_HIDDEN_CLAIM_OK`
- `NA0337_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK`
- `NA0337_METADATA_RUNTIME_PADDING_BUCKET_EXPANSION_OK`

## qshield Embedded Relay / Demo Boundary

The proof is limited to qshield embedded relay/demo candidate metadata,
qshield CLI demo config, and loopback-only executable harness behavior.

It does not prove public relay, production service, internet-path, service
deployment, or cross-repo behavior.

## qsl-server / qsl-attachments Production Boundary

qsl-server production padding remains unimplemented and unproven.

qsl-attachments production object-size padding remains unimplemented and
unproven.

Attachment-size padding remains future-gated.

## Claim Boundaries

This lane does not claim:

- anonymity;
- metadata-free behavior;
- untraceable behavior;
- production readiness;
- public-internet readiness;
- external review completion;
- that timing metadata is hidden;
- that traffic shape is hidden;
- that padding removes all metadata.

Padding may reduce bounded demo size-class observability for the configured
table, but timing, traffic shape, queue depth, contact graph, and service
metadata remain explicit gaps.

## Selected Successor

Selected successor:

`NA-0338 -- Metadata Runtime Attachment Size-Class Authorization Plan`

Rationale: NA-0337 completed the qshield demo padding bucket expansion
implementation/harness without an exact blocker. The next safest metadata lane
is authorization planning for attachment size-class behavior, not production
service timing or blocker resolution.

Rejected alternatives:

- `NA-0338 -- Metadata Runtime Padding Bucket Expansion Blocker Resolution`,
  because the implementation and harness completed.
- `NA-0338 -- Metadata Runtime Service Timing Cross-Repo Authorization`,
  because attachment size-class authorization is the next narrower metadata
  boundary after qshield demo padding proof.
- Direct production qsl-server or qsl-attachments padding, because those remain
  cross-repo-gated.
- NA-0338 implementation, because this lane selects only the successor.

## Backup-Plan Impact Statement

No backup-plan update is required. Changes remain tracked qsl-protocol files in
the existing qbuild worktree scope. Runtime artifacts are temporary harness
directories and are not committed. No durable evidence location outside the
existing repo and Codex response archive is introduced.

## Next Recommendation

Merge NA-0337 after required checks are green. If post-merge public-safety is
green, close out NA-0337 and restore exactly one READY successor:

`NA-0338 -- Metadata Runtime Attachment Size-Class Authorization Plan`
