# NA-0106 Identity Secret At Rest Plan

## Storage Design (Vault/Keyring)
- Store `kem_sk` in encrypted vault secret key `identity.kem_sk.<self_label>`.
- Keep identity file on disk public-only (`kem_pk`), with no `kem_sk`.
- Fail closed with deterministic marker `identity_secret_unavailable` when vault is missing/locked.

## Legacy Migration
- Detect legacy `identities/self_<label>.json` that still contains `kem_sk`.
- If vault is available:
  - Import `kem_sk` into vault.
  - Rewrite legacy file to public-only record (tombstone-style removal of secret).
  - Emit `identity_secret_migrate ok=true action=imported`.
- If vault is unavailable:
  - Do not mutate legacy file.
  - Emit `identity_secret_migrate ok=false action=skipped reason=vault_unavailable`.

## Test Vectors
- `identity_secret_not_plaintext_on_disk`
- `migrate_legacy_identity_into_vault`
- `migration_requires_vault_fail_closed_no_mutation`
- `no_secrets_in_identity_outputs`

## Rollback
- Revert NA-0106 branch changes in `qsl/qsl-client/qsc/src/main.rs` and `qsl/qsl-client/qsc/src/vault.rs`.
- Remove `qsl/qsl-client/qsc/tests/identity_secret_at_rest.rs` and test harness vault-init additions.

## Executed Evidence
- `cargo fmt -p qsc -- --check`
- `cargo test -p qsc --locked`
- `cargo clippy -p qsc --all-targets -- -D warnings`
- Forensics output root:
  - `/home/victor/work/qsl/_forensics/na0106_impl_20260207T234017Z`
