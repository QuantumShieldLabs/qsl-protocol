"""Bounded executable model of the Suite-2 ROOT COMPOSITION layer (NA-0625 / ENG-0023).

Goals: G4

Where `model_scka_bounded` covers SCKA *logic* invariants (ADV monotonicity, one-time CTXT
targeting, reject => no mutation), this module models the layer underneath: how the classical
DH ratchet, the PQ reseed, and the SCKA advertisement compose over the shared root `RK` and the
directional chains. That layer is where every NA-0624 finding lived (recv.rk/dh.rk coherence),
and it is exactly the surface NA-0625 changes (§8.5.1 NHK boundary headers, the authenticated
ADV receive with chain-consume).

Crypto is abstracted to injective tuple hashes: a key is the *derivation history* that produced
it, so two parties hold the same key iff they applied the same derivation to the same inputs.
This models agreement/coherence, NOT secrecy — the model can prove two parties diverge, and can
prove a stale snapshot cannot reconstruct a key, but says nothing about computational hardness.

Two parties, bounded event alphabet:
    A/B x { dh_boundary_send/recv, pq_reseed_send/recv, adv_send/recv }
Deliveries are in order per direction (the control plane is in-order-only by design).

Asserted invariants (each guards a specific NA-0624/NA-0625 behaviour):
  1. ROOT CONVERGENCE — after any delivered schedule: A.rk == B.rk, and per party the PQ-path
     root and the DH-ratchet root agree (`recv_rk == dh_rk`). Catches the NA-0624 dh.rk-sync bug.
  2. PQ HEALING SURVIVES A DH BOUNDARY — a reseed's epoch secret remains in the root lineage
     across every subsequent DH boundary (the D560 amendment property; epoch-granular PCS shape).
  3. CHAIN CONTINUITY UNDER ADV CHAIN-CONSUME — `nr` advances exactly once per delivered frame
     including an ADV, so an in-order schedule leaves NO receive-chain gap (mkskipped empty).
     This is the Operator Decision 2 retirement of the NA-0624 pack-exclusion + mkskipped growth.
  4. SEND-SCHEDULE COHERENCE AFTER A RESEED RECEIVE — the reseed receiver's send header key and
     send PQ chain must be re-derived from the advanced root, or its next control pre-envelope
     is sealed under a schedule the peer no longer holds. (The NA-0625 qsc CTXT-arm finding.)
  5. REJECT => NO MUTATION — a frame that fails header/body/MAC authentication leaves the
     receiver's state bit-identical (extended here to the ADV receive events).

Authoritative meaning is defined by DOC-CAN-003 §8.5 and DOC-CAN-004 §3.
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
    """§3.3.6: directional PQ chain seeds from the PRE-reseed root (both parties converge)."""
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
# Party state (mirrors Suite2SessionState: send / recv / dh)
# ---------------------------------------------------------------------------

@dataclass(frozen=True, slots=True)
class Party:
    role: str
    # The two root slots the refimpl carries (ENG-0024 will unify them).
    recv_rk: Key = RK0
    dh_rk: Key = RK0
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
        # Invariant 1 (per-party half): the PQ-path root and the DH-ratchet root agree.
        assert self.recv_rk == self.dh_rk, (
            f"{self.role}: root duality broke (recv.rk != dh.rk) — the NA-0624 dh.rk-sync class"
        )
        assert self.ns >= 0 and self.nr >= 0
        # Invariant 3: in-order delivery never parks a skipped message key.
        assert self.mkskipped == 0, f"{self.role}: receive-chain gap opened in an in-order schedule"


def initial(role: str) -> Party:
    """Both parties start from the shared root RK0 with coherent directional keys."""
    send_dir = direction(role)
    recv_dir = direction(peer(role))
    return Party(
        role=role,
        recv_rk=RK0,
        dh_rk=RK0,
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
    kind: str  # "dh" | "reseed" | "adv"
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
    boundary_hk = header_key(p.dh_rk, d, nxt=True)
    dh_out = ("DH", min(fresh_pub, p.peer_dh_pub), max(fresh_pub, p.peer_dh_pub))
    rk1, ck_ec0 = kdf_rk_dh(p.dh_rk, dh_out)
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
        recv_rk=rk1,
        dh_rk=rk1,
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
    rk_old = p.dh_rk
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
        recv_rk=new_rk,
        dh_rk=new_rk,
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
        adv_mac_rk=p.dh_rk,  # the canonical session root
    )
    np = replace(
        p,
        ck_ec_s=ck_step(p.ck_ec_s),
        ck_pq_s=ck_step(p.ck_pq_s),
        ns=p.ns + 1,
    )
    return np, frame


# ---------------------------------------------------------------------------
# Receive events. Each returns (party, accepted). Reject => the SAME party object
# (invariant 5: reject implies no mutation).
# ---------------------------------------------------------------------------

def recv_dh_boundary(p: Party, f: Frame) -> Tuple[Party, bool]:
    rd = direction(f.sender)
    # §8.5.1: MUST open under the receiver's CURRENT NHK_r.
    if f.hdr_key != header_key(p.dh_rk, rd, nxt=True) or f.n != 0:
        return p, False
    dh_out = ("DH", min(f.dh_pub, p.dh_pub), max(f.dh_pub, p.dh_pub))
    rk1, ck_ec0 = kdf_rk_dh(p.dh_rk, dh_out)
    ck_pq0 = pq0(rk1, rd)
    if f.body_ck_ec != ck_ec0 or f.body_ck_pq != ck_pq0:
        return p, False
    # §8.5.2: a DH boundary reinitialises ONLY the receiving direction; the receiver's send
    # chain is untouched (it creates a fresh send epoch when IT ratchets). So the peer's
    # send schedule and this receiver's receive schedule stay coherent, and vice versa.
    return (
        replace(
            p,
            recv_rk=rk1,
            dh_rk=rk1,
            hk_r=header_key(rk1, rd, nxt=False),
            ck_ec_r=ck_step(ck_ec0),
            ck_pq_r=ck_step(ck_pq0),
            nr=1,
            peer_dh_pub=f.dh_pub,
        ),
        True,
    )


def recv_pq_reseed(p: Party, f: Frame) -> Tuple[Party, bool]:
    """§8.5.3 receiver (NA-0625): NHK-only header open; strict in-order (n == nr)."""
    d = direction(p.role)
    rd = direction(f.sender)
    if f.hdr_key != header_key(p.dh_rk, rd, nxt=True):
        return p, False  # includes the pre-NA-0625 HK-downgrade frame
    if f.n != p.nr:
        return p, False
    if f.body_ck_ec != p.ck_ec_r or f.body_ck_pq != p.ck_pq_r:
        return p, False

    rk_old = p.dh_rk
    seed_recv = pq_seed(rk_old, f.pq_ss, f.target, rd)  # the sender's send direction
    seed_send = pq_seed(rk_old, f.pq_ss, f.target, d)
    new_rk = kdf_rk_pq(rk_old, f.pq_ss)
    return (
        replace(
            p,
            recv_rk=new_rk,
            dh_rk=new_rk,  # THE ADOPT (NA-0624)
            hk_r=header_key(new_rk, rd, nxt=False),
            # Invariant 4: the receiver MUST mirror the send half from the advanced root.
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
    if f.adv_mac_rk != p.dh_rk:
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
        assert self.a.dh_rk == self.b.dh_rk, "root convergence broke between the parties"
        # Invariant 4 (agreement form): each party's send schedule matches the peer's receive
        # schedule, in both directions — header keys, both chains, and the message counters.
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
            assert _root_contains(self.a.dh_rk, ss), (
                "a PQ epoch secret was lost from the root lineage (healing did not survive)"
            )


def _root_contains(rk: Key, ss: Key) -> bool:
    """Structural: is `ss` present anywhere in the root's derivation history?"""
    if rk == ss:
        return True
    return any(_root_contains(x, ss) for x in rk if isinstance(x, tuple))


