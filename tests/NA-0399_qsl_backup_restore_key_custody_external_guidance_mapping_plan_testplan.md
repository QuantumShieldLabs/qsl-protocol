Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-01

# NA-0399 QSL Backup / Restore / Key Custody External Guidance Mapping Plan Testplan

Goals: G1, G2, G3, G4, G5

Directive: QSL-DIR-2026-06-01-219

## Objective

Validate that NA-0399 records a qsl-protocol backup, restore, key custody, key
recovery, off-host backup, and disaster-recovery external-guidance mapping plan
without implementing backup/restore/key/off-host operations, changing runtime
behavior, changing cryptography, changing dependencies, mutating workflows,
mutating sibling repositories, updating public docs/website, changing backup
configuration, handling secrets, or expanding public claims.

## Protected Invariants

- READY_COUNT remains exactly one.
- READY remains NA-0399 until closeout.
- NA-0398 is DONE.
- D-0778 exists once.
- D-0779 exists once.
- D-0780 exists once after this PR.
- D-0781 is absent before closeout.
- No runtime, service, protocol, crypto, dependency, Cargo, workflow,
  public-doc, website, backup-script, backup-timer, fstab, source-list,
  qsl-server, qsl-attachments, qshield runtime, qstart/qresume, response
  archive, local tool, off-host target, key, credential, passphrase, private
  key, recovery envelope, known_hosts, or secret-bearing path is changed.
- Source discovery is not external review.
- Planning is not implementation.
- Same-host continuity is not complete disaster recovery.
- No off-host-backup-complete, restore-proven, restore-drill-complete,
  real-restore-complete, key-custody-implemented, key-recovery-implemented,
  recovery-envelope-ready, target-configured, host-identity-verified,
  production-ready, public-internet-ready, bug-free, perfect-crypto, or
  external-review-complete claim is introduced.

## Allowed Scope

