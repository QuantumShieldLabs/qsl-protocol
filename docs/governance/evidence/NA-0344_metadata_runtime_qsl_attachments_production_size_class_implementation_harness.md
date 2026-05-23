Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-23

# NA-0344 Metadata Runtime qsl-attachments Production Size-Class Implementation Harness

## Executive summary

NA-0344 executed the bounded qsl-attachments implementation lane selected by
NA-0343. The qsl-attachments implementation PR merged as PR #37 with required
`rust` green on the PR head and post-merge `main`.

Result: `QSL_ATTACHMENTS_SIZE_CLASS_HARNESS_MERGED`.

The implementation adds an opt-in qsl-attachments policy named
`qsl_attachments_production_size_class_v1`. It computes deterministic
production size classes, rejects invalid configuration and oversize objects
fail-closed, persists size-class metadata in session/object journals, and keeps
startup reconciliation fail-closed for malformed class metadata.

Important boundary: this does not claim attachment size, timing metadata,
traffic shape, or all metadata is hidden. It does not deploy a service, expose
public-internet behavior, complete external review, or change qsl-server.
Committed qsl-attachments object fetches still return the exact opaque
ciphertext bytes under the existing service contract.

Selected successor:

`NA-0345 -- Metadata Runtime qsl-server Integration Boundary Plan`

## Live NA-0344 scope

The live queue entry on `origin/main` was:

- `NA-0344 -- Metadata Runtime qsl-attachments Production Size-Class
  Implementation Harness`;
- Status: READY;
- Goals: G1, G2, G3, G4, G5;
- Objective: execute the qsl-attachments production size-class implementation
  harness selected by NA-0343, or stop with exact changed-prerequisite evidence.

Protected boundaries preserved:

- qsl-attachments mutation stayed inside `src/lib.rs` and a focused test file;
- qsl-server was not changed;
- qshield runtime was not changed;
- qsc/qsp/protocol/crypto/key-schedule behavior was not changed;
- no dependency, manifest, lockfile, workflow, branch-protection,
  public-safety, website, README, START_HERE, or docs/public change was made;
- no unsupported readiness or privacy claim was introduced.

## Inherited NA-0343 authorization

NA-0343 recorded `IMPLEMENTATION_AUTHORIZATION_READY`.

Inherited facts:

- selected qsl-attachments path:
  `/srv/qbuild/work/NA-0237D/qsl-attachments`;
- selected qsl-attachments source SHA: `320be68fe632`;
- remote default branch: `main`;
- viewer permission: `ADMIN`;
- branch protection required strict `rust`;
- latest listed main `rust` run was success;
- future qsl-attachments branch:
  `na-0344-production-size-class-harness`;
- future qsl-protocol companion branch:
  `na-0344-qsl-attachments-size-class-governance-companion`;
- future policy name: `qsl_attachments_production_size_class_v1`;
- candidate table: qshield demo-compatible small classes through `8192`,
  then `8 KiB` increments through `1 MiB`, then `1 MiB` increments to the
  configured max;
- default max must not exceed the existing `101 MiB` qsl-attachments ceiling.

## qsl-attachments source/authority refresh

NA-0344 refreshed qsl-attachments before mutation:

| Field | Result |
| --- | --- |
| selected path | `/srv/qbuild/work/NA-0237D/qsl-attachments` |
| initial local HEAD | `320be68fe632` |
| remote `origin/main` | `320be68fe632` |
| live remote `main` | `320be68fe632` |
| worktree state | clean detached HEAD |
| viewer permission | `ADMIN` |
| branch protection | present |
| required strict check | `rust` |
| open PRs before branch | none |
| latest listed main `rust` run before mutation | success |
| source classification | `FRESH_SOURCE` |
| authority classification | `COMPLETE_MUTATION_AUTHORITY` |
| CI classification | `COMPLETE_CI_AUTHORITY` |

Backup scope was acceptable because the selected source and qsl-protocol
companion remained under `/srv/qbuild/work`, and tests used temporary
directories only.

## qsl-attachments PR branch/head/merge

