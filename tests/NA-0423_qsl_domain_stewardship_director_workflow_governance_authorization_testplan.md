Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-05

# NA-0423 Domain Stewardship Director Workflow Governance Authorization Testplan

## Purpose

Verify that NA-0423 authorizes an internal advisory domain stewardship model
without changing runtime, crypto, dependency, workflow, public, service, or
backup behavior, and without weakening the Lead Director final authority or the
one-READY queue invariant.

## Preconditions

- qwork proof files exist and are read without running qwork:
  - `/srv/qbuild/work/NA-0423/.qwork/startup.qsl-protocol.kv`;
  - `/srv/qbuild/work/NA-0423/.qwork/startup.qsl-protocol.json`.
- qwork proof fields report lane NA-0423, repo qsl-protocol, clean worktree,
  READY_COUNT 1, queue top READY NA-0423, requested lane status READY, and
  matching head/origin_main.
- Live `HEAD` and `origin/main` match the qwork proof after fetch.
- PR #1114 is MERGED at `f0e96ae7e0b1`.
- Queue helper reports READY_COUNT 1 and READY NA-0423.
- Decision helper reports latest D-0833 and duplicate count zero.
- D-0832 exists once, D-0833 exists once, and D-0834 is absent at start.
- public-safety is green on current `origin/main`.
- `cargo audit --deny warnings` is green.
- qsl-backup SHA/source-count boundary matches the directive.

## Scope expectations

Allowed changed paths for the NA-0423 evidence PR:

- `docs/governance/evidence/NA-0423_qsl_domain_stewardship_director_workflow_governance_authorization_plan.md`;
- `tests/NA-0423_qsl_domain_stewardship_director_workflow_governance_authorization_testplan.md`;
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

## Required content assertions

Evidence doc must include:

- executive summary;
- live NA-0423 scope;
- qwork proof-file verification;
- project goal canon inheritance;
- backup/log-code chain completion inheritance;
- stewardship need and problem statement;
- authority model;
- candidate stewardship domains;
- steward workflow model;
- steward review template;
- conflict/escalation model;
- implementation options;
- selected successor;
- future path/scope bundle;
- future validation/marker plan;
- public claim/external review/website boundary;
- rejected alternatives;
- backup-impact statement;
- next recommendation.

Decision D-0834 must state:

- Lead Director remains final authority;
- stewards are advisory only;
- exactly one READY remains mandatory;
- recommended stewardship domains;
- recommended future canon artifact;
- selected NA-0424 successor;
- no independent autonomous Directors;
- no runtime/dependency/workflow/backup/public mutation;
- no public overclaim.

TRACEABILITY must map NA-0423 to G1 through G5 and cite the new evidence doc,
testplan, D-0834, and selected NA-0424 successor.

Rolling journal must record:

- directive identity;
- qwork proof verification;
- repo SHAs;
- READY proof;
- qsl-backup boundary proof;
- recovered command-shape issue from the NA-0401 context review;
- validation/CI notes.

## Stewardship model assertions

The model must preserve:

- Lead Director final authority for directives, READY promotion, queue order,
  PR merge recommendation, public-claim boundary, conflict resolution, and
  stop/retry decisions;
- domain stewards as advisory reviewers only;
- exactly one READY item;
- live queue authority over stale or conflicting steward recommendations;
- DECISIONS, TRACEABILITY, testplans, evidence docs, public-safety, branch
  protection, and scope guards as controlling;
- public-claim conservatism;
- fail-closed stop behavior.

The model must forbid stewards from:

- independently promoting READY items;
- issuing final Codex directives;
- merging PRs;
- creating public claims;
- overriding governance spine files or public-safety;
- bypassing scope guards;
- authorizing forbidden-scope work;
- requesting secret material or privileged operations directly.

## Candidate-domain assertions

The evidence must cover these candidate domains:

- Crypto / Protocol Steward;
- Local Ops / Backup / Restore Steward;
- Public Claims / External Review Steward;
- CI / Dependency / Release Health Steward;
- Product / Demo / Service Boundary Steward.

For each domain, the evidence must state:

- purpose;
- protected invariants;
- review inputs;
- recommended outputs;
- stop/escalation triggers;
- forbidden authority;
- sample future lanes.

The evidence must also evaluate rejected/merged alternatives:

- Security/Threat Steward as separate role;
- Backup Steward separate from Local Ops;
- Documentation Steward separate from Public Claims;
- Release Steward separate from CI/Dependency;
- Human Operator Steward separate from Local Ops.

## Workflow and conflict assertions

Required workflow stages:

1. Lead Director assigns or requests steward review.
2. Steward reviews only allowed scope.
3. Steward produces a bounded recommendation.
4. Lead Director decides whether to incorporate the recommendation.
5. Lead Director issues the single final directive if accepted.
6. Codex executes only the final Lead Director directive.
7. Evidence is recorded in governance artifacts as appropriate.

Required conflict cases:

- steward vs steward conflict;
- steward vs Lead Director conflict;
- steward recommendation vs NEXT_ACTIONS;
- steward recommendation vs public-safety/CI;
- steward recommendation vs public-claim boundary;
- steward recommendation requiring out-of-scope changes;
- steward recommendation requiring external source lookup;
- steward recommendation requiring operator secrets or privileged actions.

Required conflict resolution:

- stop if safety/scope conflict cannot be resolved;
- Lead Director records decision and rationale;
- new queue candidate only if one-READY invariant is preserved;
- no independent READY promotion by stewards;
- no background work promises.

## Public-claim boundary assertions

NA-0423 must state that stewardship governance is internal project governance
only and is not:

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

No README, START_HERE, docs/public, website, public paper, qsl-server,
qsl-attachments, qshield runtime, workflow, dependency, runtime, crypto, backup
script, backup status, or backup plan path may change.

## Validation commands

Run:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --head HEAD \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/governance/evidence/NA-0423_qsl_domain_stewardship_director_workflow_governance_authorization_plan.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md \
  --allowed tests/NA-0423_qsl_domain_stewardship_director_workflow_governance_authorization_testplan.md \
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

Additional local checks:

- PR body preflight with the required metadata fields.
- Goal-lint using a synthetic PR event whose body includes `Goals: G1, G2, G3,
  G4, G5`.
- Changed-line overclaim scan that rejects positive unsupported readiness,
  review, privacy, backup/restore, software-quality, dependency-health, and
  crypto-assurance statements while allowing explicit negative/no-go language.
- Classifier proof that the changed path set is docs/governance only, not
  runtime-critical, not workflow security, and not qsc-adversarial trigger.

## Required pre-PR state

- READY_COUNT 1.
- READY NA-0423.
- D-0834 exists once.
- D-0835 absent.
- Duplicate decision count 0.
- Only the five allowed qsl-protocol paths changed.
- cargo audit green.
- rustls-webpki is v0.103.13 or newer safe version.
- pqcrypto unmaintained RustSec blocker is not active.
- No runtime/dependency/workflow mutation.
- No backup/restore by Codex.
- No qsl-backup mutation.
- No status/plan mutation.
- No qwork/qstart/qresume/qshell mutation.
- No public-readiness or public-security overclaim.

## Post-fix hardening review checklist

Before declaring complete, report:

- Correctness under stress: steward model resolves domain disagreements and
  stale/conflicting evidence fail-closed.
- Minimality: changed paths are limited to allowed governance/testplan files.
- Maintainability: future NA-0424 canon path and template are reusable without
  overriding live queue authority.
- Coverage quality: validation proves queue, decisions, scope, link, leak,
  overclaim, dependency, qsc, and formal/model health.
- Cross-lane stability: macOS/Linux public-safety remains green through required
  checks and no platform-specific runtime path changes are introduced.

## PR requirements

Branch:

`na-0423-domain-stewardship-governance-authorization`

Commit:

`NA-0423 authorize domain stewardship governance`

PR title:

`NA-0423: authorize domain stewardship governance`

PR body must include:

```md
Goals: G1, G2, G3, G4, G5
Impact: Authorizes advisory domain stewardship while preserving Lead Director final authority, one-READY queue discipline, scope guards, and public-claim conservatism.
No-regression: No runtime, crypto, dependency, workflow, backup, public-doc, website, qsl-server, qsl-attachments, qshield runtime, or qwork/qstart/qresume/qshell mutation; no independent Directors; exactly one READY remains.
Tests/Vectors: Added NA-0423 governance authorization testplan; ran scope, link, leak, overclaim, classifier, PR-body, goal-lint, dependency, qsc, and formal/model checks.
```

Merge only after required checks pass. Use merge commit; no squash/rebase,
force-push, amend-after-PR, or branch deletion flags.
