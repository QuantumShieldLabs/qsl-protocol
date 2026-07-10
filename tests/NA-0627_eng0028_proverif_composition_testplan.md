# NA-0627 — ENG-0028: independent formal analysis of the Suite-2 DH+PQ composition (ProVerif) — Test Plan

Directive: QSL-DIR-2026-07-09-564 (D564). Decision: D-1249. Implements ENG-0028; files ENG-0034
(P2) and ENG-0035 (P3).

## Objective

Prove that the Suite-2 hybrid key schedule **as shipped post-NA-0626** (base `d9baed9d`) resists an
active Dolev-Yao adversary in a symbolic model, that it heals in **both** directions (PQ-only and
classical-only compromise), and that the SCKA control plane is unforgeable — with the model's
fidelity to the shipped code **proved, not assumed**, and every abstraction naming the property it
masks. **Analysis lane: no source, vector, canonical-doc, or primitive change.** A disproved
property would be a STOP + filing; a non-termination is a FINDING, not a lane failure.

## Tool gate (fail-closed, and PROVED able to fail)

Tool: **ProVerif 2.05**, version-pinned (Operator Decision 1). Runner:
`formal/proverif/run_proverif_checks.py`. The runner asserts the expected `RESULT` line **per
query** — never a bare exit code — and its **FIRST** assertion is the tool sanity pair.

Each expectation is a **regex matched against the run's `RESULT` lines**, not a plain substring:
ProVerif renames bound query variables (`n_28`, …) as a model evolves, and an assertion keyed on a
generated name would fail for the wrong reason. What is pinned exactly is the SEMANTIC content —
which event implies which, and `is true.` versus `is false.`

1. **Tool sanity pair (binding, D564).** Positive control (secret never emitted) →
   `RESULT not attacker(s[]) is true.`; negative control (`process out(c,s)`) →
   `RESULT not attacker(s[]) is false.` GREEN. *The negative control is the load-bearing one: it
   proves the verifier REFUTES a false secrecy claim rather than vacuously accepting.*
2. **Version assertion.** `proverif -help` must report `2.05` or the runner stops. GREEN.
3. **Mutation test A — the gate fails closed on a wrong expectation.** A deliberately corrupted
   expected `RESULT` line for `suite2_dhpq_q5_dh_healing.pv` makes the runner exit **1** with
   `[MISSING] … [FAIL] 1 expected RESULT line(s) absent.` GREEN (exit 1 as required).
4. **Mutation test B — a LYING verifier stops the gate before any protocol model runs.**
   Simulating a negative control that returns `is true.` (by making it not leak) makes the runner
   exit **1** at the sanity pair with `[STOP] The tool sanity pair failed`, and **zero** protocol
   models execute. GREEN (exit 1; `grep -c suite2_dhpq` on the output = 0).

   *Tests 3 and 4 exist because a green gate that cannot fail is not a gate. Both were run against
   temporary copies; neither artifact is committed.*

## Model fidelity (the claim this lane must prove about itself)

`formal/proverif/suite2_dhpq_lib.pvl` is the single place the encoding lives (loaded by every model
via `proverif -lib`, so no query can drift). Each of the 11 modeled steps M1–M11 names the shipped
function and its normative section — `establish.rs:23` (§8.2); `ratchet.rs:1283`/`:1383` (§8.5.2 +
§8.5.1); `:1700`/`:2210` (§8.5.3 + §3.3.6); `:1848`/`:2293` (DOC-G5-008 §4); `scka.rs:60`/`:37`
(DOC-CAN-004 §3.4); `:1588`/`:2017`+`:2506` (§8.5.4 + ADVAUTH); `derive_mk_step` (§3.3) — and the
complete **20-label derivation alphabet** is encoded as distinct symbolic labels, so label
separation is inspectable rather than asserted.

Abstractions A1–A8 (Operator Decision 3, ratified) are published with the property each MASKS in
`docs/design/DOC-G4-002` §2. The load-bearing ones: **A1** establishment auth assumed; **A4**
X25519 as an idealized group (masks low-order behaviour → answered outside the model, ENG-0034);
**A6** bounded epoch unrolling, **as reduced** (→ ENG-0035); **A7** in-order receives only.

