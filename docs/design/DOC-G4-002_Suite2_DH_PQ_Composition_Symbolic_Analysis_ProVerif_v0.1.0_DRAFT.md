Goals: G1, G2, G4

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-09

# DOC-G4-002 — Suite-2 DH+PQ Composition: Symbolic Analysis (ProVerif) v0.1.0 DRAFT

Purpose:
- record, **for a future external human reviewer**, exactly what the ProVerif model of the
  Suite-2 hybrid key schedule proves, under which abstractions, and — the part that matters most
  — **what each result does NOT license**;
- discharge `FORMAL_VERIFICATION_PLAN.md`'s standing tooling milestone ("select ProVerif vs
  Tamarin and record in DECISIONS.md once chosen") with the evidence that motivated the choice;
- state the two findings this analysis produced (ENG-0034, ENG-0035) and the one question a
  symbolic model **cannot** answer.

Lane: NA-0627. Directive: QSL-DIR-2026-07-09-564 (D564), Decisions 1/2/4/5 approved,
Decision 3 (abstraction boundary) ratified at design-lock. Governance: D-1249. Base: main
`d9baed9d` (the post-NA-0626 single-root state).

> **Read this first.** A green ProVerif result is **necessary input to**, not **sufficient
> grounds for**, any post-quantum / Triple-Ratchet / post-compromise claim. It is a *symbolic*
> result over *abstracted* primitives against a Dolev-Yao adversary. It says nothing about
> computational hardness, nothing about side channels, and nothing about whether *this Rust
> implementation* matches the model. **Independent human review remains an open prerequisite.**
> Per Operator Decision 4, **no project claim changes on the strength of this document.**

---

## 1. What was modeled

One Suite-2 session between two **authenticated** parties (A = initiator, B = responder), with
the network under **full Dolev-Yao control** (read / inject / drop / reorder / replay), plus
`phase`-scoped compromise of selected secrets for the healing queries.

Ground truth is the shipped refimpl at `d9baed9d`:
`tools/refimpl/quantumshield_refimpl/src/suite2/{establish,ratchet,scka}.rs`, whose session state
is the post-NA-0626 **single root** `Suite2SessionState { rk, send, recv, dh }`.

**Fidelity is a claim, and it is proved against the code, not assumed.** Each modeled step names
the shipped function and the normative section it implements:

| # | Modeled step | Shipped function | Normative |
|---|---|---|---|
| M1 | establishment: `RK0 = KDF(dh_init,"QSP5.0/RK0",sid‖0x01)`, `RK = KDF(RK0,"QSP5.0/RKPQ",pq_init_ss‖0x01)`, then `HK/CK0/PQ0` per direction | `establish.rs:23 init_from_base_handshake` | DOC-CAN-003 §8.2 |
| M2 | DH boundary SEND: fresh keypair, `(RK',CK_ec0)=KDF_RK_DH(RK,dh_out)`, `CK_pq0=PQ0(RK')`, header under **pre-boundary NHK_s**, n=0 | `ratchet.rs:1283 send_boundary` | §8.5.2 + §8.5.1 |
| M3 | DH boundary RECV: NHK-only open at n=0, same derivation, commit `rk`/`dhr`/recv chains, `nr:=1` | `ratchet.rs:1383 recv_dh_boundary` | §8.5.2 + §8.5.1 |
| M4 | PQ reseed SEND: seeds from `RK_old`, then `RK'=KDF_RK_PQ(RK_old,ss)`; header under pre-reseed NHK_s | `ratchet.rs:1700 send_pq_reseed` | §8.5.3 + §3.3.6 |
| M5 | PQ reseed RECV (session-level; full schedule incl. the send half) | `ratchet.rs:2210 recv_pq_reseed` | §8.5.3 (+ ENG-0030) |
| M6 | **COMBINED DH+PQ boundary SEND**: DH first (`RK_dh`), seeds + transient `PQ0` from `RK_dh`, then `RK_final=KDF_RK_PQ(RK_dh,ss)`; body mk from the fresh pre-seed chains; n=0 of the new epoch | `ratchet.rs:1848 send_combined_boundary` | §8.5.2+§8.5.3, DOC-G5-008 §4 |
| M7 | **COMBINED boundary RECV**: fresh-`DH_pub` discrimination, NHK-only at n=0, identical composition | `ratchet.rs:2293 recv_combined_boundary` | §8.5.2+§8.5.3 |
| M8 | SCKA reseed validation: monotonicity / one-time / tombstone; directional seeds | `scka.rs:60 apply_pq_reseed`, `scka.rs:37 kdf_pq_reseed_seeds` | DOC-CAN-004 §3.4, §3.3.6 |
| M9 | ADV SEND: header under `HK_s`; `adv_mac=KMAC32(RK,"QSP5.0/ADVAUTH",id‖pub‖0x01)` as the first 32 B of the sealed body | `ratchet.rs:1588 send_pq_advertise` | §8.5.4 + ENG-0023 |
| M10 | ADV RECV (authenticated, chain-consuming, in-order) | `ratchet.rs:2017 recv_pq_adv` / `:2506 recv_pq_adv_session` | §8.5.4, DOC-CAN-004 §3.2 |
| M11 | message keys: `mk = KDF_HYBRID(ec_mk, pq_mk)`; chains step per message | `ratchet.rs:295 derive_mk_step` | §3.3 |

