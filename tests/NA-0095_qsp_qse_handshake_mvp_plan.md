# NA-0095 QSP/QSE Handshake MVP Plan

## Scope & Assumptions
- Client-only (qsc) handshake over inbox transport.
- Server remains blind; uses PUSH/PULL contract only.

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
- Gates: fmt/test/clippy pass.

## Rollback
- Revert handshake changes; preserve inbox contract usage.
