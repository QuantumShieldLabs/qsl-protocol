Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-23

# NA-0338 Metadata Runtime Attachment Size-Class Authorization

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0338 is an authorization/design-only lane. It reviews current qshield demo
attachment behavior, inherited qshield padding/metadata-runtime evidence, and
qsl-server/qsl-attachments production boundaries, then selects the next exact
successor.

Result: a future bounded qshield embedded relay/demo implementation harness is
authorized as:

`NA-0339 -- Metadata Runtime qshield Demo Attachment Size-Class Implementation Harness`

The future lane is safe only inside the qshield demo attachment surface. It may
prove a deterministic demo attachment object size-class policy, but it must not
implement or imply qsl-attachments production object-size padding, qsl-server
production padding/timing behavior, public internet behavior, or any stronger
privacy/readiness claim.

NA-0338 implements no attachment size-class padding, no runtime mitigation, no
qshield runtime change, no qsl-server/qsl-attachments change, no protocol or
crypto change, and no dependency/workflow/public-doc change.

## Live NA-0338 Scope

Live `NEXT_ACTIONS.md` scope:

- `NA-0338 -- Metadata Runtime Attachment Size-Class Authorization Plan`
- Status: READY
- Goals: G1, G2, G3, G4, G5
- Objective: execute the next metadata-runtime padding/size-class lane selected
  by NA-0337: an attachment size-class authorization plan, or stop on an exact
  prerequisite.

Protected boundaries:

- no unsupported production, public-internet, external-review, anonymity,
  unsupported metadata-free, or unsupported untraceable claim;
- no claim that timing metadata or traffic shape is hidden unless exact future
  evidence proves it;
- qsl-server and qsl-attachments production boundary remains explicit;
- qshield embedded relay/demo proof remains distinct from qsl-server and
  qsl-attachments production behavior;
- no qsl-server, qsl-attachments, qsc/qsp/protocol/crypto/key-schedule,
  dependency, workflow, website, README, START_HERE, docs/public,
  branch-protection, or public-safety configuration change unless exact future
  scope authorizes it;
- no attachment-size padding, qsl-server production padding,
  qsl-attachments production object-size padding, public-internet traffic
  observation, or production service timing implementation unless exact future
  scope authorizes it.

NA-0338 stays inside that scope.

## Inherited NA-0337 qshield Demo Padding Bucket Proof

NA-0337 implemented the bounded qshield embedded relay/demo padding bucket
expansion policy `qshield_demo_padding_bucket_expansion_v1`.

Inherited proof:

- deterministic table:
  `[256, 512, 768, 1024, 1536, 2048, 3072, 4096, 5120, 6144, 7168, 8192]`;
- max padded payload: 8192 bytes;
- max overhead: 1023 bytes;
- strict invalid-config rejection;
- relay and receive padding metadata caps;
- zero-padding verify before actor decode;
- strip only after verification;
- no remote delete before local verification;
- no accepted state/output on invalid padding;
- artifact safety proof;
- batching, retry, bounded jitter, and cover prototype bounds preserved.

This proof is local qshield embedded relay/demo evidence only. It is not
qsl-server production padding proof, not qsl-attachments object-size proof, not
attachment padding proof, not public-internet proof, and not evidence that
timing metadata, traffic shape, or all metadata is removed.

## Inherited Attachment / Metadata-Runtime Evidence

Current relevant evidence:

- NA-0260 proved qshield demo attachment descriptor/fetch/decrypt/integrity
  readiness. The qshield demo path encrypts a demo descriptor and a payload
  wire separately through the Suite-2 demo actor path, queues opaque wire hex in
  the local relay, validates descriptor fields plus ciphertext length/hash on
  receive, decrypts the payload, and writes output only on the valid path.
- NA-0318 proved qshield candidate fetch plus explicit ack/commit after local
  verification.
- NA-0319 proved bounded qshield identifier/default-padding behavior and
  invalid padding no-delete/no-state/no-output behavior.
