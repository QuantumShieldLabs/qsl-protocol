Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-15
Replaces: n/a
Superseded-By: n/a

# NA-0292 Metadata Phase-2 Sanitized Errors and Retention/Purge Design

## Executive Summary

NA-0292 defines the next metadata phase-2 design path for sanitized-error
expansion and retention/purge metadata policy. It connects the current demo,
qsl-server, qsl-attachments, metadata smoke, and NA-0291 identifier/padding
evidence into a single claim-safe plan for a later executable harness.

This lane is design/governance only. It does not implement sanitized-error
runtime behavior, retention or purge runtime behavior, qsp protocol-core
behavior, cryptographic state-machine behavior, qsl-server behavior,
qsl-attachments behavior, qsc-desktop behavior, website copy, README copy,
START_HERE copy, workflows, scripts, Cargo files, dependencies, branch
protection, or public-safety configuration.

Metadata phase-2 remains evidence-bound and incomplete. This document does not
claim anonymity, metadata-free messaging, untraceability, external review
completion, production readiness, or public internet readiness.

## Scope and Non-Goals

In scope:

- inventory current sanitized-error and retention/purge evidence;
- classify current surfaces as `PROVEN_EXECUTABLE`, `DOCS_ONLY`,
  `NOT_READY`, `FUTURE_GATE`, or `OUT_OF_SCOPE`;
- define sanitized-error taxonomy, allowed fields, and forbidden fields;
- define retention/purge metadata policy boundaries for protocol, demo,
  service, and evidence artifacts;
- design the NA-0293 executable harness successor.

Non-goals:

- no protocol, wire, crypto, auth, negotiation, or state-machine change;
- no qsp protocol-core, qsc/qsl runtime, qshield demo implementation,
  qsl-server implementation, qsl-attachments implementation, qsc-desktop,
  website, README, START_HERE, workflow, script, Cargo, lockfile, dependency,
  branch-protection, or public-safety configuration change;
- no public-copy implementation;
- no production, public internet, external-review-complete, anonymity,
  metadata-free, or untraceable claim;
- no claim that sanitized-error policy or retention/purge policy is
  implemented by NA-0292.

## Classification Vocabulary

- `PROVEN_EXECUTABLE`: local or CI-backed executable proof exists for the
  bounded behavior.
- `DOCS_ONLY`: the behavior or boundary is documented, but stronger behavior
  is not directly proven.
- `NOT_READY`: current evidence does not support the claim.
- `FUTURE_GATE`: a future lane must add proof before the claim can change.
- `OUT_OF_SCOPE`: the claim or mitigation is outside this lane.

## Current Sanitized-Error Baseline

| Surface | Current evidence | Classification | Boundary |
| --- | --- | --- | --- |
| Metadata conformance smoke | `scripts/ci/metadata_conformance_smoke.sh`, NA-0244 evidence | `PROVEN_EXECUTABLE` for selected demo rejects | Covers malformed JSON, content type, auth-scheme, padding metadata, padding config, duplicate/invalid id status checks, and no relay-token/sentinel echo for the checked bodies. |
| Demo CLI smoke | `scripts/ci/demo_cli_smoke.sh` | `PROVEN_EXECUTABLE` for bounded demo rejects | Covers selected missing auth, malformed register, invalid relay id, replay, attachment-integrity, and KT-negative paths. |
| NA-0291 identifier/padding harness | `scripts/ci/metadata_phase2_identifier_padding_harness.sh` and NA-0291 evidence | `PROVEN_EXECUTABLE` for policy fixtures | Proves deterministic reject/no-mutation for invalid identifiers and invalid padding fixture inputs; not runtime error-normalization. |
| Refimpl reason-code formatting | `tools/refimpl/quantumshield_refimpl/src/refimpl_error.rs` | `PROVEN_EXECUTABLE` for refimpl display tests | Stable `reason_code` display is refimpl-local and does not normalize every runtime/service error surface. |
| qsl-server auth/reject/logging | NA-0273, NA-0280, NA-0281, NA-0287 evidence; qsl-server sibling worktree | `PROVEN_EXECUTABLE` for local service hardening | Service-local deterministic errors and secret-safe logs do not prove qsl-protocol metadata phase-2 completion or production operation. |
| qsl-attachments reject taxonomy/capability logging | NA-0274, NA-0284, NA-0287 evidence; qsl-attachments sibling worktree | `PROVEN_EXECUTABLE` for local service hardening | Canonical `reason_code`, capability rejects, and no-secret logs are service-local. |
| DOC-G5 logging contract | DOC-G5-004 | `DOCS_ONLY` with linked executable support | Defines secret-bearing values and log fields, but does not itself run a cross-surface harness. |
| Broader cross-surface error normalization | NA-0288/NA-0290 gap language | `NOT_READY` / `FUTURE_GATE` | No current lane proves every demo/service/public-review reject family uses the same sanitized taxonomy. |
| Public claim about complete sanitized-error policy | none | `NOT_READY` | Must not be claimed until an executable lane proves the scoped surfaces. |

