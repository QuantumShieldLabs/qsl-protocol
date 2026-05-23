Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-23

# NA-0343 Metadata Runtime qsl-attachments Production Size-Class Implementation Authorization

## Executive summary

NA-0343 is a qsl-protocol governance and authorization-planning lane for a
future qsl-attachments production size-class implementation harness. It does
not mutate qsl-attachments, qsl-server, qshield runtime, qsc/qsp/protocol,
crypto, dependencies, workflows, public copy, branch protection, or service
deployment.

Result: `IMPLEMENTATION_AUTHORIZATION_READY`.

The refreshed qsl-attachments source and authority proof still holds:

- repository: `QuantumShieldLabs/qsl-attachments`;
- URL: `https://github.com/QuantumShieldLabs/qsl-attachments`;
- selected local path: `/srv/qbuild/work/NA-0237D/qsl-attachments`;
- selected local ref state: detached `HEAD`;
- selected local HEAD: `320be68fe632`;
- remote default branch: `main`;
- live remote default branch SHA: `320be68fe632`;
- freshness classification: `FRESH_SOURCE`;
- viewer permission: `ADMIN`;
- mutation authority classification: `COMPLETE_MUTATION_AUTHORITY`;
- CI/protection classification: `COMPLETE_CI_AUTHORITY`;
- final authorization gate: `IMPLEMENTATION_AUTHORIZATION_READY`.

Selected successor:

`NA-0344 -- Metadata Runtime qsl-attachments Production Size-Class Implementation Harness`

NA-0343 only authorizes the next lane shape. A later directive must separately
authorize any qsl-attachments mutation and must restate repository, branch, base
SHA, allowed files, forbidden files, CI, rollback, deploy boundary, backup
boundary, qsl-server integration boundary, qshield demo compatibility boundary,
and public-claim boundary before implementation begins.

## Live NA-0343 scope

The live `NEXT_ACTIONS.md` entry on `origin/main` was:

- `NA-0343 -- Metadata Runtime qsl-attachments Production Size-Class
  Implementation Authorization Plan`;
- Status: READY;
- Goals: G1, G2, G3, G4, G5;
- Objective: execute the next metadata-runtime
  qsl-attachments/source/implementation-authorization lane selected by
  NA-0342.

The live scope required:

- exact qsl-attachments implementation authorization plan, or exact stop
  evidence if a prerequisite changed;
- exact qsl-attachments repo URL, local path, default branch/ref, and base SHA
  with refreshed source freshness proof;
- exact allowed and forbidden qsl-attachments files plus qsl-protocol
  governance/evidence files;
- exact build, test, lint, CI, required-check, rollback, deploy/non-deploy,
  backup, secrets/env, qsl-server integration, qshield demo compatibility, and
  public-claim boundaries;
- exact successor selection.

The live scope forbade unsupported production, public-internet,
external-review, anonymity, unsupported metadata-free, unsupported untraceable,
attachment-size-hidden, timing-hidden, traffic-hidden, qsl-server,
qsl-attachments, qshield, qsc/qsp/protocol/crypto/key-schedule, dependency,
workflow, website, README, START_HERE, docs/public, branch-protection, and
public-safety configuration changes unless a future exact directive authorizes
them.

## Inherited NA-0342 complete source/authority result

NA-0342 recorded `COMPLETE_SOURCE_AUTHORITY`.

Known from NA-0342:

- selected qsl-attachments source:
  `/srv/qbuild/work/NA-0237D/qsl-attachments`;
- local observed HEAD: `320be68fe632`;
- local observed state: detached clean checkout;
- live remote default branch `main` matched `320be68fe632`;
- qsl-attachments viewer permission was `ADMIN`;
- default branch protection existed;
- strict required check context was `rust`;
- the latest listed qsl-attachments `rust` run on `main` at `320be68fe632`
  completed success;
- workflow commands were:
  - `cargo fmt --all -- --check`;
  - `cargo clippy --all-targets -- -D warnings`;
  - `cargo build --locked`;
  - `cargo test --locked`.

