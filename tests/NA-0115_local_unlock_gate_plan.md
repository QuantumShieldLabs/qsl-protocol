# NA-0115 Local Unlock Gate Plan

## Scope and assumptions
- Scope limited to `qsl/qsl-client/qsc/**`.
- Local unlock only; no server-side presence signaling.

## Threat model notes
- Unauthorized local actions while vault/session state is locked.
- Secret leakage through errors when locked operations are attempted.

## Must-never list
- Must never allow send/receive/handshake/rotate while locked.
- Must never mutate state on lock rejects.
- Must never print secrets/tokens in lock error paths.

## Proposed design
- Add explicit unlock surface for CLI/TUI.
- Gate sensitive operations behind lock-state check.
- Emit deterministic reject marker: `event=error code=vault_locked`.

## Test vectors
- locked state rejects sensitive operations.
- unlock permits previously blocked operations.
- rejects are deterministic and non-mutating.
- no secrets in output.

## Verification checklist
- `cargo fmt -p qsc -- --check`
- `cargo test -p qsc --locked`
- `cargo clippy -p qsc --all-targets -- -D warnings`

## Rollback
- Revert unlock-gate changes if lock bypass, nondeterminism, or secret leakage appears.
