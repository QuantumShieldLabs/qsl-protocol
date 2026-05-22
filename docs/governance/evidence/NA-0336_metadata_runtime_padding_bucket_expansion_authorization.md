Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-22

# NA-0336 Metadata Runtime Padding Bucket Expansion Authorization

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0336 authorizes one future bounded qshield embedded relay/demo padding bucket
expansion implementation harness as:

`NA-0337 -- Metadata Runtime qshield Demo Padding Bucket Expansion Implementation Harness`

The authorization is limited to local qshield demo evidence. It builds on
NA-0319 default-padding proof and NA-0335 cover-traffic prototype proof, and it
does not implement padding bucket expansion in this lane.

The future lane may prove an expanded deterministic qshield demo bucket table,
maximum overhead bounds, invalid configuration rejection, valid strip/verify,
malformed padding rejection, no remote delete before local verification, no
accepted state/output on reject, and secret-safe artifacts. The future lane
must stop if those proofs require production service behavior, qsl-server or
qsl-attachments changes, protocol/crypto changes, dependency changes, workflow
changes, public-copy changes, or stronger privacy/readiness claims.

This evidence does not claim anonymity, metadata-free behavior, untraceable
behavior, public-internet readiness, production readiness, external review
completion, or that timing metadata or traffic shape is hidden.

## Live NA-0336 Scope

Live `NEXT_ACTIONS.md` entry:

- `NA-0336 -- Metadata Runtime Padding Bucket Expansion Authorization Plan`
- Status: READY
- Goals: G1, G2, G3, G4, G5
- Objective: execute the next metadata-runtime timing/traffic-shape lane
  selected by NA-0335: an authorization plan for future padding bucket
  expansion, or stop on an exact prerequisite.

Allowed work used by this lane:

- review NA-0319 identifier/default-padding evidence and later metadata runtime
  timing/traffic-shape evidence;
- define semantic requirements for future padding bucket expansion;
- define cap, compatibility, fallback, invalid-config, strip/verify, artifact,
  and claim boundaries;
- define qshield demo versus qsl-server/qsl-attachments production boundaries;
- select one exact NA-0337 successor;
- update governance evidence, testplan, decisions, traceability, and the
  rolling operations journal.

Forbidden work preserved:

- no padding bucket expansion implementation in NA-0336;
- no runtime transport padding change;
- no qshield runtime change;
- no qsl-server or qsl-attachments change;
- no qsc/qsp/protocol/crypto/key-schedule change;
- no dependency, Cargo, workflow, branch-protection, public-safety, website,
  README, START_HERE, docs/public, qsc-desktop, formal, input, tools/refimpl,
  app runtime, or production-service change;
- no unsupported production, public-internet, external-review, anonymity,
  metadata-free, or untraceable claim;
- no claim that timing metadata or traffic shape is hidden.

The live scope matches the directive and supports this authorization-only
evidence patch.

## Inherited NA-0335 Cover Prototype Proof

NA-0335 D-0652 implemented only opt-in bounded qshield embedded relay/demo
cover traffic:

- policy `qshield_demo_cover_traffic_v1`;
- opt-in gate `QSHIELD_DEMO_COVER_TRAFFIC=1`;
- deterministic test mode `QSHIELD_DEMO_COVER_TRAFFIC_TEST_MODE=1`;
- synthetic local cover, active-session cover, and batch-fill cover only;
- inherited NA-0333 caps before generation;
- disk-floor fail-closed behavior;
- in-memory retained artifact summaries capped at four and 1 MiB;
- purge, real-message priority, no recursive cover generation, batching,
  retry-cadence, and bounded-jitter preservation;
- receive-side cover skip/ack so cover is not delivered as user plaintext;
- `COVER_ARTIFACT_SECRET_FINDING_COUNT 0`.

NA-0335 explicitly rejected or deferred fixed-rate cover, qsl-server production
relay cover, qsl-attachments production object cover, public-internet cover,
transport padding expansion, and stronger privacy/readiness claims.

## Inherited NA-0319 Default-Padding Proof

NA-0319 D-0617 implemented bounded qshield embedded relay/demo proof for
identifier/default-padding behavior:

- current bounded default-padding profile:
  `metadata-runtime-default-padding-v1`;
