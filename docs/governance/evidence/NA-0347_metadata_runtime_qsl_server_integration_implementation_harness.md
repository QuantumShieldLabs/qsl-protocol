Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-24

# NA-0347 Metadata Runtime qsl-server Integration Implementation Harness

## Executive summary

NA-0347 executed the bounded qsl-server integration implementation harness
authorized by NA-0346. qsl-server PR #55 merged as `b194a95b19a7` from
validated head `e8b2de95426c`, and post-merge qsl-server `main` completed the
required `rust` check successfully.

Result: `QSL_SERVER_INTEGRATION_HARNESS_MERGED`.

The implementation added one qsl-server executable harness file:
`tests/qsl_attachments_integration_contract.rs`. It proves the conservative
qsl-server/qsl-attachments boundary: qsl-server treats qsl-attachments-shaped
payloads as opaque relay bytes, does not call qsl-attachments APIs, does not
store durable attachment objects, preserves route/API fail-closed behavior, and
keeps quota/rate, retention/purge, secret/env, loopback-only public-ingress,
backup, deploy/rollback, qsl-attachments service-local, qshield reference, and
public-claim boundaries explicit.

No qsl-attachments, qshield runtime, qsc/qsp/protocol/crypto, dependency,
workflow, deployment, website, README, START_HERE, docs/public, or production
service path changed.

Selected successor:

`NA-0348 -- Metadata Runtime End-to-End qsl-server / qsl-attachments Integration Evidence Plan`

## Live NA-0347 scope

The live `NEXT_ACTIONS.md` entry required NA-0347 to refresh qsl-server
source/authority/CI, open a qsl-server implementation branch only if
prerequisites remained ready, implement only the bounded qsl-server integration
harness selected by NA-0346, preserve qsl-attachments PR #37 as service-local
prerequisite evidence, preserve qshield embedded relay/demo evidence as
reference/oracle evidence only, and keep all production/public-claim boundaries
explicit.

The live scope protected:

- no unsupported production, public-internet, external-review, anonymity,
  metadata-free, or untraceable claim;
- no claim that attachment size, timing metadata, or traffic shape is hidden;
- executable qsl-server proof or exact prerequisite stop;
- qsl-server/qsl-attachments production boundary;
- qsl-attachments service-local proof from PR #37 as prerequisite evidence only;
- qshield embedded relay/demo proof as reference/oracle evidence only.

## Inherited NA-0346 authorization

NA-0346 recorded `IMPLEMENTATION_AUTHORIZATION_READY` with selected qsl-server
source `/srv/qbuild/work/NA-0237D/qsl-server` at `3f28d7433e74`. It authorized
bounded future qsl-server source/test changes only after a fresh
source/authority/CI refresh.

Relevant inherited authorization:

- allowed qsl-server source files: `src/lib.rs`, `src/main.rs`, only if needed;
- allowed focused qsl-server tests, including one new exact file
  `tests/qsl_attachments_integration_contract.rs`;
- forbidden qsl-server manifests, lockfiles, workflows, deployment automation,
  public README/legal copy, broad refactors, dependency changes, and production
  deployment changes;
- default qsl-server/qsl-attachments contract is opaque pass-through unless a
  later exact directive authorizes qsl-attachments API calls.

## qsl-server source/authority refresh

Refreshed before qsl-server mutation:

| Field | Result |
| --- | --- |
| selected path | `/srv/qbuild/work/NA-0237D/qsl-server` |
| local HEAD before branch | `3f28d7433e74` |
| live remote `main` before branch | `3f28d7433e74` |
| freshness | `FRESH_SOURCE` |
| viewer permission | `ADMIN` |
| branch protection | present |
| required check | strict `rust` |
| force pushes | disabled |
| deletions | disabled |
| admins enforced | enabled |
| open PRs before branch | none listed |
| latest listed qsl-server main CI before branch | success on `3f28d7433e74` |
| mutation authority | `COMPLETE_MUTATION_AUTHORITY` |
| CI authority | `COMPLETE_CI_AUTHORITY` |