### Existing Executable Sanitized-Error Evidence

Current executable evidence proves selected cases only:

- malformed JSON and unsupported content type on the qshield demo relay;
- wrong/missing relay auth shapes in selected demo paths;
- invalid padding metadata and invalid padding bucket configuration;
- invalid demo identifiers in metadata smoke;
- identifier and padding fixture rejects in the NA-0291 policy harness;
- qsl-server auth, route-token, rate, route-cap, queue, body-size, and TTL
  rejects in local service tests;
- qsl-attachments malformed JSON, capability, expired, deleted/aborted,
  quota, disk, and backup/recovery rejects in local service tests.

### Existing Executable Reject / No-Mutation Evidence

Current no-mutation evidence is strong but bounded:

- metadata smoke proves selected demo rejects do not remove existing bundles,
  drain replay state incorrectly, or leak checked sentinels;
- NA-0291 proves invalid identifier inputs do not mutate the accepted
  identifier-state snapshot;
- qsl-server local tests prove auth/route/body/rate/queue/TTL rejects do not
  unexpectedly mutate queues or resurrect stale messages;
- qsl-attachments local tests prove malformed, capability, expired, aborted,
  partial-restore, and wrong-resource rejects do not create or resurrect
  recoverable session/object state.

### Service Reason-Code Evidence Related But Service-Scoped

qsl-server uses deterministic status/code bodies such as auth, route-token,
body-size, overload, route-cap, rate-limit, and TTL-related outcomes. Those
codes are service-local operational evidence.

qsl-attachments uses canonical `reason_code` values for malformed JSON,
session state, resume-token, fetch-capability, expired, locator, quota, disk,
and recovery outcomes. Those codes are service-local attachment evidence.

Neither service evidence should be reworded as metadata phase-2 completion,
production service readiness, or anonymity.

### Current Safe Error Examples

Safe examples are coarse, deterministic, and do not echo raw secrets:

- qshield demo malformed JSON returns a sanitized invalid-JSON body in the
  checked fixture;
- qshield demo auth failure returns a coarse missing/invalid relay-token body
  without the token value;
- qshield invalid padding metadata returns a coarse invalid-padding metadata
  body;
- qshield invalid padding config returns a coarse invalid-padding bucket body;
- qsl-server auth failure returns a deterministic unauthorized code and keeps
  route tokens, bearer values, and payload sentinels out of captured logs;
- qsl-attachments malformed JSON maps to a bounded service reject body and
  canonical `reason_code` rather than echoing request content.

### Unknown / Error-Risk Areas

The following remain unproven or only partly proven:

- all qshield CLI stderr paths for config, relay availability, and malformed
  local store states;
- raw send/receive ciphertext-shape errors outside the selected padding cases;
- every qshield public demo artifact/log created by stress, soak, and
  cross-host lanes;
- refimpl/qsp internal errors that still include precise internal state names;
- qsl-server proxy/access-log, metrics-label, and deployment-support bundles;
- qsl-attachments proxy/access-log, audit-retention, backup artifact, and
  support-bundle metadata;
- timing/order differences between reject families;
- panic/backtrace absence across every future error family.

## Current Retention/Purge Metadata Baseline

