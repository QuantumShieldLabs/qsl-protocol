# NA-0091 — Receive-path E2E plan

## CLI contract (args, required fields)
- `qsc receive --transport relay --relay <http(s)://host:port> --from <channel> --max <N> --out <DIR>`
- `--transport relay` required; `--relay` must be http/https for inbox.
- `--from` selects the inbox channel label (ascii alnum, '-' or '_').
- `--max` bounded (>0).
- `--out` required; safe-parent checks enforced.

## Receive marker schema
- `recv_start transport=relay from=<channel> max=<N>`
- `recv_item idx=<i> size=<bytes>`
- `recv_commit count=<k>`
- `recv_none`
- `event=error code=<deterministic>`

## E2E test vectors (two-way)
- Local embedded inbox server (PUSH/PULL, delete-on-deliver).
- Alice sends to channel `bob`, Bob receives from channel `bob`.
- Bob sends to channel `alice`, Alice receives from channel `alice`.

## TUI /receive behavior
- `/receive` uses relay from TUI config and default peer `peer-0`.
- Headless test asserts `event=tui_receive` and `event=recv_item`.

## No-mutation-on-failure checks
- Receive does not advance state unless `recv_commit` is emitted.

## No-secrets checks
- Test output must not contain: TOKEN|SECRET|KEY|PASS|PRIVATE|BEARER|CREDENTIAL.

## Verification checklist
- `cargo test -p qsc --locked`
- `cargo clippy -p qsc --all-targets -- -D warnings`

## Rollback
- Revert receive CLI additions and inbox HTTP helper functions.

## Executed evidence (implementation)
- Tests: `qsl/qsl-client/qsc/tests/receive_e2e.rs`
- Embedded inbox server: `qsl/qsl-client/qsc/tests/common/mod.rs`
