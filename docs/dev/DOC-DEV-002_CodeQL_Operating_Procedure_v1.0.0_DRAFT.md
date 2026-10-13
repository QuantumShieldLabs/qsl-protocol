# DOC-DEV-002_CodeQL_Operating_Procedure_v1.0.0_DRAFT: current use

Goals: G1, G2, G3, G4, G5

Revision: 2026-10-13. This current procedure replaces the older process text under the operator-approved project method.

Read [the project workflow](../../PROJECT_WORKFLOW.md).

Retain required CI CodeQL and actual alert triage. For a reachable key/nonce/randomness flaw, fix and regress the affected behavior; test/sentinel classification needs source and point-of-use refusal evidence. Use a targeted local query only when useful to the changed boundary; it is not an unconditional full local audit before every push. Keep DB/SARIF under scoped private evidence/cache roots, outside product source. Never widen authentication scopes or dismiss alerts without human authority. A permission refusal is not zero findings.


The [complete prior document](https://github.com/QuantumShieldLabs/qsl-protocol/blob/65059fad067119c54528e5c9b56b65e1f00720d3/docs/dev/DOC-DEV-002_CodeQL_Operating_Procedure_v1.0.0_DRAFT.md) is an immutable historical snapshot; its old procedural instructions do not govern new work. Its accepted product requirements remain subject to the current roadmap and later accepted amendments.
