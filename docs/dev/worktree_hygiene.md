# worktree_hygiene: current use

Goals: G1, G2, G3, G4, G5

Revision: 2026-10-13. This current procedure replaces the older process text under the operator-approved project method.

Read [the project workflow](../../PROJECT_WORKFLOW.md).

Use isolated branches for implementation and exact detached refs where appropriate for review. Preserve dirty/untracked work; never pin main in a second checkout or silently reset an existing one. Durable source/evidence belongs in lane/state roots; generated output in caches. Remove/prune only proven unneeded worktrees under explicit cleanup authority, preserving load-bearing object alternates and evidence.


The [complete prior document](https://github.com/QuantumShieldLabs/qsl-protocol/blob/65059fad067119c54528e5c9b56b65e1f00720d3/docs/dev/worktree_hygiene.md) is an immutable historical snapshot; its old procedural instructions do not govern new work. Its accepted product requirements remain subject to the current roadmap and later accepted amendments.
