Goals: G3, G5

# NA-0018 — Demo prekey/bundle lifecycle test plan

Status: DRAFT
Scope: Demo relay/CLI only. No protocol-core changes.

## Objective
- Enforce at-most-once bundle consumption after successful establish.
- Ensure reuse attempts reject deterministically without state mutation.
- Ensure failed establish does not consume bundles.

## CI-gated assertions
Enforced by: `scripts/ci/metadata_conformance_smoke.sh`

- Establish without unauthenticated override fails (no bundle consumption).
- Establish with override succeeds and consumes the peer bundle.
- GET `/bundle/<peer>` returns 404 after successful establish (consumed).
- Second establish attempt for the same peer fails deterministically.
- POST `/consume` without token rejects (401/403).
- POST `/consume` with token and missing bundle rejects (404).

## Evidence
- metadata-conformance-smoke CI job logs.
