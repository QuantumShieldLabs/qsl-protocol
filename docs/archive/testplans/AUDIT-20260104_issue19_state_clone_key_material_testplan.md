# Audit Issue #19 — Reduce SessionState cloning of key material testplan

Goals: G4, G5  
Status: DRAFT  
Date: 2026-01-11

Invariant:
- Ratchet paths must not clone SessionState in a way that duplicates secret key material.
- Rejects must be deterministic and must not mutate session state on failure.

Tests:
- ratchet_encrypt_rejects_deterministically_and_no_state_mutation (tools/refimpl/quantumshield_refimpl/src/qsp/ratchet.rs)
- ratchet_decrypt_rejects_deterministically_and_no_state_mutation (tools/refimpl/quantumshield_refimpl/src/qsp/ratchet.rs)
