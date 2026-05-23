Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-23

# NA-0339 Metadata Runtime qshield Demo Attachment Size-Class Harness Test Plan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the bounded qshield embedded relay/demo attachment size-class
implementation and executable harness selected by NA-0338, while preserving all
qshield demo versus qsl-server/qsl-attachments production boundaries and
metadata-claim limits.

## Protected Invariants

- Attachment size-class behavior remains qshield embedded relay/demo only.
- Existing attachment send/fetch remains compatible when the env gate is absent.
- The descriptor remains separately encrypted and metadata-bearing.
- The ciphertext object boundary is deterministic, capped, and verified before
  accepted state/output.
- Invalid config rejects deterministically.
- Oversized object rejects deterministically.
- Malformed descriptor rejects deterministically.
- Malformed ciphertext rejects deterministically.
- Invalid objects create no accepted state and no output.
- No remote ACK/delete occurs before local verification.
- Retention/purge and backup impact remain bounded.
- Artifacts and logs remain secret-free.
- Padding, cover, batching, retry-cadence, and bounded jitter boundaries remain
  intact.
- qsl-server and qsl-attachments production behavior remains unproven and
  cross-repo-gated.

## Allowed Scope

- `apps/qshield-cli/src/commands/attachment.rs`
- `apps/qshield-cli/src/config.rs`
- `apps/qshield-cli/tests/na_0339_metadata_runtime_attachment_size_class.rs`
- `docs/governance/evidence/NA-0339_metadata_runtime_qshield_demo_attachment_size_class_harness.md`
- this testplan
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

- qsl-server implementation changes.
- qsl-attachments implementation changes.
- qsc/qsp/protocol/crypto/state-machine/key-schedule implementation changes.
- Dependency, Cargo manifest, or lockfile changes.
- Workflow, branch-protection, or public-safety configuration changes.
- Website, README, START_HERE, docs/public, qsc-desktop, formal, input,
  tool/refimpl, production-service, or public-copy changes.
- qsl-attachments production object-size padding.
- qsl-server production timing/storage behavior.
- Public-internet attachment behavior.
- NA-0340 implementation.

## Prior Authorization Review Requirements

Before implementation, review:

- live NA-0339 scope in `NEXT_ACTIONS.md`;
- NA-0338 authorization evidence and closeout testplan;
- NA-0337 qshield demo padding bucket evidence and harness;
- NA-0336 padding authorization;
- NA-0335 cover prototype evidence;
- current qshield attachment source, config source, relay client, and relevant
  qshield metadata-runtime tests;
- qsl-server/qsl-attachments production-boundary governance evidence;
- `TRACEABILITY.md` and `DECISIONS.md`.

## Attachment Size-Class Implementation Requirements

The implementation must:

- use policy `qshield_demo_attachment_size_class_v1`;
- use deterministic table `[256, 512, 768, 1024, 1536, 2048, 3072, 4096,
  5120, 6144, 7168, 8192]`;
- cap padded qshield demo attachment ciphertext objects at 8192 bytes;
- cap overhead at 1023 bytes;
- reject invalid class configuration before session or actor work;
- pad only the attachment ciphertext relay object;
- keep descriptor and ciphertext as separate relay candidates;
- strip and verify zero padding before ciphertext length/hash/decrypt;
- ACK only after descriptor validation, object verification, hash check, and
  payload decrypt;
- write output only on valid receive.

## Harness Marker Requirements

The harness must declare and emit:

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

## Invalid Config Requirements

Executable proof must cover:

- empty config;
- zero class;
- negative class;
- duplicate-after-normalization class;
- unsorted class list;
- class above cap;
- very large class;
- invalid nonnumeric value.

Invalid config must create no qshield config/state.

## Oversize and Malformed Object Requirements

Executable proof must cover:

- oversized attachment object reject before queue mutation;
- malformed descriptor reject before ACK/output;
- malformed ciphertext or padding reject before ACK/output;
- deterministic reject strings that do not expose secret-bearing diagnostics.

## Retention / Purge Requirements

Valid receive must purge descriptor and ciphertext candidates only after local
verification. Invalid descriptor and malformed ciphertext paths must retain the
same queued ACK IDs.

## Backup-Boundary Requirements

Runtime artifacts must remain temporary and bounded. No durable evidence
location outside qsl-protocol or current qbuild backup coverage may be
introduced.

## No-Local-Output / State Requirements

Invalid descriptor and malformed ciphertext tests must compare state before and
after reject and prove no output file is written.

## No-Secret-Artifact Requirements

Harness output must be scanned for route token, raw handle, descriptor,
ciphertext, plaintext, padding, passphrase, raw key, panic/backtrace, and
sensitive path sentinels. Required artifact markers:

- `ATTACHMENT_ARTIFACT_SECRET_FINDING_COUNT 0`
- `ATTACHMENT_ARTIFACT_SIZE_WITHIN_CAP_OK`
- `ATTACHMENT_NO_PAYLOAD_SENTINEL_LEAK_OK`

## Production-Boundary Requirements

Evidence must state that:

- qshield proof is local/demo only;
- qsl-attachments production object-size padding remains unimplemented and
  cross-repo-gated;
- qsl-server production timing/storage behavior remains unproven and
  cross-repo-gated;
- public-internet behavior remains unproven.

## Claim-Boundary Requirements

Do not claim:

- attachment size is hidden;
- timing metadata is hidden;
- traffic shape is hidden;
- padding hides all metadata;
- metadata-free behavior;
- anonymity;
- untraceable behavior;
- production readiness;
- public-internet readiness;
- external review completion;
- quantum-proof hype, unbreakable, military-grade, or guaranteed secure
  properties.

Allowed only when negated, prohibited, or classified as future/unproven.

## Backup-Impact Requirements

Record whether the patch creates durable evidence outside current qbuild backup
scope. Expected result: no backup-plan update required.

## Required Local Checks

Run:

- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- `cargo +stable test -p qshield-cli --locked --test na_0339_metadata_runtime_attachment_size_class -- --test-threads=1 --nocapture`
- relevant prior qshield metadata-runtime harnesses from NA-0337, NA-0335,
  NA-0331, NA-0329, NA-0327, NA-0324, NA-0322, NA-0320, NA-0319, and NA-0318
- `cargo +stable test -p qshield-cli --locked -- --test-threads=1`
- `cargo +stable build -p qshield-cli --locked`
- demo smoke/stress/soak checks when feasible
- metadata runtime plan, identifier/padding, sanitized-errors/retention, and
  conformance smoke harnesses
- qsc send_commit and formal/model checks
- queue/decision helpers
- scope guard, link-check, leak-scan, goal-lint, classifier proof, and
  overclaim scan.

## CI Expectations

Required checks must attach to the PR head and complete green before merge.
`public-safety` must remain required and green. Merge must use normal merge with
`--match-head-commit`, no admin bypass, no squash, no rebase, no direct push,
and no delete-branch flag.

## Successor Handoff

If NA-0339 succeeds, select:

`NA-0340 -- Metadata Runtime qsl-attachments Production Size-Class Cross-Repo Authorization`

Do not implement NA-0340 in NA-0339.