| Surface | Current evidence | Classification | Boundary |
| --- | --- | --- | --- |
| qsl-server route TTL / lifecycle | NA-0281 evidence and qsl-server PR #54 | `PROVEN_EXECUTABLE` for local service behavior | Access-triggered TTL removes expired route state, discards queued messages, releases capacity/rate accounting, and logs redacted route identifiers plus bounded counts. |
| qsl-server rate/global caps | NA-0280 evidence | `PROVEN_EXECUTABLE` for local service behavior | Route-cap, rate-limit, and queue-overload rejects are local in-memory service proof, not public metadata phase-2 policy. |
| qsl-attachments retention/cleanup/recovery | NA-0282 evidence and qsl-attachments PR #33 | `PROVEN_EXECUTABLE` for local service behavior | Request-path cleanup removes expired bytes/capability hashes and preserves coherent unexpired state. |
| qsl-attachments capability/delete behavior | NA-0284 evidence | `PROVEN_EXECUTABLE` for local service behavior | Abort/delete semantics are session-scoped; object delete is not introduced. |
| qsl-attachments backup/restore recovery | NA-0286 evidence and qsl-attachments PR #36 | `PROVEN_EXECUTABLE` for local tempdir full-root restore and fail-closed partial restore | Cold/quiesced full-root local recovery is proven; production backup automation, hot backup, partial restore support, and cross-node recovery are not. |
| Metadata phase-2 retention/purge policy | NA-0288 and release map | `NOT_READY` / `FUTURE_GATE` | Service-local retention evidence does not define a cross-surface qsl-protocol metadata policy. |
| Demo artifact retention | demo evidence docs and qbuild artifact notes | `DOCS_ONLY` | Artifact retention is operational evidence handling, not runtime purge policy. |
| Production retention policy | NA-0287 service production-gate map | `NOT_READY` | Requires exact deployment profile, logs, metrics, support bundles, backup/restore, and runbook proof. |

### Service Evidence Relationship

qsl-server evidence proves local route-slot and queue lifecycle behavior under
bounded tests. It helps define what future metadata phase-2 retention tests
should avoid leaking, but it does not prove deployment logs, proxy metadata,
source-IP policy, long-running soak, or production retention policy.

qsl-attachments evidence proves local session/object cleanup, recovery, and
fail-closed unsupported restore shapes. It helps define retention/purge
metadata boundaries, but it does not prove production backup/restore readiness,
hot backup, partial restore support, multi-node recovery, or deployment support
bundle hygiene.

### Demo Evidence Relationship

Demo evidence proves non-production qshield flows and selected rejects. The
demo retains local stores, transcripts, and qbuild artifacts for evidence and
debugging. That operational evidence retention is separate from runtime
retention/purge policy and must be described as evidence handling, not privacy
or production deletion behavior.

### Metadata Phase-2 Gap Summary

Current metadata phase-2 retention/purge gaps:

- no executable qsl-protocol harness that combines sanitized errors with
  expired/deleted/purged fixture state;
- no cross-surface policy for what error bodies may reveal after deletion or
  expiry;
- no qsl-protocol fixture proving no resurrection after purge;
- no transcript/log scan proving purge paths avoid route tokens, capabilities,
  identifiers, descriptors, plaintext, ciphertext, panic text, and backtraces;
- no public metadata phase-2 policy stating retention/purge evidence is bounded
  and incomplete.

## Cross-Surface Metadata Risk Matrix

