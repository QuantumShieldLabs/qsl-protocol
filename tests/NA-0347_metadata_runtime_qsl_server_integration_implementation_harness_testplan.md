Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-24

# NA-0347 Metadata Runtime qsl-server Integration Implementation Harness Test Plan

## Objective

Verify that NA-0347 implements and proves only the bounded qsl-server
integration harness authorized by NA-0346, then records qsl-protocol companion
evidence without changing qsl-protocol runtime or overclaiming metadata,
production, public-internet, or external-review status.

## Protected invariants

- qsl-server implementation stays within the authorized file map.
- qsl-attachments remains unchanged.
- qshield runtime remains unchanged.
- qsc/qsp/protocol/crypto/key-schedule behavior remains unchanged.
- No dependency, workflow, branch-protection, public-safety, deployment,
  website, README, START_HERE, docs/public, or production service path changes.
- qsl-attachments PR #37 remains service-local prerequisite evidence only.
- qshield embedded relay/demo evidence remains reference/oracle evidence only.
- No claim that attachment size, timing metadata, traffic shape, all metadata,
  anonymity, metadata-free behavior, or untraceable behavior is achieved.
- No production-readiness, public-internet-readiness, or external-review
  completion claim is made.

## Allowed qsl-server scope

- `src/lib.rs` only if required by the bounded harness.
- `src/main.rs` only if required by deterministic non-secret config parsing.
- Existing focused qsl-server tests authorized by NA-0346.
- One new focused test file:
  `tests/qsl_attachments_integration_contract.rs`.

## Forbidden qsl-server scope

- `Cargo.toml`
- `Cargo.lock`
- `.github/**`
- deployment scripts or packaging automation
- README/public/legal copy
- qsl-attachments, qshield, qsc, qsp, protocol, crypto, dependency, workflow,
  branch-protection, or production deployment changes
- broad refactors or unrelated formatting churn

## Allowed qsl-protocol companion scope

- `docs/governance/evidence/NA-0347_metadata_runtime_qsl_server_integration_implementation_harness.md`
- `tests/NA-0347_metadata_runtime_qsl_server_integration_implementation_harness_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Closeout may later use queue files only under a separate closeout directive.

## Prior authorization review requirements

The companion evidence must record:

- live NA-0347 queue scope;
- inherited NA-0346 `IMPLEMENTATION_AUTHORIZATION_READY` decision;
- refreshed qsl-server source/authority/CI proof;
- qsl-server changed files;
- qsl-server PR branch/head/merge;
- qsl-attachments PR #37 service-local prerequisite boundary;
- qshield reference/oracle boundary;
- backup/deploy/secrets/public-ingress boundaries;
- selected exact NA-0348 successor.

## qsl-server implementation requirements

The implementation must:

- preserve existing route/API behavior;
- preserve auth and route-token fail-closed behavior;
- preserve max-body, queue, route-cap, push-rate, and TTL controls;
- treat qsl-attachments-shaped payloads as opaque bytes unless a future exact
  directive authorizes qsl-attachments API calls;
- avoid durable object storage;
- avoid production deployment and deployment config mutation;
- avoid secret-dependent tests;
- keep malformed/invalid inputs fail closed.

## qsl-server harness/marker requirements

The harness must emit or support companion evidence for:

- `NA0347_QSL_SERVER_SOURCE_AUTHORITY_OK`
- `NA0347_QSL_SERVER_IMPLEMENTATION_AUTHORIZATION_OK`
- `NA0347_QSL_ATTACHMENTS_CONTRACT_OK`
- `NA0347_QSL_SERVER_ROUTE_BOUNDARY_OK`
- `NA0347_QSL_SERVER_STORAGE_BOUNDARY_OK`
- `NA0347_QSL_SERVER_QUOTA_BOUNDARY_OK`
- `NA0347_QSL_SERVER_RETENTION_PURGE_BOUNDARY_OK`
- `NA0347_QSL_SERVER_BACKUP_BOUNDARY_OK`
- `NA0347_QSL_SERVER_SECRET_ENV_BOUNDARY_OK`
- `NA0347_QSL_SERVER_DEPLOY_ROLLBACK_BOUNDARY_OK`
- `NA0347_QSL_SERVER_PUBLIC_INGRESS_BOUNDARY_OK`
- `NA0347_QSHIELD_DEMO_REFERENCE_BOUNDARY_OK`
- `NA0347_QSL_ATTACHMENTS_SERVICE_LOCAL_BOUNDARY_OK`
- `NA0347_NO_ATTACHMENT_SIZE_HIDDEN_CLAIM_OK`
- `NA0347_NO_TIMING_HIDDEN_CLAIM_OK`
- `NA0347_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK`
- `NA0347_NO_METADATA_FREE_CLAIM_OK`
- `NA0347_METADATA_RUNTIME_QSL_SERVER_INTEGRATION_OK`

## qsl-server CI requirements

Required local commands:

- `bash scripts/ci/test_aws_update_and_verify.sh`
- `bash scripts/ci/test_update_checksum.sh`
- `cargo fmt --all -- --check`
- `cargo test -q`
- `cargo clippy -q -- -D warnings`
- focused new harness test with marker output

Required GitHub state:

- qsl-server PR required `rust` check green before merge;
- qsl-server post-merge main required `rust` check green;
- no admin bypass, direct push, squash, rebase, or branch deletion command.

## qsl-attachments boundary requirements

- qsl-attachments is read-only.
- qsl-attachments PR #37 is prerequisite service-local evidence only.
- qsl-server proof must be qsl-server-local executable proof.
- Any need for qsl-attachments mutation is a stop condition.

## qshield demo boundary requirements

- qshield embedded relay/demo evidence remains reference/oracle only.
- No qshield runtime mutation.
- No production claim from demo/reference proof.

## Backup/deploy/secrets requirements

- Source changes remain under `/srv/qbuild/work`.
- Test artifacts, if any, remain under `/srv/qbuild/tmp`.
- No durable deployed config, service data root, or backup root is introduced.
- No production deployment.
- No deployment rollback is executed or claimed.
- Logs and artifacts must not include route-token, bearer-token, payload, raw
  key, passphrase, or secret diagnostics.

## Public claim boundary requirements

Evidence, PR bodies, logs, and changed files must not introduce:

- attachment-size-hidden claim;
- timing-hidden claim;
- traffic-shape-hidden claim;
- metadata-free, anonymity, or untraceable claim;
- production-readiness or public-internet-readiness claim;
- external-review-complete claim;
- quantum-proof, unbreakable, guaranteed-secure, or military-grade language.

Negative assertion strings in tests are allowed when they prove the claim scan.

## Required qsl-protocol local checks

- `git diff --check`
- qsl-protocol queue helper
- qsl-protocol decisions helper
- qsl-protocol scope guard
- qsl-protocol link check
- qsl-protocol leak scan
- classifier proof
- goal-lint / PR body preflight
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- qsc `send_commit`
- formal/model checks
- qshield/reference harnesses when directly feasible

## CI expectations

- qsl-protocol PR checks must complete successfully.
- `public-safety` must remain required and green.
- Post-merge qsl-protocol main `public-safety` must complete success.

## Successor handoff

If qsl-server implementation and qsl-protocol companion evidence merge
successfully, close NA-0347 in a separate transition and restore exactly:

`NA-0348 -- Metadata Runtime End-to-End qsl-server / qsl-attachments Integration Evidence Plan`

The closeout must not implement NA-0348.
