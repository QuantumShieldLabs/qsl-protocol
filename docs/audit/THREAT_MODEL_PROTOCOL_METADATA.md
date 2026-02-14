# Threat Model: Protocol + Metadata

## Trust Boundaries
- Relay/server is untrusted for confidentiality and integrity of message/file payloads.
- Network path is untrusted and can be observed and modified.
- Local host security differs by state:
  - Locked state: vault-backed protections should prevent plaintext-at-rest exposure.
  - Unlocked state: local compromise risk increases; protections are reduced to process/runtime controls.

## Attacker Models
1. Passive network observer:
- Observes timing, packet sizes, frequency, and endpoint correlation signals.
- Cannot decrypt ciphertext directly.

2. Active network attacker:
- Attempts replay, injection, reordering, suppression, and downgrade signaling.
- Attempts to induce false delivery/receipt claims.

3. Malicious relay:
- Manipulates store/forward behavior.
- Performs traffic-correlation attempts and selective DoS.
- Cannot be trusted for truthfulness of transport events.

4. Local attacker:
- Attempts disk access (vault/config artifacts) in locked and unlocked states.
- Attempts runtime memory scraping under host compromise assumptions.

## Protected Assets
- Message/file confidentiality and integrity.
- Identity authenticity and key-binding correctness.
- Session-state correctness and anti-replay properties.
- Metadata minimization goals (content-independent observables reduced as feasible).

## Target Security Properties
- Authentication:
  - Peer identity and session establishment must be bound to cryptographic evidence.
- Transcript binding:
  - Session progression must correspond to validated handshake/transcript context.
- Forward Secrecy (FS):
  - Past ciphertext should remain protected if current long-term material is compromised later.
- Post-Compromise Security (PCS):
  - After compromise and subsequent honest key evolution, future messages should recover confidentiality/integrity.
- Replay resistance:
  - Duplicate/reordered/injected protocol events are rejected or handled without false state claims.
- Metadata goals:
  - Minimize leakage of sensitive identifiers/content hints in UI, logs, and transport-visible metadata.
  - Explicitly acknowledge what cannot be hidden (timing/size/availability classes).

## Must-Never-Happen Invariants
- Client claims DELIVERED/received states without explicit protocol evidence.
- Protocol marked ACTIVE without proven session establishment.
- Sensitive content or identifiers leak while client is locked.
- Invalid transitions mutate persisted state when reject path should be fail-closed.
- Relay-originated events are treated as trusted truth without verification.

## Mapping To Existing Artifacts
- Message-state truth semantics:
  - `qsl/qsl-client/qsc/tests/message_state_model.rs`
  - `qsl/qsl-client/qsc/tests/receipts_delivered.rs`
- File-transfer integrity/truth semantics:
  - `qsl/qsl-client/qsc/tests/file_transfer_mvp.rs`
- Locked-state and leakage behavior:
  - `qsl/qsl-client/qsc/tests/tui_locked_cmd_init_ux.rs`
  - `qsl/qsl-client/qsc/tests/tui_lock_unlock_polish.rs`
  - `qsl/qsl-client/qsc/tests/tui_autolock.rs`
- Prior canonical UI/security references:
  - `docs/qsc/QSC_TUI_SPEC.md`
  - `docs/qsc/QSC_TUI_INVARIANTS.md`

## Notes For NA-0133 / NA-0134
- NA-0133 should test these target properties against current protocol behavior and identify gaps.
- NA-0134 should quantify metadata leakage classes and map each to mitigations plus residual risk.
