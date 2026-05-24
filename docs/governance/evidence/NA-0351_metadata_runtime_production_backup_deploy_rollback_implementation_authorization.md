Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-24

# NA-0351 Metadata Runtime Production Backup Deploy Rollback Implementation Authorization

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0351 converts the NA-0350 production backup/deploy/rollback hardening plan
into an exact future authorization boundary.

Authorization result: `IMPLEMENTATION_AUTHORIZATION_READY` for a bounded future
implementation harness, not for live production operation.

Selected successor:

`NA-0352 -- Metadata Runtime Production Backup / Deploy / Rollback Implementation Harness`

The future NA-0352 lane may build executable, non-production harness evidence
for qsl-server and qsl-attachments backup, deploy, rollback, restore,
secrets/env, monitoring/logging, and public-ingress boundaries. It must not run
a live backup, live restore, live deploy, live rollback, public-ingress cutover,
secret-dependent test, or production service operation unless a later directive
explicitly authorizes that operation with exact backup and rollback evidence.

This lane changed only qsl-protocol governance/testplan files. It did not mutate
qsl-server, qsl-attachments, qshield runtime, qsc, qsp, protocol/crypto, Cargo
dependencies, workflows, service configs, backup scripts, timers, fstab, public
docs, website copy, README, START_HERE, or any live service state.

## Live NA-0351 scope

The live `NEXT_ACTIONS.md` entry marks NA-0351 READY and requires an
authorization decision after NA-0350:

- refresh qsl-server and qsl-attachments source/authority/CI proof before any
  implementation authorization;
- convert the NA-0350 gap matrix into exact implementation prerequisites,
  allowed repositories, allowed paths, validation markers, and rollback/restore
  proof requirements;
- decide whether future work can implement a bounded hardening harness or must
  first resolve blockers such as off-host encrypted backup, source authority,
  deploy/rollback runbooks, secrets/env boundaries, monitoring/logging, public
  ingress, or external-review gaps;
- preserve all public-claim constraints and avoid stronger privacy/readiness
  language.

Protected boundaries:

- no unsupported production, public-internet, external-review, anonymity,
  metadata-free, or untraceable claim;
- no claim that attachment size, timing metadata, or traffic shape is hidden;
- qsl-server/qsl-attachments production boundary remains explicit;
- backup, deploy, rollback, and restore operations remain explicitly
  authorization-gated;
- qshield embedded relay/demo proof remains reference/oracle evidence only.

## Inherited NA-0350 hardening plan

NA-0350 recorded the production backup/deploy/rollback hardening plan after
qsl-server PR #56 and qsl-attachments PR #37.

Inherited proof:

- qsl-server PR #56 merged as `d40e6003fdf0` and remains bounded end-to-end
  modeled harness evidence.
- qsl-attachments PR #37 merged as `96b9352bd63e` and remains service-local
  prerequisite evidence.
- qshield embedded relay/demo evidence remains reference/oracle evidence only.
- qsl-server and qsl-attachments source/authority/CI were fresh in NA-0350.
- local qbuild/codex backup evidence was same-host continuity only, not full
  disaster recovery.

Inherited gaps:

- off-host encrypted backup remains unproven;
- production service data roots, config roots, secrets/env sources,
  monitoring/logging artifacts, deploy artifacts, rollback artifacts, public
  ingress, and restore-drill boundaries remain future-gated;
- no production backup, deploy, rollback, restore, public service, monitoring,
  proxy/TLS/DNS, or external-review completion was performed or proven.

## Source/authority/CI refresh for qsl-server and qsl-attachments

### qsl-server

| Field | Result |
| --- | --- |
| repository | `QuantumShieldLabs/qsl-server` |
| default branch | `main` |
| local path inspected | `/srv/qbuild/work/NA-0237D/qsl-server` |
| local HEAD | `d40e6003fdf0` |
| remote `origin/main` / `HEAD` | `d40e6003fdf0` |
| branch state | detached HEAD, clean |
| PR #56 | merged, merge `d40e6003fdf0` |
| viewer permission | `ADMIN` |
| branch protection | present |
| required check | strict `rust` |
| force pushes | disabled |
| deletions | disabled |
| admins enforced | enabled |
| open PRs | none listed |
| latest listed main CI | `ci` success on `d40e6003fdf0` |
| source classification | `FRESH_SOURCE` |
| authority classification | `COMPLETE_AUTHORITY` |
| CI classification | `COMPLETE_CI` |

### qsl-attachments

