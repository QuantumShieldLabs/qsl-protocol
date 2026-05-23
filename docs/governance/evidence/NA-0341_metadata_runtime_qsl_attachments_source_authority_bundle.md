Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-23

# NA-0341 Metadata Runtime qsl-attachments Source Authority Bundle

## Executive summary

NA-0341 is a qsl-protocol governance/evidence lane. It does not authorize or
make any qsl-attachments, qsl-server, qshield runtime, qsc/qsp/protocol,
crypto, workflow, dependency, service, website, README, START_HERE, or public
copy change.

Result: `PARTIAL_SOURCE_AUTHORITY`.

The current local source inventory identifies the strongest local
qsl-attachments source as:

- local path: `/srv/qbuild/work/NA-0237D/qsl-attachments`;
- remote URL: `https://github.com/QuantumShieldLabs/qsl-attachments.git`;
- observed HEAD: `320be68fe632`;
- observed ref state: detached `HEAD`, clean worktree, local `origin/main`
  also at `320be68fe632`;
- source freshness caveat: latest remote freshness was not proven because this
  directive prohibited qsl-attachments fetch/clone/remote inspection;
- authority caveat: qsl-attachments mutation, branch creation, PR creation, and
  merge authority were not proven because this directive authorized
  qsl-protocol mutation only.

The source is sufficient to map likely future qsl-attachments implementation
surfaces and CI entrypoints, but it is not sufficient to authorize production
size-class implementation. The selected successor is:

`NA-0342 -- Metadata Runtime qsl-attachments Source / Authority Blocker Resolution`

## Live NA-0341 scope

The live `NEXT_ACTIONS.md` entry on `origin/main` was:

- `NA-0341 -- Metadata Runtime qsl-attachments Source / Authority Bundle`;
- Status: READY;
- Goals: G1, G2, G3, G4, G5;
- Objective: execute the source/authority bundle lane selected by NA-0340
  before any qsl-attachments production size-class implementation
  authorization or mutation.

The live scope requires:

- exact qsl-attachments repository URL/path, branch/ref, base SHA, source
  freshness, and clean-worktree proof;
- exact merge authority and cross-repo PR permission status;
- exact allowed and forbidden future qsl-attachments files plus build, test,
  and CI entrypoints;
- storage, retention, purge, backup, deploy, rollback, qsl-server integration,
  secret/env, abuse/DoS, and public-claim boundaries;
- exact successor selection.

The live scope forbids unsupported production, public-internet,
external-review, anonymity, unsupported metadata-free, unsupported untraceable,
attachment-size-hidden, timing-hidden, traffic-hidden, qsl-server,
qsl-attachments, qshield, qsc/qsp/protocol/crypto/key-schedule, dependency,
workflow, website, README, START_HERE, docs/public, branch-protection, and
public-safety configuration changes unless a future exact directive authorizes
them.

## Inherited NA-0340 source-authority result

NA-0340 recorded a qsl-protocol-only cross-repo authorization/source-authority
plan for future qsl-attachments production size-class work. It found local
qsl-attachments sources but concluded that current source freshness, mutation
authority, exact allowed files, CI, deploy/rollback, qsl-server integration,
backup, and public-claim boundaries still required a future source/authority
bundle before mutation.

NA-0340 selected `NA-0341 -- Metadata Runtime qsl-attachments Source /
Authority Bundle` and preserved these boundaries:

- qsl-attachments production object-size padding remains unimplemented and
  unproven;
- qsl-server production timing/storage behavior remains unimplemented and
  unproven;
- qshield embedded relay/demo evidence is reference evidence only, not
  qsl-server or qsl-attachments production proof.

## Inherited NA-0339 qshield demo attachment size-class proof

NA-0339 implemented only bounded qshield embedded relay/demo attachment
size-class behavior:

- policy: `qshield_demo_attachment_size_class_v1`;
- opt-in env: `QSHIELD_DEMO_ATTACHMENT_SIZE_CLASSES=expanded`;
- deterministic demo table: `256, 512, 768, 1024, 1536, 2048, 3072, 4096,
  5120, 6144, 7168, 8192`;
- max padded qshield demo attachment ciphertext object: `8192` bytes;
- max overhead: `1023` bytes;
- invalid config, oversize object, malformed descriptor, and malformed
  ciphertext reject before accepted local state/output and before remote
  ack/delete.

That proof remains a qshield embedded relay/demo oracle only. It does not prove
qsl-attachments production upload, fetch, retention, purge, backup, deployment,
public-internet behavior, qsl-server timing/storage behavior, or any stronger
privacy/readiness claim.

