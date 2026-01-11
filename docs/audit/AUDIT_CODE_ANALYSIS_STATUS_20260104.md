# Audit Status — Code Analysis 2026-01-04

Report SHA256: 6802435f26ca459b3aa9485afb25f0532842205b3640168a4f21561b6cc7633b

## Severity counts

| Severity | Count |
|----------|-------|
| CRITICAL | 3 |
| HIGH | 6 |
| MEDIUM | 14 |
| LOW | 5 |

## Issue status table

Status legend: OPEN, IMPLEMENTED (needs CI regression proof), VERIFIED

| Issue | Severity | Title | File | Status | Evidence |
|------:|----------|-------|------|--------|----------|
| #1 | CRITICAL | Signature verification default fallback | tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs:78-92 | VERIFIED | Code: stdcrypto.rs:78-92; Test: ed25519_verify_rejects_invalid_pubk_len (stdcrypto.rs:108-113); PR #25 |
| #2 | CRITICAL | nr overflow in skip key derivation loop | tools/refimpl/quantumshield_refimpl/src/qsp/ratchet.rs:320-371 | VERIFIED | Code: ratchet.rs:320-371; Test: checked_inc_nr_overflow_rejects (ratchet.rs:377-381); PR #25 |
| #3 | CRITICAL | Panic on invalid Ed25519 key length | tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs:68-76 | VERIFIED | Code: stdcrypto.rs:68-76; Test: ed25519_sign_invalid_priv_len_is_fail_closed (stdcrypto.rs:115-119); PR #25 |
| #4 | HIGH | Weak RNG initialization | tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs | FIXED (guarded) | Code: stdcrypto.rs:64-112 (OsRng); Test: x25519_keypair_uses_os_rng / random_nonce12_not_all_zero (stdcrypto.rs); PR #22 |
| #5 | HIGH | Panic on AEAD operations | tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs | FIXED (guarded) | Code: stdcrypto.rs:29-52; Test: aead_seal_invalid_key_len_is_fail_closed / aead_seal_invalid_nonce_len_is_fail_closed (stdcrypto.rs); PR #21 |
| #6 | HIGH | ck_pq_recv not updated on boundary | tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs | FIXED (guarded) | Code: suite2/ratchet.rs:365-412; Tests: boundary_reject_is_deterministic_and_no_state_mutation_on_bad_ct_len / boundary_success_advances_ck_pq_recv_from_reseed (ratchet.rs:685-744); PR #28 |
| #7 | HIGH | State mutation before send completion | tools/refimpl/quantumshield_refimpl/src/qsp/ratchet.rs | FIXED (guarded) | Code: ratchet.rs:190-260; Test: ratchet_encrypt_rejects_deterministically_and_no_state_mutation (ratchet.rs:457-476); PR #23 |
| #8 | HIGH | expect() calls on struct invariants | tools/refimpl/quantumshield_refimpl/src/qsp/types.rs | FIXED (guarded) | Code: types.rs:136-144; Test: handshake_init_encode_fails_closed_on_missing_opk_fields (types.rs); PR #20 |
| #9 | HIGH | Missing key zeroization | tools/refimpl/quantumshield_refimpl/src/crypto/traits.rs | FIXED (guarded) | Code: traits.rs:28-58; Tests: x25519_priv_zeroize_traits / x25519_priv_zeroize_clears_bytes (traits.rs); PR #25 |
| #10 | MEDIUM | Timing side-channel in header decryption | tools/refimpl/quantumshield_refimpl/src/qsp/ratchet.rs; suite2/ratchet.rs | FIXED (guarded) | Code: qsp/ratchet.rs:145-187; suite2/ratchet.rs:205-260; Tests: header_decrypt_attempts_all_candidates_even_on_first_success / header_decrypt_rejects_deterministically_and_no_state_mutation (qsp/ratchet.rs); nonboundary_header_attempts_all_candidates_even_on_first_success / nonboundary_rejects_deterministically_and_no_state_mutation (suite2/ratchet.rs); PR #30 |
| #11 | MEDIUM | Nonce reuse on counter overflow | tools/refimpl/quantumshield_refimpl/src/qsp/ratchet.rs | FIXED (verified) | Evidence: qsp/ratchet.rs:194-199 checked_add with ns overflow reject; PR #20 |
| #12 | MEDIUM | take_mk_skipped leaves stale mk_order | tools/refimpl/quantumshield_refimpl/src/qsp/state.rs | FIXED (guarded) | Code: qsp/state.rs:111-115; Tests: take_mk_skipped_removes_from_mk_order / take_mk_skipped_on_missing_does_not_corrupt_order (state.rs); PR #32 |
| #13 | MEDIUM | SCKA monotonicity check insufficient | tools/refimpl/quantumshield_refimpl/src/suite2/scka.rs | FIXED (guarded) | Code: suite2/scka.rs:46-102; Tests: scka_rejects_nonmonotonic_epoch_deterministically_and_no_mutation / scka_accepts_next_monotonic_epoch_and_updates_state (suite2/scka.rs:120-215); PR #34 |
| #14 | MEDIUM | store_mk_skipped silent failure | tools/refimpl/quantumshield_refimpl/src/qsp/state.rs | FIXED (guarded) | Code: state.rs:99-111; Tests: store_mk_skipped_rejects_deterministically_and_no_state_mutation_on_failure / store_mk_skipped_success_stores_and_indexes (state.rs:464-499); PR #36 |
| #15 | MEDIUM | DH ratchet corrupts pn on ns overflow | tools/refimpl/quantumshield_refimpl/src/qsp/ratchet.rs | FIXED (guarded) | Code: ratchet.rs:87-138; Tests: dh_ratchet_rejects_on_ns_overflow_deterministically_and_no_state_mutation / dh_ratchet_success_near_boundary_does_not_corrupt_pn (ratchet.rs:522-549); PR #38 |
| #16 | MEDIUM | DoS via large collection deserialization | tools/refimpl/quantumshield_refimpl/src/suite2/state.rs | OPEN (triage) | PR TBD |
| #17 | MEDIUM | Multiple unwraps on header_pt | tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs | OPEN (triage) | PR TBD |
| #18 | MEDIUM | Unsafe unwraps in OPK handling | tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs | OPEN (triage) | PR TBD |
| #19 | MEDIUM | State cloning proliferates key material | tools/refimpl/quantumshield_refimpl/src/qsp/ratchet.rs | OPEN (triage) | PR TBD |
| #20 | MEDIUM | Mutex::lock().unwrap() in CLI | tools/refimpl/quantumshield_refimpl/apps/qshield-cli/src/commands/relay.rs | OPEN (triage) | PR TBD |
| #21 | MEDIUM | MKSKIPPED removal without recovery | tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs | OPEN (triage) | PR TBD |
| #22 | MEDIUM | Boundary message window not enforced | tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs | OPEN (triage) | PR TBD |
| #23 | MEDIUM | ss3 entropy discarded in handshake | tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs | OPEN (triage) | PR TBD |
| #24 | LOW | Hardcoded ZERO32 initialization | tools/refimpl/quantumshield_refimpl/src/suite2/establish.rs | OPEN (triage) | PR TBD |
| #25 | LOW | Inconsistent error types | tools/refimpl/quantumshield_refimpl/src/* | OPEN (triage) | PR TBD |
| #26 | LOW | Asymmetric initial state | tools/refimpl/quantumshield_refimpl/src/suite2/establish.rs | OPEN (triage) | PR TBD |
| #27 | LOW | Signature verification order | tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs | OPEN (triage) | PR TBD |
| #28 | LOW | Redundant safe unwraps | various | OPEN (triage) | PR TBD |

## Next closure order

1) Issues #1–#3 (CI regression proof + status VERIFIED)
2) Issue #5 (panic-free AEAD) / Issue #8 (struct invariant panics) / Issue #9 (zeroize)
