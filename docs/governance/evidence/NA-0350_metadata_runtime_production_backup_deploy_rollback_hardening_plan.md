Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-24

# NA-0350 Metadata Runtime Production Backup Deploy Rollback Hardening Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0350 records the production backup, deploy, and rollback hardening plan for
the qsl-server / qsl-attachments metadata-runtime path after the bounded
end-to-end harness proof from NA-0349.

Result: `NA0350_PRODUCTION_BACKUP_DEPLOY_ROLLBACK_HARDENING_PLAN_RECORDED`.

Read-only refresh found no qsl-server or qsl-attachments source, authority, CI,
or open-PR blocker. qsl-server `main` is `d40e6003fdf0`, PR #56 is merged, the
required `rust` check is green, branch protection is present, and no open
qsl-server PRs were listed. qsl-attachments `main` is `96b9352bd63e`, PR #37 is
merged, the required `rust` check is green, branch protection is present, and
no open qsl-attachments PRs were listed.

The plan classifies current evidence as sufficient for a qsl-protocol
governance hardening plan, but not sufficient for deploy, rollback, restore,
public ingress, external-review completion, or any stronger public/privacy
claim. The next lane should define exact future implementation authorization
before mutating qsl-server, qsl-attachments, service configuration, deployment
scripts, backup scripts, monitoring, or public surfaces.

Selected successor:

`NA-0351 -- Metadata Runtime Production Backup / Deploy / Rollback Implementation Authorization Plan`

## Live NA-0350 scope

The live `NEXT_ACTIONS.md` entry authorizes NA-0350 to harden and plan the
production backup, deploy, and rollback boundary for the qsl-server /
qsl-attachments metadata-runtime path, or stop with exact prerequisite evidence
if source, authority, CI, backup, deploy, rollback, secrets, monitoring,
public-ingress, or public-claim prerequisites regress.

Allowed behavior in this lane:

- refresh qsl-server and qsl-attachments source, authority, and CI evidence
  read-only;
- inventory backup, deploy, rollback, secrets/env, monitoring/logging, public
  ingress, and operator-runbook prerequisites;
- produce a bounded hardening plan or exact blocker evidence;
- preserve all public-claim limits.

Forbidden behavior in this lane:

- qsl-server or qsl-attachments mutation;
- qshield runtime, qsc, qsp, protocol, crypto, key schedule, dependency,
  workflow, website, README, START_HERE, docs/public, branch-protection,
  public-safety, or backup-script/timer/fstab mutation;
- production deploy, rollback, restore, purge, public-ingress enablement, or
  service operation;
- unsupported claims about production readiness, public-internet readiness,
  external review, anonymity, metadata-free behavior, untraceability, hidden
  attachment size, hidden timing metadata, hidden traffic shape, or all
  metadata being hidden.

## Inherited NA-0349 end-to-end harness proof

qsl-server PR #56 merged as `d40e6003fdf0` from validated head
`9f51b5a691f`. It changed only:

- `tests/na0349_end_to_end_integration_contract.rs`

The harness proves a bounded deterministic qsl-server / qsl-attachments
contract model:

- qsl-server receives and returns a qsl-attachments descriptor exactly as
  opaque payload bytes;
- a modeled qsl-attachments object can be committed, fetched, snapshot/restored,
  expired, and purged;
- route-token and bearer-auth rejects fail closed before accepted queue state;
- oversize and wrong-route cases do not create delivery state;
- qsl-server logs and modeled qsl-attachments audit events do not include the
  test route token, bearer token, fetch capability, payload sentinel, raw
  locator, or attachment id;
- public-ingress and deploy/rollback markers remain loopback/non-deploy
  boundaries.

Limitations:

- no production qsl-server or qsl-attachments host was changed or operated;
- no real service-to-service deployment topology was exercised;
- no production backup, restore, deploy, rollback, monitoring, alerting,
  public ingress, TLS, proxy, DNS, or CDN path was exercised;
