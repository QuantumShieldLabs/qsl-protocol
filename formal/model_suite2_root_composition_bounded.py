"""Bounded executable model of the Suite-2 ROOT COMPOSITION layer (NA-0626 / ENG-0024+0026).

Goals: G4

Where `model_scka_bounded` covers SCKA *logic* invariants (ADV monotonicity, one-time CTXT
targeting, reject => no mutation), this module models the layer underneath: how the classical
DH ratchet, the PQ reseed, the SCKA advertisement, and — new at NA-0626 — the COMBINED DH+PQ
boundary compose over the session root `RK` and the directional chains. That layer is where
every NA-0624/NA-0625 finding lived.

NA-0626 (ENG-0024) REPLACEMENT, not deletion: the pre-NA-0626 model asserted, per party,
`recv.rk == dh.rk` — the caller-owned coherence of the two duplicated root slots. After the
unification that invariant is TRIVIALLY TRUE (the state carries ONE `rk`), so keeping it would
make the model silently vacuous. The model now carries the single root and asserts what the
unification is FOR: two-party root convergence, PQ healing in the root lineage (now including
the combined boundary), chain continuity, STRUCTURAL send/recv schedule coherence (the
ENG-0030 invariant asserted OF the receive entry points rather than of a caller mitigation),
and reject => no mutation extended to the combined tamper shapes.

Crypto is abstracted to injective tuple hashes: a key is the *derivation history* that produced
it, so two parties hold the same key iff they applied the same derivation to the same inputs.
This models agreement/coherence, NOT secrecy — the model can prove two parties diverge, and can
prove a stale snapshot cannot reconstruct a key, but says nothing about computational hardness.

Two parties, bounded event alphabet:
    A/B x { dh_boundary_send/recv, pq_reseed_send/recv, adv_send/recv, combined_send/recv }
Deliveries are in order per direction (the control plane is in-order-only by design).

Asserted invariants (each guards a specific NA-0624/NA-0625/NA-0626 behaviour):
  1. ROOT CONVERGENCE — after any delivered schedule: A.rk == B.rk (single-slot form; the old
     per-party recv.rk == dh.rk half is unrepresentable by construction after ENG-0024).
  2. PQ HEALING — every PQ epoch secret ever absorbed (reseed OR combined boundary) remains in
     the root lineage across every subsequent boundary (the D560 amendment property, extended
     to the ENG-0026 combined event).
  3. CHAIN CONTINUITY — `nr` advances exactly once per delivered frame (ADV chain-consume;
     combined => the receiver lands at nr == 1 of the NEW DH epoch); an in-order schedule
     leaves NO receive-chain gap (mkskipped empty), including the [ADV, combined] pack.
  4. SEND-SCHEDULE COHERENCE IS STRUCTURAL — after any reseed- or combined-receive, the
     receiver's send header key and send PQ chain equal what the peer will open with, BY
     CONSTRUCTION of the receive event (NA-0626 ENG-0030: the session-level entry point owns
     the whole schedule; no caller composition exists to forget).
  5. REJECT => NO MUTATION — a frame that fails authentication leaves the receiver
     bit-identical, extended to the combined tamper shapes (wrong-NHK header, n != 0,
     stale-DH_pub replay, body tamper).

The combined-boundary composition order is design-locked DH-FIRST-THEN-PQ
(RK_final = KDF_RK_PQ(KDF_RK_DH(RK_pre, dh_out), ss) — §8.2 establishment order, DOC-G5-008
§4, §3.3.6's "RK value before applying KDF_RK_PQ"); `check_regression_shapes` carries the
PQ-FIRST COUNTERFACTUAL proving the model detects the seed-clobbering that order would cause.

Authoritative meaning is defined by DOC-CAN-003 §3.3/§8.5 and DOC-CAN-004 §3.
"""

from __future__ import annotations

from dataclasses import dataclass, replace
from typing import Dict, List, Optional, Set, Tuple

ROLE_A = "A"
ROLE_B = "B"

# A "key" is its derivation history: an immutable, hashable tuple. Injectivity of the tuple
# constructor stands in for collision resistance of the KDF.
Key = Tuple

RK0: Key = ("RK0",)


# ---------------------------------------------------------------------------
# Abstract KDFs (DOC-CAN-003 §3.3/§3.4/§8.1) — injective by construction.
# ---------------------------------------------------------------------------

def kdf_rk_dh(rk: Key, dh_out: Key) -> Tuple[Key, Key]:
    """§3.3.2: (RK', CK_ec0) = KDF_RK_DH(RK, dh_out)."""
    return ("RKDH", rk, dh_out), ("CKEC0", rk, dh_out)


def kdf_rk_pq(rk: Key, pq_ss: Key) -> Key:
    """§3.3.3: RK' = KDF_RK_PQ(RK, pq_ss). This is what carries PQ hardening into the root."""
    return ("RKPQ", rk, pq_ss)


