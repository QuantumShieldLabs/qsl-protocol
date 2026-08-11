Goals: G4 (primary), supports G1–G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-08-10

# DOC-OPS-008 — Design Questions & Non-Defect Register

Purpose: the companion to `DOC-OPS-007` (the Improvement & Findings Ledger). This register holds
items that **reproduce, or remain open, but are NOT defects**: design questions, product calls,
proposals for new work, and items accepted by operator ruling.

⚠ **WHY THIS REGISTER EXISTS, stated so it survives the lane that created it:**

> **The verdict vocabulary had no state for "reproduces, but is not a defect" and none for
> "accepted by ruling", so both landed in LIVE by default.**

That is the same gap that made the defect ledger grow a `P4` severity outside its own documented
`P0`–`P3` schema: **the ledger grew a severity outside its own schema because it had no register
for non-defects.** `P4` was the symptom; this file is the cure.

## The binding rules

- ⚠ **MOVING IS NOT CLOSING.** Nothing here is resolved, and nothing was discarded. Every entry
  keeps **its original id and its text verbatim**. `DOC-OPS-007` keeps a stub pointing here, and
  each entry below points back.
- **Ids are never reused and never renumbered.** An `ENG-####` that moves here keeps that id.
- **An entry moves back** to `DOC-OPS-007` if it is ever shown to describe a defect in shipped
  behaviour, with the measurement that showed it.
- ⚠ **A count of open defects must EXCLUDE this file.** Counting these as defects is what
  inflated the number this register exists to correct.

## Provenance

Seeded 2026-08-10 by **NA-0709 (D-1346)**, the ledger triage, from a read of all 169 `ENG` entries
at spine main `b845e678`. ⚠ **The move set was READ-DERIVED, not marker-derived** — and that was
load-bearing: the measured markers (an out-of-schema `P4`, a question-form heading) found **6**
candidates, while reading found **13**. ⚠ **Not one of the five outright design proposals
(`ENG-0022`, `ENG-0027`, `ENG-0029`, `ENG-0036`, `ENG-0037`) carries a `P4` or a question mark.**
A keyword sweep would have missed more than half of this file's contents.

⚠ **`FILING-ONLY` is NOT a criterion for this register.** In `DOC-OPS-007` that phrase is a
**provenance** marker meaning *"filed, not fixed in-lane"* — the house analysis-lane discipline.
49 entries carry it. Moving on that keyword would have evicted **42 live defects** from the defect
ledger.

---

## Entries

### ENG-0011 — Attachment upload/fetch timing and cover-traffic (deferred, cross-repo)
- Severity: P3 (metadata; deferred)
- Status: open — originating lane NA-0613 (D-1223); last-updated 2026-07-07
- Surface: qsl-attachments service/deployment (primary); optional qsc send/fetch jitter.
- Why it matters: upload/fetch timing and access pattern (C4) are observable by the
  service/network and are largely a qsl-attachments/deployment property, not a qsc-only
  concern; cover traffic is high-cost.
- Recommended directive shape: separate cross-repo design/implementation in
  qsl-attachments; optional small qsc-side jitter follow-up. Lower priority than ENG-0010.

> ⚠ **Moved from `DOC-OPS-007` 2026-08-10 by NA-0709 (D-1346). NOT closed, NOT resolved — this is a design question, a product call, or an item accepted by ruling. Text preserved verbatim.**

---

### ENG-0022 — DH-boundary cadence is an observable metadata distinguisher (G5)
- Severity: P3 (metadata; no confidentiality/integrity impact) — filed D-1239 from the NA-0622
  (ENG-0012 Stage 1b-ii) metadata decision
- Problem: with ratchet-on-reply, a Suite-2 DH boundary (FLAG_BOUNDARY + a fresh on-wire DH_pub)
  is observable and correlates with conversation turn-taking; these are the first boundary
  messages on the wire (PQ-reseed boundaries are Stage 2). The NA-0622 operator decision was
  ACCEPT + DOCUMENT (the leak is minor beyond what message timing/direction already exposes, and
  the bounded fallback prevents long silent gaps); the observable is recorded in DOC-G5-004.