- bucket table proven by harness: `[512, 1024, 2048, 4096, 8192]`;
- send-side qshield padding chooses the first configured bucket that can hold
  the candidate ciphertext;
- receive-side strip/verify checks bucket length, pad length, and zero padding
  before actor decode;
- invalid padding configuration rejects before config creation;
- malformed padded candidates with non-zero padding reject before actor decode;
- invalid padding rejects do not post `/ack` and therefore do not delete the
  remote candidate from the qshield embedded relay candidate queue;
- invalid rejects create no accepted local state or accepted plaintext output;
- reject output is scanned for route/token, ack id, peer-handle, plaintext,
  padding, panic, and backtrace sentinels.

Important limitation: this proof is bounded qshield demo evidence. It is not
global production default padding, qsl-server production relay proof,
qsl-attachments object-size proof, timing/traffic-shape proof, or a claim that
all metadata is removed.

## Sources Inspected

- `NEXT_ACTIONS.md` live NA-0336 entry.
- `tests/NA-0335_closeout_restore_na0336_testplan.md`.
- `docs/governance/evidence/NA-0335_metadata_runtime_qshield_demo_cover_traffic_prototype_harness.md`.
- `tests/NA-0335_metadata_runtime_qshield_demo_cover_traffic_prototype_harness_testplan.md`.
- `docs/governance/evidence/NA-0334_metadata_runtime_qshield_demo_cover_traffic_prototype_authorization.md`.
- `docs/governance/evidence/NA-0333_metadata_runtime_cover_traffic_cost_quota_retention_prerequisite_plan.md`.
- `docs/governance/evidence/NA-0319_metadata_runtime_identifier_default_padding_harness.md`.
- `tests/NA-0319_metadata_runtime_identifier_default_padding_harness_testplan.md`.
- `docs/governance/evidence/NA-0314_metadata_runtime_identifier_padding_transition_plan.md`.
- `docs/governance/evidence/NA-0291_metadata_phase2_identifier_padding_harness.md`.
- `docs/governance/evidence/NA-0293_metadata_phase2_sanitized_errors_retention_harness.md`.
- `inputs/metadata_runtime/identifier_padding_runtime_fixture_v1.json`.
- `scripts/ci/metadata_runtime_identifier_padding_harness_plan.sh`.
- `scripts/ci/metadata_phase2_identifier_padding_harness.sh`.
- `scripts/ci/metadata_phase2_sanitized_errors_retention_harness.sh`.
- `scripts/ci/metadata_conformance_smoke.sh`.
- `apps/qshield-cli/src/commands/send.rs`.
- `apps/qshield-cli/src/commands/recv.rs`.
- `apps/qshield-cli/src/commands/relay.rs`.
- `apps/qshield-cli/src/commands/init.rs`.
- `apps/qshield-cli/src/config.rs`.
- `apps/qshield-cli/src/relay_client.rs`.
- `apps/qshield-cli/tests/na_0319_metadata_runtime_identifier_padding.rs`.
- `apps/qshield-cli/tests/na_0335_metadata_runtime_cover_traffic_prototype.rs`.
- `TRACEABILITY.md`.
- `DECISIONS.md`.

Search coverage included padding, padding bucket, bucket, default padding,
strip verify, malformed padding, invalid padding, max overhead, size class,
size distribution, transport padding, attachment size, qshield demo,
qsl-server, qsl-attachments, production, metadata-free, anonymity,
untraceable, timing hidden, traffic hidden, `FUTURE_GATE`, and `NOT_READY`.

## Current Padding Behavior

Current qshield demo padding is configuration-driven:

- `qshield init --padding-enable --padding-buckets <csv>` stores padding
  configuration in `config.json`;
- invalid non-numeric or zero buckets reject at init;
- bucket values are sorted and deduplicated;
- send pads ciphertext bytes with zeros to the first configured bucket that is
  at least the ciphertext length;
- send rejects if padding is enabled but no bucket can hold the ciphertext;
- `/send` and `/send-batch` reject invalid padding metadata when the hex wire
  length does not match the declared bucket or when `pad_len > bucket`;
- `qshield recv` decodes candidate hex, checks `pad_len`, checks bucket length
  when present, verifies stripped padding bytes are zero, truncates the padding,
  and only then calls actor receive;
