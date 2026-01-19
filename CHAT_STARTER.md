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

Project directive (must follow):
- Continuous improvement: implement safe/in-scope improvements immediately; ask approval if unsure.
- ALL Codex directives must be fully contained in a single code block (or split into multiple code-block directives).
- See docs/dev/DOC-DEV-003_Assistant_Operating_Rules_v1.0.0_DRAFT.md.


# DEPRECATED — Superseded by NEXT_ACTIONS.md (New Chat Starter)

This file is retained only for backwards compatibility with older chat workflows.

**Do not paste this file into new chats.**

Where to look now:
- The authoritative “New Chat Starter” block lives at the top of **NEXT_ACTIONS.md**.
- **START_HERE.md** is the operational constitution and the first mandatory read.
- **GOALS.md / AGENTS.md / PROJECT_CHARTER.md / DECISIONS.md / TRACEABILITY.md** are the authoritative governance spine.

If any tool, script, or prior note instructs you to use this file as the chat starter, treat that instruction as stale.
