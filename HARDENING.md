# Hardening ledger location

Goals: G1, G2, G3, G4, G5

Revision: 2026-10-13. Read [the project workflow](PROJECT_WORKFLOW.md).

The one project hardening ledger is the Director-maintained HARDENING.md in the managed project record. This file is a location pointer, not a duplicate ledger. Record each actual cut there as cut/deferred, risk and mandatory trigger. Check it before issue/review and before crossing a gate; triggered work enters the same single task queue and blocks that boundary. Consult the current task for its exact location/access. A component checkout without that record cannot assume its debts are closed.
