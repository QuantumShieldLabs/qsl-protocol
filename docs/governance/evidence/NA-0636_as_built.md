# NA-0636 — QSC.HS.* bounded handshake-authentication model (as-built)

Goals: G1, G2, G4

Directive: QSL-DIR-2026-07-11-572 (D572, APPROVED). Decision: D-1259. Lane class: formal/ + governance
only — ZERO qsc/refimpl/protocol/wire/crypto/state-machine source change. Base main `8e8699db`
(merge of PR #1551, the NA-0636 queue promotion). Pays the ⚠ OPEN ENG-0038 verification obligation
(`docs/ops/IMPROVEMENT_LEDGER.md`, NA-0634 caveat): the responder→initiator sig-pin redundancy is
decided by the MODEL, not by reasoning.

This document has four parts, written in phase order: §1 the abstract accept/reject semantics
extracted read-only from the landed code (Phase 1 — written BEFORE any model code); §2 the model
design and its P1–P4 encoding (Phase 2); §3 the model run results and exact bound (Phase 2/4);
§4 the P3 obligation verdict and the ledger action taken (Phase 3).

---

## §1 Abstract authentication semantics of the landed handshake (Phase 1, read-only extraction)

Sources (read-only): `qsl/qsl-client/qsc/src/handshake/mod.rs`, `src/identity/mod.rs`,
`src/contacts/mod.rs` at `8e8699db`; `docs/governance/evidence/NA-0634_as_built.md`;
`docs/governance/evidence/NA-0633_design_lock.md`. Line references are to `8e8699db`.

### 1.1 Token universe (what the model abstracts)

- **Identity** = pair `(kem_id, sig_id)` — the ML-KEM identity keypair and the ML-DSA signing
  keypair. Each `*_id` has a public half (sendable) and a secret half (possession).
- **Combined verification code** `CODE(kem_id, sig_id)` =
  `identity_fingerprint_from_identity(kem_pk, sig_pk)` (`identity/mod.rs:202`) — the single
  human-verified code. Crypto-agnostic model assumption: **injective on the pair** (distinct
  pairs ⇒ distinct codes; no collision modeling).
- **Single-key code** `CODE1(k)` = `identity_fingerprint_from_pk(pk)` (`identity/mod.rs:190`)
  — the legacy (pre-NA-0634) KEM-only code. Model assumption: `CODE(·,·)` and `CODE1(·)` never
  collide with each other or across distinct inputs (different pre-image domains).
- **Signing fingerprint** `SIGFP(sig_id)` = `hs_sig_fingerprint(sig_pk)`
  (`handshake/mod.rs:925`) = `identity_fingerprint_from_pk(sig_pk)` (`contacts/mod.rs:1059`) —
  the sig-pin comparand. Injective on `sig_id`.
- **KEM possession**: `encap(kem_pk) → (ct, ss)`; `ss` is derivable from `ct` ONLY by the holder
  of that `kem_pk`'s secret half. No other party can compute `ss` (crypto-agnostic possession
  semantics — no computational hardness claim).
- **Signature possession**: a signature over `msg` verifying under `sig_pk` is producible ONLY
  by the holder of that `sig_pk`'s secret half. Verification under a peer-SUPPLIED `sig_pk` is
  self-consistent for ANY key (the ENG-0038 lesson) — authentication comes only from pinning.
- **MAC/confirm**: `MAC(key, data)` computable only with `key`; the handshake keys derive from
  `pq_init_ss = hs_root_combine("QSC.HS.PQ", sid, [ss_pq, resp_kem_ss])`
  (`handshake/mod.rs:827-839`), so MAC forgery reduces to KEM-secret possession.

### 1.2 Contact-store states reachable from provisioning (the pin store)

Per contact label the store carries `pin` (the fp field), `kem_pk`, `sig_fp`
(read via `identity_read_pin` `identity/mod.rs:635`, `identity_read_sig_pin` `:649`,
`identity_read_peer_kem_pk` `:661`). Reachable states from `contacts_add`
(`contacts/mod.rs:1024-1127`) and `contacts_device_add` (`:1139`):

