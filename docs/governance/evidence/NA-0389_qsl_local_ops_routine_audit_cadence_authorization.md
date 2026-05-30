Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-30

# NA-0389 QSL Local Ops Routine Audit Cadence Authorization

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0389 authorizes a future bounded routine-audit-cadence implementation
harness, not recurring audit automation.

Classification:

`ROUTINE_AUDIT_CADENCE_IMPLEMENTATION_AUTHORIZATION_READY_FOR_TEMP_OUTPUT_HARNESS`

The first future lane should implement a standalone qsl-protocol helper with
deterministic fixtures and temporary proof output only. It should validate audit
classes, triggers, depth levels, severity taxonomy, queue insertion rules,
report-output boundaries, and public-claim safeguards. It must not create a
scheduler, cron job, workflow, durable audit report store, backup mutation,
runtime change, dependency change, qsl-server change, qsl-attachments change,
secret-handling path, or public/readiness/privacy claim expansion.

Selected successor:

`NA-0390 -- QSL Local Ops Routine Audit Cadence Implementation Harness`

## Live NA-0389 scope

Live `NEXT_ACTIONS.md` records:

- READY_COUNT `1`.
- READY `NA-0389 -- QSL Local Ops Routine Audit Cadence Authorization Plan`.
- NA-0388 DONE.
- D-0758 exists once.
- D-0759 exists once.
- D-0760 absent at startup.
- public-safety remains required and green.

Live objective:

- authorize a recurring overall-project and code/crypto audit cadence;
- define trigger conditions, scope classes, report locations, backup impact,
  public-claim boundaries, and queue-entry handling;
- preserve the one-READY queue.

Live protections:

- no runtime, service, protocol, crypto, dependency, or workflow implementation
  unless a future exact scope authorizes exact files;
- no secret handling;
- no backup script, timer, fstab, or source-list mutation unless a future exact
  scope authorizes it;
- no target setup;
- no public, readiness, or privacy overclaim.

NA-0389 live scope is compatible with this authorization-only directive. It
does not authorize a helper implementation in NA-0389.

## Inherited NA-0388 catalog result

NA-0388 implemented the response/archive/history catalog temp-output harness:

- helper: `scripts/ci/qsl_response_history_catalog.py`;
- fixtures: `inputs/local_ops/response_history_catalog_fixtures/`;
- qsl-protocol PR #1039 merged as `c928998a298f`;
- qsl-protocol PR #1040 closed NA-0388 and restored NA-0389 as sole READY;
- live catalog:
  `/srv/qbuild/tmp/NA0388_response_history_catalog_20260530T141441-0500/live/catalog.json`;
- live catalog SHA-256:
  `3ab3fbec0309a1167a7a55ede4cd55b4836b3ed8b632b5eac3749b7f6b94418c`;
- fixture catalog SHA-256 prefix: `9422809fde32`;
- fixture log SHA-256 prefix: `53a51dc1c7c3`.

The live scan recorded responses `169`, requests `2`, directives absent,
journals absent, ops `8`, total file entries `179`, and live sentinel count
`0`.

Inherited boundaries:

- metadata-only catalog output;
- no full body copy;
- no response, request, directive, journal, or ops-history mutation;
- no durable catalog or index;
- no helper mutation by NA-0389;
- no backup-plan update for temp proof output;
- future durable catalog or audit report output requires separate
  backup-impact review.

## Prior audit evidence and routine-audit need review

The NA-0380 read-only audit reports were present and matched expected SHA-256
values:

| Report | Path | SHA-256 status |
|---|---|---|
| Overall project audit | `/srv/qbuild/tmp/NA0380_post_completion_audit_20260529T005653-0500/NA0380_overall_project_readonly_audit.md` | matched `66dd26c0b35b97113f160e4dd67fdc9992bd3be91c72452359fbef74dcef0913` |
| Code/crypto audit | `/srv/qbuild/tmp/NA0380_post_completion_audit_20260529T005653-0500/NA0380_code_crypto_readonly_audit.md` | matched `70c21179e7a57dd168dff77e2d5bb18ac2ad1c7c285b216da7875ca712d1c099` |

