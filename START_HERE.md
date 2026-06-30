# START_HERE — QuantumShield / QSL Build (Operational Constitution)


## Working directory (mandatory)

If you started Codex in `~/work/qsl`, first change into the **qsl-protocol repo root**:

    cd ~/work/qsl/qsl-protocol

All relative paths in directives assume you are running from the repo root above.


**MUST READ (project directive):** docs/dev/DOC-DEV-003_Assistant_Operating_Rules_v1.0.0_DRAFT.md


Goals: G4 (primary), supports G1–G3

## 0. Purpose

This document is the single entry point for anyone (human or AI) working in this repo.
It defines the authoritative sources, the non-negotiable constraints, and the required workflow to prevent drift.

If any instruction conflicts, resolve conflicts using the “Authoritative Sources” order below.
If ambiguity remains, STOP and fail-closed: do not guess; do not “infer” protocol behavior.

## 0.1 Audience Entry Points

Use the shortest path that matches your role, then return to the governance
spine before making changes.

### I want the fastest public overview

1) Read [README.md](README.md).
2) Open the public evidence landing page: [docs/public/INDEX.md](docs/public/INDEX.md).
3) Check the current `NOT_READY` boundaries in the [release-readiness evidence map](docs/public/RELEASE_READINESS_EVIDENCE_MAP.md).

### I want to inspect evidence

1) Start at [docs/public/RELEASE_READINESS_EVIDENCE_MAP.md](docs/public/RELEASE_READINESS_EVIDENCE_MAP.md).
2) Follow the reviewer route in [docs/public/EXTERNAL_REVIEW_PACKAGE.md](docs/public/EXTERNAL_REVIEW_PACKAGE.md).
3) Use [TRACEABILITY.md](TRACEABILITY.md) and [DECISIONS.md](DECISIONS.md) to map claims to goals, decisions, tests, and remaining gaps.

### I want to run demos

1) Read [docs/demo/DEMO_ACCEPTANCE_CRITERIA.md](docs/demo/DEMO_ACCEPTANCE_CRITERIA.md).
2) Treat all demo evidence as non-production.
3) Compare demo results with the boundaries in [docs/public/INDEX.md](docs/public/INDEX.md).

### I want to review security or claims

1) Read [docs/public/EXTERNAL_REVIEW_PACKAGE.md](docs/public/EXTERNAL_REVIEW_PACKAGE.md).
2) Check claim wording against [docs/public/SUITE2_TRIPLE_RATCHET_CLAIM_BOUNDARY.md](docs/public/SUITE2_TRIPLE_RATCHET_CLAIM_BOUNDARY.md) and [docs/public/WEBSITE_CLAIM_MATRIX.md](docs/public/WEBSITE_CLAIM_MATRIX.md).
3) Record findings as missing evidence, ambiguous wording, or claim-boundary issues; do not treat the package as completed external review.

### I want to contribute

1) Read [CONTRIBUTING.md](CONTRIBUTING.md) and the active READY item in [NEXT_ACTIONS.md](NEXT_ACTIONS.md).
2) Prefer small evidence improvements: negative tests, reproducible demo proof, link fixes, clearer `NOT_READY` boundaries, or documentation that ties claims to evidence.
3) Preserve fail-closed behavior and stop before changing protocol, wire, crypto, state-machine, service, website, workflow, dependency, branch-protection, or public-safety semantics without explicit queue authorization.

## 1. Authoritative sources (strict priority order)

### 1.1 Governance spine (authoritative)

1) START_HERE.md (this file)
2) GOALS.md
3) AGENTS.md
4) PROJECT_CHARTER.md
5) NEXT_ACTIONS.md
6) DECISIONS.md
7) TRACEABILITY.md

### 1.2 Canonical protocol specs (authoritative for behavior, format, semantics)

- docs/canonical/DOC-CAN-003_*  (Suite-2 / QSP v5.0 lane)
- docs/canonical/DOC-CAN-004_*  (SCKA / Suite-2 supporting agreement)

### 1.3 Test plan and categories (authoritative for “what must be tested”)

- docs/test/DOC-TST-005_*  (vector categories, gates, required coverage)

### 1.4 Inputs (authoritative for actual test vectors)

