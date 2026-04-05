Status: Archive
Owner: QSL governance
Last-Updated: 2026-04-05

# NA-0221 Authenticated-Establishment Fail-Closed Remediation Evidence

Goals: G4, G5

## Scope

- Repo: `qsl-protocol`
- Lane: `NA-0221`
- Posture: implementation already merged; this closeout is governance-only and introduces no runtime changes
- Implementation PR: #663
- Implementation branch head before merge: `000eb8376b06`
- Implementation merge SHA: `e369d65bb1f6`
- Implementation mergedAt: `2026-04-05T19:45:04Z`

## Authority Proof

- refreshed `qsl-protocol` checkout `HEAD`, `main`, `mirror/main`, and `origin/main` all resolved to `e369d65bb1f6`
- refreshed merged main contains `DECISIONS.md` `D-0378`, the `TRACEABILITY.md` `NA-0221 implementation/evidence` entry, `qsl/qsl-client/qsc/src/handshake/mod.rs`, and the merged protected-test updates from PR #663
- refreshed live queue still showed `READY_COUNT=1` with `NA-0221` as the sole live `READY` item before this closeout
- refreshed sibling-repo proof stayed read-only and truthful: `qsl-server READY=0`, `qsl-attachments READY=0`
- PR `#663` is already `MERGED`; refreshed merged `main` is authoritative for queue truth, and this closeout lane does not mutate that PR

## Implementation Evidence Source On Main

- `DECISIONS.md` `D-0378`
- `TRACEABILITY.md` `NA-0221 implementation/evidence`
- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/tests/handshake_contract_na0217i.rs`
- `qsl/qsl-client/qsc/tests/handshake_mvp.rs`
- `qsl/qsl-client/qsc/tests/handshake_security_closure.rs`
- `qsl/qsl-client/qsc/tests/identity_binding.rs`
- `qsl/qsl-client/qsc/tests/identity_foundation_contract_na0217d.rs`
- `qsl/qsl-client/qsc/tests/identity_secret_at_rest.rs`
- `qsl/qsl-client/qsc/tests/send_ready_markers_na0168.rs`
- `qsl/qsl-client/qsc/tests/receive_e2e.rs`

## Exact Implementation Summary

- fail-closed reject now occurs before `hs_pending_store(...)` / `qsp_session_store(...)` when authenticated peer identity is absent
- truthful authenticated-establishment state is now passed into session construction instead of being hardcoded
- first-contact TOFU establishment is retired across the currently known protected Suite-2 surfaces on this path: initiator, responder, legacy identity-migration, send-ready bootstrap, receive/bootstrap, and route-only handshake-canary expectations

## Acceptance-Proof Surface

- unknown or unpinned establishment attempts now reject with zero pending/session mutation
- pinned mismatch and transcript-tamper regressions remain green
- known-peer and verification-code authenticated establishment paths remain green
- no runtime surfaces outside the approved handshake seam changed

## Implementation / CI Nuance Summary

- the implementation landed from refreshed `main` on superseding PR #663
- stale PR #660 was not authoritative and was superseded cleanly from refreshed `main`
- protected CI completed green before merge
- this closeout PR is governance-only and introduces no runtime changes

## Closeout Note

- `NA-0221` is now closed truthfully because its merged implementation/evidence state is already durable on refreshed `main`
- the next truthful successor is `NA-0222 — Handshake Status / Marker Honesty Remediation`