def pq_seed(rk: Key, pq_ss: Key, target: int, direction: str) -> Key:
    """§3.3.6: directional PQ chain seeds from the pre-KDF_RK_PQ root (both parties converge).

    For a PQ-only reseed that root is the session root; for a combined boundary it is RK_dh
    (§3.3.6's "the RK value before applying KDF_RK_PQ for this event").
    """
    return ("PQSEED", direction, rk, pq_ss, target)


def pq0(rk: Key, direction: str) -> Key:
    """§8.5.2 step 6: PQ chain reinit from the root after a DH boundary."""
    return ("PQ0", direction, rk)


def header_key(rk: Key, direction: str, *, nxt: bool) -> Key:
    """§3.4/§8.1: directional header key. `nxt` selects NHK (True) vs HK (False)."""
    return ("NHK" if nxt else "HK", direction, rk)


def ck_step(ck: Key) -> Key:
    """One symmetric chain step (the message key is discarded in this model)."""
    return ("CK", ck)


def direction(sender: str) -> str:
    return "A->B" if sender == ROLE_A else "B->A"


def peer(role: str) -> str:
    return ROLE_B if role == ROLE_A else ROLE_A


# ---------------------------------------------------------------------------
# Party state (mirrors Suite2SessionState: rk / send / recv / dh)
# ---------------------------------------------------------------------------

@dataclass(frozen=True, slots=True)
class Party:
    role: str
    # NA-0626 (ENG-0024): THE session root — the only copy, exactly as in the refimpl.
    rk: Key = RK0
    # Send half.
    hk_s: Key = ("HK", "A->B", RK0)
    ck_ec_s: Key = ("CKEC_S0",)
    ck_pq_s: Key = ("CKPQ_S0",)
    ns: int = 0
    # Receive half.
    hk_r: Key = ("HK", "B->A", RK0)
    ck_ec_r: Key = ("CKEC_R0",)
    ck_pq_r: Key = ("CKPQ_R0",)
    nr: int = 0
    # Bookkeeping.
    dh_pub: int = 0
    peer_dh_pub: int = 0
    mkskipped: int = 0
    # Every pq_ss ever absorbed into this party's root lineage (invariant 2).
    healed_with: Tuple[Key, ...] = ()

    def assert_invariants(self) -> None:
        assert self.ns >= 0 and self.nr >= 0
        # Invariant 3: in-order delivery never parks a skipped message key.
        assert self.mkskipped == 0, f"{self.role}: receive-chain gap opened in an in-order schedule"


def initial(role: str) -> Party:
    """Both parties start from the shared root RK0 with coherent directional keys."""
    send_dir = direction(role)
    recv_dir = direction(peer(role))
    return Party(
        role=role,
        rk=RK0,
        hk_s=header_key(RK0, send_dir, nxt=False),
        ck_ec_s=("CKEC0", send_dir),
        ck_pq_s=("CKPQ0", send_dir),
        hk_r=header_key(RK0, recv_dir, nxt=False),
        ck_ec_r=("CKEC0", recv_dir),
        ck_pq_r=("CKPQ0", recv_dir),
        dh_pub=0 if role == ROLE_A else 100,
        peer_dh_pub=100 if role == ROLE_A else 0,
    )


# ---------------------------------------------------------------------------
# Frames on the wire
# ---------------------------------------------------------------------------

@dataclass(frozen=True, slots=True)
class Frame:
    kind: str  # "dh" | "reseed" | "adv" | "combined"
    sender: str
    # Header authentication material, as the receiver must reconstruct it.
    hdr_key: Key
    n: int
    # Body authentication material.
    body_ck_ec: Key
    body_ck_pq: Key
    # Payload-specific.
    dh_pub: int = 0
    pq_ss: Optional[Key] = None
    target: int = 0
    adv_id: int = 0
    adv_mac_rk: Optional[Key] = None  # the root the ADVAUTH MAC was computed under
    pre_rk: Optional[Key] = None


# ---------------------------------------------------------------------------
# Send events
# ---------------------------------------------------------------------------

def send_dh_boundary(p: Party, fresh_pub: int) -> Tuple[Party, Frame]:
    """§8.5.2 + §8.5.1: header under the PRE-boundary NHK_s; a fresh epoch (n = 0)."""
    d = direction(p.role)
    boundary_hk = header_key(p.rk, d, nxt=True)
    dh_out = ("DH", min(fresh_pub, p.peer_dh_pub), max(fresh_pub, p.peer_dh_pub))
    rk1, ck_ec0 = kdf_rk_dh(p.rk, dh_out)
    ck_pq0 = pq0(rk1, d)
    frame = Frame(
        kind="dh",
        sender=p.role,
        hdr_key=boundary_hk,
        n=0,
        body_ck_ec=ck_ec0,
        body_ck_pq=ck_pq0,
        dh_pub=fresh_pub,
    )
    np = replace(
        p,
        rk=rk1,
        hk_s=header_key(rk1, d, nxt=False),
        ck_ec_s=ck_step(ck_ec0),
        ck_pq_s=ck_step(ck_pq0),
        ns=1,
        dh_pub=fresh_pub,
    )
    return np, frame


