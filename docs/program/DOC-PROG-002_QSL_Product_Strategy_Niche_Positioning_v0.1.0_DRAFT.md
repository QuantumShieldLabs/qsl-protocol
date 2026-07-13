Goals: G4, G5

Status: Supporting (product strategy; subordinate to canonical specs, GOALS, the queue, and independent review)
Owner: QSL governance
Last-Updated: 2026-07-13
Authority: Non-normative product strategy. Does NOT override START_HERE, GOALS,
the canonical specs, NEXT_ACTIONS, the IMPROVEMENT_LEDGER, or any recorded
decision. Where this document and any of those disagree, THEY win and this
document is corrected.

# DOC-PROG-002 — QSL Product Strategy: Niche Positioning and Competitive Program v0.1.0 DRAFT

> Provenance: Distilled from the operator/Director strategy review of
> 2026-07 (Signal competitive analysis, niche-thesis decision, four-pillar
> program). This document exists so the strategy is repo-backed and readable
> by every session, rather than living only in chat context. It is expected
> to be maintained by the lanes it governs (each pillar's closeout updates it)
> so it cannot drift from the queue.
>
> Live-state anchor (2026-07-11, verified against main 5d990055): several
> pillar-relevant items are already filed and advancing — the X25519
> contributory-DH gap is CLOSED (ENG-0034, Pillar II hygiene); the handshake
> responder-auth bypass is REMEDIATED (ENG-0038 / NA-0633, construction C1);
> the private-relay public/private-toggle + admission-token work is filed
> (ENG-0036, niche workstream §4); sealed-sender is filed as a metadata
> item (ENG-0037, Pillar IV); an external-review bundle has been assembled
> (NA-0631, cross-cutting workstream §5). The handshake remains UNMODELED
> (no QSC.HS.* formal model yet) and the Signal-shaped prekey end-state is
> deferred pending the D571 decision. This anchor should be refreshed
> whenever this document is re-baselined per §9.

---

## 1. The thesis (one paragraph)

QSL does not attempt to beat Signal at Signal's center — mass consumer
messaging — which is unwinnable on trust, scale, and network effects. QSL
targets what Signal **structurally cannot serve**: **self-hosted,
phone-number-free, small high-trust organizations** (legal, medical, security,
activist, and similar groups that must run their own infrastructure, cannot
use phone-number identity, and value provable properties over reach).
Simultaneously, a set of technical pillars closes specific gaps so that
"stronger than Signal on certain axes" becomes true property-by-property. This
path is **robust to slow AI progress** (winnable with today's tooling and
discipline) and **leveraged to fast AI progress** (better executors accelerate
every lane). The durable advantages are the **structural** ones Signal cannot
adopt without breaking its own model.

## 2. Why the niche fits QSL by construction (verified against the codebase)

These are not aspirations; they are current properties of the code as of
mid-2026:

- **Self-hostable relay.** qsl-server is a single transport-only Rust binary,
  loopback-default bind, one optional bearer token, systemd packaging, no
  database, no registration backend, no hardcoded central endpoint.
- **Phone-number-free identity.** No E.164 / SMS / email dependency anywhere.
  Identity is keys + fingerprints + route-tokens, exchanged out of band. The
  niche's hardest requirement is met by construction — and this is Signal's
  own acknowledged limitation.
- **High-trust trust model.** AGPL-3.0; self-custodied qsc vault (Argon2id +
  AEAD); out-of-band contact exchange. The whole model already assumes small,
  self-provisioning, high-trust groups.

Consequence: the niche is not a pivot away from what QSL is. It is a naming and
sharpening of what QSL already is. The strategy is to make this posture a
**stated, deliberate, verified** product position rather than an accident of
architecture.

## 3. The competitive pillars (technical differentiators)

Each pillar maps to the existing program goals (GOALS.md G1–G5) and closes a
specific gap. None ships a comparative claim; each proves a **property**.

- **Pillar I — Post-Quantum-Native Authentication (flagship).** Signal (PQXDH)
  protects confidentiality against quantum adversaries but authenticates
  **classically** — post-quantum authentication is the gap the industry, and
  Signal, have publicly deferred. QSL already specifies hybrid Ed25519+ML-DSA
  bundle signatures. Closing this gap end-to-end (identity, prekey/bundle,
  device auth) such that forgery requires breaking BOTH classical and PQ is the
  clearest opening. **Hard part:** PQ signatures reduce deniability; the
  deniability stance is a recorded design decision, not a default. Relates to
  G4. See the standing tension in §6.

- **Pillar II — Strict No-Downgrade, Fail-Closed Posture.** QSL has no legacy
  sessions and no classical-fallback path to defend, so "no downgrade surface,
  by construction" can be a **stated, verified, reviewable invariant** — one
  Signal structurally cannot claim because it must support fallback for
  un-upgraded clients. Maps directly to G3.

- **Pillar III — Verifiable Key Transparency, Shipped Early.** Auditable,
  user-verifiable KT (append-only log, signed tree heads, third-party
  auditability, split-view resistance), PQ-authenticated via Pillar I's
  signatures. A maturity Signal reached late; QSL can build it in from early.
  DOC-CAN-008 already carries a KT profile. Relates to G4.

- **Pillar IV — Metadata Minimization by Design (= the G5 lane family).**
  Match Signal's sealed-sender / padding / private-group properties and push
  further where greenfield allows, against a **named adversary per lane**.
  This is the existing "Hostile Analyst / Metadata Minimization" roadmap,
  re-anchored competitively. Maps to G5. Attachment padding already shipped
  (NA-0614 / DOC-G5-007). Non-goals recorded in the G5 docs (mixnets, global
  cover traffic) stand.

## 4. The niche workstream (near-horizon product features)

These make the self-hostable / phone-number-free / high-trust posture concrete
and visible. They are product-shaped, not pillar-shaped, and are sourced from
the operator requirements captured in the N3 lane-family plan.

- **Public/private relay posture + per-client admission tokens.** Setup asks
  public or private; private generates per-client revocable admission tokens
  and prints/QRs an invite (relay address + token). Revoking a token kills
  exactly that client. Admission control, never message security. (N3 family;
  operator-sourced.)
- **Self-host deployability.** Move the relay from demo-class (in-memory,
  TTL-discard) to a production high-trust story: durability, per-client
  authorization, backup/restore, TLS. Consciously scoped to the high-trust-org
  profile, not consumer scale.
- **Client feature parity where it serves the niche.** The qsc feature plan
  (DOC-PROG-003) identifies the Signal Desktop features worth matching (disappearing messages,
  reactions, quoting, voice notes, search, groups, calls, etc.), prioritized
  for this audience, plus niche differentiators Signal lacks (PQ status
  indicator, KT verifier UI, "no account" UX, screen security). A Tauri GUI
  (Rust + system WebView) is a structural advantage over Signal's Electron.

## 5. The cross-cutting workstream (earns the right to any claim)

No pillar means anything competitively until QSL has what Signal has:
**independent scrutiny.** This runs continuously and gates every claim.

- **Formal verification from the start** (bounded models already live and
  CI-gated; extend coverage; the handshake model is a named gate — the
  `QSC.HS.*` model).
- **Independent human cryptographic review** before any real-traffic claim.
  The IMPROVEMENT_LEDGER already functions as a known-findings register; an
  external-review readiness package makes review cheap and effective. This is
  the item that most directly closes the "AI-built" trust gap.
- **Dependency maturity** (the pre-1.0 ml-kem / ml-dsa crate risk; NIST KATs
  in CI; migration to audited/verified implementations).
- **Reproducible, signed builds** as a trust signal.

## 6. Standing tensions and hard rules (do not lose these)

These are the traps a future lane can fall into if it forgets the strategy.

- **"Match Signal" can pull against "beat Signal."** Signal's handshake is the
  thing whose authentication is NOT yet post-quantum. Faithfully copying
  PQXDH's shape is the single move most likely to surrender Pillar I's edge.
  Any handshake/authentication lane MUST treat "does matching Signal forfeit
  the PQ-native-auth edge?" as a first-class, recorded decision — never a
  silent default. (This is why D571's prekey gate exists.)
- **The overclaim ban is mechanical.** No lane artifact may use comparative or
  superiority language ("more secure than Signal," "most secure," "unbreakable,"
  unqualified "anonymous"). Lanes prove **properties**; the operator makes
  comparisons externally, on gated evidence. This is consistent with the
  existing no-overclaim rules in the canonical specs.
- **Claims are gated.** A pillar's competitive property may be stated
  externally only after: conformance vectors + a bounded formal model (where
  modelable) + independent human review, all recorded. Internal PASS is
  necessary but not sufficient for a claim.
- **Independent human review is not optional and not replaceable** by internal
  audit, however capable the executor. The audit lanes exist to make external
  review efficient, not to substitute for it.
- **Deniability vs PQ authentication** is a genuine, unsolved-in-general
  tension (PQ signatures reduce deniability). Pillar I must resolve QSL's
  stance explicitly; for a high-trust-org audience, strong authentication may
  rightly outrank deniability — but that must be RECORDED, not assumed.

## 7. Definition of done — two horizons

- **Near horizon (niche viability):** self-host durability + authorization
  story shipped; phone-number-free posture stated as a commitment; the
  no-downgrade invariant proven; an external-review package published and a
  reviewer engaged; the live metadata channels triaged against named
  adversaries. At this point QSL is a **credibly deployable** high-trust,
  self-hosted, phone-number-free messenger with an honest security story —
  serving users Signal structurally cannot.
- **Far horizon (property-by-property parity-plus):** Pillars I–IV each hold
  their gate (vectors + models + independent human review); crypto
  supply-chain on verified implementations. At this point QSL can **truthfully
  state**, property by property, that it is PQ-native in confidentiality AND
  authentication, fail-closed with no downgrade surface, verifiable by design,
  and metadata-minimizing against named adversaries — several of which Signal
  has publicly deferred.

Whether that is "better than Signal" is a judgment for others on the evidence.
QSL states what it can prove. That restraint is itself part of the position:
for a sophisticated niche audience, a protocol whose claims survive scrutiny —
because it never made one it couldn't back — is the product.

## 8. Relationship to other documents

- **GOALS.md** — the pillars map to G1–G5; this document does not add or change
  goals, it orients them competitively.
- **DOC-PROG-001** (Goal-to-Release Roadmap) — the sequencing spine; this
  document supplies the "why" and the niche framing behind it.
- **The IMPROVEMENT_LEDGER** — the live findings register; the external-review
  package (§5) draws from it.
- **The N3 lane-family plan and the qsc feature plan (DOC-PROG-003)** — the product-shaped
  detail behind §4; those enter the repo via their own design lanes.
- **DOC-CAN-003 / DOC-CAN-008** — the canonical crypto specs the pillars build
  on; canonical, and authoritative over this document.

## 9. Maintenance rule (prevents drift)

This document is updated as part of the closeout of any lane that materially
advances or changes a pillar, the niche workstream, or the cross-cutting
workstream. A lane that completes a pillar gate updates §3 and §7. A lane that
resolves a standing tension (§6) records the resolution here. The goal is that
this document stays true **by construction** — maintained by the work it
governs — rather than becoming a stale snapshot. If this document has not been
touched across several strategy-relevant lanes, that is itself a signal it has
drifted and should be re-baselined against the live queue and ledger.

---

*End DOC-PROG-002 v0.1.0 DRAFT. Subordinate at all times to the canonical
specs, GOALS, the queue, the ledger, and independent review.*

*Landing note (verified against live repo, 2026-07-11): docs/program/ exists
with DOC-PROG-001 as its only member; DOC-PROG-002 is the next free number.
By the precedent DOC-PROG-001 sets, program docs are listed in NONE of the
four indexes (DOC-CTRL-001 Master Index, DOCS_MAP.md, INDEX.md,
MANIFEST.sha256) — DOC-PROG-001 itself is unindexed — so landing this file the
same way requires no index edits. If the operator chooses to index it (which
reduces the risk of the strategy being forgotten — the whole point of moving
it into the repo), that establishes a new convention rather than matching an
existing row: add a DOC-CTRL-001 governance-table row (columns: Priority |
Document | Purpose | Normative Scope) and a MANIFEST.sha256 line
(`<sha256>  docs/program/<file>`). Whether indexing is REQUIRED is not
settled by precedent; the landing lane should confirm against the doc-control
policy before relying on "not required." Assign the final DOC number and any
Master Index entry per DOC-CTRL-001 conventions at the lane that lands this
file.*
