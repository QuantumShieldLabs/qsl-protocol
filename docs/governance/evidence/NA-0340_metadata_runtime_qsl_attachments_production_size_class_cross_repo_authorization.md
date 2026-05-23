Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-23

# NA-0340 Metadata Runtime qsl-attachments Production Size-Class Cross-Repo Authorization

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0340 is a qsl-protocol governance and authorization-planning lane. It does
not implement qsl-attachments behavior, qsl-server behavior, qshield runtime
behavior, qsc/qsp/protocol/crypto behavior, dependency changes, workflow
changes, service deployment, website changes, or public-copy changes.

The result is a prerequisite authorization plan, not an implementation
approval. A future qsl-attachments production size-class lane must first obtain
an exact source/authority bundle: repository URL, local path, branch, base SHA,
merge authority, allowed files, CI entrypoints, rollback/deploy boundary,
storage/retention/backup model, qsl-server integration boundary, and public
claim boundary. Until that bundle exists and is authorized by a later directive,
qsl-attachments production object-size padding remains unimplemented and
unproven, and qsl-server production timing/storage behavior remains unproven.

Selected successor:

`NA-0341 -- Metadata Runtime qsl-attachments Source / Authority Bundle`

## Live NA-0340 Scope

Live `NEXT_ACTIONS.md` authorized:

- `NA-0340 -- Metadata Runtime qsl-attachments Production Size-Class
  Cross-Repo Authorization`
- Status: READY
- Goals: G1, G2, G3, G4, G5
- Objective: execute a qsl-attachments production size-class cross-repo
  authorization plan, or stop on an exact prerequisite.

Protected live boundaries:

- no unsupported production, public-internet, external-review, anonymity,
  unsupported metadata-free, or unsupported untraceable claim;
- no claim that attachment size, timing metadata, or traffic shape is hidden
  unless exact future evidence proves it;
- executable proof or exact prerequisite stop;
- qshield embedded relay/demo proof remains distinct from qsl-server and
  qsl-attachments production behavior;
- qsl-server production timing/storage behavior remains explicit and
  cross-repo-gated unless exact future scope authorizes it;
- qsl-attachments production object-size padding remains unimplemented until a
  future cross-repo authorization proves the exact safe scope;
- no qsl-server, qsl-attachments, qsc/qsp/protocol/crypto/key-schedule,
  dependency, workflow, website, README, START_HERE, docs/public,
  branch-protection, or public-safety configuration change unless exact future
  scope authorizes it.

Expected deliverables were review of NA-0339 qshield demo proof, inventory of
qsl-attachments production object-size/storage/retention/backup constraints,
authorization or exact blocker selection, preservation of the qshield demo
versus production boundary, and exact successor selection.

## Inherited NA-0339 qshield Demo Attachment Size-Class Proof

NA-0339 D-0660 implemented only bounded qshield embedded relay/demo attachment
size-class handling:

- opt-in gate: `QSHIELD_DEMO_ATTACHMENT_SIZE_CLASSES`;
- policy: `qshield_demo_attachment_size_class_v1`;
- deterministic qshield demo size-class table: `[256, 512, 768, 1024, 1536,
  2048, 3072, 4096, 5120, 6144, 7168, 8192]`;
- max padded qshield demo attachment ciphertext object: 8192 bytes;
- max overhead: 1023 bytes;
- descriptor stays separate, encrypted, and metadata-bearing;
- descriptor continues to carry exact unpadded ciphertext length and hash;
- receive verifies and strips the padded ciphertext object before hashing,
  decrypting, ACKing, or writing output;
- invalid config, oversize, malformed descriptor, and malformed ciphertext
  reject deterministically with no accepted state or output.

This proof is useful as a local/demo oracle for fail-closed size-class handling,
artifact hygiene, and compatibility with qshield padding/cover/batching/retry
and jitter lanes. It is not qsl-attachments production object-size padding, not
qsl-server production timing/storage proof, and not public-internet behavior
proof.

## Sources Inspected

qsl-protocol sources inspected:

- `NEXT_ACTIONS.md` live NA-0340 entry.
- `tests/NA-0339_closeout_restore_na0340_testplan.md`.
- `docs/governance/evidence/NA-0339_metadata_runtime_qshield_demo_attachment_size_class_harness.md`.
- `tests/NA-0339_metadata_runtime_qshield_demo_attachment_size_class_harness_testplan.md`.
- `docs/governance/evidence/NA-0338_metadata_runtime_attachment_size_class_authorization.md`.
- `docs/governance/evidence/NA-0337_metadata_runtime_qshield_demo_padding_bucket_expansion_harness.md`.
- `docs/governance/evidence/NA-0335_metadata_runtime_qshield_demo_cover_traffic_prototype_harness.md`.
- `apps/qshield-cli/src/commands/attachment.rs`.
- `docs/canonical/DOC-CAN-005_QSP_Attachment_Descriptor_and_Control_Plane_v0.1.0_DRAFT.md`.
- `docs/canonical/DOC-CAN-006_QATT_Attachment_Service_Contract_v0.1.0_DRAFT.md`.
- `docs/canonical/DOC-CAN-007_QATT_Attachment_Encryption_Context_and_Part_Cipher_v0.1.0_DRAFT.md`.
- `docs/design/DOC-ATT-002_qsl-attachments_Deployment_and_Operational_Hardening_Contract_v0.1.0_DRAFT.md`.
- `TRACEABILITY.md`.
- `DECISIONS.md`.

Read-only local qsl-attachments sources inspected:

- `/srv/qbuild/work/NA-0237C/qsl-attachments` at `1e1ae272a4cb`.
- `/srv/qbuild/work/NA-0237/qsl-attachments` at `1e1ae272a4cb`.
- `/srv/qbuild/work/NA-0237A/qsl-attachments` at `1e1ae272a4cb`.
- `/srv/qbuild/work/NA-0237B/qsl-attachments` at `1e1ae272a4cb`.
- `/srv/qbuild/work/NA-0237D/qsl-attachments` detached at `320be68fe632`, with
  local `origin/main` also at `320be68fe632`.

No qsl-attachments checkout was fetched, cloned, checked out, branched,
committed, pushed, built, tested, or modified by NA-0340.

Search coverage included `qsl-attachments`, `attachment`, `object size`,
`object-size`, `size class`, `size-class`, `padding`, `bucket`, `upload`,
`fetch`, `object`, `descriptor`, `ciphertext`, `storage`, `retention`,
`purge`, `backup`, `quota`, `abuse`, `DoS`, `production`, `public internet`,
`metadata-free`, `anonymity`, `untraceable`, `size hidden`, `timing hidden`,
`traffic hidden`, `FUTURE_GATE`, and `NOT_READY`.

## qsl-attachments Source / Authority / Local Availability Inventory

| Field | Result |
| --- | --- |
| qsl-attachments source repo/path known? | Partial. Local historical qbuild checkouts point to `https://github.com/QuantumShieldLabs/qsl-attachments.git`. |
| local checkout present? | Yes, under historical NA worktrees, not as a current NA-0340 sibling checkout. |
| remote URL if local checkout present | `https://github.com/QuantumShieldLabs/qsl-attachments.git`; mirror remote also present. |
| current branch/ref if local checkout present | Most current local source inspected: detached `HEAD` at `/srv/qbuild/work/NA-0237D/qsl-attachments`; local `origin/main` at `320be68fe632`. Other clean historical checkouts were on `main` at `1e1ae272a4cb`. |
| current SHA if local checkout present | `320be68fe632` for the most current local source inspected; `1e1ae272a4cb` for older historical checkouts. |
| worktree clean if local checkout present | Yes for inspected paths. |
| CI/test entrypoints known? | Partially. Local source has Cargo package `qsl-attachments`, `.github/workflows/rust.yml`, and `tests/service_contract.rs`; exact future required commands must be frozen by NA-0341. |
| package/build system known? | Rust/Cargo package, based on local manifest inspection. |
| allowed future files known? | Not yet. NA-0340 defines categories only; exact files require NA-0341. |
| production storage/retention model known? | Partially. Local code and docs show single-node local-disk storage, sessions, objects, retention classes, expiry sweep, quota, abuse tracking, and cold whole-root backup boundary. Exact size-class mutation points are not authorized here. |
| deployment/public-internet status known? | Partial and not current-authoritative. Local docs describe loopback/private/reference deployment evidence and TLS requirements before public exposure; no public-internet size-class behavior is proven. |
| merge authority known? | Not proven for a future lane. |
| cross-repo PR permission known? | Not proven for a future lane. |
| rollback/deploy boundary known? | Partially described in docs; exact rollback and deploy/non-deploy boundary must be provided by NA-0341. |
| secrets/env status known? | Partially. Local docs and tests emphasize secret-bearing headers, redacted logs, and no secret-bearing URLs. Exact future env/secrets handling must be frozen by NA-0341. |
| backup impact known? | Partially. Current qsl-protocol patch is inside existing backup scope. Future qsl-attachments artifacts may require backup-plan confirmation if outside `/srv/qbuild/work` coverage or if non-rebuildable production evidence is captured elsewhere. |
| conclusion | `DOCS_ONLY_PLANNING`: local source exists but future implementation cannot be authorized until NA-0341 proves exact source, authority, base SHA, allowed files, CI, and deployment/backup boundaries. |

