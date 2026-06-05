Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-05

# NA-0423 QSL Domain Stewardship Director Workflow Governance Authorization Plan

## Executive summary

NA-0423 authorizes an internal advisory domain stewardship model for QSL. The
model improves review quality as the project spans protocol/crypto, local
operations, public claims, dependency/CI health, and service/demo boundaries,
while preserving the current operating discipline.

Authorization result:

- Lead Director remains the final authority for directive issuance, READY
  promotion, queue order, PR merge recommendation, public-claim boundary,
  conflict resolution, and stop/retry decisions.
- Domain stewards are advisory reviewers only.
- Exactly one READY item remains mandatory.
- `NEXT_ACTIONS.md`, `DECISIONS.md`, `TRACEABILITY.md`, testplans, evidence
  docs, public-safety, branch protection, and scope guards remain controlling.
- Steward recommendations must be short, evidence-backed, and bounded to their
  requested review question.
- The selected successor is `NA-0424 -- QSL Domain Stewardship Operating Model
  Canon Implementation Harness`.

NA-0423 is internal governance only. It does not create public docs, a website
update, public technical paper content, external review, runtime behavior,
dependency changes, workflow changes, backup or restore changes, qsl-server
changes, qsl-attachments changes, qshield runtime changes, or public readiness
claims.

## Live NA-0423 scope

Live `NEXT_ACTIONS.md` lists:

`NA-0423 -- QSL Domain Stewardship / Director Workflow Governance Authorization Plan`

The live block marks NA-0423 READY with goals G1 through G5 and authorizes a
governance model for advisory domain stewardship while preserving:

- one-READY queue discipline;
- Lead Director final directive authority;
- DECISIONS and TRACEABILITY consistency;
- public-claim boundaries;
- cross-domain dependency visibility;
- the security-before-speed project goal;
- no-scope-creep governance.

Allowed qsl-protocol mutation paths for this evidence PR are limited to:

- `docs/governance/evidence/NA-0423_qsl_domain_stewardship_director_workflow_governance_authorization_plan.md`;
- `tests/NA-0423_qsl_domain_stewardship_director_workflow_governance_authorization_testplan.md`;
- `DECISIONS.md`;
- `TRACEABILITY.md`;
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

Forbidden scope includes runtime, crypto, dependency, workflow, qsl-server,
qsl-attachments, qshield runtime, website, public docs, README, START_HERE,
backup/restore, qsl-backup, qwork/qstart/qresume/qshell, branch protection,
public technical paper content, secret material, multiple READY items, and
independent autonomous Directors.

Acceptance criteria:

- stewardship roles are advisory unless a future exact scope says otherwise;
- the Lead Director remains final authority for directives and READY promotion;
- exactly one READY item remains;
- public-claim and scope boundaries are preserved;
- no runtime/dependency/workflow/public/backup mutation occurs;
- public-safety is green before and after merge.

Stop conditions:

- qwork proof mismatch or stale proof;
- PR #1114 not merged;
- queue not READY NA-0423 at start;
- D-0833 missing or D-0834 already present at start;
- public-safety red or missing;
- cargo audit not green;
- qsl-backup hash/source-list regression;
- any out-of-scope mutation;
- any model that creates independent autonomous Directors or more than one
  READY item;
- any public overclaim.

## qwork proof-file verification

Codex read, but did not run, qwork proof files:

- `/srv/qbuild/work/NA-0423/.qwork/startup.qsl-protocol.kv`;
- `/srv/qbuild/work/NA-0423/.qwork/startup.qsl-protocol.json`.

The `.kv` proof reported:

- `startup_result=OK`;
- `lane=NA-0423`;
- `repo=qsl-protocol`;
- `path=/srv/qbuild/work/NA-0423/qsl-protocol`;
- `head=f0e96ae7e0b139c49d2a0dd0850e9965342dd639`;
- `origin_main=f0e96ae7e0b139c49d2a0dd0850e9965342dd639`;
- `head_equals_origin_main=yes`;
- `worktree_clean=yes`;
- `index_clean=yes`;
- `untracked_clean=yes`;
- `ready_count=1`;
- `queue_top_ready=NA-0423`;
- `requested_lane_status=READY`.