| Risk category | Current evidence | Remaining gap | Design control | Future executable test | Claim boundary |
| --- | --- | --- | --- | --- | --- |
| Error body leakage | Metadata smoke, NA-0244, qsl-attachments reject taxonomy | Not every reject family is covered | Allow coarse status, stable reason code, and optional short correlation handle only | Scan all fixture error bodies for forbidden values | Do not claim complete sanitized errors yet. |
| Log leakage | DOC-G5-004, qsl-server/qsl-attachments logging tests | Deployment/proxy/support logs remain unproven | Logs may include event class, reason code, coarse counts, and redacted handles only | Capture harness logs and scan for tokens/capabilities/descriptors/plaintext/ciphertext | Service-local logs are not production metadata proof. |
| Reason-code over-specificity | Refimpl and service reason-code evidence | Codes can reveal exact existence/state if too granular | Use stable but coarse taxonomy per surface | Assert invalid, unauthorized, expired/deleted, and unknown cases do not disclose secret-bearing state | Reason codes support debugging, not anonymity. |
| Route/contact/identifier leakage | NA-0290/NA-0291 and DOC-G5 inventories | Stable runtime ids remain visible | For future harnesses use opaque fixture handles and redacted ids | Invalid/stale/wrong-contact handle fixtures reject without raw id echo | Runtime rotation remains not implemented. |
| Capability/token leakage | Metadata smoke, qsl-server/qsl-attachments logging tests | All future artifacts must be scanned | Capabilities and tokens are forbidden fields | Inject fixture secrets and scan body/log/transcript output | Capabilities remain authz secrets, not public handles. |
| Timing/order leakage | NA-0288 and DOC-G5 residuals | No timing-analysis mitigation proof | Avoid timing claims; keep retry hints coarse | Future harness may assert deterministic ordering only where claimed | No traffic-analysis resistance claim. |
| Retention duration leakage | qsl-server TTL and qsl-attachments TTL evidence | Exact deletion windows can reveal state | Public errors should use coarse expired/deleted classes | Expired/deleted fixtures return coarse class and no exact internal age unless explicit test marker | Not production retention policy. |
| Purge/deletion state leakage | qsl-attachments delete/abort/recovery evidence | Cross-surface deleted-vs-never-existed policy is not proven | Avoid over-specific bodies that reveal object history unnecessarily | Deleted, expired, purged, and unknown fixture accesses produce allowed taxonomy | No metadata-free claim. |
| Backup/restore metadata leakage | NA-0286 local restore evidence | Production backup artifacts and support bundles unproven | Treat backups/support bundles as evidence artifacts needing leak scans | Restore fixture scans recovery summaries and logs | No production backup/restore readiness claim. |
| Rejected-state leakage | Demo/service no-mutation tests | Unified reject-state taxonomy is missing | Rejected inputs must not create state or reveal internal counters | Malformed/auth/invalid/purge rejects preserve pre-state snapshot | No complete policy claim until harness lands. |
| Panic/backtrace leakage | Existing no-panic checks in selected lanes | Full cross-surface panic scan is missing | Error paths must not expose panic, backtrace, unwrap, or internal stack text | Harness scans stdout/stderr/logs for panic/backtrace markers | No implementation claim in NA-0292. |
| Public claim overreach | Release map, external package, NA-0288/0290/0291 | Public docs need NA-0292 alignment | Keep `NOT_READY` and future-gated wording | Link/overclaim scan on docs changes | No production/external-review/anonymity claims. |

## Sanitized-Error Design

### Protected Values

Future sanitized-error work must protect:

- route tokens;
- relay bearer tokens;
- qsl-attachments resume tokens and fetch capabilities;
- stable identifiers, opaque handles, session ids, locator refs, and contact or
  peer metadata unless explicitly designated as safe fixture output;
- attachment descriptors, decrypt contexts, plaintext sentinels, ciphertext
  sentinels, and integrity material;
- internal state names, queue/session/object existence, and retry/timing hints
  when they would reveal more than the allowed taxonomy;
- panic text, backtraces, file paths, environment values, and secret-bearing
  operator paths.

### Allowed Error Fields

Allowed fields for future harness scope:

- coarse status class or HTTP status when the surface already exposes HTTP;
- stable `reason_code` drawn from an approved taxonomy;
- short request correlation handle that is generated for diagnostics and is not
  a raw route token, capability, session id, locator ref, peer id, or long
  stable identifier;
- deterministic test marker in executable harness output, never in runtime
  public error bodies unless the harness owns the surface;
- bounded retry class such as retryable/not-retryable, only if it does not
  reveal exact queue depth, purge age, or resource existence.

### Forbidden Error Fields

Forbidden fields:

- raw route tokens, relay bearer tokens, resume tokens, fetch capabilities, or
  future capability-like secrets;
- raw stable identifiers, raw opaque handles, raw descriptors, decrypt context,
  plaintext, ciphertext, or integrity sentinels;
- exact internal path or file names when they identify secret-bearing stores or
  host-specific evidence paths;
- sensitive state counts, exact queue depth, exact purge age, exact object
  existence history, or retry timing that is not explicitly approved;
- panic, backtrace, unwrap, debug dump, stack trace, environment, or secret
  manager output;
- public wording that upgrades planned work into implemented behavior.

