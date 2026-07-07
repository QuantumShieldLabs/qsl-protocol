Goals: G3 (primary), supports G1, G2, G4, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-07

# NA-0616 — ENG-0001 Self-Label Footgun Remediation

## Summary

NA-0616 remediates ledger ENG-0001 (DOC-G5-005 §9 rank 5) under directive
QSL-DIR-2026-07-07-553 (D553): the qsc self-identity selection path silently auto-minted
a fresh identity for any `--as <label>` that did not yet exist, so a typo'd or
inconsistent self-label operated a divergent identity and the peer saw a mismatched
fingerprint (`peer_mismatch`). This lane makes it **fail closed**. Client-side identity
selection/creation gating and a CLI default only — no handshake/crypto/wire/state-machine
semantic change.

Result classification: `SELF_LABEL_FOOTGUN_FAIL_CLOSED`.

## Design (design-lock)

- A config dir is meant to hold one self-identity (stored as `identities/self_<label>.json`;
  secrets in the vault). Confirmed all auto-create call sites are the handshake paths
  (`handshake/mod.rs:1247/1556/1890`); explicit `identity rotate` (`main.rs`) creates
  directly and bypasses the gate. Multi-party tests use separate config dirs, so no
  legitimate same-dir multi-identity-via-handshake flow exists (design-lock STOP not
  triggered).
- Fix: in the auto-create branch of `identity_self_kem_keypair` (`identity/mod.rs`),
  before minting, scan the identities dir for a `self_*.json` under a **different** label;
  if one exists, emit `identity_self_ambiguous` and return the new
  `ErrorCode::IdentitySelfAmbiguous`. First-run (empty dir) still auto-creates; the same
  label still loads; explicit `identity rotate` still creates any label intentionally.
- Consistency: the handshake `--as` default is aligned with `identity show`'s `"self"`
  (`cmd/mod.rs`), the canonical single-self-label.

## Changed surface

- `qsl/qsl-client/qsc/src/model/mod.rs`: `ErrorCode::IdentitySelfAmbiguous` (+ `as_str`
  → `"identity_self_ambiguous"`).
- `qsl/qsl-client/qsc/src/identity/mod.rs`: the fail-closed second-identity gate in the
  auto-create branch.
- `qsl/qsl-client/qsc/src/cmd/mod.rs`: handshake `Init`/`Poll` `--as` default `"self"`.
- `qsl/qsl-client/qsc/tests/na_0616_self_label_footgun.rs`: deterministic tests.

No Cargo/handshake/SCKA/crypto-suite/protocol_state/message-plane/workflow/`.claude`
change; no new dependency; no change to identity key derivation/storage/authentication
beyond the create-gate and the CLI default.

## Validation

- `cargo fmt --check` OK; `cargo build` OK; `cargo clippy` clean; `cargo metadata
  --locked` OK; Cargo.toml/Cargo.lock unchanged.
- `na_0616_self_label_footgun` (4): `rotate_creates_canonical_self`;
  `second_divergent_self_label_fails_closed` (end-to-end via the real handshake path — a
  peer set up as an authenticated contact with a real fingerprint + route token; the gate
  fires with `identity_self_ambiguous` before the network push and no `self_alice.json`
  is minted — no-mutation-on-reject); `consistent_self_label_is_not_blocked_by_the_gate`;
  `explicit_rotate_of_second_label_is_allowed`.
- All qsc binary unit tests pass (38). No regression across the identity/handshake/
  adversarial suites: `identity_binding`, `kem_signature_transcript_binding_negative`,
  `identity_foundation_contract_na0217d`, `handshake_contract_na0217i`,
  `handshake_security_closure`, `aws_tui_handshake_na0191`, `adversarial_properties`,
  `identity_secret_at_rest`, `desktop_gui_contract_na0215b`.

## Claim boundary

Client-side identity selection/creation gating and a CLI default; no handshake/
crypto-suite/wire/state-machine change and no change to how an established identity's
keys are derived, stored, or authenticated. No endpoint, port, token, capability, key,
seed, plaintext, ciphertext body, or raw private material is published (identity labels
are operator-chosen names, class-safe). No public-readiness, production-readiness,
security-completion, crypto-complete, or bug-free claim is made.