- NA-0320 proved selected sanitized-error and retention/purge behavior,
  including attachment reject retention in qshield demo paths.
- NA-0321 and NA-0324 explicitly classify attachment descriptor/ciphertext size
  and timing as still observable/unproven. They measure/instrument selected
  qshield demo surfaces but do not prove timing or traffic-shape hiding.
- NA-0335 proved bounded opt-in qshield demo cover traffic with retention,
  purge, backup, abuse, artifact, batching/retry/jitter, and production-boundary
  proof.
- NA-0337 proved the expanded qshield demo padding bucket policy and its
  compatibility with batching/retry/jitter/cover.
- DOC-CAN-005, DOC-CAN-006, DOC-CAN-007, DOC-ATT-001, DOC-ATT-002, and
  NA-0287 keep production attachment service behavior separate from qshield
  demo behavior.

## Sources Inspected

Sources reviewed before patching:

- `NEXT_ACTIONS.md` live NA-0338 entry.
- `tests/NA-0337_closeout_restore_na0338_testplan.md`.
- `docs/governance/evidence/NA-0337_metadata_runtime_qshield_demo_padding_bucket_expansion_harness.md`.
- `tests/NA-0337_metadata_runtime_qshield_demo_padding_bucket_expansion_harness_testplan.md`.
- `docs/governance/evidence/NA-0336_metadata_runtime_padding_bucket_expansion_authorization.md`.
- `docs/governance/evidence/NA-0335_metadata_runtime_qshield_demo_cover_traffic_prototype_harness.md`.
- `docs/governance/evidence/NA-0319_metadata_runtime_identifier_default_padding_harness.md`.
- `docs/governance/evidence/NA-0321_metadata_runtime_timing_traffic_shape_threat_model.md`.
- `docs/governance/evidence/NA-0324_metadata_runtime_timing_traffic_instrumentation_harness.md`.
- `docs/governance/evidence/NA-0260_attachment_demo_readiness_audit.md`.
- `docs/canonical/DOC-CAN-005_QSP_Attachment_Descriptor_and_Control_Plane_v0.1.0_DRAFT.md`.
- `docs/canonical/DOC-CAN-006_QATT_Attachment_Service_Contract_v0.1.0_DRAFT.md`.
- `docs/canonical/DOC-CAN-007_QATT_Attachment_Encryption_Context_and_Part_Cipher_v0.1.0_DRAFT.md`.
- `docs/design/DOC-ATT-001_Signal_Class_Attachment_Architecture_Program_v0.1.0_DRAFT.md`.
- `docs/design/DOC-ATT-002_qsl-attachments_Deployment_and_Operational_Hardening_Contract_v0.1.0_DRAFT.md`.
- `docs/governance/evidence/NA-0285_qsl_attachments_backup_restore_recovery_boundary_plan.md`.
- `docs/governance/evidence/NA-0286_qsl_attachments_backup_restore_recovery_harness.md`.
- `docs/governance/evidence/NA-0287_service_production_gate_evidence_map.md`.
- `apps/qshield-cli/src/commands/attachment.rs`.
- `apps/qshield-cli/src/commands/send.rs`.
- `apps/qshield-cli/src/commands/recv.rs`.
- `apps/qshield-cli/src/commands/relay.rs`.
- `apps/qshield-cli/src/relay_client.rs`.
- `apps/qshield-cli/tests/na_0319_metadata_runtime_identifier_padding.rs`.
- `apps/qshield-cli/tests/na_0337_metadata_runtime_padding_bucket_expansion.rs`.
- `TRACEABILITY.md`.
- `DECISIONS.md`.

Search coverage included attachment, object size, size class, size-class,
attachment size, upload, fetch, object, descriptor, ciphertext, padding, bucket,
object-size padding, qsl-attachments, production, qshield demo, backup,
retention, purge, timing hidden, traffic hidden, metadata-free, anonymity,
untraceable, `FUTURE_GATE`, and `NOT_READY`.