### Sanitized Error Taxonomy

Recommended coarse taxonomy for NA-0293 fixture proof:

| Class | Meaning | Example allowed reason-code shape | Notes |
| --- | --- | --- | --- |
| `MALFORMED` | Parse/schema/content-type/shape failure | `REJECT_METADATA_MALFORMED` | Must not echo rejected body. |
| `UNAUTHORIZED` | Missing/wrong auth or capability | `REJECT_METADATA_UNAUTHORIZED` | Must not distinguish raw secret value. |
| `INVALID_IDENTIFIER` | Bad/stale/wrong handle or identifier | `REJECT_METADATA_IDENTIFIER` | Must not echo raw identifier/handle. |
| `INVALID_PADDING` | Bad bucket/config/padding metadata | `REJECT_METADATA_PADDING` | Must not echo plaintext/ciphertext. |
| `EXPIRED_OR_DELETED` | Access to expired/deleted/purged state | `REJECT_METADATA_GONE` | Avoid over-specific object history unless fixture-only. |
| `CONFLICT_OR_STATE` | Coarse state transition rejection | `REJECT_METADATA_STATE` | Must not expose exact internal state counts. |
| `RATE_OR_CAPACITY` | Coarse bounded resource refusal | `REJECT_METADATA_CAPACITY` | Avoid exact capacity/queue details. |
| `INTERNAL_SAFE` | Safe internal failure wrapper | `REJECT_METADATA_INTERNAL` | No panic/backtrace/body dump. |

Surface-specific reason codes can remain more specific inside qsl-server or
qsl-attachments local tests, but NA-0293 should prove the metadata phase-2
taxonomy can map those specifics to safe cross-surface classes.

### Cross-Surface Mapping

| Surface | Current shape | Future mapping |
| --- | --- | --- |
| qshield CLI demo | Human stderr plus demo markers | Normalize invalid config, invalid relay response, malformed store, and relay reject output into coarse class plus stable reason. |
| qshield public demo relay | HTTP status and short body | Keep status plus coarse body; scan for token, identifier, descriptor, plaintext, ciphertext, panic, and backtrace absence. |
| Metadata conformance smoke | Demo HTTP fixture checks | Extend to expired/deleted/purged fixture if available, otherwise stop with prerequisite. |
| NA-0291 harness | Deterministic policy fixture output | Reuse fixture style for invalid identifier/padding, adding body/log scan. |
| qsl-server | Service-specific status/code | Treat as related evidence; do not change service behavior unless a separate service directive authorizes it. |
| qsl-attachments | Service `reason_code` JSON | Treat as related evidence; do not change service behavior unless a separate service directive authorizes it. |
| Public evidence docs | Claim boundary and reproduction guidance | Keep policy future-gated until NA-0293 executable proof exists. |

### Future Negative Tests

NA-0293 should prove, or stop with a prerequisite, for:

- malformed metadata input;
- auth failure and unauthorized capability/route inputs;
- invalid identifier and invalid padding config;
- expired, deleted, and purged object/session/route fixture access;
- rejected-state no-mutation;
- panic/backtrace absence;
- no raw token, capability, identifier, descriptor, plaintext, ciphertext, or
  secret-bearing path in error bodies, logs, transcripts, and evidence output.

### Out-of-Scope Items

Out of scope for NA-0292 and only future-gated for NA-0293 unless separately
authorized:

- qsp protocol-core reject taxonomy changes;
- cryptographic state-machine error changes;
- qsl-server or qsl-attachments implementation changes;
- production deployment log/metrics/proxy behavior;
- timing resistance, batching, jitter, cover traffic, or contact-graph hiding.

## Retention/Purge Metadata Policy Design

### Retention/Purge Metadata Taxonomy

