# NA-0627 Phase-2 DESIGN-LOCK — ENG-0028: independent formal analysis of the Suite-2 DH+PQ composition (D564)

Goals: G1, G2, G4

Lane: NA-0627 (sole READY at D-1249). Directive: QSL-DIR-2026-07-09-564 (APPROVED; Decisions 1, 2,
4, 5 approved as recommended, Decision 3 = design-lock output ratified here). Base: main ==
origin/main == `d9baed9d` (clean). Recorded: 2026-07-09 (UTC).
Status: DESIGN-LOCK COMPLETE — **no model code written** (Phase-2 discipline). Implementation
(Phase 3+) resumes in a fresh chat per the session-handoff convention (ONE handoff, at this
boundary).

**HEADLINE: Decision 5 is RE-PRESENTED, not silently satisfied.** Probe evidence (§4) shows the
contributory/low-order query as framed is *not soundly answerable in a symbolic model*, and the
naive route makes ProVerif diverge. The design-lock recommends a different, honest discharge.

---

## 0. Phase 0/1 verification snapshot

Phase 0 (live, 2026-07-09T22:20Z): `HEAD == origin/main == main == d9baed9d`; `git status
--porcelain` empty; NEXT_ACTIONS.md exactly one anchored `^Status: READY` = NA-0627; DECISIONS.md
highest canonical `**ID:**` = D-1248 and ZERO canonical `D-1249` records; highest directive on disk
= 564; disk 34% (<95%); `/backup/qsl` mounted; on-disk kv proof matches the D564 splice
(`proof_written_at_utc=2026-07-09T22:19:04Z`). Tool: `proverif -help` → `Proverif 2.05`; the
mandated sanity pair re-ran `is true.` / `is false.`. No STOP condition.

Phase 1: D-1247 and D-1248 consumed once each, both `Status: Accepted`. Read: the NA-0626
design-lock + evidence (all five files survive tracked on main), DOC-G5-008 §4 + its NA-0626 note,
ledger ENG-0028/ENG-0029, `FORMAL_VERIFICATION_PLAN.md`, and `formal/` (five bounded Python models
+ runner + README).

**The milestone this lane discharges, verbatim from FORMAL_VERIFICATION_PLAN.md §Tooling:** "select
ProVerif vs Tamarin and record in DECISIONS.md once chosen," for "cryptographic properties
(secrecy/authentication/PCS) beyond the bounded logic model."

## 1. Pin (a) — tool selection: ProVerif 2.05 (Decision 1, APPROVED; evidence recorded)

**Version re-check (design-lock duty, not drift):** opam publishes `proverif.2.02 … 2.05`; 2.05 is
the newest. The D564 pin `proverif.2.05` stands. Installed and verified: `Proverif 2.05`,
clean-env invocable, no GTK linkage.

**Capability probes (run, not assumed):**
- Sanity pair: proves (`is true.`) AND refutes (`is false.`) — a verifier that only ever accepts
  would be worse than none.
- **Phases work**: `query attacker(s) phase 1` → `RESULT not attacker_p1(s[]) is false.` This is the
  mechanism for post-compromise / healing queries (compromise in phase 1, prove secrecy of phase-2
  traffic). PCS modeling is therefore *available*, which is the single most important capability
  this lane needs and the main thing Tamarin was going to be chosen for.
- Baseline DH theory (`exp(exp(g,x),y) = exp(exp(g,y),x)`) terminates instantly.

**Tradeoff on THIS protocol, recorded (Decision 1 was approved on this reasoning; evidence now
supports it):** ProVerif's applied-pi/Horn-clause abstraction terminates well on key-exchange
secrecy and injective-agreement queries and has first-class `phase` support for PCS — the exact
query shapes in the DoD. Tamarin's multiset rewriting handles unbounded ratchet state and PCS
lemmas more naturally, at the cost of hand-written oracles/lemmas and a much larger lane.
**Residual risk, stated:** the Suite-2 ratchet is unbounded; ProVerif may not terminate on healing
queries over an unrolled epoch schedule. §6 pins exactly what happens then. **It must never
silently weaken a query to reach a green result.**