NA-0342 did not authorize qsl-attachments mutation, qsl-server mutation,
runtime mitigation implementation, deployment, public-copy changes, or stronger
privacy/readiness claims.

## Refreshed source/authority proof

Read-only qsl-attachments local evidence from
`/srv/qbuild/work/NA-0237D/qsl-attachments`:

| Field | Result |
| --- | --- |
| worktree status | clean detached `HEAD` |
| selected local HEAD | `320be68fe632` |
| selected local ref state | detached `HEAD` |
| active branch | none |
| remote URL | `https://github.com/QuantumShieldLabs/qsl-attachments.git` |
| local `origin/main` | `320be68fe632` |
| live `git ls-remote origin HEAD` | `320be68fe632` |
| live `git ls-remote refs/heads/main` | `320be68fe632` |
| local non-active `main` ref | `1e1ae272a4cb` |
| local mirror ref | `1e1ae272a4cb` |

The stale non-active local `main` and local mirror refs do not block NA-0343
because the selected detached local HEAD and local `origin/main` match the live
remote default branch. A future implementation directive must create the
implementation branch from live remote `main` at the refreshed base SHA, not
from the stale non-active local `main` ref.

Read-only GitHub evidence:

- `gh repo view QuantumShieldLabs/qsl-attachments` reported default branch
  `main` and viewer permission `ADMIN`;
- default branch protection exists on `main`;
- required status checks are strict;
- required check context is `rust`;
- force pushes are disabled;
- deletions are disabled;
- open qsl-attachments PR list was empty at inspection time;
- latest listed qsl-attachments workflow run on `main` at `320be68fe632`
  completed `success`.

Classifications:

| Dimension | Classification | Basis |
| --- | --- | --- |
| source freshness | `FRESH_SOURCE` | selected local HEAD equals live remote `main` |
| mutation authority | `COMPLETE_MUTATION_AUTHORITY` | viewer permission is `ADMIN`; branch protection and normal PR path are known |
| CI authority | `COMPLETE_CI_AUTHORITY` | required `rust` check and workflow commands are known; latest main run is green |
| authorization gate | `IMPLEMENTATION_AUTHORIZATION_READY` | no source, authority, CI, or planning prerequisite regressed |

## Sources inspected

qsl-protocol sources inspected:

