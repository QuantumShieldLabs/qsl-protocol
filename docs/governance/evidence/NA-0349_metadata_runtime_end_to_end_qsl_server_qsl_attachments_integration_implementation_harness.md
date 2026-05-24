Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-24

# NA-0349 Metadata Runtime End-to-End qsl-server / qsl-attachments Integration Implementation Harness

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0349 implements the bounded qsl-server / qsl-attachments integration
implementation harness selected by NA-0348.

Result: `NA0349_METADATA_RUNTIME_END_TO_END_INTEGRATION_OK`.

qsl-server PR #56 merged as `d40e6003fdf0` from validated head
`9f51b5a691f`. The implementation changed only:

- `tests/na0349_end_to_end_integration_contract.rs`

The harness is qsl-server-local and deterministic. It models the
qsl-attachments create/commit/fetch/purge contract with in-test fixtures,
routes the resulting qsl-attachments descriptor through qsl-server, verifies
exact opaque descriptor delivery, verifies modeled attachment fetch and purge
behavior, and emits the NA-0349 marker set.

qsl-attachments was not changed. Its refreshed `main` remained
`96b9352bd63e`; local validation passed with `cargo fmt --all -- --check`,
`cargo clippy --all-targets -- -D warnings`, `cargo build --locked`, and
`cargo test --locked`.

This is executable integration harness evidence, not production deployment,
public-ingress, public-internet-readiness, external-review completion, or a
claim that attachment size, timing metadata, traffic shape, or all metadata is
hidden.

Selected successor:

`NA-0350 -- Metadata Runtime Production Backup / Deploy / Rollback Hardening Plan`

## Live NA-0349 scope

The live `NEXT_ACTIONS.md` entry requires NA-0349 to repeat qsl-server and
qsl-attachments source/authority/CI proof before implementation, then prove or
stop on cross-service handoff, route/API, object lifecycle, size-class flow,
opaque payload, route-token/auth, quota/rate, retention/purge, backup/restore,
log redaction, monitoring, deploy/rollback, public-ingress, qshield demo
reference, and public-claim boundaries.

Protected boundaries:

- no unsupported production, public-internet, external-review, anonymity,
  metadata-free, or untraceable claim;
- no claim that attachment size, timing metadata, traffic shape, or all
  metadata is hidden;
- qsl-server/qsl-attachments production boundary remains explicit;
- qsl-attachments PR #37 remains service-local prerequisite evidence unless
  this lane proves additional behavior;
- qsl-server PR #55 remains bounded qsl-server harness evidence unless this
  lane proves additional behavior;
- qshield embedded relay/demo proof remains reference/oracle evidence only;
- no runtime, protocol, crypto, qsc/qsp, dependency, workflow, website,
  README, START_HERE, docs/public, branch-protection, public-safety, or
  deployment behavior change.

## Inherited NA-0348 plan

NA-0348 defined the evidence gap matrix after qsl-server PR #55 and
qsl-attachments PR #37. It recommended a cross-repo integration test without
production code changes as the preferred starting shape, with deterministic
contract fixtures where direct service coupling would require dependencies,
secrets, workflow changes, or deployment.

NA-0349 follows that plan by adding a qsl-server-only executable contract
harness and validating qsl-attachments `main` separately.

## Source / authority refresh for both service repos

### qsl-server

| Field | Result |
| --- | --- |
| repository | `QuantumShieldLabs/qsl-server` |
| local path | `/srv/qbuild/work/NA-0237D/qsl-server` |
| refreshed local/head before branch | `b194a95b19a7` |
| remote `origin/main` before branch | `b194a95b19a7` |
| PR #55 | merged, merge `b194a95b19a7` |
| viewer permission | `ADMIN` |
| branch protection | present |
| required check | strict `rust` |
| force pushes | disabled |
| deletions | disabled |
| admins enforced | enabled |
| open PRs before branch | none listed |
| latest main CI before branch | `rust` success on `b194a95b19a7` |
| source classification | `FRESH_SOURCE` |
| authority classification | `COMPLETE_AUTHORITY` |
| CI classification | `COMPLETE_CI` |

### qsl-attachments

| Field | Result |
| --- | --- |
| repository | `QuantumShieldLabs/qsl-attachments` |
| local path | `/srv/qbuild/work/NA-0237D/qsl-attachments` |
| refreshed local/head | `96b9352bd63e` |
| remote `origin/main` | `96b9352bd63e` |
| PR #37 | merged, merge `96b9352bd63e` |
| viewer permission | `ADMIN` |
| branch protection | present |
| required check | strict `rust` |
| force pushes | disabled |
| deletions | disabled |
| admins enforced | not enabled on current protection |
| open PRs | none listed |
| latest main CI | `rust` success on `96b9352bd63e` |
| source classification | `FRESH_SOURCE` |
| authority classification | `COMPLETE_AUTHORITY` |
| CI classification | `COMPLETE_CI` |

## Implementation strategy selected

Selected strategy: qsl-server-only executable integration harness with a
deterministic qsl-attachments contract fixture.

