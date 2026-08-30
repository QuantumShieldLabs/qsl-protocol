# QSL THREAT MODEL — WHAT EACH ADVERSARY LEARNS, MEASURED

Status: DRAFT (NA-0772 / D-1415). Every row below is cited to a source line, a log
field, or a run. Rows resting on a read rather than a run say so. Where something was
not measured, it says that too, with its reason.

Bases: qsl-protocol `25e6f14961d6da6188924c891cd1f459b8c507da` · qsl-desktop
`11f695dfcb3e6c1f3b3ff78a14eee71e878b0439` · **qsl-server read and run at
`37ec82072cbbd68e4eaba83e192282fbcb96e5b4`**, the revision the operator's measurement
brackets as the deployed relay.

**Every relay row below states "at 37ec8207".** The operator measured the AWS binary
(sha256 `3439aa04…`, installed 2026-08-11T02:05:06Z) and bracketed it to `131d63f4`
(exclusive) … `37ec8207` (inclusive) — seven commits, all 2026-07-29. The bracket is not
closed to a point: a local rebuild does not reproduce the deployed digest (this lane's
build of 37ec8207 measures `342d6e3c…`, not `3439aa04…`), because Rust binaries embed
build paths and toolchain identity. **No row claims the deployed revision is identified.**
⛳ What *is* established: `src/store.rs` and `src/main.rs` are **byte-identical at all
nine revisions** from the old pin `131d63f4` through current main `f201bb3a`, so every
row resting on those two files holds whichever revision is deployed. Only `src/lib.rs`
moves, and it is identical at 37ec8207 and at current main. **The eleven-commit sweep FROM THE PIN to
main is otherwise unmeasured by this lane and this document does not claim it.**

---

## WHERE THIS DOCUMENT SITS AMONG THE REPO'S EXISTING ONES

This document **does not supersede** either existing threat model. Both remain in force
for what they cover.

| document | sha256 at this base | what it covers that this one does not |
|---|---|---|
| `docs/audit/THREAT_MODEL_PROTOCOL_METADATA.md` | `7da54936…be80e6` | Trust boundaries and **protocol/cryptographic** attacker models — the active network attacker (replay, injection, reordering, suppression, downgrade), local-host attackers in locked and unlocked states, and the target properties **Forward Secrecy** and **Post-Compromise Security**. It also carries the measured **delivery-receipt** findings this document cites at C3. |
| `docs/privacy/DOC-G5-001_Metadata_Threat_Model_v1.0.0_DRAFT.md` | `c04f3ff0…d76c79` | A metadata-category taxonomy for the **Suite-2 demo transport**, scoped by its own line 6 to *"non-production"*. ⚠ Its baseline "reject unauthenticated register/send/poll/bundle requests" is **not met by the shipped relay**, which reports `"auth":{"mode":"open"}`; that clash is bounded by its own non-production scope, and **this document, not DOC-G5-001, describes the shipped relay.** |

**What this document adds that neither has:** a deployment model, an adversary × asset
matrix, per-cell citation to a line/log field/run, and the relay-operator and
seizure adversaries measured against the shipped relay.

---

## PART ONE — THE FIVE-MINUTE VERSION

**What this product protects.** Message content is encrypted end to end. The hybrid
post-quantum construction (ML-KEM-768 + ML-DSA-65) is the protocol's, stated by the
governing kickoff's sec 4 and by `docs/audit/THREAT_MODEL_PROTOCOL_METADATA.md` — **this
lane measured the relay side and does not independently attest the cipher suite.** What
this lane did measure is the consequence: the relay performs **no cryptographic operation
of any kind** on a stored frame. It stores the bytes it is handed and hands them back.

**What this product does not protect, and will not claim to.** The delivery layer leaks
*metadata*: that a mailbox received something, roughly when, and how large it was. That
is the honest limit of this design, and it is stated here rather than buried.

**Who runs the relay.** One organization running the relay for its own members — a firm,
a clinic, a newsroom. The relay operator is that organization's own administrator, treated
here as **semi-trusted**: honest, but able to read everything the relay holds. Two other
deployments are described later but not designed for.