## Sources inspected

qsl-protocol sources inspected:

- `NEXT_ACTIONS.md`;
- `DECISIONS.md`;
- `TRACEABILITY.md`;
- `tests/NA-0340_closeout_restore_na0341_testplan.md`;
- `docs/governance/evidence/NA-0340_metadata_runtime_qsl_attachments_production_size_class_cross_repo_authorization.md`;
- `tests/NA-0340_metadata_runtime_qsl_attachments_production_size_class_cross_repo_authorization_testplan.md`;
- `docs/governance/evidence/NA-0339_metadata_runtime_qshield_demo_attachment_size_class_harness.md`;
- `docs/governance/evidence/NA-0338_metadata_runtime_attachment_size_class_authorization.md`;
- relevant canonical/design qsl-attachments and metadata-runtime references in
  qsl-protocol.

Read-only qsl-attachments sources found under `/srv/qbuild/work`:

- `/srv/qbuild/work/NA-0237C/qsl-attachments` at `1e1ae272a4cb`;
- `/srv/qbuild/work/NA-0237D/qsl-attachments` detached at `320be68fe632`;
- `/srv/qbuild/work/NA-0237/qsl-attachments` at `1e1ae272a4cb`;
- `/srv/qbuild/work/NA-0237A/qsl-attachments` at `1e1ae272a4cb`;
- `/srv/qbuild/work/NA-0237B/qsl-attachments` at `1e1ae272a4cb`.

No qsl-attachments checkout was fetched, cloned, checked out, branched,
committed, pushed, merged, rebased, dependency-installed, or mutated.

Search coverage included qsl-attachments, attachment, object size,
object-size, size class, size-class, padding, bucket, upload, fetch, object,
descriptor, ciphertext, storage, retention, purge, backup, quota, abuse, DoS,
production, public internet, deploy, rollback, secret, env, CI, test,
metadata-free, anonymity, untraceable, size hidden, timing hidden, traffic
hidden, FUTURE_GATE, and NOT_READY.

## qsl-attachments local source discovery

The selected authoritative local source for future planning is
`/srv/qbuild/work/NA-0237D/qsl-attachments`, because it is the only observed
checkout whose local `origin/main` and detached `HEAD` both point at
`320be68fe632`. The other four local checkouts are clean historical `main`
checkouts at `1e1ae272a4cb`.

Selected local source facts:

- repository name: `qsl-attachments`;
- fetch remote: `https://github.com/QuantumShieldLabs/qsl-attachments.git`;
- local mirror remote: `/srv/qbuild/mirrors/qsl-attachments.git`;
- push URL shown by local config: `https://github.com/QuantumShieldLabs/qsl-attachments.git`;
- branch/ref: detached `HEAD`;
- HEAD SHA: `320be68fe632`;
- clean worktree: yes;
- local `origin/main`: present at `320be68fe632`;
- local `main`: present at older `1e1ae272a4cb`;
- no latest remote proof: qsl-attachments fetch and remote inspection were out
  of scope.

Local source top-level shape:

- Rust package `qsl-attachments`;
- `.github/workflows/rust.yml`;
- `Cargo.toml`, `Cargo.lock`;
- `src/lib.rs`, `src/main.rs`;
- repo-local docs under `docs/`;
- repo-local tests and evidence under `tests/`.

## Source/authority bundle matrix