## Production qsl-attachments Problem Statement

Production qsl-attachments stores and serves opaque ciphertext objects with
descriptor-visible object metadata. The current qshield demo size-class proof
does not alter that production service. A future qsl-attachments size-class
lane would need to decide where size-class padding belongs in the object
lifecycle, how it interacts with descriptor fields, how receivers verify and
strip any padded object, how storage/retention/purge/backup costs are bounded,
and how qsl-server integration remains honest about timing and traffic-shape
residuals.

Candidate authorization options:

| Option | Result | Rationale |
| --- | --- | --- |
| Direct qsl-attachments implementation now | Rejected | NA-0340 only mutates qsl-protocol governance. Exact source/authority, allowed files, CI, rollback, deploy boundary, and qsl-server integration evidence are not yet frozen. |
| qsl-protocol-only planning with future source/authority bundle | Selected | Preserves scope while making the next prerequisite exact. |
| Block all future size-class work | Rejected | Local source and prior evidence are sufficient to define prerequisites; they do not prove impossibility. |
| Service timing authorization first | Rejected for immediate successor | qsl-attachments source/authority is the narrower next blocker before implementation authorization. |
| External review gap audit first | Rejected for immediate successor | Review remains important, but exact source/authority is needed before a reviewable implementation boundary can be stated. |

Stop conditions for any future implementation lane:

- qsl-attachments source/ref/authority cannot be proven;
- requested changes touch files outside the future allowed list;
- source inspection shows descriptor, storage, retention, purge, backup, auth,
  quota, or abuse semantics would be weakened;
- qsl-server timing/storage integration is required but not authorized;
- public-internet exposure, deployment, or public claims would be changed;
- production evidence is missing but local/demo evidence is being used as a
  substitute;
- any language claims attachment size, timing metadata, traffic shape, or all
  metadata is hidden without exact future proof.

## Production Size-Class Threat / Value Model

Potential production threats that a size-class design might reduce:

- observer inference from committed ciphertext object length;
- upload/fetch size correlation across service requests;
- coarse attachment type or size-class inference from object distribution;
- queue/storage object distribution analysis;
- retry/fetch timing correlation with larger object sizes.

Threats it does not solve:

- endpoint compromise;
- production logs or operator surfaces that record too much;
- route/contact relationship leakage;
- content key compromise;
- public-network IP and traffic observation;
- broad metadata-free behavior;
- production or public-internet readiness.

Production hazards:

- storage amplification from padded committed objects;
- egress cost amplification on fetch;
- abuse and DoS through padded maximum objects;
- backup growth and restore-time growth;
- retention and purge complexity;
- object lifecycle corruption if padded bytes, descriptor lengths, and
  integrity roots are not consistently bound;
- deployment risk if live services need migrations;
- compatibility risk for legacy objects and qshield demo evidence.

Required evidence before implementation:

- exact qsl-attachments source/ref and authority;
- object storage and lifecycle map;
- upload/fetch APIs and auth/authorization boundaries;
- quota, rate-limit, and abuse surfaces;
- retention/purge semantics and stale-object cleanup;
- tests/CI commands and markers;
- rollback and deploy/non-deploy plan;
- qsl-server integration boundary;
- public-claim and external-review boundary.

## Cross-Repo Implementation Boundary

NA-0340 does not authorize implementation. A later implementation directive
must include both qsl-protocol governance updates and a qsl-attachments
PR/verification bundle, or explicitly keep qsl-attachments local-only and
unmutated.

Required future authorization fields:

| Field | Required future content |
| --- | --- |
| exact repository URL/path | `QuantumShieldLabs/qsl-attachments` URL plus local qbuild worktree path. |
| exact branch/base SHA | Branch name and 12-char base SHA, with proof of clean worktree and no stale ref ambiguity. |
| allowed files | Exact file list or exact glob set for source, tests, docs, and evidence. |
| forbidden files | qsl-protocol runtime, qsl-server, qshield runtime, qsc/qsp/protocol/crypto/key-schedule, workflows, dependency manifests unless expressly authorized, website/public copy, and branch protection. |
| build/test commands | Cargo build/test/fmt/audit commands and any repo-local service-contract harnesses. |
| CI expectations | Required qsl-attachments checks and qsl-protocol public-safety expectations. |
| storage/object model | Where padded bytes live, whether object metadata records padded/unpadded lengths, how integrity roots are computed, and how receivers verify. |
| migration requirement | Explicit legacy-object and in-flight-session compatibility plan. |
| retention/purge requirement | Expiry, purge, failed upload, failed fetch, stale object, and backup cleanup rules. |
| backup-plan requirement | Confirmation that future durable artifacts and production evidence are inside backup scope, or explicit backup-plan update before mutation. |
| rollback plan | How to disable or revert size-class behavior without corrupting committed objects or descriptors. |
| production deploy boundary | Whether changes are service code only, docs only, local-only, or deploy-affecting; no public deployment without explicit directive. |
| qsl-server integration boundary | Whether server behavior is untouched or explicitly updated in a separate authorized lane. |
| qshield demo compatibility boundary | qshield demo remains a reference/oracle only; no production proof substitution. |
| secret/env handling | No secret-bearing URL/log/artifact drift; exact env gates if any. |
| public-claim boundary | No size-hidden, timing-hidden, traffic-shape-hidden, metadata-free, anonymity, untraceable, production-readiness, public-internet-readiness, or external-review-complete claim. |
| external-review recommendation | Mark review-sensitive surfaces and require review evidence before stronger public claims. |
| stop conditions | Stop on unclear root cause, weakened fail-closed semantics, source/authority mismatch, out-of-scope files, CI red, or overclaim pressure. |

## Authorization Requirements

A future implementation authorization must prove all of the following before
any qsl-attachments mutation:

1. Exact qsl-attachments repo, branch, base SHA, and clean worktree.
2. Exact authority to create and merge the qsl-attachments PR.
3. Exact allowed and forbidden files.
4. Exact build, test, audit, and CI entrypoints.
5. Exact object lifecycle design for padded and unpadded lengths.
6. Exact descriptor compatibility rule and receiver verification order.
7. Exact retention, purge, and backup effect.
8. Exact abuse/DoS quota and latency bound.
9. Exact qsl-server non-change or integration-change boundary.
10. Exact deployment and rollback posture.
11. Exact public-claim and external-review boundary.
12. Required qsl-protocol governance and traceability updates in the same
    implementation set.

## Storage / Retention / Purge / Backup / Ops Model

The production model below is required before implementation. Fields marked
`REQUIRED_BEFORE_IMPLEMENTATION` are not proven by NA-0340.

| Area | Required model |
| --- | --- |
| object lifecycle | `REQUIRED_BEFORE_IMPLEMENTATION`: create session, upload parts, commit padded/unpadded object, fetch, expire, purge, and rollback behavior must be mapped before mutation. |
| descriptor lifecycle | Descriptor-visible fields must stay authenticated and transcript-bound; any padded-object design must state what lengths remain descriptor-visible and why. |
| ciphertext/object lifecycle | Padded bytes, original ciphertext bytes, integrity roots, and fetch bytes must have one exact source of truth. |
| size-class padding object lifecycle | Padding generation, validation, stripping, and rejection must be deterministic and must not create accepted state on malformed objects. |
| retention duration | Existing retention classes may not be redefined silently; size-class storage growth must be accounted for per class. |
| purge trigger | Expiry, explicit abort, failed upload cleanup, failed fetch cleanup, and stale object cleanup must be deterministic. |
| stale object cleanup | `REQUIRED_BEFORE_IMPLEMENTATION`: startup/reconciliation and operator cleanup behavior must be updated or proven unaffected. |
| failed upload cleanup | Padded staging overhead must not cause partial object exposure. |
| failed fetch cleanup | Fetch rejects must not mutate object state except where existing capability/abuse semantics explicitly permit. |
| backup inclusion/exclusion | Cold whole-root backup/restore is the current known boundary; future padded artifacts must be included or explicitly excluded with a backup-plan update. |
| log redaction | No plaintext, raw capabilities, secret-bearing URLs, full stable identifiers, or padding contents in logs. |
| artifact redaction | Evidence artifacts must use short handles, summary counts, and short SHAs. |
| monitoring | Required metrics include object-size distribution by class, reject counts, quota pressure, disk growth, egress growth, retention/purge lag, and restart recovery summaries. |
| alert thresholds | Required for disk floor, quota saturation, repeated invalid upload/fetch attempts, purge lag, abnormal object growth, and error spikes. |
| operator runbook | Must include enable/disable, rollback, storage growth, backup/restore, abuse response, and incident triage steps. |
| rollback | Must not strand objects in a shape receivers cannot verify; must define legacy-object compatibility. |
| migration/compatibility | Legacy objects and in-flight sessions must remain readable or reject truthfully with no silent corruption. |
| abuse/cost threshold | Must cap padded object size, per-deployment storage, per-session upload, fetch egress, and invalid attempts. |