The JSON proof parsed successfully and mirrored the required lane, repo, path,
head, origin/main, clean-state, READY count, queue-top READY, and requested-lane
status fields.

After `git fetch --all --prune`, live `HEAD` and `origin/main` both remained at
`f0e96ae7e0b1`, matching the qwork proof. PR #1114 was verified MERGED with
merge commit `f0e96ae7e0b1`.

Proof root:

`/srv/qbuild/tmp/NA0423_domain_stewardship_governance_20260605T120329-0500`

The qwork proof files were copied into the proof root under `qwork/`.

## Project goal canon inheritance

NA-0401 created the internal project goal and operating principles canon at
`docs/governance/PROJECT_GOAL_AND_OPERATING_PRINCIPLES.md`. The canon states
that QSL is security-first, evidence-driven, auditable, and honest about what is
and is not proven.

Relevant inherited markers:

- `QSL_PROJECT_GOAL_CANON_INTERNAL_ONLY`;
- `QSL_SECURITY_BEFORE_SPEED`;
- `QSL_EVIDENCE_OVER_VIBES`;
- `QSL_NO_PUBLIC_OVERCLAIMING`;
- `QSL_NO_PUBLIC_READINESS_CLAIM`;
- `QSL_ONE_READY_QUEUE_DISCIPLINE`;
- `QSL_NO_RUNTIME_CHANGE`;
- `QSL_NO_CRYPTO_IMPLEMENTATION_CHANGE`;
- `QSL_NO_DEPENDENCY_CHANGE`;
- `QSL_NO_WORKFLOW_CHANGE`;
- `QSL_DIRECTOR_CODEX_HUMAN_ROLE_BOUNDARY`;
- `QSL_PUBLIC_PAPER_TIMING_BOUNDARY`;
- `QSL_NO_SECRET_MATERIAL`.

NA-0423 follows that canon. It uses stewardship to improve evidence review and
scope discipline, not to bypass live queue authority.

## Backup/log-code chain completion inheritance

NA-0399 through NA-0422 created the current local-ops backup/log-code chain:

- NA-0399 classified backup/restore/key custody as same-host local continuity
  only, with off-host backup, real restore, key custody, and key recovery still
  future-gated.
- NA-0406 through NA-0414 authorized and implemented Codex ops backup coverage
  and status/plan updates.
- NA-0415 through NA-0421 tracked the code 23 warning, operator remediation,
  scheduled same-host log proof, and status-refresh authorization.
- NA-0422 updated the exact local backup status and plan files to cite the
  clean scheduled same-host log/manifest pair.
- D-0832 selected NA-0423 as the next stewardship governance successor.
- D-0833 closed NA-0422 and restored NA-0423 as the sole READY item.

Current inherited classification:

`STATUS_REFRESH_AUTHORIZED_CLEAN_SAME_HOST_CODE23_CLEARED`

Current clean scheduled same-host evidence remains:

- log `/backup/qsl/logs/daily-20260605T023308-0500.log`;
- manifest `/backup/qsl/manifests/daily-20260605T023308-0500.manifest.txt`;
- Codex ops source inclusion count exactly one in qsl-backup;
- `/usr/local/sbin/qsl-backup` SHA256
  `e9ecff3d22eda21ceb0e889e4dd5d6f4e270e09349c77a1f4872bfc0052f6232`.

This inheritance does not prove off-host backup, does not prove disaster
recovery, does not prove real restore, does not prove key custody, does not
prove public readiness, does not prove external review, does not prove bug-free
status, does not prove vulnerability-free status, and does not prove perfect
crypto.

## Stewardship need and problem statement

Current project scale and complexity drivers:

