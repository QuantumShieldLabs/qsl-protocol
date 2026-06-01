Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-01

# NA-0401 Project Goal and Operating Principles Canon Authorization Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Verify that NA-0401 creates the internal qsl-protocol governance canon at
`docs/governance/PROJECT_GOAL_AND_OPERATING_PRINCIPLES.md` without creating
public docs, public claims, runtime behavior, crypto behavior, dependency,
workflow, sibling-repo, backup, restore, key, secret, or response-archive
changes.

## Protected Invariants

- READY_COUNT remains exactly 1.
- READY remains NA-0401 until closeout.
- NA-0400 is DONE.
- D-0782 exists once.
- D-0783 exists once.
- D-0784 is added once.
- D-0785 is absent until optional NA-0401 closeout.
- The canon is internal governance only.
- The canon does not override `NEXT_ACTIONS.md`, `DECISIONS.md`,
  `TRACEABILITY.md`, CI, branch protection, or live evidence.
- No public readiness claim is introduced.
- No production readiness claim is introduced.
- No external-review-complete claim is introduced.
- No metadata-free claim is introduced.
- No anonymity claim is introduced.
- No untraceable claim is introduced.
- No disaster-recovery-complete claim is introduced.
- No off-host-backup-complete claim is introduced.
- No restore-proven claim is introduced.
- No key-custody-implemented claim is introduced.
- No key-recovery-implemented claim is introduced.
- No bug-free or perfect-crypto claim is introduced.
- No compliance or certification claim is introduced.

## Allowed Scope

