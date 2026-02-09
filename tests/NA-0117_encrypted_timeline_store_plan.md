# NA-0117 Encrypted Timeline Store Plan

## Scope and assumptions
- Scope limited to `qsl/qsl-client/qsc/**`.
- Timeline persistence is encrypted at rest and client-local.

## Threat model notes
- Plaintext conversation leakage on disk.
- Tampered timeline artifacts causing silent state drift.

## Must-never list
- Must never write plaintext message content to disk.
- Must never accept tampered timeline blobs.
- Must never mutate timeline state on reject paths.

## Proposed design
- Add encrypted timeline store/list/view with bounded retention knobs.
- Add deterministic ordering and dedupe strategy.

## Test vectors
- encrypted-at-rest verification.
- tamper reject with no mutation.
- deterministic timeline ordering.
- bounded retention behavior.

## Verification checklist
- `cargo fmt -p qsc -- --check`
- `cargo test -p qsc --locked`
- `cargo clippy -p qsc --all-targets -- -D warnings`

## Rollback
- Revert timeline-store changes if at-rest protection or reject semantics regress.
