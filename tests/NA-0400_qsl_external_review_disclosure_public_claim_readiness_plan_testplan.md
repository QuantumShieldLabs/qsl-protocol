Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-01

# NA-0400 QSL External Review / Disclosure / Public Claim Readiness Plan Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Prove NA-0400 creates a governance-only external review, coordinated
disclosure, public security policy, public claim, and public technical paper
readiness map without implementing any external review, disclosure policy,
public doc, website, runtime, crypto, dependency, workflow, service, backup,
restore, key, or secret-handling change.

## Protected Invariants

- READY_COUNT remains exactly one.
- READY remains NA-0400 until closeout.
- D-0780 and D-0781 exist once.
- D-0782 is added once.
- Source guidance is not treated as external review.
- Public claim readiness planning is not public claim authorization.
- Service-local evidence is not production proof.
- qshield demo evidence is not production proof.
- Same-host continuity is not disaster recovery.
- No public technical paper work begins.

## Allowed Scope

- `docs/governance/evidence/NA-0400_qsl_external_review_disclosure_public_claim_readiness_plan.md`
- `tests/NA-0400_qsl_external_review_disclosure_public_claim_readiness_plan_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

- Runtime/service/protocol/crypto/qsc/qsp/qsl/qshield implementation paths.
- qsl-server or qsl-attachments mutation.
- qsc-desktop mutation.
- `.github/**`, workflows, branch protection, public-safety configuration.
- `Cargo.toml`, `Cargo.lock`, dependency updates.
- README, START_HERE, docs/public, website, external website repo.
- `SECURITY.md`, security.txt, issue templates, public disclosure policy files.
- Backup scripts/timers/fstab/source lists, real backup, real restore,
  off-host target setup, key generation, credential/passphrase/private-key
  handling, or recovery envelopes.
- Response archive/history mutation except the required final D220 response
  file outside the repository.

## NA-0399 Inheritance Requirements

- Evidence must record NA-0399 PR #1061 and closeout PR #1062 as merged.
- Evidence must record READY NA-0400, NA-0399 DONE, D-0780 once, D-0781 once,
  and D-0782 absent before the NA-0400 patch.
- Evidence must carry forward same-host continuity, off-host backup, restore,
  key custody, and key recovery boundaries.

## Official Source Verification Requirements

The evidence must cite official or primary sources for:

- CISA CVD guidance.
- CERT/CC CVD guidance.
- ISO/IEC 29147 and ISO/IEC 30111 official pages or public summaries.
- FIRST PSIRT / vulnerability handling guidance.
- RFC 9116 security.txt.
- GitHub private vulnerability reporting / security policy documentation.
- NIST SSDF / SP 800-218.
- NIST CSF 2.0 if used.
- OWASP ASVS if used.
- OpenSSF Scorecard / SLSA / Best Practices Badge if used.

## Source Citation Requirements

Each source entry must include:

- title;
- authority / publisher;
- URL;
- access date;
- source tier;
- source classification;
- relevance to QSL;
- readiness or claim-boundary implication.

## Cumulative Evidence Intake Requirements

The evidence must consume NA-0392 through NA-0399 read-only evidence and record
what exists, what remains partial, what is absent, and what claims remain
forbidden.

## Read-Only Public-Surface Scan Requirements

The evidence must record bounded read-only public-surface/claim inventory,
including:

- public-claim term scan result;
- `SECURITY.md` status;
- security.txt status;
- docs/public and reviewer package status;
- GitHub private vulnerability reporting status if checked.

## External Review Readiness Requirements

The map must classify reviewer-package status conservatively and must not mark
READY_FOR_EXTERNAL_REVIEW_PACKAGE unless exact evidence supports it.

Expected classification: `PARTIAL_REVIEW_PACKAGE` / `EVIDENCE_INCOMPLETE`.

## Coordinated Vulnerability Disclosure Requirements

The map must cover:

- disclosure policy status;
- security contact status;
- security.txt / SECURITY.md status;
- vulnerability handling workflow;
- GitHub private reporting / advisories;
- triage, embargo, credits, remediation, and publication gaps.

## Public Security Policy Readiness Requirements

The map must cover:

- policy files present/absent;
- website/public docs status;
- public contact status;
- future options;
- claim boundary.

## Public Claim Readiness Requirements

The map must explicitly classify and constrain:

- production readiness;
- public-internet readiness;
- external-review completion;
- PQC/FIPS/RFC/TLS/HPKE/MLS claims;
- metadata-free, anonymity, untraceable, timing, traffic-shape, and
  attachment-size claims;
- off-host backup, disaster recovery, restore, key custody, and key recovery
  claims;
- bug-free, perfect-crypto, and vulnerability-free claims.

## Public Technical Paper Prerequisite Requirements

The evidence must list prerequisites before public technical paper drafting and
must classify paper work as future-gated.

## Service / Demo Boundary Requirements

The evidence must record qsl-server PR #56, qsl-attachments PR #37, qshield
demo, and qshield-cli boundaries as service-local or demo/harness evidence only.

## Secure Development / Assurance Posture Requirements

The evidence must record public-safety, branch protection, cargo audit,
rustls-webpki, formal models, qsc/qshield checks, no-secret harnesses, routine
audits, known gaps, and what cannot be claimed.

## Vulnerability / Advisory Response Posture Requirements

The evidence must carry forward NA-0396 advisory trigger policy and record
missing disclosure policy / public security policy process gaps.

## External-Review Package Requirements

The evidence must define future package contents and prohibited material,
including no secrets, credentials, private keys, passphrases, recovery
envelopes, live target identities, or sensitive host material.

## Public Claim Language Policy Requirements

The evidence must define allowed bounded language and forbidden strong language
unless exact future evidence exists.

## Future Queue Candidate Requirements

The evidence must list future candidate lanes and explain why the selected
successor is next.

## No Implementation Requirements

NA-0400 must not implement NA-0401, create a project-goal canon artifact,
create security.txt, edit SECURITY.md, update public docs, contact reviewers,
or create durable external-review/public-claim package files outside the
authorized governance evidence.

## Successor Selection Requirements

The evidence must select exactly one NA-0401 successor.

Expected normal successor:

`NA-0401 -- QSL Project Goal and Operating Principles Canon Authorization Plan`

## Project Goal Canon Successor Requirements

The future bundle must preserve internal-governance-only scope and must include
future markers for security before speed, evidence over vibes, code/crypto
excellence, no public overclaiming, one-READY queue discipline, routine audit
rhythm, external awareness without hype, public paper timing, role boundaries,
and no runtime/crypto/dependency/workflow/secret mutation.

## Backup-Impact Requirements

The evidence must record:

- changed paths are governance/testplan/traceability/journal only;
- no backup-plan update is required;
- future durable external-review/public-claim/paper artifacts need separate
  backup-impact review;
- same-host continuity is not disaster recovery.

## Required Local Checks

At minimum run or record:

- `git status --porcelain=v1 --branch`
- `git diff --name-only origin/main...HEAD`
- `git diff --check`
- qsl local-ops helper help/fixture checks where applicable.
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- qsc send_commit test.
- formal model checks.
- qshield-cli build/test if feasible.
- queue/decisions helper checks.
- scope guard, link-check, leak-scan, classifier, and goal-lint.

## CI Expectations

Open a PR with the required Goals line and merge only after required checks,
including public-safety, are attached and green. Do not use admin bypass,
squash, rebase, direct push, force-push, amend after PR creation, or
delete-branch flags.

## Successor Handoff

After NA-0400 PR merge and post-merge public-safety success, optional closeout
may mark NA-0400 DONE and restore the exact selected NA-0401 successor without
implementing NA-0401.
