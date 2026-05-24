Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-23

# NA-0346 Metadata Runtime qsl-server Integration Implementation Authorization

## Executive summary

NA-0346 refreshes qsl-server source, authority, protection, CI, and open-PR
state after the NA-0345 qsl-server integration boundary plan. It then defines
the exact authorization bundle required for a future qsl-server integration
implementation lane.

Result: `IMPLEMENTATION_AUTHORIZATION_READY`.

The selected qsl-server source is `/srv/qbuild/work/NA-0237D/qsl-server` at
`3f28d7433e74`. Read-only live evidence showed remote `main` also at
`3f28d7433e74`, viewer permission `ADMIN`, protected `main`, strict required
check `rust`, no open PRs, and latest listed `main` CI success.

This qsl-protocol lane does not implement qsl-server, qsl-attachments, qshield
runtime, qsc/qsp/protocol behavior, deployment, public ingress, timing
mitigation, traffic-shape mitigation, or public-copy changes. The selected
successor is:

`NA-0347 -- Metadata Runtime qsl-server Integration Implementation Harness`

## Live NA-0346 scope

The live `NEXT_ACTIONS.md` entry on `origin/main` required NA-0346 to:

- refresh qsl-server source freshness, mutation authority, CI authority, branch
  protection, open PR state, and required checks from live remote;
- define the exact future qsl-server implementation authorization bundle,
  including allowed and forbidden files, branch/base, qsl-protocol companion
  scope, qsl-attachments integration contract, routing, storage, retention,
  purge, backup, deploy/rollback, secrets/env, monitoring/logging, and public
  ingress boundaries;
- decide whether qsl-server integration implementation can be authorized next
  or whether a precise blocker remains;
- preserve public-claim constraints and avoid stronger privacy or readiness
  language.

The live scope forbade:

- unsupported production, public-internet, external-review, anonymity,
  metadata-free, or untraceable claims;
- any claim that attachment size, timing metadata, or traffic shape is hidden
  unless exact future evidence proves it;
- presenting qsl-attachments service-local proof as qsl-server integration
  proof;
- presenting qshield embedded relay/demo proof as production proof;
- qsl-server mutation unless a future exact directive names repository, branch,
  base SHA, allowed files, CI, rollback, deploy, backup, qsl-attachments
  integration, secrets/env, and public-claim boundaries.

## Inherited NA-0345 boundary plan

NA-0345 recorded the qsl-server integration boundary after NA-0344 delivered
bounded qsl-attachments service-local size-class proof.

Inherited boundary facts:

- selected qsl-server source was `/srv/qbuild/work/NA-0237D/qsl-server` at
  `3f28d7433e74`;
- qsl-server source matched live remote `main`;
- qsl-server viewer permission was `ADMIN`;
- qsl-server `main` was protected with strict required check `rust`;
- qsl-server open PR list was empty at inspection time;
- latest listed qsl-server `main` CI was success;
- qsl-attachments PR #37 evidence remained service-local prerequisite proof,
  not qsl-server proof;
- qshield embedded relay/demo evidence remained reference/oracle evidence only;
- qsl-server production timing, storage, public-ingress, deployment,
  qsl-attachments handoff, monitoring, logging, and public-claim behavior
  remained unimplemented and unproven.

## Refreshed qsl-server source/authority proof

Read-only qsl-server evidence from `/srv/qbuild/work/NA-0237D/qsl-server`:

| Field | Result |
| --- | --- |
| repository | `QuantumShieldLabs/qsl-server` |
| URL | `https://github.com/QuantumShieldLabs/qsl-server` |
| selected local path | `/srv/qbuild/work/NA-0237D/qsl-server` |
| local branch | `main` |
| local HEAD | `3f28d7433e74` |
| local worktree | clean |
| remote default branch | `main` |
| remote HEAD / `main` | `3f28d7433e74` |
| source freshness | `FRESH_SOURCE` |
| viewer permission | `ADMIN` |
| branch protection | present |
| required check | strict `rust` |
| force pushes | disabled |
| deletions | disabled |
| admins enforced | enabled |
| open PRs | none listed |
| latest listed main CI | `ci` success on `3f28d7433e74` |
| mutation authority | `COMPLETE_MUTATION_AUTHORITY` |
| CI authority | `COMPLETE_CI_AUTHORITY` |
| final gate | `IMPLEMENTATION_AUTHORIZATION_READY` |

qsl-server workflow command evidence:

- `bash scripts/ci/test_aws_update_and_verify.sh`;
- `bash scripts/ci/test_update_checksum.sh`;
- `cargo fmt --all -- --check`;
- `cargo test -q`;
- `cargo clippy -q -- -D warnings`.

qsl-server source was not fetched, cloned, checked out, branched, committed,
pushed, merged, rebased, dependency-installed, built, tested, deployed, or
mutated by NA-0346.

## Sources inspected

qsl-protocol sources inspected:

- `NEXT_ACTIONS.md`;
- `DECISIONS.md`;
- `TRACEABILITY.md`;
- `tests/NA-0345_closeout_restore_na0346_testplan.md`;
- `docs/governance/evidence/NA-0345_metadata_runtime_qsl_server_integration_boundary_plan.md`;
- `tests/NA-0345_metadata_runtime_qsl_server_integration_boundary_testplan.md`;
- `docs/governance/evidence/NA-0344_metadata_runtime_qsl_attachments_production_size_class_implementation_harness.md`;
- `docs/governance/evidence/NA-0343_metadata_runtime_qsl_attachments_production_size_class_implementation_authorization.md`;
- `docs/governance/evidence/NA-0339_metadata_runtime_qshield_demo_attachment_size_class_harness.md`;
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

qsl-server sources inspected read-only:

- `.github/workflows/ci.yml`;
- `Cargo.toml`;
- `src/lib.rs`;
- `src/main.rs`;
- `README.md`;
- `docs/server/DOC-SRV-001_Deployment_Hardening_Contract_v1.0.0_DRAFT.md`;
- `docs/server/DOC-SRV-002_Systemd_Hardening_Plan_v1.0.0_DRAFT.md`;
- `docs/server/DOC-SRV-003_Relay_Inbox_Contract_v1.0.0_DRAFT.md`;
- `docs/server/DOC-SRV-004_Relay_Auth_And_Hardening_Contract_v1.0.0_DRAFT.md`;
- `packaging/runbook_ubuntu.md`;
- `packaging/systemd/relay.env.example`;
- `packaging/systemd/qsl-server.service`;
- current qsl-server tests for config semantics, auth reject logging, abuse and
  queue pressure, global route caps, route lifecycle TTL, idempotency semantics,
  and no-secret examples.

## Implementation authorization readiness decision

Decision: authorize the next lane as an implementation harness, not as
deployment and not as a public-claim lane.

Reasons:

- source freshness is current and remote-matched;
- mutation authority is sufficient for a future directive to create a normal
  qsl-server branch/PR;
- branch protection and required CI are known and enforced;
- latest listed qsl-server `main` CI is green;
- open PR state is empty, so no known qsl-server queue conflict blocks a future
  implementation branch;
- qsl-server already has bounded route/API, auth, queue, TTL, logging, deploy,
  and rollback surfaces that can host a narrow integration harness;
- qsl-attachments PR #37 can serve as prerequisite service-local evidence, but
  future qsl-server proof must be gathered in qsl-server itself.

This authorization does not claim qsl-server integration is already
implemented. It only proves the next exact lane can be selected if it repeats
the source/authority refresh before mutation.

## Future qsl-server implementation authorization bundle

| Field | Required future value |
| --- | --- |
| repository | `QuantumShieldLabs/qsl-server` |
| local path | `/srv/qbuild/work/NA-0237D/qsl-server`, or a fresh qbuild worktree named by the future directive |
| base branch | `main` |
| base SHA | refresh live remote `main`; NA-0346 observed `3f28d7433e74a41a91ef53c7200c3eb2fe7205e4` |
| qsl-server branch | `na-0347-qsl-server-integration-implementation-harness` |
| qsl-protocol companion branch | `na-0347-qsl-server-integration-governance-companion` |
| qsl-server PR strategy | qsl-server implementation/harness PR first, with required `rust` green before qsl-protocol companion records evidence |
| qsl-protocol PR strategy | companion governance PR after qsl-server evidence exists, preserving claim boundaries |
| merge strategy | normal merge commits only; no direct push, squash, rebase, admin bypass, or branch deletion command |
| post-merge verification | qsl-server `main` required `rust` green, qsl-protocol `public-safety` required/green, queue/decisions sane |
| deploy boundary | no production deployment unless separately authorized |
| rollback boundary | source revert or feature-disable/test-harness removal only unless a future directive authorizes deployment rollback |
| secrets/env boundary | no secret-dependent tests; no committed tokens; invalid env/config must fail closed |
| backup boundary | qsl-server source/artifacts must be under current backup scope or backup plan must be updated before mutation evidence leaves `/srv/qbuild/work` |
| qsl-attachments boundary | no qsl-attachments mutation unless a future directive separately authorizes it |
| qshield boundary | qshield embedded relay/demo evidence remains reference/oracle only |

