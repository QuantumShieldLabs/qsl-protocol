Goals: G4, G5

Status: Supporting (product planning; subordinate to canonical specs, GOALS, the queue, and independent review)
Owner: QSL governance
Last-Updated: 2026-07-18 (NA-0656/D592: DOC-PROG-004 v0.2.0 back-pointer refresh, §6)
Authority: Non-normative product planning. Does NOT override START_HERE, GOALS,
the canonical specs, NEXT_ACTIONS, the IMPROVEMENT_LEDGER, or any recorded
decision. Where this document and any of those disagree, THEY win and this
document is corrected.

# DOC-PROG-003 — QSC Feature Plan: Tiered Target Feature Set and Build Order v0.1.0 DRAFT

> Provenance: Records the operator feature decision of 2026-07-13, landed per
> QSL-DIR-2026-07-13-577 (D577, as amended A1) by lane NA-0641 (D-1264). This
> document is "the qsc feature plan" that DOC-PROG-002 §4 and §8 previously
> cited as a dangling reference — the feature set now lives in the repo rather
> than only in chat context. Current-state facts are from the 2026-07-13
> read-only built-vs-planned investigation (verified against main `7d7c7550`;
> the qsc capability inventory below cites that investigation's findings).
>
> **Authorization boundary: this document authorizes NO implementation.** It is
> the PLAN, not the build. Each feature's actual build is its OWN future lane
> with its own operator-approved directive. No lane may cite this document as
> execution authority; NEXT_ACTIONS.md remains the only execution queue.
>
> Position in the operator's pre-GUI sequence: (1) integration proven [DONE,
> NA-0640]; (2) DECIDE the feature set [THIS document records that decision];
> (3) close core gaps for the chosen features; (4) then GUI.

---

## 1. Current state (honest inventory, verified 2026-07-13)

What qsc IS today: a working 1:1 PQ-hybrid E2EE messenger over a relay —

- **Messaging:** send/receive with prepare→send→commit semantics, explicit
  padding controls, delivered-receipt round-trip to `peer_confirmed`.
- **Attachments/files:** bounded chunked transfer with manifest integrity; a
  ≤4 MiB in-message lane plus the streaming attachment-service lane (>4 MiB)
  with signed confirm — proven end-to-end against the REAL qsl-server and REAL
  qsl-attachments at NA-0640 (byte-verified, both relay auth modes).
- **Handshake/identity:** the `QSC.HS.*` PQ-hybrid handshake with pinned-
  identity, out-of-band verification-code trust (responder authenticated
  against the pinned KEM identity since NA-0633); per-device contact records,
  trust modes, inbound-request handling.
- **Storage:** encrypted-at-rest vault (Argon2id + AEAD); encrypted timeline;
  redacted diagnostics export. (The failed-unlock wipe limit was TUI-only and
  was co-deleted with the TUI at NA-0645 — owed back in the GUI phase via
  ENG-0044, together with idle autolock and account-destroy.)
- **UI:** the qsc TUI (the read-mostly "Security Lens") was **RETIRED at
  NA-0645 (D581, 2026-07-14)** — operator product decision: the GUI is the
  only end-user UI and the CLI stays a thin test-harness/operator surface;
  the TUI's simplified receive loop lacked the NA-0644 ack-lease/dedup
  durability. Its conversation view, panes, and command surface are prior
  art for the GUI.

What qsc is NOT (none of this is built):

- **NONE of the seven Signal-parity features exist:** disappearing messages,
  reactions, quoting/replies, voice notes, search, groups, calls. (Every
  keyword hit in the 2026-07-13 investigation was a false positive — e.g. the
  only "expiry" code is attachment-retention metadata; the only "search" hit
  is the word "research" in a disclaimer string.)
- **NONE of the four differentiator UIs exist, except the no-account posture,
  which is structural:** identity is a local keypair under a label — no
  registration, no phone number, no account concept anywhere. There is no
  PQ-status indicator in any user-facing surface (the protocol IS hybrid-PQ;
  nothing user-facing says so — the indicator lands in the GUI, the TUI being
  retired). There is no KT verifier UI.
- **KT exists only in the refimpl** (`tools/refimpl/.../kt/`, interface defined,
  verification plumbing stubbed pending DOC-CAN-008 wire formats). The qsc
  client has zero KT code.
- **Self-host is partial:** a demo-class relay (`relay serve`: in-memory,
  TTL-discard, fault injection for tests) and client-side bearer-token auth
  exist and are e2e-proven; production durability, TLS, backup/restore, and
  per-client admission tokens (ENG-0036) do NOT exist.