| State | pin | kem_pk | sig_fp | How reached | Add-time check |
|---|---|---|---|---|---|
| S-FULL | `CODE(k,s)` | `k` | `SIGFP(s)` | `contacts add --fp --kem-pk --sig-pk` | pin MUST equal `CODE(k,s)` (fail-closed `contacts_identity_fp_mismatch`, `:1053-1056`) |
| S-KEM (legacy) | `CODE1(k)` | `k` | ∅ | `contacts add --fp --kem-pk` | pin MUST equal `CODE1(k)` (`:1067-1070`) |
| S-BARE | any code `C` | ∅ | ∅ | `contacts add --fp` (no keys), or `contacts_device_add` (`:1165-1166`) | none possible |
| S-ABSENT | ∅ | ∅ | ∅ | no contact | — |

A signing key without the KEM key is refused (`contacts_sig_pk_requires_kem_pk`, `:1073`).
Re-provisioning is an atomic upsert (`contacts_entry_upsert`, `:1115`), so the store can
transition between these states BETWEEN protocol steps (modeled as adversary-scheduled repin
actions). **Store-coherence observation (proved as a model invariant, not assumed): in S-FULL
the stored `kem_pk` and `sig_fp` are exactly the components of the pair the pin fingerprints**
— enforced by the add-time check; no reachable provisioning writes an incoherent triple.
`sig_fp`-absent-with-pin-present states (S-KEM, S-BARE) ARE reachable — they are exactly the
states in which the OPTIONAL reverse pin skips, i.e. the P3 configuration.

### 1.3 Initiator (A) — INIT preconditions and A1 (`perform_handshake_init_with_route`, `handshake/mod.rs:1343-1468`)

Fail-closed preconditions, in code order:

- **I-1** pin present, else REJECT `identity_unknown` / `identity_pin_failed` (`:1351-1370`).
- **I-2** stored peer `kem_pk` present, else REJECT `peer_identity_key_missing` (`:1374-1384`)
  — S-BARE and S-ABSENT cannot initiate; a legacy S-KEM contact CAN.
- **I-3** `encap(pinned kem_pk) → (resp_kem_ct, resp_kem_ss)`, else `peer_identity_key_invalid`
  (`:1385-1395`).

Effects: sends `A1 = {sid, init.kem_pk, init.sig_pk, dh_pub, resp_kem_ct}` (the initiator's own
identity public keys travel in A1; `resp_kem_ct` is the C1 encapsulation to the PINNED responder
KEM key); stores `pending_init = {sid, kem_sk, resp_kem_ss, peer_fp := the pinned code read at
init time, …}` (`:1420-1440`).

### 1.4 Initiator (A) — ACCEPT rule at B1 (`handshake/mod.rs:1545-1794`)

All of the following, in code order, else deterministic labeled REJECT with no session commit:

- **A-1** `sid` equals `pending.sid` (`session_id_mismatch`); suite-context checks
  (`:1557-1576`).
- **A-2** `ss_pq = decap(pending.kem_sk, resp.kem_ct)` succeeds (`pq_decap_failed`, `:1579`).
- **A-3** `pq_init_ss = COMBINE(ss_pq, pending.resp_kem_ss)` (`:1593`) — mixes the
  responder-identity KEM secret from the init-time encapsulation to the PINNED key.
- **A-4** DH valid: non-zero encoding and contributory output (`dh_pub_invalid`,
  `dh_noncontributory`/`dh_failed`, `:1599-1621`).
- **A-5** transcript MAC: `resp.mac == MAC(pq_init_ss, A1 ‖ B1_no_auth)` (`bad_transcript`,
  `:1643-1656`). Because `pq_init_ss` mixes `resp_kem_ss`, a B1 producer that cannot
  decapsulate `resp_kem_ct` — i.e. does not hold the pinned KEM identity's secret — cannot
  pass. **This is the C1 KEM-possession binding of the responder.**
- **A-6** B1 signature verifies under `resp.sig_pk` — the key CARRIED IN B1; self-consistent
  for any key (`sig_invalid`, `:1657-1662`).
- **A-7** `pending.peer_fp` present (`identity_unknown`, `:1664-1676`).
- **A-8** PRIMARY pin re-check (`hs_require_primary_identity_pin`, `:978-1027`): the LIVE pin
  exists and matches `pending.peer_fp` — the code read at init (`identity_unknown` /
  `peer_mismatch` / `identity_pin_failed`). Guards pin stability across init→B1 (a mid-run
  repin to a different identity rejects here).
