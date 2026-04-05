Goals: G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-04

# NA-0220 Handshake Security Audit Test Plan

## Scope

- validate the docs/governance-only `NA-0220` implementation lane;
- confirm the audit report stays read-only and evidence-first; and
- confirm only the approved governance companions changed while closeout and remediation promotion remain deferred.

## Docs-only validation checkpoints

- local goal-lint using a synthesized `GITHUB_EVENT_PATH`
- markdown inventory counts from the `AGENTS.md` manual runbook
- deterministic local-link existence check from `AGENTS.md`
- changed-path scope proof limited to:
  - `DECISIONS.md`
  - `TRACEABILITY.md`
  - `docs/audit/DOC-AUD-002_qsc_Handshake_Execution_Security_Audit_v0.1.0_DRAFT.md`
  - `tests/NA-0220_handshake_security_audit_testplan.md`
- added-line leak-safe scan confirming no secret-like values were introduced

## Reference targets

- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/audit/DOC-AUD-002_qsc_Handshake_Execution_Security_Audit_v0.1.0_DRAFT.md`
- `tests/NA-0220_handshake_security_audit_testplan.md`

## Acceptance checkpoints

- the audit report states its read-only authority posture and names the exact code/spec/test surfaces reviewed
- the audit report includes the required sections:
  - scope and authority used
  - methods used
  - read-only evidence commands/tests used
  - findings by severity
  - consolidated answers to the five core audit questions
  - recommended remediation shapes
  - explicit conclusion
- every non-trivial finding uses the mandatory schema:
  - Finding ID
  - Severity
  - Exact surfaces
  - Violated claim/invariant
  - Why it matters
  - Evidence/proof
  - Minimal fix direction
  - Proof gap
  - Recommended directive shape
- `DECISIONS.md` records this lane as read-only/evidence-first, notes whether `P0`/`P1` findings were present, and does not authorize fixes
- `TRACEABILITY.md` records `NA-0220` as implementation/evidence only, links the new audit report and this test-plan stub, and states that closeout plus queue promotion remain pending
