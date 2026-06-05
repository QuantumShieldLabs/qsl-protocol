Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-05

# NA-0424 QSL Domain Stewardship Operating Model Canon Implementation Harness

## Executive summary

NA-0424 implements the internal governance canon authorized by NA-0423 for QSL
advisory domain stewardship and Director workflow:

`docs/governance/DOMAIN_STEWARDSHIP_AND_DIRECTOR_WORKFLOW.md`

The canon preserves Lead Director final authority, advisory-only steward roles,
exactly-one-READY queue discipline, fail-closed scope control, public-safety
authority, evidence discipline, and conservative public-claim boundaries.

NA-0424 is internal governance only. It does not mutate runtime, crypto,
dependency, workflow, backup, qsl-server, qsl-attachments, qshield runtime,
website, public docs, README, START_HERE, qwork/qstart/qresume/qshell,
qsl-backup, status/plan, rollback, `/backup/qsl`, branch-protection, or public
paper paths.

Selected normal successor:

`NA-0425 -- QSL Code / Crypto Audit Follow-Up Resumption Plan`

## Live NA-0424 scope

Live `NEXT_ACTIONS.md` lists:

`NA-0424 -- QSL Domain Stewardship Operating Model Canon Implementation Harness`

Status: READY.

Goals: G1, G2, G3, G4, G5.

Allowed mutation paths:

- `docs/governance/DOMAIN_STEWARDSHIP_AND_DIRECTOR_WORKFLOW.md`;
- `docs/governance/evidence/NA-0424_qsl_domain_stewardship_operating_model_canon_implementation_harness.md`;
- `tests/NA-0424_qsl_domain_stewardship_operating_model_canon_implementation_testplan.md`;
- `DECISIONS.md`;
- `TRACEABILITY.md`;
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

Forbidden mutation scope:

- runtime, crypto, dependency, workflow, qsl-server, qsl-attachments, qshield
  runtime, website, public docs, README, START_HERE;
- qwork, qstart, qresume, qshell;
- qsl-backup, backup status, backup plan, rollback subtree, `/backup/qsl`,
  systemd, timer, fstab;
- branch protection;
- public technical paper content;
- secret material;
- no public readiness, no production readiness, no public-internet readiness;
- no external-review completion, no metadata-free behavior, no anonymity;
- no untraceability, no off-host backup completion, no disaster recovery
  completion;
- no restore proof, no backup completion, no bug-free status, no
  vulnerability-free status, and no perfect-crypto claims.

Acceptance criteria:

- Lead Director remains final authority for directives and READY promotion.
- Stewards are advisory only.
- Exactly one READY item remains.
- Steward domains and review template are explicit.
- Conflict/escalation rules are explicit.
- No runtime/dependency/workflow/public/backup mutation occurs.
- public-safety is green before merge and after merge.

Stop conditions:

- qwork proof files missing, stale, malformed, or inconsistent;
- qwork, qstart, or qresume is run by Codex;
- PR #1116 not merged;
- origin/main not equal to or descended from PR #1116 merge commit;
- queue not READY NA-0424 at start;
- D-0835 absent or D-0836 already present at start;
- cargo audit not green;
- qsl-backup source-list regression;
- any out-of-scope mutation;
- any model that creates independent Directors, allows more than one READY item,
  or weakens public-claim boundaries.

## qwork proof-file verification

Codex read, but did not run, qwork proof files:

- `/srv/qbuild/work/NA-0424/.qwork/startup.qsl-protocol.kv`;
- `/srv/qbuild/work/NA-0424/.qwork/startup.qsl-protocol.json`.

The `.kv` proof reported:

- `startup_result=OK`;
- `lane=NA-0424`;
- `repo=qsl-protocol`;
- `path=/srv/qbuild/work/NA-0424/qsl-protocol`;
- `head=f6021ab900b3b2f51b1301eb4723ff714e332951`;
- `origin_main=f6021ab900b3b2f51b1301eb4723ff714e332951`;
- `head_equals_origin_main=yes`;
- `worktree_clean=yes`;
- `index_clean=yes`;
- `untracked_clean=yes`;
- `ready_count=1`;
- `queue_top_ready=NA-0424`;
- `requested_lane_status=READY`.

The JSON proof parsed successfully and mirrored the required `.kv` fields for
lane, repo, path, head, origin/main, clean state, READY count, queue-top READY,
and requested-lane status.

After `git fetch --all --prune`, live `HEAD` and `origin/main` remained at
`f6021ab900b3`, matching the qwork proof. PR #1116 was verified MERGED with
merge commit `f6021ab900b3`.

Proof root:

`/srv/qbuild/tmp/NA0424_domain_stewardship_canon_20260605T125616-0500`

