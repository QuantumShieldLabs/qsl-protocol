# API Contracts (Reference Skeleton)

This document is **engineering-facing** and defines stable function contracts and invariants.
Governing normative specs remain **QSP v4.3.1** and **QSE v1.8.1**.

## Encode/decode
- `qsp::HandshakeInit::{encode,decode}`
- `qsp::HandshakeResp::{encode,decode}`
- `qsp::ProtocolMessage::{encode,decode}`
- `qse::Envelope::{encode,decode}`

Canonical parsing rules:
- reject truncated fields
- reject lengths exceeding remaining bytes
- reject trailing bytes not described by the message format
- reject unknown flags (QSP/QSE)

## Handshake
- `qsp::handshake::initiator_build(...) -> (HandshakeInit, InitiatorState)`
- `qsp::handshake::responder_process(...) -> (HandshakeResp, SessionState)`
- `qsp::handshake::initiator_finalize(...) -> SessionState`

Required behaviors:
- HS1 = SHA512("QSP4.3/HS1" || HS1_input) with signatures zeroed.
- HS2 = SHA512("QSP4.3/HS2" || HandshakeInit || HS2_input) with responder signatures zeroed.
- conf_B = KMAC(RK0, "QSP4.3/CONF", HS2, 32) and initiator MUST verify it.

## Ratchet
- `qsp::ratchet::ratchet_encrypt(...) -> ProtocolMessage`
- `qsp::ratchet::ratchet_decrypt(...) -> plaintext`

Normative invariants:
- no durable state commit unless both header and body decrypt succeed
- bounded header decrypt attempts (`MAX_HEADER_ATTEMPTS`)
- enforce `MAX_SKIP` and bounded MKSKIPPED size

## KT
- `kt::KtVerifier::verify_bundle(...)`
- Stub verifier returns NotImplemented (prevents silent skipping).

## PQ
- `crypto::traits::PqKem768` and `PqSigMldsa65` define required operations.
- The `pqcrypto` feature is a placeholder; verify bindings before production use.
