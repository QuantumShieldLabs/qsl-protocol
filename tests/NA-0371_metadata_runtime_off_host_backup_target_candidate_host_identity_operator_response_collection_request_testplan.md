Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-27

# NA-0371 Metadata Runtime Off-Host Backup Target Candidate / Host Identity Operator Response Collection Request Testplan

## Objective

Validate that NA-0371 creates a parseable no-secret operator response
collection request after NA-0370 recorded `OPERATOR_RESPONSE_NOT_PRESENT`, and
that it selects the exact NA-0372 intake-after-request successor without target
setup, remote connection, host-key scan, `known_hosts` mutation, credential
handling, secret handling, repository init, tool installation, backup, restore,
deploy, rollback, runtime mutation, dependency change, workflow change, or
public-claim expansion.

## Protected Invariants

- qsl-protocol is the only mutable repository.
- qsl-server and qsl-attachments are read-only.
- qshield runtime is unchanged.
- qsc/qsp/protocol/crypto/key-schedule implementation is unchanged.
- Cargo dependencies and workflows are unchanged.
- README, START_HERE, website, and docs/public are unchanged.
- Backup scripts, timers, fstab, service units, source lists, restore paths,
  rollback paths, remote destinations, key paths, credential paths, recovery
  envelope contents, and local backup configuration are unchanged.
- No remote/off-host connection, host-key scan, `known_hosts` mutation,
  repository init, tool installation, backup, restore, deploy, rollback, real
  restore target creation/mount/copy, real key generation, key upload,
  passphrase collection, credential handling, private-key inspection, recovery
  envelope content creation, or secret handling occurs.
- No claim states or implies production readiness, public-internet readiness,
  external-review completion, anonymity, metadata-free behavior,
  untraceability, hidden attachment size, hidden timing metadata, hidden
  traffic shape, hidden all metadata, complete off-host backup, complete
  disaster recovery, real restore completion, host identity verification,
  target configuration, real key custody implementation, or real key recovery
  implementation.

## Allowed Scope

Allowed files:

- `docs/governance/evidence/NA-0371_metadata_runtime_off_host_backup_target_candidate_host_identity_operator_response_collection_request.md`
- `tests/NA-0371_metadata_runtime_off_host_backup_target_candidate_host_identity_operator_response_collection_request_testplan.md`
- `inputs/metadata_runtime/off_host_backup_target_candidate_host_identity_operator_response_collection_request_v1.json`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

Forbidden changes include qsl-server, qsl-attachments, qshield runtime,
qsc/qsp implementation, protocol/crypto, dependencies, workflows, README,
START_HERE, docs/public, website, service configuration, deployment scripts,
rollback scripts, backup scripts, timers, fstab, local backup configuration,
off-host destinations, restore paths, key material, passphrase paths,
credential paths, recovery envelope content, and any real
target/repository/tool setup.

## Prior Response-Intake Review Requirements

Validation must confirm the evidence reviews and preserves:

- live NA-0371 scope;
- NA-0370 response intake result `OPERATOR_RESPONSE_NOT_PRESENT`;
- NA-0369 no-secret operator action packet;
- NA-0368 operator prerequisite field plan;
- NA-0367 target-access / host-identity prerequisite result;
- NA-0366 real target/tool blocker-resolution result;
- NA-0365 no-secret isolated restore harness boundary;
- NA-0363 no-secret off-host target/tool harness boundary;
- NA-0361 no-secret key custody/recovery harness boundary;
- NA-0359 no-secret restore dry-run harness boundary.

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
`OPERATOR_PACKET_EXISTS`, `OPERATOR_RESPONSE_NOT_PRESENT`,
`COLLECTION_REQUEST_READY`, `TARGET_CANDIDATE_REAL_VALUE_ABSENT`,
`HOST_IDENTITY_REAL_VALUE_ABSENT`, `CREDENTIAL_BOUNDARY_ABSENT`,
`CREDENTIAL_SECRET_HANDLING_FORBIDDEN`,
`CAPACITY_RETENTION_REAL_VALUE_ABSENT`,
`MONITORING_ALERTING_REAL_VALUE_ABSENT`,
`REAL_TARGET_ACCESS_NOT_AUTHORIZED`, and `OFF_HOST_BACKUP_NOT_READY`.

## Collection Request Field Requirements

Evidence and JSON must include:

- request metadata and purpose;
- where the operator should place the future response;
- target candidate non-secret fields;
- host identity non-secret fields;
- credential placeholder fields;
- capacity/quota/retention fields;
- monitoring/alerting/runbook fields;
- public-claim acknowledgement fields;
- forbidden inputs;
- stop conditions;
- future validation markers.

Expected classifications:

- `COLLECTION_REQUEST_FIELD_DESIGN_OK`
- `OPERATOR_RESPONSE_COLLECTION_REQUIRED`
- `NO_SECRET_COLLECTION_REQUEST_OK`
- `NO_REMOTE_CONNECTION_COLLECTION_REQUEST_OK`
- `NO_HOST_KEY_SCAN_COLLECTION_REQUEST_OK`

