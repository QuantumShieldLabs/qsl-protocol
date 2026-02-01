# NA-0082 — qsc doctor clarity plan

## Scope & assumptions
- Scope: qsl/qsl-client/qsc/** only; no protocol-core changes.
- `qsc doctor` output must remain safe-to-share and deterministic.

## Marker schema additions (exact fields)
- checked_dir=<path>
- dir_writable_required=true|false
- dir_writable=true|false
- reason=<string> (only if needed; stable vocabulary)

## Test vectors
- Writable dir: checked_dir set; dir_writable=true; dir_writable_required=false/true as appropriate.
- Read-only dir: checked_dir set; dir_writable=false; dir_writable_required clarifies requirement.

## No-secrets checks
- Grep guard for TOKEN|SECRET|KEY|PASS|PRIVATE|BEARER|CREDENTIAL in doctor output.

## Verification checklist
- cargo test -p qsc --locked
- cargo clippy -p qsc --all-targets -- -D warnings

## Rollback
- Revert marker field additions and test updates if they regress determinism or safety.
