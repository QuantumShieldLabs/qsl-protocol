# NA-0093 — QSC Truthful Protocol Status Plan

## Scope & assumptions
- Applies to qsc CLI + TUI status output.
- No network calls; no disk writes for self-check.

## Marker schema
- qsp_status status=<ACTIVE|INACTIVE> reason=<reason> version=<version>

## Test vectors
- Seeded (QSC_QSP_SEED set) => ACTIVE
- Missing seed => INACTIVE reason=missing_seed
- Unsafe parent => INACTIVE reason=unsafe_parent
- Determinism: exact marker string stable
- No-secrets guard

## Verification checklist
- CLI status markers include status + reason
- TUI Status shows same ACTIVE/INACTIVE and reason
- Tests cover all cases

## Rollback
- Revert status logic changes; restore previous status marker format

## Executed Evidence
- Tests: qsp_status_truthy.rs
  - status_seeded_is_active
  - status_missing_seed_reason_missing_seed
  - status_unsafe_parent_reason_unsafe_parent
  - status_no_secrets
- Local gates:
  - cargo fmt -p qsc -- --check
  - cargo test -p qsc --locked
  - cargo clippy -p qsc --all-targets -- -D warnings
