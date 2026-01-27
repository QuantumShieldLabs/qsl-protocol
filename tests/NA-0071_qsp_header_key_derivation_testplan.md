Goals: G1, G3, G4, G5
Status: DRAFT

Scope:
- QSP v4.3 header key derivation (refimpl protocol-core lane).
- No wire-format changes; correctness hardening only.

Objective:
- Prove header keys are derived from RK using KMAC-based KDF and correct domain separation labels.
- Ensure wrong-RK or invalid inputs fail deterministically without state mutation.

Invariants under test:
- Header keys depend on RK (KMAC-based KDF); placeholders/static labels are forbidden.
- Wrong RK fails to decrypt headers (deterministic reject).
- Rejected inputs do not mutate session state.

Positive/negative vectors planned:
- Positive (T1): `header_keys_depend_on_rk` — two SessionState instances with different RK0 yield different HK/NHK (KMAC-based).
- Positive (T2): boundary header decrypt succeeds under same RK0 (initiator→responder).
- Negative (T2): boundary header decrypt under wrong RK0 fails deterministically and does not mutate session state.
- Negative: placeholder/static derivation not used in protocol lanes (compile-time removal).

CI commands expected to gate:
- rustfmt --edition 2021 --check tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs tools/refimpl/quantumshield_refimpl/src/qsp/mod.rs tools/refimpl/quantumshield_refimpl/src/qsp/ratchet.rs tools/refimpl/quantumshield_refimpl/src/qsp/state.rs tools/refimpl/quantumshield_refimpl/src/suite2/establish.rs tools/refimpl/quantumshield_refimpl/src/suite2/parse.rs tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs tools/refimpl/quantumshield_refimpl/src/suite2/scka.rs tools/refimpl/quantumshield_refimpl/tests/na_0071_header_key_derivation.rs
- cargo test -p quantumshield_refimpl --locked
- cargo clippy -p quantumshield_refimpl --all-targets -- -D warnings

No-mutation-on-reject checks:
- Ensure session state (rk, hk/nhk, counters) unchanged after failed derivation/decrypt attempt.

Evidence:
- Local runs (see PR log): `rustfmt --edition 2021 --check <changed refimpl files>`, `cargo test -p quantumshield_refimpl --locked`, `cargo clippy -p quantumshield_refimpl --all-targets -- -D warnings`.