**What a subpoena or a seizure of the relay yields.** The buyer's first question, answered
at the level a seizure actually operates at — **the files on disk, not a database query.**

> **There is no REQUEST log.** At the shipped default the relay installs no request-logging
> middleware of any kind — no `TraceLayer`, no `.layer()` call anywhere, no client IP
> captured anywhere — and emits nothing per push, pull or ack. Measured at 37ec8207 across
> a scripted exchange: **0 bytes.**
>
> ⚠ **But the default is ERROR, not OFF.** `EnvFilter::from_default_env()` with the
> variable unset admits ERROR level, and the relay has **nine** `tracing::error!` sites,
> **one of them on a live request path** (`lib.rs:1019`, inside `run_store`); a second
> (`lib.rs:480`) is the background retention sweeper, and the remaining seven are startup
> and configuration. Three arms with `RUST_LOG` unset produced timestamped ERROR lines of
> **95, 109 and 117 bytes** (a port clash, an unopenable store path, a malformed config).
> **A long-lived relay accumulates a timestamped record of its own faults and nothing
> else** — those lines carry no mailbox identifier, no route token and no message body
> — **seven of the nine emit a fixed error code; two interpolate the store's own error
> string, which was not observed firing (n=0).**
>
> ⚠⚠ **And a seizure takes the FILES.** The relay runs SQLite in WAL mode with
> `secure_delete`, `VACUUM` and `auto_vacuum` all absent from the source. So when a
> conversation is fully delivered and acknowledged, **every query of the `routes` and `messages` tables
> returns nothing — and the bytes are still on disk.** Measured at 37ec8207 on a fully-drained
> mailbox: SQL reported `routes` 0 rows and `messages` 0 rows, while the raw files still
> contained the message body **3 times**, its message id **3 times**, and the mailbox
> pseudonym **9 times**. A party holding the files can recover recent deleted traffic with
> ordinary SQLite. **This is the single most important correction in this document, and an
> earlier draft of it claimed the opposite.**

What a seizure therefore yields: the undelivered backlog *and* recoverable recent deleted
traffic; one row per mailbox holding undelivered mail, with creation and last-activity
timestamps; **invite tombstones** recording when each invite was created and whether and
when it was consumed — a record of *when two parties became contacts* — for the invite's
own lifetime; and a lifetime message counter that survives every delete. Nothing in the
store holds a phone number or an account name, and **the relay never records who sent
anything**: there is no sender column anywhere in its schema, and a push carries only the
destination. ⚠ Sender attribution is still available to the relay operator by correlating
IP, TLS session and timing at the network layer — it is simply not a protocol field.

**Three things this document asks you not to be reassured about.**

1. **The mailbox pseudonyms are reversible by anyone holding the route token.** In logs a
   mailbox appears — **if logging is raised above the shipped default** — as a 16-hex `channel_id`; at rest as a 64-hex `route_key`. The first is
   unkeyed FNV-1a, the second unkeyed SHA-256. Both were reimplemented independently and
   reproduced every observed identifier, with a negative control matching none. ⚠ **Attribution:** the reimplementations reproduce at 37ec8207; the negative control is the FIRST read's, run at the old pin, and it transfers because `sha256_hex`, `route_key_for` and `channel_log_id` are byte-identical across the bracket. Since the
   route token is the value published in an invite, every contact of a member — and any
   administrator who can see an invite — can confirm a mailbox at will.
2. **The relay "still knows its own mailbox truth" at every rung of the delivery ladder.** Shaping
   traffic (ladder rungs 2 and 3) hides *timing and size* from a network observer and
   hides *sender rhythm* from the relay; it does not hide *that a mailbox received mail*
   from the relay, which is the party being asked to route it. The academic cure is
   private information retrieval — a horizon, not a promise. ⚠ Rungs 2 and 3 **do not
   exist**; that sentence is the banked design's, not a measurement.
3. **A global adversary watching both endpoints retains residual correlation.** That is
   mixnet territory, and this document carries it as a boundary rather than solving it.

**No compliance claim of any kind is made in this document.**

---

