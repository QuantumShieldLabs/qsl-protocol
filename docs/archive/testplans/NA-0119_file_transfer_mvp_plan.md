# NA-0119 File Transfer MVP Plan

## Scope and assumptions
- Scope limited to `qsl/qsl-client/qsc/**`.
- File transfer uses existing qsp encrypted transport; no server/protocol wire changes.
- File-transfer payloads are encoded as deterministic control JSON (`file_chunk`/`file_manifest`) and processed in client receive path.

## Threat model notes
- Oversize payload resource exhaustion.
- Tampered chunk/manifest acceptance.
- Replay/duplicate chunk attempts that could corrupt transfer state.

## Must-never list
- Must never process unbounded file payloads.
- Must never accept failed integrity verification.
- Must never mutate timeline/session transfer state on tamper/oversize/replay reject.
- Must never persist plaintext transfer store outside vault-backed secrets.

## Proposed design
- Add explicit `qsc file send` with bounded chunking and deterministic file id.
- Enforce sender bounds: `max_file_size`, `chunk_size`, `max_chunks`.
- Emit deterministic markers:
  - `file_xfer_prepare`
  - `file_xfer_chunk`
  - `file_xfer_manifest`
  - `file_xfer_complete`
  - `file_xfer_reject reason=<explicit>`
- Receive path:
  - Parse file transfer control payloads after `qsp_unpack`.
  - Verify per-chunk hash and strict in-order chunk index.
  - Verify manifest hash bound to `peer|file_id|total_size|chunk_count|chunk_hashes`.
  - Append timeline entry `kind=file` only on fully verified transfer.
  - Reject path is fail-closed with no session/timeline mutation.

## Test vectors
- `bounds_reject_file_too_large_no_mutation`
- `file_transfer_accepts_valid_chunks_and_manifest`
- `tampered_chunk_reject_no_mutation`
- `replay_chunk_reject_deterministic_no_mutation`
- `no_plaintext_and_marker_determinism`

## Verification checklist
- `cargo fmt -p qsc -- --check`
- `cargo test -p qsc --locked`
- `cargo clippy -p qsc --all-targets -- -D warnings`

## Executed evidence
- Added: `qsl/qsl-client/qsc/tests/file_transfer_mvp.rs`
- Extended: `qsl/qsl-client/qsc/src/main.rs` with file transfer command, bounded sender, receive verify path, and vault-backed transfer staging.
- Gate runs (all PASS):
  - `cargo fmt -p qsc -- --check`
  - `cargo test -p qsc --locked --test file_transfer_mvp`
  - `cargo test -p qsc --locked`
  - `cargo clippy -p qsc --all-targets -- -D warnings`

## Rollback
- Revert file-transfer MVP changes if bounded-integrity invariants regress.
