"""Bounded executable model checks for SCKA logic invariants.

Goals: G4

This module is intentionally crypto-agnostic. It explores a bounded state space for:
- ADV monotonicity (strict >)
- one-time CTXT targeting with tombstones
- fail-closed reject semantics (reject => no state change)

Authoritative meaning is defined by DOC-CAN-004.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Iterable, Optional, Tuple, Set, List, Dict

ROLE_A = "A"
ROLE_B = "B"


def _sorted_unique_ints(xs: Iterable[int]) -> Tuple[int, ...]:
    return tuple(sorted(set(xs)))


@dataclass(frozen=True, slots=True)
class Party:
    peer_max_adv_id_seen: int = 0
    peer_current_adv_id: Optional[int] = None
    local_next_adv_id: int = 0
    local_keys: Tuple[int, ...] = ()
    tombstones: Tuple[int, ...] = ()

    def assert_invariants(self) -> None:
        assert self.peer_max_adv_id_seen >= 0
        if self.peer_current_adv_id is not None:
            assert self.peer_current_adv_id == self.peer_max_adv_id_seen
        assert self.local_next_adv_id >= 0
        assert self.local_keys == tuple(sorted(self.local_keys))
        assert self.tombstones == tuple(sorted(self.tombstones))
        assert len(set(self.local_keys)) == len(self.local_keys)
        assert len(set(self.tombstones)) == len(self.tombstones)
        assert set(self.local_keys).isdisjoint(self.tombstones)


@dataclass(frozen=True, slots=True)
class Message:
    kind: str  # "ADV" or "CTXT"
    src: str
    dst: str
    adv_id: Optional[int] = None
    target_id: Optional[int] = None

    def key(self) -> Tuple:
        return (self.kind, self.src, self.dst, self.adv_id or 0, self.target_id or 0)


def _canon_msgs(msgs: Iterable[Message]) -> Tuple[Message, ...]:
    uniq = set(msgs)
    return tuple(sorted(uniq, key=lambda m: m.key()))


@dataclass(frozen=True, slots=True)
class World:
    a: Party = Party()
    b: Party = Party()
    net: Tuple[Message, ...] = ()
    seen: Tuple[Message, ...] = ()

    def assert_invariants(self) -> None:
        self.a.assert_invariants()
        self.b.assert_invariants()
        assert self.net == tuple(sorted(self.net, key=lambda m: m.key()))
        assert self.seen == tuple(sorted(self.seen, key=lambda m: m.key()))

    def get_party(self, role: str) -> Party:
        if role == ROLE_A:
            return self.a
        if role == ROLE_B:
            return self.b
        raise AssertionError("unknown role")

    def with_party(self, role: str, p: Party) -> "World":
        if role == ROLE_A:
            return World(a=p, b=self.b, net=self.net, seen=self.seen)
        if role == ROLE_B:
            return World(a=self.a, b=p, net=self.net, seen=self.seen)
        raise AssertionError("unknown role")


def _other(role: str) -> str:
    return ROLE_B if role == ROLE_A else ROLE_A


def _add_message(w: World, msg: Message, *, max_net: int, max_seen: int) -> Optional[World]:
    if len(w.net) >= max_net:
        return None
    seen = _canon_msgs((*w.seen, msg))
    if len(seen) > max_seen:
        return None
    net = _canon_msgs((*w.net, msg))
    w2 = World(a=w.a, b=w.b, net=net, seen=seen)
    w2.assert_invariants()
    return w2


def action_send_adv(w: World, role: str, *, max_net: int, max_seen: int) -> Optional[World]:
    p = w.get_party(role)
    new_id = p.local_next_adv_id + 1
    # modeled overflow: abort (fail-closed)
    if new_id <= p.local_next_adv_id:
        raise AssertionError("local_next_adv_id must be strictly increasing")

    p2 = Party(
        peer_max_adv_id_seen=p.peer_max_adv_id_seen,
        peer_current_adv_id=p.peer_current_adv_id,
        local_next_adv_id=new_id,
        local_keys=_sorted_unique_ints((*p.local_keys, new_id)),
        tombstones=p.tombstones,
    )

    w2 = w.with_party(role, p2)
    msg = Message(kind="ADV", src=role, dst=_other(role), adv_id=new_id)
    return _add_message(w2, msg, max_net=max_net, max_seen=max_seen)


def action_send_ctxt(w: World, role: str, *, max_net: int, max_seen: int) -> Optional[World]:
    p = w.get_party(role)
    if p.peer_current_adv_id is None:
        return None
    msg = Message(kind="CTXT", src=role, dst=_other(role), target_id=p.peer_current_adv_id)
    return _add_message(w, msg, max_net=max_net, max_seen=max_seen)


def _deliver(w: World, idx: int) -> World:
    msg = w.net[idx]
    net_rest = tuple(m for i, m in enumerate(w.net) if i != idx)
    w0 = World(a=w.a, b=w.b, net=_canon_msgs(net_rest), seen=w.seen)

    recv_role = msg.dst
    recv = w0.get_party(recv_role)

    if msg.kind == "ADV":
        assert msg.adv_id is not None
        recv2, accepted = _recv_adv(recv, msg.adv_id)
    elif msg.kind == "CTXT":
        assert msg.target_id is not None
        recv2, accepted = _recv_ctxt(recv, msg.target_id)
    else:
        raise AssertionError("unknown message kind")

    # P4: reject => no state change
    if not accepted:
        assert recv2 == recv
        w0.assert_invariants()
        return w0

    w1 = w0.with_party(recv_role, recv2)
    w1.assert_invariants()
    return w1


def _recv_adv(recv: Party, adv_id: int) -> Tuple[Party, bool]:
    old_max = recv.peer_max_adv_id_seen
    if adv_id <= old_max:
        return recv, False

    recv2 = Party(
        peer_max_adv_id_seen=adv_id,
        peer_current_adv_id=adv_id,
        local_next_adv_id=recv.local_next_adv_id,
        local_keys=recv.local_keys,
        tombstones=recv.tombstones,
    )

    # P1: strict monotonicity
    assert recv2.peer_max_adv_id_seen > old_max
    return recv2, True


def _recv_ctxt(recv: Party, target_id: int) -> Tuple[Party, bool]:
    # P3: tombstones are checked first
    if target_id in recv.tombstones:
        return recv, False
    if target_id not in recv.local_keys:
        return recv, False

    new_keys = tuple(k for k in recv.local_keys if k != target_id)
    recv2 = Party(
        peer_max_adv_id_seen=recv.peer_max_adv_id_seen,
        peer_current_adv_id=recv.peer_current_adv_id,
        local_next_adv_id=recv.local_next_adv_id,
        local_keys=new_keys,
        tombstones=_sorted_unique_ints((*recv.tombstones, target_id)),
    )

    # P2/P5: consumed => tombstoned and removed from registry
    assert target_id not in recv2.local_keys
    assert target_id in recv2.tombstones
    return recv2, True


def explore(*, max_depth: int = 7, max_net: int = 6, max_seen: int = 10) -> Dict[str, int]:
    """Explore executions within the given bounds.

    Returns exploration stats; raises AssertionError on any invariant violation.
    """

    init = World()
    init.assert_invariants()

    visited: Set[World] = set([init])
    frontier: List[Tuple[World, int]] = [(init, 0)]

    states = 0
    transitions = 0

    while frontier:
        w, depth = frontier.pop()
        states += 1
        if depth >= max_depth:
            continue

        # 1) sender actions
        for role in (ROLE_A, ROLE_B):
            for action in (action_send_adv, action_send_ctxt):
                w1 = action(w, role, max_net=max_net, max_seen=max_seen)
                if w1 is not None and w1 not in visited:
                    visited.add(w1)
                    frontier.append((w1, depth + 1))
                    transitions += 1

        # 2) adversary delivery (reorder/delay by selecting any queued message)
        for i in range(len(w.net)):
            w1 = _deliver(w, i)
            if w1 not in visited:
                visited.add(w1)
                frontier.append((w1, depth + 1))
                transitions += 1

        # 3) adversary replay (re-inject any previously seen message)
        for msg in w.seen:
            w1 = _add_message(w, msg, max_net=max_net, max_seen=max_seen)
            if w1 is not None and w1 not in visited:
                visited.add(w1)
                frontier.append((w1, depth + 1))
                transitions += 1

    return {"states": states, "transitions": transitions, "visited": len(visited)}