## PART TWO — THE MEASURED TABLES

### The adversaries

| id | adversary | capability assumed |
|----|-----------|--------------------|
| A1 | The org administrator, honest but curious | full read of the relay's store and logs |
| A2 | A subpoena or seizure of the relay | the **files** at rest, plus whatever logs exist |
| A3 | A breach of the relay | A2, plus live observation from that moment on |
| A4 | A malicious insider who can also **inject** | A3, plus pushing forged frames to any mailbox whose token they hold |
| A5 | The network **observer** between member and relay | TLS in place; sees timing, size, and the relay's address |
| A6 | A malicious member | holds valid tokens for their **own** contacts |
| A7 | The global observer at both ends | the ladder's stated honest limit; carried, not solved |
| **A8** | **The active network attacker** | **replay, injection, reordering, suppression, downgrade signalling; inducing false delivery/receipt claims** |

⚠ **A8 is ADDED by this lane** (the governing kickoff permits adding, not removing). It is
carried by `docs/audit/THREAT_MODEL_PROTOCOL_METADATA.md` as its attacker model 2, and the
other seven have no home for it: A5 is an *observer*, and A4's injection is scoped to an
insider **at the relay**. ⚠ **Its cells are the protocol's, not this lane's**: A8 is
answered by the existing model's target properties (authentication, transcript binding,
replay resistance, FS/PCS) and by the handshake's own construction. **This lane measured
the relay, not the protocol**, so A8's row below states what the *relay* gives an active
attacker and defers the protocol half by reference rather than asserting it.

### The assets

| id | asset |
|----|-------|
| C1 | Message content |
| C2 | Who talks to whom (the contact graph) |
| C3 | When (timing) and how much (size) |
| C4 | Who is online (presence) |
| C5 | Identity linkage — what ties a route token to a person |
| C6 | Availability — what an adversary can deny |

---

### T1 — THE SHIPPED DESIGN, AT 37ec8207

*One route token per account · rung 1 jittered polling · `ack=lease` · 60 s lease ·
7-day retention TTL.*