## Query results (raw outputs in the lane proof root; class-only here)

`formal/proverif/suite2_dhpq_main.pv` — no compromise, active adversary:

1. **Q1 message-key secrecy.** `not attacker(m1)` / `(m2)` / `(m4)` — pre-boundary A→B,
   post-DH-boundary B→A, post-PQ-reseed A→B. All **`is true.`** GREEN.
2. **Q2 injective transcript agreement.** `inj-event(MsgAcc(kind,dir,DH_pub,n,payload)) ==>
   inj-event(MsgSent(…))` over every accepted frame — plain, DH boundary, PQ reseed, both
   advertisements. **`is true.`** GREEN.
3. **Q6 control plane.** `inj-event(AdvTracked(id,pk)) ==> inj-event(AdvSent(id,pk))` — a planted
   or replayed advertisement is never tracked. **`is true.`** GREEN.
4. **Q7 guard-form (DH-boundary arm).** `not event(BoundaryAccepted(zeroG))` — the accept path
   never carries the all-zero `DH_pub`, mirroring `is_zero32` (`ratchet.rs:1420`). **`is true.`**
   GREEN. **NOT an attack-existence proof** (§ Decision 5 below); `zeroG` carries no equation.

   *Q1's `m4` target additionally exercises the **ENG-0030 full-schedule commit**: A never
   recomputes its own send half — the reseed RECEIVE refreshes it (`ratchet.rs:2263-2275`) — so a
   stale send half would make B unable to open `m4` and Q2 would fail.*

`suite2_dhpq_q3_pq_reseed_healing.pv` — phase-1 compromise of **all** classical X25519 secrets
(`dh_init`, both establishment privates, the boundary private) **plus a post-establishment root
snapshot** (a strict superset of D564's set: strengthen, do not weaken):

5. **Q3 PQ healing across a PQ reseed.** `not attacker_p2(secret_q3)` **`is true.`** GREEN.
6. **Canary.** `not attacker_p2(m0)` **`is false.`** GREEN-as-RED. *Pre-heal traffic MUST be
   readable, or the Q3 green above would be vacuous.*
7. **Expected-red probe.** `not attacker_p2(m_rs)` **`is false.`** GREEN-as-RED. *The reseed
   frame's own body rides the PRE-reseed key schedule by design (§8.5.3; `ratchet.rs:1752-1758`);
   the reseed protects the messages AFTER it.*

`suite2_dhpq_q4_combined_healing.pv` — same compromise, across the NA-0626 combined boundary:

8. **Q4 PQ healing across the COMBINED DH+PQ boundary.** `not attacker_p2(secret_q4)`
   **`is true.`** GREEN.
9. **Canary** `m0` **`is false.`**; **expected-red probe** `m_cb` **`is false.`** (the combined
   frame's body derives from the fresh pre-seed epoch chains, `ratchet.rs:1908-1911`,
   DOC-G5-008 §4). GREEN-as-RED.
10. **Q7 guard-form (combined-boundary arm).** `not event(BoundaryAccepted(zeroG))` **`is true.`**
    GREEN, mirroring `ratchet.rs:2317`. Still **not an attack-existence proof**.

`suite2_dhpq_q5_dh_healing.pv` — phase-1 compromise of the **ML-KEM decapsulation key** and
`pq_init_ss` **plus the root snapshot**, X25519 intact:

11. **Q5 classical healing across a DH boundary.** `not attacker_p2(secret_q5)` **`is true.`**
    GREEN.
12. **Canary** `m0` **`is false.`** GREEN-as-RED.

> **Q3 + Q4 + Q5 TOGETHER are the hybrid claim** — security survives if **either** primitive
> survives. Quoting one direction alone would misstate the composition, and neither the analysis
> record nor this plan does so.

**No query disproved a security property of the shipped composition.** D564's STOP rule was not
triggered.

## Findings (filed, not fixed — analysis lane)

- **ENG-0034 (P2)** — Decision 5's discharge. The X25519 **DH output** is never checked for the
  all-zero (non-contributory) value at any of the four Suite-2 call sites (`ratchet.rs:1306`,
  `:1475`, `:1885`, `:2390`) or the QSP handshake's `dh1`/`dh2` (`qsp/handshake.rs:134`, `:144`,
  `:285`, `:297`); `was_contributory()` is never called; `X25519Dh::dh` returns a bare `[u8;32]`,
  discarding the flag at the trait boundary; and `is_zero32(&parsed.dh_pub)` rejects exactly ONE of
  Curve25519's eight small-order encodings. Evidence:
  `docs/governance/evidence/NA-0627_decision5_contributory_code_inspection.md`.
