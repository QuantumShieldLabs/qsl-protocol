# NA-0119 File Transfer MVP Plan

## Scope and assumptions
- Scope limited to `qsl/qsl-client/qsc/**`.
- MVP file transfer remains bounded, integrity-checked, and fail-closed.

## Threat model notes
- Oversize payload resource exhaustion.
- Tampered file payload acceptance.

## Must-never list
- Must never process unbounded file payloads.
- Must never accept failed integrity verification.
- Must never mutate receive state on tamper/oversize reject.

## Proposed design
- Add bounded send/receive file transfer with hash/MAC verification.
- Emit deterministic markers for accept/reject outcomes.

## Test vectors
- tamper reject/no mutation.
- oversize reject/no mutation.
- bounded memory behavior.
- deterministic marker replay.

## Verification checklist
- `cargo fmt -p qsc -- --check`
- `cargo test -p qsc --locked`
- `cargo clippy -p qsc --all-targets -- -D warnings`

## Rollback
- Revert file-transfer MVP changes if bounded-integrity invariants regress.
