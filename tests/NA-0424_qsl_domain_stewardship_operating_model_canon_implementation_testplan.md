Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-05

# NA-0424 Domain Stewardship Operating Model Canon Implementation Testplan

## Purpose

Verify that NA-0424 implements the internal governance canon for QSL advisory
domain stewardship and Director workflow without changing runtime, crypto,
dependency, workflow, public, service, or backup behavior, and without
weakening Lead Director final authority, advisory-only steward boundaries, the
one-READY invariant, public-safety, or public-claim discipline.

## Preconditions

- qwork proof files exist and are read without running qwork:
  - `/srv/qbuild/work/NA-0424/.qwork/startup.qsl-protocol.kv`;
  - `/srv/qbuild/work/NA-0424/.qwork/startup.qsl-protocol.json`.
- qwork proof fields report lane NA-0424, repo qsl-protocol, clean worktree,
  READY_COUNT 1, queue top READY NA-0424, requested lane status READY, and
  matching head/origin_main.
- Live `HEAD` and `origin/main` match the qwork proof after fetch.
- PR #1116 is MERGED at `f6021ab900b3`.
- Queue helper reports READY_COUNT 1 and READY NA-0424.
- Decision helper reports latest D-0835 and duplicate count zero.
- D-0834 exists once, D-0835 exists once, and D-0836 is absent at start.
- public-safety is green on current `origin/main`.
- `cargo audit --deny warnings` is green.
- `cargo tree -i rustls-webpki --locked` reports `rustls-webpki v0.103.13` or
  newer safe version.
- pqcrypto unmaintained RustSec blocker packages are not active in the locked
  dependency graph.
- qsl-backup SHA/source-count boundary matches the directive.

## Scope expectations

Allowed changed paths for the NA-0424 implementation PR:

- `docs/governance/DOMAIN_STEWARDSHIP_AND_DIRECTOR_WORKFLOW.md`;
- `docs/governance/evidence/NA-0424_qsl_domain_stewardship_operating_model_canon_implementation_harness.md`;
- `tests/NA-0424_qsl_domain_stewardship_operating_model_canon_implementation_testplan.md`;
- `DECISIONS.md`;
- `TRACEABILITY.md`;
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

No other qsl-protocol paths may change.

Forbidden mutation classes:

- runtime code;
- protocol or crypto semantics;
- dependencies or lockfiles;
- workflows or branch protection;
- qsl-server, qsl-attachments, qshield runtime, website, public docs, README, or
  START_HERE;
- qwork, qstart, qresume, or qshell;
- qsl-backup;
- backup status or plan files;
- `/backup/qsl`;
- rollback subtree paths;
- public paper content;
- secret material.

## Required canon assertions

The canon must include these top-level sections:

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

The canon must explicitly state:

- it does not override `NEXT_ACTIONS.md`;
- it does not override `DECISIONS.md`;
- it does not override `TRACEABILITY.md`;
- it does not override branch protection;
- it does not override public-safety;
- it does not override the Project Goal canon;
- it does not authorize independent Directors;
- it does not authorize more than one READY item;
- it does not authorize public claims;
- it does not authorize runtime/crypto/dependency/workflow/backup changes.

## Required stewardship-domain assertions

The canon must include:

- Crypto / Protocol Steward;
- Local Ops / Backup / Restore Steward;
- Public Claims / External Review Steward;
- CI / Dependency / Release Health Steward;
- Product / Demo / Service Boundary Steward.

Each domain must include:

- purpose;
- review inputs;
- recommended outputs;
- escalation triggers;
- forbidden authority;
- example future lanes.

Crypto / Protocol Steward must cover qsc/qsp/qsl surfaces, KEM/signature/
provider choices, formal model alignment, code/crypto audit findings,
side-channel/misuse-boundary caveats, and no crypto-complete public claim.

Local Ops / Backup / Restore Steward must cover qwork and proof-file handoff,
response archives, backup status/plan, operator packets, restore/key
custody/off-host gaps, same-host caveats, and no disaster-recovery or
no backup-complete claim.