The `qsl-attachments/**` directory is not present inside this qsl-protocol
checkout; production boundary review therefore used the qsl-protocol canonical,
design, traceability, and governance evidence that records sibling
qsl-attachments behavior.

## Current Attachment Behavior

The current qshield demo attachment flow is bounded and local/demo-only:

1. `qshield attachment send` reads a local file fully into memory and rejects
   empty payloads.
2. It creates a demo descriptor with `v = 1`,
   `t = qshield_demo_attachment_descriptor`, a deterministic attachment id,
   filename hint, unpadded attachment wire length, unpadded attachment wire
   SHA-256, demo encryption context name, demo locator kind, and
   `non_production = true`.
3. It encrypts the attachment payload through the existing Suite-2 demo actor
   path, then encrypts the descriptor through the same actor path.
4. It queues descriptor wire hex first and ciphertext wire hex second through
   qshield embedded relay `/send`.
5. Current attachment send does not use qshield padding bucket metadata, does
   not use batching, and does not call qsl-attachments.
6. `qshield attachment recv` fetches candidates through `/poll-candidate`, so
   candidate fetch itself does not delete the remote queue entries.
7. Receive requires a descriptor/ciphertext pair from the same sender, decrypts
   and validates the descriptor, validates ciphertext length and hash, decrypts
   the payload, then acks both candidates and writes the output file.
8. Descriptor reject, integrity reject, and decrypt reject happen before
   remote ack in the current code path.

Current gaps:

- no attachment object size-class padding exists;
- no attachment object-size padding exists in qsl-attachments production;
- descriptor/ciphertext size linkage remains observable in bounded demo
  evidence;
- current qshield demo attachment behavior does not prove production
  retention, purge, quota, backup, deployment, public-internet, or
  qsl-attachments behavior.

## qsl-attachments Production Boundary

qsl-attachments production behavior is owned outside this qshield demo lane.
Current qsl-protocol evidence records:

- qsl-attachments is an opaque ciphertext service, not a plaintext attachment
  service;
- DOC-CAN-006 says committed object retrieval returns ciphertext bytes exactly
  as committed and does not synthesize, pad, transform, or recompress them;
- DOC-CAN-006 logging/metadata rules treat ciphertext length, part count, part
  size class, retention class, session/object expiry timestamps, upload/download
  timing, and service-side access events as unavoidable service metadata;
- NA-0287 classifies qsl-attachments local hardening evidence as meaningful but
  still future-gates production service operation, public internet exposure,
  observability, long-running operations, backup automation, and external
  review.

Therefore a qshield demo attachment size-class harness may be authorized without
claiming qsl-attachments production object-size padding. Any production
qsl-attachments object-size padding or object-size class behavior requires
separate cross-repo authorization and service-owned tests.

## Attachment Size-Class Problem Statement

Current qshield demo attachment proof validates descriptor/fetch/decrypt and
integrity behavior, but it leaves the second queued attachment object at its
natural Suite-2 demo wire size. A relay observer in the local demo can still
observe two queued candidates and the ciphertext-object wire size class. NA-0337
proved a deterministic qshield demo padding bucket policy for ordinary message
candidates, but that does not automatically cover the attachment descriptor/
ciphertext pair.

The future question is narrow:

- can qshield demo attachment ciphertext objects be padded into deterministic
  bounded size classes;
- can invalid size-class config and malformed padded objects reject
  deterministically;
- can no-output/no-state and no-remote-delete-before-local-verification
  behavior be preserved;
- can retention, purge, backup, artifact, cover, batching, retry, and jitter
  boundaries remain explicit;
- can all production and public-claim boundaries stay conservative.

## Attachment Size-Class Semantic Design

Authorized future qshield demo policy name:

`qshield_demo_attachment_size_class_v1`

Future deterministic demo size-class table:

`[256, 512, 768, 1024, 1536, 2048, 3072, 4096, 5120, 6144, 7168, 8192]`

Future maximum padded attachment object size:

- `8192` bytes, measured on the padded qshield demo attachment ciphertext wire
  object before hex encoding for relay JSON carriage.

Future maximum overhead:

- `1023` bytes, measured as `size_class_bytes - unpadded_ciphertext_wire_len`
  for unpadded object wire lengths in `1..=8192`.

Future descriptor/version shape:

- use a demo descriptor version that unambiguously identifies size-class
  semantics before receive accepts the object;
- keep the descriptor encrypted in the qshield demo message plane;
- include the unpadded ciphertext object length and hash needed to verify and
  strip;
- include the size-class policy name, selected class, and padding length or
  equivalent deterministic fields;
- keep any size-class descriptor fields demo-only and not a production QATT
  contract.

Future send semantics:

- encrypt the attachment payload through the existing Suite-2 demo actor path;
- reject if the unpadded attachment wire object is empty or exceeds 8192 bytes;
- choose the smallest class that can hold the unpadded object;
- append zero padding to the attachment object wire to the selected class;
- queue the encrypted descriptor and padded attachment object through qshield
  embedded relay only;
- reject invalid size-class config before actor/session/relay mutation where
  feasible;
- never silently fall back from invalid size-class config to a weaker or
  broader profile.

Future receive semantics:

- fetch descriptor/object candidates through `/poll-candidate`;
- validate same-sender pair and descriptor type/version before object handling;
- validate policy name, selected class, padding length, object byte length, and
  zero padding before actor decrypt;
- strip padding only after validation;
- verify the unpadded object hash and length before actor decrypt;
- actor-decrypt only the unpadded object;
- produce no plaintext output and no accepted attachment state on invalid
  descriptor, invalid config, oversized object, malformed object, padding
  mismatch, hash mismatch, or actor decrypt failure;
- do not ack/delete remote descriptor/object candidates before descriptor,
  size-class, strip, hash, and actor-decrypt verification succeeds; if a future
  directive preserves the current ack-before-final-write ordering, it must
  bound and test that exact attachment output-persistence behavior.

Compatibility requirements:

- existing valid attachment send/fetch/decrypt behavior must still pass when
  size-class mode is disabled;
- existing qshield message padding behavior must remain unchanged;
- batching, retry-cadence, bounded jitter, and cover prototype tests must
  remain bounded;
- qsl-server and qsl-attachments production behavior must remain unchanged.

Claim boundary:

- the future table may reduce observable qshield demo attachment object sizes
  into bounded local/demo classes;
- it does not remove all size metadata;
- it does not prove that attachment size is hidden;
- it does not prove timing metadata, traffic shape, contact graph, IP metadata,
  queue depth, production service behavior, public-internet behavior, or
  qsl-attachments behavior.

## Future Implementation Boundary

Future allowed qshield files for NA-0339, if separately directed:

- `apps/qshield-cli/src/commands/attachment.rs`
- `apps/qshield-cli/src/commands/init.rs` or `apps/qshield-cli/src/config.rs`
  only if the future directive chooses persistent demo config rather than an
  environment-gated deterministic test profile
- `apps/qshield-cli/src/relay_client.rs` only if bounded demo metadata
  compatibility requires it
- `apps/qshield-cli/tests/na_0339_metadata_runtime_attachment_size_class.rs`
- established qshield test helpers already used by adjacent harnesses

Future allowed fixture/script paths only if the future directive explicitly
permits them:

- `inputs/metadata_runtime/attachment_size_class_fixture_v1.json`
- `scripts/ci/metadata_runtime_attachment_size_class_harness.sh`

Future forbidden files without new authorization:

- `qsl-attachments/**`
- `qsl-server/**`
- `qsc/**`
- `qsp/**`
- protocol, crypto, or key-schedule implementation paths
- `Cargo.toml`
- `Cargo.lock`
- `.github/**`
- `website/**`
- `README.md`
- `START_HERE.md`
- `qsc-desktop/**`
- branch-protection or public-safety configuration

