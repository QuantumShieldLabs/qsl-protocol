# DOC-DEV-003 — Assistant Operating Rules (Project Directive) v1.0.0 DRAFT

Status: DRAFT
Date: 2026-01-19
Owner: QSL governance
Applies to: All assistants and Codex sessions in this project

## 1. Non-negotiable operating directive

### 1.1 Continuous improvement (“perfection” standard)
- If an improvement is clearly safe and in-scope, implement it immediately.
- If an improvement could change scope, security posture, protocol behavior, or create churn: STOP and request explicit approval with clear options.

### 1.2 Codex directives must be code-block complete
- Every directive sent to Codex MUST be fully contained in a single code block.
- If a directive is long, split it into multiple directives; each directive must be in its own code block.
- No directive content may appear outside a code block.

### 1.3 Mandatory directive structure (minimum)
Each Codex directive MUST include:
- Objective (what “done” means)
- Allowed scope (paths allowed to change)
- Stop conditions (when to halt and report)
- Verification bundle (what evidence must be pasted back)

## 2. Security invariants (project baseline)
- Fail-closed on rejects; deterministic error behavior where specified by policy.
- “No mutation on reject” for any stateful reject path.
- CodeQL: CI is authoritative; local targeted queries are used as fast regression checks.

## 3. Process invariants
- Clean-tree preflight for any verification directive.
- If unexpected diffs appear: stop, salvage if needed, and resume from a clean base.