- qsl-attachments PR #37 remains service-local prerequisite proof;
- qshield embedded relay/demo proof remains reference/oracle evidence only;
- the harness does not support stronger privacy or readiness claims.

## Source/authority/CI refresh for qsl-server and qsl-attachments

### qsl-server

| Field | Result |
| --- | --- |
| repository | `QuantumShieldLabs/qsl-server` |
| default branch | `main` |
| local path inspected | `/srv/qbuild/work/NA-0237D/qsl-server` |
| local HEAD | `d40e6003fdf0` |
| remote `origin/main` / `HEAD` | `d40e6003fdf0` |
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

## Existing backup/deploy/rollback evidence inventory

### qsl-server

Current evidence:

- `docs/server/DOC-SRV-001_Deployment_Hardening_Contract_v1.0.0_DRAFT.md`
  defines HTTP-only service posture, upstream TLS termination, network ACLs,
  runtime limits, logging policy, optional `RELAY_TOKEN`, deployment checklist,
  observability limits, and a rollback note.
- `docs/server/DOC-SRV-002_Systemd_Hardening_Plan_v1.0.0_DRAFT.md` defines
  systemd hardening and rollback steps.
- `packaging/runbook_ubuntu.md` records install, update, rollback, token
  rotation, Caddy/reverse-proxy guidance, and audit commands.
- `packaging/systemd/qsl-server.service` and `packaging/systemd/relay.env.example`
  define the current packaged unit and environment shape.
- `scripts/aws_update_and_verify.sh` backs up the env file, proxy config,
  systemd unit, binary, and deployment metadata before checksum-verified update.
- `scripts/ci/test_aws_update_and_verify.sh` exercises the update wrapper
  success path and checksum-mismatch failure path in CI mode.
- `scripts/verify_remote.sh` and `scripts/qsl_relay_audit.sh` provide
  host-side verification/audit shapes.

Limitations:

- qsl-server runtime state is process memory only; queued relay messages are not
  a durable production data root.
- qsl-server deployment/rollback evidence is packaging and script-level proof,
  not a current production rollout or rollback drill.
- no qsl-server production host, proxy, DNS/TLS, monitoring, or alert evidence
  was operated in NA-0350.

### qsl-attachments

Current evidence:

- `docs/NA-0002_operational_hardening_contract.md` classifies local
  development, constrained-host validation, and reference deployment profiles.
- `docs/NA-0004_reference_deployment_runbook.md` records a stronger reference
  host layout with loopback qsl-attachments, Caddy TLS termination, local build
  and binary copy, systemd unit, env file, state root, and verification probes.
- `docs/NA-0009_durability_recovery_contract.md` freezes the single-node
  local-disk durability contract: cold or quiesced full storage-root copy plus
  matching service configuration is the only supported backup/restore shape;
  hot/live backup and partial restore are unsupported.
- `tests/backup_restore_recovery.rs` proves cold full-root restore, partial
  object/session restore fail-closed behavior, expiry/abort non-resurrection,
  and mismatched metadata rejection.
- `tests/backup_restore_logging.rs` proves backup/restore logs and audit/recovery
  summaries do not include deterministic test resume tokens, fetch
  capabilities, descriptor sentinels, plaintext sentinels, or ciphertext
  sentinels.
- PR #37 adds service-local production size-class handling and cold full-root
  backup boundary proof.

Limitations:

- qsl-attachments has a service-local cold restore harness, not a production
  backup system or off-host restore drill.
- the current durability boundary is one operator-managed local storage root on
  one node.
- public ingress, monitoring, deployment automation, rollback automation, and
  off-host encrypted backup remain future-gated.

### qsl-protocol and local qbuild/codex continuity

Current evidence:

- `docs/ops/DOC-OPS-001_qbuild_Continuity_and_Disaster_Recovery_Runbook_v0.1.0_DRAFT.md`
  defines qbuild continuity and authority proof.
- `docs/ops/DOC-OPS-002_Continuity_Snapshot_Manifest_and_Offhost_Procedure_v0.1.0_DRAFT.md`
  defines off-host continuity snapshot requirements.
