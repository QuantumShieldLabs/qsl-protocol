Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-14
Replaces:
Superseded-By:

# NA-0290A Public Attention Visibility Strategy Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0290A creates a public attention and visibility strategy,
public-surface audit, and companion governance records while preserving
evidence-bound claim discipline and avoiding implementation or public-copy
changes.

## Protected invariants

- Exactly one READY queue item remains active during the strategy packet:
  NA-0290A.
- D-0550 remains the insertion decision and D-0551 records the strategy audit.
- D-0552 is absent until closeout.
- Public-safety remains required and green.
- The strategy is attention-grabbing but does not exceed evidence.
- Limitations remain visible.
- Future public-copy implementation requires a separate NA/directive.

## Allowed scope

Allowed paths for the strategy packet:

- `docs/public/PUBLIC_ATTENTION_AND_VISIBILITY_STRATEGY.md`
- `docs/governance/evidence/NA-0290A_public_attention_visibility_audit.md`
- `tests/NA-0290A_public_attention_visibility_strategy_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

Forbidden changes:

- `README.md`
- `START_HERE.md`
- website or external website files
- `.github/**`
- `scripts/**`
- `Cargo.toml`
- `Cargo.lock`
- qsl-protocol runtime, protocol, crypto, demo, or service code
- qsl-server implementation files
- qsl-attachments implementation files
- qsc-desktop files
- `tools/**`
- `inputs/**`
- `formal/**`
- branch-protection settings
- public-safety configuration
- dependency changes
- branch deletion

## Web audit requirements

- Inspect only authorized public official surfaces.
- Record every public source inspected in the audit.
- Paraphrase web-derived content.
- Do not post, submit forms, authenticate, scrape private material, or mutate
  any public surface.
- If a live official website is unavailable or body content cannot be
  inspected, state that plainly.

## Repo audit requirements

- Inspect qsl-protocol README, START_HERE, public docs, demo docs,
  governance/evidence docs, design docs, SUPPORT, CONTRIBUTING, NEXT_ACTIONS,
  DECISIONS, and TRACEABILITY.
- Inspect qsl-server public README and docs read-only.
- Inspect qsl-attachments public README and docs read-only.
- Record strengths, attention gaps, safe opportunities, unsafe implications,
  and what would make the project easier to discuss.

## Claim-boundary requirements

High-risk wording may appear only as prohibited wording, explicit negation, or
future/unproven boundary text. The packet must not introduce affirmative claims
for:

- production-ready
- deployment-ready
- production relay ready
- production attachment ready
- public internet ready
- external review complete
- externally reviewed
- metadata-free
- anonymity
- anonymous messaging
- untraceable
- quantum-proof
- unbreakable
- military-grade
- guaranteed secure
- proven true Triple Ratchet
- ready for production
- release ready

The strategy must include:

- approved language
- prohibited language
- safe substitutions
- examples of what not to say
- examples of stronger safe public copy
- evidence gates before implementation

## Link/leak/goal-lint expectations

Required checks:

- `git diff --check`
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main`
  with the exact allowed paths above
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- direct overclaim phrase scan over added lines
- PR body preflight
- goal-lint

Expected result:

- `READY_COUNT 1`
- READY item is NA-0290A
- D-0550 exists once
- D-0551 exists once
- D-0552 is absent
- duplicate decisions count is zero
- missing local-link count is zero
- secret finding count is zero
- scope guard reports no forbidden paths

## CI expectations

Required validation before merge:

- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/run_model_checks.py` when present
- required PR checks complete successfully, with CodeQL neutral accepted only
  through the existing helper policy if applicable
- post-merge main public-safety completes successfully

Docs-only cost-control expectation:

- The changed paths must classify as docs/governance-only under the current
  CI scope classifier.

## Future implementation gate

This testplan does not authorize public-copy implementation. A future
implementation lane must:

- declare exact target surfaces and allowed paths;
- re-run claim-boundary scans;
- link each public claim to current evidence;
- keep metadata phase-2, external review, service-hardening, and deployment
  gates visible;
- re-audit live website content before website changes;
- preserve public-safety and branch-protection invariants.