- `docs/governance/PROJECT_GOAL_AND_OPERATING_PRINCIPLES.md`
- `tests/NA-0401_project_goal_operating_principles_canon_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

- `.github/**`
- `scripts/**`
- `inputs/**`
- `formal/**`
- `qsc/**`
- `qsp/**`
- `qsl/**`
- `qsl-client/**`
- `apps/**`
- `tools/**`
- `Cargo.toml`
- `Cargo.lock`
- workflows
- runtime, service, protocol, crypto, qshield runtime, qsl-server,
  qsl-attachments, qsc-desktop, website, README, START_HERE, docs/public,
  public technical paper, external-review package, SECURITY.md, security.txt,
  disclosure policy, issue template, backup script/timer/fstab/source-list,
  off-host target, restore target, key, credential, passphrase, private key,
  recovery envelope, response archive, request archive, directive archive,
  local qstart/qresume tooling, and secret-bearing path mutation.

## NA-0400 Inheritance Requirements

- Consume NA-0400 evidence that public claim and external-review readiness
  remain evidence-incomplete and future-gated.
- Preserve NA-0400's selected successor rationale: internal operating
  principles should be canonized before public paper or public-claim work.
- Carry forward qsl-server PR #56 as service-local bounded harness evidence
  only.
- Carry forward qsl-attachments PR #37 as service-local prerequisite evidence
  only.
- Carry forward qshield demo evidence as demo/harness evidence only.

## Canon Artifact Requirements

The canon must include:

- title and status;
- north star;
- security before speed;
- evidence over vibes;
- code and crypto excellence;
- no public overclaiming;
- one-READY queue discipline;
- scope control and no drift;
- routine audit rhythm;
- external awareness without hype;
- public technical paper timing;
- Director / Codex / human roles;
- backup / restore / key honesty;
- service and demo honesty;
- how future directives use the canon;
- update policy.

## Internal-Governance-Only Requirements

- The artifact must state it is internal governance only.
- The artifact must state it is not public docs, not a public claim, not
  production readiness, not public-internet readiness, not external review, and
  not a public technical paper.
- The artifact must not update README, START_HERE, docs/public, website, or
  public-facing material.

## No-Public-Claim Requirements

- Strong public/readiness/privacy/security claims must appear only as forbidden
  or future-gated language.
- The artifact must not claim bug-free behavior or perfect crypto.
- The artifact must not claim metadata-free behavior.
- The artifact must not claim anonymity or untraceability.
- The artifact must not claim production readiness.
- The artifact must not claim public-internet readiness.
- The artifact must not claim external-review completion.
- The artifact must not claim off-host backup completion.
- The artifact must not claim disaster recovery completion.
- The artifact must not claim restore proof.
- The artifact must not claim key custody implementation or key recovery implementation.
- The artifact must not claim compliance or certification.

## No-Overriding-Queue Requirements

- The artifact must state that it does not override live `NEXT_ACTIONS.md`,
  `DECISIONS.md`, `TRACEABILITY.md`, CI, branch protection, or evidence.
- It must not promote or implement NA-0402.

## Update Policy Requirements

- Future edits require a governance PR.
- Future edits require a `DECISIONS.md` entry, `TRACEABILITY.md` update,
  testplan, scope guard, link/leak checks, and public-safety.
- Silent edits are prohibited.

## Marker Requirements

The canon must contain:

- `QSL_PROJECT_GOAL_CANON_INTERNAL_ONLY`
- `QSL_SECURITY_BEFORE_SPEED`
- `QSL_EVIDENCE_OVER_VIBES`
- `QSL_CODE_CRYPTO_EXCELLENCE`
- `QSL_NO_PUBLIC_OVERCLAIMING`
- `QSL_ONE_READY_QUEUE_DISCIPLINE`
- `QSL_ROUTINE_AUDIT_RHYTHM`
- `QSL_EXTERNAL_AWARENESS_WITHOUT_HYPE`
- `QSL_PUBLIC_PAPER_TIMING_BOUNDARY`
- `QSL_DIRECTOR_CODEX_HUMAN_ROLE_BOUNDARY`
- `QSL_NO_RUNTIME_CHANGE`
- `QSL_NO_CRYPTO_IMPLEMENTATION_CHANGE`
- `QSL_NO_DEPENDENCY_CHANGE`
- `QSL_NO_WORKFLOW_CHANGE`
- `QSL_NO_PUBLIC_READINESS_CLAIM`
- `QSL_NO_SECRET_MATERIAL`

## Role-Boundary Requirements

- Director owns plan, scope, queue, and next directive.
- Codex executes locally and returns evidence / verification bundles.
- Human operator provides external inputs, safe authorized secrets when
  explicitly permitted, and final judgment.
- No background/asynchronous promises.
- No hidden public claims.

## Routine Audit Rhythm Requirements

- The canon should justify routine overall and code/crypto audits.
- Audit output should create evidence and candidates rather than panic changes.
- Critical/high blockers must use queue discipline.

## Public Paper Timing Requirements

- Public technical paper work must remain future-gated.
- The canon must identify prerequisite evidence groups before public paper work.
- The canon must prohibit marketing ahead of evidence.

## Service / Demo / Backup / Crypto Honesty Requirements

- qsl-server and qsl-attachments evidence remains service-local unless future
  evidence proves otherwise.
- qshield demo evidence remains demo/harness evidence only.
- Same-host continuity is not disaster recovery.
- No-secret harnesses are not real key custody or recovery.
- Dry-run restore is not real restore proof.
- No bug-free or perfect-crypto claim is permitted.

## Scope Guard Requirements

- Changed paths must be limited to the allowed NA-0401 Packet C-E path set.
- No code/runtime/dependency/workflow/public-doc/website/sibling-repo/helper
  path may be changed.
- No response archive mutation is permitted before final response writing.

## Link / Leak / Overclaim Requirements

- Local markdown link check must report no missing links.
- Leak scan over changed files must report no secret material.
- High-risk phrase scan must classify all matches as negated, forbidden,
  future-gated, evidence-limited, or internal-governance wording.

## CI Expectations

- public-safety remains required and green before merge.
- `cargo audit --deny warnings` passes.
- `cargo tree -i rustls-webpki --locked` reports `v0.103.13` or a newer safe
  version.
- Queue and decision helpers report READY_COUNT 1, READY NA-0401, D-0784 once,
  D-0785 absent, and no duplicate decisions.
- Relevant helper fixture checks and heavy validation complete or any skipped
  item is recorded with a scope-safe reason.

## Successor Handoff

Select exactly one successor without implementing it.

Expected normal successor:

`NA-0402 -- QSL Director State Index Authorization Plan`

Alternate blocker successor:

`NA-0402 -- QSL Project Goal Canon Scope Conflict Resolution`