## Abuse / DoS / Quota / Latency / Compatibility Matrix

| Scenario | Risk | Required future bound | Future test | Failure mode | Stop condition | Compatibility impact | Claim boundary |
| --- | --- | --- | --- | --- | --- | --- | --- |
| valid small production attachment | needless amplification | minimum class and overhead cap | small object round trip | reject or corrupt fetch | descriptor/object mismatch | legacy small objects unaffected | no size-hidden claim |
| valid medium production attachment | storage/egress growth | class table and quota cap | medium object round trip | quota reject | unbounded growth | existing retention unaffected | no timing-hidden claim |
| valid large production attachment | large cost and latency | max padded object and egress cap | large object round trip | bounded quota reject | host saturation ambiguity | large-file ladder preserved | no public-internet-readiness claim |
| oversized production object | DoS | reject before mutation | oversize create/upload | quota reject | accepted state on oversize | no legacy break | no readiness claim |
| malformed descriptor | fetch/decrypt confusion | reject before fetch/output | descriptor negative test | descriptor reject | any output or state mutation | descriptor contract preserved | no metadata-free claim |
| malformed ciphertext/object | integrity bypass | verify before accepted state | object tamper test | integrity reject | ACK/fetch success on tamper | existing object fetch preserved | no attachment-size-hidden claim |
| invalid size-class config | silent downgrade | reject before mutation | invalid config table | config reject | fallback to unpadded without explicit policy | operator-visible config behavior | no silent downgrade |
| repeated invalid upload/fetch attempts | abuse escalation | attempt limits and rate policy | abuse counter test | abuse reject | unbounded retries | existing capability semantics preserved | no public-internet claim |
| retention/purge failure | stale data | purge lag alert and deterministic cleanup | expiry/purge test | stale object retained | unclassified retention drift | current retention classes preserved | no production-readiness claim |
| backup growth | backup pressure | backup sizing and restore proof | backup/restore evidence | restore mismatch | backup-plan gap | cold whole-root boundary preserved | no readiness claim |
| storage quota exhaustion | service instability | disk floor plus class cap | disk pressure test | quota reject | disk exhaustion without deterministic reject | existing quota semantics preserved | no availability overclaim |
| public internet abuse | exposed endpoint pressure | explicit public-exposure lane | public-abuse simulation only if authorized | rate/abuse reject | public exposure without authorization | deployment boundary explicit | no public-internet-readiness claim |
| qsl-server integration mismatch | timing/storage confusion | separate qsl-server boundary | integration conformance test | mismatch reject | qsl-server mutation without scope | transport-only posture preserved | no timing-hidden claim |
| qshield demo compatibility | false production proof | demo remains oracle only | qshield harness regression | demo drift | demo proof presented as production proof | demo behavior unaffected | no production proof claim |
| migration/legacy object compatibility | stranded objects | migration/rollback plan | legacy read/fetch tests | truthful reject or success | silent unreadable objects | legacy compatibility explicit | no broad readiness claim |
| external-review-sensitive claim | public overclaim | review gate and claim lint | claim-boundary scan | blocked claim | claim stronger than evidence | public docs unchanged unless authorized | no external-review-complete claim |

## qsl-server Integration Boundary

qsl-server integration is not authorized by NA-0340. A future
qsl-attachments implementation may be possible without qsl-server changes only
if qsl-server remains transport-only and the attachment service continues to
use existing endpoint semantics. If future size-class behavior affects service
timing, routing, retry behavior, storage coordination, or public ingress, that
requires a separate explicit qsl-server/source authorization lane.