- Protocol and crypto evidence spans Suite-2 goals G1 through G5, PQC standards
  mapping, RFC/draft boundaries, formal models, qsc tests, and provider
  dependency health.
- Local operations now include qwork proof-file handoff, response archives,
  proof roots, rolling journal discipline, same-host backup evidence, status
  and plan caveats, and future off-host/restore/key-custody residuals.
- Public claims require separate review because source discovery, internal
  governance evidence, service-local harnesses, and external review are distinct
  evidence states.
- CI/dependency health can become an immediate release gate, as shown by the
  NA-0418 RustSec pqcrypto blocker triage and remediation.
- Product/demo/service boundaries require care because qshield demo evidence,
  qsl-server service-local evidence, and qsl-attachments service-local evidence
  must not be confused with production or public-internet evidence.

Stewardship is useful now because the Lead Director needs specialized,
evidence-backed review without surrendering queue control. The model should make
specialized concerns visible earlier, reduce missed domain-specific caveats,
and improve future directive quality.

Stewardship must not change:

- Lead Director final authority;
- exactly-one-READY queue discipline;
- public-safety and branch-protection authority;
- scope guards;
- DECISIONS / TRACEABILITY / testplan / evidence discipline;
- public-claim conservatism;
- fail-closed stop behavior.

Stewardship governance objective:

Define advisory reviewer domains, bounded recommendation format, conflict
handling, future canon path, and no-go authority boundaries so future directives
can request expert review without creating queue drift or authority confusion.

## Authority model

Lead Director final authority remains unchanged. The Lead Director owns:

- directive issuance;
- READY promotion;
- queue order;
- PR merge recommendation;
- public-claim boundary;
- conflict resolution;
- stop/retry decision.

Domain stewards are advisory reviewers. They may:

- review evidence in their requested domain;
- identify risks, blockers, evidence gaps, and claim-boundary implications;
- propose candidate lanes;
- propose DECISIONS / TRACEABILITY text;
- recommend stop, defer, accept, or future review.

Domain stewards may not:

- independently promote READY items;
- issue final Codex directives;
- merge PRs;
- create public claims;
- override `NEXT_ACTIONS.md`, `DECISIONS.md`, `TRACEABILITY.md`, branch
  protection, public-safety, or the project goal canon;
- bypass scope guards;
- approve forbidden-scope work;
- authorize secret handling or privileged operations.

Exactly one READY item remains mandatory. Steward recommendations can propose a
future candidate only; they do not create queue state.

If stewards disagree, the Lead Director records the final decision and rationale
in `DECISIONS.md`.

## Candidate stewardship domains

### 1. Crypto / Protocol Steward

Purpose:

Review qsc, qsp, qsl protocol surfaces, KEM/signature/provider choices, formal
model alignment, code/crypto audit findings, side-channel caveats, and
misuse-boundary caveats.

Protected invariants:

- G1 always-hybrid per-message keys;
- G2 SCKA monotonicity and persistence safety;
- G3 fail-closed downgrade resistance;
- G4 verification gates;
- G5 metadata-minimization review;
- no public crypto-complete claims.

Review inputs:

- canonical specs, protocol tests, formal models, DECISIONS, TRACEABILITY,
  dependency health, code/crypto audit evidence, standards mapping evidence.

Recommended outputs:

- bounded risk review;
- evidence-gap list;
- protocol/security invariant impact;
- recommended tests/models/vectors;
- candidate lane proposal where needed.

Stop/escalation triggers:

- wire/protocol/crypto/state-machine semantics would change outside exact
  scope;
- fail-closed behavior would be diluted;
- provider or KEM/signature changes lack audit and test evidence;
- public claims imply crypto completion or external review without evidence.

Forbidden authority:

- no independent directive;
- no READY promotion;
- no crypto implementation authorization;
- no audit waiver authorization;
- no public assurance claim.

Sample future lanes:

- code/crypto audit follow-up triage;
- FIPS 203/204/205 evidence-to-implementation mapping;
- side-channel caveat and provider review;
- formal model coverage expansion for selected protocol surfaces.