The qwork proof files were copied into the proof root under `qwork/`.

Host timestamp note: local host `date --iso-8601=seconds` reported
`2026-06-05T12:55:46-05:00` and UTC `2026-06-05T17:55:46+00:00` during startup
capture, which is earlier than the embedded directive begin time. This is
recorded as operational friction and is not used as authority over qwork proof
or live repo evidence.

## NA-0423 inheritance

NA-0423 authorized the advisory domain stewardship model in D-0834 and selected
NA-0424 as the normal canon implementation successor.

Inherited authority model:

- Lead Director remains final authority for directive issuance, READY
  promotion, queue order, PR merge recommendation, public-claim boundary,
  conflict resolution, and stop/retry decisions.
- Domain stewards are advisory reviewers only.
- Exactly one READY item remains mandatory.
- Live `NEXT_ACTIONS.md`, `DECISIONS.md`, `TRACEABILITY.md`, public-safety,
  branch protection, evidence docs, testplans, and scope guards remain
  controlling.
- Steward recommendations can propose evidence, risks, blockers, claim-boundary
  implications, candidate lanes, and suggested governance text, but cannot
  independently create queue state.

Inherited project-goal canon:

- security before speed;
- evidence over vibes;
- no public overclaiming;
- one-READY queue discipline;
- no runtime, crypto, dependency, or workflow drift without exact scope;
- Director/Codex/human role boundaries;
- no secret material.

Inherited backup/log-code chain:

- backup/log-code cleanup chain is complete through NA-0422;
- local backup status and plan files were refreshed by NA-0422;
- same-host continuity caveat remains required;
- off-host backup, real restore, key custody, and disaster recovery remain
  future residuals.

## Canon artifact summary

The canon artifact is:

`docs/governance/DOMAIN_STEWARDSHIP_AND_DIRECTOR_WORKFLOW.md`

It includes the required top-level sections:

1. Status and Authority
2. Purpose
3. Non-Override Rule
4. Lead Director Final Authority
5. Steward Advisory Boundary
6. One-READY Queue Discipline
7. Domain Stewardship Roles
8. Steward Review Workflow
9. Steward Review Template
10. Conflict and Escalation Rules
11. Public Claim and External Review Boundaries
12. Scope Control and Fail-Closed Rules
13. Evidence, DECISIONS, and TRACEABILITY Requirements
14. Update Policy
15. Explicit Non-Claims
16. Markers

The canon explicitly states that it does not override `NEXT_ACTIONS.md`,
`DECISIONS.md`, `TRACEABILITY.md`, branch protection, public-safety, or the
Project Goal canon.

The canon explicitly states that it does not authorize independent Directors,
more than one READY item, public claims, or runtime/crypto/dependency/workflow/
backup changes.

## Stewardship domains implemented

The canon implements five advisory domains:

1. Crypto / Protocol Steward.
2. Local Ops / Backup / Restore Steward.
3. Public Claims / External Review Steward.
4. CI / Dependency / Release Health Steward.
5. Product / Demo / Service Boundary Steward.

Each domain includes:

- purpose;
- review inputs;
- recommended outputs;
- escalation triggers;
- forbidden authority;
- example future lanes.

The Crypto / Protocol Steward covers qsc/qsp/qsl surfaces, KEM/signature/
provider choices, formal model alignment, code/crypto audit findings,
side-channel and misuse-boundary caveats, and the no crypto-complete public
claim boundary.

The Local Ops / Backup / Restore Steward covers qwork proof handoff, response
archives, backup status/plan, operator packets, restore/key custody/off-host
gaps, same-host caveats, and no disaster-recovery or backup-complete claim.

The Public Claims / External Review Steward covers website/public docs/public
paper timing, external review package readiness, no unsupported metadata/privacy/anonymity/untraceability claim boundaries,
disclosure/security policy boundaries, and no public-readiness overclaim.

The CI / Dependency / Release Health Steward covers public-safety, cargo audit,
RustSec/GHSA/dependency health, branch protection, required checks, release
gates, and no vulnerability-free claim.

The Product / Demo / Service Boundary Steward covers qshield demo boundaries,
qsl-server/qsl-attachments production boundaries, service-local versus
public-internet evidence, demo/refimpl/test-only boundaries, and no
production-readiness claim.

## Lead Director authority proof

The canon states that the Lead Director owns final directive issuance, READY
promotion, queue order, PR merge recommendation, public-claim boundary,
conflict resolution, stop/retry decisions, and final acceptance or rejection of
steward recommendations.

The canon also states that steward input is evidence for the Lead Director and
that Codex executes only the final Lead Director directive.

## Advisory-only steward proof

The canon states that domain stewards are advisory reviewers only.

