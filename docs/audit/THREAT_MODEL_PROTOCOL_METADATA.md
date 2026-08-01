# Threat Model: Protocol + Metadata

## Trust Boundaries
- Relay/server is untrusted for confidentiality and integrity of message/file payloads.
- Network path is untrusted and can be observed and modified.
- Local host security differs by state:
  - Locked state: vault-backed protections should prevent plaintext-at-rest exposure.
  - Unlocked state: local compromise risk increases; protections are reduced to process/runtime controls.

## Attacker Models
1. Passive network observer:
- Observes timing, packet sizes, frequency, and endpoint correlation signals.
- Cannot decrypt ciphertext directly.

2. Active network attacker:
- Attempts replay, injection, reordering, suppression, and downgrade signaling.
- Attempts to induce false delivery/receipt claims.

3. Malicious relay:
- Manipulates store/forward behavior.
- Performs traffic-correlation attempts and selective DoS.
- Cannot be trusted for truthfulness of transport events.

4. Local attacker:
- Attempts disk access (vault/config artifacts) in locked and unlocked states.
- Attempts runtime memory scraping under host compromise assumptions.

## Protected Assets
- Message/file confidentiality and integrity.
- Identity authenticity and key-binding correctness.
- Session-state correctness and anti-replay properties.
- Metadata minimization goals (content-independent observables reduced as feasible).

## Target Security Properties
- Authentication:
  - Peer identity and session establishment must be bound to cryptographic evidence.
- Transcript binding:
  - Session progression must correspond to validated handshake/transcript context.
- Forward Secrecy (FS):
  - Past ciphertext should remain protected if current long-term material is compromised later.
- Post-Compromise Security (PCS):
  - After compromise and subsequent honest key evolution, future messages should recover confidentiality/integrity.
- Replay resistance:
  - Duplicate/reordered/injected protocol events are rejected or handled without false state claims.
- Metadata goals:
  - Minimize leakage of sensitive identifiers/content hints in UI, logs, and transport-visible metadata.
  - Explicitly acknowledge what cannot be hidden (timing/size/availability classes).

## Delivery Receipts: Default, Mechanism, And What It Does Not Hide
*(NA-0688 / D-1327, directive D622 R2a/R2b/R2d. Written against the MEASURED mechanism.)*

**Default (both halves, since NA-0688):** a client requests a delivery receipt on the messages it
sends, and answers the ones it receives. One setting governs both — turning receipts off stops the
asking as well as the answering. `--receipt off` suppresses the request for a single message;
`--receipt-mode off` suppresses answering.

**The mechanism is the OUTBOX CADENCE, not a timing window.** A receipt is a durable queued row.
Receipts are coalesced into the **end-of-pull flush**, so a pull that delivers four messages
produces **one** send rather than four. There is **no wall-clock deferral** in v1: the
`RECEIPT_BATCH_WINDOW_MS` constant is read only to echo it into a diagnostic marker and nothing
waits on it, and the jitter value is a stable-sort key bias that reorders receipts within a flush
without delaying any of them. **Neither knob attenuates a timing signal, and neither should be
described as if it did.**

⚠ **DELIVERED IS NOT AVAILABLE UNTIL THE RECIPIENT HAS SENT AT LEAST ONCE.** This is a product
statement, not a test detail, and it is a direct consequence of the design being correct rather
than a shortcoming to be worked around:

> With acks on by default and A6 reversed, DELIVERED is not available for a message until its
> recipient has sent at least once; until then the sender sees SENT, the receipt waits in the
> durable hold, and it flushes to DELIVERED on the recipient's first send.

**Why it is this way.** A control send originates no cryptography, *including chain establishment*
— because an establishing ack mints a fresh DH keypair and advances the shared root, which was
measured to wedge sessions permanently and in both directions. A recipient who has never sent
therefore has no chain an ack can ride, so the obligation is recorded durably instead of dropped.
**Nothing is lost:** the receipt is delivered on their first send, and both halves of that window
are pinned — the SENT state during it, and the transition to DELIVERED after it.

**UX options that could shrink or reframe this window** — none implemented here, all Slice-4
design candidates: suppressing delivery indicators until a conversation is bidirectional;
proactively establishing the send chain (prekey-style) so a recipient is never chainless; or having
the app generate a first message on the recipient's behalf. The first act of any such lane is
measuring what the invite bundle already publishes.

**Accepted consequences, recorded rather than mitigated:**
- A DELIVERED state may lag by up to one drain interval. `qsc` is not a daemon; a CLI drain
  happens on the next invocation.
- A lost receipt is not retried on its own schedule. It self-heals through redelivery, because a
  duplicate incoming message is re-acked idempotently — a property that **depends on lease being
  the receive default** (ENG-0043).

**What an observer of the relay can still see. This list is deliberately not reassuring:**
- **Traffic existence and timing between two mailboxes remain observable.** Every receive
  eventually produces a send. Coalescing changes the count from per-message to per-pull; it does
  not remove the correlation.
- **An ack is distinguishable from a user reply by envelope SIZE in v1.** Measured from the
  relay's stored bytes: an ack is padded to the Standard **1024**-byte floor, while user messages
  are **unbucketed** — so a short reply happens to measure 1024 too, and a 4096-byte body measures
  **17682**. Any message that does not fit under the floor is therefore distinguishable from an ack
  by length alone. Padding receipts further cannot close this, because the ack is already the
  padded one; only bucketing the user path would. See ENG-0098.
- **Envelope COUNT is a second signal.** A user send that also carries a key advertisement emits
  two envelopes back-to-back where an ack emits one.
- **Cover traffic and default user-path bucketing are deliberately post-Stage-2.** They are the
  two halves of the same problem and are scheduled to be ruled together.

⚠ **No immunity is claimed.** Message *contents* stay inside the session AEAD, and none of the
above is a confidentiality claim about plaintext — it is about what the shape and timing of
traffic reveal, which in v1 is a real and unmitigated class.

## Must-Never-Happen Invariants
- Client claims DELIVERED/received states without explicit protocol evidence.
- Protocol marked ACTIVE without proven session establishment.
- Sensitive content or identifiers leak while client is locked.
- Invalid transitions mutate persisted state when reject path should be fail-closed.
- Relay-originated events are treated as trusted truth without verification.

## Mapping To Existing Artifacts
- Message-state truth semantics:
  - `qsl/qsl-client/qsc/tests/message_state_model.rs`
  - `qsl/qsl-client/qsc/tests/receipts_delivered.rs`
- File-transfer integrity/truth semantics:
  - `qsl/qsl-client/qsc/tests/file_transfer_mvp.rs`
- Locked-state and leakage behavior:
  - `qsl/qsl-client/qsc/tests/tui_locked_cmd_init_ux.rs`
  - `qsl/qsl-client/qsc/tests/tui_lock_unlock_polish.rs`
  - `qsl/qsl-client/qsc/tests/tui_autolock.rs`
- Prior canonical UI/security references:
  - `docs/qsc/QSC_TUI_SPEC.md`
  - `docs/qsc/QSC_TUI_INVARIANTS.md`

## Notes For NA-0133 / NA-0134
- NA-0133 should test these target properties against current protocol behavior and identify gaps.
- NA-0134 should quantify metadata leakage classes and map each to mitigations plus residual risk.
