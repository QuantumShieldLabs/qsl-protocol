# NA-0057 — Public Demo Runbook — Test Plan (DRAFT)

Goals: G4, G5

## Purpose
Provide minimal evidence that the runbook exists, is registered, and governance wiring is updated, including required privacy discipline sections.

## Validation steps
1. Confirm the runbook file exists:
   - docs/dev/DOC-DEV-004_Public_Demo_Runbook_v0.1.0_DRAFT.md
2. Confirm the runbook includes required sections (by heading):
   - "Privacy envelopes"
   - "Receipt/ACK camouflage"
   - "Logging/metrics privacy budget"
   - "Uniform rejects" and "No mutation on reject"
3. Confirm docs inventory registration:
   - docs/DOCS_MAP.md includes DOC-DEV-004 path
4. Confirm governance updates:
   - DECISIONS.md includes D-0007
   - TRACEABILITY.md includes NA-0057 entry
   - NEXT_ACTIONS.md includes NA-0057 with Status: READY
5. CI/Policy:
   - PR body contains a standalone `Goals: G4, G5` line near the top
   - goal-lint and required checks attach and pass

## Expected results
- All artifacts exist and cross-reference correctly.
- Exactly one READY item exists in NEXT_ACTIONS.md after merge (NA-0057).
- The runbook explicitly encodes envelope strategy, reject/test discipline, ACK camouflage claims discipline, and analytics budget.
