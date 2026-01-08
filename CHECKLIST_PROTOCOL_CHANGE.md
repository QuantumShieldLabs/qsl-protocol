# Protocol Change Checklist (Required)

Use this checklist for any change that affects:
- protocol state machines
- key schedule / KDF inputs
- negotiation / capabilities
- envelope fields / associated data
- persistence / rollback behavior
- conformance vectors or CI gates

## Required declarations
- [ ] **Goals advanced (G1–G5):** ____
- [ ] **No-regression invariants preserved:** ____
- [ ] **Security-sensitive surfaces touched:** (key schedule / SCKA / negotiation / metadata / persistence) ____

## Suite-2 (G1) requirements
- [ ] Every application message derives `mk = KDF_HYBRID(ec_mk, pq_mk)` in Suite-2.
- [ ] pq_chain advances every message; reseed rules are explicit and tested.
- [ ] AD binding covers all negotiation and PQ-relevant fields.

## SCKA (G2) requirements
- [ ] Epoch monotonicity rules are explicit.
- [ ] Accept/reject semantics are deterministic and fail-closed.
- [ ] Persistence and rollback detection requirements are specified and tested.

## Downgrade resistance (G3)
- [ ] Capability negotiation is transcript-bound.
- [ ] If Suite-2 is mutually supported, fallback is impossible (fail-closed).
- [ ] Negative tests cover downgrade attempts.

## Verification gates (G4)
- [ ] Conformance vectors/tests updated or added.
- [ ] FORMAL_VERIFICATION_PLAN.md updated if relevant.
- [ ] CI gates updated if new surfaces introduced.

## Metadata (G5)
- [ ] Observable identifiers reviewed and minimized where feasible.
- [ ] Padding/fingerprinting impacts considered and documented.
- [ ] Error behavior consistency reviewed.

## Documentation & traceability
- [ ] TRACEABILITY.md updated (or explicitly not applicable, with reason).
- [ ] DECISIONS.md updated (or explicitly not applicable, with reason).
