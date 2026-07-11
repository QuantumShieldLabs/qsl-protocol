# NA-0633 design-lock — ENG-0038 fix: authenticate the responder to the initiator

Goals: G1, G2, G3, G4

Directive QSL-DIR-2026-07-11-570 (D570, APPROVED). Base `main == ef6575b9`. Fix lane (wire/crypto/auth
change ALLOWED, fail-closed). Operator Decision 1 goal: authenticate the responder against the SAME KEM
identity the user verifies out-of-band, so the ONE verification code authenticates both directions.

## Phase 0 result (PoC-first): ENG-0038 REPRODUCED

`qsl/qsl-client/qsc/tests/NA_0633_eng0038_reproduction.rs` passes: with two REAL qsc identities and no
forged frames, initiator `alice` (who pinned + verified `bob`'s KEM code `QSCFP-76f78f39…`) commits a
Suite-2 session to `mallory` (`QSCFP-dc37d531…`, a different KEM AND signing identity) that occupied bob's
channel and answered the handshake. No `peer_mismatch`; the session file is written. **The finding is
confirmed by running PoC, not just the NA-0632 trace.** Proceed (the Phase-0 STOP was "cannot reproduce").

## Root cause (why the responder is unauthenticated to the initiator)

The initiator's identity is authenticated to the responder because the initiator's identity KEM key is
(a) pinned by fingerprint AND (b) *used* — the responder encapsulates to it and the initiator proves
possession via the A2 confirm MAC. The responder has an identity KEM key too, but the handshake never
sends it and never makes the responder prove possession of it; the responder's only credential is a
signing key that is neither pinned (sig_fp is always None) nor bound to the verified KEM identity.

**Therefore the fix must make the responder PROVE possession of the secret behind its pinned KEM
identity.** Possession proof is an encapsulate→decapsulate exchange: someone encapsulates to the
responder's identity KEM public key `ek_B`, and only the holder of the secret can decapsulate. Two facts
constrain the construction:
- A contact currently stores only `ek_B`'s FINGERPRINT, not `ek_B` itself (`store/mod.rs` ContactRecord),
  and there is no directory/prekey fetch. So either the responder must SEND `ek_B` in the handshake, or
  provisioning must be extended to carry `ek_B` (verified against the human fingerprint code).
- The initiator only learns `ek_B` in time to encapsulate AFTER it has `ek_B`. If `ek_B` arrives in B1,
  the initiator's encapsulation `ct_B` can only go out in A2, so an EXPLICIT responder-possession proof
  the initiator can check needs a message AFTER A2 (a 4th message) — unless `ek_B` is provisioned up
  front (then the initiator encapsulates in A1 and the responder proves possession by producing a valid
  B1, no extra message).

## Candidate constructions

All three make `ss_B` (the shared secret from encapsulating to the responder's identity KEM key) a REQUIRED
input to the handshake key schedule (mixed into the transcript/root/confirm), so only the holder of the
responder's KEM secret can derive the session. They differ in provisioning, message count, and whether the
initiator gets an EXPLICIT handshake-time rejection of a wrong responder.

### C1 — provisioned `ek_B` + encapsulate in A1 (explicit reject at B1; fewest messages)
Provisioning carries the peer's full identity KEM public key `ek_B` (in the contact blob / QR), verified at
add-time: `fingerprint(ek_B) == the pinned verification code`. The initiator encapsulates to `ek_B` in A1
(new A1 field `ct_B`) and mixes `ss_B` into the transcript key. The responder decapsulates `ct_B` with its
identity KEM secret and mixes `ss_B`; B1's MAC now depends on `ss_B`. A wrong responder cannot derive `ss_B`
⇒ cannot produce a B1 the initiator accepts ⇒ **explicit reject at B1**.
- Pros: cleanest; explicit reject; no extra message; the contact finally carries the real key (how secure
  messengers actually work — the fingerprint stays the human-comparable element). Fits the trusted-group /
  out-of-band contact model of the product.
- Cons: **provisioning-model change** — `contacts add` must accept + verify the full `ek_B` (1184 B),
  not just the code; the contact record + blob format grow; the reproduction test's contact setup changes.

### C2 — responder sends `ek_B` in B1 + a 4th message B2 for possession (explicit reject at B2; symmetric)
B1 carries `ek_B` (initiator checks `fingerprint(ek_B) == pin`). A2 carries the initiator's encapsulation
`ct_B`. A new **B2** (responder→initiator) carries the responder's possession proof (a MAC under a key
derived from `ss_B`); the initiator **rejects at B2** if it fails. This is the exact symmetric mirror of the
existing design: just as the initiator proves KEM possession in A2 (msg 3), the responder proves it in B2
(msg 4).
- Pros: explicit reject; NO provisioning change (the key travels in B1 like the initiator's does in A1);
  most symmetric completion of the current "almost-mutual" handshake.
- Cons: adds a 4th message (a real message-flow change: transport, state machine, replay/ordering, the
  handshake spec's message table). The initiator's "complete" only after B2.

### C3 — responder sends `ek_B` in B1 + implicit channel binding (3 messages; NO explicit reject)
B1 carries `ek_B` (fingerprint-checked); A2 carries `ct_B`; `ss_B` mixed into the root. The handshake
"completes", but the session root is bound to `ss_B`, so a wrong responder cannot derive it — every
subsequent message is unreadable/uninjectable by anyone but the real responder.
- Pros: smallest change (B1/A2 fields + key-schedule mix); no extra message; no provisioning change.
- Cons: **NO explicit handshake-time rejection** — the initiator commits a session and only the
  *confidentiality* is protected (a wrong responder gets ciphertext it cannot read). The ENG-0038
  reproduction test asserts an explicit REJECT; under C3 that assertion cannot hold (the test would have to
  become "mallory cannot read alice's first message"), a weaker and less obvious security property for a UI.

## DECISION (operator, 2026-07-11): C1 selected.

The operator chose **C1** at this design-lock checkpoint. Implementation binds the responder's identity KEM
key as follows: the contact carries the peer's full identity KEM public key `ek_B` (verified
`fingerprint(ek_B) == the pinned verification code` at add-time); the initiator encapsulates to `ek_B`
(`ct_B`, carried as a NEW A1 field) and mixes the resulting `ss_B` into `pq_init_ss` (which keys the
transcript MAC and the Suite-2 root); the responder decapsulates `ct_B` with its identity KEM secret and
mixes the same `ss_B`. A wrong responder cannot derive `ss_B` ⇒ its B1 transcript MAC fails the initiator's
check ⇒ **explicit reject at B1** (`REJECT`/`bad_transcript`-class), and, defense-in-depth, its session root
would differ. B1/A2 formats are UNCHANGED (only A1 grows by the 1088-byte `ct_B`); provisioning grows to
carry `ek_B`; `hs_pq_init_ss` mixes `ss_B`. Failing to provision `ek_B` for a peer is fail-closed (the
initiator refuses to initiate rather than fall back to the unauthenticated path).

## Recommendation (pre-decision, retained)

**C1** for the cleanest, explicit, fewest-message result that fits the product's trusted-group contact
model — provided the provisioning-model change (contacts carry the peer's full identity KEM key, verified
against the code) is acceptable. **C2** if provisioning must stay code-only (accept a 4th message instead).
**C3 is not recommended** (no explicit reject; weaker, less legible property).

The choice is a genuine product fork (provisioning UX vs message count vs explicit-vs-implicit), so it is
surfaced to the operator at this design-lock checkpoint before implementation, per D570 Operator Decision 4.

## Invariants any construction must satisfy (from D570 COVERAGE)
1. A wrong responder (attacker KEM+sig keys) is REJECTED / cannot read the initiator's messages.
2. The out-of-band KEM code authenticates the responder — no second mandatory out-of-band fingerprint.
3. The initiator→responder direction is UNCHANGED and un-regressed (the responder still authenticates the
   initiator via the initiator's KEM key + A2 confirm).
4. Honest peers still establish (positive round-trip).
5. Fail-closed: unknown/unpinned peer, `fingerprint(ek_B) != pin`, possession-proof failure, malformed
   field ⇒ stable reject code, no committed session.
6. No silent TOFU introduced.
7. Moves no post-compromise/PQ claim; may substantiate a bidirectional-peer-auth statement (claim-adjacent,
   fail-closed) — the design-lock names the exact claim-matrix wording, if any, at implementation.

## Spec + vector + claim surface (to update at implementation)
- The `QSC.HS.*` handshake spec: the message table (A1/B1[/B2] fields), the `ss_B` key-schedule mix, the new
  reject codes, and the responder-authentication property.
- Vectors: a positive round-trip; a wrong-responder negative (the reproduction test, flipped to REJECT);
  a fingerprint-mismatch negative; a malformed-`ek_B` negative.
- Claim matrix / DOC-CAN-003 §6.3: note that the shipped handshake now authenticates BOTH directions
  (previously only the initiator→responder direction held). No post-compromise/PQ claim moves.