## Future qsl-protocol companion governance bundle

Allowed qsl-protocol companion files for NA-0347:

- `docs/governance/evidence/NA-0347_metadata_runtime_qsl_server_integration_implementation_harness.md`;
- `tests/NA-0347_metadata_runtime_qsl_server_integration_implementation_harness_testplan.md`;
- `DECISIONS.md`;
- `TRACEABILITY.md`;
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`;
- `NEXT_ACTIONS.md` only for closeout/restoring the exact next successor after
  qsl-server evidence is merged and public-safety is green.

The qsl-protocol companion must not edit qsl-protocol runtime, qshield runtime,
qsc/qsp/protocol/crypto/key-schedule paths, Cargo files, workflows,
public-safety configuration, qsc-desktop, website, README, START_HERE,
docs/public, formal, inputs, or tool/runtime code unless a later directive
explicitly replaces this bundle.

## Future qsl-server allowed and forbidden files

Allowed qsl-server implementation/harness files for NA-0347, subject to a fresh
source/authority check:

- `src/lib.rs` only for the minimum route/API, opaque payload, storage/proxy,
  retention/TTL, rate/quota, logging/redaction, or qsl-attachments contract
  hooks required by the harness;
- `src/main.rs` only for minimum non-secret env/config parsing needed by the
  harness, with deterministic startup reject behavior;
- `tests/relay_smoke.rs` for canonical push/pull integration regression;
- `tests/config_semantics.rs` for env/config fail-closed proof;
- `tests/hardening_auth_reject_logging.rs` for auth, redaction, and
  no-mutation proof;
- `tests/abuse_rate_queue.rs` and `tests/abuse_rate_queue_logging.rs` for
  queue, rate, and redacted abuse/cost proof;
- `tests/rate_global_cap.rs` and `tests/rate_global_cap_logging.rs` for route
  cap proof;
- `tests/route_lifecycle_ttl.rs` and `tests/route_lifecycle_ttl_logging.rs`
  for retention/purge/TTL proof;
- one new exact qsl-server test file, if needed:
  `tests/qsl_attachments_integration_contract.rs`.

Allowed qsl-server technical docs only if needed to keep service-local contract
text aligned with implementation evidence:

- `docs/server/DOC-SRV-001_Deployment_Hardening_Contract_v1.0.0_DRAFT.md`;
- `docs/server/DOC-SRV-003_Relay_Inbox_Contract_v1.0.0_DRAFT.md`;
- `docs/server/DOC-SRV-004_Relay_Auth_And_Hardening_Contract_v1.0.0_DRAFT.md`.

Forbidden qsl-server files unless a later directive explicitly authorizes them:

- `Cargo.toml`;
- `Cargo.lock`;
- `.github/**`;
- release workflow or branch-protection settings;
- `README.md`, `START_HERE.md`, `NOTICE`, `LICENSE`, public/legal copy, or
  external website copy;
- `packaging/**`, `scripts/install_*`, `scripts/update_*`,
  `scripts/aws_update_and_verify.sh`, `scripts/verify_remote.sh`, or deployment
  automation, unless the future directive is specifically a deploy/rollback
  prerequisite lane;
- broad refactors, dependency changes, formatting-only churn, unrelated tests,
  or production deployment changes.

## Future build/test/CI/lint/format requirements

Future qsl-server local validation must run:

- `bash scripts/ci/test_aws_update_and_verify.sh`;
- `bash scripts/ci/test_update_checksum.sh`;
- `cargo fmt --all -- --check`;
- `cargo test -q`;
- `cargo clippy -q -- -D warnings`;
- any targeted qsl-server test added for NA-0347, by exact name;
- read-only qsl-server `gh run list`, branch-protection, and open-PR refresh;
- required qsl-server PR `rust` check before merge.

Future qsl-protocol companion validation must run:

- `cargo audit --deny warnings`;
- `cargo tree -i rustls-webpki --locked`;
- `cargo fmt --check`;
- queue and decision helpers;
- scope guard for allowed governance paths;
- markdown link check;
- leak scan over added lines;
- goal-lint / PR body preflight;
- qshield/demo metadata harnesses that remain directly runnable;
- qsc `send_commit`;
- formal model checks;
- metadata runtime plan, phase-2, conformance, and NA-0310/NA-0313 evidence
  checks when directly runnable.

## Future implementation semantics and marker plan

Future NA-0347 semantics must stay bounded to proving qsl-server can safely
carry or coordinate qsl-attachments-compatible opaque objects through the
current relay boundary. The default safe contract is opaque pass-through unless
the future directive explicitly authorizes qsl-attachments API calls.

Required NA-0347 markers:

- `NA0347_QSL_SERVER_SOURCE_AUTHORITY_OK`;
- `NA0347_QSL_SERVER_IMPLEMENTATION_AUTHORIZATION_OK`;
- `NA0347_QSL_ATTACHMENTS_CONTRACT_OK`;
- `NA0347_QSL_SERVER_ROUTE_BOUNDARY_OK`;
- `NA0347_QSL_SERVER_STORAGE_BOUNDARY_OK`;
- `NA0347_QSL_SERVER_QUOTA_BOUNDARY_OK`;
- `NA0347_QSL_SERVER_RETENTION_PURGE_BOUNDARY_OK`;
- `NA0347_QSL_SERVER_BACKUP_BOUNDARY_OK`;
- `NA0347_QSL_SERVER_SECRET_ENV_BOUNDARY_OK`;
- `NA0347_QSL_SERVER_DEPLOY_ROLLBACK_BOUNDARY_OK`;
- `NA0347_QSL_SERVER_PUBLIC_INGRESS_BOUNDARY_OK`;
- `NA0347_QSHIELD_DEMO_REFERENCE_BOUNDARY_OK`;
- `NA0347_QSL_ATTACHMENTS_SERVICE_LOCAL_BOUNDARY_OK`;
- `NA0347_NO_ATTACHMENT_SIZE_HIDDEN_CLAIM_OK`;
- `NA0347_NO_TIMING_HIDDEN_CLAIM_OK`;
- `NA0347_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK`;
- `NA0347_NO_METADATA_FREE_CLAIM_OK`.

Future blocker markers if the harness cannot proceed:

- `NA0347_BLOCKED_SOURCE_FRESHNESS`;
- `NA0347_BLOCKED_MUTATION_AUTHORITY`;
- `NA0347_BLOCKED_CI_AUTHORITY`;
- `NA0347_BLOCKED_QSL_ATTACHMENTS_CONTRACT`;
- `NA0347_BLOCKED_BACKUP_DEPLOY_ROLLBACK`;
- `NA0347_BLOCKED_SECRET_ENV`;
- `NA0347_BLOCKED_PUBLIC_INGRESS_CLAIM_BOUNDARY`.

## Storage/retention/purge/backup/secrets/deploy/rollback boundary

Future NA-0347 requirements:

- service lifecycle: qsl-server remains transport-only unless the future
  directive explicitly authorizes a new role;
- request lifecycle: canonical `POST /v1/push` and `GET /v1/pull?max=N` remain
  the known route/API surface unless the future directive authorizes a new API;
- qsl-attachments handoff lifecycle: default to opaque byte pass-through; any
  qsl-attachments API call requires an exact service contract and no
  qsl-attachments mutation unless separately authorized;
- storage/proxy lifecycle: prove whether qsl-server stays memory-only or
  introduces a proxy/storage handoff; any durable object storage is a stop
  unless separately authorized;
- retention duration: align with route drain/delete-on-empty and
  `ROUTE_IDLE_TTL_MS`, or stop with a new retention prerequisite;
- purge triggers: prove expired-route cleanup and failed handoff cleanup;
- stale cleanup: prove stale queued bytes are not delivered after expiry;
- failed upload/fetch cleanup: no accepted state after reject or partial
  failure;
- backup inclusion/exclusion: source/build artifacts stay under `/srv/qbuild`;
  any deployed config, `/opt/qsl-server`, `/etc/qsl-server`, or backup root
  evidence outside current backup scope requires backup-plan review;
- log redaction: route tokens, auth headers, payload bytes, and secret-bearing
  env values must not appear in logs or evidence;
- artifact redaction: PR artifacts, test logs, and response files must not
  include raw tokens, bearer values, or payload contents;
- monitoring and alerts: record safe counters/status only; no payload-derived
  metrics;
- operator runbook: future deployment runbook changes are forbidden unless
  separately authorized;
- rollback: source revert or feature disable only; production rollback is not
  part of NA-0347 unless separately authorized;
- migration/compatibility: no silent route/API fallback; compatibility behavior
  must be explicit and tested;
- abuse/cost threshold: route count, queue depth, body size, push burst, refill,
  TTL, and external rate controls must remain bounded;
- secrets/env variables: `RELAY_TOKEN` remains optional auth only; any new env
  value must be non-secret by default or explicitly secret-managed and omitted
  from logs;
- deployment boundary: no production deployment or public service mutation.

Unknown fields are `REQUIRED_BEFORE_IMPLEMENTATION` and become NA-0347 stop
conditions if not resolved by the future directive before mutation.

## qsl-attachments integration contract

NA-0347 may rely on qsl-attachments PR #37 only as prerequisite service-local
evidence. It must not present that proof as qsl-server proof.

Default qsl-server/qsl-attachments contract:

- qsl-server treats attachment payloads as opaque bytes;
- qsl-server does not parse qsl-attachments size-class metadata unless a future
  directive explicitly authorizes that behavior;
- qsl-server does not strip or rewrite qsl-attachments metadata by default;
- qsl-server does not call qsl-attachments APIs by default;
- qsl-server tests can use deterministic opaque fixtures shaped like
  qsl-attachments outputs, but those fixtures are not qsl-attachments runtime
  proof;
- qsl-server integration can be tested without production deployment by using
  loopback qsl-server tests and deterministic fixtures;
- qsl-server integration must not require secrets;
- if qsl-server needs qsl-attachments source changes, NA-0347 must stop or
  select a cross-repo blocker successor.

## qshield demo reference boundary

qshield embedded relay/demo evidence remains reference/oracle evidence only. It
may inform fixture shape, marker names, or compatibility checks, but it is not
qsl-server production proof and not qsl-attachments production proof.

## Public claim / external-review / production boundary

External review remains not complete. qsl-server integration is review-sensitive
because it touches route/API behavior, storage/proxy behavior, logs, retention,
timing visibility, public ingress, and attachment handoff assumptions.

NA-0347 and any public copy must not claim or imply:

- attachment size is hidden;
- timing metadata is hidden;
- traffic shape is hidden;
- all metadata is hidden;
- anonymity, metadata-free behavior, or untraceable behavior is achieved;
- production readiness, public-internet readiness, or external-review
  completion is achieved.

Any stronger claim requires implementation evidence, service evidence,
deployment evidence, monitoring/logging evidence, public-ingress evidence, and
external-review evidence. No website or docs/public update is authorized by
NA-0346.

## Selected successor

Selected successor:

`NA-0347 -- Metadata Runtime qsl-server Integration Implementation Harness`

Rationale: qsl-server source, mutation authority, and CI authority are complete
enough to select the implementation harness as the next exact lane, provided
the future directive repeats live source/authority checks before mutation and
keeps deploy/rollback/public-claim gates bounded.

## Rejected alternatives

- `NA-0347 -- Metadata Runtime qsl-server Integration Implementation Blocker
  Resolution`: rejected because no current source, authority, CI, or file-map
  blocker remains after this refresh.
- `NA-0347 -- Metadata Runtime qsl-server Source / Authority Blocker
  Resolution`: rejected because source freshness and authority are complete at
  this inspection point.
- `NA-0347 -- Metadata Runtime qsl-server Backup / Deploy / Rollback
  Prerequisite Plan`: rejected because NA-0347 can use an explicit no-deploy
  implementation harness with backup/deploy/rollback as stop conditions.
- `NA-0347 -- Metadata Runtime External Review Readiness Gap Audit`: rejected
  as premature before qsl-server integration harness evidence exists.
- `NA-0347 -- Metadata Runtime Website / Public Claim Boundary Audit`: rejected
  as premature before qsl-server integration harness evidence exists, with
  public claims already bounded by this plan.

## Backup-plan impact statement

No backup-plan update is required for NA-0346 because the durable changes are
limited to qsl-protocol governance, evidence, testplan, and journal files under
`/srv/qbuild/work`.

Future qsl-server work must review backup-plan coverage before mutating or
recording durable evidence outside `/srv/qbuild/work`, especially for deployed
config under `/etc/qsl-server`, deployed binaries under `/opt/qsl-server`,
release artifacts, or backup roots such as `/root/qsl-backups`.

## Next recommendation

Execute NA-0347 as a qsl-server implementation harness directive only if it
refreshes qsl-server source/authority/CI proof, names the exact branch/base SHA,
limits mutation to the allowed file map above, runs qsl-server `rust` locally and
in CI, opens qsl-server PR first, and records qsl-protocol companion evidence
only after qsl-server proof exists.
