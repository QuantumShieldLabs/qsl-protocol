Goals: G1, G2, G3, G4, G5

Status: Internal Governance Canon
Owner: QSL governance
Last-Updated: 2026-06-01
Scope: qsl-protocol internal governance; not public documentation

# QSL Project Goal and Operating Principles

QSL_PROJECT_GOAL_CANON_INTERNAL_ONLY

This artifact records QSL's internal north star and operating principles for
future Directors, Codex runs, and human operators. It is internal governance
only. It is not public docs, not a public claim, not production readiness, not
public-internet readiness, not external review, and not a public technical
paper.

This canon does not override live `NEXT_ACTIONS.md`, `DECISIONS.md`,
`TRACEABILITY.md`, CI, branch protection, or evidence. If live evidence and
this document conflict, stop and resolve the live governance/evidence conflict
through a future governance PR.

## North Star

QSL's goal is to build a security-first, evidence-driven, high-assurance
protocol and tooling project that stands out because it is clean, secure,
well-organized, auditable, and honest about what is and is not proven.

The project should be the kind of project serious reviewers notice because it
is careful, not because it overclaims.

## Security Before Speed

QSL_SECURITY_BEFORE_SPEED

Prefer small, atomic, reviewed changes. Prefer fail-closed behavior. Prefer no
mutation on rejected input. Prefer bounded waits and deterministic outcomes.
Stop rather than guess when evidence is missing, authority is unclear, or a
fallback would dilute a protected invariant.

## Evidence Over Vibes

QSL_EVIDENCE_OVER_VIBES

Every claim should map to evidence. Green CI is necessary but not sufficient. A
test only counts when it asserts the protected invariant. Source-cited external
claims require current sources. Internal evidence, public claims, and external
review are separate categories and must not be conflated.

## Code and Crypto Excellence

QSL_CODE_CRYPTO_EXCELLENCE

Code should be secure, clean, organized, minimal, and maintainable. Crypto and
protocol logic receive extra scrutiny. Key, nonce, RNG, transcript,
suite/domain, side-channel, panic/unwrap, unsafe, dependency, formal-model,
fuzz/property, oracle/refimpl, and service-boundary risks remain priority audit
themes.

No bug-free or perfect-crypto claim is permitted.

## No Public Overclaiming

QSL_NO_PUBLIC_OVERCLAIMING
QSL_NO_PUBLIC_READINESS_CLAIM

- Do not claim production readiness without explicit evidence and future authorization.
- Do not claim public-internet readiness without explicit evidence and future authorization.
- Do not claim external-review completion without explicit evidence and future authorization.
- Do not claim metadata-free behavior without explicit evidence and future authorization.
- Do not claim anonymity or untraceability without explicit evidence and future authorization.
- Do not claim hidden timing, hidden traffic shape, or hidden attachment size without explicit evidence and future authorization.
- Do not claim off-host backup completion without explicit evidence and future authorization.
- Do not claim disaster recovery completion without explicit evidence and future authorization.
- Do not claim restore proof without explicit evidence and future authorization.
- Do not claim key custody implementation or key recovery implementation without explicit evidence and future authorization.
- Do not claim FIPS/TLS/HPKE/MLS compliance or certification without explicit evidence and future authorization.

## One-READY Queue Discipline

QSL_ONE_READY_QUEUE_DISCIPLINE

Exactly one `NEXT_ACTIONS.md` item should be READY. Execute in order. Closeout
only after merge and post-merge public-safety evidence. Do not override the
queue unless a documented blocker requires it. Findings may propose candidates,
but they do not auto-promote READY items.

## Scope Control and No Drift

QSL_NO_RUNTIME_CHANGE
QSL_NO_CRYPTO_IMPLEMENTATION_CHANGE
QSL_NO_DEPENDENCY_CHANGE
QSL_NO_WORKFLOW_CHANGE

Each directive must define allowed and forbidden paths. Scope guards are
mandatory. Avoid drive-by refactors. Do not cross from docs/governance to
runtime without exact authorization. Do not mutate a sibling repo without exact
directive authority.

## Routine Audit Rhythm

QSL_ROUTINE_AUDIT_RHYTHM

Overall project audits and code/crypto audits should be routine. Audits should
create evidence and candidates, not panic changes. Critical/high blockers should
move through queue discipline. Audit reports should distinguish findings,
evidence gaps, claim boundaries, and backlog candidates.

## External Awareness Without Hype

QSL_EXTERNAL_AWARENESS_WITHOUT_HYPE

Watch standards, RFCs/drafts, advisories, research, secure messaging,
backup/restore/key guidance, disclosure guidance, and public-claim context.
Drafts, preprints, and vendor claims must be labeled. External source discovery
is not external review. External review is a separate evidence state.

## Public Technical Paper Timing

QSL_PUBLIC_PAPER_TIMING_BOUNDARY

A public paper is desirable only after evidence supports bounded claims.
Prerequisites include project goal canon, standards mappings, advisory policy,
code/crypto audit status, metadata/privacy boundaries, backup/restore/key
boundaries, service boundaries, external-review readiness, public-claim audit,
and citation policy.

The paper must not become marketing ahead of evidence.

## Director / Codex / Human Roles

QSL_DIRECTOR_CODEX_HUMAN_ROLE_BOUNDARY

The Director owns plan, scope, queue, and next directive. Codex executes
locally and returns raw evidence / verification bundles. The human operator
provides external inputs, secrets only when explicitly safe and authorized, and
final judgment.

No background/asynchronous promises are allowed. No hidden public claims are
allowed.

## Backup / Restore / Key Honesty

Same-host continuity is not disaster recovery. No-secret harnesses are not real
key custody or recovery. Dry-run restore is not real restore proof. Off-host
target, host identity, key custody, key recovery, monitoring, runbook, and real
restore remain distinct evidence states.

## Service and Demo Honesty

qsl-server and qsl-attachments evidence remains service-local unless future
production/public-internet evidence exists. qshield demo evidence is not
production proof. Demos and harnesses are valuable, but must be labeled.

## How Future Directives Use This Canon

Use this canon as a tie-breaker against scope creep and overclaiming. Use it to
justify audit cadence and conservative public-paper timing. Use it to explain
why work is deliberately gated.

It does not override live `NEXT_ACTIONS.md`, `DECISIONS.md`, `TRACEABILITY.md`,
CI, branch protection, or evidence.

## Update Policy

QSL_NO_SECRET_MATERIAL

Future changes require a governance PR, `DECISIONS.md` entry, `TRACEABILITY.md`
update, testplan, scope guard, link/leak checks, and public-safety. Do not
silently edit this canon. Do not add secret material.