## 2. Pin (b) — execution + CI-gating (Decision 2, APPROVED = CI-gated)

Mechanism (additive job in `.github/workflows/formal.yml`; that file is the ONLY `.github/**` path
this lane may touch, and only this new job):

```
- uses: ocaml/setup-ocaml@v3      with: { ocaml-compiler: "4.14" }   # opam + cache
- run:  opam install -y proverif.2.05                                 # PINNED
- run:  opam exec -- proverif -in pitype formal/proverif/<model>.pv   # assert expected outcomes
```

**KNOWN COST, surfaced pre-lane (not at Phase 5):** the opam package `proverif.2.05` builds against
`lablgtk` (ProVerif's *interactive* mode) → `conf-gtk2` → the system package `libgtk2.0-dev` (~24
apt packages, ~24.5 MB). **The headless binary links no GTK** (`ldd` confirms), so the GTK
dependency is a BUILD cost only. Design-lock decision, ratified by the operator here:
- **CI installs `libgtk2.0-dev`** (simple, honest, ~30–60 s uncached) and **caches the opam root**
  keyed on `proverif.2.05`, so the OCaml/lablgtk build is paid once per cache generation.
- A headless build path (INRIA tarball + `./build` without interactive) is NOT taken in-lane: it
  trades a documented apt install for an undocumented build script. If CI cost proves unacceptable,
  the **pre-authorized fallback (2b)** applies.

**Gate assertions (fail-closed).** The CI job MUST assert *per query* the expected `RESULT` line,
not merely a zero exit. **The tool sanity pair (positive + negative control) is the job's FIRST
assertion**: if the negative control ever returns `true`, the gate is lying and the job fails.
A green gate that cannot fail is not a gate.

**Fallback (2b), pre-authorized by D564 without re-presenting:** if the CI install proves flaky, the
executor degrades to an operator-run model (checked in, documented invocation, class-only results
pasted into the response) and MUST record the degradation, its cause, and the evidence at D-1249.

The five bounded Python explorers stay untouched: they are the fast, always-on regression guard.
ProVerif is the layer above, not a replacement.

## 3. Pin (c) — model shape + FIDELITY MAP to the shipped code

**Scope: one Suite-2 session between two authenticated parties (A = initiator, B = responder), the
network under full Dolev-Yao control.** The model's ground truth is the post-NA-0626 single-root
state (`Suite2SessionState { rk, send, recv, dh }`).

**Establishment is modeled as AUTHENTICATED-BY-ASSUMPTION** (long-term keys pre-shared in the
model, `authenticated == true` as `init_from_base_handshake` requires). The handshake's own
signature/KEM/transcript binding is already covered by
`formal/model_qsc_kem_signature_transcript_binding_bounded.py` and is OUT OF SCOPE here. This is an
abstraction with a masked property — recorded in §4, row A1.

**Fidelity map — every modeled step names the shipped function and the normative section.**
Fidelity is a CLAIM and is proved against the code, not assumed (WF-0014's discipline, applied to a
model):