Backup scope was acceptable because qsl-server and qsl-protocol source changes
stayed under `/srv/qbuild/work`, and the focused test log artifact was kept
under `/srv/qbuild/tmp`.

## qsl-server PR branch/head/merge

- Repository: `QuantumShieldLabs/qsl-server`
- PR: #55, `https://github.com/QuantumShieldLabs/qsl-server/pull/55`
- Branch: `na-0347-integration-implementation-harness`
- Base: `main` at `3f28d7433e74`
- Head: `e8b2de95426c`
- Merge: `b194a95b19a7`
- Merge method: normal merge commit with `--match-head-commit`
- Delete-branch flag: not used
- Required PR `rust`: success after one failed-job rerun
- Post-merge main `rust`: success on `b194a95b19a7`

## qsl-server changed files

- `tests/qsl_attachments_integration_contract.rs`

No qsl-server `Cargo.toml`, `Cargo.lock`, `.github/**`, deployment scripts,
packaging, docs, README, START_HERE, source behavior, qsl-attachments, qshield,
qsc, qsp, protocol, crypto, key schedule, or workflow file changed.

## Implementation summary

The qsl-server harness adds service-local executable proof for:

- qsl-attachments-shaped opaque payload relay round-trip;
- missing route-token reject without mutation;
- route isolation;
- exact byte preservation on delivery;
- no stale item after delivery;
- body-size reject without mutation;
- local in-app rate limiting;
- global route cap and route-cap release after drain;
- idle TTL purge;
- no durable file artifact created by the harness;
- log redaction for route token, bearer token, and payload sentinels;
- loopback-only test listener;
- unsupported public-claim phrase scan.

The implementation deliberately avoids qsl-attachments API calls. The qsl-server
proof is therefore bounded to the conservative opaque pass-through contract.

## Route/API boundary proof

The harness exercises canonical push/pull behavior through the current
qsl-server API surface. It proves that route-token absence fails closed before
mutation, route isolation is preserved, and delivered bytes match the accepted
opaque payload.

Marker:

- `NA0347_QSL_SERVER_ROUTE_BOUNDARY_OK`

## Storage/proxy boundary proof

The harness uses a qsl-attachments-shaped fixture and verifies exact opaque byte
round-trip through qsl-server. qsl-server does not parse, strip, rewrite, store
durably, or proxy the attachment metadata/object through a qsl-attachments API.

Marker:

- `NA0347_QSL_SERVER_STORAGE_BOUNDARY_OK`

## Quota/rate boundary proof

The harness proves body-size rejection, route-local rate limiting, and global
route-cap enforcement remain deterministic and fail closed.

Marker:

- `NA0347_QSL_SERVER_QUOTA_BOUNDARY_OK`

## Retention/purge boundary proof

The harness proves an idle route expires and stale queued data is not delivered
after TTL expiry.

Marker:

- `NA0347_QSL_SERVER_RETENTION_PURGE_BOUNDARY_OK`

## Backup-boundary proof

The qsl-server implementation changed only a source-controlled test file under
`/srv/qbuild/work/NA-0237D/qsl-server`. The harness asserts its unique expected
durable-artifact path is absent, and no test-created durable storage outside the
source tree or `/srv/qbuild/tmp` is required.

Marker:

- `NA0347_QSL_SERVER_BACKUP_BOUNDARY_OK`

## Secret/env proof

The harness uses deterministic non-production auth and route sentinels, captures
qsl-server logs, and verifies route-token, bearer-token, and payload sentinels
do not appear in logs. `RELAY_TOKEN` remains optional existing auth behavior;
no secret-dependent test or committed secret is added.

Marker:

- `NA0347_QSL_SERVER_SECRET_ENV_BOUNDARY_OK`

## Deploy/rollback proof

The harness binds only to loopback ephemeral ports and does not touch packaging,
systemd, install/update scripts, release workflows, branch protection, or
deployment configuration. Rollback remains a source revert of the harness PR;
no production rollback is claimed or executed.