def _reject_no_mutation(p: Party, f: Frame, watermark: int) -> None:
    """Invariant 5: a frame that fails authentication leaves the receiver bit-identical."""
    if f.kind == "dh":
        out, ok = recv_dh_boundary(p, f)
    elif f.kind == "reseed":
        out, ok = recv_pq_reseed(p, f)
    else:
        out, ok = recv_pq_adv(p, f, watermark)
    if not ok:
        assert out == p, f"reject mutated receiver state ({f.kind})"


def _tampered_variants(f: Frame) -> List[Frame]:
    """Negative frames a network attacker can produce, incl. the NA-0625 downgrade cases."""
    out = [
        replace(f, hdr_key=("FORGED",)),                    # spoofed/foreign header key
        replace(f, body_ck_ec=("FORGED",)),                 # body tamper
        replace(f, n=f.n + 1),                              # out-of-order / replay shifted
    ]
    if f.kind == "reseed":
        # THE HK-DOWNGRADE: a pre-NA-0625 frame sealed under HK instead of NHK.
        assert f.pre_rk is not None
        out.append(replace(f, hdr_key=header_key(f.pre_rk, direction(f.sender), nxt=False)))
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
    for bad in _tampered_variants(f):
        _reject_no_mutation(rx, bad, watermark)

    if f.kind == "dh":
        nrx, ok = recv_dh_boundary(rx, f)
    elif f.kind == "reseed":
        nrx, ok = recv_pq_reseed(rx, f)
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

    return out


def explore(*, max_depth: int = 6) -> Dict[str, int]:
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
    """The specific NA-0624/NA-0625 shapes, asserted directly (not just as reachable states)."""
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
        assert _root_contains(w.a.dh_rk, ("SS", 1)), f"{order}: PQ healing lost"
        assert _root_contains(w.b.dh_rk, ("SS", 1)), f"{order}: PQ healing lost at the peer"
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
    # (Models the counterfactual the pre-NA-0625 pack-exclusion rule existed to avoid.)
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

    # (v) THE NA-0625 FINDING: if the reseed receiver does NOT mirror its send schedule from the
    # advanced root, its next advertisement fails the peer's authenticated ADV receiver.
    w = World(a=initial(ROLE_A), b=initial(ROLE_B))
    np, f = send_pq_reseed(w.party(ROLE_A), ("SS", 3), 3)
    w = w.with_party(np)
    rx, ok = recv_pq_reseed(w.party(ROLE_B), f)
    assert ok
    # Simulate the BUG: keep the pre-reseed send schedule on the receiver.
    buggy = replace(rx, hk_s=w.party(ROLE_B).hk_s, ck_pq_s=w.party(ROLE_B).ck_pq_s)
    _, adv_f = send_pq_advertise(buggy, 1)
    _, accepted = recv_pq_adv(w.party(ROLE_A), adv_f, 0)
    assert not accepted, "the stale-send-schedule advertisement must fail (the finding)"
    # And with the correct composition it authenticates.
    _, adv_f = send_pq_advertise(rx, 1)
    _, accepted = recv_pq_adv(w.party(ROLE_A), adv_f, 0)
    assert accepted, "after mirroring the send schedule the advertisement authenticates"
    checks += 1

    return {"regression_shapes": checks}


def emit_suite2_root_composition_model_report() -> Dict[str, int]:
    stats = explore(max_depth=6)
    stats.update(check_regression_shapes())
    return stats