Public Claims / External Review Steward must cover website/public docs/public
paper timing, external review package readiness, no unsupported metadata/privacy/anonymity/untraceability claim boundaries,
disclosure/security policy boundaries, and no public-readiness overclaim.

CI / Dependency / Release Health Steward must cover public-safety, cargo audit,
RustSec/GHSA/dependency health, branch protection, required checks, release
gates, and no vulnerability-free claim.

Product / Demo / Service Boundary Steward must cover qshield demo boundaries,
qsl-server/qsl-attachments production boundary, service-local versus
public-internet evidence, demo/refimpl/test-only boundaries, and no
production-readiness claim.

## Required review-template assertions

The canon must include the required review template fields:

- Steward domain.
- Requested review question.
- Current NA item.
- Current authoritative state checked.
- Evidence reviewed.
- Findings.
- Risk classification.
- Goals affected.
- Scope impact.
- Public-claim impact.
- Test/validation impact.
- Backup/restore/key impact.
- Dependency/CI impact.
- Recommended action.
- Explicit no-go statements.
- Suggested DECISIONS / TRACEABILITY entries.
- Steward confidence and caveats.

The canon must include risk classifications:

- BLOCKER;
- HIGH;
- MEDIUM;
- LOW;
- INFO;
- CLAIM_BOUNDARY;
- EVIDENCE_INCOMPLETE.

The canon must include allowed recommendation outcomes:

- accept current lane;
- add future candidate;
- stop current lane;
- require external/source review;
- no action.

## Required conflict/escalation assertions

The canon must specify how to resolve:

- steward vs steward conflict;
- steward vs Lead Director conflict;
- steward recommendation vs `NEXT_ACTIONS.md`;
- steward recommendation vs public-safety/CI;
- steward recommendation vs public-claim boundary;
- steward recommendation requiring out-of-scope changes;
- steward recommendation requiring external source lookup;
- steward recommendation requiring operator secrets or privileged actions.

Required rules:

- live queue wins over steward preference;
- stop if safety/scope conflict cannot be resolved;
- Lead Director records final decision and rationale;
- new queue candidate only if one-READY invariant remains preserved;
- no independent READY promotion by stewards;
- no background work promises.

## Required marker assertions

The canon must include each exact marker:

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

## Required evidence assertions

Evidence doc must include:

- executive summary;
- live NA-0424 scope;
- qwork proof-file verification;
- NA-0423 inheritance;
- canon artifact summary;
- stewardship domains implemented;
- Lead Director authority proof;
- advisory-only steward proof;
- one-READY invariant proof;
- review template proof;
- conflict/escalation proof;
- public claim/external review/website boundary;
- marker proof;
- rejected alternatives;
- backup-impact statement;
- next recommendation.

Decision D-0836 must state:

- canon artifact path;
- Lead Director final authority preserved;
- stewards advisory only;
- exactly one READY remains mandatory;
- no independent autonomous Directors;
- five stewardship domains implemented;
- selected NA-0425 successor;
- no runtime/dependency/workflow/backup/public mutation;
- no public overclaim.

TRACEABILITY must map NA-0424 to G1 through G5 and cite the canon artifact,
evidence doc, testplan, D-0836, qwork proof, public-safety, cargo audit, qsc
send_commit, and formal/model validation.

Rolling journal must record directive identity, timestamps, repo SHAs, qwork
proof verification, READY proof, qsl-backup boundary proof, validation/CI notes,
disk watermark, recovered failures if any, and next-watch items.

## Public-claim boundary assertions

NA-0424 must state that stewardship canon implementation is internal project
governance only and is not:

- public docs;
- website work;
- public technical paper work;
- external review;
- not production readiness;
- not public-internet readiness;
- not metadata-free proof;
- not anonymity proof;
- not untraceability proof;
- backup proof;
- not restore proof;
- off-host proof;
- disaster recovery proof;
- not vulnerability-free status;
- not bug-free status;
- not perfect-crypto status.