It forbids stewards from independently promoting READY items, issuing final
Codex directives, merging PRs, creating public claims, overriding governance
spine files, bypassing scope guards, approving forbidden-scope work,
authorizing secret handling, authorizing privileged operations, or creating
independent autonomous Directors.

## One-READY invariant proof

The canon states that exactly one `NEXT_ACTIONS.md` item must be READY.

It states that steward recommendations do not create queue state, that live
queue state wins over steward preference, and that a future candidate is valid
only when the Lead Director records it through the governance spine while
preserving the one-READY invariant.

## Review template proof

The canon includes the required steward review template fields:

- steward domain;
- requested review question;
- current NA item;
- current authoritative state checked;
- evidence reviewed;
- findings;
- risk classification;
- goals affected;
- scope impact;
- public-claim impact;
- test/validation impact;
- backup/restore/key impact;
- dependency/CI impact;
- recommended action;
- explicit no-go statements;
- suggested DECISIONS / TRACEABILITY entries;
- steward confidence and caveats.

Risk classifications implemented:

- BLOCKER;
- HIGH;
- MEDIUM;
- LOW;
- INFO;
- CLAIM_BOUNDARY;
- EVIDENCE_INCOMPLETE.

Allowed recommendation outcomes implemented:

- accept current lane;
- add future candidate;
- stop current lane;
- require external/source review;
- no action.

## Conflict/escalation proof

The canon specifies resolution for:

- steward vs steward conflict;
- steward vs Lead Director conflict;
- steward recommendation vs `NEXT_ACTIONS.md`;
- steward recommendation vs public-safety/CI;
- steward recommendation vs public-claim boundary;
- steward recommendation requiring out-of-scope changes;
- steward recommendation requiring external source lookup;
- steward recommendation requiring operator secrets or privileged actions.

Required conflict rules implemented:

- live queue wins over steward preference;
- stop if safety/scope conflict cannot be resolved;
- Lead Director records final decision and rationale when material;
- new queue candidate only if one-READY invariant remains preserved;
- no independent READY promotion by stewards;
- no background work promises.

## Public claim/external review/website boundary

The canon states that it is internal governance only and is not public docs,
website work, public technical paper content, or external review.

The canon states that it is not production readiness, not public-internet
readiness, not metadata-free proof, not anonymity proof, and not untraceability
proof.

The canon states that it is not backup proof, not restore proof, not off-host
proof, not disaster recovery proof, not bug-free proof, not vulnerability-free
proof, and not perfect-crypto proof.

The canon does not update README, START_HERE, public docs, website paths, public
paper content, qsl-server, qsl-attachments, qshield runtime, workflows,
dependencies, runtime code, crypto code, backup scripts, backup status files, or
backup plan files.

## Marker proof

Required markers in `docs/governance/DOMAIN_STEWARDSHIP_AND_DIRECTOR_WORKFLOW.md`:

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

## Rejected alternatives

Keep stewardship only in NA-0423 evidence:

- Rejected because a reusable canon is easier to cite and review in future
  Director/Codex handoffs.

Create independent Directors:

- Rejected because it conflicts with Lead Director final authority and
  one-READY discipline.

Allow multiple READY items:

- Rejected because it conflicts with canonical queue discipline.

Add helper/tool automation now:

- Rejected because the stewardship model should stabilize as canon before
  tooling is introduced.

Start code/crypto audit implementation now:

- Rejected because NA-0424 is internal governance only. The normal successor is
  a bounded code/crypto audit follow-up resumption plan.

Do not start public paper, website, public docs, or public-readiness work now:

- Rejected because public-claim prerequisites remain future-gated and NA-0424
  is internal governance only.

## Backup-impact statement

NA-0424 mutates only tracked qsl-protocol governance files under exact allowed
scope. It does not run backup or restore. It does not mutate
`/usr/local/sbin/qsl-backup`, `/backup/qsl`, backup status files, backup plan
files, rollback subtree paths, keys, passphrases, restore targets, off-host
targets, systemd, timers, fstab, or local backup architecture.

Same-host backup/log-code evidence remains inherited and caveated. NA-0424 does
not prove off-host backup, disaster recovery, restore, backup completion, or key
custody.

## Next recommendation

After NA-0424 merges and post-merge public-safety is green, close out NA-0424
and restore this exact normal successor:

`NA-0425 -- QSL Code / Crypto Audit Follow-Up Resumption Plan`

Rationale:

- backup/log-code chain is complete through NA-0422;
- stewardship canon is implemented;
- the next major technical priority is the deferred code/crypto audit follow-up
  identified by NA-0397 and later forward audits;
- the successor remains planning-only unless a future directive authorizes
  implementation.