def send_pq_reseed(p: Party, pq_ss: Key, target: int) -> Tuple[Party, Frame]:
    """§8.5.3 + §8.5.1 (NA-0625): the PQ-CTXT boundary header seals under the PRE-reseed NHK_s."""
    d = direction(p.role)
    rd = direction(peer(p.role))
    rk_old = p.rk
    nhk_s = header_key(rk_old, d, nxt=True)  # NA-0625: NHK, not HK
    frame = Frame(
        kind="reseed",
        sender=p.role,
        hdr_key=nhk_s,
        n=p.ns,
        body_ck_ec=p.ck_ec_s,
        body_ck_pq=p.ck_pq_s,
        pq_ss=pq_ss,
        target=target,
        pre_rk=rk_old,
    )
    # §3.3.6 ordering: directional seeds from RK_old FIRST, then advance the root.
    seed_send = pq_seed(rk_old, pq_ss, target, d)
    seed_recv = pq_seed(rk_old, pq_ss, target, rd)
    new_rk = kdf_rk_pq(rk_old, pq_ss)
    np = replace(
        p,
        rk=new_rk,
        hk_s=header_key(new_rk, d, nxt=False),
        hk_r=header_key(new_rk, rd, nxt=False),
        ck_ec_s=ck_step(p.ck_ec_s),
        ck_pq_s=seed_send,
        ck_pq_r=seed_recv,
        ns=p.ns + 1,
        healed_with=p.healed_with + (pq_ss,),
    )
    return np, frame


def send_pq_advertise(p: Party, adv_id: int) -> Tuple[Party, Frame]:
    """§8.5.4 (NA-0625): ADV header stays under HK_s; the ADVAUTH MAC keys off the session root.

    The advertisement rides the CURRENT send chain as a control pre-envelope: it consumes one
    step of BOTH send chains, exactly as the receiver consumes both receive chains.
    """
    frame = Frame(
        kind="adv",
        sender=p.role,
        hdr_key=p.hk_s,  # HK, not NHK (an ADV advances no root)
        n=p.ns,
        body_ck_ec=p.ck_ec_s,
        body_ck_pq=p.ck_pq_s,
        adv_id=adv_id,
        adv_mac_rk=p.rk,  # the canonical session root
    )
    np = replace(
        p,
        ck_ec_s=ck_step(p.ck_ec_s),
        ck_pq_s=ck_step(p.ck_pq_s),
        ns=p.ns + 1,
    )
    return np, frame


def send_combined(p: Party, fresh_pub: int, pq_ss: Key, target: int) -> Tuple[Party, Frame]:
    """NA-0626 (ENG-0026): combined DH+PQ boundary SEND — design-locked DH-FIRST-THEN-PQ.

    RK_dh = KDF_RK_DH(RK_pre, dh_out); the ct-bound directional seeds and the transient
    PQ0 chain derive from RK_dh (§3.3.6's pre-KDF_RK_PQ root); RK_final = KDF_RK_PQ(RK_dh, ss).
    The frame is n = 0 of the NEW DH epoch, header under the PRE-boundary NHK_s (§8.5.1);
    the body rides the fresh PRE-seed epoch chains (the seeds apply to FUTURE messages).
    """
    d = direction(p.role)
    rd = direction(peer(p.role))
    rk_pre = p.rk
    boundary_hk = header_key(rk_pre, d, nxt=True)
    dh_out = ("DH", min(fresh_pub, p.peer_dh_pub), max(fresh_pub, p.peer_dh_pub))
    rk_dh, ck_ec0 = kdf_rk_dh(rk_pre, dh_out)
    ck_pq0 = pq0(rk_dh, d)  # transient: keys the n=0 body only
    seed_send = pq_seed(rk_dh, pq_ss, target, d)
    seed_recv = pq_seed(rk_dh, pq_ss, target, rd)
    rk_final = kdf_rk_pq(rk_dh, pq_ss)
    frame = Frame(
        kind="combined",
        sender=p.role,
        hdr_key=boundary_hk,
        n=0,
        body_ck_ec=ck_ec0,
        body_ck_pq=ck_pq0,
        dh_pub=fresh_pub,
        pq_ss=pq_ss,
        target=target,
        pre_rk=rk_pre,
    )
    np = replace(
        p,
        rk=rk_final,
        hk_s=header_key(rk_final, d, nxt=False),
        hk_r=header_key(rk_final, rd, nxt=False),
        ck_ec_s=ck_step(ck_ec0),
        ck_pq_s=seed_send,
        ck_pq_r=seed_recv,
        ns=1,
        dh_pub=fresh_pub,
        healed_with=p.healed_with + (pq_ss,),
    )
    return np, frame