Future proof requirements:

- valid attachment send/fetch/decrypt still succeeds;
- size-class table and policy name are deterministic;
- invalid config rejects deterministically;
- oversized object rejects deterministically;
- malformed object rejects deterministically;
- no accepted state/output on invalid object;
- no remote delete before local verification succeeds;
- retention/purge behavior is deterministic and bounded to qshield candidate
  ack/queue behavior;
- backup impact is bounded;
- artifacts/logs are secret-free;
- qsl-attachments production boundary is explicit;
- no metadata-free, size-hidden, timing-hidden, traffic-shape-hidden,
  anonymity, untraceable, production-readiness, public-internet-readiness, or
  external-review-complete claim.

## Abuse / Cost / Latency / Compatibility Matrix

| Scenario | Risk | Proposed bound | Future test | Failure mode | Stop condition | Compatibility impact | Claim boundary |
| --- | --- | --- | --- | --- | --- | --- | --- |
| valid small attachment | Tiny object can reveal a small size class. | Pad to first class at or above object wire length, maximum class 8192. | Send/fetch/decrypt a small fixture and assert selected class. | wrong class, decrypt failure, output mismatch | class not deterministic or valid path breaks | disabled mode still passes | local demo class only |
| valid medium attachment | Mid-size object can expose exact object length without padding. | Pad to deterministic medium class. | Fixture whose wire lands in a mid table range. | class mismatch or hash mismatch | selected class not stable | no message padding drift | not a size-hidden claim |
| valid large demo attachment within cap | Large local demo object can approach relay/body limits. | Max padded object size 8192 bytes; reject above cap. | Fixture near 7169/8192 boundary. | relay body/cap confusion | object over cap accepted | qshield relay cap remains intact | not production object proof |
| oversized attachment | Memory/cost and request-size amplification. | Reject unpadded object wire above 8192 before relay mutation. | Generate object above cap. | queued oversized object | any relay mutation on oversize | no qsl-attachments behavior | local demo only |
| malformed descriptor | Receiver could accept ambiguous or old shape. | Type/version/policy/domain checks before object handling. | Descriptor with wrong type/version/policy. | object decrypted or acked | accept/ack on malformed descriptor | old disabled mode unaffected | descriptor is encrypted demo metadata |
| malformed ciphertext object | Padded object could be corrupt or ambiguous. | Require selected class length, padding length, zero padding, unpadded hash. | Nonzero padding, wrong class, hash mismatch. | actor decrypt before strip/verify | actor called or output written before verify | preserves fail-closed reject | no hidden-size claim |
| invalid size-class config | Silent fallback weakens proof. | Reject empty, zero, nonnumeric, duplicate, unsorted, too-large configs. | Config/env matrix. | fallback to unpadded send | invalid config accepted or fallback silent | disabled explicit path still works | config proof only |
| repeated invalid attachment attempts | Attacker can create local work/retry cadence. | Inherit NA-0327 retry cap/ledger and artifact-safety requirements. | Repeat invalid descriptor/object receives. | unbounded loop or leak | cap exceeded or candidate deleted early | retry policy remains qshield demo only | timing still observable |
| attachment retention/purge failure | Bad reject path can delete before local verify or retain unexpectedly. | `/poll-candidate` fetch only; ack both candidates only after verification boundary. | Invalid object remains queued; valid path removes exactly pair. | early ack or wrong delete | any delete before local verification | preserves candidate/ack model | not service retention proof |
| backup growth | Padded objects and artifacts can increase temp/log size. | Runtime test artifacts under temp/qbuild only; no new durable root; cap object at 8192. | Artifact-size and no-secret scan. | unbounded artifact growth | artifact exceeds cap or contains secrets | no backup plan update for tracked docs | no production backup proof |
| local demo stress | Bounded local abuse may still amplify relay queue. | Existing queue caps plus 8192 object cap and invalid retry bounds. | Baseline demo stress plus attachment class harness. | queue cap bypass or panic | stress failure with this root cause | local stress only | not production readiness |
| cover prototype plus attachment size-class | Cover/user confusion or cost overrun. | Cover remains tagged; attachment class proof does not generate cover. | Run cover coexistence harness plus attachment class. | cover delivered as user attachment | cover/user boundary broken | cover remains opt-in | cover does not prove concealment |
| batching/jitter/retry interactions | Existing mitigations could be overclaimed. | Batching/jitter/retry harnesses remain green; attachment send does not silently batch. | Run adjacent harnesses under toggles. | cap drift or partial mutation | adjacent harness regression | no broad scheduling change | timing/shape gaps remain visible |
| qsl-attachments production equivalent | Service stores immutable committed objects and exposes object size/timing differently. | Future cross-repo authorization required. | Future qsl-attachments lane only. | qshield proof presented as service proof | any qsl-attachments path change or production claim | none in NA-0339 | future-gated |
| public-internet object-size observation | Network observer sees TLS/IP/timing/size behavior outside local demo. | Future public/deployment evidence required. | Future production/public lane only. | public readiness implied | any public-internet claim | none | future-gated |