| | **C1 content** | **C2 contact graph** | **C3 when / how much** | **C4 presence** | **C5 identity linkage** | **C6 availability** |
|---|---|---|---|---|---|---|
| **A1** org admin | **Nothing.** Bodies are opaque blobs; **the relay performs no cryptographic operation on a stored frame** — the only crypto it performs is hashing of tokens, capabilities and credentials, never of content. By **frame format**: it can read the cleartext magic `QHSM` (`handshake/mod.rs:22`, `:459`), the version, the frame **type**, and the 16-byte `session_id` — so it can tell a handshake frame from a message and which stage it is, and nothing more [READ] | Sees all traffic to one account converge on one mailbox — but **not who sent it**: no sender column anywhere in the schema, and `push_message` (`lib.rs:1026-1040`) takes only destination + body. Sender attribution needs network-layer correlation [READ+RUN] | Per-message arrival time and size at rest (`messages.enqueued_at`, `length(body)`); with logging on, `bytes=` and µs timestamps [RUN] | Poll instants. Rung-1 beat **15–25 s** while unlocked (`ui/main.js:1904`, `:1911`; `TICK_DEFAULT="instant"`, b=20000 j=5000) [READ, not observed] | **The invite bundle is the account's long-term identity public keys** `(kem_pk, sig_pk)`, stable per account (`invite/mod.rs:235-249`; `identity/mod.rs:566-580`) ⇒ every invite one account creates links to every other by byte equality. Bearer is **relay-wide, not per-member** (`lib.rs:961-981`) so it identifies the org, not the person [READ] | **Global** route cap 256 (`store.rs:382`, `:674`); per-route queue depth 257; 7-day TTL; `ENG-0198` budget exhaustion — see the C6 subsection [READ] |
| **A2** subpoena / seizure | Nothing | **The files, not the query.** See the correction below: deleted traffic is recoverable | `routes.created_at`/`last_touched`; `messages.enqueued_at` and size; **`invites.created_at`/`consumed_at`/`state`** [READ] | Only `last_touched`, and only for productive pulls | `invites.bundle`; `route_key` = **unkeyed SHA-256**, confirmable by a token-holder; **`cap_hash` likewise confirmable by anyone holding the invite code** [READ] | Live-route and invite-slot occupancy |
| **A3** breach | Nothing | A2 **plus the route token in the clear** — it crosses the boundary in the `x-qsl-route-token` header on every request and is only hashed for storage. **This is the discriminator between a breach and a seizure** | A2 plus live µs timing if `RUST_LOG` is raised | Live poll instants | Holds the raw tokens, so computes every pseudonym | Drop, delay, reorder at will |
| **A4** insider who injects | Nothing | A3 | A3 | A3 | A3 | **F-B: an A1 (`HsInit`) frame carries no authenticator** — no MAC, no signature — so any token-holder can create a responder pending record. **Cost: one push. Bound:** since NA-0771 a forged frame no longer *destroys* the record (14 clear sites → 4); the residual is `ENG-0198` budget exhaustion at N ≥ `--max` (default 4) |
| **A5** network observer | Nothing (TLS) | **Nothing by address** — the token is inside TLS | Packet timing and size. Handshake frames **4279 / 6436 B** (NA-0736, carried from that lane's sealed evidence, **not re-measured here**) | Client-online at poll instants, quantized to the 15–25 s grid | The **relay's address**, carried in the invite payload (`relay_ep`) | Can block, not selectively by mailbox |
| **A6** malicious member | Nothing | Cannot read others' mail without their token. No directory, no key transparency to enumerate against — measured in `server-info`: `"directory":{"mode":"none"}`, `"kt":{"mode":"none"}` [RUN] | — | — | — | **Can exhaust the GLOBAL 256-route cap** with 256 fresh tokens, after which **no new mailbox can be created relay-wide**; likewise the global invite-slot cap. Rate limit and queue depth are **per-route** and do not bound it. ⚠ `server-info` is unauthenticated and advertises `"auth":{"mode":"open"}`, `max_slots` 256, `max_queue_depth` 257, `max_body_bytes` 1048576 and the 7-day TTL — i.e. the exact budgets to exhaust, and that no credential is needed. The route cap is *not* advertised |
| **A7** global observer | Nothing | Residual end-to-end correlation | Residual | Residual | Residual | — |
| **A8** active network attacker | Nothing from the relay (TLS + AEAD) | The relay offers it no contact-graph capability beyond A5's | — | — | — | **From the relay:** nothing A5/A4 do not already give. ⚠ **The protocol half — replay, reordering, suppression, downgrade, false delivery claims — is `docs/audit/THREAT_MODEL_PROTOCOL_METADATA.md`'s, whose target properties (authentication, transcript binding, replay resistance, FS/PCS) answer it. NOT MEASURED BY THIS LANE; n=0** |

#### Two corrections to this table, both made by measurement rather than by reading

**(1) A fully-delivered conversation does NOT leave the disk clean.** Two earlier drafts got
this cell wrong, each through an instrument that could not see the thing being claimed —
first a *reading* of the retention sweep, then a *SQL query*. **SQL is a model of the store;
the store is a directory of files, and a seizure takes the directory.**

Measured at 37ec8207, on a mailbox drained by push → pull → ack:

| arm | instrument | result |
|---|---|---|
| A | the SQL view | `routes` **0 rows**, `messages` **0 rows** — exactly what the earlier draft claimed |
| B | the raw files | body `CIPHERTEXT-SIERRA-…` **×3**, msg_id **×3**, `route_key` pseudonym **×9**, `log_id` pseudonym **×2** |

Cause, at 37ec8207: `store.rs:208-212` sets exactly three pragmas — `journal_mode=WAL`,
`synchronous=FULL`, `foreign_keys=ON`. Needle count across `src/` for `secure_delete`,
`wal_checkpoint`, `VACUUM`, `auto_vacuum`, `journal_size_limit`: **zero**. SQLite therefore
leaves deleted row images in WAL frames and freelist pages until they are reused.
⛳ The raw route **token** appears **0 times** — that at-rest property does hold.
⚠ `sqlite_sequence` retains a **lifetime message count** (`messages seq=1` after the full
drain), so "leaves no row" is false at the table level too.

**What is true, and is a good claim:** the relay's *logical* store retains no row for a
fully-delivered conversation, so an operator running ordinary queries sees nothing; the
residue on disk is unindexed, bounded by page reuse, and requires file-level access.
⚠ How long a page survives before reuse depends on traffic volume and **was not measured**;
n=0 field measurements.

**(2) The relay can link the two ends of a handshake by a CLEARTEXT correlator, whatever
the addressing.** The 16-byte `session_id` sits at a fixed offset in **all three** handshake
frame types (`handshake/mod.rs:541` A1, `:626` B1, `:685` A2). A1 and A2 land in the
responder's mailbox and B1 in the initiator's ⇒ **the same correlator appears in frames
stored in both peers' mailboxes**, and the relay pairs those mailboxes by exact byte
equality with no cryptography. ⇒ **a channel per-contact route tokens do not touch.**

#### C6 in full: what an adversary can DENY, and what bounds it

- **`ENG-0198`'s poison shape.** Strict insertion order (`store.rs:723`) and a lease that
  moves no message (`:752`) mean N ≥ `--max` (default 4) unconsumable frames at a mailbox
  head exhaust the caller's budget. NA-0771 raised the threshold from one frame to `--max`
  and **did not remove it**.
- **`F-B`'s pending creation.** Cost one push; bound as in the A4 cell.
- **Saturation under the 7-day TTL.** `RETENTION_TTL_SECS` 604800 (operator-measured,
  `D-1411` `DV-11`; corroborated as the compiled default at 37ec8207). This **bounds a
  finite flood** and **does not bound a sustained one**: a pusher refilling faster than the
  sweep drains keeps the head occupied indefinitely. Per-route `max_queue_depth` 257 caps
  depth, so the denial's shape is head-of-line starvation, not unbounded growth.

#### C3 and C2: delivery receipts — a channel already measured, carried forward

`docs/audit/THREAT_MODEL_PROTOCOL_METADATA.md` records, from the relay's stored bytes:

> "Traffic existence and timing between two mailboxes remain observable. **Every receive
> eventually produces a send.** Coalescing changes the count from per-message to per-pull;
> it does not remove the correlation."
> "An ack is distinguishable from a user reply by envelope **SIZE** in v1 … an ack is
> padded to the Standard **1024**-byte floor, while user messages are unbucketed … a
> 4096-byte body measures **17682**." "Envelope **COUNT** is a second signal."

⇒ delivery receipts are on by default and automatic, so **every delivered message generates
a return push**, and the relay observes a bidirectional pair even when the human never
replies — with the returning envelope size-distinguishable from a real reply. **This is a
C2/C3 channel that per-contact route tokens do not close either.** Cited from the existing
model's measurement; **not re-measured by this lane.**

---

### T2 — UNDER PER-CONTACT ROUTE TOKENS

**The send side genuinely improves.** Two contacts of one account no longer push to the
same address, so the relay cannot link them **by destination address** — and, more than
that, it can no longer join a member's outbound pushes to the recipients' **account-stable**
self-inbox addresses.

**The pull side does not improve.** The account must pull N mailboxes from one client
session, one IP, inside one beat window; the relay links them by session, IP and timing
regardless of address. No relay-side cure exists at this base.

⚠ **STRUCK — a claim an earlier draft made, measured FALSE.** That draft asserted the pull
correlation is *recorded at rest* because "`routes.last_touched` is written on every touch".
**It is not written on every touch.** At 37ec8207 the post-pull `UPDATE` is guarded by
`else if !seqs.is_empty()` (`store.rs:783-787`): an **empty pull writes nothing**, and a
pull on a never-pushed route **creates no row at all**. `last_touched` is written on enqueue
and on a lease-pull *that returns mail*, and on nothing else. ⇒ **the polling beat leaves no
at-rest signature whatever.** The claim is struck, not downgraded; the ruling does not need it.

**What changes if pulls are BATCHED, SPREAD, or ROUTED**

- **BATCHED** — one request carrying all N addresses would remove the per-pull timing
  signal, but **the relay does not offer it**: NA-0768's menu states *"there is no
  multi-mailbox pull at this base"*. It is a relay-side change, and it would hand the relay
  the contact set explicitly in one request.
- **SPREAD** — scattering the N pulls attenuates same-instant correlation and costs latency
  proportional to N. It does not defeat A1/A2, who see the pulls arrive on one TLS session
  from one IP, and it **cannot hide the count**.
- **ROUTED** — distinct network paths per mailbox (Tor, separate sessions) is the **only**
  one of the three that addresses the pull side at its root, because it breaks the
  session/IP join rather than the timing join. It is also the one this product does not
  have, has not designed, and has not scheduled.

⇒ two of the three do not exist at this base and the third is a relay-side change that
makes the contact set explicit. **This is why ruling (a)'s honest form is
*IN-WITH-PULL-SIDE-CHANGE*, and why, absent that change, the answer is OUT.**

**The global route cap is consumed N× faster** — 256 is relay-wide, so per-contact tokens
reduce the accounts a relay can serve roughly in proportion to contact count.

---

### T3 — AT LADDER RUNG 2 (SHAPED PUSH, ONE HELD CONNECTION)

For the relay to wake a held connection it must know **which mailboxes to wake it for**, so
with per-contact tokens the client subscribes one connection to all N addresses.

⇒ **the contact set stops being an inference from timing and becomes an explicit, durable
declaration** held by the relay in one object. Against the relay, per-contact tokens are
therefore **worse at rung 2 than at rung 1**. Against the *network* observer rung 2 is an
improvement — but that comes from the shaping, not from the tokens.

The ladder order's R6 wording binds and is stated in full: the relay **"still knows its own
mailbox truth"** at every rung; what rungs 2 and 3 hide **from the relay is sender rhythm**,
and what they hide from the network observer is timing and size.

⚠ **CLAIM BOUNDARY ON T3.** Rung 2 does not exist. Every T3 cell is a **read** of the banked
ladder design plus an inference from the subscription requirement. **Nothing in T3 is a run.**

---

### THE DELTA — T2 MINUS T1, UNDER THE PRIMARY DEPLOYMENT

⚠ **THE DELTA IS REASONED FROM MEASURED PREMISES; IT IS NOT ITSELF MEASURED.** No relay was
ever run with per-contact mailboxes. Every *premise* below is a measured fact at the stated
base; the *inference* from premise to cell is reasoning, and is labelled as such.

| cell | delta | net |
|---|---|---|
| A1/A2/A3 × C2, **send side** | address-linkage of one account's inbound traffic closes | **+** |
| A1/A2/A3 × C2, **send-side identity join** | ⛳ **a (+) no earlier draft credited:** at T1 a member's pushes go to recipients' **account-stable** addresses, so the relay can join them to those recipients' own sessions and recover the graph *with identities*. Per-contact addresses break that join | **+** |
| A1/A2/A3 × C2, **pull side** | unchanged. ⚠ The at-rest `last_touched` signature an earlier draft claimed is **STRUCK** — the write does not happen | **0** |
| A1/A2/A3 × C2, **contact count** | ⚠ **CORRECTED from (−) to 0.** T1 **already** leaks the count by the same session/IP join, on the send side: N distinct destinations in one session *is* the contact count, today, with no design change | **0** |
| A1/A2/A3 × C5 | **unchanged** — the invite bundle carries the account's stable identity keys | **0** |
| A1/A2/A3 × C2, **handshake correlator** | **unchanged** — the cleartext 16-byte `session_id` pairs both mailboxes of a handshake by byte equality, whatever the addressing | **0** |
| A1/A2/A3 × C2/C3, **delivery receipts** | **unchanged** — every receive produces a send, size-distinguishable | **0** |
| A5 × C2 | **unchanged** — the token is inside TLS | **0** |
| A6 × C6 | global route cap consumed N× faster | **−** |
| rung 2 (T3) × C2 | contact set becomes an explicit subscription object (**read + inference**, never a run) | **−** |

**Verdict on the withdrawn claim.** *"Per-contact route tokens would close it"* was
withdrawn by the Director before this lane began. The measured form is: **the mechanism is
not the remedy.** It closes exactly two channels — send-side address linkage, and the
send-side join to a recipient's account identity — while **four it does not touch stay
open**: the pull-side session/IP join, the per-account identity bundle, the cleartext
`session_id` correlator, and the delivery-receipt pair. Against those it consumes a global
route budget in proportion to contact count and, at rungs 2/3, converts an inference into a
declaration.

⚠ **THE HONEST DELTA IS SMALLER IN BOTH DIRECTIONS THAN EARLIER DRAFTS CLAIMED** — one cost
was struck (`last_touched`), one was corrected to zero (contact count), and one benefit was
added that no draft had credited (the identity join). **That is a better argument for the
ruling than the one it replaces, because it no longer rests on a cost measurement removed.**

⚠ **On the corroboration from NA-0768's menu, stated at its own scope:** finding `X1` says
*"at rungs 2/3 a beat that scales with contact count LEAKS THE CONTACT COUNT"*. **X1 scopes
that leak to rungs 2 and 3**; T2 here is rung 1. X1 supports the **T3** row and the
statement that no client-side fetch architecture avoids N-scaling; it does **not** establish
a rung-1 contact-count cost, and is not used for one.

---

### THE OTHER TWO DEPLOYMENTS — WHICH ROWS CHANGE

**(i) EACH USER RUNS THEIR OWN RELAY.** A1/A2/A3 change meaning, not content — the operator
*is* the user. C2 gets **worse for the correspondent**: every peer who pushes to Alice
reaches Alice's own machine, so Alice learns each correspondent's IP directly. A6 largely
disappears, and with it the global cap as a shared resource. **Per-contact tokens buy the
least here** — the order's sec 2 says so, and it is the deployment where operator and
account are the same party.

**(ii) A SHARED OR PUBLIC RELAY.** A1 becomes a stranger and the semi-trusted assumption is
gone: every A1/A2/A3 row keeps its content but rises in severity. C2's send side is where
per-contact tokens would finally pay. C6 worsens — the global caps are shared with
strangers. ⚠ **But the four untouched channels stay untouched here too**, so even here the
mechanism is a partial measure, not a closure.

⚠ **NO ROW IN THE TABLES ABOVE IS CLAIMED FOR THESE TWO DEPLOYMENTS**, and consequently
**this document states no ruling for them.** The disposition for a shared relay is a
*condition*, not a destination — see the ruling record.

### THE TABLES CARRY THEIR OWN CLAIMS, AND NOTHING MORE

No claim is made in this document that its tables do not carry. Every cell is cited to a
line, a log field, or a run; cells resting on a read say so; the two conclusions this lane
withdrew after measurement are recorded above rather than removed; the cipher-suite sentence
in PART ONE is **attributed**, not asserted; and the adversary set is the kickoff's seven
**plus A8, added** — the kickoff permits adding.

---

## PART THREE — WHAT THIS DOCUMENT DOES NOT COVER

- **Endpoint compromise.** A compromised member device yields that member's vault. Nothing
  at the relay layer addresses it.
- **The protocol's own security properties.** Authentication, transcript binding, replay
  resistance, forward secrecy and post-compromise security are
  `docs/audit/THREAT_MODEL_PROTOCOL_METADATA.md`'s subject and were **not measured here**.
- **The relay's hosting boundary.** The relay *application* logs no client IP — no
  `ConnectInfo`, `peer_addr`, `remote_addr` or `X-Forwarded-For` anywhere at 37ec8207.
  **Whatever terminates TLS in front of it, and the hosting provider's network, are outside
  this measurement and may log addresses.**
- **The deployed revision, exactly.** Bracketed, not identified — see the header. n=0
  contacts with any deployed relay.
- **How long file-level residue survives** before page reuse. n=0 field measurements.
- **Rung 2 and rung 3.** They do not exist; T3 is read + inference.
- **The desktop GUI.** Not driven; the beat figures are read from `ui/main.js`.