- `NEXT_ACTIONS.md`;
- `DECISIONS.md`;
- `TRACEABILITY.md`;
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`;
- `tests/NA-0342_closeout_restore_na0343_testplan.md`;
- `docs/governance/evidence/NA-0342_metadata_runtime_qsl_attachments_source_authority_blocker_resolution.md`;
- `tests/NA-0342_metadata_runtime_qsl_attachments_source_authority_blocker_resolution_testplan.md`;
- `docs/governance/evidence/NA-0341_metadata_runtime_qsl_attachments_source_authority_bundle.md`;
- `docs/governance/evidence/NA-0340_metadata_runtime_qsl_attachments_production_size_class_cross_repo_authorization.md`;
- `docs/governance/evidence/NA-0339_metadata_runtime_qshield_demo_attachment_size_class_harness.md`;
- `docs/canonical/DOC-CAN-005_QSP_Attachment_Descriptor_and_Control_Plane_v0.1.0_DRAFT.md`;
- `docs/canonical/DOC-CAN-006_QATT_Attachment_Service_Contract_v0.1.0_DRAFT.md`;
- `docs/canonical/DOC-CAN-007_QATT_Attachment_Encryption_Context_and_Part_Cipher_v0.1.0_DRAFT.md`;
- `docs/design/DOC-ATT-002_qsl-attachments_Deployment_and_Operational_Hardening_Contract_v0.1.0_DRAFT.md`;
- `docs/design/DOC-G5-004_Metadata_Leakage_Surface_Review_and_Logging_Contract_v0.1.0_DRAFT.md`.

qsl-attachments sources inspected read-only:

- `.github/workflows/rust.yml`;
- `Cargo.toml`;
- `README.md`;
- `START_HERE.md`;
- `DECISIONS.md`;
- `TRACEABILITY.md`;
- `docs/NA-0002_operational_hardening_contract.md`;
- `docs/NA-0007_authn_authz_policy_subject_contract.md`;
- `docs/NA-0009_durability_recovery_contract.md`;
- `src/lib.rs`;
- `src/main.rs`;
- `tests/service_contract.rs`;
- `tests/retention_cleanup_recovery.rs`;
- `tests/retention_cleanup_logging.rs`;
- `tests/disk_pressure_quota_abuse.rs`;
- `tests/disk_pressure_quota_logging.rs`;
- `tests/capability_scope_abuse.rs`;
- `tests/capability_scope_logging.rs`;
- `tests/backup_restore_recovery.rs`;
- `tests/backup_restore_logging.rs`;
- `tests/reject_taxonomy_harness.rs`;
- `tests/support/mod.rs`;
- `tests/NA-0003_constrained_host_validation_evidence.md`;
- `tests/NA-0004_reference_deployment_validation_evidence.md`;
- `tests/NA-0005_stress_soak_chaos_evidence.md`;
- `tests/NA-0010A_durability_recovery_validation_evidence.md`.

No qsl-attachments checkout was fetched, cloned, checked out, branched,
committed, pushed, merged, rebased, dependency-installed, built, tested,
deployed, or mutated.

## Implementation authorization readiness decision

Decision: `IMPLEMENTATION_AUTHORIZATION_READY`.

The future qsl-attachments implementation harness is authorized as the next
successor because:

- the selected local source exists and is fresh against live remote `main`;
- mutation/PR/merge authority is sufficient for a later exact directive to use
  the normal PR path;
- branch protection and required CI are known;
- the implementation surface is compact enough to define exact future allowed
  files;
- qsl-server integration is not required for bounded qsl-attachments
  service-local tests, provided the future implementation does not change
  qsl-server routing, timing, retry, public ingress, or relay storage behavior;
- backup, deploy, and public-claim gaps can be encoded as stop conditions for
  the future implementation harness rather than blocking authorization planning
  now.

This decision is not production implementation proof. It does not mean
attachment size, timing metadata, traffic shape, or metadata is hidden.

## Future qsl-attachments implementation authorization bundle

Required future bundle:

| Field | Future requirement |
| --- | --- |
| repository | `QuantumShieldLabs/qsl-attachments` |
| local path | `/srv/qbuild/work/NA-0237D/qsl-attachments`, or a fresh qbuild worktree explicitly named by the later directive |
| base branch | `main` |
| base SHA | refreshed live qsl-attachments `main`; NA-0343 observed `320be68fe632f8fe797bee78ae24f95c8d788905` |
| future qsl-attachments branch | `na-0344-production-size-class-harness` |
| future qsl-protocol companion branch | `na-0344-qsl-attachments-size-class-governance-companion` |
| qsl-attachments PR strategy | open qsl-attachments implementation PR first so code/test evidence exists before qsl-protocol companion closeout records it |
| qsl-protocol companion strategy | prepare a qsl-protocol governance PR that records the qsl-attachments PR/head/CI evidence and keeps public claims bounded |
| merge strategy | no direct push, no squash, no rebase, no admin bypass, no delete-branch flag; merge commits only after required checks pass |
| post-merge verification | qsl-attachments `main` `rust` green, qsl-protocol `public-safety` required/green, queue/decisions sane |
| qsl-server boundary | no qsl-server mutation unless a later directive selects an integration lane |
| deploy boundary | no production deployment unless separately authorized |
| rollback boundary | rollback is source/PR revert or policy disable only; no live data migration without separate authorization |
| secrets/env boundary | no secret-dependent tests; invalid env/config must fail closed; no raw capability or token artifacts |
| backup boundary | local source/artifacts must remain under current backup scope or the backup plan must be updated before mutation/deploy evidence is captured elsewhere |

## Future qsl-protocol companion governance bundle

Allowed qsl-protocol companion paths for the future NA-0344 lane:

- `docs/governance/evidence/NA-0344_metadata_runtime_qsl_attachments_production_size_class_implementation_harness.md`;
- `tests/NA-0344_metadata_runtime_qsl_attachments_production_size_class_implementation_harness_testplan.md`;
- `DECISIONS.md`;
- `TRACEABILITY.md`;
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`;
- `NEXT_ACTIONS.md` only for closeout/restoring the next successor.

