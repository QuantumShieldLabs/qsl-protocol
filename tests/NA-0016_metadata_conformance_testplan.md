Goals: G5

# NA-0016 — Metadata conformance smoke (PR1)

This file exists to satisfy goal-lint coupling for NA-0016 PR1.

Scope (demo-only):
- Enforces safe-by-default relay/CLI behavior via CI smoke:
  - token required for /register, /send, /poll, /bundle
  - loopback-only binding unless explicitly overridden
  - bounded request sizes and queue caps

Artifacts:
- scripts/ci/metadata_conformance_smoke.sh
- .github/workflows/ci.yml (metadata-conformance-smoke job)

Authoritative references:
- docs/privacy/DOC-G5-001_Metadata_Threat_Model_v1.0.0_DRAFT.md
- docs/privacy/DOC-G5-002_Metadata_Leakage_Inventory_v1.0.0_DRAFT.md
- docs/privacy/DOC-G5-003_Envelope_Transport_Profile_v0.1.0_DRAFT.md (§8 Conformance invariants)

Trace mapping:
- Token required (register/send/poll/bundle) -> DOC-G5-003 §3; DOC-G5-002 table (Token presence)
- Loopback-only default -> DOC-G5-003 §4; DOC-G5-002 table (Relay bind address)
- Request size limit -> DOC-G5-003 §5; DOC-G5-002 table (Request body size limit)
- Queue caps -> DOC-G5-003 §8; DOC-G5-002 table (Queue length/overflow)
- Store permissions (0700/0600) -> DOC-G5-003 §8; DOC-G5-002 table (Store file permissions)
- Demo unauthenticated override explicit/off by default -> DOC-G5-003 §8
- Padding bucket behavior (when enabled) -> DOC-G5-003 §6/§8; DOC-G5-002 table (Message size / Padding bucket)

Notes:
- Non-production demo only; no protocol-core changes; Suite-1/1B unaffected.