- `qshield recv` posts `/ack` or `/ack-batch` only after local verification;
- cover candidates are tagged and acked without actor delivery, preserving
  NA-0335 cover/user-message separation.

The current profile is enough to authorize a future bounded qshield demo
expansion harness, provided the future lane keeps explicit caps and stops
before any production or cross-repo semantics.

## Padding Bucket Expansion Problem Statement

The current NA-0319 bucket table proves one bounded profile. It does not answer
whether a richer size-class table can be used safely in a qshield demo lane
without excessive overhead, compatibility drift, invalid-config ambiguity, or
privacy overclaim.

The future lane must answer only local/demo implementation questions:

- can a larger deterministic qshield demo bucket table be configured and
  exercised without changing protocol/crypto semantics;
- can max overhead and max padded payload size be bounded and tested;
- can invalid configs and malformed padded payloads fail closed before accepted
  state/output;
- can valid padded payloads still strip/verify and ack only after local
  verification;
- can batching, retry-cadence, bounded jitter, cover traffic, sanitized-error,
  retention/purge, and artifact-safety evidence remain intact.

## Padding Bucket Expansion Semantic Design

Authorized future qshield demo policy name:

`qshield_demo_padding_bucket_expansion_v1`

Future deterministic demo bucket table:

`[256, 512, 768, 1024, 1536, 2048, 3072, 4096, 5120, 6144, 7168, 8192]`

This table is intentionally capped at the existing largest NA-0319 and NA-0333
payload boundary of 8192 bytes. It adds intermediate buckets so bounded demo
tests can distinguish small, medium, and large size classes while keeping the
largest padded payload unchanged.

Required semantics for the future lane:

- choose the smallest bucket that can contain the ciphertext;
- reject payloads larger than the largest allowed demo bucket;
- define `max_padded_payload_size = 8192` bytes for qshield demo proof unless
  a future directive narrows it;
- define `max_overhead = 1023` bytes for payload lengths in the 1..8192 range
  covered by the proposed table, because the largest adjacent gap above the
  first bucket is 1024 and representative tests must include exact boundaries
  and boundary-plus-one cases; if the future implementation computes a
  different worst case from ciphertext framing, it must report that value and
  stop unless it remains bounded by the future directive;
- reject invalid bucket config before config creation or before accepting
  runtime work;
- require buckets to be strictly positive, sorted after parsing, unique after
  normalization, bounded by `8192`, and compatible with the maximum overhead
  requirement;
- reject missing, malformed, mismatched, or non-zero stripped padding before
  actor decode and before accepted plaintext output;
- preserve `/poll-candidate` plus `/ack` semantics: no remote delete before
  local verify, and ack only after valid strip/verify and actor receive;
- preserve cover candidate handling: cover remains tagged, skipped as user
  plaintext, and acked only through the cover path;
- keep deterministic test mode local and finite;
- keep artifacts/logs secret-free and bounded.

Compatibility requirements:

- existing no-padding and NA-0319 default-padding behavior must continue to
  work;
- single-message and batch send/receive must remain compatible;
- valid current buckets remain accepted when explicitly configured;
- invalid config must fail closed rather than silently falling back to a weaker
  or broader profile.

Claim boundary:

- a richer bucket table can reduce some size-class observability inside a
  bounded local demo profile;
- it does not remove all metadata;
- it does not prove public-internet, production-service, qsl-server, or
  qsl-attachments behavior;
- timing metadata, traffic shape, contact graph, IP-level metadata, queue
  depth, retry cadence, and cover traffic cost remain visible risk areas.

## Future Implementation Boundary

NA-0337 may proceed only if its directive keeps the future work inside the
qshield embedded relay/demo boundary and includes exact tests for every marker
below.

Exact future qshield implementation/harness files authorized for consideration
by NA-0337:

- `apps/qshield-cli/src/commands/init.rs` for bucket profile/config validation
  only if existing config parsing is insufficient;
- `apps/qshield-cli/src/config.rs` only if a named demo padding profile must be
  represented in config without changing protocol semantics;
- `apps/qshield-cli/src/commands/send.rs` for smallest-bucket selection and
  max-overhead/max-size enforcement;