| # | Modeled step | Shipped function (post-NA-0626) | Normative |
|---|---|---|---|
| M1 | establishment: `RK0 = KDF(dh_init, "QSP5.0/RK0", sid‖0x01)`; `RK = KDF(RK0, "QSP5.0/RKPQ", pq_init_ss‖0x01)`; then `HK/CK0/PQ0` per direction | `establish.rs:23 init_from_base_handshake` | DOC-CAN-003 §8.2 |
| M2 | DH boundary SEND: fresh keypair; `(RK',CK_ec0)=KDF_RK_DH(RK,dh_out)`; `CK_pq0=PQ0(RK')`; header under **pre-boundary NHK_s**; n=0 | `ratchet.rs:1283 send_boundary` | §8.5.2 + §8.5.1 |
| M3 | DH boundary RECV: NHK-only open at n=0; same derivation; commit `rk`, `dhr`, recv chains, `nr:=1` | `ratchet.rs:1383 recv_dh_boundary` | §8.5.2 + §8.5.1 |
| M4 | PQ reseed SEND: seeds from `RK_old` then `RK'=KDF_RK_PQ(RK_old,ss)`; header under pre-reseed NHK_s | `ratchet.rs:1700 send_pq_reseed` | §8.5.3 + §3.3.6 |
| M5 | PQ reseed RECV (session-level, full schedule incl. send half) | `ratchet.rs:2210 recv_pq_reseed` | §8.5.3 (+ ENG-0030) |
| M6 | **COMBINED DH+PQ boundary SEND**: DH first (`RK_dh`), seeds + transient `PQ0` from `RK_dh`, then `RK_final=KDF_RK_PQ(RK_dh,ss)`; body mk from the fresh pre-seed epoch chains; n=0 of the new epoch | `ratchet.rs:1848 send_combined_boundary` | §8.5.2+§8.5.3, DOC-G5-008 §4 |
| M7 | **COMBINED boundary RECV**: fresh-`DH_pub` discrimination; NHK-only at n=0; identical composition | `ratchet.rs:2293 recv_combined_boundary` | §8.5.2+§8.5.3 |
| M8 | SCKA reseed validation: monotonicity / one-time / tombstone; directional seeds | `scka.rs:60 apply_pq_reseed`, `scka.rs:37 kdf_pq_reseed_seeds` | DOC-CAN-004 §3.4, §3.3.6 |
| M9 | ADV SEND: header under `HK_s`; `adv_mac=KMAC32(RK,"QSP5.0/ADVAUTH",id‖pub‖0x01)` first 32 B of the sealed body | `ratchet.rs:1588 send_pq_advertise` | §8.5.4 + ENG-0023 |
| M10 | ADV RECV (authenticated, chain-consuming, in-order) | `ratchet.rs:2017 recv_pq_adv` / `:2506 recv_pq_adv_session` | §8.5.4, DOC-CAN-004 §3.2 |
| M11 | message keys: `mk = KDF_HYBRID(ec_mk, pq_mk)`; chains step per message | `ratchet.rs derive_mk_step` | §3.3 |

**Complete derivation alphabet the model must encode (extracted from the shipped code, 20 labels):**
`RK0, RKPQ, RKDH, HK/{A->B,B->A}, NHK/{A->B,B->A}, CK0/A->B, PQ0/{A->B,B->A}, CK, MK, PQCK, PQMK,
HYBRID, ADVAUTH, HDR-NONCE, BODY-NONCE, PQSEED/{A->B,B->A}, SCKA/CTXT`.

**Adversary:** full network control (read/inject/drop/reorder), plus `phase`-scoped compromise of
selected long-term/ephemeral/KEM secrets for the healing queries.

## 4. Pin (d) — ABSTRACTION TABLE (Decision 3 — operator ratifies)

Each idealization, and **the property it could mask**. A model whose abstraction hides the property
it claims to prove is worse than no model.

