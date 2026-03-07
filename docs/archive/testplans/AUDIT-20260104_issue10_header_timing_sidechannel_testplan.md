# AUDIT-20260104 Issue #10 — Header Decryption Timing Side-Channel Test Plan

Goals: G4, G5

## Invariant
Header decryption must not short-circuit candidate-key trials based on success position. Rejects must be deterministic and must not mutate state.

## Scope
- QSP header decryption candidate-key trials (qsp/ratchet.rs)
- Suite-2 non-boundary header candidate trials (suite2/ratchet.rs)

## Regression guards (CI-exercised)
- QSP:
  - `header_decrypt_attempts_all_candidates_even_on_first_success` (qsp/ratchet.rs)
  - `header_decrypt_rejects_deterministically_and_no_state_mutation` (qsp/ratchet.rs)
- Suite-2:
  - `nonboundary_header_attempts_all_candidates_even_on_first_success` (suite2/ratchet.rs)
  - `nonboundary_rejects_deterministically_and_no_state_mutation` (suite2/ratchet.rs)

## Expected results
- All candidate keys are attempted in bounded order, regardless of first success.
- Reject paths return deterministic error outcomes and do not mutate state.

## Evidence
- CI: `ci-4a` and `suite2-vectors` (and required lanes) green on the PR.