Forbidden qsl-protocol companion paths unless a later directive explicitly
changes scope:

- qsl-protocol runtime paths;
- qshield runtime paths;
- qsc/qsp/protocol/crypto/key-schedule paths;
- `Cargo.toml`;
- `Cargo.lock`;
- `.github/**`;
- `README.md`;
- `START_HERE.md`;
- `docs/public/**`;
- website/external website;
- qsc-desktop;
- formal, inputs, scripts, tools/refimpl, app runtime, qsl-server, or
  qsl-attachments vendored paths inside qsl-protocol.

## Future qsl-attachments allowed/forbidden files

Exact allowed qsl-attachments implementation files for the future NA-0344
implementation harness:

- `src/lib.rs` for size-class policy, validation, storage lifecycle, reject,
  fetch, retention/purge, and no-accepted-state behavior;
- `src/main.rs` only if an operator-safe startup/config summary must expose a
  new non-secret policy/config value;
- `tests/production_size_class_policy.rs` for new focused size-class harness
  coverage;
- `tests/service_contract.rs` for existing API contract regression coverage;
- `tests/retention_cleanup_recovery.rs` for expiry/purge recovery coverage;
- `tests/retention_cleanup_logging.rs` for redacted retention/purge logging
  coverage;
- `tests/disk_pressure_quota_abuse.rs` for quota/abuse/storage-amplification
  coverage;
- `tests/disk_pressure_quota_logging.rs` for redacted quota/abuse logging
  coverage;
- `tests/backup_restore_recovery.rs` for cold full-root backup/restore and
  startup reconciliation coverage;
- `tests/backup_restore_logging.rs` for backup/recovery logging coverage;
- `tests/support/mod.rs` for reusable local test helpers only;
- qsl-attachments `DECISIONS.md`, `TRACEABILITY.md`, and `NEXT_ACTIONS.md`
  only if the future qsl-attachments directive requires repo-local governance;
- `tests/NA-0344_production_size_class_implementation_harness_evidence.md`
  only if the future qsl-attachments directive requires repo-local evidence.

Forbidden qsl-attachments files unless a later exact directive explicitly
authorizes them:

- `Cargo.toml`;
- `Cargo.lock`;
- `.github/**`;
- dependency updates;
- workflow changes;
- deployment automation;
- secret material or secret-dependent test fixtures;
- branch-protection configuration;
- public website or public-copy files;
- `README.md`;
- `START_HERE.md`;
- broad docs changes unrelated to the exact implementation evidence;
- unrelated refactors;
- protocol/crypto/key-schedule code if any appears in a future layout;
- qsl-server, qshield, qsc, qsp, qsl-protocol runtime, qsc-desktop, website,
  or external repository paths.

If future source inspection shows that correct implementation requires files
outside the allowed list, NA-0344 must stop and select a blocker or integration
successor rather than widening scope silently.

## Future build/test/CI/lint/format requirements

Required qsl-attachments local commands for a future implementation PR:

- `cargo fmt --all -- --check`;
- `cargo clippy --all-targets -- -D warnings`;
- `cargo build --locked`;
- `cargo test --locked`.

Required qsl-attachments remote CI:

- workflow: `rust`;
- required context/job: `rust`;
- required outcome: completed success on the validated PR head and on post-merge
  `main`.

Required qsl-protocol companion validation:

- `python3 scripts/ci/qsl_evidence_helper.py queue`;
- `python3 scripts/ci/qsl_evidence_helper.py decisions`;
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main`
  with exact allowed paths;
- `python3 scripts/ci/qsl_evidence_helper.py link-check`;
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`;
- PR body preflight / goal-lint with standalone `Goals: G1, G2, G3, G4, G5`;
- `cargo audit --deny warnings`;
- `cargo tree -i rustls-webpki --locked`;
- `cargo fmt --check`;
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`;
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`;
- `python3 formal/run_model_checks.py`;
- directly runnable metadata/qshield/qsc harnesses relevant to the changed
  governance claim set;
- qsl-protocol `public-safety` required and green.

## Future implementation semantics and marker plan

Future policy name:

- `qsl_attachments_production_size_class_v1`.

Future policy posture:

- disabled by default for deployment unless a later deployment directive
  explicitly enables it;
- tests may enable it through in-memory `Config` construction or a non-secret
  env/config value named by the future directive;
- invalid config must reject before service start or before accepted session
  state.

Candidate deterministic production size-class table:

- preserve qshield demo-compatible small classes:
  `256, 512, 768, 1024, 1536, 2048, 3072, 4096, 5120, 6144, 7168, 8192`;
- then use 8 KiB increments from 16 KiB through 1 MiB;
- then use 1 MiB increments from 2 MiB through the configured max object
  class;
- default configured max object class must not exceed the existing
  qsl-attachments `101 MiB` ciphertext ceiling unless a later directive
  explicitly authorizes the storage/cost/backup impact.

Candidate maximum overhead:

- at most 1023 bytes for objects mapped inside the qshield demo-compatible
  small-class table;
- at most 8191 bytes for objects above 8192 bytes through 1 MiB;
- at most 1 MiB minus 1 byte for objects above 1 MiB;
- zero overhead for exact class-boundary objects.

Required fail-closed behavior:

- invalid policy/config rejection before accepted session/object state;
- oversize rejection before session creation, part staging, object promotion, or
  output;
- malformed session descriptor/body rejection before accepted state;
- malformed stored object rejection during startup reconciliation;
- commit mismatch rejection before object promotion;
- failed upload cleanup preserves no successful object state;
- failed fetch does not mutate object bytes, capabilities, or retention state;
- expiry/purge removes padded object bytes and clears fetch capability material
  under the existing fail-closed retention model;
- no accepted state/output on reject.

Candidate NA-0344 markers:

- `NA0344_QSL_ATTACHMENTS_IMPLEMENTATION_AUTHORIZATION_OK`;
- `NA0344_QSL_ATTACHMENTS_SOURCE_FRESHNESS_OK`;
- `NA0344_QSL_ATTACHMENTS_AUTHORITY_OK`;
- `NA0344_QSL_ATTACHMENTS_CI_AUTHORITY_OK`;
- `NA0344_SIZE_CLASS_POLICY_OK`;
- `NA0344_VALID_SMALL_OBJECT_OK`;
- `NA0344_VALID_MEDIUM_OBJECT_OK`;
- `NA0344_VALID_LARGE_OBJECT_OK`;
- `NA0344_OVERSIZE_REJECT_OK`;
- `NA0344_MALFORMED_OBJECT_REJECT_OK`;
- `NA0344_RETENTION_PURGE_BOUNDARY_OK`;
- `NA0344_BACKUP_BOUNDARY_OK`;
- `NA0344_NO_SECRET_ARTIFACT_OK`;
- `NA0344_QSL_SERVER_BOUNDARY_OK`;
- `NA0344_QSHIELD_DEMO_COMPATIBILITY_OK`;
- `NA0344_NO_SIZE_HIDDEN_CLAIM_OK`;
- `NA0344_NO_TIMING_HIDDEN_CLAIM_OK`;
- `NA0344_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK`;
- `NA0344_NO_METADATA_FREE_CLAIM_OK`.

