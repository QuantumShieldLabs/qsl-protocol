Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-27

# NA-0372 Metadata Runtime Off-Host Backup Target Candidate / Host Identity Operator Response Intake After Collection Request Testplan

## Objective

Validate that NA-0372 intakes any deliberate no-secret operator response
created from the NA-0371 collection request, or records
`OPERATOR_RESPONSE_STILL_ABSENT` when no response exists, while preserving
no-secret, no-connection, no-host-key-scan, no-real-operation, service,
backup-plan, and public-claim boundaries.

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

- `docs/governance/evidence/NA-0372_metadata_runtime_off_host_backup_target_candidate_host_identity_operator_response_intake_after_collection_request.md`
- `tests/NA-0372_metadata_runtime_off_host_backup_target_candidate_host_identity_operator_response_intake_after_collection_request_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

The optional intake status JSON artifact is not required for this testplan
because the live NA-0372 queue entry does not explicitly authorize a new
`inputs/` status artifact.

## Forbidden Scope

Forbidden changes include qsl-server, qsl-attachments, qshield runtime,
qsc/qsp implementation, protocol/crypto, dependencies, workflows, README,
START_HERE, docs/public, website, service configuration, deployment scripts,
rollback scripts, backup scripts, timers, fstab, local backup configuration,
off-host destinations, restore paths, key material, passphrase paths,
credential paths, recovery-envelope content, and any real
target/repository/tool setup.

## Prior Collection-Request Review Requirements

Validation must confirm the evidence reviews and preserves:

- live NA-0372 scope;
- NA-0371 no-secret collection request;
- NA-0370 response intake result `OPERATOR_RESPONSE_NOT_PRESENT`;
- NA-0369 no-secret operator action packet;
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
`COLLECTION_REQUEST_CREATED`, `OPERATOR_RESPONSE_STILL_ABSENT`,
`TARGET_CANDIDATE_REAL_VALUE_ABSENT`,
`HOST_IDENTITY_REAL_VALUE_ABSENT`, `CREDENTIAL_BOUNDARY_ABSENT`,
`CREDENTIAL_SECRET_HANDLING_FORBIDDEN`,
`CAPACITY_RETENTION_REAL_VALUE_ABSENT`,
`MONITORING_ALERTING_REAL_VALUE_ABSENT`,
`REAL_TARGET_ACCESS_NOT_AUTHORIZED`, and `OFF_HOST_BACKUP_NOT_READY` when no
response candidate is found.

## Response Discovery Requirements

Validation must confirm authorized discovery only:

- list files under `/home/victor/work/qsl/codex/requests/` whose names or
  first lines indicate NA-0371 operator response, NA-0372 intake response,
  off-host target candidate response, host identity response, or target
  candidate operator packet response;
- list files under qsl-protocol `inputs/metadata_runtime/` matching operator
  response, target-candidate response, or host-identity response terms;
- classify `NO_RESPONSE_CANDIDATE_FOUND`,
  `RESPONSE_CANDIDATE_FOUND_SAFE_TO_PARSE`,
  `RESPONSE_CANDIDATE_FOUND_NEEDS_OPERATOR_CONFIRMATION`, or
  `RESPONSE_CANDIDATE_SECRET_RISK_STOP`;
- if no response candidate exists, record `OPERATOR_RESPONSE_STILL_ABSENT`.

Discovery must not recursively scan arbitrary home directories, inspect
private key files, inspect `.ssh` secret material, inspect credential stores,
or quote suspected secrets.

## Sensitive-Material Stop Requirements

Validation must confirm the evidence says to stop without quoting or copying
content if a candidate response contains private keys, passphrases, passwords,
tokens, raw credentials, recovery-envelope contents, private material paths,
secret-bearing command output, unredacted sensitive `known_hosts` content, live
connection output, or host fingerprint material supplied outside an authorized
storage boundary.

If a host fingerprint is later supplied in a safe authorized no-secret context,
it can only be classified as `HOST_IDENTITY_VALUE_PRESENT_UNVERIFIED` until a
future validation lane performs authorized verification.

## Response Classification Requirements

Evidence must classify:

- operator response status;
- target candidate status;
- host identity status;
- credential response status;
- capacity/retention status;
- monitoring/runbook status;
- real target connection blocker;
- real backup operation blocker.

Expected no-response classifications:

- `OPERATOR_RESPONSE_STILL_ABSENT`
- `TARGET_CANDIDATE_VALUE_ABSENT`
- `HOST_IDENTITY_VALUE_ABSENT`
- `CREDENTIAL_PLACEHOLDER_ONLY`
- `CREDENTIAL_VALUE_FORBIDDEN`
- `CAPACITY_RETENTION_VALUE_ABSENT`
- `MONITORING_RUNBOOK_VALUE_ABSENT`
- `REAL_TARGET_CONNECTION_BLOCKED`
- `REAL_BACKUP_OPERATION_BLOCKED`

## Completeness Matrix Requirements

Evidence must include a matrix covering target label, target class,
owner/contact label, trust boundary, host identity evidence source,
fingerprint algorithm, fingerprint format, fingerprint value, credential model
class, credential storage boundary, capacity estimate, retention intent,
monitoring destination class, operator runbook owner, emergency stop contact,
public-claim boundary acknowledgement, and no-secret affirmation.

For each row, evidence must record status, raw value storage, redacted summary,
blocker, next action, future authorization requirement, and whether the row can
precede remote connection, credential handling, real backup, or NA-0373
successor selection.

## No-Secret Validation Requirements

Run:

- `python3 -m json.tool inputs/metadata_runtime/off_host_backup_target_candidate_host_identity_operator_action_packet_v1.json >/dev/null`
- `python3 -m json.tool inputs/metadata_runtime/off_host_backup_target_candidate_host_identity_operator_response_collection_request_v1.json >/dev/null`

Validation must confirm:

- no real target endpoint;
- no real host identity value;
- no real host-key fingerprint;
- no real credential;
- no private key material;
- no recovery-envelope contents;
- no sensitive private-material path;
- no executable command;
- no connection command;
- no backup or restore command;
- request markers and forbidden-input guidance are present.

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
- response intake is not target setup;
- response intake is not host identity verification;
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

Evidence must include a matrix covering operator response existence, target
candidate, host identity, credential boundary, capacity/quota, retention/purge,
monitoring/alerting, operator runbook, local-ops/backup-plan, and external
review/public claims.

Each matrix row must identify status, evidence source, blocker, next action,
whether additional operator response collection is needed, whether credential
boundary planning is needed, whether target/host validation can proceed, and
whether the row must precede remote connection, credential handling, or real
backup.

## Successor Selection Requirements

If no operator response data is present, the selected successor must be:

`NA-0373 -- Metadata Runtime Off-Host Backup Operator Response Availability Blocker / Collection Follow-Up Plan`

Evidence must reject response completion, response validation, credential
boundary, tool installation, key custody/recovery, local-ops, external-review,
website/public-claim, and public-paper successors as premature unless current
evidence supports a different exact successor.

## Required Local Checks

Run:

- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- JSON parse checks for NA-0369 and NA-0371 input artifacts
- metadata runtime no-secret harness syntax and execution checks
- qshield/qsc/formal/demo checks as feasible for this governance lane
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main` with exact allowed paths
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- goal-lint
- changed-path classifier proof

## CI Expectations

- The PR must include a standalone Goals line:
  `Goals: G1, G2, G3, G4, G5`.
- Required checks, including public-safety, must complete green before merge.
- Merge must use normal merge with `--match-head-commit`.
- Do not use admin bypass, direct push, squash, rebase, or delete-branch flags.
- After merge, post-merge public-safety must complete success.

## Successor Handoff

The expected no-response successor is:

`NA-0373 -- Metadata Runtime Off-Host Backup Operator Response Availability Blocker / Collection Follow-Up Plan`

The NA-0372 implementation PR must leave NA-0372 READY pending a separate
closeout PR. NA-0373 must not be implemented by NA-0372.