### 2. Local Ops / Backup / Restore Steward

Purpose:

Review qwork, qstart/qresume compatibility, response archives, proof files,
rolling journal state, backup status/plan evidence, operator packets, restore
readiness, key custody, and off-host gaps.

Protected invariants:

- no backup/restore execution without exact scope;
- no qsl-backup mutation without exact scope;
- no status/plan mutation without exact scope;
- no same-host evidence inflated into off-host or restore proof;
- proof roots and response archives remain non-secret.

Review inputs:

- qwork proof files, response files, rolling journal, backup status/plan, backup
  logs/manifests, qsl-backup hash/source count, operator packet evidence.

Recommended outputs:

- local-ops evidence classification;
- backup/restore/key residual list;
- operator action boundary;
- future lane proposal if required.

Stop/escalation triggers:

- privileged action or secret material is required;
- backup/restore/qsl-backup/status/plan mutation appears outside exact scope;
- same-host evidence is overclaimed;
- proof files conflict with live repo state.

Forbidden authority:

- no backup or restore authorization;
- no root/sudo request outside exact Director directive;
- no qsl-backup mutation;
- no off-host/restore/key-custody claim.

Sample future lanes:

- off-host target tool no-secret planning;
- real restore authorization plan;
- key custody/recovery no-secret prerequisite mapping;
- durable Director State Index storage after backup impact is resolved.

### 3. Public Claims / External Review Steward

Purpose:

Review public docs, website timing, public technical paper timing, external
review package readiness, metadata/privacy/anonymity/untraceability claim
boundaries, disclosure/security policy boundaries, and public wording caveats.

Protected invariants:

- internal evidence is not public proof;
- source discovery is not external review;
- service-local evidence is not public-internet evidence;
- metadata/privacy claims require explicit evidence;
- no public readiness overclaims.

Review inputs:

- NA-0398 metadata/privacy plan, NA-0400 external review/public claim plan,
  project goal canon, public-surface inventory, service boundary evidence,
  standards mapping evidence.

Recommended outputs:

- claim-boundary risk list;
- public wording no-go list;
- readiness prerequisite map;
- public-claim review recommendation.

Stop/escalation triggers:

- proposed public-facing wording expands claims;
- website, README, START_HERE, public docs, or public paper work appears
  outside exact scope;
- public wording implies external review completion or readiness without
  evidence;
- metadata/privacy wording implies more than evidence supports.

Forbidden authority:

- no public claim creation;
- no website/public docs/public paper mutation authorization;
- no external-review-complete statement;
- no readiness claim.

Sample future lanes:

- public paper prerequisite evidence map;
- website/source verification readiness;
- external review package readiness blocker resolution;
- disclosure policy and security.txt readiness plan.

### 4. CI / Dependency / Release Health Steward

Purpose:

Review public-safety, cargo audit, RustSec/GHSA/NVD dependency health, branch
protection, required checks, release gates, stale/flaky check handling, and
dependency remediation urgency.

Protected invariants:

- public-safety remains required and green;
- cargo audit remains fail-closed;
- dependency blockers are not waived without exact authority;
- required checks and branch protection are not weakened;
- no vulnerability-free claim is made.

Review inputs:

- CI runs/checks, cargo audit, cargo tree, RustSec/GHSA/NVD results, workflow
  status, branch protection evidence, dependency decisions.

Recommended outputs:

- dependency/advisory classification;
- release-gate health summary;
- remediation lane recommendation;
- check-context risk note.

Stop/escalation triggers:

- cargo audit red;
- public-safety missing/red;
- required check mismatch;
- dependency advisory affects runtime/security-critical code;
- proposed change weakens CI or branch protection.

Forbidden authority:

- no branch-protection changes;
- no dependency mutation outside exact scope;
- no audit waiver;
- no vulnerability-free claim.

Sample future lanes:

- RustSec/GHSA blocker remediation;
- required-check context repair;
- dependency provider review;
- release-gate evidence hardening.

