# NA-0094 qsc protocol gate plan

## Scope & assumptions
- qsc send/receive must refuse unless QSP/QSE status ACTIVE.
- No insecure override flags.

## Truth table
- ACTIVE => send/receive proceed.
- INACTIVE (missing_seed/unsafe_parent/pack_failed/unpack_failed) => refuse.

## Test vectors
- send_refuses_when_protocol_inactive (missing seed)
- receive_refuses_when_protocol_inactive (missing seed)
- send_allows_when_active (seed set)
- receive_allows_when_active (seed set + inbox item)

## No-mutation checks
- refuse path must not create outbox or advance state.

## No-secrets checks
- outputs contain no secrets.

## Verification checklist
- deterministic error marker code=protocol_inactive reason=<explicit>
- READY gate enforced for both send and receive

## Rollback
- revert protocol gate checks