## Storage/retention/purge/backup/secrets/deploy/rollback boundary

Current qsl-attachments lifecycle:

- sessions live under `sessions/<session_id>/session.json`;
- staged parts live under `sessions/<session_id>/parts/<part_index>.part`;
- committed object metadata lives under `objects/<locator_ref>/object.json`;
- committed object bytes live under `objects/<locator_ref>/ciphertext.bin`;
- startup reconciliation keeps only coherent open sessions and committed
  objects, and discards incoherent/orphaned local artifacts fail-closed;
- expiration marks sessions/objects expired, clears capability hashes, removes
  staged or committed bytes, and emits redacted audit events.

Future requirements:

- object-size class state must remain inside the existing session/object
  lifecycle or stop if new durable state files are needed;
- if metadata fields are added to `session.json` or `object.json`, startup
  reconciliation must reject malformed or inconsistent values fail-closed;
- if object bytes are padded, `ciphertext.bin` must contain exactly the stored
  opaque bytes for the committed class and retrieval must return exactly those
  bytes unless a separate qsl-protocol/control-plane directive authorizes a
  different contract;
- no plaintext attachment bytes, decrypt context, raw resume token, raw fetch
  capability, route token, auth header, or long stable identifier may appear in
  passive logs or committed evidence;
- retention duration remains governed by existing short/standard/extended TTLs
  unless a later directive authorizes a policy change;
- purge triggers remain expiry, abort, failed/incoherent recovery cleanup, and
  explicit operator-directed cleanup under existing contracts unless a later
  directive authorizes more;
- backup support remains cold/quiesced full storage-root plus matching service
  configuration;
- hot/live backup, partial restore, multi-node storage, and cross-file
  transactional durability remain unsupported;
- no deployment, production enablement, or public-internet exposure is
  authorized by NA-0344 unless a later directive explicitly says so;
- rollback is either code revert before deployment or disabling the policy
  before live data migration; existing committed objects must remain
  retrievable or fail closed with explicit evidence.

Unknowns marked `REQUIRED_BEFORE_DEPLOYMENT`:

- production retention window for padded objects;
- production backup capacity and restore-time growth from size classes;
- production monitoring and alert thresholds for storage amplification;
- production abuse/cost threshold for maximum-class objects;
- production migration plan for pre-policy objects;
- operator runbook for enabling, disabling, and reverting the policy.

These unknowns do not block a bounded qsl-attachments implementation harness
that performs no deployment and makes no public-readiness claim. They do block
deployment or stronger claims.

## qsl-server integration boundary

Bounded qsl-attachments size-class implementation and tests can be performed
without qsl-server changes if all of the following remain true:

- qsl-attachments API routes remain unchanged;
- qsl-server remains transport-only and source-untouched;
- no qsl-server routing, queueing, retry, batching, cover traffic, timing,
  storage, public ingress, or compatibility behavior is changed;
- qsl-attachments tests exercise service-local HTTP handlers and storage
  lifecycle without requiring the live relay;
- no public claim is made that qsl-server timing metadata or traffic shape is
  hidden.

If future implementation requires qsl-server timing, storage, routing, retry,
or public-ingress changes, NA-0344 must stop and select:

`NA-0344 -- Metadata Runtime qsl-server Integration Boundary Plan`

qshield demo attachment size-class evidence remains reference/oracle evidence
for deterministic table shape and fail-closed handling only. It is not
production proof.

## qshield demo compatibility boundary

The future qsl-attachments harness should preserve compatibility with the
qshield demo oracle by:

- using the same small-class table through 8192 bytes unless later evidence
  justifies a different production small-object table;
- preserving deterministic invalid config, oversize, malformed object, and
  no-accepted-state reject behavior;
- preserving explicit retention/purge and artifact-safety proof;
- keeping qshield demo descriptor/ciphertext semantics out of qsl-attachments
  production claims.

It must not present qshield embedded relay/demo evidence as qsl-attachments or
qsl-server production proof.

## Public claim / external-review / production boundary