- `apps/qshield-cli/src/commands/recv.rs` for strip/verify and no-output
  reject behavior;
- `apps/qshield-cli/src/commands/relay.rs` only if current padding metadata
  validation must be narrowed to preserve the new demo boundary;
- `apps/qshield-cli/src/relay_client.rs` only if bounded demo padding metadata
  serialization needs an existing-field compatibility adjustment;
- `apps/qshield-cli/tests/na_0337_metadata_runtime_padding_bucket_expansion.rs`;
- established qshield test helper code already used by the qshield metadata
  runtime harnesses.

Future fixture/script paths may be added only if the future directive permits:

- `inputs/metadata_runtime/padding_bucket_expansion_fixture_v1.json`;
- `scripts/ci/metadata_runtime_padding_bucket_expansion_harness.sh`.

Future-gated, not authorized for NA-0337 without new explicit scope:

- `apps/qshield-cli/src/commands/attachment.rs`;
- qsl-server production relay paths;
- qsl-attachments production object/storage paths;
- qsc/qsp/protocol/crypto/key schedule paths;
- Cargo manifests and lockfiles;
- workflows, branch-protection, public-safety configuration;
- website, README, START_HERE, docs/public, qsc-desktop, formal, tools/refimpl,
  external website, or production service paths.

If NA-0337 needs any future-gated file to pass honestly, it must stop and
select a blocker or cross-repo authorization successor.

## Abuse / Cost / Latency / Compatibility Matrix

| Scenario | Risk | Proposed bound | Future test | Failure mode | Stop condition | Compatibility impact | Claim boundary |
| --- | --- | --- | --- | --- | --- | --- | --- |
| valid small message | over-padding tiny messages | bucket to 256 or 512 | exact small payload round trip | reject if strip/verify fails | bucket choice exceeds max overhead without authorization | no-padding and existing padding still pass | local demo size class only |
| valid medium message | bucket transition bug | buckets 768 through 2048 | boundary and boundary-plus-one cases | reject before ack on mismatch | accepted output before verify | existing send/recv semantics preserved | no traffic-shape proof |
| valid large message within demo cap | resource growth | max padded payload 8192 | 4096, 6144, 8192 cases | reject if no bucket or bad zero pad | any payload above cap accepted | largest existing demo cap unchanged | no production object proof |
| oversized payload | queue/resource amplification | reject above 8192 padded bytes | payload just over cap | fail closed before send/queue | oversized accepted or silently truncated | no fallback to unpadded send | no readiness claim |
| invalid padding bytes | accepted corrupt payload | zero padding required | non-zero stripped bytes | reject before actor decode | actor decode or output occurs | current NA-0319 reject preserved | no metadata-free claim |
| missing padding metadata | ambiguity | unpadded allowed only when pad_len absent/zero | missing bucket with pad_len > 0 | reject invalid metadata | ambiguous padded payload accepted | legacy unpadded compatibility preserved | explicit metadata remains |
| malformed padding metadata | parsing confusion | numeric bounded fields only | bad types and mismatched sizes | reject at relay or recv | panic, raw secret, accepted state | existing coarse errors preserved | no hidden gap |
| invalid bucket config | weak profile fallback | reject before config creation/work | zero, nonnumeric, unordered duplicates, over cap | fail closed | silent fallback or config write | no existing valid config regression | not a global default claim |
| repeated invalid padded messages | retry/DoS | inherit NA-0327 bounded retry cadence | repeated same candidate reject | bounded reject | unbounded attempts or remote delete | retry ledger remains bounded | timing still visible |
| batching plus padding | partial batch mutation | all members valid before mutation | mixed valid/invalid batch | reject batch before accepted output | partial ack or output | NA-0331 batch semantics preserved | no production batching proof |
| jitter/retry plus padding | latency surprises | inherit NA-0327/NA-0329 caps | invalid padded candidate under toggles | bounded composed delay | cap exceeded | existing toggles remain opt-in | not public-internet evidence |
| cover prototype plus padding | cover/user confusion | cover remains tagged and skipped | cover plus padded real messages | cover ack path only | cover delivered as plaintext | NA-0335 priority preserved | cover does not prove concealment |
| attachment-size padding | object-size overclaim | future-gated | blocker marker only | stop | attachment file touched | no qsl-attachments drift | production object size unproven |
| qsl-server production padding | cross-repo drift | future-gated | blocker marker only | stop | qsl-server file touched | no service mutation | production remains unproven |
| public-internet traffic observation | external metadata overclaim | future-gated | blocker marker only | stop | public-internet claim added | local-only evidence | timing/shape not claimed hidden |

