# Protocol-change checks

Goals: G1, G2, G3, G4, G5

Revision: 2026-10-13. Read [the project workflow](PROJECT_WORKFLOW.md).

Before dependent implementation, settle the changed wire/security contract, registry identifiers, compatibility/refusal and ownership semantics. Implement the complete affected caller path with meaningful positive/refusal/vector checks and necessary independent sensitive review. Update affected canonical/decision/traceability references. Log any broader deferred qualification with risk and mandatory trigger; retain required CI and human merge/release gates. Do not choose a temporary production format or quietly alter required caps.