# ---------------------------------------------------------------------------
# Receive events. Each returns (party, accepted). Reject => the SAME party object
# (invariant 5: reject implies no mutation). Each receive event returns the FULL
# coherent schedule — the ENG-0030 structural guarantee lives HERE, not in a caller.
# ---------------------------------------------------------------------------

def recv_dh_boundary(p: Party, f: Frame) -> Tuple[Party, bool]:
    rd = direction(f.sender)
    # §8.5.1: MUST open under the receiver's CURRENT NHK_r.
    if f.hdr_key != header_key(p.rk, rd, nxt=True) or f.n != 0:
        return p, False
    dh_out = ("DH", min(f.dh_pub, p.dh_pub), max(f.dh_pub, p.dh_pub))
    rk1, ck_ec0 = kdf_rk_dh(p.rk, dh_out)
    ck_pq0 = pq0(rk1, rd)
    if f.body_ck_ec != ck_ec0 or f.body_ck_pq != ck_pq0:
        return p, False
    # §8.5.2: a DH boundary reinitialises ONLY the receiving direction; the receiver's send
    # chain is untouched (it creates a fresh send epoch when IT ratchets).
    return (
        replace(
            p,
            rk=rk1,
            hk_r=header_key(rk1, rd, nxt=False),
            ck_ec_r=ck_step(ck_ec0),
            ck_pq_r=ck_step(ck_pq0),
            nr=1,
            peer_dh_pub=f.dh_pub,
        ),
        True,
    )


def recv_pq_reseed(p: Party, f: Frame) -> Tuple[Party, bool]:
    """§8.5.3 receiver (NA-0625/NA-0626): NHK-only header open; strict in-order (n == nr).

    NA-0626 (ENG-0030 structural): this event IS the session-level entry point — it returns
    the receiver's FULL schedule (root, receive half, AND the mirrored send half). There is
    no caller step left to forget.
    """
    d = direction(p.role)
    rd = direction(f.sender)
    if f.hdr_key != header_key(p.rk, rd, nxt=True):
        return p, False  # includes the pre-NA-0625 HK-downgrade frame
    if f.n != p.nr:
        return p, False
    if f.body_ck_ec != p.ck_ec_r or f.body_ck_pq != p.ck_pq_r:
        return p, False

    rk_old = p.rk
    seed_recv = pq_seed(rk_old, f.pq_ss, f.target, rd)  # the sender's send direction
    seed_send = pq_seed(rk_old, f.pq_ss, f.target, d)
    new_rk = kdf_rk_pq(rk_old, f.pq_ss)
    return (
        replace(
            p,
            rk=new_rk,
            hk_r=header_key(new_rk, rd, nxt=False),
            # ENG-0030 STRUCTURAL: the send half is part of the entry point's commit.
            hk_s=header_key(new_rk, d, nxt=False),
            ck_pq_s=seed_send,
            ck_ec_r=ck_step(p.ck_ec_r),
            ck_pq_r=seed_recv,
            nr=p.nr + 1,
            healed_with=p.healed_with + (f.pq_ss,),
        ),
        True,
    )


def recv_pq_adv(p: Party, f: Frame, watermark: int) -> Tuple[Party, bool]:
    """§8.5.4 (NA-0625): authenticated ADV receive, in-order-only, chain-consuming."""
    if f.hdr_key != p.hk_r:
        return p, False  # spoofed / foreign-key header
    if f.n != p.nr:
        return p, False  # in-order-only control plane
    if f.body_ck_ec != p.ck_ec_r or f.body_ck_pq != p.ck_pq_r:
        return p, False
    if f.adv_mac_rk != p.rk:
        return p, False  # ADVAUTH MAC under a foreign root
    if f.adv_id <= watermark:
        return p, False  # DOC-CAN-004 §3.2 monotonicity
    # Chain-consume: BOTH receive chains step, nr advances, no mkskipped growth.
    return (
        replace(
            p,
            ck_ec_r=ck_step(p.ck_ec_r),
            ck_pq_r=ck_step(p.ck_pq_r),
            nr=p.nr + 1,
        ),
        True,
    )


