Goals: G1, G3, G4, G5
Status: DRAFT

Scope:
- QSP v4.3 header key derivation (refimpl protocol-core lane).
- No wire-format changes; correctness hardening only.

Objective:
- Prove header keys are derived from RK using KMAC-based KDF and correct domain separation labels.
- Ensure wrong-RK or invalid inputs fail deterministically without state mutation.

Invariants under test:
- Header keys depend on RK (KMAC-based KDF); placeholders/static labels are forbidden.
- Wrong RK fails to decrypt headers (deterministic reject).
- Rejected inputs do not mutate session state.

Positive/negative vectors planned:
- Positive: correct RK produces expected header keys for both directions (A->B, B->A) and NHK labels.
- Negative: wrong RK (or mismatched role) fails with deterministic error.
- Negative: placeholder/static derivation not used in protocol lanes (guard test in refimpl).

CI commands expected to gate:
- cargo test -p quantumshield_refimpl --locked
- cargo test -p refimpl_actor_rs --locked
- suite2-ci (if vectors added)

No-mutation-on-reject checks:
- Ensure session state (rk, hk/nhk, counters) unchanged after failed derivation/decrypt attempt.

Evidence:
- CI logs for refimpl unit tests and suite2-ci where applicable.