| Field | Result |
| --- | --- |
| repository | `QuantumShieldLabs/qsl-attachments` |
| default branch | `main` |
| local path inspected | `/srv/qbuild/work/NA-0237D/qsl-attachments` |
| local HEAD | `96b9352bd63e` |
| remote `origin/main` / `HEAD` | `96b9352bd63e` |
| branch state | detached HEAD, clean |
| PR #37 | merged, merge `96b9352bd63e` |
| viewer permission | `ADMIN` |
| branch protection | present |
| required check | strict `rust` |
| force pushes | disabled |
| deletions | disabled |
| admins enforced | not enabled in current protection |
| open PRs | none listed |
| latest listed main CI | `rust` success on `96b9352bd63e` |
| source classification | `FRESH_SOURCE` |
| authority classification | `COMPLETE_AUTHORITY` |
| CI classification | `COMPLETE_CI` |

No source, authority, CI, or open-PR blocker was found for a future bounded
implementation harness.

## Backup/deploy/rollback authorization readiness decision

Decision category:

`IMPLEMENTATION_AUTHORIZATION_READY`

This means a future bounded implementation harness may be authorized if it stays
inside exact service-repo and qsl-protocol paths, uses test fixtures or
dry-run/read-only checks only, and keeps all live operations forbidden.

The following blocker categories were evaluated and not selected as the primary
NA-0352 successor:

- `IMPLEMENTATION_AUTHORIZATION_BLOCKED_BACKUP`: not selected for a
  non-production harness because current `/srv/qbuild/work` coverage is enough
  for source/test/governance changes. Live operations and public claims remain
  blocked on future backup-plan review and off-host encrypted backup proof.
- `IMPLEMENTATION_AUTHORIZATION_BLOCKED_DEPLOY`: not selected for a harness
  because deploy behavior can be modeled with CI fixtures and dry-run evidence.
  Live deployment remains forbidden.
- `IMPLEMENTATION_AUTHORIZATION_BLOCKED_ROLLBACK`: not selected for a harness
  because rollback behavior can be modeled with known-good binary/config/data
  fixtures. Live rollback remains forbidden.
- `IMPLEMENTATION_AUTHORIZATION_BLOCKED_SECRETS`: not selected for a harness
  because secret names, references, permissions, and redaction rules can be
  tested without secret values. Secret-dependent tests remain forbidden.
- `IMPLEMENTATION_AUTHORIZATION_BLOCKED_MONITORING`: not selected for a harness
  because monitoring/logging redaction fixtures can be added without live
  observability systems.
- `IMPLEMENTATION_AUTHORIZATION_BLOCKED_SOURCE_AUTHORITY`: not selected because
  both service repos are fresh, CI-green, protected, and mutation-authorized for
  a future exact directive.
- `IMPLEMENTATION_AUTHORIZATION_BLOCKED_SCOPE`: not selected because exact
  future allowed paths can be constrained.
- `IMPLEMENTATION_AUTHORIZATION_DEFERRED_LOCAL_OPS`: not selected because
  workflow-support/history-index work would reduce friction but is not the next
  production-hardening proof gate.

## Future implementation authorization bundle

Future implementation belongs in multiple staged lanes:

1. qsl-server service-repo harness and documentation/runbook evidence.
2. qsl-attachments service-repo harness and documentation/runbook evidence.
3. qsl-protocol governance companion and closeout after service proof merges.

The future lane must choose the smallest executable shape that proves the
boundary without live operations. If it cannot keep the work bounded, it must
stop and restore a blocker or prerequisite successor.

### Future allowed qsl-server files

- `tests/na0352_production_backup_deploy_rollback_harness.rs`
- `tests/na0352_production_backup_deploy_rollback_logging.rs`
- `docs/server/DOC-SRV-001_Deployment_Hardening_Contract_v1.0.0_DRAFT.md`
- `docs/server/DOC-SRV-002_Systemd_Hardening_Plan_v1.0.0_DRAFT.md`
- `docs/server/DOC-SRV-003_Relay_Inbox_Contract_v1.0.0_DRAFT.md`
- `docs/server/DOC-SRV-004_Relay_Auth_And_Hardening_Contract_v1.0.0_DRAFT.md`
- `packaging/runbook_ubuntu.md`
- `packaging/systemd/qsl-server.service`
- `packaging/systemd/relay.env.example`
- `scripts/aws_update_and_verify.sh`
- `scripts/ci/test_aws_update_and_verify.sh`
- `scripts/ci/test_canonical_packaging_alignment.sh`
- `scripts/ci/test_relay_deploy_compatibility_guard.sh`
- `scripts/check_relay_compatibility.sh`
- `scripts/lib/deploy_metadata.sh`
- `scripts/qsl_relay_audit.sh`
- `scripts/verify_remote.sh`