- Repository: `QuantumShieldLabs/qsl-attachments`
- PR: #37, `https://github.com/QuantumShieldLabs/qsl-attachments/pull/37`
- Branch: `na-0344-production-size-class-harness`
- Base: `main` at `320be68fe632`
- Head: `7e6d82570b7d`
- Merge: `96b9352bd63e`
- Merge method: normal merge commit with `--match-head-commit`
- Delete-branch flag: not used
- Required PR `rust`: success on head `7e6d82570b7d`
- Post-merge main `rust`: success on `96b9352bd63e`

## qsl-attachments changed files

- `src/lib.rs`
- `tests/production_size_class_policy.rs`

No `Cargo.toml`, `Cargo.lock`, `.github/**`, README, START_HERE, docs,
deployment, secret, qsl-server, qshield, qsc, qsp, protocol, crypto,
key-schedule, or workflow file changed in qsl-attachments.

## Implementation summary

The implementation adds:

- public policy constant `qsl_attachments_production_size_class_v1`;
- `SizeClassPolicy` and `SizeClassInfo`;
- deterministic table and class helpers;
- opt-in env/config parsing for `QATT_SIZE_CLASS_POLICY`;
- deterministic rejection for invalid policy max values;
- deterministic oversize rejection before session state is accepted;
- persisted optional size-class metadata in session and object JSON;
- startup reconciliation rejection for malformed size-class metadata;
- additive response metadata for created sessions, status, and commits.

The implementation preserves:

- existing service-local API routes;
- exact opaque ciphertext byte storage and fetch behavior;
- existing capability, quota, retention, purge, recovery, and range semantics;
- backward compatibility for existing session/object JSON without size-class
  metadata because the new field is optional.

## Size-class policy

Policy: `qsl_attachments_production_size_class_v1`.

The deterministic table is:

- qshield demo-compatible small classes:
  `256, 512, 768, 1024, 1536, 2048, 3072, 4096, 5120, 6144, 7168, 8192`;
- `8 KiB` increments from `16 KiB` through `1 MiB`;
- `1 MiB` increments from `2 MiB` through configured max.

Default configured max: `101 MiB`.

Rejected config examples include:

- max below `256` bytes;
- max not aligned to the applicable table step;
- max above the `101 MiB` qsl-attachments ceiling;
- `QATT_SIZE_CLASS_MAX_BYTES` without enabling the policy.

## Valid small/medium/large proof

Focused test:

`tests/production_size_class_policy.rs`

Test name:

`na0344_valid_small_medium_large_object_ok`

Coverage:

- small object: `777` bytes;
- medium object: `65,537` bytes;
- large object: `1 MiB + 1` bytes;
- each object creates a session, uploads parts, commits, persists size-class
  metadata, fetches exact opaque ciphertext bytes, and verifies the fetched
  bytes equal the original service input.

## Oversize reject proof

Test name:

`na0344_invalid_config_and_oversize_reject_ok`

Coverage:

- invalid in-memory config rejects during state construction;
- object above a configured `1 MiB` class max rejects with
  `REJECT_QATTSVC_QUOTA`;
- no session directory is accepted on oversize reject.

## Malformed object reject proof

Test name:

`na0344_malformed_descriptor_and_object_reject_ok`

Coverage:

- malformed JSON create request rejects with
  `REJECT_QATTSVC_MALFORMED_JSON`;
- malformed persisted size-class metadata in `object.json` is discarded by
  startup reconciliation as an incoherent object;
- no object remains fetchable after that malformed-object recovery reject.

## Retention/purge proof

Test name:

`na0344_retention_purge_and_backup_boundary_ok`

Coverage:

- committed object remains recoverable before expiry;
- advancing the test clock past retention causes fetch to return
  `REJECT_QATTSVC_EXPIRED`;
- the committed `ciphertext.bin` is removed on purge.

## Backup-boundary proof

Test name:

`na0344_retention_purge_and_backup_boundary_ok`

Coverage:

- cold full-root copy with matching service config recovers one committed
  object;
