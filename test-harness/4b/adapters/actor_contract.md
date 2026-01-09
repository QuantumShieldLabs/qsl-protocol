# QuantumShield Phase 4B — Interop Actor Contract (QSHIELD-4B-ACTOR-1)

An "actor" is a command invoked by the 4B harness to execute one side of a P3-04 interop case.

## Invocation
The harness runs the actor command with a single JSON object on STDIN.

The actor MUST:
- Read STDIN fully
- Emit exactly one JSON object on STDOUT
- Exit code 0 for "well-formed response" (pass or fail); non-zero is treated as adapter failure.

## Input JSON (from harness)
Fields:
- contract: "QSHIELD-4B-ACTOR-1"
- actor: "A" or "B"
- peer:  "B" or "A"
- case_id: e.g. "IT-HS-001"
- suite_id: 1 (Suite-1) or 2 (Suite-1B)
- roles: "A->B" or "B->A"
- services: { rsf_url, pds_url, ktl_url }
- case_dir: unique per case+suite+direction
- state_dir: private to this actor instance
- xfer_dir: shared directory for file-based handoff between actors
- phase2_zip: optional path to frozen Phase2 zip
- phase3_zip: path to frozen Phase3 zip
- run_id, git_commit

## Output JSON (to harness)
Required:
- ok: boolean

Optional:
- failure_stage: e.g. "adapter", "setup", "hs", "msg", "ratchet", "kt", "qse", "opk", "policy"
- notes: string
- evidence: object (paths relative to case_dir recommended)
