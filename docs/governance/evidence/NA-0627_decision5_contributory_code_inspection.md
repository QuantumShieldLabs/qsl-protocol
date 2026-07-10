# NA-0627 — Decision-5 discharge: the X25519 contributory / low-order question, answered OUTSIDE the symbolic model

Goals: G1, G2, G4

Lane: NA-0627 (ENG-0028). Directive: QSL-DIR-2026-07-09-564 (D564), Decision 5 **re-presented at
design-lock and ratified by the operator 2026-07-09**: discharge **(c) is BINDING** — answer by
code inspection against RFC 7748 and FILE AN ENG ITEM with evidence + severity; **(a) optional**
(guard-form query, labelled as NOT an attack-existence proof); **(b) FORBIDDEN** (extending the
symbolic DH theory — rejected by probe evidence: non-termination); **(d) Tamarin: not taken**.
Base: main `d9baed9d`. Recorded 2026-07-09 (UTC). **Analysis lane: this is a FILING, not a fix.**

---

## 1. Why this question cannot be answered inside the model (restated from the design-lock)

Probe evidence, proof root `dh_theory_finding.txt` / `dh_theory_probe.out`:

- baseline DH theory (`exp(exp(g,x),y) = exp(exp(g,y),x)`) + a trivial process terminates
  instantly (`is true.`);
- the same theory **plus a degenerate element** (`const zeropoint: G. equation forall x;
  exp(zeropoint,x) = zeropoint`) makes ProVerif **diverge in equational completion on
  `process 0`** (timeout at 45 s and again at 120 s). The divergence is caused by the THEORY,
  not by any protocol.

The deeper point, which is the one that binds: **standard symbolic DH cannot express
small-subgroup / low-order-point attacks at all.** The equational theory *idealizes* the group;
the attack lives in the concrete group's structure. A model on the standard theory returns
"secure" **regardless of whether the implementation screens low-order keys** — a vacuous green,
and precisely the failure the Decision-3 abstraction table exists to prevent. Reporting such a
result as "the contributory question is answered" would be an overclaim.

Recorded as abstraction **A4** in the design-lock table: *X25519 as an idealized symbolic group;
MASKS small-subgroup / low-order points, non-contributory shared secrets, and therefore the
presence or absence of a contributory check in the implementation.*

**What the model DOES contribute (option (a), the guard-form):** query Q7,
`not event(BoundaryAccepted(zeroG))`, is proved `is true.` on both root-advancing receive arms
(`suite2_dhpq_main.pv` for the DH boundary; `suite2_dhpq_q4_combined_healing.pv` for the combined
boundary). `zeroG` is an **algebra-free constant** — it carries no equation, exactly because (b)
is forbidden. Q7 therefore says only: *the modeled accept path mirrors the shipped
`is_zero32(DH_pub)` reject, so the guarantees below are stated relative to "the peer's DH public
key is not the all-zero encoding."* **Q7 IS NOT AN ATTACK-EXISTENCE PROOF and must never be
reported as one.**

## 2. The shipped code, read against RFC 7748

**Primitive.** `x25519-dalek` **2.0.1** (`Cargo.lock:3560`), feature `static_secrets`
(`tools/refimpl/quantumshield_refimpl/Cargo.toml:21`). The single DH entry point is
`StdCrypto::dh` (`src/crypto/stdcrypto.rs:177-182`):

```rust
fn dh(&self, privk: &X25519Priv, pubk: &X25519Pub) -> [u8; 32] {
    let sk = StaticSecret::from(privk.0);
    let pk = PublicKey::from(pubk.0);
    (sk.diffie_hellman(&pk)).to_bytes()          // -> SharedSecret::to_bytes()
}
```

`diffie_hellman` returns a `SharedSecret` that exposes **`was_contributory()`**. The repo never
calls it: `grep -rn "was_contributory\|non_contributory\|low_order\|small_subgroup\|contributory"
--include=*.rs .` returns **zero matches in any source file** (only prose hits, §4 below).
The `[u8; 32]` return type discards the flag at the trait boundary — `X25519Dh::dh`
(`src/crypto/traits.rs:36`) has nowhere to report non-contributory behaviour even if a caller
wanted to check.

