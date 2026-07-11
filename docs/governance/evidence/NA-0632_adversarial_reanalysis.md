# NA-0632 — Internal adversarial re-analysis of the Suite-2 cryptographic core

Goals: G1, G2, G4

Directive: QSL-DIR-2026-07-11-569 (D569, APPROVED). Base: `main == 80698e82` (≥ `99ee097b`).
Class: ANALYSIS (single PR, findings FILED not fixed). Scope (Operator Decision 1): the shipped
`qsc` Suite-2 path + the refimpl Suite-2 it calls (qsp legacy skeleton + attachments out of scope).

## 0. Binding framing (do not weaken)

This is **INTERNAL scrutiny, NOT the independent external review.** It cannot certify the absence of
flaws: a fresh copy of the same model shares the deep blind spots of the sessions that built and
verified this code. A clean result here means "this pass found nothing," never "it is secure." **No
claim is moved by this lane.** The independent human/third-party review remains the true release gate.

The standing conclusion (NA-0619..0631, memory index) that the crypto core is "correctness-complete
with no known gap" was treated as the **hypothesis under test**, re-derived from code + DOC-CAN-003/004,
and each prior "verified / no-mutation / covered" assertion re-checked against current code.

## 1. Headline result

**One significant FINDING (F1), rated P1.** It is NOT in the Suite-2 ratchet/SCKA math — that core held
up under the walk (see §3 CHECKED-OK). F1 is in the **shipped establishment/identity layer** (`qsc`'s
`QSC.HS.*` handshake), which DOC-CAN-003 §6.3/§0.2 REQUIRE to "authenticate peer identity before Suite-2
state is committed." On the shipped path that precondition does not hold for the **initiator's**
authentication of the **responder**: an active on-path attacker (e.g., the relay) can impersonate the
responder to the initiator, and the out-of-band verification code a user checks does not prevent it.

F1 directly contradicts the prior conclusion recorded under **ENG-0001 / NA-0609B** — "the
verification-fingerprint model is COHERENT … there is no KEM-vs-SIG binding flaw" — which is exactly the
kind of prior "verified" claim D569 mandated re-testing. It does not hold on current code for the
initiator→responder direction.

## 2. FINDING F1 (P1) — Asymmetric handshake authentication: the responder is not authenticated to the initiator

Filed as **ENG-0038**. Active-MITM, remote-reachable (the relay is on-path), deterministic.

### 2.1 The defect — each step a verified code fact

1. **The responder's only identity credential in B1 is its ML-DSA signing key.** `HsResp` =
   `{session_id, kem_ct, mac, sig_pk, sig, dh_pub}` — it carries **no KEM public key of the responder**.
   The responder encapsulates to the *initiator's* public `kem_pk` and sends only the ciphertext.
   `qsl/qsl-client/qsc/src/handshake/mod.rs:138-148` (struct), `:1885` (`c.encap(&init.kem_pk)`),
   `:1938-1944` (`hs_encode_resp_no_auth` — kem_ct + sig_pk + dh_pub, no kem_pk).
2. **The initiator verifies the responder's signature under the key the responder SENT** (`resp.sig_pk`)
   — self-consistent for *any* key. `handshake/mod.rs:1509` `hs_sig_verify(&resp.sig_pk, &sig_msg, &resp.sig, …)`.
3. **The responder's signing key is pinned only via the OPTIONAL `sig_fp`, which is structurally always
   `None` on the shipped path:**
   - `contacts_add` sets `sig_fp: None` even when `verify=true` (contact marked "verified"):
     `qsl/qsl-client/qsc/src/contacts/mod.rs:1047,1053`.
   - `contacts_device_add` sets `sig_fp: None`: `contacts/mod.rs:1110`.
   - No code path writes a *learned* peer `sig_fp` back into a contact after a handshake (exhaustive grep;
     the only `hs_sig_fingerprint` results are the transient pending-handshake + the optional check).
   - ⇒ `identity_read_sig_pin(peer)` always returns `None` ⇒ `hs_check_optional_identity_pin` takes its
     `Ok(None) => Ok(())` branch and checks nothing: `handshake/mod.rs:1532`, `identity/mod.rs:634-641`,
     `handshake/mod.rs:1001`.
4. **The initiator's REQUIRED "primary" pin is the KEM fingerprint `fp`, but it is inert for this
   direction:** the responder's KEM key is never sent/used in the B→A direction (step 1), and the check
   is **tautological** — `pending.peer_fp` is set to `identity_read_pin(peer)` at initiate
   (`handshake/mod.rs:1241,1295`) and re-compared to `identity_read_pin(peer)` at B1 processing (`:1527`).
   It compares the initiator's own pre-handshake pin to itself; it validates nothing in B1.
