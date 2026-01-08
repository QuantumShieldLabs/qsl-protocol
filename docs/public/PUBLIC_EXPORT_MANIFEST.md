# Public Export Manifest (DRAFT)

Status: DRAFT

## Policy
Export is allowlist-only; denylist is always excluded.

## Allowlist (normative)
- docs/canonical/**
- docs/privacy/**
- docs/public/**
- inputs/suite2/vectors/**
- DECISIONS.md
- TRACEABILITY.md
- NEXT_ACTIONS.md
- README.md
- LICENSE
- SECURITY.md
- CONTRIBUTING.md
- THIRD_PARTY_NOTICES.md

## File List (as of commit dddcd4acd751cd409dc6120171a7c2e1e1646de1)
Command:
  git ls-files     'docs/canonical/**'     'docs/privacy/**'     'docs/public/**'     'inputs/suite2/vectors/**'     'DECISIONS.md'     'TRACEABILITY.md'     'NEXT_ACTIONS.md'     'README.md'     'LICENSE'     'SECURITY.md'     'CONTRIBUTING.md'     'THIRD_PARTY_NOTICES.md'

CONTRIBUTING.md
DECISIONS.md
LICENSE
NEXT_ACTIONS.md
README.md
SECURITY.md
THIRD_PARTY_NOTICES.md
TRACEABILITY.md
docs/canonical/DOC-CAN-003_QSP_Suite-2_True_Triple_Ratchet_v5.0.0_DRAFT.md
docs/canonical/DOC-CAN-004_QSP_SCKA_Sparse_Continuous_Key_Agreement_v1.0.0_DRAFT.md
docs/privacy/DOC-G5-001_Metadata_Threat_Model_v1.0.0_DRAFT.md
docs/privacy/DOC-G5-002_Metadata_Leakage_Inventory_v1.0.0_DRAFT.md
docs/privacy/DOC-G5-003_Envelope_Transport_Profile_v0.1.0_DRAFT.md
docs/public/PUBLIC_ALLOWLIST_INVENTORY.md
docs/public/PUBLIC_EXPORT_MANIFEST.md
docs/public/PUBLIC_RELEASE_RUNBOOK.md
docs/public/PUBLIC_WORKSPACE_AND_NAMING.md
inputs/suite2/vectors/README.md
inputs/suite2/vectors/qshield_suite2_boundary_vectors_v1.json
inputs/suite2/vectors/qshield_suite2_crash_restart_vectors_v1.json
inputs/suite2/vectors/qshield_suite2_downgrade_vectors_v1.json
inputs/suite2/vectors/qshield_suite2_e2e_recv_vectors_v1.json
inputs/suite2/vectors/qshield_suite2_establish_vectors_v1.json
inputs/suite2/vectors/qshield_suite2_interop_vectors_v1.json
inputs/suite2/vectors/qshield_suite2_interop_ximpl_vectors_v1.json
inputs/suite2/vectors/qshield_suite2_kdf_vectors_v1.json
inputs/suite2/vectors/qshield_suite2_mk_hybrid_vectors_v1.json
inputs/suite2/vectors/qshield_suite2_ooo_replay_vectors_v1.json
inputs/suite2/vectors/qshield_suite2_parse_vectors_v1.json
inputs/suite2/vectors/qshield_suite2_pq_reseed_vectors_v1.json
inputs/suite2/vectors/qshield_suite2_scka_kem_vectors_v1.json
inputs/suite2/vectors/qshield_suite2_scka_logic_vectors_v1.json
inputs/suite2/vectors/qshield_suite2_transcript_vectors_v1.json

## Known exclusions (denylist examples)
- .env
- *.pem
- *.key
- *.p12
- *secrets*
- *credentials*
- Operational configs, tokens, internal endpoints, private infrastructure
- Anything outside the allowlist