### 5. Product / Demo / Service Boundary Steward

Purpose:

Review qshield demo boundaries, qsl-server and qsl-attachments production
boundaries, service-local versus public-internet proof, demo/refimpl/test-only
boundaries, and user-facing readiness caveats.

Protected invariants:

- demo evidence is not production evidence;
- service-local evidence is not public-internet evidence;
- qsl-server and qsl-attachments sibling repos are not mutated without exact
  scope;
- runtime/demo/service boundaries remain explicit.

Review inputs:

- qshield evidence, qsl-server and qsl-attachments PR/evidence references,
  service boundary docs, public claim plans, runtime/demo validation.

Recommended outputs:

- service-boundary classification;
- demo/readiness caveat list;
- future production prerequisite lane proposal;
- public-claim impact note.

Stop/escalation triggers:

- production/public-internet wording is proposed without evidence;
- sibling repo mutation appears without exact scope;
- demo harness evidence is used as deployment proof;
- runtime/service behavior changes are requested under governance-only scope.

Forbidden authority:

- no service production authorization;
- no sibling repo mutation authorization;
- no demo-to-production claim;
- no public readiness claim.

Sample future lanes:

- qsl-server production boundary prerequisite map;
- qsl-attachments service readiness evidence map;
- qshield demo public-claim boundary review;
- service-local to public-internet gap analysis.

## Rejected or merged domain options

Security / Threat Steward as separate role:

- Rejected as a standalone initial domain because threat work cuts across
  crypto/protocol, public claims, CI/dependency, and service boundaries.
- Fold threat review into the relevant domain and let the Lead Director request
  cross-domain steward review when needed.

Backup Steward separate from Local Ops:

- Rejected initially because backup, qwork, response archives, proof roots,
  operator packets, restore, and key custody are operationally coupled.
- Revisit only if backup/restore work becomes frequent enough to justify a
  narrower role.

Documentation Steward separate from Public Claims:

- Rejected initially because the highest documentation risk is public claim
  expansion and evidence-bound wording.
- Internal docs hygiene can stay under Lead Director/local governance unless a
  future lane needs a specialized docs placement reviewer.

Release Steward separate from CI/Dependency:

- Rejected initially because release health is currently inseparable from
  required checks, public-safety, and advisory gates.

Human Operator Steward separate from Local Ops:

- Rejected initially because operator packets, qwork proof files, backup
  status, and local-ops caveats share the same evidence boundary.

## Steward workflow model

Required workflow stages:

1. Lead Director assigns or requests steward review.
2. Steward reviews only the allowed scope and requested question.
3. Steward produces a bounded recommendation with:
   - summary;
   - evidence references;
   - affected goals G1 through G5;
   - risk level;
   - claim-boundary implications;
   - proposed NEXT_ACTION candidate, if any;
   - stop condition, if any.
4. Lead Director decides whether to incorporate the recommendation.
5. If accepted, Lead Director issues the single final directive.
6. Codex executes only the final Lead Director directive.
7. Evidence is recorded in DECISIONS, TRACEABILITY, evidence docs, or testplans
   as appropriate.

Fail-closed rules:

- If a steward recommendation conflicts with the live queue, the live queue
  wins.
- If a recommendation requires forbidden scope, it becomes a future candidate,
  not current work.
- If a recommendation implies public claims, public-claim review is mandatory.
- If a recommendation affects runtime/crypto/security-critical code, code/crypto
  steward and CI/dependency steward review should be considered.
- If a recommendation affects backup/restore/key custody, local ops steward
  review should be considered.
- If a recommendation affects public docs/website/paper, public claims steward
  review is mandatory.
- If live evidence is stale, contradictory, or incomplete for a safety decision,
  stop and resolve the evidence conflict before continuing.

## Steward review template

Reusable internal template for future steward reviews:

