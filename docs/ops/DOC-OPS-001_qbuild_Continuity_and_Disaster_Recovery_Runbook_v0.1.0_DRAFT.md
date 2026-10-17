# DOC-OPS-001_qbuild_Continuity_and_Disaster_Recovery_Runbook_v0.1.0_DRAFT: current use

Goals: G1, G2, G3, G4, G5

Revision: 2026-10-13. This current procedure replaces the older process text under the operator-approved project method.

Read [the project workflow](../../PROJECT_WORKFLOW.md).

Restore current workflow, single TASK, hardening ledger, source branches/dirty work, evidence and load-bearing mirror objects together. Compare exact source/remote/configuration and consumed-task state before resuming. Never reset unfinished work to a historical pin. Use the managed build root lifetime layout; legacy /srv/qbuild paths are historical. Off-host continuity requires an actual independently verified copy or GitHub branch; local/platter save alone does not prove it. Credentials are restored privately, never in Git/public packages.


The [complete prior document](https://github.com/QuantumShieldLabs/qsl-protocol/blob/65059fad067119c54528e5c9b56b65e1f00720d3/docs/ops/DOC-OPS-001_qbuild_Continuity_and_Disaster_Recovery_Runbook_v0.1.0_DRAFT.md) is an immutable historical snapshot; its old procedural instructions do not govern new work. Its accepted product requirements remain subject to the current roadmap and later accepted amendments.