## Future Validation / Marker Plan

Required NA-0337 marker candidates if implementation is authorized:

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

If implementation is not safe, the successor must become:

`NA-0337 -- Metadata Runtime Padding Bucket Expansion Blocker Resolution`

Minimum blocker markers:

- `NA0337_PADDING_BUCKET_BLOCKER_FOUND`
- `NA0337_PADDING_BUCKET_BLOCKER_SCOPE_OK`
- `NA0337_PADDING_BUCKET_NO_RUNTIME_DRIFT_OK`
- `NA0337_PADDING_BUCKET_NO_OVERCLAIM_OK`

## Production Boundary

qshield embedded relay/demo padding bucket expansion is local/demo only.

Production boundaries:

- qsl-server production padding requires separate cross-repo authorization and
  production-service evidence;
- qsl-attachments production object-size padding requires separate cross-repo
  authorization and object-store/resource evidence;
- public-internet traffic observation remains future-gated;
- attachment-size padding remains future-gated unless a later exact directive
  authorizes the attachment demo surface;
- external review is recommended before any stronger privacy claim.

## External-Review Sensitivity

Padding bucket expansion touches metadata reduction claims and therefore must
remain conservative. A future qshield demo harness may prove local bounded
behavior, but it must not imply external review completion or third-party
validation. External review remains a separate evidence lane.

## Public Claim Boundary

Allowed wording:

- bounded qshield demo padding bucket expansion evidence;
- local/demo size-class reduction evidence;
- explicit max-overhead, invalid-config, strip/verify, and reject boundaries;
- production and public-internet gaps remain open.

Forbidden wording:

- anonymity or anonymous messaging;
- metadata-free behavior;
- untraceable behavior;
- production readiness or public-internet readiness;
- external review completion;
- that timing metadata is hidden;
- that traffic shape is hidden;
- that padding removes all metadata.

## Selected Successor

Selected successor:

`NA-0337 -- Metadata Runtime qshield Demo Padding Bucket Expansion Implementation Harness`

Rationale:

- NA-0319 proved bounded qshield default padding and the required ack/commit
  no-delete boundary;
- NA-0335 proved bounded qshield demo cover traffic coexists with batching,
  retry cadence, bounded jitter, artifact safety, and service-production
  boundaries;
- no blocker was found that prevents a narrow qshield demo implementation
  harness from proving an expanded bucket table;
- production service timing and attachment object-size behavior remain too
  broad for the next immediate lane.

## Rejected Alternatives

- `NA-0337 -- Metadata Runtime Padding Bucket Expansion Blocker Resolution`:
  rejected because no exact prerequisite blocker was found for a bounded
  qshield demo harness.
- `NA-0337 -- Metadata Runtime Service Timing Cross-Repo Authorization`:
  rejected because the next safest lane is local qshield demo padding proof
  before service timing authorization.
- `NA-0337 -- Metadata Runtime Attachment Size-Class Authorization Plan`:
  rejected because attachment object-size padding remains future-gated behind
  separate qshield attachment and qsl-attachments production analysis.
- Direct production padding: rejected because qsl-server and qsl-attachments
  production behavior require cross-repo authorization.
- Claiming metadata-free behavior: rejected because padding bucket expansion
  does not remove all metadata.

## Backup-Plan Impact Statement

No backup-plan update is required. This lane changes only tracked
qsl-protocol governance/evidence/testplan files under `/srv/qbuild/work`.
No durable evidence location outside the existing qbuild worktree or Codex
response archive is created. Future NA-0337 artifacts must remain tracked,
temporary, or explicitly covered by a later backup-impact statement.

## Next Recommendation

After NA-0336 merges and post-merge `public-safety` is green, close out
NA-0336 and restore exactly one READY item:

`NA-0337 -- Metadata Runtime qshield Demo Padding Bucket Expansion Implementation Harness`