- local `/home/victor/work/qsl/codex/QSL_BACKUP_PLAN.md` and
  `/home/victor/work/qsl/codex/ops/backup/QSL_BACKUP_STATUS.md` report a local
  rsync hard-link continuity backup on `/backup/qsl`, with `/srv/qbuild/work`,
  Codex logs, Codex responses, and the backup plan in scope.

Limitations:

- local continuity backup is on the same host and is not complete disaster
  recovery.
- current installed source list includes Codex responses but not the directives,
  requests, journals, or ops history directories as durable source roots.
- production qsl-server/qsl-attachments runtime data, service configs,
  monitoring artifacts, deploy artifacts, rollback artifacts, and restore
  fixtures may live outside current qbuild backup scope and require future
  backup-plan review before claims.

## Production backup/deploy/rollback threat and value model

Production hardening must prove the operator can move a known-good source and
configuration into service, observe it, back up every durable state root needed
for recovery, restore it under bounded conditions, and roll back a bad release
without hiding failures or leaking secrets.

Value to prove before stronger claims:

- source, artifact, config, env, service-unit, proxy/TLS, runtime-data, logs,
  monitoring, backup, restore, deploy, rollback, and public-ingress boundaries
  are each explicit;
- qsl-server queue loss is understood as non-durable relay state unless future
  work changes that contract;
- qsl-attachments committed-object recovery uses coherent `object.json` plus
  `ciphertext.bin` under a cold/quiesced full-root copy plus matching config;
- secrets/env values are backed up only through approved secret references or
  redacted material, not copied into public evidence;
- failed deploy and partial rollback states are detected and stop safely;
- retention/purge behavior remains consistent after restore;
- monitoring and log artifacts are useful to operators without exposing route
  tokens, bearer tokens, resume tokens, fetch capabilities, payload bytes, raw
  keys, passphrases, or secret-bearing URLs;
- public ingress can be disabled or held private until explicit future
  authorization.

Primary hazards:

- local continuity backup being confused with off-host disaster recovery;
- service-local harness proof being confused with production operation proof;
- qshield demo evidence being confused with qsl-server/qsl-attachments
  production behavior;
- hidden production data roots or env/config files outside backup scope;
- rollback to a binary/config pair that is inconsistent with stored object data;
- hot/live or partial qsl-attachments backup being treated as supported;
- route-token or capability leakage through URLs, logs, backup manifests,
  monitoring, proxy access logs, or screenshots;
- public claim expansion before deploy, restore, monitoring, rollback, and
  review evidence exists.

## Backup/deploy/rollback evidence gap matrix