```md
Steward domain:
Requested review question:
Current NA item:
Current authoritative state checked:
Evidence reviewed:
Findings:
Risk classification:
- BLOCKER
- HIGH
- MEDIUM
- LOW
- INFO
- CLAIM_BOUNDARY
- EVIDENCE_INCOMPLETE
Goals affected:
Scope impact:
Public-claim impact:
Test/validation impact:
Backup/restore/key impact, if any:
Dependency/CI impact, if any:
Recommended action:
- accept current lane
- add future candidate
- stop current lane
- require external/source review
- no action
Explicit no-go statements:
Suggested DECISIONS / TRACEABILITY entries:
Steward confidence and caveats:
```

Template rules:

- Use short evidence references rather than broad narrative.
- State whether the recommendation is within current scope or future-gated.
- Do not include public-facing wording unless public-claim review is explicitly
  requested.
- Do not imply queue changes. Candidate wording is advisory until the Lead
  Director acts.
- Use risk labels consistently.

No separate public-facing template artifact is created by NA-0423.

## Conflict/escalation model

Conflict cases and required resolution:

Steward vs steward conflict:

- Lead Director records the chosen path and rationale.
- If the conflict affects safety/scope and cannot be resolved, stop.

Steward vs Lead Director conflict:

- Lead Director final authority controls, but rationale must be evidence-backed
  and recorded when material.
- If live evidence contradicts the Director path, stop and repair the evidence
  conflict.

Steward recommendation vs `NEXT_ACTIONS.md`:

- Live queue wins.
- Candidate can be proposed only if one-READY discipline is preserved.

Steward recommendation vs public-safety/CI:

- public-safety/CI controls merge readiness.
- Red/missing required checks require diagnostic evidence and cannot be bypassed.

Steward recommendation vs public-claim boundary:

- Public-claim review is mandatory.
- Unsupported public claims are rejected or future-gated.

Steward recommendation requiring out-of-scope changes:

- Treat as future candidate or stop if it blocks truthfulness.

Steward recommendation requiring external source lookup:

- Use current primary sources when authorized. If exact source review is outside
  current scope, future-gate it.

Steward recommendation requiring operator secrets or privileged actions:

- Stop unless a future exact Director directive authorizes safe handling.
- Stewards cannot request secret material directly.

No independent READY promotion, no background work promises, and no autonomous
Director authority are authorized.

## Implementation options

Option 1 - Keep stewardship as evidence-doc policy only.

- Pros: lightweight, no new canon file, low churn.
- Cons: harder to reuse, easy to lose in historical evidence.

Option 2 - Create a governance canon artifact in a future lane.

- Pros: reusable, easy to cite, better onboarding, clearer authority model.
- Cons: must explicitly remain subordinate to live queue, DECISIONS,
  TRACEABILITY, CI, branch protection, and evidence.

Option 3 - Create local helper/templates in future.

- Pros: structured recommendations and repeatable review shape.
- Cons: additional tooling and maintenance; avoid until model stabilizes.

Option 4 - Split into independent Directors now.

- Rejected. It risks queue drift, conflicting directives, inconsistent
  public-claim boundaries, and scope confusion. No exact safeguards justify it.

Recommended option:

Authorize a future internal governance canon artifact:

`docs/governance/DOMAIN_STEWARDSHIP_AND_DIRECTOR_WORKFLOW.md`

Do not authorize independent Directors. Keep helper/template tooling
future-gated.

## Selected successor

Selected normal successor:

`NA-0424 -- QSL Domain Stewardship Operating Model Canon Implementation Harness`

Rationale:

- The stewardship model is clear enough to canonize.
- No scope blocker was found.
- Deferring to code/crypto audit follow-up would lose the newly authorized
  stewardship model before it becomes reusable.
- The future canon can improve Director/Codex handoff while preserving the
  Lead Director final authority and one-READY invariant.

## Future path/scope bundle

Future allowed paths for normal NA-0424:

