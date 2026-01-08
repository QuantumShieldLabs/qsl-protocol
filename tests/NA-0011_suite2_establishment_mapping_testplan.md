# NA-0011 — Suite-2 session establishment mapping (coverage note)

This file exists to satisfy the repository’s goal-lint coupling rule:
when canonical “core protocol path” documents change, a corresponding
test/vector/harness artifact must also change.

NA-0011 is a docs-only change set that:
- Adds Suite-2 session establishment and negotiation mapping to
  DOC-CAN-003 (new §6) and registers establishment reject identifiers
  in DOC-SCL-002.
- Does not introduce executable behavior changes on its own.

Executable coverage is intentionally deferred to NA-0012, which is
BLOCKED on NA-0011 and will implement Suite-2 session establishment
in the actors/harness without changing Suite-1/Suite-1B behavior.

References:
- docs/canonical/DOC-CAN-003_QSP_Suite-2_True_Triple_Ratchet_v5.0.0_DRAFT.md (§6)
- docs/spec-closure/DOC-SCL-002_Shared_Schemas_Error_Reason_Code_Registry_v1.0_DRAFT.md
- NEXT_ACTIONS.md: NA-0011 / NA-0012
