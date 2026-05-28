Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-28

# NA-0373 Metadata Runtime Off-Host Backup Operator Response Availability Blocker Collection Follow-Up Testplan

## Objective

Validate that NA-0373 records the response-availability blocker after NA-0371
created a no-secret collection request and NA-0372 found
`OPERATOR_RESPONSE_STILL_ABSENT`, then produces a clear no-secret collection
follow-up plan and selects an exact NA-0374 successor while preserving
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

- `docs/governance/evidence/NA-0373_metadata_runtime_off_host_backup_operator_response_availability_blocker_collection_follow_up_plan.md`
- `tests/NA-0373_metadata_runtime_off_host_backup_operator_response_availability_blocker_collection_follow_up_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

The optional follow-up JSON artifact is not required for this testplan because
the live NA-0373 queue entry does not explicitly authorize a new durable
`inputs/metadata_runtime/` artifact path.

## Forbidden Scope

Forbidden changes include qsl-server, qsl-attachments, qshield runtime,
qsc/qsp implementation, protocol/crypto, dependencies, workflows, README,
START_HERE, docs/public, website, service configuration, deployment scripts,
rollback scripts, backup scripts, timers, fstab, local backup configuration,
off-host destinations, restore paths, key material, passphrase paths,
credential paths, recovery-envelope content, and any real
target/repository/tool setup.

## Prior Response-Intake Review Requirements

Validation must confirm the evidence reviews and preserves:

- live NA-0373 scope;
- NA-0372 response absence result `OPERATOR_RESPONSE_STILL_ABSENT`;
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
`OPERATOR_RESPONSE_AVAILABILITY_BLOCKER`,
`TARGET_CANDIDATE_REAL_VALUE_ABSENT`,
`HOST_IDENTITY_REAL_VALUE_ABSENT`, `CREDENTIAL_BOUNDARY_ABSENT`,
`CREDENTIAL_SECRET_HANDLING_FORBIDDEN`,
`CAPACITY_RETENTION_REAL_VALUE_ABSENT`,
`MONITORING_ALERTING_REAL_VALUE_ABSENT`,
`REAL_TARGET_ACCESS_NOT_AUTHORIZED`, and `OFF_HOST_BACKUP_NOT_READY`.

## Response-Availability Blocker Requirements

Validation must confirm authorized discovery only:

- list files under `/home/victor/work/qsl/codex/requests/` whose names or
  first lines indicate NA-0371 operator response, NA-0372 intake response,
  NA-0373 follow-up response, off-host target candidate response, host identity
  response, or target candidate operator packet response;
- list files under qsl-protocol `inputs/metadata_runtime/` matching operator
  response, target-candidate response, or host-identity response terms;
- classify `RESPONSE_STILL_ABSENT`, `RESPONSE_NOW_PRESENT_NO_SECRET`,
  `RESPONSE_NOW_PRESENT_INVALID`, or `RESPONSE_NOW_PRESENT_SECRET_RISK_STOP`;
- if no response candidate exists, record `OPERATOR_RESPONSE_STILL_ABSENT` and
  `OPERATOR_RESPONSE_AVAILABILITY_BLOCKER`.

Discovery must not recursively scan arbitrary home directories, inspect
private key files, inspect `.ssh` secret material, inspect credential stores,
or quote suspected secrets.

## Collection Follow-Up Requirements

Evidence must answer:

- what exact response is missing;
- what exact artifact already requests it;
- why response absence blocks target/host validation;
- why credential/tool/key/restore work remains premature;
- whether the existing NA-0371 request is sufficient;
- whether a follow-up request artifact is needed;
- whether operator action must occur outside Codex;
- whether local-ops/history-index should happen before another intake attempt;
- what NA-0374 should do.

Expected classifications:

- `OPERATOR_RESPONSE_AVAILABILITY_BLOCKER_OK`
- `COLLECTION_REQUEST_REFERENCE_OK`
- `OPERATOR_RESPONSE_STILL_REQUIRED_OK`
- `TARGET_CANDIDATE_RESPONSE_REQUIRED_OK`
- `HOST_IDENTITY_RESPONSE_REQUIRED_OK`
- `CREDENTIAL_PLACEHOLDER_ONLY_OK`
- `NO_REMOTE_CONNECTION_OK`
- `NO_HOST_KEY_SCAN_OK`
- `NO_SECRET_MATERIAL_OK`

## Forbidden Input Requirements

Evidence must tell the operator to provide only non-secret fields and never
provide private keys, passphrases, passwords, tokens, raw credentials,
recovery-envelope contents, secret paths, private material paths, screenshots
or command output containing secrets, live connection outputs, unredacted
`known_hosts` content if sensitive, or real host fingerprints unless a future
directive explicitly authorizes collection/storage.

## JSON Parse Requirements If Artifact Added

If a follow-up request artifact is added, run:

- `python3 -m json.tool inputs/metadata_runtime/off_host_backup_operator_response_collection_follow_up_request_v1.json >/dev/null`

For this NA-0373 patch, no follow-up JSON artifact is expected unless live
scope is updated to explicitly authorize the `inputs/` path.

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
- response-availability blocker planning is not target setup;
- response-availability blocker planning is not host identity verification;
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

Evidence must include a matrix covering response availability, existing
collection request, follow-up request, target candidate, host identity,
credential boundary, capacity/quota, retention/purge, monitoring/alerting,
operator runbook, local-ops/backup-plan, and external review/public claims.

Each matrix row must identify status, evidence source, blocker, next action,
whether additional operator response collection is needed, whether credential
boundary planning is needed, whether target/host validation can proceed, and
whether the row must precede remote connection, credential handling, or real
backup.

## Successor Selection Requirements

If no operator response data is present and the follow-up plan is clear, the
selected successor should be:

`NA-0374 -- Metadata Runtime Off-Host Backup Operator Response Intake After Collection Follow-Up`

If evidence shows another intake would loop uselessly, select:

`NA-0374 -- Metadata Runtime Off-Host Backup Operator Response Required Stop / Await Operator Input`

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

`NA-0374 -- Metadata Runtime Off-Host Backup Operator Response Intake After Collection Follow-Up`

The NA-0373 implementation PR must leave NA-0373 READY pending a separate
closeout PR. NA-0374 must not be implemented by NA-0373.
