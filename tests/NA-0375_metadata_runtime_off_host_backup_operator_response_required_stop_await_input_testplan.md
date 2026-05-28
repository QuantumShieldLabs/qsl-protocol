Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-28

# NA-0375 Metadata Runtime Off-Host Backup Operator Response Required Stop / Await Input Test Plan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0375 records the operator-response required stop after
NA-0374, states the missing and forbidden operator input, preserves all
no-secret/no-connection/no-host-key-scan boundaries, and selects the exact
NA-0376 local-ops/history-index planning successor if no response exists.

## Protected Invariants

- Exactly one READY item remains during NA-0375 evidence work.
- READY remains NA-0375 until a separate closeout restores NA-0376.
- D-0732 exists once after NA-0375 evidence lands.
- D-0733 remains absent until closeout.
- Response absence is not treated as target candidate evidence.
- Host identity absence is not treated as verified host identity.
- Local continuity is not presented as complete disaster recovery.
- Off-host encrypted backup is not presented as complete.

## Allowed Scope

- `docs/governance/evidence/NA-0375_metadata_runtime_off_host_backup_operator_response_required_stop_await_input.md`
- `tests/NA-0375_metadata_runtime_off_host_backup_operator_response_required_stop_await_input_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

The optional NA-0375 status JSON is allowed only if live NA-0375 scope
explicitly permits it. This lane does not add that artifact.

## Forbidden Scope

No qsl-server, qsl-attachments, qshield runtime, qsc/qsp/protocol/crypto,
dependency, workflow, website/public-doc, README, START_HERE, backup script,
timer, fstab, service, local backup configuration, off-host destination,
restore target, key, credential, recovery-envelope, deploy, rollback, backup,
or restore change is allowed.

No remote connection, host-key scan, `known_hosts` mutation, repository init,
tool installation, key generation, key upload, passphrase collection,
credential handling, secret handling, private-key inspection,
recovery-envelope content creation, real restore target creation/mount/copy,
backup, restore, deploy, or rollback may occur.

## Prior Response-Intake Review Requirements

- Review live NA-0375 scope in `NEXT_ACTIONS.md`.
- Review NA-0374 response absence after follow-up.
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
  capacity/retention, monitoring/runbook, repository, key custody, key
  recovery, recovery envelope, and real restore drill evidence.
- Preserve the same-host continuity boundary.

## Final Response Discovery Requirements

- Search `/home/victor/work/qsl/codex/requests/` only with authorized filename
  criteria.
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

## Required-Stop Classification Requirements

If no deliberate no-secret response exists:

- classify `OPERATOR_RESPONSE_REQUIRED_STOP`;
- classify `CODEX_ONLY_PROGRESS_ON_OFF_HOST_TARGET_CHAIN_BLOCKED`;
- record target candidate absent;
- record host identity absent;
- record credential boundary absent/forbidden;
- record capacity/retention absent;
- record monitoring/runbook absent;
- record real target connection blocked;
- record real backup operation blocked;
- do not continue another blind intake loop.

## Missing-Input and Forbidden-Input Requirements

Evidence must state missing non-secret operator input:

- target label/alias;
- target class confirmation;
- owner/contact label;
- high-level location/jurisdiction class if safe;
- host identity evidence source description;
- fingerprint format/algorithm class if safe;
- capacity/retention estimates;
- monitoring destination class;
- emergency contact label;
- runbook owner label;
- public-claim boundary acknowledgement;
- no-secret affirmation.

Evidence must forbid private keys, passphrases, passwords, tokens, raw
credentials, recovery-envelope contents, secret paths, private material paths,
screenshots or command output containing secrets, live connection outputs,
unredacted sensitive `known_hosts` content, and host fingerprints unless a
future directive explicitly authorizes collection/storage.

## Local-Ops Successor Analysis Requirements

- Explain that off-host target work is blocked on external operator response.
- Explain that another response-intake lane would loop.
- Explain why local-ops/history-index planning is bounded and useful.
- Keep NA-0376 implementation future-gated.
- Select exact successor if live scope permits.

## No-Secret Validation Requirements

- Parse NA-0369 action packet JSON.
- Parse NA-0371 collection request JSON.
- Do not copy raw secret values into evidence.
- Run added-line leak scanning before PR.

## Backup-Plan / Local-Ops Requirements

- Record backup-plan impact.
- Record same-host continuity limitation.
- Record whether requests/directives/journals/ops history coverage remains a
  future local-ops gap.
- Do not modify backup scripts, timers, fstab, service units, source lists, or
  backup destinations.

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
retention/purge, monitoring/alerting, operator runbook, local-ops/backup-plan,
and external review/public claims.

## Successor Selection Requirements

Expected successor if no operator response data exists and live scope permits:

`NA-0376 -- QSL Local Ops Codex Workflow Support and History Index Plan`

NA-0376 must not be implemented by NA-0375.

## Required Local Checks

- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- JSON parse for NA-0369 action packet and NA-0371 collection request.
- Metadata runtime no-secret harnesses for NA-0352, NA-0359, NA-0361, NA-0363,
  and NA-0365.
- qshield-cli build/test where feasible.
- qsc `send_commit` test.
- formal model checks.
- metadata conformance smoke.
- demo smoke/stress/soak where feasible.
- queue and decision helper checks.
- scope guard, link-check, leak-scan, classifier proof, and goal-lint.

## CI Expectations

The PR must merge only after required qsl-protocol checks complete normally and
`public-safety` is required and green. No admin bypass, direct push, squash,
rebase, amend-after-PR, force-push, or branch deletion command is allowed.

## Successor Handoff

After NA-0375 evidence merges and post-merge public-safety is green, a separate
closeout may mark NA-0375 DONE and restore:

`NA-0376 -- QSL Local Ops Codex Workflow Support and History Index Plan`