| # | Abstraction | Property it MASKS | Disposition |
|---|---|---|---|
| A1 | Establishment authentication assumed (long-term keys pre-shared; `authenticated=true`) | identity misbinding / downgrade AT the handshake | Covered by the existing KEM/signature/transcript bounded model; recorded, not re-proved |
| A2 | KMAC/SHA as perfect one-way functions (free constructors, no collisions) | length-extension, collision, weak-label separation | Standard symbolic assumption; label separation is verified by inspection of the 20-label alphabet |
| A3 | AEAD as ideal `senc/sdec` (no nonce-reuse semantics) | nonce reuse, AEAD misuse, ciphertext malleability | The §8 counter hard-stop (ENG-0013) guards reuse; masked here, guarded there — recorded |
| A4 | **X25519 as an idealized symbolic group** (`exp` + commutativity) | **small-subgroup / low-order points, non-contributory shared secrets — hence the presence or ABSENCE of a contributory check in the implementation** | **See §5. Cannot be lifted: probe evidence shows the theory extension does not terminate. Answered OUTSIDE the model.** |
| A5 | ML-KEM as an ideal KEM (`encap/decap`, no decapsulation failure) | decap-failure oracles, malformed-ct behaviour, implicit-rejection semantics | Recorded; the SCKA one-time/tombstone rules (M8) ARE modeled, so replay/reuse is in scope |
| A6 | Bounded epoch unrolling (see §6) | attacks that need more epochs than unrolled | Bound stated per query; non-termination handled by §6, never by weakening |
| A7 | `mkskipped` skip-window out of scope | out-of-order/skip-window key-retention attacks | Recorded as out of scope; covered by the ooo_replay conformance vectors, not by this model |
| A8 | qsc cadence/policy out of scope (D561 operator-set) | metadata/timing distinguishers | Already tracked as ENG-0022; not a secrecy property |

## 5. Pin (e/Decision 5) — the X25519 contributory query: **RE-PRESENTED, with evidence — RATIFIED**

> **OPERATOR RATIFICATION, 2026-07-09:** the recommended discharge below is ACCEPTED.
> **(c) is BINDING** — answer the contributory question by code inspection against RFC 7748 and
> file the ENG item with evidence + severity. **(a) is optional** and, if done, must be labelled as
> NOT an attack-existence proof. **(b) is FORBIDDEN** (rejected by evidence). Chat B must NOT model
> the low-order algebra. The fix, if warranted, stays out of scope.

**What D564 approved:** fold a contributory/low-order query into the model (option 5a).
**What the evidence shows (probes in the proof root, `dh_theory_finding.txt`):**

- Baseline DH theory + trivial process → terminates instantly, `is true.`
- DH theory **+ a degenerate element** (`const zeropoint: G. equation forall x; exp(zeropoint,x) =
  zeropoint`) → **ProVerif does NOT terminate** (timeout at 45 s and again at 120 s) on
  `process 0`. It printed the theory's "Linear part:" and diverged in equational completion. The
  divergence is caused by the **theory**, not by any protocol.

**Deeper point (this is the one that matters):** standard symbolic DH models **cannot express**
small-subgroup / low-order attacks at all. The equational theory *idealizes the group*; the attack
lives in the concrete group's structure. A model on the standard theory returns "secure"
**regardless of whether the implementation screens low-order keys** — a vacuous green, and exactly
the failure Decision 3 exists to prevent. Reporting such a result as "the contributory question is
answered" would be an overclaim.

**Design-lock recommendation (operator ratifies):** discharge Decision 5 as **(c) + (a)**, not (b):
- **(c) Answer it OUTSIDE the symbolic model** — by code inspection of the shipped DH paths against
  RFC 7748's contributory-behaviour guidance — and **file the ENG item with that evidence and a
  severity**. Cheap, sound, and it actually answers the question. *(Read-only finding already in
  hand from the status sync: no contributory/all-zero check exists; a zero `DH_pub` is rejected at
  the boundary receiver, and establishment inputs are authenticated — so the exposure is bounded but
  real, and the severity assessment belongs in the ENG filing.)*
- **(a) Optionally model the PRESENCE of the check** as a guard/destructor (`peer key is valid`),
  which documents the assumption the guarantees rest on. **This is not an attack-existence proof and
  must never be reported as one.**
- **(b) Extending the theory: REJECTED BY EVIDENCE** (non-termination).
- **(d) Tamarin: not recommended** — richer exponent algebra, but low-order is still outside
  symbolic scope.

The FIX, if warranted, remains out of scope: this is an analysis lane.

## 6. Pin (f) — query set, termination, and the non-termination protocol

**Query set.** Each query: formal shape, one plain sentence, the DoD item, and what it does NOT
license. All use `phase` for compromise scoping.

