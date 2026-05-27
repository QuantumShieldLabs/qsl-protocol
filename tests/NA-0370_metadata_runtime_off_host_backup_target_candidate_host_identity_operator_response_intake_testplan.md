Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-27

# NA-0370 Metadata Runtime Off-Host Backup Target Candidate / Host Identity Operator Response Intake Testplan

## Objective

Validate that NA-0370 performs a fail-closed intake of any operator response to
the NA-0369 no-secret target-candidate / host-identity operator action packet,
records `OPERATOR_RESPONSE_NOT_PRESENT` when no authorized response exists, and
selects the exact next successor without target setup, remote connection,
host-key scan, credential handling, secret handling, repository init, tool
installation, backup, restore, deploy, rollback, runtime mutation, dependency
change, workflow change, or public-claim expansion.

## Protected Invariants

- qsl-protocol is the only mutable repository.
- qsl-server and qsl-attachments are read-only.
- qshield runtime is unchanged.
- qsc/qsp/protocol/crypto/key-schedule implementation is unchanged.
- Cargo dependencies and workflows are unchanged.
- README, START_HERE, website, and docs/public are unchanged.
- Backup scripts, timers, fstab, service units, source lists, restore paths,
  rollback paths, remote destinations, key paths, credential paths,
  recovery-envelope contents, and local backup configuration are unchanged.
- No remote/off-host connection, host-key scan, `known_hosts` mutation,
  repository init, tool installation, backup, restore, deploy, rollback, real
  restore target creation/mount/copy, real key generation, key upload,
  passphrase collection, credential handling, private-key inspection,
  recovery-envelope content creation, or secret handling occurs.
- No claim states or implies production readiness.
- No claim states or implies public-internet readiness.
- No claim states or implies external-review completion.
- No claim states or implies anonymity.
- No claim states or implies metadata-free behavior.
- No claim states or implies untraceability.
- No claim states or implies hidden attachment size.
- No claim states or implies hidden timing metadata.
- No claim states or implies hidden traffic shape.
- No claim states or implies hidden all metadata.
- No claim states or implies complete off-host backup.
- No claim states or implies complete disaster recovery.
- No claim states or implies real restore completion.
- No claim states or implies host identity verification.
- No claim states or implies target configuration.
- No claim states or implies real key custody implementation.
- No claim states or implies real key recovery implementation.

## Allowed Scope

Allowed files:

- `docs/governance/evidence/NA-0370_metadata_runtime_off_host_backup_target_candidate_host_identity_operator_response_intake.md`
- `tests/NA-0370_metadata_runtime_off_host_backup_target_candidate_host_identity_operator_response_intake_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

The optional qsl-protocol intake status JSON artifact must not be added unless
live scope explicitly authorizes the `inputs/` response-status path.

## Forbidden Scope

Forbidden changes include qsl-server, qsl-attachments, qshield runtime,
qsc/qsp implementation, protocol/crypto, dependencies, workflows, README,
START_HERE, docs/public, website, service configuration, deployment scripts,
rollback scripts, backup scripts, timers, fstab, local backup configuration,
off-host destinations, restore paths, key material, passphrase paths,
credential paths, recovery-envelope content, and any real
target/repository/tool setup.

## Prior Operator-Packet Review Requirements

Validation must confirm the evidence reviews and preserves:

- NA-0369 operator action packet.
- NA-0368 operator prerequisite plan.
- NA-0367 target-access and host-identity prerequisite result.
- NA-0366 real target/tool blocker-resolution result.
- NA-0365 no-secret isolated restore harness boundary.
- NA-0363 no-secret off-host target/tool harness boundary.
- NA-0361 no-secret key custody/recovery harness boundary.
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

- local continuity backup mount status if available;
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
`OPERATOR_PACKET_EXISTS`, `OPERATOR_RESPONSE_NOT_PRESENT`,
`TARGET_CANDIDATE_REAL_VALUE_ABSENT`, `HOST_IDENTITY_REAL_VALUE_ABSENT`,
`CREDENTIAL_BOUNDARY_ABSENT`, `CREDENTIAL_SECRET_HANDLING_FORBIDDEN`,
`CAPACITY_RETENTION_REAL_VALUE_ABSENT`,
`MONITORING_ALERTING_REAL_VALUE_ABSENT`,
`REAL_TARGET_ACCESS_NOT_AUTHORIZED`, and `OFF_HOST_BACKUP_NOT_READY`.

## Operator Response Discovery Requirements

Validation must confirm discovery is limited to the authorized read-only
locations and filename patterns:

- local Codex requests files whose names indicate NA-0369/NA-0370, operator
  response, target-candidate response, or host-identity response;
- qsl-protocol metadata input files matching operator-response,
  target-candidate-response, or host-identity-response patterns.

Discovery must record exact candidate files if any. If no candidate exists,
evidence must record `NO_RESPONSE_CANDIDATE_FOUND` and
`OPERATOR_RESPONSE_NOT_PRESENT`.

## Sensitive-Material Stop Requirements

If a candidate response contains private keys, passphrases, passwords, tokens,
raw credentials, recovery-envelope contents, private material paths,
secret-bearing command output, live connection output, or sensitive unredacted
host-identity material, intake must stop and report only the path and generic
sensitive category.

The evidence must not copy or quote sensitive content.

## Response Field Classification Requirements

Evidence must classify:

- target label;
- target class;
- target owner/contact label;
- trust boundary;
- host identity evidence source;
- fingerprint algorithm;
- fingerprint format;
- fingerprint value;
- credential model class;
- credential storage boundary;
- capacity estimate;
- retention intent;
- monitoring destination class;
- operator runbook owner;
- emergency stop contact;
- public-claim boundary acknowledgement;
- no-secret affirmation.

Each field must be classified as present, absent, invalid, secret-risk, or
future-authorization-required. If no response exists, fields must be absent or
future-authorization-required as appropriate.

## No-Secret Validation Requirements

Validation must confirm:

- NA-0369 action packet JSON parses;
- no response candidate was copied or parsed when absent;
- no real target endpoint is stored;
- no real host identity value is stored;
- no real credential is stored;
- no private key material is stored;
- no recovery-envelope contents are stored;
- no sensitive path is stored;
- no executable command is stored;
- no connection command is stored;
- no backup or restore command is stored;
- sensitive-material stop behavior is defined.

## Local-Ops Requirements

Evidence must analyze:

- backup plan coverage;
- same-host continuity limitation;
- local ops authorization gaps;
- directive/response/request/journal/ops history coverage;
- D132 bundle status;
- workflow-support request relevance;
- whether local-ops/history-index is the primary next blocker.

Expected classifications include `LOCAL_OPS_BLOCKED_HISTORY_INDEX_MISSING`,
`LOCAL_OPS_BLOCKED_BACKUP_COVERAGE_GAP`,
`LOCAL_OPS_READY_FOR_AUTHORIZATION`, `LOCAL_OPS_NOT_PRIMARY_BLOCKER`, and
`LOCAL_OPS_BEFORE_REAL_OPERATIONS_REQUIRED`.

## Public-Claim Boundary Requirements

Evidence must preserve:

- external review remains incomplete;
- operator response intake is not target setup;
- target-candidate response, if present in future, is not target connection;
- host identity response, if present in future, is not live verification;
- no remote connection occurs;
- no host-key scan occurs;
- no credential is handled;
- no-secret harnesses are not real off-host backup, real restore, or real key
  custody/recovery;
- local continuity is not complete disaster recovery;
- off-host encrypted backup is not complete;
- no public claim implies production/public-internet readiness;
- no public claim implies metadata-free behavior;
- no public claim implies anonymity;
- no public claim implies untraceability;
- no public claim implies hidden attachment size;
- no public claim implies hidden timing metadata;
- no public claim implies hidden traffic shape;
- no public claim implies hidden all metadata.

## Decision Matrix Requirements

Evidence must include a decision matrix for:

- operator response existence;
- target candidate value;
- host identity value;
- credential boundary;
- capacity/quota;
- retention/purge;
- monitoring/alerting;
- operator runbook;
- local-ops/backup-plan;
- external review/public claims.

The matrix must explain blockers, next actions, whether collection is needed,
whether credential boundary planning is needed, whether target/host validation
can proceed, and whether the item should become NA-0371.

## Successor Selection Requirements

If no operator response is present, selected successor must be:

`NA-0371 -- Metadata Runtime Off-Host Backup Target Candidate / Host Identity Operator Response Collection Request`

The evidence must reject validation, credential boundary, tool installation,
key custody/recovery, local-ops, external-review, website/public-claim, and
public paper successors unless evidence makes one of those the primary blocker.

## Required Local Checks

Run at minimum:

- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- `python3 -m json.tool inputs/metadata_runtime/off_host_backup_target_candidate_host_identity_operator_action_packet_v1.json >/dev/null`
- metadata runtime backup/deploy/rollback, restore dry-run, key custody,
  off-host target/tool, and isolated restore no-secret harness checks;
- directly runnable qshield metadata runtime harnesses;
- qshield CLI build/test if feasible;
- demo smoke/stress/soak if feasible;
- metadata runtime identifier/padding and sanitized-errors/retention harnesses;
- metadata conformance smoke;
- qsc send_commit targeted test;
- formal model checks;
- NA-0310 vector JSON parse and refimpl oracle;
- queue and decisions helpers;
- scope guard;
- link-check;
- leak-scan;
- goal-lint;
- classifier proof for changed paths.

## CI Expectations

The qsl-protocol PR must include the required PR metadata line `Goals: G1, G2,
G3, G4, G5`, pass required checks normally, and keep `public-safety` required
and green before merge and after merge.

## Successor Handoff

After Packet P merges and post-merge `public-safety` is green, closeout may
mark NA-0370 DONE and restore exactly one READY successor:

`NA-0371 -- Metadata Runtime Off-Host Backup Target Candidate / Host Identity Operator Response Collection Request`

The closeout must not implement NA-0371.
