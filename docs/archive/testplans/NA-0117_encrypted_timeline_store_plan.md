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
- Add vault-backed timeline secret (`timeline.json`) with deterministic `next_ts` counter and per-peer append-only entries.
- Add CLI surface:
  - `qsc timeline list --peer <label> [--limit N]`
  - `qsc timeline show --peer <label> --id <id>`
  - `qsc timeline clear --peer <label> --confirm`
- Ingest timeline entries on successful receive (`dir=in`, `status=received`) and successful send commit (`dir=out`, `status=sent`).
- Keep reject paths non-mutating for timeline state.

## Test vectors
- receive success writes timeline entry.
- receive reject/tamper path keeps timeline unchanged.
- send commit writes timeline entry; send failure/no commit does not.
- encrypted-at-rest negative checks for plaintext leakage.
- clear requires explicit confirm.
- no secrets in timeline outputs.

## Verification checklist
- `cargo fmt -p qsc -- --check`
- `cargo test -p qsc --locked`
- `cargo clippy -p qsc --all-targets -- -D warnings`

## Executed evidence
- Added `qsl/qsl-client/qsc/tests/timeline_store.rs` with:
  - `timeline_written_on_receive_success`
  - `timeline_not_written_on_receive_reject_no_mutation`
  - `timeline_written_on_send_commit_only`
  - `timeline_is_encrypted_at_rest`
  - `timeline_clear_requires_confirm_no_mutation`
  - `no_secrets_in_timeline_output`
- Updated timeline implementation in `qsl/qsl-client/qsc/src/main.rs`:
  - vault-backed timeline store/load/save helpers
  - timeline CLI handlers (`list`, `show`, `clear`)
  - ingestion hooks in receive success and send commit paths
- Gates executed (isolated cache/evidence dir):
  - `cargo fmt -p qsc -- --check` PASS
  - `cargo test -p qsc --locked` PASS
  - `cargo clippy -p qsc --all-targets -- -D warnings` PASS

## Rollback
- Revert timeline-store changes if at-rest protection or reject semantics regress.
