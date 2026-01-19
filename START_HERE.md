# START_HERE — QuantumShield / QSL Build (Operational Constitution)

**MUST READ (project directive):** docs/dev/DOC-DEV-003_Assistant_Operating_Rules_v1.0.0_DRAFT.md


Goals: G4 (primary), supports G1–G3

## 0. Purpose

This document is the single entry point for anyone (human or AI) working in this repo.
It defines the authoritative sources, the non-negotiable constraints, and the required workflow to prevent drift.

If any instruction conflicts, resolve conflicts using the “Authoritative Sources” order below.
If ambiguity remains, STOP and fail-closed: do not guess; do not “infer” protocol behavior.

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

Use GitHub CLI for visibility (examples):
- `gh pr checks --watch`
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

## Execution Roadmap: Suite-2 → Relay → Linux TUI Demo

Current focus: Audit queue completed (READY_COUNT=0). Next work is governance-scoped roadmap + demo plumbing as BACKLOG until explicitly promoted.

### Phase 0 — Evidence gates (keep protocol stable)

Scope:
- Treat CodeQL as a continuous security regression gate (see DOC-DEV-002).
- Maintain fail-closed behavior, deterministic rejects, and no-mutation-on-reject invariants for protocol/stateful code.

Exit criteria:
- CI remains green; CodeQL/goal-lint stay enforced.
- Any new protocol work must be explicitly queued and promoted (no ad-hoc changes).

### Phase 1 — Dumb Relay/Server (transport-only)

Scope:
- Implement a minimal relay that forwards/persists opaque payloads.
- Must not interpret or alter protocol messages; no protocol-core changes.

Exit criteria:
- End-to-end relay smoke path documented in test plan.
- CI remains green; no protocol-core changes introduced.

### Phase 2 — Linux TUI Demo Client

Scope:
- A demo UX that exercises existing protocol behavior via the relay.
- Must not require protocol/wire changes.

Exit criteria:
- Demo works end-to-end using existing approved interfaces.
- CI remains green; no protocol-core changes introduced.

Scope boundary (global):
- Relay/TUI must not drive protocol-core changes. Any behavior-level change must return to governance + NEXT_ACTIONS promotion.

## 6. How to start a new chat (authoritative)

Paste the “New Chat Starter” text from the top of NEXT_ACTIONS.md.
Do not paraphrase it. Do not add new constraints unless you also add them to START_HERE.md and record a decision.

---
End of START_HERE.md
