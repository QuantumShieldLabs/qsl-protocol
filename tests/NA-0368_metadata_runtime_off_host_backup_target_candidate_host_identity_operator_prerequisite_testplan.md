Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-27

# NA-0368 Metadata Runtime Off-Host Backup Target Candidate / Host Identity Operator Prerequisite Testplan

## Objective

Validate that NA-0368 defines a no-secret, no-connection operator prerequisite
plan for future target-candidate and host-identity evidence before any remote
connection, host-key scan, credential handling, repository init, tool
installation, backup, restore, deploy, rollback, real restore target
creation/mount/copy, local backup mutation, runtime mutation, or public-claim
expansion.

## Protected Invariants

- NA-0368 is prerequisite planning only.
- qsl-protocol is the only mutable repository.
- qsl-server and qsl-attachments are read-only.
- qshield runtime is unchanged.
- qsc/qsp/protocol/crypto/key-schedule implementation is unchanged.
- Cargo dependencies and workflows are unchanged.
- README, START_HERE, website, and docs/public are unchanged.
- Backup scripts, timers, fstab, service units, source lists, restore paths,
  rollback paths, remote destinations, key paths, passphrase paths, credential
  paths, recovery-envelope content, and local backup configuration are
  unchanged.
- No remote/off-host connection, host-key scan, `known_hosts` mutation,
  repository init, tool installation, backup, restore, deploy, rollback, real
  restore target creation/mount/copy, real key generation, key upload,
  passphrase collection, credential handling, private-key inspection,
  recovery-envelope content creation, or secret handling occurs.
- No claim states or implies production readiness, public-internet readiness,
  external-review completion, anonymity, metadata-free behavior, untraceability,
  hidden attachment size, hidden timing metadata, hidden traffic shape, hidden
  all metadata, complete off-host backup, complete disaster recovery, real
  restore completion, host identity verification, target configuration, real key
  custody implementation, or real key recovery implementation.

## Allowed Scope

Allowed files:

- `docs/governance/evidence/NA-0368_metadata_runtime_off_host_backup_target_candidate_host_identity_operator_prerequisite_plan.md`
- `tests/NA-0368_metadata_runtime_off_host_backup_target_candidate_host_identity_operator_prerequisite_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

The optional input artifact is not required and should be absent unless future
live scope explicitly authorizes it.

## Forbidden Scope

Forbidden changes include qsl-server, qsl-attachments, qshield runtime,
qsc/qsp implementation, protocol/crypto, dependencies, workflows, README,
START_HERE, docs/public, website, service configuration, deployment scripts,
rollback scripts, backup scripts, timers, fstab, local backup configuration,
off-host destinations, restore paths, key material, passphrase paths,
credential paths, recovery-envelope content, and any real target/repository/tool
setup.

## Prior Target-Access Review Requirements

Validation must confirm the evidence reviews and preserves:

- NA-0367 target-access and host-identity prerequisite result.
- NA-0366 real target/tool blocker-resolution result.
- NA-0365 no-secret isolated restore harness boundary.
- NA-0363 no-secret off-host target/tool harness boundary.
- NA-0361 no-secret key custody/recovery harness boundary.

## Source/Authority Refresh Requirements

Validation must confirm read-only qsl-server and qsl-attachments status:

- local path and SHA if present;
- remote default branch SHA if available;
- PR #56 / PR #37 merge status;
- latest main CI status if available;
- viewer permission if available;
- branch protection if available;
- open PR list;
- classification as fresh/stale/unknown source, complete/partial/blocked
  authority, and complete/partial/blocked CI.

No qsl-server or qsl-attachments mutation is allowed.

## Local Backup/Tool/Key/Off-Host/Restore Evidence Refresh Requirements

Validation must confirm the evidence records:

- `/backup/qsl` mount status if available;
- local snapshot, manifest, and log availability;
- backup plan/status file presence;
- qsl-backup syntax/preflight/list results if safely run;
- installed-tool status for restic, borg, rclone, age, gpg, ssh, and rsync;
- off-host target candidate, off-host configuration, off-host repository,
  target host identity, SSH/SFTP credential, capacity/quota/retention,
  monitoring/alerting, operator runbook, backup tool install policy,
  repository-init plan, key custody, key recovery, recovery envelope, real
  isolated restore target, and real restore drill status;
- local history/directive folder coverage gaps;
- D132 bundle status.

Expected classifications include `LOCAL_CONTINUITY_PROVEN`,
`NO_SECRET_TARGET_TOOL_PROVEN`, `NO_SECRET_ISOLATED_RESTORE_PROVEN`,
`TARGET_CANDIDATE_ABSENT`, `TARGET_CANDIDATE_OPERATOR_REQUIRED`,
`TARGET_CANDIDATE_TEMPLATE_READY`, `HOST_IDENTITY_ABSENT`,
`HOST_IDENTITY_OPERATOR_SUPPLIED_REQUIRED`, `CREDENTIAL_BOUNDARY_ABSENT`,
`CREDENTIAL_SECRET_HANDLING_FORBIDDEN`, `CAPACITY_RETENTION_ABSENT`,
`MONITORING_ALERTING_ABSENT`, `REAL_TARGET_ACCESS_NOT_AUTHORIZED`, and
`OFF_HOST_BACKUP_NOT_READY`.

## Target Candidate Field Requirements

Evidence must include:

- `TARGET_CANDIDATE_FIELD_PLAN_OK`
- `TARGET_CANDIDATE_OPERATOR_ACTION_REQUIRED`
- `TARGET_CANDIDATE_NO_CONNECTION_OK`
- `TARGET_CANDIDATE_NO_SECRET_OK`
- `TARGET_CANDIDATE_NOT_READY_FOR_BACKUP`

The plan must name exact non-secret operator fields and exact fields forbidden
because they would be secrets.

## Host Identity Field Requirements

Evidence must include:

- `HOST_IDENTITY_FIELD_PLAN_OK`
- `HOST_IDENTITY_OPERATOR_SUPPLIED_REQUIRED`
- `HOST_IDENTITY_FUTURE_CAPTURE_AUTHORIZATION_REQUIRED`
- `HOST_IDENTITY_NO_SCAN_OK`
- `HOST_IDENTITY_NO_KNOWN_HOSTS_MUTATION_OK`
- `HOST_IDENTITY_NOT_VERIFIED_YET`

The plan must distinguish operator-supplied host identity data from future
explicitly authorized capture fields. It must not present host-identity
planning as verified host identity.

## Credential Boundary Placeholder Requirements

Evidence must include:

- `CREDENTIAL_BOUNDARY_FIELD_PLAN_OK`
- `CREDENTIAL_MODEL_SELECTION_REQUIRED`
- `CREDENTIAL_STORAGE_BOUNDARY_REQUIRED`
- `SECRET_HANDLING_FORBIDDEN_NOW`
- `NO_SECRET_CREDENTIAL_PLACEHOLDER_ONLY`
- `CREDENTIAL_NOT_READY_FOR_CONNECTION`

The plan must not inspect, create, upload, collect, copy, log, or handle secret
material.

## Capacity/Quota/Retention Field Requirements

Evidence must include:

- `CAPACITY_FIELD_PLAN_OK`
- `QUOTA_FIELD_PLAN_OK`
- `RETENTION_FIELD_PLAN_OK`
- `CAPACITY_EVIDENCE_REQUIRED`
- `CAPACITY_NOT_READY_FOR_BACKUP`

The plan must not assert real target capacity without a named target and
evidence source.

## Monitoring/Alerting/Runbook Field Requirements

Evidence must include:

- `MONITORING_FIELD_PLAN_OK`
- `ALERTING_FIELD_PLAN_OK`
- `OPERATOR_RUNBOOK_FIELD_PLAN_OK`
- `MONITORING_MODEL_ONLY`
- `REAL_MONITORING_SETUP_FORBIDDEN_NOW`

The plan must not configure real monitoring.

## Operator Action Packet Requirements

Evidence must define a future no-secret operator action packet with:

- target-candidate fields;
- host identity fields;
- credential placeholder fields;
- capacity/quota/retention fields;
- monitoring/alerting/runbook fields;
- claim boundaries;
- stop conditions;
- no-secret/no-connection/no-host-key-scan markers.

## Local-Ops Requirements

Evidence must include:

- `LOCAL_OPS_BLOCKED_HISTORY_INDEX_MISSING`
- `LOCAL_OPS_BLOCKED_BACKUP_COVERAGE_GAP`
- `LOCAL_OPS_READY_FOR_AUTHORIZATION`
- `LOCAL_OPS_NOT_PRIMARY_BLOCKER`
- `LOCAL_OPS_BEFORE_REAL_OPERATIONS_REQUIRED`

The plan must decide whether local-ops/history-index is the next successor or a
later prerequisite.

## Public-Claim Boundary Requirements

Evidence must state that:

- target-candidate prerequisite planning is not target setup;
- host-identity prerequisite planning is not verified host identity;
- no remote connection occurs;
- no host-key scan occurs;
- no credential is handled;
- no-secret target/tool harness evidence is not real off-host backup;
- no-secret isolated restore harness evidence is not real restore;
- no-secret key harness evidence is not real key custody/recovery;
- local continuity is not complete disaster recovery;
- off-host encrypted backup is not complete;
- external review is not complete;
- no public claim implies production/public-internet readiness,
  metadata-free/anonymity/untraceable behavior, or hidden attachment size,
  timing metadata, traffic shape, or all metadata.

## Decision Matrix Requirements

The evidence must include a matrix covering:

- target candidate;
- target class;
- operator identity / owner;
- host identity;
- credential model;
- credential storage;
- capacity/quota;
- retention/purge;
- monitoring/alerting;
- operator runbook;
- local-ops/backup-plan;
- external review/public claims.

The matrix must identify status, required operator field, evidence source,
blocker, next action, whether the item must precede remote connection,
credential handling, and real backup, whether it can be modeled no-secret only,
whether it is ready for implementation authorization, and whether it should
become NA-0369.

## Successor Selection Requirements

Evidence must select exactly one successor and must not implement it. Selected
successor:

`NA-0369 -- Metadata Runtime Off-Host Backup Target Candidate / Host Identity Operator Action Packet`

## Required Local Checks

Minimum local validation:

- `git diff --check`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- metadata runtime backup/deploy/rollback harness syntax and fixture run
- restore dry-run harness syntax and fixture run
- key custody/recovery no-secret harness syntax and fixture run
- off-host target/tool no-secret harness syntax and fixture run
- isolated restore no-secret harness syntax and fixture run
- qsl evidence helper queue/decisions checks
- qsl evidence helper scope guard for allowed paths
- qsl evidence helper link-check
- qsl evidence helper leak-scan in added mode
- goal-lint
- classifier proof for changed paths

Additional heavy checks should run when feasible under the directive, including
qshield-cli build/test, demo smoke/stress/soak, metadata phase-2 harnesses,
metadata conformance smoke, qsc send_commit, formal models, NA-0310 suite-id
vector JSON parse, NA-0310 refimpl oracle, full refimpl tests, and qsc NA-0313
harnesses.

## CI Expectations

The PR must include a standalone `Goals: G1, G2, G3, G4, G5` line near the top
of the body. Required checks, including `public-safety`, must pass before
merge. Merge must use a normal merge with `--match-head-commit`, no admin
bypass, no squash, no rebase, no direct push, and no delete-branch flag.

## Successor Handoff

After Packet R merges and post-merge public-safety is green, optional closeout
may mark NA-0368 DONE and restore the exact selected NA-0369 successor as the
sole READY item. The closeout must not implement NA-0369.