def recv_combined(p: Party, f: Frame) -> Tuple[Party, bool]:
    """NA-0626 (ENG-0026): combined DH+PQ boundary RECEIVE — the fresh-DH_pub arm.

    Fail-closed rules mirrored from the refimpl: a stale/current DH_pub is not a combined
    frame (replay shape); the header MUST open under the CURRENT NHK_r at n == 0 of the NEW
    epoch; DH first, then PQ, from the design-locked composition. The commit returns the FULL
    schedule (invariant 4, structural), landing the receiver at nr == 1 of the new epoch.
    """
    d = direction(p.role)
    rd = direction(f.sender)
    if f.dh_pub == p.peer_dh_pub:
        return p, False  # stale DH_pub (replayed combined frame / not fresh)
    if f.hdr_key != header_key(p.rk, rd, nxt=True):
        return p, False  # wrong-NHK header (incl. HK downgrade)
    if f.n != 0:
        return p, False  # the combined frame is n=0 of the new DH epoch ONLY
    dh_out = ("DH", min(f.dh_pub, p.dh_pub), max(f.dh_pub, p.dh_pub))
    rk_dh, ck_ec0 = kdf_rk_dh(p.rk, dh_out)
    ck_pq0 = pq0(rk_dh, rd)
    if f.body_ck_ec != ck_ec0 or f.body_ck_pq != ck_pq0:
        return p, False
    seed_recv = pq_seed(rk_dh, f.pq_ss, f.target, rd)
    seed_send = pq_seed(rk_dh, f.pq_ss, f.target, d)
    rk_final = kdf_rk_pq(rk_dh, f.pq_ss)
    return (
        replace(
            p,
            rk=rk_final,
            hk_r=header_key(rk_final, rd, nxt=False),
            hk_s=header_key(rk_final, d, nxt=False),
            ck_ec_r=ck_step(ck_ec0),
            ck_pq_r=seed_recv,
            ck_pq_s=seed_send,
            nr=1,
            peer_dh_pub=f.dh_pub,
            healed_with=p.healed_with + (f.pq_ss,),
        ),
        True,
    )


# ---------------------------------------------------------------------------
# Bounded exploration
# ---------------------------------------------------------------------------

@dataclass(frozen=True, slots=True)
class World:
    a: Party
    b: Party
    a_watermark: int = 0  # B's advertisements seen by A
    b_watermark: int = 0  # A's advertisements seen by B
    next_pub: int = 1
    next_ss: int = 1
    next_adv: int = 1

    def party(self, role: str) -> Party:
        return self.a if role == ROLE_A else self.b

    def with_party(self, p: Party) -> "World":
        return replace(self, a=p) if p.role == ROLE_A else replace(self, b=p)

    def assert_invariants(self) -> None:
        self.a.assert_invariants()
        self.b.assert_invariants()
        # Invariant 1: after a fully delivered schedule both parties hold the same root.
        assert self.a.rk == self.b.rk, "root convergence broke between the parties"
        # Invariant 4 (agreement form): each party's send schedule matches the peer's receive
        # schedule, in both directions — header keys, both chains, and the message counters.
        # After ENG-0030 this holds BY CONSTRUCTION of the receive events.
        assert self.a.hk_s == self.b.hk_r, "A->B header keys diverged"
        assert self.b.hk_s == self.a.hk_r, "B->A header keys diverged"
        assert self.a.ck_pq_s == self.b.ck_pq_r, "A->B PQ chains diverged"
        assert self.b.ck_pq_s == self.a.ck_pq_r, "B->A PQ chains diverged"
        assert self.a.ck_ec_s == self.b.ck_ec_r, "A->B EC chains diverged"
        assert self.b.ck_ec_s == self.a.ck_ec_r, "B->A EC chains diverged"
        # Invariant 3 (counter form): in-order delivery keeps ns == peer's nr per direction.
        assert self.a.ns == self.b.nr, "A->B counters diverged (a frame was skipped)"
        assert self.b.ns == self.a.nr, "B->A counters diverged (a frame was skipped)"
        # Invariant 2: every PQ secret ever absorbed is still in BOTH parties' root lineage.
        assert set(self.a.healed_with) == set(self.b.healed_with), "healing lineage diverged"
        for ss in self.a.healed_with:
            assert _contains(self.a.rk, ss), (
                "a PQ epoch secret was lost from the root lineage (healing did not survive)"
            )


def _contains(key, needle) -> bool:
    """Structural: is `needle` present anywhere in a key's derivation history?"""
    if key == needle:
        return True
    if isinstance(key, tuple):
        return any(_contains(x, needle) for x in key)
    return False


def _reject_no_mutation(p: Party, f: Frame, watermark: int) -> None:
    """Invariant 5: a frame that fails authentication leaves the receiver bit-identical."""
    if f.kind == "dh":
        out, ok = recv_dh_boundary(p, f)
    elif f.kind == "reseed":
        out, ok = recv_pq_reseed(p, f)
    elif f.kind == "combined":
        out, ok = recv_combined(p, f)
    else:
        out, ok = recv_pq_adv(p, f, watermark)
    if not ok:
        assert out == p, f"reject mutated receiver state ({f.kind})"