| Field | NA-0341 result |
|---|---|
| local path | `/srv/qbuild/work/NA-0237D/qsl-attachments` |
| exists | yes |
| repository name | `qsl-attachments` |
| remote URL(s) | `https://github.com/QuantumShieldLabs/qsl-attachments.git`; local mirror `/srv/qbuild/mirrors/qsl-attachments.git` |
| current HEAD SHA | `320be68fe632` |
| current branch/ref | detached `HEAD` |
| detached | yes |
| local worktree clean | yes |
| origin/main or equivalent ref present | yes, local `origin/main` at `320be68fe632` |
| branch freshness known | no; latest remote freshness not proven without fetch/remote inspection |
| latest remote known without fetching | no |
| PR/merge authority known | no |
| cross-repo mutation authority known | no |
| future branch naming rule | must be named by a future qsl-attachments directive, likely `na-0342-*` or equivalent exact successor branch |
| future base SHA requirement | must use an exact qsl-attachments base branch/ref and full base SHA proven fresh enough in the future directive |
| future allowed files known | partial; likely `src/lib.rs`, `src/main.rs` only if needed, focused `tests/*.rs`, focused repo-local docs/evidence |
| future forbidden files known | yes by default: manifests/dependencies, workflows, deployment/secrets, unrelated refactors, qsl-server, qsl-protocol runtime, qshield runtime, qsc/qsp/protocol/crypto/key schedule, website/public copy unless exact future scope authorizes |
| build/test commands known | partial; local workflow defines `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`, `cargo build --locked`, `cargo test --locked` |
| CI entrypoints known | partial; `.github/workflows/rust.yml` has required `rust` job in local source, but protected context/current remote requirement not proven |
| secrets/env required | partial; local code reads `QATT_*` config env and uses `RUST_LOG`; no live secret values inspected |
| storage/retention/purge model known | partial; local source documents and implements single-node local-disk sessions/objects, TTLs, expiry sweep, cold full-root backup boundary, and startup reconciliation |
| deploy/rollback boundary known | partial; repo-local docs describe reference deployment/update path, but no future deployment authority is proven |
| backup impact known | partial; qsl-protocol changes stay under `/srv/qbuild/work`; future qsl-attachments production artifacts need explicit backup-scope proof |
| qsl-server integration requirement known | partial; future work may not need qsl-server if endpoint semantics stay unchanged, but any timing/routing/retry/public-ingress effect requires a separate qsl-server authorization |
| public-claim boundary known | yes; no stronger public/privacy/readiness claim is authorized |
| conclusion | `PARTIAL_SOURCE_AUTHORITY` |

## Future authorization requirements

Any future qsl-attachments implementation authorization must include all of the
following before mutation:

- exact repository URL/path;
- exact local path;
- exact base branch/ref;
- exact base SHA;
- proof the branch/ref is current enough;
- proof the qsl-attachments worktree is clean before edits;
- explicit qsl-attachments mutation authority and PR/merge authority;
- exact allowed files;
- exact forbidden files;
- build command;
- test command;
- CI command/context;
- lint/format command;
- storage/object model;
- descriptor/ciphertext/object lifecycle;
- size-class table and cap;
- migration requirement for existing objects and descriptors;
- retention/purge requirement;
- backup-plan requirement;
- rollback plan;
- production deploy/non-deploy boundary;
- qsl-server integration boundary;
- qshield demo compatibility boundary;
- secrets/env handling;
- public-claim boundary;
- external-review recommendation;
- branch/PR policy;
- verification bundle requirements;
- stop conditions for source freshness, authority, CI, fail-closed semantics,
  backup-plan gaps, and overclaim pressure.

NA-0341 does not authorize qsl-attachments implementation or mutation. A future
implementation directive must explicitly authorize qsl-attachments mutation and
name the exact source/base/allowed files. A future implementation directive must
include a qsl-protocol governance PR and a qsl-attachments PR unless it stops on
an exact blocker.

## Future allowed/forbidden file map

Probable future upload path files:

- `src/lib.rs`: `build_router`, create-session handler, upload-part handler,
  `CreateSessionRequest`, `UploadPartResponse`, `SessionMeta`,
  `validate_create_session_request`, `expected_part_length`, session storage.

Probable future fetch path files:

- `src/lib.rs`: fetch-object handler, `CommitResponse`, `ObjectMeta`,
  `validate_fetch_capability`, range handling, object read path.

Probable object storage path files:

- `src/lib.rs`: `Storage`, `create_object`, `save_object`, `load_object`,
  `read_object_bytes`, `remove_object_bytes`, startup reconciliation,
  `object.json`, `ciphertext.bin`, session part files.

Probable descriptor/ciphertext files:

- qsl-attachments local source stores service metadata in `ObjectMeta` and
  `SessionMeta`; qsl-protocol/qsc owns encrypted descriptor creation and
  client-side ciphertext semantics. A future directive must name whether the
  service changes padded ciphertext storage only, descriptor fields, or both.

Probable tests:

- `tests/service_contract.rs`;
- `tests/retention_cleanup_recovery.rs`;
- `tests/retention_cleanup_logging.rs`;
- focused new tests for size-class create/upload/commit/fetch/reject,
  retention, backup/recovery, quota, and compatibility.

Probable docs/evidence files:

- focused qsl-attachments repo-local docs/evidence if future directive
  authorizes them;
- qsl-protocol governance/evidence and testplan in the paired qsl-protocol PR.

Probable config files:

- `src/lib.rs` `Config` and `Config::from_env` if a future `QATT_*`
  size-class policy is required;
- `src/main.rs` only if startup/operator-safe summary fields are required.

Forbidden by default:

- `Cargo.toml` and `Cargo.lock` unless exact future scope authorizes dependency
  or manifest change;
- `.github/**` unless exact future scope authorizes workflow changes;
- deployment units, live secrets, env files, host state, branch protection, and
  public-safety configuration;
- unrelated refactors;
- public website/public docs;
- qsl-server, qshield runtime, qsc/qsp/protocol/crypto/key schedule unless an
  exact future directive and Director approval authorize that surface.

Unknowns:

- exact production size-class table for qsl-attachments;
- whether qsl-attachments should store padded object bytes, original bytes plus
  padding metadata, or reject until qsc/qsl-protocol descriptor semantics are
  updated;
- whether existing committed objects need migration or versioned fetch behavior;
- whether qsl-server retry/timing/routing behavior changes are necessary;
- current qsl-attachments branch protection and required contexts.

## Build/test/CI/secrets/deploy/rollback discovery

Read-only local qsl-attachments discovery found:

- package manifest: `Cargo.toml` for Rust crate `qsl-attachments`;
- dependencies: `axum`, `tokio`, `serde`, `sha2`, `rand_core`, `tracing`, and
  supporting crates;
- workflow: `.github/workflows/rust.yml`;
- workflow commands: `cargo fmt --all -- --check`, `cargo clippy --all-targets
  -- -D warnings`, `cargo build --locked`, `cargo test --locked`;
- runtime binary: `src/main.rs`;
- runtime library and route handlers: `src/lib.rs`;
- runtime env: `QATT_STORAGE_ROOT`, `QATT_BIND_ADDR`,
  `QATT_MAX_CIPHERTEXT_BYTES`, `QATT_MAX_OPEN_SESSIONS`,
  `QATT_STORAGE_RESERVE_BYTES`, `QATT_SESSION_TTL_SECS`,
  `QATT_RETENTION_SHORT_SECS`, `QATT_RETENTION_STANDARD_SECS`,
  `QATT_RETENTION_EXTENDED_SECS`, `QATT_INVALID_SECRET_ATTEMPTS`,
  `QATT_INVALID_RANGE_ATTEMPTS`;
- logging env: `RUST_LOG` via tracing subscriber default env filter;
- docs/runbook: `docs/NA-0004_reference_deployment_runbook.md` describes a
  local release build, binary copy, systemd unit, Caddy reverse proxy, loopback
  bind, reference host update path, and service restart verification;
- durability docs: `docs/NA-0009_durability_recovery_contract.md` and README
  describe cold full-root backup/restore plus matching configuration as the
  supported backup shape, with hot/live backup and partial restore unsupported.

No qsl-attachments build/test was run in NA-0341 because dependency
installation, service tests, destructive storage tests, and mutation were out
of scope; future qsl-attachments validation must classify exact local commands
as safe before running them.

## Storage/retention/purge/backup/ops boundary

Future implementation prerequisites:

| Area | Requirement before implementation |
|---|---|
| object lifecycle | Define whether padded bytes, unpadded bytes, or both are persisted, and ensure fetch returns exactly the intended ciphertext form. |
| descriptor lifecycle | Define whether descriptor-visible lengths/classes change and where qsl-protocol/qsc remains authoritative. |
| ciphertext lifecycle | Preserve opaque ciphertext-only service posture; no plaintext handling on service surfaces. |
| size-class object lifecycle | Define deterministic class selection, padding validation/stripping, oversize reject, and malformed-object reject before accepted state. |
| retention duration | Preserve current retention classes unless exact future scope authorizes a migration. |
| purge trigger | Preserve TTL-driven expiry and deterministic cleanup; define padded-object cleanup. |
| stale object cleanup | Define cleanup for partially migrated or malformed padded objects. |
| failed upload cleanup | Prove rejected uploads do not leave accepted state or unbounded staged bytes. |
| failed fetch cleanup | Prove invalid fetch/range/capability attempts do not mutate committed object state. |
| backup inclusion/exclusion | Confirm padded objects and metadata are included in cold full-root backup or explicitly update backup plan. |
| log redaction | Preserve short handles and exclude resume/fetch capabilities, plaintext, and full stable identifiers. |
| artifact redaction | Keep CI/test artifacts secret-safe and avoid long identifier dumps. |
| monitoring | Define storage growth, reject, and latency markers before production deploy. |
| alert thresholds | Define storage floor, quota, purge lag, abuse counter, and error-rate thresholds. |
| operator runbook | Update or create a qsl-attachments runbook only under future explicit scope. |
| rollback | Define downgrade/revert behavior without corrupting existing committed objects. |
| migration/compatibility | Define legacy object read/fetch behavior and exact failure modes. |
| abuse/cost threshold | Define maximum padded object size, maximum overhead, quota impact, and egress impact. |
| qsl-server interaction | Keep qsl-server unchanged unless separate authorization proves timing/routing/retry/public-ingress changes are required. |

