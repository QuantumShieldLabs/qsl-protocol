Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-23

# NA-0339 Metadata Runtime qshield Demo Attachment Size-Class Harness

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0339 implements the bounded qshield embedded relay/demo attachment
size-class harness authorized by NA-0338. The implementation is opt-in through
`QSHIELD_DEMO_ATTACHMENT_SIZE_CLASSES=expanded` or the exact policy name
`qshield_demo_attachment_size_class_v1`.

The policy pads only the qshield demo attachment ciphertext relay object. The
descriptor remains separately encrypted and metadata-bearing: it still carries
the exact unpadded ciphertext length and hash used by receive-side validation.
Receive strips and verifies the padded ciphertext object before hashing,
decrypting, ACKing, or writing output.

This is local qshield embedded relay/demo evidence only. It is not
qsl-attachments production object-size padding, qsl-server production
timing/storage behavior, public-internet behavior, or evidence that attachment
size, timing metadata, traffic shape, or all metadata is hidden.

## Live NA-0339 Scope

Live `NEXT_ACTIONS.md` authorized:

- `NA-0339 -- Metadata Runtime qshield Demo Attachment Size-Class Implementation Harness`
- Status: READY
- Goals: G1, G2, G3, G4, G5
- Objective: execute a bounded qshield embedded relay/demo attachment
  size-class implementation harness, or stop on an exact prerequisite.

Protected boundaries preserved:

- no qsl-server, qsl-attachments, qsc/qsp/protocol/crypto/key-schedule,
  dependency, workflow, website, README, START_HERE, docs/public,
  branch-protection, or public-safety configuration change;
- no qsl-attachments production object-size padding;
- no qsl-server production timing/storage behavior;
- no public-internet attachment behavior;
- no claim that attachment size, timing metadata, traffic shape, or all
  metadata is hidden;
- no anonymity, metadata-free, untraceable, production-readiness,
  public-internet-readiness, or external-review-complete claim.

## Inherited NA-0338 Authorization

NA-0338 D-0658 authorized exactly one future bounded qshield demo attachment
size-class implementation harness. It defined:

- policy name `qshield_demo_attachment_size_class_v1`;
- deterministic table `[256, 512, 768, 1024, 1536, 2048, 3072, 4096, 5120,
  6144, 7168, 8192]`;
- max padded qshield demo attachment object size 8192 bytes;
- max overhead 1023 bytes;
- deterministic invalid-config, oversize, malformed descriptor, and malformed
  ciphertext rejection;
- no accepted state or output on invalid object;
- no remote ACK/delete before local verification;
- retention, purge, backup, and artifact safety proof;
- explicit qshield demo versus qsl-server/qsl-attachments production boundary.

## Inherited NA-0337 Padding Bucket Proof

NA-0337 D-0656 implemented bounded qshield embedded relay/demo padding bucket
expansion using the same deterministic table and cap. It proved max overhead
1023 bytes, strict invalid-config rejection, relay/receive padding metadata
caps, zero-padding verify before actor decode, no remote delete before local
verification, no accepted state/output on invalid padding, artifact safety, and
batching/retry/jitter/cover preservation.

NA-0339 reuses the table shape and fail-closed style, but applies it only to
the qshield demo attachment ciphertext object. It does not broaden transport
padding, cover traffic, batching, retry cadence, or bounded jitter behavior.

## Sources Inspected

Sources reviewed before patching:

- `NEXT_ACTIONS.md` live NA-0339 entry.
- `tests/NA-0338_closeout_restore_na0339_testplan.md`.
- `docs/governance/evidence/NA-0338_metadata_runtime_attachment_size_class_authorization.md`.
- `tests/NA-0338_metadata_runtime_attachment_size_class_authorization_testplan.md`.
- `docs/governance/evidence/NA-0337_metadata_runtime_qshield_demo_padding_bucket_expansion_harness.md`.
- `docs/governance/evidence/NA-0336_metadata_runtime_padding_bucket_expansion_authorization.md`.
- `docs/governance/evidence/NA-0335_metadata_runtime_qshield_demo_cover_traffic_prototype_harness.md`.
- `apps/qshield-cli/src/commands/attachment.rs`.
- `apps/qshield-cli/src/commands/init.rs`.
- `apps/qshield-cli/src/config.rs`.
- `apps/qshield-cli/src/relay_client.rs`.
- `apps/qshield-cli/tests/na_0337_metadata_runtime_padding_bucket_expansion.rs`.
- qsl-attachments production evidence references in qsl-protocol governance and
  canonical docs. The `qsl-attachments/**` source tree is not present in this
  checkout and was not changed.