- **A-9** REQUIRED sig pin (`hs_require_sig_identity_pin`, `:1034-1079`, NA-0634 Decision 2a):
  the LIVE `sig_fp` EXISTS (else `responder_sig_unpinned` — a missing pin is a fail-closed
  reject) and `SIGFP(resp.sig_pk)` matches it (else `responder_sig_mismatch`). **Binds the
  responder's signing key to the pinned identity.** Consequence reproduced by the model: an
  S-KEM (legacy) contact can initiate but ALWAYS dead-ends here — "legacy contacts cannot
  complete a handshake" (NA-0634 as-built).

Accept effects: session committed with `authenticated=true` (`hs_build_session(true, …)`,
`:1703`; `qsp_session_store` `:1723`), pending cleared, A2 sent
`{sid, cmac = MAC(confirm_key(pq_init_ss, th), …), sig under own sig_sk over (sid, th, cmac)}`.

**Initiator accept therefore requires: (KEM possession of the pinned `kem_pk`'s secret by the
B1 producer) ∧ (`SIGFP(resp.sig_pk)` = pinned `sig_fp`) ∧ (pin present and stable).** In S-FULL,
store coherence makes both comparands components of the single verified code.

### 1.5 Responder (B) — A1 processing (`handshake/mod.rs:2014-2196`)

Not an accept — it produces B1 and pending state only; the durable commit is at A2 (§1.6).

- **R-1** decode ok; `init.dh_pub` non-zero (`dh_pub_invalid`, `:2016-2019`).
- **R-2** `peer_fp := CODE(init.kem_pk, init.sig_pk)` — the combined code RECOMPUTED FROM THE
  PRESENTED PAIR (`:2022`, NA-0634 Decision 2a).
- **R-3** PRIMARY pin: the LIVE pin exists and matches `peer_fp` (`identity_unknown` /
  `peer_mismatch`, `:2024-2028`). **Under code injectivity this binds BOTH presented
  components at once** — this is the redundancy ARGUMENT the model must test, not assume.
- **R-4** OPTIONAL reverse sig pin (`hs_check_optional_identity_pin`, `:1081-1121`):
  `peer_sig_fp := SIGFP(init.sig_pk)`; if a LIVE `sig_fp` exists it must match
  (`peer_mismatch`); **if absent the check SKIPS (`Ok(None) => Ok(())`, `:1111`) — exactly as
  landed. THE P3 SUBJECT.**
- **R-5** `encap(init.kem_pk) → (kem_ct, ss_pq)` (`pq_encap_failed`, `:2040`).
- **R-6** `resp_kem_ss = decap(own identity kem_sk, init.resp_kem_ct)`
  (`resp_kem_decap_failed` / `identity_missing`, `:2052-2068`) — only the true encapsulation
  target proceeds (C1, receiver half).
- **R-7** `pq_init_ss = COMBINE(ss_pq, resp_kem_ss)`; DH; the Suite-2 session is built but
  held ONLY in pending (`pending_session` bytes) together with
  `{peer_fp (from R-2), peer_sig_fp, peer_sig_pk := init.sig_pk, confirm_key, th}`
  (`:2148-2168`).
- **R-8** B1 sent `{sid, kem_ct, mac = MAC(pq_init_ss, A1 ‖ B1_no_auth), own sig_pk, sig, dh_pub}`.

### 1.6 Responder (B) — COMMIT rule at A2 (`handshake/mod.rs:1809-1996`)

All of the following, else deterministic labeled REJECT with no session commit:

- **C-1** `sid` equals `pending.sid` (`session_id_mismatch`); suite-context checks
  (`:1821-1840`).
- **C-2** confirm MAC: `confirm.mac == MAC(pending.confirm_key, sid, th)` (`bad_confirm`,
  `:1858-1873`). `confirm_key` derives from `pq_init_ss`, which mixes `ss_pq` — decapsulable
  only with the secret half of the A1-PRESENTED `init.kem_pk`. **KEM-possession binding of the
  initiator's presented KEM key.**
- **C-3** A2 signature verifies under `pending.peer_sig_pk` = the A1-presented `init.sig_pk`
  (`sig_invalid`, `:1874-1887`). Self-consistent (the key came from A1); **signature-possession
  binding of the presented `sig_id` to the A2 producer.**
- **C-4** `pending.peer_fp` / `peer_sig_fp` / `peer_sig_pk` present (`identity_missing`).
- **C-5** PRIMARY pin re-check: the LIVE pin exists and matches `pending.peer_fp` — the
  combined code of the A1-PRESENTED pair (`identity_unknown` / `peer_mismatch`, `:1920-1928`).
  A mid-run repin to a different identity rejects here.