def _tampered_variants(f: Frame, rx: Party) -> List[Frame]:
    """Negative frames a network attacker can produce, incl. the NA-0625 downgrade and the
    NA-0626 combined tamper shapes."""
    out = [
        replace(f, hdr_key=("FORGED",)),                    # spoofed/foreign header key
        replace(f, body_ck_ec=("FORGED",)),                 # body tamper
        replace(f, n=f.n + 1),                              # out-of-order / replay shifted
    ]
    if f.kind in ("reseed", "combined"):
        # THE HK-DOWNGRADE: a boundary frame sealed under HK instead of NHK.
        assert f.pre_rk is not None
        out.append(replace(f, hdr_key=header_key(f.pre_rk, direction(f.sender), nxt=False)))
    if f.kind == "combined":
        # STALE-DH_PUB REPLAY: the carried key is the receiver's CURRENT peer key, so the
        # fresh-key discriminator must refuse it as a combined frame.
        out.append(replace(f, dh_pub=rx.peer_dh_pub))
    if f.kind == "adv":
        out.append(replace(f, adv_mac_rk=("FORGED_RK",)))   # ADVAUTH MAC under a foreign root
        out.append(replace(f, adv_id=0))                    # non-monotonic advertisement
    return out


def _deliver(w: World, f: Frame) -> Optional[World]:
    """Deliver `f` to the peer of its sender, asserting the negative cases on the way."""
    rx_role = peer(f.sender)
    rx = w.party(rx_role)
    watermark = w.a_watermark if rx_role == ROLE_A else w.b_watermark

    # Invariant 5, checked against the live receiver state before the honest delivery.
    for bad in _tampered_variants(f, rx):
        _reject_no_mutation(rx, bad, watermark)

    if f.kind == "dh":
        nrx, ok = recv_dh_boundary(rx, f)
    elif f.kind == "reseed":
        nrx, ok = recv_pq_reseed(rx, f)
    elif f.kind == "combined":
        nrx, ok = recv_combined(rx, f)
    else:
        nrx, ok = recv_pq_adv(rx, f, watermark)
    if not ok:
        return None

    nw = w.with_party(nrx)
    if f.kind == "adv":
        nw = (
            replace(nw, a_watermark=f.adv_id)
            if rx_role == ROLE_A
            else replace(nw, b_watermark=f.adv_id)
        )
    return nw


def _successors(w: World) -> List[Tuple[str, World]]:
    """Send-then-deliver-in-order steps (the control plane is in-order-only by design)."""
    out: List[Tuple[str, World]] = []
    for role in (ROLE_A, ROLE_B):
        p = w.party(role)

        # DH boundary.
        np, f = send_dh_boundary(p, w.next_pub)
        nw = _deliver(replace(w, next_pub=w.next_pub + 1).with_party(np), f)
        if nw is not None:
            out.append((f"{role}:dh", nw))

        # PQ reseed (targets an advertisement the peer published; the SCKA one-time/target
        # rules themselves are covered by model_scka_bounded).
        ss: Key = ("SS", w.next_ss)
        np, f = send_pq_reseed(p, ss, w.next_ss)
        nw = _deliver(replace(w, next_ss=w.next_ss + 1).with_party(np), f)
        if nw is not None:
            out.append((f"{role}:reseed", nw))

        # SCKA advertisement.
        np, f = send_pq_advertise(p, w.next_adv)
        nw = _deliver(replace(w, next_adv=w.next_adv + 1).with_party(np), f)
        if nw is not None:
            out.append((f"{role}:adv", nw))

        # NA-0626: combined DH+PQ boundary.
        ss2: Key = ("SS", w.next_ss)
        np, f = send_combined(p, w.next_pub, ss2, w.next_ss)
        nw = _deliver(
            replace(w, next_pub=w.next_pub + 1, next_ss=w.next_ss + 1).with_party(np), f
        )
        if nw is not None:
            out.append((f"{role}:combined", nw))

    return out


def explore(*, max_depth: int = 5) -> Dict[str, int]:
    """Explore all in-order interleavings up to `max_depth` events. Fail-closed on any violation."""
    init = World(a=initial(ROLE_A), b=initial(ROLE_B))
    init.assert_invariants()

    visited: Set[World] = {init}
    frontier: List[Tuple[World, int, Tuple[str, ...]]] = [(init, 0, ())]
    states = 0
    transitions = 0
    max_reached = 0

    while frontier:
        w, depth, trace = frontier.pop()
        states += 1
        max_reached = max(max_reached, depth)
        if depth >= max_depth:
            continue
        for label, nw in _successors(w):
            transitions += 1
            ntrace = trace + (label,)
            try:
                nw.assert_invariants()
            except AssertionError as e:  # pragma: no cover - surfaces the counterexample
                raise AssertionError(f"{e} | trace: {' -> '.join(ntrace)}") from e
            if nw not in visited:
                visited.add(nw)
                frontier.append((nw, depth + 1, ntrace))

    return {
        "states": states,
        "transitions": transitions,
        "visited": len(visited),
        "max_depth": max_reached,
    }


