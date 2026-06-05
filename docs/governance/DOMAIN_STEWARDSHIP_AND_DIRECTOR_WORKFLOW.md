Goals: G1, G2, G3, G4, G5

Status: Authoritative
Owner: QSL governance
Last-Updated: 2026-06-05

# QSL Domain Stewardship and Director Workflow

## 1. Status and Authority

This document is the internal governance canon for QSL advisory domain
stewardship and Director workflow. It implements the NA-0424 canon path
authorized by D-0834 and restored by D-0835.

This canon is internal governance only. It is not public documentation, not
website content, not public technical paper content, not external review, not
production readiness, not public-internet readiness, and not a public-security
claim.

The Lead Director remains the final authority for directive issuance, READY
promotion, queue order, PR merge recommendation, public-claim boundary,
conflict resolution, and stop/retry decisions.

## 2. Purpose

QSL spans protocol and crypto design, formal models, implementation evidence,
local operations, backup/restore caveats, dependency health, CI gates,
public-claim timing, demos, and service boundaries. Domain stewardship gives the
Lead Director structured advisory review for those areas without creating
independent Directors or competing queue authority.

The purpose of this canon is to:

- define advisory steward domains;
- define the bounded review workflow;
- define a reusable steward review template;
- define conflict and escalation rules;
- preserve one-READY queue discipline;
- preserve evidence-backed public-claim conservatism;
- keep scope control fail-closed.

## 3. Non-Override Rule

This canon does not override `NEXT_ACTIONS.md`.

This canon does not override `DECISIONS.md`.

This canon does not override `TRACEABILITY.md`.

This canon does not override branch protection.

This canon does not override public-safety.

This canon does not override the Project Goal canon.

This canon does not override explicit directive scope, required checks, test
evidence, or live repo state.

If this canon conflicts with the live queue, accepted decisions, traceability,
branch protection, public-safety, or directive scope, the conflict is a stop
condition until the Lead Director resolves it through the governance spine.

## 4. Lead Director Final Authority

The Lead Director owns:

- final directive issuance;
- READY promotion;
- queue order;
- PR merge recommendation;
- public-claim boundary;
- conflict resolution;
- stop/retry decisions;
- final acceptance or rejection of steward recommendations.

Steward input is evidence for the Lead Director. It is not a directive by
itself. If a steward recommendation is accepted, the Lead Director issues the
single final directive and Codex executes only that final directive.

## 5. Steward Advisory Boundary

Domain stewards are advisory reviewers only.

Stewards may:

- review evidence within an assigned question and domain;
- identify risks, blockers, evidence gaps, and claim-boundary implications;
- recommend tests, model checks, vectors, or documentation evidence;
- propose future queue candidates;
- suggest DECISIONS or TRACEABILITY text;
- recommend accept, defer, stop, require external/source review, or no action.

Stewards may not:

- independently promote READY items;
- issue final Codex directives;
- merge PRs;
- create public claims;
- override `NEXT_ACTIONS.md`, `DECISIONS.md`, `TRACEABILITY.md`, branch
  protection, public-safety, or the Project Goal canon;
- bypass scope guards;
- approve forbidden-scope work;
- authorize secret handling;
- authorize privileged operations;
- create independent autonomous Directors.

This canon does not authorize independent Directors.

This canon does not authorize more than one READY item.

This canon does not authorize public claims.

This canon does not authorize runtime, crypto, dependency, workflow, or backup
changes.

## 6. One-READY Queue Discipline

Exactly one `NEXT_ACTIONS.md` item must be READY.

Stewards may recommend future candidates, but recommendations do not create
queue state. A new queue candidate is valid only when the Lead Director records
it through the governance spine and the one-READY invariant remains preserved.

If steward preference conflicts with live queue state, live queue state wins.

No steward may independently promote READY, reorder the queue, create a second
READY item, or promise background work.

## 7. Domain Stewardship Roles

### 7.1 Crypto / Protocol Steward

Purpose:

Review qsc, qsp, and qsl protocol surfaces; KEM, signature, and provider
choices; formal model alignment; code/crypto audit findings; side-channel
caveats; misuse-boundary caveats; and whether public wording would imply more
crypto assurance than evidence supports.

Review inputs:

- canonical specs;
- qsc/qsp/qsl implementation and test evidence when exact scope authorizes
  source review;
- formal models and model-check results;
- conformance vectors and negative tests;
- DECISIONS and TRACEABILITY rows;
- dependency health evidence for crypto/provider families;
- code/crypto audit findings.

Recommended outputs:

- bounded risk review;
- invariant impact summary for G1 through G5;
- evidence-gap list;
- recommended tests, vectors, models, or audit follow-up;
- candidate lane proposal where current scope is insufficient;
- claim-boundary note for public/security wording.

