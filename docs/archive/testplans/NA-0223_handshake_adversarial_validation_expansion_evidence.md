Status: Archive
Owner: QSL governance
Last-Updated: 2026-04-06

# NA-0223 Handshake Adversarial Validation Expansion Evidence

Goals: G4, G5

## Scope

- Repo: `qsl-protocol`
- Lane: `NA-0223`
- Posture: implementation already merged; this closeout is governance-only and introduces no runtime changes
- Implementation PR: #667
- Implementation branch head before merge: `870bd0a925c8`
- Implementation merge SHA: `05effb2d1d4d`
- Implementation mergedAt: `2026-04-06T03:39:36Z`

## Authority Proof

- refreshed `qsl-protocol` checkout `HEAD`, `main`, `mirror/main`, and `origin/main` all resolved to `05effb2d1d4d`
- refreshed merged main contains `DECISIONS.md` `D-0382`, the `TRACEABILITY.md` `NA-0223 implementation/evidence` entry, and the merged adversarial/property/Miri test surfaces from PR #667
- refreshed live queue still showed `READY_COUNT=1` with `NA-0223` as the sole live `READY` item before this closeout
- refreshed sibling-repo proof stayed read-only and truthful: `qsl-server READY=0`, `qsl-attachments READY=0`
- PR `#667` is already `MERGED`; refreshed merged `main` is authoritative for queue truth, and this closeout lane does not mutate that PR

## Implementation Evidence Source On Main

- `DECISIONS.md` `D-0382`
- `TRACEABILITY.md` `NA-0223 implementation/evidence`
- `qsl/qsl-client/qsc/tests/adversarial_properties.rs`
- `qsl/qsl-client/qsc/tests/adversarial_miri.rs`
- `qsl/qsl-client/qsc/tests/handshake_mvp.rs`
- `qsl/qsl-client/qsc/tests/qsp_protocol_gate.rs`

## Exact Implementation Summary

- handshake adversarial validation now expands beyond directed regressions
- no-mutation invariants are covered in both the property and Miri adversarial lanes
- midpoint honest-gating and replay/no-mutation canaries are extended on the direct handshake seam

## Acceptance-Proof Surface

- adversarial/property/Miri coverage expanded meaningfully beyond the earlier directed regressions
- `NA-0221` fail-closed no-mutation behavior remained green
- `NA-0222` status/marker honesty behavior remained green where touched
- no runtime surfaces outside the approved adversarial/test seam changed

## Implementation / CI Nuance Summary

- the implementation landed on PR #667 from refreshed `main`
- the existing adversarial workflow and script were sufficient; no workflow or script change was needed
- no new serious issue was found in the expansion
- protected CI completed green before merge
- this closeout PR is governance-only and introduces no runtime changes

## Closeout Note

- `NA-0223` is now closed truthfully because its merged implementation/evidence state is already durable on refreshed `main`
- the next truthful successor is `NA-0224 — qsc Modularization / File-Size Reduction Plan Refresh`