- `TRACEABILITY.md`.
- `DECISIONS.md`.

Search coverage included attachment, attachment size, size class, size-class,
object size, upload, fetch, object, descriptor, ciphertext, padding, bucket,
object-size padding, qsl-attachments, retention, purge, backup, malformed
descriptor, malformed ciphertext, oversize, no output, no accepted state,
production, qshield demo, metadata-free, anonymity, untraceable, size hidden,
timing hidden, traffic hidden, `FUTURE_GATE`, and `NOT_READY`.

## Implementation Summary or Blocker

No implementation blocker was found inside the authorized qshield surface.

Changed qshield files:

- `apps/qshield-cli/src/config.rs`
- `apps/qshield-cli/src/commands/attachment.rs`
- `apps/qshield-cli/tests/na_0339_metadata_runtime_attachment_size_class.rs`

Changed governance/evidence files:

- this evidence file;
- `tests/NA-0339_metadata_runtime_qshield_demo_attachment_size_class_harness_testplan.md`;
- `DECISIONS.md`;
- `TRACEABILITY.md`;
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

## Attachment Size-Class Policy

Policy: `qshield_demo_attachment_size_class_v1`.

Opt-in gate: `QSHIELD_DEMO_ATTACHMENT_SIZE_CLASSES`.

Accepted opt-in values:

- `expanded`;
- `qshield_demo_attachment_size_class_v1`;
- explicit sorted CSV class table.

Deterministic table:

`[256, 512, 768, 1024, 1536, 2048, 3072, 4096, 5120, 6144, 7168, 8192]`

Maximum padded qshield demo attachment object size: 8192 bytes.

Maximum overhead: 1023 bytes.

## Deterministic Test Mode

The harness is deterministic without long sleeps. It uses a loopback qshield
relay, temporary stores, the local `refimpl_actor`, fixed payload sizes,
deterministic env gates, and direct candidate inspection through
`/poll-candidate`, which does not delete queue entries.

## Future / Actual Implementation Boundary

Actual implementation:

- qshield demo attachment send validates the attachment size-class env policy
  before session or actor work;
- qshield demo attachment send pads only the ciphertext relay object and
  queues relay `pad_len` / `bucket` metadata;
- qshield demo attachment receive decrypts and validates the descriptor,
  verifies ciphertext object metadata and zero padding, strips the object,
  checks unpadded length/hash, decrypts payload, then ACKs and writes output.

Not implemented:

- qsl-attachments production object-size padding;
- qsl-server production timing/storage behavior;
- public-internet attachment behavior;
- protocol, crypto, qsc, qsp, state-machine, or key-schedule behavior;
- dependency, Cargo, workflow, branch-protection, or public-safety changes.

## Abuse / Cost / Latency / Compatibility Behavior

Abuse and cost are bounded by:

- 8192-byte maximum padded qshield demo attachment ciphertext object;
- existing embedded relay body and queue caps;
- deterministic oversize rejection before relay queue mutation;
- deterministic invalid-config rejection before session or actor work.

Latency behavior remains explicit:

- no claim is made that timing metadata is hidden;
- no fixed-rate scheduling or production timing mitigation is introduced;
- retry cadence and bounded jitter remain separate qshield demo behaviors.

Compatibility behavior:

- existing attachment send/recv remains compatible when the env gate is absent;
- descriptor and ciphertext remain separate relay candidates;
- descriptor remains metadata-bearing and continues to carry exact unpadded
  ciphertext length/hash;
- malformed padded objects reject before ACK/output.

## Valid Small / Medium / Large Attachment Proof

Harness:
`apps/qshield-cli/tests/na_0339_metadata_runtime_attachment_size_class.rs`