| Metadata type | Current owner | Current status | Policy boundary |
| --- | --- | --- | --- |
| Demo stores/transcripts | qshield demo/operator evidence | `DOCS_ONLY` plus selected executable checks | Evidence retention is for reproducibility; not runtime purge policy. |
| Demo relay queues | qshield demo relay | `PROVEN_EXECUTABLE` for selected smoke behavior | Future tests can add expired/deleted fixture state without claiming production relay retention. |
| qsl-server route queues | qsl-server | `PROVEN_EXECUTABLE` locally | TTL/retention evidence is local and service-scoped. |
| qsl-server route TTL | qsl-server | `PROVEN_EXECUTABLE` locally | Deployment retention remains future-gated. |
| qsl-attachments sessions | qsl-attachments | `PROVEN_EXECUTABLE` locally | Expiry/delete evidence is service-scoped; capabilities remain secret. |
| qsl-attachments objects | qsl-attachments | `PROVEN_EXECUTABLE` locally | Object expiry and fetch behavior are local; object delete endpoint is not introduced. |
| qsl-attachments backup/restore | qsl-attachments | `PROVEN_EXECUTABLE` locally for stopped/quiesced full-root fixtures | Production backup automation, hot/live backup, and partial restore support remain not ready. |
| Logs/audit records | demo/service/evidence surfaces | `PROVEN_EXECUTABLE` for selected local scans; `FUTURE_GATE` for deployment artifacts | Logs may include coarse class, reason, redacted handle, and bounded counts only. |
| Release/evidence artifacts | qsl-protocol governance | `DOCS_ONLY` | Use short SHAs, path-pattern wording, and no secret-like dumps. |

### Protocol / Demo / Service / Artifact Separation

Protocol metadata phase-2 policy:

- defines fixture taxonomy, claim boundaries, and harness requirements;
- may add executable qsl-protocol fixtures in NA-0293 only if authorized;
- must not change qsp wire, crypto, or runtime semantics without explicit
  successor scope and tests.

Demo metadata policy:

- may prove non-production qshield error and retention fixture behavior;
- must keep local demo stores and artifacts separate from production promises;
- must not claim contact-graph hiding, timing resistance, anonymity, or
  metadata-free operation.

Service metadata policy:

- qsl-server and qsl-attachments local evidence informs the taxonomy;
- implementation or deployment changes must remain service-scoped and separately
  authorized;
- production retention policy requires exact deployment evidence.

Evidence artifact policy:

- qbuild transcripts, logs, and docs should retain enough evidence to reproduce
  claims while scanning out secrets;
- evidence artifact retention is governance memory, not runtime purge
  semantics.

### Future Retention/Purge Tests

NA-0293 should prove, or stop with a prerequisite, for:

- deleted/expired/purged fixture access returns an allowed coarse taxonomy;
- rejected purge/retention inputs do not mutate accepted fixture state;
- expired/deleted/purged state does not resurrect after restart-like fixture
  reconstruction where the harness owns the state;
- purge error bodies and logs do not reveal raw tokens, capabilities,
  identifiers, descriptors, plaintext, ciphertext, panic text, or backtraces;
- deterministic retention-window fixtures use test-controlled clocks and do
  not claim production schedules;
- public docs still mark retention/purge metadata as future-gated until the
  executable harness lands.

### qsl-server Relationship

qsl-server route lifecycle evidence is relevant because it proves local TTL,
drain-to-empty, stale-message discard, no-resurrection, and secret-safe cleanup
logs. NA-0292 does not change qsl-server. Future qsl-protocol harnesses should
reference qsl-server evidence as a service-scoped input, not as metadata
phase-2 completion or production relay readiness.

### qsl-attachments Relationship

qsl-attachments retention, cleanup, capability, and backup/restore evidence is
relevant because it proves local expired/deleted/aborted/rejected-state
handling, coherent recovery boundaries, no resurrection, and secret-safe logs.
NA-0292 does not change qsl-attachments. Future qsl-protocol harnesses should
reference qsl-attachments evidence as a service-scoped input, not as production
attachment readiness or metadata-free behavior.

## NA-0293 Executable Harness Plan

Recommended successor:

`NA-0293 - Metadata Phase-2 Sanitized Errors and Retention/Purge Executable Harness`

Planned proof categories:

1. Sanitized error fixture tests:
   - malformed metadata input;
   - invalid identifier;
   - invalid padding config;
   - unauthorized route/capability inputs;
   - expired/deleted/purged object or state fixture access;
   - panic/backtrace absence;
   - no secret/token/capability/identifier/descriptor/plaintext/ciphertext
     leakage.
2. Retention/purge metadata fixture tests:
   - deleted/expired/purged state fixture;
   - no resurrection fixture;
   - no over-specific metadata in error fixture;
   - no secret in logs fixture;
   - deterministic retention window fixture.