This inventory is the baseline every build lane below starts from. It is
recorded honestly so the plan cannot silently overstate what exists.

## 2. Strategic sharpening — the edge is narrower and sharper than the §4 list

DOC-PROG-002 §4 sketched "client feature parity where it serves the niche"
plus differentiators. Since then the competitive picture has sharpened:
**Signal now publicly builds and markets both post-quantum ratcheting (SPQR)
and key transparency.** Consequence: "we have PQ" and "we have KT" are
becoming **table stakes**, not differentiators.

The DURABLE edge is what Signal structurally cannot or will not do:

- **(a) Self-host.** Signal's model is centralized service + client trust;
  QSL's relay is a single self-hostable binary (DOC-PROG-002 §2).
- **(b) No phone number / no account.** Already structural in qsc (§1 above);
  Signal's phone-number identity is its own acknowledged limitation.
- **(c) PQ-native AUTHENTICATION.** Signal's SPQR addresses confidentiality;
  its handshake still authenticates classically. QSL's hybrid-signature,
  PQ-authenticated line is the Pillar I flagship (DOC-PROG-002 §3), carried
  through the NA-0634/NA-0636 handshake-authentication work.

**The discipline, stated explicitly: do NOT chase Signal's parity tail.**
Effort spent matching features Signal already has (and ships at consumer
polish) is effort not spent on what only QSL can offer. Parity items enter
the plan only where the high-trust niche needs them (Tier 1), and the
expensive tail is deferred (Tier 3).

DOC-PROG-002 §6's **overclaim ban carries over in full**: this plan and every
lane it spawns prove and state QSL's own properties; no artifact makes a
comparative security claim. Statements here about what Signal builds or has
publicly deferred are factual gap observations in the DOC-PROG-002 §3 style,
not superiority claims — whether any of this makes QSL "better" is a judgment
for others, on gated evidence.

## 3. The tiered target feature set

### 3.1 Tier 1 — BUILD: the niche-serving core the product needs

1. **Self-host, OPERATOR-FIRST (top build priority).** The niche thesis is
   self-hosted, phone-number-free, high-trust orgs; today the relay is
   demo-class. See §4 for the split and the operator-first scoping.
2. **Identity-verification UI.** A usable surface over the EXISTING
   verification-code / pinned-identity mechanism — the "safety number"
   equivalent. Security-critical: the mechanism already works; the surface
   determines whether humans actually use it.
3. **Disappearing messages.** Ephemerality matters intensely for the
   high-trust niche (legal, medical, activist) — this is more than parity.
4. **Basic message search.** Table-stakes usability for a real messenger;
   scoped against the encrypted-at-rest timeline.

### 3.2 Tier 2 — BUILD: the differentiator UIs (only QSL can, and they ARE the pitch)

1. **PQ-status indicator.** The protocol is already hybrid-PQ; the UI never
   says so. A cheap UI line that surfaces the flagship property.
2. **KT verifier UI.** Bigger: KT is refimpl-only today (stub verification
   plumbing; DOC-CAN-008 profile), so this needs core work before UI. Also a
   race — Signal is building its KT; QSL's differentiator is USER-VERIFIABLE
   transparency surfaced in the client, PQ-authenticated via Pillar I.
3. **Self-host guided / admission-token UX** (ENG-0036): the public/private
   setup toggle, per-client revocable admission tokens, print/QR invite.
   Layers on the Tier-1 operator-first base (§4); admission control, never
   message security.

### 3.3 Tier 3 — DEFER (Signal has these; expensive; the niche does not need them first)

Reactions, quoting/replies, voice notes, groups, voice/video calls, stickers,
polls, stories. Deferral is a scoping decision, not a judgment that these are
worthless — several (groups especially) will matter eventually. They are
deferred because each is expensive, Signal already serves them well, and none
is what the niche hires QSL for. Revisiting a Tier-3 item requires updating
this plan first (§7).

## 4. The self-host split (two very different lanes — do not conflate them)