Routine audits are needed because the project now has many active evidence
streams: qsl-protocol governance, long qsc validation, formal models, helper
scripts, local history roots, same-host continuity backup, qsl-server service
proof, qsl-attachments service proof, external-review readiness, and
public-claim boundaries.

Open findings inherited from NA-0380 include:

- local history and response archive backup/restore coverage remains partial;
- full workspace validation is expensive and needs profiles;
- directive/response/journal/history handoff support is improving but still
  fragmented;
- duplicate crypto-related dependency families complicate review;
- broad clippy/all-targets profiles are not clean routine gates;
- refimpl/actor deterministic harness boundaries need explicit reviewer-facing
  classification;
- service-local qsl-server/qsl-attachments evidence is not production or
  public-internet proof;
- external-review and public technical paper readiness remain future-gated.

Routine audits should produce evidence and proposed queue candidates. They
should not mutate code while auditing, should not bypass the one-READY queue,
and should not turn findings into automatic READY items.

## Routine audit types and depth levels

### Overall project audit

Scope:

- queue health;
- decision sequencing and uniqueness;
- traceability coverage;
- evidence/testplan completeness;
- CI and public-safety health;
- dependency/advisory health;
- backup/restore/deploy/rollback boundaries;
- local-ops helper health;
- response/history catalog coverage;
- operator blockers;
- service and public-claim boundaries.

Purpose:

- detect governance drift, stale evidence, queue risk, and claim-boundary risk.

Typical triggers:

- fixed PR-count or NA-count threshold;
- before public paper or external-review package work;
- after public-safety, backup, deploy, rollback, or service-related changes;
- operator demand.

Expected report format:

- summary, evidence table, findings table, queue-candidate table, claim-boundary
  table, validation summary, and backup-impact summary.

Allowed sources:

- qsl-protocol repository state, GitHub PR/check status, local temp evidence,
  read-only local history roots, qsl-server/qsl-attachments read-only PR state,
  and mounted backup status.

Forbidden actions:

- runtime mutation, helper mutation unless separately authorized, scheduler
  creation, workflow mutation, backup mutation, secret handling, durable report
  writes, public-claim expansion, qsl-server/qsl-attachments mutation.

CI/local checks:

- queue/decisions, branch protection, public-safety, cargo audit, rustls-webpki
  tree, link-check, leak-scan, scope guard, and selected heavy checks.

Output storage policy:

- future first harness should write only `/srv/qbuild/tmp` proof and
  qsl-protocol governance evidence.

Severity and queue-impact rules:

- findings propose queue candidates; they do not create READY items by
  themselves.

### Code/crypto audit

Scope:

- crypto API usage;
- nonce/key lifecycle;
- RNG usage;
- deterministic/test-only boundaries;
- panic/unwrap/expect/error handling;
- unsafe code;
- side-channel/timing limitations;
- formal-model alignment;
- property/fuzz gaps;
- dependency/advisory risk;
- qsc/qshield/qsp/protocol boundaries;
- public-claim implications.

Purpose:

- identify correctness, cryptographic misuse, lifecycle, and assurance gaps.

Typical triggers:

- after protocol, crypto, runtime, qsc, qshield, dependency, formal model, or
  advisory changes;
- before external review;
- before public technical paper;
- after critical or high audit findings.

Expected report format:

- scoped component inventory, command evidence, findings table, reviewer
  readiness table, and explicit limitations.

Allowed sources:

- repository source, tests, vectors, formal models, Cargo metadata, advisory
  output, GitHub checks, and bounded read-only service proof.

Forbidden actions:

- code mutation during audit, secret collection, production claims, public
  internet claims, external-review-complete claims, or durable secret-bearing
  report output.

CI/local checks:

- cargo audit, cargo tree duplicates, focused qsc tests, formal model checks,
  qshield-cli tests/build where feasible, refimpl/actor tests where feasible,
  and leak/claim scans.