No README, START_HERE, public docs, website, public paper, qsl-server,
qsl-attachments, qshield runtime, workflow, dependency, runtime, crypto, backup
script, backup status, or backup plan path may change.

## Validation commands

Run:

```bash
for marker in \
  NA0424_DOMAIN_STEWARDSHIP_CANON_OK \
  NA0424_LEAD_DIRECTOR_FINAL_AUTHORITY_OK \
  NA0424_STEWARDS_ADVISORY_ONLY_OK \
  NA0424_ONE_READY_INVARIANT_OK \
  NA0424_NO_INDEPENDENT_DIRECTORS_OK \
  NA0424_PUBLIC_CLAIM_BOUNDARY_OK \
  NA0424_SCOPE_GUARD_OK \
  NA0424_CONFLICT_ESCALATION_MODEL_OK \
  NA0424_STEWARD_REVIEW_TEMPLATE_OK \
  NA0424_NO_RUNTIME_CHANGE_OK \
  NA0424_NO_DEPENDENCY_CHANGE_OK \
  NA0424_NO_WORKFLOW_CHANGE_OK \
  NA0424_NO_BACKUP_MUTATION_OK \
  NA0424_NO_PUBLIC_READINESS_CLAIM_OK \
  NA0424_NO_SECRET_MATERIAL_OK
do
  grep -F "$marker" docs/governance/DOMAIN_STEWARDSHIP_AND_DIRECTOR_WORKFLOW.md
done

git diff --check
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --head HEAD \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/governance/DOMAIN_STEWARDSHIP_AND_DIRECTOR_WORKFLOW.md \
  --allowed docs/governance/evidence/NA-0424_qsl_domain_stewardship_operating_model_canon_implementation_harness.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md \
  --allowed tests/NA-0424_qsl_domain_stewardship_operating_model_canon_implementation_testplan.md \
  --forbidden .github/ \
  --forbidden Cargo.toml \
  --forbidden Cargo.lock \
  --forbidden qsl/ \
  --forbidden qsl-server/ \
  --forbidden qsl-attachments/ \
  --forbidden apps/ \
  --forbidden website/ \
  --forbidden README.md \
  --forbidden START_HERE.md
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
cargo fmt --check
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

Additional checks:

- exact changed-path guard for the six allowed paths;
- added-line overclaim scan;
- classifier proof;
- PR body preflight;
- goal-lint with `Goals: G1, G2, G3, G4, G5`;
- required checks, including public-safety, green before merge;
- post-merge public-safety green after merge.

## Acceptance criteria

- READY_COUNT 1.
- READY NA-0424.
- D-0836 exists once.
- D-0837 absent.
- Duplicate decision count 0.
- Only six allowed qsl-protocol paths changed.
- All NA0424 markers are present.
- Canon artifact exists.
- `cargo audit --deny warnings` is green.
- No runtime/dependency/workflow/public/backup mutation occurs.
- No backup or restore is run by Codex.
- qsl-backup is not mutated.
- Backup status and backup plan files are not mutated.
- qwork/qstart/qresume/qshell is not mutated.
- No public-readiness, no production-readiness, no external-review-complete,
  no vulnerability-free, or no perfect-crypto overclaim is introduced.

## Post-fix hardening review checklist

- Correctness under stress: steward recommendations cannot override live queue,
  CI, branch protection, or Lead Director final authority.
- Minimality: only the six allowed governance paths change.
- Maintainability: the canon has stable sections, explicit roles, reusable
  template fields, and concrete conflict rules.
- Coverage quality: markers, scope guard, link-check, leak-scan, overclaim scan,
  classifier, PR body preflight, goal-lint, dependency checks, qsc send_commit,
  and formal/model checks verify the lane.
- Cross-lane stability: Linux/macOS runtime surfaces are unchanged because this
  is docs/governance-only work; public-safety remains the merge gate.