Escalation triggers:

- qsc, qsp, qsl, wire, crypto, key schedule, negotiation, auth, transcript,
  state-machine, provider, KEM, signature, nonce, RNG, or persistence semantics
  would change outside exact scope;
- fail-closed behavior would be diluted;
- formal model and implementation evidence conflict;
- code/crypto audit findings indicate high risk or unclear root cause;
- side-channel or misuse-boundary caveats are being ignored;
- public wording implies crypto completion or external review without evidence.

Forbidden authority:

- no independent directive;
- no READY promotion;
- no crypto implementation authorization;
- no audit waiver authorization;
- no public assurance claim;
- no crypto-complete public claim.

Example future lanes:

- QSL code/crypto audit follow-up resumption plan;
- provider and PQC standards evidence mapping;
- formal model coverage expansion for selected protocol surfaces;
- side-channel caveat and misuse-boundary review;
- fuzz, property, differential, and vector test planning.

### 7.2 Local Ops / Backup / Restore Steward

Purpose:

Review qwork and proof-file handoff, response archives, proof roots, rolling
journal state, backup status/plan evidence, operator packets, restore/key
custody/off-host gaps, same-host caveats, and local operational evidence
boundaries.

Review inputs:

- qwork proof files;
- response archives;
- rolling operations journal;
- backup status and backup plan files;
- backup logs and manifests;
- qsl-backup hash and source inclusion count;
- operator packet evidence;
- restore, key custody, and off-host residual lists.

Recommended outputs:

- local-ops evidence classification;
- qwork proof-file consistency note;
- response archive/proof-root evidence note;
- backup/restore/key/off-host residual list;
- operator action boundary;
- future lane proposal when privileged work or secret handling is needed.

Escalation triggers:

- proof files conflict with live repo or queue state;
- backup, restore, qsl-backup, status/plan, rollback subtree, systemd, timer,
  fstab, or `/backup/qsl` mutation appears outside exact scope;
- operator secrets or privileged actions are required;
- same-host evidence is being inflated into off-host, restore, key custody, or
  disaster recovery proof.

Forbidden authority:

- no backup authorization;
- no restore authorization;
- no qsl-backup mutation authorization;
- no status/plan mutation authorization outside exact scope;
- no off-host backup completion claim;
- no disaster recovery claim;
- no backup-complete claim.

Example future lanes:

- off-host target no-secret planning;
- real restore authorization plan;
- key custody and key recovery prerequisite mapping;
- durable Director State Index storage after backup impact is resolved.

### 7.3 Public Claims / External Review Steward

Purpose:

Review website, public docs, public paper timing, external review package
readiness, no unsupported metadata/privacy/anonymity/untraceability claims,
disclosure and security policy boundaries, and public wording caveats.

Review inputs:

- Project Goal canon;
- metadata/privacy plans and evidence;
- external review and public claim plans;
- public-surface inventory;
- standards mapping evidence;
- service boundary evidence;
- disclosure/security policy evidence;
- current public wording when exact scope authorizes public-surface review.

Recommended outputs:

- public-claim boundary risk list;
- public wording no-go list;
- prerequisite map before website, public docs, or public paper work;
- external/source review recommendation;
- disclosure/security policy gap note.

Escalation triggers:

- proposed public-facing wording expands claims;
- website, README, START_HERE, public docs, or public paper work appears
  outside exact scope;
- public wording implies external review completion without evidence;
- no public wording may imply production or public-internet readiness without
  evidence;
- no public wording may imply metadata-free behavior, anonymity, or
  untraceability without evidence.

Forbidden authority:

- no public claim creation;
- no website, public docs, or public paper mutation authorization;
- no external-review-complete statement;
- no public-readiness claim;
- no metadata-free, anonymity, or untraceability claim.

Example future lanes:

- public paper prerequisite evidence map;
- website/source verification readiness;
- external review package readiness blocker resolution;
- disclosure policy and security.txt readiness plan.

### 7.4 CI / Dependency / Release Health Steward

Purpose:

Review public-safety, cargo audit, RustSec/GHSA/dependency health, branch
protection, required checks, release gates, stale/flaky check handling, and
dependency remediation urgency.

Review inputs:

- required check status;
- public-safety status;
- branch protection evidence;
- cargo audit and cargo tree output;
- RustSec/GHSA/dependency findings;
- workflow run summaries;
- release gate evidence;
- dependency decisions.

Recommended outputs:

- dependency/advisory classification;
- public-safety and required-check health summary;
- release-gate risk note;
- remediation lane recommendation;
- stale/flaky check diagnosis.

Escalation triggers:

- public-safety is red or missing;
- cargo audit is red;
- required checks or branch protection evidence conflict with expectations;
- dependency advisory affects runtime or security-critical code;
- proposed change weakens CI, required checks, branch protection, or fail-closed
  dependency handling.

Forbidden authority:

- no branch-protection change;
- no dependency mutation outside exact scope;
- no audit waiver;
- no required-check weakening;
- no vulnerability-free claim.

Example future lanes:

- RustSec/GHSA blocker remediation;
- required-check context repair;
- dependency provider review;
- release-gate evidence hardening.

### 7.5 Product / Demo / Service Boundary Steward

Purpose:

Review qshield demo boundaries, qsl-server and qsl-attachments production
boundaries, service-local versus public-internet evidence, demo/refimpl/test-only
boundaries, and user-facing readiness caveats.

Review inputs:

- qshield demo evidence;
- qsl-server and qsl-attachments evidence references;
- service-local validation;
- demo/refimpl/test-only boundaries;
- public claim plans;
- runtime/demo validation when exact scope authorizes it.

Recommended outputs:

- service-boundary classification;
- demo/readiness caveat list;
- service-local versus public-internet evidence note;
- future production prerequisite lane proposal;
- public-claim impact note.

Escalation triggers:

- production or public-internet wording is proposed without evidence;
- qsl-server or qsl-attachments sibling repo mutation appears without exact
  scope;
- qshield demo evidence is used as deployment proof;
- runtime/service behavior changes are requested under governance-only scope;
- demo, refimpl, or test-only evidence is being treated as production proof.

Forbidden authority:

- no service production authorization;
- no sibling repo mutation authorization;
- no demo-to-production claim;
- no public readiness claim;
- no qsl-server or qsl-attachments mutation authorization outside exact scope.

Example future lanes:

- qsl-server production boundary prerequisite map;
- qsl-attachments service readiness evidence map;
- qshield demo public-claim boundary review;
- service-local to public-internet gap analysis.

## 8. Steward Review Workflow

1. The Lead Director assigns or requests steward review.
2. The steward reviews only the requested question and allowed scope.
3. The steward produces a bounded recommendation using the template in this
   canon.
4. The Lead Director decides whether to incorporate the recommendation.
5. If accepted, the Lead Director issues the single final directive.
6. Codex executes only the final Lead Director directive.
7. Evidence is recorded in DECISIONS, TRACEABILITY, evidence docs, testplans, or
   the rolling journal as appropriate.

Recommendations must be evidence-backed, scoped, and explicit about whether
they are current-lane work or future-gated work.

## 9. Steward Review Template

Use this template for internal steward review:

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
Backup/restore/key impact:
Dependency/CI impact:
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

Risk classifications:

- `BLOCKER`: current work cannot proceed truthfully or safely until resolved.
- `HIGH`: material security, safety, governance, or evidence risk requiring
  Lead Director attention.
- `MEDIUM`: meaningful risk or evidence gap that should be planned or bounded.
- `LOW`: minor risk or cleanup with limited safety impact.
- `INFO`: relevant observation with no immediate action required.
- `CLAIM_BOUNDARY`: public or internal wording may imply unsupported assurance.
- `EVIDENCE_INCOMPLETE`: evidence is missing, stale, contradictory, or too thin
  for the requested conclusion.

Allowed recommendation outcomes:

- accept current lane;
- add future candidate;
- stop current lane;
- require external/source review;
- no action.

Template rules:

- Use short evidence references.
- State whether the recommendation is current-scope or future-gated.
- Do not include public-facing wording unless public-claim review is explicitly
  requested.
- Do not imply queue changes.
- Do not request secrets or privileged actions directly.

## 10. Conflict and Escalation Rules

Steward vs steward conflict:

- The Lead Director records the chosen path and rationale when material.
- If the conflict affects safety or scope and cannot be resolved, stop.

Steward vs Lead Director conflict:

- Lead Director final authority controls.
- If live evidence contradicts the Director path, stop and repair the evidence
  conflict before continuing.

Steward recommendation vs `NEXT_ACTIONS.md`:

- Live queue wins over steward preference.
- A future candidate may be proposed only if the one-READY invariant remains
  preserved.
- Stewards may not independently promote READY items.

Steward recommendation vs public-safety or CI:

- public-safety and required CI control merge readiness.
- Red or missing required checks require diagnostic evidence and cannot be
  bypassed.
- A steward cannot waive cargo audit, public-safety, branch protection, or
  required checks.

Steward recommendation vs public-claim boundary:

- Public-claim review is mandatory.
- Unsupported public claims are rejected or future-gated.

Steward recommendation requiring out-of-scope changes:

- Treat it as a future candidate, or stop if it blocks truthful continuation.
- Do not implement out-of-scope work inside the current lane.

