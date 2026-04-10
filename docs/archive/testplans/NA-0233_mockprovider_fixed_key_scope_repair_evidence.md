Goals: G4, G5

Status: Archive
Owner: QSL governance
Last-Updated: 2026-04-10
Replaces: n/a
Superseded-By: n/a

# NA-0233 MockProvider Fixed Key Scope Repair Evidence

## Summary

This governance-only lane repairs `NA-0233` so the queue scope matches refreshed current-main contradiction proof.

No runtime code or runtime tests change in this PR. The lane updates governance and evidence only so the later runtime fix can proceed truthfully from refreshed `main`.

## Refreshed Contradiction Proof On Main

- `qsl/qsl-client/qsc/src/vault/mod.rs` still defines the live MockProvider source at `KeySource::MockProvider`.
- `qsl/qsl-client/qsc/src/vault/mod.rs` still exposes `unlock_if_mock_provider()` and auto-accepts vaults whose parsed envelope has `key_source == 4`.
- `qsl/qsl-client/qsc/src/vault/mod.rs` still derives the runtime key as `[0x42u8; 32]` for `key_source == 4`.
- `qsl/qsl-client/qsc/src/vault/mod.rs` still accepts `--key-source mock` / `QSC_KEY_SOURCE=mock` and writes tag `4` for MockProvider vaults.
- `qsl/qsl-client/qsc/src/main.rs` still auto-unlocks via `vault::unlock_if_mock_provider()` during bootstrap.
- `qsl/qsl-client/qsc/src/tui/controller/commands/dispatch.rs` still auto-unlocks via `vault::unlock_if_mock_provider()` inside the explicit TUI unlock flow.
- `qsl/qsl-client/qsc/tests/common/mod.rs` still provides the shared `init_mock_vault()` helper.
- `qsl/qsl-client/qsc/tests/vault.rs` still contains the direct `vault_init_mock_provider_succeeds_without_passphrase()` coverage.

## Why The Old Scope Was Insufficient

The previously promoted `NA-0233` block allowed only:

- `tools/refimpl/quantumshield_refimpl/src/crypto/**`
- `tools/refimpl/quantumshield_refimpl/src/qsp/**` only if directly touched by the bounded fix
- `qsl/qsl-client/qsc/src/handshake/**` only if directly touched
- `qsl/qsl-client/qsc/tests/handshake_*.rs`
- `qsl/qsl-client/qsc/tests/qsp_protocol_gate.rs`
- `qsl/qsl-client/qsc/tests/desktop_gui_contract_na0215b.rs` only if directly touched

Refreshed contradiction proof shows the live runtime acceptance and auto-unlock behavior instead sit in:

- `qsl/qsl-client/qsc/src/vault/mod.rs`
- `qsl/qsl-client/qsc/src/main.rs`
- `qsl/qsl-client/qsc/src/tui/controller/commands/dispatch.rs`

The directly affected test/helper seam also includes:

- `qsl/qsl-client/qsc/tests/common/mod.rs`
- `qsl/qsl-client/qsc/tests/vault.rs`

The old block therefore could not truthfully authorize the actual bounded fix.

## Additional Current-Main Mock-Vault Consumers

Refreshed current-main contradiction scanning also found additional direct mock-vault consumers under `qsl/qsl-client/qsc/tests/**`, including:

- `qsl/qsl-client/qsc/tests/attachment_streaming_na0197c.rs`
- `qsl/qsl-client/qsc/tests/attachments_contract_na0217h.rs`
- `qsl/qsl-client/qsc/tests/aws_file_confirmation_replay_na0192b.rs`
- `qsl/qsl-client/qsc/tests/aws_file_medium_boundary_na0192a.rs`
- `qsl/qsl-client/qsc/tests/aws_file_robustness_na0186.rs`
- `qsl/qsl-client/qsc/tests/aws_r2_file_integrity_na0189.rs`
- `qsl/qsl-client/qsc/tests/aws_tui_handshake_na0191.rs`
- `qsl/qsl-client/qsc/tests/file_transfer_mvp.rs`
- `qsl/qsl-client/qsc/tests/handshake_contract_na0217i.rs`
- `qsl/qsl-client/qsc/tests/handshake_mvp.rs`
- `qsl/qsl-client/qsc/tests/handshake_security_closure.rs`
- `qsl/qsl-client/qsc/tests/identity_binding.rs`
- `qsl/qsl-client/qsc/tests/identity_foundation_contract_na0217d.rs`
- `qsl/qsl-client/qsc/tests/identity_ux.rs`
- `qsl/qsl-client/qsc/tests/message_state_model.rs`
- `qsl/qsl-client/qsc/tests/meta_min.rs`
- `qsl/qsl-client/qsc/tests/meta_phase2.rs`
- `qsl/qsl-client/qsc/tests/outbox_abort.rs`
- `qsl/qsl-client/qsc/tests/peer_confirm_policy_primary_only_na0177.rs`
- `qsl/qsl-client/qsc/tests/protocol_state_contract_na0217c.rs`
- `qsl/qsl-client/qsc/tests/qsp_protocol_gate.rs`
- `qsl/qsl-client/qsc/tests/qsp_qse_onwire.rs`
- `qsl/qsl-client/qsc/tests/qsp_status_truthy.rs`
- `qsl/qsl-client/qsc/tests/ratchet_durability_na0155.rs`
- `qsl/qsl-client/qsc/tests/ratchet_step.rs`
- `qsl/qsl-client/qsc/tests/receipt_policy_mvp_na0177.rs`
- `qsl/qsl-client/qsc/tests/receipts_delivered.rs`
- `qsl/qsl-client/qsc/tests/receive_e2e.rs`
- `qsl/qsl-client/qsc/tests/relay_auth_header.rs`
- `qsl/qsl-client/qsc/tests/relay_drop_no_mutation.rs`
- `qsl/qsl-client/qsc/tests/relay_dup_no_mutation.rs`
- `qsl/qsl-client/qsc/tests/relay_reorder_no_mutation.rs`
- `qsl/qsl-client/qsc/tests/remote_fault_injection.rs`
- `qsl/qsl-client/qsc/tests/send_ready_markers_na0168.rs`
- `qsl/qsl-client/qsc/tests/send_semantics.rs`
- `qsl/qsl-client/qsc/tests/session_state_at_rest.rs`
- `qsl/qsl-client/qsc/tests/suite2_runtime_equivalence_na0198.rs`
- `qsl/qsl-client/qsc/tests/timeline_delivery_contract_na0217f.rs`
- `qsl/qsl-client/qsc/tests/timeline_store.rs`
- `qsl/qsl-client/qsc/tests/trust_gate_unify_na0177.rs`
- `qsl/qsl-client/qsc/tests/trust_model_v2_phase_a_na0177.rs`
- `qsl/qsl-client/qsc/tests/trust_model_v2_phase_b_na0177.rs`
- `qsl/qsl-client/qsc/tests/trust_model_v2_phase_c_na0177.rs`
- `qsl/qsl-client/qsc/tests/trust_onboarding_mainstream_flow_na0187.rs`
- `qsl/qsl-client/qsc/tests/trust_remediation_ux_na0178.rs`
- `qsl/qsl-client/qsc/tests/tui_conversation_first_na0177.rs`
- `qsl/qsl-client/qsc/tests/tui_product_polish_na0214a.rs`
- `qsl/qsl-client/qsc/tests/tui_relay_config.rs`
- `qsl/qsl-client/qsc/tests/tui_relay_drop_reorder.rs`
- `qsl/qsl-client/qsc/tests/two_client_local_runbook_na0182.rs`

These consumers justify the repaired fallback line allowing `qsl/qsl-client/qsc/tests/**` only when directly touched by the bounded mock-vault fix and justified by refreshed contradiction proof.

## Exact Repaired Scope Text As Committed

```md
### NA-0233 — MockProvider Fixed Vault Key Resolution
Status: READY
Problem:
- `NA-0230` ranked the MockProvider fixed vault-key issue as the next Tier-0 item after ML-DSA timing-oracle and `QSC_HANDSHAKE_SEED` resolution. Refreshed main proves the live fixed/default key path is still reachable through `qsl/qsl-client/qsc/src/vault/mod.rs`, with shipped/shared call sites through `qsl/qsl-client/qsc/src/main.rs` and `qsl/qsl-client/qsc/src/tui/controller/commands/dispatch.rs`, while the previous queue scope understated the real runtime and test-helper surfaces needed for a truthful fix.
Scope:
- `qsl/qsl-client/qsc/src/vault/**`
- `qsl/qsl-client/qsc/src/main.rs` only if directly touched by the bounded fix
- `qsl/qsl-client/qsc/src/tui/controller/commands/dispatch.rs` only if directly touched by the bounded fix
- `qsl/qsl-client/qsc/src/handshake/**` only if directly touched
- `qsl/qsl-client/qsc/tests/common/mod.rs`
- `qsl/qsl-client/qsc/tests/vault.rs`
- `qsl/qsl-client/qsc/tests/handshake_*.rs`
- `qsl/qsl-client/qsc/tests/qsp_protocol_gate.rs`
- `qsl/qsl-client/qsc/tests/desktop_gui_contract_na0215b.rs` only if directly touched
- `qsl/qsl-client/qsc/tests/**` only if directly touched by the bounded mock-vault fix and justified by refreshed contradiction proof
- `DECISIONS.md`
- `TRACEABILITY.md`
- docs/governance/evidence only as needed
- no `.github`, website, `Cargo.toml`, `Cargo.lock`, `qsc-desktop`, `qsl-server`, or `qsl-attachments` changes
Must protect:
- transcript binding
- pinned mismatch reject behavior
- NA-0221 fail-closed no-mutation behavior
- NA-0222 honest operator-visible status/marker truth
- current qsc-desktop sidecar contract
- current route-token/header discipline and honest-delivery semantics
- qsl-server remains transport-only
- qsl-attachments remains opaque ciphertext-only
Deliverables:
1) prove the exact current fixed/default MockProvider vault-key reachability truth on refreshed main
2) either remove any fixed/default MockProvider vault key from the shipped/shared crypto path or prove the staged finding stale on current main
3) add direct verification/regression evidence for the final runtime truth
4) update governance/evidence truthfully
Acceptance:
1) the production/shared path no longer carries an unresolved fixed/default MockProvider vault-key claim
2) if a runtime fix is needed, no shipped/shared path can fall back to a hardcoded MockProvider key
3) representative handshake and cross-seam canaries remain green
4) no unrelated protocol/service/wire drift is introduced
```

## Governance-Only Note

This PR repairs queue truth only. It introduces no runtime changes, no runtime-test changes, and does not close `NA-0233`.
