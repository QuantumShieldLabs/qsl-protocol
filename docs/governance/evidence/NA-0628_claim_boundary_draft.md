# NA-0628 — Claim-boundary draft (D565 Operator Decision 5). **The executor moves no claim.**

Goals: G1, G2, G4. Directive: QSL-DIR-2026-07-10-565 (D565) as amended by D565-A1. Decision: D-1251.

**Default is NO CHANGE.** This file DRAFTS the sentences that closing ENG-0034 could eventually
support, and enumerates precisely what still blocks each. **The operator decides at closeout.** Nothing
in this lane's diff moves a claim, and no artifact on `main` asserts any of the sentences below.

---

## 1. What actually changed, stated exactly

Before this lane, an authenticated peer could force every DH boundary to contribute **no fresh
classical entropy**: a small-subgroup `DH_pub` yields an all-zero X25519 output, so the new root became
a deterministic function of the pre-boundary root. An attacker who had once learned `RK` stayed
synchronized across every boundary it forced non-contributory. Nothing rejected, logged, or
reason-coded. The PQ half still healed (NA-0627's Q3/Q4 are independent of the DH share), so the hybrid
**degraded to PQ-only healing rather than collapsing**.

After this lane, all six LIVE DH outputs are checked for the all-zero value and fail closed with no
state mutation, and the reject is observable (`REJECT_S2_DH_NONCONTRIBUTORY`; `dh_noncontributory` at
qsc establishment).

**Therefore: ENG-0034 removes the CODE obstacle to post-compromise / Triple-Ratchet language. It
removes no other obstacle.**

---

## 2. Candidate sentences (DRAFTED, NOT ADOPTED)

Each is written to be true *given* its stated preconditions. **None may be published until every
blocker in §3 that it depends on is discharged.**

**C1 — narrowest; depends only on code + the existing vectors.**
> Every X25519 Diffie-Hellman output on the Suite-2 ratchet and on the client's establishment path is
> checked for the all-zero (non-contributory) value required by RFC 7748 §6.1, and a message that
> would advance the root without contributing fresh classical entropy is rejected fail-closed, with no
> state mutation and a distinct reason code.

*Status:* **supportable today** by the diff, the co-located tests, and the two negative conformance
vectors executed by a required check. It is a statement about **code behaviour**, not about a security
property of the system. Blockers: none. Risk if published: none identified, provided it is not
paraphrased into a security claim.

**C2 — the contributory property.**
> The classical half of the Suite-2 ratchet is contributory: an authenticated peer cannot cause a DH
> boundary to advance the root without contributing fresh key material.

*Status:* **supportable in substance, but weaker than it reads.** Blockers: (i) it is a statement about
the two implementations in this repo, not about the protocol as specified — DOC-CAN-003 now says MUST,
so a third implementation that ignores it is non-conformant but not detectably so at the wire; (ii)
`recv_dh_boundary` and `send_boundary` are guarded and unit-tested but **not conformance-vector
covered** (no actor op reaches them — see D-1251); (iii) no independent human review.

**C3 — post-compromise security (the sentence this project has deferred since NA-0619).**
> Suite-2 provides post-compromise security: after a full state compromise, security is restored once
> either an honest DH boundary or an honest PQ reseed completes.

*Status:* **NOT supportable today.** This is the sentence ENG-0034 was blocking, and it is now blocked
only by §3's remaining items — which are substantial. See §3.1, §3.2, §3.3.

**C4 — the hybrid / Triple-Ratchet framing.**
> Suite-2 composes a classical DH ratchet with a post-quantum KEM reseed such that healing survives the
> compromise of either primitive alone.

*Status:* **NOT supportable today** as a security claim. NA-0627 proved Q3+Q4+Q5 **together** are the
hybrid claim *inside the symbolic model*. Outside it, §3.1 applies in full. If ever published, it must
name Q3+Q4+Q5 jointly and must never quote one direction alone.

---

## 3. What still blocks C2/C3/C4 — enumerated, not hand-waved

### 3.1 The result is symbolic, over abstracted primitives (A1–A8)
NA-0627's proofs hold in ProVerif over an idealized algebra (`DOC-G4-002` §2). Each abstraction names
the property it masks. The ones that bite here:

- **A4 — X25519 as an idealized symbolic group.** Masks small-subgroup / low-order points and
  non-contributory shared secrets. **This is exactly the property ENG-0034 concerns, and the model
  cannot see it.** The guard's correctness rests on RFC 7748 and on the tests in this lane, *not* on
  the ProVerif result. Q7 mirrors the shipped **ingress encoding** check, not the new **output** check,
  and Q7 was never an attack-existence proof.
- **A2/A3/A5** — KMAC/SHA-512 as perfect one-way functions, ideal AEAD, ideal ML-KEM. A symbolic green
  says nothing about computational hardness, nonce misuse, decapsulation-failure oracles, or side
  channels.
- **A1** — establishment authentication is *assumed*, not proved, in the composition model.

**Consequence:** a symbolic proof is **necessary input to**, never **sufficient grounds for**, C3/C4.

### 3.2 ENG-0035 — the 2-epoch unrolling gap stands
ProVerif does not terminate at the design-locked 2-boundary unrolling (>102,000 rules, no `RESULT`, at
a 2400 s cap). The main model was reduced to one DH boundary + one PQ reseed, with the reduction stated
in the model header and **no query weakened**. **No single model exercises two consecutive
root-advancing DH epochs.** C3 is a statement about repeated healing; the model that would most
directly support it is precisely the one that does not terminate.

### 3.3 Independent HUMAN review remains an open prerequisite
Standing since NA-0619 and reaffirmed at D-1249/D-1250. No external cryptographer has reviewed the
composition, the abstraction table, or this lane's guard. **This blocker is not discharged by any
amount of internal proof.**

### 3.4 Two lane-local caveats, recorded so they are not forgotten
- **Coverage asymmetry.** `recv_dh_boundary` (`:1475`) and `send_boundary` (`:1306`) are unreachable
  from any conformance-actor op, so their guards are proved by co-located Rust tests, not by vectors. A
  third-party implementation exercising only the vector corpus would not be tested on them.
- **The anti-regression scan is not a PR gate.** No CI job runs `cargo test -p quantumshield_refimpl`.
  A future unguarded `.dh(` call site would not be caught by CI today. Filed for the successor.

---

## 4. Recommendation to the operator

1. **Adopt nothing beyond C1, and only if a claim must be made at all.** C1 is a code-behaviour
   statement, fully evidenced, and carries no security implication. The safest action remains **no
   change**, which is D565 Operator Decision 5's default.
2. **Do not adopt C2/C3/C4.** Their remaining blockers (§3.1–§3.3) are real, and two of them
   (independent review; the 2-epoch unrolling) are outside this lane's reach entirely.
3. **The honest one-line summary of this lane** — suitable for the journal, not for public claims:
   *"The last known code obstacle to post-compromise language is closed; the analytical and review
   obstacles are not."*
4. If the project ever wants C3, the shortest credible path is: discharge ENG-0035 (Tamarin for the
   multi-epoch unrolling), then commission the independent review, then revisit. In that order — a
   review of a model with a known non-terminating query is a review of the wrong artifact.

**No claim is moved by this lane. Still NO public-readiness, production-readiness, security-completion,
crypto-complete, attachment-complete, bug-free, vulnerability-free, post-quantum, Triple-Ratchet, or
post-compromise claim.**
