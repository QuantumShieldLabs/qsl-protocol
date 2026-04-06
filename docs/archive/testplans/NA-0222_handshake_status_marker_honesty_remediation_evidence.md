Status: Archive
Owner: QSL governance
Last-Updated: 2026-04-05

# NA-0222 Handshake Status / Marker Honesty Remediation Evidence

Goals: G4, G5

## Scope

- Repo: `qsl-protocol`
- Lane: `NA-0222`
- Posture: implementation already merged; this closeout is governance-only and introduces no runtime changes
- Implementation PR: #665
- Implementation branch head before merge: `a5f235b3d352`
- Implementation merge SHA: `1ad0875ebe30`
- Implementation mergedAt: `2026-04-05T21:51:46Z`

## Authority Proof

- refreshed `qsl-protocol` checkout `HEAD`, `main`, `mirror/main`, and `origin/main` all resolved to `1ad0875ebe30`
- refreshed merged main contains `DECISIONS.md` `D-0380`, the `TRACEABILITY.md` `NA-0222 implementation/evidence` entry, `qsl/qsl-client/qsc/src/handshake/mod.rs`, and the merged protected-test updates from PR #665
- refreshed live queue still showed `READY_COUNT=1` with `NA-0222` as the sole live `READY` item before this closeout
- refreshed sibling-repo proof stayed read-only and truthful: `qsl-server READY=0`, `qsl-attachments READY=0`
- PR `#665` is already `MERGED`; refreshed merged `main` is authoritative for queue truth, and this closeout lane does not mutate that PR

## Implementation Evidence Source On Main

- `DECISIONS.md` `D-0380`
- `TRACEABILITY.md` `NA-0222 implementation/evidence`
- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/tests/handshake_contract_na0217i.rs`
- `qsl/qsl-client/qsc/tests/handshake_mvp.rs`
- `qsl/qsl-client/qsc/tests/send_ready_markers_na0168.rs`
- `qsl/qsl-client/qsc/tests/desktop_gui_contract_na0215b.rs`

## Exact Implementation Summary

- local and peer-confirmed handshake progress is no longer overstated on the bounded qsc handshake seam
- midpoint status now distinguishes local commit from peer-confirmed completion via `status=awaiting_peer_confirm`
- marker truth remains compatibility-stable while adding explicit `peer_confirmed=yes|no` detail on `handshake_complete`

## Acceptance-Proof Surface

- local status/marker surfaces do not overstate authenticated or peer-confirmed handshake progress
- `NA-0221` fail-closed no-mutation behavior remains green
- desktop/marker regressions remain green where touched
- no runtime surfaces outside the approved handshake/output seam changed

## Implementation / CI Nuance Summary

- the implementation landed on PR #665 from refreshed `main`
- protected CI completed green before merge
- the merged protected runtime/test surfaces remain bounded to `qsl/qsl-client/qsc/src/handshake/mod.rs` plus the direct handshake, desktop, and send-ready regressions updated on that PR
- this closeout PR is governance-only and introduces no runtime changes

## Closeout Note

- `NA-0222` is now closed truthfully because its merged implementation/evidence state is already durable on refreshed `main`
- the next truthful successor is `NA-0223 — Handshake Adversarial Validation Expansion`
