# NA-0116 Contacts Verify Block Plan

## Scope and assumptions
- Scope limited to `qsl/qsl-client/qsc/**`.
- Contacts trust state is managed client-side only.

## Threat model notes
- Silent trust escalation and undetected fingerprint mismatch.
- Inconsistent or non-deterministic block behavior.

## Must-never list
- Must never silently trust unverified peer updates.
- Must never accept mismatch without explicit error state.
- Must never mutate trust state on reject paths.

## Proposed design
- Add contacts add/remove/list/verify/block flows.
- Surface pin and mismatch status in TUI.
- Keep mismatch/block transitions deterministic.

## Test vectors
- pinning and verify happy path.
- mismatch reject with no mutation.
- block denies interaction deterministically.
- no secrets in outputs.

## Verification checklist
- `cargo fmt -p qsc -- --check`
- `cargo test -p qsc --locked`
- `cargo clippy -p qsc --all-targets -- -D warnings`

## Rollback
- Revert contacts-state changes if trust invariants or determinism regress.
