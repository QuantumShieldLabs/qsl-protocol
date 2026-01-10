Goals: G4, G5
Status: DRAFT

Scope:
- Issue #8 (OPK invariant encoding in HandshakeInit) regression guard.

Assertions (CI-gated):
- HandshakeInit::encode returns fail-closed output when opk_used=true but required OPK fields are missing.

Evidence:
- Unit test: handshake_init_encode_fails_closed_on_missing_opk_fields in tools/refimpl/quantumshield_refimpl/src/qsp/types.rs.
