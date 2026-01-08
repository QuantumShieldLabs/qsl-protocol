# CODEX_RULES.md — QuantumShield Phase 4 (STRICT)

These rules are binding. If a request conflicts with these rules, STOP and ask the human.

## A) Scope and authority
You (Codex) may implement Phase 4 code/scripts/CI and run commands.
You may NOT change Phase 2 canonical specs (QSP/QSE) or any wire behavior implied by them.

## B) Immutable inputs (DO NOT MODIFY)
Treat as immutable:
- inputs/phase2/QuantumShield_Phase2_CANONICAL_FROZEN_QSP4.3.2_QSE1.8.2.zip
- inputs/phase3/QuantumShield_Phase3_SUPPORTING_COMPLETE_P3-02_to_P3-30.zip

Do not modify any content inside these bundles.
Only allowed with explicit human approval:
- updating inputs/phase2/phase2.zip.sha256
- updating inputs/phase3/phase3.zip.sha256
- regenerating inputs/phase3/phase3.lock.json

If any conflict is found: do NOT change QSP/QSE or wire behavior. Log Phase 3 errata instead.

## C) Fail-closed requirements (non-negotiable)
All parsing/validation must be fail-closed:
- reject unknown versions/IDs/flags
- reject truncation, overruns, trailing bytes
- enforce QSE/QSP bounds strictly
- no best-effort parsing, no silent coercions

Do not weaken validation to “make tests pass.”

## D) Dependencies and network
No new dependencies without explicit human approval.
No network-required build/test assumptions. CI must run from clean checkout with pinned deps.

## E) Change discipline
Every change must be minimal, scoped, and reproducible.
After edits, run:
- scripts/ci/run_4a.sh
and include the exact command(s) you ran and the artifact paths produced.

## F) Errata handling (mandatory)
When conflicts appear between Phase 2 canonical and Phase 3 supporting artifacts:
- do not change QSP/QSE or wire behavior
- create/append an errata record under:
  artifacts/<RUN_ID>/phase4_errata/
(or docs/errata/ if present)

Errata entry must include: what failed, why it conflicts (cite frozen spec section), impact, and remediation that does not change wire behavior.

## G) Output locations
All generated outputs must go to:
- artifacts/<RUN_ID>/
Do not overwrite prior runs.

## H) Allowed edit paths (extra strict)
You may edit ONLY:
- scripts/
- tools/
- tests/
- .github/workflows/
- README*.md
- requirements-ci.txt / lockfiles (only if approved)

Do NOT edit:
- inputs/phase2/* (except pins with approval)
- inputs/phase3/* (except pins/lock with approval)
- any extracted frozen spec copies