The complete **20-label derivation alphabet** extracted from the code is encoded as a distinct
symbolic label each, so label separation is explicit and inspectable:
`RK0, RKPQ, RKDH, HK/{A->B,B->A}, NHK/{A->B,B->A}, CK0/A->B, PQ0/{A->B,B->A}, CK, MK, PQCK,
PQMK, HYBRID, ADVAUTH, HDR-NONCE, BODY-NONCE, PQSEED/{A->B,B->A}, SCKA/CTXT`.

Files: `formal/proverif/suite2_dhpq_lib.pvl` (the shared theory + fidelity map — the single place
the encoding lives), `suite2_dhpq_main.pv`, `suite2_dhpq_q3_pq_reseed_healing.pv`,
`suite2_dhpq_q4_combined_healing.pv`, `suite2_dhpq_q5_dh_healing.pv`, and the fail-closed gate
`run_proverif_checks.py`.

## 2. Abstraction table (Operator Decision 3 — each row names the property it MASKS)

**A model whose abstraction hides the property it claims to prove is worse than no model.**

| # | Abstraction | Property it MASKS | Disposition |
|---|---|---|---|
| A1 | Establishment authentication assumed (`dh_init`/`pq_init_ss` pre-shared private names; `authenticated == true`) | identity misbinding / downgrade **at the handshake** | Covered by `formal/model_qsc_kem_signature_transcript_binding_bounded.py`; recorded, not re-proved |
| A2 | KMAC/SHA-512 as perfect one-way functions (free constructors); the 64-byte `KDF_RK_DH` output modeled as the independent pair `rkdh_rk`/`rkdh_ck` | collisions, length-extension, weak label separation | Standard symbolic assumption; the 20-label alphabet is verified by inspection against the code |
| A3 | AEAD ideal (`senc`/`sdec`, nonce+AD as data) | nonce reuse, AEAD misuse, ciphertext malleability | Nonce reuse is guarded in code by the ENG-0013 counter hard-stop — masked here, guarded there |
| A4 | **X25519 as an idealized symbolic group** (`exp` + commutativity only) | **small-subgroup / low-order points, non-contributory shared secrets — hence the presence or ABSENCE of a contributory check** | **Cannot be lifted (§5). Answered OUTSIDE the model → ENG-0034** |
| A5 | ML-KEM as an ideal KEM (`kemenc`/`kemdec`, no decapsulation failure) | decap-failure oracles, malformed-ct behaviour, implicit rejection | Recorded. The SCKA one-time/tombstone rules (M8) hold structurally in the bounded schedule; the exhaustive set-logic is in `formal/model_scka_bounded.py` |
| A6 | **Bounded epoch unrolling — REDUCED from the design-locked bound (§6)** | attacks needing more epochs than unrolled; specifically, two consecutive root-advancing DH epochs | **ENG-0035.** Bound stated per model; the reduction is in the model header, never in a weakened query |
| A7 | `mkskipped` / out-of-order receive out of scope (every modeled receive is the in-order arm, `n == nr`) | skip-window key-retention attacks | Out of scope; covered by the `ooo_replay` conformance vectors |
| A8 | qsc cadence/policy out of scope | metadata / timing distinguishers | Already tracked as ENG-0022; not a secrecy property |

**Fidelity note F1** (why each scripted receive point carries only its scheduled arm): the shipped
AD binds `(session_id, pv, suite, DH_pub, flags, pq_bind)` into **both** the header and body AEAD,
and `pq_bind = SHA-512-32(flags ‖ pq_prefix)`. A frame of one kind therefore cannot open in a
receive arm of another kind — cross-kind confusion is cryptographically excluded, so scripting
the arms loses no adversarial behaviour for the queried properties.

## 3. Results, per query — and what each does **NOT** license