| Row | Current evidence | Proof status | Risk | Future test/proof required | Backup-plan impact | Implementation need | Blocker or ready | Successor relation |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| qsl-server source backup | qsl-server source under `/srv/qbuild/work`; remote `main` and PR #56 merged | partial | local source loss if unpushed branches exist | qbuild source snapshot and remote SHA inventory | covered locally by current qbuild backup | none in NA-0350 | ready | NA-0351 must repeat |
| qsl-attachments source backup | qsl-attachments source under `/srv/qbuild/work`; remote `main` and PR #37 merged | partial | local source loss if unpushed branches exist | qbuild source snapshot and remote SHA inventory | covered locally by current qbuild backup | none in NA-0350 | ready | NA-0351 must repeat |
| qsl-server build artifacts | packaging/update scripts and deploy metadata helper exist | partial | unverified binary or wrong artifact deployed | checksum-verified artifact build/install evidence | future artifact location may need inclusion | future lane | ready | NA-0351 |
| qsl-attachments build artifacts | reference runbook records local build and binary copy | partial | deployed binary digest drift | checksum and deployed digest evidence | future artifact location may need inclusion | future lane | ready | NA-0351 |
| qsl-server runtime config | env example and runbook exist | partial | config/env drift or missing limits | config manifest with values redacted where needed | `/etc/qsl-server` outside qbuild backup | future lane | ready | NA-0351 |
| qsl-attachments runtime config | reference env shape and Config env parsing exist | partial | config mismatch with storage root | config manifest with values redacted where needed | `/etc/qsl-attachments` outside qbuild backup | future lane | ready | NA-0351 |
| service env/secrets | env var names known; qsl-server update wrapper backs up env file | partial | secret leakage or missing secret restore path | secret reference inventory, no secret values in evidence | secret store/backups need future policy | future lane | ready | NA-0351 |
| systemd/service units | qsl-server unit exists; qsl-attachments reference unit documented | partial | hardening drift or incompatible unit | unit capture, verify, rollback drill | `/etc/systemd/system` outside qbuild backup | future lane | ready | NA-0351 |
| qsl-server runtime data | relay queue is in-memory and bounded | partial | operator expects durable queued messages | explicit non-durable queue restart/rollback proof | no durable data root today | future proof | ready | NA-0351 |
| qsl-attachments object/session storage | local storage root contract and tests exist | partial | hot/partial restore treated as valid | cold/quiesced full-root restore drill with matching config | production storage root outside qbuild backup | future lane | ready | NA-0351 |
| logs and redaction | qsl-server and qsl-attachments tests check sentinels | partial | proxy/monitoring logs leak identifiers | combined service/proxy/log scan | log roots outside qbuild backup | future lane | ready | NA-0351 |
| monitoring/alerts | qsl-server safe metrics guidance; no combined alert proof | missing | failures hidden from operator | alert schema and redaction proof | monitoring artifacts may need inclusion | future lane | ready | NA-0351 |
| deploy scripts | qsl-server scripts exist and CI wrapper test passes | partial | script drift or host assumptions | dry-run/staging deploy harness | deploy artifacts may need inclusion | future lane | ready | NA-0351 |
| deploy manual runbook | qsl-server runbook and qsl-attachments reference runbook exist | partial | operator ambiguity | combined ordered runbook with preflight gates | runbook evidence under source if checked in | future lane | ready | NA-0351 |
| rollback scripts | qsl-server runbook has binary rollback; no combined rollback harness | partial | inconsistent rollback | rollback drill with binary/config/data compatibility checks | rollback backups outside qbuild | future lane | ready | NA-0351 |
| rollback manual runbook | qsl-server rollback command exists; qsl-attachments update path documented | partial | incomplete rollback sequence | combined runbook and verification evidence | runbook under source if checked in | future lane | ready | NA-0351 |
| backup restore drill | qsl-attachments local harness; qbuild local restore drill | partial | production restore assumptions exceed evidence | cold full-root service restore drill and qbuild/off-host restore proof | future production roots need review | future lane | ready | NA-0351 |
| off-host backup | qsl-protocol docs require off-host; local plan defers it | missing | same-host loss defeats recovery | encrypted off-host continuity/disaster-recovery proof | backup plan update likely | future prerequisite | ready with prerequisite | NA-0351 input |
| encrypted backup | planned, not implemented for current local backup | missing | backup media disclosure | encryption key custody and restore drill | backup plan update likely | future prerequisite | ready with prerequisite | NA-0351 input |
| local continuity snapshot | `/backup/qsl` local continuity snapshot/status exists | partial | mistaken for full disaster recovery | label as local continuity only | no NA-0350 update required | none now | ready | NA-0351 caveat |
| retention/purge after restore | qsl-attachments tests prove expired/deleted/aborted do not resurrect | partial | stale object resurrection | production restore retention/purge verification | production storage root backup scope | future lane | ready | NA-0351 |
| public ingress cutover | qsl-server/qsl-attachments runbooks mention proxy/TLS; NA-0349 loopback only | missing | public exposure before proof | ingress enable/disable and rollback proof | proxy/DNS/TLS artifacts outside qbuild | future lane | ready with gate | NA-0351 |
| DNS/TLS/proxy/CDN if relevant | Caddy examples and qsl-attachments reference runbook exist | partial | misconfigured TLS/logs | sanitized proxy/TLS/DNS proof | external configs need future backup policy | future lane | ready | NA-0351 |
| qsl-server/qsl-attachments compatibility | NA-0349 modeled descriptor handoff proof | partial | source/config mismatch | exact version-pair compatibility matrix | source covered; runtime configs not | future lane | ready | NA-0351 |
| external review package | public package exists but review not complete | missing | premature review claim | readiness gap audit after service evidence | no NA-0350 backup impact | future lane | deferred | later NA |
| website/public claims | no NA-0350 public docs update | not changed | overclaim | future claim-boundary audit before copy changes | no NA-0350 backup impact | future lane | deferred | later NA |
| D132/history/workflow-support evidence | D132 bundle preserved; responses present; directives/journals absent | partial | handoff evidence loss | local-ops history index and backup coverage update | future backup-plan update likely | future local-ops lane | deferred | later NA |

