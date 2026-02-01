# NA-0084 — qsc send semantics plan

## Scope & assumptions
- qsl/qsl-client/qsc only.
- No protocol-core changes.
- Explicit transport selection required for sending.

## CLI contract (flags, required args)
- `qsc send` requires explicit transport selection (e.g., `--transport relay --relay <url>`).
- `qsc relay send` remains a transport-specific tool; `qsc send` is the primary user-facing sender.

## Lifecycle markers (prepare/attempt/commit)
- prepare → attempt → commit
- failure must not advance to commit

## Test vectors (happy/fail/outbox recovery)
- Happy-path: local relay serve + `qsc send` with explicit relay transport.
- Failure path: unreachable relay; verify no commit and deterministic failure markers.
- Outbox recovery: trigger outbox_exists, run `qsc send abort`, then resend.
- No-secrets grep guard on output.

## Verification checklist
- `cargo test -p qsc --locked`
- `cargo clippy -p qsc --all-targets -- -D warnings`
- Help text clarifies send vs relay send.

## Rollback
- Revert CLI contract changes and tests.