Marker:

- `NA0347_QSL_SERVER_DEPLOY_ROLLBACK_BOUNDARY_OK`

## Public-ingress boundary proof

The harness asserts loopback-only listener construction. It does not expose a
public service, mutate reverse-proxy configuration, or claim public-internet
readiness.

Marker:

- `NA0347_QSL_SERVER_PUBLIC_INGRESS_BOUNDARY_OK`

## qsl-attachments service-local contract proof

qsl-attachments PR #37 remains prerequisite service-local evidence. qsl-server
PR #55 proves only that qsl-server can relay qsl-attachments-shaped opaque
bytes through its existing service-local boundary without qsl-attachments
mutation or API calls.

Markers:

- `NA0347_QSL_ATTACHMENTS_CONTRACT_OK`
- `NA0347_QSL_ATTACHMENTS_SERVICE_LOCAL_BOUNDARY_OK`

## qshield demo reference boundary

qshield embedded relay/demo evidence remains reference/oracle evidence only.
The qsl-server harness does not mutate qshield runtime or present demo evidence
as production proof.

Marker:

- `NA0347_QSHIELD_DEMO_REFERENCE_BOUNDARY_OK`

## Test/CI proof

Local qsl-server validation passed:

- `bash scripts/ci/test_aws_update_and_verify.sh`
- `bash scripts/ci/test_update_checksum.sh`
- `cargo fmt --all -- --check`
- `cargo test -q`
- `cargo clippy -q -- -D warnings`
- `cargo test -q --test qsl_attachments_integration_contract -- --nocapture`

Focused marker log:

- `/srv/qbuild/tmp/na0347_qsl_server_focused_test.log`

CI proof:

- qsl-server PR #55 initial `rust` check failed in existing
  `hardening_auth_reject_logging` log-capture test.
- Recovery classification: likely flaky existing CI log-capture miss because
  the failed test passed locally five consecutive focused runs and full local
  `cargo test -q` had passed.
- Corrective action: one allowed failed-job rerun.
- Final result: PR-side `rust` success; post-merge main `rust` success on
  `b194a95b19a7`.

## Public claim boundary

The qsl-server harness includes an executable phrase scan over current
qsl-server README and server docs. It asserts absence of unsupported claims
including hidden attachment size, hidden timing metadata, hidden traffic shape,
hidden all metadata, metadata-free behavior, untraceability, anonymity,
production-readiness, public-internet readiness, external-review completion,
quantum-proof, unbreakable, guaranteed-secure, and military-grade language.

Markers:

- `NA0347_NO_ATTACHMENT_SIZE_HIDDEN_CLAIM_OK`
- `NA0347_NO_TIMING_HIDDEN_CLAIM_OK`
- `NA0347_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK`
- `NA0347_NO_METADATA_FREE_CLAIM_OK`
- `NA0347_METADATA_RUNTIME_QSL_SERVER_INTEGRATION_OK`

## Selected successor

Selected successor:

`NA-0348 -- Metadata Runtime End-to-End qsl-server / qsl-attachments Integration Evidence Plan`

Rationale: qsl-server implementation/harness proof now exists for the
conservative service-local opaque-pass-through boundary. The next truthful lane
is to plan end-to-end qsl-server/qsl-attachments evidence without mutating
qsl-attachments or claiming production/public-internet readiness.

## Backup-plan impact statement

No backup-plan update is required. qsl-server source changes are under
`/srv/qbuild/work/NA-0237D/qsl-server`; qsl-protocol governance companion
changes are under `/srv/qbuild/work/NA-0347/qsl-protocol`; focused test output
is under `/srv/qbuild/tmp`. No durable deployed config, service data root, or
backup root outside current backup scope was introduced.

## Next recommendation

Merge this qsl-protocol governance companion only after local qsl-protocol
checks and required public-safety are green. Then close NA-0347 in a separate
queue transition that restores the selected NA-0348 successor without
implementing NA-0348.
