# NA-0116 Contacts Verify Block Plan

## Scope and assumptions
- Scope limited to `qsl/qsl-client/qsc/**` with governance evidence updates in root docs.
- Contacts trust state is client-local and stored via vault secret APIs.
- No server, refimpl, workflow, or scripts changes.

## Threat model notes
- Silent trust escalation from unknown to trusted peers.
- Pinned fingerprint mismatch accepted without explicit user decision.
- Blocked peers still handshaking/sending due to missing enforcement checks.
- Contact metadata leakage via plaintext disk files.

## Must-never list
- Must never silently trust unverified peer updates.
- Must never accept mismatch without explicit deterministic error state.
- Must never mutate contact/session state on reject paths.
- Must never store contact pins in plaintext files.
- Must never print secrets/tokens/private key material in markers or CLI output.

## Proposed design
- Add `qsc contacts` command family: `add/show/list/verify/block/unblock`.
- Persist contacts as a vault-backed JSON secret (`contacts.json`) keyed by peer label.
- Enforce blocked-peer refusal and pinned-mismatch refusal in handshake/send/relay-send paths.
- Keep deterministic markers for contact state transitions and reject reasons.
- Surface contacts state in TUI inspector/focus and support `/contacts` commands.

## Test vectors
- Deterministic contacts add/list order and marker output.
- Blocked peer handshake refusal with no mutation.
- Pinned mismatch refusal with no mutation.
- Verify update requires explicit `--confirm` or refuses with no mutation.
- Contacts not written as plaintext config files.
- No secrets in command output.

## Verification checklist
- `cargo fmt -p qsc -- --check`
- `cargo test -p qsc --locked`
- `cargo clippy -p qsc --all-targets -- -D warnings`

## Executed evidence
- Added tests: `qsl/qsl-client/qsc/tests/contacts_verify_block.rs`:
  - `contacts_add_list_deterministic`
  - `blocked_peer_refuses_handshake_no_mutation`
  - `pinned_mismatch_refuses_no_mutation`
  - `verify_requires_confirm_no_mutation`
  - `no_plaintext_contacts_on_disk`
  - `no_secrets_in_output`
- Updated existing suites for NA-0116 semantics:
  - `qsl/qsl-client/qsc/tests/identity_binding.rs`
  - `qsl/qsl-client/qsc/tests/identity_ux.rs`
  - `qsl/qsl-client/qsc/tests/tui_relay_drop_reorder.rs`

## Rollback
- Revert NA-0116 implementation commit(s) and restore pre-change contact/pin behavior.
- Re-run package-scoped gates to confirm baseline restoration.
