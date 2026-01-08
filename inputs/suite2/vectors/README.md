# Suite-2 conformance vectors (authoritative)

This directory contains the authoritative Suite-2 conformance vector JSON fixtures executed in CI (suite2-ci).

Key categories:
- CAT-S2-KDF-001: Suite-2 KDF vectors
- CAT-SCKA-LOGIC-001: SCKA monotonicity + one-time consumption logic vectors
- CAT-SCKA-KEM-001: SCKA KEM correctness vectors (ML-KEM-768 fixtures)
- CAT-S2-DOWNGRADE-001: Suite-2 downgrade fail-closed vectors

Policy:
- Do not change vector JSON without updating the corresponding category documentation and CI evidence expectations.
- CI is fail-closed: regressions must block merges.

## Spec alignment (NA-0003)

Goals: G1, G4

This vectors directory is aligned to the canonical Suite-2 spec:
docs/canonical/DOC-CAN-003_QSP_Suite-2_True_Triple_Ratchet_v5.0.0_DRAFT.md

- KDF label namespace and ordering: see DOC-CAN-003 §3.3 (KDF definitions) and §7.3 (send path).
- Downgrade fail-closed rules: see DOC-CAN-003 §2 (CAT-S2-DOWNGRADE-001 basis).
- Transcript/AD binding expectations: see DOC-CAN-003 §5.1.

NA-0003 is docs-only; existing vector JSON files remain valid. This README update records the normative cross-reference so core-protocol doc edits are coupled to vector documentation (fail-closed governance requirement).
## Spec alignment (NA-0004)

Goals: G2, G4

This vectors directory is also aligned to the canonical SCKA specification:

docs/canonical/DOC-CAN-004_QSP_SCKA_Sparse_Continuous_Key_Agreement_v1.0.0_DRAFT.md

- ADV monotonicity and reject behavior: see DOC-CAN-004 §3.2 and §4 (CAT-SCKA-LOGIC-001).
- One-time targeting + tombstones: see DOC-CAN-004 §3.4 and §4 (CAT-SCKA-LOGIC-001).
- Deterministic ML-KEM-768 fixtures and invalid input size rejects: see DOC-CAN-004 §3.3–§3.4 (CAT-SCKA-KEM-001).
- Transactional commit rule for SCKA state: see DOC-CAN-004 §3.4 and §5, and Suite-2 commit rule in DOC-CAN-003 §8.2.
## Category expansion (NA-0005)

Goals: G4

DOC-TST-005 now includes protocol-level composition categories (CAT-S2-TRANSCRIPT-001, CAT-S2-HYBRID-001, CAT-S2-PQRESEED-001, CAT-S2-OOO-REPLAY-001, CAT-S2-DUR-001). New vector packs should be added under this directory using the standard vector-set schema once harness ops exist, and should be wired into suite2-ci / qshield-ci fail-closed gates.
