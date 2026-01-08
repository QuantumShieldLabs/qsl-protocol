# QuantumShield Reference Implementation (Skeleton)

This repository is an **audit-friendly reference skeleton** for:
- **QSP v4.3.1** (QuantumShield Protocol)
- **QSE v1.8.1** (QuantumShield Envelope)

It is intended to:
- validate normative specs via executable behavior,
- generate and validate canonical test vectors, and
- reduce ambiguity for independent implementers.

## Status
- Wire encode/decode: implemented for QSP HandshakeInit/HandshakeResp/ProtocolMessage and QSE Envelope.
- Ratchet logic: implemented as a direct transcription of QSP §9.1–§9.6, with explicit DoS bounds.
- Key Transparency (KT): interface defined (verification plumbing is a stub until KT wire formats are finalized).
- Post-quantum crypto (ML-KEM / ML-DSA): trait defined; optional `pqcrypto` feature provides a placeholder mapping
  (must be validated against the project’s final algorithm bindings before any “real crypto” claim).

**Not for production**: this is a reference baseline for correctness and interop, not a hardened product.

## Quickstart
- Run parse-only vector tests:
  ```bash
  cargo test --features vectors
  ```

## Project layout
- `src/codec/` : canonical big-endian encoding and varbytes
- `src/qsp/`   : QSP message types + handshake + ratchet
- `src/qse/`   : envelope encode/decode + padding profiles
- `src/kt/`    : KT verification interfaces
- `vectors/`   : vector fixtures (parse-only included)

## Normative references
- `QSP v4.3.1` and `QSE v1.8.1` are the source of truth.
