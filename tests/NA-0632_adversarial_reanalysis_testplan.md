# NA-0632 — adversarial re-analysis testplan (analysis lane; findings FILED, not fixed)

Goals: G1, G2, G4

Directive QSL-DIR-2026-07-11-569 (D569). This lane adds **no product/spec/vector/`formal`/`.github`/
Cargo change** and **no compiled test** (an analysis lane; a proof-of-issue test that would FAIL today is
specified below as documentation for the ENG-0038 fix lane, per the D569 result boundary). The full
findings report is `docs/governance/evidence/NA-0632_adversarial_reanalysis.md`; the mechanical probe is
`docs/governance/evidence/NA-0632_sig_pin_probe.sh`.

## A. What was exercised (read-only)

- Full non-test read of `tools/refimpl/quantumshield_refimpl/src/suite2/{mod,types,binding,parse,
  establish,state,scka,ratchet}.rs` against DOC-CAN-003/004, walked per D569 §COVERAGE. Result: CHECKED-OK
  across KDF/key-schedule, nonce derivation, non-contributory DH, counter overflow, no-mutation-on-reject,
  §8.5.1 NHK anti-spoof, SCKA one-time/monotonic + ADVAUTH, combined-boundary composition, parsing/
  cross-kind confusion, primitive usage, snapshot/restore. See report §3.
- Full read of the shipped `qsc` establishment path: `qsc/src/handshake/mod.rs`, `identity/mod.rs`,
  `contacts/mod.rs`, and the receive dispatch `qsc/src/main.rs` `qsp_unpack`. Result: FINDING F1 (ENG-0038)
  in the initiator→responder authentication; the responder→initiator direction and the SCKA/session
  persistence ordering are CHECKED-OK. See report §2, §3.
- `NA-0632_sig_pin_probe.sh` — passes; mechanically corroborates that no shipped path pins the responder's
  signing key (`sig_fp` is `None` everywhere; the only real setter is a test JSON injector).
- Existing negative tests reviewed for coverage: `tests/kem_signature_transcript_binding_negative.rs`,
  `tests/identity_binding.rs`, `src/adversarial/binding_fuzz.rs`. None drive end-to-end whether the
  initiator authenticates the responder's *signing* key (report §2.3).

## B. Proof-of-issue test to add in the ENG-0038 FIX lane (currently would FAIL / expose the gap)

Add as a `qsc` integration test (mirroring `kem_signature_transcript_binding_negative.rs`'s harness), to
be un-`#[ignore]`d once ENG-0038 is fixed:

```
test: initiator_rejects_responder_with_substituted_signing_key_default_contact
setup:
  - new_vault_pair(alice, bob); seed alice's contact for bob via the PRODUCT path only
    (contacts add <bob-kem-verification-code> --verify)  -> sig_fp is None (the shipped default)
  - start inbox relay; alice `handshake init`; capture A1 from bob's channel
drive the MITM responder (no access to bob's keys):
  - decode A1; encapsulate to A1.kem_pk -> (kem_ct, ss_pq)
  - generate a FRESH ML-DSA keypair (mitm_sig_pk, mitm_sig_sk) and a fresh X25519 dh
  - build B1 exactly as hs_encode_resp does: pq_init_ss, transcript mac + th over (A1 || b1_no_auth),
    sig = Sign(mitm_sig_sk, "QSC.HS.SIG.B1" || session_id || th); frame {kem_ct, mac, mitm_sig_pk, sig, mitm_dh_pub}
  - place this B1 on alice's channel; alice `handshake poll`
assert (the SECURITY property that must hold after the fix):
  - alice emits handshake_reject (reason peer_mismatch / sig identity unverified)
  - assert_no_session(alice, bob)
CURRENT (pre-fix) BEHAVIOR (the bug): alice ACCEPTS — b1_verify ok=true, no peer_mismatch, session committed
  with authenticated=true — because sig_fp is None (optional check skipped) and the KEM primary pin is
  tautological. This test documents the gap; it must go green only after ENG-0038 is remediated.
```

Positive control (already passing, keep): with `set_contact_sig_pin(alice, bob, WRONG_SIG_FP)` a
substituted/real B1 rejects with `peer_mismatch` — i.e. the pin MECHANISM works; ENG-0038 is that the
product never populates the pin.

## C. Regression guard to add with the fix (so this cannot silently regress)

A machine check that the shipped contact-provisioning path yields a non-empty responder `sig_fp` before a
session is committed (or that the initiator's responder authentication is otherwise cryptographically
bound), plus a `NA-0632_sig_pin_probe.sh`-style assertion inverted (a Some(sig_fp) writer MUST exist on
the shipped path once fixed).
