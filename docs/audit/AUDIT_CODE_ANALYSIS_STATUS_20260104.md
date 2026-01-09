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
| #1 | CRITICAL | Signature verification default fallback | tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs | IMPLEMENTED (needs CI regression proof) | Commit/PR TBD |
| #2 | CRITICAL | nr overflow in skip key derivation loop | tools/refimpl/quantumshield_refimpl/src/qsp/ratchet.rs | IMPLEMENTED (needs CI regression proof) | Commit/PR TBD |
| #3 | CRITICAL | Panic on invalid Ed25519 key length | tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs | IMPLEMENTED (needs CI regression proof) | Commit/PR TBD |
| #4 | HIGH | Weak RNG initialization | tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs | OPEN (triage) | PR TBD |
| #5 | HIGH | Panic on AEAD operations | tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs | OPEN (triage) | PR TBD |
| #6 | HIGH | ck_pq_recv not updated on boundary | tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs | OPEN (triage) | PR TBD |
| #7 | HIGH | State mutation before send completion | tools/refimpl/quantumshield_refimpl/src/qsp/ratchet.rs | OPEN (triage) | PR TBD |
| #8 | HIGH | expect() calls on struct invariants | tools/refimpl/quantumshield_refimpl/src/qsp/types.rs | OPEN (triage) | PR TBD |
| #9 | HIGH | Missing key zeroization | tools/refimpl/quantumshield_refimpl/src/crypto/* | OPEN (triage) | PR TBD |
| #10 | MEDIUM | Timing side-channel in header decryption | tools/refimpl/quantumshield_refimpl/src/qsp/ratchet.rs; suite2/ratchet.rs | OPEN (triage) | PR TBD |
| #11 | MEDIUM | Nonce reuse on counter overflow | tools/refimpl/quantumshield_refimpl/src/qsp/ratchet.rs | OPEN (triage) | PR TBD |
| #12 | MEDIUM | take_mk_skipped leaves stale mk_order | tools/refimpl/quantumshield_refimpl/src/qsp/state.rs | OPEN (triage) | PR TBD |
| #13 | MEDIUM | SCKA monotonicity check insufficient | tools/refimpl/quantumshield_refimpl/src/suite2/scka.rs | OPEN (triage) | PR TBD |
| #14 | MEDIUM | store_mk_skipped silent failure | tools/refimpl/quantumshield_refimpl/src/qsp/state.rs | OPEN (triage) | PR TBD |
| #15 | MEDIUM | DH ratchet corrupts pn on ns overflow | tools/refimpl/quantumshield_refimpl/src/qsp/ratchet.rs | OPEN (triage) | PR TBD |
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