All results are `RESULT … is true.` unless stated. Raw outputs live in the lane proof root; the
gate `formal/proverif/run_proverif_checks.py` re-asserts every line below, per query, fail-closed,
with the **tool sanity pair as its first assertion** (a verifier that only ever answers "true"
would be worse than none: the negative control must return `is false.`, or the gate stops).

| Q | Plain sentence | Result | Does **NOT** license |
|---|---|---|---|
| Q1 | An active network attacker never learns a message key (targets: a pre-boundary A→B message, a post-DH-boundary B→A message, a post-reseed A→B message). | proved | any computational or side-channel claim; nothing about implementation correctness |
| Q2 | Every accepted frame — plain, DH boundary, PQ reseed, and both advertisements — was injectively sent by the honest peer on the same transcript `(kind, direction, epoch DH_pub, counter, payload)`. | proved | non-repudiation; anything about identity binding at the handshake (A1) |
| Q3 | If **every** classical X25519 secret leaks **and** the attacker holds a post-establishment root snapshot, messages sent after a **PQ reseed** stay secret. | proved | **"post-quantum secure"** — the KEM is idealized (A5). Says nothing about ML-KEM's concrete hardness |
| Q4 | Same, after the NA-0626 **combined DH+PQ boundary** (one-message hybrid transition). | proved | as Q3 |
| Q5 | If the **ML-KEM decapsulation key** leaks (DH intact) plus a root snapshot, messages sent after a **DH boundary** stay secret. | proved | anything about ML-KEM's concrete hardness; **and see ENG-0034** — this holds only because the modeled honest sender always contributes a fresh exponent |
| Q6 | A planted or replayed advertisement is never tracked: `AdvTracked(id,pk)` injectively implies the honest peer's `AdvSent(id,pk)`. | proved | the NA-0625 end-to-end property is about the *implementation*; this is the model's |
| Q7 | **Guard-form only.** The accept path of both root-advancing receivers never carries the all-zero `DH_pub` (mirroring `is_zero32`, `ratchet.rs:1420`/`:2317`). | proved | **NOT an answer to whether a low-order key breaks the protocol.** See §5. `zeroG` carries no equation |

**Q3 + Q4 + Q5 together are the hybrid claim** — *security survives if **either** primitive
survives.* Reporting one direction alone would misstate the composition, and this document
therefore never quotes Q3 without Q5.

**The compromise is real, not decorative.** Each healing model asserts **canaries** that must come
back **`is false.`**:
- `m0` (pre-heal traffic) is readable under the modeled leak in Q3/Q4/Q5 — if it were not, the
  green beside it would be vacuous;
- `m_rs` / `m_cb` (the reseed's and the combined boundary's **own** body) are readable under full
  classical compromise, because those frames ride the **pre**-reseed key schedule by design
  (§8.5.3; `ratchet.rs:1752-1758`, `:1908-1911`). The reseed protects the messages **after** it.

A gate that only asserted `is true.` lines could pass on a model that compromised nothing. This
one asserts the reds too.

## 4. What this analysis found

**ENG-0034 (P2) — X25519 accepts non-contributory (low-order) peer keys; the DH output is never
checked.** Discharges Operator Decision 5 by code inspection against RFC 7748 §6.1 (§5 below).
Full evidence: `docs/governance/evidence/NA-0627_decision5_contributory_code_inspection.md`.
Not remotely exploitable against an honest pair — the Q1/Q2 envelope holds — but an authenticated
peer can silently void the **classical** half of post-compromise security. The PQ half still heals
(Q3/Q4), so the hybrid degrades rather than collapses. **Not fixed here: analysis lane.**

**ENG-0035 (P3) — ProVerif does not terminate on the design-locked 2-boundary unrolling.** See §6.

**No query disproved a security property of the shipped composition.** Had one, D564's STOP rule
would have applied: stop, file with a minimal trace, report — never an in-lane fix.

## 5. The one question a symbolic model cannot answer (Operator Decision 5)

D564 originally approved folding a contributory/low-order query into the model. **The design-lock
re-presented that decision rather than satisfying it, and the operator ratified the
re-presentation.** Two facts, both evidenced:

1. **The naive route breaks the tool.** Adding a degenerate element to ProVerif's DH theory
   (`const zeropoint: G. equation forall x; exp(zeropoint,x) = zeropoint`) makes ProVerif
   **diverge in equational completion on `process 0`** — before any protocol is modeled (proof
   root `dh_theory_finding.txt`). Option (b), "extend the theory", is **rejected by evidence**.
2. **More fundamentally, the theory cannot express the attack.** Standard symbolic DH *idealizes*
   the group; small-subgroup behaviour lives in the concrete group's structure. Such a model
   returns "secure" **regardless of whether the implementation screens low-order keys** — a
   vacuous green, exactly the failure the abstraction table exists to prevent.