def check_regression_shapes() -> Dict[str, int]:
    """The specific NA-0624/NA-0625/NA-0626 shapes, asserted directly."""
    checks = 0

    # (i) reseed-then-DH and DH-then-reseed both keep the PQ secret in the root lineage.
    for order in (("reseed", "dh"), ("dh", "reseed")):
        w = World(a=initial(ROLE_A), b=initial(ROLE_B))
        for kind in order:
            if kind == "dh":
                np, f = send_dh_boundary(w.party(ROLE_A), w.next_pub)
                w = _deliver(replace(w, next_pub=w.next_pub + 1).with_party(np), f)
            else:
                np, f = send_pq_reseed(w.party(ROLE_A), ("SS", 1), 1)
                w = _deliver(w.with_party(np), f)
            assert w is not None, f"{order}: honest delivery rejected"
            w.assert_invariants()
        assert _contains(w.a.rk, ("SS", 1)), f"{order}: PQ healing lost"
        assert _contains(w.b.rk, ("SS", 1)), f"{order}: PQ healing lost at the peer"
        checks += 1

    # (ii) [ADV, reseed] in ONE pack round-trips (Operator Decision 2): the ADV consumes its
    # chain slot, so the strict in-order reseed receiver still sees n == nr.
    w = World(a=initial(ROLE_A), b=initial(ROLE_B))
    np, adv = send_pq_advertise(w.party(ROLE_A), 1)
    w2 = w.with_party(np)
    np, reseed = send_pq_reseed(w2.party(ROLE_A), ("SS", 9), 9)
    w3 = w2.with_party(np)
    w4 = _deliver(w3, adv)
    assert w4 is not None, "the in-pack ADV must be accepted"
    w5 = _deliver(w4, reseed)
    assert w5 is not None, "the reseed must be accepted immediately after the in-pack ADV"
    w5.assert_invariants()
    assert w5.b.mkskipped == 0, "the chain-consumed ADV must leave no receive-chain gap"
    assert w5.b_watermark == 1, "the authenticated ADV must be tracked"
    checks += 1

    # (iii) An ADV that is NOT chain-consumed would desynchronise the reseed's n == nr check.
    stale = replace(w4.b, nr=w4.b.nr - 1, ck_ec_r=w3.b.ck_ec_r, ck_pq_r=w3.b.ck_pq_r)
    _, ok = recv_pq_reseed(stale, reseed)
    assert not ok, "without chain-consume the in-pack reseed must fail the in-order check"
    checks += 1

    # (iv) THE HK-DOWNGRADE: a reseed boundary sealed under HK (pre-NA-0625) is rejected, and
    # the reject mutates nothing.
    w = World(a=initial(ROLE_A), b=initial(ROLE_B))
    np, f = send_pq_reseed(w.party(ROLE_A), ("SS", 2), 2)
    downgraded = replace(f, hdr_key=header_key(f.pre_rk, direction(ROLE_A), nxt=False))
    before = w.b
    after, ok = recv_pq_reseed(before, downgraded)
    assert not ok, "an HK-sealed boundary header must be rejected under the §8.5.1 NHK rule"
    assert after == before, "the downgrade reject must not mutate receiver state"
    checks += 1

    # (v) ENG-0030, INVERTED at NA-0626: the receive entry point returns the receiver's send
    # schedule mirrored from the advanced root BY CONSTRUCTION — its next advertisement
    # authenticates at the peer with NO caller composition. The counterfactual (a receiver
    # that somehow kept the stale send half — representable only by hand-editing the state,
    # exactly as in the code, where the deleted qsc mitigation is unrepresentable) still
    # fails at the peer, proving the model DETECTS the ENG-0030 class.
    w = World(a=initial(ROLE_A), b=initial(ROLE_B))
    np, f = send_pq_reseed(w.party(ROLE_A), ("SS", 3), 3)
    w = w.with_party(np)
    rx, ok = recv_pq_reseed(w.party(ROLE_B), f)
    assert ok
    _, adv_f = send_pq_advertise(rx, 1)
    _, accepted = recv_pq_adv(w.party(ROLE_A), adv_f, 0)
    assert accepted, "the entry point's schedule must authenticate immediately (structural)"
    buggy = replace(rx, hk_s=w.party(ROLE_B).hk_s, ck_pq_s=w.party(ROLE_B).ck_pq_s)
    _, adv_f = send_pq_advertise(buggy, 1)
    _, accepted = recv_pq_adv(w.party(ROLE_A), adv_f, 0)
    assert not accepted, "a hand-staled send schedule must still fail at the peer (detection)"
    checks += 1

    # (vi) NA-0626 COMBINED ROUND-TRIP, incl. the [ADV, combined] pack: root convergence, the
    # receiver lands at nr == 1 of the new epoch with no receive-chain gap, and the full
    # schedule agrees.
    w = World(a=initial(ROLE_A), b=initial(ROLE_B))
    np, adv = send_pq_advertise(w.party(ROLE_A), 1)
    w = w.with_party(np)
    np, comb = send_combined(w.party(ROLE_A), 7, ("SS", 4), 4)
    w = w.with_party(np)
    w = _deliver(w, adv)
    assert w is not None, "the in-pack ADV before a combined boundary must be accepted"
    w = _deliver(w, comb)
    assert w is not None, "the combined boundary must be accepted after the in-pack ADV"
    w.assert_invariants()
    assert w.b.nr == 1, "the combined receiver must land at nr == 1 of the new DH epoch"
    assert w.b.mkskipped == 0, "the combined boundary must leave no receive-chain gap"
    assert _contains(w.a.rk, ("SS", 4)) and _contains(w.b.rk, ("SS", 4)), (
        "the combined boundary's PQ secret must land in both root lineages"
    )
    assert _contains(w.a.rk, ("DH", 7, 100)) or _contains(w.a.rk, ("DH", 100, 7)), (
        "the combined boundary's DH secret must land in the root lineage"
    )
    checks += 1

    # (vii) THE PQ-FIRST MIS-COMPOSITION COUNTERFACTUAL (pins the §5 design-lock ordering at
    # model level): if KDF_RK_PQ were applied FIRST, §8.5.2 step 6 (PQ0 reinit from the
    # post-DH root) would CLOBBER §8.5.3 step 6's ct-bound directional seeds — the final PQ
    # chain would carry NO binding to the SCKA target. The model detects exactly that.
    rk_pre = RK0
    dh_out = ("DH", 0, 100)
    ss: Key = ("SS", 5)
    target = 77  # deliberately disjoint from every other constant (the containment check is structural)
    # Design-locked DH-first: the final send PQ chain IS the ct-bound seed.
    rk_dh, _ck = kdf_rk_dh(rk_pre, dh_out)
    good_chain = pq_seed(rk_dh, ss, target, "A->B")
    assert _contains(good_chain, ss) and _contains(good_chain, target), (
        "DH-first must leave the ct-bound seed as the directional PQ chain"
    )
    # PQ-first counterfactual: root advances with ss first, then the DH step reinitialises
    # the PQ chain from the post-DH root per §8.5.2 step 6 — overwriting the seed assignment.
    rk_pq = kdf_rk_pq(rk_pre, ss)
    rk_bad, _ck_bad = kdf_rk_dh(rk_pq, dh_out)
    clobbered_chain = pq0(rk_bad, "A->B")  # what §8.5.2 step 6 leaves in the chain slot
    assert not _contains(clobbered_chain, target), (
        "PQ-first: the model must DETECT that the PQ0 reinit clobbers the ct-bound seed "
        "(no target binding survives in the chain)"
    )
    assert clobbered_chain != pq_seed(rk_pq, ss, target, "A->B"), (
        "PQ-first: the chain slot no longer holds the §8.5.3 step-6 seed"
    )
    checks += 1

    # (viii) COMBINED-THEN-DH HEALING PERSISTENCE: the combined boundary's PQ secret survives a
    # subsequent classical DH boundary in the root lineage, and a pre-combined snapshot cannot
    # open that boundary (its NHK derives from a root without the combined secrets).
    w = World(a=initial(ROLE_A), b=initial(ROLE_B))
    a_snapshot = w.a  # pre-combined compromise
    np, comb = send_combined(w.party(ROLE_A), 11, ("SS", 6), 6)
    w = _deliver(w.with_party(np), comb)
    assert w is not None
    np, dh_f = send_dh_boundary(w.party(ROLE_B), 12)
    w2 = _deliver(w.with_party(np), dh_f)
    assert w2 is not None, "the live party must open the post-combined DH boundary"
    w2.assert_invariants()
    assert _contains(w2.a.rk, ("SS", 6)), "combined healing must survive the DH boundary"
    _, ok = recv_dh_boundary(a_snapshot, dh_f)
    assert not ok, (
        "a pre-combined snapshot must NOT open the post-combined DH boundary (hybrid healing)"
    )
    checks += 1

    return {"regression_shapes": checks}


def emit_suite2_root_composition_model_report() -> Dict[str, int]:
    stats = explore(max_depth=5)
    stats.update(check_regression_shapes())
    return stats