**RFC 7748 §6.1 (and §7, Security Considerations):** X25519 is deliberately *non-contributory* —
the function accepts low-order points and returns an all-zero shared secret rather than erroring.
The RFC states that *"protocols that require contributory behaviour ... MUST check for the
all-zero value"* on the DH output. **No such check exists anywhere in this repo.**

**What IS checked, and what is not.** Every `is_zero32` call site
(`grep -n is_zero32 src/suite2/ratchet.rs`) guards a *stored key or chain key*, or the *peer's
public key encoding*, never the DH **output**:

| Site | Guard | Covers |
|---|---|---|
| `ratchet.rs:1420` `recv_dh_boundary` | `is_zero32(&parsed.dh_pub)` | rejects only the **all-zero encoding** of the peer public key |
| `ratchet.rs:2317` `recv_combined_boundary` | `is_zero32(&parsed.dh_pub)` | ditto |
| `ratchet.rs:1294` `send_boundary` | `is_zero32(&st.dh.dhr)` | own stored peer key unset |
| `ratchet.rs:1416`, `:1720`, `:1613`, `:300`, … | roots / chain keys unset | fail-closed on unset material |
| **nowhere** | `is_zero32(&dh_out)` | **the DH OUTPUT is never checked** |

The all-zero encoding is *one* of the low-order points. Curve25519 has **eight** points of small
order; their canonical encodings (RFC 7748 / the classical list) include `0x00…00`, `0x01…00`,
the two of order 8, `p-1`, `p`, `p+1`, and the non-canonical high-bit variants. `is_zero32`
rejects exactly one of them. A peer supplying any *other* low-order encoding passes the check and
drives `dh_out = [0u8; 32]`.

**Where a non-contributory point can enter:**

1. **Suite-2 ratchet boundaries** — `recv_dh_boundary` (`ratchet.rs:1475`) and
   `recv_combined_boundary` (`ratchet.rs:2390`) both compute
   `dh_out = dh(dhs_priv, parsed.dh_pub)` from a **peer-supplied** `DH_pub`, then
   `KDF_RK_DH(RK, dh_out)`. With a low-order `DH_pub`, `dh_out` is all-zero and the new root is
   `RK' = KMAC(RK, "QSP5.0/RKDH", 0…0)` — a **deterministic function of the pre-boundary root**.
   The DH ratchet contributes **no fresh entropy** for that epoch.
2. **QSP base handshake** — `qsp/handshake.rs:134` `dh1 = dh(ek_priv, bundle_b.spk_dh_pub)` and
   `:144` `dh2 = dh(ek_priv, opk_dh_pub)`, both from B's **prekey bundle**. `verify_bundle`
   (`:100`) verifies identity-key signatures/KT; it does not screen the X25519 prekeys for small
   order. A bundle serving a low-order `spk_dh_pub`/`opk_dh_pub` collapses the classical half of
   `RK0` to a constant. The ML-KEM contributions `ss1`/`ss2` still bind, so establishment stays
   hybrid-protected — but its classical half can be nullified **silently**.

## 3. Exposure analysis (what an attacker actually gets) — the basis for the severity

**Not reachable by a network (Dolev-Yao) adversary against an honest pair.** A boundary frame's
header is AEAD-sealed under `NHK_r` derived from the current root, and `DH_pub` is bound into
`ad_hdr` (`binding::ad_hdr`). An off-path attacker cannot inject a low-order `DH_pub` without the
root. This is exactly the envelope this lane's Q1/Q2 proved (`is true.`), and it is why the
finding is **not** a STOP against the shipped composition: no modeled query is disproved.

