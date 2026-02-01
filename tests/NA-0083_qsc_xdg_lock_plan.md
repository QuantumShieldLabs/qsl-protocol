# NA-0083 — qsc XDG lock correctness plan

## Scope & assumptions
- Applies to qsl/qsl-client/qsc only.
- No protocol-core changes.
- Uses existing safe-parent checks.

## Expected lock/store path resolution rules
- If XDG_CONFIG_HOME is set, lock path is under:
  - $XDG_CONFIG_HOME/qsc/.qsc.lock
- If XDG_CONFIG_HOME is unset, fall back to:
  - $HOME/.config/qsc/.qsc.lock

## Error code mapping table (errno → marker)
- EACCES/EPERM/ENOENT on open/create: lock_open_failed
- EWOULDBLOCK/EAGAIN on flock: lock_contended

## Test vectors
- XDG override respected (lock path under XDG).
- Permission denied on lock path emits lock_open_failed.
- Contention emits lock_contended (non-blocking flock).

## Verification checklist
- cargo test -p qsc --locked
- cargo clippy -p qsc --all-targets -- -D warnings
- No secrets in marker output

## Rollback
- Revert XDG selection changes and lock error mapping update.
