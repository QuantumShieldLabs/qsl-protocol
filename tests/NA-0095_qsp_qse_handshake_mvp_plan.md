# NA-0095 QSP/QSE Handshake MVP Plan

## Scope & Assumptions
- Client-only (qsc) handshake over inbox transport.
- Server remains blind; uses PUSH/PULL contract only.
- Prerequisite: PQ KEM (ML-KEM-768) available in refimpl via PqKem768.

## Prerequisite: PQ KEM Implementation
- API surface: keygen, encapsulate, decapsulate (PqKem768 trait).
- Implemented in refimpl crypto module (StdCrypto implements PqKem768).
- Test vectors:
  - Consistency: decap(encap(pk)) == ss (deterministic length).
  - Error path: decap fails on malformed ct.
  - CodeQL/CI gates remain green.

## Hybrid Policy
- PQ must suffice alone; if hybrid is used, mix PQ+X25519 via KDF with domain separation.
- X25519-only handshake is forbidden.

## Message Flow (A→B, B→A)
- A initiates handshake.
- B responds.
- A completes; session becomes ACTIVE.

## Transcript Format + Versioning + Domain Separation
- Explicit version field.
- Domain separation tag included in transcript hash.
- Deterministic ordering and field encoding.

## State Machine
- pre-init → pending → established
- Rejected messages do not advance state.

## Deterministic Markers
- handshake_start
- handshake_msg
- handshake_complete
- handshake_reject

## No-Mutation-on-Reject Checks
- Reject does not update persistent session state.

## Test Vectors
- Happy path
- Tamper
- Replay
- Out-of-order
- Wrong version

## Ratchet Interface Design (Types Only)
- Define types/traits for ratchet state boundaries.
- No advancement logic activated.

## Verification Checklist
- Deterministic transcript and markers.
- No secrets in markers/UI/logs.
- PQ KEM is used; no X25519-only path.
- Gates: fmt/test/clippy pass.

## Rollback
- Revert handshake changes; preserve inbox contract usage.

## Executed Evidence
- Tests:
  - qsc/tests/handshake_mvp.rs::handshake_two_party_establishes_session
  - qsc/tests/handshake_mvp.rs::handshake_tamper_rejects_no_mutation
  - qsc/tests/handshake_mvp.rs::handshake_out_of_order_rejects_no_mutation
- Commands (local, embedded inbox server):
  - qsc handshake init --as alice --peer bob --relay <inbox>
  - qsc handshake poll --as bob --peer alice --relay <inbox> --max 4
  - qsc handshake poll --as alice --peer bob --relay <inbox> --max 4
