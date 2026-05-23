Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-23

# NA-0344 Metadata Runtime qsl-attachments Production Size-Class Implementation Harness Test Plan

## Objective

Verify that NA-0344 implements the bounded qsl-attachments production
size-class harness authorized by NA-0343, records qsl-attachments PR evidence
in qsl-protocol governance, and preserves all runtime, production, and public
claim boundaries.

## Protected invariants

- qsl-attachments mutation stays inside the authorized file map.
- qsl-protocol mutation is governance companion only.
- qsl-server is not mutated.
- qshield runtime is not mutated.
- qsc/qsp/protocol/crypto/key-schedule behavior is not changed.
- Dependencies, manifests, lockfiles, workflows, branch protection, and
  public-safety configuration are not changed.
- Website, README, START_HERE, and docs/public are not changed.
- No claim is made that attachment size, timing metadata, traffic shape, all
  metadata, anonymity, metadata-free behavior, or untraceability is achieved.
- No production-readiness, public-internet-readiness, or
  external-review-complete claim is made.

## Allowed qsl-attachments scope

- `src/lib.rs`
- `tests/production_size_class_policy.rs`
- existing authorized focused tests only if needed

## Forbidden qsl-attachments scope

- `Cargo.toml`
- `Cargo.lock`
- `.github/**`
- dependency updates
- workflow changes
- deployment automation
- secret material or secret-dependent fixtures
- README, START_HERE, broad docs, website, qsl-server, qshield, qsc, qsp,
  protocol, crypto, key-schedule, and unrelated refactors

## Allowed qsl-protocol companion scope

- `docs/governance/evidence/NA-0344_metadata_runtime_qsl_attachments_production_size_class_implementation_harness.md`
- `tests/NA-0344_metadata_runtime_qsl_attachments_production_size_class_implementation_harness_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Prior authorization review requirements

The companion evidence must record:

- live NA-0344 queue scope;
- inherited NA-0343 authorization;
- refreshed qsl-attachments source freshness;
- refreshed qsl-attachments authority and branch protection;
- refreshed required `rust` check;
- qshield demo compatibility boundary;
- qsl-server boundary;
- backup/deploy/secrets boundary;
- public claim boundary.

## qsl-attachments implementation requirements

The implementation must prove:

- policy `qsl_attachments_production_size_class_v1`;
- qshield demo-compatible small-class prefix;
- `8 KiB` increments through `1 MiB`;
- `1 MiB` increments to configured max;
- default max not above `101 MiB`;
- invalid config rejection;
- oversize rejection before accepted state;
- malformed descriptor/object rejection;
- no accepted state or output on reject;
- retention/purge behavior;
- cold full-root backup boundary;
- qsl-server remains unchanged;
- qshield demo proof remains reference/oracle only;
- no secret artifacts.

## qsl-attachments harness/marker requirements

Existing test style may use test names and assertion coverage instead of
emitting success markers. The required proof names are covered by:

- `na0344_size_class_policy_ok`
- `na0344_valid_small_medium_large_object_ok`
- `na0344_invalid_config_and_oversize_reject_ok`
- `na0344_malformed_descriptor_and_object_reject_ok`
- `na0344_retention_purge_and_backup_boundary_ok`
- `na0344_no_secret_artifact_qsl_server_boundary_and_qshield_demo_compatibility_ok`

The qsl-protocol evidence must map those test names to the NA-0344 marker
families:

- source freshness, authority, and CI authority;
- size-class policy;
- valid small/medium/large object;
- oversize reject;
- malformed reject;
- retention/purge;
- backup boundary;
- no secret artifact;
- qsl-server boundary;
- qshield demo compatibility;
- no size-hidden, timing-hidden, traffic-shape-hidden, or metadata-free claim.

## qsl-attachments CI requirements

Required local commands:

- `git diff --check`
- `cargo fmt --all -- --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo build --locked`
- `cargo test --locked`
- focused `cargo test --locked --test production_size_class_policy`

Required remote proof:

- PR `rust` check green on qsl-attachments head;
- post-merge `main` `rust` check green.

## qsl-server boundary requirements

The evidence must state that qsl-server was not changed, qsl-server production
timing/storage behavior remains unproven, and qsl-server integration is the
selected future boundary if qsl-attachments implementation succeeds.

## Backup/deploy/secrets requirements

The evidence must record:

- source and companion edits stayed under `/srv/qbuild/work`;
- tests used temporary directories;
- no live production data root was used;
- no deployment was performed;
- no raw resume token, fetch capability, route token, auth header, plaintext,
  raw key, or passphrase artifact was retained.

## Public claim boundary requirements

The companion must keep all public/readiness/privacy gaps explicit and must
not update README, START_HERE, docs/public, website, or external website.

## Required qsl-protocol local checks

- queue helper;
- decisions helper;
- scope guard;
- diff check;
- link check;
- leak scan;
- overclaim scan;
- classifier proof;
- PR body preflight / goal-lint;
- `cargo audit --deny warnings`;
- `cargo tree -i rustls-webpki --locked`;
- qsc send_commit;
- formal/model checks.

## CI expectations

The qsl-protocol governance companion PR must pass required checks, including
`public-safety`, before merge. Post-merge `public-safety` must also be green.

## Successor handoff

If qsl-attachments implementation and qsl-protocol companion both merge
cleanly, close NA-0344 in a separate closeout and restore:

`NA-0345 -- Metadata Runtime qsl-server Integration Boundary Plan`