### Future allowed qsl-attachments files

- `tests/na0352_production_backup_deploy_rollback_harness.rs`
- `tests/na0352_production_backup_deploy_rollback_logging.rs`
- `tests/backup_restore_recovery.rs`
- `tests/backup_restore_logging.rs`
- `tests/retention_cleanup_recovery.rs`
- `tests/retention_cleanup_logging.rs`
- `tests/service_contract.rs`
- `tests/support/mod.rs`
- `docs/NA-0002_operational_hardening_contract.md`
- `docs/NA-0004_reference_deployment_runbook.md`
- `docs/NA-0009_durability_recovery_contract.md`

### Future allowed qsl-protocol files

- `docs/governance/evidence/NA-0352_metadata_runtime_production_backup_deploy_rollback_implementation_harness.md`
- `tests/NA-0352_metadata_runtime_production_backup_deploy_rollback_implementation_harness_testplan.md`
- `NEXT_ACTIONS.md` only during closeout
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0352_closeout_restore_na0353_testplan.md` only during closeout

### Future forbidden files and actions

Forbidden unless a later directive explicitly expands scope:

- `.github/**`, `Cargo.toml`, `Cargo.lock`, dependency manifests, branch
  protection, public-safety configuration, website/external website paths,
  README, START_HERE, and `docs/public/**`;
- qshield runtime, qsc/qsp/protocol/crypto/key-schedule implementation paths;
- qsc-desktop, external website, public copy, and public claim surfaces;
- live backup scripts, timers, fstab, systemd host units, local backup source
  lists, production service configs, production secrets, and production data
  roots;
- branch deletion, direct push, admin bypass, squash, rebase, and
  history-rewriting operations;
- live backup, restore, deploy, rollback, purge, public-ingress cutover,
  production service start/stop, or secret-dependent tests.

### Future commands

Allowed future commands:

- service repo clean-tree, source/authority/CI, branch-protection, open-PR, and
  main-check refresh;
- `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
  `cargo build --locked`, and `cargo test --locked` in the affected service
  repo;
- fixture-only tests for deploy/rollback/restore/log-redaction behavior;
- dry-run/read-only commands that inspect local fixture directories and
  generated temporary roots;
- qsl-protocol governance validation, scope guard, link-check, leak-scan,
  overclaim scan, goal-lint, and required public-safety checks.

Forbidden future commands:

- production `qsl-backup daily`, `qsl-backup checkpoint`, restore commands, or
  source-list edits;
- `systemctl start|stop|restart|enable|disable` on production qsl-server,
  qsl-attachments, proxy, or backup units;
- production deployment, rollback, purge, DNS/TLS/proxy cutover, or public
  ingress enablement commands;
- commands that print secret values or copy secret-bearing files into public
  evidence.

### Future tests, artifacts, markers, PR order, and merge order

Future tests must include:

- qsl-server deploy/update/rollback dry-run or fixture harness;
- qsl-server non-durable queue restart/rollback boundary;
- qsl-server log/audit redaction scan for route tokens, bearer tokens, payload
  sentinels, secret references, and generated capability-like strings;
- qsl-attachments cold/quiesced full-root restore fixture harness;
- qsl-attachments partial/hot restore fail-closed fixture checks;
- qsl-attachments retention/purge after restore checks;
- combined version-pair compatibility and no-public-ingress proof;
- qsl-protocol governance scope, link, leak, overclaim, queue, and decision
  checks.

Future artifacts must be deterministic, local, redacted, and under service repo
test temp roots or qsl-protocol governance paths. Long-lived artifacts outside
`/srv/qbuild/work` require a backup-plan decision before use.

Recommended PR order:

1. qsl-server harness PR.
2. qsl-attachments harness PR.
3. qsl-protocol governance companion PR.
4. qsl-protocol closeout PR restoring the exact next NA.

If either service repo changes, merge only after required service CI passes and
post-merge main service CI is green. Merge qsl-protocol only after
public-safety is required and green.

## Future marker/verification plan

Future success marker candidates:

- `NA0352_PRODUCTION_BACKUP_DEPLOY_ROLLBACK_AUTHORIZATION_OK`
- `NA0352_SOURCE_BACKUP_SCOPE_OK`
- `NA0352_RUNTIME_CONFIG_BACKUP_SCOPE_OK`
- `NA0352_SERVICE_DATA_BACKUP_SCOPE_OK`
- `NA0352_LOCAL_CONTINUITY_BOUNDARY_OK`
- `NA0352_OFF_HOST_BACKUP_BOUNDARY_OK`
- `NA0352_RESTORE_DRILL_AUTHORIZATION_OK`
- `NA0352_DEPLOY_AUTHORIZATION_OK`
- `NA0352_ROLLBACK_AUTHORIZATION_OK`
- `NA0352_SECRETS_ENV_BOUNDARY_OK`
- `NA0352_MONITORING_LOGGING_BOUNDARY_OK`
- `NA0352_PUBLIC_INGRESS_BOUNDARY_OK`
- `NA0352_NO_PRODUCTION_READY_CLAIM_OK`
- `NA0352_NO_PUBLIC_INTERNET_READY_CLAIM_OK`
- `NA0352_NO_EXTERNAL_REVIEW_COMPLETE_CLAIM_OK`
- `NA0352_NO_METADATA_FREE_CLAIM_OK`
- `NA0352_NO_ANONYMITY_CLAIM_OK`

If blocked, future blocker marker candidates:

- `NA0352_SOURCE_AUTHORITY_BLOCKED`
- `NA0352_BACKUP_SCOPE_BLOCKED`
- `NA0352_OFF_HOST_BACKUP_BLOCKED`
- `NA0352_DEPLOY_AUTHORIZATION_BLOCKED`
- `NA0352_ROLLBACK_AUTHORIZATION_BLOCKED`
- `NA0352_SECRET_BOUNDARY_BLOCKED`
- `NA0352_MONITORING_BOUNDARY_BLOCKED`
- `NA0352_PUBLIC_INGRESS_BOUNDARY_BLOCKED`

## Backup-plan impact and local-ops dependency decision

NA-0351 itself does not require a backup-plan update. Changed files stay under
qsl-protocol source in `/srv/qbuild/work`, which is included by the current
local continuity backup. The response file is under the existing Codex responses
path, which is also included by the installed backup source list.

Current local backup posture:

- `/backup/qsl` is mounted.
- Daily local continuity snapshots are present through 2026-05-24.
- The daily timer is enabled and waiting for the next scheduled run.
- `/srv/qbuild/work` is covered by local continuity backup.
- Codex responses and `QSL_BACKUP_PLAN.md` are covered.
- Codex directives and journals paths were absent during inspection.
- Codex requests and ops paths were present but are not proven as full source
  roots in the installed backup source list.
- The backup is same-host local continuity, not full disaster recovery.
- Off-host encrypted backup remains future-gated.

Future implementation should be preceded or accompanied by:

- production root inventory before any live operation;
- backup-plan update before relying on service data roots, deploy configs,
  rollback artifacts, restore fixtures, monitoring artifacts, or local history
  directories outside current scope;
- off-host encrypted backup prerequisite before any production/public-internet
  readiness or disaster-recovery claim;
- local workflow-support/history-index lane when operational friction becomes
  the highest-value next gate.

D132 preservation status: `/srv/qbuild/tmp/NA-0322_D132_resume_bundle` is still
present and must not be deleted without explicit authorization.

## Deploy/rollback/restore/secrets/monitoring boundary

Authorization states:

- deploy authorization state: dry-run/fixture harness only; live deployment
  forbidden.
- rollback authorization state: dry-run/fixture harness only; live rollback
  forbidden.
- restore authorization state: non-destructive fixture restore only; live
  restore forbidden.
- backup-script authorization state: read-only inspection only; backup-script,
  timer, fstab, and source-list mutation forbidden.
- runtime config authorization state: redacted manifests and fixture configs
  only; production config mutation forbidden.
- secrets/env authorization state: names, references, permissions, and redaction
  only; secret values forbidden in evidence.
- monitoring/logging authorization state: fixture/log redaction tests only; live
  monitoring system mutation forbidden.
- public-ingress authorization state: disabled/private/loopback proof only;
  public cutover forbidden.
- emergency stop/runbook state: future runbook may define stop steps, but may
  not execute them on production services without exact authorization.

Must-not-run examples for future lanes:

- `systemctl restart qsl-server`
- `systemctl restart qsl-attachments`
- `systemctl restart caddy`
- `/usr/local/sbin/qsl-backup daily`
- `/usr/local/sbin/qsl-backup checkpoint <label>`
- any live restore, purge, deploy, rollback, DNS/TLS/proxy cutover, or public
  ingress enablement command.

Future allowed dry-run/read-only examples:

- `git status --porcelain=v1 --branch`
- `git rev-parse HEAD`
- `gh pr view ... --json ...`
- `gh run list ... --json ...`
- `bash -n <script>`
- fixture-only service tests in temporary directories.

## Public-ingress/timing/traffic-shape boundary

Public ingress remains future-gated. Future harnesses may prove that public
ingress is disabled, loopback-only, private, or explicitly not touched. They may
not enable public ingress without a later operation directive.

Current evidence does not prove hidden attachment size, hidden timing metadata,
hidden traffic shape, or hidden all metadata. Size-class and padding evidence is
bounded and still leaves observable size class, timing, request count, retry,
queue, and traffic-shape surfaces.

## External-review sensitivity

External review remains not complete. NA-0351 does not make or strengthen any
external-review statement.

Any stronger future statement requires implementation evidence, service
evidence, deployment evidence, monitoring/log evidence, backup/restore evidence,
rollback evidence, public-ingress evidence if relevant, and review evidence.

## Public claim boundary

NA-0351 does not update public docs or website copy. Future public wording must
not imply:

- production readiness;
- public-internet readiness;
- completed external review;
- anonymity;
- metadata-free behavior;
- untraceable behavior;
- hidden attachment size;
- hidden timing metadata;
- hidden traffic shape;
- padding hiding all metadata;
- same-host local continuity being complete disaster recovery.

qsl-server PR #56 remains bounded end-to-end harness evidence, qsl-attachments
PR #37 remains service-local prerequisite evidence, and qshield embedded
relay/demo proof remains reference/oracle evidence only.

## Workflow-support and history-index future work note

Read-only history availability:

- `/home/victor/work/qsl/codex/directives`: absent.
- `/home/victor/work/qsl/codex/responses`: present; prior D164/NA0350 response
  was present and improved identity confidence.
- `/home/victor/work/qsl/codex/journals`: absent.
- `/home/victor/work/qsl/codex/requests`: present; workflow-support and
  history-access requests were inspected read-only.

Future local-ops improvements would materially reduce friction:

- qstart/qresume fast-forward to expected `origin/main`;
- response-file writer;
- bounded PR/public-safety polling helper;
- machine-readable directive manifest;
- validation profiles;
- per-directive allow-file;
- read-only source/authority helper;
- claim-boundary scanner;
- directive/response/journal index;
- backup coverage for directives, requests, journals, and ops history folders.

These items are deferred and not implemented by NA-0351.

## Selected successor

Selected:

`NA-0352 -- Metadata Runtime Production Backup / Deploy / Rollback Implementation Harness`

Rationale:

- qsl-server and qsl-attachments source, authority, and CI are fresh.
- No source/authority/CI blocker requires a blocker-resolution successor.
- Exact future allowed service and governance paths can be constrained.
- A non-production harness can improve executable evidence without live
  operations or public claims.
- Off-host encrypted backup and local workflow-support remain important, but
  are not the immediate proof gate for a bounded harness.

## Rejected alternatives

- `Metadata Runtime Production Backup / Deploy / Rollback Blocker Resolution`:
  rejected because no current source, authority, CI, or scope blocker was found.
- `Metadata Runtime Off-Host Encrypted Backup Prerequisite Plan`: deferred
  because off-host encrypted backup is required before production/public claims
  or live operations, but not before a non-production harness.
- `QSL Local Ops Codex Workflow Support and History Index Plan`: deferred
  because it reduces friction but is not the next metadata-runtime production
  evidence gate.
- `Metadata Runtime External Review Readiness Gap Audit`: deferred because
  external review should wait until service backup/deploy/rollback evidence is
  sharper or should explicitly classify those items as gaps.
- `Metadata Runtime Website / Public Claim Boundary Audit`: deferred because no
  public docs or website copy should change before executable service evidence
  is stronger.
- `Public Technical Position Paper Evidence-Bounded Draft Plan`: deferred until
  production backup/deploy/rollback boundaries are executable and clearer.

## Backup-plan impact statement

No backup-plan update is required for NA-0351. Future production hardening may
require backup-plan updates before it introduces or relies on production service
data roots, deploy configs, rollback artifacts, restore fixtures, monitoring
artifacts, or local history directories outside current backup scope.

Future production or public claims require off-host encrypted backup and restore
proof. Same-host local continuity backup must remain labeled as local continuity
only.

## Next recommendation

Restore NA-0352 as the exact successor after the NA-0351 PR merges and
post-merge public-safety is green. NA-0352 should implement the bounded
qsl-server/qsl-attachments production backup/deploy/rollback harness selected
here, or stop with exact blocker evidence before any live operation.