| Q | Formal | Plain sentence | DoD | Does NOT license |
|---|---|---|---|---|
| Q1 | `query attacker(msg_k)` | An active network attacker never learns a message key. | (3) secrecy | any computational/side-channel claim |
| Q2 | `inj-event(accept(t)) ==> inj-event(send(t))` | Every accepted message was injectively sent by the honest peer on the same transcript. | (3) authentication | non-repudiation |
| Q3 | phase 1 compromise of **all classical DH private keys**; `query attacker(msg_k) phase 2` after a **PQ reseed** | If every X25519 secret leaks, messages after a PQ reseed stay secret. | (3) PQ healing | "post-quantum secure" — the KEM is idealized (A5) |
| Q4 | as Q3 but after a **COMBINED DH+PQ boundary** | Same, for the NA-0626 one-message hybrid boundary. | (3) PQ healing | as Q3 |
| Q5 | phase 1 compromise of the **ML-KEM decapsulation key** (DH intact); `query attacker(msg_k) phase 2` after a **DH boundary** | If the KEM secret leaks, messages after a DH boundary stay secret. | (3) classical healing | anything about ML-KEM's concrete hardness |
| Q6 | `query attacker(tracked_adv_pub)` / event-based | A planted advertisement is never tracked by an honest party. | (3) control plane | the NA-0625 e2e property is *implementation*; this is the model's |
| Q7 | guard-form only, per §5 | The guarantees are stated relative to "the peer DH key is valid." | (5), re-presented | **NOT** an answer to whether a low-order key breaks the protocol |

**Q3/Q4/Q5 together are the hybrid claim:** security survives if *either* primitive survives.
Reporting only one direction would misstate the composition.

**Termination plan.** Expected: Q1/Q2/Q6 terminate readily (standard shapes). Q3–Q5 are the risk:
they require an unrolled epoch schedule, and unbounded ratchets are ProVerif's classic
non-termination case.
- **Bound:** unroll **2 boundaries per direction** (enough to express "compromise → boundary →
  secrecy", and to distinguish DH-only, PQ-only, and combined). The bound is a *stated
  abstraction* (A6), not a hidden one.
- **On non-termination — the protocol, pinned:** (1) record the non-terminating query and its
  runtime in the proof root; (2) produce the reduced-scope model that DOES terminate and state
  exactly what was reduced; (3) file the non-termination as a **FINDING**; (4) re-present the
  Tamarin option for that query to the operator. **Never** weaken the query and report green.
  Attacks and non-termination are FINDINGS, not lane failures (D564 DoD 4).

## 7. STOP-condition sweep (all clear at design-lock)

- `proverif` invocable, 2.05, sanity pair proves and refutes. ✓
- No source/vector/canonical-doc mutation planned; `.github/**` limited to the single additive
  formal job (Decision 2). ✓
- No package installation by the executor (the operator provisioned; recorded). ✓
- The one query that could not be honestly modeled (Decision 5) is **re-presented, not silently
  substituted**. ✓
- Analysis-lane rule understood: a disproved security property → STOP + file + report, never an
  in-lane fix. ✓

## 8. Chat B starter (one line, per the session-handoff convention)

"Resume NA-0627 at Phase 3 in /srv/qbuild/work/NA-0627/qsl-protocol; do NOT re-run qnext/qwork; do
NOT re-derive the design-lock — read the archived D564 directive's DESIGN-LOCK CONCLUSIONS +
docs/governance/evidence/NA-0627_design_lock.md + the proof root
/srv/qbuild/tmp/NA0627_eng0028_proverif_dhpq_20260709T222013Z (sanity pair + the DH-theory
non-termination finding), re-verify Phase 0 live (worktree clean; proverif 2.05; sanity pair), then
author the model under formal/proverif/ per §3 and run the §6 query set through closeout
(D-1249/D-1250). Decision 5 is RE-PRESENTED — do not model the low-order algebra; discharge it per
§5 and file the ENG item."