**Discharge, as ratified:** answer the question **outside** the model, by code inspection against
RFC 7748, and file it with a severity → **ENG-0034**. Optionally model the **presence of the
check** as a guard → **Q7**, which is labelled, here and in every model header, as **not an
attack-existence proof**. Tamarin was considered and not taken: its exponent algebra is richer,
but low-order remains outside symbolic scope there too.

This is the lane's substantive methodological result: *a formal model must be allowed to report
that a question is outside its scope, rather than substituting a weaker question and reporting
success.*

## 6. Termination, and the reduction that was made (ENG-0035)

Expected: Q1/Q2/Q6 terminate readily; Q3–Q5 were the risk (unbounded ratchets are ProVerif's
classic non-termination case — the tradeoff Decision 1 recorded when choosing ProVerif over
Tamarin). **The outcome inverted that expectation.**

- Q3, Q4, Q5 each terminate (one root-advancing boundary apiece; seconds to ~1 minute).
- The **main** model at the design-locked bound (2 boundaries per direction: B's DH boundary then
  A's combined DH+PQ boundary) **does not terminate**: the root carries two nested `exp` terms
  under the commutativity equation and saturation diverged past 102 000 rules with no `RESULT`
  line at a 2400 s cap. A single secrecy query in isolation also diverged, so the cost is the
  **process**, not the query count. Raw: `nonterm_main_v1_{full,q1only}_2400s.out`.

**The design-lock's non-termination protocol was followed exactly:** (1) the non-terminating query
and its runtime are recorded in the proof root; (2) a reduced-scope model that **does** terminate
was produced, and **what was reduced is stated in the model header** — the second root-advancing
DH boundary (A's combined boundary) and the message after it were removed from the main model;
(3) the non-termination is filed as **ENG-0035**; (4) the **Tamarin option is re-presented** to
the operator for this query shape.

**No query was weakened.** Q1/Q2/Q6/Q7 are stated over the full reduced schedule and all pass. The
combined boundary is not unmodeled — it is verified with its own compromise scenario and its own
guard-form query in `suite2_dhpq_q4_combined_healing.pv`. **Residual gap, stated:** no single model
exercises two consecutive root-advancing DH epochs, so an attack requiring a second DH epoch would
not be found. Nothing suggests one exists.

The main model's post-reseed message additionally exercises the **ENG-0030 full-schedule commit**:
A never recomputes its own send half — the reseed *receive* refreshes it — so if that commit were
stale, B could not open the message and Q2 would fail.

## 7. Claim boundary (Operator Decision 4) — **UNCHANGED**

The executor drafts; the operator decides. **Recommendation: change nothing.**

What these results would, at most, support — *and only alongside the human review that remains
outstanding*:

> "The Suite-2 hybrid key schedule as shipped has been analyzed in a symbolic (Dolev-Yao) model
> with ProVerif 2.05. Under the stated abstractions — idealized KDF, AEAD, X25519 group, and
> ML-KEM — message-key secrecy and injective transcript agreement hold against an active network
> attacker; and the composition heals in **both** directions: post-reseed and post-combined-
> boundary traffic stays secret when all classical DH secrets are compromised, and post-DH-boundary
> traffic stays secret when the ML-KEM decapsulation key is compromised. This is a symbolic result
> over abstracted primitives. It is **not** a post-quantum security claim, **not** a proof about
> the implementation, and **not** a substitute for independent human review."

What they do **not** support, and what the standing boundary therefore keeps forbidding: any
"post-quantum", "Triple-Ratchet", "post-compromise secure", "crypto-complete", "bug-free", or
"vulnerability-free" claim. **ENG-0034 independently blocks post-compromise language** until the
contributory check exists: the classical half of PCS is currently voidable by the peer.

## 8. For the external reviewer: where to start

1. This document §2 (what is idealized) and §3 (what each result does not license).
2. `formal/proverif/suite2_dhpq_lib.pvl` — the fidelity map. Every derivation cites its shipped
   function and normative section. **Check the model against the code, not against this prose.**
3. `docs/governance/evidence/NA-0627_decision5_contributory_code_inspection.md` — the one question
   the model could not answer, and why.
4. `formal/proverif/run_proverif_checks.py` — the gate, including the canaries and the sanity pair.
5. The bounded Python explorers in `formal/` remain the fast, always-on regression guard for the
   SCKA control-plane *logic*; ProVerif is the layer above, not a replacement.

Known limits, restated plainly: one session, two parties, establishment authentication assumed,
in-order receives only, one DH epoch per model, idealized primitives, and **no low-order/
small-subgroup reasoning of any kind**.