- **CLIENT admission UX — qsc territory (ENG-0036).** The public/private
  setup toggle, per-client revocable admission tokens, invite print/QR.
  This is ADMISSION CONTROL, never message security (the ledger entry's
  threat-model note stands: it answers "outsiders connecting to my private
  relay," not "the relay operator is the adversary").
- **PRODUCTION RELAY — qsl-server territory (CROSS-REPO).** Move off
  demo-class (in-memory, TTL-discard) to: durability, per-client
  authorization, backup/restore, TLS. Any lane here is a cross-repo lane with
  its own directive; qsl-protocol lanes do not edit qsl-server. (ENG-0037
  sealed-sender is adjacent relay-posture work — complementary to ENG-0036,
  higher-value in PUBLIC mode, tracked separately on the ledger.)
- **NEAR-TERM TARGET: technical-OPERATOR-first.** The first shipped self-host
  story serves an admin comfortable with CLI + config, with a real
  setup/deploy guide — NOT a polished non-technical onboarding flow. The
  guided/QR/non-technical UX (Tier 2, item 3) layers on once the operator
  path is solid. This ordering is deliberate: the niche's early adopters are
  exactly the users who can run a binary from a guide, and a guided UX built
  before the underlying deploy story is solid would gild a demo.

## 5. Build order

1. **Self-host operator-path** (Tier 1, item 1 — top priority; the §4 split
   scopes what is qsc-side vs cross-repo).
2. **Identity-verification UI + PQ-status indicator** (Tier 1 item 2 + Tier 2
   item 1) — cheap, high-value surfaces of mechanisms that already exist.
3. **Disappearing messages, then basic search** (Tier 1, items 3–4).
4. **KT verifier UI and the guided/admission-token self-host UX** (Tier 2,
   items 2–3) as their underlying pieces mature (KT wire formats; the
   operator-path base).
5. **Tier 3: deferred** — no build without a plan update (§7).

**Each step is its own future lane (or lane family) with its own
operator-approved directive.** This ordering constrains sequencing; it does
not authorize any lane to begin. Where a step has protocol or claim
implications (KT especially), the DOC-PROG-002 §6 gates apply unchanged:
vectors + bounded models where modelable + independent review before any
external claim.

## 6. Relationship to other documents (spine wiring)

- **DOC-PROG-002** (Product Strategy: Niche Positioning) — the strategy this
  plan serves. Its §4 third bullet ("client feature parity where it serves
  the niche") and §8 both cite "the qsc feature plan": that is THIS document.
  §2 above records why the target set here is narrower and sharper than §4's
  illustrative parity list.
- **DOC-PROG-001** (Goal-to-Release Roadmap) — the sequencing spine; this
  plan's build order feeds it at the product layer.
- **DOC-PROG-004** (QSC GUI Phase Roadmap) — the GUI-phase path: how this
  plan's GUI-relevant §5 steps are sequenced and gated (the locked decisions,
  decision gates D-A/D-B, and parallel tracks). Landed per D586 (NA-0650); revised to v0.2.0 per D592 (NA-0656).
- **The IMPROVEMENT_LEDGER** — the self-host build inputs are already filed
  there: **ENG-0036** (token-gated relay access / public-private toggle —
  the §3.2/§4 client-admission input) and **ENG-0037** (sealed-sender —
  adjacent relay-posture work, not part of the admission UX). Their recorded
  options remain "options to weigh, not a committed design" until a build
  lane's design-lock.
- **DOC-CAN-008** — the KT profile the Tier-2 KT verifier UI ultimately
  builds against (via the refimpl's KT interface, once wire formats settle).
- **DOC-QSC-001..006 / QSC TUI spec** — the TUI-specific docs (DOC-QSC-001,
  DOC-QSC-005, QSC_TUI_SPEC/INVARIANTS/IMPLEMENTATION_CHECKLIST) are
  SUPERSEDED as of NA-0645 (the TUI is retired; they remain as history).
  Their security-lens PRINCIPLES — explicit intent, deterministic markers,
  truthful security state, no implicit mutation — carry forward as the bar
  the GUI surfaces in Tiers 1–2 must respect.
- **GOALS.md** — Tier 1/2 items serve G4 (verification-adjacent surfaces) and
  the product posture; metadata-relevant items (ENG-0037) belong to the G5
  family per the metadata roadmap.

## 7. Maintenance rule (prevents drift)

This plan is updated as part of the closeout of any lane that builds, splits,
re-tiers, or defers a feature listed here — the tier tables and build order
must always reflect the operator's latest recorded decision. Promoting a
Tier-3 item, adding a feature, or reordering §5 requires an operator decision
recorded in DECISIONS.md and a corresponding update here, BEFORE the build
lane is seated. If this document has not been touched across several
product-shaped lanes, treat that as drift and re-baseline it against the live
queue and ledger (the DOC-PROG-002 §9 discipline).

---

*End DOC-PROG-003 v0.1.0 DRAFT. Subordinate at all times to the canonical
specs, GOALS, the queue, the ledger, and independent review. This document
records a plan; it authorizes no implementation and makes no comparative
security claim.*
