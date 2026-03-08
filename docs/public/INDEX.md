# Public Docs Canon (Authoritative Public/Release Posture)

Status: Authoritative  
Owner: QSL maintainers  
Last-Updated: 2026-03-08

## Purpose
This page is the canonical public/release posture for QSL.

Use this page to understand what QSL currently claims, what it does not claim, and where to find supporting policy docs.

## Current posture
- QSL is research-stage and not production-ready.
- Public docs are maintained for transparency and review.
- Security posture is fail-closed by default where trust/identity/routing rules apply.

## Public-safe demo posture
- Demo guidance should remain safety-first and non-secret-bearing.
- Use placeholders in documentation:
  - `<RELAY_ENDPOINT>`
  - `<TOKEN_FILE>`
  - `<ROUTE_TOKEN>`
  - `<DEVICE_ID_12>`

## Trust Model v2 (current behavior)
- Trust is per device.
- `VERIFIED` means identity/code matched expected binding.
- `TRUSTED` means send-authorized.
- `CHANGED` and `REVOKED` are fail-closed send blockers until operator remediation.

## Truthful delivery semantics
- `accepted_by_relay`: relay accepted transport operation.
- `peer_confirmed`: valid end-to-end confirmation processed.
- Do not claim peer delivery when only relay acceptance is known.

## Receipt policy modes
- `off`: no receipts emitted.
- `batched`: bounded batched receipt emission.
- `immediate`: prompt receipt emission within bounded flow.

## Multi-device policy (current)
- Routing policy is `primary_only`.
- `peer_confirmed` is bound to the targeted device under `primary_only`.
- Fanout is not the default behavior in the current phase.

## Related public references
- `README.md`
- `SECURITY.md`
- `SUPPORT.md`
- `CONTRIBUTING.md`
- `CODE_OF_CONDUCT.md`
- `THIRD_PARTY_NOTICES.md`

## Historical public docs (deprecated)
The following files are retained as compatibility stubs and should not be treated as canonical:
- `docs/public/PUBLIC_RELEASE_RUNBOOK.md`
- `docs/public/PUBLIC_ALLOWLIST_INVENTORY.md`
- `docs/public/PUBLIC_EXPORT_MANIFEST.md`
- `docs/public/PUBLIC_WORKSPACE_AND_NAMING.md`