- **ENG-0035 (P3)** — ProVerif does not terminate at the design-locked 2-boundary unrolling
  (>102,000 rules, no `RESULT`, 2400 s cap; a single secrecy query in isolation also diverged).
  Reduced-scope main model produced, **reduction stated in the model header, no query weakened**,
  combined boundary separately verified in the Q4 model, **Tamarin re-presented** to the operator.

## Decision 5 (X25519 contributory) — why it is NOT in the model

Probed, not assumed: extending ProVerif's DH theory with a degenerate element makes the tool
**diverge in equational completion on `process 0`** (proof root `dh_theory_finding.txt`). More
fundamentally, standard symbolic DH **cannot express** small-subgroup attacks — the theory
idealizes the group and would return "secure" **regardless** of whether the implementation screens
low-order keys (a vacuous green). Option (b) "extend the theory" is **FORBIDDEN by evidence**.
Discharged as ratified: **(c)** answer outside the model by code inspection against RFC 7748 §6.1 →
**ENG-0034**; **(a)** the guard-form Q7, labelled in every model header as **not** an
attack-existence proof. **The low-order algebra was not modeled.**

## Phase-5 gates (derived mechanically from the touched workflows)

- `python3 formal/run_model_checks.py` (the five bounded Python explorers, untouched by this lane —
  they remain the fast, always-on regression guard). GREEN.
- `python3 formal/proverif/run_proverif_checks.py` (the new ProVerif gate; sanity pair asserted
  first). GREEN.
- `.github/workflows/formal.yml` — YAML parse verified; exactly one **additive** job
  (`formal-proverif-composition`); the existing `classify` and `formal-scka-model` jobs are
  byte-unchanged.
- No Rust file is touched, so fmt/build/clippy gates do not apply to this lane (asserted by the
  scope guard: `git diff --stat` shows zero `*.rs`, zero `inputs/**`, zero `docs/canonical/**`).

## Boundary / claim

`formal/proverif/**` (new), `.github/workflows/formal.yml` (one additive job),
`FORMAL_VERIFICATION_PLAN.md`, `formal/README.md`, `docs/design/DOC-G4-002_*`,
`docs/ops/IMPROVEMENT_LEDGER.md`, `docs/governance/evidence/NA-0627_*`, `tests/NA-0627_*`,
governance (`DECISIONS.md`, `TRACEABILITY.md`, `NEXT_ACTIONS.md`,
`docs/ops/ROLLING_OPERATIONS_JOURNAL.md`). **No** `tools/refimpl/**`, `qsl/**`, `apps/**`,
`inputs/**`, `docs/canonical/**`, Cargo/lockfile, or `.claude` change. No package installed by the
executor.

**Claim boundary (Operator Decision 4): UNCHANGED.** A green ProVerif result is necessary input to,
not sufficient grounds for, any post-quantum / Triple-Ratchet / post-compromise claim — it is a
symbolic result over abstracted primitives against a Dolev-Yao adversary and says nothing about
computational hardness, side channels, or this implementation. Independent human review remains an
open prerequisite. Candidate sentences are drafted in `DOC-G4-002` §7 **for the operator to
decide**; the executor moved no claim. **ENG-0034 independently blocks post-compromise language.**
No endpoint, token, capability, key, seed, plaintext, ciphertext body, or raw private material is
published; raw private values stay proof-root-only.