3. Cross-surface integration:
   - preserve `metadata_conformance_smoke.sh`;
   - preserve NA-0291 identifier/padding harness proof;
   - add demo smoke only when the fixture surface is available and bounded.
4. Required markers:
   - `NA0293_SANITIZED_ERROR_POLICY_OK`
   - `NA0293_RETENTION_PURGE_POLICY_OK`
   - `NA0293_METADATA_PHASE2_SANITIZED_RETENTION_OK`

Stop conditions for NA-0293:

- implementation would require qsp protocol-core changes without explicit
  successor authorization;
- implementation would require crypto/state-machine or service changes without
  explicit successor authorization;
- implementation would require dependency, workflow, branch-protection, or
  public-safety configuration changes;
- current qsl-protocol surfaces cannot host deleted/expired/purged fixture
  proof truthfully;
- public wording would need to exceed evidence.

Likely files for NA-0293, subject to its directive:

- a new deterministic fixture vector under `inputs/metadata_phase2/**`;
- a new qsl-protocol harness under `scripts/ci/**` only if scripts are
  explicitly authorized by NA-0293;
- a new evidence doc and testplan;
- conservative updates to release and external-review maps.

## Public Claim Boundary

NA-0292 preserves:

- no anonymity claim;
- no metadata-free claim;
- no untraceable claim;
- no production-readiness claim;
- no public-internet-readiness claim;
- no external-review-complete claim;
- no claim that metadata phase-2 is complete;
- no claim that sanitized-error policy is implemented;
- no claim that retention/purge policy is implemented.

## Safe Future Public Wording

Safe wording before NA-0293:

- "sanitized-error and retention/purge metadata policy is designed and
  future-gated";
- "current executable evidence covers selected demo and local service cases";
- "service-local retention evidence does not prove public metadata phase-2
  policy";
- "metadata phase-2 remains incomplete";
- "not anonymity, not metadata-free messaging, and not untraceability";
- "not production readiness, not public internet readiness, and not external
  review completion."

Safe wording after NA-0293 only if the executable harness lands:

- "bounded sanitized-error and retention/purge metadata fixture evidence exists
  for the scoped harness";
- "runtime/service/deployment claims remain limited to the exact surfaces
  tested."

## Prohibited Wording

The following phrases must not appear as affirmative claims:

- production-ready
- deployment-ready
- production relay ready
- production attachment ready
- public internet ready
- external review complete
- externally reviewed
- review complete
- metadata-free
- anonymity
- anonymous messaging
- untraceable
- quantum-proof
- proven true Triple Ratchet
- sanitized errors implemented
- retention implemented
- purge implemented

These phrases may appear only when explicitly negated, explicitly marked
`NOT_READY`, explicitly future/unproven, or listed as prohibited wording.

## Relationship to NA-0290A Public Visibility Strategy

NA-0290A recommends memorable evidence-bound storytelling. NA-0292 narrows what
that storytelling may say for sanitized errors and retention/purge metadata:

- lead with demonstrable fail-closed and no-leak properties only where tests
  exist;
- present design lanes as design, not implementation;
- keep residual metadata visible;
- do not turn future-gated metadata work into hype.

## What Is Not Implemented In NA-0292

NA-0292 does not implement:

- sanitized-error runtime expansion;
- retention/purge runtime policy;
- deleted/expired/purged fixture harnesses;
- qshield demo code changes;
- qsl-server or qsl-attachments changes;
- qsp protocol-core, crypto, auth, negotiation, state-machine, or wire-format
  changes;
- scripts, inputs, formal models, tools/refimpl, qsc/qsl runtime, desktop,
  website, README, START_HERE, workflow, Cargo, dependency, branch-protection,
  or public-safety changes.

## Recommended Successor Lane

Close out NA-0292 only after this design PR merges and post-merge
public-safety is green. The recommended successor is:

`NA-0293 - Metadata Phase-2 Sanitized Errors and Retention/Purge Executable Harness`

NA-0293 should add executable proof or stop with exact prerequisites. It must
preserve no anonymity, no metadata-free, no untraceable, no external-review-
complete, no production-readiness, and no public-internet-readiness claims.