- Recommended change: boundary-cadence obfuscation / cover traffic to blur the reply-correlation —
  e.g., decouple some ratchets from replies, or emit occasional cover boundaries. This is a
  protocol-wide G5 decision best made AFTER Stage 2 (PQ reseed) lands, alongside a holistic
  metadata pass; premature to bolt onto the ratchet lane.
- Recommended directive shape: G5 design lane (DOC-G5-004/DOC-G5-005 family) + a scoped
  qsc/refimpl source lane; sequence after ENG-0012 Stage 2. Deferred (consciously), tracked here.

> ⚠ **Moved from `DOC-OPS-007` 2026-08-10 by NA-0709 (D-1346). NOT closed, NOT resolved — this is a design question, a product call, or an item accepted by ruling. Text preserved verbatim.**

---

### ENG-0027 — Chunked / erasure-coded PQ control-plane transport (SPQR-style) with an always-progress state machine
- Severity: P3 (robustness + metadata; supersedes part of ENG-0022's scope) — filed 2026-07-09
  from the operator-directed Signal comparison study at the NA-0624 closeout (D-1244)
- Problem: our SCKA control plane ships MONOLITHIC envelopes (~1184 B FLAG_PQ_ADV, ~1088 B
  FLAG_PQ_CTXT). Consequences accepted at NA-0624: a lost/dropped ADV or reseed degrades to the
  classical status quo until the T_pq rotation; PQ control messages are size-distinguishable on
  the wire (the DOC-G5-004 §3.1 observable); cadence has idle gaps. Signal's production PQ
  ratchet (SPQR, signalapp/SparsePostQuantumRatchet) instead ERASURE-CODES the ML-KEM key and
  ciphertext into small chunks piggybacked on EVERY message header — any sufficient subset
  reconstructs, so an attacker must drop ALL traffic to suppress an epoch (loss-suppression
  becomes full DoS), per-message overhead is near-uniform (the distinguisher shrinks toward
  timing-only), and an explicit per-epoch state machine (SendingEK/ReceivingCT analogues) keeps
  both parties always making progress.
- Recommended change: a chunked PQ-transport design for the SCKA plane — polynomial/erasure
  encoding of ADV pubkeys + reseed ciphertexts across ratchet-message headers, an epoch state
  machine replacing the timer-only cadence, and (per SPQR's `SecretOutput::{Send,Recv}` shape)
  an API that tells the caller which chain the epoch secret mixes into. Wire-format change —
  a major design lane (DOC-CAN-004 §3 revision + refimpl + qsc + vectors), NOT a bolt-on.
- Recommended directive shape: a design lane first (DOC-G5-008/DOC-CAN-004 family, folding in
  what remains of ENG-0022's cadence-obfuscation scope), then staged implementation lanes;
  sequence after ENG-0023 (the frozen-receiver unfreeze it depends on). last-updated 2026-07-09

> ⚠ **Moved from `DOC-OPS-007` 2026-08-10 by NA-0709 (D-1346). NOT closed, NOT resolved — this is a design question, a product call, or an item accepted by ruling. Text preserved verbatim.**

---

### ENG-0029 — Evaluate migrating ML-KEM to a formally verified implementation (libcrux-ml-kem)
- Severity: P3 (assurance hardening; no known defect in the current dependency) — filed
  2026-07-09 from the Signal comparison study at the NA-0624 closeout (D-1244)
- Problem: we use the RustCrypto `ml-kem` crate; Signal's libsignal uses Cryspen's
  `libcrux-ml-kem`, whose ML-KEM implementation carries machine-checked functional-correctness
  and secret-independence proofs. Our KEM sits under every PQ epoch secret.
- Recommended change: an evaluation lane — API/feature fit (encap/decap/keygen surfaces used by
  `PqKem768` + `runtime_pq_kem_keypair`), maturity/audit trail, build/lockfile impact, and a
  byte-compatibility check against the existing SCKA-KEM conformance vectors; migrate only if
  the evaluation is clean (dependency mutation requires its own operator-approved lane under
  the standing rules).
- Recommended directive shape: a bounded dependency-evaluation lane (read/evaluate + report,
  then a migration lane on operator approval). last-updated 2026-07-09

> ⚠ **Moved from `DOC-OPS-007` 2026-08-10 by NA-0709 (D-1346). NOT closed, NOT resolved — this is a design question, a product call, or an item accepted by ruling. Text preserved verbatim.**

---

### ENG-0036 — Token-gated relay access for a private/self-hosted deployment (closed-network authorization) — **NEW; filed 2026-07-10 (operator product direction)**
- Severity: P3 (feature / deployment-hardening; NOT a confidentiality/integrity gap in the shipped E2EE — it is an ACCESS-CONTROL layer at the relay, orthogonal to message security)
- Status: open — filed 2026-07-10 from an operator product-direction note during NA-0628; last-updated 2026-07-10
- Idea (operator, verbatim intent): the relay/server generates an access token at install/setup; only apps that hold the token AND the server address can connect. Target: a niche "run-your-own highly-secure server" product for a small trusted group, paired with the forthcoming TUI/GUI.
- **Setup-time mode toggle (operator refinement, 2026-07-10):** server setup asks *"will this be a PUBLIC accessible server or PRIVATE?"* and drives the config from the answer. PRIVATE ⇒ token-gating ON (this ENG); PUBLIC ⇒ open relay (the default Signal-like posture, no token wall). The operator judges this "codes easy enough for both" — plausible, since it is a config branch over the EXISTING `relay_auth_header` path rather than a new transport. This toggle is also the natural seam for the metadata posture: it is where a deployment decides which protections apply (see ENG-0037 sealed-sender — high-value in PUBLIC mode, nice-to-have in PRIVATE where the operator is trusted). The UX MUST make each mode's security implications explicit so neither is mistaken for the other's guarantees. **These are options to weigh WHEN we reach that point, not a committed design.**
- **Grounding — this EXTENDS existing architecture, it does not start from zero.** The client/relay already carry a route-token / relay-auth-header mechanism (`qsl/qsl-client/qsc/tests/relay_auth_header.rs`; `route-token/header discipline`; auth-token resolution in the transport subsystem; a fail-closed `relay_unauthorized` state). ENG-0036 is the formalization of that into an install-time PROVISIONED, closed-network access credential with a specified lifecycle.
- **What it buys (state honestly):** a closed relay — unauthorized clients cannot connect/enqueue; reduced spam/DoS/enumeration surface; a "private network" property analogous to a WireGuard pre-shared key or a self-hosted-server registration token. Strong fit for the self-hosted niche where the operator runs the relay.
- **What it does NOT buy (must be stated in any spec/UX so it creates no false security):** it is NOT end-to-end security — the Suite-2 E2EE already protects message confidentiality/integrity against the relay. It does NOT hide who-talks-to-whom from the relay operator (that is the sealed-sender gap, still unfiled). It is a BEARER credential: whoever holds it can connect, so distribution, rotation, and revocation are load-bearing, and a leaked token opens the network until rotated.
- **Threat-model discipline (per the metadata roadmap rule "name the adversary"):** ENG-0036 answers "outsiders connecting to my private relay." It does NOT answer "the relay operator is the adversary" (in a self-hosted deployment the operator IS the relay, so that is an accepted posture) or "a global passive adversary" (mixnet territory, out of scope). It must not be marketed as more than closed-network authorization.
- Design questions for a future directive: token generation + entropy + storage at rest on the server; provisioning/enrollment UX (QR / paste / file) into the app alongside the server address; rotation + revocation + multi-token (per-device) support; interaction with the existing `relay_auth_header` path (extend vs replace); rate-limiting / lockout; and the exact "no false security" wording for the claim matrix.
- Recommended directive shape: a design-lock-first lane (threat model + token lifecycle spec before code), sequenced AFTER the crypto core is at its completion point and alongside the TUI/GUI work it serves. Cross-repo (qsl-server + qsc client).

> ⚠ **Moved from `DOC-OPS-007` 2026-08-10 by NA-0709 (D-1346). NOT closed, NOT resolved — this is a design question, a product call, or an item accepted by ruling. Text preserved verbatim.**

---

### ENG-0037 — Sealed-sender: hide sender↔recipient (the social graph) from the relay/qsl-server — **NEW; filed 2026-07-10 (was owed since NA-0622, never filed)**
- Severity: P3 (metadata; deferred post-Stage-2) — but it is the **flagship metadata item**: it is the concrete mechanism behind the operator's standing "eventually beat Signal on metadata" goal. Message content is already fully protected by Suite-2 E2EE; this closes a WHO-TALKS-TO-WHOM exposure to the relay operator, not a content break.
- Status: open — **filed 2026-07-10**; previously owed off a promised relay/sender-metadata audit since NA-0622 and never converted into a tracked item (the gap my own ENG-0036 entry flagged as "still unfiled"); last-updated 2026-07-10.
- The gap: today the relay/qsl-server observes enough to reconstruct the sender↔recipient social graph (route tokens, delivery routing, timing). Suite-2 hides message CONTENT from the relay; it does not hide the communicants' relationship from it. Signal's Sealed Sender is the precedent (studied — see the ROLLING_OPERATIONS_JOURNAL source-verification entry).
- **Prerequisite (operator's own stated plan): a relay/sender-metadata audit FIRST.** Enumerate exactly what qsl-server currently learns about who talks to whom (extend `docs/design/DOC-G5-004` the metadata-leakage surface review), THEN design sealed-sender off concrete findings rather than assuming the mechanism. "Prove; do not assume" applies to the threat surface too.
- Threat model it answers (name the adversary): **the relay operator / a party with relay logs.** This is EXACTLY the adversary that ENG-0036's access token does NOT address — the two are complementary, not substitutes. It does NOT answer a global passive network adversary (mixnet/Loopix territory, ENG-0022, far higher cost).
- **Interaction with the public/private server mode (ENG-0036):** in a PRIVATE self-hosted deployment for a trusted group, the operator IS the relay, so sealed-sender is lower-value (nice-to-have). In a PUBLIC deployment it is HIGH-value, because untrusted users + an untrusted operator see the full graph. The public/private setup toggle is the natural seam at which "which metadata protections apply" is decided.
- Recommended directive shape: analysis-first — the relay-metadata audit as its own lane (findings + severity), then a sealed-sender DESIGN lane (cross-repo: qsl-server routing + qsc client), then implementation. Do NOT collapse these; the audit may reshape the design. Sequenced post-crypto-core, alongside/after the metadata batch (ENG-0022/0027) and the private-server work (ENG-0036).

> ⚠ **Moved from `DOC-OPS-007` 2026-08-10 by NA-0709 (D-1346). NOT closed, NOT resolved — this is a design question, a product call, or an item accepted by ruling. Text preserved verbatim.**

---

### ENG-0061 — the wiped / "Vault erased" screen ships with no danger colour, because its heading was never red and the round-4a chrome strip removed its only signal
- Severity: P3 (cosmetic / signalling; NO runtime, protocol, or security impact — and **nothing can be mis-triggered by it**, see the acceptance ruling below)
- Exact surfaces: `qsl-desktop ui/index.html` — the wiped screen at **`:89-95`**, whose heading is a **plain** `<h1>Vault erased</h1>` carrying no danger class; `qsl-desktop ui/style.css` — `.card h1` sets size and weight but **no colour**, so it inherits `--fg` (#E8E8E8); the round-4a chrome strip that removed `.danger-card`'s border on that screen
- Description: **the F2 override's own stated rationale is factually false on this one screen, and that is why the item exists.** The override reasoned that stripping the danger border was safe because *"the 'Erase everything' / 'Vault erased' headings and the warning copy are already red."* **That premise holds for erase and NOT for wiped.** Erase is `<h1 class="ceremony-head">`, coloured `var(--danger-text)` by `.ceremony-card .ceremony-head` — a rule NA-0665 left untouched, and the operator's after-shot confirms it renders red. **Wiped's heading was NEVER red**; its only danger signal was the **`.danger-card` BORDER**, which the override instructed be stripped. The result, visible in the operator's own 11-44-34 after-shot: **a white heading and no border — no red anything.**
- **RULED ACCEPTED AS-IS by the operator (2026-07-22), and the reasoning is recorded because it bounds the severity:** the wiped screen is a **calm post-hoc NOTICE**, not an armed destructive gate. The data is **already gone**, nothing is being confirmed, and the only control is a primary "Start over". **Nothing can be mis-triggered by under-signalling it.** The operator's acceptance rested on **seeing the actual rendered pixels**, so what shipped is what was approved — **the error was in the verbal rationale, not in the approval.**
- Consequence: one pre-main screen conveys a destructive OUTCOME with no colour cue. Arguably correct for a neutral notice; arguably a gap against the override's stated intent. **The point of the filing is that the two readings differ and the choice should be deliberate rather than accidental.**
- Recommended change / scope for the future lane: **round 4c, revisited with the Settings-pane pass.** If red is wanted it is **one attribute plus one selector** — give the wiped `<h1>` the danger class and add the matching rule; if the calm-notice reading is preferred, **say so explicitly in Appendix E** so a later reader does not "fix" it back. **Do not fix it in isolation** — decide it alongside the rest of the danger-signalling vocabulary, so the app has one answer rather than per-screen accidents.
- Proof gap: no test asserts danger COLOUR on the wiped screen, in either direction. Whichever way round 4c rules, **the ruling should land as an assertion**, because this defect arrived precisely by a rule being changed with no test to notice the consequence on one screen.
- Status: open — filed 2026-07-22 by NA-0665 (D-1291), **ACCEPTED AS-IS by operator ruling and DELIBERATELY NOT FIXED FORWARD in-lane** (fixing it would have been scope the lane was not given). Deferred to round 4c.

> ⚠ **Moved from `DOC-OPS-007` 2026-08-10 by NA-0709 (D-1346). NOT closed, NOT resolved — this is a design question, a product call, or an item accepted by ruling. Text preserved verbatim.**

---

### ENG-0097 — file-completion receipts create `kind="file"` TIMELINE ENTRIES via the direct send path; whether that is intended is unresolved — **NEW; filed 2026-07-30 by NA-0688 (D-1327; directive D622), OBSERVATION ONLY, PRE-EXISTING**
- Severity: P3 (UI truth; **pre-existing, not introduced by NA-0688**, and no behaviour was changed by this lane)
- Status: open — filed 2026-07-30, **observation only**.
- **The fact.** `PendingReceipt::AttachmentComplete` routes through `transport::relay_send_with_payload`, which passes a `TimelineSendIngest { kind: "file", … }` **unconditionally on a successful push**, appending a timeline row in state `Sent`. So attachment-confirmation receipts have been writing timeline entries for as long as that path has existed.
- **The unresolved question.** `DESIGN_outbox_delivery_v1` §5 requires a delivery ack be *"invisible in their UI"*. A **file completion confirm** is arguably a different thing from a delivery ack and may legitimately belong in the timeline — but nothing states which reading is intended, and no test pins either.
- **Route:** Slice 4 live acceptance, where the operator sees the rendered timeline directly and can rule on what should appear. Cheaper to answer by looking than by argument.
- ⚠ Found while enumerating the side effects of the path D622 C0 originally proposed to move ALL receipts onto. Message-kind receipts do **not** create timeline entries — pinned by `na0688_eng0095_ack_nonce_barrier::an_ack_creates_no_timeline_entry_on_the_sender_of_the_ack`, which exists precisely so a future "let us use one send path" refactor cannot start writing them silently.
- Cross-reference: ENG-0095; ENG-0096; `DESIGN_outbox_delivery_v1` §5/§6; OBS-EC (marker layer vs user-cause layer).

> ⚠ **Moved from `DOC-OPS-007` 2026-08-10 by NA-0709 (D-1346). NOT closed, NOT resolved — this is a design question, a product call, or an item accepted by ruling. Text preserved verbatim.**

---

### ENG-0124 — the erase-countdown writers perform NO resize and stay unclipped only by a 20.0px accident: any countdown-block growth beyond 20px clips the ONLY abort affordance on a LIVE 30-second erase countdown — **NEW; filed 2026-08-08 by NA-0702 (D-1342; A1.1, the numbers verbatim) — FILING-ONLY**
- Severity: P3 (a NEAR-MISS, not a present defect — measured NOT clipping today; severity assigned at filing by the seat, labeled as such)
- The measurement (NA-0702 formalization, STOP_NA0702_001 §4 — the landed runner on a throwaway scenario, predictions written BEFORE the run, instrument positive capability proven in-run by capture 1 returning the button names on a clean form): the countdown block is SHORTER than the form it replaces — card **scrollHeight 217 == computed height 217, ZERO overflow**; `#btn-erase-countdown-cancel` measured bottom **225.0** vs the card clip at **245.0** = **20.0px margin**; ticks are TEXT-ONLY (geometry pixel-identical across 30→18→13); the REAL cancel click PASSES at the measured geometry.
- The near-miss, in plain words (A1.1): the countdown writers (base main.js :531–:532 swap-in, :509–:512 swap-back, :501–:502 tick) perform NO resize — **the safety is an accident of the current copy, not a property** — so **any growth of the countdown block beyond 20px clips THE ONLY ABORT AFFORDANCE ON A LIVE 30-SECOND ERASE COUNTDOWN, with no test to catch it.** A filed row recording only "does not clip" would justify inaction; the margin is what makes this a queued lane (A1.1's rationale, adopted verbatim).
- Remedy candidate, not this lane: extend the one-resizing-writer property to the countdown swap, or pin the countdown geometry with a test that fails when the block outgrows the clip. Successor QUEUED BY THIS FILING per the NA-0702 block.
- Status: open — filed 2026-08-08 by NA-0702. FILING ONLY; nothing fixed (D637 §6/§10: countdown code byte-untouched, measured at the committed tree — no changed line in the diff touches countdown code).
- Originating/last lane: NA-0702 (D-1342; R174 A1.1).
- Last-updated: 2026-08-08.

> ⚠ **Moved from `DOC-OPS-007` 2026-08-10 by NA-0709 (D-1346). NOT closed, NOT resolved — this is a design question, a product call, or an item accepted by ruling. Text preserved verbatim.**

---

### ENG-0125 — the rail-toggle (hamburger) design exists ONLY in operator-side mockup 02, which is SUPERSEDED-HISTORY and never ships: the pattern is one operator-side file away from being lost — **NEW; filed 2026-08-08 by NA-0703 (D-1343; R180 §2.5, ordered) — FILING-ONLY**
- Severity: P4 (a design-preservation horizon, not a defect; nothing shipped is wrong)
- The pattern as recorded in mockup 02 (Signal-style): the hamburger toggles the RAIL; rail shown = hamburger at top of rail, columns shift right; rail hidden = the hamburger MOVES INTO the column header so recovery is one click (hamburger, then destination icon) — the control relocates rather than disappearing. NOT implemented; mockup-11 carries no rail-toggle content, so NA-0703's committed set does not preserve it. Mockup 02 stays operator-side because it carries a live tailnet hostname (its SUPERSEDED-HISTORY class is load-bearing).
- Remedy horizon: a future rail-touching lane (Slice 4) re-draws these two states as a SANITIZED mockup before the pattern is lost. Rationale on the record (R180): NA-0703 MOVES ratified design and does not draw new design; this filing costs nothing and is what stops the pattern disappearing.

> ⚠ **Moved from `DOC-OPS-007` 2026-08-10 by NA-0709 (D-1346). NOT closed, NOT resolved — this is a design question, a product call, or an item accepted by ruling. Text preserved verbatim.**

---

### ENG-0128 — whether existing `QSCV01` vaults deserve a migration path at all — **NEW; filed 2026-08-09 by NA-0705 (D-1344; R185 §2.5) — FILING-ONLY, PRODUCT CALL**

`QSCV01` → `QSCV02` is a hard break with no migration and no dual-format read (D628 Ruling 2, stated verbatim in `qsl/qsl-client/qsc/src/adversarial/vault_format.rs:6-7`). NA-0705 made the refusal HONEST on both desktop doors, which is owed regardless of population; whether anyone should be able to OPEN such a vault again is a separate product question. Pre-release, the population is plausibly developer and operator vaults only — but that reach judgment has not been measured and is not this lane's to make.

> ⚠ **Moved from `DOC-OPS-007` 2026-08-10 by NA-0709 (D-1346). NOT closed, NOT resolved — this is a design question, a product call, or an item accepted by ruling. Text preserved verbatim.**

---

### ENG-0133 — what signal truthfully reports send capability at `32e572c7`? — **✅ ANSWERED 2026-08-09 by NA-0708 (D-1345), enriched — filed 2026-08-09 by NA-0705 (D-1344; R191 §3)**

⚠ **ANSWERED, AND THE QUESTION ITSELF WAS AIMED SLIGHTLY WRONG.** Three things settle it. (i) `send_ready` is **UNRELATED to capability** — it is a state-of-the-store predicate (`protocol_state/mod.rs:108`), which is why it can report `no` while the send succeeds. (ii) The signal the design actually needed was **SAFETY, not capability**: whether sending now is safe, not whether it is possible. (iii) ⚠ **The deeper point, and it redirects the whole question:** the harm this filing circled is a **RECEIVE-PATH loss, not a send-capability gap** — the payload was never cryptographically destroyed (`ratchet_skip_store count=2`); it died because the receive pull aborted and the frame behind it was never unpacked. See ENG-0134 and the reject-vocabulary normalisation successor for where the real defect and its remedy live. ⚠ The mechanism behind the `chainkey_unset` window is now measured and filed separately as **ENG-0168**: only the handshake INITIATOR has seeded send chains at establishment.

`send_ready` UNDER-REPORTS for the responder between its first inbound message and its first outbound one: measured at the new pin, the responder reports `send_ready=no / send_ready_reason=chainkey_unset` and **sends successfully**. R185 §2.2's composer gate is SUSPENDED as a result (D-1344 §6). Candidates to MEASURE, not assume: whether `chainkey_unset` is distinguishable from a real cannot-send state; whether the owed-receipt hold (`receipt_owed reason=chain_unseeded`) has an observable that closes the window; whether attempting a send and handling the refusal is more honest than predicting capability. ⚠ **Nobody derives a UI rule from NA-0705's n=1.** The design lane measures at n>1 and rules with the operator.

> ⚠ **Moved from `DOC-OPS-007` 2026-08-10 by NA-0709 (D-1346). NOT closed, NOT resolved — this is a design question, a product call, or an item accepted by ruling. Text preserved verbatim.**

---

### ENG-0169 — the Legacy ack mode's weaker guarantee — deprecate, or keep and document? — **NEW; filed 2026-08-09 by NA-0708 (D-1345; RS-B, ruled accept-and-record at R201 §2) — FILING-ONLY, AN OPEN OPERATOR QUESTION**

Lease has been the default since D-1327 C4, and under Legacy the relay delete-on-delivers, so a client that cannot process an item has already lost it. NA-0708's own fix is **Lease-only by construction** (the ack accumulator is always empty under Legacy). ⚠ The question is whether Legacy still earns its place, and it is its own lane, not a side effect of one.

> ⚠ **Moved from `DOC-OPS-007` 2026-08-10 by NA-0709 (D-1346). NOT closed, NOT resolved — this is a design question, a product call, or an item accepted by ruling. Text preserved verbatim.**

---

