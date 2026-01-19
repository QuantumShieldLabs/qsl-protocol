## Codex autonomy within guardrails (authoritative)

**Default mode:** Codex is authorized to proceed autonomously on routine work **within the declared Scope** (coding, debugging, CI-attach, mechanical doc fixes), without asking for permission at each micro-step.

**Codex MUST STOP and escalate immediately** if any of the following occurs:
1) Any command exits non-zero (real error, not warnings).
2) Any change would touch out-of-scope paths or a different repo than declared.
3) Any decision would alter protocol/wire/crypto semantics when the task is “demo/client/governance only.”
4) Any change would weaken security posture (e.g., disabling validation, loosening auth, bypassing checks).

**Non-fatal warnings are not STOP conditions** (e.g., GraphQL deprecation notices, benign stderr). Log them in the evidence bundle and continue.

**Retry policy (bounded):**
- CI attach flakiness: up to **2** allow-empty retriggers maximum.
- Avoid long polling loops; prefer short check-runs/API probes and report state.

**Perfection directive:**
If Codex notices an improvement that increases correctness, safety, clarity, or testability **within scope**, it should implement it; if unsure whether it is acceptable, STOP and ask.


Goals: G4

# AGENTS.md (Repository Agent Policy)

This file governs automated assistants (including Codex) operating on this repository.

## Mandatory reads (before any changes)
1. GOALS.md
2. PROJECT_CHARTER.md
3. DECISIONS.md
4. TRACEABILITY.md
5. CHECKLIST_PROTOCOL_CHANGE.md (when relevant)

## Non-negotiable rules
- Every change MUST advance at least one Goal ID (G1–G5) and MUST NOT regress others.
- Prefer fail-closed semantics: if uncertain, abort/reject rather than accept/continue.
- No silent downgrades: any fallback must be explicit, negotiated, and documented.
- Protocol behavior changes MUST include tests/vectors in the same PR.
- Any change to state machines, key schedules, negotiation, or wire semantics MUST add a DECISIONS.md entry and update TRACEABILITY.md.
- Keep specs self-contained: do not introduce required meaning via external references.

## Required PR / task metadata
Every task/PR description MUST include:
- **Goals:** G1, G2, ... (at least one)
- **Impact:** short description of how goals are advanced
- **No-regression:** statement of invariants preserved
- **Tests/Vectors:** what was added/updated

## Goal-lint PR body requirements (exact)
- Include a standalone line in the PR body: `Goals: G1, G2, ...`
- Use ASCII commas, no ranges (e.g., avoid `G1–G5` or `G1-G5`).
- Keep the Goals line near the top so goal-lint can detect it reliably.

## If repository layout differs
If the repository does not contain the referenced files, create them at repo root.
If CI/workflow integration requires path tuning, do so without weakening enforcement intent.

## Assistant operating rules (project directive)
- All assistants and Codex sessions MUST follow docs/dev/DOC-DEV-003_Assistant_Operating_Rules_v1.0.0_DRAFT.md.
- In particular: every Codex directive must be fully contained in a code block (or split into multiple code-block directives).