Fields not proven by NA-0341 remain `REQUIRED_BEFORE_IMPLEMENTATION`.

## Public claim / external-review / production boundary

External review remains not complete. Production qsl-attachments size-class
work is review-sensitive because it touches observable object size, storage
growth, fetch behavior, retention, backup, cost, and operator evidence.

No public claim should imply:

- attachment sizes are hidden;
- timing metadata is hidden;
- traffic shape is hidden;
- all metadata is hidden;
- metadata-free behavior;
- anonymity;
- untraceable behavior;
- production readiness;
- public-internet readiness;
- external review completion.

No website or public docs update is made by NA-0341. Any stronger claim requires
implementation evidence, service evidence, deployment evidence, and review
evidence. qshield embedded relay/demo evidence remains reference/oracle evidence
only for production work.

## Source/authority classification

Classification: `PARTIAL_SOURCE_AUTHORITY`.

Rationale:

- source exists locally and is clean;
- the strongest local source is identifiable at `320be68fe632`;
- source map, likely files, local docs, and local CI commands are discoverable;
- latest remote freshness is not proven;
- branch protection/current remote status is not proven;
- qsl-attachments mutation/PR/merge authority is not proven;
- no qsl-attachments build/test/run was authorized or executed;
- deployment, rollback, backup-plan, qsl-server integration, and production
  public-claim boundaries still require exact future authorization.

`COMPLETE_SOURCE_AUTHORITY` is rejected because freshness and mutation authority
are unresolved. `BLOCKED_SOURCE_AUTHORITY` is rejected because usable local
source and file/CI discovery exist. `DOCS_ONLY_PLANNING` is rejected because
NA-0341 performed source-backed read-only inspection rather than qsl-protocol
planning only.

## Selected successor

`NA-0342 -- Metadata Runtime qsl-attachments Source / Authority Blocker Resolution`

Rationale:

- implementation authorization would be premature until latest source freshness
  and mutation/merge authority are proven;
- test/CI discovery is partly known from local source, but protected current
  CI/branch state and authority are still unresolved;
- backup/retention and service timing remain important, but they depend on
  resolving exact qsl-attachments authority first;
- external review remains important, but there is not yet an authorized
  production implementation boundary to review.

## Rejected alternatives

| Alternative | Reason rejected |
|---|---|
| `NA-0342 -- Metadata Runtime qsl-attachments Production Size-Class Implementation Authorization Plan` | Premature until qsl-attachments latest source freshness and mutation/merge authority are proven. |
| `NA-0342 -- Metadata Runtime qsl-attachments Test / CI Discovery Plan` | Too narrow; CI is only one part of the unresolved source/authority blocker. |
| `NA-0342 -- Metadata Runtime qsl-attachments Backup / Retention Prerequisite Plan` | Important later, but authority/freshness must come first. |
| `NA-0342 -- Metadata Runtime Service Timing Cross-Repo Authorization` | qsl-server timing is downstream of the qsl-attachments source/authority blocker for this lane. |
| `NA-0342 -- Metadata Runtime External Review Readiness Gap Audit` | Review remains needed later, but exact source/authority and implementation boundary are prerequisite. |
| direct qsl-attachments implementation | Forbidden by NA-0341 and unsupported by authority evidence. |

## Backup-plan impact statement

No backup-plan update is required for NA-0341 because this patch changes only
qsl-protocol governance/evidence/testplan/journal paths under the existing
qbuild workspace. No new durable evidence root, response root, source root, or
non-rebuildable artifact location outside current backup scope is introduced.

Future qsl-attachments implementation work must confirm whether future
qsl-attachments source, production artifacts, test artifacts, deployment
evidence, or backup/restore proof live outside current backup scope. If they do,
the future directive must update the backup plan before mutation or durable
evidence capture.

## Next recommendation

Close NA-0341 after this evidence PR merges and public-safety is green, then
restore the selected `NA-0342 -- Metadata Runtime qsl-attachments Source /
Authority Blocker Resolution` successor. NA-0342 must prove exact source
freshness and qsl-attachments mutation/PR/merge authority, or stop with an exact
blocker. NA-0342 must not implement qsl-attachments production size-class
behavior unless a later directive explicitly authorizes that implementation.
