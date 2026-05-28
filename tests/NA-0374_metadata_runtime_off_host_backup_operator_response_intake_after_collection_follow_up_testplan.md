Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-28

# NA-0374 Metadata Runtime Off-Host Backup Operator Response Intake After Collection Follow-Up Test Plan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0374 searches authorized response locations after the NA-0373
collection follow-up, records the operator response as still absent, preserves
all no-secret/no-connection boundaries, and selects the exact NA-0375 required
stop successor.

## Protected Invariants

- Exactly one READY item remains during NA-0374 evidence work.
- READY remains NA-0374 until a separate closeout restores NA-0375.
- D-0730 exists once after NA-0374 evidence lands.
- D-0731 remains absent until closeout.
- Response absence is not treated as target candidate evidence.
- Host identity response evidence, if supplied in a future lane, is not treated
  as live verification without future authorization.
- Local continuity is not presented as complete disaster recovery.
- Off-host encrypted backup is not presented as complete.

## Allowed Scope

- `docs/governance/evidence/NA-0374_metadata_runtime_off_host_backup_operator_response_intake_after_collection_follow_up.md`
- `tests/NA-0374_metadata_runtime_off_host_backup_operator_response_intake_after_collection_follow_up_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

The optional NA-0374 status JSON is allowed only if live NA-0374 scope
explicitly permits it. This lane does not add that artifact.

## Forbidden Scope

No qsl-server, qsl-attachments, qshield runtime, qsc/qsp/protocol/crypto,
dependency, workflow, website/public-doc, README, START_HERE, backup script,
timer, fstab, service, local backup configuration, off-host destination,
restore target, key, credential, recovery-envelope, deploy, rollback, backup,
or restore change is allowed.

No remote connection, host-key scan, `known_hosts` mutation, repository init,
tool installation, key generation, key upload, passphrase collection,
credential handling, secret handling, private-key inspection, recovery-envelope
content creation, real restore target creation/mount/copy, backup, restore,
deploy, or rollback may occur.

## Prior Follow-Up Review Requirements

- Review live NA-0374 scope in `NEXT_ACTIONS.md`.
- Review NA-0373 response availability blocker / collection follow-up evidence.
- Review NA-0372 response absence evidence.
- Review NA-0371 collection request evidence and JSON artifact.
- Review NA-0370 prior intake evidence.
- Review NA-0369 operator action packet evidence and JSON artifact.
- Review inherited NA-0365, NA-0363, NA-0361, and NA-0359 no-secret harness
  boundaries.

## Source/Authority Refresh Requirements

- qsl-server PR #56 must remain merged and bounded evidence only.
- qsl-attachments PR #37 must remain merged and bounded evidence only.
- Local qsl-server/qsl-attachments worktrees, if present, must be read-only.
- Remote default branch, branch protection, latest CI, viewer permission, and
  open PR list must be classified without mutating either repo.

## Local Backup/Tool/Key/Off-Host/Restore Evidence Refresh Requirements

- Record `/backup/qsl` mount and capacity evidence if available.
- Record local snapshot/manifest/log availability.
- Record qsl-backup syntax/preflight/list status using read-only safe commands.
- Record installed tool availability for restic, borg, rclone, age, gpg, ssh,
  and rsync.
- Classify real off-host target, host identity, credential boundary,
  capacity/retention, monitoring/runbook, repository, key custody, key recovery,
  recovery envelope, and real restore drill evidence.
- Preserve the same-host continuity boundary.

## Response Discovery Requirements

- Search `/home/victor/work/qsl/codex/requests/` only with authorized filename
  and first-line criteria.
- Search qsl-protocol `inputs/metadata_runtime/` only with authorized filename
  criteria.
- Classify the NA-0371 collection request as request evidence, not response
  evidence.
- Record exact response candidate files if any exist.

## Sensitive-Material Stop Requirements

Stop without quoting or copying sensitive content if a response contains private
keys, passphrases, passwords, tokens, raw credentials, recovery-envelope
contents, private material paths, secret-bearing command output, unredacted
sensitive `known_hosts` content, live connection output, or unauthorized host
fingerprint content.

## Response Classification Requirements

Required classifications:

- `OPERATOR_RESPONSE_STILL_ABSENT_AFTER_FOLLOW_UP`
- `TARGET_CANDIDATE_VALUE_ABSENT`
- `HOST_IDENTITY_VALUE_ABSENT`
- `CREDENTIAL_PLACEHOLDER_ONLY`
- `CREDENTIAL_VALUE_FORBIDDEN`
- `CAPACITY_RETENTION_VALUE_ABSENT`
- `MONITORING_RUNBOOK_VALUE_ABSENT`
- `REAL_TARGET_CONNECTION_BLOCKED`
- `REAL_BACKUP_OPERATION_BLOCKED`

## Completeness Matrix Requirements

The evidence must include a field matrix for target label, target class,
owner/contact label, trust boundary, host identity evidence source, fingerprint
algorithm, fingerprint format, fingerprint value, credential model, credential
storage boundary, capacity estimate, retention intent, monitoring destination,
operator runbook owner, emergency stop contact, public-claim acknowledgement,
and no-secret affirmation.

Each row must state status, whether a raw value is stored, redacted summary,
blocker, next action, future authorization, completeness, and whether it can
precede remote connection, credential handling, or real backup.

## Response-Required Blocker Requirements

If no response exists after follow-up:

- classify `OPERATOR_RESPONSE_REQUIRED_STOP_READY`;
- state that continuing Codex-only response-intake work would loop;
- select the required-stop NA-0375 successor.

## No-Secret Validation Requirements

- Parse NA-0369 action packet JSON.
- Parse NA-0371 collection request JSON.
- Parse any future response candidate only after confirming it is safe.
- Do not copy raw secret values into evidence.
- Run added-line leak scanning before PR.

## Local-Ops Requirements

- Record whether read-only history paths are present.
- Record whether history improves confidence.
- Keep workflow-support and history-index work future-gated unless evidence
  selects it as the exact successor.

## Public-Claim Boundary Requirements

The evidence must not introduce production-readiness, public-internet-readiness,
external-review-complete, anonymity, metadata-free, untraceable, hidden-size,
hidden-timing, hidden-traffic-shape, all-metadata-hidden,
off-host-backup-complete, disaster-recovery-complete, real-restore-complete,
host-identity-verified, target-configured, real-key-custody-implemented, or
real-key-recovery-implemented claims.

## Decision Matrix Requirements

The evidence must include a decision matrix covering operator response
existence, target candidate, host identity, credential boundary, capacity/quota,
retention/purge, monitoring/alerting, operator runbook, response-required stop,
local-ops/backup-plan, and external review/public claims.

## Successor Selection Requirements

Expected successor if no operator response data exists:

`NA-0375 -- Metadata Runtime Off-Host Backup Operator Response Required Stop / Await Operator Input`

NA-0375 must not be implemented by NA-0374.

## Required Local Checks

- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- JSON parse for NA-0369 and NA-0371 artifacts
- Metadata runtime backup/deploy/rollback, restore dry-run, key custody,
  off-host target/tool, and isolated restore no-secret harnesses
- qshield-cli test/build, demo smoke, stress, and bounded soak if feasible
- metadata phase-2 and metadata conformance harnesses
- qsc send_commit test
- formal model checks
- NA-0310 refimpl oracle and full refimpl tests if feasible
- queue/decision helper checks
- scope guard
- link check
- leak scan
- goal-lint
- classifier proof for changed paths

## CI Expectations

The qsl-protocol PR must be merged only after required checks attach and pass
normally. `public-safety` must remain required and green before merge and after
merge.

## Successor Handoff

After the NA-0374 evidence PR merges and post-merge public-safety is green, a
separate closeout may mark NA-0374 DONE and restore the exact selected NA-0375
required-stop successor. The closeout must not implement NA-0375.