External review remains not complete.

The future qsl-attachments size-class work is review-sensitive. Until
implementation evidence, service evidence, deployment evidence, and review
evidence all exist, public and governance language must continue to state:

- no claim that attachment size is hidden;
- no claim that timing metadata is hidden;
- no claim that traffic shape is hidden;
- no claim that all metadata is hidden;
- no anonymity, metadata-free, or untraceable claim;
- no production-readiness or public-internet-readiness claim;
- no external-review-complete claim;
- no stronger public copy or website update without a separate directive.

Any stronger future claim requires implementation evidence, service evidence,
deployment evidence, qsl-server/qsc integration evidence if applicable, backup
evidence if durable production artifacts are involved, and external review
evidence.

## Selected successor

Selected successor:

`NA-0344 -- Metadata Runtime qsl-attachments Production Size-Class Implementation Harness`

Rationale:

- prerequisites did not regress;
- qsl-attachments source is fresh and authority/CI are complete enough for a
  later exact implementation directive;
- bounded service-local implementation can be tested without qsl-server changes;
- backup/deploy/public-review unknowns can be enforced as stop conditions
  because NA-0344 must not deploy or overclaim.

Rejected alternatives:

- `NA-0344 -- Metadata Runtime qsl-attachments Production Size-Class
  Implementation Blocker Resolution`: rejected because source freshness,
  mutation authority, and CI authority are complete enough for an implementation
  harness directive.
- `NA-0344 -- Metadata Runtime qsl-attachments Backup / Retention Prerequisite
  Plan`: rejected as immediate successor because backup/retention unknowns
  block deployment and stronger claims, not a no-deploy implementation harness
  with explicit stop conditions.
- `NA-0344 -- Metadata Runtime qsl-server Integration Boundary Plan`: rejected
  as immediate successor because bounded qsl-attachments service-local tests do
  not require qsl-server mutation.
- `NA-0344 -- Metadata Runtime External Review Readiness Gap Audit`: rejected
  as immediate successor because review remains needed before stronger public
  claims, but implementation harness evidence is the narrower next artifact to
  make review meaningful.

## Backup-plan impact statement

NA-0343 changes only qsl-protocol governance/evidence/testplan/decision/
traceability/journal paths under `/srv/qbuild/work`, which is already inside
the existing qbuild work area. No new durable evidence location outside current
backup scope is required by this lane.

No backup-plan update is required for NA-0343.

Future qsl-attachments implementation work must re-check backup scope before
mutation. A backup-plan update is required before any future lane captures
non-rebuildable artifacts or production storage evidence outside current
backup-covered paths, or before any live production storage path such as a
service data root is relied on for durable evidence.

## Stop conditions for NA-0344

NA-0344 must stop if any of the following occur:

- refreshed qsl-attachments source no longer matches the authorized base;
- qsl-attachments viewer permission, branch protection, or required `rust`
  check cannot be proven;
- the qsl-attachments worktree is dirty before mutation;
- required changes touch files outside the authorized list;
- implementation would weaken capability, auth, storage, retention, purge,
  quota, abuse, secret, or fail-closed behavior;
- implementation requires qsl-server integration not authorized by the future
  directive;
- implementation requires qsl-protocol canonical protocol/wire semantics not
  authorized by the future directive;
- tests write durable artifacts outside allowed local test directories;
- secrets, raw capabilities, route tokens, plaintext attachment bytes, or long
  stable identifiers appear in logs/artifacts;
- required qsl-attachments `rust` CI fails after the bounded retry budget;
- any language implies attachment-size-hidden, timing-hidden,
  traffic-shape-hidden, metadata-free, anonymity, untraceable behavior,
  production readiness, public-internet readiness, or external review
  completion.

## Next recommendation

Close NA-0343 after this governance PR merges and post-merge public-safety is
green, then restore the selected successor:

`NA-0344 -- Metadata Runtime qsl-attachments Production Size-Class Implementation Harness`
