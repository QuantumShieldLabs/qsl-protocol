# NA-0195E Provider Migration Evidence

## Before State
- Supported app/runtime crates already consumed PQ functionality only through `quantumshield_refimpl`.
- `qsc` no longer directly named `pqcrypto-kyber`, `pqcrypto-dilithium`, or `pqcrypto-traits` in its manifest or source.
- `quantumshield_refimpl` still owned the supported-runtime provider choice with:
  - `pqkem -> pqcrypto-kyber + pqcrypto-traits`
  - `pqcrypto -> pqcrypto-kyber + pqcrypto-dilithium`
- Raw supported-runtime advisory residuals were:
  - `RUSTSEC-2024-0381` via `pqcrypto-kyber`
  - `RUSTSEC-2024-0380` via `pqcrypto-dilithium`
- Tooling-only residual stayed separate:
  - `RUSTSEC-2025-0144` via `refimpl_actor -> ml-dsa`

## Supported Runtime Actual Use
- KEM provider functionality is required now by:
  - `tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs`
  - `tools/refimpl/quantumshield_refimpl/src/qsp/ratchet.rs`
  - qsc handshake keypair/length helpers in `qsl/qsl-client/qsc/src/main.rs`
- Signature provider functionality is required now by:
  - `tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs`
  - qsc handshake keypair/length helpers in `qsl/qsl-client/qsc/src/main.rs`
- Provider-specific public lengths and serialization helpers are part of current supported runtime behavior through the boundary helper API in `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs`.

## Candidate Evaluation Summary
- Rejected: keep `pqcrypto-kyber` / `pqcrypto-dilithium`.
  - Leaves supported-runtime RustSec residuals unchanged.
- Rejected after initial trial: `pqcrypto-mlkem` / `pqcrypto-mldsa`.
  - RustSec replacement direction matches.
  - The KEM side is a good fit.
  - The signature side leaves a supported-runtime transitive `paste` advisory through `pqcrypto-mldsa`, so it does not finish the item truthfully.
- Chosen:
  - KEM: `pqcrypto-mlkem`
  - signature: RustCrypto `ml-dsa >= 0.1.0-rc.3`
  - This is still boundary-internal, preserves the public lengths already encoded into the supported-runtime handshake contract, and clears the supported-runtime PQ advisories without reopening app ownership.
- Rejected: pruning PQ signature from supported runtime.
  - qsc handshake behavior still needs it.

## After State
- Supported runtime provider ownership remains fully concentrated in `quantumshield_refimpl`.
- Supported app/runtime crates still do not directly own third-party PQ/provider churn.
- Supported-runtime provider crates removed from residual advisory scope:
  - `pqcrypto-kyber`
  - `pqcrypto-dilithium`
- Remaining residual after this item, if any:
  - tooling-only `refimpl_actor -> ml-dsa`
