# NA-0109 — Session/ratchet state at rest: encrypt + integrity-check + legacy migration

Status: READY

## Scope & assumptions
- In scope: `qsl/qsl-client/qsc/**` session/ratchet state persistence and load paths only.
- Out of scope: server, workflow, refimpl, and protocol wire-format changes.
- Assumption: existing vault/secret access path is authoritative for local protected-at-rest material.
- Assumption: deterministic markers/error codes remain the required external contract for fail-closed behavior.

## Storage design (what is encrypted; where key comes from)
- Encrypt all session/ratchet-at-rest payload fields that can reveal current or future traffic secrets, including:
  - session/ratchet keys (send/recv chain and derived state)
  - ratchet counters/indexes linked to active key evolution
  - any other secret-bearing session state required for resume.
- Persist encrypted blob only; no plaintext key material in serialized session files.
- Encryption key material is derived from or unwrapped by vault-backed secret handling already used by qsc secure storage.
- File layout/versioning includes explicit format version to enable deterministic migration handling.

## Integrity model (MAC/AEAD; deterministic failure markers)
- Use authenticated encryption (AEAD) or encryption + strong MAC over full protected payload + required associated metadata.
- Load path must verify integrity before deserialization/use.
- On integrity failure or malformed blob:
  - deterministic reject marker/error code (fail-closed),
  - no state mutation,
  - no fallback to plaintext decode in production paths.
- Marker/output hygiene: no plaintext secrets, keys, or decrypted blobs in markers/logs.

## Migration plan (legacy detection; rollback)
- Detect legacy plaintext session file format deterministically.
- Migration precondition: vault/secret available.
- If vault/secret available:
  - migrate legacy plaintext to encrypted format atomically,
  - preserve correctness on restart,
  - repeat invocation is idempotent (no duplicate/unstable rewrites).
- If vault/secret unavailable:
  - do not mutate legacy file,
  - emit deterministic `migration_blocked` marker/error,
  - remain fail-closed for ACTIVE/send/receive.
- Rollback path:
  - retain ability to restore from pre-migration backup copy in test harness and documented operator procedure.

## Test vectors
- Plaintext scan in sandbox:
  - create/update session state, then assert no known secret fields appear plaintext on disk artifacts.
- Tamper session blob:
  - flip bytes/tag/counter in stored encrypted state,
  - expect deterministic reject marker/error and no mutation.
- Vault unavailable:
  - simulate unavailable vault/secret provider,
  - expect deterministic refusal for ACTIVE/send/receive and no mutation.
- Migration idempotent:
  - start from legacy plaintext fixture,
  - run migration twice and verify stable encrypted result + expected marker behavior.

## Verification checklist
- `cargo fmt -p qsc -- --check`
- `cargo test -p qsc --locked`
- `cargo clippy -p qsc --all-targets -- -D warnings`
- Added/updated tests prove:
  - no plaintext session/ratchet key material on disk,
  - tamper reject + no mutation,
  - vault unavailable deterministic refuse + no mutation,
  - migration idempotent and safe.

## Rollback
- Revert NA-0109 implementation commit(s) if regression is detected.
- Restore pre-migration test fixtures/backups for deterministic reproduction.
- Keep fail-closed gates active (do not permit plaintext fallback) during rollback validation.

## Execution evidence
- Implemented in `qsl/qsl-client/qsc/src/main.rs`:
  - encrypted session blob format (`.qsv`) with AEAD + AAD binding to peer/version.
  - deterministic failure markers for decrypt/integrity failure.
  - legacy plaintext migration with tombstone write and fail-closed `migration_blocked`.
- Implemented in `qsl/qsl-client/qsc/src/vault.rs`:
  - vault path resolution no longer falls back to `.`; defaults to `QSC_CONFIG_DIR` or XDG/HOME config path.
- Added/updated tests:
  - `qsl/qsl-client/qsc/tests/session_state_at_rest.rs`
  - `qsl/qsl-client/qsc/tests/vault.rs` (cwd-write regression guard)
  - `qsl/qsl-client/qsc/tests/qsp_status_truthy.rs`
  - `qsl/qsl-client/qsc/tests/handshake_mvp.rs`
  - `qsl/qsl-client/qsc/tests/identity_binding.rs`
- Local gates passed:
  - `cargo fmt -p qsc -- --check`
  - `cargo test -p qsc --locked`
  - `cargo clippy -p qsc --all-targets -- -D warnings`