## Future Validation / Marker Plan

Future NA-0339 marker candidates:

- `NA0339_ATTACHMENT_SIZE_CLASS_AUTHORIZATION_OK`
- `NA0339_ATTACHMENT_SIZE_CLASS_POLICY_OK`
- `NA0339_DETERMINISTIC_TEST_ATTACHMENT_SIZE_CLASS_OK`
- `NA0339_VALID_SMALL_ATTACHMENT_OK`
- `NA0339_VALID_MEDIUM_ATTACHMENT_OK`
- `NA0339_VALID_LARGE_ATTACHMENT_OK`
- `NA0339_ATTACHMENT_MAX_OVERHEAD_BOUNDARY_OK`
- `NA0339_ATTACHMENT_INVALID_CONFIG_REJECT_OK`
- `NA0339_ATTACHMENT_OVERSIZE_REJECT_OK`
- `NA0339_ATTACHMENT_MALFORMED_REJECT_OK`
- `NA0339_ATTACHMENT_RETENTION_PURGE_BOUNDARY_OK`
- `NA0339_ATTACHMENT_BACKUP_BOUNDARY_OK`
- `NA0339_ATTACHMENT_NO_SECRET_ARTIFACT_OK`
- `NA0339_PADDING_COVER_BATCHING_RETRY_JITTER_STILL_BOUNDED_OK`
- `NA0339_QSHIELD_DEMO_BOUNDARY_OK`
- `NA0339_QSL_ATTACHMENTS_PRODUCTION_BOUNDARY_OK`
- `NA0339_NO_METADATA_FREE_CLAIM_OK`
- `NA0339_NO_SIZE_HIDDEN_CLAIM_OK`
- `NA0339_NO_TIMING_HIDDEN_CLAIM_OK`
- `NA0339_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK`

If the future implementation discovers an exact blocker, it must emit blocker
evidence instead of weakening the policy.

## Production Boundary

Future qshield demo attachment size-class evidence would remain local/demo only.

It would not prove:

- qsl-server production padding, timing, retention, deployment, logging, or
  public-internet behavior;
- qsl-attachments production object-size padding, object transformation,
  retention, purge, backup, quota, deployment, logging, observability, or
  public-internet behavior;
- qsc production attachment streaming behavior;
- service production readiness;
- public internet readiness;
- external review completion.

qsl-attachments production object-size padding would require a separate
cross-repo authorization because DOC-CAN-006 currently requires retrieval to
return ciphertext bytes exactly as committed and treats ciphertext length,
part count, retention class, expiry, and upload/download timing as service
metadata. Any service-side object-size-class change must be designed and
tested in the service-owned repo with updated storage, retrieval, backup,
retention, quota, and logging evidence.