Output storage policy:

- temp-output proof first; durable reviewer package only under future exact
  authorization and backup-impact review.

Severity and queue-impact rules:

- CRITICAL/HIGH findings that affect active lane safety require explicit future
  directive triage before continuing affected claims or changes.

### Local-ops/history/backup audit

Scope:

- response, request, directive, journal, and ops roots;
- helper stack;
- qstart/qresume guard;
- bounded check polling helper;
- manifest/allow-file helper;
- response writer;
- response history catalog;
- backup source list/status;
- same-host continuity caveat;
- off-host and restore blockers.

Purpose:

- ensure operator evidence is recoverable, searchable, and not overstated.

Typical triggers:

- after helper changes, backup/restore/deploy/rollback changes, local history
  changes, or operator-demand events.

Expected report format:

- root inventory, source-list comparison, mutation proof, backup-impact
  classification, and blocker table.

Allowed sources:

- local history roots read-only, backup status files read-only, mounted backup
  status, qsl-protocol helper outputs, and qsl-protocol governance files.

Forbidden actions:

- backup execution, restore execution, key handling, target setup, script/timer
  mutation, source-list mutation, archive mutation, or durable report writes.

CI/local checks:

- helper `--help`, fixture matrices, py_compile, scope guard, leak scan, and
  filesystem read-only inventory.

Output storage policy:

- temp-output proof first; durable local audit directory only after backup
  coverage review.

Severity and queue-impact rules:

- backup-impact findings may propose blocker lanes before durable output lanes.

### Public-claim/external-review readiness audit

Scope:

- README/START_HERE/docs-public boundaries if future scope allows;
- website/public-doc claim changes if future scope allows;
- external-review package readiness;
- service proof boundary;
- backup/restore/deploy readiness;
- code/crypto audit status;
- evidence map completeness.

Purpose:

- prevent public language from outrunning evidence.

Typical triggers:

- before public paper, website/public-doc changes, external-review package,
  production/public-internet claims, or public-facing service claims.

Expected report format:

- claim table, evidence source, allowed wording, forbidden wording, gaps, and
  proposed queue candidates.

Allowed sources:

- qsl-protocol governance/evidence, read-only GitHub state, previous audit
  reports, service-local proof, backup status, and public docs only when future
  scope allows.

Forbidden actions:

- website/public docs update, public claim expansion, external-review-complete
  claim, production claim, anonymity claim, metadata-free claim, untraceable
  claim, or proof-of-absence claim.

CI/local checks:

- overclaim scan, leak scan, link-check, goal-lint, public-safety, and selected
  evidence-source checks.

Output storage policy:

- temp-output proof and governance evidence only unless future scope authorizes
  a durable claim register.

Severity and queue-impact rules:

- CLAIM_BOUNDARY findings may propose public-claim audit lanes and may block
  public-surface changes.

### Targeted incident/regression audit

Scope:

- one incident, failed check, advisory, suspicious regression, claim boundary,
  backup/deploy event, or high-risk helper change.

Purpose:

- produce bounded evidence without reopening a full project audit.

Typical triggers:

- red required CI, dependency advisory, branch protection issue, helper
  regression, backup/deploy/restore surprise, or a CRITICAL/HIGH finding.

Expected report format:

- incident summary, timeline, root-cause confidence, affected surfaces,
  corrective candidates, stop/go recommendation, and evidence gaps.

Allowed sources:

- only sources necessary for the incident.

Forbidden actions:

- broad unrelated audit, hidden code changes, scheduler creation, backup
  mutation, remote target setup, or public overclaim.

CI/local checks:

- targeted reproduction and validation only.

Output storage policy:

- temp-output proof first; durable incident artifact only after explicit scope.

Severity and queue-impact rules:

- if active lane safety is affected, future directive must decide whether to
  stop, promote a blocker, or continue with caveat.

### Depth levels