- `docs/governance/evidence/NA-0399_qsl_backup_restore_key_custody_external_guidance_mapping_plan.md`
- `tests/NA-0399_qsl_backup_restore_key_custody_external_guidance_mapping_plan_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

Forbidden changes include `.github/**`, workflows, `Cargo.toml`,
`Cargo.lock`, runtime/protocol/crypto implementation paths, qsc/qsp/qsl
implementation, qshield runtime, qsl-server, qsl-attachments, qsc-desktop,
website, docs/public, README, START_HERE, backup scripts/timers/fstab/services,
backup source-list mutation, durable backup/restore/key reports outside
governance evidence, response archives, request/directive/history roots,
qstart/qresume tooling, helper script mutations, off-host setup, host-key
scan, known_hosts mutation, repository initialization, key generation,
passphrase/credential/private-key handling, recovery-envelope creation, and
secret handling.

## NA-0398 Inheritance Requirements

Verify the evidence records:

- PR #1059 merge `4859cdc524aa`.
- PR #1060 merge `eb9acaa1cb76`.
- origin/main handoff SHA `eb9acaa1cb7620b3f9419632e8c07e3784d3a925`.
- READY_COUNT 1 and READY NA-0399.
- NA-0398 DONE.
- D-0778 and D-0779 presence.
- D-0780 absence before NA-0399.
- metadata/privacy claim-boundary residuals remain future-gated.
- future Project Goal / Operating Principles canon lane is carry-forward only.

## Official Source Verification Requirements

Verify official source categories are cited for:

- restic documentation.
- Borg documentation.
- rclone documentation.
- age documentation.
- GnuPG documentation.
- OpenSSH documentation.
- NIST key-management guidance, including SP 800-57 context.
- NIST contingency/recovery guidance, including SP 800-34 and SP 800-184
  context.
- NCSC/NIST official backup/resilience guidance where relevant.

## Source Citation Requirements

Each citation must include:

- source title.
- publisher/authority.
- URL.
- access date.
- source tier.
- source classification.
- relevance to QSL.
- claim-boundary implication.

Tool documentation, key-management guidance, backup/restore guidance,
disaster-recovery guidance, and local QSL evidence must be distinguished.
Vendor or adjacent claims must not be primary evidence.

## Prior Evidence Intake Requirements

Verify the evidence records:

- NA-0355 target/tool selection.
- NA-0359 restore dry-run evidence.
- NA-0361 no-secret key custody/recovery harness boundary.
- NA-0363 no-secret off-host target/tool harness boundary.
- NA-0365 no-secret isolated restore harness boundary.
- NA-0366 through NA-0375 off-host/operator blocker sequence.
- NA-0384 through NA-0388 response archive/local history posture.
- local backup plan/status docs if present.
- qsl-server PR #56 boundary.
- qsl-attachments PR #37 boundary.
- what can be claimed internally.
- what cannot be claimed publicly.

## Read-Only Backup / Local Inventory Requirements

Verify inventory includes:

- `/backup/qsl` mount/capacity.
- `/srv/qbuild` disk watermark.
- backup manifest/log listing.
- installed tool detection for restic, Borg, rclone, age, GnuPG, OpenSSH,
  rsync, and qsl-backup.
- qsl-backup help/preflight/list only if read-only.
- response archive/history/request/directive/journal/ops inventory.
- D205 smoke hash and D205-D218 final response presence.

The inventory must not run real backup, real restore, off-host connection,
host-key scan, repository init, tool install, key generation, passphrase prompt,
credential handling, private-key inspection, recovery-envelope creation,
backup source-list edit, backup timer/script/fstab edit, or service mutation.

## Same-Host Continuity Requirements

Verify the evidence:

- records current same-host local continuity status.
- records manifest/log evidence.
- records response archive coverage where evidenced.
- states same-host continuity is not complete disaster recovery.
- lists missing off-host/key/restore/monitoring/runbook evidence.

## Off-Host Backup Requirements

Verify the evidence:

- records NA-0355 target/tool class selection.
- records NA-0363 no-secret target/tool harness boundary.
- records operator response absence.
- records target candidate, host identity, credential, repository, retention,
  monitoring, and runbook absence.
- forbids off-host backup completion claims.

## Backup Tool / Repository Model Requirements

Verify the evidence:

- records restic-style repository class.
- maps restic/Borg/rclone/age/GnuPG/OpenSSH relevance.
- records local tool install status from read-only detection.
- records repository init/check/prune/restore absence.
- records future evidence needed before repository claims.

## Key Custody Requirements

Verify the evidence:

- records NA-0361 no-secret key custody/recovery harness.
- records real key custody absence.
- records no key generation/upload/passphrase/private-key handling.
- maps NIST key-management relevance.
- forbids key custody implemented or key custody ready claims.

## Key Recovery Requirements

Verify the evidence:

- records no-secret recovery metadata.
- records recovery-envelope absence.
- records old archive recovery, lost/exposed key response, and emergency access
  absence.
- forbids key recovery implemented, key recovery ready, or recovery envelope
  ready claims.

## Restore Drill Requirements

Verify the evidence:

- records NA-0359 dry-run restore harness.
- records NA-0365 isolated restore no-secret harness.
- records no real off-host restore execution.
- records no restore target creation/mount/copy in NA-0399.
- forbids restore proven, restore drill complete, and real restore complete
  claims.

## Retention / Pruning / Monitoring / Runbook Requirements

Verify the evidence:

- records no-secret target/tool retention/prune evidence.
- records local qsl-backup retention intent as same-host local continuity only.
- records real off-host retention/prune/check/monitoring/runbook absence.
- lists future evidence needed.

## Disaster Recovery Boundary Requirements

Verify the evidence maps:

- local continuity.
- off-host backup.
- key custody.
- key recovery.
- restore proof.
- monitoring/runbook.
- external review.

The evidence must state disaster recovery is not complete.

## Operator Response Blocker Requirements

Verify the evidence records:

- NA-0369/0371 operator packet/request.
- NA-0372/0373/0374/0375 absence/required-stop chain.
- target label absence.
- host identity absence.
- credential model absence.
- capacity/retention absence.
- monitoring/runbook absence.
- required no-secret operator response boundary.

## Local History / Evidence Backup Requirements

Verify the evidence records:

- response archive status.
- response writer real-archive smoke status.
- response history catalog temp-output status.
- durable catalog/report storage status.
- directives/journals absence if true.
- same-host continuity caveat.
- future evidence needed.

## Evidence Matrix Requirements

Verify the matrix contains rows for:

- same-host local continuity.
- off-host encrypted backup.
- backup tool / repository model.
- key custody.
- key recovery.
- restore dry-run.
- isolated restore no-secret.
- real restore.
- retention/pruning.
- monitoring/alerting.
- runbook/operator response.
- disaster recovery.
- local history/evidence backup.

Each row must include external source basis, QSL current evidence, evidence
class, confidence, allowed claim, forbidden claim, missing evidence, future lane,
and priority.

## Claim Language Policy Requirements

Verify allowed wording is caveated and limited to:

- same-host local continuity.
- no-secret harness.
- dry-run restore harness.
- simulated isolated restore harness.
- off-host target/tool class selected.
- operator response required.
- evidence incomplete.
- future gated.

Verify forbidden wording is negated, prohibited, or future/unproven where it
appears.

## Future Queue Candidate Requirements

Verify the evidence defines candidates for:

- external review / disclosure / public claim readiness.
- backup / restore / key custody critical evidence gap resolution.
- off-host operator response refresh.
- real off-host target / host identity authorization.
- real key custody / recovery envelope authorization.
- real isolated restore authorization.
- backup monitoring / retention / runbook.
- response/directive/journal backup coverage review.
- Project Goal / Operating Principles canon authorization.
- Director State Index authorization.
- public technical position paper prerequisite plan.

Each candidate must include source/evidence basis, why next or not next, likely
allowed scope, likely forbidden scope, and public-claim implication.

## No Implementation Requirements

Verify NA-0399 does not:

- run a real backup.
- run a real restore.
- create/mount/copy a restore target.
- connect to an off-host target.
- scan host keys.
- mutate known_hosts.
- initialize a repository.
- install a tool.
- generate or inspect key material.
- collect passphrases or credentials.
- create a recovery envelope.
- mutate backup scripts/timers/fstab/source lists.
- change runtime/security behavior.

## Public Claim Boundary Requirements

Verify the evidence forbids:

- disaster recovery complete.
- off-host backup complete.
- restore proven.
- restore drill complete.
- real restore complete.
- key custody implemented.
- key recovery implemented.
- recovery envelope ready.
- target configured.
- host identity verified.
- production ready.
- public internet ready.
- externally reviewed.

## Public Paper Boundary Requirements

Verify the evidence states public technical paper work remains future-gated and
is not started by NA-0399.

## Successor Selection Requirements

Verify the selected successor is exactly:

`NA-0400 -- QSL External Review / Disclosure / Public Claim Readiness Plan`

Verify rejected alternatives are recorded:

- backup/restore/key critical evidence gap resolution.
- source verification blocker resolution.

## Future Project Goal Canon Carry-Forward Requirements

Verify the evidence carries forward:

`QSL Project Goal and Operating Principles Canon Authorization Plan`

without implementing it or promoting it over NA-0400.

## Backup-Impact Requirements

Verify the evidence states:

- no backup-plan update is required for this governance/testplan-only lane.
- future durable backup/restore/key reports, local history catalogs, response
  archive mutations, real target/host/key/restore/monitoring artifacts, or real
  operations require separate backup-impact review.

## Required Local Checks

Required validation includes:

- `git status --porcelain=v1 --branch`
- `git diff --name-only origin/main...HEAD`
- `git diff --check`
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- helper `--help` and representative fixture checks.
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- qsc send_commit.
- formal model checks.
- qshield-cli build/test if feasible.
- scope guard for exact allowed paths.
- link-check.
- leak-scan.
- goal-lint.
- classifier proof.

## CI Expectations

- Open a qsl-protocol PR.
- Required checks must attach and pass without admin bypass.
- Merge with `--merge --match-head-commit`.
- Do not squash, rebase, force-push, amend after PR creation, direct-push to
  main, or use branch deletion flags.
- After merge, post-merge public-safety must complete successfully before
  optional closeout.

## Successor Handoff

- Before closeout: READY_COUNT 1, READY NA-0399, D-0780 once, D-0781 absent.
- After optional closeout: READY_COUNT 1, READY selected NA-0400, NA-0399 DONE,
  D-0781 once, D-0782 absent.
- NA-0400 must not be implemented by NA-0399 closeout.
