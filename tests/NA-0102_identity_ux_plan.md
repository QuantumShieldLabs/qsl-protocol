# NA-0102 Identity UX Plan

## Scope & assumptions
- Client-only changes in qsc.
- No new crypto; use existing identity fingerprints.

## CLI contract
- `qsc identity show`
- `qsc identity rotate --confirm [--invalidate-peers]`
- `qsc peers list`

## Deterministic markers
- identity_show fp=<fp> pinned=<true|false>
- identity_rotate ok=<true|false> reason=<...>
- peers_list count=<n>

## Test vectors
- Show emits deterministic fp and no secrets.
- Rotate without confirm rejects (no mutation).
- Rotate with confirm changes own fp.
- Peers list deterministic ordering and no secrets.

## No-secrets checks
- Guard against TOKEN|SECRET|KEY|PASS|PRIVATE|BEARER|CREDENTIAL in outputs.

## Verification checklist
- cargo fmt/test/clippy for qsc
- CI green
- Markers deterministic

## Rollback
- Revert CLI/TUI additions and tests.
