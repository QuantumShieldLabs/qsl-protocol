Goals: G5

Status: DRAFT

Objective:
- Establish a professional public-facing repo baseline with clear licensing, security, and contribution guidance.
- Ensure allowlist/export docs include the new public-facing root files.

CI-gated assertions:
- `docs/public/PUBLIC_EXPORT_MANIFEST.md` includes README.md, LICENSE, SECURITY.md, CONTRIBUTING.md, and THIRD_PARTY_NOTICES.md.
- `docs/public/PUBLIC_ALLOWLIST_INVENTORY.md` matches the manifest allowlist for root public files.

Manual verification:
- `README.md`, `LICENSE`, `SECURITY.md`, `CONTRIBUTING.md`, and `THIRD_PARTY_NOTICES.md` exist at repo root.
- Public workspace and naming doc exists at `docs/public/PUBLIC_WORKSPACE_AND_NAMING.md`.