**Reachable by the authenticated peer (or malware steering that peer's key selection).** A peer
that ratchets with a low-order point forces every subsequent root on the honest party to be a
deterministic function of the pre-boundary root. Consequences:

- **The classical half of post-compromise security is voidable by the peer.** An attacker who
  once learned `RK` and can influence the peer's boundary key stays synchronized across every
  DH boundary it forces to be non-contributory. This lane's **Q5** (classical healing across a DH
  boundary) is proved `is true.` *only because the model's honest sender always contributes a
  fresh exponent* — the property Q5 establishes is precisely the one a low-order point removes.
  **The PQ half still heals** (Q3/Q4 hold and do not depend on the DH share), so the hybrid does
  not collapse; the composition degrades to "PQ-only healing" without any signal to either party.
- **Silent.** Nothing rejects, logs, or reason-codes. Both parties converge on the same root and
  the session proceeds normally, so no conformance vector or runtime check observes it.

**Bounding the claim honestly:** an authenticated peer can already read the plaintext it is sent.
The harm is not confidentiality against that peer; it is the **loss of the classical ratchet's
contributory guarantee**, which the project's standing Triple-Ratchet / post-compromise language
would otherwise rest on. That is why this belongs in the ENG ledger before, not after, any claim
moves — and it is directly relevant to Operator Decision 4 (claim boundary), where the
recommendation stays **UNCHANGED**.

## 4. Prior art in-repo (this was surfaced before, and never filed)

`docs/audit/incoming/2026-04-09_security_batch/QuantumShieldLabs _ qsl-protocol — Security Audit.md:138`
states it for the QSP-4.3-era code, verbatim: *"`x25519-dalek 2.x` with `StaticSecret` does not
perform a low-order-point check … No all-zeros DH output check is present … Risk is low because
the KDF absorbs it, but a small-subgroup attack against X25519 is worth guarding against."*

That audit line was never converted into a tracked ledger item, and the D564 status sync
re-surfaced it independently. **ENG-0034 (below) closes that gap.** The audit's "risk is low"
is consistent with §3: the KDF does absorb the zero, so no key becomes zero — what is lost is
contributory freshness, not key secrecy. The audit did not analyze the post-compromise
consequence; §3 does.

## 5. The filing (ENG-0034), verbatim as entered in `docs/ops/IMPROVEMENT_LEDGER.md`

- **Severity: P2** (security-relevant correctness gap; not remotely exploitable against an honest
  pair, but it silently voids the classical half of post-compromise security and blocks the
  Triple-Ratchet/PCS claim language).
- **Recommended change:** reject non-contributory DH. Either (a) surface `was_contributory()`
  through `X25519Dh::dh` (return `Option<[u8;32]>` / `Result`) and fail closed at all four DH
  call sites, or (b) keep the trait shape and add an `is_zero32(&dh_out)` fail-closed check
  immediately after each `dh()` call (`ratchet.rs:1306`, `:1475`, `:1885`, `:2390`;
  `qsp/handshake.rs:134`, `:144`, `:285`, `:297`), plus a small-order screen on `DH_pub`
  ingress. (b) is the smaller diff; (a) is the one that cannot be forgotten at a new call site.
  Both need a new reason code (`REJECT_S2_DH_NONCONTRIBUTORY`) and negative vectors.
- **Not fixed in this lane** (D564: analysis lane; "the FIX, if warranted, stays out of scope").

## 6. Verification trail

```
$ grep -rn "was_contributory\|non_contributory\|low_order\|small_subgroup" --include=*.rs .
(no matches)
$ grep -n 'name = "x25519-dalek"' -A 2 Cargo.lock
3560:name = "x25519-dalek"
3561-version = "2.0.1"
$ grep -n "is_zero32" tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs
52,300,356,557,790,1008,1294,1416,1420,1607,1613,1716,1720,1871,1874,2102,2317,2322
   (1420 / 2317 = peer DH_pub encoding; none is the DH OUTPUT)
$ grep -n "dh\.dh(" tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs
1306 (send_boundary)  1475 (recv_dh_boundary)
1885 (send_combined_boundary)  2390 (recv_combined_boundary)
   -> four DH outputs, zero post-hoc checks
```

Q7 raw results (guard-form, both arms): `main_v2_reduced.out`, `q4_combined_healing.out` in the
proof root — `RESULT not event(BoundaryAccepted(zeroG)) is true.` **Not an attack-existence
proof.**