Steward recommendation requiring external source lookup:

- Use current primary sources when exact scope authorizes source review.
- If source review is outside current scope, future-gate it.
- External source discovery is not external review.

Steward recommendation requiring operator secrets or privileged actions:

- Stop unless a future exact Lead Director directive authorizes safe handling.
- Stewards cannot request secret material directly.
- Stewards cannot authorize sudo, backup, restore, qsl-backup mutation, or
  privileged host changes.

General escalation rules:

- Stop if safety or scope conflict cannot be resolved.
- The Lead Director records final decision and rationale when material.
- New queue candidates are allowed only when one-READY discipline is preserved.
- No independent READY promotion by stewards is allowed.
- No background work promises are allowed.

## 11. Public Claim and External Review Boundaries

This canon is internal governance only.

It is not public docs, not website work, not public technical paper content, and
not external review.

It is not production readiness, not public-internet readiness, not metadata-free
proof, not anonymity proof, and not untraceability proof.

It is not backup proof, not restore proof, not off-host proof, not disaster
recovery proof, not bug-free proof, not vulnerability-free proof, and not perfect-crypto proof.

Website, public docs, public paper, external review package, disclosure policy,
security policy, no unsupported metadata/privacy claims, no unsupported
anonymity claims, no unsupported untraceability claims, and no unsupported
readiness claims require future exact scope and evidence.

Internal evidence, source discovery, service-local evidence, demo evidence, and
green CI must not be conflated with public assurance claims.

## 12. Scope Control and Fail-Closed Rules

Every directive must define allowed and forbidden paths. Codex must stop if a
change would touch out-of-scope paths or a different repo.

Governance-only lanes do not authorize runtime, crypto, dependency, workflow,
qsl-server, qsl-attachments, qshield runtime, website, public docs, README,
START_HERE, qwork, qstart, qresume, qshell, qsl-backup, backup status, backup
plan, rollback subtree, `/backup/qsl`, systemd, timer, fstab, or branch
protection mutation unless a future exact directive says so.

If root cause is unclear enough that continuing would risk untruthful evidence
or behavior drift, stop.

If a proposed fallback would weaken fail-closed behavior, stop.

## 13. Evidence, DECISIONS, and TRACEABILITY Requirements

Material stewardship governance changes require:

- DECISIONS entry;
- TRACEABILITY update;
- evidence doc or testplan when directed;
- scope guard;
- link-check;
- leak-scan;
- public-safety evidence before merge and after merge when required;
- conservative public-claim wording.

Protocol, wire, crypto, auth, state-machine, or security-semantics changes
require future exact scope and must include tests/vectors and the required
governance updates in the same PR.

Steward review output should cite evidence directly and should separate:

- findings;
- evidence gaps;
- public-claim boundaries;
- current-lane recommendations;
- future candidate recommendations.

## 14. Update Policy

Future updates to this canon require a governance PR, DECISIONS entry,
TRACEABILITY update, testplan or evidence as directed, scope guard, link-check,
leak-scan, and public-safety evidence as required by the active directive.

Do not silently edit this canon.

Do not add secret material.

Do not use this canon to bypass live queue or CI authority.

## 15. Explicit Non-Claims

This canon makes no production-readiness claim.

This canon makes no public-internet-readiness claim.

This canon makes no external-review-complete claim.

This canon makes no metadata-free claim.

This canon makes no anonymity claim.

This canon makes no untraceability claim.

This canon makes no off-host-backup-complete claim.

This canon makes no disaster-recovery-complete claim.

This canon makes no restore-proven claim.

This canon makes no backup-complete claim.

This canon makes no bug-free claim.

This canon makes no vulnerability-free claim.

This canon makes no perfect-crypto claim.

## 16. Markers

NA0424_DOMAIN_STEWARDSHIP_CANON_OK

NA0424_LEAD_DIRECTOR_FINAL_AUTHORITY_OK

NA0424_STEWARDS_ADVISORY_ONLY_OK

NA0424_ONE_READY_INVARIANT_OK

NA0424_NO_INDEPENDENT_DIRECTORS_OK

NA0424_PUBLIC_CLAIM_BOUNDARY_OK

NA0424_SCOPE_GUARD_OK

NA0424_CONFLICT_ESCALATION_MODEL_OK

NA0424_STEWARD_REVIEW_TEMPLATE_OK

NA0424_NO_RUNTIME_CHANGE_OK

NA0424_NO_DEPENDENCY_CHANGE_OK

NA0424_NO_WORKFLOW_CHANGE_OK

NA0424_NO_BACKUP_MUTATION_OK

NA0424_NO_PUBLIC_READINESS_CLAIM_OK

NA0424_NO_SECRET_MATERIAL_OK