## Future hardening strategy options

| Option | Feasibility | Allowed future repos | Risk | CI cost | Backup impact | Claim boundary | Result |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Production backup/deploy/rollback implementation authorization plan | high | qsl-protocol first, future qsl-server/qsl-attachments only after exact authorization | moderate | medium | defines future backup-plan triggers | no readiness claim | recommended |
| Backup/restore executable drill using local qbuild-only fixtures | medium | qsl-protocol or service repo, depending on exact fixture | may miss real service config | medium | may create durable fixtures | no disaster-recovery claim | deferred inside NA-0351 |
| qsl-server/qsl-attachments deployment runbook evidence package | high | qsl-protocol and future service docs/scripts if authorized | runbook without drill may be weak | low to medium | config/log roots need review | no deploy-complete claim | recommended as NA-0351 output |
| qsl-server/qsl-attachments rollback drill harness | medium | service repos after authorization | rollback can touch config/data | medium | rollback artifacts likely outside qbuild | no rollback-proven claim until run | deferred inside NA-0351 |
| Off-host encrypted backup prerequisite lane | high | local ops/qsl-protocol, possibly backup config | operational scope outside repo | low to medium | likely backup-plan update | no disaster-recovery claim until done | prerequisite, not selected now |
| Local Codex workflow-support and history index lane | high | local ops paths, maybe qsl-protocol ops docs | out of production path | low | should add directives/requests/journals/ops coverage | no production claim | deferred |
| External review readiness gap audit | medium | qsl-protocol | premature before service gates | low | none | must say review incomplete | deferred |
| Website/public claim boundary audit | medium | qsl-protocol public docs or website if authorized | public-copy risk | low | none | conservative only | deferred |
| Blocker continuation | low | qsl-protocol | no current source/CI blocker found | low | none | no claim change | rejected |

## Backup-plan impact review

NA-0350 changed only qsl-protocol governance/testplan paths under
`/srv/qbuild/work`. The current local continuity backup status reports
`/srv/qbuild/work` and Codex responses as included. Therefore NA-0350 itself
does not require a backup-plan update.

Current coverage classification:

- `/srv/qbuild/work`: included by local continuity backup; covers qsl-protocol,
  qsl-server, and qsl-attachments local source checkouts.
- `/home/victor/work/qsl/codex/responses`: included by current installed source
  list.
- `/home/victor/work/qsl/codex/directives`: absent locally during inspection,
  not currently proven covered.
- `/home/victor/work/qsl/codex/requests`: present, not listed in current
  installed source list.
- `/home/victor/work/qsl/codex/journals`: absent locally during inspection,
  not currently proven covered.
- `/home/victor/work/qsl/codex/ops`: present, not listed as a source root in
  current installed source list except through referenced status files.

Future production hardening should require backup-plan review if it introduces
or relies on:

- qsl-server or qsl-attachments production runtime data roots;
- service env/config roots under `/etc`;
- systemd units under `/etc/systemd/system`;
- proxy/TLS/DNS/CDN configs;
- monitoring/log/alert artifacts;
- deploy/rollback backups;
- restore fixtures or evidence directories outside `/srv/qbuild/work`;
- local directive/request/journal/ops history as continuity-critical evidence.