| Depth | Purpose | Typical use | Boundaries |
|---|---|---|---|
| Quick read-only audit | Fast posture check | before closeout or after low-risk docs/governance work | no broad code scan, no durable report |
| Standard read-only audit | Routine cadence pass | fixed PR/NA thresholds or public-claim preparation | bounded heavy checks, findings table |
| Deep read-only microscope audit | High confidence local review | before external review or after major lane clusters | broader source/test/formal review, still read-only |
| Targeted code/crypto microscope audit | Focused security audit | after protocol/crypto/runtime/dependency risk | narrow component, high evidence density, no implementation |

## Cadence and trigger design

Recommended future trigger policy:

- run a quick overall audit every 5 merged qsl-protocol PRs or every 3 closed NA
  items, whichever occurs first;
- run a standard overall audit every 10 merged qsl-protocol PRs or every 5
  closed NA items;
- run a standard code/crypto audit after any protocol, crypto, qsp, qsc,
  qshield runtime, dependency, advisory, or formal-model change;
- run a local-ops/history/backup audit after helper, response archive, local
  history, backup, restore, deploy, rollback, or qstart/qresume changes;
- run a public-claim/external-review readiness audit before public technical
  paper work, website/public-doc claim changes, external-review package work, or
  any production/public-internet claim proposal;
- run a targeted incident/regression audit after red required CI, stale/flaky
  CI patterns, high-risk public-safety changes, qsl-server/qsl-attachments
  production-related changes, dependency/advisory events, or CRITICAL/HIGH
  findings;
- allow operator-demand audits at any time.

No scheduler, cron job, GitHub workflow, timer, or background automation is
authorized by NA-0389.

## Audit report location / backup / retention options

| Option | Backup impact | Secret risk | Durability | Searchability | Restore value | Authority | CI suitability | Recommendation |
|---|---|---|---|---|---|---|---|---|
| `/srv/qbuild/tmp` temp reports only | Same-host temp area; not durable authority | Low if metadata-only | Low | Medium for current session | Low | Proof only | High | Use for first NA-0390 harness |
| qsl-protocol tracked report summaries | Covered by git once merged | Low to medium; must be bounded | High | High | High for governance summaries | Authoritative/supporting if reviewed | Medium | Use only concise summaries/evidence |
| `/home/victor/work/qsl/codex/ops/audits` | Backup coverage not proven for this exact future directory | Medium | Medium to high if backed up | High | Medium | Local ops only | Low | Defer pending backup-impact review |
| `/home/victor/work/qsl/codex/responses` embedded final response only | Response archive is same-host continuity covered | Low if final response only | High locally | Medium | Medium | Handoff response only | Low | Keep final response, not report store |
| No durable report, manual response summary only | No new backup impact | Low | Low | Low | Low | Weak | n/a | Acceptable fallback, not enough for harness proof |

Expected future first implementation:

- write fixture and proof output under `/srv/qbuild/tmp/NA0390_routine_audit_cadence_*`;
- commit only helper, fixtures, evidence, testplan, decisions, traceability, and
  journal updates;
- forbid durable audit report storage unless a later directive explicitly
  authorizes exact paths and backup-impact handling.

NA-0389 itself requires no backup-plan update because it changes only
qsl-protocol governance/evidence/testplan/traceability/journal paths.

## Audit finding taxonomy and queue insertion policy

Severity taxonomy:

- CRITICAL: active safety, security, or truthful-governance issue that can make
  continuation unsafe unless triaged.
- HIGH: serious security, correctness, backup, CI, or claim-boundary risk that
  likely needs near-term queue action.
- MEDIUM: material risk or evidence gap that should become a candidate lane.
- LOW: improvement or cleanup with limited immediate risk.
- INFO: neutral evidence or context.
- EVIDENCE_INCOMPLETE: insufficient proof; no negative claim may be made.
- CLAIM_BOUNDARY: wording or public-surface risk where evidence does not support
  stronger claims.
- BACKLOG_CANDIDATE: useful future work that is not urgent.

Required finding fields:

- finding ID;
- title;
- severity;
- affected area;
- evidence;
- confidence;
- risk;
- recommended action;
- proposed NEXT_ACTIONS candidate;
- goals mapping;
- scope category;
- public-claim implication;
- backup impact;
- external-review implication;
- owner/dependency;
- blocked/unblocked status.

Queue insertion rules:

- audit findings do not automatically create READY items;
- findings may propose BACKLOG or READY candidates;
- exactly one READY item remains enforced;
- CRITICAL/HIGH findings that affect active lane safety require explicit future
  directive triage before affected work continues;
- CLAIM_BOUNDARY findings may make a public-claim or website audit the next
  candidate, but only through normal queue discipline;
- new NA entries require decisions and traceability when promoted;
- audit reports must preserve evidence, scope, recommended next lane, and claim
  impact without mutating unrelated queue items.

## Code / crypto audit cadence scope design

Recurring code/crypto audits must include:

- crypto API misuse review;
- nonce lifecycle review;
- key lifecycle review;
- RNG usage review;
- deterministic/test-only boundary review;
- panic/unwrap/expect/error-handling review;
- unsafe code review if any;
- side-channel, timing, and traffic-shape limitation review;
- formal model alignment review;
- property/fuzz test opportunities;
- duplicate dependency-family review;
- cargo audit and advisory review;
- broad clippy/test gaps separated from merge-critical gates;
- qsc/qshield/qsp/protocol boundary review;
- qsl-server/qsl-attachments service-local proof boundary review;
- public-claim implications.

Audit findings are not proof that bugs are absent. They are not external review.
They are not production readiness. They are not public-internet readiness. A
code/crypto audit report must not contain secrets, tokens, private keys,
recovery material, host fingerprints, or secret-bearing path tokens.

## Overall project audit cadence scope design

Recurring overall project audits must include:

- NEXT_ACTIONS queue health;
- DECISIONS sequencing and uniqueness;
- TRACEABILITY coverage;
- evidence/testplan completeness;
- CI/public-safety health;
- cargo audit/dependency health;
- backup/restore/deploy/rollback boundaries;
- local-ops helper health;
- response/history catalog coverage;
- operator input blockers;
- qsl-server/qsl-attachments production boundary;
- qshield demo/non-production boundary;
- public-claim/external-review readiness;
- website/public-doc boundary;
- D132 preservation or cleanup status;
- public technical position paper timing;
- audit finding backlog and closure status.

## Routine audit helper / policy implementation options

| Option | Value | Risk | Backup impact | Workflow impact | Dependency impact | Secret risk | CI suitability | Testability | Authority | Recommendation |
|---|---|---|---|---|---|---|---|---|---|---|
| Standalone `scripts/ci/qsl_routine_audit_cadence.py` | Clear owner, isolated policy schema, fixtureable | New helper surface | Temp-output first avoids new backup plan | None if not in workflow | None if stdlib | Low with metadata-only fixtures | High | High | qsl-protocol governed | Preferred for NA-0390 |
| Docs-only policy first | Lowest implementation risk | No executable guard | None | None | None | Low | Low | Low | Governance only | NA-0389 already supplies policy; insufficient next |
| Extend response history catalog helper | Reuses recent helper | Mixes catalog and audit policy concerns | Could blur output policy | None | None | Medium | Medium | Medium | Less clear | Reject for first harness |
| Extend directive manifest validator | Reuses scope schema | Overloads directive identity tool | None | None | None | Low | Medium | Medium | Less clear | Reject for first harness |
| Extend qsl_evidence_helper | Centralizes governance checks | Existing helper grows too broad | None | None | None | Medium | Medium | Medium | Broad | Defer |
| Local `/srv/qbuild/tools` helper | Operator convenient | Outside repo governance, backup/tool mutation risk | Requires local backup review | None | None | Medium | Low | Medium | Local only | Reject for NA-0390 |
| GitHub workflow / cron | Automated cadence | Hidden background work and workflow mutation | n/a | High | Possible | Medium | High | Medium | CI | Reject for first implementation |
| Manual-only status quo | No code | Repeats directive bloat | None | None | None | Low | n/a | Low | Weak | Reject as next step |

