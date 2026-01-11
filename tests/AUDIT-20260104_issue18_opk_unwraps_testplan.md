# Audit Issue #18 — OPK unwrap panic removal testplan

Goals: G4, G5  
Status: DRAFT  
Date: 2026-01-11

Invariant:
- OPK handling must never panic on missing or invalid OPK fields.
- Rejects must be deterministic (same error code/message).
- No state mutation on reject: initiator_start performs no session-state mutation; input bundle bytes remain unchanged.

Tests:
- opk_partial_bundle_rejects_deterministically_and_no_mutation (tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs)
- opk_bundle_with_both_present_succeeds (tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs)