- `docs/governance/DOMAIN_STEWARDSHIP_AND_DIRECTOR_WORKFLOW.md`;
- `docs/governance/evidence/NA-0424_qsl_domain_stewardship_operating_model_canon_implementation_harness.md`;
- `tests/NA-0424_qsl_domain_stewardship_operating_model_canon_implementation_testplan.md`;
- `DECISIONS.md`;
- `TRACEABILITY.md`;
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

Future forbidden unless exact scope authorizes:

- changing one-READY invariant;
- creating independent autonomous Directors;
- public docs/website changes;
- runtime/crypto/dependency/workflow changes;
- qsl-server/qsl-attachments changes;
- backup/restore/qsl-backup mutations;
- qwork/qstart/qresume/qshell mutations;
- public paper content;
- public readiness claims;
- secret material handling.

## Future validation/marker plan

Future NA-0424 markers:

- `NA0424_DOMAIN_STEWARDSHIP_CANON_OK`;
- `NA0424_LEAD_DIRECTOR_FINAL_AUTHORITY_OK`;
- `NA0424_STEWARDS_ADVISORY_ONLY_OK`;
- `NA0424_ONE_READY_INVARIANT_OK`;
- `NA0424_NO_INDEPENDENT_DIRECTORS_OK`;
- `NA0424_PUBLIC_CLAIM_BOUNDARY_OK`;
- `NA0424_SCOPE_GUARD_OK`;
- `NA0424_CONFLICT_ESCALATION_MODEL_OK`;
- `NA0424_STEWARD_REVIEW_TEMPLATE_OK`;
- `NA0424_NO_RUNTIME_CHANGE_OK`;
- `NA0424_NO_DEPENDENCY_CHANGE_OK`;
- `NA0424_NO_WORKFLOW_CHANGE_OK`;
- `NA0424_NO_BACKUP_MUTATION_OK`;
- `NA0424_NO_PUBLIC_READINESS_CLAIM_OK`;
- `NA0424_NO_SECRET_MATERIAL_OK`.

## Public claim/external review/website boundary

NA-0423 is internal project governance only.

It is not:

- public docs;
- website work;
- public technical paper work;
- external review;
- production readiness;
- public-internet readiness;
- not metadata-free proof, not anonymity proof, and not untraceability proof;
- not backup proof, not restore proof, not off-host proof, and not disaster
  recovery proof;
- not vulnerability-free status;
- not bug-free status;
- not perfect-crypto status.

NA-0423 does not update README, START_HERE, docs/public, website paths, public
paper content, qsl-server, qsl-attachments, qshield runtime, workflows,
dependencies, runtime code, crypto code, backup scripts, backup status files, or
backup plan files.

## Rejected alternatives

- Create independent autonomous Directors: rejected because it conflicts with
  Lead Director final authority and one-READY discipline.
- Allow more than one READY item: rejected because it conflicts with canonical
  queue discipline.
- Implement the future canon artifact now: rejected because NA-0423 is
  authorization only.
- Create helper/tooling now: rejected because the model should be canonized
  before helper automation.
- Start public technical paper content: rejected because paper prerequisites
  remain incomplete and this lane is internal governance only.
- Resume code/crypto audit immediately: rejected as successor only because the
  stewardship model should first become reusable canon; code/crypto audit remains
  a future lane.
- Make stewardship public-facing: rejected because public-claim prerequisites
  remain future-gated.

## Backup-impact statement

NA-0423 mutates only tracked qsl-protocol governance evidence/testplan,
DECISIONS, TRACEABILITY, and rolling journal files. It does not mutate
`/usr/local/sbin/qsl-backup`, `/backup/qsl`, backup status files, backup plan
files, systemd/timer/fstab/source lists, rollback subtree paths, keys,
passphrases, restore targets, off-host targets, or local backup architecture.

Same-host backup/log-code evidence remains inherited and caveated. NA-0423 does
not prove off-host backup, disaster recovery, restore, backup completion, or key
custody.

## Validation evidence

Pre-patch validation:

- qwork proof files parsed and matched required fields;
- live `HEAD` and `origin/main` matched qwork proof at `f0e96ae7e0b1`;
- PR #1114 verified MERGED at `f0e96ae7e0b1`;
- queue helper reported READY_COUNT 1 and READY NA-0423;
- decision helper reported latest D-0833 and duplicate count zero;
- structural decision counts reported D-0832 once, D-0833 once, and D-0834
  absent;
- public-safety on `f0e96ae7e0b1` completed success;
- `cargo audit --deny warnings` exited cleanly;
- `cargo tree -i rustls-webpki --locked` reported `rustls-webpki v0.103.13`;
- pqcrypto inverse-tree probes reported package absence under the directive's
  zero-failure-safe command shape;
- qsl-backup SHA256 matched the required hash and Codex ops source inclusion
  count was exactly one.

Local validation before commit:

- exact staged path guard reported five changed paths, all allowed;
- `git diff --cached --check` passed;
- helper link-check reported `TOTAL_MISSING 0`;
- helper added-line leak scan reported `SECRET_FINDING_COUNT 0`;
- added-line overclaim scan reported `POSITIVE_OVERCLAIM_COUNT 0`;
- classifier reported `docs_only=true`, `workflow_security=false`,
  `runtime_critical=false`, and `scope_class=docs_only`;
- PR body preflight reported `MISSING_FIELD_COUNT 0` and
  `PROHIBITED_PHRASE_COUNT 0`;
- queue helper reported READY_COUNT 1 and READY NA-0423;
- decision helper reported latest D-0834, D-0832 once, D-0833 once, D-0834
  once, D-0835 absent, and duplicate count zero;
- `cargo audit --deny warnings` passed;
- `cargo tree -i rustls-webpki --locked` reported `rustls-webpki v0.103.13`;
- pqcrypto inverse-tree probes were absent under the directive's
  zero-failure-safe command shape;
- `cargo fmt --check` passed;
- `cargo +stable test -p qsc --locked --test send_commit --
  --test-threads=1` passed with 3 tests;
- `python3 formal/model_qsc_handshake_suite_id_bounded.py` passed;
- `python3 formal/run_model_checks.py` passed;
- goal-lint with a synthetic PR event and exact `Goals: G1, G2, G3, G4, G5`
  body passed.

Recovered failure:

- Failing command: context review used `rg` against
  `docs/governance/evidence/NA-0401*`, but the NA-0401 canon artifact lives at
  `docs/governance/PROJECT_GOAL_AND_OPERATING_PRINCIPLES.md`.
- Classification: recoverable command-shape/context-path mistake.
- Corrective action: reran targeted search against the actual canon path and
  NA-0401 testplans.
- Final result: corrected command exited successfully and confirmed the relevant
  canon markers.

Recovered validation scan:

- Failing command: first changed-content overclaim scan was run against whole
  changed files and reported historical terms already present on `main`.
- Classification: recoverable scan-shape mistake because the required check is
  for the added PR payload.
- Corrective action: reran the scanner against added lines only.
- Final result: the added-line scan found line-wrapping/content-hardening issues
  in new negative no-go lists; the wording was tightened so claim terms carry a
  local negative guard.
- Final rerun result: `POSITIVE_OVERCLAIM_COUNT 0`.

Recovered goal-lint command shape:

- Failing command: `GITHUB_EVENT_PATH=/proc/self/fd/3 python
  tools/goal_lint.py ...`.
- Classification: recoverable CLI command-shape mistake because the host has
  `python3` but not `python`.
- Corrective action: reran the same synthetic PR event with `python3`.
- Final result: `OK: goal compliance checks passed.`

## Next recommendation

Merge the NA-0423 authorization PR if validation and required checks pass. After
post-merge public-safety is green, run a separate closeout to mark NA-0423 DONE
and restore:

`NA-0424 -- QSL Domain Stewardship Operating Model Canon Implementation Harness`

NA-0424 should implement only the internal governance canon artifact and
supporting evidence/testplan/traceability/journal updates.