## First-lane authorization decision

Decision:

`ROUTINE_AUDIT_CADENCE_IMPLEMENTATION_AUTHORIZATION_READY_FOR_TEMP_OUTPUT_HARNESS`

Rationale:

- NA-0380 audit reports are present and checksummed;
- NA-0388 proves the local history catalog helper can use metadata-only
  temp-output proof without mutating archives;
- future first implementation can avoid durable audit report storage;
- backup/report-location gaps can be handled as explicit future output
  boundaries;
- a standalone helper with fixtures improves repeatability without creating
  background automation.

This decision does not implement NA-0390.

## Future allowed/forbidden path bundle

Future NA-0390 allowed paths if its live scope matches this authorization:

- `scripts/ci/qsl_routine_audit_cadence.py`;
- `inputs/local_ops/routine_audit_cadence_fixtures/`;
- `docs/governance/evidence/NA-0390_qsl_local_ops_routine_audit_cadence_harness.md`;
- `tests/NA-0390_qsl_local_ops_routine_audit_cadence_testplan.md`;
- `DECISIONS.md`;
- `TRACEABILITY.md`;
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`;
- temporary output under `/srv/qbuild/tmp/NA0390_routine_audit_cadence_*`.

Future durable audit reports are forbidden unless future live scope explicitly
authorizes exact paths and backup-impact handling.

Forbidden future paths unless separately authorized:

- `.github/**`;
- `scripts/ci/qsl_response_history_catalog.py`;
- `scripts/ci/qsl_codex_response_writer.py`;
- `scripts/ci/qsl_evidence_helper.py`;
- `scripts/ci/qsl_bounded_check_poll.py`;
- `scripts/ci/qsl_directive_manifest_validate.py`;
- `scripts/ci/public_safety_gate.py`;
- workflows;
- Cargo files and dependency files;
- runtime, service, protocol, crypto, qsc, qsp, qshield runtime paths;
- qsl-server and qsl-attachments;
- backup scripts, timers, fstab, source lists, and system services;
- website/public docs, README, START_HERE;
- durable local audit reports under `/home/victor/work/qsl/codex/**`;
- response archive mutation;
- directive, journal, request, or ops-history mutation.

## Audit cadence / public technical paper interaction

The public technical position paper remains future-gated. Routine audit cadence
can improve evidence discipline before a paper, but NA-0389 does not draft the
paper.

Before paper work, require:

- fresh public-claim boundary audit;
- external-review readiness assessment;
- current code/crypto audit status;
- service boundary status;
- backup/restore status;
- evidence map linking each claim to bounded proof.

The paper must avoid overstating anonymity, metadata-free behavior,
untraceability, production readiness, public-internet readiness, external review
completion, or proof that bugs are absent.

## Governance/security/fail-closed requirements

Required future behavior:

- no audit scheduler;
- no background work;
- no cron/workflow;
- no durable report output unless separately authorized;
- no code mutation while auditing;
- no secret copying;
- no public-claim expansion;
- no audit finding auto-READY insertion;
- one READY item preserved;
- deterministic report schema;
- bounded local checks;
- no secret material in reports;
- evidence gaps marked `EVIDENCE_INCOMPLETE`;
- audit reports must not claim bug-free status;
- audit reports must not claim perfect crypto.

## Public-claim/external-review/website boundary

NA-0389 is authorization only. It does not prove:

- production readiness;
- public-internet readiness;
- external review completion;
- metadata-free behavior;
- anonymity;
- untraceability;
- absence of bugs;
- perfect crypto.

No website, docs/public, README, or START_HERE update is made by NA-0389. Public
technical paper work remains future-gated.

## Future validation/marker/verification plan

Future NA-0390 fixture and proof output should include these markers:

- `NA0390_ROUTINE_AUDIT_CADENCE_AUTHORIZATION_OK`;
- `NA0390_ROUTINE_AUDIT_CADENCE_HELPER_OK`;
- `NA0390_OVERALL_PROJECT_AUDIT_PROFILE_OK`;
- `NA0390_CODE_CRYPTO_AUDIT_PROFILE_OK`;
- `NA0390_LOCAL_OPS_HISTORY_BACKUP_AUDIT_PROFILE_OK`;
- `NA0390_PUBLIC_CLAIM_REVIEW_AUDIT_PROFILE_OK`;
- `NA0390_AUDIT_TRIGGER_POLICY_OK`;
- `NA0390_AUDIT_SEVERITY_TAXONOMY_OK`;
- `NA0390_AUDIT_QUEUE_INSERTION_POLICY_OK`;
- `NA0390_TEMP_OUTPUT_BOUNDARY_OK`;
- `NA0390_NO_DURABLE_REPORT_WRITE_OK`;
- `NA0390_NO_BACKGROUND_SCHEDULER_OK`;
- `NA0390_NO_WORKFLOW_CHANGE_OK`;
- `NA0390_NO_DEPENDENCY_CHANGE_OK`;
- `NA0390_NO_RUNTIME_CHANGE_OK`;
- `NA0390_NO_SECRET_MATERIAL_OK`;
- `NA0390_NO_BUG_FREE_CLAIM_OK`;
- `NA0390_NO_CRYPTO_PERFECT_CLAIM_OK`;
- `NA0390_NO_METADATA_FREE_CLAIM_OK`;
- `NA0390_NO_ANONYMITY_CLAIM_OK`;
- `NA0390_NO_UNTRACEABLE_CLAIM_OK`;
- `NA0390_NO_PRODUCTION_READY_CLAIM_OK`;
- `NA0390_NO_PUBLIC_INTERNET_READY_CLAIM_OK`.

Verification bundle should include helper `--help`, fixture matrix,
py_compile, temp-output validation, queue/decisions, scope guard, link-check,
leak-scan, overclaim scan, goal-lint, cargo audit, rustls-webpki tree, qsc
send_commit, formal checks, and public-safety.

## Selected successor

Selected successor:

`NA-0390 -- QSL Local Ops Routine Audit Cadence Implementation Harness`

Rationale:

- temp-output harness implementation is safe to authorize without durable audit
  report storage;
- backup/report-location gaps are preserved as explicit future blockers for
  durable output, not blockers for the first harness;
- standalone helper fixtures can prove policy logic without touching workflows,
  runtime, dependencies, backups, qsl-server, qsl-attachments, or local history
  archives.

## Rejected alternatives

- implement the audit helper in NA-0389;
- add cron, timer, scheduler, or GitHub workflow now;
- write durable audit reports now;
- store audit reports under local Codex ops paths now;
- extend response history catalog helper now;
- extend directive manifest validator now;
- extend qsl_evidence_helper now;
- change runtime, crypto, service, protocol, dependencies, workflows, backups,
  website, public docs, README, or START_HERE now;
- start public technical paper work now;
- block NA-0390 on durable report storage before proving a temp-output harness.

## Backup-plan impact statement

No NA-0389 backup-plan update is required because changed paths are limited to
qsl-protocol governance/evidence/testplan/traceability/journal files.

Future NA-0390 temp-output proof under `/srv/qbuild/tmp` does not itself
require durable backup coverage. Future durable audit reports under
`/home/victor/work/qsl/codex/**`, qsl-protocol tracked generated reports, or any
other durable local path require explicit backup-impact review before use.

Same-host local continuity must not be described as disaster recovery.

## Next recommendation

Close NA-0389 after this authorization PR merges and restore:

`NA-0390 -- QSL Local Ops Routine Audit Cadence Implementation Harness`

The NA-0390 directive should implement only the bounded standalone temp-output
harness and fixture suite. Durable audit report storage should remain forbidden
unless a later live scope explicitly authorizes exact paths and backup-impact
coverage.