Rationale:

- qsl-server and qsl-attachments do not currently share a repo-local test
  dependency, and adding one would require manifest or workflow changes outside
  NA-0349 scope;
- direct production deployment, public ingress, real secrets, or external
  service calls are not authorized;
- a qsl-server test can exercise the qsl-server route/API surface while
  modeling qsl-attachments object lifecycle, size class, fetch capability,
  backup/restore, retention/purge, audit/log, and public-claim boundaries;
- qsl-attachments `main` was validated separately using its existing executable
  service-local tests.

## Implementation PR branch/head/merge evidence

- Repository: `QuantumShieldLabs/qsl-server`
- PR: #56, `https://github.com/QuantumShieldLabs/qsl-server/pull/56`
- Branch: `na-0349-end-to-end-integration-harness`
- Head: `9f51b5a691f`
- Merge: `d40e6003fdf0`
- Merge method: normal merge commit with `--match-head-commit`
- Delete-branch flag: not used
- Required PR check: `rust` success
- Post-merge main check: `rust` success on `d40e6003fdf0`

qsl-attachments was not mutated; final source stayed at `96b9352bd63e`.

## Changed files

qsl-server:

- `tests/na0349_end_to_end_integration_contract.rs`

qsl-attachments:

- no changed files

qsl-protocol governance companion:

- this evidence file
- `tests/NA-0349_metadata_runtime_end_to_end_qsl_server_qsl_attachments_integration_implementation_harness_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Harness summary

The qsl-server harness:

- spawns qsl-server on a loopback ephemeral listener;
- commits a modeled qsl-attachments ciphertext object;
- produces a modeled `service_ref_v1` descriptor with visible size-class
  metadata and fetch capability;
- pushes the descriptor through `/v1/push` with route-token and bearer auth;
- pulls the descriptor from `/v1/pull` and verifies exact byte preservation;
- fetches the modeled qsl-attachments object using the delivered descriptor;
- rejects missing route-token, missing auth, oversized route payload, wrong
  route, expired/purged attachment object, and unqueued oversize data;
- snapshots/restores the modeled object store for cold backup boundary proof;
- scans captured qsl-server logs and modeled qsl-attachments audit events for
  route-token, auth-token, fetch-capability, payload, locator, and attachment
  sentinels;
- emits the required NA-0349 marker set.

## Source authority proof

Marker:

- `NA0349_END_TO_END_SOURCE_AUTHORITY_OK`

The marker is backed by refreshed qsl-server/qsl-attachments source, authority,
protection, open-PR, and CI proof before mutation.

## qsl-server main proof

Marker:

- `NA0349_QSL_SERVER_MAIN_PROOF_OK`

qsl-server PR #56 merged and post-merge `main` `rust` completed success on
`d40e6003fdf0`.

## qsl-attachments main proof

Marker:

- `NA0349_QSL_ATTACHMENTS_MAIN_PROOF_OK`

qsl-attachments remained unchanged at `96b9352bd63e`; local validation passed
with format, clippy, build, and locked test commands, including the
`production_size_class_policy` harness from PR #37.

## Cross-service contract proof

Marker:

- `NA0349_QSL_SERVER_QSL_ATTACHMENTS_CONTRACT_OK`

The qsl-server harness models a qsl-attachments committed object, passes the
descriptor through qsl-server, and uses the delivered descriptor to fetch the
modeled opaque ciphertext. This proves the bounded descriptor handoff contract
without adding service dependencies or deployment.

## Route/API boundary proof

Marker:

- `NA0349_ROUTE_API_BOUNDARY_OK`

The harness uses qsl-server `/v1/push` and `/v1/pull`, verifies missing
route-token reject, wrong-route isolation, and exact descriptor delivery.

## Attachment object lifecycle proof

Marker:

- `NA0349_ATTACHMENT_OBJECT_LIFECYCLE_OK`

The modeled qsl-attachments fixture commits, fetches, snapshots/restores,
expires, purges, and rejects stale objects deterministically.

## Size-class flow boundary proof

Marker:

- `NA0349_SIZE_CLASS_FLOW_BOUNDARY_OK`

The descriptor carries visible `qsl_attachments_production_size_class_v1`
metadata. The harness checks the qshield-compatible small-class prefix but does
not claim that exact size, timing metadata, traffic shape, or all metadata is
hidden.

## Opaque payload boundary proof

Marker:

- `NA0349_OPAQUE_PAYLOAD_BOUNDARY_OK`

qsl-server preserves the descriptor bytes exactly and does not parse or log the
modeled ciphertext payload.

## Route-token/auth boundary proof

Marker:

- `NA0349_ROUTE_TOKEN_AUTH_BOUNDARY_OK`

The harness verifies missing route-token and missing bearer auth reject before
accepted handoff.

## Quota/rate boundary proof

Marker:

- `NA0349_QUOTA_RATE_BOUNDARY_OK`

The harness verifies qsl-server max-body rejection and no queued state for an
oversized descriptor.

## Retention/purge consistency proof

Marker:

- `NA0349_RETENTION_PURGE_CONSISTENCY_OK`

The harness verifies qsl-server queue drain after pull and qsl-attachments
modeled expiry/purge rejects a stale object.

## Backup/restore boundary proof

Marker:

- `NA0349_BACKUP_RESTORE_BOUNDARY_OK`

The modeled qsl-attachments store snapshot restores the committed object and
fetches exact bytes. This is harness backup-boundary evidence only; production
backup/restore remains the selected successor lane.

## Log redaction proof

Marker:

- `NA0349_LOG_REDACTION_BOUNDARY_OK`

The harness checks qsl-server logs and modeled qsl-attachments audit events do
not contain route-token, bearer-token, fetch-capability, payload, locator, or
attachment sentinels.

## Monitoring boundary proof

Marker:

- `NA0349_MONITORING_BOUNDARY_OK`

The modeled audit events use redacted handles and no raw secrets. Production
monitoring and alerting remain future-gated.

## Deploy/rollback boundary proof

Marker:

- `NA0349_DEPLOY_ROLLBACK_BOUNDARY_OK`

The harness binds only to loopback ephemeral ports and changes only a test
file. No deploy, rollback, packaging, service manager, workflow, or public
ingress configuration changes.

## Public-ingress boundary proof

Marker:

- `NA0349_PUBLIC_INGRESS_BOUNDARY_OK`

The harness is loopback-only and does not expose or configure public ingress.

## qshield demo reference boundary

Marker:

- `NA0349_QSHIELD_DEMO_REFERENCE_BOUNDARY_OK`

The harness checks the qshield-compatible small-class prefix as reference
evidence only. qshield embedded relay/demo proof remains distinct from
qsl-server/qsl-attachments production proof.

## Test/CI proof

qsl-server local validation passed:

- `bash scripts/ci/test_aws_update_and_verify.sh`
- `bash scripts/ci/test_update_checksum.sh`
- `cargo fmt --all -- --check`
- `cargo test -q --test na0349_end_to_end_integration_contract -- --nocapture`
- `cargo test -q`
- `cargo clippy -q -- -D warnings`

qsl-server PR #56 CI:

- required `rust` passed on head `9f51b5a691f`
- post-merge `rust` passed on merge `d40e6003fdf0`

qsl-attachments local validation passed unchanged:

- `cargo fmt --all -- --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo build --locked`
- `cargo test --locked`

## Public claim boundary

Markers:

- `NA0349_NO_ATTACHMENT_SIZE_HIDDEN_CLAIM_OK`
- `NA0349_NO_TIMING_HIDDEN_CLAIM_OK`
- `NA0349_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK`
- `NA0349_NO_METADATA_FREE_CLAIM_OK`

The harness and this evidence do not claim anonymity, metadata-free behavior,
untraceability, production readiness, public-internet readiness, external
review completion, or that attachment size, timing metadata, traffic shape, or
all metadata is hidden.

## Selected successor

Selected:

`NA-0350 -- Metadata Runtime Production Backup / Deploy / Rollback Hardening Plan`

Rationale:

- NA-0349 delivered executable integration harness evidence without production
  deployment;
- the remaining highest-value gate is production backup/deploy/rollback
  hardening before stronger service/public/review claims;
- qsl-server/qsl-attachments production boundaries, public ingress, monitoring,
  and backup/restore remain explicit and not overclaimed.

Rejected alternatives:

- `NA-0350 -- Metadata Runtime End-to-End Integration Blocker Resolution`:
  rejected because NA-0349 completed the bounded harness and CI proof.
- `NA-0350 -- Metadata Runtime External Review Readiness Gap Audit`: deferred
  until production backup/deploy/rollback gates are sharper.
- `NA-0350 -- Metadata Runtime Website / Public Claim Boundary Audit`: deferred
  because NA-0349 made no public docs or website claim change.
- `NA-0350 -- Metadata Runtime Service Production Gate Audit`: useful, but
  the production backup/deploy/rollback plan is the narrower next prerequisite.
- `NA-0350 -- Public Technical Position Paper Evidence-Bounded Draft Plan`:
  deferred until service production boundaries are better hardened.

## Backup-plan impact statement

No backup-plan update is required. qsl-server changed one source-controlled test
file under `/srv/qbuild/work/NA-0237D/qsl-server`; qsl-protocol companion files
remain under `/srv/qbuild/work/NA-0349/qsl-protocol`; qsl-attachments was not
changed; temporary validation artifacts remained rebuildable under existing
build/test locations.

No durable evidence, service data root, deployment config, monitoring export,
or backup root outside current backup scope was introduced.

## Next recommendation

Close NA-0349 only after this qsl-protocol governance companion merges and
post-merge `public-safety` is green. Restore exactly:

`NA-0350 -- Metadata Runtime Production Backup / Deploy / Rollback Hardening Plan`

NA-0350 must not be implemented by NA-0349 closeout.