Future production or public claims require an off-host encrypted backup and
restore proof. Same-host local continuity backup must remain labeled as local
continuity only.

## Deploy/rollback/secrets/monitoring plan

Future implementation authorization should define:

- deployment topology: hosts, qsl-server, qsl-attachments, reverse proxy, TLS,
  private app ports, public ingress toggle, and version pair;
- service manager boundary: exact systemd units, users/groups, working
  directories, writable paths, restart policies, and hardening;
- environment/secret source boundary: env var names, storage location,
  permissions, secret manager references, rotation, and no-value evidence rule;
- qsl-server runtime config boundary: body/queue/route/rate/TTL/auth settings;
- qsl-attachments runtime config boundary: storage root, object ceiling,
  reserve bytes, session/object TTLs, size-class policy, and capability limits;
- data/storage boundary: qsl-server non-durable queue state and qsl-attachments
  storage root with sessions/objects;
- backup preflight: disk, source roots, excluded roots, mounted backup target,
  off-host destination, encryption status, and restore target;
- deploy preflight: clean source, artifact checksum, config diff, service unit
  diff, proxy/TLS diff, backup capture, and rollback material;
- rollback preflight: known-good binary/config/unit/proxy state and data
  compatibility check;
- post-deploy verification: local service probes, public or private ingress
  probes as authorized, auth rejects, route/API behavior, attachment create/
  commit/fetch, log redaction, and monitoring events;
- rollback verification: service health, version identity, data compatibility,
  retention/purge behavior, and log redaction;
- emergency stop: disable public ingress, stop service, preserve logs, and keep
  storage roots intact for inspection;
- backup restore drill: non-destructive restore to a separate target first,
  then exact authorization before any live restore.

## Public-ingress/timing/traffic-shape boundary

Public ingress remains future-gated. qsl-server runbooks require upstream TLS
and private app port posture. qsl-attachments reference deployment binds the app
to loopback behind Caddy. NA-0349 used a loopback test listener only.

This plan does not prove hidden timing metadata, hidden traffic shape, hidden
attachment size, or hidden all-metadata behavior. Size-class evidence reduces
precision under bounded policies but still leaves observable size-class, timing,
request count, retry, and traffic-shape surfaces.

## External-review sensitivity

External review remains not complete. Service-local, demo-local, and modeled
end-to-end harnesses are prerequisite evidence, not a completed external-review
package. Any future external-review readiness lane should wait until production
backup/deploy/rollback, monitoring/logging, public-ingress, and backup/restore
evidence are sharper, or should explicitly classify those items as gaps.

## Future validation/marker/verification-bundle plan

Future NA-0351 marker candidates:

- `NA0351_PRODUCTION_BACKUP_HARDENING_PLAN_OK`
- `NA0351_QSL_SERVER_SOURCE_BACKUP_BOUNDARY_OK`
- `NA0351_QSL_ATTACHMENTS_SOURCE_BACKUP_BOUNDARY_OK`
- `NA0351_RUNTIME_CONFIG_BACKUP_BOUNDARY_OK`
- `NA0351_SERVICE_DATA_BACKUP_BOUNDARY_OK`
- `NA0351_LOG_REDACTION_BACKUP_BOUNDARY_OK`
- `NA0351_RESTORE_DRILL_PLAN_OK`
- `NA0351_DEPLOY_BOUNDARY_OK`
- `NA0351_ROLLBACK_BOUNDARY_OK`
- `NA0351_SECRETS_ENV_BOUNDARY_OK`
- `NA0351_MONITORING_BOUNDARY_OK`
- `NA0351_PUBLIC_INGRESS_BOUNDARY_OK`
- `NA0351_OFF_HOST_BACKUP_BOUNDARY_OK`
- `NA0351_NO_PRODUCTION_READY_CLAIM_OK`
- `NA0351_NO_PUBLIC_INTERNET_READY_CLAIM_OK`
- `NA0351_NO_METADATA_FREE_CLAIM_OK`
- `NA0351_NO_ANONYMITY_CLAIM_OK`