5. ⇒ The initiator commits a Suite-2 session with `authenticated=true` (`handshake/mod.rs:1550-1551`)
   for **any** responder signing key.

Contrast — the **responder→initiator** direction IS sound: the responder recomputes the initiator's KEM
fingerprint from `init.kem_pk` and checks it against a required pin (`handshake/mod.rs:1867-1872`), the
KEM key is *used* (encaps), and the initiator proves possession of the KEM secret via the A2 confirm MAC.
An MITM cannot impersonate the initiator to the responder (it lacks the initiator's KEM secret and cannot
substitute the KEM key without failing the responder's fingerprint pin). The gap is one-directional.

### 2.2 Concrete failure scenario

On-path attacker M (a malicious or compromised relay — squarely the product's self-hosted-relay threat
model puts the relay on path) intercepts the initiator A's `A1`. A's `kem_pk` is public in A1, so M:
encapsulates to it → `(kem_ct, ss_pq)`; derives `pq_init_ss`, the transcript MAC and `th`; generates its
OWN ML-DSA keypair; signs `SIG.B1 = "QSC.HS.SIG.B1" || session_id || th`; sends
`B1 = {kem_ct, mac, M.sig_pk, M.sig, M.dh_pub}`. A accepts: decap→`ss_pq` ✓, transcript MAC ✓, signature
verifies under `M.sig_pk` ✓, primary KEM pin tautologically ✓, optional sig pin skipped (`None`). A
commits a session with M as "B". Every message A sends to "B" is readable by M. A's out-of-band
verification of B's code gave no protection. This is not first-contact-only: because `sig_fp` is never
populated, it recurs for every A-initiated session.

### 2.3 Why the existing tests/analysis did not catch it (proof gap)

- `qsl/qsl-client/qsc/src/adversarial/binding_fuzz.rs` exercises only frame *decoding* + the pin-string
  comparison primitive (`classify_stale_public_record` asserts a mismatched pin rejects). It never drives
  end-to-end whether the initiator authenticates the responder's *signing* key.
- The one test that exercises the sig-pin path,
  `tests/kem_signature_transcript_binding_negative.rs::signature_wrong_identity_…`, must **hand-inject**
  `sig_fp` via a JSON helper `set_contact_sig_pin` (`:308-331`, `:683`) because no product path sets it,
  and it only tests a *wrong-pinned* value → reject. There is no test for the shipped default (`sig_fp`
  unset) presenting a substituted responder `sig_pk`. `seed_authenticated_pair` ("authenticated"
  contacts) does not set `sig_fp` either — which is exactly why the injector exists.
- The ProVerif model (DOC-G4-002) covers the **Suite-2 ratchet composition**, not the `qsc` handshake
  authentication — so this layer is UNMODELED; a symbolic model of `QSC.HS.*` would decide it.

### 2.4 Honest caveat

This is an internal code-trace, corroborated by the test infrastructure above; it is **not** demonstrated
by a running PoC in this lane (a lane rule: fix/PoC-drive nothing that mutates product code; the PoC is a
successor's job — see the testplan). It should be independently confirmed. It sits in the identity/KT
layer that DOC-CAN-003 §0.2 treats as an assumed abstraction — but §6.3 makes authenticated peer identity
a *precondition* the Suite-2 core rests on, and this is the SHIPPED handshake a GUI would build on, so it
is squarely a before-GUI issue. (Shape mirrors ENG-0019 — `authenticated=true` asserted while the actual
authentication is absent — but here on the shipped `qsc` path, not the `qsp` reference actor.)

## 3. Coverage walk — CHECKED-OK (could not break; assumption logged)

The refimpl Suite-2 core (`tools/refimpl/quantumshield_refimpl/src/suite2/*.rs`) was read in full
(non-test) and walked against the D569 attack classes. Each item below held.

- **KDF / key schedule (DOC-CAN-003 §3).** Every label and construction matches byte-for-byte:
  `kdf_rk_dh` (RKDH→64B split), `header_key` (HK/NHK A→B/B→A), `kdf_rk_pq` (RKPQ), `derive_mk_step`
  (CK/MK/PQCK/PQMK, then HYBRID = `KMAC(ec_mk, "QSP5.0/HYBRID", pq_mk||01)` — always-hybrid holds even if
  one input is compromised), `kdf_pq_reseed_seeds` (SCKA/CTXT ctx + PQSEED A→B/B→A). `ratchet.rs:60-138,
  295-315`; `scka.rs:37-56`. Establishment RK0/RKPQ match §8.2 (`establish.rs:54-73`).
- **Nonce derivation (§5.2).** `nonce_hdr`/`nonce_body` = `SHA512(label || session_id || dh_pub ||
  u32be(N))[..12]`. `(dh_pub, N)` is globally unique per direction: `N` is strictly monotonic and
  overflow-checked (never saturates) within a fixed `dh_pub`, and `dh_pub` changes only at a DH boundary
  that resets `N`; HK≠NHK separates boundary vs non-boundary headers. No `(key, nonce)` collision found.
  `ratchet.rs:269-291`. **Assumption:** `dh.keypair()` yields unique public keys (RNG) and AES-GCM is used
  within its nonce space.
- **Non-contributory X25519 (RFC 7748 §6.1 / ENG-0034).** `is_zero32(dh_out)` reject on **every** live DH
  site: `send_boundary:1310`, `recv_dh_boundary:1485`, `send_combined_boundary:1900`,
  `recv_combined_boundary:2411`, plus the handshake `hs_dh_shared:819`. The comment correctly notes the
  zero-`dh_pub` screen catches only 1 of 8 low-order encodings and relies on the `dh_out` check for the rest.
- **Counter overflow (§10).** `checked_counter_inc` (no saturation) guards every `ns`/`nr` advance; a
  frozen counter under static header keys (nonce reuse) cannot occur. `ratchet.rs:30-32` + all send/recv.
- **No-state-mutation-on-reject (§7/§9.2).** Every reject path returns the *input* state unchanged
  (verified across `recv_nonboundary_ooo`, `recv_boundary_in_order`, `recv_dh_boundary`, `recv_pq_adv`,
  `recv_pq_reseed`, `recv_combined_boundary`, `apply_pq_reseed`, and the send functions).
- **Boundary anti-spoof (§8.5.1).** Epoch-transition headers open under CURRENT `NHK_r` only (NHK-only,
  no fallback), at `n==0` (DH/combined) or `n==nr` (PQ-only): `recv_dh_boundary:1440-1478`,
  `recv_combined_boundary:2362-2404`, `recv_boundary_in_order:729-788`. ADV stays under `HK` (§8.5.4).
- **SCKA control plane.** ADV is authenticated (ADVAUTH MAC, constant-time `ct_eq32`) —
  `recv_pq_adv:2140-2146`; a planted ADV is rejected, never tracked. Reseed is one-time + monotonic:
  `apply_pq_reseed` marks the target consumed AND tombstoned on commit and rejects tombstoned/consumed/
  unknown/non-monotonic targets (`scka.rs:81-107`).
- **Combined DH+PQ boundary (ENG-0026).** Sender (`send_combined_boundary:1896-1917`) and receiver
  (`recv_combined_boundary:2406-2483`) use the identical DH-first-then-PQ ordering from the pre-boundary
  root; `pq0_send_label`/`pq0_recv_label` pair correctly (A's send-label = B's recv-label), so both
  parties converge. The combined/PQ-only discriminator (`parsed.dh_pub != st.dh.dhr`) is safe because
  `DH_pub` is bound in the header AD — a tampered discriminator fails header AEAD either way (`:2261`).
- **Parsing / cross-frame-kind confusion.** `parse.rs` rejects unknown flag bits, enforces
  ADV/CTXT⇒BOUNDARY, exact ML-KEM lengths (1184/1088), and no trailing bytes. `qsc` routes by
  *unauthenticated* flags but every handler re-derives the AD from those flags (`ad_hdr` binds `u16(flags)`
  + `pq_bind`), so a routing-flag flip fails the header AEAD → reject, no mutation. Routing
  `main.rs:2554-2560`.
- **Primitive usage (not internals).** Real backends, correct sizes, fail-closed: AES-256-GCM with `open`
  returning `Err(AuthFail)` on tag/AD/nonce mismatch and a caught seal-failure; KMAC-256
  (`KeccakKmac::v256(key, label)`); SHA-512; `ml_kem::MlKem768`; `ml_dsa::MlDsa65`; `x25519_dalek`. The
  `pqcrypto` feature qsc enables pulls in the real ML-KEM + ML-DSA. `crypto/stdcrypto.rs:40-174`.
- **Snapshot/restore.** v3-only, fail-closed (bounded lengths, exact-consume, distinct version marker).
  `state.rs:120-281`.
- **No silent TOFU.** An unpinned peer is rejected `identity_unknown` (no auto-pin);
  `hs_require_primary_identity_pin` fails closed on `Ok(None)`. `handshake/mod.rs:950-968,1241-1251`.
- **Handshake non-crypto establishment.** Transcript MAC + `th` bind A1||B1 (incl. both `dh_pub`, KEM
  material); signatures cover `session_id || th`; A2 confirm MAC proves the initiator's KEM-secret
  possession. Sound for the responder→initiator direction (see §2.1 contrast).

## 4. CAN'T-TELL (outside what code-reading decides — the tool that would decide)

- **`qsc` handshake authentication as a symbolic model.** F1 was resolved to a FINDING by manual trace,
  but the *positive* properties of `QSC.HS.*` (mutual authentication, KCI/UKS resistance, transcript
  agreement) are UNMODELED — DOC-G4-002's ProVerif covers the ratchet composition, not this handshake.
  Decider: a ProVerif/Tamarin model of `QSC.HS.*` (a natural extension of the ENG-0035 formal track).
- **Nonce-uniqueness under adversarial scheduling across many reseeds/boundaries.** The manual argument
  (§3) is an invariant proof, not exhaustive. Decider: the `formal/` state explorer extended to the
  combined-boundary + interleaved-reseed shapes, or a differential fuzzer over long randomized transcripts.
- **Timing / side channels of the shipped path.** Out of scope for code-reading (see §5). Decider:
  `dudect`-style timing measurement on the real binary.

## 5. Blind-spot honesty — what this pass could NOT cover

- **The abstraction ceiling (DOC-G4-002 §2, A1–A8).** Findings here live *below* the A1–A8 idealizations
  (perfect AEAD/KEM/DH, honest RNG, secure primitives). This pass does not revisit those; if an
  abstraction is wrong, this analysis would not see it.
- **Primitive internals.** AES-GCM / ML-KEM-768 / ML-DSA-65 / X25519 / KMAC *implementations* (upstream
  crates) were treated as correct. Only their *usage* was audited.
- **Side channels & timing.** Constant-timeness beyond the two explicit `ct_eq` compares (AEAD tag
  handling, KMAC, ML-KEM decap, the header trial-decrypt search order) was not measured. Code-reading
  cannot decide timing leaks.
- **RNG quality.** `OsRng` is assumed sound; a biased/broken RNG (session_id, DH/KEM keygen, ML-DSA seed)
  would break guarantees this pass takes for granted.
- **Concurrency / persistence at the OS layer.** Crash-atomicity of the vault/secret store writes, TOCTOU
  on the config dir, and filesystem durability were reasoned about (the session-vs-SCKA ordering is
  sound, §3) but not stress-tested.
- **Same-model blind spots.** This executor shares the training and reasoning style of the sessions that
  wrote and "verified" this code. A clean bill from this pass is evidence of *nothing found*, not of
  security. **The independent external review remains the true release gate.**

## 6. Before-GUI triage

| # | Finding | Sev | Fix before GUI? | Rationale |
|---|---------|-----|-----------------|-----------|
| F1 | Handshake responder-auth asymmetry (ENG-0038) | P1 | **YES** | It is the shipped establishment layer a GUI ships on; it defeats the out-of-band verification the product's security model relies on; the self-hosted-relay niche puts the attacker on-path. Fixing it later means shipping a GUI over a known active-MITM path. The mechanism to fix it largely EXISTS (the `sig_fp` pin path) and needs wiring, plus binding the responder's identity into the B→A direction. |

No other FINDINGs. "Found nothing else" is **not** "proven safe" (see §0, §5).

## 7. Successor triage (Phase 7 / WF-0003 — PROPOSED, not promoted; operator triages)

- **Fix lane for ENG-0038 (before the GUI).** Options for the operator to choose among (design-lock-first):
  (a) wire `sig_fp` into contact provisioning so the responder's signing key is pinned and the optional
  check becomes effective (make it *required* once the identity model expects it); AND/OR (b) bind the
  responder's identity into the B→A direction cryptographically (e.g., carry + pin the responder's
  identity KEM key, or have the responder's signing key be certified by the pinned identity), so the
  out-of-band code authenticates *both* directions; (c) make the initiator's primary pin non-tautological.
- **A ProVerif/Tamarin model of `QSC.HS.*`** (extends the ENG-0035 formal track) to decide the §4
  CAN'T-TELLs about the handshake.
- **The independent external human review** — unchanged, still the true release gate.

## 8. Framing reminder

Internal scrutiny only. No claim moved. A confirmed-by-trace finding (F1) is surfaced and FILED
(ENG-0038); it is NOT fixed in this lane (the analysis-lane rule). "This pass found one issue and
otherwise found nothing" ≠ "the core is secure."