The harness sends and receives valid small, medium, and large attachments with
`QSHIELD_DEMO_ATTACHMENT_SIZE_CLASSES=expanded`. For each case it inspects the
queued descriptor/ciphertext pair before receive, proves the descriptor remains
separate and unpadded, proves the ciphertext object length equals a configured
size class, receives and decrypts, compares output bytes to source bytes, and
proves the relay queue is purged only after successful local verification.

Markers:

- `NA0339_VALID_SMALL_ATTACHMENT_OK`
- `NA0339_VALID_MEDIUM_ATTACHMENT_OK`
- `NA0339_VALID_LARGE_ATTACHMENT_OK`
- `NA0339_ATTACHMENT_RETENTION_PURGE_BOUNDARY_OK`

## Max Overhead Proof

The harness iterates object lengths `1..=8192` against the deterministic table
and proves the worst overhead is 1023 bytes.

Marker: `NA0339_ATTACHMENT_MAX_OVERHEAD_BOUNDARY_OK`

## Invalid Config Proof

The harness proves deterministic rejection for:

- empty env value;
- zero class;
- negative class;
- duplicate-after-normalization class;
- unsorted class list;
- class above cap;
- very large numeric class;
- invalid nonnumeric value.

Invalid config rejects before qshield store config or state files are created.

Marker: `NA0339_ATTACHMENT_INVALID_CONFIG_REJECT_OK`

## Oversize Reject Proof

The harness sends an attachment whose produced demo ciphertext object cannot fit
within the 8192-byte cap. Send rejects with
`attachment size class object exceeds demo maximum` before descriptor or
ciphertext relay queue mutation.

Marker: `NA0339_ATTACHMENT_OVERSIZE_REJECT_OK`

## Malformed Descriptor Proof

The harness queues a malformed descriptor/ciphertext pair directly through the
embedded relay candidate path, runs `qshield attachment recv`, proves
`attachment_descriptor_reject`, proves both candidates remain queued with the
same ACK IDs, proves receiver state is unchanged, and proves no output file is
written.

Marker: `NA0339_ATTACHMENT_MALFORMED_DESCRIPTOR_REJECT_OK`

## Malformed Ciphertext Proof

The harness uses `qshield attachment send --tamper-ciphertext` under the
size-class policy, then proves receive rejects with `attachment_integrity_reject`
before ACK/output/state mutation.

Marker: `NA0339_ATTACHMENT_MALFORMED_CIPHERTEXT_REJECT_OK`

## Retention / Purge Proof

Valid attachment receive ACKs both descriptor and ciphertext candidates only
after descriptor validation, ciphertext object strip/verify, length/hash check,
and payload decrypt. Invalid descriptor and malformed ciphertext paths retain
the queued candidates and preserve ACK IDs.

Marker: `NA0339_ATTACHMENT_RETENTION_PURGE_BOUNDARY_OK`

## Backup-Boundary Proof

The implementation creates no durable runtime artifact location outside the
qsl-protocol tracked tree. The harness uses temporary directories under the
host temp root and removes them through test cleanup. No backup-plan update is
required.

Marker: `NA0339_ATTACHMENT_BACKUP_BOUNDARY_OK`

## No Accepted State / Output Proof

Malformed descriptor and malformed ciphertext tests compare `state.json` before
and after reject and assert the output directory remains absent or empty.

Markers:

- `NA0339_ATTACHMENT_NO_ACCEPTED_STATE_ON_REJECT_OK`
- `NA0339_ATTACHMENT_NO_OUTPUT_ON_REJECT_OK`

## Artifact / Log Safety Proof

The harness scans command output for route-token, raw-handle, descriptor,
ciphertext, plaintext, padding, passphrase, raw-key, panic, backtrace, and
sensitive absolute-path sentinels. The malformed ciphertext proof emits:

- `ATTACHMENT_ARTIFACT_SECRET_FINDING_COUNT 0`
- `ATTACHMENT_ARTIFACT_SIZE_WITHIN_CAP_OK`
- `ATTACHMENT_NO_PAYLOAD_SENTINEL_LEAK_OK`

Marker: `NA0339_ATTACHMENT_NO_SECRET_ARTIFACT_OK`

## Padding / Cover / Batching / Retry / Jitter Preservation Proof