## Forbidden Input Requirements

The operator guide must tell the operator not to provide private keys,
passphrases, passwords, tokens, raw credentials, recovery envelope contents,
secret paths, private material paths, screenshots or command output containing
secrets, live connection outputs, unredacted `known_hosts` content if
sensitive, or host fingerprints unless a future directive explicitly
authorizes collection and storage.

## JSON Parse Requirements If Artifact Added

Run:

- `python3 -m json.tool inputs/metadata_runtime/off_host_backup_target_candidate_host_identity_operator_response_collection_request_v1.json >/dev/null`

The JSON must parse successfully and remain deterministic.

## No-Secret Validation Requirements

Validation must confirm:

- no real target endpoint;
- no real host identity value;
- no real host-key fingerprint;
- no real credential;
- no private key material;
- no recovery envelope contents;
- no sensitive private-material path;
- no executable command;
- no connection command;
- no backup or restore command;
- all requested values are marked `REQUEST_OPERATOR_NON_SECRET_INPUT`,
  `DO_NOT_INCLUDE_SECRET`, `REQUIRED_BEFORE_CONNECTION`, or
  `FUTURE_AUTHORIZATION_REQUIRED`;
- no-secret/no-connection/no-host-key-scan markers are present;
- sensitive words appear only as prohibited-input guidance, not as actual
  secret fields or values.

## Local-Ops Requirements

Evidence must include:

- `LOCAL_OPS_BLOCKED_HISTORY_INDEX_MISSING`
- `LOCAL_OPS_BLOCKED_BACKUP_COVERAGE_GAP`
- `LOCAL_OPS_READY_FOR_AUTHORIZATION`
- `LOCAL_OPS_NOT_PRIMARY_BLOCKER`
- `LOCAL_OPS_BEFORE_REAL_OPERATIONS_REQUIRED`

The evidence must decide whether local-ops/history-index is the next successor
or a later prerequisite.

## Public-Claim Boundary Requirements

Evidence must preserve:

- external review remains incomplete;
- collection request is not target setup;
- collection request is not operator response intake;
- collection request is not host identity verification;
- no remote connection occurs;
- no host-key scan occurs;
- no credential is handled;
- no-secret target/tool harness is not real off-host backup;
- no-secret isolated restore harness is not real restore;
- no-secret key harness is not real key custody/recovery;
- local continuity is not complete disaster recovery;
- off-host encrypted backup is not complete;
- no website/public-doc update is made.

## Decision Matrix Requirements

Evidence must include a matrix covering collection request, response location,
target candidate, host identity, credential boundary, capacity/quota,
retention/purge, monitoring/alerting, operator runbook, local-ops/backup-plan,
and external review/public claims.

Each matrix row must identify status, requested field, forbidden data, blocker,
next action, and whether the row drives NA-0372.

Expected categories include:

- `COLLECTION_REQUEST_CREATED`
- `OPERATOR_RESPONSE_STILL_REQUIRED`
- `TARGET_CANDIDATE_RESPONSE_REQUIRED`
- `HOST_IDENTITY_RESPONSE_REQUIRED`
- `CREDENTIAL_BOUNDARY_PREREQUISITE_REQUIRED`
- `LOCAL_OPS_PREREQUISITE_REQUIRED`
- `REAL_TARGET_CONNECTION_BLOCKED`
- `REAL_BACKUP_OPERATION_BLOCKED`

## Successor Selection Requirements

Validation must confirm selected successor:

`NA-0372 -- Metadata Runtime Off-Host Backup Target Candidate / Host Identity Operator Response Intake After Collection Request`

Rejected alternatives must be recorded, and NA-0372 must not be implemented.

## Required Local Checks

Run at minimum:

- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- `python3 -m json.tool inputs/metadata_runtime/off_host_backup_target_candidate_host_identity_operator_action_packet_v1.json >/dev/null`
- `python3 -m json.tool inputs/metadata_runtime/off_host_backup_target_candidate_host_identity_operator_response_collection_request_v1.json >/dev/null`
- metadata runtime no-secret harness syntax, fixture parse, and harness checks
  for production backup/deploy/rollback, restore dry-run, key
  custody/recovery, off-host target/tool, and isolated restore;
- directly runnable qshield/qsc/formal/refimpl checks required by the
  directive;
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main`
  with the exact allowed paths;
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- goal-lint;
- classifier proof for changed paths.

## CI Expectations

The qsl-protocol PR must merge only after required checks complete normally and
`public-safety` is required and green. The PR body must include a standalone
`Goals: G1, G2, G3, G4, G5` line near the top.

## Successor Handoff

Packet P must leave READY as NA-0371. Packet Q, if executed after Packet P
merge and green post-merge public-safety, may close NA-0371 and restore the
exact selected NA-0372 successor without implementing NA-0372.