## External-Review Sensitivity

Attachment size-class behavior is privacy-sensitive because object size, timing,
queue shape, retention, and service access events are observable metadata.

External review is recommended before any stronger privacy claim. NA-0338 does
not claim external review completion and does not authorize public copy that
would imply such completion.

## Public Claim Boundary

Allowed wording:

- qshield may have a future bounded local/demo attachment size-class harness if
  NA-0339 implements and proves it.
- qsl-server and qsl-attachments production attachment/timing behavior remains
  future-gated and cross-repo scoped.
- attachment size-class work is a bounded metadata-minimization experiment, not
  a complete metadata solution.

Disallowed wording unless explicitly negated or listed as prohibited:

- attachment size is hidden;
- timing metadata is hidden;
- traffic shape is hidden;
- padding hides all metadata;
- metadata-free behavior;
- anonymity;
- untraceable behavior;
- production readiness;
- public internet readiness;
- external review complete;
- quantum-proof hype, unbreakable, military-grade, or guaranteed secure
  language.

## Selected Successor

Selected successor:

`NA-0339 -- Metadata Runtime qshield Demo Attachment Size-Class Implementation Harness`

Rationale:

- current qshield demo attachment behavior is already local/demo scoped and
  has descriptor/fetch/decrypt/integrity proof;
- current qshield candidate/ack behavior supports no-delete-before-verification
  proof;
- NA-0337 provides a deterministic bounded size-class table, cap, overhead, and
  invalid/malformed proof pattern that can be adapted to qshield demo
  attachment objects;
- production qsl-attachments object-size padding remains too broad for the next
  immediate lane and needs separate cross-repo authorization later;
- no exact blocker requires resolving qsl-attachments or qsl-server production
  semantics before a local qshield demo harness.

## Rejected Alternatives

- `NA-0339 -- Metadata Runtime Attachment Size-Class Blocker Resolution`:
  rejected because no exact blocker prevents a bounded qshield demo-only
  implementation harness.
- `NA-0339 -- Metadata Runtime qsl-attachments Production Size-Class
  Cross-Repo Authorization`: rejected as the immediate successor because the
  smaller next proof is local qshield demo attachment object size-class
  behavior; production object-size padding remains future-gated after that.
- `NA-0339 -- Metadata Runtime Service Timing Cross-Repo Authorization`:
  rejected as the immediate successor because NA-0338 is about attachment
  object size classes and the qshield demo attachment harness can proceed
  without service timing changes.
- Direct qsl-attachments production implementation: rejected because it needs
  cross-repo service authorization and would change production service
  semantics.
- Direct public/website claim changes: rejected because no stronger public,
  production, or review claim is proven.
- Claiming metadata-free behavior: rejected because size classes do not remove
  all metadata.

## Backup-Plan Impact Statement

No backup-plan update is required for NA-0338.

NA-0338 changes only tracked qsl-protocol governance/evidence/testplan files
under `/srv/qbuild/work`, which is already within the local backup scope. It
creates no new durable evidence root, response root, source root, excluded
backup path, or non-rebuildable artifact outside current qbuild storage.

Future NA-0339 should keep runtime artifacts under bounded temp/qbuild paths and
must stop if it needs a new durable evidence location outside the current backup
scope.

## Next Recommendation

After NA-0338 merges and closes out, restore:

`NA-0339 -- Metadata Runtime qshield Demo Attachment Size-Class Implementation Harness`

The future directive should implement only the bounded qshield demo attachment
size-class harness or stop on an exact prerequisite. It must not implement or
claim qsl-server production behavior, qsl-attachments production behavior,
public-internet behavior, external-review completion, anonymity,
metadata-free behavior, untraceable behavior, or that attachment size, timing,
traffic shape, or all metadata is hidden.