- recovered object fetch returns exact opaque ciphertext bytes;
- no hot/live backup, partial restore, multi-node, or deployment claim is made.

## No secret artifact proof

Test name:

`na0344_no_secret_artifact_qsl_server_boundary_and_qshield_demo_compatibility_ok`

Coverage:

- object JSON does not contain raw fetch capability or resume token;
- audit event handles do not equal raw session, locator, or attachment refs;
- audit handles remain short hex handles;
- tests use temporary directories and no secret-dependent fixtures.

## qsl-server boundary

qsl-server was not cloned, edited, tested as a dependency, deployed, or
configured. The qsl-attachments focused harness verifies the service-local
router still does not provide a qsl-server integration route. This is only a
boundary check, not qsl-server production timing/storage proof.

## qshield demo compatibility boundary

The qsl-attachments table preserves the qshield demo-compatible small-class
prefix exactly:

`256, 512, 768, 1024, 1536, 2048, 3072, 4096, 5120, 6144, 7168, 8192`

qshield embedded relay/demo proof remains reference/oracle evidence for table
shape and fail-closed handling only. It is not qsl-attachments or qsl-server
production proof.

## Test/CI proof

Local qsl-attachments validation:

- `git diff --check`: pass;
- `cargo fmt --all -- --check`: pass after rustfmt recovery;
- `cargo clippy --all-targets -- -D warnings`: pass after clippy recovery;
- `cargo build --locked`: pass;
- `cargo test --locked`: pass;
- focused `cargo test --locked --test production_size_class_policy`: pass,
  `6` tests.

Remote qsl-attachments CI:

- PR #37 required `rust`: success on `7e6d82570b7d`;
- post-merge `main` required `rust`: success on `96b9352bd63e`.

## Public claim boundary

This evidence does not claim:

- attachment size is hidden;
- timing metadata is hidden;
- traffic shape is hidden;
- all metadata is hidden;
- anonymity;
- metadata-free behavior;
- untraceability;
- production readiness;
- public-internet readiness;
- external review completion.

The size-class policy is an implementation/harness step. Deployment, operator
enablement, qsl-server integration, public service readiness, and external
review remain future work.

## Selected successor

Selected successor:

`NA-0345 -- Metadata Runtime qsl-server Integration Boundary Plan`

Rationale:

- qsl-attachments source, authority, implementation, local validation, PR,
  merge, and post-merge `rust` are complete;
- qsl-server was intentionally not changed by NA-0344;
- qsl-server production timing/storage/integration boundaries remain the next
  relevant metadata-runtime service boundary before stronger service claims.

Rejected alternatives:

- `NA-0345 -- Metadata Runtime qsl-attachments Production Size-Class Blocker
  Resolution`: rejected because no implementation blocker remained after PR
  #37 and post-merge `rust` green.
- `NA-0345 -- Metadata Runtime qsl-attachments Production Backup / Retention
  Hardening Plan`: rejected as immediate successor because cold full-root
  backup and retention/purge boundaries were covered for the bounded harness;
  production backup hardening remains future deployment work.
- `NA-0345 -- Metadata Runtime External Review Readiness Gap Audit`: rejected
  as immediate successor because qsl-server boundary evidence is needed before
  review can evaluate service-level metadata claims truthfully.
- `NA-0345 -- Metadata Runtime Website / Public Claim Boundary Audit`: rejected
  because no public-copy mutation or stronger public claim was made.

## Backup-plan impact statement

No backup-plan update is required for NA-0344.

The qsl-attachments implementation remained under
`/srv/qbuild/work/NA-0237D/qsl-attachments`; qsl-protocol companion files remain
under `/srv/qbuild/work`; qsl-attachments tests use temporary directories; no
new durable evidence/storage location outside the current qbuild work area was
introduced; and no production data root or live deployment artifact was used as
evidence.

## Next recommendation

Merge this qsl-protocol governance companion after public-safety is green, then
close NA-0344 and restore:

`NA-0345 -- Metadata Runtime qsl-server Integration Boundary Plan`
