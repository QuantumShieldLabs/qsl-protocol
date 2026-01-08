Goals: G4

# QuantumShield Documentation Tree (`docs/`)

This directory is the canonical, version-controlled home for QuantumShield documentation.

Authoritative navigation:
- docs/master/DOC-CTRL-001_Documentation_Master_Index_Release_Packet_v1.0.1_DRAFT.md
- START_HERE.md (repo root) for authoritative source order and workflow

## Layout

- `master/` — Program control docs (master index, cross-doc errata)
- `canonical/` — Canonical specs stored in-repo (Suite-2 + SCKA).
  Phase-2 QSP/QSE remain frozen and are referenced via pointers under `_external/`.
- `spec-closure/` — Spec-closure pack (registry, conformance, persistence, etc.)
- `schemas/` — Shared JSON registries/schemas referenced by docs and tooling
- `public/` — Public-facing materials (whitepaper, architecture overview)
- `_external/` — Large external reference bundles (kept out of git by default)

## Source

Built from:
- `qsl_docs_updates_dur6.zip` (updated markdown docs pack)
- `DOC-SCL-002_Shared_Schemas_v1.0.json`
- `DOC-SCL-002_Reason_Codes_v1.0.json`

Build date: 2025-12-27