The NA-0339 patch does not modify padding, cover, batching, retry-cadence, or
jitter source files. The harness also starts a relay with batching and cover
enabled, proves `/send-batch` still reports `qshield_demo_batching_v1`, proves
cover traffic still reports `qshield_demo_cover_traffic_v1`, and confirms the
attachment-sized padded object coexists with cover candidates without changing
those caps.

Marker: `NA0339_PADDING_COVER_BATCHING_RETRY_JITTER_STILL_BOUNDED_OK`

## Harness Markers

The harness declares and emits all NA-0339 markers:

- `NA0339_ATTACHMENT_SIZE_CLASS_AUTHORIZATION_OK`
- `NA0339_ATTACHMENT_SIZE_CLASS_POLICY_OK`
- `NA0339_DETERMINISTIC_TEST_ATTACHMENT_SIZE_CLASS_OK`
- `NA0339_VALID_SMALL_ATTACHMENT_OK`
- `NA0339_VALID_MEDIUM_ATTACHMENT_OK`
- `NA0339_VALID_LARGE_ATTACHMENT_OK`
- `NA0339_ATTACHMENT_MAX_OVERHEAD_BOUNDARY_OK`
- `NA0339_ATTACHMENT_INVALID_CONFIG_REJECT_OK`
- `NA0339_ATTACHMENT_OVERSIZE_REJECT_OK`
- `NA0339_ATTACHMENT_MALFORMED_DESCRIPTOR_REJECT_OK`
- `NA0339_ATTACHMENT_MALFORMED_CIPHERTEXT_REJECT_OK`
- `NA0339_ATTACHMENT_RETENTION_PURGE_BOUNDARY_OK`
- `NA0339_ATTACHMENT_BACKUP_BOUNDARY_OK`
- `NA0339_ATTACHMENT_NO_ACCEPTED_STATE_ON_REJECT_OK`
- `NA0339_ATTACHMENT_NO_OUTPUT_ON_REJECT_OK`
- `NA0339_ATTACHMENT_NO_SECRET_ARTIFACT_OK`
- `NA0339_PADDING_COVER_BATCHING_RETRY_JITTER_STILL_BOUNDED_OK`
- `NA0339_QSHIELD_DEMO_BOUNDARY_OK`
- `NA0339_QSL_ATTACHMENTS_PRODUCTION_BOUNDARY_OK`
- `NA0339_NO_METADATA_FREE_CLAIM_OK`
- `NA0339_NO_SIZE_HIDDEN_CLAIM_OK`
- `NA0339_NO_TIMING_HIDDEN_CLAIM_OK`
- `NA0339_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK`
- `NA0339_METADATA_RUNTIME_ATTACHMENT_SIZE_CLASS_OK`

## qshield Embedded Relay / Demo Boundary

All executable proof is bounded to qshield embedded relay/demo behavior. It
uses the local demo relay candidate queue and qshield CLI attachment proof path.
It does not exercise or imply production relay, public internet, or service
deployment behavior.

## qsl-server / qsl-attachments Production Boundary

qsl-server production timing/storage behavior remains unproven and
cross-repo-gated. qsl-attachments production object-size padding remains
unimplemented and cross-repo-gated. qsl-attachments production upload/fetch,
retention, backup growth, observability, deployment, and public-internet
behavior require a separate authorization lane.

## Claim Boundaries

NA-0339 does not claim:

- attachment size is hidden;
- timing metadata is hidden;
- traffic shape is hidden;
- padding hides all metadata;
- metadata-free behavior;
- anonymity;
- untraceable behavior;
- production readiness;
- public-internet readiness;
- external review completion.

The descriptor remains metadata-bearing and the evidence intentionally keeps all
remaining metadata-runtime gaps visible.

## Selected Successor

Selected successor:

`NA-0340 -- Metadata Runtime qsl-attachments Production Size-Class Cross-Repo Authorization`

## Backup-Plan Impact Statement

No backup-plan update is required. Tracked changes remain under qsl-protocol in
the `/srv/qbuild/work` tree. Runtime artifacts are temporary test directories
and are not committed.

## Next Recommendation

Close NA-0339 after Packet K merges and public-safety is green, then restore
the selected NA-0340 authorization lane. NA-0340 must not implement production
object-size padding until cross-repo authorization is explicit.
