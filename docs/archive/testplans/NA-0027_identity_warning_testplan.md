Goals: G5
Status: DRAFT

Scope:
- Demo CLI UX only; no protocol-core changes.

Objective:
- Warn on first establish to prompt out-of-band identity verification.
- Require explicit override flag to suppress the warning.

CI-gated assertions:
- First establish without --demo-identity-verified prints an identity warning.
- Establish with --demo-identity-verified suppresses the warning.

Evidence:
- metadata-conformance-smoke CI logs.
