# NA-0101 PQ Signature Identity Plan

## Scope
- Add ML-DSA identity signing to handshake transcript binding in `qsc`.
- Enforce signature verification fail-closed for pinned peers.
- Keep signature secrets vault-backed (no plaintext secret-at-rest regression).

## Required refimpl primitives (ML-DSA)
- `PqSigMldsa65` trait implemented by `StdCrypto`.
- `keygen() -> (pk, sk)`, `sign(sk, msg) -> sig`, `verify(pk, msg, sig) -> bool`.

## Signed transcript binding
- B1 carries responder `sig_pk` and detached signature over canonical transcript core.
- A2 carries initiator detached signature over confirm transcript (`session_id || transcript_hash || confirm_mac`).
- Domain separation labels:
  - `QSC.HS.SIG.B1`
  - `QSC.HS.SIG.A2`

## Key rotation/revocation sketch
- Rotation stays explicit through identity UX; pinned peer fingerprint checks remain fail-closed.
- Signature identity pinning is independent from KEM TOFU pin and must both pass.

## Verification checklist
- `cargo fmt -p qsc -- --check`
- `cargo test -p qsc --locked`
- `cargo clippy -p qsc --all-targets -- -D warnings`
- Handshake tests cover:
  - valid signature path succeeds,
  - tamper rejects with deterministic marker,
  - pinned-key mismatch rejects with no mutation.

## Executed evidence
- Forensics OUT: `/home/victor/work/qsl/_forensics/na0101_finish_20260208T002306Z`
- `cargo test -p qsc --locked` passed.
- `cargo clippy -p qsc --all-targets -- -D warnings` passed.
- Relevant tests green:
  - `handshake_two_party_establishes_session`
  - `handshake_tamper_rejects_no_mutation`
  - `tofu_mismatch_rejected_no_mutation`