- inputs/suite2/vectors/*.json
- inputs/suite2/vectors/README.md

### 1.5 Everything else (supporting)

Any other documents, diagrams, plans, or notes are supporting and may be reworked, renamed, or deprecated as needed,
but MUST NOT contradict the governance spine or canonical specs.

### 1.6 Continuity and roadmap references (supporting only)

Use the following checked-in supporting artifacts when the selected queue item needs continuity, recovery, operational memory, audit planning, or long-range release context:
- `docs/ops/DOC-OPS-001_qbuild_Continuity_and_Disaster_Recovery_Runbook_v0.1.0_DRAFT.md`
- `docs/ops/DOC-OPS-002_Continuity_Snapshot_Manifest_and_Offhost_Procedure_v0.1.0_DRAFT.md`
- `docs/ops/DOC-OPS-003_Rolling_Operations_Journal_Procedure_v0.1.0_DRAFT.md`
- `docs/ops/TEMPLATE_Rolling_Operations_Journal_v0.1.0.md`
- `docs/ops/DOC-OPS-004_Promotion_of_Recurring_Operational_Lessons_to_Canon_v0.1.0_DRAFT.md`
- `docs/program/DOC-PROG-001_Goal_to_Release_Roadmap_v0.1.0_DRAFT.md`
- `docs/audit/DOC-AUD-001_qsc_Director_Ready_Crypto_and_Code_Audit_Program_v0.1.0_DRAFT.md`

Authority split:
- the governance spine remains authoritative;
- `NEXT_ACTIONS.md` remains the execution source of truth;
- `DOC-PROG-001` and `DOC-AUD-001` are strategic/supporting only and must not reorder or override the queue; and
- `DOC-OPS-001` through `DOC-OPS-004` plus the rolling-journal template are operational guidance/supporting memory only and must not override live repo truth.

### 1.7 Bounded Codex Operational Authority

Project-wide bounded Codex operational authority is defined in
`docs/ops/CODEX_BOUNDED_OPERATIONAL_AUTHORITY.md`.

The authority model does not grant default remote action. Operational tiers
require active directive opt-in with an exact tier, host/workspace or local
path, command family, allowed mutation paths, raw-output quarantine path,
redaction/publication policy, rollback/manifest requirements, stop conditions,
and final response claim boundaries.

Tier 1 redacted diagnostics and Tier 2 bounded test action are allowed on
approved test hosts/workspaces only when the active directive names exact
boundaries. Tier 3 operator/admin action remains operator-owned unless a later
directive explicitly authorizes that privileged lane. The model does not weaken
qwork proof, the one-READY queue invariant, evidence gates, public-safety,
advisories, required-check visibility, private-material controls, or claim
boundaries.

## 2. Non-negotiable constraints (fail-closed)

### 2.1 Safety rails on changes

Do NOT change protocol behavior, wire semantics, cryptographic logic, or state machines unless the top READY item in NEXT_ACTIONS.md explicitly allows it.

If you are unsure whether a change affects behavior or semantics, treat it as behavior-changing and DO NOT proceed.

### 2.2 Fail-closed always

- Ambiguity => reject / error / stop.
- Missing fields => reject.
- Unknown versions/suites => reject.
- Unexpected state transitions => reject.
- Partial parsing or heuristics => forbidden.

### 2.3 Governance compliance for docs

AGENTS.md / goal-lint rules are binding. As a baseline expectation:

- Any new or modified documentation file MUST include a `Goals:` line near the top.
- Any non-trivial change requires governance updates (DECISIONS.md and/or TRACEABILITY.md) as appropriate.
- Never “silently” change semantics in docs: record it as a decision and trace it.

(If AGENTS.md specifies additional formatting rules, those supersede this section.)

## 3. Definitions (to reduce confusion)

### 3.1 Canonical spec vs supporting artifact

Canonical spec:
- Defines normative behavior/semantics and is self-contained.
- MUST be implementable without external references for required meaning.

Supporting artifact:
- Schema, checklist, vector category, test harness adapter, CI scripts, etc.
- May be smaller and modular, but MUST be atomic: no hidden semantics in other docs.

### 3.2 “Evidence” and “CI-gated”

“Evidence” means artifacts produced by the harness/CI that demonstrate:
- fail-closed behavior,
- vector execution,
- conformance results,
- and green status of required pipelines (goal-lint, qshield-ci lanes, suite2-ci).

## 4. Standard workflow for every work session (mandatory)

### Step 1 — Bootstrap
1) Read this START_HERE.md.
2) Read GOALS.md, AGENTS.md, PROJECT_CHARTER.md.
3) Open NEXT_ACTIONS.md and identify the top-most item with Status = READY.

### Step 2 — Confirm scope and constraints
Before touching code or docs, write down (in your working notes / PR description):
- Which NEXT_ACTIONS item you are executing (ID + title).
- Whether it allows behavior/wire/state changes (explicitly).
- The minimal change set required.

If anything is unclear, STOP and fail-closed.

### Step 3 — Implement the smallest change set
- Only implement what is required for the selected item.
- Avoid opportunistic refactors.
- Maintain compatibility and fail-closed defaults.
- Do not reorder NEXT_ACTIONS priorities.

### Step 4 — Run required checks and capture evidence
Minimum expectation before merging:
- goal-lint / governance checks are green
- qshield-ci lanes are green (4a–4d + durability lane if applicable)
- suite2-ci is green when Suite-2 docs/vectors/harness are touched

Use GitHub CLI/REST for visibility.

Authoritative rule: avoid watch modes; use bounded REST polling (see DOC-DEV-003).

Examples:
- Do NOT use `gh pr checks --watch`.
- `gh run view <RUN_ID> --log-failed`

### Step 5 — Governance updates (required when applicable)
- Update DECISIONS.md for any semantic/normative decisions, security-relevant constraints, or compatibility policies.
- Update TRACEABILITY.md to link:
  - goals -> specs -> tests -> implementation/harness evidence.

If unsure whether something warrants a decision/trace update, assume yes.

### Step 6 — Prepare an “overlay bundle” for review / chat handoff
When collaborating across chats or machines, prefer a fail-closed “overlay zip” containing only changed files.

Recommended overlay creation (changed files only):
- Determine changed files:
  - `git diff --name-only <base>..HEAD`
- Create zip from the file list:
  - `git diff --name-only <base>..HEAD | zip -@ overlay.zip`

Rules:
- The overlay must unzip at repo root and only touch intended files.
- Do not include build outputs, caches, virtualenvs, or `.git/`.

## 5. Document organization: how we avoid “doc drowning”

The authoritative spine is intentionally short:

1) START_HERE.md
2) GOALS.md
3) AGENTS.md
4) PROJECT_CHARTER.md
5) NEXT_ACTIONS.md
6) DECISIONS.md
7) TRACEABILITY.md
8) Canonical Suite-2 specs (DOC-CAN-003/004)
9) Test categories plan (DOC-TST-005)

Everything else is supporting and must be treated as:
- optional to read,
- not allowed to override the spine,
- and allowed to be reorganized or deprecated to reduce cognitive load.

Practical rule:
If a document cannot be clearly classified as either “canonical” or “supporting,” it is a governance problem and should be fixed by:
- updating the document header with its classification and scope,
- and/or moving its authoritative meaning into the canonical spec.

## CodeQL operating procedure (local fast check + CI gate)

- CI CodeQL is the authoritative security regression gate for PRs.
- For high-risk code changes (crypto/handshake/ratchet/state), run the fast local targeted CodeQL check before pushing.
- See: docs/dev/DOC-DEV-002_CodeQL_Operating_Procedure_v1.0.0_DRAFT.md

## Strategic continuity references

For continuity, operational memory, and strategic audit planning:
- use `docs/ops/DOC-OPS-001_qbuild_Continuity_and_Disaster_Recovery_Runbook_v0.1.0_DRAFT.md` for qbuild control-plane recovery and merge-refresh procedure;
- use `docs/ops/DOC-OPS-002_Continuity_Snapshot_Manifest_and_Offhost_Procedure_v0.1.0_DRAFT.md` for off-host snapshot requirements; and
- use `docs/ops/DOC-OPS-003_Rolling_Operations_Journal_Procedure_v0.1.0_DRAFT.md` plus `docs/ops/TEMPLATE_Rolling_Operations_Journal_v0.1.0.md` for rolling per-directive journal practice;
- use `docs/ops/DOC-OPS-004_Promotion_of_Recurring_Operational_Lessons_to_Canon_v0.1.0_DRAFT.md` for promoting stable recurring lessons into canon;
- use `docs/program/DOC-PROG-001_Goal_to_Release_Roadmap_v0.1.0_DRAFT.md` for goal-to-release context; and
- use `docs/audit/DOC-AUD-001_qsc_Director_Ready_Crypto_and_Code_Audit_Program_v0.1.0_DRAFT.md` for candidate future qsc audit/remediation sequencing.

These documents are subordinate to the governance spine. They provide continuity, operational memory, and strategic framing only; they do not change execution order, and they do not weaken the rule that `NEXT_ACTIONS.md` is the live execution queue.

Scope boundary (global):
- Relay/TUI must not drive protocol-core changes. Any behavior-level change must return to governance + NEXT_ACTIONS promotion.

## 6. How to start a new chat (authoritative)

Paste the “New Chat Starter” text from the top of NEXT_ACTIONS.md.
Do not paraphrase it. Do not add new constraints unless you also add them to START_HERE.md and record a decision.

---
End of START_HERE.md
