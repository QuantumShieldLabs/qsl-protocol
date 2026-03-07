# NA-0108 — Remote Handshake Lane Plan

## Scope
- `.github/workflows/remote-handshake-tests.yml`
- `scripts/demo/qsc_remote_handshake_smoke.sh`
- `docs/qsc/DOC-QSC-006_Remote_Relay_Testing_Contract_v1.0.0_DRAFT.md`
- Governance tracking only (`NEXT_ACTIONS.md`, `DECISIONS.md`, `TRACEABILITY.md`)

## Objective
Add a manual/nightly remote lane that proves real handshake-established session behavior (no seed fallback) before bidirectional send/receive assertions.

## Invariants
1. No `QSC_ALLOW_SEED_FALLBACK` in this lane.
2. Handshake sequence is explicit and four-step (`init`, `poll`, `poll`, `poll`).
3. Both peers must be ACTIVE due to handshake before send/receive checks.
4. Fail closed on any `protocol_inactive`, `relay_unauthorized`, missing `qsp_pack ok=true`, missing `qsp_unpack ok=true`, or `recv_commit count=0`.
5. Workflow is `workflow_dispatch` + `schedule` only (no `pull_request` trigger).
6. Artifacts are safe-to-share and redact relay URL/token.

## Planned Verification
- Dispatch `remote-handshake-tests` for:
  - `happy-path` with seed `1`
  - `drop-reorder` with seed `7`
- Confirm artifact set contains:
  - `alice.log`, `bob.log`, `alice_recv.log`, `bob_recv.log`, `summary.txt`, `normalized_subset.txt`, `normalized_counts.txt`, `markers`
- Confirm summary/counts prove bidirectional `qsp_pack`/`qsp_unpack` success and non-zero receive commits.

## Executed Evidence
- Pending implementation PR and workflow run URLs.