Recommended verification bundle:

- qsl-server/qsl-attachments source/authority/CI refresh;
- changed-path scope guard for any future service repo mutation;
- backup source-root inventory and restore target inventory;
- service config/unit/proxy/env redacted manifests;
- deploy dry-run or staging proof;
- rollback dry-run or staging proof;
- qsl-attachments cold/quiesced full-root restore drill;
- qsl-server route/API and qsl-attachments object lifecycle probes;
- log/monitoring redaction scan;
- public-ingress disabled/enabled boundary proof as authorized;
- overclaim scan and public-copy no-change proof.

If future hardening is blocked, use blocker markers such as:

- `NA0351_SOURCE_AUTHORITY_BLOCKED`
- `NA0351_BACKUP_SCOPE_BLOCKED`
- `NA0351_OFF_HOST_BACKUP_BLOCKED`
- `NA0351_DEPLOY_AUTHORIZATION_BLOCKED`
- `NA0351_ROLLBACK_AUTHORIZATION_BLOCKED`
- `NA0351_SECRET_BOUNDARY_BLOCKED`

## Public claim boundary

NA-0350 does not update public docs or website copy. Future public wording must
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
- local continuity backup being complete disaster recovery.

Any stronger public statement requires implementation evidence, service
evidence, deployment evidence, monitoring/log evidence, backup/restore evidence,
rollback evidence, and review evidence.

## Workflow-support and history-index future work note

Read-only history paths improved handoff confidence where present. The responses
directory was present and contained the prior NA-0349/D163 response. The
requests directory was present and contained workflow-support and history-access
requests. Directives and journals paths were absent. Local ops paths were
present.

Future local-ops work would materially reduce friction:

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

These items are not implemented in NA-0350.

## Selected successor

Selected:

`NA-0351 -- Metadata Runtime Production Backup / Deploy / Rollback Implementation Authorization Plan`

Rationale:

- qsl-server and qsl-attachments source, authority, and CI are fresh enough for
  planning;
- no current source/authority blocker requires blocker resolution;
- production backup/deploy/rollback evidence remains the highest-value next
  service gate before external review or public-claim work;
- implementation is not authorized by NA-0350 and must be scoped exactly before
  any service repo, backup, deploy, rollback, monitoring, or public-ingress
  mutation.

## Rejected alternatives

- `Metadata Runtime Production Backup / Deploy / Rollback Blocker Resolution`:
  rejected because no current source/authority/CI blocker was found.
- `Metadata Runtime Off-Host Encrypted Backup Prerequisite Plan`: deferred as a
  prerequisite to encode within NA-0351 or a following local-ops lane, because
  NA-0351 first needs to authorize exact production hardening evidence.
- `Metadata Runtime External Review Readiness Gap Audit`: deferred until
  production service evidence is sharper.
- `Metadata Runtime Website / Public Claim Boundary Audit`: deferred because
  public copy should not change before production hardening boundaries are
  explicit.
- `QSL Local Ops Codex Workflow Support and History Index Plan`: deferred as
  useful local-ops work, not the next metadata-runtime production gate.
- `Public Technical Position Paper Evidence-Bounded Draft Plan`: deferred until
  production backup/deploy/rollback boundaries are clearer.

## Backup-plan impact statement

No backup-plan update is required for NA-0350 because changed paths stay under
qsl-protocol source in `/srv/qbuild/work`, already included by the current local
continuity backup. Future production hardening may require backup-plan updates
before it introduces or relies on production service data roots, deploy configs,
rollback artifacts, restore fixtures, monitoring artifacts, or local history
directories outside current backup scope.

## Next recommendation

Proceed with NA-0351 as an implementation authorization plan. It should define
the exact future qsl-server/qsl-attachments deployment, backup, restore,
rollback, secrets/env, monitoring, log redaction, public-ingress, and off-host
backup evidence bundle before any service or local backup mutation is attempted.
