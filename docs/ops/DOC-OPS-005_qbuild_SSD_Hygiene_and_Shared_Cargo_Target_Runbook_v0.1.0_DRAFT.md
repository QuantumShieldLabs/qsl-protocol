# DOC-OPS-005_qbuild_SSD_Hygiene_and_Shared_Cargo_Target_Runbook_v0.1.0_DRAFT: current use

Goals: G1, G2, G3, G4, G5

Revision: 2026-10-13. Legacy topology and maintenance activation procedure retired.

Read [the project workflow](../../PROJECT_WORKFLOW.md).

The old /srv/qbuild maintenance subject is retired. Current managed layout is state/lanes/mirrors/cache/targets, with exact pinned dependency caches and scoped private evidence. Read the current build-root README and state/tools/units reference for existing maintenance contracts. No cleanup, archive, privilege, timer/installation or fallback acts are authorized here. Source/build work preserves unrelated files and load-bearing mirrors; ordinary caches are not artifact provenance.

The [complete prior document](https://github.com/QuantumShieldLabs/qsl-protocol/blob/65059fad067119c54528e5c9b56b65e1f00720d3/docs/ops/DOC-OPS-005_qbuild_SSD_Hygiene_and_Shared_Cargo_Target_Runbook_v0.1.0_DRAFT.md) is an immutable historical snapshot; its old procedural instructions do not govern new work. Its accepted product requirements remain subject to the current roadmap and later accepted amendments.