- **C-6** OPTIONAL sig pin re-check against `pending.peer_sig_fp` — skip-on-None (`:1929-1937`).

Accept effects: pending session restored and committed (`qsp_session_store`, `:1938`),
pending cleared, `handshake_complete peer_confirmed=yes`.

**Responder accept therefore requires: (`CODE(presented pair)` = LIVE pin, checked at A1 AND
re-checked at A2) ∧ (KEM possession of the presented `kem_pk`'s secret, via C-2) ∧ (signature
possession of the presented `sig_pk`'s secret, via C-3) — with the reverse sig-pin contributing
ONLY when a `sig_fp` is stored (R-4/C-6 skip on None).**

### 1.7 Reject-hygiene facts (for P4)

Every reject above: emits a deterministic reason label (`handshake_reject` marker with a fixed
`reason` string); commits no session (`qsp_session_store` is reached only on the accept paths);
emits no success output (`handshake_complete` only on accept); leaks no secret (reject markers
carry labels and fingerprint DISPLAY forms only). Some rejects additionally CLEAR the pending
handshake (suite-context mismatch classes, replay: e.g. `:1568-1575`, `:1645-1647`,
`:1966-1990`) — an abort, modeled faithfully as pending-clear-on-reject; "durable state" for
P4 means committed sessions and the contact store, which no reject path mutates.

### 1.8 The P3 question, stated precisely over §1.5/§1.6

With R-4/C-6 exactly as landed (skip-on-None): does there exist a reachable responder COMMIT
(C-1…C-6 all pass) in which the A1-presented `init.sig_id` is NOT the sig component of the
pair whose code is pinned — i.e. a run that a present-and-required reverse sig-pin would have
rejected but the primary combined pin does not? The model SEARCHES the bounded space for this;
it does not argue it. Families the search must cover: `sig_fp`-absent stores (S-BARE with a
genuine combined code — the pure primary-pin-only configuration); legacy/cross-domain pins
(S-KEM: pin is `CODE1`, never equal to a combined code — expected unreachable-commit);
mid-run repins between A1 and A2; mix-and-match presented pairs `(b_kem, m_sig)` /
`(m_kem, b_sig)` including a kem-secret-compromise configuration (M holds `b_kem`'s secret);
replayed/substituted A1/A2 tokens across the two bounded sessions.

---

## §2 Model design — the P1–P4 encoding (Phase 2)

`formal/model_qsc_handshake_authentication_bounded.py`, registered by explicit import + call in
`formal/run_model_checks.py` `main()` (the house registration pattern — **no workflow change**;
`.github/workflows/formal.yml` just executes the runner, so the new model is exercised by the
existing REQUIRED `formal-ci` job on every PR and main push).

### 2.1 Abstraction choices (and what each one costs)

| Real thing | Model | Cost / assumption |
|---|---|---|
| `identity_fingerprint_from_identity(kem_pk,sig_pk)` | injective token `CODE(kem,sig)` | **collision-resistance ASSUMED**, not proved. Distinct pairs ⇒ distinct tokens, structurally. |
| `identity_fingerprint_from_pk(pk)` | `CODE1(kem)` — a **different token domain** | a legacy single-key code can never equal a combined code (models domain separation). |
| `hs_sig_fingerprint(sig_pk)` | `SIGFP(sig)` | injective on the signing key. |
| ML-KEM encap/decap | capability set: only the secret-holder derives `ss` | no computational hardness claim. |
| ML-DSA sign/verify | capability set: only the secret-holder signs | verification under a *presented* key is self-consistent for any key — the ENG-0038 lesson, preserved. |
| transcript / confirm MAC | derivable only with the KEM possession that feeds `pq_init_ss` | the C1 mechanism (`hs_pq_init_ss`) modeled as possession, not as a MAC. |
| contact store | `PinState` ∈ {FULL, KEM, BARE_COMBINED, BARE_SINGLE, ABSENT} | every state provisioning can reach (§1.2). |
| re-provisioning mid-handshake | `pin_at_a1` and `pin_at_a2` explored **independently** | the adversary may re-pin between messages. |
| adversary | party `M` holding its own secrets **plus any subset** of {AK,BK,AS,BS} | all 16 compromise subsets enumerated. |

### 2.2 The four properties, as encoded

- **P1 (mutual auth)** — `_check_p1_p4_responder` / `_check_p1_p2_p4_initiator`. For **every**
  accepting state (initiator-accept and responder-commit), assert the counterparty holds BOTH
  the KEM secret and the signing secret of the identity the pin authenticates, and that the
  bound pair is exactly the presented pair. An accept that binds only one half fails the model.
- **P2 (wrong-signing-key rejection)** — over the initiator exploration, every configuration in
  which the peer holds the *correct pinned KEM identity* but presents a *wrong signing identity*
  is asserted to REJECT, deterministically, with no commit and no success output. This is
  NA-0634's `hs_require_sig_identity_pin` (A-9) as a model property. **720 such configurations**;
  all reject.
- **P3 (THE OBLIGATION)** — `_check_p3`. Run the responder exploration with the reverse sig-pin
  **OPTIONAL exactly as landed** (skip-on-absent). For every reachable COMMIT, ask whether the
  initiator's presented `sig_id` is the one the live pinned code authenticates. A commit where it
  is **not** — and which a REQUIRED reverse pin **would** have rejected — is the counterexample
  that disproves redundancy. See §4.
- **P4 (fail-closed hygiene)** — every non-accepting outcome is asserted to carry a reason label
  from a **fixed declared set** (`REASONS`, mirroring the real `handshake_reject` markers),
  commit no session, emit no success output, and mutate no durable state. `_reject()` asserts the
  label is declared, so an undeclared reason is itself a model failure.

### 2.3 Two counterfactuals — why a "no counterexample" result is worth anything (WF-0017)

A negative claim is only as strong as the search's demonstrated ability to find a positive. The
model therefore carries two rewind-the-defences checks that **must** find the flaw:

1. **Faithfulness anchor** (`_check_eng0038_original_reproduction`). Rewind to PRE-C1 +
   PRE-NA-0634 (no responder-KEM possession binding; sig-pin optional with `sig_fp` structurally
   absent) and the model must reproduce the **real, known ENG-0038 flaw**. It does: **54
   impersonation traces**, the canonical witness being an adversary that has stolen **nothing**,
   signing B1 with a keypair it generated itself, making the initiator commit `authenticated=true`
   to a peer holding **neither** of Bob's identity secrets — verbatim the NA-0632 failure
   scenario. Under the **landed** rules the same search yields **0** such commits, and **0** for
   the sharper case of an adversary holding Bob's KEM identity secret but not his signing secret.
   *A model that cannot express the real flaw cannot be trusted to certify its absence.*
2. **P3 non-vacuity** (`_check_p3_counterfactual_nonvacuous`). Rewind **only** the primary pin to
   its pre-NA-0634 KEM-only form (`CODE1`) and the P3 search must surface an unbound-signing-key
   commit. It does: **128**, the reported witness being M — holding only Alice's stolen *KEM*
   secret — presenting Alice's KEM key with **its own** signing key, and the responder committing;
   a REQUIRED reverse pin would have rejected it (`responder_sig_unpinned`).

**Consequence for the verdict's honesty:** the redundancy of the reverse pin is **contingent** —
it holds *because and only because* the primary pin's code covers the signing key injectively.
That coverage is exactly what NA-0634 bought. The model makes the dependency machine-visible
instead of burying it in prose.

---

## §3 Model run results (Phase 2/4)

Full suite, from the repo root — `python3 formal/run_model_checks.py` → **exit 0**, all five
bounded models green (SCKA, Suite-2 negotiation, qsc suite-id, NA-0478 binding, NA-0626 root
composition, plus this one).

**The bound (exhaustively enumerated — no reduction was needed, so this is a PASS, not a
bounded-PARTIAL):** 3 identities (2 honest + adversary), 3 KEM keys × 3 signing keys, 5 contact-pin
states explored independently at A1 and A2 (mid-run re-pin), 3 message producers, all 16 adversary
compromise subsets of the four honest identity secrets, 3 reverse-pin modes.
⇒ **10,800 responder configurations and 10,800 initiator configurations, fully enumerated.**
Termination is structural (finite product, no fixpoint) — the ENG-0035 constraint is honored by
construction; no ProVerif, no unrolling.

```
QSC.HS auth responder configs explored: 10800   (LANDED: 80 commits / 10720 rejects)
QSC.HS auth initiator configs explored: 10800   (60 commits / 10740 rejects)
P1 responder full-identity bindings: 80         P1 initiator full-identity bindings: 60
P2 wrong-signing-key rejects: 720
P4 reject-hygiene assertions: 10720 (responder) + 10740 (initiator)
P3 LANDED responder commits judged: 80
P3 unbound-sig commits (gap size): 0
P3 reverse-pin redundant: True
P3 bound-yet-REQUIRED-would-reject (false-reject-only, not a gap): 60
P3 non-vacuity: pre-NA-0634 counterfactual unbound commits detected: 128
ENG-0038 faithfulness anchor — pre-fix impersonation traces reproduced: 54
ENG-0038 under LANDED rules — impersonation commits: 0; KEM-only-compromise commits: 0
```

**Fail-closed proof of the gate itself.** The model's P3 machinery was re-run against the PRE-FIX
rule set: it returns `redundant=False` with 128 unbound commits, and the `emit_*` guard
(`assert p3.redundant`) raises `QSC_HS_HANDSHAKE_AUTH_MODEL_GAP_FOUND` → the runner exits
non-zero. **A P3 disproof turns the formal CI check RED; it cannot silently pass.**

---

## §4 THE OBLIGATION VERDICT (Phase 3)

### Verdict: the reverse (responder→initiator) sig-pin is **REDUNDANT**. Obligation DISCHARGED affirmatively.

Across the entire bounded space — every reachable contact-pin state including the `sig_fp`-absent
ones where the optional reverse pin **skips**, every mid-run re-pin, every adversary compromise
subset, every presented key pair — **there is no reachable responder-commit in which the
initiator's presented signing key escapes binding to the single verified code.** 80 commits were
judged; 0 were unbound. There is no run the reverse pin would have caught that the primary
combined pin does not.

**Why it holds (the mechanism the search confirms, stated so it can be falsified later):** the
responder's PRIMARY pin is REQUIRED on every path to a commit and recomputes
`CODE(init.kem_pk, init.sig_pk)` from the *presented* pair. Because the combined code is injective
on the pair, code equality forces the presented signing key to be the pinned one — so the separate
`SIGFP` comparison adds no discrimination. Possession is then forced independently by C-2 (confirm
MAC ⇒ KEM secret) and C-3 (A2 signature ⇒ signing secret).

**The verdict's load-bearing assumption, stated plainly:** it depends on the combined code
actually covering the signing key injectively (§2.3). Rewind that one property and the model
immediately finds 128 unbound commits. **If the verification-code format is ever narrowed back to
the KEM half, the reverse pin becomes load-bearing again and this discharge is void.** That is a
regression-guard obligation on the code format, not on the pin.

### Two secondary results the operator should have

1. **Making the reverse pin REQUIRED would buy zero security and cost a working provisioning
   path.** 60 configurations are commits that are correctly bound *and* that a required reverse
   pin would reject (`responder_sig_unpinned`). They are exactly the **S-BARE** contacts —
   provisioned with `contacts add --fp <code>` (code only, no keys), which populates no `sig_fp`.
   Such a contact cannot *initiate* (it has no stored `kem_pk` — fail-closed
   `peer_identity_key_missing`) but it can *respond*, and its handshakes are correctly bound today.
   This answers the open question NA-0634 recorded ("a strict reading of 'retire the whole
   asymmetry class' might want it required too"): **the model says don't** — it converts sound
   handshakes into rejects and catches nothing.
2. **The NA-0633/NA-0634 fix chain is confirmed closed, in model form.** An adversary holding the
   responder's KEM identity secret but *not* its signing secret reaches **zero** initiator-accepts
   (the NA-0634 half); an adversary holding neither reaches zero (the C1 half). The same search
   with those defences rewound reproduces 54 impersonations.

### Claim boundary — UNCHANGED

A PASS here substantiates a **bounded authentication-binding property over an abstract state
machine**. It is **not** cryptographic security, **not** a side-channel property, **not** a
post-compromise or PQ guarantee, and **not** a qsc/refimpl equivalence claim. No claim moves.
Fingerprint collision-resistance is assumed, not proved. Independent external review remains an
open prerequisite for any public security claim.

### Result classification

**`QSC_HS_HANDSHAKE_AUTH_MODEL_PASS`** — P1, P2, P4 proven over the stated bound; P3 resolved
AFFIRMATIVELY (reverse pin redundant); the ENG-0038 obligation is DISCHARGED; the formal suite is
green. Not PARTIAL: the stated bound was enumerated exhaustively with no reduction.