qshield demo proof may be used only as a reference/oracle for bounded class
selection, fail-closed rejection, artifact hygiene, and compatibility
thinking. It cannot prove qsl-server or qsl-attachments production behavior.

Public-internet behavior remains unproven. Before any public claim exists,
future evidence must cover implementation, service behavior, deployment
configuration, ingress/TLS/log redaction, abuse controls, quota/cost behavior,
backup/restore, rollback, qsl-server integration if any, and external review
as applicable.

## Public-Internet Boundary

NA-0340 does not authorize public exposure or public-internet behavior changes.
Future public-internet work must define:

- ingress/TLS and reverse-proxy log redaction;
- preservation of secret-bearing headers;
- rate limiting, quota, abuse and DoS controls;
- operator monitoring and alert thresholds;
- backup, restore, purge and incident-response runbooks;
- rollback and disablement without object corruption;
- explicit public claim limits.

## External-Review Sensitivity

External review remains incomplete. Production qsl-attachments object-size
padding would be review-sensitive because it touches metadata minimization,
object lifecycle, service storage, quota, abuse, retention, and potentially
public claims.

Any stronger claim requires:

- implementation evidence;
- service evidence;
- deployment evidence where deployment is relevant;
- qsl-server integration evidence where integration is relevant;
- external review evidence for review-dependent claims.

## Public Claim Boundary

Allowed wording:

- planning-only;
- future-gated;
- local/demo-only for qshield proof;
- production qsl-attachments object-size padding remains unimplemented and
  unproven;
- qsl-server production timing/storage behavior remains unproven;
- size, timing, traffic-shape, and metadata residuals remain explicit.

Prohibited wording unless a later directive supplies exact proof and allows the
claim:

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
- quantum-proof hype, unbreakable, guaranteed-secure, or military-grade
  language.

No website or public docs update is made by NA-0340.

## Selected Successor

`NA-0341 -- Metadata Runtime qsl-attachments Source / Authority Bundle`

Rationale:

- Local qsl-attachments source exists, but current authority/freshness and
  future merge/CI/deploy boundaries are not frozen by this qsl-protocol-only
  directive.
- A production size-class implementation authorization plan would be premature
  without exact source, base SHA, allowed files, test commands, merge authority,
  rollback, and backup/deploy constraints.
- Service timing and external-review lanes remain important, but source and
  authority are the next narrow blocker before implementation authorization.

## Rejected Alternatives

| Alternative | Rejection reason |
| --- | --- |
| `NA-0341 -- Metadata Runtime qsl-attachments Production Size-Class Implementation Authorization Plan` | Premature until source/authority and exact allowed files/CI/deploy boundaries are frozen. |
| `NA-0341 -- Metadata Runtime qsl-attachments Production Size-Class Blocker Resolution` | Too vague; the blocker is specifically source/authority and implementation boundary evidence. |
| `NA-0341 -- Metadata Runtime Service Timing Cross-Repo Authorization` | Important later, but qsl-attachments source/authority must be established first. |
| `NA-0341 -- Metadata Runtime External Review Readiness Gap Audit` | Review remains relevant, but not the immediate prerequisite for an implementation boundary. |
| Direct qsl-attachments implementation | Out of scope and unsafe without exact cross-repo authorization. |
| Public docs or website update | Out of scope and unnecessary; no public claim is strengthened. |

## Backup-Plan Impact Statement

No backup-plan update is required for NA-0340. The patch changes only tracked
qsl-protocol files under `/srv/qbuild/work`, which is already inside the local
backup scope. No durable evidence location, response root, source root, or
excluded backup path is moved.

Future qsl-attachments implementation or production evidence may require a
backup-plan update if source worktrees, deployed service artifacts, large
validation objects, backup/restore evidence, or non-rebuildable production
artifacts live outside the current `/srv/qbuild/work` coverage.

## Next Recommendation

Execute `NA-0341 -- Metadata Runtime qsl-attachments Source / Authority Bundle`
as a governance/source-authority lane. It should prove the exact qsl-attachments
repository, current base SHA, local worktree, merge authority, CI commands,
allowed files, forbidden files, deployment boundary, rollback boundary,
backup-plan posture, qsl-server integration boundary, and public-claim
boundary. It must not implement qsl-attachments behavior unless a later
directive explicitly expands scope after that source/authority bundle is green.
