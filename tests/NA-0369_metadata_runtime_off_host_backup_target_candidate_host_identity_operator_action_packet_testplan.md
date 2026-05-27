Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-27

# NA-0369 Metadata Runtime Off-Host Backup Target Candidate / Host Identity Operator Action Packet Testplan

## Objective

Validate that NA-0369 creates a parseable no-secret operator action packet
template for future target-candidate and host-identity evidence without any
target setup, remote connection, host-key scan, `known_hosts` mutation,
credential handling, secret handling, repository init, tool installation,
backup, restore, deploy, rollback, backup script/timer/fstab mutation, runtime
implementation change, or public-claim expansion.

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
- No claim states or implies production readiness, public-internet readiness,
  external-review completion, anonymity, metadata-free behavior,
  untraceability, hidden attachment size, hidden timing metadata, hidden
  traffic shape, hidden all metadata, complete off-host backup, complete
  disaster recovery, real restore completion, host identity verification,
  target configuration, real key custody implementation, or real key recovery
  implementation.

## Allowed Scope

Allowed files:

- `docs/governance/evidence/NA-0369_metadata_runtime_off_host_backup_target_candidate_host_identity_operator_action_packet.md`
- `tests/NA-0369_metadata_runtime_off_host_backup_target_candidate_host_identity_operator_action_packet_testplan.md`
- `inputs/metadata_runtime/off_host_backup_target_candidate_host_identity_operator_action_packet_v1.json`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

Forbidden changes include qsl-server, qsl-attachments, qshield runtime,
qsc/qsp implementation, protocol/crypto, dependencies, workflows, README,
START_HERE, docs/public, website, service configuration, deployment scripts,
rollback scripts, backup scripts, timers, fstab, local backup configuration,
off-host destinations, restore paths, key material, passphrase paths,
credential paths, recovery-envelope content, and any real target/repository/tool
setup.

## Prior Target-Candidate Review Requirements

Validation must confirm the evidence reviews and preserves:

- NA-0368 operator prerequisite plan.
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
`TARGET_CANDIDATE_PACKET_READY`, `TARGET_CANDIDATE_REAL_VALUE_ABSENT`,
`HOST_IDENTITY_REAL_VALUE_ABSENT`, `CREDENTIAL_BOUNDARY_ABSENT`,
`CREDENTIAL_SECRET_HANDLING_FORBIDDEN`,
`CAPACITY_RETENTION_REAL_VALUE_ABSENT`,
`MONITORING_ALERTING_REAL_VALUE_ABSENT`,
`REAL_TARGET_ACCESS_NOT_AUTHORIZED`, and `OFF_HOST_BACKUP_NOT_READY`.

## Operator Packet Field Requirements

Evidence and JSON must include:

- packet metadata;
- target candidate fields;
- host identity fields;
- credential placeholder fields;
- capacity/quota/retention fields;
- monitoring/alerting/runbook fields;
- forbidden inputs;
- stop conditions;
- public claim boundaries;
- future validation markers.

Expected classifications:

- `OPERATOR_PACKET_FIELD_DESIGN_OK`
- `OPERATOR_PACKET_NO_SECRET_DESIGN_OK`
- `OPERATOR_PACKET_NO_CONNECTION_DESIGN_OK`
- `OPERATOR_PACKET_NO_HOST_KEY_SCAN_DESIGN_OK`

## Forbidden Input Requirements

The operator guide must tell the operator not to provide private keys,
passphrases, passwords, tokens, raw credentials, recovery-envelope contents,
secret paths, private material paths, screenshots or command output containing
secrets, live connection outputs, unredacted `known_hosts` content if
sensitive, or host fingerprints unless a future directive explicitly
authorizes collection and storage.

## JSON Parse Requirements

Run:

- `python3 -m json.tool inputs/metadata_runtime/off_host_backup_target_candidate_host_identity_operator_action_packet_v1.json >/dev/null`

The JSON must parse successfully and remain deterministic.

## No-Secret Validation Requirements

Validation must confirm:

- no real target endpoint;
- no real host identity value;
- no real host-key fingerprint;
- no real credential;
- no private key material;
- no recovery-envelope contents;
- no sensitive path;
- no executable command;
- no connection command;
- no backup or restore command;
- all future operator values are placeholders marked
  `REQUIRED_OPERATOR_INPUT`, `REQUIRED_BEFORE_CONNECTION`, or
  `FUTURE_AUTHORIZATION_REQUIRED`;
- sensitive words appear only as prohibited-input guidance, not as actual
  secret fields or values.

## Target Candidate Field Requirements

The packet must include target label/alias, target class, owner/contact label,
jurisdiction/location class, trust boundary, reachability class, intended
repository path class, capacity estimate, retention intent, availability,
failure mode, cost/quota concern, emergency stop contact, real-host gate, and
future connection authorization marker.

## Host Identity Field Requirements

The packet must include evidence source type, fingerprint algorithm,
fingerprint format, fingerprint placeholder, source timestamp, verifier label,
future pinning storage plan, mismatch response, rotation response, revocation
response, capture method gate, and `known_hosts` mutation gate. The packet must
not present planning as verified host identity.

## Credential Placeholder Requirements

The packet must include credential model class, storage boundary, rotation
expectation, revocation expectation, no-secret proof requirement, credential
material prohibition, and connection-use authorization gate.

## Capacity/Quota/Retention Field Requirements

The packet must include expected source size, expected growth, minimum
capacity, minimum free-space threshold, transfer estimate, cost boundary,
retention window/count, prune model, capacity evidence source, and capacity
stop condition.

## Monitoring/Alerting/Runbook Field Requirements

The packet must include backup success, backup failure, missed backup, target
unreachable, capacity, retention/prune, restore-drill, and credential/key error
alert fields; notification destination class; acknowledgement; emergency stop;
manual verification; escalation boundary; and real monitoring setup gate.

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

- the operator packet is not target setup;
- the target-candidate template is not a configured target;
- host-identity placeholder is not verified host identity;
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

The evidence must include a matrix covering target candidate packet, target
class, operator identity/owner, host identity, credential placeholder,
capacity/quota, retention/purge, monitoring/alerting, operator runbook,
local-ops/backup-plan, and external review/public claims.

Each row must identify status, required operator field, placeholder presence,
future operator response requirement, evidence source, blocker, next action,
and whether it must precede remote connection, credential handling, or real
backup.

## Successor Selection Requirements

If no operator response data is present, select:

`NA-0370 -- Metadata Runtime Off-Host Backup Target Candidate / Host Identity Operator Response Intake`

Do not implement NA-0370.

## Required Local Checks

Run:

- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- JSON parse for the NA-0369 packet
- existing metadata runtime harness syntax/fixture checks
- directly runnable qshield/qsc/formal/refimpl checks where feasible
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main`
  with exact allowed paths
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- classifier proof for the changed path set
- goal-lint using the PR body

## CI Expectations

The PR must merge only after required qsl-protocol checks complete normally and
`public-safety` is green. No admin bypass, direct push, squash, rebase, or
delete-branch flag is allowed.

## Successor Handoff

After merge, NA-0369 remains READY until optional closeout. Closeout, if
authorized by a later packet in this directive, may mark NA-0369 DONE and
restore exactly one READY successor: the selected NA-0370 operator response
intake lane. The closeout must not implement NA-0370.
